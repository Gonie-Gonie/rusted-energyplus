[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")

$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot

function Read-RepoText {
    param([Parameter(Mandatory = $true)][string]$Path)
    return Get-Content -Encoding UTF8 -Raw -LiteralPath $Path
}

function Assert-FileExists {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "$Description missing: $Path"
    }
}

function Assert-Contains {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $text = Read-RepoText -Path $Path
    if ($text -notmatch $Pattern) {
        throw "$Description missing in $Path"
    }
}

function Assert-NotContains {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $text = Read-RepoText -Path $Path
    if ($text -match $Pattern) {
        throw "$Description unexpectedly present in $Path"
    }
}

function Assert-LineLimit {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][int]$Limit,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $lineCount = (Get-Content -Encoding UTF8 -LiteralPath $Path | Measure-Object -Line).Lines
    if ($lineCount -gt $Limit) {
        throw "$Description exceeds $Limit LOC: $Path has $lineCount LOC"
    }
}

$calcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$noOaCalc = "crates\ep_runtime\src\ideal_loads\calc\no_oa.rs"
$noOaTests = "crates\ep_runtime\src\ideal_loads\calc\no_oa_tests.rs"
$idealLoadsMod = "crates\ep_runtime\src\ideal_loads\mod.rs"
$idealLoadsMeters = "crates\ep_runtime\src\ideal_loads\meters.rs"
$idealLoadsReport = "crates\ep_runtime\src\ideal_loads\report.rs"
$idealLoadsReportSemantics = "crates\ep_runtime\src\ideal_loads\report\semantics.rs"
$outdoorAir = "crates\ep_runtime\src\ideal_loads\outdoor_air.rs"
$outdoorAirTests = "crates\ep_runtime\src\ideal_loads\outdoor_air_tests.rs"
$outdoorAirDesignFlow = "crates\ep_runtime\src\ideal_loads\outdoor_air\design_flow.rs"
$outdoorAirDcv = "crates\ep_runtime\src\ideal_loads\outdoor_air\dcv.rs"
$outdoorAirEconomizer = "crates\ep_runtime\src\ideal_loads\outdoor_air\economizer.rs"
$outdoorAirMixedAir = "crates\ep_runtime\src\ideal_loads\outdoor_air\mixed_air.rs"
$outdoorAirPsychrometrics = "crates\ep_runtime\src\ideal_loads\outdoor_air\psychrometrics.rs"
$outdoorAirSupply = "crates\ep_runtime\src\ideal_loads\outdoor_air\supply.rs"
$dispatch = "crates\ep_runtime\src\ideal_loads\dispatch.rs"
$idealLoadsCli = "crates\ep_cli\src\ideal_loads.rs"
$runtime = "crates\ep_runtime\src\runtime.rs"
$lib = "crates\ep_runtime\src\lib.rs"
$output = "crates\ep_runtime\src\output.rs"
$node = "crates\ep_runtime\src\node\mod.rs"
$nodeProjection = "crates\ep_runtime\src\node\projection.rs"
$nodeState = "crates\ep_runtime\src\node\state.rs"
$plant = "crates\ep_runtime\src\plant\mod.rs"
$plantState = "crates\ep_runtime\src\plant\state.rs"
$zoneEquipment = "crates\ep_runtime\src\zone_equipment\mod.rs"
$zoneEquipmentDemand = "crates\ep_runtime\src\zone_equipment\demand.rs"
$zoneEquipmentDispatch = "crates\ep_runtime\src\zone_equipment\dispatch.rs"
$zoneEquipmentTests = "crates\ep_runtime\src\zone_equipment\tests.rs"

