[CmdletBinding()]
param(
    [string]$Version = "0.2.0",
    [switch]$SkipPackage,
    [switch]$KeepExtracted
)
$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
. (Join-Path $PSScriptRoot "..\lib\common.ps1")
$RepoRoot = Get-RepoRoot
if ($Version -ne '0.2.0') { throw 'This gate verifies the bounded-scope v0.2.0 release.' }
Set-Location -LiteralPath $RepoRoot

function Get-CheckoutContext {
    $commit = (& git -C $RepoRoot rev-parse --verify HEAD | Out-String).Trim()
    if ($LASTEXITCODE -ne 0 -or $commit -notmatch '^[0-9a-f]{40}$') { throw 'Cannot identify verification Git HEAD.' }
    $status = @(& git -C $RepoRoot status --porcelain=v1 --untracked-files=all)
    if ($LASTEXITCODE -ne 0) { throw 'Cannot read verification checkout status.' }
    $diff = @(& git -C $RepoRoot diff --no-ext-diff --no-textconv --no-color --binary HEAD --)
    if ($LASTEXITCODE -ne 0) { throw 'Cannot identify verification tracked changes.' }
    $untracked = @(& git -c core.quotepath=false -C $RepoRoot ls-files --others --exclude-standard)
    if ($LASTEXITCODE -ne 0) { throw 'Cannot identify verification untracked inputs.' }
    $extra = @($untracked | ForEach-Object { "$_ $((Get-ReleaseFileHash -LiteralPath (Join-Path $RepoRoot $_) -Algorithm SHA256).Hash)" })
    $fingerprint = ($status -join "`n") + "`nDIFF`n" + ($diff -join "`n") + "`nUNTRACKED`n" + ($extra -join "`n")
    $encoding = New-Object System.Text.UTF8Encoding($false)
    $hasher = [Security.Cryptography.SHA256]::Create()
    try { $hash = [BitConverter]::ToString($hasher.ComputeHash($encoding.GetBytes($fingerprint))).Replace('-', '').ToLowerInvariant() }
    finally { $hasher.Dispose() }
    return [pscustomobject]@{ source_commit = $commit; source_tree_clean = ($status.Count -eq 0); source_checkout_sha256 = $hash }
}

# These validate metadata/docs only; no conformance, timing or PDF actors.
foreach ($command in @('docs-generate', 'docs-check', 'spec-cross-check', 'project-contract-check')) {
    Invoke-DevCommand -Command $command
}
$verificationCheckout = Get-CheckoutContext
if (-not $SkipPackage) {
    & (Join-Path $PSScriptRoot 'package.ps1') -Version $Version
    if (-not $?) { throw 'Portable package build failed.' }
}
$zip = Join-Path $RepoRoot "dist\eplus-rs-v$Version-windows-x64.zip"
$zipHash = (Get-ReleaseFileHash -LiteralPath $zip -Algorithm SHA256).Hash.ToLowerInvariant()
$externalChecksum = (Get-Content -LiteralPath ($zip + '.sha256') -Raw).Trim()
if ($externalChecksum -ne "$zipHash  $([IO.Path]::GetFileName($zip))") { throw 'ZIP checksum differs.' }

