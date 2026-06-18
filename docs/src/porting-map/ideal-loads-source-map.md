---
status: active
claim_level: limited-ideal-loads-no-oa-sensible-conformance
owner: runtime
last_reviewed: 2026-06-15
---

# IdealLoads Source Map

Reference version: EnergyPlus 26.1.0

Purpose: map the first limited `ZoneHVAC:IdealLoadsAirSystem`
numerical-conformance claim to EnergyPlus source functions while keeping
availability, humidity control, outdoor-air, sizing, broad fuel/meter
conformance, and broad HVAC compatibility outside the claim.

## Required EnergyPlus Source Anchors

The IdealLoads conformance boundary is guarded against these EnergyPlus
source anchors:

- `src/EnergyPlus/PurchasedAirManager.cc`: source-order
  `SimPurchasedAir`, `InitPurchasedAir`, `CalcPurchAirLoads`,
  `UpdatePurchasedAir`, and `ReportPurchasedAir` behavior.
- `src/EnergyPlus/PurchasedAirManager.hh`: IdealLoads data shape and
  PurchasedAir manager declarations.
- `src/EnergyPlus/ZoneEquipmentManager.cc`: zone equipment dispatch path into
  `PurchasedAirManager::SimPurchasedAir`.
- `src/EnergyPlus/DataZoneEnergyDemands.hh`: `ZoneSysEnergyDemand` fields for
  active heating/cooling demand.
- `src/EnergyPlus/DataLoopNode.hh`: system node state fields used by
  `UpdatePurchasedAir` and node output reporting.
- `src/EnergyPlus/ScheduleManager.hh`: schedule lookup semantics for
  availability, outdoor-air, and fuel-efficiency schedule fixtures.
- `src/EnergyPlus/Psychrometrics.hh`: `PsyCpAirFnW`, enthalpy, humidity-ratio,
  and standard-density psychrometric helpers.
- `src/EnergyPlus/OutputProcessor.cc`: `SetupOutputVariable`, detailed
  timestep reporting, and OutputProcessor sum/meter aggregation semantics.
- `src/EnergyPlus/HVACSizingSimulationManager.cc`: deferred autosizing
  extension anchor only; autosized IdealLoads flow/capacity conformance remains
  outside the current claim.

## Initial Claim Boundary

The first promoted case is
`ideal_loads_no_oa_sensible_conformance_001`. It now uses
`comparison_class = "conformance"` and `conformance_claim = true` for declared
conformance-level variables only. The compare lane produces timestamp-aligned
Rust result-store artifacts, requires zero tolerance failures, and remains a
blocking gate.

The initial supported boundary is intentionally narrow:

- one zone
- one `ZoneHVAC:IdealLoadsAirSystem`
- no outdoor air requirement
- no economizer
- no or diagnostic-only Sensible/Enthalpy heat recovery
- no humidistat
- no demand-controlled ventilation
- no finite flow or capacity limit
- no autosizing branch
- no return plenum
- no air loop or plant loop
- constant heating and cooling setpoints through
  `ThermostatSetpoint:DualSetpoint`

All excluded features must stay diagnostic-only or unsupported until they have
their own source map, Rust state, oracle evidence, and blocking gate.

## EnergyPlus Function Map

| EnergyPlus function | Source file | Rust target |
|---|---|---|
| `PurchasedAirManager::SimPurchasedAir` | `src/EnergyPlus/PurchasedAirManager.cc` | `ep_runtime::ideal_loads::sim_purchased_air_compat`; `ep_runtime::ideal_loads::sim_purchased_air_outdoor_air_compat` |
| `PurchasedAirManager::GetPurchasedAir` | `src/EnergyPlus/PurchasedAirManager.cc` | `ep_compiler::objects::ideal_loads`; `ep_model::objects::ideal_loads` |
| `PurchasedAirManager::InitPurchasedAir` | `src/EnergyPlus/PurchasedAirManager.cc` | `crates/ep_runtime/src/ideal_loads/init.rs::IdealLoadsInitFlags` |
| `PurchasedAirManager::SizePurchasedAir` | `src/EnergyPlus/PurchasedAirManager.cc` | `ep_runtime::ideal_loads::size_ideal_loads_air_system_compat` |
| `PurchasedAirManager::CalcPurchAirLoads` | `src/EnergyPlus/PurchasedAirManager.cc` | `crates/ep_runtime/src/ideal_loads/calc/no_oa.rs::calc_no_oa_no_limit_sensible_compat` |
| `PurchasedAirManager::CalcPurchAirMinOAMassFlow` | `src/EnergyPlus/PurchasedAirManager.cc` | `crates/ep_runtime/src/ideal_loads/outdoor_air/dcv.rs::calc_occupancy_schedule_dcv_outdoor_air_mass_flow_rate_kg_per_s` |
| `PurchasedAirManager::UpdatePurchasedAir` | `src/EnergyPlus/PurchasedAirManager.cc` | `crates/ep_runtime/src/ideal_loads/update.rs::supply_node_update_from_result` |
| `PurchasedAirManager::ReportPurchasedAir` | `src/EnergyPlus/PurchasedAirManager.cc` | `crates/ep_runtime/src/ideal_loads/report.rs::IdealLoadsReportSnapshot` |
| `DataSizing::calcDesignSpecificationOutdoorAir` | `src/EnergyPlus/DataSizing.cc` | `crates/ep_runtime/src/ideal_loads/outdoor_air/dcv.rs::occupancy_schedule_dcv_outdoor_air_volume_flow_components_m3_per_s` |
| `ZoneEquipmentManager::ManageZoneEquipment` | `src/EnergyPlus/ZoneEquipmentManager.cc` | `crates/ep_runtime/src/zone_equipment/dispatch.rs::ideal_loads_zone_equipment_stages` |
| `ZoneEquipmentManager::SimZoneEquipment` | `src/EnergyPlus/ZoneEquipmentManager.cc` | `crates/ep_runtime/src/zone_equipment/dispatch.rs::ZoneEquipmentCompatibilityStage` |
| `ZoneTempPredictorCorrector` predicted load state | `src/EnergyPlus/ZoneTempPredictorCorrector.cc` | `crates/ep_runtime/src/zone_equipment/demand.rs::ZoneSysEnergyDemand` |

## Runtime Order

EnergyPlus calls the IdealLoads component through the zone equipment manager:

```text
ZoneEquipmentManager::ManageZoneEquipment
  -> InitZoneEquipment
  -> SimZoneEquipment
  -> PurchasedAirManager::SimPurchasedAir
  -> PurchasedAirManager::InitPurchasedAir
  -> PurchasedAirManager::CalcPurchAirLoads
  -> PurchasedAirManager::UpdatePurchasedAir
  -> PurchasedAirManager::ReportPurchasedAir
```

