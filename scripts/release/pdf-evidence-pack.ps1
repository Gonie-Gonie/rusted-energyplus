[CmdletBinding()]
param(
    [string]$Version = "0.32.0",
    [string]$Target = "windows-x64",
    [switch]$SkipPackage,
    [switch]$SkipGateRun,
    [switch]$RunDynamicDiagnostic,
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

Invoke-DevCommand -Command "conformance-index-report" -Arguments @("-Version", $Version)
Invoke-DevCommand -Command "support-coverage-report" -Arguments @("-Version", $Version)
Invoke-DevCommand -Command "user-coverage-handbook" -Arguments @("-Version", $Version)

$numericArgs = @(
    "-Version", $Version,
    "-TimingRepeats", [string]$TimingRepeats,
    "-DynamicTimingRepeats", [string]$DynamicTimingRepeats
)
if ($SkipGateRun) {
    $numericArgs += "-SkipGateRun"
}
if ($RunDynamicDiagnostic) {
    $numericArgs += "-RunDynamicDiagnostic"
}
Invoke-DevCommand -Command "conformance-evidence-report" -Arguments $numericArgs
Invoke-DevCommand -Command "plot-evidence" -Arguments @("-Version", $Version)
Invoke-DevCommand -Command "release-evidence-manifest" -Arguments @("-Version", $Version, "-Target", $Target)
