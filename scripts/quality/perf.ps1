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
foreach ($measurement in @(
        "parse_time",
        "compile_time",
        "graph_build_time",
        "execution_plan_build_time",
        "runtime_time",
        "output_export_time",
        "report_generation_time",
        "energyplus_cli_time",
        "rust_cli_time",
        "rust_runtime_only_time",
        "trace_overhead_time"
    )) {
    $entry = $summary.required_measurements.$measurement
    if (-not $entry) {
        throw "Performance summary is missing required measurement: $measurement"
    }
    if ([int]$entry.statistics.count -lt 1) {
        throw "Performance summary has no samples for required measurement: $measurement"
    }
}
if (-not $summary.cold_repeated_runs.cold_run) {
    throw "Performance summary is missing cold run timing evidence."
}
if (-not $summary.cold_repeated_runs.repeated_runs) {
    throw "Performance summary is missing repeated run timing evidence."
}
if (-not $summary.artifacts.plots.stage_timing_stacked_bar) {
    throw "Performance summary is missing stage timing plot artifact path."
}
if (-not $summary.artifacts.plots.trace_overhead) {
    throw "Performance summary is missing trace overhead plot artifact path."
}

Write-Host "Performance check complete."
Write-Host "  version: $Version"
Write-Host "  cases: $(@($summary.cases).Count)"
Write-Host "  summary: $summaryPath"
