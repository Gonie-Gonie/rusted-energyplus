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
| `ZoneEquipmentManager::InitZoneEquipment` | `src/EnergyPlus/ZoneEquipmentManager.cc` | no exact Rust target; adjacent `ZoneSysEnergyDemand`, diagnostic node state, and time-axis begin-environment metadata only |
| `ZoneEquipmentManager::sizeZoneSpaceEquipmentPart1` | `src/EnergyPlus/ZoneEquipmentManager.cc` | no exact Rust target; adjacent fixed-option `ZoneSysEnergyDemand` snapshot, IdealLoads design supply limits, psychrometric helpers, and narrow node updates only |
| `ZoneEquipmentManager::sizeZoneSpaceEquipmentPart2` | `src/EnergyPlus/ZoneEquipmentManager.cc` | no exact Rust target; adjacent typed return-node identities, diagnostic node temperatures, and constant thermostat schedule reports only |
| `ZoneEquipmentManager::SizeZoneEquipment` | `src/EnergyPlus/ZoneEquipmentManager.cc` | no exact Rust target; existing three-label stage metadata skips the sizing parent and Rust blocks `Sizing:Zone` before runtime |
| `ZoneEquipmentManager::CalcDOASSupCondsForSizing` | `src/EnergyPlus/ZoneEquipmentManager.cc` | no exact Rust target; adjacent PurchasedAir outdoor-air supply logic and psychrometrics are not the `Sizing:Zone` DOAS selector |
| `ZoneEquipmentManager::SetUpZoneSizingArrays` | `src/EnergyPlus/ZoneEquipmentManager.cc` | no exact Rust target; adjacent typed Zone, Space, thermostat, equipment, individual DSOA, and ordinary SpaceList structures do not implement the `Sizing:Zone` setup transaction |
| `ZoneEquipmentManager::calcSizingOA` | `src/EnergyPlus/ZoneEquipmentManager.cc` | no exact Rust target; adjacent typed Zone/Space/People, schedules, individual DSOA, and PurchasedAir OA helpers do not implement the mutable sizing, effectiveness, SpaceList-validation, report, or design-day fanout contract |
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
  -> if ZoneSizingCalc
       -> SizeZoneEquipment
            -> if SizeZoneEquipmentOneTimeFlag
                 -> SetUpZoneSizingArrays
                      -> conditional AllocateIntGains
                      -> validate Sizing:Zone and VerifyThermostatInZone
                      -> AutoCalcDOASControlStrategy
                      -> allocate/fill Zone and optional Space sizing plus EMS
                      -> populate DSOA SpaceList indexes
                      -> calcSizingOA for controlled Zone/Space roles
                           -> validate positive DSOA/SpaceList membership and derive OA rates
                           -> accumulate multiplier-scaled People/area and Zone-only VozMin
                           -> store equipment OA/air-distribution indexes
                           -> calculate and effectiveness-adjust MinOA
                           -> scale role flow limits and fan five fields into Zone day arrays
                      -> sizing-factor EIO and late accumulated-error fatal
                 -> SizeZoneEquipmentOneTimeFlag = false
            -> sizeZoneSpaceEquipmentPart1 (complete Zone/optional-Space pass)
                 -> per role if AccountForDOAS
                      -> CalcDOASSupCondsForSizing
            -> CalcZoneMassBalance
            -> CalcZoneLeavingConditions
            -> sizeZoneSpaceEquipmentPart2 (complete Zone/optional-Space pass)
     else
       -> SimZoneEquipment
            -> PurchasedAirManager::SimPurchasedAir
            -> PurchasedAirManager::InitPurchasedAir
            -> PurchasedAirManager::CalcPurchAirLoads
            -> PurchasedAirManager::UpdatePurchasedAir
            -> PurchasedAirManager::ReportPurchasedAir
       -> ZoneEquipSimulatedOnce = true
  -> UpdateZoneEquipment
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

## CP239 `InitZoneEquipment` Multi-Cadence Initializer

CP239 adds required `routine.init_zone_equipment` after
`routine.get_zone_equipment` and before `routine.sim_zone_equipment`, plus the
same ordered HVAC project item. `InitZoneEquipment` is declared at
`ZoneEquipmentManager.hh` line 90 and its complete implementation is
`ZoneEquipmentManager.cc` lines 199-316. The algorithm already cites
`ZoneEquipmentManager.cc`, so no algorithm-level source or Rust target changes.

### Caller and cadence regions

The sole direct production call expression is the unconditional
`InitZoneEquipment(state, FirstHVACIteration)` at `ManageZoneEquipment` line
155. It runs before that parent tests `ZoneSizingCalc`, so both sizing and
ordinary simulation traverse CP239. It is not called by `GetZoneEquipment`,
does not acquire input itself, and any abnormal exit suppresses the parent's
Size-or-Sim child, Update child, and final `SimZone = false`. The `unused 1208`
comments beside the declaration and definition are stale.

The body has three ordered cadence regions:

1. an `InitZoneEquipmentOneTimeFlag` allocation block;
2. an `InitZoneEquipmentEnvrnFlag && BeginEnvrnFlag` environment block,
   followed by the independent false-BeginEnvironment rearm; and
3. an every-call HVAC-timestep and primary-air-loop reset suffix.

### One-time demand and sizing storage

When `InitZoneEquipmentOneTimeFlag` is true, line 210 clears it before any
allocation. Line 211 then allocates `ZoneEqSizing` to `NumOfZones`, replacing
all entries with default sizing records. The ascending Zone loop from one
through `NumOfZones` skips an uncontrolled configuration and independently
skips a controlled configuration whose `EquipListIndex` is zero.

For every retained Zone, CP239 reads
`ZoneEquipList(EquipListIndex).NumOfEquipTypes` and performs this exact order:

1. write sensible `NumZoneEquipment`;
2. allocate sensible `SequencedOutputRequired`,
   `SequencedOutputRequiredToHeatingSP`, then
   `SequencedOutputRequiredToCoolingSP`;
3. write moisture `NumZoneEquipment`;
4. allocate moisture `SequencedOutputRequired`,
   `SequencedOutputRequiredToHumidSP`, then
   `SequencedOutputRequiredToDehumidSP`; and
5. allocate `ZoneEqSizing(Zone).SizingMethod` to
   `HVAC::NumOfSizingTypes`, which is 35, then fill it with zero.

The six `EPVector` allocations provide zero-valued elements, but CP239
populates no load sequence. Uncontrolled and zero-list-pointer Zones retain
only their default outer sizing record and receive none of the selected-Zone
demand writes.

If either `doSpaceHeatBalanceSimulation` or
`doSpaceHeatBalanceSizing` is true, CP239 then visits that selected parent
Zone's stored `spaceIndexes` without testing the corresponding Space
configuration's controlled flag. Each member Space receives the same equipment
count and the same six sensible-then-moisture sequence allocations. It receives
no Space sizing-method array. This stored-membership traversal differs from the
later full Space-configuration traversals.

### Environment availability and node state

The begin-environment block first assigns every current `ZoneEquipAvail` entry,
including uncontrolled slots, to `Avail::Status::NoAction`. If `ZoneComp` is
allocated, CP239 visits exactly component types 1 through
`NumValidSysAvailZoneComponents`, which is 14. For each type with an allocated
`ZoneCompAvailMgrs` array, it visits component indexes one through
`TotalNumComp` and resets only `availStatus`, `StartTime`, and `StopTime` to
NoAction/zero/zero. Names, manager lists, Zone identity, input/count state, and
other fields remain unchanged.

CP239 next range-visits the complete Zone configuration array and calls
`EquipConfiguration::beginEnvirnInit` only for controlled records. Only when
`doSpaceHeatBalanceSimulation` is true, not for sizing alone, it similarly
visits the complete Space configuration array and calls the same child for
controlled records. The child is declared at `DataZoneEquipment.hh` line 375
and implemented at `DataZoneEquipment.cc` lines 2234-2301; it remains dependency
context rather than a new ledger row.

For each configuration the child visits its Zone node, then inlet nodes, then
exhaust nodes, and finally return nodes only when `NumReturnNodes > 0`. Every
visited node receives temperature 20 C, mass flow zero, quality one, current
outdoor barometric pressure and humidity ratio, and enthalpy from
`PsyHFnTdbW(20, OutHumRat)`. It copies outdoor CO2 and generic contaminant only
when each simulation flag is active, so disabled contaminant fields are left
unchanged. It does not reset every node member or the flow-availability bounds.

Only after all availability and node work returns does line 283 clear
`InitZoneEquipmentEnvrnFlag`. A later reached CP239 call while
`BeginEnvrnFlag` is false sets that flag true. Thus repeated calls during one
still-true BeginEnvironment interval skip successful environment work, but a
new environment is recognized only if CP239 ran during an intervening false
interval.

### Every-call HVAC and air-loop reset

Every CP239 call range-visits all Zone configurations and invokes
`EquipConfiguration::hvacTimeStepInit` on controlled records. Space
configurations receive the same call only during Space heat-balance simulation.
This child is declared at `DataZoneEquipment.hh` line 377 and implemented at
`DataZoneEquipment.cc` lines 2303-2327.

The child first accesses its configuration Zone node and always sets
configuration `ExcessZoneExh = 0`. Only when `FirstHVACIteration` is true does
it visit exhaust nodes, copy Zone-node temperature, humidity ratio, enthalpy,
pressure, and quality, zero `MassFlowRate`, `MassFlowRateMaxAvail`, and
`MassFlowRateMinAvail`, and conditionally copy active contaminants. It does not
touch inlet or return nodes, absolute max/min flows, or every node field.
`FirstHVACIteration` affects no other CP239 branch.

Finally the ascending loop over one through `NumPrimaryAirSys` zeros exactly
six `AirLoopFlow` aggregates in order: `SupFlow`, `ZoneRetFlow`, `SysRetFlow`,
`RecircFlow`, `LeakFlow`, and `ExcessZoneExhFlow`. Design, outdoor-air,
previous-flow, ratio, fan, and other air-loop fields remain intact.

### Failure, replay, and reset

Both manager latches default true and manager `clear_state()` reconstructs
them. The one-time latch never naturally rearms. The environment latch rearms
only through a reached CP239 call with `BeginEnvrnFlag` false. Sizing, demand,
equipment, availability, node, contaminant, and air-loop state are owned and
cleared separately, so resetting the manager alone neither undoes CP239's
writes nor restores a coordinated initial state.

CP239 has no local topology, membership, count, bounds, allocation, node,
finite-value, or cross-owner validation and no assertion, diagnostic, status,
catch, cleanup, transaction, or rollback. It assumes the input/configuration,
demand, sizing, availability, node, and air-loop arenas agree.

Because the one-time latch clears first, failure from the outer sizing
allocation onward leaves an ordered partial prefix and same-state retry skips
all unfinished one-time work. Environment failure before line 283 leaves its
latch true, so retry repeats availability and earlier node overwrites. Failure
in the every-call suffix after line 283 leaves the environment latch false, so
retry during that same BeginEnvironment interval does not replay environment
initialization. On a false-BeginEnvironment call, rearm precedes the timestep
suffix and survives a later failure.

