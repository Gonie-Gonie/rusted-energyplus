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
- `src/EnergyPlus/HVACSizingSimulationManager.cc`: autosizing source anchor for
  out-of-claim IdealLoads sizing semantics; autosized IdealLoads flow/capacity
  conformance remains outside the current claim.

`crates/ep_cli/src/ideal_loads/case_adapter/time_axis.rs` adapts the shared
`ep_runtime::TimeAxis` nominal zone/system timestep metadata and ESO sample
start/end timestamps for the comparison harness. It restores integer TimeAxis
subdivisions when the difference is within ESO's two-decimal-minute display
precision, keeps other valid durations unchanged, and falls back to the nominal
zone timestep for missing or invalid timestamps. The adapter explicitly keeps
adaptive system timestep behavior outside the current claim; it does not
implement `SystemTimeStepState`.

`ep_runtime::IdealLoadsReportSnapshot` owns the complete no-OA
`ReportPurchasedAir` rate and final supply-state payload used by reports. The
comparison CLI consumes that snapshot from the source-order PurchasedAir and
Humidistat wrappers; it does not import or assemble `IdealLoadsSensibleResult`.

`sim_purchased_air_outdoor_air_compat` also owns the
`CalcPurchAirMinOAMassFlow` boundary. Callers provide the resolved outdoor-air
specification plus raw timestep schedule, occupancy, and CO2-demand signals;
the runtime resolves design components, schedule and standard-density
conversion, and the selected DCV branch before the OA load calculation. The
comparison CLI consumes the wrapper result for component and min/max metadata
and does not call design-flow or DCV physics helpers directly.
When the unit is unavailable, the wrapper follows `SimPurchasedAir` by
skipping minimum-OA resolution and returning an explicit absent result with
zero OA flow. For an available unit, the resolved flow applies the EnergyPlus
`HVAC::VerySmallMassFlow` cutoff: values at or below 1e-30 kg/s become zero.

The arbitrary-run compatibility runtime does not yet own timestep evaluators
for nonblank OA schedules, current occupancy, or CO2 contaminant demand. Those
active inputs therefore return a typed `OutdoorAirCalculation` error rather
than silently falling back to design flow; the conformance harness supplies
the declared occupancy and CO2 proof signals to the wrapper directly.

autosized IdealLoads flow/capacity conformance remains outside the current
claim; `SizePurchasedAir` is represented by the runtime policy constant
`IDEAL_LOADS_SIZE_PURCHASED_AIR_POLICY`, and arbitrary-run compatibility blocks
unresolved autosized flow/capacity fields before `CalcPurchAirLoads`.

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
| `PurchasedAirManager::SizePurchasedAir` | `src/EnergyPlus/PurchasedAirManager.cc` | `crates/ep_runtime/src/ideal_loads/dispatch.rs::IDEAL_LOADS_SIZE_PURCHASED_AIR_POLICY` |
| `PurchasedAirManager::CalcPurchAirLoads` | `src/EnergyPlus/PurchasedAirManager.cc` | `crates/ep_runtime/src/ideal_loads/calc/no_oa.rs::calc_no_oa_no_limit_sensible_compat` |
| `PurchasedAirManager::CalcPurchAirMinOAMassFlow` | `src/EnergyPlus/PurchasedAirManager.cc` | `crates/ep_runtime/src/ideal_loads/outdoor_air/minimum_flow.rs::resolve_minimum_outdoor_air_compat`, orchestrated by `sim_purchased_air_outdoor_air_compat` |
| `PurchasedAirManager::UpdatePurchasedAir` | `src/EnergyPlus/PurchasedAirManager.cc` | `crates/ep_runtime/src/ideal_loads/update.rs::supply_node_update_from_result` |
| `PurchasedAirManager::ReportPurchasedAir` | `src/EnergyPlus/PurchasedAirManager.cc` | `crates/ep_runtime/src/ideal_loads/report.rs::IdealLoadsReportSnapshot`; `crates/ep_runtime/src/output/meter_registry.rs::meter_rate_to_energy_j` |
| `DataSizing::calcDesignSpecificationOutdoorAir` | `src/EnergyPlus/DataSizing.cc` | `crates/ep_runtime/src/ideal_loads/outdoor_air/dcv.rs::occupancy_schedule_dcv_outdoor_air_volume_flow_components_m3_per_s` |
| `ZoneEquipmentManager::ManageZoneEquipment` | `src/EnergyPlus/ZoneEquipmentManager.cc` | `crates/ep_runtime/src/zone_equipment/dispatch.rs::ideal_loads_zone_equipment_stages`; `validate_ideal_loads_zone_equipment_dispatch`; `crates/ep_runtime/src/execution_plan.rs::ExecutionPlan` |
| `ZoneEquipmentManager::GetZoneEquipment` | `src/EnergyPlus/ZoneEquipmentManager.cc` | no exact Rust target; adjacent eager typed equipment lists/connections and `TimestepConfig` only |
| `ZoneEquipmentManager::SimZoneEquipment` | `src/EnergyPlus/ZoneEquipmentManager.cc` | `crates/ep_runtime/src/zone_equipment/dispatch.rs::ZoneEquipmentCompatibilityStage` |
| `ZoneTempPredictorCorrector` predicted load state | `src/EnergyPlus/ZoneTempPredictorCorrector.cc` | `crates/ep_runtime/src/zone_equipment/demand.rs::ZoneSysEnergyDemand` |

