[CmdletBinding()]
param(
    [string]$Version = "0.2.0",
    [ValidateSet("windows-x64")][string]$Target = "windows-x64",
    [string]$OracleRuntimeRoot = ".runtime\energyplus\26.1.0"
)
$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
. (Join-Path $PSScriptRoot "..\lib\common.ps1")
Add-CargoBinToPath
$RepoRoot = Get-RepoRoot
Set-Location -LiteralPath $RepoRoot
if ($Version -notmatch '^\d+\.\d+\.\d+$') { throw "Expected numeric MAJOR.MINOR.PATCH." }
$DistRoot = [IO.Path]::GetFullPath((Join-Path $RepoRoot "dist"))
$StageRoot = Join-Path $DistRoot "stage\eplus-rs-v$Version-$Target"
$ZipPath = Join-Path $DistRoot "eplus-rs-v$Version-$Target.zip"
$Utf8 = New-Object System.Text.UTF8Encoding($false)

function Get-CheckoutContext {
    $commit = (& git -C $RepoRoot rev-parse --verify HEAD | Out-String).Trim()
    if ($LASTEXITCODE -ne 0 -or $commit -notmatch '^[0-9a-f]{40}$') { throw 'Cannot identify producing Git HEAD.' }
    $status = @(& git -C $RepoRoot status --porcelain=v1 --untracked-files=all)
    if ($LASTEXITCODE -ne 0) { throw 'Cannot read producing checkout status.' }
    $diff = @(& git -C $RepoRoot diff --no-ext-diff --no-textconv --no-color --binary HEAD --)
    if ($LASTEXITCODE -ne 0) { throw 'Cannot identify producing tracked changes.' }
    $untracked = @(& git -c core.quotepath=false -C $RepoRoot ls-files --others --exclude-standard)
    if ($LASTEXITCODE -ne 0) { throw 'Cannot identify producing untracked inputs.' }
    $extra = @($untracked | ForEach-Object {
        $file = Join-Path $RepoRoot $_
        "$_ $((Get-ReleaseFileHash -LiteralPath $file -Algorithm SHA256).Hash)"
    })
    $fingerprint = ($status -join "`n") + "`nDIFF`n" + ($diff -join "`n") + "`nUNTRACKED`n" + ($extra -join "`n")
    $hasher = [Security.Cryptography.SHA256]::Create()
    try { $hash = [BitConverter]::ToString($hasher.ComputeHash($Utf8.GetBytes($fingerprint))).Replace('-', '').ToLowerInvariant() }
    finally { $hasher.Dispose() }
    return [pscustomobject]@{ source_commit = $commit; source_tree_clean = ($status.Count -eq 0); source_checkout_sha256 = $hash }
}