A configuration-child failure suppresses later configurations and all air-loop
resets. Air-loop failure leaves earlier loops zero and later loops stale.
Retries repeat earlier configuration mutations. Even successful calls are not
globally pure: every call clears excess exhaust and six air-loop aggregates,
first-iteration calls overwrite exhaust nodes from current configuration node
state, and manually rearming the one-time latch reallocates stored demand and
sizing data. Traversals neither sort nor deduplicate, so malformed shared
identities can be visited repeatedly.

### C++ and active-corpus evidence

No C++ unit test directly calls `InitZoneEquipment`,
`EquipConfiguration::beginEnvirnInit`, or
`EquipConfiguration::hvacTimeStepInit`. Nine non-sizing direct
`ManageZoneEquipment` calls across eight tests enter CP239 indirectly. Eight
pass `FirstHVACIteration = true`; the later UnitHeater call passes false after
an earlier full simulation. The PTHP/plenum test contains two calls under the
same true BeginEnvironment value, but its assertions target descendant
equipment/plenum behavior.

Across those eight contexts there are zero assertions on either CP239 latch,
`ZoneEqSizing`, equipment counts or sequence storage, availability status or
times, excess exhaust, or the six reset air-loop fields. Node and equipment
assertions occur only after descendants have run and do not isolate CP239's
node-reset order. There is no failure, retry, rearm, reset, sizing-branch,
Space-allocation, or zero-Zone focused test.

The broader active unit corpus contains 115 call expressions in five ancestor
categories across 95 unique test contexts: direct parent, `ManageSizing`,
`SetupSimulation`, `ManageHeatBalance`, and `ManageSimulation`. Categories
overlap and are not runtime-call counts. One plant-only `ManageSizing`
expression does not itself reach CP239 but its later `SetupSimulation` does.
The intentional EMS-fatal `ManageSimulation` expression stops before HVAC and
is the sole context that never reaches CP239 by any path.

The other 56 active full simulations reach CP239 at least once: 55 contain
Zones and one WeatherManager fixture is a zero-Zone run whose fresh one-time
block still allocates zero-length `ZoneEqSizing`. Thirty-four first reach CP239
during requested Zone sizing and the other 22 first reach it during setup; the
one-time block completes exactly once per fresh successful state.

Twenty-one of those configurations also request system sizing.
`ManageSystemSizingAdjustments` forces `BeginEnvrnFlag` true, reaches CP239,
then sets the global flag false without another CP239 call. CP239's environment
latch therefore remains false, so the next setup environment's first true-Begin
call suppresses environment initialization; a later false-Begin call rearms it.
This source-order edge is unasserted.

Seven sizing-Space configurations statically reach one-time sequence allocation
for 21 Space identities under the simulation-or-sizing condition. The sole
simulation-Space configuration has three uncontrolled Spaces, so the active
corpus yields zero controlled Space `beginEnvirnInit` or `hvacTimeStepInit`
child entries. Exact warmup, multi-environment, system-timestep, later
HVAC-iteration, equipment-list, and air-loop multiplicity remains
uninstrumented, and no full-simulation assertion isolates CP239-owned state.

### Rust boundary and status

Rust's typed equipment lists, connections, and graph edges provide immutable
IdealLoads-only identities and sequences, not CP239 storage or lifecycle.
Rust `ZoneSysEnergyDemand` is a copied four-scalar sensible/moisture snapshot;
it has no equipment count, sequenced arrays, Zone moisture arena, Space
counterpart, or `ZoneEqSizing`. `NodeStateStore` is explicitly diagnostic and
stores only typed identity, temperature, humidity ratio, mass flow, and
temperature setpoint; it has no role-specific environment/timestep protocol,
pressure, quality, enthalpy, contaminants, or flow-availability bounds.

The Rust time axis precomputes `begin_environment` metadata but owns no mutable
rearm latch. `IdealLoadsInitFlags` mirrors `InitPurchasedAir`, not
`InitZoneEquipment`. Availability-manager identities have no runtime
status/start/stop lifecycle, and static AirLoopHVAC graph state has no six
aggregate-flow fields. Existing psychrometric helpers are not wired into this
node-reset path. Stage metadata also omits CP239.

CP239 adds no algorithm-level EnergyPlus source, Rust target, executable code,
mapped state, test, object support, capability, output implementation,
comparator, case, manifest, numerical, performance, or conformance promotion.
The algorithm remains `scaffold` with claim level `none`. The inventory becomes
32 algorithms and 244 routines, split 58 `state_mapped` plus 186
`source_mapped`, with 121 required; the heat-balance and HVAC project lists
become 88 and 10.

## CP240 `sizeZoneSpaceEquipmentPart1` Sizing Load Projection

CP240 adds canonical required
`routine.size_zone_space_equipment_part1` after
`routine.init_zone_equipment` and before `routine.sim_zone_equipment`, plus the
same ordered HVAC project item. The exact lowercase routine is declared at
`ZoneEquipmentManager.hh` lines 92-99 and implemented at
`ZoneEquipmentManager.cc` lines 317-597. The algorithm already cites that
source file, so no algorithm-level source or Rust target changes.

### Parent traversal and argument identity

The only two production call expressions are inside
`SizeZoneEquipment`: the Zone call at line 660 and the Space-loop call at lines
663-670. `ManageZoneEquipment` selects that parent only when
`ZoneSizingCalc` is true. The normal sizing path is therefore
`HVACManager::SimHVAC` lines 826-833 to `ManageZoneEquipment`, then CP239 Init,
then `SizeZoneEquipment`, then CP240. The separate post-Zone-sizing
`ManageZoneEquipment` calls in `SizingManager` occur after
`ZoneSizingCalc = false` and do not enter CP240.

`SizeZoneEquipment` first calls `SetUpZoneSizingArrays` under its own one-time
flag and clears that flag only after setup returns. It then visits Zone indexes
ascending, skips configurations whose `IsControlled` is false, calls CP240 once
for the Zone, and, when the current aggregate `doSpaceHeatBalance` flag is
true, calls CP240 for every stored `Zone.spaceIndexes` member without checking
the Space configuration's controlled flag. During sizing,
`HeatBalanceManager` assigns that aggregate flag from
`doSpaceHeatBalanceSizing`.

Only after every Zone and Space CP240 call returns does the parent call
`CalcZoneMassBalance(state, true)`, then
`CalcZoneLeavingConditions(state, true)`, and then perform its separate
Zone/Space Part2 pass. A CP240 abnormal non-return suppresses that entire
suffix and the remainder of the current traversal.

The Zone call binds Zone configuration, `CalcZoneSizing`, Zone sensible and
moisture demand, parent `ZoneData`, and the default `spaceNum = 0`. A Space call
binds Space configuration, `CalcSpaceSizing`, Space demand, Space heat-balance
records, and the Space system node. It nevertheless still passes the parent
`ZoneData` as `zoneOrSpace` and the parent `zoneNum`. Thus Space calls use
parent-Zone deadband, cooling ITE adjustment, multipliers, and
`CalcFinalZoneSizing(zoneNum).MinOA`; replacing that argument with `SpaceData`
would change the source behavior.

### Demand reset and no-DOAS snapshots

Every call selects Zone or Space `NonAirSystemResponse` and `SysDepZoneLoads`
by `spaceNum > 0`, zeros both, selects the corresponding system Zone node, and
then calls

```text
initOutputRequired(state, zoneNum, energy, moisture, true, false, spaceNum)
```

The child resets twelve remaining/unadjusted sensible and moisture scalars from
their total and setpoint outputs. In the production sizing path, allocated
sequence arrays are filled with the six full demands because
`ZoneSizingCalc = true`; CP240 does not allocate them and passes
`ResetSimOrder = false`. Every Zone and Space entry also restores shared
`CurDeadBandOrSetback(zoneNum)` from original
`DeadBandOrSetback(zoneNum)`.

After the child returns, CP240 snapshots sensible and latent remaining outputs
before any DOAS effect. Original `DeadBandOrSetback(zoneNum)` forces only the
sensible no-DOAS snapshot to zero. The latent no-DOAS snapshot survives only
when the original humidifying and dehumidifying setpoint outputs are both
strictly positive or both strictly negative; zero, mixed-sign, or
opposite-side values force it to zero.

### Dedicated-outdoor-air prefix

When `AccountForDOAS` is true, at least one inlet is mandatory. Two or more
inlets select inlet 1 for DOAS and inlet 2 for the residual sizing load; exactly
one selects inlet 1 for DOAS and leaves the residual destination at zero. A
nonpositive inlet count emits a severe plus continuation and then fatal before
any DOAS psychrometric calculation.

CP240 computes 90-percent-RH humidity ratios at the configured high and low
setpoints using standard barometric pressure, then sets

```text
DOAS mass = CalcFinalZoneSizing(zoneNum).MinOA * StdRhoAir
sensible  = mass * PsyCpAirFnW(supply W) * (supply T - selected-node T)
total     = mass * (PsyHFnTdbW(supply) - PsyHFnTdbW(selected node))
latent    = mass * (supply W - selected-node W), only for latent sizing
```

`CalcDOASSupCondsForSizing` owns the control-strategy selection and remains a
separate dependency. CP240 passes the DOAS sensible and optional latent mass
outputs to `updateSystemOutputRequired`, then writes only temperature, humidity
ratio, mass flow, and enthalpy to inlet 1. It records sensible heat addition,
total-minus-sensible latent addition, supply state, and raw positive-heat versus
nonpositive-cooling load fields; exactly zero sensible output takes the
cooling/else branch. `DOASLatAdd` is written even when latent sizing is off.

When `AccountForDOAS` is false, the residual sizing load uses inlet 1 when
present and otherwise uses the non-air path. CP240 does not clear any of the
eight DOAS sizing fields on this branch, so values from an earlier call can
remain stale. In a one-inlet DOAS call, inlet 1 receives DOAS while the residual
load deliberately follows the non-air path.

### Sensible and latent sizing calculations

The main sensible calculation runs only when the original Zone deadband flag is
false and the absolute remaining load exceeds `HVAC::SmallLoad`, 1 W. Cooling
chooses configured supply temperature or a negative absolute design
temperature difference. Only cooling can use adjusted ITE return temperature,
and only after `BeginSimFlag` clears: the explicit-temperature method changes
the denominator delta while retaining configured supply temperature, whereas
the temperature-difference method changes the resulting supply temperature.
Heating uses the analogous configured temperature or positive absolute
difference without ITE adjustment.

CP240 computes supply enthalpy and specific heat, and when the absolute
temperature delta exceeds `HVAC::SmallTempDiff`, 1e-5 C, sets mass flow to the
nonnegative load-over-`Cp*DeltaT` quotient. A
`SupplyAirAdjustFactor` multiplies it only when strictly greater than one. A
deadband, small-load, or too-small-delta path instead retains zero main output
or mass flow as applicable. Heat and cooling load/mass fields are overwritten
by sign, and both heating and cooling Zone/outdoor temperature and humidity
snapshots are always written.

Latent work is independently gated by `zoneLatentSizing`. It uses the original
humidifying/dehumidifying setpoint outputs' strict same-sign test to select the
current remaining moisture load. Cooling and heating choose configured supply
humidity ratios or authored signed differences; no absolute-value normalization
is applied. Only an absolute humidity-ratio difference exceeding
`HVAC::VerySmallMassFlow`, 1e-30, yields a nonnegative latent mass flow.