Assert-FileExists -Path $calcRoot -Description "IdealLoads calc module root"
Assert-FileExists -Path $noOaCalc -Description "IdealLoads no-OA calc module"
Assert-FileExists -Path $noOaTests -Description "IdealLoads no-OA calc tests"
Assert-FileExists -Path $idealLoadsMod -Description "IdealLoads module root"
Assert-FileExists -Path $idealLoadsMeters -Description "IdealLoads meter binding module"
Assert-FileExists -Path $idealLoadsReport -Description "IdealLoads report module"
Assert-FileExists -Path $idealLoadsReportSemantics -Description "IdealLoads report semantics module"
Assert-FileExists -Path $outdoorAir -Description "IdealLoads outdoor-air module"
Assert-FileExists -Path $outdoorAirTests -Description "IdealLoads outdoor-air tests module"
Assert-FileExists -Path $outdoorAirDesignFlow -Description "IdealLoads outdoor-air design-flow module"
Assert-FileExists -Path $outdoorAirDcv -Description "IdealLoads outdoor-air DCV module"
Assert-FileExists -Path $outdoorAirEconomizer -Description "IdealLoads outdoor-air economizer module"
Assert-FileExists -Path $outdoorAirMixedAir -Description "IdealLoads outdoor-air mixed-air module"
Assert-FileExists -Path $outdoorAirPsychrometrics -Description "IdealLoads outdoor-air psychrometrics module"
Assert-FileExists -Path $outdoorAirSupply -Description "IdealLoads outdoor-air supply module"
Assert-FileExists -Path $dispatch -Description "IdealLoads source-order dispatch module"
Assert-FileExists -Path $idealLoadsCli -Description "IdealLoads CLI report generator"
Assert-FileExists -Path $runtime -Description "Runtime root"
Assert-FileExists -Path $lib -Description "Runtime crate facade"
Assert-FileExists -Path $output -Description "Runtime output registry"
Assert-FileExists -Path $node -Description "Node compatibility facade"
Assert-FileExists -Path $nodeProjection -Description "Node projection module"
Assert-FileExists -Path $nodeState -Description "Node state module"
Assert-FileExists -Path $plant -Description "Plant compatibility facade"
Assert-FileExists -Path $plantState -Description "Plant state module"
Assert-FileExists -Path $zoneEquipment -Description "Zone equipment compatibility facade"
Assert-FileExists -Path $zoneEquipmentDemand -Description "Zone equipment demand module"
Assert-FileExists -Path $zoneEquipmentDispatch -Description "Zone equipment dispatch module"
Assert-FileExists -Path $zoneEquipmentTests -Description "Zone equipment tests module"

Assert-LineLimit -Path $calcRoot -Limit 1200 -Description "IdealLoads calc module root"
Assert-LineLimit -Path $noOaCalc -Limit 1200 -Description "IdealLoads no-OA calc module"
Assert-LineLimit -Path $noOaTests -Limit 1200 -Description "IdealLoads no-OA calc tests"
Assert-LineLimit -Path $idealLoadsMeters -Limit 300 -Description "IdealLoads meter binding module"
Assert-LineLimit -Path $idealLoadsReportSemantics -Limit 300 -Description "IdealLoads report semantics module"
Assert-LineLimit -Path $outdoorAir -Limit 1200 -Description "IdealLoads outdoor-air module"
Assert-LineLimit -Path $outdoorAirTests -Limit 500 -Description "IdealLoads outdoor-air tests module"
Assert-LineLimit -Path $outdoorAirDesignFlow -Limit 300 -Description "IdealLoads outdoor-air design-flow module"
Assert-LineLimit -Path $outdoorAirDcv -Limit 400 -Description "IdealLoads outdoor-air DCV module"
Assert-LineLimit -Path $outdoorAirEconomizer -Limit 400 -Description "IdealLoads outdoor-air economizer module"
Assert-LineLimit -Path $outdoorAirMixedAir -Limit 400 -Description "IdealLoads outdoor-air mixed-air module"
Assert-LineLimit -Path $outdoorAirPsychrometrics -Limit 400 -Description "IdealLoads outdoor-air psychrometrics module"
Assert-LineLimit -Path $outdoorAirSupply -Limit 300 -Description "IdealLoads outdoor-air supply module"
Assert-LineLimit -Path $nodeProjection -Limit 600 -Description "Node projection module"
Assert-LineLimit -Path $nodeState -Limit 400 -Description "Node state module"
Assert-LineLimit -Path $plantState -Limit 900 -Description "Plant state module"
Assert-LineLimit -Path $zoneEquipment -Limit 120 -Description "Zone equipment compatibility facade"
Assert-LineLimit -Path $zoneEquipmentDemand -Limit 200 -Description "Zone equipment demand module"
Assert-LineLimit -Path $zoneEquipmentDispatch -Limit 400 -Description "Zone equipment dispatch module"
Assert-LineLimit -Path $zoneEquipmentTests -Limit 400 -Description "Zone equipment tests module"

