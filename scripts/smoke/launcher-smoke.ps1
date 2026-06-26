[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")

$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot

function Assert-Equal {
    param(
        [Parameter(Mandatory = $true)]$Actual,
        [Parameter(Mandatory = $true)]$Expected,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if ($Actual -ne $Expected) {
        throw "Unexpected $Description`: expected $Expected, got $Actual"
    }
    Write-Host "OK $Description`: $Actual"
}

function Assert-ContainsValue {
    param(
        [Parameter(Mandatory = $true)]$Values,
        [Parameter(Mandatory = $true)][string]$Expected,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if (@($Values) -notcontains $Expected) {
        throw "Missing $Description`: $Expected"
    }
    Write-Host "OK $Description`: $Expected"
}

function Assert-NotContainsValue {
    param(
        [Parameter(Mandatory = $true)]$Values,
        [Parameter(Mandatory = $true)][string]$Unexpected,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if (@($Values) -contains $Unexpected) {
        throw "Unexpected $Description`: $Unexpected"
    }
    Write-Host "OK absent $Description`: $Unexpected"
}

function Assert-Matches {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if ($Text -notmatch $Pattern) {
        throw "Missing $Description`: $Pattern"
    }
    Write-Host "OK $Description`: $Pattern"
}

function Assert-NotMatches {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if ($Text -match $Pattern) {
        throw "Unexpected $Description`: $Pattern"
    }
    Write-Host "OK absent $Description`: $Pattern"
}

$launcherScript = Join-Path $RepoRoot "scripts\gui\eplus-rs-launch.ps1"
if (-not (Test-Path -LiteralPath $launcherScript -PathType Leaf)) {
    throw "Missing launcher script: $launcherScript"
}
$launcherBuildScript = Join-Path $RepoRoot "scripts\gui\build-launcher-exe.ps1"
if (-not (Test-Path -LiteralPath $launcherBuildScript -PathType Leaf)) {
    throw "Missing launcher build script: $launcherBuildScript"
}
$releasePackageScript = Join-Path $RepoRoot "scripts\release\package.ps1"
if (-not (Test-Path -LiteralPath $releasePackageScript -PathType Leaf)) {
    throw "Missing release package script: $releasePackageScript"
}
$launcherText = Get-Content -Encoding UTF8 -Raw -LiteralPath $launcherScript
Assert-Matches -Text $launcherText -Pattern "Resolve-EplusRsExe" -Description "launcher binary resolver"
Assert-Matches -Text $launcherText -Pattern "New-LauncherRunArguments" -Description "launcher run command builder"
Assert-Matches -Text $launcherText -Pattern "Read-RunSummaryStatus" -Description "launcher run-summary reader"
Assert-Matches -Text $launcherText -Pattern "Read-RunDiagnostics" -Description "launcher diagnostics reader"
Assert-Matches -Text $launcherText -Pattern "support-report\.md" -Description "launcher support report link"
Assert-NotMatches -Text $launcherText -Pattern "support-assessment\s" -Description "launcher support pre-step command"
$releasePackageText = Get-Content -Encoding UTF8 -Raw -LiteralPath $releasePackageScript
Assert-Matches -Text $releasePackageText -Pattern "build-launcher-exe\.ps1" -Description "release launcher build wiring"
Assert-Matches -Text $releasePackageText -Pattern "eplus-rs-launch\.exe" -Description "release launcher exe asset"

Write-Host "Running launcher script self-test."
$selfTestOutput = & powershell -NoProfile -ExecutionPolicy Bypass -File $launcherScript -SelfTest 2>&1
if ($LASTEXITCODE -ne 0) {
    $selfTestOutput | ForEach-Object { Write-Host $_ }
    throw "Launcher script self-test failed with exit code $LASTEXITCODE"
}
$selfTest = ($selfTestOutput -join "`n") | ConvertFrom-Json
Assert-Equal -Actual $selfTest.self_test -Expected "passed" -Description "launcher self-test status"

$diagnosticArgs = @($selfTest.diagnostic_arguments)
foreach ($required in @("run", "--mode", "diagnostic", "--partial", "allow", "--format", "rust-native", "--trace-level", "debug", "--fail-on-warning", "--overwrite")) {
    Assert-ContainsValue -Values $diagnosticArgs -Expected $required -Description "diagnostic command argument"
}
foreach ($unexpected in @("-w", "--oracle-baseline", "--compare-oracle")) {
    Assert-NotContainsValue -Values $diagnosticArgs -Unexpected $unexpected -Description "diagnostic command argument"
}

$baselineArgs = @($selfTest.baseline_arguments)
foreach ($required in @("run", "input.idf", "-w", "weather.epw", "--mode", "compatibility", "--partial", "deny", "--oracle-baseline", "--oracle-root", "oracle-root")) {
    Assert-ContainsValue -Values $baselineArgs -Expected $required -Description "baseline command argument"
}
Assert-NotContainsValue -Values $baselineArgs -Unexpected "--compare-oracle" -Description "baseline command argument"

$compareArgs = @($selfTest.compare_arguments)
foreach ($required in @("run", "input.idf", "-w", "weather.epw", "--mode", "compatibility", "--partial", "deny", "--format", "both", "--trace-level", "detailed", "--compare-oracle", "--oracle-root", "oracle-root")) {
    Assert-ContainsValue -Values $compareArgs -Expected $required -Description "compare command argument"
}
Assert-NotContainsValue -Values $compareArgs -Unexpected "--oracle-baseline" -Description "compare command argument"

$presentations = @($selfTest.state_presentations)
foreach ($state in @("run_blocked", "partial_supported_run", "supported_compatibility_run")) {
    $match = @($presentations | Where-Object { $_.state_id -eq $state })
    if ($match.Count -lt 1) {
        throw "Missing launcher state presentation: $state"
    }
    Assert-Matches -Text ([string]$match[0].detail) -Pattern "exit_code=" -Description "$state exit code detail"
    Assert-Matches -Text ([string]$match[0].detail) -Pattern "conformance_claim=false" -Description "$state claim boundary"
}
$blockedOracle = @($presentations | Where-Object {
        $_.state_id -eq "run_blocked" -and
        $_.detail -match "oracle=generated" -and
        $_.detail -match "compare=skipped-rust-unsupported-or-oracle-missing"
    })
Assert-Equal -Actual $blockedOracle.Count -Expected 1 -Description "blocked run oracle/compare presentation"
Assert-Matches -Text ([string]$selfTest.phase_line) -Pattern "support_assessment" -Description "phase timing support assessment"
Assert-Matches -Text ([string]$selfTest.phase_line) -Pattern "ep_run" -Description "phase timing engine"

$launcherExe = Join-Path $RepoRoot ".runtime\launcher-smoke\eplus-rs-launch.exe"
Write-Host "Building launcher executable self-test: $launcherExe"
$buildOutput = & powershell -NoProfile -ExecutionPolicy Bypass -File $launcherBuildScript -OutputPath $launcherExe -SelfTest 2>&1
if ($LASTEXITCODE -ne 0) {
    $buildOutput | ForEach-Object { Write-Host $_ }
    throw "Launcher executable self-test failed with exit code $LASTEXITCODE"
}
$build = ($buildOutput -join "`n") | ConvertFrom-Json
Assert-Equal -Actual $build.output_type -Expected "WindowsApplication" -Description "launcher executable output type"
if ([int64]$build.bytes -le 0) {
    throw "Launcher executable was empty: $($build.output_path)"
}
Write-Host "OK launcher executable bytes: $($build.bytes)"

Write-Host "Launcher smoke passed."