If sensible mass flow is positive, CP240 recomputes supply humidity ratio,
specific heat, temperature, and enthalpy so the same flow serves both loads. If
sensible flow is zero but latent flow is positive, it retains the current
temperature and recomputes humidity ratio and enthalpy. It promotes latent flow
to the main flow only when that flow also exceeds
`HVAC::VerySmallMassFlow`, 1e-30, and otherwise sets the main flow to zero. It
then writes four sign-selected latent load/mass fields and
four no-DOAS sensible/latent load fields. When latent sizing is false, all eight
of those fields remain untouched rather than being reset.

The invoked psychrometric dependencies also remain part of the exact boundary:
`PsyHFnTdbW` and `PsyCpAirFnW` floor humidity ratio at 1e-5, the latter uses
its source one-value cache, and `PsyHgAirFnWTdb` ignores its humidity-ratio
argument and evaluates `2500940 + 1858.95 * T`.

### Node, non-air, Space, and demand commits

A positive residual supply-node identity receives only temperature, humidity
ratio, enthalpy, and mass flow; all other node members remain unchanged. With
no residual node, CP240 assigns `NonAirSystemResponse = SysOutputProvided`.
For latent sizing it adds latent watts divided by the integer product of the
parent Zone multipliers to the selected record's `latentGain`.

For a Zone call with no residual node and active Space heat balance, CP240 also
overwrites each stored Space `NonAirSystemResponse` by its volume fraction and
adds the corresponding latent fraction. The immediately following Space CP240
call starts by zeroing that Space response, so the Zone distribution is
overwritten at Space-call entry. That call then routes its result to its
residual node, or writes its own non-air result only when no residual node
exists. In that no-residual case, latent gain is additive and can receive both
the Zone-distribution and Space additions.

The final operation calls `updateSystemOutputRequired` with the main sensible
and latent outputs. A DOAS call therefore invokes that child twice, once before
DOAS node/sizing writes and once after the residual result. With default
priority `-1`, an uncontrolled `ZoneData` or controlled Sequential scheme
subtracts outputs and updates current deadband state, controlled Uniform/PLR
schemes do no update, and an invalid scheme is fatal. CP240's own calculation
continues to read the original `DeadBandOrSetback` array, not the child's
`CurDeadBandOrSetback` result.

### Failure, replay, and stale state

CP240 has no local latch, allocation, topology, membership, bounds, node,
multiplier, denominator, enum, finite-value, or cross-owner validation and no
assertion, status, catch, cleanup, transaction, or rollback. Besides dependency
failures, explicit fatal paths include DOAS with a nonpositive inlet count, an
invalid DOAS control strategy, and an invalid load-distribution scheme.

A nonpositive-inlet DOAS fatal occurs after selected responses and demand state
were reset. Failure after the first DOAS demand update can retain changed demands
without the later node, sizing, or final-update suffix. Failure after a node or
non-air write retains that output. Parent traversal failure retains earlier
Zones and Spaces and suppresses later records, mass balance, leaving
conditions, Part2, and the outer manager suffix.

If one-time sizing setup succeeded, its parent flag is already false when a
later CP240 failure occurs. Same-state retry therefore skips setup, restarts the
ascending Zone traversal, resets many scalar fields again, and overwrites most
node/sizing outputs. It is not generally idempotent: `latentGain` uses `+=`,
earlier Space distribution can be followed by another Space addition, and
branch-disabled DOAS or latent fields can retain older values. Setup failure is
different because the parent clears its flag only after setup returns.

### C++ and active-corpus evidence

No C++ test directly calls CP240. Six direct `SizeZoneEquipment` expressions
across three tests produce seven Zone CP240 entries and zero Space entries: one
two-Zone DOAS heating/cooling call, three no-sensible-load then humidification
and dehumidification calls, and two DOAS-load calls. Their 88 assertion lines
span CP240, Part2, `UpdateZoneSizing`, and fixture inputs, so they do not isolate
the complete CP240 transaction.

All three direct wrapper tests force `SizeZoneEquipmentOneTimeFlag = false` and
leave `doSpaceHeatBalance` false. They set
`ZoneEquipConfig.IsControlled = true` while `ZoneData.IsControlled` remains its
default false, so the demand-update child takes its uncontrolled branch rather
than coherent production controlled-Zone distribution. They cover no Space,
cooling ITE, one-inlet DOAS residual, zero-inlet fatal, non-air Zone output,
adjustment factor above one, invalid strategy/scheme, failure, retry, or reset.
Seven standalone `CalcDOASSupCondsForSizing` calls cover that child's strategy
selection but not its CP240 composition.

Among 57 active full `ManageSimulation` contexts, 56 complete and one
intentionally fails in EMS before HVAC. Exactly 34 completing configurations
request Zone sizing and reach CP240; their active static first-sweep topology is
48 controlled Zone identities. The other 22 completing configurations and the
EMS-fatal context never reach CP240.

Seven of those sizing configurations enable Space sizing. Each has one
controlled parent Zone and three stored Spaces, giving 21 Space CP240 identities
per complete sizing sweep. They have no `SpaceHVAC:EquipmentConnections`, so
the Space configurations are uncontrolled and have zero inlet nodes; CP240
still calls them, with DOAS disabled, through the non-air Space path. Their
tests assert final Space sizing results, not helper order or argument identity.

Across the resulting 69 static roles, one Zone enables DOAS, four roles enable
latent sizing, 43 have a usable residual supply node, and 26 use non-air output:
five Zones plus all 21 Spaces. The five Zone non-air roles comprise four
blank-inlet cases and the one-inlet DOAS case whose inlet is consumed by DOAS.
No active role can enter cooling ITE adjustment or an adjustment factor above
one. These are per-sweep static branch identities, not dynamic runtime counts;
exact design-day, warmup, timestep, retry, and repeated sizing-sweep
multiplicity remains uninstrumented.

### Rust boundary and status

Rust contains one raw `Sizing:Zone` epJSON fixture, but that test expects
`UnsupportedSizing` before runtime; the active IDF corpus contains no
`Sizing:Zone`. Rust has no typed or executable `Sizing:Zone`,
`CalcZoneSizing`, `ZoneSystemMoistureDemand`, Space sizing-demand arena,
`NonAirSystemResponse`, `SysDepZoneLoads`, or source-shaped
`initOutputRequired`/`updateSystemOutputRequired` transaction.
`ZoneSysEnergyDemand` is a four-setpoint-scalar snapshot constructed from
compatibility fixed options (neutral zeros by default), while unit tests can
inject literals. It lacks the total, unadjusted, sequenced, mutable
distribution, and Space state CP240 uses.

Rust does have adjacent exact psychrometric helpers, IdealLoads maximum/minimum
supply limits, selected no-OA load/flow formulas, a narrow purchased-air supply
node update, and diagnostic `NodeStateStore`. Those pieces are neither composed
through the sizing parent nor owners of DOAS sizing, two-inlet routing,
Zone/Space demand reset, ITE adjustment, non-air/latent distribution, stale
branch fields, or failure/replay lifecycle. Existing sizing-like fixture values
and final IdealLoads outputs do not implement CP240.

CP240 adds no algorithm-level EnergyPlus source, Rust target, executable code,
mapped state, test, object support, capability, output implementation,
comparator, case, manifest, numerical, performance, or conformance promotion.
The algorithm remains `scaffold` with claim level `none`. The inventory becomes
32 algorithms and 245 routines, split 58 `state_mapped` plus 187
`source_mapped`, with 122 required; the heat-balance and HVAC project lists
become 88 and 11.

## CP241 `sizeZoneSpaceEquipmentPart2` Return and Thermostat Snapshot

CP241 adds canonical required
`routine.size_zone_space_equipment_part2` immediately after Part1 and before
`routine.sim_zone_equipment`, plus the same ordered HVAC project item. The
exact lowercase routine is declared at `ZoneEquipmentManager.hh` lines 101-105
and implemented completely at `ZoneEquipmentManager.cc` lines 599-625. The
algorithm already cites that source file, so no algorithm-level source or Rust
target changes.

### Parent order and asymmetric Space binding

The only two production call expressions are inside `SizeZoneEquipment`:
the Zone call at lines 685-686 and the Space-loop call at lines 689-690. The
normal route remains `HVACManager::SimHVAC` to `ManageZoneEquipment` with
`ZoneSizingCalc = true`, then CP239 Init and the sizing parent. The separate
post-Zone-sizing `SizingManager` calls occur after that flag is false and do not
enter this parent through `ManageZoneEquipment`.

CP241 is not interleaved with CP240. The sizing parent first completes its
entire ascending controlled-Zone Part1 pass, including each stored Space when
the current `doSpaceHeatBalance` flag is true. It then calls
`CalcZoneMassBalance(state, true)` and
`CalcZoneLeavingConditions(state, true)`. Only after both return does a second
ascending controlled-Zone pass call CP241 for the Zone and then every stored
Space, again without testing a Space configuration's controlled flag. Any
abnormal non-return in that prefix suppresses every CP241 call.

The Zone call binds parent `zoneEquipConfig`,
`CalcZoneSizing(CurOverallSimDay, zoneNum)`, parent `zoneNum`, and default
`spaceNum = 0`. The Space call binds
`CalcSpaceSizing(CurOverallSimDay, spaceNum)`, parent `zoneNum`, and positive
`spaceNum`, but critically passes the same parent Zone `zoneEquipConfig`
instead of `spaceEquipConfig(spaceNum)`. A port that substitutes the Space
configuration changes source behavior.

Consequently, a Space call uses the parent Zone return-node count and first
return identity, and it reads the parent Zone thermostat triplet. Only the
fallback system node and destination sizing record are Space-specific. Any
Space-owned inlet, return, equipment-control, or thermostat-like configuration
is irrelevant to this routine.

### Return-temperature selector

Every entry evaluates:

```text
returnNodeNum =
    zoneEquipConfig.NumReturnNodes > 0
      ? zoneEquipConfig.ReturnNode(1)
      : 0

zoneNodeNum =
    spaceNum > 0
      ? Space(spaceNum).SystemZoneNodeNumber
      : Zone(zoneNum).SystemZoneNodeNumber

RetTemp =
    returnNodeNum > 0
      ? Node(returnNodeNum).Temp
      : Node(zoneNodeNum).Temp
```

Thus a positive count is not enough: a nonpositive first return identity still
falls back to the selected system node. A zero or negative count also falls
back. Only return entry 1 is observed; every later return is ignored. The
Zone/Space record is still read to form `zoneNodeNum` before the final node
choice. Any nonpositive `spaceNum` selects the Zone record; only a positive
value selects Space. CP241 reads only node temperature and performs no node
write. Because the parent has just completed `CalcZoneLeavingConditions`, a
positive return path snapshots that dependency's current return temperature.

### Strict load and thermostat branches

The routine then references the parent Zone
`zoneTstatSetpts(zoneNum)` and applies this exact order:

```text
if HeatLoad > 0:
    HeatZoneRetTemp = RetTemp
    HeatTstatTemp = setpt if setpt > 0 else setptLo
    CoolTstatTemp = setptHi
else if CoolLoad > 0:
    CoolZoneRetTemp = RetTemp
    CoolTstatTemp = setpt if setpt > 0 else setptHi
    HeatTstatTemp = setptLo
else:
    CoolZoneRetTemp = RetTemp
    HeatTstatTemp = setptLo
    CoolTstatTemp = setptHi
```

