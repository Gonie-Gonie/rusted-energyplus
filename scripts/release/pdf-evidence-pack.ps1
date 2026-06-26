[CmdletBinding()]
param(
    [string]$Version = "0.1.0",
    [string]$Target = "windows-x64",
    [switch]$SkipPackage,
    [switch]$SkipGateRun,
    [switch]$RunDynamicDiagnostic,
    [switch]$SkipDynamicDiagnostic,
    [switch]$SkipArbitraryRunSmoke,
    [int]$TimingRepeats = 3,
    [int]$DynamicTimingRepeats = 1
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")

$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot

if (-not $SkipPackage) {
    Invoke-DevCommand -Command "package" -Arguments @("-Version", $Version, "-Target", $Target)
}

if (-not $SkipArbitraryRunSmoke) {
    Invoke-DevCommand -Command "arbitrary-run-smoke"
}

Invoke-DevCommand -Command "conformance-index-report" -Arguments @("-Version", $Version)
Invoke-DevCommand -Command "support-coverage-report" -Arguments @("-Version", $Version)
Invoke-DevCommand -Command "user-coverage-handbook" -Arguments @("-Version", $Version)
Invoke-DevCommand -Command "one-zone-family-report" -Arguments @("-Version", $Version)

$numericArgs = @(
    "-Version", $Version,
    "-TimingRepeats", [string]$TimingRepeats,
    "-DynamicTimingRepeats", [string]$DynamicTimingRepeats
)
if ($SkipGateRun) {
    $numericArgs += "-SkipGateRun"
}
$includeDynamicDiagnostic = -not $SkipDynamicDiagnostic
if ($RunDynamicDiagnostic) {
    $includeDynamicDiagnostic = $true
}
if ($includeDynamicDiagnostic) {
    $numericArgs += "-RunDynamicDiagnostic"
}
Invoke-DevCommand -Command "conformance-evidence-report" -Arguments $numericArgs
Invoke-DevCommand -Command "performance-summary" -Arguments @("-Version", $Version)
Invoke-DevCommand -Command "stability-summary" -Arguments @("-Version", $Version)
Invoke-DevCommand -Command "plot-evidence" -Arguments @("-Version", $Version)
Invoke-DevCommand -Command "release-evidence-manifest" -Arguments @("-Version", $Version, "-Target", $Target)