## Runtime Order

Zone equipment input is acquired earlier and independently of the simulation
driver:

```text
SurfaceGeometry::SetupZoneGeometry
  -> ZoneEquipmentManager::GetZoneEquipment
  -> DataZoneEquipment::GetZoneEquipmentData
```

`ManageZoneEquipment` does not call that input wrapper. EnergyPlus later calls
the IdealLoads component through the zone equipment manager:

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

This is the canonical source dependency order, not an implemented Rust parent
driver. Rust records selected stage labels and validates typed IdealLoads graph
edges, while its compatibility runtime enters prebound PurchasedAir systems
directly. It does not execute the exact `ManageZoneEquipment`
Init/Size-or-Sim/Update protocol, so this ordering alone promotes no broader
IdealLoads or HVAC behavior.

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
source-order direct evaluation with EnergyPlus `PsyPsatFnTemp` cache-temperature
quantization and no reordering, and `ideal_loads_psychrometric_cache_policy`
records that saturation-pressure evaluation mirrors the default EnergyPlus
temperature-key truncation before the raw polynomial.
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

Future official ExampleFile IdealLoads conformance must feed this wrapper from
the source-order zone heat-balance candidate lane instead of the current
oracle-demand fixture injection.

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
thermostat, heating/cooling total/sensible/latent rate, supply-air heating/cooling report-rate,
ReportPurchasedAir energy/fuel rows, supply-node temperature/mass-flow/humidity
rows, and hourly/monthly/run-period `DistrictHeatingWater:Facility`/
`DistrictCooling:Facility` meters. It uses the EnergyPlus minimum cooling supply humidity ratio, preserves
the source's small opposite-side latent-heating report rows when heating
availability is on during cooling, and keeps the same return-node mixed-air and
EPW barometric-pressure saturation proof path.
The original `ideal_loads_constant_supply_humidity_diagnostic_001` remains
available as non-claim regression/proof evidence for the broader diagnostic
output set.

`ideal_loads_constant_supply_humidity_heating_conformance_candidate_001`
promotes the heating-side no-OA `ConstantSupplyHumidityRatio` lane for declared
thermostat, heating/cooling total/sensible/latent rate, supply-air heating/cooling report-rate,
ReportPurchasedAir energy/fuel rows, supply-node temperature/mass-flow/humidity
rows, and hourly/monthly/run-period `DistrictHeatingWater:Facility`/
`DistrictCooling:Facility` meters. It uses the EnergyPlus maximum heating supply humidity ratio in heating
mode, keeps the same return-node mixed-air and saturation proof path, and
matches active latent heating report rows with zero tolerance failures. The original
`ideal_loads_constant_supply_humidity_heating_diagnostic_001` remains available
as non-claim regression/proof evidence for the broader diagnostic output set.

`ideal_loads_humidistat_dehumidification_conformance_candidate_001` promotes
the no-OA Humidistat dehumidification lane for declared thermostat,
heating/cooling total/sensible/latent rate, supply-air heating/cooling
report-rate, and supply-node temperature/mass-flow/humidity rows, plus
ReportPurchasedAir energy/fuel rows and hourly/monthly/run-period
`DistrictHeatingWater:Facility`/`DistrictCooling:Facility` meters. The compare
path computes the humidifying and dehumidifying moisture transfer rates with the
Rust no-OA ThirdOrder moisture-demand predictor, using EnergyPlus trace
temperature, barometric pressure, and `Zone Mean Air Humidity Ratio` row-lag
history plus warmup tail as the declared evidence inputs. Those Rust-computed
values feed the promoted Humidistat branch and match the Humidistat
dehumidification supply mass flow, supply humidity ratio, and cooling report
rows with zero tolerance failures. The original
`ideal_loads_humidistat_dehumidification_diagnostic_001` remains available as
non-claim regression/proof evidence for the broader diagnostic output set.
The seeded closed-loop ThirdOrder predictor and `SimPurchasedAir` results feed
the promoted calculation and moisture-demand rows. The accompanying corrected
zone-humidity and history-residual comparisons remain diagnostic evidence for
fully owned zone-moisture history closure. The promoted path is still
trace-seeded and trace-forced, not a broad standalone Humidistat simulation
claim.
The ThirdOrder coefficient evaluation is shared through
`ep_runtime::third_order_humidity_history_term` by the predictor, corrector,
and CLI residual diagnostic; the history samples and closure remain
trace-driven and this ownership cleanup does not widen the claim.
`ep_runtime::advance_no_oa_humidistat_zone_timestep_compat` now owns each seeded
fixed zone-timestep predictor, moisture-demand injection, `SimPurchasedAir`,
`correctHumRat`, and history-push transition atomically. Adaptive or multiple
system substeps remain outside this boundary. The CLI still adapts EnergyPlus
warmup seed histories, sensible load, temperatures, latent gain, RH schedules,
and pressure, then compares returned runtime values; this does not widen the
claim.