The Rust compatibility path preserves this ordering for the promoted
no-OA/no-limit sensible boundary and must be extended before any excluded
feature is promoted.

`GetPurchasedAir` name lookup is represented as compile-stage typed binding in
Rust. Conformance reports expose `ideal_loads_runtime_binding_source` as
`compile-stage typed IdealLoadsAirSystemId, ZoneId, and NodeId binding` and
`purchased_air_name_lookup_policy` as `PurchAirName string lookup is compile/report only; simulation loop uses prebound typed IDs`.
Current conformance reports expose `ideal_loads_invocation_path` as
`zone-equipment-validated source-order PurchasedAir wrapper`,
`direct_calc_helper_invocation` as `false`, and
`zone_equipment_dispatch_execution_boundary` as `validated typed ZoneEquipmentManager path; report generator invokes source-order PurchasedAir wrapper`.
Reports also expose `ideal_loads_feature_dispatch_policy` and
`ideal_loads_prebound_id_contract`: compile feature flags select the
branch-specific source-order compatibility function, unsupported active feature
combinations emit diagnostics instead of approximate fallback, and the
IdealLoads system, zone, node, and availability schedule references are typed
before the simulation loop.
Psychrometric evidence metadata is intentionally conservative:
`ideal_loads_psychrometric_evaluation_policy` says the compatibility reports use
source-order direct evaluation with no cross-timestep cache or reordering, and
`ideal_loads_psychrometric_cache_policy` requires any future cache to use the
exact temperature, humidity ratio, and pressure tuple while preserving
EnergyPlus evaluation order.
Output-handle evidence now resolves each manifest-declared IdealLoads output to
a stable `OutputHandle` before comparison rows are evaluated. Rate and node
series are exported through `ResultStore` with those handles, meter rows are
resolved through `RuntimeMeterRegistry` before aggregation, diagnostic rows are
emitted only when declared by the manifest, and duplicate output requests fail
during handle setup before artifact export.
Each IdealLoads case manifest sets `[trace].level`; reports echo that value in
`trace_level` and identify `trace_level_source` as `case manifest [trace].level`.
Trace level only selects the evidence payload: `ResultStore` values are computed
before report serialization, and trace/report overhead is accounted separately
from numerical conformance comparisons.

## No-OA Sensible Fast Path

The first implementation target may dispatch to a narrow helper only when all
of these compile-time facts are true:

- `has_outdoor_air = false`
- `outdoor_air_economizer_type = NoEconomizer`
- `heat_recovery_type = None`
- `heating_limit = NoLimit`
- `cooling_limit = NoLimit`
- no humidistat object is active for the zone
- no autosized flow or capacity limit participates in the calculation

`calc_no_oa_no_limit_sensible_compat` now implements the first isolated helper.
It still requires upstream, source-order zone demand. The conformance report
generator feeds it from EnergyPlus proof outputs instead of a simplified RC
load shortcut:

- `Zone System Predicted Sensible Load to Setpoint Heat Transfer Rate` is the
  active signed demand source. Positive values become
  `RemainingOutputReqToHeatSP`; negative values become
  `RemainingOutputReqToCoolSP`.
- `Zone System Predicted Sensible Load to Heating Setpoint Heat Transfer Rate`
  and `... Cooling Setpoint ...` stay as diagnostic proof rows because they
  are setpoint-distance outputs, not active branch selectors.
- The IdealLoads mass-flow formula uses the source-order pre-update zone air
  node state. Same-timestamp zone air node outputs are retained as diagnostic
  proof rows because they show the post-update node state.

The helper uses the EnergyPlus formula order for:

- zone remaining load to heat and cool setpoints
- `PsyCpAirFnW`
- heating and cooling supply temperature selection
- sensible supply mass flow
- final nonnegative supply mass flow
- supply node temperature, humidity ratio, enthalpy, and mass flow writes
- reported zone and supply-air IdealLoads rates

## Finite Flow/Capacity Evidence

Finite no-OA capacity, flow, and flow-and-capacity limits are promoted by:

- `ideal_loads_capacity_limit_conformance_001`
- `ideal_loads_flow_limit_conformance_001`
- `ideal_loads_flow_capacity_limit_conformance_001`

The original diagnostic lanes remain available for finite flow/capacity
regression evidence:

- `ideal_loads_capacity_limit_diagnostic_001`
- `ideal_loads_flow_limit_diagnostic_001`
- `ideal_loads_flow_capacity_limit_diagnostic_001`

The Rust helper `calc_no_oa_sensible_with_limits_compat` covers the numeric
flow and capacity limit reconstruction. The compare lane resolves the
EnergyPlus return/exhaust recirculation node and records
`ZONE ONE RETURN` `System Node Temperature` and `System Node Humidity Ratio`
as proof rows. The finite-limit reconstruction uses that same-call
recirculation state for the no-OA mixed-air and ReportPurchasedAir
calculations, matching the declared Detailed rate and supply-node rows in the
finite fixtures.

The promoted capacity-limit fixture has zero tolerance failures across 18
Detailed series and 188 samples. The promoted flow-limit fixture has zero
tolerance failures across 18 Detailed series and 128 samples. The promoted
flow-and-capacity-limit fixture has zero tolerance failures across 18 Detailed
series and 189 samples. All three promote the same 10 user-facing rows as the
no-OA/no-limit sensible claim: thermostat setpoints, zone total/sensible
heating/cooling rates, supply-air total heating/cooling rates, supply-node
temperature, and supply-node mass flow. Return-node temperature/humidity,
supply-node humidity, and predictor/corrector proof rows remain diagnostic.

## Humidity-Control Evidence

`ideal_loads_constant_shr_conformance_001` promotes a narrow no-OA
`ConstantSensibleHeatRatio` cooling lane for the declared thermostat,
cooling total/sensible/latent rate, and supply-node temperature/mass-flow/
humidity rows. `ideal_loads_constant_shr_diagnostic_001` remains available as
non-claim regression/proof evidence for the broader diagnostic output set. The
compare lane resolves `ZONE ONE RETURN` as the recirculation/mixed-air proof
node, preserves the source-order pre-update zone state for the sensible demand
calculation, and passes EPW barometric pressure into the saturation clamp used
by EnergyPlus psychrometric routines.

The Rust reconstruction uses EnergyPlus `PsyHFnTdbW`/`PsyWFnTdbH` enthalpy
constants for the latent split and records zero tolerance failures for the
promoted Constant SHR cooling rows plus return-node and zone-air-node proof
rows.

