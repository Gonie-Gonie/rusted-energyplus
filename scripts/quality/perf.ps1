[CmdletBinding()]
param(
    [string]$Version = "0.1.0",
    [switch]$SkipGateRun
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")

$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot

$evidenceRoot = Join-Path $RepoRoot ".runtime\release-evidence\v$Version"
$numericJson = Join-Path $evidenceRoot "numeric-conformance-evidence.json"
if (-not (Test-Path -LiteralPath $numericJson -PathType Leaf)) {
    $args = @("-Version", $Version, "-TimingRepeats", "1")
    if ($SkipGateRun) {
        $args += "-SkipGateRun"
    }
    Invoke-DevCommand -Command "conformance-evidence-report" -Arguments $args
}

Invoke-DevCommand -Command "performance-summary" -Arguments @("-Version", $Version)

$summaryPath = Join-Path $evidenceRoot "performance-summary.json"
if (-not (Test-Path -LiteralPath $summaryPath -PathType Leaf)) {
    throw "Missing performance summary: $summaryPath"
}
$summary = Get-Content -LiteralPath $summaryPath -Raw -Encoding UTF8 | ConvertFrom-Json
if (@($summary.cases).Count -lt 1) {
    throw "Performance summary contains no cases."
}
if (-not $summary.measurement_definitions.energyplus_cli_oracle) {
    throw "Performance summary is missing measurement definitions."
}

Write-Host "Performance check complete."
Write-Host "  version: $Version"
Write-Host "  cases: $(@($summary.cases).Count)"
Write-Host "  summary: $summaryPath"