`ideal_loads_humidistat_humidification_conformance_candidate_001` promotes the
matching no-OA Humidistat humidification lane for declared thermostat,
heating/cooling total/sensible/latent rate, supply-air heating/cooling
report-rate, and supply-node temperature/mass-flow/humidity rows, plus
ReportPurchasedAir energy/fuel rows and hourly/monthly/run-period
`DistrictHeatingWater:Facility`/`DistrictCooling:Facility` meters. It computes
the humidifying moisture request with the same trace-driven Rust predictor,
lets the humidification mass-flow request exceed the sensible heating flow when
needed, uses the maximum heating supply humidity ratio with the same saturation
clamp, and matches the supply humidity and heating report rows with zero
tolerance failures. The original
`ideal_loads_humidistat_humidification_diagnostic_001` remains available as
non-claim regression/proof evidence for the broader diagnostic output set.
The compare path uses the same seeded closed-loop predictor and
`SimPurchasedAir` results for promoted humidification rows. Corrected
zone-humidity and history-residual comparisons remain diagnostic until the
zone-moisture history is reproduced without EnergyPlus trace state.

These remain diagnostic-only: fully owned Humidistat schedule-to-moisture-demand
calculation without EnergyPlus trace state/history, `WPrevZoneTSTemp`
warmup/system-history closure, annual facility meter rows in the short-run
humidity-control candidates, outdoor-air humidity control,
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
  `ZoneSysContDemand(ZoneNum).OutputRequiredToCO2SP`
- `calc_co2_setpoint_dcv_outdoor_air_mass_flow_rate_kg_per_s` in
  `crates/ep_runtime/src/ideal_loads/outdoor_air/dcv.rs` implements the
  `max(minimum OA, CO2 demand)` scalar operation used by the declared
  Flow/Person CO2Setpoint DCV candidate
- `resolve_minimum_outdoor_air_compat` in
  `crates/ep_runtime/src/ideal_loads/outdoor_air/minimum_flow.rs` owns the
  source-order design-components, OA-schedule, `StdRhoAir`, OccupancySchedule,
  CO2Setpoint, `HVAC::VerySmallMassFlow`, and final finite-value guard sequence. The
  `sim_purchased_air_outdoor_air_compat` wrapper calls it before the OA load
  calculation and exposes both design and selected component snapshots.
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
`Zone Ideal Loads Mixed Air Humidity Ratio`, the six inactive no-heat-recovery
rate rows, and the inactive economizer/heat-recovery active-time rows. The compare lane derives
EnergyPlus `StdRhoAir` from `Site:Location`, applies the blank OA schedule as
always 1.0, and writes Rust `ResultStore` series for the 96 Detailed oracle
samples. The outdoor-air mass/volume, no-humidity latent, supply-air
mass/volume/humidity, and mixed-air rows are exact in this fixture; the
sensible/total report rows use a 1 W conformance tolerance, and supply-air
temperature uses 0.02 C because EnergyPlus sorts them from source-order zone/OA
state and report-rate mode gates. The guard requires Flow/Zone, blank OA
schedule, `NoEconomizer`, no heat recovery, no finite flow/capacity limits, no
DCV, default `ConstantSensibleHeatRatio` dehumidification, and no
humidification control. The inactive economizer/heat-recovery active-time rows
and inactive heat-recovery rate rows are exact-zero conformance rows inside
this declared candidate set.

`ideal_loads_outdoor_air_design_flow_diagnostic_001` remains the diagnostic
predecessor artifact for the same Flow/Zone fixture shape.

`ideal_loads_outdoor_air_flow_area_conformance_candidate_001` promotes the
Flow/Area proof lane. The fixture uses a 1 m2 typed floor surface area and
0.05 m3/s-m2 outdoor air, so the derived design volume is 0.05 m3/s before the
same `StdRhoAir` mass-flow conversion. The compare path derives the zone floor
area from typed floor surfaces and promotes the same 22 outdoor-air,
supply-air, mixed-air, inactive heat-recovery rate, and inactive active-time
rows as the Flow/Zone and Flow/Person candidates.