Assert-Contains -Path $calcRoot -Pattern 'mod no_oa;' -Description "no-OA calc submodule declaration"
Assert-Contains -Path $calcRoot -Pattern 'pub use no_oa::\*;' -Description "no-OA calc public re-export"
Assert-Contains -Path $calcRoot -Pattern 'mod no_oa_tests;' -Description "no-OA calc test module declaration"
Assert-NotContains -Path $calcRoot -Pattern 'calc_no_oa_no_limit_sensible_compat\s*\(' -Description "branch formula in calc module root"
Assert-NotContains -Path $calcRoot -Pattern 'fn heating_result_with_limits\s*\(' -Description "heating branch helper in calc module root"
Assert-NotContains -Path $calcRoot -Pattern 'fn cooling_result_with_limits\s*\(' -Description "cooling branch helper in calc module root"

Assert-Contains -Path $noOaCalc -Pattern 'pub fn calc_no_oa_no_limit_sensible_compat\s*\(' -Description "no-OA/no-limit sensible calc"
Assert-Contains -Path $noOaCalc -Pattern 'pub fn calc_no_oa_sensible_with_limits_compat\s*\(' -Description "finite-limit sensible calc"
Assert-Contains -Path $noOaCalc -Pattern 'fn heating_result_with_limits\s*\(' -Description "heating branch helper"
Assert-Contains -Path $noOaCalc -Pattern 'fn cooling_result_with_limits\s*\(' -Description "cooling branch helper"
Assert-Contains -Path $noOaCalc -Pattern 'fn humidistat_dehumidification_mass_flow_rate_kg_per_s\s*\(' -Description "dehumidification diagnostic branch helper"
Assert-Contains -Path $noOaCalc -Pattern 'fn humidistat_humidification_mass_flow_rate_kg_per_s\s*\(' -Description "humidification diagnostic branch helper"
Assert-NotContains -Path $noOaCalc -Pattern '#\[test\]' -Description "unit tests in no-OA implementation module"
Assert-Contains -Path $noOaTests -Pattern '#\[test\]' -Description "unit tests in no-OA test module"

Assert-Contains -Path $idealLoadsMod -Pattern 'mod meters;' -Description "IdealLoads meter submodule declaration"
Assert-Contains -Path $idealLoadsMod -Pattern 'pub use meters::\*;' -Description "IdealLoads meter public re-export"
Assert-Contains -Path $idealLoadsMeters -Pattern 'pub struct IdealLoadsFacilityMeterBinding' -Description "IdealLoads facility-meter binding type"
Assert-Contains -Path $idealLoadsMeters -Pattern 'pub const IDEAL_LOADS_METER_AGGREGATION_SOURCE' -Description "IdealLoads meter aggregation source label"
Assert-Contains -Path $idealLoadsMeters -Pattern 'pub fn ideal_loads_facility_meter_binding\s*\(' -Description "IdealLoads facility-meter binding helper"
Assert-Contains -Path $output -Pattern 'ideal_loads_facility_meter_binding' -Description "Runtime meter registry uses IdealLoads meter binding helper"
Assert-NotContains -Path $output -Pattern 'pub struct IdealLoadsFacilityMeterBinding' -Description "IdealLoads facility-meter binding type in generic output registry"
Assert-NotContains -Path $output -Pattern 'pub fn ideal_loads_facility_meter_binding\s*\(' -Description "IdealLoads facility-meter binding helper in generic output registry"

