[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot

if ($null -eq (Get-Command cargo -ErrorAction SilentlyContinue)) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

cargo fmt --all -- --check
if ($LASTEXITCODE -ne 0) { throw "cargo fmt failed" }

cargo clippy --workspace --all-targets -- -D warnings
if ($LASTEXITCODE -ne 0) { throw "cargo clippy failed" }

cargo test --workspace
if ($LASTEXITCODE -ne 0) { throw "cargo test failed" }

Invoke-DevCommand -Command "source-quality-gate"
Invoke-DevCommand -Command "schedule-compact-smoke"
Invoke-DevCommand -Command "geometry-smoke"
Invoke-DevCommand -Command "compare-geometry-smoke"
Invoke-DevCommand -Command "compare-surface-geometry-smoke"
Invoke-DevCommand -Command "compare-construction-materials-smoke"
Invoke-DevCommand -Command "compare-internal-gains-smoke"
Invoke-DevCommand -Command "compare-internal-convective-gain-smoke"
Invoke-DevCommand -Command "compare-internal-convective-gain-conformance"
Invoke-DevCommand -Command "conformance-schema-smoke"
Invoke-DevCommand -Command "manifest-validate-all"
Invoke-DevCommand -Command "family-manifest-validate"
Invoke-DevCommand -Command "conformance-report-smoke"
Invoke-DevCommand -Command "official-baseline-smoke"
Invoke-DevCommand -Command "conformance-diagnostic-report-smoke"
Invoke-DevCommand -Command "compare-heat-balance-conformance"
Invoke-DevCommand -Command "compare-surface-temperature-conformance"
Invoke-DevCommand -Command "official-dynamic-heat-balance-diagnostic"
Invoke-DevCommand -Command "compare-schedule-conformance"
Invoke-DevCommand -Command "compare-weather-conformance"
Invoke-DevCommand -Command "compare-static-model-conformance"
Invoke-DevCommand -Command "compare-series-v2-smoke"
Invoke-DevCommand -Command "arbitrary-run-smoke"
Invoke-DevCommand -Command "launcher-smoke"
Invoke-DevCommand -Command "runtime-registry-smoke"
Invoke-DevCommand -Command "heat-balance-generalization-smoke"
Invoke-DevCommand -Command "ideal-loads-thermostat-smoke"
Invoke-DevCommand -Command "air-side-node-diagnostic-smoke"
Invoke-DevCommand -Command "plant-loop-skeleton-smoke"
Invoke-DevCommand -Command "plant-loop-diagnostic-smoke"
Invoke-DevCommand -Command "plant-loop-projection-smoke"
Invoke-DevCommand -Command "docs-check"
Invoke-DevCommand -Command "file-size-check"
Invoke-DevCommand -Command "heat-balance-structure-audit"
Invoke-DevCommand -Command "ideal-loads-structure-audit"
Invoke-DevCommand -Command "ideal-loads-claim-inventory-audit"
Invoke-DevCommand -Command "strict-no-false-conformance"
Invoke-DevCommand -Command "project-contract-check"
Invoke-DevCommand -Command "capability-registry-check"
Invoke-DevCommand -Command "source-smoke"
Invoke-DevCommand -Command "algorithm-ledger-check"
Invoke-DevCommand -Command "variable-coverage-check"
Invoke-DevCommand -Command "pr-port-ticket-check" -Arguments @("-SelfTest")
Invoke-DevCommand -Command "python-smoke"
Invoke-DevCommand -Command "one-zone-family-report"
Invoke-DevCommand -Command "ideal-loads-family-report"
Invoke-DevCommand -Command "conformance-index-report"
Invoke-DevCommand -Command "support-coverage-report"

Write-Host "Check complete."