`ideal_loads_outdoor_air_flow_area_diagnostic_001` remains the diagnostic
predecessor artifact for the same Flow/Area fixture shape.

`ideal_loads_outdoor_air_air_changes_conformance_candidate_001` promotes the
AirChanges/Hour proof lane. The fixture uses 180 ACH over the explicit 1 m3
zone volume, so the derived design volume remains 0.05 m3/s before the same
`StdRhoAir` mass-flow conversion. The compare path derives the typed zone
volume and promotes the same 22 outdoor-air, supply-air, mixed-air, inactive
heat-recovery rate, and inactive active-time rows as the Flow/Zone,
Flow/Person, and Flow/Area candidates.

`ideal_loads_outdoor_air_air_changes_diagnostic_001` remains the diagnostic
predecessor artifact for the same AirChanges/Hour fixture shape.

`ideal_loads_outdoor_air_sum_conformance_candidate_001` promotes the Sum proof
lane. The fixture combines 0.015 m3/s Flow/Area, 0.025 m3/s Flow/Zone, and
0.010 m3/s AirChanges/Hour component terms to 0.05 m3/s, and the compare gate
checks each component term in `compare-summary.json`, `stage-summary.json`, and
`compare-report.md` before claiming the same 22 outdoor-air, supply-air,
mixed-air, inactive heat-recovery rate, and inactive active-time rows.
`ideal_loads_outdoor_air_sum_diagnostic_001` remains the diagnostic predecessor
artifact for the same Sum fixture shape.

`ideal_loads_outdoor_air_maximum_conformance_candidate_001` promotes the
Maximum proof lane. The fixture combines 0.015 m3/s Flow/Area, 0.025 m3/s
Flow/Zone, and 0.050 m3/s AirChanges/Hour component terms, then selects the
AirChanges/Hour term as the governing 0.05 m3/s design outdoor-air volume. The
compare gate checks each component term in `compare-summary.json`,
`stage-summary.json`, and `compare-report.md` before claiming the same 22
outdoor-air, supply-air, mixed-air, inactive heat-recovery rate, and inactive
active-time rows.
`ideal_loads_outdoor_air_maximum_diagnostic_001` remains the diagnostic
predecessor artifact for the same Maximum fixture shape.

`ideal_loads_outdoor_air_differential_dry_bulb_economizer_conformance_candidate_001`
promotes the Flow/Zone outdoor-air method with the minimum design flow lowered
to 0.001 m3/s so the cooling branch can exercise the EnergyPlus
`DifferentialDryBulb` economizer reset. The compare lane reports 110
source-order Detailed samples, including system substep active-time rows,
promotes the same 22 outdoor-air, supply-air, mixed-air, economizer
active-time, inactive heat-recovery rate, and inactive heat-recovery
active-time rows, and checks that economizer active time is nonzero and
outdoor-air mass flow rises above the design minimum.

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
same 22 outdoor-air, supply-air, mixed-air, economizer active-time, inactive
heat-recovery rate, and inactive heat-recovery active-time rows.

`ideal_loads_outdoor_air_sensible_heat_recovery_diagnostic_001` is the
diagnostic predecessor for the same Flow/Zone outdoor-air method with
`NoEconomizer` and `HeatRecoveryType = Sensible`.

`ideal_loads_outdoor_air_sensible_heat_recovery_conformance_candidate_001`
promotes that Sensible heat-recovery fixture shape for declared outdoor-air
mass/volume, no-humidity report-rate, supply-air state, mixed-air state,
Sensible heat-recovery rate, heat-recovery active-time, and inactive
economizer active-time rows. The Rust lane applies the EnergyPlus sensible
heat-recovery outdoor-air tempering branch when recirculation air can
beneficially warm or cool outdoor air, reports 96 Detailed samples, and
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
candidate also promotes the inactive economizer active-time zero row; general
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
timestep, broad meter conformance beyond the declared no-OA hourly,
humidity-control hourly/monthly/run-period, and meter-only monthly/annual/run-period
facility meter candidates, multi-year annual grouping, and
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
`ideal_loads_no_oa_facility_meter_conformance_candidate_001`, a narrow
hourly/monthly/run-period oracle-MTR vs Rust aggregated fuel-energy
conformance path in the four no-OA humidity-control conformance candidates,
and a narrow monthly/annual/run-period oracle-MTR vs Rust aggregated
fuel-energy conformance candidate in
`ideal_loads_no_oa_facility_meter_monthly_run_period_conformance_candidate_001`;
outside those cases, facility meter rows remain diagnostic. The no-OA
`ConstantSensibleHeatRatio`
cooling total/sensible/latent rows and supply-node humidity ratio have narrow
conformance evidence in `ideal_loads_constant_shr_conformance_001`; its
return-node and zone-air-node humidity proof rows remain diagnostic. The no-OA
`ConstantSupplyHumidityRatio` cooling fixture heating/cooling zone/supply latent and sensible rows,
ReportPurchasedAir energy/fuel rows, supply-node humidity ratio, and
hourly/monthly/run-period facility meter rows have narrow conformance evidence in
`ideal_loads_constant_supply_humidity_cooling_conformance_candidate_001`; its
return-node humidity, zone-air humidity, annual meter rows, other broader meter
frequencies, and broad meter behavior remain diagnostic. The no-OA
`ConstantSupplyHumidityRatio` heating fixture heating/cooling zone/supply latent and sensible rows,
ReportPurchasedAir energy/fuel rows, supply-node humidity ratio, and
hourly/monthly/run-period facility meter rows have narrow conformance evidence in
`ideal_loads_constant_supply_humidity_heating_conformance_candidate_001`; its
return-node humidity, zone-air humidity, annual meter rows, other broader
meter frequencies, and broad meter behavior remain diagnostic.
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
That run compares 36 Detailed ESO series plus six MTR facility meter series:
hourly, monthly, and run-period for both facility meters. The 29 declared ESO
conformance rows and the six conformance meter rows pass their tolerances, the
7 diagnostic proof ESO rows pass, and
`tolerance-failures.csv` is empty. This adds only the no-OA
ConstantSupplyHumidityRatio cooling claim for declared heating/cooling rate
outputs, ReportPurchasedAir energy/fuel rows, and hourly/monthly/run-period
facility meters; Humidistat, return-node and zone-air-node humidity proof rows,
annual meter rows, other broader meter frequencies, outdoor-air, economizer,
heat-recovery, and broad HVAC behavior remain outside the claim.