$temporaryBase = [IO.Path]::GetFullPath([IO.Path]::GetTempPath()).TrimEnd('\')
$temporaryRoot = [IO.Path]::GetFullPath((Join-Path $temporaryBase ('eplus-rs-release-' + [Guid]::NewGuid().ToString('N'))))
if (-not $temporaryRoot.StartsWith($temporaryBase + '\', [StringComparison]::OrdinalIgnoreCase) -or
    $temporaryRoot.StartsWith($RepoRoot + '\', [StringComparison]::OrdinalIgnoreCase)) { throw 'Portable smoke must use an external temporary directory.' }
[void][IO.Directory]::CreateDirectory($temporaryRoot)
$extractRoot = Join-Path $temporaryRoot 'package'
$workRoot = Join-Path $temporaryRoot 'work'
[void][IO.Directory]::CreateDirectory($workRoot)
$runs = New-Object 'System.Collections.Generic.List[object]'
$success = $false
$utf8 = New-Object System.Text.UTF8Encoding($false)

function PackagePath([string]$Relative) {
    $full = [IO.Path]::GetFullPath((Join-Path $extractRoot $Relative))
    if (-not $full.StartsWith($extractRoot + '\', [StringComparison]::OrdinalIgnoreCase)) { throw "Package path escapes extraction: $Relative" }
    return $full
}
function QuotedArgument([string]$Value) {
    $escaped = [regex]::Replace($Value, '(\\*)"', '$1$1\"')
    $escaped = [regex]::Replace($escaped, '(\\+)$', '$1$1')
    return '"' + $escaped + '"'
}
function Run-Packaged([string]$Name, [string[]]$Arguments, [int]$ExpectedExit, [string]$Executable = 'bin/eplus-rs.exe') {
    $start = New-Object System.Diagnostics.ProcessStartInfo
    $start.FileName = PackagePath $Executable
    $start.Arguments = (@($Arguments | ForEach-Object { QuotedArgument $_ }) -join ' ')
    $start.WorkingDirectory = $workRoot
    $start.UseShellExecute = $false
    $start.CreateNoWindow = $true
    $start.RedirectStandardOutput = $true
    $start.RedirectStandardError = $true
    # No Cargo, checkout, MinGW or developer-tool DLL directories in child PATH.
    $start.EnvironmentVariables['PATH'] = "$env:SystemRoot\System32;$env:SystemRoot;$env:SystemRoot\System32\Wbem"
    $start.EnvironmentVariables.Remove('RUSTED_ENERGYPLUS_ORACLE_ROOT')
    $process = New-Object System.Diagnostics.Process
    $process.StartInfo = $start
    [void]$process.Start()
    $stdout = $process.StandardOutput.ReadToEndAsync()
    $stderr = $process.StandardError.ReadToEndAsync()
    if (-not $process.WaitForExit(180000)) { $process.Kill(); throw "Packaged $Name exceeded 180 seconds." }
    $exit = $process.ExitCode
    $out = $stdout.Result; $err = $stderr.Result
    [IO.File]::WriteAllText((Join-Path $workRoot "$Name.stdout.txt"), $out, $utf8)
    [IO.File]::WriteAllText((Join-Path $workRoot "$Name.stderr.txt"), $err, $utf8)
    $process.Dispose()
    $runs.Add(@{ name = $Name; arguments = $Arguments; exit_code = $exit; expected_exit = $ExpectedExit })
    if ($exit -ne $ExpectedExit) { throw "Packaged $Name returned $exit, expected $ExpectedExit. See $workRoot" }
    return $out
}
try {
    Add-Type -AssemblyName System.IO.Compression.FileSystem
    $archive = [IO.Compression.ZipFile]::OpenRead($zip)
    try {
        $zipNames = New-Object 'System.Collections.Generic.HashSet[string]' ([StringComparer]::OrdinalIgnoreCase)
        foreach ($entry in $archive.Entries) {
            $name = $entry.FullName.Replace('\', '/')
            if ($name.StartsWith('/') -or $name -match '(^|/)\.\.(/|$)|:' -or -not $zipNames.Add($name)) { throw "Unsafe/duplicate ZIP entry: $name" }
            if ($name -match '(^|/)(__pycache__|\.runtime|\.reference|target|crates|scripts|tools|Lib|site-packages|ExampleFiles|Documentation|PreProcess|PostProcess)(/|$)' -or
                $name -match '\.(pyc|pdb|lib|a)$') { throw "Development/SDK payload in portable package: $name" }
        }
    }
    finally { $archive.Dispose() }
    Expand-Archive -LiteralPath $zip -DestinationPath $extractRoot
    $manifest = Get-Content -LiteralPath (PackagePath 'package-manifest.json') -Raw | ConvertFrom-Json
    if ($manifest.schema -ne 'eplus-rs.portable-package.v1' -or $manifest.version -ne $Version -or
        $manifest.target -ne 'windows-x64' -or $manifest.algorithm_baseline -ne 'accepted Source212') { throw 'Wrong package manifest identity.' }
    if ($manifest.source_tree_clean -isnot [bool] -or $manifest.source_commit -ne $verificationCheckout.source_commit -or
        $manifest.source_tree_clean -ne $verificationCheckout.source_tree_clean -or
        $manifest.source_checkout_sha256 -ne $verificationCheckout.source_checkout_sha256) { throw 'Package producer differs from the current unchanged checkout.' }
    $expected = New-Object 'System.Collections.Generic.HashSet[string]' ([StringComparer]::OrdinalIgnoreCase)
    foreach ($row in $manifest.files) {
        if (-not $expected.Add($row.path)) { throw "Duplicate manifest file: $($row.path)" }
        $file = Get-Item -LiteralPath (PackagePath $row.path)
        if ($file.Length -ne $row.bytes -or (Get-ReleaseFileHash -LiteralPath $file.FullName -Algorithm SHA256).Hash.ToLowerInvariant() -ne $row.sha256) { throw "Manifest bytes differ: $($row.path)" }
    }
    [void]$expected.Add('package-manifest.json'); [void]$expected.Add('SHA256SUMS')
    $actual = @(Get-ChildItem -LiteralPath $extractRoot -Recurse -File)
    if ($actual.Count -ne $expected.Count) { throw 'Unexpected extracted file count.' }
    foreach ($file in $actual) {
        if (-not $expected.Contains($file.FullName.Substring($extractRoot.Length + 1).Replace('\', '/'))) { throw "Unlisted package file: $($file.FullName)" }
    }
    $checksumNames = New-Object 'System.Collections.Generic.HashSet[string]' ([StringComparer]::OrdinalIgnoreCase)
    foreach ($line in Get-Content -LiteralPath (PackagePath 'SHA256SUMS')) {
        if ($line -notmatch '^([0-9a-f]{64})  (.+)$') { throw 'Invalid SHA256SUMS line.' }
        $hash = $Matches[1]; $name = $Matches[2]
        if (-not $checksumNames.Add($name) -or -not $expected.Contains($name) -or $name -eq 'SHA256SUMS' -or
            (Get-ReleaseFileHash -LiteralPath (PackagePath $name) -Algorithm SHA256).Hash.ToLowerInvariant() -ne $hash) { throw "Checksum differs: $name" }
    }
    if ($checksumNames.Count -ne $expected.Count - 1) { throw 'Incomplete SHA256SUMS.' }
    foreach ($required in @('bin/eplus-rs.exe', 'eplus-rs-launch.exe', 'QUICKSTART.md', 'SUPPORT.md', 'LICENSE',
            'THIRD_PARTY_NOTICES.md', 'licenses/RE2-LICENSE.txt', 'licenses/EnergyPlus-LICENSE.txt', 'licenses/rust/index.json',
            'licenses/rust-runtime/COPYRIGHT-library.html', 'licenses/rust-runtime/licenses/MIT.txt',
            'licenses/rust-runtime/licenses/Apache-2.0.txt', 'SUPPORTED-INPUTS.md', 'OBJECTS.md', 'OBJECT-DETAILS.md',
            'examples/source-a.idf', 'examples/blocked.epJSON', 'examples/weather.epw', 'specs/object_coverage.toml',
            'oracle/energyplus/26.1.0/energyplus.exe', 'oracle/energyplus/26.1.0/ConvertInputFormat.exe',
            'oracle/energyplus/26.1.0/Energy+.idd', 'oracle/energyplus/26.1.0/Energy+.schema.epJSON')) {
        if (-not $expected.Contains($required)) { throw "Missing required portable file: $required" }
    }
    $launcher = Get-Content -LiteralPath (PackagePath 'launcher-build.json') -Raw | ConvertFrom-Json
    if ($launcher.output_type -ne 'WindowsApplication' -or $launcher.starts_powershell -ne $false -or
        $launcher.rejects_stale_saved_exe -ne $true) { throw 'Launcher builder self-test differs.' }
    if ((Run-Packaged 'version' @('--version') 0).Trim() -ne "eplus-rs $Version") { throw 'Extracted version differs.' }
    $help = Run-Packaged 'help' @('--help') 0
    if ($help -notmatch 'eplus-rs') { throw 'Extracted CLI help missing.' }
    $oracleVersion = Run-Packaged 'oracle-version' @('--version') 0 'oracle/energyplus/26.1.0/energyplus.exe'
    if ($oracleVersion -notmatch 'EnergyPlus.*26\.1\.0') { throw 'Extracted reference engine version differs.' }
    $positive = Join-Path $workRoot 'source-a-output'
    [void](Run-Packaged 'source-a' @('run', (PackagePath 'examples/source-a.idf'), '--weather', (PackagePath 'examples/weather.epw'),
        '--output-dir', $positive, '--porting-scope', 'A') 0)
    $summary = Get-Content -LiteralPath (Join-Path $positive 'run-summary.json') -Raw | ConvertFrom-Json
    # pipeline.rs::finish_successful_summary serializes schema1/numeric exit_code/nested support.
    if ($summary.schema_version -ne 1 -or $summary.exit_code -ne 0 -or
        $summary.support.run_result_state -ne 'supported_compatibility_run' -or
        $summary.support.conformance_claim -ne $false -or $null -eq $summary.rust_runtime) { throw 'Source sample completion/claim state differs.' }
    foreach ($relative in @('eplusrs.eso', 'eplusrs.mtr', 'results/source-reported-results.json', 'results/source-eso.csv', 'results/source-mtr.csv')) {
        $file = Get-Item -LiteralPath (Join-Path $positive $relative)
        if ($file.Length -eq 0) { throw "Empty Source export: $relative" }
    }
    $blocked = Join-Path $workRoot 'blocked-output'
    [void](Run-Packaged 'blocked' @('run', (PackagePath 'examples/blocked.epJSON'), '--output-dir', $blocked, '--porting-scope', 'A') 4)
    $blockedSummary = Get-Content -LiteralPath (Join-Path $blocked 'run-summary.json') -Raw | ConvertFrom-Json
    if ($blockedSummary.schema_version -ne 1 -or $blockedSummary.exit_code -ne 4 -or $blockedSummary.support.run_result_state -ne 'run_blocked' -or
        $blockedSummary.support.conformance_claim -ne $false -or $null -ne $blockedSummary.rust_runtime -or
        (Test-Path -LiteralPath (Join-Path $blocked 'results/source-reported-results.json'))) { throw 'Blocked input must remain unsupported with no reported Source results.' }
    $endingCheckout = Get-CheckoutContext
    if ($endingCheckout.source_commit -ne $verificationCheckout.source_commit -or
        $endingCheckout.source_tree_clean -ne $verificationCheckout.source_tree_clean -or
        $endingCheckout.source_checkout_sha256 -ne $verificationCheckout.source_checkout_sha256) { throw 'Checkout changed during portable verification.' }
    $report = @{ schema = 'eplus-rs.portable-verification.v1'; version = $Version; zip_sha256 = $zipHash; files_checked = $actual.Count;
        source_commit = $manifest.source_commit; source_tree_clean = $manifest.source_tree_clean;
        source_checkout_sha256 = $manifest.source_checkout_sha256;
        outside_checkout = $true; developer_PATH_removed = $true; runs = $runs.ToArray(); accepted_algorithm_baseline = 'Source212';
        Native_simulation_calls = 0; comparison_actors = 0; new_scientific_claims = 0; passed = $true }
    [IO.File]::WriteAllText(($zip + '.verification.json'), (($report | ConvertTo-Json -Depth 10) + "`n"), $utf8)
    $success = $true
    Write-Host "v0.2.0 portable release verification passed: $zip"
    if (-not $manifest.source_tree_clean) { Write-Host 'This verified local package is dirty and must not be published.' }
}
finally {
    if ($success -and -not $KeepExtracted) {
        $resolved = [IO.Path]::GetFullPath($temporaryRoot)
        if (-not $resolved.StartsWith($temporaryBase + '\eplus-rs-release-', [StringComparison]::OrdinalIgnoreCase)) { throw 'Refusing unexpected temporary cleanup path.' }
        Remove-Item -LiteralPath $resolved -Recurse -Force
    }
    else { Write-Host "Retained portable verification folder: $temporaryRoot" }
}