`ideal_loads_constant_supply_humidity_cooling_conformance_candidate_001` promotes
the matching no-OA `ConstantSupplyHumidityRatio` cooling lane for declared
thermostat, cooling total/sensible/latent rate, supply-air cooling report-rate,
and supply-node temperature/mass-flow/humidity rows. It uses the EnergyPlus
minimum cooling supply humidity ratio, allows the source's small latent-heating
diagnostic report rows when heating availability is on during cooling, and keeps
the same return-node mixed-air and EPW barometric-pressure saturation proof path.
The original `ideal_loads_constant_supply_humidity_diagnostic_001` remains
available as non-claim regression/proof evidence for the broader diagnostic
output set.

`ideal_loads_constant_supply_humidity_heating_conformance_candidate_001`
promotes the heating-side no-OA `ConstantSupplyHumidityRatio` lane for declared
thermostat, heating total/sensible/latent rate, supply-air heating report-rate,
and supply-node temperature/mass-flow/humidity rows. It uses the EnergyPlus
maximum heating supply humidity ratio in heating mode, keeps the same
return-node mixed-air and saturation proof path, and matches active latent
heating report rows with zero tolerance failures. The original
`ideal_loads_constant_supply_humidity_heating_diagnostic_001` remains available
as non-claim regression/proof evidence for the broader diagnostic output set.

`ideal_loads_humidistat_dehumidification_conformance_candidate_001` promotes
the no-OA Humidistat dehumidification lane for declared thermostat,
cooling total/sensible/latent rate, supply-air cooling report-rate, and
supply-node temperature/mass-flow/humidity rows. The compare path reads
EnergyPlus `ZoneSysMoistureDemand` proof rows for the humidifying and
dehumidifying moisture transfer rates, uses the same-timestamp return node as
the source-order zone state for the first run-period sample, and matches the
Humidistat dehumidification supply mass flow, supply humidity ratio, and
cooling report rows with zero tolerance failures. The original
`ideal_loads_humidistat_dehumidification_diagnostic_001` remains available as
non-claim regression/proof evidence for the broader diagnostic output set.

`ideal_loads_humidistat_humidification_conformance_candidate_001` promotes the
matching no-OA Humidistat humidification lane for declared thermostat,
heating total/sensible/latent rate, supply-air heating report-rate, and
supply-node temperature/mass-flow/humidity rows. It reads the EnergyPlus
humidifying moisture proof row into `ZoneSysEnergyDemand`, lets the
humidification mass-flow request exceed the sensible heating flow when needed,
uses the maximum heating supply humidity ratio with the same saturation clamp,
and matches the supply humidity and heating report rows with zero tolerance
failures. The original `ideal_loads_humidistat_humidification_diagnostic_001`
remains available as non-claim regression/proof evidence for the broader
diagnostic output set.

These remain diagnostic-only: Humidistat schedule-to-moisture-demand
calculation, outdoor-air humidity control,
DifferentialEnthalpy economizer humidity interactions, heat-recovery humidity
interactions, finite-limit humidity-control behavior, and broad humidity-control
conformance.

## Outdoor-Air Prerequisites

Outdoor-air IdealLoads conformance is promoted only for the declared Flow/Zone,
Flow/Person, Flow/Person OccupancySchedule DCV, Flow/Person CO2Setpoint DCV,
Flow/Area, AirChanges/Hour, Sum, Maximum, Flow/Zone
DifferentialDryBulb/DifferentialEnthalpy economizer, and Flow/Zone
Sensible/Enthalpy heat-recovery candidate rows. The current Rust surface is:

- `DesignSpecification:OutdoorAir` typed intake with method, flow terms, and
  schedule references preserved in `TypedModel`
- `ModelGraph::ideal_loads_outdoor_air_specs` linking an IdealLoads system to
  its referenced outdoor-air design specification
- `People` typed intake for zone design occupant count used by the Flow/Person
  conformance candidate, current People occupancy schedule values used by the
  Flow/Person OccupancySchedule DCV conformance candidate, and the diagnostic
  proof lane
- `design_outdoor_air_volume_flow_components_m3_per_s` for reporting the
  Flow/Person, Flow/Area, Flow/Zone, and AirChanges/Hour component terms plus
  the selected final design volume flow
- `occupancy_schedule_dcv_outdoor_air_volume_flow_components_m3_per_s` for
  source-order `UseOccSchFlag=true` Flow/Person component terms using current
  People schedule occupancy
- `calc_design_outdoor_air_volume_flow_m3_per_s` for supported
  `Flow/Person`, `Flow/Area`, `Flow/Zone`, `AirChanges/Hour`, `Sum`, and
  `Maximum` methods
- `calc_scheduled_outdoor_air_mass_flow_rate_kg_per_s` for applying the
  current OA schedule fraction and `StdRhoAir`
- `calc_occupancy_schedule_dcv_outdoor_air_mass_flow_rate_kg_per_s` for the
  EnergyPlus OccupancySchedule DCV path that recomputes the Flow/Person
  minimum outdoor-air flow from current People schedule occupancy before the
  `StdRhoAir` conversion
- `Zone Air CO2 Predicted Load to Setpoint Mass Flow Rate` as the EnergyPlus
  System/Average proof input for
  `ZoneSysContDemand(ZoneNum).OutputRequiredToCO2SP`, combined with the
  source-order `CalcPurchAirMinOAMassFlow` `max(minimum OA, CO2 demand)` branch
  for the declared Flow/Person CO2Setpoint DCV candidate
- `calc_outdoor_air_sensible_report_rates_compat` for the no-humidity
  Flow/Person, Flow/Zone, Flow/Area, AirChanges/Hour, Sum, Maximum, and
  DifferentialDryBulb/DifferentialEnthalpy economizer OA report-rate and
  mixed-air state evidence, including no-heat-recovery rows,
  DifferentialDryBulb and DifferentialEnthalpy economizer OA flow reset,
  and Sensible/Enthalpy heat-recovery OA tempering/rate reporting
- `crates/ep_cli/src/ideal_loads.rs::build_outdoor_air_design_flow_context`
  resolves the EnergyPlus return/exhaust recirculation node and, when the
  diagnostic fixture requests its `System Node Temperature` and
  `System Node Humidity Ratio`, feeds that same-call state into the Rust
  outdoor-air mixed-air, economizer, and heat-recovery comparison lane