`scripts/dev.cmd compare-ideal-loads-constant-supply-humidity-heating-conformance-candidate`
generates the ConstantSupplyHumidityRatio heating conformance evidence set under
`.runtime/ideal-loads-constant-supply-humidity-heating-conformance/26.1.0/ideal_loads_constant_supply_humidity_heating_conformance_candidate_001/compare/`.
That run compares 36 Detailed ESO series plus six MTR facility meter series:
hourly, monthly, and run-period for both facility meters. The 29 declared ESO
conformance rows and the six conformance meter rows pass their tolerances, the
7 diagnostic proof ESO rows pass, and
`tolerance-failures.csv` is empty. This adds only the no-OA
ConstantSupplyHumidityRatio heating claim for declared heating/cooling rate
outputs, ReportPurchasedAir energy/fuel rows, and hourly/monthly/run-period
facility meters; Humidistat, return-node and zone-air-node humidity proof rows,
annual meter rows, other broader meter frequencies, outdoor-air, economizer,
heat-recovery, and broad HVAC behavior remain outside the claim.

`scripts/dev.cmd compare-ideal-loads-humidistat-dehumidification-conformance-candidate`
generates the Humidistat dehumidification conformance evidence set under
`.runtime/ideal-loads-humidistat-dehumidification-conformance/26.1.0/ideal_loads_humidistat_dehumidification_conformance_candidate_001/compare/`.
That run compares 38 Detailed ESO series plus six MTR facility meter series:
hourly, monthly, and run-period for both facility meters. The 31 declared ESO
conformance rows and the six conformance meter rows pass their tolerances, the
7 diagnostic proof ESO rows pass, and
`tolerance-failures.csv` is empty. This adds only the no-OA Humidistat
dehumidification claim for declared heating/cooling rate outputs,
ReportPurchasedAir energy/fuel rows, paired trace-driven moisture-demand rows,
and hourly/monthly/run-period facility meters; fully owned moisture-history
closure without EnergyPlus trace state, return-node and zone-air-node humidity
proof rows, annual meter rows, other broader meter frequencies, outdoor-air,
economizer, heat-recovery, and broad HVAC behavior remain outside the claim.

`scripts/dev.cmd compare-ideal-loads-humidistat-humidification-conformance-candidate`
generates the Humidistat humidification conformance evidence set under
`.runtime/ideal-loads-humidistat-humidification-conformance/26.1.0/ideal_loads_humidistat_humidification_conformance_candidate_001/compare/`.
That run compares 38 Detailed ESO series plus six MTR facility meter series:
hourly, monthly, and run-period for both facility meters. The 31 declared ESO
conformance rows and the six conformance meter rows pass their tolerances, the
7 diagnostic proof ESO rows pass, and
`tolerance-failures.csv` is empty. This adds only the no-OA Humidistat
humidification claim for declared heating/cooling rate outputs,
ReportPurchasedAir energy/fuel rows, paired trace-driven moisture-demand rows,
and hourly/monthly/run-period facility meters; fully owned moisture-history
closure without EnergyPlus trace state, return-node and zone-air-node humidity
proof rows, annual meter rows, other broader meter frequencies, outdoor-air,
economizer, heat-recovery, and broad HVAC behavior remain outside the claim.