Assert-Contains -Path $idealLoadsReport -Pattern 'mod semantics;' -Description "IdealLoads report semantics submodule declaration"
Assert-Contains -Path $idealLoadsReport -Pattern 'pub use semantics::\*;' -Description "IdealLoads report semantics public re-export"
Assert-Contains -Path $idealLoadsReportSemantics -Pattern 'pub const IDEAL_LOADS_RATE_OUTPUT_SOURCE' -Description "IdealLoads ReportPurchasedAir rate source metadata"
Assert-Contains -Path $idealLoadsReportSemantics -Pattern 'pub const IDEAL_LOADS_ENERGY_OUTPUT_TIMESTEP_SOURCE' -Description "IdealLoads report-energy timestep metadata"
Assert-Contains -Path $idealLoadsReportSemantics -Pattern 'pub const IDEAL_LOADS_FUEL_ENERGY_OUTPUT_LEVEL_POLICY' -Description "IdealLoads fuel-energy level policy metadata"
Assert-NotContains -Path $idealLoadsReport -Pattern 'pub const IDEAL_LOADS_RATE_OUTPUT_SOURCE' -Description "ReportPurchasedAir semantics in report module root"

Assert-Contains -Path $outdoorAir -Pattern 'mod dcv;' -Description "outdoor-air DCV submodule declaration"
Assert-Contains -Path $outdoorAir -Pattern 'mod design_flow;' -Description "outdoor-air design-flow submodule declaration"
Assert-Contains -Path $outdoorAir -Pattern 'mod economizer;' -Description "outdoor-air economizer submodule declaration"
Assert-Contains -Path $outdoorAir -Pattern 'mod mixed_air;' -Description "outdoor-air mixed-air submodule declaration"
Assert-Contains -Path $outdoorAir -Pattern 'mod psychrometrics;' -Description "outdoor-air psychrometrics submodule declaration"
Assert-Contains -Path $outdoorAir -Pattern 'mod supply;' -Description "outdoor-air supply submodule declaration"
Assert-Contains -Path $outdoorAir -Pattern '#\[path = "outdoor_air_tests\.rs"\]' -Description "outdoor-air test module path declaration"
Assert-Contains -Path $outdoorAir -Pattern 'pub use dcv::\*;' -Description "outdoor-air DCV public re-export"
Assert-Contains -Path $outdoorAir -Pattern 'pub use design_flow::\*;' -Description "outdoor-air design-flow public re-export"
Assert-Contains -Path $outdoorAirTests -Pattern '#\[test\]' -Description "outdoor-air root unit tests"
Assert-Contains -Path $outdoorAirDesignFlow -Pattern 'pub fn design_outdoor_air_volume_flow_components_m3_per_s\s*\(' -Description "outdoor-air design-flow component helper"
Assert-Contains -Path $outdoorAirDesignFlow -Pattern 'pub fn calc_scheduled_outdoor_air_mass_flow_rate_kg_per_s\s*\(' -Description "outdoor-air scheduled mass-flow helper"
Assert-Contains -Path $outdoorAirDesignFlow -Pattern 'fn nonnegative_product\s*\(' -Description "outdoor-air design-flow scalar helper"
Assert-Contains -Path $outdoorAirDcv -Pattern 'pub fn calc_occupancy_schedule_dcv_outdoor_air_mass_flow_rate_kg_per_s\s*\(' -Description "OccupancySchedule DCV helper"
Assert-Contains -Path $outdoorAirEconomizer -Pattern 'fn calc_economizer_adjusted_outdoor_air_mass_flow_rate_kg_per_s\s*\(' -Description "economizer OA flow helper"
Assert-Contains -Path $outdoorAirMixedAir -Pattern 'fn mixed_air_state\s*\(' -Description "mixed-air state helper"
Assert-Contains -Path $outdoorAirMixedAir -Pattern 'fn heat_recovery_allows_outdoor_air_tempering\s*\(' -Description "heat recovery activation helper"
Assert-Contains -Path $outdoorAirPsychrometrics -Pattern 'fn heat_recovery_saturation_adjusted_state\s*\(' -Description "heat recovery saturation helper"
Assert-Contains -Path $outdoorAirSupply -Pattern 'fn outdoor_air_supply_mass_flow_rate_kg_per_s\s*\(' -Description "outdoor-air supply mass-flow helper"
Assert-Contains -Path $outdoorAirSupply -Pattern 'fn supply_air_state\s*\(' -Description "outdoor-air supply state helper"
Assert-NotContains -Path $outdoorAir -Pattern 'pub fn design_outdoor_air_volume_flow_components_m3_per_s\s*\(' -Description "outdoor-air design-flow component helper in outdoor-air root"
Assert-NotContains -Path $outdoorAir -Pattern 'pub fn calc_scheduled_outdoor_air_mass_flow_rate_kg_per_s\s*\(' -Description "outdoor-air scheduled mass-flow helper in outdoor-air root"
Assert-NotContains -Path $outdoorAir -Pattern 'fn nonnegative_product\s*\(' -Description "outdoor-air design-flow scalar helper in outdoor-air root"
Assert-NotContains -Path $outdoorAir -Pattern 'fn calc_occupancy_schedule_dcv_outdoor_air_mass_flow_rate_kg_per_s\s*\(' -Description "OccupancySchedule DCV helper in outdoor-air root"
Assert-NotContains -Path $outdoorAir -Pattern 'fn calc_economizer_adjusted_outdoor_air_mass_flow_rate_kg_per_s\s*\(' -Description "economizer OA flow helper in outdoor-air root"
Assert-NotContains -Path $outdoorAir -Pattern 'fn mixed_air_state\s*\(' -Description "mixed-air state helper in outdoor-air root"
Assert-NotContains -Path $outdoorAir -Pattern 'fn heat_recovery_allows_outdoor_air_tempering\s*\(' -Description "heat recovery activation helper in outdoor-air root"
Assert-NotContains -Path $outdoorAir -Pattern 'fn heat_recovery_saturation_adjusted_state\s*\(' -Description "heat recovery saturation helper in outdoor-air root"
Assert-NotContains -Path $outdoorAir -Pattern 'fn outdoor_air_supply_mass_flow_rate_kg_per_s\s*\(' -Description "outdoor-air supply mass-flow helper in outdoor-air root"
Assert-NotContains -Path $outdoorAir -Pattern 'fn supply_air_state\s*\(' -Description "outdoor-air supply state helper in outdoor-air root"
Assert-NotContains -Path $outdoorAir -Pattern '#\[test\]' -Description "unit test body in outdoor-air root"
Assert-NotContains -Path $outdoorAir -Pattern 'energyplus_psychrometric_humidity_ratio_from_rh' -Description "psychrometric humidity-ratio helper import in outdoor-air root"