- `manifest_allows_outdoor_air_flow_zone_conformance_manifest`,
  `manifest_allows_outdoor_air_flow_person_conformance_manifest`,
  `manifest_allows_outdoor_air_occupancy_dcv_conformance_manifest`,
  `manifest_allows_outdoor_air_co2_dcv_conformance_manifest`,
  `manifest_allows_outdoor_air_flow_area_conformance_manifest`,
  `manifest_allows_outdoor_air_air_changes_conformance_manifest`,
  `manifest_allows_outdoor_air_sum_conformance_manifest`,
  `manifest_allows_outdoor_air_maximum_conformance_manifest`,
  `manifest_allows_outdoor_air_differential_dry_bulb_economizer_conformance_manifest`,
  `manifest_allows_outdoor_air_differential_enthalpy_economizer_conformance_manifest`,
  and
  `validate_outdoor_air_conformance_boundary` in
  `crates/ep_cli/src/ideal_loads.rs` for the promoted Flow/Zone, Flow/Person,
  Flow/Person OccupancySchedule DCV, Flow/Person CO2Setpoint DCV, Flow/Area,
  AirChanges/Hour, Sum, Maximum, and DifferentialDryBulb/DifferentialEnthalpy
  economizer candidates

`ideal_loads_outdoor_air_flow_person_conformance_candidate_001` promotes the
Flow/Person proof lane. The fixture declares five `People` design occupants
and 0.01 m3/s-person outdoor air, so the derived design volume is 0.05 m3/s
before the `StdRhoAir` mass-flow conversion used by the rest of the outdoor-air
lane. The People occupancy schedule is zero to avoid adding People heat gains;
People heat-gain conformance remains outside the claim.

`ideal_loads_outdoor_air_occupancy_dcv_conformance_candidate_001` promotes the
Flow/Person proof lane with `Demand Controlled Ventilation Type =
OccupancySchedule`. The fixture declares five `People` design occupants, a
non-constant all-days compact occupancy schedule that steps from 0.0 to 1.0 to
0.5, a zero activity schedule, and 0.01 m3/s-person outdoor air. That isolates
EnergyPlus `UseOccSchFlag=true` outdoor-air behavior: current People occupancy
varies from 0 to 5 people, the minimum outdoor-air volume varies from 0.0 to
0.05 m3/s, and People heat gains remain outside the claim. The mass/volume,
latent, supply-air, mixed-air, and inactive proof rows keep the existing strict
tolerances; outdoor-air sensible and total heating rows use the case-declared
4 W absolute and 1 W RMSE tolerance for a single source-order timestep edge.
`ideal_loads_outdoor_air_co2_dcv_conformance_candidate_001` promotes the
Flow/Person proof lane with `Demand Controlled Ventilation Type =
CO2Setpoint`. The fixture declares five `People` design occupants, a
non-constant all-days compact occupancy schedule, 120 W/person activity,
3.82E-8 m3/s-W CO2 generation, a 600 ppm CO2 setpoint, a 400 ppm outdoor CO2
schedule, and 0.01 m3/s-person outdoor air. EnergyPlus
`ZoneContaminantPredictorCorrector::PredictZoneContaminants` writes
`ZoneSysContDemand(ZoneNum).OutputRequiredToCO2SP` and exposes it through
`Zone Air CO2 Predicted Load to Setpoint Mass Flow Rate`; `CalcPurchAirMinOAMassFlow`
then applies `max(minimum OA, OutputRequiredToCO2SP)`. The candidate claims only
the resulting IdealLoads outdoor-air, supply-air, and mixed-air rows. CO2
contaminant-balance conformance, CO2 concentration conformance, People
heat-gain conformance, and broader DCV method combinations remain outside the
claim.

`ideal_loads_outdoor_air_flow_person_diagnostic_001` remains the diagnostic
predecessor artifact for the same Flow/Person fixture shape.

`ideal_loads_outdoor_air_flow_zone_conformance_candidate_001` promotes the
same Flow/Zone fixture shape for declared EnergyPlus report variables
`Zone Ideal Loads Outdoor Air Mass Flow Rate` and
`Zone Ideal Loads Outdoor Air Standard Density Volume Flow Rate`, plus
`Zone Ideal Loads Outdoor Air Sensible Heating Rate` and
`Zone Ideal Loads Outdoor Air Sensible Cooling Rate`,
`Zone Ideal Loads Outdoor Air Latent Heating Rate`,
`Zone Ideal Loads Outdoor Air Latent Cooling Rate`,
`Zone Ideal Loads Outdoor Air Total Heating Rate`,
`Zone Ideal Loads Outdoor Air Total Cooling Rate`, plus
`Zone Ideal Loads Supply Air Mass Flow Rate`,
`Zone Ideal Loads Supply Air Standard Density Volume Flow Rate`,
`Zone Ideal Loads Supply Air Temperature`,
`Zone Ideal Loads Supply Air Humidity Ratio`, plus
`Zone Ideal Loads Mixed Air Temperature` and
`Zone Ideal Loads Mixed Air Humidity Ratio`. Inactive heat-recovery/economizer
report variables remain diagnostic proof rows. The compare lane derives
EnergyPlus `StdRhoAir` from `Site:Location`, applies the blank OA schedule as
always 1.0, and writes Rust `ResultStore` series for the 96 Detailed oracle
samples. The outdoor-air mass/volume, no-humidity latent, supply-air
mass/volume/humidity, and mixed-air rows are exact in this fixture; the
sensible/total report rows use a 1 W conformance tolerance, and supply-air
temperature uses 0.02 C because EnergyPlus sorts them from source-order zone/OA
state and report-rate mode gates. The guard requires Flow/Zone, blank OA
schedule, `NoEconomizer`, no heat recovery, no finite flow/capacity limits, no
DCV, default `ConstantSensibleHeatRatio` dehumidification, and no
humidification control. Inactive economizer/heat-recovery rows are exact zeros
but are not promoted.

`ideal_loads_outdoor_air_design_flow_diagnostic_001` remains the diagnostic
predecessor artifact for the same Flow/Zone fixture shape.

`ideal_loads_outdoor_air_flow_area_conformance_candidate_001` promotes the
Flow/Area proof lane. The fixture uses a 1 m2 typed floor surface area and
0.05 m3/s-m2 outdoor air, so the derived design volume is 0.05 m3/s before the
same `StdRhoAir` mass-flow conversion. The compare path derives the zone floor
area from typed floor surfaces, promotes the same 14 outdoor-air/supply/mixed
rows as the Flow/Zone and Flow/Person candidates, and keeps inactive
economizer/heat-recovery rows diagnostic-only.

`ideal_loads_outdoor_air_flow_area_diagnostic_001` remains the diagnostic
predecessor artifact for the same Flow/Area fixture shape.

`ideal_loads_outdoor_air_air_changes_conformance_candidate_001` promotes the
AirChanges/Hour proof lane. The fixture uses 180 ACH over the explicit 1 m3
zone volume, so the derived design volume remains 0.05 m3/s before the same
`StdRhoAir` mass-flow conversion. The compare path derives the typed zone
volume, promotes the same 14 outdoor-air/supply/mixed rows as the Flow/Zone,
Flow/Person, and Flow/Area candidates, and keeps inactive
economizer/heat-recovery rows diagnostic-only.