## CP237 `ManageZoneEquipment` Parent Contract

CP237 expands the already required `routine.manage_zone_equipment` entry; it
does not add a routine, source file, or HVAC project item.
`ManageZoneEquipment` is declared at `ZoneEquipmentManager.hh` lines 82-86 and
implemented completely at `ZoneEquipmentManager.cc` lines 141-167. Its mutable
interface takes `FirstHVACIteration` by value and caller-owned `SimZone` and
`SimAir` booleans by reference.

### Exact order, branch, and writes

Every reached invocation performs this order:

1. call `InitZoneEquipment(state, FirstHVACIteration)`;
2. read the current global `ZoneSizingCalc`;
3. when true, call `SizeZoneEquipment(state)`;
4. otherwise call
   `SimZoneEquipment(state, FirstHVACIteration, SimAir)` and only after that
   returns set shared `ZoneEquipSimulatedOnce = true`;
5. call `UpdateZoneEquipment(state, SimAir)`; and
6. only after Update returns set the caller's `SimZone = false`.

Incoming `SimZone` is never read, so false input does not suppress any work.
`FirstHVACIteration` reaches Init and only the non-sizing Sim child. The parent
never clears or otherwise assigns `SimAir`: the non-sizing branch passes it
through Sim and then Update, while the sizing branch passes it only to Update.
The children may raise it when air-loop interface state requires another
simulation, and an already true value remains true.

The direct children remain separate source boundaries. Init owns one-time
Zone/Space demand-sequence allocation, begin-environment resets, every-call
HVAC-timestep initialization, and air-loop aggregate clearing. Size owns its
one-time sizing arrays and ordered Zone/Space sizing, mass-balance, and leaving
conditions. Sim owns supply paths, simple airflow, mixers, ordered Zone
equipment and residual loads, reverse paths, exhaust/mass/leaving/duct/return
work. Update walks return interfaces and passes `SimAir` by reference into
their convergence update. CP237 maps only the parent orchestration and does
not promote those complete child implementations.

### Failure, replay, and reset

The wrapper performs no local input, allocation, count, availability, or flag
validation and has no diagnostic, status result, catch, cleanup, transaction,
or rollback. Failure in Init prevents the branch, Update, and final flag
write. Failure in Size prevents Update and the final write. Failure in Sim
prevents the simulated-once assignment, Update, and the final write. A
non-sizing Update failure occurs after `ZoneEquipSimulatedOnce` has become
true but before `SimZone` becomes false; a sizing Update failure similarly
preserves the caller's prior `SimZone`.

Retry is unconditional because neither incoming `SimZone` nor
`ZoneEquipSimulatedOnce` gates entry. Init clears its one-time flag before its
first allocation, so abnormal exit can leave partial initialization that a
same-state retry skips. Its environment flag clears only after the complete
environment block and rearms while `BeginEnvrnFlag` is false. Size clears its
one-time flag only after setup returns, and Sim clears its first-pass flag late
in its body. The parent is not generally idempotent because the children reset
timestep fields, simulate equipment and residual demand, update histories and
nodes, and accumulate or overwrite child-owned state.
`ZoneEquipSimulatedOnce` defaults false, is written here only on a completed
non-sizing Sim child, is not reset per environment, and returns to false only
when the owning `DataZoneEquipment` state is reconstructed. The manager's own
one-time flags default true and are restored by its full state clear.

### Production callers and evidence

There are seven production call expressions. `HVACManager::SimHVAC` invokes
the parent in its `ZoneSizingCalc` path. `HVACManager::SimSelectedEquipment`
has a begin-environment prepass, the unconditional first-HVAC-iteration call,
and its later `SimZoneEquipment`-guarded iteration call. `SizingManager` has
two `ManageSizing` calls plus one system-sizing-adjustment call that
temporarily establishes its environment state. The wrapper itself never calls
`GetZoneEquipment`; that is the next separate source routine.

Nine direct C++ invocations occur across eight tests: four PurchasedAir
fixtures, two `HVACUnitaryBypassVAV` fixtures, one packaged-terminal
heat-pump fixture that calls twice, and one UnitHeater fixture. Eight pass
`FirstHVACIteration = true` and one passes false. All retain
`ZoneSizingCalc = false`, so none composes the sizing branch. Their assertions
cover descendant equipment, plenum, node, capacity, or residual-load effects,
not exact parent order, `SimZone`, `SimAir`, `ZoneEquipSimulatedOnce`,
failure, replay, or reset. Separate direct child coverage has six Size calls
across three tests and four Sim calls across two tests, but no direct Init or
Update call and no composition of the CP237 parent contract.

