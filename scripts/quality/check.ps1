[CmdletBinding()]
param(
    [string]$Package = "ep_runtime",
    [string]$Filter = "",
    [switch]$Lib,
    [switch]$List,
    [switch]$Full,
    [switch]$Plan
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Set-Location (Get-RepoRoot)

if ($Full) {
    foreach ($name in @("Package", "Filter", "Lib", "List")) {
        if ($PSBoundParameters.ContainsKey($name)) {
            throw "Full integration cannot be combined with Package, Filter, Lib, or List."
        }
    }
}
$packages = @($Package.Split(",") | ForEach-Object { $_.Trim() })
foreach ($name in $packages) {
    if ($name -notmatch '^[A-Za-z0-9][A-Za-z0-9_-]*$') {
        throw "Package must contain comma-separated Cargo package names."
    }
}
if (@($packages | Select-Object -Unique).Count -ne $packages.Count) {
    throw "Package contains duplicate Cargo package names."
}

function Invoke-CargoCheck {
    param([string[]]$Arguments)
    $displayArguments = @($Arguments | ForEach-Object {
        if ($_ -match '[\s"'']') { "'" + $_.Replace("'", "''") + "'" } else { $_ }
    })
    Write-Host ("cargo " + ($displayArguments -join " "))
    if (-not $Plan) {
        & cargo @Arguments
        if ($LASTEXITCODE -ne 0) { throw "cargo check step failed" }
    }
}

if (-not $Plan) {
    Add-CargoBinToPath
    if ($null -eq (Get-Command cargo -ErrorAction SilentlyContinue)) {
        throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
    }
}
$testParameters = @{ Plan = $Plan }
$clippyArguments = @("clippy", "--locked")
if ($Full) {
    Write-Host "Check scope: full workspace quality and broader integration."
    Write-Host "Rust tests run once for the workspace; child smokes retain their additional report/marker checks."
    $clippyArguments += "--workspace"
}
else {
    Write-Host "Check scope: $($packages -join ', '); use -Full for workspace integration."
    foreach ($name in $packages) { $clippyArguments += @("-p", $name) }
    $testParameters.Package = $Package
    if ($Filter) { $testParameters.Filter = $Filter }
    if ($Lib) { $testParameters.Lib = $true }
    if ($List) { $testParameters.List = $true }
}
$clippyArguments += @("--all-targets", "--", "-D", "warnings")
Invoke-CargoCheck -Arguments @("fmt", "--all", "--", "--check")
Invoke-CargoCheck -Arguments $clippyArguments
& (Join-Path $PSScriptRoot "test.ps1") @testParameters
if (-not $Full) {
    if ($Plan) { Write-Host "Plan only; no checks were executed." }
    else { Write-Host "Package check complete." }
    return
}

Invoke-DevCommand -Command "source-quality-gate" -Plan:$Plan
Invoke-DevCommand -Command "schedule-compact-smoke" -Plan:$Plan
Invoke-DevCommand -Command "geometry-smoke" -Plan:$Plan
Invoke-DevCommand -Command "compare-geometry-smoke" -Plan:$Plan
Invoke-DevCommand -Command "compare-surface-geometry-smoke" -Plan:$Plan
Invoke-DevCommand -Command "compare-construction-materials-smoke" -Plan:$Plan
Invoke-DevCommand -Command "compare-internal-gains-smoke" -Plan:$Plan
Invoke-DevCommand -Command "compare-internal-convective-gain-smoke" -Plan:$Plan
Invoke-DevCommand -Command "compare-internal-convective-gain-conformance" -Plan:$Plan
Invoke-DevCommand -Command "conformance-schema-smoke" -Plan:$Plan
Invoke-DevCommand -Command "manifest-validate-all" -Plan:$Plan
Invoke-DevCommand -Command "family-manifest-validate" -Plan:$Plan
Invoke-DevCommand -Command "conformance-report-smoke" -Plan:$Plan
Invoke-DevCommand -Command "official-baseline-smoke" -Plan:$Plan
Invoke-DevCommand -Command "conformance-diagnostic-report-smoke" -Plan:$Plan
Invoke-DevCommand -Command "official-dynamic-heat-balance-diagnostic" -Plan:$Plan
Invoke-DevCommand -Command "compare-schedule-conformance" -Plan:$Plan
Invoke-DevCommand -Command "compare-calendar-schedule-hourly-exact" -Plan:$Plan
Invoke-DevCommand -Command "compare-weather-effective-calendar" -Plan:$Plan
Invoke-DevCommand -Command "compare-calendar-dst-fixed-date-exact" -Plan:$Plan
Invoke-DevCommand -Command "compare-calendar-special-day-fixed-date-exact" -Plan:$Plan
Invoke-DevCommand -Command "compare-calendar-epw-holiday-policy-exact" -Plan:$Plan
Invoke-DevCommand -Command "compare-calendar-weekend-holiday-policy-exact" -Plan:$Plan
Invoke-DevCommand -Command "compare-calendar-weekend-saturday-policy-exact" -Plan:$Plan
Invoke-DevCommand -Command "compare-calendar-special-day-weekday-rules-exact" -Plan:$Plan
Invoke-DevCommand -Command "compare-calendar-special-day-nonexistent-nth-error" -Plan:$Plan
Invoke-DevCommand -Command "compare-calendar-special-day-duration-wrap-exact" -Plan:$Plan
Invoke-DevCommand -Command "compare-calendar-special-day-overlap-order-exact" -Plan:$Plan
Invoke-DevCommand -Command "compare-calendar-special-day-epw-idf-precedence-exact" -Plan:$Plan
Invoke-DevCommand -Command "compare-weather-record-selection" -Plan:$Plan
Invoke-DevCommand -Command "compare-weather-conformance" -Plan:$Plan
Invoke-DevCommand -Command "compare-static-model-conformance" -Plan:$Plan
Invoke-DevCommand -Command "compare-series-v2-smoke" -Arguments @("-SkipRustTests") -Plan:$Plan
Invoke-DevCommand -Command "arbitrary-run-smoke" -Plan:$Plan
Invoke-DevCommand -Command "launcher-smoke" -Plan:$Plan
Invoke-DevCommand -Command "heat-balance-generalization-smoke" -Arguments @("-SkipRustTests") -Plan:$Plan
Invoke-DevCommand -Command "ideal-loads-thermostat-smoke" -Plan:$Plan
Invoke-DevCommand -Command "air-side-node-diagnostic-smoke" -Plan:$Plan
Invoke-DevCommand -Command "plant-loop-skeleton-smoke" -Plan:$Plan
Invoke-DevCommand -Command "plant-loop-diagnostic-smoke" -Plan:$Plan
Invoke-DevCommand -Command "plant-loop-projection-smoke" -Plan:$Plan
Invoke-DevCommand -Command "docs-check" -Plan:$Plan
Invoke-DevCommand -Command "file-size-check" -Plan:$Plan
Invoke-DevCommand -Command "heat-balance-structure-audit" -Plan:$Plan
Invoke-DevCommand -Command "ideal-loads-structure-audit" -Plan:$Plan
Invoke-DevCommand -Command "ideal-loads-claim-inventory-audit" -Plan:$Plan
Invoke-DevCommand -Command "strict-no-false-conformance" -Plan:$Plan
Invoke-DevCommand -Command "project-contract-check" -Arguments @("-SelfTest") -Plan:$Plan
Invoke-DevCommand -Command "spec-cross-check" -Arguments @("-SelfTest") -Plan:$Plan
Invoke-DevCommand -Command "source-smoke" -Plan:$Plan
Invoke-DevCommand -Command "algorithm-ledger-check" -Plan:$Plan
Invoke-DevCommand -Command "algorithm-ledger-check" -Arguments @("-SelfTest") -Plan:$Plan
Invoke-DevCommand -Command "diagnostic-probe-check" -Plan:$Plan
Invoke-DevCommand -Command "diagnostic-probe-check" -Arguments @("-SelfTest") -Plan:$Plan
Invoke-DevCommand -Command "variable-coverage-check" -Plan:$Plan
Invoke-DevCommand -Command "pr-port-ticket-check" -Arguments @("-SelfTest") -Plan:$Plan
Invoke-DevCommand -Command "python-smoke" -Plan:$Plan
Invoke-DevCommand -Command "one-zone-family-report" -Plan:$Plan
Invoke-DevCommand -Command "ideal-loads-family-report" -Plan:$Plan
Invoke-DevCommand -Command "conformance-index-report" -Plan:$Plan
Invoke-DevCommand -Command "support-coverage-report" -Plan:$Plan

if ($Plan) { Write-Host "Full integration plan complete; no checks were executed." }
else { Write-Host "Full integration check complete." }