Assert-Contains -Path $dispatch -Pattern 'pub fn sim_purchased_air_compat\s*\(' -Description "SimPurchasedAir source-order wrapper"
Assert-Contains -Path $dispatch -Pattern 'purchased_air_source_order_stages\s*\(' -Description "PurchasedAir source-order stage summary"
Assert-Contains -Path $dispatch -Pattern 'calc_no_oa_sensible_with_limits_and_recirculation_compat\s*\(' -Description "finite-limit branch dispatch"
Assert-Contains -Path $dispatch -Pattern 'calc_no_oa_no_limit_sensible_with_recirculation_context_compat\s*\(' -Description "no-limit branch dispatch"
Assert-NotContains -Path $dispatch -Pattern 'calc_outdoor_air_sensible_report_rates_compat\s*\(' -Description "outdoor-air diagnostic calculation in source-order conformance wrapper"

$runtimeFormulaPatterns = @(
    'fn calc_no_oa_',
    'fn calc_outdoor_air_',
    'fn heating_result_with_limits\s*\(',
    'fn cooling_result_with_limits\s*\(',
    'fn calc_scheduled_outdoor_air_mass_flow_rate_kg_per_s\s*\('
)
foreach ($pattern in $runtimeFormulaPatterns) {
    Assert-NotContains -Path $runtime -Pattern $pattern -Description "IdealLoads branch formula in runtime root"
}