`ideal_loads_outdoor_air_air_changes_diagnostic_001` remains the diagnostic
predecessor artifact for the same AirChanges/Hour fixture shape.

`ideal_loads_outdoor_air_sum_conformance_candidate_001` promotes the Sum proof
lane. The fixture combines 0.015 m3/s Flow/Area, 0.025 m3/s Flow/Zone, and
0.010 m3/s AirChanges/Hour component terms to 0.05 m3/s, and the compare gate
checks each component term in `compare-summary.json`, `stage-summary.json`, and
`compare-report.md` before claiming the same 14 outdoor-air/supply/mixed rows.
`ideal_loads_outdoor_air_sum_diagnostic_001` remains the diagnostic predecessor
artifact for the same Sum fixture shape.

`ideal_loads_outdoor_air_maximum_conformance_candidate_001` promotes the
Maximum proof lane. The fixture combines 0.015 m3/s Flow/Area, 0.025 m3/s
Flow/Zone, and 0.050 m3/s AirChanges/Hour component terms, then selects the
AirChanges/Hour term as the governing 0.05 m3/s design outdoor-air volume. The
compare gate checks each component term in `compare-summary.json`,
`stage-summary.json`, and `compare-report.md` before claiming the same 14
outdoor-air/supply/mixed rows.
`ideal_loads_outdoor_air_maximum_diagnostic_001` remains the diagnostic
predecessor artifact for the same Maximum fixture shape.

`ideal_loads_outdoor_air_differential_dry_bulb_economizer_conformance_candidate_001`
promotes the Flow/Zone outdoor-air method with the minimum design flow lowered
to 0.001 m3/s so the cooling branch can exercise the EnergyPlus
`DifferentialDryBulb` economizer reset. The compare lane reports 110
source-order Detailed samples, including system substep active-time rows,
promotes the same 14 outdoor-air/supply/mixed rows plus
`Zone Ideal Loads Economizer Active Time`, and checks that economizer active
time is nonzero and outdoor-air mass flow rises above the design minimum. Its
inactive heat-recovery rows remain diagnostic-only proof rows.

`ideal_loads_outdoor_air_differential_dry_bulb_economizer_diagnostic_001`
remains the diagnostic predecessor artifact for the same DifferentialDryBulb
fixture shape.

`ideal_loads_outdoor_air_differential_enthalpy_economizer_diagnostic_001`
remains the diagnostic predecessor artifact for the same DifferentialEnthalpy
fixture shape.

`ideal_loads_outdoor_air_differential_enthalpy_economizer_conformance_candidate_001`
promotes the same Flow/Zone low-minimum fixture shape for the EnergyPlus
`DifferentialEnthalpy` economizer reset. The Rust lane compares outdoor-air
enthalpy against the recirculation enthalpy before applying the same source
cooling-flow reset, reports 110 source-order Detailed samples, promotes the
same 14 outdoor-air/supply/mixed rows plus `Zone Ideal Loads Economizer Active
Time`, and keeps inactive heat-recovery rows diagnostic-only.

`ideal_loads_outdoor_air_sensible_heat_recovery_diagnostic_001` is the
diagnostic predecessor for the same Flow/Zone outdoor-air method with
`NoEconomizer` and `HeatRecoveryType = Sensible`.

`ideal_loads_outdoor_air_sensible_heat_recovery_conformance_candidate_001`
promotes that Sensible heat-recovery fixture shape for declared outdoor-air
mass/volume, no-humidity report-rate, supply-air state, mixed-air state,
Sensible heat-recovery rate, and heat-recovery active-time rows. The Rust lane
applies the EnergyPlus sensible heat-recovery outdoor-air tempering branch
when recirculation air can beneficially warm or cool outdoor air, reports 96
Detailed samples, keeps inactive economizer active-time diagnostic-only, and
requires heat-recovery active time to be nonzero. Humidity ratio is unchanged
by Sensible heat recovery, so the latent heat-recovery rows are zero-valued
conformance rows inside this narrow fixture.

`ideal_loads_outdoor_air_enthalpy_heat_recovery_conformance_candidate_001`
promotes the same Flow/Zone and `NoEconomizer` fixture shape with
`HeatRecoveryType = Enthalpy`. The Rust lane follows the EnergyPlus enthalpy
gate, applies sensible and latent heat-recovery effectiveness to the outdoor-air
state before mixing, passes EPW barometric pressure into the heat-recovery
saturation check, reads `ZONE ONE RETURN` as the EnergyPlus
`ZoneRecircAirNodeNum` same-call recirculation state, and reports 96 Detailed
samples for active-time, sensible, latent, and total heat-recovery rows. The
candidate keeps inactive economizer active-time diagnostic-only; general
saturation-limit heat-recovery branch parity beyond this declared fixture
remains outside the promoted claim.

Indoor air quality beyond the declared CO2Setpoint proof input,
proportional-control, heat-recovery saturation-limit generality, outdoor-air
finite limits, outdoor-air humidity-control, other active humidity-control, and
broader DCV method combinations remain diagnostic or unresolved and are not
part of the promoted IdealLoads claim.

## Required Proof Variables

The conformance output surface is:

- `Zone Thermostat Heating Setpoint Temperature`
- `Zone Thermostat Cooling Setpoint Temperature`
- `Zone Ideal Loads Zone Total Heating Rate`
- `Zone Ideal Loads Zone Total Cooling Rate`
- `Zone Ideal Loads Zone Sensible Heating Rate`
- `Zone Ideal Loads Zone Sensible Cooling Rate`
- `Zone Ideal Loads Supply Air Total Heating Rate`
- `Zone Ideal Loads Supply Air Total Cooling Rate`
- `Zone Ideal Loads Supply Air Total Heating Energy` and
  `Zone Ideal Loads Supply Air Total Cooling Energy` for the no-OA
  ReportPurchasedAir non-fuel energy candidate
- `Zone Ideal Loads Zone Total Heating Energy` and
  `Zone Ideal Loads Zone Total Cooling Energy` for the no-OA ReportPurchasedAir
  non-fuel energy candidate
- `Zone Ideal Loads Supply Air Total Heating Fuel Energy Rate`,
  `Zone Ideal Loads Supply Air Total Cooling Fuel Energy Rate`,
  `Zone Ideal Loads Zone Heating Fuel Energy Rate`, and
  `Zone Ideal Loads Zone Cooling Fuel Energy Rate` for the no-OA blank
  fuel-efficiency, constant Schedule:Constant fuel-efficiency, and all-days
  Schedule:Compact fuel-efficiency candidates