Both load comparisons and the central-setpoint comparison are strict and have
no epsilon. Heating wins when both loads are positive. Zero, negative, or NaN
heat does not select heating; if cooling is not strictly positive either, the
catch-all branch runs. A zero, negative, or NaN central `setpt` uses the
branch-specific low or high fallback. Values are copied without finite, range,
ordering, or control-mode checks.

Every successful entry overwrites both thermostat snapshot fields and exactly
one return-temperature field. The heating branch leaves `CoolZoneRetTemp`
untouched. Cooling and catch-all leave `HeatZoneRetTemp` untouched. Therefore
a branch transition can retain the inactive return snapshot from an earlier
sizing timestep or sweep. CP241 writes no load, sequence array, node, demand,
global thermostat state, or configuration field and performs no allocation.

The later `UpdateZoneSizing` consumer nevertheless accumulates both heating
and cooling return snapshots into their sequences on every sizing timestep,
then averages and selects peak/no-load values. It also sequences both
thermostat snapshots. The inactive stale return value is therefore
downstream-visible rather than dead state.

Under the production parent, CP240 has just rewritten `HeatLoad` and
`CoolLoad` as mutually exclusive positive-magnitude fields or zeros. The
heating-precedence behavior nevertheless remains part of the callable routine
for malformed or direct states in which both are positive.

### Failure, replay, and reset boundary

CP241 has no child call and no local latch, allocation, topology validation,
diagnostic, assertion, status, catch, cleanup, transaction, or rollback. Its
potentially failing indexed accesses occur before any branch assignment:
conditional `ReturnNode(1)`, selected Zone/Space heat-balance record, selected
node, and parent thermostat record. A bounds or invalid-node failure in those
reads therefore leaves the current sizing record without a new CP241 write.

Failure in an earlier CP240 call, mass balance, or leaving conditions prevents
the second pass entirely. Failure on a later CP241 record retains the complete
Part1/mass/leaving prefix and all earlier Part2 record writes, suppresses later
Zone/Space records, and prevents the outer manager's update suffix. The parent
one-time setup flag is already false by this point.

On same-state retry, `SizeZoneEquipment` starts its Part1 pass again, repeats
mass balance and leaving conditions, and then restarts CP241 from the first
controlled Zone. CP241's selected three scalar assignments are overwrite-stable
for unchanged inputs, but the inactive return snapshot remains stale and the
whole parent replay is not idempotent because CP240 can repeat additive latent
effects. Manager `clear_state()` reconstructs its setup latch but does not
retroactively clear independently owned sizing records, nodes, thermostat
state, or prior parent effects. The external
`ZoneSizingData::zeroMemberData` reset can zero all four CP241 scalars only
after its sequence-allocation guard passes, and production pulse-to-normal
reset is owned by `RezeroZoneSizingArrays`; CP241 itself has no per-call reset.

### C++ and active-corpus evidence

No C++ test directly calls CP241. Six direct `SizeZoneEquipment` expressions
across three tests produce seven Zone entries and zero Space entries. Their
branch distribution is one heating, one cooling, and five catch-all entries;
all seven have zero return-node count and use the Zone system node. The two
heat/cool entries have central `setpt = 0`, so they use low/high fallbacks, but
none of their CP241 fields is asserted.

Across the existing 88 mixed wrapper assertion lines, only four directly name
CP241-owned fields: the no-load test at lines 4583-4584 and the DOAS-load test
at lines 4882-4883 assert the two thermostat snapshots after catch-all calls.
No wrapper assertion names `HeatZoneRetTemp` or `CoolZoneRetTemp`. The separate
`RezeroZoneSizingArrays` test asserts all four fields reset to zero in two
arrays but never invokes CP241, so it is reset evidence rather than execution
evidence.

Eighteen direct `ManageSizing` contexts exist. The plant-only
`WWHP_AutosizeTest1` does not reach Zone sizing; the other 17 complete CP241
for 24 static Zone roles and no Spaces. All 24 have exactly one positive return
node, but no assertion names a CP241-owned field.

Among 57 active full `ManageSimulation` contexts, 56 complete and one
intentionally fails in EMS before HVAC. Exactly 34 completing Zone-sizing
configurations reach CP241; the other 22 completing contexts and the fatal
context do not. One complete sizing sweep across those 34 configurations has
48 Zone plus 21 Space roles.

The return selector splits those 69 static roles into 56 first-return reads and
13 system-node fallbacks. Zones split 44/4. All 44 positive Zone roles resolve
exactly one return node; no active role has multiple returns. Spaces split
12/9: four Space-sizing parent configurations give their three Spaces the same
parent first return, while the other three give each Space its own system-node
fallback. CP241 routing is independent of CP240 residual-supply routing; for
example, the one-inlet DOAS Zone uses CP240 non-air residual output but still
uses a first return in CP241.

One full-corpus downstream assertion,
`BaseSizer_SupplyAirTempLessThanZoneTStatTest`, observes final
`CalcFinalZoneSizing.HeatTstatTemp = 21` after `ManageSimulation`; it does not
isolate CP241 call order, source node, or intermediate field ownership. No test
covers a valid positive return temperature directly, first-of-multiple return
selection, the Space parent-configuration alias, branch transitions and stale
inactive fields, a strict threshold, malformed topology, partial prefix,
retry, or manager reset.

The exact full-corpus heat/cool/catch-all distribution and central-`setpt`
selector distribution depend on each sizing timestep's CP240 results and
thermostat state. Design-day, warmup, timestep, retry, and repeated-sweep
invocation counts and branch sequences are uninstrumented; the static role
counts above are not dynamic execution counts.

### Rust boundary and status

Exact Rust searches find no CP241 symbol or snake-case counterpart, no
`CalcZoneSizing` or `ZoneSizingData`, and none of the four CP241 sizing fields.
Rust does retain typed `ZoneHVAC:EquipmentConnections` return node or NodeList
identities. Its diagnostic node projection expands return identities into
`NodeStateStore` rows initialized from default Zone-air state, rather than
running source mass/leaving conditions and selecting only the first return.

The finite-limit IdealLoads CLI recirculation helper may select an authored
exhaust node first and otherwise resolve the first Zone return/list node. That
oracle/diagnostic helper is not the CP241 parent or sizing record transaction.
Rust also types dual-setpoint schedule identities and emits thermostat report
series directly from currently required constant schedules. It has no mutable
source-shaped `setpt`/`setptLo`/`setptHi` triplet, strict load/setpoint branch,
or stale return-snapshot lifecycle.

Rust has no SpaceHVAC connection or Space system-node counterpart for this
path, no post-leaving Zone/Space sizing sweep, no parent-configuration Space
alias, and no Part2 failure/replay test. The sole raw `Sizing:Zone` epJSON
fixture expects a pre-runtime unsupported-sizing block; active IDFs contain no
`Sizing:Zone`. Existing return-node graph tests, node diagnostics, thermostat
output conformance, and IdealLoads final results are adjacency, not CP241
execution evidence.

CP241 adds no algorithm-level EnergyPlus source, Rust target, executable code,
mapped state, test, object support, capability, output implementation,
comparator, case, manifest, numerical, performance, or conformance promotion.
The algorithm remains `scaffold` with claim level `none`. The inventory becomes
32 algorithms and 246 routines, split 58 `state_mapped` plus 188
`source_mapped`, with 123 required; the heat-balance and HVAC project lists
become 88 and 12.

## CP242 `SizeZoneEquipment` Two-Pass Sizing Parent

CP242 adds canonical required `routine.size_zone_equipment` immediately after
Part2 and before `routine.sim_zone_equipment`, plus the same ordered HVAC
project item. The exact capitalized routine is declared at
`ZoneEquipmentManager.hh` line 107 and implemented completely at
`ZoneEquipmentManager.cc` lines 627-694. The algorithm already cites that
source file, so no algorithm-level source or Rust target changes.

### Entry selection and one-time setup

The sole production call expression is `ManageZoneEquipment` line 158. That
parent first completes CP239 Init, then selects CP242 only while the current
global `ZoneSizingCalc` is true. Its non-sizing branch calls
`SimZoneEquipment` instead. `SizeZoneEquipment` itself accepts only `state`,
does not read `ZoneSizingCalc`, and therefore remains directly callable
outside that manager gate.

`ManageZoneEquipment` does not forward `FirstHVACIteration` into CP242. The
sizing parent later passes the literal `true` independently to its mass- and
leaving-condition children on every invocation.

`ZoneEquipmentManagerData::SizeZoneEquipmentOneTimeFlag` defaults true at
`ZoneEquipmentManager.hh` line 275. The entry prefix is exactly:

```text
if SizeZoneEquipmentOneTimeFlag:
    SetUpZoneSizingArrays(state)
    SizeZoneEquipmentOneTimeFlag = false
```

The clear occurs only after normal child return. A setup abnormal non-return
therefore retains the true latch and any partial `SetUpZoneSizingArrays`
effects. Once setup returns, the latch is false before any Zone traversal;
failure anywhere later leaves it false and a same-state retry skips setup.
No begin-environment branch rearms it. Manager `clear_state()` placement-new
reconstructs the default-true manager data, but does not retroactively clear
independently owned `dataSize` sizing arrays, nodes, demands, air-loop state,
or other child-owned effects. External `RezeroZoneSizingArrays` is a
pulse-to-normal sizing reset, not a CP242 per-call reset; it does not change
the latch or undo every child effect. CP242 maps the parent call boundary,
not the still-separate complete setup body.

### Exact two-pass order and bindings

After the optional setup prefix, every invocation executes this source order:

```text
for zoneNum = 1 .. NumOfZones:
    zoneEquipConfig = ZoneEquipConfig(zoneNum)
    if not zoneEquipConfig.IsControlled:
        continue
    Part1(zone)
    if doSpaceHeatBalance:
        for spaceNum in Zone(zoneNum).spaceIndexes:
            Part1(space)

CalcZoneMassBalance(state, true)
CalcZoneLeavingConditions(state, true)

for zoneNum = 1 .. NumOfZones:
    zoneEquipConfig = ZoneEquipConfig(zoneNum)
    if not zoneEquipConfig.IsControlled:
        continue
    Part2(zone)
    if doSpaceHeatBalance:
        for spaceNum in Zone(zoneNum).spaceIndexes:
            Part2(space)
```

Both loops visit numeric Zone indexes ascending and fetch
`ZoneEquipConfig(zoneNum)` before applying `IsControlled`. An uncontrolled
Zone therefore skips its Zone and Space sizing roles, but still requires the
configuration arena access to succeed. For a controlled Zone, Part1 binds
`CalcZoneSizing(CurOverallSimDay, zoneNum)`, Zone sensible and moisture
demands, and the parent Zone record before entering CP240.

A gated Space Part1 call follows its parent Zone in the stored
`Zone.spaceIndexes` order. It binds `spaceEquipConfig(spaceNum)`,
`CalcSpaceSizing(CurOverallSimDay, spaceNum)`, and Space sensible and moisture
demands, but still passes the parent Zone record and parent `zoneNum`.
There is no Space `IsControlled` check.

The full first pass must return before the two global barriers. Mass balance
always precedes leaving conditions, and both receive literal
`FirstHVACIteration = true`; they run even when `NumOfZones` is zero or every
configuration is uncontrolled. Only after both return does the complete
second pass begin.

