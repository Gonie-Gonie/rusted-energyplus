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

Invoke-DevCommand -Command "schedule-compact-smoke"
Invoke-DevCommand -Command "geometry-smoke"
Invoke-DevCommand -Command "compare-geometry-smoke"
Invoke-DevCommand -Command "compare-surface-geometry-smoke"
Invoke-DevCommand -Command "compare-construction-materials-smoke"
Invoke-DevCommand -Command "compare-internal-gains-smoke"
Invoke-DevCommand -Command "compare-internal-convective-gain-smoke"
Invoke-DevCommand -Command "conformance-schema-smoke"
Invoke-DevCommand -Command "conformance-report-smoke"
Invoke-DevCommand -Command "conformance-diagnostic-report-smoke"
Invoke-DevCommand -Command "compare-heat-balance-conformance"
Invoke-DevCommand -Command "compare-surface-temperature-conformance"
Invoke-DevCommand -Command "ideal-loads-thermostat-smoke"
Invoke-DevCommand -Command "air-side-node-diagnostic-smoke"
Invoke-DevCommand -Command "docs-check"
Invoke-DevCommand -Command "strict-no-false-conformance"
Invoke-DevCommand -Command "source-smoke"

Write-Host "Check complete."