- `Zone Ideal Loads Supply Air Total Heating Fuel Energy`,
  `Zone Ideal Loads Supply Air Total Cooling Fuel Energy`,
  `Zone Ideal Loads Zone Heating Fuel Energy`, and
  `Zone Ideal Loads Zone Cooling Fuel Energy` for the no-OA blank
  fuel-efficiency, constant Schedule:Constant fuel-efficiency, and all-days
  Schedule:Compact fuel-efficiency candidates
- `System Node Temperature`
- `System Node Mass Flow Rate`
- `Zone Ideal Loads Zone Latent Cooling Rate` for the no-OA
  `ConstantSensibleHeatRatio` and `ConstantSupplyHumidityRatio` cooling cases
- `Zone Ideal Loads Supply Air Sensible Cooling Rate` for the no-OA
  `ConstantSensibleHeatRatio` and `ConstantSupplyHumidityRatio` cooling cases
- `Zone Ideal Loads Supply Air Latent Cooling Rate` for the no-OA
  `ConstantSensibleHeatRatio` and `ConstantSupplyHumidityRatio` cooling cases
- `Zone Ideal Loads Zone Latent Heating Rate` for the no-OA
  `ConstantSupplyHumidityRatio` heating case
- `Zone Ideal Loads Supply Air Sensible Heating Rate` for the no-OA
  `ConstantSupplyHumidityRatio` heating case
- `Zone Ideal Loads Supply Air Latent Heating Rate` for the no-OA
  `ConstantSupplyHumidityRatio` heating case
- `System Node Humidity Ratio` for the no-OA `ConstantSensibleHeatRatio` and
  `ConstantSupplyHumidityRatio` cooling/heating supply nodes only
- `Zone Ideal Loads Outdoor Air Mass Flow Rate` and
  `Zone Ideal Loads Outdoor Air Standard Density Volume Flow Rate` for the
  promoted Flow/Zone, Flow/Person, Flow/Person OccupancySchedule DCV,
  Flow/Person CO2Setpoint DCV, Flow/Area, AirChanges/Hour, Sum, and Maximum
  outdoor-air candidates
- `Zone Ideal Loads Outdoor Air Sensible/Latent/Total Heating/Cooling Rate`
  for the no-active-humidity-control promoted outdoor-air candidates
- `Zone Ideal Loads Supply Air Mass Flow Rate`,
  `Zone Ideal Loads Supply Air Standard Density Volume Flow Rate`,
  `Zone Ideal Loads Supply Air Temperature`, and
  `Zone Ideal Loads Supply Air Humidity Ratio` for the promoted outdoor-air
  candidates
- `Zone Ideal Loads Mixed Air Temperature` and
  `Zone Ideal Loads Mixed Air Humidity Ratio` for the promoted outdoor-air
  candidates
- `Zone Ideal Loads Heat Recovery Sensible/Latent/Total Heating/Cooling Rate`
  and `Zone Ideal Loads Heat Recovery Active Time` for the promoted Sensible
  and Enthalpy heat-recovery outdoor-air candidates

The active signed `Zone System Predicted Sensible Load to Setpoint Heat
Transfer Rate`, non-promoted humidity-ratio rows, zone-air-node proof rows,
heating/cooling setpoint-distance proof rows, active humidity-control
outdoor-air latent behavior,
economizer outputs, finite-limit humidity or energy behavior, adaptive system
timestep, broad meter conformance beyond the declared no-OA hourly and
monthly/annual/run-period facility meter candidates, multi-year annual
grouping, and
fuel-efficiency schedules beyond the declared
blank/constant/all-days Schedule:Compact candidates remain diagnostic-only or
unsupported until their source-order branches are ported or explicitly
included in a promoted claim. Blank fuel-efficiency energy/rate rows have
narrow conformance evidence in
`ideal_loads_blank_fuel_efficiency_conformance_candidate_001`; constant
Schedule:Constant fuel-efficiency energy/rate rows have narrow conformance
evidence in
`ideal_loads_constant_fuel_efficiency_conformance_candidate_001`; all-days
Schedule:Compact fuel-efficiency energy/rate rows have narrow conformance
evidence in
`ideal_loads_non_constant_fuel_efficiency_conformance_candidate_001`; raw
IdealLoads rate rows and facility meter rows remain diagnostic.
`DistrictHeatingWater:Facility` and `DistrictCooling:Facility` have a narrow
hourly oracle-MTR vs Rust aggregated fuel-energy conformance candidate in
`ideal_loads_no_oa_facility_meter_conformance_candidate_001` and a narrow
monthly/annual/run-period oracle-MTR vs Rust aggregated fuel-energy conformance
candidate in
`ideal_loads_no_oa_facility_meter_monthly_run_period_conformance_candidate_001`;
outside those cases, facility meter rows remain diagnostic. The no-OA
`ConstantSensibleHeatRatio`
cooling total/sensible/latent rows and supply-node humidity ratio have narrow
conformance evidence in `ideal_loads_constant_shr_conformance_001`; its
return-node and zone-air-node humidity proof rows remain diagnostic. The no-OA
`ConstantSupplyHumidityRatio` cooling zone/supply latent and sensible rows plus
supply-node humidity ratio have narrow conformance evidence in
`ideal_loads_constant_supply_humidity_cooling_conformance_candidate_001`; its
return-node humidity, zone-air humidity, energy/fuel, and meter rows remain
diagnostic. The no-OA `ConstantSupplyHumidityRatio` heating zone/supply latent
and sensible rows plus supply-node humidity ratio have narrow conformance
evidence in `ideal_loads_constant_supply_humidity_heating_conformance_candidate_001`;
its cooling, return-node humidity, zone-air humidity, energy/fuel, and meter
rows remain diagnostic.
The outdoor-air mass-flow, standard-density volume-flow, no-humidity
outdoor-air report-rate, supply-air state, and mixed-air state rows have
promoted conformance evidence in the Flow/Zone, Flow/Person,
Flow/Person OccupancySchedule DCV, Flow/Area, AirChanges/Hour, Sum, Maximum,
and DifferentialDryBulb/DifferentialEnthalpy economizer and Sensible/Enthalpy
heat-recovery conformance candidates.
Inactive economizer/heat-recovery outputs and the original diagnostic
outdoor-air predecessor fixtures remain diagnostic evidence:
`ideal_loads_outdoor_air_flow_person_diagnostic_001`,
`ideal_loads_outdoor_air_design_flow_diagnostic_001`,
`ideal_loads_outdoor_air_flow_area_diagnostic_001`,
`ideal_loads_outdoor_air_air_changes_diagnostic_001`,
`ideal_loads_outdoor_air_sum_diagnostic_001`,
`ideal_loads_outdoor_air_maximum_diagnostic_001`,
`ideal_loads_outdoor_air_differential_dry_bulb_economizer_diagnostic_001`,
`ideal_loads_outdoor_air_differential_enthalpy_economizer_diagnostic_001`,
`ideal_loads_outdoor_air_sensible_heat_recovery_diagnostic_001`, and
`ideal_loads_outdoor_air_enthalpy_heat_recovery_diagnostic_001`.
The capacity-limit, flow-limit, and flow-and-capacity-limit fixtures now have
blocking conformance gates for their declared thermostat, IdealLoads rate, and
supply-node temperature/mass-flow rows.