A Zone Part2 call binds the current Zone configuration and sizing record.
A Space Part2 call binds its Space sizing record and positive `spaceNum`, but
deliberately reuses the parent `zoneEquipConfig` rather than
`spaceEquipConfig`. Thus CP242 preserves the asymmetric CP240/CP241 Space
contracts instead of normalizing them.

The two loops read `doSpaceHeatBalance` and each parent `spaceIndexes` list
again. CP242 does not snapshot, compare, sort, filter, or deduplicate Space
membership across the passes. It also does not snapshot `NumOfZones`,
`CurOverallSimDay`, or any child-owned state. Normal callers keep this
topology stable, but source parity must not invent a local invariant.

### State ownership, failure prefixes, and replay

Apart from the successful setup-latch clear, CP242 performs no direct output,
node, demand, sizing-record, or equipment-configuration assignment. All
allocation and numerical mutation belongs to `SetUpZoneSizingArrays`, CP240,
mass balance, leaving conditions, or CP241. The parent has no local topology
validation, diagnostic, assertion, return status, catch, cleanup,
checkpoint, transaction, or rollback.

An indexed failure while fetching a Zone configuration can occur before its
control filter. For a controlled role, current-day sizing, demand, Zone,
Space, configuration, and node accesses add further child failure points.
The resulting abnormal prefixes are ordered:

- setup failure keeps the latch true, retains any partial setup effects, and
  suppresses every sizing pass;
- first-pass failure keeps a false post-setup latch, retains completed earlier
  CP240 roles plus any partial current CP240 effects, and suppresses both
  global barriers and all Part2 work;
- mass-balance failure retains the complete Part1 pass plus any partial
  mass-balance effects and suppresses leaving conditions and Part2;
- leaving-condition failure retains setup, Part1, mass-balance, and any
  partial leaving-condition effects but suppresses Part2;
- second-pass failure retains the complete earlier prefix and preceding
  CP241 records. CP241's indexed failures occur before its current-record
  assignments; later roles and the outer manager update remain suppressed.

A same-state retry begins again at the setup test and then the first Zone
pass. After prior successful setup it skips that child, but replays every
other reached child from the beginning. CP240 can repeat additive latent
effects, and mass balance can add air-distribution flow into aggregates that
production Init normally zeros before CP242. Therefore the complete parent is
not an idempotent or transactional retry boundary even though many selected
scalar assignments overwrite stable inputs.

### C++ and active-corpus evidence

The source tree has six direct test call expressions across three tests:

- `DOASEffectOnZoneSizing_SizeZoneEquipment` calls once for two controlled
  Zones;
- `ZoneEquipmentManager_SizeZoneEquipment_NoLoadTest` calls three times for
  one controlled Zone;
- `ZoneEquipmentManager_SizeZoneEquipment_DOASLoadTest` calls twice for one
  controlled Zone.

Together they execute six complete parents, seven Zone Part1/Part2 role
pairs, six mass-balance calls, six leaving-condition calls, and no Space
role. Each test explicitly sets `SizeZoneEquipmentOneTimeFlag = false` before
its first call, while `doSpaceHeatBalance` remains false. Direct tests
therefore bypass both the manager selector and the setup-true branch.

The three tests contain 22, 45, and 21 assertion lines, respectively. Their
88 assertions observe CP240/CP241 descendant state or later
`UpdateZoneSizing` results. None isolates mass balance or leaving conditions,
asserts CP242's only direct latch write, records an exact child trace,
distinguishes the two complete passes, or injects a failure. Repeated calls
in the latter two tests are ordinary same-state descendant calculations,
not failure/recovery evidence.

Eighteen direct `ManageSizing` contexts exist. The plant-only
`WWHP_AutosizeTest1` has no `Sizing:Zone` or equipment connection and does
not enter CP242. Across one parent invocation in each of the other 17, the
static aggregate is 24 controlled Zones and no Spaces; each context
contributes only its subset. Their fresh default latch enters setup on the
first successful parent, but no assertion observes the transition or exact
repeated sizing cadence.

Among 57 active full `ManageSimulation` contexts, 56 complete and one
intentionally stops in EMS before HVAC. Exactly 34 completing configurations
perform Zone sizing. Across one parent invocation in each of those 34
configurations, the static aggregate is 48 controlled Zones and 21 stored
Spaces: 69 Part1 and 69 Part2 role calls. Individual configurations own only
their respective subset. The other 22 completing configurations and the
fatal context do not enter CP242.

Seven full Space-sizing tests exercise the 21 Space roles without explicit
`SpaceHVAC:EquipmentConnections`, so CP242 reaches the source path that
ignores the Space configuration's false `IsControlled` value as a traversal
filter. No focused assertion isolates that traversal choice or its binding
semantics. No test covers setup failure, a
post-setup failure with a false latch, uncontrolled-only or zero-Zone
execution of the global barriers, a changing Space gate/list between passes,
malformed current-day sizing indexes, a mass-versus-leaving failure prefix,
or retry/reset recovery. Exact design-day, warmup, timestep,
HVAC-iteration, pulse, and total dynamic parent invocation counts are not
instrumented; the corpus figures above are static topology, not call counts.

### Rust boundary and status

Exact Rust-code searches find no `SizeZoneEquipment`,
`size_zone_equipment`, Zone/Space sizing arena, `CalcZoneMassBalance`,
`CalcZoneLeavingConditions`, `doSpaceHeatBalance`, current overall sizing
day, or one-time sizing latch. The existing
`ideal_loads_zone_equipment_stages()` array contains only three labels:
ManageZoneEquipment, SimZoneEquipment, and SimPurchasedAir. Its tests verify
metadata/graph order, not CP242.

Rust directly traverses typed IdealLoads equipment and invokes
`sim_purchased_air_compat*` with a prebound four-scalar
`ZoneSysEnergyDemand`. Equipment graph validation, psychrometrics,
diagnostic node projection, and narrow supply-node updates are adjacent
components, not the source setup, complete first pass, fixed-true global
barriers, complete second pass, or failure/replay transaction.

`Sizing:*` and `ZoneSizing*` remain run-blocked in the capability contract.
The sole raw `Sizing:Zone` epJSON fixture expects `UnsupportedSizing` before
runtime, and the active data-model IDF corpus contains no `Sizing:Zone`.
Existing sizing-like fixture values and final IdealLoads outputs therefore
provide no CP242 execution evidence.

CP242 adds no algorithm-level EnergyPlus source, Rust target, executable
code, mapped state, test, object support, capability, output implementation,
comparator, case, manifest, numerical, performance, or conformance
promotion. The algorithm remains `scaffold` with claim level `none`. The
inventory becomes 32 algorithms and 247 routines, split 58 `state_mapped`
plus 189 `source_mapped`, with 124 required; the heat-balance and HVAC
project lists become 88 and 13.

## CP243 `CalcDOASSupCondsForSizing` DOAS Supply Selector

CP243 adds canonical required
`routine.calc_doas_sup_conds_for_sizing` immediately after
`routine.size_zone_equipment` and before `routine.sim_zone_equipment`, plus
the same ordered HVAC project item. The exact routine is declared at
`ZoneEquipmentManager.hh` lines 244-254 and implemented completely at
`ZoneEquipmentManager.cc` lines 696-765. The algorithm already cites that
source file, so no algorithm-level source or Rust target changes.

### Production call and input/output contract

The sole production call expression is CP240
`sizeZoneSpaceEquipmentPart1` line 387. It is reached only for the current
Zone or Space sizing role when selected `zsCalcSizing.AccountForDOAS` is
true (`CalcZoneSizing` for Zone and `CalcSpaceSizing` for Space).
The helper receives the outdoor dry-bulb and humidity ratio, low and high
DOAS temperature limits, humidity ratios at 90% RH at each limit, and a
`DataSizing::DOASControl` by value. Its two result scalars are mutable
references.

Every entry first performs these ordered writes:

```text
DOASSupTemp = 0.0
DOASSupHR   = 0.0
```

It then implements this complete strategy table:

| `DOASControl` | Condition | `DOASSupTemp` | `DOASSupHR` |
|---|---|---:|---:|
| `NeutralSup` | `OutDB < DOASLowTemp` | `DOASLowTemp` | `OutHR` |
| `NeutralSup` | otherwise and `OutDB > DOASHighTemp` | `DOASHighTemp` | `min(OutHR, W90H)` |
| `NeutralSup` | otherwise | `OutDB` | `OutHR` |
| `NeutralDehumSup` | `OutDB < DOASLowTemp` | `DOASHighTemp` | `OutHR` |
| `NeutralDehumSup` | otherwise | `DOASHighTemp` | `min(OutHR, W90L)` |
| `CoolSup` | `OutDB < DOASLowTemp` | `DOASHighTemp` | `OutHR` |
| `CoolSup` | otherwise | `DOASLowTemp` | `min(OutHR, W90L)` |

The enum values are `Invalid = -1`, `NeutralSup = 0`,
`NeutralDehumSup = 1`, `CoolSup = 2`, and sentinel `Num = 3`.
`W90H` is read only by the `NeutralSup` high branch; `W90L` is read only by
the other two strategies' else branches.

### Boundaries and Objexx minimum semantics

All temperature tests are raw strict `<` or `>` comparisons with no epsilon.
For ordinary ordered Low and High limits, `NeutralSup` equality at either
limit enters its pass-through branch. `NeutralDehumSup` and `CoolSup`
equality at Low enter their else branches. CP243 does not validate Low versus
High ordering: when Low exceeds High, the first `OutDB < Low` test owns the
overlapping `NeutralSup` range. It also does not reject or clamp negative
humidity, non-finite values, or otherwise inconsistent inputs.

The unqualified `min` is the ObjexxFCL double overload imported by
`EnergyPlus.hh`, whose body is exactly `a < b ? a : b`; it is not
`std::fmin`. Therefore:

- ordinary comparable values produce the numerical minimum;
- a tie selects the second `W90*` operand, including its signed-zero bit;
- NaN `OutHR` with finite `W90*` selects the finite second operand;
- finite `OutHR` with NaN `W90*` selects that second NaN;
- two NaNs select the second NaN; and
- infinities follow the ordinary raw comparison.

The branch comparisons have the same raw IEEE behavior. `OutDB = NaN` makes
both comparisons false, so `NeutralSup` returns `(NaN, OutHR)` while
`NeutralDehumSup` and `CoolSup` take their min-bearing else branches. A NaN
Low makes every `< Low` false; a NaN High makes the `NeutralSup` `> High`
test false. Temperature or humidity special values can consequently pass
through to the selected result without a local diagnostic.

### Invalid strategy, aliasing, and failure prefix

`Invalid`, `Num`, and any cast enum value outside the three valid enumerators fall through after the
two zero writes and issue this exact fatal:

```text
CalcDOASSupCondsForSizing:illegal DOAS design control strategy
```

`ShowFatalError` does not return. A direct harness that catches the fatal
therefore observes the two-zero output prefix and the diagnostic state.
Valid strategies do not read or mutate `state`; it is used only to emit this
invalid-strategy fatal.

The two output references may alias. CP243 performs no alias check and every
valid branch writes temperature before humidity, so one shared location ends
with the humidity result. All scalar/control calculation inputs other than
`state` are passed by value, so overlapping caller storage cannot change the
entry snapshots used by the calculation.