Assert-Contains -Path $node -Pattern 'mod state;' -Description "node state submodule declaration"
Assert-Contains -Path $node -Pattern 'pub use state::\*;' -Description "node state public re-export"
Assert-Contains -Path $node -Pattern 'mod projection;' -Description "node projection submodule declaration"
Assert-Contains -Path $node -Pattern 'pub use projection::\*;' -Description "node projection public re-export"
Assert-Contains -Path $node -Pattern 'pub struct IdealLoadsSupplyNodeUpdate' -Description "IdealLoads node update transfer struct"
Assert-Contains -Path $node -Pattern 'IDEAL_LOADS_NODE_OUTPUT_STORE_TYPE' -Description "IdealLoads node output store metadata"
Assert-Contains -Path $nodeProjection -Pattern 'pub struct NodeStateProjectionOptions' -Description "node projection options"
Assert-Contains -Path $nodeProjection -Pattern 'pub struct NodeStateProjectionEvidencePolicy' -Description "node projection evidence policy"
Assert-Contains -Path $nodeProjection -Pattern 'pub fn simulate_ideal_loads_node_state_projection\s*\(' -Description "node projection runtime function"
Assert-Contains -Path $nodeState -Pattern 'pub struct NodeStateStore' -Description "NodeStateStore implementation"
Assert-Contains -Path $nodeState -Pattern 'pub struct AirNodeState' -Description "AirNodeState implementation"
Assert-Contains -Path $nodeState -Pattern 'pub enum NodeStateRole' -Description "NodeStateRole implementation"
Assert-Contains -Path $nodeState -Pattern 'pub const NODE_STATE_SOURCE_MAP_PATH' -Description "node-state source-map metadata"
Assert-Contains -Path $nodeState -Pattern 'pub fn node_temperature_setpoint_from_energyplus\s*\(' -Description "EnergyPlus node setpoint sentinel adapter"
Assert-NotContains -Path $runtime -Pattern 'pub struct NodeStateProjectionOptions' -Description "node projection options in runtime root"
Assert-NotContains -Path $runtime -Pattern 'pub struct NodeStateProjectionEvidencePolicy' -Description "node projection evidence policy in runtime root"
Assert-NotContains -Path $runtime -Pattern 'pub fn simulate_ideal_loads_node_state_projection\s*\(' -Description "node projection runtime function in runtime root"
Assert-NotContains -Path $runtime -Pattern 'pub struct NodeStateStore' -Description "NodeStateStore implementation in runtime root"
Assert-NotContains -Path $runtime -Pattern 'pub struct AirNodeState' -Description "AirNodeState implementation in runtime root"
Assert-NotContains -Path $runtime -Pattern 'pub enum NodeStateRole' -Description "NodeStateRole implementation in runtime root"
Assert-NotContains -Path $runtime -Pattern 'pub const NODE_STATE_SOURCE_MAP_PATH' -Description "node-state source-map metadata in runtime root"

Assert-Contains -Path $lib -Pattern 'pub mod plant;' -Description "plant module public declaration"
Assert-Contains -Path $lib -Pattern 'pub use plant::\*;' -Description "plant module public re-export"
Assert-Contains -Path $plant -Pattern 'mod state;' -Description "plant state submodule declaration"
Assert-Contains -Path $plant -Pattern 'pub use state::\*;' -Description "plant state public re-export"
Assert-Contains -Path $plantState -Pattern 'pub struct PlantStateStore' -Description "PlantStateStore implementation"
Assert-Contains -Path $plantState -Pattern 'pub struct PlantStateProjectionEvidencePolicy' -Description "plant projection evidence policy"
Assert-Contains -Path $plantState -Pattern 'pub const PLANT_STATE_SOURCE_MAP_PATH' -Description "plant-state source-map metadata"
Assert-Contains -Path $plantState -Pattern 'pub fn simulate_plant_state_projection\s*\(' -Description "plant projection runtime function"
Assert-NotContains -Path $runtime -Pattern 'pub struct PlantStateStore' -Description "PlantStateStore implementation in runtime root"
Assert-NotContains -Path $runtime -Pattern 'pub struct PlantStateProjectionEvidencePolicy' -Description "plant projection evidence policy in runtime root"
Assert-NotContains -Path $runtime -Pattern 'pub const PLANT_STATE_SOURCE_MAP_PATH' -Description "plant-state source-map metadata in runtime root"
Assert-NotContains -Path $runtime -Pattern 'pub fn simulate_plant_state_projection\s*\(' -Description "plant projection runtime function in runtime root"

