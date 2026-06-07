[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot

function Assert-FileExists {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "Missing $Description`: $Path"
    }
    Write-Host "OK $Description`: $Path"
}

function Assert-Contains {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )
    Assert-FileExists -Path $Path -Description $Description
    $match = Select-String -LiteralPath $Path -SimpleMatch -Pattern $Pattern -ErrorAction SilentlyContinue
    if ($null -eq $match) {
        throw "Missing $Description marker in $Path`: $Pattern"
    }
    Write-Host "OK $Description marker: $Pattern"
}

Assert-FileExists -Path "docs\src\operations\v0.10.0-plan.md" -Description "v0.10 plan"
Assert-FileExists -Path "docs\src\operations\v0.10.0-readiness.md" -Description "v0.10 readiness"
Assert-FileExists -Path "data\conformance_cases\ideal_loads_thermostat_001\case.toml" -Description "v0.10 IdealLoads thermostat case"
Assert-FileExists -Path "data\conformance_cases\ideal_loads_thermostat_001\ideal_loads_thermostat.idf" -Description "v0.10 IdealLoads thermostat IDF"
Assert-FileExists -Path "scripts\smoke\ideal-loads-thermostat-smoke.ps1" -Description "v0.10 typed-graph gate"

Assert-Contains -Path "data\conformance_cases\ideal_loads_thermostat_001\case.toml" -Pattern 'comparison_class = "smoke"' -Description "v0.10 smoke case class"
Assert-Contains -Path "data\conformance_cases\ideal_loads_thermostat_001\case.toml" -Pattern "conformance_claim = false" -Description "v0.10 no conformance claim"
Assert-Contains -Path "data\conformance_cases\ideal_loads_thermostat_001\case.toml" -Pattern 'class = "hvac-state"' -Description "v0.10 HVAC output class"
Assert-Contains -Path "data\conformance_cases\ideal_loads_thermostat_001\case.toml" -Pattern "blocking = true" -Description "v0.10 blocking smoke gate"

Assert-Contains -Path "crates\ep_model\src\lib.rs" -Pattern "pub struct IdealLoadsAirSystem" -Description "v0.10 IdealLoads typed model"
Assert-Contains -Path "crates\ep_model\src\lib.rs" -Pattern "pub struct ZoneThermostat" -Description "v0.10 thermostat typed model"
Assert-Contains -Path "crates\ep_model\src\lib.rs" -Pattern "pub struct ZoneEquipmentConnection" -Description "v0.10 equipment connection typed model"
Assert-Contains -Path "crates\ep_model\src\lib.rs" -Pattern "pub struct NodeList" -Description "v0.10 NodeList typed model"
Assert-Contains -Path "crates\ep_compiler\src\lib.rs" -Pattern '"ZoneHVAC:IdealLoadsAirSystem"' -Description "v0.10 IdealLoads compiler coverage"
Assert-Contains -Path "crates\ep_compiler\src\lib.rs" -Pattern '"NodeList"' -Description "v0.10 NodeList compiler coverage"
Assert-Contains -Path "crates\ep_runtime\src\lib.rs" -Pattern "EvaluateZoneThermostat" -Description "v0.10 thermostat execution step"
Assert-Contains -Path "crates\ep_runtime\src\lib.rs" -Pattern "EvaluateIdealLoadsAirSystem" -Description "v0.10 IdealLoads execution step"

Assert-Contains -Path "docs\src\operations\v0.10.0-plan.md" -Pattern "ideal_loads_thermostat_001" -Description "v0.10 plan case"
Assert-Contains -Path "docs\src\operations\v0.10.0-plan.md" -Pattern "not an IdealLoads load-conformance claim" -Description "v0.10 plan claim boundary"
Assert-Contains -Path "docs\src\operations\v0.10.0-readiness.md" -Pattern "typed-graph-ready" -Description "v0.10 readiness status"
Assert-Contains -Path "docs\src\operations\v0.10.0-readiness.md" -Pattern "not an IdealLoads load-conformance claim" -Description "v0.10 readiness claim boundary"
Assert-Contains -Path "docs\src\porting-map\hvac.md" -Pattern "IdealLoads typed graph foundation" -Description "v0.10 HVAC map"
Assert-Contains -Path "docs\src\conformance\output-variable-matrix.md" -Pattern "Zone Ideal Loads Zone Total Heating Rate" -Description "v0.10 output matrix"

Write-Host "milestone: v0.10.0"
Write-Host "scope: thermostat, zone equipment, and IdealLoads typed graph"
Write-Host "claim: baseline-only smoke evidence for ideal_loads_thermostat_001; no IdealLoads load conformance"

Invoke-DevCommand -Command "source-smoke"
Invoke-DevCommand -Command "ideal-loads-thermostat-smoke"
Invoke-DevCommand -Command "test"
Invoke-DevCommand -Command "docs-check"
Invoke-DevCommand -Command "strict-no-false-conformance"

Write-Host "result: pass"
Write-Host "v0.10.0 IdealLoads thermostat typed-graph verification passed."