CP243 has no local latch, allocation, assertion, return status, catch,
cleanup, checkpoint, transaction, or rollback. Before its call, CP240 has
already reset the selected non-air/system-dependent response, called
`initOutputRequired`, snapshotted pre-DOAS sensible and optional latent
loads, validated inlet count, calculated `W90H` and `W90L`, and calculated
DOAS mass flow. Only a normal CP243 return permits the later heat-capacity,
enthalpy, load, demand, inlet-node, and sizing-record writes.

An invalid-strategy fatal retains that completed CP240 model-state prefix,
writes only its stack-local outputs to zero without publishing them to node
or sizing state, and suppresses the current CP240 suffix, all later Part1
roles, mass balance, leaving conditions, all Part2 roles, and the production
`ManageZoneEquipment` update suffix. A valid direct repeat deterministically
overwrites both outputs for stable value inputs. An invalid repeat can zero
them again and repeat the fatal diagnostic. Retry through CP242 remains
generally non-idempotent because the wider Part1 traversal and its child
effects are replayed.

### C++ and active-corpus evidence

The direct
`DOASEffectOnZoneSizing_CalcDOASSupCondsForSizing` test has seven helper
calls and 14 temperature/humidity assertions:

- `NeutralSup` covers below-Low, above-High, and pass-through once each;
- `NeutralDehumSup` covers below-Low and else once each; and
- `CoolSup` covers below-Low and else once each.

All inputs are finite with ordinary Low-below-High ordering. Every direct min
case has `OutHR > W90*`, so only cap selection is proven there. The test does
not cover either equality, `OutHR <= W90*` within a cap branch, inverted
limits, NaN, infinity, signed zero, invalid enum, aliasing, failure, or retry.

Six direct `SizeZoneEquipment` call expressions across three wrapper tests
produce only three CP243 executions. The two-Zone
`DOASEffectOnZoneSizing_SizeZoneEquipment` call reaches two `CoolSup`
else/cap roles. Of five one-Zone calls in the two
`ZoneEquipmentManager` sizing tests, only the second DOAS-load call enables
DOAS and reaches a `NeutralSup` high branch whose `OutHR` remains below the
cap; the three no-load calls and first DOAS-load call do not reach CP243.
Four stored-output assertions in the two-Zone test and two in the DOAS-load
test observe the resulting supply values. Separate node-copy assertions are
not counted as direct CP243 output oracles. No wrapper injects an invalid
strategy or a CP243 failure.

Eighteen direct `ManageSizing` contexts exist. The plant-only context does
not reach the sizing parent; across one parent invocation in each of the
other 17, all 24 Zone roles have `AccountForDOAS` false, so CP243 is reached
zero times regardless of later dynamic sizing repetitions.

Among 57 active full `ManageSimulation` contexts, 56 complete and one stops
in EMS before HVAC. Across one parent invocation in each of the 34 completing
configurations that perform Zone sizing, the static aggregate is 48 Zone
plus 21 Space roles. Exactly one Zone role and no Space role enable DOAS; the
other 68 roles do not. That sole fixture omits its strategy field and
therefore uses the IDD default `NeutralSupplyAir`/`NeutralSup`. Its summer and
winter design-day inputs provide high- and low-side outdoor conditions,
respectively, but assertions inspect only downstream DOAS table loads rather
than CP243 supply outputs. Exact repeated sizing, design-day iteration, and
total dynamic CP243 call counts remain uninstrumented. Each context
contributes only its own topology subset.

### Rust boundary and status

Exact Rust searches find no `CalcDOASSupCondsForSizing`,
`calc_doas_sup_conds_for_sizing`, `DOASControl`, `AccountForDOAS`,
`DOASSupTemp`, or `DOASSupHR`. A downstream psychrometric sensible-enthalpy
test uses DOAS wording, and the typed PurchasedAir outdoor-air graph,
mixed-air supply calculation, IdealLoads supply limits, and psychrometric
helpers are adjacent. They implement operational
`ZoneHVAC:IdealLoadsAirSystem` behavior, not this `Sizing:Zone` low/high/90%-RH
DOAS selector.

`Sizing:*` and `ZoneSizing*` remain run-blocked in the capability contract.
The sole raw `Sizing:Zone` fixture expects `UnsupportedSizing` before
runtime, and the active data-model corpus contains no `Sizing:Zone`.
Consequently no existing result, output, or conformance case provides CP243
execution evidence.

CP243 adds no algorithm-level EnergyPlus source, Rust target, executable
code, mapped state, test, object support, capability, output implementation,
comparator, case, manifest, numerical, performance, or conformance
promotion. The algorithm remains `scaffold` with claim level `none`. The
inventory becomes 32 algorithms and 248 routines, split 58 `state_mapped`
plus 190 `source_mapped`, with 125 required; the heat-balance and HVAC
project lists become 88 and 14.

## CP244 `SetUpZoneSizingArrays` One-Time Sizing-State Constructor

CP244 adds canonical required `routine.set_up_zone_sizing_arrays`
immediately after `routine.calc_doas_sup_conds_for_sizing` and before
`routine.sim_zone_equipment`, plus the same ordered HVAC project item. The
exact routine is declared at `ZoneEquipmentManager.hh` line 109 and
implemented completely at `ZoneEquipmentManager.cc` lines 767-1082. The
algorithm already cites that source file, so no algorithm-level source or
Rust target changes.

### Production gate and ownership

The signature accepts only mutable `EnergyPlusData &state`; it has no return
value or output argument. Its sole production call expression is
`SizeZoneEquipment` line 644:

```text
if SizeZoneEquipmentOneTimeFlag:
    SetUpZoneSizingArrays(state)
    SizeZoneEquipmentOneTimeFlag = false
```

The manager-data latch defaults true and is cleared at line 645 only after
CP244 returns normally. Direct CP244 calls neither inspect nor mutate it.
The normal production path therefore executes setup once at the first
reached sizing-parent entry in a fresh state, not once per design-day
timestep. Later sizing-parent entries skip it. A setup abnormal non-return
retains the true latch and its partial effects; a failure after successful
setup sees a false latch and skips CP244 on parent retry.

No begin-environment transition rearms this latch. Manager `clear_state()`
placement-news manager-owned data and restores true, but does not coordinate
resets of `dataSize`, HeatBalance, ZoneControls, ZoneEquipment, EMS, output,
diagnostic, or OA-requirement state. Direct callers bypass the lifecycle
contract entirely.

### Conditional internal-gain allocation

CP244 initializes local `ErrorsFound = false`. It checks only whether
`dataHeatBal->ZoneIntGain` is allocated. When false it calls
`DataHeatBalance::AllocateIntGains`, which allocates Zone and Space internal
gain storage, Space gain-device storage, and daylight Space power-reduction
state. The parent does not independently inspect those other arrays.

This is a narrow readiness guard, not a transactional allocation barrier. If
the child abnormally leaves `ZoneIntGain` allocated before finishing the
other arrays, a same-state retry skips the child and cannot repair that
partial prefix.

### Ordered `Sizing:Zone` validation

For each `ZoneSizingInput` in one-based stored order, CP244 performs this
logic:

1. It exact-name searches the HeatBalance Zone arena. A miss emits a severe
   and sets local `ErrorsFound`, but does not stop the loop.
2. It recomputes `any_of(ZoneEquipConfig.IsControlled)` for the current
   record. With no controlled configuration anywhere, it emits a severe and
   sets `ErrorsFound` once per sizing input.
3. When any controlled configuration exists, it exact-name searches the
   full equipment-configuration arena. A match writes that one-based
   configuration index to `ZoneSizingInput.ZoneNum`; the matched record is
   not itself rechecked for `IsControlled`. A miss is warning-only and is
   silent during pulse sizing.
4. Independently of that configuration-match result, if cooling or heating
   `AirflowSizingMethod` is exactly `FromDDCalc`, CP224
   `VerifyThermostatInZone` runs. A false result owns a second warning that
   is also silent during pulse sizing.

An empty sizing-input arena skips the entire loop, including the
no-controlled-equipment severe. Pulse sizing suppresses only the two warning
classes here; it does not suppress unknown-Zone or no-equipment severe
errors. CP224 can lazily acquire Zone air controls and has its own
shared-latch failure lifecycle.

### DOAS auto-calculation before bulk allocation

`AutoCalcDOASControlStrategy` runs unconditionally after the validation loop,
even if CP244's local `ErrorsFound` is already true. That still-separate
child can overwrite enabled DOAS low/high setpoints and emit DOAS EIO rows.
An inverted result invokes its own earlier fatal:

```text
Errors found in DOAS sizing input. Program terminates.
```

That child fatal prevents all CP244 bulk allocations and suffix work, but
retains the internal-gain, validation, ZoneNum, thermostat-input, DOAS
setpoint, report, and diagnostic prefix already reached. Its local error flag
is separate from CP244's accumulated flag.

### Exact allocation order and extents

After normal DOAS-child return, CP244 allocates the following storage even
when its own accumulated error is already true. Let
`D = TotDesDays + TotRunDesPersDays`,
`Z = NumOfZones`, `S = numSpaces`, `A = NumAirTerminalUnits`,
`T = NumOfTimeStepInDay`, and `H = TimeStepsInHour * 24`.

The exact outer order is `ZoneSizing`, `FinalZoneSizing`, `CalcZoneSizing`,
`CalcFinalZoneSizing`; then, under the Space gate, `SpaceSizing`,
`FinalSpaceSizing`, `CalcSpaceSizing`, `CalcFinalSpaceSizing`; then
`TermUnitFinalZoneSizing` and its member loop, the `DesDayWeath` outer
container, `AvgData`, and finally each weather record's `Temp`, `HumRat`, and
`Press` allocation and zeroing.

| Storage | Extent and initialization |
|---|---|
| `ZoneSizing` | `D x Z` |
| `FinalZoneSizing` | `Z`; `EPVector::allocate` zero-initializes every record |
| `CalcZoneSizing` | `D x Z` |
| `CalcFinalZoneSizing` | `Z`; `EPVector::allocate` zero-initializes every record |
| optional `SpaceSizing` | `D x S` when `doSpaceHeatBalanceSizing` |
| optional `FinalSpaceSizing` | `S` under the same gate; every record is zero-initialized |
| optional `CalcSpaceSizing` | `D x S` under the same gate |
| optional `CalcFinalSpaceSizing` | `S` under the same gate; every record is zero-initialized |
| `TermUnitFinalZoneSizing` | `A`; each record dimensions eight member sequences to `T` and zeroes them |
| `DesDayWeath` | `D` outer records; after `AvgData`, each record dimensions and zeroes `Temp`, `HumRat`, and `Press` to `H` |
| manager `AvgData` | `T`, before the weather-member loop |

These are unconditional `allocate` or member-`dimension` operations, not
shape checks. The four `Final*` EPVectors are filled with `T{}` even at an
unchanged extent; same-extent ObjexxFCL sizing arrays do not receive that
blanket reset here. CP244 does not validate negative/inconsistent extents,
preexisting allocations, the relationship between `T` and live
`TimeStepsInHour`, or prior contents.

### Controlled-Zone fill, Space reuse, and EMS registration