Assert-Contains -Path $zoneEquipment -Pattern 'mod demand;' -Description "zone equipment demand submodule declaration"
Assert-Contains -Path $zoneEquipment -Pattern 'pub use demand::\*;' -Description "zone equipment demand public re-export"
Assert-Contains -Path $zoneEquipment -Pattern 'mod dispatch;' -Description "zone equipment dispatch submodule declaration"
Assert-Contains -Path $zoneEquipment -Pattern 'pub use dispatch::\*;' -Description "zone equipment dispatch public re-export"
Assert-Contains -Path $zoneEquipment -Pattern '#\[path = "tests\.rs"\]' -Description "zone equipment test module path declaration"
Assert-Contains -Path $zoneEquipmentDemand -Pattern 'pub struct ZoneSysEnergyDemand' -Description "ZoneSysEnergyDemand source-order demand struct"
Assert-Contains -Path $zoneEquipmentDispatch -Pattern 'pub const fn ideal_loads_zone_equipment_stages\s*\(' -Description "Zone equipment compatibility stages"
Assert-Contains -Path $zoneEquipmentDispatch -Pattern 'pub fn validate_ideal_loads_zone_equipment_dispatch\s*\(' -Description "IdealLoads zone equipment dispatch validation"
Assert-Contains -Path $zoneEquipmentDispatch -Pattern 'SupplyNodeNotInZoneInletList' -Description "IdealLoads supply node inlet-list validation"
Assert-Contains -Path $zoneEquipmentDispatch -Pattern 'SequenceAmbiguity' -Description "IdealLoads sequence ambiguity validation"
Assert-Contains -Path $zoneEquipmentDispatch -Pattern 'MultipleZoneEquipmentDiagnosticOnly' -Description "IdealLoads multiple-equipment diagnostic boundary"
Assert-NotContains -Path $zoneEquipment -Pattern 'pub struct ZoneSysEnergyDemand' -Description "ZoneSysEnergyDemand implementation in zone equipment facade"
Assert-NotContains -Path $zoneEquipment -Pattern 'pub fn validate_ideal_loads_zone_equipment_dispatch\s*\(' -Description "dispatch validation implementation in zone equipment facade"
Assert-NotContains -Path $zoneEquipment -Pattern '#\[test\]' -Description "unit tests in zone equipment facade"
Assert-Contains -Path $runtime -Pattern 'ExecutionStep::ManageZoneEquipment' -Description "ZoneEquipmentManager stage in execution plan"
Assert-Contains -Path $runtime -Pattern 'ExecutionStep::SimZoneEquipment' -Description "SimZoneEquipment stage in execution plan"
Assert-Contains -Path $runtime -Pattern 'ExecutionStep::EvaluateIdealLoadsAirSystem' -Description "IdealLoads evaluation stage in execution plan"
Assert-Contains -Path $idealLoadsCli -Pattern 'zone_equipment_dispatch_path' -Description "IdealLoads report zone-equipment dispatch path metadata"
Assert-Contains -Path $idealLoadsCli -Pattern 'zone_equipment_dispatch_validation' -Description "IdealLoads report zone-equipment dispatch validation metadata"
Assert-Contains -Path $idealLoadsCli -Pattern 'zone_equipment_conformance_candidate' -Description "IdealLoads report zone-equipment conformance-candidate metadata"

Write-Host "IdealLoads structure audit complete."
