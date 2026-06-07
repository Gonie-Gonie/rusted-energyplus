[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

$tests = @(
    "heat_balance_adiabatic_surfaces_do_not_create_artificial_losses",
    "heat_balance_interzone_surface_uses_adjacent_zone_temperature",
    "heat_balance_missing_interzone_surface_target_fails",
    "heat_balance_trace_writes_zone_air_temperature_results"
)

foreach ($test in $tests) {
    Write-Host "Running ep_runtime::$test"
    & $cargo.Source test -p ep_runtime $test --quiet
    if ($LASTEXITCODE -ne 0) {
        throw "Heat-balance generalization smoke failed at $test."
    }
}

Invoke-DevCommand -Command "compare-heat-balance-conformance"
Invoke-DevCommand -Command "compare-surface-temperature-conformance"

Write-Host "Heat-balance generalization smoke passed."