CP244 then visits numeric Zone indexes ascending and obtains
`ZoneEquipConfig(zone)` before its `IsControlled` filter. For each controlled
Zone it exact-name searches `ZoneSizingInput`. A match selects that record.
A miss warns outside pulse sizing and unguardedly selects
`ZoneSizingInput(1)` as the fallback; an empty input arena is therefore an
indexed failure rather than a local diagnostic.

The separate `fillZoneSizingFromInput` child fills the Zone design-day,
calculation, final, and calculation-final records from the selected input.
When `doSpaceHeatBalanceSizing` is true, CP244 visits the parent Zone's
stored `spaceIndexes` order and invokes the same child once per Space with
the same selected Zone input and the Space name/identity. It does not sort,
deduplicate, validate membership, or consult a Space-controlled flag.

If `AnyEnergyManagementSystemInModel` is true, each controlled Zone then
registers exactly 17 internal variables and six actuators over final and
intermediate heating/cooling mass flow, load, density, volume flow, and
outdoor-air fields. Spaces receive no analogous registrations. Registration
failure or duplicate behavior belongs to the EMS dependencies; CP244 has no
local status or cleanup.

### DSOA SpaceList population and OA child order

A single local `dsoaError` starts false. CP244 scans every
`OARequirements` record, and records with positive `numDSOA` scan each stored
Space name in order. A valid exact-name Space index is appended to persistent
`dsoaSpaceIndexes`; the vector is not cleared first. A missing Space emits a
severe plus continue diagnostic and sets both `dsoaError` and
`ErrorsFound`. The duplicate loop then runs after either lookup branch and
compares `thisSpaceNum` with every earlier persistent index. A successful
duplicate is already appended; it emits another severe/continue pair, sets
both flags, and remains appended. A missing lookup supplies zero and normally
matches nothing because CP244 itself appends only positive indexes.

Consequently the check detects duplicates within one declaration and
duplicates introduced by direct replay. One bad SpaceList sets the shared
`dsoaError` for all later work, not just that requirement.

CP244 next calls the still-separate `calcSizingOA` child for controlled Zones
in ascending order. Under `doSpaceHeatBalanceSizing`, it then scans every
global Space index ascending, reads its parent Zone, and calls the child when
that parent's equipment configuration is controlled. This second topology
source is the global Space arena, unlike the earlier per-Zone
`spaceIndexes` fill traversal.

The shared `dsoaError` suppresses each child's DSOA dereference and
`calcDesignSpecificationOutdoorAir` call, leaving the local OA-volume
accumulator at `0.0`. The later air-distribution-efficiency branch still
runs, so final `MinOA` is not guaranteed to remain positive zero. The child
continues other People, area, equipment, and sizing mutations. Cross-Zone
SpaceList membership is diagnosed by the child through shared `ErrorsFound`
but does not set `dsoaError`, so calculation continues. CP245 will map the
child's arithmetic and its exact state anomalies separately.

For parent transaction purposes, each Space child accumulates its People
contributions into its zero-initialized final-Space peak-occupancy field; a
full CP244 replay zero-initializes that record again. Its design-day suffix
nevertheless overwrites a subset of the owning Zone's design-day sizing
fields rather than the Space design-day arrays.

### EIO suffix and late fatal

After all reached OA child calls, CP244 writes this EIO order once:

1. averaging-window header and `NumTimeStepsInAvg`;
2. heating-factor header and global factor;
3. one row for each controlled Zone whose heating factor is not exactly
   `1.0`;
4. cooling-factor header and global factor; and
5. one row for each controlled Zone whose cooling factor is not exactly
   `1.0`.

There are no per-Space sizing-factor rows. The raw `!= 1.0` test has no
epsilon. Only after all these writes does the parent inspect
`ErrorsFound`. A true value issues this exact fatal:

```text
SetUpZoneSizingArrays: Errors found in Sizing:Zone input
```

The main accumulated flag covers an unknown HeatBalance Zone, globally
absent controlled equipment for a nonempty sizing-input arena, missing or
duplicate DSOA Space members, and cross-Zone DSOA membership reported by the
OA child. Configuration miss, thermostat miss, and first-input fallback are
warning-only. Allocation, indexing, dependency, registration, or file-output
failures can abnormally stop at their own earlier point.

### Failure prefixes, replay, and reset

CP244 owns no return status, catch, cleanup, checkpoint, transaction, or
rollback. An early AutoCalc fatal preserves its prefix and suppresses bulk
allocation. A bulk allocation/fill/EMS failure preserves each earlier
allocation and mutation. A DSOA or OA failure preserves all earlier sizing
state plus appended indexes. The normal accumulated-error path deliberately
preserves every allocation, fill, OA mutation, EIO row, and diagnostic before
its tail fatal. Production failure also leaves the caller latch true.

Same-state replay is neither idempotent nor a repair boundary:

- DSOA indexes append again and a formerly valid list can become duplicate
  input that sets the late fatal;
- the four `Final*` EPVectors are zero-filled on every allocation, so
  `calcSizingOA` rebuilds rather than carries peak occupancy across a full
  replay, while OA, equipment, and daily sizing writes still repeat;
- DOAS and main EIO rows and diagnostics can repeat;
- EMS internal-variable registration attempts can diagnose duplicates while
  actuator attempts are dependency-owned no-ops;
- terminal and weather member sequences are reset, while same-extent
  ObjexxFCL Zone/Space sizing arrays and `AvgData` can retain untouched prior
  fields; and
- the single `ZoneIntGain` guard can skip repair of a partial child
  allocation.

`RezeroZoneSizingArrays` resets selected computed Zone/Space sizing data but
does not clear the setup latch, DSOA indexes, EMS registry, output, every
allocation, or every child-owned field. Manager `clear_state()` rearms the
latch and resets manager-owned data only. A clean replay requires a
coordinated full-state reset across all owners.

### Direct C++ evidence

Exactly three tests call CP244 directly:

- `AirTerminalSingleDuctMixer_GetInputDOASpecs` supplies two matching
  `ZoneSizingInput` records and two controlled Zones. Both design-day airflow
  records reach false thermostat verification and unasserted warnings. The
  later blank-mixer lookup supplies one CP244-dependent OA-pointer assertion.
- The DataSizing SpaceList test has no sizing input or controlled Zone and
  maps four valid unique DSOA Space members. One later OA-sum assertion
  depends on those Space indexes.
- The MixedAir mechanical-ventilation test also has no sizing input or
  controlled Zone and maps six valid unique DSOA Space members. Three later
  mechanical-OA assertions depend on those indexes.

No assertion immediately inspects a CP244-owned allocation, `ZoneNum`,
copied sizing field, `MinOA`, EIO row, EMS binding, or latch. The three
direct calls provide five descendant oracles only. All omit Space sizing,
EMS registration, and enabled DOAS. The two SpaceList fixtures plus the later
full-simulation fixture cover three lists and 12 members, all valid and
unique.

Six direct `SizeZoneEquipment` call expressions explicitly force the setup
latch false before their first call and execute CP244 zero times.

### Sizing and active-simulation census

Eighteen direct `ManageSizing` expressions exist. The plant-only context
does not enter the sizing parent. Each of the other 17 starts with a fresh
true latch and completes exactly one CP244 call. Across those states the
static aggregate is:

- 24 sizing inputs, 24 controlled Zones, 24 successful thermostat checks,
  24 Zone fills, and 24 Zone `calcSizingOA` calls;
- heating `FromDDCalc` in all 24 records, with cooling `FromDDCalc` in 17 and
  `DesignDayWithLimit` in seven;
- 23 Zone roles linked to an individual DSOA and one PIU role without OA;
- no Space sizing, enabled DOAS, EMS, or DSOA SpaceList; and
- 30 design-day weather records across the 17 independent states.

These are per-state aggregates; no single context contains all records.

Among 57 active `ManageSimulation` expressions, 56 complete and one stops
in EMS before HVAC. Thirty-four completing configurations perform Zone
sizing, begin with fresh latches, and each complete CP244 exactly once. Their
static aggregate is:

- 48 matching sizing inputs, controlled Zones, successful thermostat checks,
  Zone fills, and Zone OA-child calls;
- 48 records with both airflow methods `FromDDCalc`;
- seven Space-sizing configurations with three Spaces each, producing 21
  Space fills and OA-child calls;
- one enabled-DOAS Zone and no enabled-DOAS Space;
- one valid unique two-member DSOA SpaceList configuration;
- no explicit EMS, ExternalInterface, or PythonPlugin object and therefore
  no EMS-registration branch; and
- 64 design-day weather records across the 34 independent states.

The remaining 22 completing configurations and the pre-HVAC fatal context
do not enter CP244. Repeated `SizeZoneEquipment` calls after the first setup
see the false latch, so the figures are setup calls rather than sizing
timestep or design-day iteration counts.

No test covers an unknown sizing Zone, a nonempty sizing input with no
controlled equipment, a controlled-list miss, first-input fallback, pulse
warning suppression, a missing/duplicate/cross-Zone Space member, shared
`dsoaError`, direct Space-sizing array or overwrite assertions, EMS
registration, a nondefault per-Zone sizing-factor EIO row, exact terminal,
weather, or averaging extents, child failure, late-fatal prefix, replay, or
coordinated reset.

### Rust boundary and status

Crate-wide exact searches find no `SetUpZoneSizingArrays`,
`set_up_zone_sizing_arrays`, `ZoneSizingInput`, `FinalZoneSizing`,
`CalcZoneSizing`, `TermUnitFinalZoneSizing`, `DesDayWeath`, `calcSizingOA`,
`fillZoneSizingFromInput`, `AutoCalcDOASControlStrategy`,
`NumTimeStepsInAvg`, or global sizing-factor state.

Rust does type normalized Zones, Spaces, ordinary building `SpaceList`
records, bounded direct-Zone thermostats, equipment connections, and
individual `DesignSpecification:OutdoorAir` records. It also has
IdealLoads-only OA calculations and time-axis structures. These are adjacent
typed or limited-runtime subsets. Authored `Space` and ordinary `SpaceList`
remain run-blocked, and the latter is not
`DesignSpecification:OutdoorAir:SpaceList`; Rust has no typed DSOA
SpaceList, Zone/Space sizing arena, one-time setup latch, source validation
and fallback order, design-day weather sizing storage, EMS sizing registry,
EIO writer, or late-fatal/replay transaction.

`Sizing:*` and `ZoneSizing*` remain run-blocked in the capability contract,
and EMS/Python/Airflow modifiers are independently run-blocked. The sole raw
`Sizing:Zone` fixture expects `UnsupportedSizing` before runtime, while the
active data-model corpus contains no `Sizing:Zone`. Existing typed graph,
thermostat, OA, schedule, output, and IdealLoads evidence therefore cannot
promote CP244.

CP244 adds no algorithm-level EnergyPlus source, Rust target, executable
code, mapped state, test, object support, capability, output implementation,
comparator, case, manifest, numerical, performance, or conformance
promotion. The algorithm remains `scaffold` with claim level `none`. The
inventory becomes 32 algorithms and 249 routines, split 58 `state_mapped`
plus 191 `source_mapped`, with 126 required; the heat-balance and HVAC
project lists become 88 and 15.

## CP245 `calcSizingOA` Zone/Space Outdoor-Air Sizing Mutator