The active full-simulation corpus has 57 `ManageSimulation` expressions. One
expected EMS fatal stops before HVAC. Each of the other 56 successful
expressions reaches at least one CP237 production call, establishing a
conservative lower bound of 56 executions. The eight sizing-only
`SizingManager_ZoneSizing_*` fixtures reach the `SimHVAC`
`ZoneSizingCalc` expression. Begin-environment prepasses, per-expression
repetition, HVAC iterations, warmup/timesteps, and additional sizing calls can
increase the dynamic count and are not instrumented.

Across the corpus, 34 completing configurations, including the eight
sizing-only fixtures, contain 50 `Sizing:Zone` definitions and provide static
sizing-branch potential; the dynamic Size call count remains uninstrumented.
The 55 zoned successful configurations contain 81 Zone
identities, 55 controlled and 26 uncontrolled. That is child-topology context,
not 81 parent calls. No active full-simulation assertion isolates the
wrapper-owned protocol.

### Rust boundary and status

`ideal_loads_zone_equipment_stages()` returns three constant metadata records
for Manage, SimZone, and SimPurchasedAir. The typed validator checks only the
IdealLoads equipment graph, connections, nodes, and sequence. `ExecutionPlan`
emits Manage and Sim labels for Zones that own IdealLoads and groups later
PurchasedAir stages. The compatibility runtime then loops prebound systems,
validates each, creates a demand snapshot, and invokes PurchasedAir wrappers
directly.

Rust therefore has no exact CP237 parent, `FirstHVACIteration` lifecycle,
`ZoneSizingCalc` switch, Size branch, Init/Update orchestration, caller-owned
`SimZone`/`SimAir` protocol, `ZoneEquipSimulatedOnce` lifecycle, complete
multi-family Zone equipment dispatch, failure-prefix behavior, or reset/retry
test. Its validator and execution-plan tests prove graph and metadata
properties, not this routine. CP237 adds no Rust target, code, mapped state,
test, support, capability, output implementation, comparator, case, manifest,
numerical, performance, or conformance promotion. The inventory remains 32
algorithms and 242 routines, split 58 `state_mapped` plus 184
`source_mapped`, with 119 required; the heat-balance and HVAC project lists
remain 88 and 8.

## CP238 `GetZoneEquipment` One-Time Input Barrier

CP238 adds `routine.get_zone_equipment` as required and `source_mapped` after
`manage_zone_equipment` and before `sim_zone_equipment`. The HVAC project list
gets the same ordered item. The algorithm already cites
`ZoneEquipmentManager.cc`, so no algorithm-level source or Rust target changes.
`GetZoneEquipment` is declared at `ZoneEquipmentManager.hh` line 88 and its
complete body is `ZoneEquipmentManager.cc` lines 169-197.

### Exact gate, order, and stored state

`GetZoneEquipmentInputFlag` guards every body operation. False input returns
without calling a child, reading current timestep or Zone counts, validating
state, or changing any stored value. A true entry performs this exact order:

1. call `DataZoneEquipment::GetZoneEquipmentData(state)`;
2. set manager-owned `GetZoneEquipmentInputFlag = false`;
3. set separately owned `ZoneEquipInputsFilled = true`;
4. set `NumOfTimeStepInDay` to integer
   `TimeStepsInHour * Constant::iHoursInDay`;
5. initialize local `MaxNumOfEquipTypes = 0`;
6. scan Zone indexes one through `NumOfZones`, skip each configuration whose
   `IsControlled` is false, and take the maximum same-index
   `ZoneEquipList(Counter).NumOfEquipTypes`; and
7. allocate `PrioritySimOrder` to that maximum.

`Constant::iHoursInDay` is 24 and all three arithmetic values are integers.
Normal input validation constrains `TimeStepsInHour`, but CP238 performs no
local positivity, divisor, range, or overflow check and direct state can retain
literal invalid values. The result is a one-time snapshot: later changes to
`TimeStepsInHour` do not update it.

For valid source input, the child allocates both Zone configuration and
equipment-list arrays by `NumOfZones` and stores each parsed list at its actual
Zone index, so the same-index maximum scan is intentional. Only controlled
Zones contribute. Zero controlled Zones or zero equipment produces a maximum
of zero and the source still calls the allocation API with extent zero.
Allocated `SimulationOrder` elements default to empty names, Invalid equipment
type, and zero pointer/priorities. CP238 does not populate or sort them; later
`SetZoneEquipSimOrder` fills leading entries for the active Zone and clears
unused-tail names, equipment type, and pointer; unused-tail priorities remain
untouched.

### Dependency, caller, and lifecycle

`DataZoneEquipment::GetZoneEquipmentData`, separately declared at
`DataZoneEquipment.hh` line 558 and implemented at
`DataZoneEquipment.cc` lines 167-812, remains dependency context rather than a
new mapped routine. It owns the full Zone/Space equipment connection and list,
node, schedule, supply/return path, splitter/mixer, allocation, duplicate, and
fatal-input behavior. Its final sticky-error fatal occurs before CP238 can
clear its guard or publish readiness.