## Conformance Compare Artifacts

`scripts/dev.cmd compare-ideal-loads-no-oa-sensible-conformance` generates the
current evidence set under
`.runtime/ideal-loads-no-oa-sensible/26.1.0/ideal_loads_no_oa_sensible_conformance_001/compare/`.
The artifact contract is:

- `selected_outputs.json`
- `rust-result-store.json`
- `compare-summary.json`
- `compare-report.md`
- `variable-deltas.csv`
- `first-divergence.csv`
- `tolerance-failures.csv`
- `stage-summary.json`

The current conformance run compares 28 Detailed series over 110 samples. The
10 declared conformance rows pass their tolerances, diagnostic proof rows also
pass, the two hourly facility meter diagnostic rows pass in
`compare-summary.json`/`compare-report.md`, and `tolerance-failures.csv` is
empty. This creates only the limited no-OA/no-limit sensible IdealLoads claim
for declared outputs; ReportPurchasedAir non-fuel energy rows are promoted
only by the separate report-energy candidate, constant Schedule:Constant
fuel-efficiency energy/rate rows are promoted only by the separate constant
fuel-efficiency candidate, blank fuel-efficiency energy/rate rows are promoted
only by the separate blank fuel-efficiency candidate, all-days Schedule:Compact
fuel-efficiency energy/rate rows are promoted only by the separate non-constant
fuel-efficiency candidate, and hourly facility meters remain diagnostic in this
case.

`scripts/dev.cmd compare-ideal-loads-no-oa-report-energy-conformance-candidate`
generates the report-energy evidence set under
`.runtime/ideal-loads-no-oa-report-energy-conformance/26.1.0/ideal_loads_no_oa_report_energy_conformance_candidate_001/compare/`.
That run compares 28 Detailed series over 110 samples. Only the four declared
non-fuel ReportPurchasedAir energy rows are promoted; constant
Schedule:Constant fuel-efficiency rows are promoted only by their separate
candidate, blank fuel-efficiency rows are promoted only by their separate
candidate, all-days Schedule:Compact fuel-efficiency rows are promoted only by
their separate candidate, and raw rate, thermostat, demand, humidity, node, and
facility meter rows remain diagnostic proof evidence; `tolerance-failures.csv`
is empty.

`scripts/dev.cmd compare-ideal-loads-blank-fuel-efficiency-conformance-candidate`
generates the blank fuel-efficiency evidence set under
`.runtime/ideal-loads-blank-fuel-efficiency-conformance/26.1.0/ideal_loads_blank_fuel_efficiency_conformance_candidate_001/compare/`.
That run compares 12 Detailed series over 110 samples. Only the eight declared
blank fuel energy-rate and fuel energy rows are promoted; raw IdealLoads rate
rows and the two facility meter rows remain diagnostic proof evidence, and
`tolerance-failures.csv` is empty. Constant Schedule:Constant and all-days
Schedule:Compact efficiency schedules plus broad meter conformance remain
outside this claim.

`scripts/dev.cmd compare-ideal-loads-constant-fuel-efficiency-conformance-candidate`
generates the constant fuel-efficiency evidence set under
`.runtime/ideal-loads-constant-fuel-efficiency-conformance/26.1.0/ideal_loads_constant_fuel_efficiency_conformance_candidate_001/compare/`.
That run compares 12 Detailed series over 110 samples. Only the eight declared
constant Schedule:Constant fuel energy-rate and fuel energy rows are promoted;
raw IdealLoads rate rows and the two facility meter rows remain diagnostic
proof evidence, and `tolerance-failures.csv` is empty. Blank and all-days
Schedule:Compact efficiency schedules plus broad meter conformance remain
outside this claim.

`scripts/dev.cmd compare-ideal-loads-non-constant-fuel-efficiency-conformance-candidate`
generates the all-days Schedule:Compact fuel-efficiency evidence set under
`.runtime/ideal-loads-non-constant-fuel-efficiency-conformance/26.1.0/ideal_loads_non_constant_fuel_efficiency_conformance_candidate_001/compare/`.
That run compares 12 Detailed series over 110 samples. Only the eight declared
all-days Schedule:Compact fuel energy-rate and fuel energy rows are promoted;
raw IdealLoads rate rows and the two facility meter rows remain diagnostic
proof evidence, and `tolerance-failures.csv` is empty. Blank and constant
Schedule:Constant efficiency schedules plus broad meter conformance remain
outside this claim.

`scripts/dev.cmd compare-ideal-loads-no-oa-facility-meter-conformance-candidate`
generates the meter-only evidence set under
`.runtime/ideal-loads-no-oa-facility-meter-conformance/26.1.0/ideal_loads_no_oa_facility_meter_conformance_candidate_001/compare/`.
That run compares 28 Detailed diagnostic proof rows over 110 samples and two
hourly MTR facility meter rows over 24 samples. Only
`DistrictHeatingWater:Facility` and `DistrictCooling:Facility` are promoted;
ReportPurchasedAir rate, energy, fuel-energy, thermostat, demand, humidity,
and node rows remain diagnostic proof evidence inside the meter candidate, and
`tolerance-failures.csv` is empty.

`scripts/dev.cmd compare-ideal-loads-no-oa-facility-meter-monthly-run-period-conformance-candidate`
generates the monthly/annual/run-period meter-only evidence set under
`.runtime/ideal-loads-no-oa-facility-meter-monthly-run-period-conformance/26.1.0/ideal_loads_no_oa_facility_meter_monthly_run_period_conformance_candidate_001/compare/`.
That run compares 28 Detailed diagnostic proof rows over 39292 samples plus six
monthly/annual/run-period MTR facility meter rows, one monthly, one annual,
and one run-period row for each declared facility meter. Only the
monthly/annual/run-period
`DistrictHeatingWater:Facility` and `DistrictCooling:Facility` meter rows are
promoted; ReportPurchasedAir rate, energy, fuel-energy, thermostat, demand,
humidity, and node rows remain diagnostic proof evidence inside the meter
candidate, multi-year annual grouping remains outside the claim, and
the two full-year diagnostic node rows in `tolerance-failures.csv` remain
outside the meter conformance gate.