CP245 adds canonical required `routine.calc_sizing_oa` immediately after
`set_up_zone_sizing_arrays` and before `sim_zone_equipment`. The source
boundary is the declaration at `ZoneEquipmentManager.hh` lines 111-117 and
the complete definition at `ZoneEquipmentManager.cc` lines 1084-1206. The
function returns `void` and accepts final and calculated-final sizing records,
two referenced error flags, `zoneNum`, and optional `spaceNum = 0`.

### Caller, role, and phase order

The only production call expressions are CP244 lines 1032 and 1042. The
parent first visits controlled Zones and then, under Space sizing, all global
Spaces in ascending index order whose parents are controlled. It passes one
shared `dsoaError` and `ErrorsFound` pair through the complete pass. CP245
itself does not inspect the setup latch and validates neither indexes,
allocation, record distinctness, bool distinctness, nor the relationship
between a supplied Space and Zone.

The routine executes these phases without a rollback boundary:

1. Snapshot `ZoneDesignSpecOAIndex` from final sizing, read the parent Zone,
   compute its signed integer `Multiplier * ListMultiplier`, and choose Zone
   or Space floor area.
2. With a positive pointer and false `dsoaError`, validate DSOA SpaceList
   Zone membership and write final-only per-person and per-area design rates.
3. Scan all People and accumulate design, peak, and minimum occupancy for the
   selected Zone or Space role.
4. Write final-only multiplied area, People, and OA aggregates and optionally
   publish Zone `VozMin`.
5. Store DSOA and air-distribution indexes in the selected equipment config.
6. With false `dsoaError`, call `calcDesignSpecificationOutdoorAir`.
7. Write and effectiveness-adjust final/calculated-final `MinOA`.
8. Derive per-area flow limits and scale four input-flow fields in place.
9. Fan five fields into final/calculated Zone arrays for every design day.

### DSOA validation and design rates

A nonpositive DSOA pointer skips the local dereference block. A negative
pointer can still reach the later OA child. When a positive pointer denotes a
DSOA SpaceList, CP245 loops `1..numDSOA`. Every positive member whose
`space.zoneNum` differs from `zoneNum` emits:

```text
SetUpZoneSizingArrays: DesignSpecification:OutdoorAir:SpaceList=<name>
is invalid for Sizing:Zone=<final-record ZoneName>
All spaces in the list must be part of this zone.
```

Each mismatch sets `ErrorsFound = true`, but there is no break, removal,
`dsoaError` assignment, or local fatal; the invalid member remains available
to later OA calculation. A zero member index is silently skipped by this
check, while child logic can still fail on malformed storage. Because a Space
record's `ZoneName` contains the Space name, Space-role diagnostics can label
that name as `Sizing:Zone`. Every parent and Space role can repeat validation
of the same list.

The same guard writes `DesOAFlowPPer` and `DesOAFlowPerArea` only on the final
record. A false guard retains their old values. `dsoaError` is read-only in
CP245 even though its signature is non-const; cross-Zone validation changes
only `ErrorsFound`.

### People, multiplier, aggregates, and predefined report

Zone roles select a People record by `ZonePtr`; Space roles select it by
`spaceIndex`. For each match:

```text
numPeople = NumberOfPeople * Zone.Multiplier * Zone.ListMultiplier
design total += numPeople
peak += numPeople * schedule_max, only when schedule_max > 0
peak += numPeople, otherwise
minimum += numPeople * schedule_min
```

Peak uses `+=`, not assignment. A zero, negative, or NaN maximum therefore
takes the full-design fallback, whereas minimum schedule values are never
clamped. The extrema accessors can populate persistent lazy schedule caches.
Null schedule pointers, inconsistent Zone/Space People topology, negative
counts, non-finite values, and signed-int multiplier overflow have no local
guard.

Final sizing receives `TotalZoneFloorArea`, `TotPeopleInZone`,
`TotalOAFromPeople`, and `TotalOAFromArea`. Floor area and People both carry
the Zone multiplier. `VozMin` uses minimum scheduled occupancy plus area OA,
divided by `std::min` of final cooling/heating air-distribution
effectiveness. Equality selects the cooling operand; a cooling NaN
propagates, while a heating-only NaN normally leaves cooling selected. Either
signed zero is replaced by 1.0, but negative and NaN values remain raw. Only
a Zone role writes `ZonePreDefRep(zoneNum).VozMin`; Space roles compute but do
not publish an equivalent.

### Equipment pointers, OA child, effectiveness, and fanout

The selected `ZoneEquipConfig(zoneNum)` or `spaceEquipConfig(spaceNum)`
always receives final sizing's DSOA and air-distribution indexes, even after
an earlier shared DSOA error. `OAVolumeFlowRate` starts at zero. With false
`dsoaError`, CP245 calls `calcDesignSpecificationOutdoorAir` using false
occupancy-schedule, minimum-OA-schedule, per-person-not-set, and maximum-flow
flags, plus the current Space index. The omitted `calcIAQMethods` argument
defaults true. Ordinary methods therefore use design occupancy and no
minimum-OA schedule, while IAQ/proportional methods may still visit
contaminant controller state. The child owns DSOA method and SpaceList
arithmetic, multiplier application, diagnostics, fatals, and warning flags;
CP245 does not multiply the returned OA again.

Both records first receive the local OA accumulator as `MinOA`: zero when
the child is suppressed, otherwise the child result. If either final
effectiveness is positive, CP245 divides final `MinOA` by unqualified
ObjexxFCL `min(a,b)`, which is shaped as `a < b ? a : b`, and copies that
answer to calculated-final. A tie therefore selects heating; cooling NaN
selects heating, while heating NaN selects the NaN. The OR guard permits a
positive effectiveness paired with zero, negative, or NaN, so division can
produce infinity, NaN, or a negative result. No finite/nonnegative clamp
exists, and calculated-final's own effectiveness fields are ignored. Thus
even `dsoaError` leaving the OA accumulator at positive zero does not
guarantee a final positive-zero `MinOA`.

CP245 freshly derives `DesCoolMinAirFlow2` and `DesHeatMaxAirFlow2` for each
record from its per-area value, role floor area, and Zone multiplier. It then
multiplies final and calculated-final `DesCoolMinAirFlow`,
`DesHeatMaxAirFlow`, `InpDesCoolAirFlow`, and `InpDesHeatAirFlow` in place.
For every design or run-period design day it writes exactly `MinOA`,
`DesCoolMinAirFlow2`, `DesCoolMinAirFlow`, `DesHeatMaxAirFlow2`, and
`DesHeatMaxAirFlow` to `ZoneSizing(day, zoneNum)` and
`CalcZoneSizing(day, zoneNum)`.

That destination remains the Zone arrays during a Space role. CP245 never
writes the corresponding five fields in `SpaceSizing` or `CalcSpaceSizing`.
Because CP244 orders the Zone first and Spaces globally ascending, each Space
overwrites its parent's Zone daily column and the highest global Space index
for that Zone wins.

### Failure, aliasing, retry, and reset

There is no status, catch, checkpoint, cleanup, transaction, or rollback.
An invalid early reference can fail before writes; a second design-rate
failure can retain the first rate; a People-loop failure retains earlier
peak accumulation and caches; an OA-child fatal retains aggregates,
`VozMin`, and equipment indexes but precedes the new `MinOA`; and a daily-loop
failure retains completed days and the current day's completed field prefix.
A cross-Zone mismatch is soft locally: CP245 completes, then CP244 can issue
its accumulated tail fatal after all other OA calls and sizing-factor EIO.

Direct same-state replay is not idempotent. Peak occupancy accumulates,
the four `*=` flow fields compound a nonunit multiplier, diagnostics repeat,
and schedule/OA-child caches or flags can change later behavior. If the final
and calculated-final references alias, each of the four fields receives the
multiplier twice per call. If `dsoaError` and `ErrorsFound` alias, a
cross-Zone assignment makes the later guard false and suppresses that same
call's OA child.

A complete CP244 replay allocates and zero-fills the final EPVectors and
refills input state, so it rebuilds rather than directly compounds those
record fields. The parent is still non-idempotent through persistent DSOA
indexes, diagnostics, and child state. `RezeroZoneSizingArrays` calls
`zeroMemberData` but does not clear all CP245 static fields, its daily five,
equipment pointers, predefined-report value, DSOA state, or schedule caches.
Manager `clear_state()` rearms only manager-owned lifecycle. A clean replay
requires coordinated reset across all owners.

### C++ evidence and uncovered branches

No C++ test calls `calcSizingOA` directly or immediately asserts one of its
owned writes. Static reachable execution is 95 calls:

- two direct-CP244 Zone roles;
- 24 Zone roles across 17 fresh `ManageSizing` contexts; and
- 48 Zone plus 21 Space roles across 34 sizing-active simulations.

This totals 74 Zone and 21 Space roles. All 95 enter with false `dsoaError`
and call the OA child. Ninety-four DSOA pointers are positive: 93 individual
objects and one valid two-member SpaceList role; one PIU role has pointer
zero. Methods are 34 Sum, 54 Flow/Person, and six Flow/Zone plus the blank
pointer-zero role. Forty-one calls match exactly one People object and 54
match none; every matched schedule maximum is positive. All effectiveness
pairs are 1.0/1.0, so all 95 take the division branch with divisor one.
Sixty-seven roles use multiplier one and 28 use multiplier ten.

The strongest descendants are assertions in HVACFourPipeBeam,
OccupantDiversity, OutputReportTabular, and Standard621 fixtures, but none
isolates CP245. There is no direct evidence for any owned scalar/config/report
or daily-array write, true `dsoaError`, cross-Zone or malformed SpaceList,
multiple People, schedule fallback or null schedule, nonunit or IEEE-special
effectiveness, negative/non-finite arithmetic, unusual multipliers, Maximum
OA, child failure, aliasing, partial prefix, retry/reset, or Space-to-Zone
daily overwrite.

### Rust boundary and status

Crate-wide searches find no exact `calcSizingOA`/`calc_sizing_oa` routine and
no CP245 Zone/Space sizing arenas, daily arrays, shared error protocol,
air-distribution effectiveness, equipment OA-index mutation,
`ZonePreDefRep.VozMin`, or sizing fanout. Typed Zone and Space records exist, and Zone carries the multiplier fields,
but authored Space/SpaceList and ZoneList/ZoneGroup execution
remain blocked. Typed People targets only a Zone, not a Space. Current
IdealLoads OA compatibility accepts a bounded direct-Zone context and
individual DSOA methods; it has no DSOA SpaceList validation, sizing-state
mutation, or equivalent per-People extrema fold. The PurchasedAir design-flow
helper is an adjacent calculation, not this transaction.

`Sizing:*`, `ZoneSizing*`, broad HVAC, authored Space/SpaceList, and zone
grouping remain run-blocked. CP245 therefore adds no algorithm-level
EnergyPlus source, Rust target/code/state, test, object support, capability,
output implementation, comparator, case, manifest, numerical, performance,
or conformance promotion. The algorithm remains `scaffold` with claim level
`none`. Inventory becomes 32 algorithms and 250 routines, split 58
`state_mapped` plus 192 `source_mapped`, with 127 required; heat-balance and
HVAC project lists become 88 and 16.

CP246 next maps `ZoneEquipmentManager::fillZoneSizingFromInput`, declared at
`ZoneEquipmentManager.hh` lines 119-126 and implemented at
`ZoneEquipmentManager.cc` lines 1208-1400.

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