function Assert-DistPath([string]$Path) {
    $full = [IO.Path]::GetFullPath($Path)
    if (-not $full.StartsWith($DistRoot + [IO.Path]::DirectorySeparatorChar, [StringComparison]::OrdinalIgnoreCase)) {
        throw "Refusing replacement outside dist: $full"
    }
    $ancestor = $full
    while ($ancestor.StartsWith($DistRoot, [StringComparison]::OrdinalIgnoreCase)) {
        if ((Test-Path -LiteralPath $ancestor) -and
            ((Get-Item -LiteralPath $ancestor -Force).Attributes -band [IO.FileAttributes]::ReparsePoint)) {
            throw "Refusing a reparse point in dist replacement path: $ancestor"
        }
        if ($ancestor.Equals($DistRoot, [StringComparison]::OrdinalIgnoreCase)) { break }
        $ancestor = Split-Path -Parent $ancestor
    }
    return $full
}
function Copy-PackageFile([string]$Source, [string]$Relative) {
    $destination = Assert-DistPath (Join-Path $StageRoot $Relative)
    if (-not (Test-Path -LiteralPath $Source -PathType Leaf)) { throw "Missing package input: $Source" }
    [void][IO.Directory]::CreateDirectory((Split-Path -Parent $destination))
    if (Test-Path -LiteralPath $destination) {
        if ((Get-ReleaseFileHash -LiteralPath $Source -Algorithm SHA256).Hash -ne (Get-ReleaseFileHash -LiteralPath $destination -Algorithm SHA256).Hash) { throw "Conflicting package input: $Relative" }
        return
    }
    Copy-Item -LiteralPath $Source -Destination $destination
}
function Write-PackageText([string]$Relative, [string]$Text) {
    $destination = Assert-DistPath (Join-Path $StageRoot $Relative)
    [void][IO.Directory]::CreateDirectory((Split-Path -Parent $destination))
    [IO.File]::WriteAllText($destination, $Text, $Utf8)
}
# Import metadata only: this scanner never loads a dependency binary.
function Get-PeImports([string]$Path) {
    $bytes = [IO.File]::ReadAllBytes($Path)
    function U16([int]$At) { return [BitConverter]::ToUInt16($bytes, $At) }
    function U32([int]$At) { return [BitConverter]::ToUInt32($bytes, $At) }
    $pe = [int](U32 60)
    if ((U16 0) -ne 0x5a4d -or (U32 $pe) -ne 0x4550) { throw "Invalid PE image: $Path" }
    $optional = $pe + 24
    if ((U16 ($pe + 4)) -ne 0x8664 -or (U16 $optional) -ne 0x20b) { throw "Expected x64 PE: $Path" }
    $sections = @(); $sectionAt = $optional + (U16 ($pe + 20))
    for ($i = 0; $i -lt (U16 ($pe + 6)); $i++) {
        $at = $sectionAt + 40 * $i
        $sections += @{ Rva = U32 ($at + 12); Size = [Math]::Max((U32 ($at + 8)), (U32 ($at + 16))); Raw = U32 ($at + 20) }
    }
    function Offset([uint32]$Rva) {
        foreach ($section in $sections) {
            if ($Rva -ge $section.Rva -and $Rva -lt $section.Rva + $section.Size) { return [int]($section.Raw + $Rva - $section.Rva) }
        }
        throw "Unmapped PE import RVA: $Path"
    }
    function DllName([uint32]$Rva) {
        $at = Offset $Rva; $end = $at
        while ($end -lt $bytes.Length -and $bytes[$end] -ne 0) { $end++ }
        if ($end -eq $bytes.Length) { throw "Unterminated PE import: $Path" }
        return [Text.Encoding]::ASCII.GetString($bytes, $at, $end - $at)
    }
    $names = New-Object 'System.Collections.Generic.HashSet[string]' ([StringComparer]::OrdinalIgnoreCase)
    $rva = U32 ($optional + 120)
    if ($rva -ne 0) {
        $at = Offset $rva
        while ((U32 ($at + 12)) -ne 0) { [void]$names.Add((DllName (U32 ($at + 12)))); $at += 20 }
    }
    $rva = U32 ($optional + 216)
    if ($rva -ne 0) {
        $at = Offset $rva
        while ((U32 ($at + 4)) -ne 0) {
            if (((U32 $at) -band 1) -eq 0) { throw "Unsupported VA-based delay import: $Path" }
            [void]$names.Add((DllName (U32 ($at + 4)))); $at += 32
        }
    }
    return @($names | Sort-Object)
}
$DependencyIndex = New-Object 'System.Collections.Generic.List[object]'
function Copy-NativeNotices([string]$Name, [string]$Source, [string]$Kind) {
    if ($Kind -eq 'oracle' -and $Name -ieq 'energyplusapi.dll') { return 'licenses/EnergyPlus-LICENSE.txt' }
    if ($Kind -eq 'oracle' -and $Name -match '^python\d+\.dll$') {
        Copy-PackageFile (Join-Path (Split-Path -Parent $Source) 'PythonLicense.txt') 'licenses/PythonLicense.txt'
        return 'licenses/PythonLicense.txt'
    }
    $category = if ($Name -match '^lib(gcc|stdc\+\+|winpthread|gomp|atomic|quadmath|gfortran|ssp)') { 'gnu-runtime' }
        elseif ($Name -match '^(msvcp|vcruntime|concrt|vcomp)' -or $Name -ieq 'ucrtbase.dll') { 'msvc-runtime' }
        else { [IO.Path]::GetFileNameWithoutExtension($Name) }
    $noticeRoot = Join-Path $RepoRoot "licenses\native\$category"
    $notices = @(Get-ChildItem -LiteralPath $noticeRoot -File -ErrorAction SilentlyContinue | Where-Object { $_.Name -match '^(LICENSE|COPYING|NOTICE)' })
    if ($notices.Count -eq 0) { throw "Missing distributor notices for $Name. Supply actual LICENSE/COPYING/NOTICE in licenses/native/$category." }
    foreach ($notice in $notices) { Copy-PackageFile $notice.FullName "licenses/native/$category/$($notice.Name)" }
    return "licenses/native/$category"
}
function Copy-NativeClosure([string[]]$Seeds, [string[]]$SearchRoots, [string]$Destination, [string]$Kind) {
    $queue = New-Object 'System.Collections.Generic.Queue[string]'
    $seen = New-Object 'System.Collections.Generic.HashSet[string]' ([StringComparer]::OrdinalIgnoreCase)
    foreach ($seed in $Seeds) { $queue.Enqueue($seed) }
    while ($queue.Count -gt 0) {
        $image = $queue.Dequeue()
        if (-not $seen.Add([IO.Path]::GetFullPath($image))) { continue }
        foreach ($name in @(Get-PeImports $image)) {
            if ($name -ne [IO.Path]::GetFileName($name)) { throw "Invalid dependency file name: $name" }
            if ($name -match '^(api-ms-|ext-ms-)') { continue } # Windows10+ API sets.
            $found = $null
            foreach ($root in @((Split-Path -Parent $image)) + $SearchRoots) {
                if ([string]::IsNullOrWhiteSpace($root)) { continue }
                $fullRoot = [IO.Path]::GetFullPath($root)
                if ($fullRoot.StartsWith($env:SystemRoot + '\', [StringComparison]::OrdinalIgnoreCase)) { continue }
                $candidate = Join-Path $fullRoot $name
                if (Test-Path -LiteralPath $candidate -PathType Leaf) { $found = $candidate; break }
            }
            if ($null -eq $found) {
                if ($name -match '^(msvcp|vcruntime|concrt|vcomp|python\d+|energyplusapi)|^lib(gcc|stdc\+\+|winpthread|gomp|atomic|quadmath|gfortran|ssp)' -or $name -ieq 'ucrtbase.dll') {
                    throw "Non-system runtime $name must be bundled from its distributor; System32 presence is insufficient."
                }
                if (Test-Path -LiteralPath (Join-Path $env:SystemRoot "System32\$name") -PathType Leaf) {
                    $DependencyIndex.Add(@{ image = [IO.Path]::GetFileName($image); name = $name; provider = 'Windows System32' })
                    continue
                }
                throw "Unresolved portable dependency $name imported by $image"
            }
            $notice = Copy-NativeNotices $name $found $Kind
            Copy-PackageFile $found "$Destination/$name"
            $DependencyIndex.Add(@{ image = [IO.Path]::GetFileName($image); name = $name; provider = "$Destination/$name"; notice = $notice })
            $queue.Enqueue($found)
        }
    }
}
$producingCheckout = Get-CheckoutContext
cargo build --locked -p ep_cli --release
if ($LASTEXITCODE -ne 0) { throw 'Locked CLI release build failed.' }
$metadataText = & cargo metadata --locked --format-version 1
if ($LASTEXITCODE -ne 0) { throw 'Locked Cargo dependency metadata failed.' }
$metadata = ($metadataText -join "`n") | ConvertFrom-Json
$sysroot = (& rustc --print sysroot | Out-String).Trim()
if ($LASTEXITCODE -ne 0) { throw 'Cannot resolve pinned Rust sysroot.' }
foreach ($old in @($StageRoot, $ZipPath)) {
    $resolved = Assert-DistPath $old
    if (Test-Path -LiteralPath $resolved) { Remove-Item -LiteralPath $resolved -Recurse -Force }
}
[void][IO.Directory]::CreateDirectory($StageRoot)
$files = [ordered]@{
    'target/release/eplus-rs.exe' = 'bin/eplus-rs.exe'
    'README.md' = 'README.md'; 'QUICKSTART.md' = 'QUICKSTART.md'; 'SUPPORT.md' = 'SUPPORT.md'
    'LICENSE' = 'LICENSE'; 'THIRD_PARTY_NOTICES.md' = 'THIRD_PARTY_NOTICES.md'; 'CHANGELOG.md' = 'CHANGELOG.md'
    'licenses/EnergyPlus-LICENSE.txt' = 'licenses/EnergyPlus-LICENSE.txt'
    'crates/ep_re2/LICENSE.RE2' = 'licenses/RE2-LICENSE.txt'
    'Cargo.lock' = 'Cargo.lock'
    'specs/object_coverage.toml' = 'specs/object_coverage.toml'; 'specs/capabilities.toml' = 'specs/capabilities.toml'
    'docs/validation/cases/A-24H/input.idf' = 'examples/source-a.idf'
    'data/testcases/minimal/ptac-diagnostic.epJSON' = 'examples/blocked.epJSON'
    "docs/src/releases/v$Version.md" = 'RELEASE-NOTES.md'
    'docs/src/guides/supported-inputs.md' = 'SUPPORTED-INPUTS.md'
    'docs/src/generated/example-object-union.md' = 'OBJECTS.md'
    'docs/src/generated/object-coverage.md' = 'OBJECT-DETAILS.md'
}
function Copy-UserMarkdown([string]$Source, [string]$Destination) {
    $sourcePath = Join-Path $RepoRoot $Source
    if (-not (Test-Path -LiteralPath $sourcePath -PathType Leaf)) { throw "Missing user documentation: $Source" }
    $text = [IO.File]::ReadAllText($sourcePath)
    $text = [regex]::Replace($text, '(?<!!)\[(?<label>[^\]\r\n]*)\]\((?<link>[^)\r\n]+)\)', {
        param($match)
        $link = $match.Groups['link'].Value
        if ($link -match '^(https?://|mailto:|#)') { return $match.Value }
        $parts = $link -split '#', 2
        $target = [IO.Path]::GetFullPath((Join-Path (Split-Path -Parent $sourcePath) ([Uri]::UnescapeDataString($parts[0]))))
        if (-not $target.StartsWith($RepoRoot + '\', [StringComparison]::OrdinalIgnoreCase) -or -not (Test-Path -LiteralPath $target -PathType Leaf)) {
            throw "Unresolved relative documentation link in ${Source}: $link"
        }
        $relative = $target.Substring($RepoRoot.Length + 1).Replace('\', '/')
        if ($files.Contains($relative)) { $rewritten = $files[$relative] }
        else {
            $encoded = (($relative -split '/' | ForEach-Object { [Uri]::EscapeDataString($_) }) -join '/')
            $rewritten = "https://github.com/Gonie-Gonie/rusted-energyplus/blob/v$Version/$encoded"
        }
        if ($parts.Length -eq 2) { $rewritten += '#' + $parts[1] }
        return '[' + $match.Groups['label'].Value + '](' + $rewritten + ')'
    })
    Write-PackageText $Destination $text
}
foreach ($source in $files.Keys) {
    if ($source.EndsWith('.md', [StringComparison]::OrdinalIgnoreCase)) { Copy-UserMarkdown $source $files[$source] }
    else { Copy-PackageFile (Join-Path $RepoRoot $source) $files[$source] }
}
$oracle = if ([IO.Path]::IsPathRooted($OracleRuntimeRoot)) { $OracleRuntimeRoot } else { Join-Path $RepoRoot $OracleRuntimeRoot }
Copy-PackageFile (Join-Path $oracle 'WeatherData/USA_CO_Golden-NREL.724666_TMY3.epw') 'examples/weather.epw'
foreach ($sample in @(@('examples/source-a.idf', 'db66ac75d3ed26b6e7cd35c6f09b02a05110e800a16713eccee47e387d6c3755'), @('examples/weather.epw', 'c184b947cd34d41c6d6474d63d66dbb82bc0e6cae4c888edcf837348282bce7f'))) {
    if ((Get-ReleaseFileHash -LiteralPath (Join-Path $StageRoot $sample[0]) -Algorithm SHA256).Hash.ToLowerInvariant() -ne $sample[1]) { throw "Accepted sample bytes differ: $($sample[0])" }
}
$oracleDestination = 'oracle/energyplus/26.1.0'
foreach ($name in @('energyplus.exe', 'ConvertInputFormat.exe', 'Energy+.idd', 'Energy+.schema.epJSON', 'LICENSE.txt')) { Copy-PackageFile (Join-Path $oracle $name) "$oracleDestination/$name" }
Copy-PackageFile (Join-Path $RepoRoot 'tools/oracle/energyplus.lock.toml') 'oracle/energyplus.lock.toml'
$nativeSearch = @((Join-Path $RepoRoot 'target/release'), (Join-Path $sysroot 'bin'), (Join-Path $sysroot 'lib/rustlib/x86_64-pc-windows-gnu/bin'), (Join-Path $sysroot 'lib/rustlib/x86_64-pc-windows-gnu/lib/self-contained')) + ($env:PATH -split ';')
Copy-NativeClosure @((Join-Path $RepoRoot 'target/release/eplus-rs.exe')) $nativeSearch 'bin' 'cli'
Copy-NativeClosure @((Join-Path $oracle 'energyplus.exe'), (Join-Path $oracle 'ConvertInputFormat.exe')) @($oracle) $oracleDestination 'oracle'
$rustLicenses = @()
foreach ($package in @($metadata.packages | Sort-Object name, version)) {
    if ($metadata.workspace_members -contains $package.id) { continue }
    $crateRoot = Split-Path -Parent $package.manifest_path
    $notices = @(Get-ChildItem -LiteralPath $crateRoot -File | Where-Object { $_.Name -match '^(LICENSE|COPYING|NOTICE)' })
    if ($package.license_file) {
        $declared = [IO.Path]::GetFullPath((Join-Path $crateRoot $package.license_file))
        if (-not $declared.StartsWith($crateRoot + [IO.Path]::DirectorySeparatorChar, [StringComparison]::OrdinalIgnoreCase)) { throw "Dependency license escapes crate: $($package.name)" }
        $notices += Get-Item -LiteralPath $declared
    }
    $notices = @($notices | Sort-Object FullName -Unique)
    if ($notices.Count -eq 0) { throw "No distributor notice for resolved Rust dependency $($package.name) $($package.version)." }
    $relative = "licenses/rust/$($package.name)-$($package.version)"
    foreach ($notice in $notices) { Copy-PackageFile $notice.FullName "$relative/$($notice.Name)" }
    $rustLicenses += @{ name = $package.name; version = $package.version; declared_license = $package.license; notices = @($notices | ForEach-Object { "$relative/$($_.Name)" }) }
}
Write-PackageText 'licenses/rust/index.json' (($rustLicenses | ConvertTo-Json -Depth 8) + "`n")
$rustDocs = Join-Path $sysroot 'share/doc/rust'
Copy-PackageFile (Join-Path $rustDocs 'COPYRIGHT-library.html') 'licenses/rust-runtime/COPYRIGHT-library.html'
$runtimeLicenses = @(Get-ChildItem -LiteralPath (Join-Path $rustDocs 'licenses') -File)
if ($runtimeLicenses.Count -eq 0) { throw 'Pinned Rust standard-library license texts are missing.' }
foreach ($notice in $runtimeLicenses) { Copy-PackageFile $notice.FullName "licenses/rust-runtime/licenses/$($notice.Name)" }
foreach ($notice in @('COPYING3', 'COPYING.RUNTIME')) {
    Copy-PackageFile (Join-Path $sysroot "share/doc/$notice") "licenses/rust-runtime/$notice"
}
$launcher = & (Join-Path $RepoRoot 'scripts/launcher/build-launcher-exe.ps1') -OutputPath (Join-Path $StageRoot 'eplus-rs-launch.exe') -SelfTest
if (-not $?) { throw 'Launcher build/self-test failed.' }
$launcherRecord = (($launcher | Out-String).Trim() | ConvertFrom-Json)
$launcherRecord.output_path = 'eplus-rs-launch.exe'
Write-PackageText 'launcher-build.json' (($launcherRecord | ConvertTo-Json -Depth 5) + "`n")
$binaryVersion = & (Join-Path $StageRoot 'bin/eplus-rs.exe') --version
if ($LASTEXITCODE -ne 0 -or $binaryVersion -ne "eplus-rs $Version") { throw "Packaged binary version differs: $binaryVersion" }
$inventory = @(Get-ChildItem -LiteralPath $StageRoot -Recurse -File | Sort-Object FullName | ForEach-Object {
    @{ path = $_.FullName.Substring($StageRoot.Length + 1).Replace('\', '/'); bytes = $_.Length; sha256 = (Get-ReleaseFileHash -LiteralPath $_.FullName -Algorithm SHA256).Hash.ToLowerInvariant() }
})
$endingCheckout = Get-CheckoutContext
if ($endingCheckout.source_commit -ne $producingCheckout.source_commit -or
    $endingCheckout.source_tree_clean -ne $producingCheckout.source_tree_clean -or
    $endingCheckout.source_checkout_sha256 -ne $producingCheckout.source_checkout_sha256) { throw 'Checkout changed while producing the package.' }
$manifest = [ordered]@{ schema = 'eplus-rs.portable-package.v1'; version = $Version; target = $Target;
    source_commit = $producingCheckout.source_commit; source_tree_clean = $producingCheckout.source_tree_clean;
    source_checkout_sha256 = $producingCheckout.source_checkout_sha256;
    algorithm_baseline = 'accepted Source212'; os = 'Windows10+ x64'; oracle_version = '26.1.0'; numerical_claim = 'Existing accepted evidence only; packaging smoke adds no comparison credit.'; rust_dependencies = $rustLicenses; native_dependencies = $DependencyIndex.ToArray(); rust_runtime_notices = 'licenses/rust-runtime'; files = $inventory; inventory_excludes = @('package-manifest.json', 'SHA256SUMS') }
Write-PackageText 'package-manifest.json' (($manifest | ConvertTo-Json -Depth 14) + "`n")
$manifestPath = Join-Path $StageRoot 'package-manifest.json'
$sums = @($inventory | ForEach-Object { "$($_.sha256)  $($_.path)" }) + @("$((Get-ReleaseFileHash -LiteralPath $manifestPath -Algorithm SHA256).Hash.ToLowerInvariant())  package-manifest.json")
Write-PackageText 'SHA256SUMS' (($sums -join "`n") + "`n")
$items = Get-ChildItem -LiteralPath $StageRoot
Compress-Archive -LiteralPath $items.FullName -DestinationPath $ZipPath
$zipHash = (Get-ReleaseFileHash -LiteralPath $ZipPath -Algorithm SHA256).Hash.ToLowerInvariant()
[IO.File]::WriteAllText($ZipPath + '.sha256', "$zipHash  $([IO.Path]::GetFileName($ZipPath))`n", $Utf8)
Write-Host "Package created: $ZipPath"
Write-Host "SHA256: $zipHash"