`scripts/dev.cmd compare-ideal-loads-capacity-limit-conformance` generates the
capacity-limit conformance evidence set under
`.runtime/ideal-loads-capacity-limit-conformance/26.1.0/ideal_loads_capacity_limit_conformance_001/compare/`.
That run compares 18 Detailed series over 188 samples. The 10 declared
conformance rows pass their tolerances, the 8 diagnostic proof rows pass, and
`tolerance-failures.csv` is empty. This adds only the no-OA numeric
capacity-limit sensible claim for declared outputs.

`scripts/dev.cmd compare-ideal-loads-flow-limit-conformance` generates the
flow-limit conformance evidence set under
`.runtime/ideal-loads-flow-limit-conformance/26.1.0/ideal_loads_flow_limit_conformance_001/compare/`.
That run compares 18 Detailed series over 128 samples. The 10 declared
conformance rows pass their tolerances, the 8 diagnostic proof rows pass, and
`tolerance-failures.csv` is empty. This adds only the no-OA numeric flow-limit
sensible claim for declared outputs.

`scripts/dev.cmd compare-ideal-loads-flow-capacity-limit-conformance`
generates the flow-and-capacity-limit conformance evidence set under
`.runtime/ideal-loads-flow-capacity-limit-conformance/26.1.0/ideal_loads_flow_capacity_limit_conformance_001/compare/`.
That run compares 18 Detailed series over 189 samples. The 10 declared
conformance rows pass their tolerances, the 8 diagnostic proof rows pass, and
`tolerance-failures.csv` is empty. Together these finite-limit gates still
exclude humidity, outdoor-air, economizer, heat-recovery, and broad HVAC
behavior from the claim.

`scripts/dev.cmd compare-ideal-loads-constant-shr-conformance` generates the
ConstantSensibleHeatRatio cooling conformance evidence set under
`.runtime/ideal-loads-constant-shr-conformance/26.1.0/ideal_loads_constant_shr_conformance_001/compare/`.
That run compares 18 Detailed series over 96 samples. The 11 declared
conformance rows pass their tolerances, the 7 diagnostic proof rows pass, and
`tolerance-failures.csv` is empty. This adds only the no-OA Constant SHR
cooling claim for declared outputs; ConstantSupplyHumidityRatio is handled by
its separate cooling candidate, while Humidistat, return-node and zone-air-node
humidity proof rows, outdoor-air, economizer, heat-recovery, and broad HVAC
behavior remain outside this claim.

`scripts/dev.cmd compare-ideal-loads-constant-supply-humidity-cooling-conformance-candidate`
generates the ConstantSupplyHumidityRatio cooling conformance evidence set under
`.runtime/ideal-loads-constant-supply-humidity-cooling-conformance/26.1.0/ideal_loads_constant_supply_humidity_cooling_conformance_candidate_001/compare/`.
That run compares 36 Detailed/hourly series. The 11 declared conformance rows
pass their tolerances, the 25 diagnostic proof rows pass, and
`tolerance-failures.csv` is empty. This adds only the no-OA ConstantSupplyHumidityRatio
cooling claim for declared outputs; Humidistat, return-node and zone-air-node
humidity proof rows, ReportPurchasedAir energy/fuel rows, meters, outdoor-air,
economizer, heat-recovery, and broad HVAC behavior remain outside the claim.

`scripts/dev.cmd compare-ideal-loads-constant-supply-humidity-heating-conformance-candidate`
generates the ConstantSupplyHumidityRatio heating conformance evidence set under
`.runtime/ideal-loads-constant-supply-humidity-heating-conformance/26.1.0/ideal_loads_constant_supply_humidity_heating_conformance_candidate_001/compare/`.
That run compares 36 Detailed/hourly series. The 11 declared conformance rows
pass their tolerances, the 25 diagnostic proof rows pass, and
`tolerance-failures.csv` is empty. This adds only the no-OA ConstantSupplyHumidityRatio
heating claim for declared outputs; Humidistat, return-node and zone-air-node
humidity proof rows, ReportPurchasedAir energy/fuel rows, meters, outdoor-air,
economizer, heat-recovery, and broad HVAC behavior remain outside the claim.

`scripts/dev.cmd compare-ideal-loads-humidistat-dehumidification-conformance-candidate`
generates the Humidistat dehumidification conformance evidence set under
`.runtime/ideal-loads-humidistat-dehumidification-conformance/26.1.0/ideal_loads_humidistat_dehumidification_conformance_candidate_001/compare/`.
That run compares 38 Detailed/hourly series. The 11 declared conformance rows
pass their tolerances, the 27 diagnostic proof rows pass, and
`tolerance-failures.csv` is empty. This adds only the no-OA Humidistat
dehumidification claim for declared outputs; EnergyPlus moisture-demand rows,
humidistat schedule-to-moisture-demand calculation, return-node and
zone-air-node humidity proof rows, ReportPurchasedAir energy/fuel rows, meters,
outdoor-air, economizer, heat-recovery, and broad HVAC behavior remain outside
the claim.

`scripts/dev.cmd compare-ideal-loads-humidistat-humidification-conformance-candidate`
generates the Humidistat humidification conformance evidence set under
`.runtime/ideal-loads-humidistat-humidification-conformance/26.1.0/ideal_loads_humidistat_humidification_conformance_candidate_001/compare/`.
That run compares 38 Detailed/hourly series. The 11 declared conformance rows
pass their tolerances, the 27 diagnostic proof rows pass, and
`tolerance-failures.csv` is empty. This adds only the no-OA Humidistat
humidification claim for declared outputs; EnergyPlus moisture-demand rows,
humidistat schedule-to-moisture-demand calculation, return-node and
zone-air-node humidity proof rows, ReportPurchasedAir energy/fuel rows, meters,
outdoor-air, economizer, heat-recovery, and broad HVAC behavior remain outside
the claim.

## Claim Requirements

The claim remains valid only while all of these exist:

- `comparison_class = "conformance"`
- `conformance_claim = true`
- conformance-level output requests only for variables that pass tolerance
- EnergyPlus oracle selected output artifacts
- Rust `ResultStore` artifacts for the same keys, variables, and timestamps
- timestamp and warmup alignment notes
- absolute or relative tolerance rules
- compare summary with zero tolerance failures
- first-divergence artifacts
- markdown report artifact
- blocking gate

Any broader IdealLoads feature must add its own evidence before joining the
conformance boundary.