There is exactly one production call expression, at
`SurfaceGeometry.cc` line 298 inside `SetupZoneGeometry`. It follows
`GetSurfaceData`; if `ErrorsFound` is true after that call, lines 292-296 return
and suppress CP238. A reached call precedes window-gap airflow and storm-window
input. The transitive input chain comes from
`HeatBalanceManager::GetBuildingData`, not HVAC simulation:
`GetZoneData -> SetupZoneGeometry -> GetZoneEquipment`.
`ManageZoneEquipment` does not call it, and the old HVACManager comment that
the manager call forces input acquisition is explicitly marked incorrect in
the source.

Manager state defaults to day count zero, guard true, and an empty priority
array. DataZoneEquipment separately defaults readiness and its input-error
latch false. Neither flag rearms per environment. The manager clear
reconstructs the former state, while the DataZoneEquipment clear reconstructs
the latter; resetting only one owner can create a true guard with stale
readiness/data or a false guard with cleared readiness. Full state clear
restores the coordinated initial pair.

### Failure, replay, and evidence

The wrapper owns no input, range, arena, allocation, or consistency validation,
no `ErrorsFound` parameter or status result, and no local diagnostic, catch,
cleanup, transaction, or rollback. If the child fatals, the manager guard
remains true; readiness, day count, and priority state remain at their entry
values (readiness is false on a fresh-state entry). The child can still retain
partial allocations, node/schedule mutations, input-used markers, diagnostics,
and its sticky error flag. A caught retry is therefore not a clean parse.

After the child returns, line 182 clears the guard before readiness, arithmetic,
the scan, and allocation. Failure in that suffix preserves the completed
prefix. In particular, failure after line 183 can advertise readiness while
the day count or priority scratch remains incomplete; every later call then
silently no-ops and cannot repair it. A successful replay is narrowly
idempotent only because it performs no work and never revalidates, recomputes,
or resizes.

Twenty-three direct C++ calls occur across 22 tests. Twenty-one tests use one
call as downstream setup. The focused
`ZoneEquipmentManager_GetZoneEquipmentTest` calls twice: it proves the
default-true guard, a normal first return with guard false and populated
configuration/Zone node, `TimeStepsInHour = 1` producing a stored day count of
24, then a second no-op after changing the source value to 2 while 24 remains.
It does not assert `ZoneEquipInputsFilled`, priority extent or default content,
zero-Zone behavior, validation/failure prefixes, retry, or coordinated reset.
A UnitHeater full simulation later checks two priority entries only after the
later ordering routine has populated them, so it does not isolate CP238
allocation.

All 57 active `ManageSimulation` expressions traverse the input chain and
complete CP238 once on their fresh state. This includes the intentional EMS
fatal fixture: its zero-equipment child returns normally and CP238 commits
before the later first EMS calling point. Fifty-six expressions subsequently
complete the simulation. The static corpus spans controlled, uncontrolled,
and zero-Zone input topology, but no active assertion isolates CP238's guard,
readiness publication, day snapshot, maximum extent, failure, or reset.

### Rust boundary and status

Rust owns an eager immutable compiler subset for
`ZoneHVAC:EquipmentList` and `ZoneHVAC:EquipmentConnections`, but its equipment
entry enum supports only `ZoneHVAC:IdealLoadsAirSystem`. It normalizes typed
identities, validates references and sequences, projects IdealLoads graph
edges, and builds execution labels at compile/plan time. That timing, state,
family coverage, and failure model do not implement the lazy source wrapper or
the full child parser.

Rust `TimestepConfig` and time-axis construction separately perform adjacent
24-times-timesteps-per-hour arithmetic using guarded unsigned operations.
They do not own the equipment-manager day snapshot or its second-call freeze.
The typed graph's static edge sort is not source `PrioritySimOrder`; CP238 only
allocates that scratch, and later source demand-dependent ordering fills it.
Rust has no `GetZoneEquipmentInputFlag`, `ZoneEquipInputsFilled`, coordinated
one-shot owners, full Zone/Space configuration, controlled-Zone maximum scan,
`SimulationOrder` scratch defaults/allocation, exact partial-failure prefix, or
retry/reset test.

CP238 adds no algorithm-level EnergyPlus source, Rust target, executable code,
mapped state, test, object support, capability, output implementation,
comparator, case, manifest, numerical, performance, or conformance promotion.
The inventory becomes 32 algorithms and 243 routines, split 58 `state_mapped`
plus 185 `source_mapped`, with 120 required; the heat-balance and HVAC project
lists become 88 and 9.

CP239 next maps `ZoneEquipmentManager::InitZoneEquipment`, declared at
`ZoneEquipmentManager.hh` line 90 and implemented at
`ZoneEquipmentManager.cc` lines 199-316.

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
