[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot

$required = @(
    "crates\ep_conformance\Cargo.toml",
    "crates\ep_conformance\src\lib.rs",
    "data\conformance_cases\schedule_constant_001\case.toml",
    "data\conformance_cases\schedule_constant_001\schedule_constant.idf",
    "data\conformance_cases\weather_fields_001\case.toml",
    "data\conformance_cases\weather_fields_001\weather_fields.idf",
    "data\conformance_cases\surface_geometry_001\case.toml",
    "data\conformance_cases\surface_geometry_001\surface_geometry.idf",
    "data\conformance_cases\construction_materials_001\case.toml",
    "data\conformance_cases\construction_materials_001\construction_materials.idf",
    "data\conformance_cases\internal_gains_001\case.toml",
    "data\conformance_cases\internal_gains_001\internal_gains.idf",
    "data\conformance_cases\zone_temperature_diagnostic_001\case.toml",
    "data\conformance_cases\zone_temperature_diagnostic_001\zone_temperature.idf",
    "data\conformance_cases\ideal_loads_thermostat_001\case.toml",
    "data\conformance_cases\ideal_loads_thermostat_001\ideal_loads_thermostat.idf",
    "data\conformance_suites\foundation.toml"
)

foreach ($relative in $required) {
    $path = Join-Path $RepoRoot $relative
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing conformance schema fixture: $path"
    }
}

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

& $cargo.Source test -p ep_conformance
if ($LASTEXITCODE -ne 0) {
    throw "ep_conformance tests failed"
}

Write-Host "Conformance schema smoke passed."
