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
    "runtime_output_registry_resolves_declared_model_outputs",
    "runtime_output_registry_diagnoses_unavailable_output",
    "runtime_meter_registry_diagnoses_unavailable_meter",
    "result_store_diagnostics_report_duplicate_handles",
    "execution_plan_orders_weather_schedule_zone_and_output"
)

foreach ($test in $tests) {
    Write-Host "Running ep_runtime::$test"
    & $cargo.Source test -p ep_runtime $test --quiet
    if ($LASTEXITCODE -ne 0) {
        throw "Runtime registry smoke failed at $test."
    }
}

Write-Host "Runtime registry smoke passed."
