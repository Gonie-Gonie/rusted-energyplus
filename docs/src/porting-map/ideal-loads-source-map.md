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
| `ZoneEquipmentManager::fillZoneSizingFromInput` | `src/EnergyPlus/ZoneEquipmentManager.cc` | no exact Rust target; adjacent typed identities, schedules, Humidistat, individual DSOA, IdealLoads operational limits, and time-axis metadata do not implement `Sizing:Zone` field projection or sequence allocation |
| `ZoneEquipmentManager::RezeroZoneSizingArrays` | `src/EnergyPlus/ZoneEquipmentManager.cc` | no exact Rust target; Rust has no Zone/Space sizing records, sentinel-selective reset, component-load pulse lifecycle, or sizing reset latch |
| `ZoneEquipmentManager::updateZoneSizingBeginDay` | `src/EnergyPlus/ZoneEquipmentManager.cc` | no exact Rust target; adjacent run-period timing, design-day schedule labels, and standard-density IdealLoads limits do not implement current-day calculated sizing metadata |
| `ZoneEquipmentManager::updateZoneSizingDuringDay` | `src/EnergyPlus/ZoneEquipmentManager.cc` | no exact Rust target; adjacent thermostat schedules, system-timestep heat-balance averaging, demand snapshots, and IdealLoads rate timing do not implement Zone/Space sizing sequence accumulation |
| `ZoneEquipmentManager::updateZoneSizingEndDayMovingAvg` | `src/EnergyPlus/ZoneEquipmentManager.cc` | no exact Rust target; adjacent adaptive heat-balance weighting, schedule averages, output-frequency classification, and run-period time state do not implement circular Zone/Space sizing-day smoothing |
| `ZoneEquipmentManager::updateZoneSizingEndDay` | `src/EnergyPlus/ZoneEquipmentManager.cc` | no exact Rust target; current-timestep demand, IdealLoads limits/OA mixing, warmup extrema, and sizing-name detection do not implement persistent Zone/Space daily peak and cross-period final reduction |
| `ZoneEquipmentManager::updateZoneSizingEndZoneSizingCalc1` | `src/EnergyPlus/ZoneEquipmentManager.cc` | no exact Rust target; compile-time Zone/Space topology, demand snapshots, equipment load sequences, and sizing-name detection do not implement noncoincident calculated-final Space-to-Zone aggregation |
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
                      -> allocate Zone and optional Space sizing stores
                      -> per controlled Zone, fillZoneSizingFromInput for Zone
                           -> when doSpaceHeatBalanceSizing, fill stored Spaces from the same input
                           -> project asymmetric input subsets into daily/final records
                           -> dimension and zero 36 timestep sequences per destination
                      -> optionally register controlled-Zone EMS bindings
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

Across the resulting 69 static roles, six Zones and no Space enable DOAS;
13 roles (four Zones and nine Spaces) enable latent sizing. Forty-three have a
usable residual supply node, and 26 use non-air output: five Zones plus all 21
Spaces. The five Zone non-air roles comprise four
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
plus 21 Space roles. Exactly six Zone roles and no Space role enable DOAS;
the other 63 roles do not. Five HeatRecovery Zones use fixed
`ColdSupplyAir`/`CoolSup` at 12.8/15.6 C, while one OutputReportTabular Zone
defaults to auto-resolved `NeutralSupplyAir`/`NeutralSup`. Assertions inspect
only downstream results rather than these CP243 supply outputs. Exact repeated sizing, design-day iteration, and
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
- six enabled-DOAS Zones and no enabled-DOAS Space;
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

## CP246 `fillZoneSizingFromInput` Sizing-Input Projection and Sequence Allocation

CP246 adds canonical required `routine.fill_zone_sizing_from_input` after
`calc_sizing_oa` and before `sim_zone_equipment`. That is source-definition
inventory order; production executes CP246 earlier as a CP244 child before
CP245. The exact boundary is the declaration at
`ZoneEquipmentManager.hh` lines 119-126 and complete definition at
`ZoneEquipmentManager.cc` lines 1208-1400.

```cpp
void fillZoneSizingFromInput(
    EnergyPlusData &state,
    ZoneSizingInputData const &zoneSizingInput,
    Array2D<ZoneSizingData> &zsSizing,
    Array2D<ZoneSizingData> &zsCalcSizing,
    ZoneSizingData &zsFinalSizing,
    ZoneSizingData &zsCalcFinalSizing,
    std::string_view const zoneOrSpaceName,
    int const zoneOrSpaceNum);
```

The input is const. Mutable outputs are two daily arrays and two final
records. Module-state reads are limited to the sum of design and run-period
design days and manager `NumOfTimeStepInDay`.

### Production role order

The only production expressions are CP244 lines 876-883 for a Zone and
886-893 for a Space. CP244 visits controlled Zones ascending. For each Zone,
it selects the exact-name `ZoneSizingInput` or input 1 fallback and fills the
Zone. When `doSpaceHeatBalanceSizing` is true, it then fills the Zone's
`spaceIndexes` in stored order from that same parent input. This differs from
CP245's later all-Zones-then-global-Spaces order.

CP246 has no role flag and performs no Zone or Space lookup. Its caller
chooses the target stores and supplies identity. A Space role writes
`SpaceSizing(day, spaceNum)`, `CalcSpaceSizing`, and final Space records, but
the inherited field named `ZoneNum` receives the Space index. Input
`ZoneName` and `ZoneNum` are ignored, as are its object-name strings.
Space-parent consistency is not checked.

### Exact operation order

For each day in
`1..TotDesDays + TotRunDesPersDays`, CP246 performs:

1. obtain `zsSizing(day, zoneOrSpaceNum)`;
2. obtain `zsCalcSizing(day, zoneOrSpaceNum)`;
3. write normal name and numeric identity;
4. write calculated name and numeric identity;
5. write 35 normal input fields;
6. write the calculated common fields plus four latent humidity fields;
7. call normal `allocateMemberArrays`;
8. call calculated `allocateMemberArrays`.

Only after all days finish does it perform:

1. write final name/index;
2. write calculated-final name/index;
3. write the complete final input subset;
4. write the calculated-final subset;
5. allocate/zero final member arrays;
6. allocate/zero calculated-final member arrays.

A nonpositive summed day count skips every daily operation but does not skip
either final projection or allocation.

### Member-projection contract

All four destination kinds receive 37 member assignments: caller name/index
plus 35 raw input fields. The common input set comprises:

- sensible cooling/heating design supply-air methods, temperatures,
  temperature differences, and humidity ratios;
- cooling/heating airflow sizing enums, input flow, per-area constraint,
  absolute constraint, and fraction;
- heating/cooling sizing factors;
- DOAS enable, strategy, and low/high setpoints;
- Space concurrence and Zone sizing method;
- latent-sizing enable, RH setpoints, two shallow schedule-pointer copies,
  and latent design method integers; and
- heat-coil sizing method and maximum heating-to-cooling sizing ratio.

The destination `InpDesCoolAirFlow` and `InpDesHeatAirFlow` names receive
input `DesCoolAirFlow` and `DesHeatAirFlow`. No multiplier or calculation is
applied. `AutoCalcDOASControlStrategy` has already run at CP244 line 828, so
production copies its current DOAS strategy/setpoint state.

The destination-specific differences are exact:

| Destination | Writes | Suffix beyond common 37 |
|---|---:|---|
| normal daily | 37 | none |
| calculated daily | 41 | `LatentCoolDesHumRat`, `CoolDesHumRatDiff`, `LatentHeatDesHumRat`, `HeatDesHumRatDiff` |
| final | 47 | latent four; `ZoneAirDistributionIndex`; `ZoneDesignSpecOAIndex`; `ZoneADEffCooling`; `ZoneADEffHeating`; `ZoneSecondaryRecirculation`; `ZoneVentilationEff` |
| calculated-final | 45 | latent four; both indexes; both air-distribution effectiveness values |

Consequences follow directly from omission rather than explicit clearing:

- normal daily records can retain old latent humidity values/differences;
- neither daily record receives indexes or air-distribution effectiveness;
- only final receives secondary recirculation and ventilation efficiency;
- calculated-final can retain old values in those two fields; and
- only resolved OA/air-distribution indexes are copied, not the two input
  object-name strings.

Every enum and method is copied as stored. CP246 does not switch on
`AirflowSizingMethod`, `DOASControl`, `SizingConcurrence`, `ZoneSizing`, or
`HeatCoilSizMethod`; it also does not interpret the sensible/latent integer
methods. Invalid enums, out-of-range integers, negative or non-finite
numbers, and arbitrary schedule pointers have no local guard.

### Sequence allocation

Each destination projection is followed by
`ZoneSizingData::allocateMemberArrays(NumOfTimeStepInDay)`.
`DataSizing.cc` lines 280-318 dimension exactly 36 sequences to `0.0`, ordered
from `HeatFlowSeq` through `LatentHeatFlowSeq`. They cover sensible flow/load,
Zone/return/thermostat/outdoor temperature and humidity, DOAS, no-DOAS, and
latent flow/load histories.

ObjexxFCL `Array1D::dimension(range, value)` calls `assign(value)` when the
existing extent already matches, so this is also a zeroing operation on
re-entry. Each completed CP246 role invokes the helper
`2 * max(day_count, 0) + 2` times and performs 36 dimension calls per helper
invocation. Members outside the
projection are not initialized by this child.

### Validation, failure prefix, and aliasing

The day loop is the only local guard. CP246 has no bounds, allocation,
identity, topology, record-distinctness, enum, finite/nonnegative, timestep,
or old-state validation. It issues no warning, severe, fatal, or error flag
and owns no status, catch, checkpoint, transaction, cleanup, or rollback.
An already-true CP244 error therefore does not suppress projection.

For a day, both array references are obtained before the first write. Failure
obtaining the calculated reference leaves the current day untouched but
preserves prior completed days. Subsequent failure can preserve a normal
member-assignment prefix, a complete normal projection plus a calculated
member-assignment prefix, or a partially dimensioned normal/calculated
36-sequence prefix.

After the day loop, both final identities are written before final member
projection starts. Both final member-assignment blocks complete before the first
final sequence helper. A final allocation failure can therefore retain both
projected records and only a prefix of zeroed sequences. No child effect is rolled
back. A CP246 abnormal exit stops the CP244 role loop and prevents later EMS
bindings, DSOA list population, CP245 OA work, sizing-factor EIO, and the
successful setup-latch clear.

Mutable destination distinctness is not enforced:

- if daily arrays alias, the later calculated projection adds its latent four
  to the common union and the same 36 sequences are zeroed twice;
- if final references alias, the later calculated-final block does not erase
  final-only secondary recirculation or ventilation efficiency, and sequences
  are zeroed twice; and
- if a final reference aliases a daily element, the final suffix overwrites
  that record after all daily work.

Production passes separate Zone or Space stores. The caller's string view is
also not validated for lifetime or identity consistency.

### Replay and reset

With stable arguments and nonaliased valid stores, completed re-entry is
idempotent over the touched subset: assigned members overwrite deterministically
and all sequences become zero. Unlike CP245, there is no additive or
multiplicative accumulation. Re-entry after downstream sizing has populated
sequences is destructive because those histories are zeroed again.

CP246 is not a blanket record reset. It preserves every unlisted member,
including calculated-final secondary/ventilation fields and normal-daily
latent values, plus unrelated EMS, peak, OA, and calculated sizing results.
On a CP244 replay, final EPVectors are zero-filled before refilling, but
same-extent daily ObjexxFCL arrays can retain untouched members. CP246 resets
only its sequences and copied subset.

`RezeroZoneSizingArrays` uses `zeroMemberData`, which returns without change
unless `DOASSupMassFlowSeq` is allocated. A passing guard zero-fills the
current extents of 36 sequence fields and resets only 104 selected members;
CP246 identity and static input projection remain. A partial CP246 allocation
that does not reach the sentinel therefore makes the whole record a CP247
no-op. `ZoneEquipmentManager` state reset does not own the DataSizing stores,
so a clean replay still requires coordinated state-owner reset.

### C++ evidence

No C++ test calls CP246 directly, and the productive direct CP244 fixture has
no immediate assertion on a CP246-owned field. Static fresh-state role counts
are:

- two Zone roles from the AirTerminal direct CP244 fixture;
- 24 Zone roles from 17 direct `ManageSizing` contexts; and
- 48 Zone plus 21 Space roles from 34 sizing-active full simulations.

This is 95 total calls, split 74 Zone and 21 Space. The DataSizing and
MixedAir direct CP244 fixtures have no controlled role, and six direct
`SizeZoneEquipment` wrappers force setup false, so all eight contexts execute
CP246 zero times.

Normal input variation is narrow. `AccountForDOAS` is false in 89 roles.
Five HeatRecovery Zones use fixed `CoolSup` at 12.8/15.6 C, and one
OutputReportTabular Zone uses auto-resolved `NeutralSup` at 21.1/23.9 C.
Zone sizing methods are 82 `SensibleOnly`, nine `Sensible`, and four
`SensibleAndLatent`, so latent sizing is active in 13 roles. Both latent
methods are `HumidityRatioDifference` and both sizing RH schedule pointers
are null in all 95 roles. Downstream equipment, OA, diversity, tabular, and
Standard 62.1 assertions compose over CP246 but do not isolate it.

No test proves the four member-assignment sets, 36-array order/zeros, Space index in
`ZoneNum`, parent-input reuse, omitted-field persistence, zero-day final-only
path, invalid values/enums, nonnull schedule pointers, malformed extents,
aliasing, allocation failure, exact failure prefix, direct replay,
same-extent staleness, or coordinated reset.

### Rust boundary and governance

Exact crate/data searches find no `fillZoneSizingFromInput`,
`fill_zone_sizing_from_input`, `ZoneSizingInputData`, `ZoneSizingData`,
`allocateMemberArrays`, CP246 design-air/flow/latent/air-distribution field
families, or typed daily/final sizing stores. The sole raw `Sizing:Zone`
fixture expects `UnsupportedSizing`; active cases contain none.
`Sizing:*`/`ZoneSizing*` remain run-blocked.

Typed Zone/Space identity and floor area, schedules, Humidistat controls,
individual DSOA, IdealLoads operational supply temperature/humidity/flow
limits, equipment graph, sizing-checked flags, and time-axis metadata are
adjacent subsets only. IdealLoads supply limits are not `Sizing:Zone` inputs,
Autosize is rejected, and no consumer projects those fields into sizing
records. Authored Space/SpaceList, ZoneList/ZoneGroup, sizing/autosizing, and
broad HVAC remain blocked.

CP246 adds no algorithm-level EnergyPlus source, Rust target/code/state, test,
object support, capability, output implementation, comparator, case,
manifest, numerical, performance, or conformance promotion. The parent
algorithm remains `scaffold` with claim level `none`. Inventory becomes 32
algorithms and 251 routines, split 58 `state_mapped` plus 193
`source_mapped`, with 128 required; heat-balance and HVAC project lists become
88 and 17.

## CP247 `RezeroZoneSizingArrays` Pulse-to-Normal Selective Sizing Reset

CP247 adds canonical required `routine.rezero_zone_sizing_arrays` after
`fill_zone_sizing_from_input` and before `sim_zone_equipment`. That is
source-definition inventory order. The exact public wrapper is declared at
`ZoneEquipmentManager.hh` line 128 and implemented completely at
`ZoneEquipmentManager.cc` lines 1401-1430:

```cpp
void RezeroZoneSizingArrays(EnergyPlusData &state);
```

Its child mutation is `ZoneSizingData::zeroMemberData`, declared at
`DataSizing.hh` line 646 and implemented at `DataSizing.cc` lines 131-278.

### Production pulse lifecycle

The only production call expression is `SizingManager.cc` lines 400-402. A
requested Zone/AirLoop/Facility component-load summary,
`AllSummaryAndSizingPeriod`, or `AllSummaryMonthlyAndSizingPeriod` makes Zone
sizing run twice only when `DoZoneSizing` is true, at least one Zone sizing
input exists, and sizing periods are present. The first iteration sets
`isPulseZoneSizing = true`; the second is the normal pass.

After an iteration completes its environment loops, the caller performs
`UpdateZoneSizing(EndZoneSizingCalc)` and
`UpdateFacilitySizing(EndZoneSizingCalc)`, then sets
`ZoneSizingRunDone = true`. If no sizing period ran, it instead emits a severe
and sets `ErrorsFound`. Both branches then reach:

```text
if isPulseZoneSizing && runZeroingOnce
  -> RezeroZoneSizingArrays
  -> runZeroingOnce = false
```

There is no `ErrorsFound` predicate. `runZeroingOnce` defaults true and
`SizingManagerData::clear_state()` restores true. It clears only after CP247
returns normally. The child does not change the pulse flag, component-report
flag, latch, run-done flag, or errors. Successful production therefore resets
once between the pulse and normal passes; abnormal return retains a true
latch and partial state for retry. Component-load accumulation and pulse
decay arrays are separate and remain available to
`ComputeLoadComponentDecayCurve` after both passes.

### Wrapper traversal and record order

The first CP247 effect is unconditional:

```cpp
DisplayString(state, "Re-zeroing zone sizing arrays");
```

Then Zone records are visited as follows:

1. traverse `ctrlZoneNum = 1..NumOfZones`;
2. read `ZoneEquipConfig(ctrlZoneNum).IsControlled` and skip false;
3. for every `desDayNum = 1..TotDesDays + TotRunDesPersDays`, call normal
   `ZoneSizing` then calculated `CalcZoneSizing`;
4. after every day, call `CalcFinalZoneSizing` then `FinalZoneSizing`.

Only after all Zones, `doSpaceHeatBalanceSizing` gates Space processing:

1. traverse global `spaceNum = 1..numSpaces`, not stored per-Zone
   `spaceIndexes`;
2. read `space(spaceNum).zoneNum`, then select the Space only when that parent
   `ZoneEquipConfig` is controlled;
3. per day call normal `SpaceSizing` then calculated `CalcSpaceSizing`;
4. after every day, call `CalcFinalSpaceSizing` then `FinalSpaceSizing`.

There is no Space-local controlled flag, membership validation, sorting,
deduplication, or topology snapshot. A nonpositive summed day count skips
daily records but still dispatches each eligible calculated-final/final pair.
For `D = TotDesDays + TotRunDesPersDays`, `Cz` controlled Zones, and `Cs`
global Spaces whose parent Zone is controlled, a completed valid-state call
dispatches exactly:

```text
(Cz + (doSpaceHeatBalanceSizing ? Cs : 0))
    * (2 * max(D, 0) + 2)
```

child calls.

### Allocation sentinel and 36 sequence fills

`zeroMemberData` begins with its sole guard:

```cpp
if (!allocated(this->DOASSupMassFlowSeq)) {
    return;
}
```

A false guard is a successful whole-record no-op: it emits no diagnostic and
does not touch another sequence or member. In CP246
`allocateMemberArrays`, `DOASSupMassFlowSeq` is allocation step 25 of 36.
A failure before that step can therefore leave a zero-initialized allocated
prefix among steps 1-24, yet make CP247 skip the record completely. Dirty
preexisting prefixes require a separate malformed or re-entry state; they do
not follow from that sequential CP246 failure.

When the guard passes, CP247 does not allocate, redimension, deallocate, or
normalize extents. It independently `std::fill`s each current range with
`0.0`; another unallocated array is simply an empty range. The exact
36-field fill order is:

```text
DOASSupMassFlowSeq
DOASHeatLoadSeq
DOASCoolLoadSeq
DOASHeatAddSeq
DOASLatAddSeq
DOASSupTempSeq
DOASSupHumRatSeq
DOASTotCoolLoadSeq
HeatFlowSeq
HeatFlowSeqNoOA
HeatLoadSeq
HeatZoneTempSeq
DesHeatSetPtSeq
HeatOutTempSeq
HeatZoneRetTempSeq
HeatTstatTempSeq
HeatZoneHumRatSeq
HeatOutHumRatSeq
CoolFlowSeq
CoolFlowSeqNoOA
CoolLoadSeq
CoolZoneTempSeq
DesCoolSetPtSeq
CoolOutTempSeq
CoolZoneRetTempSeq
CoolTstatTempSeq
CoolZoneHumRatSeq
CoolOutHumRatSeq
HeatLoadNoDOASSeq
CoolLoadNoDOASSeq
LatentHeatLoadSeq
LatentCoolLoadSeq
HeatLatentLoadNoDOASSeq
CoolLatentLoadNoDOASSeq
LatentCoolFlowSeq
LatentHeatFlowSeq
```

Every sequence fill completes before the first nonsequence member assignment.

### Exact 104-member assignment suffix

After the sequence fields, the helper performs exactly 104 assignments in
seven source-order groups:

1. Eight strings become empty:
   `CoolDesDay`, `HeatDesDay`, `CoolNoDOASDesDay`, `HeatNoDOASDesDay`,
   `LatCoolDesDay`, `LatHeatDesDay`, `LatCoolNoDOASDesDay`,
   `LatHeatNoDOASDesDay`.
2. Forty-four `Real64` members become `0.0`:

   ```text
   DesHeatMassFlow, DesCoolMassFlow, DesHeatLoad, DesCoolLoad,
   DesHeatDens, DesCoolDens, DesHeatVolFlow, DesCoolVolFlow,
   DesHeatVolFlowMax, DesCoolVolFlowMin,
   DesHeatCoilInTemp, DesCoolCoilInTemp,
   DesHeatCoilInHumRat, DesCoolCoilInHumRat,
   DesHeatCoilInTempTU, DesCoolCoilInTempTU,
   DesHeatCoilInHumRatTU, DesCoolCoilInHumRatTU,
   HeatMassFlow, CoolMassFlow, HeatLoad, CoolLoad,
   HeatZoneTemp, HeatOutTemp, HeatZoneRetTemp, HeatTstatTemp,
   CoolZoneTemp, CoolOutTemp, CoolZoneRetTemp, CoolTstatTemp,
   HeatZoneHumRat, CoolZoneHumRat, HeatOutHumRat, CoolOutHumRat,
   ZoneTempAtHeatPeak, ZoneRetTempAtHeatPeak, OutTempAtHeatPeak,
   ZoneTempAtCoolPeak, ZoneRetTempAtCoolPeak, OutTempAtCoolPeak,
   ZoneHumRatAtHeatPeak, ZoneHumRatAtCoolPeak,
   OutHumRatAtHeatPeak, OutHumRatAtCoolPeak
   ```

3. Eight integers become zero:
   `TimeStepNumAtHeatMax`, `TimeStepNumAtCoolMax`, `HeatDDNum`,
   `CoolDDNum`, `LatentHeatDDNum`, `LatentCoolDDNum`,
   `LatentHeatNoDOASDDNum`, `LatentCoolNoDOASDDNum`.
4. Four strings become empty:
   `cHeatDDDate`, `cCoolDDDate`, `cLatentHeatDDDate`,
   `cLatentCoolDDDate`.
5. Thirty `Real64` members become `0.0`:

   ```text
   DOASHeatLoad, DOASCoolLoad, DOASSupMassFlow, DOASSupTemp,
   DOASSupHumRat, DOASTotCoolLoad,
   HeatLoadNoDOAS, CoolLoadNoDOAS,
   HeatLatentLoad, CoolLatentLoad,
   HeatLatentLoadNoDOAS, CoolLatentLoadNoDOAS,
   ZoneHeatLatentMassFlow, ZoneCoolLatentMassFlow,
   ZoneHeatLatentVolFlow, ZoneCoolLatentVolFlow,
   DesHeatLoadNoDOAS, DesCoolLoadNoDOAS,
   DesLatentHeatLoad, DesLatentCoolLoad,
   DesLatentHeatLoadNoDOAS, DesLatentCoolLoadNoDOAS,
   DesLatentHeatMassFlow, DesLatentCoolMassFlow,
   DesLatentHeatVolFlow, DesLatentCoolVolFlow,
   DesLatentHeatCoilInTemp, DesLatentCoolCoilInTemp,
   DesLatentHeatCoilInHumRat, DesLatentCoolCoilInHumRat
   ```

6. Four integers become zero:
   `TimeStepNumAtLatentHeatMax`, `TimeStepNumAtLatentCoolMax`,
   `TimeStepNumAtLatentHeatNoDOASMax`,
   `TimeStepNumAtLatentCoolNoDOASMax`.
7. Six `Real64` members become `0.0`:
   `OutTempAtLatentCoolPeak`, `OutHumRatAtLatentCoolPeak`,
   `OutTempAtLatentHeatPeak`, `OutHumRatAtLatentHeatPeak`,
   `ZoneRetTempAtLatentCoolPeak`, `ZoneRetTempAtLatentHeatPeak`.

The totals are 12 empty strings, 80 zero `Real64` values, and 12 zero
integers. No bool, enum, pointer, allocator, or extent is assigned.

### Preserved state and reset scope

This is deliberately not a blanket record reset. Among the preserved members
are:

- Zone/Space identity, `ADUName`, sizing types, input design-air
  temperatures/humidity ratios, airflow methods/limits, and sizing factors;
- DOAS enable/strategy/setpoints, concurrence, OA and air-distribution
  indexes/effectiveness, secondary recirculation, and ventilation efficiency;
- latent enable, RH constants and schedule pointers, latent method/target
  values, and heat-coil sizing method/ratio;
- all EMS override flags and values;
- People, floor-area, OA, primary-air-fraction, adjustment, `Zpz`/`Voz`,
  `ZonePeakOccupancy`, and related static/aggregate values;
- non-air results, no-OA mass/volume flows and fractions, thermostat extrema,
  and selected no-DOAS sensible peak time/day/date state;
- scalar `DOASHeatAdd` and `DOASLatAdd` even though their sequence fields are
  zeroed; and
- latent Zone temperature/humidity peak fields and every other unlisted
  member.

Thus CP247 preserves all CP246 input projection while clearing only selected
calculated state. It does not clear sizing weather, facility/terminal sizing,
equipment, component-load pulse/decay arrays, report flags, or output files.
A sentinel-missing record preserves even the nominal reset subset.

### Failure prefix and replay

There is no local allocation, bounds, topology, parent-index, extent,
day-count, or preexisting-state validation. Apart from its progress line,
CP247 owns no diagnostic or error mutation and has no status, catch,
checkpoint, cleanup, transaction, or rollback.

The progress line precedes every indexed read. A later failure retains the
exact wrapper prefix:

- completed earlier Zones;
- normal daily before calculated daily within the current day;
- all days before calculated-final, then final;
- all Zones before any Space; and
- completed earlier Spaces, while an invalid current parent Zone fails before
  that Space's record access.

Within a guard-passing child, all 36 fills precede the 104 assignment suffix.
A hypothetical suffix failure retains every sequence zero plus its member
prefix. A guard failure is not an error: the silent no-op returns and wrapper
traversal continues.

The state owns ordinary distinct arrays rather than accepting mutable record
references. No production alias contract is needed. Malformed overlapping
attached sequence storage would only receive repeated zeros. A completed
direct replay repeats the progress message and deterministically rezeros the
selected subset without changing extents; guard-skipped and unlisted members
remain stale. A production failure leaves `runZeroingOnce = true`; normal
return clears it, so the same caller skips subsequent resets until
`SizingManagerData::clear_state()` rearms the latch.

### C++ evidence

The focused `ZoneEquipmentManager_RezeroZoneSizingArrays` unit test makes one
direct call with five controlled Zones, 12 design days, three run-period
design days, four timesteps, `doSpaceHeatBalanceSizing = false`, and no
Spaces. Its wrapper dispatch is:

- 75 normal daily plus 75 calculated daily records, all guard-passing;
- five calculated-final plus five final records, all sentinel-missing and
  therefore unchanged.

For each daily kind, the fixture seeds and actively checks only 58 of the 104
assigned members and 28 of 36 sequences. The 28 are eight DOAS plus 20
sensible sequences. The unproved eight are
`HeatLoadNoDOASSeq`, `CoolLoadNoDOASSeq`, `LatentHeatLoadSeq`,
`LatentCoolLoadSeq`, `HeatLatentLoadNoDOASSeq`,
`CoolLatentLoadNoDOASSeq`, `LatentCoolFlowSeq`, and
`LatentHeatFlowSeq`.

There are 172 active assertion source lines. Across 75 Zone-day records and
four sequence slots they execute 8,700 member checks plus 16,800 sequence
checks, 25,500 total. Forty-six assigned latent/no-DOAS members are neither
seeded nor asserted. The fixture also seeds 75 intentionally preserved
members, but no active assertion checks their preservation; 154 expectation
lines are commented out, including obsolete/nonmember expectations. It does
not make final mutation or the sentinel no-op observable.

Exactly six fresh production contexts reach CP247:

- `BranchNodeConnections_ReturnPlenumNodeCheckFailure`;
- `BranchNodeConnections_ReturnPlenumNodeCheck`;
- `BaseSizer_SupplyAirTempLessThanZoneTStatTest`;
- `AirloopHVAC_ZoneSumTest`;
- `DOASDirectToZone_ZoneMultiplierRemoved`; and
- `UpdateSizing_EndSysSizingCalc`.

All six request `AllSummaryAndSizingPeriod`, execute a pulse then normal
Zone-sizing iteration, enter with a fresh true latch, call CP247 once, and
clear the latch. Two are among 18 direct `ManageSizing` contexts and four are
among 57 full-simulation contexts. Their aggregate is nine controlled Zones,
zero Spaces, 18 design-day role iterations, and zero run-period design-day
iterations. They reset 36 daily plus 18 final guard-passing records. Six
records have sequence extent 24 and 48 have extent 96, so the 36 sequence
fields contain 171,072 statically zero-filled slots.

The strongest descendant checks are Zone Component Load Summary DOAS values
598.2/600.28, an AirLoop Component Load Summary value 5080.22, normal-pass
final sizing, and final OA flow. They do not observe the intermediate reset
or prove CP247 ownership. A seventh active `AllSummaryAndSizingPeriod`
simulation requests no Zone sizing and contains no `Sizing:Zone`, so it never
reaches the caller.

There is no active proof of mixed controlled/uncontrolled selection,
controlled-parent Spaces, malformed parents, zero/nonpositive days,
sentinel-missing dirty records, guard-passing heterogeneous extents, the full
104/36 set, final mutation, preservation, exact progress output, failure
prefix, direct idempotence, second same-state sizing call, latch skip/rearm,
or retry after failure.

### Rust boundary and governance

Exact crate/data searches find no `RezeroZoneSizingArrays`,
`rezero_zone_sizing_arrays`, `zeroMemberData`, `zero_member_data`,
`isPulseZoneSizing`, `runZeroingOnce`, `ZoneSizingData`, or component-load
pulse/reset/decay transaction. There is no Zone/Space daily/final sizing
arena or sentinel-selective record reset.

Typed Zone/Space identities, equipment graph, IdealLoads scalar demand,
operational supply limits, individual OA helpers, time-axis metadata, and
`IdealLoadsInitFlags::sizing_checked` are adjacent only. They do not implement
`Sizing:Zone`, the two-pass pulse/normal sizing lifecycle, parent-controlled
global Space traversal, exact progress/latch behavior, or selective record
clearing. The sole raw `Sizing:Zone` fixture expects `UnsupportedSizing`,
active cases contain no executable sizing input or component-load summary,
and sizing/autosizing, authored Space/SpaceList, grouping, and broad HVAC
remain blocked.

CP247 adds no algorithm-level EnergyPlus source, Rust target/code/state,
test, object support, capability, output implementation, comparator, case,
manifest, numerical, performance, or conformance promotion. The parent
algorithm remains `scaffold` with claim level `none`. Inventory becomes 32
algorithms and 252 routines, split 58 `state_mapped` plus 194
`source_mapped`, with 129 required; heat-balance and HVAC project lists become
88 and 18.

## CP248 `updateZoneSizingBeginDay` Calculated Daily Metadata Seed

CP248 adds canonical required `routine.update_zone_sizing_begin_day` after
`rezero_zone_sizing_arrays` and before `sim_zone_equipment`. That is the
physical source-definition order. The exact public helper is declared at
`ZoneEquipmentManager.hh` line 132 and implemented completely at
`ZoneEquipmentManager.cc` lines 1431-1453:

```cpp
void updateZoneSizingBeginDay(
    EnergyPlusData const &state,
    DataSizing::ZoneSizingData &zsCalcSizing);
```

The helper is role-agnostic. Its sole mutable argument may be any
`ZoneSizingData`, although production supplies only current-day calculated
Zone or Space records.

### Production cadence and parent traversal

The only helper call expressions are in the `UpdateZoneSizing` `BeginDay` arm
at `ZoneEquipmentManager.cc` lines 3240-3255. The sole production expression
that selects that parent arm is `SizingManager.cc` line 307. For each Zone
sizing iteration, `SizingManager` resets `CurOverallSimDay` to zero, skips
`RunPeriodWeather` environments, advances the index for each retained sizing
day, and calls the parent once on every non-warmup day before Facility
begin-day handling and the hour, weather, and heat-balance loops. Warmup days
do not call CP248.

A component-load report makes the complete Zone sizing calculation run first
as a pulse and then normally. Both iterations start the overall-day index
again at zero, so they target the same daily records. CP247 runs between those
iterations under its own latch; the next normal `BeginDay` calls CP248 again.

Within one parent call, traversal is:

1. scan global Zone indexes from one through `NumOfZones`;
2. read the same-index `ZoneEquipConfig` and skip an uncontrolled Zone;
3. update `CalcZoneSizing(CurOverallSimDay, zone)` first;
4. only when `doSpaceHeatBalanceSizing` is true, visit that Zone's stored
   `spaceIndexes` in container order and update
   `CalcSpaceSizing(CurOverallSimDay, space)`.

This is Zone-then-its-Spaces interleaving, not CP247's all-Zones then global
Space traversal. There is no Space-local control check, parent-membership
validation, sort, deduplication, or global Space scan. Duplicate or
cross-listed stored indexes cause repeated writes. If `C` is the controlled
Zone count and `M` is the number of stored Space membership occurrences under
those Zones, one completed valid-state parent call dispatches:

```text
C + (doSpaceHeatBalanceSizing ? M : 0)
```

helpers. It touches calculated daily records only. Normal daily, final, and
calculated-final records are not selected. A nonpositive Zone count or all
uncontrolled Zones produces zero dispatches even when Spaces exist.

The parent's old numeric comment says BeginDay zeroes result arrays, but the
authoritative enum selects this metadata helper and neither the arm nor CP248
zeros an array. Other `CallIndicator` arms call separate helpers, while
invalid, system-sizing-end, and default values do not reach CP248.

### Exact 20-write transaction

There is no branch, child call, local snapshot, explicit allocation, or return
value. The source performs exactly these ordered groups:

```text
1-2   CoolDesDay, HeatDesDay
      <- EnvironmentName
3-4   DesHeatDens, DesCoolDens
      <- StdRhoAir
5-6   HeatDDNum, CoolDDNum
      <- CurOverallSimDay
7-12  CoolNoDOASDesDay, HeatNoDOASDesDay,
      LatCoolDesDay, LatHeatDesDay,
      LatCoolNoDOASDesDay, LatHeatNoDOASDesDay
      <- EnvironmentName
13-18 CoolNoDOASDDNum, HeatNoDOASDDNum,
      LatentCoolDDNum, LatentHeatDDNum,
      LatentCoolNoDOASDDNum, LatentHeatNoDOASDDNum
      <- CurOverallSimDay
19    CoolSizingType <- "Cooling"
20    HeatSizingType <- "Heating"
```

The totals are ten string assignments, two `Real64` assignments, and eight
integer assignments. `EnvironmentName` is read eight times, `StdRhoAir` twice,
and `CurOverallSimDay` eight times rather than being snapshotted. The helper
therefore accepts and copies an empty or long environment name, a negative or
non-finite density, and an arbitrary day integer on a direct call. It neither
validates role identity nor normalizes any value.

Outside the 20 named metadata members, no sequence,
load/flow/condition peak value, peak timestep/date-string field, thermostat
state, OA/DOAS load state, latent calculation state, input setting, pointer,
EMS flag, identity, extent, or allocation state is changed. CP247's guard-passing reset clears 16 of these 20 fields between
pulse and normal passes: the eight environment-name fields, both densities,
the sensible Heat/Cool day numbers, and four latent day numbers. It preserves
`CoolNoDOASDDNum`, `HeatNoDOASDDNum`, and both sizing-type strings. CP248
nevertheless overwrites all 20 on the following normal day.

### Failure prefix and replay

The helper has no local bounds, allocation-state, topology, finite-value, day,
or old-state validation. It emits no output or diagnostic, changes no error
status or latch, and owns no catch, checkpoint, cleanup, transaction, or
rollback.

A parent configuration or current-record index failure occurs before the
current helper entry. A malformed Space index occurs after its Zone and
earlier stored Spaces have completed but before that Space receives a write.
If a string assignment fails, earlier statements remain committed and the
suffix remains stale; the ordinary numeric copies add no rollback boundary.
A failure inside one helper likewise preserves all earlier Zone and Space
records.

There is one mutable record output and no ordinary output-output alias.
Production source values reside in shared environment and sizing state.
A completed replay with unchanged sources deterministically overwrites the
same 20 fields and is idempotent over that subset while every omitted field
persists. Changed source values replace the subset. Duplicate stored Space
occurrences simply replay identical current global values. A retry after
failure restarts at statement one and can repair the stale suffix if the
cause is removed.

### C++ evidence

No C++ test calls the helper directly. Two unit tests call the parent
`UpdateZoneSizing(BeginDay)` directly:

- `ZoneEquipmentManager_SizeZoneEquipment_NoLoadTest`;
- `ZoneEquipmentManager_SizeZoneEquipment_DOASLoadTest`.

Each dispatches one controlled Zone, no Space, and day index one. Both use an
empty environment name; the standard density is the default zero in the first
and `1.20` in the second. Neither seeds dirty CP248 fields nor asserts any of
the 20 writes, immediately or later.

A static fresh-state census of production-style active tests finds:

- 17 of 18 direct `ManageSizing` expressions reach Zone sizing: 33 parent
  begin-day calls and 51 Zone helper dispatches, split 43 normal and eight
  pulse;
- 34 of 57 full simulations reach sizing: 72 parent calls and 144 helper
  dispatches, comprising 102 Zone and 42 Space records;
- combined, 105 parent calls dispatch 195 helpers: 153 Zone plus 42 Space,
  split into 135 normal Zone, 42 normal Space, and 18 pulse Zone writes.

Every counted sizing period is a `SizingPeriod:DesignDay`; the census has no
warmup or run-period-design-day CP248 dispatch. The only active production
assertion that directly names a derived member is
`BaseSizer_SupplyAirTempLessThanZoneTStatTest` on final
`CalcFinalZoneSizing.HeatDesDay`, after normal sizing, end-day peak selection,
and end-of-sizing propagation. Eight `SizingManager_ZoneSizing_*` simulations
have 58 Zone/Space design-day predefined-table assertions, and two WindowAC
tests have `N/A` design-day table assertions, but all are composite
peak/report descendants rather than immediate CP248 oracles.

There is no independent assertion for the two density copies, eight day
indexes, two sizing-type literals, or the six no-DOAS/latent name families.
Mixed control, exact stored-Space order and duplicate/malformed membership,
raw invalid values, dirty overwrite, run-period multi-day cadence, explicit
warmup exclusion, pulse/normal count, failure prefix, retry, and stable replay
also remain unproved.

### Rust boundary and governance

Exact `crates` and `data` searches find no `updateZoneSizingBeginDay`,
`update_zone_sizing_begin_day`, `CurOverallSimDay`, either design density,
the design-day provenance field family, either sizing-type field, or
`ZoneSizingData`. Rust owns no current-overall-day indexed Zone/Space
calculated sizing arena, `UpdateZoneSizing` dispatcher, exact 20-field seed,
stored-Space sizing traversal, or downstream peak/report transaction.

`ep_runtime::TimeAxis` carries run-period environment and begin-day timing,
but its environment kind is only `WeatherRunPeriod`; design/sizing
environments are explicitly outside that runtime and its reported design-day
sample count is zero. Design-day schedule labels, parsed oracle EIO
environment rows, standard-density-derived IdealLoads limits, Zone/Space
identities, and the equipment graph are adjacent only.

Four active-case IDFs contain raw `SizingPeriod:DesignDay` declarations, but
each disables Zone sizing and the compatibility runtime warns and ignores the
design-day and simulation-control objects. The raw `Sizing:Zone` fixture still
expects `UnsupportedSizing`. There is therefore no executable design-day and
Zone-sizing case, and sizing remains run-blocked.

CP248 adds no algorithm-level EnergyPlus source, Rust target/code/state, test,
object support, capability, output implementation, comparator, case,
manifest, numerical, performance, or conformance promotion. The parent
algorithm remains `scaffold` with claim level `none`. Inventory becomes 32
algorithms and 253 routines, split 58 `state_mapped` plus 195
`source_mapped`, with 130 required; heat-balance and HVAC project lists become
88 and 19.

## CP249 `updateZoneSizingDuringDay` System-Substep Sizing Accumulation

CP249 adds canonical required `routine.update_zone_sizing_during_day` after
`update_zone_sizing_begin_day` and before `sim_zone_equipment`. That is the
physical source-definition order. The exact public helper is declared at
`ZoneEquipmentManager.hh` lines 134-141 and implemented completely at
`ZoneEquipmentManager.cc` lines 1455-1506:

```cpp
void updateZoneSizingDuringDay(
    DataSizing::ZoneSizingData &zsSizing,
    DataSizing::ZoneSizingData &zsCalcSizing,
    Real64 const tstatHi,
    Real64 const tstatLo,
    Real64 &sizTstatHi,
    Real64 &sizTstatLo,
    int const timeStepInDay,
    Real64 const fracTimeStepZone);
```

The complete body has three `if` statements and no child, diagnostic, explicit
allocation, or return value. Its maximum mutation is 36 assignment
statements: two conditional thermostat-extrema scalars, four unconditional
normal-daily sequence overwrites, 22 unconditional calculated-daily
accumulations, and eight additional latent-gated accumulations.

### Accepted system-substep cadence

The only helper expressions are in the `UpdateZoneSizing` `DuringDay` arm at
`ZoneEquipmentManager.cc` lines 3256-3292. The sole production parent
expression is `HVACManager.cc` line 475. It is inside
`ManageHVAC`'s `SysTimestepLoop` at lines 330-568 and under both the outer
`!WarmupFlag` branch and `ZoneSizingCalc`.

`ManageHVAC` first performs a full-zone-timestep trial before choosing the
accepted system-step count. That trial has no separate CP249 expression. If
no downstep is needed, its resulting state is accumulated once by the
one-iteration system-step loop. If downstepping is selected, each recalculated
smaller system substep reaches CP249 once. Optional optimized-condenser
`SimHVAC` repeats likewise do not independently call CP249. A top-of-loop
`stopSimulation` break or abnormal exit can suppress the current and
remaining calls.

Immediately before every production call, `HVACManager` refreshes:

```text
FracTimeStepZone = TimeStepSys / TimeStepZone
```

The parent then snapshots that fraction and calculates one zone-timestep slot:

```text
timeStepInDay = (HourOfDay - 1) * TimeStepsInHour + TimeStep
```

All accepted system substeps within that zone timestep target the same slot.
The helper neither checks that the substep fractions form a positive,
finite, unit-sum partition nor performs the division itself. The parent has
no local warmup or sizing guard, so direct callers bypass the production
cadence. Its legacy comment numbers DuringDay as two, while the authoritative
enum value is one; the enum arm, not that comment, selects CP249.

### Parent record and Space routing

For each parent call, traversal is:

1. scan global Zone indexes ascending and skip an uncontrolled Zone;
2. bind that Zone's `zoneTstatSetpts`;
3. call CP249 first with current-day `ZoneSizing` and `CalcZoneSizing`,
   together with the Zone's `FinalZoneSizing.ZoneSizThermSetPtHi/Lo`;
4. when `doSpaceHeatBalanceSizing` is true, visit the Zone's stored
   `spaceIndexes` in container order and call CP249 with current-day
   `SpaceSizing` and `CalcSpaceSizing`.

A Space call still receives the parent Zone's thermostat values and the same
parent `FinalZoneSizing` high/low references. It never receives
`FinalSpaceSizing` extrema. There is no Space-local control check, global
Space scan, sort, deduplication, stored-membership/parent validation, or
topology snapshot.

With `C` controlled Zones and `M` stored Space membership occurrences under
them, one completed valid-state parent call dispatches:

```text
C + (doSpaceHeatBalanceSizing ? M : 0)
```

helpers. With stable topology, one sizing day's helper count is that role
count multiplied by the sum of completed `SysTimestepLoop` iterations over
its zone timesteps. Duplicate or cross-listed Space indexes receive repeated
normal-sequence overwrites and repeated additive contributions. Cross-listing
can also make the final overwrite use a different parent Zone thermostat.

### Thermostat-extrema source-actual behavior

The first two possible writes are:

```cpp
if (tstatHi > 0.0 && tstatHi > sizTstatHi) {
    sizTstatHi = tstatHi;
}
if (tstatLo > 0.0 && tstatLo < sizTstatHi) {
    sizTstatLo = tstatLo;
}
```

`ZoneSizThermSetPtHi` and `ZoneSizThermSetPtLo` have declaration defaults
`0.0` and `1000.0`. For ordinary finite values, the first condition is a
strict-positive maximum-like update. The second condition executes after that
possible update and compares the low input with the current **high**, never
with the existing low. The existing `sizTstatLo` value is not read at all.
The low output is therefore the last eligible positive low below current
high, not a running minimum; a later larger low can replace an earlier
smaller one.

Equal, zero, and negative values fail their strict tests. A NaN in the
relevant comparison also makes that condition false; positive infinity
follows the ordinary strict comparisons. These extrema writes are unweighted
and precede every sequence access. In valid production storage the high and
low fields are distinct. A Zone helper always precedes its Spaces with the
same inputs and references, so a valid Space helper cannot make the high test
true after its Zone call; it can redundantly rewrite the low.

### Four normal-daily overwrites

Regardless of either extrema condition, the helper performs four unweighted
assignments to `zsSizing` in this exact order:

```text
DesHeatSetPtSeq(timeStepInDay) = tstatLo
HeatTstatTempSeq(timeStepInDay) = zsCalcSizing.HeatTstatTemp
DesCoolSetPtSeq(timeStepInDay) = tstatHi
CoolTstatTempSeq(timeStepInDay) = zsCalcSizing.CoolTstatTemp
```

These copy raw values even when a thermostat input is zero, negative, NaN, or
infinite and its extrema test failed. Multiple accepted system substeps
overwrite the same four normal-daily slots; the last completed helper for that
role and zone timestep wins. They are not fraction-weighted or averaged.

### Twenty-two unconditional calculated accumulations

Every following unconditional destination is in `zsCalcSizing`. In this exact
order, each element applies:

```text
destination(timeStepInDay) += source_scalar * fracTimeStepZone
```

The 22 destination/source pairs are:

```text
HeatFlowSeq            <- HeatMassFlow
HeatLoadSeq            <- HeatLoad
HeatZoneTempSeq        <- HeatZoneTemp
HeatOutTempSeq         <- HeatOutTemp
HeatZoneRetTempSeq     <- HeatZoneRetTemp
HeatZoneHumRatSeq      <- HeatZoneHumRat
HeatOutHumRatSeq       <- HeatOutHumRat

CoolFlowSeq            <- CoolMassFlow
CoolLoadSeq            <- CoolLoad
CoolZoneTempSeq        <- CoolZoneTemp
CoolOutTempSeq         <- CoolOutTemp
CoolZoneRetTempSeq     <- CoolZoneRetTemp
CoolZoneHumRatSeq      <- CoolZoneHumRat
CoolOutHumRatSeq       <- CoolOutHumRat

DOASHeatLoadSeq        <- DOASHeatLoad
DOASCoolLoadSeq        <- DOASCoolLoad
DOASHeatAddSeq         <- DOASHeatAdd
DOASLatAddSeq          <- DOASLatAdd
DOASSupMassFlowSeq     <- DOASSupMassFlow
DOASSupTempSeq         <- DOASSupTemp
DOASSupHumRatSeq       <- DOASSupHumRat
DOASTotCoolLoadSeq     <- DOASTotCoolLoad
```

The eight DOAS fields have no `AccountForDOAS` gate. Disabled or stale raw
DOAS scalars are still multiplied and accumulated.

### Eight latent-gated calculated accumulations

Only when `zsCalcSizing.zoneLatentSizing` is true, these eight statements
follow in exact order with the same `+= source * fraction` expression:

```text
LatentHeatLoadSeq          <- HeatLatentLoad
LatentHeatFlowSeq          <- ZoneHeatLatentMassFlow
LatentCoolLoadSeq          <- CoolLatentLoad
LatentCoolFlowSeq          <- ZoneCoolLatentMassFlow
CoolLatentLoadNoDOASSeq    <- CoolLatentLoadNoDOAS
HeatLatentLoadNoDOASSeq    <- HeatLatentLoadNoDOAS
CoolLoadNoDOASSeq          <- CoolLoadNoDOAS
HeatLoadNoDOASSeq          <- HeatLoadNoDOAS
```

The last two destinations are sensible no-DOAS load sequences but are still
inside the latent-sizing gate. A false gate leaves all eight existing
elements untouched. CP249 never writes `HeatFlowSeqNoOA` or
`CoolFlowSeqNoOA`.

The complete sequence destination count is 34: four normal-daily overwrites
plus 30 calculated-daily additive fields. A latent-false entry writes 26
sequence elements; a latent-true entry writes all 34, in addition to whichever
scalar extrema conditions succeed.

### IEEE arithmetic, reset dependency, and replay

The additive expressions use the existing destination, raw source scalar, and
raw fraction without finite, sign, range, unit, or mass normalization.
Negative or greater-than-one fractions subtract or overweight. A zero
fraction is not a semantic no-op for a NaN or infinity source because the
product can become NaN. Overflow, infinity, and NaN propagate through normal
floating-point evaluation; repartitioning and accumulation order can alter
rounding, while fused contraction is toolchain-dependent.

CP249 does not clear a destination before `+=`. CP246 initially dimensions
and zeros the sizing sequences. At the pulse-to-normal boundary and under
consistent production topology, a completed guard-passing CP247 resets every
sequence CP249 touches in both normal and calculated daily records. CP247's
global-Space traversal and CP249's stored-membership traversal do not
guarantee that reset for malformed membership; CP248 then reseeds its
disjoint metadata before CP249 refills the normal pass. CP247 does not reset either final thermostat
extrema field, so pulse high/low state carries into the normal pass. High
remains maximum-like across both passes, while later eligible normal calls
rewrite low under the high-comparison rule.

A completed replay is deliberately non-idempotent for any nonzero additive
contribution: it adds the same 22 or 30 products again. The four direct
sequence assignments and stable successful extrema assignments alone are
repeat-safe. Duplicate Space membership has the same double-add consequence.
The source relies on caller cadence and reset boundaries rather than a local
latch or transaction.

### Bounds, failure prefix, and aliasing

There is no local allocation, bounds, extent, timestep, fraction, topology,
role, or old-state validation. All 34 sequence destinations independently use
the same raw one-based index. A direct or malformed call can therefore reach
an unallocated, heterogeneous, zero, or out-of-range array. Valid arithmetic
normally does not throw, while malformed ObjexxFCL access may assert, throw,
or have unchecked behavior depending on the build.

Both extrema conditions precede the first array access. An abnormal failure
retains only its completed prefix: any extrema writes, then zero to four
normal overwrites, then completed unconditional additions, and then any
latent prefix. There is no diagnostic, status, catch, checkpoint, cleanup,
transaction, or rollback. Retry restarts at the extrema tests and first
overwrite, but repeats every already committed additive statement and can
double-count a partial prefix.

All parent argument expressions finish before helper entry, but C++ does not
provide a relative evaluation order among the record and final-field
arguments. An argument lookup failure therefore writes none of the current
helper's fields, without defining which argument was attempted first; prior
roles remain committed.

Production passes distinct normal, calculated, and final storage. The public
API nevertheless permits the two record references to alias, the high and low
references to alias each other, or a scalar reference to alias a record scalar
or sequence element. A high/low alias lets the second condition replace the
first condition's result. A scalar-to-record alias can change a later
right-hand-side value before accumulation. CP249 has no alias protection.

### C++ evidence

No C++ test calls `updateZoneSizingDuringDay` directly. Two unit tests call
`UpdateZoneSizing(DuringDay)` directly:

- `ZoneEquipmentManager_SizeZoneEquipment_NoLoadTest`;
- `ZoneEquipmentManager_SizeZoneEquipment_DOASLoadTest`.

Each uses one controlled Zone, no Space, one allocated slot,
`fracTimeStepZone = 1.0`, and latent sizing false. Starting from final
high/low defaults `0.0/1000.0`, their positive thermostat pairs `24/22` and
`23.5/22.5` make both scalar conditions true and produce those same extrema.
Neither test asserts an extrema or any CP249 sequence element. Their later
scalar and final-peak assertions span earlier sizing producers plus CP250 and
later end-day/final propagation.

Because adaptive step traces are not recorded, active-corpus calls cannot be
counted exactly. A one-accepted-system-substep-per-zone-timestep nominal floor
for completing production-style tests is:

- 17 reaching direct `ManageSizing` contexts: 4,176 parent calls and 6,384
  Zone helpers;
- 34 sizing-active among 57 full simulations: 8,112 parents and 17,040
  helpers, split 10,992 Zone plus 6,048 Space;
- combined: 12,288 parents and 23,424 helpers, split 17,376 Zone plus 6,048
  Space.

The combined floor contains 21,840 normal-pass and 1,584 pulse helpers.
Latent sizing is true for a nominal 3,744 helper entries over 13 static roles
and false for 19,680. Every retained environment is a
`SizingPeriod:DesignDay`; warmup dispatch is zero by the caller guard and the
corpus contains no run-period design-day dispatch. Adaptive downsteps can
increase every applicable count, and actual fraction values and branch
outcomes are uninstrumented.

The only test-tree expectations naming the 26 unconditional CP249 sequence
fields are 52 CP247 reset assertions over manually seeded normal and
calculated records; that test never calls DuringDay. There is no test
reference to either extrema or the eight latent-gated sequence fields.
Strongest descendants include final thermostat/peak members, one final Space
latent-cooling peak timestep, and sizing table load/flow/design-day/peak-time
cells after moving averages, maximum selection, final propagation, and
reporting. They do not isolate CP249's order, fractions, or ownership.

There is no focused proof of the exact 26/34 writes, nonzero dirty
destinations, adaptive fraction partition, last-substep overwrite, repeated
addition, low-versus-high comparison behavior, invalid IEEE inputs, bounds,
heterogeneous extents, aliases, mixed control, stored-Space ordering,
duplicate/cross-listed/malformed membership, shared parent extrema,
pulse-normal history, failure prefix, retry, or replay.

### Rust boundary and governance

Exact `crates` and `data` searches find no `updateZoneSizingDuringDay`,
`update_zone_sizing_during_day`, `FracTimeStepZone`, `ZoneSizingData`, either
thermostat extrema, or any of the 34 sequence destinations. Rust owns no
Zone/Space daily sizing arena, normal/calculated record pair, final sizing
thermostat extrema, `UpdateZoneSizing` dispatcher, stored-Space sizing
traversal, or 22-plus-eight weighted accumulation transaction.

Typed thermostat schedule links, constant-setpoint diagnostic series,
four-scalar `ZoneSysEnergyDemand`, IdealLoads rate-duration metadata, and
run-period time state are adjacent only. The heat-balance runtime has
adaptive system-step zone-air correction and fraction-weighted temperature,
humidity, and report averages, but those are run-period heat-balance fields,
not this sizing record, source set, traversal, or failure/replay lifecycle.

No active conformance-case IDF contains `Sizing:Zone`. Four contain raw
`SizingPeriod:DesignDay`, but all disable Zone, System, and Plant sizing; the
runtime records those design-day and simulation-control objects only as
ignored unsupported algorithms. The sole raw `Sizing:Zone` fixture expects
`UnsupportedSizing`, and authored Space/SpaceList and sizing workflows remain
run-blocked.

CP249 adds no algorithm-level EnergyPlus source, Rust target/code/state, test,
object support, capability, output implementation, comparator, case,
manifest, numerical, performance, or conformance promotion. The parent
algorithm remains `scaffold` with claim level `none`. Inventory becomes 32
algorithms and 254 routines, split 58 `state_mapped` plus 196
`source_mapped`, with 131 required; heat-balance and HVAC project lists become
88 and 20.

## CP250 `updateZoneSizingEndDayMovingAvg` Circular End-Day Smoothing

CP250 adds canonical required
`routine.update_zone_sizing_end_day_moving_avg` after
`update_zone_sizing_during_day` and before `sim_zone_equipment`. The exact
source-definition sequence is CP249 at `ZoneEquipmentManager.hh` lines
134-141, CP250 at line 143, and CP251 at lines 145-149; the corresponding
implementations are CP249 lines 1455-1506, CP250 lines 1508-1529, and CP251
lines 1531-1944. CP250's public signature is:

```cpp
void updateZoneSizingEndDayMovingAvg(
    DataSizing::ZoneSizingData &zsCalcSizing,
    int const numTimeStepsInAvg);
```

The complete wrapper has one `if`, no direct assignment or return value, and
at most 16 ordered `General::MovingAvg` child expressions. That child is
declared at `General.hh` line 107 and implemented at `General.cc` lines
374-393.

### Exact selected sequence set

The first 12 child calls are unconditional and target the supplied calculated
daily sizing record in this exact order:

```text
CoolFlowSeq
CoolLoadSeq
HeatFlowSeq
HeatLoadSeq
CoolZoneRetTempSeq
HeatZoneRetTempSeq
DOASHeatAddSeq
DOASLatAddSeq
CoolLatentLoadNoDOASSeq
HeatLatentLoadNoDOASSeq
CoolLoadNoDOASSeq
HeatLoadNoDOASSeq
```

There is no `AccountForDOAS` gate. All four no-DOAS load fields are in the
unconditional set even though CP249 accumulates them only inside its
`zoneLatentSizing` gate. A latent-false record can therefore have existing,
dirty, or malformed-call values in those fields smoothed rather than
preserved.

Only when `zsCalcSizing.zoneLatentSizing` is true, four more calls follow:

```text
LatentHeatLoadSeq
LatentHeatFlowSeq
LatentCoolLoadSeq
LatentCoolFlowSeq
```

Thus a latent-false helper dispatches 12 children and a latent-true helper 16.
Each successful non-no-op child replaces every element of one target array.
The wrapper writes no scalar, flag, topology, diagnostic, normal-daily record,
or final record.

Of CP249's 30 calculated-daily destinations, CP250 smooths at most 16. It
leaves these 14 untouched:

```text
HeatZoneTempSeq
HeatOutTempSeq
HeatZoneHumRatSeq
HeatOutHumRatSeq
CoolZoneTempSeq
CoolOutTempSeq
CoolZoneHumRatSeq
CoolOutHumRatSeq
DOASHeatLoadSeq
DOASCoolLoadSeq
DOASSupMassFlowSeq
DOASSupTempSeq
DOASSupHumRatSeq
DOASTotCoolLoadSeq
```

It also leaves CP249's four normal-daily thermostat sequences and the separate
`HeatFlowSeqNoOA` and `CoolFlowSeqNoOA` fields untouched.

### Source-actual circular trailing mean

Let `L = DataIn.size()` and `N = NumItemsInAvg`. The complete
`General::MovingAvg` child first applies:

```text
if N <= 1:
    return
```

That return occurs before querying the extent, allocating scratch storage, or
accessing the array. Negative, zero, and one are therefore exact no-ops for a
direct call.

For `N > 1`, the child allocates a `2 * L` scratch array. Its first loop
copies each original `x(i)` into scratch positions `i` and `L + i`, then
assigns `x(i) = 0.0`. Only after every original element has been copied and
zeroed does the second loop evaluate, in ascending `i` and then ascending
`j` order:

```text
x(i) += scratch(L - N + i + j),  j = 1..N
x(i) /= N
```

For `2 <= N <= L`, this is an inclusive circular trailing window: output
position `i` averages its original value and the preceding `N - 1` values.
The duplicated scratch halves make the first positions wrap through the last
positions of the same day. Every output uses the complete pre-call snapshot;
the in-place writes do not recursively feed later outputs.

`N = L` averages the whole day at every position, although rotated addition
order can still change floating-point rounding. `N = L + 1` remains within
the two scratch halves but includes every original value and the current value
a second time. When `L = 0`, both loops skip even for `N > 1`.

For positive `L` and `N > L + 1`, the earliest expression underflows through
the mixed unsigned-size arithmetic before ObjexxFCL indexing.
`Array1::operator()(int)` checks `assert(contains(i))` before returning raw
storage, so an invalid index terminates in an assert-enabled build and has
undefined behavior when assertions are disabled. The loops also hard-code
indexes `1..size`, so a directly constructed non-one-based array is not
supported. The child validates neither allocation, lower bound, window against
extent, nor scratch-size arithmetic.

Production sizing arrays have:

```text
L = 24 * TimeStepsInHour
```

The `Sizing:Parameters` averaging-window field is integer, has minimum one,
and has no maximum. Blank, absent, and source nonpositive fallback paths set
`NumTimeStepsInAvg = TimeStepsInHour`; fast-mode override does the same.
Sizing input warns when the window is shorter than one hour but never clamps
an oversized window to the daily extent. The fresh data-state default is zero.
A direct helper, or each CP250 child actually reached by an otherwise-valid
parent, therefore takes the no-op path. The EndDay parent itself is not a
no-op: after its first sweep completes it continues to CP251, while invalid
record or topology access can stop it before a child is reached.

Additions and division use raw floating-point values with no finite, range, or
sign check on array elements. NaN, infinity, mixed-sign infinity, overflow,
signed zero, and rounding follow the ordered source arithmetic. In exact
arithmetic the circular filter preserves the total; for finite floating-point
data, bitwise results can depend on summation order.

### EndDay parent routing and CP251 barrier

The only two helper expressions are in the `UpdateZoneSizing` `EndDay` arm at
`ZoneEquipmentManager.cc` lines 3293-3307. Its first complete sweep:

1. scans global Zone indexes ascending;
2. skips an uncontrolled Zone;
3. smooths that Zone's
   `CalcZoneSizing(CurOverallSimDay, CtrlZoneNum)`;
4. when Space sizing is enabled, smooths each stored
   `Zone(CtrlZoneNum).spaceIndexes` entry in container order through
   `CalcSpaceSizing(CurOverallSimDay, spaceNum)`.

The parent performs no Space-local control check, global Space scan, sorting,
deduplication, stored-membership validation, parent-Zone validation, or
topology snapshot. It passes the same global `NumTimeStepsInAvg` by value to
every role.

With `C` controlled Zones, `M` stored Space membership occurrences, and `R`
latent-true role occurrences, one completed valid-state parent has:

```text
H = C + (doSpaceHeatBalanceSizing ? M : 0)
CP250 helper calls = H
General::MovingAvg calls = 12 * H + 4 * R
```

A duplicate Space occurrence applies the filter repeatedly to the same
calculated record. Cross-listing under multiple controlled Zones has the same
compounding effect; there is no parent thermostat input in CP250 to
differentiate those calls. Heterogeneous target-array extents are processed
independently without an equality check.

Critically, the parent finishes this whole CP250 traversal before starting a
second complete Zone/Space traversal for CP251
`updateZoneSizingEndDay`. No role's peak selection begins while another
role still awaits smoothing. A duplicate record is therefore fully
multiply-smoothed before the first CP251 call.

CP251 selects sensible and latent peaks from the smoothed load arrays and
reads paired smoothed flow and return-temperature arrays. It also processes
the four smoothed no-DOAS load arrays. At the selected smoothed-load
timestep, however, it samples Zone/outdoor temperature and humidity arrays
that CP250 deliberately left unsmoothed. `DOASHeatAddSeq` and
`DOASLatAddSeq` are transformed but are not consumed by CP251. CP250 itself
writes no peak time, load, flow, name, or final result.

### Production cadence and pulse boundary

The sole production `UpdateZoneSizing(EndDay)` expression is
`SizingManager.cc` line 374. It executes only after all hours and zone
timesteps have completed for a non-warmup sizing day, and before
`UpdateFacilitySizing(EndDay)` or any `CurOverallSimDay` increment. An
abnormal stop before that point suppresses CP250 for the day. Unlike CP249,
there is no adaptive system-substep multiplier: each completed accepted
sizing day reaches the parent once.

The `UpdateZoneSizing` parent has no local warmup, sizing, end-day, or
current-day validity guard. Direct callers can invoke the enum arm at any
time. Its legacy comment labels EndDay as three, while the authoritative
`CallIndicator::EndDay` enum value is two.

When a component-load report is requested, the first Zone-sizing iteration is
a pulse pass and CP250 smooths every completed pulse day before pulse peak
selection and end-of-sizing propagation. Only after that whole sizing
iteration does SizingManager invoke CP247. Under consistent production
topology, a completed guard-passing CP247 zeroes every CP250 target before
the later normal pass. CP247 globally scans Space parent identities, while
CP250 follows stored Zone membership; a malformed cross-listed Space whose
actual parent is uncontrolled can be CP250-smoothed yet CP247-skipped.

Within either the pulse or normal iteration, each sizing day uses its own
`CurOverallSimDay` calculated record. The normal iteration then reuses the
same day indexes after CP247 resets the pulse sequences. The circular filter
wraps within one daily record; it does not intentionally blend different day
indexes.

### Failure prefix, retry, and identity reuse

CP250 has no diagnostic, status, catch, checkpoint, cleanup, transaction, or
rollback. Each child owns a temporary scratch array, but prior record
mutations are already committed.

A scratch-construction `std::bad_alloc` occurs before the current target is
touched; prior child arrays and prior Zone/Space roles remain transformed.
Once scratch construction succeeds, the copy-and-zero and averaging loops have
no source-defined recoverable exception path. Invalid indexing terminates at
an enabled assertion or has undefined behavior without assertions, so the
source guarantees neither a post-failure array state nor same-state retry.

Only as a source-statement-order model, a hypothetical external interruption
during the copy loop could expose an ordered zeroed prefix. Interruption during
the averaging loop could expose completed outputs, a partial current output,
and later zeros after the whole target was zeroed. Later fields and roles would
remain untouched. Those prefixes describe write order, not a recoverable C++
failure guarantee.

For defined execution, every `N <= 1` child is an exact no-op. A successfully
completed filter with `N > 1` generally composes with itself and is
non-idempotent. Mathematically constant vectors and exact whole-day means are
fixed cases, but raw floating-point resummation does not guarantee bitwise
replay stability. Re-entry after a caught scratch-allocation failure restarts
at the first Zone and first selected array: the failed current target was
untouched, while earlier completed arrays are processed again.

If scratch allocation prevents the first parent sweep from returning, the
parent has made zero CP251 calls. A later CP251 non-return occurs after every
CP250 role is fully committed. Any defined parent re-entry begins CP250 again
before retrying peak selection; invalid-index assertion or undefined behavior
provides no defined continuation.

The public helper has only one record reference and an integer by value, so it
has no two-record or output-reference alias channel. Production
`Array1D` members are distinct, and each child snapshots its own target before
writing. Duplicate or cross-listed Space identity is the material
same-record alias/replay route in the parent.

### C++ evidence

No C++ test calls `updateZoneSizingEndDayMovingAvg` directly. Two unit tests
call `UpdateZoneSizing(EndDay)` directly:

- `ZoneEquipmentManager_SizeZoneEquipment_NoLoadTest`;
- `ZoneEquipmentManager_SizeZoneEquipment_DOASLoadTest`.

Each provides one controlled Zone, no Space, latent sizing false, extent one,
and `NumTimeStepsInAvg = 1`. Each therefore dispatches 12 child expressions,
all of which return before touching the array. No post-call assertion reads
any of the 16 CP250 sequence families. Later load, temperature, peak, and
final assertions combine CP249, CP250's no-op, CP251, end-of-sizing
propagation, and other sizing producers.

The only focused child test is `General_MovingAvg`. It builds a 12-element
quadratic sequence and checks:

- all 12 values unchanged for `N = 1`;
- all 12 exact circular trailing results for `N = 2`;
- all 12 near-equal circular trailing results for `N = 4`.

Those 36 dynamic checks across seven source assertion statements prove the
generic child for three ordinary windows. They do not prove CP250's selected
fields, call order, latent gate, parent record identity, or barrier.

A fresh completed production-style census finds:

- 17 of 18 direct `ManageSizing` expressions: 33 parent calls and 51 Zone
  helpers, split 43 normal and eight pulse;
- 34 of 57 full simulations: 72 parent calls and 144 helpers, split 102 Zone
  plus 42 Space;
- combined: 105 parent calls and 195 helpers, comprising 153 Zone plus 42
  Space and 177 normal plus 18 pulse.

The exact parent window distribution is `N = 1/4/6` in counts `4/49/52`.
The exact helper distribution is `4/87/104`. The four `N = 1` helpers are
BaseSizer normal/pulse no-ops; the remaining 191 helpers execute smoothing.
The latent gate is true for 26 helpers, split eight Zone plus 18 Space, all at
`N = 6`; it is false for 169.

The resulting child-call census is:

```text
N = 1:    4 * 12                         =   48 no-op calls
N = 4:   87 * 12                         = 1,044 transformations
N = 6:   78 * 12 + 26 * 16               = 1,352 transformations
total                                      2,444 child calls
```

The two direct parent unit calls add two parents, two helpers, and 24 more
`N = 1` no-op children if combined with, rather than kept separate from, the
production-style census.

`ZoneEquipmentManager_RezeroZoneSizingArrays` has 16 source `EXPECT_EQ`
statements producing 4,800 dynamic reset checks over eight CP250-overlapping
sequence fields in normal and calculated records. That test calls CP247,
never CP250. The four unconditional no-DOAS targets and four conditional
latent targets have no direct sequence assertion anywhere in the test tree.

Eight `SizingManager` production runs use `N = 6` and assert exact Zone/Space
design load, flow, design-day, and peak-time report cells. One latent run
asserts final Space latent-cooling peak timestep 72. These are valuable
end-to-end descendants but combine CP249 accumulation, generic MovingAvg,
CP250 routing, CP251 maximum selection and companion sampling, final
propagation, and reporting. `SizingManager_OverrideAvgWindowInSizing` proves
only that fast-mode override sets the window to one; it never calls CP250.

There is no helper-level proof of all 16 fields or their order, no immediate
parent `N > 1` before/after array oracle, and no latent-false dirty-sentinel or
latent-true exact-vector oracle. Mixed control, duplicate/cross-listed or
malformed Space topology, `N <= 0`, `N = L`, `N = L + 1`, oversized windows,
empty/unallocated or non-one-based arrays, unequal extents, NaN/infinity,
overflow, rounding, scratch-allocation failure, invalid-access termination or
undefined behavior, hypothetical statement-prefix interruption, defined
re-entry, and non-idempotent replay remain unproved.

### Rust boundary and governance

Exact `crates` and `data` searches find zero occurrences of
`updateZoneSizingEndDayMovingAvg`,
`update_zone_sizing_end_day_moving_avg`, `MovingAvg`, `moving_avg`,
`NumTimeStepsInAvg`, `num_time_steps_in_avg`, `Sizing:Parameters`,
`ZoneSizingData`, `zoneLatentSizing`, or any of the 16 selected sequence
fields. Rust owns no calculated Zone/Space sizing-day arena, global sizing
window, circular trailing filter transaction, `UpdateZoneSizing` EndDay
dispatcher, stored-Space sizing traversal, or CP251 peak-selection handoff.

The adaptive heat-balance runtime does maintain weighted system-substep
temperature, humidity, and report averages. Schedule code computes fixed
interval or hourly averages, output storage classifies report variables as
`Average`, and `TimeAxis` carries run-period state. None uses CP250's source
record, daily circular wrap, selected field set, caller order, or
failure/replay lifecycle. `TimeAxis` supports only `WeatherRunPeriod` and
reports zero design-day samples.

No active conformance case contains `Sizing:Zone` or `Sizing:Parameters`.
Four active-case files contain five raw `SizingPeriod:DesignDay` objects, but
every case sets Zone, System, and Plant sizing to `No`; the runtime records
those objects only as ignored unsupported algorithms. The sole raw
`Sizing:Zone` fixture expects `UnsupportedSizing` before runtime. Sizing,
`ZoneSizing*`, and authored Space/SpaceList workflows remain run-blocked.

CP250 adds no algorithm-level EnergyPlus source, Rust target/code/state, test,
object support, capability, output implementation, comparator, case,
manifest, numerical, performance, or conformance promotion. The parent
algorithm remains `scaffold` with claim level `none`. Inventory becomes 32
algorithms and 255 routines, split 58 `state_mapped` plus 197
`source_mapped`, with 132 required; heat-balance and HVAC project lists become
88 and 21. HVAC readiness remains `0/21`, the inventory is incomplete, and
all 21 required routines remain below `family_gated`.

## CP251 `updateZoneSizingEndDay` Daily Peak and Final-Period Reduction

CP251 adds canonical required `routine.update_zone_sizing_end_day` after
`update_zone_sizing_end_day_moving_avg` and before `sim_zone_equipment`. The
physical declaration order is CP250 at `ZoneEquipmentManager.hh` line 143,
CP251 at lines 145-149, and CP252
`updateZoneSizingEndZoneSizingCalc1` at line 151. Their implementations are
CP250 at `ZoneEquipmentManager.cc` lines 1508-1529, CP251 at lines 1531-1944,
and CP252 beginning at line 1946. CP251's public signature is:

```cpp
void updateZoneSizingEndDay(
    DataSizing::ZoneSizingData &zsCalcSizing,
    DataSizing::ZoneSizingData &zsCalcFinalSizing,
    int const numTimeStepInDay,
    DataSizing::DesDayWeathData const &desDayWeath,
    Real64 const stdRhoAir);
```

The complete leaf has no EnergyPlus child routine, state argument, diagnostic,
status, or return value. It reads or writes 99 members of the current-day
calculated record and 100 members of the all-period calculated-final record,
with a 103-member union. It also reads only `Temp(index)`, `HumRat(index)`,
and `DateString` from the supplied design-day weather record. For the
description below:

```text
A = zsCalcSizing
F = zsCalcFinalSizing
T = numTimeStepInDay
W = desDayWeath
```

### Ordered current-day peak reducers

Before any scan, CP251 unconditionally assigns `F.CoolSizingType` and then
`F.HeatSizingType` from A. Those strings therefore describe the latest
successfully reached call rather than necessarily the design period that owns
a final load or flow peak.

The daily reduction then proceeds in this exact order:

1. scan sensible heating;
2. optionally scan latent heating;
3. scan all four no-DOAS loads in one loop;
4. derive sensible and optional latent heating volume/coil inputs;
5. scan sensible cooling;
6. optionally scan latent cooling;
7. derive sensible and optional latent cooling volume/coil inputs.

Every load comparison is strict `candidate > incumbent`:

| reducer | gate and candidate | writes when the candidate wins |
|---|---|---|
| sensible heat | unconditional `HeatLoadSeq(t)` | load, `HeatFlowSeq(t)` mass, Zone/outdoor/return temperature, Zone/outdoor humidity ratio, and timestep: eight fields |
| latent heat | `A.zoneLatentSizing`; `LatentHeatLoadSeq(t)` | load, both `DesLatentHeatMassFlow` and `ZoneHeatLatentMassFlow` from the same flow, latent Zone/outdoor temperature and humidity, return temperature, and timestep: nine fields |
| no-DOAS | unconditional, in heat, latent-heat, cool, latent-cool order | each winning load plus its timestep; neither `AccountForDOAS` nor the latent flag gates these eight writes |
| sensible cool | unconditional `CoolLoadSeq(t)` | the cooling counterparts of the eight sensible-heat fields |
| latent cool | `A.zoneLatentSizing`; `LatentCoolLoadSeq(t)` | load, `DesLatentCoolMassFlow`, latent Zone/outdoor temperature and humidity, return temperature, and timestep: eight fields; it does not write `ZoneCoolLatentMassFlow` |

With a reset zero incumbent, only a strictly positive value wins. Ascending
timestep ties retain the first winner. A NaN candidate never wins, while a NaN
incumbent prevents every later ordinary candidate from winning. Positive
infinity can win; negative infinity does not beat a zero incumbent.

CP250 has already smoothed sensible load, flow, and return-temperature arrays,
all four no-DOAS loads, and the enabled latent load/flow arrays. CP251 selects
those smoothed loads and pairs them with smoothed flow/return values, but it
samples the unsmoothed Zone/outdoor temperature and humidity arrays at the
selected timestep. It reads no `DOAS*Seq` field. In particular,
`DOASHeatAddSeq` and `DOASLatAddSeq`, which CP250 smooths, are not consumed.

### Volume flow and coil-inlet mixtures

A positive sensible heating mass flow executes:

```text
A.DesHeatVolFlow = A.DesHeatMassFlow / A.DesHeatDens
f = clamp(A.MinOA / max(A.DesHeatVolFlow, 0.001), 0, 1)
coil = f * W(heat-peak timestep) + (1 - f) * sensible Zone peak
```

Temperature and humidity use the same fraction and their corresponding
weather/Zone values. Sensible cooling is symmetric with the cooling fields.

The latent paths are asymmetric. Their volume flow divides
latent mass by the supplied `stdRhoAir`, but their OA fractions divide
`A.MinOA` by the corresponding *sensible* `DesHeatVolFlow` or
`DesCoolVolFlow`. Weather is sampled at the latent peak timestep, while the
Zone-side mixture uses the *sensible* `ZoneTempAtHeat/CoolPeak` and
`ZoneHumRatAtHeat/CoolPeak`, not the latent peak fields.

Source `max(a,b)` returns `b` only when `a < b`; `min(a,b)` returns `a` only
when `a < b`. A NaN first volume operand survives the first `max`, but a NaN
OA division result is the second operand of `max(0, raw)` and becomes zero,
so the nested clamp selects zero. Arithmetic does not short-circuit:
`0 * NaN` or `0 * infinity` can still make a coil result NaN. Density,
`stdRhoAir`, minimum OA, weather, and peak fields have no finite, sign, or
zero validation. A positive stored mass with a zero or stale peak index can
reach an invalid weather access.

### Cross-period calculated-final reducers

The current-day scalars are next folded into F. Ordinary flow, load, and
no-DOAS comparisons are all strict, so equal cross-day candidates retain the
prior winner. The final record is not a single atomic winning-day snapshot:

| family | primary volume winner | non-winning-volume load path |
|---|---|---|
| sensible heat | strict larger volume copies 22 fields: volume, load, mass, day, density, seven sequences, five peak companions, DD/date/time, and two coil inputs; it does not copy `HeatTstatTemp` | every `else` first overwrites `F.DesHeatDens = stdRhoAir`; a strict larger load then copies 19 more fields including the thermostat and six sequences but not volume, mass, or `HeatFlowSeq` |
| sensible cool | the symmetric 22-field copy, without `CoolTstatTemp` | the symmetric unconditional density overwrite plus 19-field load copy, excluding volume, mass, and `CoolFlowSeq` |
| latent heat | under `A.zoneLatentSizing`, strict larger volume copies 14 fields; final mass comes from `A.ZoneHeatLatentMassFlow`, and no outdoor latent peak is copied | strict larger load copies only load, date, DD number, timestep, load sequence, and flow sequence; it omits `LatHeatDesDay`, peaks, coil inputs, mass, and volume |
| latent cool | under `A.zoneLatentSizing`, strict larger volume copies 14 fields; mass comes from `A.DesLatentCoolMassFlow`, and no outdoor latent peak is copied | strict larger load copies load, date, DD number, `LatCoolDesDay`, timestep, and load sequence; unlike latent heat it omits the flow sequence |
| four no-DOAS loads | each strict larger load copies scalar, whole sequence, DD number, day name, and timestep | there is no alternate branch and no `c*DDDate` copy |

A larger-volume day can therefore replace an earlier larger load with a
smaller associated load. Conversely, a lower or equal volume day with a
larger load replaces most load companions while retaining the earlier
volume, mass, and flow sequence. The result can be a source-permitted hybrid of
multiple periods. Even when the load also loses, merely taking a sensible
volume `else` overwrites the prior winning density with the current
`stdRhoAir`.

There is no `AccountForDOAS` branch. The adjusted sensible arrays and the four
no-DOAS arrays are reduced independently. Dirty no-DOAS values can win even
when latent sizing and DOAS accounting are false.

### Exact zero-load fallbacks

After every ordinary final reducer, CP251 applies four separate fallbacks:

| guard | current-day selection | cross-period decision and final writes |
|---|---|---|
| `F.DesHeatLoad == 0` | a local `FirstIteration` forces timestep one, then strict lower `HeatZoneTempSeq` wins; ties retain the first timestep | inclusive paired outdoor `A.OutTempAtHeatPeak <= F.OutTempAtHeatPeak`; a win copies day, five companion sequences, five peaks, DD/date/time, two coil fields, and thermostat, but no load/flow sequence |
| `F.zoneLatentSizing && F.DesLatentHeatLoad == 0` | strict lower heating Zone temperature after a forced first sample writes only A latent Zone temperature and paired outdoor temperature/humidity | an independent test of every `HeatOutTempSeq(t) <= F.OutTempAtLatentHeatPeak` writes only F outdoor temperature/humidity, latent day/DD/date, and the stale or prior `A.TimeStepNumAtLatentHeatMax`, not `t`; no final latent Zone peak or sequence is copied |
| `F.DesCoolLoad == 0` | forced-first then strict higher `CoolZoneTempSeq`; ties retain the first timestep | strict paired outdoor `A.OutTempAtCoolPeak > F.OutTempAtCoolPeak`; a win copies the same 17-field shape as sensible heat |
| `F.zoneLatentSizing && F.DesLatentCoolLoad == 0` | a running strict maximum Zone temperature writes only A latent Zone temperature and paired outdoor temperature/humidity | every timestep tests that running paired outdoor value `>= F.OutTempAtLatentCoolPeak`, but a win writes only latent day/DD/date and the stale or prior `A.TimeStepNumAtLatentCoolMax`; CP251 never updates F's latent-cool outdoor threshold, so this is not a maximum reducer |

The sensible heat cross-day comparison is inclusive, so a later equal
outdoor condition replaces an earlier one; sensible cooling is strict and
keeps the earlier tie. The latent-heating outdoor scan is inclusive and later
equal samples win. The latent-cooling threshold remains its prior/default
value while qualifying metadata can be overwritten repeatedly. Default final
outdoor thresholds are zero unless earlier setup or mutation changes them.

Both signed zeros satisfy each `== 0` guard. A forced first temperature sample
is accepted even when NaN and then freezes the strict within-day comparison.
Any final comparison involving NaN is false. When `T <= 0`, all nine possible
loops skip, but the string, derived-scalar, final-reducer, and stale-scalar
fallback logic still executes; the helper is not a no-op.

### Parent routing, cadence, and downstream boundary

The only parent expressions are in `UpdateZoneSizing(EndDay)` at
`ZoneEquipmentManager.cc` lines 3317-3328. Only after the entire CP250
smoothing traversal returns does the parent start a second traversal:

1. scan Zone indexes ascending;
2. skip an uncontrolled Zone;
3. pass current-day `CalcZoneSizing(day, zone)` and the persistent
   `CalcFinalZoneSizing(zone)`;
4. when Space sizing is enabled, visit that Zone's stored `spaceIndexes` in
   container order and pass the analogous current-day/final Space pair;
5. pass the common `NumOfTimeStepInDay`, current `DesDayWeath`, and
   `StdRhoAir` to every role.

There is no Space-local control check, global Space scan, sorting,
deduplication, membership validation, parent validation, or topology
snapshot. With `C` controlled Zones and `M` stored Space membership
occurrences:

```text
H = C + (doSpaceHeatBalanceSizing ? M : 0)
CP251 helper calls per completed EndDay parent = H
```

Duplicate or cross-listed Space identity reduces the same current-day and
final records repeatedly. All duplicate CP250 smoothing has already completed
before the first CP251 call, so the first CP251 call observes the fully
multiply-smoothed arrays. The second CP251 call can then take the successful
replay density-overwrite path.

The sole production parent call is `SizingManager.cc` line 374. It is reached
once after all timestep work for each completed non-warmup sizing day, before
facility EndDay processing and before the current-overall-day increment.
`UpdateZoneSizing` itself has no local warmup, sizing, EndDay, or day-validity
guard, so direct callers can bypass that cadence.

At the end of all periods, `UpdateZoneSizing(EndZoneSizingCalc)` first runs
EMS and applies Zone final overrides. Inside the non-pulse block, Space sizing
makes the parent visit controlled Zones and skip exactly `numSpaces == 1`
before calling physical-next CP252. CP252 returns for Coincident concurrence;
otherwise it rebuilds the Zone final from stored Spaces. A malformed
zero-Space Zone is not excluded by the parent. Later EndZone helpers,
reporting, and calculated-to-user final copies remain separate downstream
owners; CP251 itself writes no report or user-final record.

### Pulse reset, failure, retry, and aliasing

A component-load request runs CP251 during the pulse sizing iteration. A
successfully returned pulse iteration later reaches CP247 before the normal
iteration reuses the same day indexes. Under valid allocated topology CP247
clears most CP251 state, but `ZoneSizingData::zeroMemberData` does not clear
these CP251-written fields in daily or final records:

- `TimeStepNumAtHeatNoDOASMax` and `TimeStepNumAtCoolNoDOASMax` in both
  daily and final records, plus `HeatNoDOASDDNum` and `CoolNoDOASDDNum` in
  final records (CP251 reads the daily DD numbers and writes the final ones);
- latent heat/cool Zone peak temperature and Zone peak humidity;
- sizing-type strings, which normal CP248/CP251 overwrite.

Consequently, a normal path with no new winner can retain pulse metadata.
The latent zero-load fallbacks repair neither all latent companions nor their
timestep. No current test has a latent pulse role. CP247 globally selects
Spaces by their actual parent identity, while CP251 follows stored
membership; malformed cross-listing under a controlled Zone can therefore
reach CP251 yet evade the reset.

CP251 has no validation, diagnostic, status, catch, checkpoint, cleanup,
transaction, or rollback. `T` can disagree with every independently sized
array. Out-of-range sequence or weather access assert-terminates when
assertions are enabled and has undefined behavior otherwise, so it provides
no defined post-failure continuation. `T <= 0` can still expose stale positive
mass and a stale peak index to weather.

String and whole-array copies can allocate. A final winner installs its
strict decisive volume, load, or no-DOAS scalar before later strings and
arrays. If a later allocation throws, a defined retry sees equality and can
skip the unfinished companion copy permanently. The sensible cooling
zero-load path similarly installs its strict outdoor threshold before its
date string; a later allocation failure can make replay skip the rest. Heat's
inclusive fallback remains replay-eligible, as do latent heat's inclusive
threshold and latent cool's never-written threshold.

Even a fully successful repeated direct call is not generally idempotent. A
first sensible volume winner copies A's density; the repeated equal volume
takes the `else` and overwrites F density with `stdRhoAir`. Parent re-entry
first reruns CP250, so its smoothing can also compose before CP251 starts.

The two record references are not required to be distinct. If A and F alias,
all ordinary final strict comparisons become self-comparisons and fail,
sensible density is overwritten with `stdRhoAir`, and final/no-DOAS winner
copies collapse. The heat zero-load `<=` self-test can pass, the cool `>`
self-test fails, and latent fallback reads and writes interact inside the same
loop. Production supplies distinct records; malformed direct aliasing is an
unvalidated in-place hybrid.

### C++ evidence

No C++ test calls `updateZoneSizingEndDay` directly. Two
`ZoneEquipmentManager` unit tests call `UpdateZoneSizing(EndDay)` directly,
each with one Zone, no Space, `T = 1`, averaging window one, and latent sizing
false. Their four apparent calculated-final sensible peak assertions occur
only after `EndZoneSizingCalc`; later helper 7 rewrites both calculated-final
peaks. They prove parent reach and an integrated no-load outcome, not an
isolated CP251 destination.

Stronger successful-path evidence includes:

- `BaseSizer_SupplyAirTempLessThanZoneTStatTest`, whose full simulation
  produces positive heating load with zero design flow and pins the
  CP251 load-fallback-owned calculated-final thermostat, design day, and
  positive load; zero volume/mass establish the zero-flow context, while
  helper 7 later owns the asserted Zone peak;
- the latent Space sizing test, which pins
  `CalcFinalSpaceSizing.TimeStepNumAtLatentCoolMax == 72`; downstream latent
  mapping reads but does not overwrite that latent index;
- seven Space-heat-balance sizing simulations whose exact Space load, flow,
  design-day, and peak-time report assertions cover the integrated
  CP249/CP250/CP251 and later EndZone/report chain.

A fresh completing production-style census finds:

```text
direct ManageSizing contexts: 33 parents, 51 Zone helpers
full ManageSimulation contexts: 72 parents, 102 Zone + 42 Space helpers
combined: 105 parents, 195 helpers
role split: 153 Zone + 42 Space; 177 normal + 18 pulse
```

Helper counts at `TimeStepsInHour = 1/4/6` are `4/87/104`, giving extents
`24/96/144` and 23,424 role-timepoints. The 26 latent-true helpers, split
eight Zone plus 18 Space, all have extent 144 and add 3,744 latent
role-timepoints. Before any zero-load fallback, the fixed five daily scans
therefore execute 77,760 loop-body iterations and 148,032 load comparisons.
The two direct parent tests add two helpers and 12 ordinary comparisons if
combined rather than kept separate.

Six unique DOAS-enabled production Zones contribute 14 CP251 helper calls,
all latent false. CP251 has no DOAS gate and reads no DOAS sequence; those
tests assert distant DOAS/heat-recovery descendants. Because CP249's four
no-DOAS producers are inside the latent gate, these DOAS cases do not exercise
a positive no-DOAS winner. No test asserts a CP251 final no-DOAS field.

There is no focused oracle for all daily/final branch shapes or source order.
Equal peaks, NaN/infinity, density zero, latent heating, either zero-load
latent fallback, lower-flow/higher-load hybrid records, density clobber,
latent coil asymmetry, dirty no-DOAS values, duplicate/cross-listed topology,
mismatched extents, stale weather indexes, record aliasing, allocation
failure, torn winner state, defined retry, and parent re-entry remain
unproved. The reset test does not seed or assert the omitted pulse fields.

### Rust boundary and governance

Exact `crates` and `data` searches find no CP251 helper or canonical key, no
Zone/Space calculated daily/final sizing record, no EndDay peak reducer, and
no exact counterpart for any of the 103 accessed members. Searches for all
103 member tokens and their snake-case candidates are zero; the only `MinOA`
substring hits belong to unrelated `CalcPurchAirMinOAMassFlow` names.

Rust `ZoneSysEnergyDemand` is a current-timestep four-scalar snapshot.
IdealLoads limits and outdoor-air mixing are current-timestep equipment
logic. Warmup tracks a separate Zone temperature extremum, and the retained
`DesignSpecification:ZoneHVAC:Sizing` name only detects unsupported
autosizing. Radiation/CLI `.max_by` uses, report-frequency peak labels, and
PurchasedAir rate outputs do not implement CP251's persistent design-day
reducers or output descendants.

Active conformance has zero `Sizing:Zone`, `Sizing:Parameters`, authored
Space, or authored SpaceList objects. Four files contain five design-day
objects, but all set Zone, System, and Plant sizing to `No`; 74 active
`SimulationControl` declarations likewise have Zone sizing `No` and none
has `Yes`. The sole raw `Sizing:Zone` fixture expects `UnsupportedSizing`
before runtime. Sizing, `ZoneSizing*`, authored Space/SpaceList, and their
reports remain run-blocked.

CP251 adds no algorithm-level EnergyPlus source, Rust target/code/state, test,
object support, capability, output implementation, comparator, case,
manifest, numerical, performance, or conformance promotion. The parent
algorithm remains `scaffold` with claim level `none`. Inventory becomes 32
algorithms and 256 routines, split 58 `state_mapped` plus 198
`source_mapped`, with 133 required; heat-balance and HVAC project lists become
88 and 22. HVAC readiness remains `0/22`, the inventory is incomplete, and
all 22 required routines remain below `family_gated`.

## CP252 `updateZoneSizingEndZoneSizingCalc1` Noncoincident Space Aggregation

CP252 adds canonical required
`routine.update_zone_sizing_end_zone_sizing_calc1` after
`update_zone_sizing_end_day` and before `sim_zone_equipment`. The physical
declaration sequence is CP251 at `ZoneEquipmentManager.hh` lines 145-149,
CP252 at line 151, and CP253 `updateZoneSizingEndZoneSizingCalc2` at line 153.
The complete CP252 body is `ZoneEquipmentManager.cc` lines 1946-2278:

```cpp
void updateZoneSizingEndZoneSizingCalc1(EnergyPlusData &state,
                                        int const zoneNum);
```

The leaf has no EnergyPlus child, diagnostic, output, status, catch, or return
value. It writes 92 distinct calculated-final Zone members: 54 can be touched
outside latent processing and 38 more are latent-gated. Its first-Space and
per-Space source union is also 92 members. Including the two target-only gate
members and source-only `ZoneHeatLatentMassFlow`, the routine accesses 95
unique sizing-record member names. It has six explicit `for` loops, four
unconditional `std::max_element` scans, and four more maximum scans under the
Zone latent flag.

### Parent order and the exact concurrence gate

The sole production selector is `SizingManager.cc` line 391, which calls
`UpdateZoneSizing(EndZoneSizingCalc)` only after at least one sizing period
has completed. Inside that parent, source order is:

1. call Zone-sizing EMS, then independently apply each of six Zone
   calculated-final volume, mass, and load overrides only when EMS is
   present, that actuator's flag is on, and its preoverride target is strictly
   positive;
2. enter the following block only when `isPulseZoneSizing` is false;
3. when Space sizing is enabled, visit controlled Zones in ascending index
   order;
4. skip exactly a Zone whose stored `numSpaces == 1`;
5. call CP252 for every other selected Zone;
6. only after the entire CP252 traversal completes, visit every controlled
   Zone and optional stored Space through physical-next CP253;
7. write ZSZ/SPSZ, perform later latent selection, and eventually run the
   calculated-to-user final-copy helpers.

CP252 binds `CalcFinalZoneSizing(zoneNum)` before its only leaf guard:

```text
if F.spaceConcurrence == Coincident:
    return without mutation
otherwise:
    rebuild the mapped subset, including for Invalid or malformed enum values
```

Thus a normally completing eligible NonCoincident Zone resets and rebuilds
all six EMS-adjustable Zone volume, mass, and load fields from Space records,
replacing any override that was actually applied. Coincident and
exactly-one-Space paths preserve them. The leaf itself checks none of the
parent conditions: no Zone bound, controlled status, pulse state, Space-sizing
flag, `numSpaces`, list length, membership parent, duplicate, cross-listing,
Space latent flag, or record extent is validated.

The parent tests `Zone.numSpaces`, but CP252 later indexes
`Zone(zoneNum).spaceIndexes[0]`. A malformed zero-Space or inconsistent Zone
is therefore not protected by the `numSpaces == 1` skip. Stored list order and
multiplicity are authoritative: duplicates contribute repeatedly and a
cross-listed Space contributes to every Zone that stores it. The local
`numSpaces` counter increments once per occurrence but is never consumed.

For the remaining description:

```text
F = CalcFinalZoneSizing(zoneNum)
S = stored Space membership occurrences
T = NumOfTimeStepInDay
L = 1 when F.zoneLatentSizing is true, otherwise 0
```

### Ordered reset prefix

After the Coincident return, CP252 zeroes this exact target subset before
reading the first Space:

| group | unconditional reset | additional reset when `F.zoneLatentSizing` |
|---|---|---|
| simple scalar sums | eight heat/cool volume, load, mass, and no-DOAS load fields | eight latent heat/cool volume, mass, load, and no-DOAS load fields |
| scalar weighted numerators | 16: density, five peak conditions, and two coil-inlet values for sensible heat and cool | ten latent Zone/return peak and coil-inlet values |
| per-timestep arrays over `1..T` | 16: flow, load, no-DOAS load, and five condition arrays for heat and cool | six latent load, flow, and no-DOAS load arrays |

This is not a whole-record reset. Thermostat values, sizing
labels/configuration, latent outdoor peak temperature and humidity,
`ZoneHeatLatentMassFlow`, `ZoneCoolLatentMassFlow`, DOAS state, EMS
flags/values, identity/input fields, and many other members retain their
pre-CP252 Zone values.

### First-Space metadata seed

Only after that numeric prefix does CP252 take the first stored Space without
an emptiness check. The first Space unconditionally seeds 11 fields:

- sensible heat day, DD number, date, and timestep;
- sensible heat no-DOAS DD number, day, and timestep;
- sensible cooling day, DD number, date, and timestep.

The latent block seeds another 17 fields: four latent-heating peak fields,
three latent-heating no-DOAS fields, four latent-cooling peak fields, three
latent-cooling no-DOAS fields, and, anomalously, the three *ordinary sensible*
cooling no-DOAS fields. Consequently, when Zone latent sizing is false,
`CoolNoDOASDDNum` and `CoolNoDOASDesDay` are not initialized from the first
Space. Their later consensus begins from incoming Zone state. All copied
timestep fields are overwritten by maximum scans if the normal tail is
reached; the copies remain observable only in an interrupted prefix.

### Stored-Space fold

The Space loop includes the first Space again. For every occurrence, CP252
performs these source-ordered reductions:

| group | exact operation |
|---|---|
| sensible simple sums | add heat/cool volume, load, mass, and no-DOAS load: eight scalars |
| sensible scalar numerators | add each Space density, five peak companions, and two coil inputs multiplied by that Space's corresponding design mass flow: 16 products |
| sensible timestep sums | add heat/cool flow, load, and no-DOAS load arrays: six additions per timestep |
| sensible timestep numerators | add five heat conditions times Space heat flow and five cool conditions times Space cool flow: ten products per timestep |
| ordinary metadata | reconcile heat, heat no-DOAS, cool, and cool no-DOAS DD groups |
| latent simple sums | under `L`, add eight volume/mass/load/no-DOAS fields |
| latent scalar numerators | under `L`, add five latent-heat and five latent-cool peak/coil products |
| latent metadata | under `L`, reconcile four latent DD groups |
| latent timestep sums | under `L`, add six load/flow/no-DOAS arrays per timestep |

The latent scalar formulas preserve two important asymmetries:

```text
F.DesLatentHeatMassFlow =
    sum(Space.ZoneHeatLatentMassFlow)

latent-heat numerator(x) =
    sum(Space.x * Space.ZoneHeatLatentMassFlow)

F.DesLatentCoolMassFlow =
    sum(Space.DesLatentCoolMassFlow)

latent-cool numerator(x) =
    sum(Space.x * Space.DesLatentCoolVolFlow)
```

The five latent-cooling Zone/return peak and coil-inlet numerators therefore
use Space volume flow, while the later denominator is summed Space mass flow.

### Design-day consensus is a one-way latch

Each of four ordinary and four latent checks compares only a DD number.
Names, dates, and copied timesteps are never compared independently. A check
runs only while the current Zone DD is nonzero and clears on the first
different Space DD:

- ordinary or latent peak groups become day `"N/A"`, DD `0`, and date `""`;
- no-DOAS groups become DD `0` and day `"N/A"`.

Once zero, all later mismatches are ignored. If the first Space DD is already
zero, disagreement from every later Space is also ignored, so first-Space
names or dates can coexist with a zero DD. The nonlatent sensible-cooling
no-DOAS anomaly is stronger: its result can depend entirely on the incoming
Zone DD/name rather than the first Space.

### Normalization and raw denominator behavior

After every Space occurrence has been folded, CP252 normalizes in this order:

1. if summed sensible heating mass is strictly positive, divide the eight
   heating density/peak/coil numerators by that mass;
2. do the symmetric eight cooling divisions only for strictly positive
   cooling mass;
3. for each `t = 1..T`, divide five heat condition arrays by summed heat flow
   only when that flow is positive, then do the same for cooling;
4. compute the four ordinary peak indexes described below;
5. under `L`, divide five latent-heating numerators by summed
   `ZoneHeatLatentMassFlow`;
6. divide five latent-cooling *volume-weighted* numerators by summed latent
   cooling *mass* when that mass is positive;
7. compute the four latent peak indexes described below.

Loads, flows, and no-DOAS arrays remain sums. Zero, either signed zero,
negative, and NaN denominators skip their division, leaving the raw weighted
numerator rather than an average. Positive infinity enters division. There is
no finite, sign, unit, or cancellation validation on any Space operand or
accumulator.

### Eight full-extent peak-index scans

The four ordinary scans run before latent normalization, and the four
latent-gated scans run after it. Each replaces copied timestep metadata with a
one-based index:

```text
1 + distance(array.begin(), max_element(array.begin(), array.end()))
```

The four ordinary scans and four latent-gated scans use:

| result | scanned target array |
|---|---|
| `TimeStepNumAtHeatMax` | `HeatLoadSeq` |
| `TimeStepNumAtHeatNoDOASMax` | `HeatLoadNoDOASSeq` |
| `TimeStepNumAtCoolMax` | `CoolLoadSeq` |
| `TimeStepNumAtCoolNoDOASMax` | `CoolLoadNoDOASSeq` |
| `TimeStepNumAtLatentHeatMax` | `LatentHeatFlowSeq`, notably flow rather than load |
| `TimeStepNumAtLatentHeatNoDOASMax` | `HeatLatentLoadNoDOASSeq` |
| `TimeStepNumAtLatentCoolMax` | `LatentCoolLoadSeq` |
| `TimeStepNumAtLatentCoolNoDOASMax` | `CoolLatentLoadNoDOASSeq` |

Every scan covers that array's complete allocated extent, not `1..T`.
Ordinary finite ties retain the first maximum, and a nonempty all-zero array
selects index one. Floating NaN violates the strict-order requirement used by
the standard algorithm, so no portable NaN selection rule is claimed.

The summed scalar noncoincident loads and flows remain sums of independent
Space peaks. CP252 does not replace them with maxima of the aggregated
sequences. The recomputed timestep can therefore describe a different
coincident sequence peak, while DD metadata describes consensus among
independent Space scalar peaks.

For positive `T`, the six explicit loops execute:

```text
S + T * (2 + L + S * (1 + L))
```

loop bodies, before the `4 + 4L` hidden linear maximum scans over their
independent full extents. This includes the `S` outer Space iterations.

### Preserved hybrid state and downstream ownership

A successful NonCoincident call can combine:

- summed Space scalar peaks;
- mass/flow-weighted Space peak conditions;
- full-extent maxima of aggregate sequences;
- consensus, zeroed, or stale day metadata;
- untouched pre-CP252 Zone fields.

CP252 mutates only the calculated-final Zone record. It does not change any
Space record, user-final Zone record, report, or output stream. CP253 later
checks every controlled Zone and stored Space, emits its own diagnostics, and
formats peak timestamps. ZSZ/SPSZ output and the later calculated-to-user
copies are separate owners. Those stages can overwrite or transform
descendants, so their report values are not all isolated CP252 fields.

### Invalid state, failure, retry, and pulse behavior

CP252 has no validation, transaction, rollback, or cleanup:

- `T <= 0` skips every timestep reset, fold, and normalization loop, but
  scalar reset/fold, metadata mutation, and full-array maximum scans still
  execute;
- if `T` is smaller than a target extent, untouched target tail values can
  win a maximum;
- if `T` exceeds any target or Space extent, indexed access triggers an
  assertion when enabled or has undefined behavior in an unchecked build after
  the ordered prefix;
- empty or unallocated iterator ranges have no locally validated result
  contract;
- floating additions and multiplications preserve source order, so NaN,
  infinity, overflow, cancellation, and duplicate-order effects propagate;
- an empty membership list reaches unchecked `[0]` only after numeric reset.

First-Space strings and mismatch labels can allocate. An exception preserves
every earlier reset, sum, copy, and consensus mutation. Every no-DOAS mismatch
arm assigns DD `0` before assigning `"N/A"`, so failure in that later string
assignment leaves a torn label during the failing invocation. On retry, heat
no-DOAS and both latent no-DOAS groups are reseeded from the first Space before
their comparisons. Only ordinary cooling no-DOAS with latent sizing false
lacks that reseed, so its retained zero latch can skip the unfinished label
repair.

With stable valid topology and matching extents, a completed replay normally
resets and reconstructs the touched numerical subset rather than compounding
it. It is not a full repair boundary: untouched fields, target tails, and the
nonlatent ordinary cooling no-DOAS torn-label state can persist. Production
owns separate Zone and Space arenas, so the direct
record-reference alias accepted by CP251 is unavailable here; duplicate and
cross-listed Space identities are the material alias-like cases.

The whole CP252/CP253/report block is skipped during pulse EndZone processing.
The parent still runs EMS and later calculated-to-user helpers on that pulse
entry. After CP247 zeroing, the normal EndZone pass can run CP252, and any
Space state omitted by the reset remains eligible to enter its aggregation.
Direct callers can bypass the pulse and Space-sizing gates.

### C++ evidence

No C++ test calls CP252 directly. The two direct
`ZoneEquipmentManager` EndZone parent tests set pulse sizing true immediately
before the call, so both dispatch zero CP252 helpers.

A fresh completing production-style census finds 57 EndZone parent entries:

```text
direct ManageSizing: 19 = 17 normal + 2 pulse
full ManageSimulation: 38 = 34 normal + 4 pulse
combined: 51 normal + 6 pulse
```

The direct contexts have no Space sizing. Among the 48 normal full-simulation
controlled-Zone roles, exactly seven enable Space sizing with more than one
Space. All seven call CP252 with `T = 144`, one controlled Zone, and three
unique stored Spaces:

- five are Coincident and return immediately;
- two are NonCoincident, latent false, and complete the body;
- three of the five Coincident returns are latent true; both NonCoincident bodies are latent false.

The two body completions contribute six Space-loop visits and 1,440 explicit
timestep-loop iterations. Their eight ordinary `max_element` calls scan 1,152
elements and make 1,144 ordinary comparisons.

`SizingManager_ZoneSizing_NonCoincident1` uses one design day. Its downstream
Zone calculated cooling load and volume are the three Space sums; the common
day remains named while the aggregate sequence peak formats as
`7/21 16:00:00`. `SizingManager_ZoneSizing_NonCoincident2` gives the three
Spaces different design days. It asserts the summed Zone load/volume, a Zone
day of `"N/A"`, and a time-only `16:00:00`. These are strong full-chain
descendants of CP252 sums, DD consensus, and maximum selection. Five
Coincident tests retain Zone calculated values distinct from Space sums,
providing descendant evidence of the early return.

The two body tests have zero positive heating, no latent sizing, DOAS, EMS, or
pulse. No immediate test inspects the CP252 target. Weighted density and
condition fields, mass sums, individual arrays, all no-DOAS fields, the
latent volume/mass denominator mismatch, latent-flow heat maximum, EMS
override loss, one-Space guard, malformed/duplicate/cross-listed topology,
mixed control, invalid concurrence, extent mismatch, IEEE-special values,
failure prefixes, completed replay, and failure retry remain unisolated.
The asserted zero-heating load/flow and N/A day/time cells come from
`reportZoneSizing`'s no-flow branch. Helper 7 separately owns the unasserted
calculated-final zero-heating Zone peak conditions, so neither provides an
isolated CP252 oracle.

### Rust boundary and governance

Exact `crates` and `data` searches find no CP252 helper or canonical key, no
`SizingConcurrence`, `spaceConcurrence`, `NonCoincident`,
`CalcFinalZoneSizing`, or `CalcFinalSpaceSizing`, and no counterpart for any
of the 95 unique sizing-record member names in exact-token or snake-case
form.

Rust typed `Zone.spaces` and `Space` arenas are compile-time topology only.
`ZoneSysEnergyDemand`, equipment-list load sequences, design-day report
counters, `AutosizeOrNumber`, and the retained ZoneHVAC sizing-object name are
adjacent but do not provide a production Space sizing consumer,
calculated-final Zone/Space arena, concurrence gate, weighted reducer, or
peak metadata consensus.

Active data contain no `Sizing:Zone`, `Sizing:Parameters`, authored Space or
SpaceList, `NonCoincident`, Space-sizing enablement, or Zone-sizing-enabled
`SimulationControl`. `unsupported_sizing` and
`unsupported_space_partitioning` remain `run_blocked`; the raw autosizing
fixture still expects `UnsupportedSizing` before runtime.

CP252 adds no algorithm-level EnergyPlus source, Rust target/code/state, test,
object support, capability, output implementation, comparator, case,
manifest, numerical, performance, or conformance promotion. The parent
algorithm remains `scaffold` with claim level `none`. Inventory becomes 32
algorithms and 257 routines, split 58 `state_mapped` plus 199
`source_mapped`, with 134 required; heat-balance and HVAC project lists become
88 and 23. HVAC readiness remains `0/23`, the inventory is incomplete, and
all 23 required routines remain below `family_gated`.

## CP253 `updateZoneSizingEndZoneSizingCalc2` Supply-Delta Diagnostics and Peak Timestamps

CP253 adds canonical required
`routine.update_zone_sizing_end_zone_sizing_calc2` immediately after
`update_zone_sizing_end_zone_sizing_calc1` and before `sim_zone_equipment`.
The physical declaration sequence is CP252 at `ZoneEquipmentManager.hh` line
151, CP253 at line 153, and physical-next CP254 `writeZszSpsz` at lines
155-160. The complete CP253 leaf is `ZoneEquipmentManager.cc` lines
2280-2387:

```cpp
void updateZoneSizingEndZoneSizingCalc2(
    EnergyPlusData &state,
    DataSizing::ZoneSizingData &zsCalcSizing);
```

Its header-visible bundled formatting dependency is declared at header line 162 and
implemented at source lines 2389-2399:

```cpp
std::string sizingPeakTimeStamp(
    EnergyPlusData const &state,
    int timeStepIndex);
```

The child is part of the CP253 implementation boundary rather than a second
ledger row. The public leaf returns `void`, takes a mutable calculated-final
Zone-or-Space sizing record by reference, and has no entry guard, latent gate,
loop, status, `ErrorsFound`, catch, rollback, or cleanup.

### Production traversal and topology

The sole production selector is `SizingManager.cc` line 391. Its
`UpdateZoneSizing(EndZoneSizingCalc)` parent performs this source order:

1. run Zone-sizing EMS and independently apply each of six Zone volume, mass,
   and load overrides when EMS exists, that actuator flag is set, and the
   preoverride target is strictly positive;
2. only under `!isPulseZoneSizing`, optionally complete the entire CP252
   controlled-Zone noncoincident aggregation sweep;
3. visit every controlled Zone in ascending numeric order and call CP253 on
   `CalcFinalZoneSizing`;
4. immediately after each Zone, when Space sizing is enabled, visit every
   stored `Zone.spaceIndexes` occurrence and call CP253 on
   `CalcFinalSpaceSizing`;
5. only after all selected Zone and Space records complete, choose and write
   ZSZ/SPSZ;
6. then run Calc3 latent selection for latent-enabled controlled Zones and
   their stored Spaces;
7. outside the pulse block, run Calc4-7 copies and later final reports.

The two production call expressions are therefore
`ZoneEquipmentManager.cc` lines 3394 and 3397. Unlike CP252, this traversal
does not skip a Zone with exactly one Space. With:

```text
C = number of controlled Zone indexes visited by the parent
M = number of stored Space membership occurrences under those Zones
```

a normally completing call dispatches:

```text
H = C + (doSpaceHeatBalanceSizing ? M : 0)
```

CP253 leaves. The parent provides the control and Space-sizing gates, but the
leaf validates no role, controlled status, Zone/Space identity, topology,
membership parent, duplicate, cross-listing, index, allocation, or extent.
Stored order and multiplicity are authoritative. Repeated or cross-listed
Space indexes repeat diagnostics and overwrite the same record strings once
per occurrence.

Pulse EndZone processing skips CP252, the complete CP253 traversal, ZSZ/SPSZ,
and Calc3. Calc4-7 still run afterward, so pulse behavior is preservation plus
downstream copying rather than a CP253 reset. Direct leaf calls can bypass all
parent gates.

### Exact record boundary and source order

The leaf accesses 29 unique `ZoneSizingData` member names. Twenty-five are
read and only four timestamp strings are written:

| role | members |
|---|---|
| identity and loads | `ZoneName`, `DesCoolLoad`, `DesHeatLoad` |
| cooling supply inputs and diagnostics | `ZnCoolDgnSAMethod`, `CoolDesTemp`, `CoolDesTempDiff`, `ZoneTempAtCoolPeak`, `CoolTstatTemp`, `DesCoolVolFlow`, `DesCoolMassFlow` |
| heating supply inputs and diagnostics | `ZnHeatDgnSAMethod`, `HeatDesTemp`, `HeatDesTempDiff`, `ZoneTempAtHeatPeak`, `HeatTstatTemp`, `DesHeatVolFlow`, `DesHeatMassFlow` |
| date/index inputs | `cHeatDDDate`, `TimeStepNumAtHeatMax`, `cCoolDDDate`, `TimeStepNumAtCoolMax`, `cLatentHeatDDDate`, `TimeStepNumAtLatentHeatMax`, `cLatentCoolDDDate`, `TimeStepNumAtLatentCoolMax` |
| only writes | `HeatPeakDateHrMin`, `CoolPeakDateHrMin`, `LatHeatPeakDateHrMin`, `LatCoolPeakDateHrMin` |

The exact operation sequence is:

1. test cooling load for the zero-load warning;
2. independently test heating load for its zero-load warning;
3. resolve and diagnose cooling only when its absolute load exceeds the
   zero-load threshold;
4. resolve and diagnose heating only when its absolute load exceeds the
   threshold;
5. assign the four peak strings in Heat, Cool, LatHeat, LatCool order.

The `ShowContinueError` records emitted after a warning or severe are
diagnostic continuations, not C++ control-flow `continue` operations. No
diagnostic path returns; the timestamp tail remains unconditional.

### Load threshold and supply-temperature resolution

Cooling and heating use the same exact load partition:

| `L = abs(Des*Load)` | behavior |
|---|---|
| `L <= 1e-8` | emit one zero-load warning and one continuation; skip that mode's supply analysis |
| `L > 1e-8` | skip the zero warning and enter that mode's supply analysis |
| neither comparison, as for NaN | emit neither zero warning nor mode analysis |

This includes both signed zero values and exact `+/-1e-8` in the warning
branch. A negative load whose magnitude exceeds the threshold is analyzed just
like a positive load. Infinity enters analysis.

For either mode, exact integer equality with `SupplyAirTemperature` selects
the supplied design temperature. Every other method value, including the
intended temperature-difference method and an invalid enum integer, takes the
fallback path:

| mode | exact supplied-temperature method | every other method value |
|---|---|---|
| cooling | `SupplyTemp = CoolDesTemp`; `DeltaTemp = SupplyTemp - ZoneTempAtCoolPeak` | `DeltaTemp = -abs(CoolDesTempDiff)`; `SupplyTemp = ZoneTempAtCoolPeak + DeltaTemp` |
| heating | `SupplyTemp = HeatDesTemp`; `DeltaTemp = SupplyTemp - ZoneTempAtHeatPeak` | `DeltaTemp = HeatDesTempDiff`; `SupplyTemp = ZoneTempAtHeatPeak + DeltaTemp` |

Cooling therefore forces its fallback delta nonpositive, while heating
preserves the raw sign. There is no method validity, finite-value, unit, or
direction check before these calculations.

### Delta thresholds and diagnostic event shapes

Let `D = abs(DeltaTemp)` and
`HVAC::SmallTempDiff = 1e-5`. Cooling's wrong direction is
`SupplyTemp > ZoneTempAtCoolPeak`; heating's is
`SupplyTemp < ZoneTempAtHeatPeak`. The source partition is:

| exact condition | primary event | fixed continuations | optional continuation |
|---|---|---:|---|
| `1e-5 < D < 2` | severe near-delta | 9 | one wrong-direction note |
| `2 <= D < 5` | warning near-delta | 9 | one wrong-direction note |
| outside `D < 5 && D > 1e-5`, then `D > 1e-5` and wrong direction | severe direction error | 7 | none |
| all other cases | none | 0 | none |

The outer direction branch is an `else if`; for ordinary finite values it is
therefore reachable at `D >= 5`, not after a near-delta event. Exact
`D = 1e-5` is silent, exact `D = 2` is a warning, and exact `D = 5` reaches
the outer severe only when direction is wrong. A near-delta warning or severe
can still append its wrong-direction note.

Across both modes the source contains four `ShowWarningError`, four
`ShowSevereError`, and 36 `ShowContinueError` call sites. A receiver whose
cooling and heating modes both take the near-delta path with wrong direction
executes 22 `Show*` calls, the per-receiver maximum. Heat and cooling are
independent, so one mode's zero warning, severity, or malformed values do not
suppress the other mode.

A NaN load reaches neither load arm. A NaN delta reaches neither delta arm.
Infinite or otherwise extreme inputs follow raw IEEE comparisons and
formatting; no local diagnostic escalation, termination, or normalization is
added.

### Unconditional timestamp finalization

After all diagnostics, CP253 always assigns:

```text
HeatPeakDateHrMin =
    cHeatDDDate + " " + sizingPeakTimeStamp(TimeStepNumAtHeatMax)
CoolPeakDateHrMin =
    cCoolDDDate + " " + sizingPeakTimeStamp(TimeStepNumAtCoolMax)
LatHeatPeakDateHrMin =
    cLatentHeatDDDate + " " + sizingPeakTimeStamp(TimeStepNumAtLatentHeatMax)
LatCoolPeakDateHrMin =
    cLatentCoolDDDate + " " + sizingPeakTimeStamp(TimeStepNumAtLatentCoolMax)
```

The four calls occur even when latent sizing is false. No date validity or
emptiness check precedes concatenation. An empty date therefore yields a raw
leading space before the time.

`sizingPeakTimeStamp` initializes hour/minute/second outputs, then evaluates:

```text
timeInSeconds =
    timeStepIndex * MinutesInTimeStep * 60
```

The three operands are signed integers during multiplication; conversion to
`Real64` occurs only after the product. A sufficiently large product therefore
has C++ signed-integer overflow undefined behavior before `ParseTime`.
Otherwise the child calls `General::ParseTime`, ignores the returned seconds,
and formats only hour and minute through
`PeakHrMinFmt = "{:02}:{:02}:00"`.

The child does not inspect `TimeStepsInHour`, the number of timesteps in a day,
the source record's day, or the latent flag. It validates no zero, negative,
out-of-day, inconsistent-cadence, or finite range and does not clamp or wrap.
A normal last timestep can format as `24:00:00`; zero, negative, and
greater-than-day indexes are passed through mechanically. The literal seconds
field remains `00`.

### Downstream ownership and replay

`writeZszSpsz` follows only after every CP253 receiver completes, but does not
consume these four strings. Calc3 then conditionally selects latent sizing. It
can replace ordinary heating/cooling peak fields, including the sensible peak
strings, with latent values. CP253 diagnostics therefore always describe the
prelatent sensible fields, while a later predefined report can display a
latent-selected sensible string. Calc4-7 do not copy these strings.
`reportZoneSizing` reads the calculated-final sensible strings only when the
corresponding final volume flow is positive; otherwise it emits literal
`N/A`.

CP253 has observable output side effects before state finalization and no
transaction boundary:

- diagnostic output commits in source order before any of the four strings;
- each string assignment commits before the next child call or allocation;
- a failure preserves all earlier diagnostics and assignments;
- later Zone/Space receivers, sizing-file output, Calc3, copies, and reports
  are suppressed by propagation.

A failing invocation can therefore preserve no new string, Heat only,
Heat/Cool only, or Heat/Cool/LatHeat while retaining all earlier diagnostic
records. Stable direct replay overwrites the same four destinations and
reconstructs their strings, but repeats every diagnostic side effect.
Whole-parent replay is not necessarily observationally identical: a prior
successful Calc3 may already have replaced sensible fields that the next
CP253 diagnostic pass reads. Invalid record indices and helper integer
overflow have no defined local recovery.

### C++ evidence

No C++ test directly calls `updateZoneSizingEndZoneSizingCalc2` or
`sizingPeakTimeStamp`. The two direct `ZoneEquipmentManager` EndZone parent
tests set pulse sizing true immediately before their calls, so both dispatch
zero CP253 leaves.

A fresh completing production-style census finds 57 EndZone parent entries:

```text
direct ManageSizing:     19 = 17 normal + 2 pulse
full ManageSimulation:   38 = 34 normal + 4 pulse
combined:                57 = 51 normal + 6 pulse
```

The 51 normal parents dispatch 93 CP253 leaves:

```text
Zone receivers:   72
Space receivers:  21
total:            93
```

Each leaf unconditionally calls the child four times, for 372
`sizingPeakTimeStamp` and `General::ParseTime` calls. This is 186 ordinary
heat/cool formats plus 186 latent heat/cool formats. Only 13 receivers are
latent-enabled, split four Zone plus nine Space. Consequently, 26 latent
formats occur on latent-enabled receivers and another 160 occur on 80
latent-disabled receivers.

Eight `SizingManager` Zone-sizing simulations contain 58 predefined
peak-time table assertions. Eight heating `N/A` cells belong to
`reportZoneSizing`'s later nonpositive-flow branch, not CP253. The remaining
50 non-N/A values are composite CP253 timestamp descendants:

```text
Space descendants: 36
Zone descendants:  14
total:             50 of 372 formatted outputs
```

All 21 Space receivers are exactly the seven Space-sizing runs, so their 42
heat/cool report paths are represented: six become no-flow `N/A` and 36
retain CP253 descendants. The same suite covers only eight of 72 Zone
receivers, with two no-flow cells and 14 descendants.

The 50 expected strings are distributed as:

| count | expected report value |
|---:|---|
| 13 | `7/21 16:00:00` |
| 11 | `1/21 08:00:00` |
| 10 | `1/21 12:00:00` |
| 6 | `7/21 19:00:00` |
| 5 | `7/21 17:30:00` |
| 2 | `7/21 12:00:00` |
| 1 | `8/21 16:00:00` |
| 1 | `9/21 20:00:00` |
| 1 | `16:00:00` |

The time-only value descends from an empty cooling date and a raw leading
space; `RetrievePreDefTableEntry` trims it downstream. Of the 50 descendants,
49 are sensible strings. Exactly one `7/21 12:00:00` cooling assertion is a
latent-cooling string selected by Calc3 after
`TimeStepNumAtLatentCoolMax = 72`. No latent-heating string is asserted.

The two NonCoincident simulations execute eight zero-heating
warning/continuation pairs, proven by their asserted zero calculated heating
loads across Zone and Space records. Their common 12 C cooling and 50 C
heating supply inputs keep all positive-load deltas correctly directed and
outside the near-delta interval. A separate
`BaseSizer_SupplyAirTempLessThanZoneTStatTest` full call proves a positive
heating load with a 12 C supply, 17.08 C peak, and therefore at least one
wrong-direction heating severe event. No test asserts diagnostic text,
severity count, continuation count, or output ordering.

`General_ParseTime` unit coverage exercises 16 second inputs and 48 parsed
components. A separate `SizingManager` `TimeIndexToHrMinString` test asserts
14 strings for 15- and 3-minute timesteps. Neither directly exercises the
CP253 wrapper, its integer multiplication, its fixed seconds, or date
concatenation. The 50 downstream descendants cover only six times of day at a
10-minute cadence.

Unisolated branches include direct member writes, raw leading-space state,
latent heating, latent-disabled unconditional latent writes, exact load and
delta boundaries, near-delta warning/severe and optional notes, cooling wrong
direction, fallback and invalid method integers, combined heat/cool severity,
Space records labeled as `zone`, negative/NaN/infinite values, zero/24-hour/
out-of-day/negative/overflow time inputs, nonstandard minutes, duplicate and
cross-listed topology, malformed indexes, pulse stale-string preservation,
diagnostic or assignment failure prefixes, direct replay, and whole-parent
retry. No exact EIO diagnostic, ZSZ row, or SPSZ row assertion isolates CP253.

### Rust boundary and governance

A fresh exact search across 721 `crates` and `data` files finds zero main or
child helper names and zero canonical-key occurrences. Rust has no
`ZoneSizingData`, calculated-final Zone/Space sizing arena,
`SupplyAirTemperature` sizing method, `PeakHrMinFmt`,
`MinutesInTimeStep`, CP253 diagnostic literal, or ZSZ/SPSZ/ERR artifact
counterpart. None of the 29 exact member tokens appears. Twenty-eight
snake-case counterparts are also absent; the generic `zone_name` appears only
in unrelated structures and does not supply this boundary.

Thermostat and IdealLoads operational supply constraints, the separate
`0.001` mass-flow delta guard, schedule `minutes_per_timestep`, and normalized
ESO date/time labels are adjacent only. They provide no Zone-sizing
diagnostic traversal, calculated-final peak state, source severity partition,
or four-string finalizer.

All 61 active-data `SimulationControl` objects set Zone sizing to `No`.
Active data contain no `Sizing:Zone`, `Sizing:Parameters`,
`NonCoincident`, authored `Space`, or `SpaceList`. Five design-day objects
exist only in inactive/raw fixtures. The sole autosizing fixture expects
`UnsupportedSizing` before runtime, while Space fixtures expect
`UnsupportedSpacePartitioning`.

CP253 therefore adds no algorithm-level EnergyPlus source, Rust target/code/
state, test, object support, capability, output implementation, comparator,
case, manifest, numerical, performance, or conformance promotion. The parent
algorithm remains `scaffold` with claim level `none`. Inventory becomes 32
algorithms and 258 routines, split 58 `state_mapped` plus 200
`source_mapped`, with 135 required; heat-balance and HVAC project lists become
88 and 24. HVAC readiness remains `0/24`, the inventory is incomplete, and
all 24 required routines remain below `family_gated`.

## CP254 `writeZszSpsz` Zone/Space Sizing-Series File Writer

CP254 adds canonical required `routine.write_zsz_spsz` immediately after
`update_zone_sizing_end_zone_sizing_calc2` and before `sim_zone_equipment`.
The physical declaration is `ZoneEquipmentManager.hh` lines 155-160, and the
complete body is `ZoneEquipmentManager.cc` lines 2401-2644:

```cpp
void writeZszSpsz(
    EnergyPlusData &state,
    EnergyPlus::InputOutputFile &outputFile,
    int const numSpacesOrZones,
    EPVector<DataSizing::ZoneSizingData> const &zsCalcFinalSizing,
    Array2D<DataSizing::ZoneSizingData> const &zsCalcSizing,
    bool const forSpaces);
```

The raw role count, calculated-final vector, design-day matrix, output handle,
and `forSpaces` flag are selected by the parent. The leaf returns `void` and
does not modify a sizing record, but it incrementally mutates the output
stream and can invoke stateful psychrometric cache and diagnostic machinery.
It owns no result/status object, cleanup guard, transaction, or rollback.

### Parent order and file routing

The sole production selector is `SizingManager.cc` lines 390-393. After
`NumSizingPeriodsPerformed > 0`, it calls EndZone sizing, then facility
sizing, and only afterward marks `ZoneSizingRunDone = true`. The relevant
EndZone order in `ZoneEquipmentManager.cc` is:

1. Unconditionally call `ManageEMS` at the Zone-sizing calling point, retain
   but do not inspect `anyEMSRan`, then, only when
   `AnyEnergyManagementSystemInModel` is true, traverse every Zone and apply
   each of six overrides independently when its flag is on and its current
   load, mass flow, or volume flow is strictly positive.
2. Under `!isPulseZoneSizing && doSpaceHeatBalanceSizing`, traverse controlled
   `ZoneEquipConfig` receivers, skip only `Zone.numSpaces == 1`, and dispatch
   CP252. Malformed zero or negative counts therefore still dispatch.
3. Traverse controlled `ZoneEquipConfig` receivers and run CP253 for each
   Zone, then each stored `Zone.spaceIndexes` member when Space sizing is on.
4. Select and open the ZSZ path, then call CP254 with `NumOfZones`,
   `CalcFinalZoneSizing`, `CalcZoneSizing`, and `forSpaces = false`.
5. When Space sizing is on, select and open the SPSZ path, then call CP254
   with global `numSpaces`, `CalcFinalSpaceSizing`, `CalcSpaceSizing`, and
   `forSpaces = true`.
6. Only after both writers return, traverse controlled equipment Zones and
   skip a whole Zone unless its calculated-final `zoneLatentSizing` is true.
   For a passing Zone, run CP255 for that Zone and, when Space sizing is on,
   every stored Space member without a per-Space latent-enable check.
7. Run Calc4-7 later, outside the pulse guard.

Pulse sizing therefore skips CP252, CP253, both CP254 files, and CP255, while
still reaching Calc4-7. A normal CP254 file captures calculated-final state
after EMS/noncoincident/CP253 processing but before CP255 can replace sensible
peaks with latent-selected values. CP254 does not consume CP253's four peak
timestamp strings.

`SizingFileColSep` selects comma output paths ending in `.csv`, tab paths
ending in `.tab`, and every other separator's paths ending in `.txt`.
`ensure_open` is a parent operation. It reuses an already-good stream even if
the path was just reassigned and regardless of the current output-control
flag. Only a not-good handle is replaced: it opens a truncating real stream
when the flag is enabled or a null stream when disabled. Failure to open a
requested real stream is fatal before CP254 begins. False output control can
therefore retain an existing sink, including a preopened test stringstream,
but it never skips the writer's loops, indexing, or psychrometric calls.

At entry CP254 snapshots the raw `SizingFileColSep` character. It does not
normalize it to the extension branch and does not quote or escape any field.
On the normal tail it calls `outputFile.close()`, including for a stream the
caller opened or injected. There is no explicit flush or post-close status
check.

### Receiver identity and eligibility

The header, every time row, `Peak`, and `Peak Vol Flow (m3/s)` each repeat the
same independent traversal:

```text
for i = 1..numSpacesOrZones:
    owner = forSpaces ? space(i).zoneNum : i
    skip unless Zone(owner).IsControlled
    record = zsCalcFinalSizing(i)
```

Eligibility is the HeatBalance `Zone(owner).IsControlled` flag, not the
parent's earlier `ZoneEquipConfig(owner).IsControlled` flag. A disagreement
between those flags can therefore make CP253 and CP254 select different
receivers. Zone mode assumes candidate `i` is both the owner and the sizing
record index. Space mode first dereferences global `space(i).zoneNum`, tests
that owner, and then uses global Space index `i` for both calculated-final and
daily arrays.

SPSZ walks every global Space exactly once in global index order. It does not
walk stored `Zone.spaceIndexes`. Duplicate or cross-listed membership entries
that repeated CP253 do not repeat SPSZ, while an orphan or mismatched global
Space is still governed only by its stored `zoneNum`. The routine validates
none of the following:

- whether `forSpaces` matches the supplied arrays;
- whether candidate, owner, Zone, Space, daily-day, or sequence indices exist;
- whether HeatBalance and equipment-control flags agree;
- whether Space ownership agrees with Zone membership lists;
- whether the two sizing containers have compatible extents;
- whether names, design-day labels, separator characters, or numeric values
  are safe for the chosen text format.

For every eligible receiver, the header appends 16 fields to the leading
literal `Time`. A field is raw concatenation of
`ZoneName + ":" + design-day-string + ":" + suffix`. An empty design-day
string consequently produces `ZoneName::suffix`, and embedded separators,
colons, carriage returns, or newlines remain unescaped. Space output retains
the sizing record's `ZoneName` field and labels the last four columns as Zone
temperature and relative humidity.

### Exact 16-column schema

All 16 receiver columns preserve their position across the header, time rows,
and both summary rows. The exact projection is:

| col. | header design-day member and literal suffix | time-row source | `Peak` source |
|---:|---|---|---|
| 1 | `HeatDesDay`, `Des Heat Load [W]` | `HeatLoadSeq(TimeStepIndex)` | `DesHeatLoad` |
| 2 | `CoolDesDay`, `Des Sens Cool Load [W]` | `CoolLoadSeq(TimeStepIndex)` | `DesCoolLoad` |
| 3 | `HeatDesDay`, `Des Heat Mass Flow [kg/s]` | `HeatFlowSeq(TimeStepIndex)` | `DesHeatMassFlow` |
| 4 | `CoolDesDay`, `Des Cool Mass Flow [kg/s]` | `CoolFlowSeq(TimeStepIndex)` | `DesCoolMassFlow` |
| 5 | `LatHeatDesDay`, `Des Latent Heat Load [W]` | `LatentHeatLoadSeq(TimeStepIndex)` | `DesLatentHeatLoad` |
| 6 | `LatCoolDesDay`, `Des Latent Cool Load [W]` | `LatentCoolLoadSeq(TimeStepIndex)` | `DesLatentCoolLoad` |
| 7 | `LatHeatDesDay`, `Des Latent Heat Mass Flow [kg/s]` | `LatentHeatFlowSeq(TimeStepIndex)` | `DesLatentHeatMassFlow` |
| 8 | `LatCoolDesDay`, `Des Latent Cool Mass Flow [kg/s]` | `LatentCoolFlowSeq(TimeStepIndex)` | `DesLatentCoolMassFlow` |
| 9 | `HeatNoDOASDesDay`, `Des Heat Load No DOAS [W]` | `HeatLoadNoDOASSeq(TimeStepIndex)` | `DesHeatLoadNoDOAS` |
| 10 | `CoolNoDOASDesDay`, `Des Sens Cool Load No DOAS [W]` | `CoolLoadNoDOASSeq(TimeStepIndex)` | `DesCoolLoadNoDOAS` |
| 11 | `LatHeatNoDOASDesDay`, `Des Latent Heat Load No DOAS [W]` | `HeatLatentLoadNoDOASSeq(TimeStepIndex)` | `DesLatentHeatLoadNoDOAS` |
| 12 | `LatCoolNoDOASDesDay`, `Des Latent Cool Load No DOAS [W]` | `CoolLatentLoadNoDOASSeq(TimeStepIndex)` | `DesLatentCoolLoadNoDOAS` |
| 13 | `HeatDesDay`, `Heating Zone Temperature [C]` | derived `ZoneTHeat` | blank |
| 14 | `HeatDesDay`, `Heating Zone Relative Humidity [%]` | derived `ZoneRHHeat` | blank |
| 15 | `CoolDesDay`, `Cooling Zone Temperature [C]` | derived `ZoneTCool` | blank |
| 16 | `CoolDesDay`, `Cooling Zone Relative Humidity [%]` | derived `ZoneRHCool` | blank |

Every time-row and nonblank summary number uses `{:12.6E}`. The format width is
a minimum rather than a truncation rule; large magnitudes and nonfinite text
can exceed it. The writer emits all twelve sizing-series columns regardless
of `zoneLatentSizing`, `AccountForDOAS`, sensible/latent sizing method, supply
air method, or whether the corresponding header day string is empty. It uses
the four no-DOAS design-day *strings* for headers but does not consult separate
latent/no-DOAS day-number selectors for the sequence data.

Across header, time, and summary work, the calculated-final record contributes
39 unique members:

- `ZoneName` and eight design-day strings:
  `HeatDesDay`, `CoolDesDay`, `LatHeatDesDay`, `LatCoolDesDay`,
  `HeatNoDOASDesDay`, `CoolNoDOASDesDay`, `LatHeatNoDOASDesDay`, and
  `LatCoolNoDOASDesDay`;
- two sensible day numbers, `HeatDDNum` and `CoolDDNum`;
- twelve time sequences from columns 1-12;
- twelve design scalars from columns 1-12 of `Peak`;
- four volume-flow scalars:
  `DesHeatVolFlow`, `DesCoolVolFlow`, `DesLatentHeatVolFlow`, and
  `DesLatentCoolVolFlow`.

The daily matrix adds four distinct members:
`HeatZoneTempSeq`, `HeatZoneHumRatSeq`, `CoolZoneTempSeq`, and
`CoolZoneHumRatSeq`. CP254 therefore depends on 43 unique sizing-record member
names. It does not read the CP253 timestamp strings or indices, the latent
enable flag, `AccountForDOAS`, a sizing-method enum, a supply method, or
latent/no-DOAS design-day numbers.

### Time loop and derived sensible conditions

After the header newline, CP254 initializes `Minutes = 0` and
`TimeStepIndex = 0`, then executes exactly this loop shape:

```text
for HourCounter = 1..24:
    for TimeStepCounter = 1..TimeStepsInHour:
        ++TimeStepIndex
        Minutes += MinutesInTimeStep
        HourPrint = HourCounter - 1
        if Minutes == 60:
            Minutes = 0
            HourPrint = HourCounter
        print "{:02}:{:02}:00"
        project every eligible receiver
        print newline
```

The inner counter's value is otherwise unused. A positive
`TimeStepsInHour = H` produces `24H` time rows and sequence indices
`1..24H`; zero or negative `H` produces none. Normal compatible cadence ends
at `24:00:00`. Because reset occurs only at exact equality, inconsistent,
zero, negative, or overflowing minute state can repeat labels, move backward,
or print values above 60. The routine does not use
`MinutesPerTimeStep`, `NumOfTimeStepInDay`, CP253's
`sizingPeakTimeStamp`, or any parser/clamp/wrap operation, and it does not
check signed integer overflow.

For each eligible receiver and time index, all twelve calculated-final
sequences are accessed unconditionally before formatting. The last four
columns start from local zeros. A strictly positive `HeatDDNum` replaces
`ZoneTHeat` with
`zsCalcSizing(HeatDDNum, i).HeatZoneTempSeq(TimeStepIndex)` and computes
`ZoneRHHeat` from a second read of that temperature, one read of
`HeatZoneHumRatSeq`, and the current `OutBaroPress`. A strictly positive
`CoolDDNum` does the analogous cooling reads. Nonpositive day numbers leave
that mode's temperature and RH at formatted zero; positive out-of-range day
numbers are not checked.

Relative humidity is
`100 * PsyRhFnTdbWPb(state, Tdb, W, OutBaroPress)`. The call omits the optional
`CalledFrom` label. Its already-mapped psychrometric child owns the humidity
ratio floor at `1e-5`, saturation-pressure cache and diagnostics, ordinary
out-of-range RH clamp to `[0.01, 1]`, and the behavior for nonfinite input;
CP254 adds no finite/range validation. Thus a false output-control flag still executes the same daily reads and
psychrometric work. When a not-good handle is replaced by a null sink, those
side effects occur without persistent ZSZ/SPSZ bytes. An already-good
physical or string sink continues receiving bytes despite the false flag,
while a reused dev-null sink continues discarding them.

### Peak rows and physical layout

After all time rows, CP254 writes literal `Peak` and repeats the receiver
filter. Columns 1-12 are the corresponding design load or mass-flow scalars;
columns 13-16 are four empty fields. It terminates that row, then starts the
next print with literal `"\nPeak Vol Flow (m3/s)"`. That leading newline
creates a completely blank physical line.

The volume row again preserves all 16 receiver positions:

| columns | volume-row content |
|---|---|
| 1-2 | blank |
| 3-4 | `DesHeatVolFlow`, `DesCoolVolFlow` |
| 5-6 | blank |
| 7-8 | `DesLatentHeatVolFlow`, `DesLatentCoolVolFlow` |
| 9-16 | blank |

The four values deliberately sit under the sensible and latent *mass-flow*
headers; the file does not create separate volume-flow headers. Blank fields
are emitted as adjacent raw separators. The final newline is followed by
`outputFile.close()`.

For `K` eligible receivers, every nonblank logical row has one leading label
and `16K` receiver fields. When the raw separator, name, and design-day text
contain no line breaks, the physical file contains one header, all time rows,
one Peak row, one blank line, and one volume row. Raw line breaks can add
physical lines without changing the writer-issued structural boundaries. The
file has no units row, metadata preamble, quoting declaration, checksum,
footer, or success marker.

### Operation and output counts

For stable state, define:

- `N = max(numSpacesOrZones, 0)`;
- `K` as candidates among those `N` whose derived owner has
  `Zone(owner).IsControlled`;
- `R = 24 * max(TimeStepsInHour, 0)`;
- `P_H` and `P_C` as eligible receivers whose `HeatDDNum` and `CoolDDNum`
  are respectively positive.

Ignoring signed overflow and assuming all referenced storage exists, one leaf
call performs:

| operation/output | exact count |
|---|---:|
| candidate owner/filter visits | `N(R + 3)` |
| eligible receiver projections/formats | `K(R + 3)` |
| dynamic `print` calls | `(R + 3)(K + 2)` |
| calculated-final sequence reads | `12RK` |
| daily-matrix sequence reads | `3R(P_H + P_C)` |
| `PsyRhFnTdbWPb` calls | `R(P_H + P_C)` |
| numeric fields | `16K(R + 1)` |
| columns in each nonblank logical row | `1 + 16K` |
| writer-issued structural LF literals | `R + 4` |

The three receiver-summary traversals in `R + 3` are header, Peak, and volume.
The daily count is three reads per positive mode because the selected
temperature is read once for output and again as the psychrometric argument,
then its humidity ratio is read once. At most both sensible modes are
positive, giving a psychrometric upper bound of `2RK`.

The writer allocates/formats incrementally and has no receiver, timestep,
field-width, or total-byte size guard. Its stable runtime is
`Theta(N(R+3) + formatted byte length)`, and output volume scales with both
timestep cadence and every eligible Zone/Space record. Negative signed counts
produce empty loops, but large or inconsistent signed counts, timestep
indices, minute accumulation, and multiplication used to reason about totals
are not protected against overflow.

### Invalid state, partial output, and retry

CP254 performs no preflight across all receivers. Header output begins before
the first topology and sizing-record checks have completed, and each time
label is printed before that row's receiver reads and psychrometric calls.
Consequently an invalid owner, missing array entry, bad positive design-day
index, short sequence, allocation failure, formatting failure, or diagnostic
exception can leave a prefix containing any combination of header fields,
complete prior rows, and a bare current time label. Psychrometric side effects
for a receiver occur after that label but before the receiver's 16-field
formatted buffer is written.

Each `print` constructs its formatted buffer before its stream write, so a
failure in one format does not roll back earlier print calls. C++ exceptions
other than the project's formatting-error path propagate; source-side fatal
handling does not establish a return status. A non-return before line 2643
also bypasses the explicit close and leaves the handle in whatever open/good
state the failure produced.

Ordinary iostream bad/fail state is normally nonthrowing. The leaf never tests
the stream after a write, flush, or close, so a disk/device failure can instead
silently truncate the artifact, reach `close()`, and permit all downstream
sizing work and `ZoneSizingRunDone` to proceed. The parent has no byte count,
schema validation, reopen/readback, or artifact-level golden comparison.

Failure propagation is source-ordered:

- a ZSZ non-return prevents SPSZ selection/open/write, CP255, the remaining
  EndZone copies, facility sizing, and the final run-done latch;
- an SPSZ non-return preserves the already-closed ZSZ but prevents CP255 and
  the same downstream work;
- a silent badbit suppresses none of that downstream work.

After an ordinary successful close, replay through the production parent sees
a non-good closed handle. With the corresponding output-control flag enabled,
it reopens the selected path with truncation and rebuilds stable bytes; with
the flag disabled, it installs a null sink and leaves any prior physical file
untouched. Both paths repeat psychrometric cache/diagnostic effects. If an
exception left a still-good stream open, `ensure_open` reuses it and a
retry appends a second header or prefix. A not-good ordinary sink is replaced
according to the current output-control flag: a selected real file is
truncated when enabled, while a null sink is installed when disabled.
`InputOutputFile::good()` deliberately treats a bad dev-null sink as good, so
that special handle is reused instead. A whole-parent retry after CP255
previously completed can
produce different bytes because CP255 may already have replaced
calculated-final sensible load/flow fields with latent-selected values.
There is no idempotence contract spanning bytes and psychrometric side
effects.

### C++ evidence

No C++ test calls `writeZszSpsz` directly. Two unit contexts call the EndZone
parent directly, at the call sites near lines 4576 and 4877 of
`ZoneEquipmentManager.unit.cc`, but both first set `isPulseZoneSizing = true`
and therefore exercise no CP254 branch.

A fresh completing production-style census finds 51 nonpulse parent entries:
17 through direct `ManageSizing` contexts and 34 through `ManageSimulation`
contexts. Six of those contexts also execute a preceding pulse parent entry,
for 57 total production parent entries: 51 normal and six pulse. The 51
normal entries make 51 ZSZ calls; seven enable Space sizing and add SPSZ, for
58 leaf calls overall.

Those calls have this aggregate receiver/cadence shape:

| evidence dimension | aggregate |
|---|---:|
| Zone candidates supplied to 51 ZSZ calls | 84 |
| controlled Zone selections | 72 |
| filtered Zone candidates | 12 |
| global Space candidates across seven SPSZ calls | 21 |
| controlled Space selections | 21 |
| total eligible record projections | 93 |
| calls at one timestep/hour; eligible records | 1; 1 |
| calls at four timesteps/hour; eligible records | 21; 37 |
| calls at six timesteps/hour; eligible records | 36; 55 |

Applying the source formulas gives 7,224 time rows, 11,496 eligible time-record
blocks, 11,775 eligible receiver formats including the three summary
traversals, 13,251 candidate/filter visits, 26,571 dynamic print calls, 7,456
physical lines, and 185,424 numeric fields. The tests do not assert exact
positive heating/cooling day counts, so daily reads and psychrometric calls
remain bounded and expressed by `P_H` and `P_C`, with zero through 11,496
psychrometric calls possible per mode in the aggregate census.

Thirteen of the 93 selected records are latent-enabled—four Zone projections
and nine Space projections—and 80 are not, but every one formats latent
columns. Six selected Zone records are in known DOAS-enabled writer contexts,
but the source has no DOAS gate on either ordinary or no-DOAS columns.

Seven known Space-writer contexts enter through `SizingManager` call sites
near lines 982, 1453, 1897, 2413, 2874, 3338, and 4246. The `NoSpaceHB`
context near line 3802 writes Zone output only. None of these tests specifies
`OutputControl:Sizing:Style`; the default comma route covers all 58 calls,
while tab and other/text path branches receive zero completing writer calls.

The fixture preopens ZSZ and SPSZ stringstreams near
`EnergyPlusFixture.cc` lines 92-93. CP254 resets and destroys those stream
objects. Teardown later invokes `zsz.del()` and `spsz.del()` near lines
133-134, but `InputOutputFile::del()` does nothing because `os` is already
null. The separate
output-control test supplies booleans and asserts those booleans only. No
test asserts a CP254 header, separator, path, extension, field order, format
precision, timestamp, blank field, row count, byte sequence, close state,
output failure, or tracked golden artifact.

Upstream tests do vary some sizing records, latent settings, DOAS settings,
Space topology, cadence, and psychrometric inputs. They do not isolate those
variations at the writer boundary or prove resulting bytes. Remaining direct
gaps include both role branches in one leaf oracle, filtered owners, malformed
owner/membership/extent state, zero/negative/large cadence, invalid positive
day numbers, nonfinite values, embedded delimiter/newline text, output
disabled with psychrometric side effects, open/write/close failure, silent
badbit, partial prefixes, successful replay, exceptional retry, and
post-CP255 whole-parent replay.

### Rust boundary and governance

An exact audit across 721 Rust-crate and active-data files finds no CP254
helper or canonical key; no ZSZ/SPSZ artifact, path, schema, header, Peak-row,
or formatting literal; no calculated sizing arena; and no integration for
`SizingFileColSep`, `TimeStepsInHour`, `MinutesInTimeStep`, or
`OutBaroPress`. None of the 43 C++ member spellings appears. Forty-two
snake-case projections are absent as well; the only lexical resemblance is
an unrelated generic `zone_name`.

Rust has a nearby ordinary-finite psychrometric projection, but it is not
connected to a sizing-series writer and does not establish CP254 topology,
sequencing, diagnostics, stream lifecycle, or artifact bytes. The existing
psychrometric `PsyRhFnTdbWPb` row remains its own non-required
`state_mapped` child; CP254 does not duplicate or promote it.

All 61 active-data `SimulationControl` objects disable Zone sizing. Five raw
`SizingPeriod:DesignDay` objects exist, but active data contain no
`Sizing:Zone`, `Sizing:Parameters`, authored `Space`, or `SpaceList`.
Existing autosizing and Space-partition fixtures remain blocked by
`UnsupportedSizing` and `UnsupportedSpacePartitioning`. They cannot serve as
a Rust-side CP254 artifact oracle.

This checkpoint therefore adds only canonical
`routine.write_zsz_spsz` as required `source_mapped`. It adds no Rust target
or state, support declaration, unit/integration test, capability row, output
implementation, comparator, case, manifest evidence, numerical/performance
claim, or conformance promotion. The parent
`algorithm.ideal_loads_zone_equipment_purchased_air_source_order` remains
`scaffold` with claim level `none`.

The generated inventory becomes 32 algorithms and 259 routines: 58
`state_mapped` plus 201 `source_mapped`, with 136 required. Project-contract
required domains become heat balance 88, HVAC 25, plant 1, and time 22. HVAC
readiness remains `0/25`; the inventory is incomplete and all 25 required
HVAC routines remain below `family_gated`.

## CP255 `updateZoneSizingEndZoneSizingCalc3` Latent Peak Selection

CP255 adds canonical required
`routine.update_zone_sizing_end_zone_sizing_calc3` immediately after
`write_zsz_spsz` and before `sim_zone_equipment`. The physical declaration is
`ZoneEquipmentManager.hh` lines 164-167, and the complete body is
`ZoneEquipmentManager.cc` lines 2646-2764:

```cpp
void updateZoneSizingEndZoneSizingCalc3(
    DataSizing::ZoneSizingData &zsCalcFinalSizing,
    Array2D<DataSizing::ZoneSizingData> &zsCalcSizing,
    bool &anyLatentLoad,
    int const zoneOrSpaceNum);
```

The leaf receives one mutable calculated-final record, a mutable daily sizing
matrix for the same role family, a shared mutable boolean, and an unchecked
Zone-or-Space index. It returns `void`, emits no direct output or diagnostic,
and has no EnergyPlus state argument. Its only work is conditional in-place
selection of latent peaks into ordinary sensible-named fields.

### Parent gate, roles, and downstream order

The sole production calls are the two sites at
`ZoneEquipmentManager.cc` lines 3444-3451. They remain inside the nonpulse
EndZone block and occur after every CP253 receiver and after CP254 has selected,
opened, written, and closed ZSZ plus optional SPSZ. CP254 therefore serializes
pre-CP255 state. The parent then executes:

```text
for zoneNum = 1..NumOfZones:
    skip unless ZoneEquipConfig(zoneNum).IsControlled
    skip unless CalcFinalZoneSizing(zoneNum).zoneLatentSizing
    Calc3(CalcFinalZoneSizing(zoneNum), CalcZoneSizing,
          shared isAnyLatentLoad, zoneNum)
    if doSpaceHeatBalanceSizing:
        for spaceNum in Zone(zoneNum).spaceIndexes:
            Calc3(CalcFinalSpaceSizing(spaceNum), CalcSpaceSizing,
                  shared isAnyLatentLoad, spaceNum)
```

The Zone calculated-final `zoneLatentSizing` flag gates both that Zone and
all of its stored Spaces. There is no Space-local latent flag check, Space
control check, owner lookup, deduplication, membership validation, or
comparison between Zone and Space methods. A stored duplicate or cross-list
entry repeats the same Space record in source container order. A direct leaf
caller bypasses every parent gate and can pass either role or arbitrary
storage.

Pulse sizing skips CP255 entirely. After every selected Zone and stored Space
returns, the parent exits the pulse guard and starts CP256 Calc4 at lines
3459-3464, then Calc5-7. Those routines copy or consume the ordinary fields
that CP255 may have replaced. A CP255 non-return preserves the already-closed
preselection ZSZ/SPSZ artifacts but suppresses the remaining Calc3 roles,
Calc4-7, facility sizing, and the final `ZoneSizingRunDone` latch.

Let `Q` be controlled Zones whose calculated-final Zone latent flag is true
and `M` be stored Space-membership occurrences beneath those Zones when Space
sizing is enabled. A normal parent entry dispatches `Q + M` leaves. A
false Zone flag can suppress a Space whose own record requests latent sizing;
a true Zone flag can dispatch a `Sensible`, `SensibleOnly`, `Invalid`, or
otherwise inconsistent Space record that then performs no selection.

### Independent cooling and heating predicates

Cooling is evaluated first, and heating is evaluated afterward even when
cooling completed. Their exact predicates are:

| mode | `ZoneSizing::Latent` | `ZoneSizing::SensibleAndLatent` | other methods |
|---|---|---|---|
| cooling | `DesLatentCoolVolFlow > 0.0` | `DesLatentCoolLoad > DesCoolLoad` | false |
| heating | `DesLatentHeatVolFlow > 0.0` | `DesLatentHeatLoad > DesHeatLoad` | false |

The source comment says combined sensible-and-latent selection uses the larger
volume flow, but the executable code compares loads. Exact `Latent` ignores
the load comparison and can select a smaller or negative latent load when its
volume flow is positive. Exact `SensibleAndLatent` ignores latent volume and
can select zero or negative volume when its latent load wins. All comparisons
are strict and raw: ties and NaN are false, infinities follow ordinary IEEE
comparison, and companion NaN/Inf/negative fields propagate after a branch is
selected.

`Sensible`, `SensibleOnly`, `Invalid`, and out-of-range enum values satisfy
neither explicit equality. The method can be read once or twice in each ordered
condition: a successful `Latent` conjunct short-circuits the second access;
otherwise the `SensibleAndLatent` conjunct reads it again. There is no
consistency check against the parent `zoneLatentSizing` gate.

Cooling's first statement is `anyLatentLoad = true`. No statement resets that
reference, and the heating branch never writes it. Across Zone and Space
calls it is therefore a monotonic, cooling-only latch: a selected heating-only
case can leave it false, while a prior selected cooling case leaves it true
for every later role and replay.

### Calculated-final projection

A selected mode writes exactly 16 calculated-final destinations in statement
order. Cooling and heating use the following symmetric projection:

| destination concept | cooling source | heating source |
|---|---|---|
| sizing type | literal `Latent Cooling` | literal `Latent Heating` |
| design volume flow | `DesLatentCoolVolFlow` | `DesLatentHeatVolFlow` |
| design mass flow | `DesLatentCoolMassFlow` | `DesLatentHeatMassFlow` |
| design load | `DesLatentCoolLoad` | `DesLatentHeatLoad` |
| design-day name | `LatCoolDesDay` | `LatHeatDesDay` |
| design-day date string | `cLatentCoolDDDate` | `cLatentHeatDDDate` |
| design-day number | `LatentCoolDDNum` | `LatentHeatDDNum` |
| peak timestep | `TimeStepNumAtLatentCoolMax` | `TimeStepNumAtLatentHeatMax` |
| ordinary flow sequence | `LatentCoolFlowSeq` | `LatentHeatFlowSeq` |
| coil-in temperature | `DesLatentCoolCoilInTemp` | `DesLatentHeatCoilInTemp` |
| coil-in humidity ratio | `DesLatentCoolCoilInHumRat` | `DesLatentHeatCoilInHumRat` |
| return temperature at peak | `ZoneRetTempAtLatentCoolPeak` | `ZoneRetTempAtLatentHeatPeak` |
| Zone temperature at peak | `ZoneTempAtLatentCoolPeak` | `ZoneTempAtLatentHeatPeak` |
| Zone humidity ratio at peak | `ZoneHumRatAtLatentCoolPeak` | `ZoneHumRatAtLatentHeatPeak` |
| peak date/time string | `LatCoolPeakDateHrMin` | `LatHeatPeakDateHrMin` |
| design supply humidity ratio | mode-specific formula | mode-specific formula |

The ordinary destinations are respectively `CoolSizingType` or
`HeatSizingType`, `DesCool*` or `DesHeat*`, `CoolDesDay` or `HeatDesDay`,
`cCoolDDDate` or `cHeatDDDate`, `CoolDDNum` or `HeatDDNum`,
`TimeStepNumAtCoolMax` or `TimeStepNumAtHeatMax`, `CoolFlowSeq` or
`HeatFlowSeq`, the corresponding ordinary coil/Zone/return fields,
`CoolPeakDateHrMin` or `HeatPeakDateHrMin`, and `CoolDesHumRat` or
`HeatDesHumRat`.

The sequence assignment copies the entire current latent flow sequence into
the ordinary flow sequence. CP255 does not copy `LatentCoolLoadSeq` or
`LatentHeatLoadSeq` into the ordinary load sequences. It also leaves outdoor
peak state, densities, thermostat values, no-DOAS state, all latent source
fields, and every other ordinary field outside this 16-member destination set
untouched.

The calculated-final humidity assignment is:

```text
if ZnLatCoolDgnSAMethod == SupplyAirHumidityRatio:
    CoolDesHumRat = LatentCoolDesHumRat
else:
    CoolDesHumRat =
        ZoneHumRatAtLatentCoolPeak - CoolDesHumRatDiff

if ZnLatHeatDgnSAMethod == SupplyAirHumidityRatio:
    HeatDesHumRat = LatentHeatDesHumRat
else:
    HeatDesHumRat =
        ZoneHumRatAtLatentHeatPeak + HeatDesHumRatDiff
```

`SupplyAirHumidityRatio` is exact raw integer value 3. Every other integer,
including the expected difference method, zero, negative, or invalid values,
takes the `else` formula. Neither formula calls psychrometrics or applies
clamp, saturation, nonnegativity, finite, or physical-range validation.

### Selected daily projection

Only after all 16 final assignments for a mode, CP255 tests the final record's
unchanged latent day source:

```text
if final.LatentCoolDDNum > 0:
    daily = zsCalcSizing(final.LatentCoolDDNum, zoneOrSpaceNum)
if final.LatentHeatDDNum > 0:
    daily = zsCalcSizing(final.LatentHeatDDNum, zoneOrSpaceNum)
```

Zero or negative latent day numbers skip daily mutation even though the final
ordinary fields, including ordinary day number, were already replaced.
Positive values receive no upper-bound check. `zoneOrSpaceNum` is not checked,
and the matrix role, allocation, dimensions, and selected record identity are
not validated. Invalid positive state can therefore fail only after the final
record has a complete selected-mode prefix.

The selected daily record receives 15 assignments: the same projection as
the final record except that no sizing-type field is written. Most right-hand
sides come from that daily record's own latent fields. Three identity details
are deliberately asymmetric:

- lookup uses the *final* record's positive latent day number;
- ordinary daily `CoolDDNum`/`HeatDDNum` receives the selected daily record's
  own `LatentCoolDDNum`/`LatentHeatDDNum`, which may differ from the lookup
  day;
- ordinary daily `cCoolDDDate`/`cHeatDDDate` and
  `CoolPeakDateHrMin`/`HeatPeakDateHrMin` receive the *final* record's latent
  date and peak strings rather than the daily record's corresponding latent
  strings.

The daily `ZnLatCoolDgnSAMethod` or `ZnLatHeatDgnSAMethod` and daily humidity
inputs choose the same direct-versus-difference formula independently of the
final record. Inconsistent final/daily state can therefore mix lookup day,
stored day number, date string, peak string, and humidity method in one
selected daily record.

Only one design-day record per selected mode is considered. All other daily
records remain preselection state. The final and daily arguments are not
declared disjoint; a direct caller can bind the final reference to an element
of the supplied matrix, collapsing the two logical roles without any alias
check.

### Member and assignment boundary

Exact token census finds 67 unique `ZoneSizingData` member names across the
body. The final record reads or writes all 67; selected daily records use a
60-member subset. Of those names, 32 are distinct final destinations—16
cooling and 16 heating—and 30 are distinct daily destinations. The other 35
members are source-only within CP255.

The source contains 66 record-assignment sites and one shared-bool assignment
site. Four record destinations have mutually exclusive humidity
`if`/`else` assignment sites, so at most 62 record assignments execute in one
leaf. Let `C` and `H` be zero/one cooling and heating selection indicators,
and let `d_C` and `d_H` indicate that the selected mode's final latent day
number is positive. The exact executed counts are:

| operation | count |
|---|---:|
| final record assignments | `16(C + H)` |
| daily record assignments | `15(C*d_C + H*d_H)` |
| shared flag assignments | `C` |
| whole flow-sequence copies | `C + H + C*d_C + H*d_H` |
| daily matrix element selections | `C*d_C + H*d_H` |

The maximum is 62 record assignments, one bool assignment, four sequence
copies, and two daily element selections. Runtime is constant apart from the
lengths and allocation behavior of copied strings and flow sequences; there
is no size guard.

### Invalid state and partial mutation

CP255 has no preflight over either mode or record family. There is no local
validation, diagnostic, return status, catch, transaction, cleanup guard, or
rollback. Each branch is a sequence of ordinary assignments.

For cooling, `anyLatentLoad = true` commits before `CoolSizingType` and every
record assignment. A string or flow-sequence allocation/assignment failure can
therefore leave the flag and an arbitrary final-field prefix. Daily matrix
lookup occurs after the final humidity assignment, so an invalid positive
day/role index leaves the final cooling record fully selected before any daily
write. Cooling non-return prevents the independent heating predicate.

Heating starts only after cooling returns. A heating failure retains every
completed cooling effect and an ordered heating prefix. A failure in a Space
retains the Zone and prior stored-Space results; later Space occurrences,
later Zones, and Calc4-7 are skipped. CP254 artifacts remain closed and
unchanged because they were written before this leaf.

Zero or negative selected latent day numbers are not errors. They deliberately
leave a complete selected final record paired with entirely unchanged daily
records. An invalid role or unallocated matrix can remain latent and
unobserved when neither selected mode has a positive day. Positive malformed
indices can assert, throw, or enter undefined behavior according to the
container/build boundary; CP255 establishes no defined recovery.

Raw method values, comparisons, date strings, vector extents, and IEEE values
are trusted. Once a branch selects, NaN/Inf/negative companion loads, flows,
temperatures, humidity ratios, and differences copy or combine without a
diagnostic. Final and daily humidity methods can disagree, and no invariant
requires a selected daily record's latent day number to equal its matrix
index.

### Replay and duplicate behavior

The two selection methods have different replay behavior:

- Under exact `ZoneSizing::Latent`, the positive latent volume-flow source is
  not modified. A replay normally re-enters that mode, reapplies the prefix,
  and can repair later fields if the original abnormal condition has been
  removed. A persistent bad day/index fails again.
- Under exact `ZoneSizing::SensibleAndLatent`, the branch writes
  `DesCoolLoad = DesLatentCoolLoad` or
  `DesHeatLoad = DesLatentHeatLoad` near the start. Any replay after that
  statement sees equality in the strict load predicate and skips the whole
  mode. A failure in the later final tail, humidity assignment, daily lookup,
  or daily tail can therefore create a torn state that direct leaf retry
  cannot repair.

The same equality conversion makes a completed `SensibleAndLatent` branch
one-shot under duplicate stored Space membership: the first call selects and
the next call normally skips. Exact `Latent` duplicates repeat assignments.
If an external caller clears `anyLatentLoad` after a partial or completed
combined-method cooling selection, replay can skip cooling and leave that
latch false despite retained selected fields.

A direct caller may alias `zsCalcFinalSizing` with the selected daily array
element. The source performs all final assignments before acquiring the daily
reference and has no alias barrier, so final and daily roles collapse into a
single record and later assignments overwrite the same ordinary destinations.
There is likewise no protection against aliasing inside string or sequence
storage beyond the underlying assignment types.

Whole-parent replay has a broader non-idempotence boundary. Before CP255 it
reruns EMS/CP252/CP253 and CP254. CP253 and the writer can observe a partially
or fully latent-selected ordinary record left by the first attempt; a reopened
ZSZ/SPSZ can therefore contain different peak-load, flow, design-day-header,
and ordinary-day-selector-dependent condition bytes than the first preselection
file. CP254's fixed time labels do not change, and it consumes none of the four
CP253 peak timestamp strings. Later Calc4-7 copies can also see a mixed record
even when the combined-method branch now skips.

### Parent failure propagation

The parent ordering distinguishes three failure prefixes:

- a Zone cooling or heating non-return suppresses all of its Spaces, later
  Zones, Calc4-7, facility sizing, and the run-done latch;
- a Space non-return preserves its completed parent Zone and earlier Spaces
  but suppresses the remaining traversal and same downstream work;
- a normally returning no-op role changes nothing and permits all downstream
  copies.

Because cooling sets the shared flag first, a later failure can expose
`isAnyLatentLoad = true` with no complete selected record. Heating-only
selection can expose the opposite asymmetry: fully selected heating state with
the shared flag still false. Nothing in CP255 resets either record fields or
the flag on a later parent entry.

### C++ evidence

No C++ test calls `updateZoneSizingEndZoneSizingCalc3` directly. The two
direct EndZone parent calls in `ZoneEquipmentManager.unit.cc`, near lines
4576 and 4877, set `isPulseZoneSizing = true` immediately beforehand and
dispatch zero CP255 leaves.

A fresh completing production-style census contains 51 normal and six pulse
EndZone entries. Normal entries visit 72 controlled Zone roles at the CP255
parent gate: four calculated-final Zone records have `zoneLatentSizing = true`
and 68 are false. Only four `SizingManager.unit.cc` simulations dispatch the
leaf:

| parent call site | dispatched roles |
|---:|---|
| near line 2874 | one Zone plus three Spaces |
| near line 3338 | one Zone plus three Spaces |
| near line 3802 | one Zone; Space heat balance disabled |
| near line 4246 | one Zone plus three Spaces |

The total is 13 leaves: four Zone and nine Space. The other 47 normal parent
entries and all six pulse entries dispatch none.

The first three dispatching inputs use exact `Sensible`, producing nine leaf
no-ops. The last uses `SensibleAndLatent` for one Zone and three Spaces.
Within those four calls, cooling load comparison is true only for Space 1 and
false for the other three; heating comparison is false for all four. The
complete outcome distribution is therefore:

| outcome | calls |
|---|---:|
| no selected branch | 12 |
| cooling only | 1 |
| heating only | 0 |
| cooling and heating | 0 |
| exact `Latent` volume predicate | 0 |

The selected Space 1 has final `LatentCoolDDNum = 2`, a 144-timestep flow
sequence, and `HumidityRatioDifference` at both final and daily levels. It
executes 16 final plus 15 daily record assignments, two difference-formula
arms, two whole flow-sequence copies, and the bool assignment. That is 31
record-assignment statements plus one flag statement. Treating each
144-element sequence projection as element writes gives 317 record
scalar/elements plus the flag, 318 writes total.

No assertion directly targets an ordinary CP255 destination, selected daily
record, sizing-type literal, humidity ratio, or shared flag. The assertion
near `SizingManager.unit.cc` line 4250 checks source member
`TimeStepNumAtLatentCoolMax == 72`, not the ordinary destination. Five
downstream table assertions near lines 4258-4263 cover the selected Space's
cooling load, calculated and user flow, design day, and peak timestamp.

Across the 13 roles, the four dispatching simulations assert 130 composite
table cells. The five positive-selection descendants above are the strongest
CP255 evidence; the other 125 are no-op/pass-through descendants and do not
prove exact copy ownership. No test isolates assignment order or daily
identity.

Direct gaps include exact `Latent` selection, every heating selection, both
direct-humidity branches, nonpositive final latent days, invalid day/role/
extent state, final-versus-daily method or day mismatch, type/date labels,
shared-latch asymmetry, ties, NaN/Inf/negative companion values, aliasing,
duplicate/cross-list topology, partial failure, direct retry, and whole-parent
replay.

### Rust and active-data boundary

An exhaustive audit of 721 Rust-crate and active-data files finds no exact or
snake-case CP255 symbol, `zoneSizingMethod`, `anyLatentLoad`,
`SensibleAndLatent`, or exact `SupplyAirHumidityRatio`. All 67 C++ member
spellings and all 67 mechanical snake-case projections are absent. Rust owns
no calculated-final/daily Zone or Space sizing arena on which this projection
could operate.

Rust does contain nearby `supply_air_humidity_ratio` fields and operational
IdealLoads latent calculation, outdoor-air, node, and reporting code. Those
implement bounded runtime IdealLoads behavior and output labels, not the
EnergyPlus `ZoneSizingData` method enum, latent-versus-sensible peak selector,
final/daily mutation set, or shared system-sizing latch. CP255 therefore must
not be described as absence of all latent functionality, but none of that
adjacent functionality is its counterpart.

All 61 active-data `SimulationControl` objects disable Zone sizing. Five raw
design-day objects exist, but active data contain no `Sizing:Zone`,
`Sizing:Parameters`, authored `Space`, or `SpaceList`, nor corresponding
active epJSON keys. The sole Rust autosizing fixture expects
`UnsupportedSizing` and the sizing-not-ported diagnostic; capabilities
continue to block `Sizing:*` and `ZoneSizing*`. Typed Space support exists
only behind authored-Space fail-closed behavior, and active data exercise none.

### Governance

CP255 adds only required `source_mapped`
`routine.update_zone_sizing_end_zone_sizing_calc3`. It adds no Rust target or
state, support declaration, C++ or Rust test, capability, output
implementation, comparator, case, manifest evidence, numerical or performance
claim, or conformance promotion. The parent
`algorithm.ideal_loads_zone_equipment_purchased_air_source_order` remains
`scaffold` with claim level `none`.

The generated inventory becomes 32 algorithms and 260 routines: 58
`state_mapped` plus 202 `source_mapped`, with 137 required. Project-contract
required domains become heat balance 88, HVAC 26, plant 1, and time 22. HVAC
readiness remains `0/26`; the inventory is incomplete and all 26 required
HVAC routines remain below `family_gated`.

CP256 next maps
`ZoneEquipmentManager::updateZoneSizingEndZoneSizingCalc4`, declared at
`ZoneEquipmentManager.hh` line 169 and implemented completely at
`ZoneEquipmentManager.cc` lines 2765-2799.
## CP256 `updateZoneSizingEndZoneSizingCalc4` Daily User-Array Projection

CP256 adds canonical required
`routine.update_zone_sizing_end_zone_sizing_calc4` immediately after
`update_zone_sizing_end_zone_sizing_calc3` and before `sim_zone_equipment`.
The source boundary is the declaration at `ZoneEquipmentManager.hh` line 169
and the complete definition at `ZoneEquipmentManager.cc` lines 2765-2799:

```cpp
void updateZoneSizingEndZoneSizingCalc4(
    DataSizing::ZoneSizingData &zsSizing,
    DataSizing::ZoneSizingData const &zsCalcSizing);
```

The leaf owns no `EnergyPlusData`, day/Zone/Space index, extent, or status
argument. It receives one mutable user daily record and one const calculated
daily record. The source comment calls this movement from calculated arrays
to user-modified arrays.

### Parent placement and dispatch topology

The sole production parent call sites are the EndZoneSizingCalc loops at
`ZoneEquipmentManager.cc` lines 3459-3466. On a normal pass they follow the
complete CP255 controlled/latent Zone-and-Space sweep. The closing brace at
line 3455 ends the nonpulse guard, so a pulse pass skips CP252-255 and both
sizing-file writers but still enters CP256. Calc5 starts only after every
Calc4 call completes; Calc6, Calc7, facility sizing, and the Zone-sizing
run-done latch are later still.

The parent uses zero-based linear subscripting:

```cpp
for (std::size_t i = 0; i < ZoneSizing.size(); ++i) {
    Calc4(ZoneSizing[i], CalcZoneSizing[i]);
    if (doSpaceHeatBalanceSizing) {
        for (std::size_t j = 0; j < SpaceSizing.size(); ++j) {
            Calc4(SpaceSizing[j], CalcSpaceSizing[j]);
        }
    }
}
```

For each Zone target `i`, that Zone record is copied first. With Space sizing
enabled, the complete Space target array is then copied before the next Zone
target. There is no `ZoneEquipConfig.IsControlled`, `zoneLatentSizing`,
Zone-owner, `Zone.spaceIndexes`, global-Space identity, design-day identity,
or corresponding-day filter. Unlike CP255, duplicate/cross-listed membership
cannot affect topology because membership lists are never read; the
structural repetition comes from the nested full-array sweep itself.

Let

- `Z = ZoneSizing.size()`,
- `S = SpaceSizing.size()`, and
- `I = 1` when `doSpaceHeatBalanceSizing` is true, otherwise zero.

One parent entry dispatches

`L = Z * (1 + I*S)`

leaves. It writes `Z` distinct Zone targets once and, only for `Z > 0` and
`I = 1`, writes `S` distinct Space targets `Z` times each. If `Z = 0`, the
inner loop is unreachable even with allocated Space targets. If `I = 0`,
all Space targets retain their prior values.

Normal setup at lines 830-838 allocates the arrays as

- `ZoneSizing` and `CalcZoneSizing`: `(D, N)`;
- `SpaceSizing` and `CalcSpaceSizing`: `(D, P)` when Space sizing is enabled;

where `D = TotDesDays + TotRunDesPersDays`, `N = NumOfZones`, and
`P = numSpaces`. Therefore `Z = D*N`, `S = D*P`, and the normal dispatch
count is

`D*N + I*D*D*N*P`.

Objexx `Array2::operator()` computes `(i1 * z2_) + i2`, while `operator[]`
uses the resulting zero-based flat storage directly. With normal allocation,
flat order is day-major and the Zone/Space second index varies fastest within
each day. The CP256 nesting does not pair a Zone-day with same-day Spaces:
after each individual Zone-day it replays every day and every Space.

### Extent and shape assumptions

Each loop is bounded only by its mutable destination array. The corresponding
calculated source size and both 2D shapes are never compared:

- a longer calculated source has an ignored tail;
- equal flat sizes with different 2D shapes silently pair different semantic
  day/role coordinates at the same linear index;
- a shorter calculated source can trip the Objexx assertion in an asserted
  build or become invalid access in a release build;
- an unallocated/empty Zone destination suppresses all Space work;
- source/destination overlap is not rejected.

An invalid source access occurs while preparing a call, before that leaf has
begun its field assignments. CP256 supplies no diagnostic, status, catch, or
defined recovery for such an access.

### Exact 29-field projection

The body is branchless. Every assignment has `zsSizing.<member>` on the
left and `zsCalcSizing.<same member>` on the right. The complete statement
order is:

| Order | Source line | Member | Type | Meaning / unit |
|---:|---:|---|---|---|
| 1 | 2768 | `CoolDesDay` | `std::string` | Cooling design-day name |
| 2 | 2769 | `HeatDesDay` | `std::string` | Heating design-day name |
| 3 | 2770 | `DesHeatDens` | `Real64` | Heating design air density, kg/m3 |
| 4 | 2771 | `DesCoolDens` | `Real64` | Cooling design air density, kg/m3 |
| 5 | 2772 | `HeatDDNum` | `int` | Heating design-day index |
| 6 | 2773 | `CoolDDNum` | `int` | Cooling design-day index |
| 7 | 2775 | `DesHeatLoad` | `Real64` | Heating design load, W |
| 8 | 2776 | `DesHeatMassFlow` | `Real64` | Heating mass flow, kg/s |
| 9 | 2777 | `ZoneTempAtHeatPeak` | `Real64` | Zone temperature at heating peak, C |
| 10 | 2778 | `OutTempAtHeatPeak` | `Real64` | Outdoor temperature at heating peak, C |
| 11 | 2779 | `ZoneRetTempAtHeatPeak` | `Real64` | Return temperature at heating peak, C |
| 12 | 2780 | `ZoneHumRatAtHeatPeak` | `Real64` | Zone humidity ratio at heating peak, kg/kg |
| 13 | 2781 | `OutHumRatAtHeatPeak` | `Real64` | Outdoor humidity ratio at heating peak, kg/kg |
| 14 | 2782 | `TimeStepNumAtHeatMax` | `int` | Heating peak timestep index |
| 15 | 2783 | `DesHeatVolFlow` | `Real64` | Heating volume flow, m3/s |
| 16 | 2784 | `DesHeatCoilInTemp` | `Real64` | Heating coil-in temperature, C |
| 17 | 2785 | `DesHeatCoilInHumRat` | `Real64` | Heating coil-in humidity ratio, kg/kg |
| 18 | 2786 | `CoolDesHumRat` | `Real64` | Cooling design supply humidity ratio, kg/kg |
| 19 | 2788 | `DesCoolLoad` | `Real64` | Cooling design load, W |
| 20 | 2789 | `DesCoolMassFlow` | `Real64` | Cooling mass flow, kg/s |
| 21 | 2790 | `ZoneTempAtCoolPeak` | `Real64` | Zone temperature at cooling peak, C |
| 22 | 2791 | `OutTempAtCoolPeak` | `Real64` | Outdoor temperature at cooling peak, C |
| 23 | 2792 | `ZoneRetTempAtCoolPeak` | `Real64` | Return temperature at cooling peak, C |
| 24 | 2793 | `ZoneHumRatAtCoolPeak` | `Real64` | Zone humidity ratio at cooling peak, kg/kg |
| 25 | 2794 | `OutHumRatAtCoolPeak` | `Real64` | Outdoor humidity ratio at cooling peak, kg/kg |
| 26 | 2795 | `TimeStepNumAtCoolMax` | `int` | Cooling peak timestep index |
| 27 | 2796 | `DesCoolVolFlow` | `Real64` | Cooling volume flow, m3/s |
| 28 | 2797 | `DesCoolCoilInTemp` | `Real64` | Cooling coil-in temperature, C |
| 29 | 2798 | `DesCoolCoilInHumRat` | `Real64` | Cooling coil-in humidity ratio, kg/kg |

This is exactly two strings, four integers, and 23 `Real64` values: 29
unique member names, 29 destination writes, 29 source reads, and 58 member
accesses. The apparent grouping is two names, two densities, two day
indexes, 11 heating values, the isolated cooling design humidity ratio, and
11 cooling values. `CoolDesHumRat` appears between the heating and cooling
groups; the adjacent `HeatDesHumRat` member is not copied.

There is no predicate, branch, loop, arithmetic, unit conversion, clamp,
finite/range check, psychrometric call, allocation of member arrays, child
call, or diagnostic. Raw negative, NaN, and infinite `Real64` values
propagate as ordinary assignments. Invalid design-day or timestep integers
are copied without dereference in this leaf and can become downstream user
state.

### CP255 carry-forward and omissions

Across its selected cooling and heating daily projections, CP255 can write
30 unique calculated-daily destinations. CP256 carries only 23:

- cooling carries 12 of 15 and omits `cCoolDDDate`, `CoolFlowSeq`, and
  `CoolPeakDateHrMin`;
- heating carries 11 of 15 and omits `cHeatDDDate`, `HeatFlowSeq`,
  `HeatPeakDateHrMin`, and `HeatDesHumRat`.

The other six Calc4 destinations are `DesHeatDens`, `DesCoolDens`,
`OutTempAtHeatPeak`, `OutHumRatAtHeatPeak`, `OutTempAtCoolPeak`, and
`OutHumRatAtCoolPeak`; CP255 does not mutate them. Calc4 also copies no
latent-source member, load sequence, sizing-type label, no-DOAS field,
thermostat field, method enum, or shared latent flag.

`fillZoneSizingFromInput` initializes user-daily `HeatDesHumRat` separately
from the calculated daily record. CP255 can later replace calculated
`HeatDesHumRat` for selected latent heating, but CP256 leaves the user field
untouched while copying cooling design humidity ratio. A user daily record
can therefore combine the new heating peak/load/flow/coil fields with its
prior/input heating design humidity ratio. This is source behavior, not a
symmetric-projection inference.

Calc6 later copies the ordinary flow and load sequences plus condition
sequences. That later work does not make those arrays part of Calc4, and
Calc4 failure prevents Calc6 from starting. The CP254 ZSZ/SPSZ writer has
already closed its output before CP255 and CP256 and consumes calculated
rather than user arrays, so CP256 cannot retroactively alter those bytes.

### Operation and storage bounds

For `L = Z*(1 + I*S)` leaves, CP256 executes exactly

- `29L` assignment statements,
- `2L` potentially allocation-bearing string assignments, and
- `27L` scalar assignments.

Its local work is `Theta(L + B)`, where `B` is the sum of the two copied
design-day-name lengths over every invocation, including redundant Space
invocations. Local auxiliary state is constant; destination strings may
reuse capacity or allocate according to their library state. No sequence
length contributes because no sequence is touched.

The number of distinct targets in one parent entry is
`Z + I*S` only when `Z > 0`; the number of writes is larger by
`29*I*S*(Z-1)` whenever Space sizing is enabled and `Z > 1`. Those extra
writes have no different source index or field transformation.

### Failure, replay, and alias boundary

There is no local validation, status, catch, transaction, cleanup, or
rollback. `CoolDesDay` and `HeatDesDay` are the first two statements and the
only assignment sites that can allocate for otherwise valid live records:

- if the first string copy throws, no CP256 statement has completed;
- if the second throws, the completed cooling name remains and no scalar has
  been written;
- after both strings complete, the remaining 27 built-in scalar assignments
  are nonthrowing for valid live records.

The failing string object's state follows the `std::string` operation's own
guarantee; Calc4 adds none. Invalid references, assertion termination, or
release invalid access have no defined local recovery and must not be
described as an exception-safe continuation.

Parent mutation order is Zone `i`, then all Space `j`, then Zone `i+1`. A
defined string-copy failure in a Space leaf preserves the current completed
Zone, every earlier Zone, all earlier Space copies, and any earlier repeated
copy of the same Space target. It suppresses later Space/Zone work and all
following Calc5-7, facility-sizing, and run-done work. The `SizingManager`
caller invokes EndZone `UpdateZoneSizing`, then facility sizing, then sets
`ZoneSizingRunDone`; CP256 abnormal exit therefore blocks both later steps.

A later successful direct retry always starts again at `CoolDesDay` and
re-executes all 29 fields. For a stable distinct source it repairs a torn
destination and completed replay is value-idempotent. The redundant Space
sweeps likewise converge to the same values only while their source is
stable; allocation behavior and work still repeat.

The signature permits a valid exact alias between source and destination.
Because every assignment is same-name, exact alias becomes 29 self-copies
and causes no cross-field transformation. The const source reference does
not assert disjointness, and there is no overlap check. Equal-sized
production state arrays are distinct by construction, but direct callers
receive no such enforcement from the leaf.

Whole-parent replay is broader. On a nonpulse retry the parent reruns EMS
and any gated CP252 work, then CP253, the CP254 writer, and any gated CP255
work before reaching Calc4 again. Those stages observe the fully retained
calculated/final state established before the Calc4 failure; their replay can
then change the calculated daily source. A later completed Calc4 retry can
therefore overwrite its earlier user-array prefix with different source
values. CP254 also reruns before CP255/Calc4, so rebuilt sizing artifacts can
differ from the first attempt. Pulse retry skips that upstream nonpulse
block and projects the retained calculated arrays directly.

### C++ test reachability census

There is no direct Calc4 call in the C++ unit corpus. Excluding two direct
parent tests, completing high-level sizing paths contain 51 normal EndZone
entries plus six additional pulse entries:

- normal Zone target-size histogram:
  `Z=1:10`, `2:22`, `3:1`, `4:11`, `6:4`, `10:1`, `12:2`,
  producing 159 Zone leaves;
- pulse histogram: `Z=2:3`, `4:1`, `6:2`, producing 22 more Zone leaves;
- seven normal Space-enabled contexts have `(Z,S)=(2,6)` five times,
  `(1,3)` once, and `(3,9)` once, producing
  `5*2*6 + 1*1*3 + 1*3*9 = 90` Space leaves;
- no pulse entry has a Space leaf.

Those high-level paths execute 271 Calc4 leaves. The two bare
`UpdateZoneSizing(EndZoneSizingCalc)` tests at
`ZoneEquipmentManager.unit.cc` lines 4576 and 4877 are pulse entries with
`Z=1`, `S=0`; each adds one Zone leaf. The complete related corpus therefore
contains 59 parent entries, 273 leaf invocations, and

`273 * 29 = 7,917`

assignment-statement executions: 546 string plus 7,371 scalar assignments.

There are 203 distinct test-local targets. Production owns 159 distinct
Zone targets and 42 distinct Space targets; the two direct tests add two.
The 22 pulse-capable production Zone targets are each revisited by their
normal entry. Space multiplicity is three targets once, 30 twice, and nine
three times. Overall target multiplicity is therefore 142 once, 52 twice,
and nine three times. The 42 Space targets receive 90 calls, so 48 calls are
structurally redundant re-copies.

The production Zone leaves include 153 controlled and 28 uncontrolled
records. All 90 Space calls occur in seven controlled-Zone contexts, and
both direct-parent leaves are controlled. Of the 271 production leaves, 44
have latent sizing enabled: 30 use `Sensible`, 14 use
`SensibleAndLatent`, and none uses exact `Latent`; 227 are latent-off.
Because Calc4 reads none of those gates or methods, every category executes
the same 29 assignments. Exercised flat Zone sizes are
`{1,2,3,4,6,10,12}` and Space sizes are `{0,3,6,9}`.

All 29 assignment sites execute on every leaf, but execution is not an
identity oracle. No test calls the leaf directly, no Calc4-executing test
asserts a `SpaceSizing` destination, and no such test asserts any of the 29
`ZoneSizing` destination members. The direct parent tests inspect
`CalcZoneSizing`, `CalcFinalZoneSizing`, or `FinalZoneSizing` instead.

There are 803 static post-call gtest assertion sites plus one
invocation-site `EXPECT_NO_THROW` around `ManageSizing`. None compares a
Calc4 source member with its target. Sizing-table assertions consume
calculated-final/final state, and the only daily user-array reads identified
in that table path are `CoolTstatTempSeq` and `HeatTstatTempSeq`, both
outside the 29-field projection.
Calc7 later accesses a 19-member subset of Calc4 destinations: 13
right-hand-side reads and six conditional left-hand-side rewrites. Reported
results nevertheless combine Calc5/Calc7 state and provide no uniquely
attributable Calc4 copy oracle. No exact nondefault source-field census can
be inferred without a target read or sentinel.

The separate Rezero unit test aliases `ZoneSizing` and asserts reset behavior
for 28 of the 29 Calc4 fields; `CoolDesHumRat` is commented out. That test
never dispatches EndZone or Calc4, so it proves adjacent reset behavior, not
calculated-to-user copy identity or statement order. Nested repetition,
mismatched shape/extent, alias, raw IEEE/invalid indexes, string failure,
partial parent state, and replay remain unisolated.

### Rust, data, and claim boundary

The Rust/data audit covers 721 current-worktree files under `crates` and
`data`. It finds no exact or snake-case
Calc4 key/helper, no `ZoneSizingData`, `CalcZoneSizing`, `SpaceSizing`, or
daily calculated/user arena counterpart, and zero occurrences of all 29
exact member names and all 29 mechanical snake-case projections in that
scope.

This does not mean Rust has no adjacent HVAC state. It owns current-timestep
`ZoneSysEnergyDemand`, operational IdealLoads supply/rate/mass-flow/humidity
results, typed IdealLoads limits, and density, outdoor-air, node, reporting,
and design-day schedule-label state. Those are input, timestep, or report
concepts rather than a calculated-daily to user-daily Zone/Space sizing
record projection.

The active data census contains 61 `SimulationControl` objects, all with
Zone sizing disabled, and five raw `SizingPeriod:DesignDay` objects. It
contains no active `Sizing:Zone`, `Sizing:Parameters`, authored `Space`, or
`SpaceList`, and no corresponding epJSON keys. SimulationControl and design
days are ignored partial inputs rather than sizing execution. Sizing and
authored-Space object families remain explicit runtime blockers; the sole
autosizing fixture is expected to fail with `UnsupportedSizing`.

CP256 therefore adds only one canonical required `source_mapped` row and the
matching ordered HVAC project-contract requirement. It adds no Rust target,
state mapping, support declaration, test, capability, output implementation,
comparator, case, manifest evidence, numerical claim, performance claim, or
conformance promotion.

The inventory becomes 32 algorithms and 261 routines, split 58
`state_mapped` plus 203 `source_mapped`, with 138 required. Domain-required
counts are heat-balance 88, HVAC 27, plant 1, and time/schedule 22. The
`ideal_loads_zone_equipment_purchased_air_source_order` parent now owns 27
rows but remains `scaffold` at claim level `none`; HVAC readiness remains
`0/27`.

CP257 next maps
`ZoneEquipmentManager::updateZoneSizingEndZoneSizingCalc5`, declared at
`ZoneEquipmentManager.hh` line 171 and implemented completely at
`ZoneEquipmentManager.cc` lines 2801-2842. Calc6 begins at line 2844.
## CP257 `updateZoneSizingEndZoneSizingCalc5` Final User-Array Projection

CP257 adds canonical required
`routine.update_zone_sizing_end_zone_sizing_calc5` immediately after
`update_zone_sizing_end_zone_sizing_calc4` and before `sim_zone_equipment`.
The source boundary is the declaration at `ZoneEquipmentManager.hh` line 171
and the complete definition at `ZoneEquipmentManager.cc` lines 2801-2842:

```cpp
void updateZoneSizingEndZoneSizingCalc5(
    DataSizing::ZoneSizingData &zsFinalSizing,
    DataSizing::ZoneSizingData const &zsCalcFinalSizing);
```

The leaf owns no `EnergyPlusData`, Zone/Space index, extent, or status
argument. It receives one mutable user-final sizing record and one const
calculated-final record. The source comment describes movement from
calculated-final arrays into user-modified final arrays.

### Parent placement and final-record topology

The sole production parent call sites are the EndZoneSizingCalc loops at
`ZoneEquipmentManager.cc` lines 3468-3475. The complete Calc4 daily-array
sweep ends first at line 3466. The nonpulse guard ended at line 3455, so
Calc5 executes on pulse and normal passes. Calc6 daily/final sequence loops,
Calc7 final adjustments, facility sizing, and the run-done latch all wait
for the entire Calc5 sweep.

The parent uses zero-based `EPVector` subscripting:

```cpp
for (std::size_t i = 0; i < FinalZoneSizing.size(); ++i) {
    Calc5(FinalZoneSizing[i], CalcFinalZoneSizing[i]);
    if (doSpaceHeatBalanceSizing) {
        for (std::size_t j = 0; j < FinalSpaceSizing.size(); ++j) {
            Calc5(FinalSpaceSizing[j], CalcFinalSpaceSizing[j]);
        }
    }
}
```

Each final Zone `i` is copied before the complete final Space array. There
is no `ZoneEquipConfig.IsControlled`, `zoneLatentSizing`, sizing-method,
Space owner, `Zone.spaceIndexes`, global-Space identity, design-day, name,
or corresponding-role filter. Stored duplicate/cross-listed memberships do
not affect Calc5 because membership lists are not read; any repetition is
created by the nested full-array loop.

Let

- `F = FinalZoneSizing.size()`,
- `G = FinalSpaceSizing.size()`, and
- `I = 1` when `doSpaceHeatBalanceSizing` is true, otherwise zero.

One parent entry dispatches

`L = F * (1 + I*G)`

leaves. It writes `F` distinct final Zone targets once and, only for
`F > 0` and `I = 1`, writes `G` distinct final Space targets `F` times each.
If `F = 0`, allocated Space targets are unreachable. If `I = 0`, every
Space target retains prior state.

Normal setup allocates `FinalZoneSizing` and `CalcFinalZoneSizing` with
`N = NumOfZones` elements and, under Space sizing, allocates
`FinalSpaceSizing` and `CalcFinalSpaceSizing` with `P = numSpaces`.
Therefore normal dispatch is

`N + I*N*P`.

Unlike Calc4's 2D daily arrays, these are 1D final-role vectors and have no
design-day multiplier. Nevertheless, the source still copies all `P` final
Spaces after every one of the `N` final Zones instead of pairing by owner.

### Extent and identity assumptions

Each loop is bounded only by its mutable destination vector. The calculated
source size and record identity are never compared:

- a longer calculated source has an ignored tail;
- equal sizes pair solely by zero-based flat index, without checking
  `ZoneNum`, name, owner, or other identity;
- in an asserted build, a shorter source reaches `EPVector`'s debug
  `vector::at` path and throws before the leaf begins;
- release subscripting is unchecked and a short source has undefined
  behavior;
- an empty final Zone destination suppresses all Space work;
- source/destination overlap is not rejected.

Calc5 adds no diagnostic, status, catch, or recovery around those accesses.
Normal allocation produces distinct, equal-sized source/destination pairs;
the leaf signature does not enforce that production invariant.

### Exact 35-assignment projection

The body is branchless. The complete destination/right-hand-side order is:

| Order | Line | Destination | Right-hand side | Type / unit |
|---:|---:|---|---|---|
| 1 | 2805 | `CoolDesDay` | `CoolDesDay` | `std::string`, cooling day name |
| 2 | 2806 | `HeatDesDay` | `HeatDesDay` | `std::string`, heating day name |
| 3 | 2807 | `DesHeatDens` | `DesHeatDens` | `Real64`, kg/m3 |
| 4 | 2808 | `DesCoolDens` | `DesCoolDens` | `Real64`, kg/m3 |
| 5 | 2809 | `HeatDDNum` | `HeatDDNum` | `int`, design-day index |
| 6 | 2810 | `CoolDDNum` | `CoolDDNum` | `int`, design-day index |
| 7 | 2812 | `DesHeatLoad` | `DesHeatLoad` | `Real64`, W |
| 8 | 2813 | `DesLatentHeatLoad` | `DesLatentHeatLoad` | `Real64`, W |
| 9 | 2814 | `NonAirSysDesHeatLoad` | `DesHeatLoad` | `Real64`, W |
| 10 | 2815 | `DesHeatMassFlow` | `DesHeatMassFlow` | `Real64`, kg/s |
| 11 | 2816 | `ZoneTempAtHeatPeak` | `ZoneTempAtHeatPeak` | `Real64`, C |
| 12 | 2817 | `OutTempAtHeatPeak` | `OutTempAtHeatPeak` | `Real64`, C |
| 13 | 2818 | `ZoneRetTempAtHeatPeak` | `ZoneRetTempAtHeatPeak` | `Real64`, C |
| 14 | 2819 | `ZoneHumRatAtHeatPeak` | `ZoneHumRatAtHeatPeak` | `Real64`, kg/kg |
| 15 | 2820 | `OutHumRatAtHeatPeak` | `OutHumRatAtHeatPeak` | `Real64`, kg/kg |
| 16 | 2821 | `TimeStepNumAtHeatMax` | `TimeStepNumAtHeatMax` | `int`, timestep index |
| 17 | 2822 | `DesHeatVolFlow` | `DesHeatVolFlow` | `Real64`, m3/s |
| 18 | 2823 | `NonAirSysDesHeatVolFlow` | `DesHeatVolFlow` | `Real64`, m3/s |
| 19 | 2824 | `DesHeatCoilInTemp` | `DesHeatCoilInTemp` | `Real64`, C |
| 20 | 2825 | `DesHeatCoilInHumRat` | `DesHeatCoilInHumRat` | `Real64`, kg/kg |
| 21 | 2826 | `CoolDesHumRat` | `CoolDesHumRat` | `Real64`, kg/kg |
| 22 | 2828 | `DesCoolLoad` | `DesCoolLoad` | `Real64`, W |
| 23 | 2829 | `DesLatentCoolLoad` | `DesLatentCoolLoad` | `Real64`, W |
| 24 | 2830 | `NonAirSysDesCoolLoad` | `DesCoolLoad` | `Real64`, W |
| 25 | 2831 | `DesCoolMassFlow` | `DesCoolMassFlow` | `Real64`, kg/s |
| 26 | 2832 | `ZoneTempAtCoolPeak` | `ZoneTempAtCoolPeak` | `Real64`, C |
| 27 | 2833 | `OutTempAtCoolPeak` | `OutTempAtCoolPeak` | `Real64`, C |
| 28 | 2834 | `ZoneRetTempAtCoolPeak` | `ZoneRetTempAtCoolPeak` | `Real64`, C |
| 29 | 2835 | `ZoneHumRatAtCoolPeak` | `ZoneHumRatAtCoolPeak` | `Real64`, kg/kg |
| 30 | 2836 | `OutHumRatAtCoolPeak` | `OutHumRatAtCoolPeak` | `Real64`, kg/kg |
| 31 | 2837 | `TimeStepNumAtCoolMax` | `TimeStepNumAtCoolMax` | `int`, timestep index |
| 32 | 2838 | `DesCoolVolFlow` | `DesCoolVolFlow` | `Real64`, m3/s |
| 33 | 2839 | `NonAirSysDesCoolVolFlow` | `DesCoolVolFlow` | `Real64`, m3/s |
| 34 | 2840 | `DesCoolCoilInTemp` | `DesCoolCoilInTemp` | `Real64`, C |
| 35 | 2841 | `DesCoolCoilInHumRat` | `DesCoolCoilInHumRat` | `Real64`, kg/kg |

There are 35 unique destinations, 35 destination writes, 35 source reads,
and 70 member accesses. The right-hand sides have only 31 unique names:
31 sites are same-name copies, while four sites fan ordinary design
load/volume flow into distinct NonAir destinations. The fan-outs are
interleaved at statements 9, 18, 24, and 33; source NonAir fields are never
read.

The body contains two strings, four integers, and 29 `Real64` assignments.
There is no predicate, loop, arithmetic, unit conversion, clamp, finite or
range check, psychrometric call, sequence access, child call, state
argument, or diagnostic. Negative, NaN, infinite, and invalid design-day or
timestep values are assigned without dereference or interpretation here.

### Calc4 delta and source-comment boundary

Every one of Calc4's 29 destinations appears in Calc5. The executable adds
six destination/statement sites:

- `DesLatentHeatLoad` and `DesLatentCoolLoad` as same-name copies;
- heating and cooling `NonAirSysDes*Load`;
- heating and cooling `NonAirSysDes*VolFlow`.

The source comment at line 2804 says the routines differ by two extra
fields. That is not literal destination or statement count: the delta is
six. It is accurate only under the narrower observation that the two latent
loads are the only new unique right-hand-side names. Four additional
destinations reuse ordinary load or volume-flow sources. The two volume
fan-out lines carry explicit `SpaceSizing TODO: Suspicious` comments; Calc5
does not resolve or condition those comments.

`DataSizing` distinguishes ordinary scaled `Des*` fields from `NonAirSys*`
base fields, but Calc5 ignores calculated-final NonAir source values and
forces each user-final NonAir load/volume equal to the corresponding
ordinary calculated-final value. It copies latent design loads but not
latent design volume, mass flow, conditions, or sequences. It again copies
`CoolDesHumRat` while omitting `HeatDesHumRat`.

### CP255 carry-forward and omissions

CP255 can write 32 unique calculated-final destinations across cooling and
heating. Calc5 directly carries 23:

- cooling carries 12 of 16 and omits `CoolSizingType`, `cCoolDDDate`,
  `CoolFlowSeq`, and `CoolPeakDateHrMin`;
- heating carries 11 of 16 and omits `HeatSizingType`, `cHeatDDDate`,
  `HeatFlowSeq`, `HeatPeakDateHrMin`, and `HeatDesHumRat`.

The other 12 Calc5 destinations are six densities/outdoor peak conditions,
two latent loads, and four NonAir destinations. CP255 does not write those
destinations, but it can replace the four ordinary load/volume sources read
by the NonAir fan-outs. A selected cooling branch can therefore influence
14 Calc5 destinations, a selected heating branch 13, and both modes 27
unique destinations: 23 direct same-name carry-forwards plus four fan-outs.

`fillZoneSizingFromInput` initializes user-final `HeatDesHumRat` separately
from calculated-final state. CP255 can replace calculated-final heating
design humidity ratio, but Calc5 leaves the user-final field at prior/input
state while copying the selected heat load, flow, conditions, coil inputs,
and NonAir load/volume. This is an executable asymmetry, not a symmetric
projection.

Calc5 copies no sizing-type label, day-date string, peak timestamp, flow or
load sequence, method enum, thermostat field, no-DOAS field, or shared
latent flag. Calc6 later copies 14 sequence families but has zero overlap
with these 35 scalar/string destinations.

### Operation and storage bounds

For `L = F*(1 + I*G)` leaves, CP257 executes exactly

- `35L` assignment statements,
- `2L` potentially allocation-bearing string assignments,
- `4L` integer assignments, and
- `29L` `Real64` assignments.

Its local work is `Theta(L + B)`, where `B` is the total copied
design-day-name length over every invocation, including repeated final
Space calls. Local auxiliary state is constant; destination strings may
reuse capacity or allocate. No member-array extent contributes.

One parent entry owns `F + I*G` distinct targets only for `F > 0`.
When Space sizing is enabled and `F > 1`, the structural repetition adds
`I*G*(F-1)` redundant leaves and `35*I*G*(F-1)` assignment statements.
No field or source index differs between repeated copies while sources stay
stable.

### Failure, replay, and alias boundary

There is no local validation, status, catch, transaction, cleanup, or
rollback. `CoolDesDay` and `HeatDesDay` are the first two statements and the
only allocation-bearing operations for otherwise valid live records:

- if the first string copy throws, no Calc5 assignment has completed;
- if the second throws, the completed cooling name remains and no scalar has
  been written;
- after both strings complete, the remaining 33 built-in scalar assignments
  are nonthrowing for valid live records.

The failing string's state follows its library operation; Calc5 adds no
guarantee. A debug `EPVector::at` extent exception occurs while preparing a
call, before the leaf starts. Release invalid access has no defined
continuation.

Parent mutation order is final Zone `i`, all final Space `j`, then Zone
`i+1`. A defined Space string-copy failure preserves the completed current
Zone, all earlier Zones, earlier Space targets, and any earlier repeated
copy of the current Space. It suppresses later leaves and all following
Calc6, Calc7, facility-sizing, and run-done work.

A later successful retry from a stable distinct source starts again at
`CoolDesDay`, overwrites all 35 destinations, repairs a torn target, and is
value-idempotent after completion. Exact alias is different from Calc4:
31 same-name self-copies are interleaved with four
`NonAirSys* <- Des*` overwrites. None of the four fan-out targets is later
read as a right-hand side, so the alias transform is deterministic and a
completed replay remains fixed at that projection. The const source
reference does not prove disjointness.

Whole-parent replay always reruns EMS. At retry entry the calculated-final
source is fully retained, while the user-final destination retains the
completed Calc5 prefix. For every controlled Zone registered during setup, eight heat/cool
mass-flow, load, density, and volume-flow scalars from its Final Zone record
are registered as EMS internal variables. EMS can observe the registered
subset of the completed prefix and feed it into the six calculated-final
load/flow actuators. A defined failure in the current
leaf's first or second string occurs before its scalars, so numeric feedback
comes only from previously completed Zone leaves.

On a nonpulse retry the parent then replays any gated CP252 work, CP253,
the CP254 writer, any gated CP255 work, and CP256 before returning to Calc5.
Those stages can change the calculated-final source. A completed retry may
therefore overwrite the prior user-final prefix with different values, and
rebuilt CP254 artifacts can differ.

Pulse skips CP252-255 but not EMS and retains the same user-final feedback
path. EMS can apply the six Zone calculated-final load/flow overrides before
Calc5; pulse replay is therefore not universally a pure retained-source
copy. Space calculated-final sources and non-overridden fields remain
outside those six Zone override sites. Within the current attempt Calc5
cannot retroactively alter already closed CP254 output.

### C++ test execution census

There is no direct Calc5 leaf call in the C++ unit tree. Excluding two bare
parent tests, the completing high-level corpus contains 51 normal EndZone
entries plus six additional pulse entries.

Normal final Zone target-size histogram is

`F=1:33, 2:11, 3:4, 5:1, 6:2`,

which yields 84 Zone leaves. The seven Space-enabled normal contexts all
have `(F,G)=(1,3)`, adding 21 Space leaves. The source permits repeated
Space copying for `F > 1`, but the test corpus never combines that condition
with Space sizing, so actual structurally redundant Space calls are zero.

The six pulse entries have histogram

`F=1:3, 2:1, 3:2`,

which adds 11 Zone leaves and no Space leaf. The two bare pulse parent calls
at `ZoneEquipmentManager.unit.cc` lines 4576 and 4877 each add one
`(F,G)=(1,0)` Zone leaf. Total execution is therefore

- 59 parent entries;
- 118 Calc5 leaves;
- 4,130 assignment statements;
- 236 string, 472 integer, and 3,422 `Real64` assignments.

There are 107 distinct test-local targets. Ninety-six execute once, while
11 Zone targets are revisited by pulse and normal entries, giving
`96*1 + 11*2 = 118`. The contexts comprise 104 controlled or
controlled-owner leaves and 14 uncontrolled Zone leaves.

Thirteen leaves have latent sizing enabled: nine `Sensible`, four
`SensibleAndLatent`, and zero exact `Latent`; 105 are latent-off. Calc5 has
no corresponding gate or method read, so all categories execute the same
35 statements.

### C++ assertion and reset evidence

The 53 relevant test blocks contain 803 static assertion sites after their
calls plus one invocation-site `EXPECT_NO_THROW`. Only eight sites read a
direct final target, covering six Calc5 destination names:

- four positive, downstream-preserved Calc5-dependent reads: two WindowAC
  `DesCoolVolFlow` sites and BaseClassSizing `ZoneTempAtHeatPeak` plus
  `DesHeatLoad`;
- two BaseClassSizing default-zero reads for `DesHeatVolFlow` and
  `DesHeatMassFlow`;
- two direct-parent peak-temperature reads that Calc7 demonstrably
  overwrites from calculated-final 23/23 to final 22/24.

No site compares a Calc5 source member with its destination or isolates
statement order, and `FinalSpaceSizing` has no direct target assertion.

A bounded set of 300 report descendants consists of 290 heat/cool table cells for
29 SizingManager roles plus ten WindowAC zero-heating cells. All 300 use
final design-volume-flow sign as a gate. Positive heating for 21 roles and
positive cooling for 29 roles cause 100 user-flow/design-day cells to render
target values; the other 200 cells render calculated-final values or
zero/N/A constants after consulting the final-flow gate. These are useful
downstream descendants, but all run after Calc7 and none is a source-target
copy oracle.

Calc6 copies 14 sequence families and overlaps zero of the 35 Calc5
destinations. Calc7 accesses 32 of the 35 names: the two densities are
read-only there and 30 names are write-capable. Only
`DesLatentHeatLoad`, `DesLatentCoolLoad`, and `CoolDesHumRat` are untouched
by Calc7's final-record body. Direct and report assertions are therefore
composite evidence unless separately isolated.

`ZoneSizingData::zeroMemberData` has a 30-name overlap with Calc5: 28
Calc4-common reset fields plus the two latent loads. It leaves the four
NonAir fields and `CoolDesHumRat` untouched. The focused Rezero test actively
checks only the 28 common daily fields over five Zones, 15 days, and two
daily arrays: 56 static statements and 4,200 dynamic checks. It does not
seed or assert the two latent loads; expectations for NonAir fields and
`CoolDesHumRat` are commented out. Ten final calculated/user records
quick-return because their member-array sentinel is unallocated, so there
is no final mutation assertion, no Space assertion, and no Calc5 execution.

The suite therefore leaves direct copy identity/order, latent-load
retention, every NonAir fan-out, the omitted heating humidity ratio,
`F > 1` Space repetition, malformed extent/identity, exact alias, raw
IEEE/index values, string failure, partial parent state, and replay
unisolated.

### Rust, data, and claim boundary

The Rust/data audit covers 721 UTF-8-readable current-worktree files
returned by `rg --files crates data`.
It finds no exact or snake-case Calc5 key/helper, no
`FinalZoneSizing`/`CalcFinalZoneSizing` or final Space sizing arena
counterpart, zero occurrences of all 35 exact destination names, and zero
occurrences of all 35 mechanical snake-case projections in that scope. The
31 unique right-hand-side names are a subset and are absent as well.

Rust does own adjacent current-timestep `ZoneSysEnergyDemand`, operational
IdealLoads sensible/latent rates and supply temperature/humidity/mass flow,
typed `AutosizeOrNumber` limits, and density, outdoor-air, node, and report
state. Those are input, timestep, or report concepts, not a
calculated-final to user-final Zone/Space sizing record, retained latent
design-load projection, or four NonAir fan-outs.

The active data census contains 61 `SimulationControl` objects, all with
Zone sizing disabled, and five raw `SizingPeriod:DesignDay` objects. It
contains no active `Sizing:Zone`, `Sizing:Parameters`, authored `Space`, or
`SpaceList`, and no corresponding epJSON keys. SimulationControl and design
days remain ignored partial inputs rather than sizing execution. Sizing and
authored-Space object families remain run-blocked; the sole autosizing
fixture expects `UnsupportedSizing`.

CP257 therefore adds only one canonical required `source_mapped` row and
the matching ordered HVAC project-contract requirement. It adds no Rust
target, state mapping, support declaration, test, capability, output
implementation, comparator, case, manifest evidence, numerical claim,
performance claim, or conformance promotion.

The inventory becomes 32 algorithms and 262 routines, split 58
`state_mapped` plus 204 `source_mapped`, with 139 required. Domain-required
counts are heat-balance 88, HVAC 28, plant 1, and time/schedule 22. The
`ideal_loads_zone_equipment_purchased_air_source_order` parent now owns 28
rows but remains `scaffold` at claim level `none`; HVAC readiness remains
`0/28`.

CP258 next maps
`ZoneEquipmentManager::updateZoneSizingEndZoneSizingCalc6`, declared at
`ZoneEquipmentManager.hh` lines 173-175 and implemented completely at
`ZoneEquipmentManager.cc` lines 2844-2865. Calc7 begins at line 2867.

## CP258 `updateZoneSizingEndZoneSizingCalc6` Daily and Final User-Sequence Projection

CP258 adds canonical required
`routine.update_zone_sizing_end_zone_sizing_calc6` immediately after
`update_zone_sizing_end_zone_sizing_calc5` and before `sim_zone_equipment`.
The source boundary is the declaration at `ZoneEquipmentManager.hh` lines
173-175 and the complete definition at `ZoneEquipmentManager.cc` lines
2844-2865:

```cpp
void updateZoneSizingEndZoneSizingCalc6(
    DataSizing::ZoneSizingData &zsSizing,
    DataSizing::ZoneSizingData const &zsCalcSizing,
    int const numTimeStepsInDay);
```

The leaf owns no `EnergyPlusData`, Zone/Space identity, day identity,
extent, status, or diagnostic argument. It receives one mutable user sizing
record, one const calculated record, and an integer loop bound.

### Parent placement and four-call topology

The EndZoneSizingCalc parent completes the full Calc4 daily scalar sweep at
lines 3459-3466 and the full Calc5 final scalar sweep at lines 3468-3475
before entering Calc6. The nonpulse guard ended at line 3455, so the four
sole production call sites all execute on pulse and normal entries:

| Phase | Receiver | Source | Parent lines |
|---|---|---|---:|
| Daily Zone | `ZoneSizing(day, zone)` | `CalcZoneSizing(day, zone)` | 3482-3484 |
| Daily Space | `SpaceSizing(day, space)` | `CalcSpaceSizing(day, space)` | 3487-3489 |
| Final Zone | `FinalZoneSizing(zone)` | `CalcFinalZoneSizing(zone)` | 3500-3502 |
| Final Space | `FinalSpaceSizing(space)` | `CalcFinalSpaceSizing(space)` | 3505-3507 |

The complete daily sweep precedes the complete final sweep. Daily traversal
orders design/run-design day ascending, then Zone number ascending, skips
uncontrolled Zones, calls the Zone leaf, and visits that Zone's stored
`spaceIndexes` in list order when Space sizing is enabled. Only after every
day completes does the final traversal repeat controlled Zone then stored
Space order. Calc7 lines 3511-3531 cannot begin until both sweeps finish.

Define:

- `D = TotDesDays + TotRunDesPersDays` and `E = max(D, 0)`;
- `Z = max(NumOfZones, 0)`;
- `C` as the number of controlled Zones among `1..Z`;
- `M` as the number of stored Space-index occurrences under those Zones;
- `U` as the number of unique valid Space identities among those
  occurrences;
- `I` as one when `doSpaceHeatBalanceSizing` is true, otherwise zero;
- `T = numTimeStepsInDay` and `Q = max(T, 0)`.

For valid topology, the parent dispatches

`K = (E + 1) * (C + I*M)`

leaves: `E*(C+I*M)` daily plus `C+I*M` final. It addresses
`(E+1)*(C+I*U)` distinct records and issues
`(E+1)*I*(M-U)` duplicate calls. Duplicate entries within one list and
cross-list Space occurrences count independently. There is no per-Space
control, owner, deduplication, latent method, name, or identity check.

Calc6 is therefore not a dense counterpart of Calc4/5. An uncontrolled
Zone and an unreferenced or orphan Space can receive Calc4/5 scalar fields
but retain earlier sequence contents. The leaf comment at line 2848 says
it is called for all daily/final arrays for Zones and Spaces; literal
parent topology is only controlled Zones and their stored membership
occurrences. The daily sweep trusts global day/Zone coordinates, and the
final sweep trusts controlled Zone numbers, rather than deriving bounds
from destination container sizes.

### Exact 14-sequence projection

For every one-based `TimeStepIndex` from 1 through `T`, the leaf performs
these unconditional assignments in exact source order:

| # | Destination and const source member | Container | Unit/meaning | Calc7 access |
|---:|---|---|---|---|
| 1 | `HeatFlowSeq(t) <- HeatFlowSeq(t)` | `Array1D<Real64>` | heating mass flow, kg/s | final/daily write-capable |
| 2 | `HeatLoadSeq(t) <- HeatLoadSeq(t)` | `Array1D<Real64>` | heating load, W | final/daily write-capable |
| 3 | `CoolFlowSeq(t) <- CoolFlowSeq(t)` | `Array1D<Real64>` | cooling mass flow, kg/s | final/daily write-capable |
| 4 | `CoolLoadSeq(t) <- CoolLoadSeq(t)` | `Array1D<Real64>` | cooling load, W | final/daily write-capable |
| 5 | `HeatZoneTempSeq(t) <- HeatZoneTempSeq(t)` | `EPVector<Real64>` | heating Zone temperature, C | daily read-only |
| 6 | `HeatOutTempSeq(t) <- HeatOutTempSeq(t)` | `Array1D<Real64>` | heating outdoor temperature, C | daily read-only |
| 7 | `HeatZoneRetTempSeq(t) <- HeatZoneRetTempSeq(t)` | `Array1D<Real64>` | heating return temperature, C | none |
| 8 | `HeatZoneHumRatSeq(t) <- HeatZoneHumRatSeq(t)` | `Array1D<Real64>` | heating Zone humidity ratio, kg/kg | daily read-only |
| 9 | `HeatOutHumRatSeq(t) <- HeatOutHumRatSeq(t)` | `Array1D<Real64>` | heating outdoor humidity ratio, kg/kg | daily read-only |
| 10 | `CoolZoneTempSeq(t) <- CoolZoneTempSeq(t)` | `EPVector<Real64>` | cooling Zone temperature, C | daily read-only |
| 11 | `CoolOutTempSeq(t) <- CoolOutTempSeq(t)` | `Array1D<Real64>` | cooling outdoor temperature, C | daily read-only |
| 12 | `CoolZoneRetTempSeq(t) <- CoolZoneRetTempSeq(t)` | `Array1D<Real64>` | cooling return temperature, C | none |
| 13 | `CoolZoneHumRatSeq(t) <- CoolZoneHumRatSeq(t)` | `Array1D<Real64>` | cooling Zone humidity ratio, kg/kg | daily read-only |
| 14 | `CoolOutHumRatSeq(t) <- CoolOutHumRatSeq(t)` | `Array1D<Real64>` | cooling outdoor humidity ratio, kg/kg | daily read-only |

The body has 14 static assignment sites, 14 unique destination names, 14
unique right-hand-side names, and no cross-field fan-out. Each timestep
performs 14 reads plus 14 writes. All values are `Real64`; there is no
predicate, arithmetic, unit conversion, clamp, finite/range check,
allocation, state access, child call, status, or diagnostic.

`ZoneSizingData::allocateMemberArrays` at `DataSizing.cc` lines 280-318
normally dimensions all 36 sequence families to
`NumOfTimeStepInDay = TimeStepsInHour*24`. Calc6 copies only 14. Its 22
omissions are:

- `HeatFlowSeqNoOA` and `CoolFlowSeqNoOA`;
- `DesHeatSetPtSeq`, `DesCoolSetPtSeq`, `HeatTstatTempSeq`, and
  `CoolTstatTempSeq`;
- the eight `DOAS*Seq` families;
- `HeatLoadNoDOASSeq` and `CoolLoadNoDOASSeq`;
- `LatentHeatLoadSeq` and `LatentCoolLoadSeq`;
- `HeatLatentLoadNoDOASSeq` and `CoolLatentLoadNoDOASSeq`;
- `LatentHeatFlowSeq` and `LatentCoolFlowSeq`.

### Calc3 carry-forward and Calc7 overwrite boundary

Calc4 and Calc5 copy no sequences and have zero member-name intersection
with Calc6. CP255 Calc3 intersects at exactly two names:
`HeatFlowSeq` and `CoolFlowSeq`.

When a CP255 cooling or heating latent branch wins, it assigns the
corresponding `Latent*FlowSeq` into the ordinary-name flow sequence of the
calculated-final record. When the selected design-day number is positive,
it also assigns that latent flow sequence into the selected calculated
daily record. CP255 deliberately does not substitute either latent load
sequence. Calc6 then copies:

- the possibly selected latent-origin ordinary flow sequence;
- the still-ordinary `HeatLoadSeq` or `CoolLoadSeq`;
- none of the original latent flow or load sequence families.

The resulting user daily/final record can therefore combine selected
latent-flow history with ordinary sensible-load history. Only the selected
daily day can carry the CP255 mutation; other daily records remain their
ordinary calculated sources.

Calc7 starts after both Calc6 sweeps. Static source analysis finds 12 of
the 14 names:

- the four flow/load sequences are write-capable in final and selected
  daily records through sizing-factor multiplication, with later flow
  minimum/OA clamps;
- eight daily `ZoneTemp`, `OutTemp`, `ZoneHumRat`, and `OutHumRat`
  sequences are read in the cooling/heating zero-load fallback paths;
- `HeatZoneRetTempSeq` and `CoolZoneRetTempSeq` are untouched.

The environmental sequence copies in final records are not those fallback
readers; Calc7 reads the selected daily record. A post-Calc7 value or
derived scalar is therefore composite evidence. An identity oracle must
call Calc6 directly or observe its destination before Calc7.

### Output and retry ordering

The CP254 ZSZ/SPSZ writer closes before CP255, Calc4, Calc5, and Calc6.
Current-attempt Calc6 writes only user arrays and cannot retroactively
change those artifact bytes.

A retry has a separate source-history trap. CP255 mutates calculated
ordinary flow sequences in place. If a later Calc6 element fails, that
selected mutation remains. On a nonpulse whole-parent retry, CP254 executes
before CP255 and can observe the retained selected calculated sequence, so
a rebuilt ZSZ/SPSZ artifact can differ even though Calc6 itself never writes
an output stream.

Calc6 has no `EnergyPlusData` or EMS call. None of its 14 sequences is one
of the eight registered Final-Zone scalar internal variables or one of the
six calculated-final actuators. At retry entry EMS can observe a completed
Calc5 scalar prefix, but it cannot observe a partial Calc6 sequence prefix
through those registrations.

A normal retry can then repeat any gated CP252 work, CP253, CP254, any
gated CP255 work, Calc4, and Calc5 before reaching Calc6. Scalar EMS/CP252
changes can indirectly change CP255 mode selection and the two calculated
flow-sequence sources. Pulse skips CP252-255 but still reruns EMS, Calc4,
and Calc5 before copying the retained sequence sources.

### Loop bound, extent, and identity assumptions

The leaf trusts the caller's integer `T` independently of every source and
destination sequence extent:

- `T <= 0` executes no assignment and reports success;
- `0 < T < extent` copies only indexes `1..T` and leaves destination tails
  untouched;
- `T` beyond any accessed extent eventually fails or enters undefined
  behavior;
- unequal source/destination extents are neither compared nor diagnosed.

Twelve families use Objexx `Array1D::operator()`. Asserted builds require
the one-based index to be contained; violation assert-terminates. Release
access is unchecked and invalid access has undefined behavior.

The two Zone-temperature families use `EPVector::operator()`, which maps
the one-based index through `operator[]`. Debug `operator[]` calls
`std::vector::at`, so an out-of-range source or destination can throw.
Release uses unchecked vector subscripting. At a recoverable debug
exception in assignment 5, all earlier timesteps and assignments 1-4 of
the current timestep remain. A failure in assignment 10 additionally
retains assignments 5-9. The current failed assignment is not completed.

The parent independently trusts daily `Array2D` day/Zone coordinates,
stored Space indexes, and final `EPVector` Zone/Space indexes. A malformed
receiver/source coordinate can fail while preparing arguments, before
Calc6 enters. A bad Space occurrence also prevents every later membership,
final sweep, Calc7 call, facility-sizing update, and run-done latch.

There is no local status, catch, diagnostic, transaction, cleanup, or
rollback. Valid `Real64` element reads/writes allocate nothing and do not
throw. Completed daily/final leaves and element prefixes survive any
recoverable later failure.

A stable distinct-source retry restarts at timestep 1, overwrites the
prefix, and becomes value-idempotent after completion. Passing the same
record as mutable destination and const source is legal: every operation is
a same-member, same-index self-copy. Exact alias is a value no-op but still
performs all extent checks, accesses, and work. Stable duplicate Space
membership converges to the same values but repeats the complete loop.

### Operation and storage bounds

For `Q = max(T,0)` and `K = (E+1)*(C+I*M)`, a completing valid parent
entry executes:

- `14QK` `Real64` assignment statements;
- `28QK` sequence-member accesses;
- `Theta((E+1)*(Z+1) + K*(Q+1))` total parent-slice time;
- constant leaf-local and parent-loop storage.

The first time term retains the design-day and full Zone-control scans even
when no leaf copies an element; the second retains leaf-call/membership
overhead at `Q = 0` and element copying otherwise. The routine allocates no
scratch array. Complexity does not depend on sequence value magnitudes,
latent method, or pulse state. It does depend on global Zone scans,
duplicate membership multiplicity, and the caller-supplied loop bound
rather than the actual shortest sequence extent.

### C++ test execution census

There is no direct Calc6 leaf call in the C++ unit tree. The completing
high-level corpus has the same 59 EndZone parent entries audited for
Calc5: 51 normal entries, six additional component-load pulse entries, and
two bare pulse parent calls at `ZoneEquipmentManager.unit.cc` lines 4576
and 4877.

The parent day-count histogram is:

`E=1:12, 2:46, 3:1`.

The independent timestep-loop histogram is:

`Q=144:29, 96:26, 24:2, 1:2`.

Their joint distribution is:

| `E` | `Q` | Parent entries |
|---:|---:|---:|
| 1 | 144 | 7 |
| 1 | 96 | 3 |
| 1 | 1 | 2 |
| 2 | 144 | 21 |
| 2 | 96 | 23 |
| 2 | 24 | 2 |
| 3 | 144 | 1 |

Leaf roles split as follows:

| Role | Normal | Additional pulse | Bare parent | Total |
|---|---:|---:|---:|---:|
| Daily Zone | 135 | 18 | 2 | 155 |
| Daily Space | 42 | 0 | 0 | 42 |
| Final Zone | 72 | 9 | 2 | 83 |
| Final Space | 21 | 0 | 0 | 21 |
| **Total** | **270** | **27** | **4** | **301** |

Thus the corpus executes 197 daily and 104 final leaves, or 238 Zone and
63 Space leaves. It performs:

- 33,336 normal, 2,376 pulse, and four bare-parent timestep-loop
  iterations;
- 35,716 timestep-loop iterations total;
- `14*35,716 = 500,024` dynamic `Real64` assignments;
- `28*35,716 = 1,000,048` source-plus-destination member accesses.

There are 274 distinct test-local target records: 270 normal plus four
bare-parent records. The 27 pulse leaves revisit normal targets, giving
247 records executed once and 27 twice. All 301 invocations are controlled
Zone or controlled-owner Space contexts. The seven Space-enabled contexts
each own three unique Spaces under one controlled Zone, so no duplicate or
cross-listed Space occurrence executes.

Thirty-nine leaves have latent sizing enabled, all normal at
`E=2,Q=144`: 27 `Sensible` and 12 `SensibleAndLatent`. Exact `Latent` has
zero execution; the other 262 leaves are latent-off. Calc6 does not inspect
the flag or method, so every category runs the same loop.

### C++ assertion, Calc7, and reset evidence

The two bare parent tests execute two daily Zone and two final Zone leaves,
for 56 assignments at `Q=1`, but assert no sequence element. Across the
full corpus there is no source-to-destination identity comparison and no
assertion that isolates assignment order.

Exactly four genuine downstream scalar sites cover only two daily
destination families:

- `ZoneEquipmentManager.unit.cc` lines 4588-4589;
- the same file's lines 4885-4886.

Calc7 reads copied `CoolZoneTempSeq` and `HeatZoneTempSeq` from the daily
user record at source lines 3047 and 3196 to derive calculated-final peak
temperatures. These four sites execute after Calc7 and do not prove direct
copy identity. No final-record sequence copy has downstream proof.

Calc7's static field intersection is 12/14. The four flow/load families
are write-capable; the eight Zone/outdoor temperature and humidity
families are daily read-only; both return-temperature families are
untouched. A later flow/load value cannot be attributed solely to Calc6,
and the environmental scalar descendants cover only two of the eight
read-only names.

`ZoneSizingData::zeroMemberData` resets all 14 Calc6 families when its
`DOASSupMassFlowSeq` sentinel is allocated. `RezeroZoneSizingArrays`
dispatches that reset over daily calculated/user and final calculated/user
records, with Space paths when enabled.

The focused `ZoneEquipmentManager_RezeroZoneSizingArrays` test uses 15
days, five Zones, and four timesteps. It has 28 static Calc6-name assertion
sites: 14 user-daily plus 14 calculated-daily. Dynamic execution is
`28*15*5*4 = 8,400` checks. It never calls Calc6, so it proves reset
coverage rather than calculated-to-user copying. Ten final calculated/user
records quick-return because the sentinel arrays are unallocated; no final
or Space sequence assertion executes.

Calc6 has no attributable report assertion. `writeZszSpsz` precedes Calc6
and reads calculated arrays. Component-load table tests seed and consume
`CalcFinal*` sources directly through `OutputReportTabular.cc` lines
16421-16423, 16440-16462, and 16492-16514, bypassing Calc6 destinations.

The suite therefore leaves direct identity and order for all 14 families,
12 families beyond the two temperature descendants, both return-temperature
families, every final copy, nonpositive/short/long loop bounds, malformed
record shapes and identities, exact alias, partial failure, retry, and
duplicate/cross-listed Space topology unisolated.

### Rust, data, and claim boundary

The Rust/data audit covers 721 UTF-8-readable current-worktree files
returned by `rg --files crates data`. Exact and mechanical-snake searches
find no:

- Calc6 canonical key or helper;
- `ZoneSizingData`, daily/final Zone/Space sizing arena, or calculated/user
  pairing;
- any of the 14 exact sequence member names;
- any of their 14 mechanical snake-case projections.

Rust does own adjacent current-timestep `ZoneSysEnergyDemand`, operational
IdealLoads sensible/latent rates and supply temperature/humidity/mass flow,
typed `AutosizeOrNumber` limits, and density, outdoor-air, node, report, and
design-day label state. Those are input, timestep, or report concepts, not
a 14-family calculated-to-user daily/final sequence projection.

The active data census contains 61 `SimulationControl` objects, all with
Zone sizing disabled, and five raw `SizingPeriod:DesignDay` objects. It
contains no active `Sizing:Zone`, `Sizing:Parameters`, authored `Space`, or
`SpaceList`, and no corresponding epJSON keys. SimulationControl and design
days remain ignored partial inputs rather than sizing execution. Sizing and
authored-Space object families remain run-blocked; the sole autosizing
fixture expects `UnsupportedSizing`.

CP258 therefore adds only one canonical required `source_mapped` row and
the matching ordered HVAC project-contract requirement. It adds no Rust
target, state mapping, support declaration, test, capability, output
implementation, comparator, case, manifest evidence, numerical claim,
performance claim, or conformance promotion.

The inventory becomes 32 algorithms and 263 routines, split 58
`state_mapped` plus 205 `source_mapped`, with 140 required. Domain-required
counts are heat-balance 88, HVAC 29, plant 1, and time/schedule 22. The
`ideal_loads_zone_equipment_purchased_air_source_order` parent now owns 29
rows but remains `scaffold` at claim level `none`; HVAC readiness remains
`0/29`.

CP259 next maps
`ZoneEquipmentManager::updateZoneSizingEndZoneSizingCalc7`, declared at
`ZoneEquipmentManager.hh` lines 177-182 and implemented completely at
`ZoneEquipmentManager.cc` lines 2867-3221. The next parent
`UpdateZoneSizing` definition begins at line 3223.
## CP259 `updateZoneSizingEndZoneSizingCalc7` Final Sizing Adjustment

CP259 adds canonical required
`routine.update_zone_sizing_end_zone_sizing_calc7` immediately after Calc6
and before `sim_zone_equipment`. The declaration is
`ZoneEquipmentManager.hh` lines 177-182 and the complete definition is
`ZoneEquipmentManager.cc` lines 2867-3221:

```cpp
void updateZoneSizingEndZoneSizingCalc7(
    EnergyPlusData &state,
    DataSizing::ZoneSizingData &zsFinalSizing,
    DataSizing::ZoneSizingData &zsCalcFinalSizing,
    Array2D<DataSizing::ZoneSizingData> &zsSizing,
    Array2D<DataSizing::ZoneSizingData> &zsCalcSizing,
    int const zoneOrSpaceNum);
```

### Parent order and addressed records

The EndZone parent reaches Calc7 only after the complete CP256 Calc4,
CP257 Calc5, and both CP258 Calc6 sweeps. The Calc7 loop at lines
3511-3531 is after the nonpulse guard closed at line 3455. Normal and pulse
entries therefore use the same order:

1. scan Zone numbers from one through `NumOfZones`;
2. skip an uncontrolled equipment Zone;
3. complete that Zone's final-record Calc7 leaf;
4. when Space sizing is active, complete every stored `spaceIndexes`
   occurrence for that Zone in list order;
5. only after the entire leaf returns, advance to the next Zone.

Let `Z=max(NumOfZones,0)`, `C` be the controlled Zone count, `M` the number
of stored Space-index occurrences below controlled Zones, `U` the unique
valid referenced Space count, and `I` be one when Space sizing is active.
The parent makes

`L = C + I*M`

leaf calls over `C+I*U` distinct final identities and repeats
`I*(M-U)` leaves. The Zone scan contributes `Theta(Z+1)` even when no leaf
runs. There is no per-Space controlled/owner/name/identity check, no
latent or sizing-method filter, and no deduplication. Duplicate or
cross-listed Space identities execute the entire mutating body repeatedly
without an intervening Calc4-6 baseline copy. Uncontrolled Zones,
unreferenced/orphan Spaces, and dense final-array entries outside this
topology receive Calc4-5 copies and may retain prior Calc6 state, but
receive no Calc7 adjustment.

Unlike Calc6, the parent passes only final records. Each leaf itself scans
the daily arrays for the same raw `zoneOrSpaceNum`. The caller selects
Zone or Space final and daily arenas consistently, but the signature and
body do not verify target kind, corresponding identities, dimensions, or
aliasing.

### Mutable state and static body inventory

The six arguments have these literal roles:

| Input | Source behavior |
|---|---|
| `state` | reads total design/run-design days, global timesteps per day, design-day weather, and diagnostic services |
| `zsFinalSizing` | mutable user-final record and primary branch/denominator state |
| `zsCalcFinalSizing` | calculated-final source, but also mutated by exact-zero day/time defaults and six peak/return fields |
| `zsSizing` | mutable daily user array; all daily rescaling, no-OA snapshots, and OA floors land here |
| `zsCalcSizing` | syntactically mutable calculated-daily array, but the body reads only ten ordinary flow/load scalar or sequence fields |
| `zoneOrSpaceNum` | unchecked raw second-dimension index and zero-load selected-record identity |

Across those records the body touches 76 unique `ZoneSizingData` member
names. Per alias:

| Alias | Unique touched | Unique written | Record LHS sites |
|---|---:|---:|---:|
| user final | 64 | 44 | 61 |
| calculated final | 22 | 10 | 10 |
| daily user aliases combined | 44 | 20 | 41 |
| calculated daily aliases | 10 | 0 | 0 |
| **union/body total** | **76** | **44** | **112** |

The 44 user-final write-capable names cover four `NonAirSys` fields;
ordinary volume, mass, load, and flow/load/no-OA sequence state; coil inlet
state; zero-load day/time/day-name and user/calculated peak state; two
derived volume limits; and two conditional supply temperatures. Daily
writes cover ordinary volume/mass/load, flow/load sequences, no-OA
flow/scalars, and coil inlet state. Calculated-final writes are exactly the
two day numbers, two peak timesteps, and Zone temperature/humidity/return
temperature for cooling and heating.

The body has 18 explicit `for` sites and 34 `if` sites. A
comment/string-stripped lexical census has 55 comparisons, five `&&`
operators, 181 assignment/initializer tokens including four `*=`, and
four `abs`, four `min`, 12 `max`, three `min_element`, and one
`max_element` call. These are source-shape counts rather than semantic
operation totals inside helpers or array assignment.

### Cooling factor and positive-flow projection

Cooling is fully ordered before any heating mutation. Lines 2875-2877
first execute, without a guard:

```text
NonAirSysDesCoolLoad    *= CoolSizingFactor
NonAirSysDesCoolVolFlow *= CoolSizingFactor
```

The local multiplier `Mc` is then:

```text
if InpDesCoolAirFlow > 0
   and CoolAirDesMethod == InpDesAirFlow
   and current DesCoolVolFlow > 0:
    Mc = (InpDesCoolAirFlow / current DesCoolVolFlow)
         * CoolSizingFactor
else:
    Mc = CoolSizingFactor
```

The source comment says the input design flow is used, but the executable
formula still multiplies the input-derived ratio by the sizing factor.
Only `abs(Mc-1) > 1e-5` enters the rescaling branch. Equality at the
tolerance, unordered comparisons from NaN, and other false comparisons
take the no-rescale branch.

When the current user-final cooling volume is strictly positive, Calc7
rebuilds user-final volume, mass, and load from calculated-final values
times `Mc`. A zero-based loop bounded only by destination
`CoolFlowSeq.size()` then rebuilds destination flow and load sequences
while independently indexing calculated flow/load sequences. It computes

```text
OAFrac = clamp(MinOA / newly rebuilt DesCoolVolFlow, 0, 1)
coil inlet = OAFrac * design-day weather at the stored peak
             + (1-OAFrac) * user-final Zone peak state
```

The positivity guard applies to the old user value. The new calculated
flow times `Mc` is not checked before division, so zero, NaN, or infinity
can reach the OA fraction and coil inlet path. Design day and peak timestep
are also used without positive/range validation.

A nonpositive old final flow writes only `InpDesCoolAirFlow` and its
density-derived mass. It does not rebuild load, sequences, or coil state.

Every design/run-design day then takes the same positive-current-flow or
input-flow fallback against that daily user record. Positive daily flow
rebuilds volume/mass/load and the two sequences from the corresponding
calculated daily record, and recomputes coil inlet from that day's weather.
Regardless of the daily subbranch, it snapshots:

- whole `CoolFlowSeq` into `CoolFlowSeqNoOA`;
- `DesCoolVolFlow` into `DesCoolVolFlowNoOA`;
- `DesCoolMassFlow` into `DesCoolMassFlowNoOA`.

When `abs(Mc-1) <= 1e-5` or the comparison is false, Calc7 skips all
rescaling/fallback work but still performs those three no-OA snapshots for
every day. Thus an input-flow fallback can be bypassed when the global
multiplier lies within tolerance.

### Cooling no-OA and including-OA floors

The final no-OA phase initializes a zero minimum. Only exact
`DesAirFlowWithLim` changes it to

`max(DesCoolMinAirFlow, DesCoolMinAirFlow2)`.

It snapshots current final volume and mass, raises those no-OA scalars when
the minimum is strictly greater, copies each ordinary final cooling-flow
timestep into `CoolFlowSeqNoOA`, and raises only the no-OA flow element
when the density-derived minimum mass is greater. Load and coil state are
not adjusted.

Daily behavior is source-asymmetric. Each day resnapshots the no-OA
scalars, then lines 2969-2970 literally use

```text
max(DesCoolMinAirFlow, DesCoolMinAirFlow)
```

with no method guard. The first minimum is duplicated and
`DesCoolMinAirFlow2` is ignored. The subsequent timestep loop changes a
no-OA element only when that mass floor exceeds the ordinary
`CoolFlowSeq`; otherwise the earlier whole-array snapshot remains.

The including-OA final phase uses:

```text
if method == DesAirFlowWithLim:
    min volume = max(min1, min2, MinOA)
else:
    min volume = MinOA
```

It raises ordinary final volume/mass and each `CoolFlowSeq` element, again
without changing load, load sequence, or coil inlet state. Every daily
record instead evaluates `max(min1, min1, MinOA)` without any method guard,
repeating the same omitted-second-minimum source behavior.

### Cooling exact-zero-load fallback

The comment at line 3017 says cooling flow is zero, but the executable
gate is exact `zsFinalSizing.DesCoolLoad == 0`. A MinOA or authored flow
floor can therefore coexist with entry into this branch.

Only exact-zero calculated-final `CoolDDNum` or
`TimeStepNumAtCoolMax` values are changed to one. Negative and oversized
values pass through. Calc7 copies calculated timestep, day number, and day
name to user-final, then indexes that daily user record.

An empty `DesCoolSetPtSeq` emits the routine-specific Severe diagnostic
and the shared noreturn Fatal. Otherwise the user-final Zone peak
temperature becomes the whole setpoint-sequence minimum. Outdoor
temperature becomes the whole `CoolOutTempSeq` minimum with no empty
guard, rather than the value at the selected peak. Outdoor humidity is
the selected `CoolOutHumRatSeq` element, and user-final Zone humidity is
the daily scalar `CoolDesHumRat`.

Calculated-final Zone temperature and humidity instead come from
`CoolZoneTempSeq` and `CoolZoneHumRatSeq` at the selected timestep, and
its return temperature becomes that calculated Zone temperature.
User-final coil inlet and return state are set from the user-final Zone
peak state. The user and calculated records therefore intentionally retain
different temperature/humidity sources.

### Heating factor, cap, and projection

Heating begins only after the complete cooling path returns. It
unconditionally executes:

```text
NonAirSysDesHeatLoad    *= HeatSizingFactor
NonAirSysDesHeatVolFlow *= HeatSizingFactor
```

The local multiplier `Mh` follows three branches:

```text
if InpDesHeatAirFlow > 0
   and HeatAirDesMethod == InpDesAirFlow
   and current DesHeatVolFlow > 0:
    Mh = (InpDesHeatAirFlow / current DesHeatVolFlow)
         * HeatSizingFactor
else if HeatAirDesMethod == DesAirFlowWithLim
        and current DesHeatVolFlow > 0:
    Hmax = max(DesHeatMaxAirFlow,
               DesHeatMaxAirFlow2,
               already adjusted DesCoolVolFlow
                 * DesHeatMaxAirFlowFrac)
    if Hmax < current DesHeatVolFlow:
        Mh = (Hmax / current DesHeatVolFlow) * HeatSizingFactor
    else:
        Mh = HeatSizingFactor
else:
    Mh = HeatSizingFactor
```

Thus cooling OA floors and other cooling changes precede and can affect
the heating limit formula. The explicit-flow formula and limited-flow
ratio both retain the sizing factor, despite the source comment that an
input flow overrides it. A cap equal to or above the current heating flow
does not form a ratio.

Only `abs(Mh-1) > 1e-5` enters heating rescaling. Positive final and daily
heating flows rebuild volume, mass, load, flow sequence, and load sequence
from calculated state times `Mh`, then recompute OA-mixed coil inlet state.
The old flow is the positivity guard, while the denominator for OA
fraction is the unchecked newly rebuilt flow. Nonpositive old flows
receive input volume plus density-derived mass only.

Every daily record snapshots whole `HeatFlowSeq` and its current
volume/mass into no-OA state after the scale-or-fallback branch. The
no-rescale alternative still makes those daily snapshots.

Heating then snapshots final current volume/mass and copies each final
`HeatFlowSeq` element into `HeatFlowSeqNoOA`. Unlike cooling, this no-OA
phase applies neither `DesHeatMaxAirFlow` nor
`DesHeatMaxAirFlow2`; the maximums affect only the multiplier branch and
the final derived field.

The following phase floors ordinary final heating volume/mass and every
final heating-flow element against density-derived `MinOA`. It repeats the
same `MinOA`-only floor for every daily record. Loads, load sequences, and
coil inlet state again retain their pre-floor values.

### Heating exact-zero-load fallback and derived tail

The heating comment describes zero flow, but the executable gate is exact
`zsFinalSizing.DesHeatLoad == 0`. Exact-zero calculated-final `HeatDDNum`
and `TimeStepNumAtHeatMax` become one; negative and oversized values are
not corrected. User-final then receives the calculated day/timestep/day
name and selects that daily user record.

An empty `DesHeatSetPtSeq` emits Severe then noreturn Fatal. Otherwise
user-final Zone peak temperature becomes the whole setpoint-sequence
maximum. The source still takes the whole `HeatOutTempSeq` minimum, not a
maximum or selected-peak outdoor temperature, and does so without an empty
guard. Outdoor humidity uses the selected timestep, while user-final Zone
humidity uses daily scalar `HeatDesHumRat`.

Calculated-final Zone temperature/humidity come from the daily heating
Zone sequences at the selected timestep and its return temperature becomes
that calculated Zone temperature. User-final coil inlet and return fields
use the separate user-final Zone peak state.

After both modes complete, the tail unconditionally derives:

```text
DesCoolVolFlowMin =
    max(DesCoolMinAirFlow,
        DesCoolMinAirFlow2,
        DesCoolVolFlow * DesCoolMinAirFlowFrac)

DesHeatVolFlowMax =
    max(DesHeatMaxAirFlow,
        DesHeatMaxAirFlow2,
        max(DesCoolVolFlow, DesHeatVolFlow)
          * DesHeatMaxAirFlowFrac)
```

The source comment immediately above the cooling result says its own
minimum-flow description appears incorrect. These final derived values are
not projected to daily records.

When `ZnCoolDgnSAMethod` equals the unscoped integer constant
`TemperatureDifference` (`2`), Calc7 overwrites `CoolDesTemp` with
`ZoneTempAtCoolPeak - abs(CoolDesTempDiff)`. The analogous heating method
writes `ZoneTempAtHeatPeak + abs(HeatDesTempDiff)`. Other integers,
including invalid values, leave existing temperatures unchanged.

### Extents, indexing, and complexity

Let `D=max(TotDesDays+TotRunDesPersDays,0)` for ordinary valid global
state and `T=max(NumOfTimeStepInDay,0)`. Each leaf necessarily chooses one
cooling scale-or-snapshot day sweep and one heating
scale-or-snapshot day sweep, then performs three more full day sweeps:
cooling no-OA minima, cooling including-OA minima, and heating including-OA
floors. It therefore visits `5D` daily records before any optional
zero-load selected-day access.

Four final loops and three loops per daily record use one-based `1..T`
Objexx indexing. Their explicit baseline is:

`T * (4 + 3D)`

iterations per leaf. Conditional cooling/heating scaling instead uses
zero-based `[]`, bounded solely by the destination `FlowSeq.size()`, while
also indexing destination `LoadSeq` and both calculated peers. Whole
daily no-OA sequence assignments copy according to Objexx array assignment
semantics rather than global `T`.

For normal conformable equal-length records, time is
`Theta((D+1)*(T+1))` per leaf and
`Theta((Z+1)+L*(D+1)*(T+1))` at the parent. Exact work depends on flow
signs, multiplier tolerance, sequence sizes, zero-load branches, and
setpoint/outdoor reduction lengths. Scratch storage is scalar only, but
whole-array copies can allocate or redimension an unallocated destination
and can fail.

`T <= 0` is not a leaf no-op. It skips only the seven explicit
one-based loop sites; four NonAir multiplications, scalar projection and
floors, zero-based size-driven scaling, whole-array daily snapshots,
zero-load sequence reductions, and tail derivation still run. `D <= 0`
skips the five daily sweeps, but exact-zero load can default day number to
one and immediately index an absent daily/weather record.

The routine independently assumes:

- final and daily flow/load sequence extents match their calculated peers;
- no-OA sequences accept whole-array copies and at least `T` one-based
  elements;
- daily Array2D first and second coordinates exist;
- final and daily design day/timestep identities address weather and
  selected sequences;
- setpoint and outdoor temperature sequences are nonempty;
- outdoor/Zone humidity and Zone temperature sequences contain the
  selected peak timestep.

Only setpoint emptiness receives an explicit check. An empty outdoor
temperature sequence reaches a dereferenced `min_element` result.
Objexx asserted indexing can terminate; unchecked release access can be
undefined behavior; the two daily Zone-temperature `EPVector` accesses
can throw in debug. Allocation or string assignment can also throw.

### Failure, alias, and replay semantics

There is no local status, catch, transaction, cleanup, or rollback.
Cooling is ordered first. A cooling failure preserves its NonAir
multiplication, any scalar/sequence prefixes, completed days, no-OA/OA
floors, and calculated-final defaults, then blocks all heating and the
tail. A heating failure preserves the entire cooling result plus the
heating prefix. Earlier Zone/Space leaves in the parent remain committed.
Failure prevents later leaves, `UpdateZoneSizing` completion, and the
later `UpdateFacilitySizing`/`ZoneSizingRunDone` actions at
`SizingManager.cc` lines 391-393.

The two guarded setpoint failures emit one Severe and then one noreturn
Fatal each. Other invalid record/weather/sequence coordinates fail without
Calc7 diagnostics. A weather failure in a positive-flow branch occurs
after volume, mass, load, and sequence writes. A zero-load failure can
occur after changing calculated-final day/time identities and copying them
to user-final.

Direct valid replay is not generally value-idempotent:

- four `NonAirSys` fields multiply in place on every entry;
- an explicit-input multiplier divides by the current mutable user flow;
- daily user arrays may already contain prior scaling/floors;
- zero-load fallback mutates calculated-final day/time and peak state;
- strict comparisons and tolerance gates can choose a different branch
  after earlier mutation.

Same-parent duplicate Space membership repeats the leaf with no baseline
restore and can compound. Stable `zsFinalSizing` /
`zsCalcFinalSizing` alias makes positive projection in-place. The
zero-load branch first writes user peak state and then calculated peak
state; exact alias lets the latter overwrite the former before coil and
return writes. Aliased daily user/calculated arrays likewise turn daily
source projection into in-place scaling.

A whole-parent replay is different from direct leaf replay. It reruns EMS;
on a normal entry it also reruns CP252-255 and the writer; every entry then
reruns Calc4-6 before Calc7. Calc4/5 restore only their listed user fields,
and Calc6 restores only its 14 sequences. Calculated-final day/time/peak
mutations from an earlier zero-load Calc7 and calculated state retained
from Calc3 survive unless an earlier stage happens to replace them.
Consequently a valid whole replay is not guaranteed to reproduce the first
entry.

Calc7 owns no EMS call. The parent EMS calling point is earlier than
Calc4/5. Eight Final-Zone scalars are registered as EMS internal
variables; Calc7 can later mutate the six flow/load members but not the two
densities. The six calculated-final actuators are applied before the
nonpulse work, and Calc7's calculated-final writes affect different
day/time/peak fields. No analogous Space EMS registration is established
by this routine.

On a normal nonpulse entry, CP254 has already closed current-attempt
ZSZ/SPSZ streams before Calc7, so the current leaf cannot retroactively
change those bytes. On a later nonpulse whole replay, CP254 reads
calculated-final cooling/heating design
day identities at `writeZszSpsz` lines 2513 and 2521 and can observe
Calc7's retained zero defaults; it can also observe retained Calc3
sequence selections. Pulse replay skips CP252-255 and the writer but still
repeats EMS and Calc4-7.

### C++ execution census

No C++ test calls the Calc7 leaf directly. The completing high-level corpus
has the same 59 EndZone parent entries audited for Calc5-6: 51 normal
entries, six additional component-load pulse entries, and two bare pulse
parent calls at `ZoneEquipmentManager.unit.cc` lines 4576 and 4877.

Calc7 leaf roles are:

| Role | Normal | Additional pulse | Bare parent | Total |
|---|---:|---:|---:|---:|
| final Zone | 72 | 9 | 2 | 83 |
| final Space | 21 | 0 | 0 | 21 |
| **Total** | **93** | **9** | **2** | **104** |

There are 95 distinct final targets. Eighty-six execute once; nine normal
Zone targets are revisited by pulse, giving `86 + 9*2 = 104`. The seven
Space-enabled contexts each own three unique Spaces under one controlled
Zone, so the completing corpus has no duplicate or cross-listed Space
execution.

The parent-entry timestep distribution weighted by final leaves is:

| `T` | Calc7 leaves | Final-leaf timestep units |
|---:|---:|---:|
| 144 | 55 | 7,920 |
| 96 | 45 | 4,320 |
| 24 | 2 | 48 |
| 1 | 2 | 2 |
| **Total** | **104** | **12,290** |

The same leaves make 197 aggregate daily-record visits. Weighted by their
parent timestep count, those daily visits contribute 23,426
daily-record/timestep units. The four unconditional final timestep loops
plus three unconditional daily timestep loops therefore execute

`4*12,290 + 3*23,426 = 119,438`

iterations. The five unconditional day sweeps make `5*197 = 985` record
visits. These totals exclude conditional sequence-rescaling loops,
whole-array no-OA copies, scalar work, and zero-load reductions. Under the
audited conformable extents, the optional positive-flow rescaling loops
can add at most 7,392 iterations across both modes.

Thirteen final targets have latent sizing enabled: nine `Sensible` and
four `SensibleAndLatent`; exact `Latent` is absent. The other 91 are
latent-off. Calc7 reads none of those flags or methods.

### C++ branch and assertion evidence

Production inputs choose cooling `FromDDCalc` for 95 leaves and
`DesAirFlowWithLim` for seven; the two bare records retain default/invalid
method state. Heating uses `FromDDCalc` for all 102 production leaves and
default/invalid state for the two bare records. No production or bare
record selects `InpDesAirFlow`, so both explicit user-flow multiplier
formulas have zero coverage. Neither final supply-air
`TemperatureDifference` branch executes.

Eleven normal Zone leaves use nonunit cooling and heating sizing factors
and necessarily enter both outer multiplier gates: two AirLoop DOAS leaves
at 1.2, three FourPipeBeam leaves at 1.33, two OA-preheat leaves at 1.3,
one PIU leaf at cooling 1.15/heating 1.25, two WSHP leaves at 1.2, and one
UnitHeater leaf at 1.5. The other 93 leaves use unit factors. Exact
positive/nonpositive flow subbranch counts across all production
simulations are not statically recoverable from input alone.

The two bare parent tests provably enter both exact-zero-load fallbacks,
have initialized setpoint/outdoor/Zone sequences, and avoid both fatal
paths. Six post-call assertions cover four peak-state identities:

- calculated-final `ZoneTempAtHeatPeak` and `ZoneTempAtCoolPeak` at lines
  4588-4589 and 4885-4886;
- user-final `ZoneTempAtHeatPeak` and `ZoneTempAtCoolPeak` at lines
  4592-4593.

They directly prove the calculated 23/23 versus user 22/24 split in one
case and calculated 23.9/23.9 state in the other. They do not isolate the
outdoor minimum, humidity, return, coil, day/time default, or diagnostic
rules.

The BaseClassSizing completion test reads calculated/user heating peak
temperature, final zero heating volume/mass, and final positive load after
Calc7. Two WindowAC completion tests read final cooling volume. These are
useful descendants but combine Calc5, Calc6, Calc7, and later sizing
consumers rather than serving as Calc7 formula or ordering oracles.

A bounded 300 post-Calc7 report assertions comprise 290 heating/cooling
sizing cells across 29 SizingManager roles plus ten WindowAC zero-heating
cells. Relevant user branches and gates consume final design volume,
load, and flow values, while calculated cells consume calculated-final
state. They witness retained composite state but do not isolate
multiplier, daily floor, sequence, OA mixing, or zero-load projection.
CP254 ZSZ/SPSZ assertions cannot witness same-attempt nonpulse Calc7
because that writer precedes the leaf; pulse entries skip it.
Component-load tables seed and read
calculated-final rather than Calc7 user destinations.

Calc7 writes 44 distinct user-final field names. `zeroMemberData` clears
34 and retains ten: the four `NonAirSys` fields, cooling/heating no-OA
volume and mass scalars, and `CoolDesTemp`/`HeatDesTemp`. All ten
calculated-final peak/index destinations are reset when member arrays are
allocated. Of the 20 daily destination names, 16 are cleared and the four
no-OA volume/mass scalars are retained.

`RezeroZoneSizingArrays` calls that reset for controlled Zone and
controlled-owner Space daily/final records, but `zeroMemberData`
quick-returns when `DOASSupMassFlowSeq` is unallocated. The sole focused
Rezero test allocates, seeds, and asserts daily arrays; final member arrays
remain unallocated, so final calls quick-return. It is reset evidence, not
a direct final/Space Calc7 oracle.

There is no direct test for:

- either explicit-input multiplier or heating limited-flow multiplier;
- the 1e-5 tolerance edge, nonpositive flow fallback, or post-overwrite
  zero denominator;
- daily cooling's duplicated first minimum and missing method guard;
- no-OA versus including-OA scalar/sequence identity;
- load/coil inconsistency after flow floors;
- either empty-setpoint fatal, unguarded empty outdoor sequence, or invalid
  day/timestep;
- supply-temperature-difference tails;
- any Space destination field, duplicate membership, alias, partial
  failure, or replay.

### Rust, data, and claim boundary

The Rust/data audit covers all 721 current-worktree files returned by
`rg --files crates data`; strict UTF-8 decoding fails for zero files.
Exact searches find no camel/snake Calc7 helper or routine key and no
Zone/Space daily/final sizing arena.

All 76 exact C++ member tokens have zero matches. Mechanical snake-case
tokens are also absent except generic `zone_name`: the exact token occurs
321 times in 62 files, while the raw substring occurs 350 times in 66
files because four more files contain plural or longer identifiers.
Those uses are ordinary typed input/model/report Zone identities, not
Calc7 sizing state. Raw `MinOA` appears five times in four files only as a
substring of `CalcPurchAirMinOAMassFlow`; exact token `MinOA` is absent.

A separate exact-token census of 44 sizing arena, lifecycle, method, enum,
and local-alias identifiers finds zero matches. This includes
`ZoneSizingData`, all Zone/Space calculated/user daily/final arena names,
`ZoneSizingCalc`, `ZoneSizingRunDone`, `AirflowSizingMethod`,
`FromDDCalc`, `InpDesAirFlow`, `DesAirFlowWithLim`,
`SupplyAirTemperature`, and `TemperatureDifference`. Longer operational
IdealLoads field names containing supply-air or temperature-difference
phrases are not counterparts.

Comment-stripped active IDF data contain 61 `SimulationControl` objects in
61 files, and field one disables Zone sizing in all 61. Five
`SizingPeriod:DesignDay` objects occur in four files. Active IDF objects
and all 12 epJSON documents contain zero `Sizing:Zone`,
`Sizing:Parameters`, authored `Space`, or `SpaceList` objects/keys.

The sole raw `Sizing:Zone` in `crates data` is the
`AUTOSIZING_EPJSON` arbitrary-run test fixture. Its test requires
`UnsupportedSizing` and the message that sizing workflows are not ported.
Capabilities keep `Sizing:*`/`ZoneSizing*` and Space partitioning
run-blocked; SimulationControl and design days remain inactive/unused
partial inputs. No execution or support claim follows.

CP259 therefore adds only one canonical required `source_mapped` row and
the matching ordered HVAC project-contract requirement. It adds no Rust
target, state mapping, support declaration, test, capability, output
implementation, comparator, case, manifest evidence, numerical claim,
performance claim, or conformance promotion.

The inventory becomes 32 algorithms and 264 routines, split 58
`state_mapped` plus 206 `source_mapped`, with 141 required. Domain-required
counts are heat-balance 88, HVAC 30, plant 1, and time/schedule 22. The
`ideal_loads_zone_equipment_purchased_air_source_order` parent now owns 30
rows but remains `scaffold` at claim level `none`; HVAC readiness remains
`0/30`.

CP260 next maps the complete
`ZoneEquipmentManager::UpdateZoneSizing` parent, declared at
`ZoneEquipmentManager.hh` line 130 and implemented at
`ZoneEquipmentManager.cc` lines 3223-3536. The following
`SimZoneEquipment` definition begins at line 3538 and is declared at
header line 184.

## CP260 `UpdateZoneSizing` Four-Phase Sizing Dispatcher

CP260 adds canonical required `routine.update_zone_sizing` immediately after
Calc7 and before the existing `routine.sim_zone_equipment` row. The pinned
EnergyPlus v26.1.0 source revision is
`6f2e40d10250a105b49966baa24d843711e61048`. The public declaration is
`ZoneEquipmentManager.hh` line 130, and the complete definition is
`ZoneEquipmentManager.cc` lines 3223-3536:

```cpp
void UpdateZoneSizing(
    EnergyPlusData &state,
    Constant::CallIndicator const CallIndicator);
```

The header omits the definition's top-level `const` on the by-value
indicator. That does not change the C++ function type. The physical next
definition is `SimZoneEquipment` at lines 3538-4193, declared at header
line 184; `SetZoneEquipSimOrder` starts at line 4195.

### Enum, stale comments, and silent default

`DataGlobalConstants.hh` lines 539-548 define the authoritative values:

- `Invalid = -1`;
- `BeginDay = 0`;
- `DuringDay = 1`;
- `EndDay = 2`;
- `EndZoneSizingCalc = 3`;
- `EndSysSizingCalc = 4`;
- `Num = 5`.

The legacy body comments instead describe the four handled stages as 1
through 4. They are off by one. The BeginDay comment also says that the
routine zeros result arrays, but the CP248 child only stamps calculated
daily metadata; it does not clear the sequence arrays.

The switch at line 3239 has explicit BeginDay, DuringDay, EndDay, and
EndZoneSizingCalc cases. `Invalid`, `EndSysSizingCalc`, `Num`, and any
arbitrary cast value reach the default at lines 3533-3534 and silently do
nothing. There is no validation, diagnostic, status, or fallback for an
unsupported indicator.

### Production placement and downstream gates

The production call sites establish lifecycle policy outside this parent:

- `SizingManager.cc` line 307 calls BeginDay only after its non-warmup
  sizing-day gate, immediately before Facility BeginDay;
- `HVACManager.cc` line 475 calls DuringDay under the non-warmup
  `ZoneSizingCalc` path once for every accepted system substep, immediately
  before Facility DuringDay;
- `SizingManager.cc` lines 373-375 call EndDay under
  `EndDayFlag && !WarmupFlag`, immediately before Facility EndDay;
- `SizingManager.cc` lines 390-393 call EndZoneSizingCalc only after at
  least one sizing period, then call Facility EndZone and set
  `ZoneSizingRunDone = true`.

A successful pulse EndZone pass can then reach Rezero at
`SizingManager.cc` lines 400-403. Direct callers bypass all of these gates.
An exception or fatal exit from this parent blocks its caller's subsequent
Facility stage and, at EndZone, the run-done latch and pulse Rezero.

### Traversal notation

For the case-level topology, let:

- `Z = max(NumOfZones, 0)`;
- `C` be the number of Zones whose `ZoneEquipConfig.IsControlled` is true;
- `I` be one when `doSpaceHeatBalanceSizing` is true and zero otherwise;
- `M` be the number of stored `spaceIndexes` occurrences under those
  controlled Zones, including duplicates and cross-list occurrences;
- `U` be the number of unique valid Space identities among those
  occurrences;
- `H = C + I*M`.

The parent validates neither the global count nor any equipment, day,
Zone, Space, membership, owner, or allocation index before access.
Therefore the formulas describe normally representable state and call
cardinality, not a safety guarantee.

### BeginDay metadata traversal

BeginDay visits ascending Zone indexes, skips uncontrolled equipment
Zones, calls `updateZoneSizingBeginDay` on the current day's calculated
Zone record, and then, when Space sizing is active, visits that Zone's
stored Space indexes in list order.

This is `H` child calls over `C + I*U` distinct records. A duplicate Space
identity is stamped repeatedly. A cross-listed Space is stamped once in
each referring Zone position, although this child receives no owner
parameter. Replay normally rewrites the same metadata and is mostly
stable, but it neither zeros result arrays nor repairs malformed topology.

### DuringDay accumulation traversal

DuringDay first computes the signed integer index

```text
(HourOfDay - 1) * TimeStepsInHour + TimeStep
```

and snapshots `FracTimeStepZone`. It then repeats the BeginDay membership
topology for exactly `H` calls. The Zone child receives its daily user and
calculated records, current Zone thermostat high/low setpoints, mutable
high/low extrema in `FinalZoneSizing(CtrlZoneNum)`, the computed index, and
the system-to-Zone fraction.

A Space child receives the Space daily user and calculated records, but
still receives the referring Zone's thermostat setpoints and references to
that Zone's final extrema. It never receives `FinalSpaceSizing` extrema.
Consequently duplicate and cross-listed Spaces repeat additive
accumulation, and the same Space identity can be processed with different
thermostat/final-Zone references. The signed arithmetic and every record
index are unchecked. Replay repeats the CP249 `+=` work and double-counts
the accepted substep rather than replacing it.

### EndDay two-pass barrier

EndDay first completes a full controlled-membership traversal of
`updateZoneSizingEndDayMovingAvg`. Only after all `H` smoothing calls
return does it begin a second complete `H` traversal of
`updateZoneSizingEndDay`.

Each reducer receives the current calculated daily record, its
calculated-final Zone or Space record, `NumOfTimeStepInDay`, the current
day's `DesDayWeath`, and `StdRhoAir`. The parent therefore dispatches
exactly `2H` leaves while preserving a global phase barrier: a smoothing
failure suppresses every reducer, while a reducer failure occurs after
all records have already been smoothed.

Replay smooths already smoothed arrays before selecting peaks again, so
the complete EndDay case is not generally idempotent.

### EndZone ordered barriers

EndZoneSizingCalc has four ordered regions.

First, it unconditionally calls `EMSManager::ManageEMS` with the ZoneSizing
calling point and an empty `Optional_int_const`. `ManageEMS` initializes
the caller-owned `anyEMSRan`, including on a quick return, but this parent
never reads that value. The parent independently re-reads
`AnyEnergyManagementSystemInModel` and later `isPulseZoneSizing` after the
callback.

Second, when the global AnyEMS flag is true, it scans all `Z` calculated-
final Zone records, including uncontrolled Zones. It has no corresponding
Space pass. Six independent flag/current-target pairs are applied in this
exact order:

1. heating design mass flow;
2. cooling design mass flow;
3. heating design load;
4. cooling design load;
5. heating design volume flow;
6. cooling design volume flow.

Each assignment requires its override flag and the current destination to
be strictly greater than zero. It then copies the raw EMS value without a
finite, sign, range, or cross-field check. A zero, negative, or NaN result
can make the current-target gate false on retry. The values are not
necessarily final: an eligible noncoincident multi-Space Calc1 call later
resets and rebuilds the same six Zone fields from Space records. Exact
Coincident, exactly-one-Space, disabled Space sizing, uncontrolled-for-
Calc1, and pulse paths preserve the override into later copy stages,
although Calc3 and Calc7 can still transform downstream final state.

Third, only when `isPulseZoneSizing` is false, the parent completes these
barriers in order:

1. under Space sizing, visit controlled Zones and call Calc1 unless
   `Zone.numSpaces == 1`;
2. visit controlled Zones and their stored Space occurrences with Calc2;
3. route, open, write, and close ZSZ;
4. under Space sizing, route, open, write, and close SPSZ;
5. visit controlled Zones whose calculated-final Zone latent flag is true
   with Calc3, then every stored Space occurrence under each passing Zone.

The Calc1 prose says "more than one space", but the implementation skips
only exact one. Zero, negative, stale, or membership-inconsistent
`numSpaces` can still dispatch Calc1. Calc3 tests only the Zone's latent
flag; its Space calls have no Space-local latent, owner, control, or
deduplication gate and share `isAnyLatentLoad` by mutable reference.

Fourth, after the pulse guard closes, both pulse and normal invocations
complete these barriers:

1. Calc4 over every flat Zone daily target, with a complete flat Space
   daily target sweep nested inside every Zone target when Space sizing is
   active;
2. Calc5 over every flat Zone final target, with a complete flat Space
   final target sweep nested inside every Zone target;
3. Calc6 over controlled membership for every design/run-design day;
4. Calc6 again over controlled final membership;
5. Calc7 over controlled final membership.

Calc4 and Calc5 therefore use dense target-size Cartesian topology, not
the controlled membership topology used by the other traversal groups.
They have no day pairing, owner, membership, control, or deduplication
filter. Calc6 and Calc7 switch back to controlled Zone plus stored-Space
membership.

### EndZone cardinality

Let:

- `n` be one for a normal/nonpulse call and zero for a pulse call;
- `N` be the controlled Zone count whose declared `numSpaces` is not
  exactly one;
- `L` be the controlled Zone count whose calculated-final Zone latent flag
  is true;
- `M_L` be stored Space occurrences under those `L` Zones;
- `K = L + I*M_L`;
- `D = max(TotDesDays + TotRunDesPersDays, 0)`;
- `A` and `B` be the flat Zone and Space daily target sizes;
- `F` and `G` be the flat Zone and Space final target sizes.

For normally representable extents, the exact mapped-child counts are:

| Child barrier | Calls |
|---|---:|
| ManageEMS | `1` |
| Calc1 | `n*I*N` |
| Calc2 | `n*H` |
| ZSZ/SPSZ writers | `n*(1+I)` |
| Calc3 | `n*K` |
| Calc4 | `A*(1+I*B)` |
| Calc5 | `F*(1+I*G)` |
| Calc6 plus Calc7 | `(D+2)*H` |

Thus the full mapped-child total, excluding file-open services, is

```text
1 + n*(I*N + H + (1+I) + K)
  + A*(1+I*B) + F*(1+I*G) + (D+2)*H
```

and the operational total including `ensure_open` replaces the single
`(1+I)` writer term inside the `n` group with `2*(1+I)`. A normal call
simplifies to

```text
2 + I + I*N + K
  + A*(1+I*B) + F*(1+I*G) + (D+3)*H
```

while a pulse call is

```text
1 + A*(1+I*B) + F*(1+I*G) + (D+2)*H.
```

Normal allocation at source lines 830-838 gives `A=D*Z`, `B=D*S`,
`F=Z`, and `G=S`, where `S` is the global Space count. The literal Calc4
count is therefore `D*Z + I*D^2*Z*S`, and Calc5 is
`Z + I*Z*S`. The zero-based flat `[]` traversal is real ObjexxFCL/EPVector
linear indexing; the repeated Cartesian Space copies must not be
normalized into a day- or owner-aligned pass during a faithful port.

### File routing and writer-topology boundary

For each normal invocation, comma selects the CSV path, tab selects the
TAB path, and every other separator selects the TXT path. ZSZ routing,
`ensure_open`, complete CP254 write, and close all precede the optional
SPSZ equivalents. Both files precede Calc3 and every Calc4-7 transform.

The CP254 writer has a different topology from this dispatcher. It scans
dense Zone indexes or every global Space, maps each Space to its owner,
and filters with HeatBalance `Zone.IsControlled`, not
`ZoneEquipConfig.IsControlled` and not stored membership. An unreferenced
Space whose owner passes that different flag can be printed, while
disagreement between the two control flags changes which records appear.
False output control can route a newly opened handle to a null stream, but
does not skip the writer's loops or psychrometric calculations; an
already-good handle is reused.

The current-attempt bytes therefore reflect state after EMS, optional
Calc1, and Calc2, but before Calc3 latent selection, dense Calc4/5 copy,
Calc6 sequence projection, and Calc7 final adjustment. Later work cannot
retroactively modify those bytes.

### Static parent inventory

The direct body contains:

- 25 loops, comprising 16 classic and nine range loops;
- 43 `if` statements;
- four explicit cases plus default;
- 12 `continue` and five `break` statements;
- 26 mapped-child call sites;
- two additional `ensure_open` service sites.

Per handled case, the loop/if/mapped-call-site counts are BeginDay
`2/2/2`, DuringDay `2/2/2`, EndDay `4/4/4`, and EndZone
`17/35/18`. Counting file opens gives 28 operational child/service sites.
All call-form expressions total 90: those 28 operational sites, 57
array/object accessors, four `.size()` calls, and the empty Optional
constructor.

Comment-stripped direct state/file access has 159 occurrences over 38
unique first-leaf chains. The root counts are `dataEnvrn=4`,
`dataGlobal=18`, `dataHeatBal=25`, `dataHeatBalFanSys=1`,
`dataHVACGlobal=1`, `dataSize=76`, `dataZoneEquip=10`,
`dataZoneEquipmentManager=6`, and `files=18`.

There are 12 inline persistent assignment statements over eight unique
targets: six calculated-final EMS destinations, three mutually exclusive
assignments to `zsz.filePath`, and three to `spsz.filePath`. The local
`forSpaces` false-to-true transition is not persistent model state. Every
other mutation belongs to a child or stream operation.

### Failure, output, and replay semantics

The parent owns no local validation, status return, catch, checkpoint,
transaction, rollback, or cleanup guard. An argument lookup can fail
before a child enters; any child, allocation, diagnostic fatal, output
open, or write failure preserves every completed prefix and suppresses
the remaining case barriers.

At EndDay, a smoothing failure prevents all reductions; a reducer failure
retains the complete smoothing phase. At EndZone, a failure can retain the
EMS callback, a prefix of the six overrides, earlier complete child
barriers, a closed ZSZ, or a partial/open SPSZ. It prevents Facility
EndZone, `ZoneSizingRunDone = true`, and pulse Rezero in the production
caller.

A successful writer closes its stream. Re-entry through `ensure_open`
opens a completed closed file with non-append mode and truncates/rebuilds
it. Retry after interruption depends on whether the existing stream is
still good: it can reuse and append to an open prefix, or reopen after a
bad/closed state. Ordinary iostream badbit is not checked, so a truncated
artifact can also return normally while later barriers continue. ZSZ can
be complete and closed before an SPSZ failure.

Whole-parent retry can rebuild output from state retained by a prior
Calc3/Calc7 prefix, even though those stages followed the first attempt's
writer. EndZone replay also repeats EMS callbacks, diagnostics, dense
copies, latent selection, and Calc7 factor/floor work. Duplicate
memberships compound the relevant child behavior. There is no
whole-parent idempotence or repair boundary.

### C++ execution census

The completing C++ test corpus has this `(E,Q)` histogram, where `E` is
the number of design periods traversed and `Q` is Zone timesteps per day:

| `(E,Q)` | Sessions |
|---|---:|
| `(1,144)` | 7 |
| `(1,96)` | 3 |
| `(1,1)` | 2 |
| `(2,144)` | 21 |
| `(2,96)` | 23 |
| `(2,24)` | 2 |
| `(3,144)` | 1 |

This gives 107 BeginDay parent calls and 107 EndDay parent calls: 105
production calls plus two direct calls in each case. BeginDay dispatches
197 leaves, 155 Zone plus 42 Space. Each EndDay phase dispatches the same
197 leaves.

The one-accepted-system-substep DuringDay floor is 12,290 parent calls,
12,288 production plus two direct, and 23,426 children, 17,378 Zone plus
6,048 Space. DuringDay is inside the adaptive `NumOfSysTimeSteps` loop;
system downsteps can increase runtime counts, so these are nominal floors,
not exact dynamic maxima.

There are 59 EndZone sessions: 51 normal production passes, six
component-load pulse passes, and two direct pulse sessions. The exact
mapped-child matrix is:

| EndZone child | Calls |
|---|---:|
| ManageEMS | 59 |
| Calc1 | 7 |
| Calc2 | 93 = 72 Zone + 21 Space |
| ZSZ/SPSZ writer | 58 = 51 ZSZ + 7 SPSZ |
| Calc3 | 13 = 4 Zone + 9 Space |
| Calc4 | 273 = 183 Zone + 90 Space |
| Calc5 | 118 = 97 Zone + 21 Space |
| Calc6 | 301 = 197 daily + 104 final |
| Calc7 | 104 = 83 Zone + 21 Space |

These total 1,026 mapped-child invocations plus 58 `ensure_open` calls.
All 58 completed writers take the default comma route; tab and text have
zero completion coverage.

The seven Space-enabled normal sessions each contain one controlled Zone
and three Spaces. The six production pulse contexts are in
`Autosizing/BaseClassSizing`, `BranchNodeConnections`, and
`OutputReportTabular` unit tests. The two direct contexts are the
`ZoneEquipmentManager` NoLoad and DOASLoad tests. Both direct tests use
one controlled Zone, no Space sizing, unit timestep/fraction data,
AnyEMS false, and explicitly set pulse before EndZone.

Calc4 executes 273 calls over 203 distinct session-target identities,
including 48 structurally redundant Space recopies. Calc5 executes 118
calls over 107 identities. Because neither dense pass filters control
state, tested execution includes 28 uncontrolled daily Zone records and
14 uncontrolled final Zone records. That is execution evidence for the
literal traversal, not an oracle for intended day/owner alignment.

### C++ assertion and coverage boundary

The two direct parent tests contain eight call expressions, two per
handled indicator. They make no assertion immediately after an individual
BeginDay, DuringDay, or EndDay call. Their 14 immediate post-chain checks
comprise eight unchanged upstream calculated inputs and six actual chained
Zone peak-temperature/final outputs. Both EndZone calls are pulse, so
neither directly exercises Calc1-3 or a writer.

Production post-sizing evidence has 11 final/calculated field assertions
in WindowAC and BaseClassSizing tests. Downstream reporting adds 300
composite assertions: 290 heating/cooling cells over 29 SizingManager
Zone/Space roles and ten WindowAC zero-heating cells. Those assertions
occur after the complete sizing chain and do not isolate this switch,
individual barrier order, or most parent topology.

There is no ZSZ/SPSZ byte, path, separator, header, row, close, or failure
assertion. Because the nonpulse writer precedes Calc3-7, even a current
file oracle could not prove same-attempt latent selection or user-final
transformations.

ManageEMS executes 59 times, but no EMS-enabled EndZone session or
override assignment is established. No test input contains any of the six
exact Zone-sizing actuator control names. The only direct test assignments
to the six override flags occur in the unrelated Rezero test. Coverage
does not establish the outer AnyEMS gate, any flag/current-value pair,
raw zero/negative/NaN replacement, uncontrolled-Zone mutation, replay, or
the absence of a Space override pass.

There is also no direct oracle for:

- `Invalid`, `EndSysSizingCalc`, `Num`, or arbitrary-cast default dispatch;
- normal/nonpulse direct execution or the intermediate pulse run-done then
  Rezero state;
- tab/TXT routing, false output control, open/write/close failure, or
  retry;
- duplicate, cross-owner, or unreferenced Space indexes;
- a Space-enabled controlled Zone with exactly one Space;
- more than one total Zone while Space sizing is active;
- mixed control flags, stale counts, malformed indexes/extents, a child
  fatal, retained failure prefixes, or whole-parent replay.

### Rust, data, and claim boundary

The Rust/data audit covers all 721 current-worktree files returned by
`rg --files crates data`; strict UTF-8 decoding fails for zero files.
Exact and mechanical snake-case searches find no `UpdateZoneSizing`,
`update_zone_sizing`, routine key, sizing `CallIndicator` dispatcher,
handled-stage enum protocol, `ZoneSizingRunDone`, Facility-sizing handoff,
pulse/Space-sizing flag pair, Zone/Space daily/final sizing arena, six EMS
override destinations, `writeZszSpsz`, ZSZ/SPSZ artifact, or sizing output
path family.

Rust does contain adjacent run-period time, thermostat, equipment graph,
operational IdealLoads, generic output, and unsupported-EMS concepts.
None supplies this design-sizing switch, its mutable record graph, its
barrier topology, or its output lifecycle.

Comment-stripped active IDF data contain 61 `SimulationControl` objects in
61 files, and all 61 disable Zone sizing. Five
`SizingPeriod:DesignDay` objects occur in four files. Active IDF objects
and all 12 epJSON documents contain no `Sizing:Zone`,
`Sizing:Parameters`, authored `Space`, or `SpaceList` object/key.

The sole raw `Sizing:Zone` fixture in `crates` or `data` is an
arbitrary-run unsupported-sizing test. It requires `UnsupportedSizing` and
the diagnostic that sizing workflows are not ported. Capabilities keep
`Sizing:*`, `ZoneSizing*`, and Space partitioning run-blocked. No execution
or support inference follows from generic time or dormant design-day
input.

### Governance and next source boundary

CP260 adds only canonical required `source_mapped`
`routine.update_zone_sizing` and its ordered HVAC project-contract
requirement. A parent row is necessary because the mapped leaves do not
encode enum/default behavior, phase barriers, EMS and pulse gates, output
routing, or the dense-versus-membership topology.

It adds no Rust target or state, support declaration, C++ or Rust test,
capability, output implementation, comparator, case, manifest evidence,
numerical claim, performance claim, or conformance promotion. The
inventory becomes 32 algorithms and 265 routines, split 58
`state_mapped` plus 207 `source_mapped`, with 142 required. Domain-required
counts are heat-balance 88, HVAC 31, plant 1, and time/schedule 22. The
parent algorithm remains `scaffold` at claim level `none`; HVAC readiness
is `0/31`.

CP261 must expand the already-existing required
`routine.sim_zone_equipment` row in place rather than add a duplicate
routine or project-contract entry. `SimZoneEquipment` is declared at
`ZoneEquipmentManager.hh` line 184 and implemented at
`ZoneEquipmentManager.cc` lines 3538-4193. The next physical definition,
`SetZoneEquipSimOrder`, starts at line 4195 and is declared at header line
186.

## CP261 `SimZoneEquipment` Complete Mutable Parent Protocol

CP261 expands the existing required `routine.sim_zone_equipment` row in
place; its five ledger fields and its already-ordered HVAC project item do
not change. The declaration is `ZoneEquipmentManager.hh` line 184 and the
complete definition is `ZoneEquipmentManager.cc` lines 3538-4193:

```cpp
void SimZoneEquipment(
    EnergyPlusData &state,
    bool const FirstHVACIteration,
    bool &SimAir);
```

The header declares the by-value Boolean without top-level `const`, which is
the same C++ function type. `state` is the mutable shared simulation graph.
`FirstHVACIteration` reaches simple airflow, supply-path children, equipment
simulators, capacity caching, and the exhaust-system, mass-balance, and
leaving-condition tail children. It is not passed to exhaust controls,
whole-system duct loss, or the return path. `SimAir` is an in/out reference only by signature: this body never
reads or clears it and assigns only `true` after a completed reverse supply
path reports an inlet change. When no such path changes, the incoming value
is preserved.

### Caller and cadence boundary

There is one direct production call expression, in
`ManageZoneEquipment` at source line 160. `ZoneSizingCalc=true` selects
`SizeZoneEquipment` instead. On the non-sizing branch, only a successful
return allows that wrapper to set `ZoneEquipSimulatedOnce=true`, call
`UpdateZoneEquipment`, and then clear its caller's `SimZone`.

Production reaches the wrapper during begin-environment priming, the
mandatory first HVAC pass, later air/Zone convergence passes, system-sizing
setup, and post-sizing system adjustment. The Zone-sizing call in
`HVACManager` does not reach CP261 because it establishes the sizing gate.
`isPulseZoneSizing` is not read here; normal wrapper routing likewise keeps
a pulse sizing pass on the sizing branch. A direct caller can bypass those
outer lifecycle gates.

### Local setup and forward supply-path pass

The body starts with `SupPathInletChanged=false`, four zero output locals,
`FirstCall=true`, and `ErrorFlag=false`. It then visits supply paths in
ascending path order and each path's components in ascending component
order.

| Component type | Forward behavior |
|---|---|
| `AirLoopHVAC:ZoneSplitter` | call `SimAirLoopSplitter` unless both `AirflowNetworkFanActivated` and `distribution_simulated` are true |
| `AirLoopHVAC:SupplyPlenum` | always call `SimAirZonePlenum` |
| any other value | emit severe and continue diagnostics, then fatal termination |

`FirstCall` remains true for every component in every forward path; it
identifies the complete pass, not the first component. The splitter
suppression predicate does not suppress a plenum. Forward
`SupPathInletChanged` is not reset between paths and is never inspected
after the pass, so all forward change reports are discarded. Only then does
the routine set `FirstCall=false`.

If `EnforceZoneMassBalance` is false, no simple-airflow call occurs. When
true, the first HVAC iteration calls `CalcAirFlowSimple(state, 0)`; later
iterations use the overload carrying the mixing- and infiltration-adjustment
flags. This gate is distinct from the unconditional mass-balance child in
the final tail.

When Space heat-balance simulation is active and global `DoingSizing` is
false, every stored `zoneEquipMixer` receives `setOutletConditions` before
any controlled Zone or equipment is processed. The same conjunction gates
all later SpaceHB resets, splitter load adjustment/output distribution, and
the post-equipment mixer inlet pass.

### Controlled-Zone reset and priority initialization

The outer Zone loop tests every integer from one through `NumOfZones`.
Uncontrolled `ZoneEquipConfig` entries immediately continue. A controlled
Zone first clears:

- its heat-balance `NonAirSystemResponse` and `SysDepZoneLoads`;
- `ZoneExh`, `ZoneExhBalanced`, and `PlenumMassFlow`; and
- sets global `CurZoneEqNum` to the current Zone.

Under the SpaceHVAC gate, every stored Space membership has both
heat-balance response fields cleared before its own `IsControlled` test.
An uncontrolled Space configuration then continues; a controlled one also
clears its three exhaust/plenum fields. Thus response reset coverage and
equipment-configuration reset coverage are intentionally different.

The routine next calls
`InitSystemOutputRequired(state, ControlledZoneNum,
FirstHVACIteration, true)` exactly once per controlled Zone. The literal
true is `ResetSimOrder`, so the child path invokes
`SetZoneEquipSimOrder` on every CP261 call, not only on the first HVAC
iteration. CP262 maps that separate child body; CP261 records only this
parent call, its gate, and its placement.

### Per-equipment pre-dispatch state

The equipment loop visits priority positions from one through
`NumOfEquipTypes`, then obtains the actual list entry through
`PrioritySimOrder(EquipTypeNum).EquipPtr`. Before type dispatch, every slot
clears:

- global `TurnFansOn` and `TurnFansOff`;
- global `UnbalExhMassFlow`, `BalancedExhMassFlow`, and
  `PlenumInducedMassFlow`;
- local sensible, latent, and non-air output receivers; and
- global `DataCoolCoilCap`.

While `FirstPassZoneEquipFlag` is true, each slot also clears 18 fields in
the current Zone's single `ZoneEqSizing` record:

`AirVolFlow`, `MaxHWVolFlow`, `MaxCWVolFlow`, `OAVolFlow`,
`DesCoolingLoad`, `DesHeatingLoad`, `CoolingAirVolFlow`,
`HeatingAirVolFlow`, `SystemAirVolFlow`, `AirFlow`,
`CoolingAirFlow`, `HeatingAirFlow`, `SystemAirFlow`, `Capacity`,
`CoolingCapacity`, `HeatingCapacity`, `SystemCapacity`, and
`DesignSizeFromParent`.

The first-pass latch is not cleared inside the slot or Zone loops, so that
same reset block executes for every slot of the first completing invocation.

Availability processing derives `ZoneCompNum` from the actual equipment
entry. `ValidSAMComp` checks only that the integer equipment type is at
most `NumValidSysAvailZoneComponents` (14); there is no local lower-bound
check. A positive component index plus that upper-bound result calls
`GetZoneEqAvailabilityManager`. Its shared `ErrorFlag` starts false once per
parent call but is never inspected or reset locally. `CycleOn` writes the
fan pair true/false; `ForceOff` writes false/true; all other statuses retain
the per-slot false/false reset. ADU and exhaust-fan branches can then apply
the Zone-wide availability status as additional writes.

Under the SpaceHVAC gate, a nonnegative equipment splitter index causes
`adjustLoads` immediately before type dispatch. This uses the priority
position as its equipment-order argument; output distribution for the same
slot occurs only after the simulator returns.

### Thirty-three equipment labels and 27 simulator bodies

The type switch covers all 33 ordinary source enum values, with nine labels
grouped into three shared bodies, six fewer bodies than labels. The precise
branch effects are:

| Explicit type label(s) | Simulator and local reconciliation |
|---|---|
| `AirDistributionUnit` | apply Zone availability fan flags, call `ManageZoneAirLoopEquipment`, then set sensible output to air plus non-air output |
| `VariableRefrigerantFlowTerminal` | call `SimulateVRF` with Zone-equipment mode true |
| `WindowAirConditioner` | call `SimWindowAC` |
| `PackagedTerminalHeatPump`, `PackagedTerminalAirConditioner`, `PackagedTerminalHeatPumpWaterToAir`, `UnitarySystem` | call the shared component pointer's `simulate` body |
| `DehumidifierDX` | call `SimZoneDehumidifier`, add its sensible result directly to Zone `SysDepZoneLoads`, then zero the common sensible result |
| `FourPipeFanCoil` | call `SimFanCoilUnit` |
| `UnitVentilator` | call `SimUnitVentilator` |
| `UnitHeater` | call `SimUnitHeater` |
| `PurchasedAir` | call `PurchasedAirManager::SimPurchasedAir` with the priority name, sensible/latent receivers, iteration flag, controlled Zone, and equipment index |
| `BaseboardWater` | call `SimHWBaseboard`, copy sensible output to non-air output, and zero latent output |
| `BaseboardSteam` | call `SimSteamBaseboard`, copy sensible output to non-air output, and zero latent output |
| `BaseboardConvectiveWater` | call `SimBaseboard`, copy sensible output to non-air output, and zero latent output |
| `BaseboardConvectiveElectric` | call `SimElectricBaseboard`, copy sensible output to non-air output, and zero latent output |
| `CoolingPanel` | call `SimCoolingPanel`, copy sensible output to non-air output, and zero latent output |
| `HighTemperatureRadiant` | call `SimHighTempRadiantSystem` and zero latent output; its non-air effects are not copied into the local non-air receiver |
| `LowTemperatureRadiantConstFlow`, `LowTemperatureRadiantVarFlow`, `LowTemperatureRadiantElectric` | call the shared `SimLowTempRadiantSystem` body and zero latent output |
| `ExhaustFan` | apply Zone availability fan flags, lazily cache `GetFanIndex` when the list index is zero, then call the fan object's `simulate` method |
| `HeatExchanger` | call `SimHeatRecovery` in continuous-fan mode |
| `EnergyRecoveryVentilator` | call `SimStandAloneERV` |
| `HeatPumpWaterHeaterPumpedCondenser`, `HeatPumpWaterHeaterWrappedCondenser` | call the shared `SimHeatPumpWaterHeater` body |
| `VentilatedSlab` | call `SimVentilatedSlab` |
| `OutdoorAirUnit` | call `SimOutdoorAirUnit` |
| `BaseboardElectric` | call `SimElecBaseboard`, copy sensible output to non-air output, and zero latent output |
| `RefrigerationChillerSet` | call `SimAirChillerSet` and copy sensible output to non-air output |
| `UserDefinedHVACForcedAir` | call `SimZoneAirUserDefined` |
| `EvaporativeCooler` | call `SimZoneEvaporativeCoolerUnit` |
| `HybridEvaporativeCooler` | call `SimZoneHybridUnitaryAirConditioners` |

A value not matched by those 33 labels reaches a silent default. It still
runs the ordinary common reconciliation with the slot's reset outputs; this
is intentionally different from the fatal supply-path defaults.

PurchasedAir is therefore one branch inside a larger parent, not an alias
for the parent. The direct Rust compatibility call can reproduce selected
PurchasedAir behavior while bypassing every other row in this table and all
parent-owned state around it.

### Common per-slot reconciliation and normal cleanup

After every matched or default branch, the parent performs this fixed work:

1. add unbalanced plus balanced exhaust flow to Zone `ZoneExh`;
2. add balanced exhaust flow to `ZoneExhBalanced`;
3. add induced plenum flow to `PlenumMassFlow`;
4. on a first HVAC iteration with a non-sequential load scheme, write
   exactly one capacity cell: positive sensible output writes heating,
   while zero or negative output writes cooling;
5. under the SpaceHVAC/splitter gate call `distributeOutput`; otherwise add
   local non-air output to the Zone's `NonAirSystemResponse`;
6. call `updateSystemOutputRequired` for sensible and moisture demand; and
7. reset global `CurTermUnitSizingNum` to zero.

The capacity branch does not clear the opposite sign cell. A later replay
whose output sign changes can therefore leave a new cell beside an older
opposite-sign value. Zero is classified as cooling. The common demand
update executes even for the silent switch default and for equipment
branches whose local outputs remain zero.

After all controlled Zones complete, the SpaceHVAC gate calls
`setInletFlows` once per mixer. Only then are `CurZoneEqNum` and
`FirstPassZoneEquipFlag` cleared. Those two writes precede the complete
reverse-path and final-tail regions.

### Reverse supply paths and fixed tail

Supply paths remain in ascending path order, but their components are
visited in descending order. `SupPathInletChanged` is reset at the start of
each reverse path.

A splitter uses the same two-flag AirflowNetwork suppression predicate and
receives `FirstCall=false`. Only after an executed splitter, and only when
`DuctLossSimu` is true, does the parent call the path-specific supply
`SimulateDuctLoss`. A supply plenum always simulates. Any other component
again emits severe/continue/fatal diagnostics.

After a complete reverse path, a true change flag assigns `SimAir=true`.
There is no false assignment and multiple paths may redundantly assign
true. When every path has completed, six children execute unconditionally
in this exact order:

1. `SimZoneHVACExhaustControls`;
2. `SimExhaustAirSystem`;
3. `CalcZoneMassBalance`;
4. `CalcZoneLeavingConditions`;
5. whole-system `DuctLoss::SimulateDuctLoss`; and
6. `SimReturnAirPath`.

The final whole-system duct-loss call is independent of the reverse
splitter's path-specific duct-loss gate. Likewise the final Zone mass
balance is independent of the earlier optional `CalcAirFlowSimple`.

### State ownership and cardinality

Direct persistent writes span these parent-owned groups:

- Zone and optional Space heat-balance response resets and accumulation;
- Zone and controlled-Space exhaust/plenum resets and Zone accumulation;
- `CurZoneEqNum`, `CurTermUnitSizingNum`, and `DataCoolCoilCap`;
- global fan commands and three exhaust/plenum scratch flows;
- all 18 first-pass `ZoneEqSizing` fields;
- ADU/exhaust availability fan rewrites;
- the dehumidifier's system-dependent Zone-load addition;
- lazy exhaust-fan equipment-index caching;
- one sign-selected equipment-capacity cell;
- the first-pass latch; and
- the caller's monotonic `SimAir` flag.

The source contains 55 static direct persistent assignment sites over 41
normalized lvalue families when `SimAir` is included. Removing that in/out
write leaves 54 state-graph sites over 40 families. These counts exclude
local assignments, reference binding, and all state mutation performed
inside child calls.

For one successful invocation, define:

- `P` supply paths, `X` splitter components, and `Y` supply plenums;
- `A=1` unless both AirflowNetwork suppression flags are true;
- `D=1` when supply duct-loss simulation is enabled;
- `B=1` when Zone mass balance is enforced;
- `Z` total Zones and `C` controlled Zones;
- `H=1` for active non-sizing SpaceHVAC and `M` mixer entries;
- `J` stored Space occurrences under controlled Zones and `Jc` occurrences
  whose Space equipment configuration is controlled;
- `Q` equipment slots over all controlled Zones;
- `V` slots with a positive component index and integer equipment type at
  most 14;
- `T` slots passing the SpaceHVAC splitter gate;
- `R` slots matching one of the 33 explicit equipment labels;
- `G` exhaust-fan slots whose equipment index is zero on branch entry;
- `N` slots in non-sequential equipment lists;
- `F=1` for the first HVAC iteration; and
- `P1=1` when the first-pass latch is true on entry.

Then the two supply passes execute `2*(X+Y)` component switches,
`2*A*X` splitter simulations, `2*Y` plenum simulations, and `A*D*X`
path-specific duct-loss calls. Zone-loop tests equal `Z`; full Zone bodies
equal `C`. Space response resets equal `H*J`, controlled-Space flow resets
equal `H*Jc`, and Space continues equal `H*(J-Jc)`.

Equipment bodies and demand updates each equal `Q`; the 18-field reset block
executes `P1*Q` times. Availability calls equal `V`; Space load adjustments
and distributions each equal `T`; matched equipment simulator calls equal
`R`; lazy fan lookups equal `G`; and first-iteration non-sequential
capacity writes equal `F*N`. Each slot performs three Zone flow
accumulations. Reverse-path `SimAir=true` assignments range from zero
through `P`.

The principal operational child/service invocation count is therefore

`2*(A*X+Y) + B + 2*H*M + C + V + 2*T + R + G + Q + A*D*X + 6`.

This excludes diagnostics, formatting, and nested child work. Statically,
the body has 48 operational child/service call sites. Adding the six
severe/continue/fatal sites gives 54, and adding four diagnostic formatting
sites gives 58.

The exact control inventory is nine loops, 25 `if` tokens including one
`else if`, three plain `else` branches, three switches, 37 explicit case
labels, three defaults, two continues, and 34 breaks. There is no explicit
return, while, do, or goto.

### Failure, partial effects, and replay

The parent has no up-front topology validation, local result status, catch,
cleanup guard, transaction, checkpoint, rollback, or replay-repair phase.
Any diagnostic fatal, invalid index, allocation failure, or child failure
retains every completed direct write and child mutation in source order.

Important cut points are:

- forward-path failure retains completed component mutations and prevents
  every Zone, reverse-path, and tail action;
- a failure in the Zone/equipment region can leave `CurZoneEqNum` set to
  the current Zone and `FirstPassZoneEquipFlag` true;
- a simulator can succeed and then fail before common reconciliation,
  retaining its child state while skipping exhaust/plenum accumulation,
  capacity write, output distribution, remaining-demand update, and
  `CurTermUnitSizingNum` reset;
- failure during Space mixer inlet work occurs after all equipment
  reconciliation but before the two normal cleanup writes;
- reverse-path failure occurs after `CurZoneEqNum=0` and first-pass clear,
  retains all earlier Zone/equipment state plus its completed reverse
  prefix, and blocks the six-call tail; and
- a tail failure retains every earlier Zone/reverse effect plus its
  completed tail prefix.

A successful retry is not a no-op. Zone and Space responses are reset and
rebuilt, but child histories and availability state can advance. Priority
order is rebuilt through the true reset argument. Supply components and all
tail children run again. The first-pass 18-field resets disappear once an
invocation reaches the post-Zone latch clear, a lazy fan index remains
cached, and only the currently selected capacity sign cell for each eligible
slot is overwritten. `SimAir` can become true but cannot become false here.

A failure before the first-pass clear causes retry to repeat the 18-field
reset for every reached slot; a failure after that clear never restores the
latch. A failure before `ManageZoneEquipment` regains control also prevents
its simulated-once write, later Update call, and `SimZone=false` write.
Thus neither the child body nor its wrapper supplies an atomic retry
boundary.

### C++ test and lifecycle evidence

Four direct calls occur across two tests. One PTAC/plenum test passes
`FirstHVACIteration=false`; three calls in the PTAC availability test pass
true. Nine further `ManageZoneEquipment` calls across eight tests reach
CP261 with `ZoneSizingCalc=false`: two bypass-VAV, two PTAC/plenum, four
PurchasedAir, and one unit-heater context. This gives 13 directly
attributable successful invocations, 11 first-iteration and two later-
iteration.

There are 18 direct `ManageSizing` test contexts. The one plant-only water-
to-water heat-pump test has no Zone equipment and does not reach CP261. The
other 17 reach it exactly once after the sizing gate is cleared: ten through
the first system-sizing route and seven through the alternate route, all
with `FirstHVACIteration=true`. Eight explicitly set `DoingSizing`; nine
leave its default false, whereas production wraps `ManageSizing` with
`DoingSizing=true`.

Across those 30 statically attributable successful calls, the audited
child topology is:

| Evidence | Count |
|---|---:|
| handled equipment dispatches | 65 |
| `updateSystemOutputRequired` calls | 65 |
| ADU dispatches | 36 |
| PTAC dispatches | 18 |
| PurchasedAir dispatches | 4 |
| unitary-system dispatches | 3 |
| unit-heater dispatches | 2 |
| window-AC dispatches | 2 |
| availability-manager calls | 26 |
| splitter simulations over both passes | 34 |
| supply-plenum simulations over both passes | 2 |
| executions of each one of the six tail children | 30 |

All equipment lists in this attributable set are sequential, so the
non-sequential capacity-cache branch has zero hits. There is no SpaceHVAC
mixer/splitter topology, no enabled `EnforceZoneMassBalance`, no invalid
supply-path component, and no silent equipment-default slot.

The PTAC/plenum test asserts mass conservation and return/plenum/mixer node
relationships after repeated successful entry, but does not isolate retry
semantics. The availability test covers the initial no-crash path, false/
false fan reset, and `CycleOn`; it has no `ForceOff` assertion. PurchasedAir
tests assert selected plenum/node/flow, exhaust/fuel, mixed-air, and zero-
capacity descendants. The unit-heater test observes an ADU-before-unit-
heater two-slot order and remaining-load result. Bypass-VAV assertions occur
after further air-loop/component work and are integration evidence rather
than isolated CP261 oracles.

There are 57 active `ManageSimulation` expressions in the C++ corpus. Fifty-
six complete; one EMS fatal test terminates before HVAC. The successful
zoned subset has 55 configurations, 81 Zones, 55 controlled Zones, and 26
uncontrolled Zones. Its comment-labelled equipment-list entries total 64:
39 ADU, five fan coils, five IdealLoads, four radiant/electric baseboards,
three VRF terminals, two water-to-air heat pumps, two window ACs, and one
each of dehumidifier, ERV, unit heater, and Zone exhaust fan.

Those complete simulations exercise repeated first/later HVAC passes, but
warmup duration, environment count, timestep count, and convergence loops
prevent an exact dynamic CP261 invocation count without instrumentation.
The duct-loss integration case asserts numerics only after additional
direct duct-loss calls, so it does not isolate the reverse-path call. Two
full simulations contain AirflowNetwork controls, but static input cannot
prove runtime splitter suppression counts. No successful full-simulation
case enables Space heat-balance simulation or enforced Zone mass balance.

No test directly asserts reverse change propagation to `SimAir=true`,
non-sequential capacity-cell retention, Space load distribution, the
18-field first-pass lifecycle, invalid supply-path diagnostics, the silent
equipment default, an availability error, child failure, partial state,
cleanup, rollback, or retry. Repeated successful PTAC calls establish only
re-entry, not idempotence. The EMS fatal case stops before CP261 and supplies
no failure evidence.

### Rust execution and active-data boundary

Rust's adjacent `ideal_loads_zone_equipment_stages()` array contains three
reporting labels: ManageZoneEquipment, SimZoneEquipment, and
SimPurchasedAir. The runtime does not execute that array. The execution
plan places `ManageZoneEquipment` and `SimZoneEquipment` steps together in
one `ZoneEquipmentManager` stage, then places PurchasedAir in its later
stage. `actual_source_order_stage_ids()` lists represented stage names; it
does not trace step execution. Consequently, a plan-order assertion or
matching stage list is not evidence that this parent body ran.

The active `ep_run` pipeline does not interpret the plan's
`SimZoneEquipment` step. It calls
`simulate_ideal_loads_purchased_air_compat` directly. That function
iterates prebound typed IdealLoads records, validates their graph edges,
then enters the PurchasedAir compatibility wrapper or OA helper. It never
runs supply paths, priority construction, per-slot resets, the 33-type
switch, common reconciliation, reverse paths, or the six-child tail.

Across `crates` and `data`, the exact `SimZoneEquipment` spelling occurs
only eight times in one documentation comment, enum/plan construction, two
formatters, reporting metadata, a constant, and a test assertion. There is no snake-case
`sim_zone_equipment` function. Searches for 29 canonical parent state or
child identifiers find no Rust implementation, including
`PrioritySimOrder`, `FirstPassZoneEquipFlag`, `SupPathInletChanged`,
`NonAirSystemResponse`, `SysDepZoneLoads`, `CurZoneEqNum`, the fan-command
pair, simple airflow, splitter/plenum simulation, exhaust controls/system,
Zone mass balance, leaving conditions, and return-path simulation.

`ZoneEquipmentObjectType` contains only `IdealLoadsAirSystem`, and the
compiler accepts only that equipment-list type. A direct compiler test
rejects a `Fan:ConstantVolume` list entry as an invalid enum value. Four
load-distribution enum values are parsed and stored, but the PurchasedAir
runtime does not consume the distribution scheme. Cooling/heating sequence
identities participate only in graph validation; there is no source
`PrioritySimOrder` construction or load-distribution execution.

The five Rust Zone-equipment tests cover four graph-validation cases and
one demand-sign metadata case. One execution-plan test asserts array order,
and one compiler test covers an unsupported equipment type. There is no
Rust test for supply-path forward/reverse order, priority execution,
availability/fan state, the other 32 ordinary equipment types, per-slot
reset and reconciliation, Space distribution, capacity caching, first-pass
lifecycle, mass-balance/tail order, failure prefixes, or replay.

The active IDF census over the same 721 strict-UTF-8 `crates` and `data`
files finds 30 column-zero `ZoneHVAC:EquipmentConnections`, 30
`ZoneHVAC:EquipmentList`, and 30 `ZoneHVAC:IdealLoadsAirSystem` objects.
All 30 list-bearing files are under `data/conformance_cases`; no epJSON file
contains an equipment list. Each list has exactly eight fields, uses
`SequentialLoad`, contains one IdealLoads entry, uses cooling/heating
sequence `1/1`, and leaves both fraction-schedule fields blank.

There are no active `AirLoopHVAC:SupplyPath`,
`AirLoopHVAC:ZoneSplitter`, `AirLoopHVAC:SupplyPlenum`,
`Fan:ZoneExhaust`, or Space/Zone HVAC equipment mixer/splitter objects.
The current executable lane therefore witnesses only a prebound,
single-entry sequential IdealLoads graph. It supplies no support evidence
for supply-path/return-air-path/exhaust/plenum topology, SpaceHVAC, availability/fan/
capacity state, the other 32 ordinary equipment types, or the fixed tail.

### Governance and next source boundary

CP261 changes no routine metadata and adds no project-contract entry. It
adds no Rust target or state, support declaration, C++ or Rust test,
capability, output implementation, comparator, case, manifest evidence,
numerical claim, performance claim, or conformance promotion.

The inventory remains 32 algorithms and 265 routines, split 58
`state_mapped` plus 207 `source_mapped`, with 142 required. Domain-required
counts remain heat-balance 88, HVAC 31, plant 1, and time/schedule 22, with
readiness `0/88`, `0/31`, `0/1`, and `0/22`. The IdealLoads parent remains
`scaffold` at claim level `none`.

CP262 next adds required source-mapped
`routine.set_zone_equip_sim_order` immediately after
`routine.sim_zone_equipment` and before `routine.sim_purchased_air`, plus
the same ordered HVAC project-contract item. `SetZoneEquipSimOrder` is
declared at `ZoneEquipmentManager.hh` line 186 and implemented completely
at `ZoneEquipmentManager.cc` lines 4195-4255. The next physical definition,
`InitSystemOutputRequired`, starts at source line 4257 and is declared at
header line 188.

## CP262 `SetZoneEquipSimOrder` Shared Priority-Scratch Rebuilder

CP262 adds canonical required `routine.set_zone_equip_sim_order`
immediately after `routine.sim_zone_equipment` and before
`routine.sim_purchased_air`, plus the same ordered HVAC project-contract
item. The routine is declared at `ZoneEquipmentManager.hh` line 186 and
implemented completely at `ZoneEquipmentManager.cc` lines 4195-4255:

```cpp
void SetZoneEquipSimOrder(
    EnergyPlusData &state,
    int const ControlledZoneNum);
```

The declaration omits the definition's top-level `const` on the by-value
integer; the C++ function type is unchanged. `ControlledZoneNum` is the
actual Zone index, not an ordinal within the controlled-Zone subset. The
routine returns `void` and writes the manager-global `PrioritySimOrder`
scratch array while reading the canonical equipment list and current Zone
sensible demand.

The exact source order is:

1. alias `ZoneEquipList(ControlledZoneNum)`, read its `NumOfEquipTypes` as
   `N`, and copy each active list row into scratch positions `1..N`;
2. copy six fields per row: equipment type name, equipment name, enum type,
   cooling priority, heating priority, and the original list ordinal as
   `EquipPtr`;
3. visit scratch positions `N+1..U`, where `U` is the scratch upper bound,
   clearing the two names, setting the enum to `Invalid`, and setting
   `EquipPtr` to zero, while leaving both priority integers untouched; then
4. for each active position `i`, compare it with every position `j=i..N`
   and immediately exchange all six fields whenever the selected candidate
   has a smaller priority, refreshing both current-priority locals after
   every exchange.

This is an in-place exchange-selection pass, not a conventional selection
sort that finds one minimum before one swap. Negative
`RemainingOutputRequired` selects ascending cooling priority. Zero or
positive demand selects ascending heating/no-load priority; both positive
and negative zero therefore use heating. A NaN demand satisfies neither
sign comparison and leaves the freshly copied source order unchanged.
The routine does not read `LoadDistScheme`, availability counts, capacity
caches, schedules, `FirstHVACIteration`, or Space demand.

Source input parsing rejects values below zero or above `N`, so zero is
accepted despite the diagnostic text saying priorities must be positive.
It rejects duplicate positive priorities and warns about missing positive
sequence numbers; multiple zeros can remain. CP262 itself neither skips
zero nor consults `NumAvailHeatEquip` or `NumAvailCoolEquip`, so a selected
zero sorts before every positive value. The comparison is strict, which
prevents a direct equal-key exchange, but the immediate-exchange algorithm
is not globally stable if malformed equal priorities reach it because an
intervening smaller row can reverse equal-key rows.

All six fields move as one logical record. In particular, `EquipPtr`
continues to identify the original list row after sorting, and the
unselected priority dimension travels with that row rather than being
recomputed. Names, enum, and pointer above `N` are scrubbed, but the two priority fields
there preserve their pre-existing bytes: allocation defaults until written,
then values from whichever prior larger Zone populated them.
`PrioritySimOrder` is allocated by `GetZoneEquipment` to the maximum
equipment-list count and shared across Zones, so the last caller wins; it
is not a per-Zone cache.

For valid `0 <= N <= U`, let `S` be the number of successful exchanges.
The copy performs `6N` field writes, upper cleanup performs `4(U-N)`
mutations, and the nested loops visit exactly `N(N+1)/2` pairs, including
self-pairs. Each exchange has six swap calls and twelve destination-field
endpoints, with `0 <= S <= N(N-1)/2`. Dynamic persistent mutation-statement
count is

`6N + 4(U-N) + 6S = 2N + 4U + 6S`;

counting the two endpoints of every swap separately gives
`2N + 4U + 12S`. Exactly `2(U-N)` upper priority cells are not written.
The body has four `for` loops and one `if`, with no `else`, switch, return,
break, or continue. It has eight direct persistent assignment sites and
eight mutating call sites: two string clears, two string swaps, and four
scalar `std::swap` calls. It allocates no local scratch array and calls no
EnergyPlus child, service, or diagnostic routine.

The only direct production caller is `initOutputRequired` line 4315, gated
by `ResetSimOrder && spaceNum == 0`. That child first restores the selected
Zone's `RemainingOutputRequired` from `TotalOutputRequired`, so ordinary
runtime sorting uses total Zone-load sign rather than a prior equipment
residual. `InitSystemOutputRequired` initializes the Zone first and may then
initialize its Spaces with nonzero `spaceNum`; those Space calls do not
reorder, even when a Space load sign differs from the Zone sign.

CP261 `SimZoneEquipment` invokes the reset-true wrapper once before
dispatch for every controlled Zone. Its tail
`CalcZoneLeavingConditions` invokes it a second time only for controlled
Zones with at least one return node. A successful CP261 call therefore
sorts `C+K` times for `C` controlled Zones and `K` such Zones with return
nodes. The sizing path can reach the same return-node call without the
front dispatch call. The second normal-runtime sort occurs after equipment
effects and overwrites the same shared scratch; it does not preserve a
Zone-specific result.

There is no comprehensive up-front validation, local status, diagnostic,
catch, cleanup guard, checkpoint, transaction, or rollback. An invalid Zone
identity, missing demand/list state, short canonical field array, or
unallocated or undersized scratch can fail before completion; depending on
the failing access, it can leave no new write or retain an already copied
or cleared prefix. A string-copy allocation failure can similarly leave
mixed old and new active rows. No rollback restores prior scratch bytes,
and a tail failure in CP261 also retains all earlier equipment and
mass-balance effects.

A successful replay with unchanged canonical list and load sign is
active-prefix idempotent: all six fields are recopied before sorting, so it
reconstructs a prior torn or permuted active prefix. The untouched upper
priority cells remain history-dependent. A sign change deliberately
rebuilds from canonical order and selects the other priority dimension;
it does not incrementally sort the previous result.

The direct unit census finds one explicit `SetZoneEquipSimOrder` call and
seven reset-true `InitSystemOutputRequired` calls. Four use `N=3` and four
use `N=4`, for 28 copied rows and 64 pair visits, but all heating and
cooling priorities are already `1..N`, so no exchange occurs. Named
parent-level unit paths bring the statically attributable total to 59
successful executions, yet every nonempty list is already ordered and no
exchange executes. Four UnitHeater assertions read slots one and two
`EquipTypeName`/`EquipName` fields and prove only the already-ordered
ADU-before-UnitHeater tuple. Nearby parsed data with different cooling
order never calls the routine.

The tests therefore do not prove an unsorted heating or cooling result,
different orders across load signs, exact-zero or NaN selection, zero or
gapped priorities, full-record tuple integrity, `U>N` cleanup and retained
priority tails, Space non-reordering, shared-scratch overwrite, successful
replay, invalid extents, or partial-failure state. Uniform, UniformPLR, and
SequentialUniformPLR distribution tests call initialization with
`ResetSimOrder=false`; only Sequential paths exercise this boundary.

Rust contains no exact or snake-case occurrence of the routine, scratch,
remaining-demand discriminator, two priority fields, or equipment pointer.
It parses all four distribution-scheme names but the three nonsequential
variants have no active runtime consumer. Compiler input requires positive
`u32` sequences and rejects duplicate cooling or heating priorities,
whereas the source accepts zero. `ModelGraph` performs a one-time static
sort by `(zone, heating, cooling, ideal_loads_id)`, and node projection uses
a similar heating-first minimum. Neither can express source cooling-first
ordering under negative demand or a runtime sign change.

The active compatibility runtime does not interpret the
`SimZoneEquipment` execution step or graph order; it directly iterates
prebound IdealLoads systems. All 30 active equipment lists contain exactly
one SequentialLoad IdealLoads entry at cooling/heating sequence `1/1`, with
both fraction schedules blank. Thus the current Rust lane makes sorting a
one-record no-op and owns no dynamic priority scratch, upper-tail policy,
record exchange, replay behavior, or list capacity cache.

CP262 changes no Rust target/state, support declaration, test, capability,
output, comparator, case, manifest, numerical claim, performance claim, or
conformance status. Counts become 32 algorithms and 266 routines, split 58
`state_mapped` plus 208 `source_mapped`, with 143 required. Domain-required
counts become heat-balance 88, HVAC 32, plant 1, and time/schedule 22, with
readiness `0/88`, `0/32`, `0/1`, and `0/22`. The IdealLoads parent remains
`scaffold` at claim level `none`.

CP263 next adds required source-mapped
`routine.init_system_output_required` immediately after
`routine.set_zone_equip_sim_order` and before
`routine.sim_purchased_air`. `InitSystemOutputRequired` is declared at
`ZoneEquipmentManager.hh` line 188 and implemented completely at
`ZoneEquipmentManager.cc` lines 4257-4290. Its child
`initOutputRequired` begins at source line 4292.

## CP263 `InitSystemOutputRequired` Zone/Space Demand-Initialization Wrapper

CP263 adds canonical required `routine.init_system_output_required`
immediately after `routine.set_zone_equip_sim_order` and before
`routine.sim_purchased_air`, plus the same ordered HVAC project-contract
item. The public wrapper is declared at `ZoneEquipmentManager.hh` line 188
and implemented completely at `ZoneEquipmentManager.cc` lines 4257-4290:

```cpp
void InitSystemOutputRequired(
    EnergyPlusData &state,
    int const ZoneNum,
    bool const FirstHVACIteration,
    bool const ResetSimOrder);
```

The header omits the definition's top-level `const` on all three by-value
parameters; the C++ function type is unchanged. Only the header supplies
`ResetSimOrder = false`. `ZoneNum` is an actual parent Zone index,
`FirstHVACIteration` is forwarded to initialization and distribution, and
`ResetSimOrder` is forwarded only to the initializers.

The exact wrapper order is:

1. call `initOutputRequired` for
   `ZoneSysEnergyDemand(ZoneNum)` and
   `ZoneSysMoistureDemand(ZoneNum)`;
2. when `doSpaceHeatBalance` is true, visit every occurrence in
   `Zone(ZoneNum).spaceIndexes` in stored order and call the same child for
   the selected Space sensible/moisture pair, passing both the unchanged
   parent `ZoneNum` and explicit `spaceNum`, which is nonzero for a valid
   Space identity; then
3. after every initializer returns, call
   `DistributeSystemOutputRequired(state, ZoneNum,
   FirstHVACIteration)` exactly once.

The wrapper owns no controlled-Zone, `ZoneSizingCalc`, `DoingSizing`,
simulation-only Space, per-Space control, owner, uniqueness, or validity
gate. Its Space flag is `doSpaceHeatBalance`, not the narrower
`doSpaceHeatBalanceSimulation` used by parts of CP261. Duplicate or
cross-listed Space identities therefore repeat, and every occurrence uses
the referring parent Zone's equipment-list, sizing, control, and deadband
context. The wrapper does not deduplicate or verify Zone membership.

The Zone initializer receives references to the Zone demand pair. Space
initializers receive references into the separate Space demand arenas but
still receive the parent `ZoneNum`; a duplicated identity revisits the same
Space records. Each successful child unconditionally copies
six sensible and six moisture remaining/unadjusted scalars from predictor
totals and setpoint totals, and finally copies
`DeadBandOrSetback(ZoneNum)` into the same
`CurDeadBandOrSetback(ZoneNum)` cell. Thus Space traversal repeats that one
Zone deadband write rather than owning a per-Space flag.

`ResetSimOrder` reaches every initializer unchanged, but CP262
`SetZoneEquipSimOrder` runs only from the Zone child because the child gate
also requires `spaceNum == 0`. Valid Space calls never select their own
priority order; all share the Zone-selected manager-global scratch. The
Zone child performs its 12 scalar demand copies before CP262, so sorting
uses the newly restored Zone `RemainingOutputRequired` sign.

After those base writes, `initOutputRequired` conditionally initializes
sequenced arrays. It tests allocation of only the main sensible sequence
before assuming that the other two sensible and all three moisture arrays
are conformable. Uncontrolled or Zone-sizing entries bulk-fill all six
sequences from predictor totals. On a controlled, nonsizing first HVAC
iteration, Sequential and Uniform do the same, whereas UniformPLR and
SequentialUniformPLR seed all three sensible arrays from the sign-selected
final design load and copy moisture totals. A later entry writes only
sequence slot one from predictor totals and may add current sensible and
latent duct loss, leaving higher slots untouched at that phase. CP264 maps
that complete child separately.

`DistributeSystemOutputRequired` is called even when it will return
immediately. Its current source expands into one Zone and the same stored
Space occurrences only when

```text
G = Zone.IsControlled
    && !ZoneSizingCalc
    && !(FirstHVACIteration
         && LoadDistScheme != Uniform
         && LoadDistScheme != Sequential)
```

so first-iteration UniformPLR and SequentialUniformPLR, uncontrolled, and
Zone-sizing calls retain initialization-only state. Later iterations of all
valid schemes can distribute. CP265 maps the distributor and its leaf
separately; CP263 records this parent ordering and gate interaction without
promoting those children.

Let `I` be one when Space heat balance is enabled and `M` the number of
stored Space-index occurrences. Then `H = 1 + I*M` initializers run and the
wrapper makes exactly `H+1 = 2+I*M` direct child calls. When `G` is true,
the distributor expands into another `H` distribution leaves. A successful
reset-true call also reaches CP262 exactly once. The fully expanded
principal count is therefore

`H + 1 + G*H + R`,

where `R` is one for reset-true and zero otherwise. Successful initializer
base work contributes `12H` demand-scalar assignments before conditional
sequence writes plus `H` deadband assignments afterward, `13H` total.

The wrapper body itself has one `if`, one range `for`, no `else`, switch,
return, break, or continue, and zero direct assignment or persistent-write
sites. It has eight syntactic call expressions when five Zone/Space array
accessors are counted with the two initializer sites and one distributor.
It performs no direct arithmetic, allocation, diagnostic, or cleanup.

There are exactly two executable production call sites, and both explicitly
pass `ResetSimOrder=true`. CP261 `SimZoneEquipment` calls the wrapper once
before equipment dispatch for every controlled Zone.
`CalcZoneLeavingConditions` calls it once, not once per return node, for
each controlled Zone having at least one return node. The latter parent is
reached from normal CP261 and from `SizeZoneEquipment` with
`FirstHVACIteration=true`.

A normal successful equipment pass therefore invokes CP263 `C+K` times for
`C` controlled Zones and `K` controlled Zones with return nodes. The second
call for a return-node Zone occurs after equipment, mass-balance, and
leaving-condition effects and restores predictor demand again. A controlled
Zone without return nodes receives only the front reset and can finish with
post-equipment residual demand. The sizing path reaches only the return-node
call, and its distributor returns immediately because `ZoneSizingCalc` is
true.

There is no local validation, result status, diagnostic, catch, cleanup
guard, checkpoint, transaction, or rollback. Invalid Zone or top-level
demand identity can fail before child entry. A Zone-child failure blocks
every Space and distribution. CP262 failure occurs after the 12 Zone scalar
copies but before sequenced-array and deadband completion. Sequence shape
failure can preserve a partial bulk-write prefix.

Failure resolving or initializing Space occurrence `k` retains the
completed Zone and prior-Space prefixes and prevents all later Spaces and
distribution. A distribution failure retains every initialization effect;
failure in a later distribution leaf additionally retains the Zone and
earlier-Space distribution prefix. No wrapper action restores any of those
states.

With every mutable dependency fixed, an immediate successful replay is
overwrite-idempotent on fields actually rewritten. It is not a canonical
whole-state repair: later Sequential initialization leaves sequence slots
above one, upper priority fields retain history, and reset-false calls can
consume another Zone's shared order. Capacities, fraction schedules, duct
loss, load sign, Space membership, and control flags are resampled, while
duplicate Space occurrences are replayed independently.

The direct C++ census finds 24 wrapper expressions across six tests and all
four distribution schemes. Ten use the first HVAC iteration and 14 a later
iteration; seven pass reset true and 17 use the header default false. Basic
Sequential and each of Uniform and UniformPLR contribute four calls,
SequentialUniformPLR contributes eight, and two mixed-equipment Sequential
tests contribute two each. Every direct call uses one controlled,
nonsizing Zone with allocated sequences and no Space traversal.

Those tests strongly assert sensible sequence arrays for heating and
cooling, first and later iterations, active-equipment counts, design-load
PLR seeding, learned-capacity PLR distribution, and sequential fraction
schedules. However, every one of the 17 reset-false calls is followed by a
separate explicit distributor call; the mixed test also inserts a direct
priority reset. Their end-state assertions therefore do not isolate
CP263's final child. Only the seven reset-true Sequential calls exercise
the wrapper end to end without a duplicate distribution.

Named parent paths add 51 statically attributable wrapper executions, for
an audited total of 75: 58 reset true versus 17 false and 49 first versus
26 later iterations. None enables Space heat balance, so zero Space-demand
children are tested. No assertion proves any of the six unadjusted scalars,
a meaningful current-deadband copy, a nonzero moisture sequence, duct-loss
addition, Space traversal, uncontrolled or sizing behavior, an invalid
scheme, mismatched allocation, failure prefix, or replay repair. Three
moisture remaining assertions use zero inputs/defaults, and both sides of
the observed deadband copy are false.

Rust contains no exact or snake-case wrapper, child, distributor,
`FirstHVACIteration`, `ResetSimOrder`, `doSpaceHeatBalance`, or matching
reset-field implementation. Its `ZoneSysEnergyDemand` subset owns only a
Zone ID plus four heating/cooling/humidifying/dehumidifying
setpoint-remaining values. It has no total or unadjusted predictor fields,
six sequenced arrays, Zone/Space demand arenas, deadband state, shared
priority scratch, or distribution lifecycle.

The active compatibility runtime constructs one fresh four-field demand
snapshot per IdealLoads system from options and calls the prebound
PurchasedAir path directly. Execution-plan Zone-equipment labels are not
interpreted as CP263. The active data census has 30 equipment lists,
30 connections, and 30 IdealLoads systems but no Space, SpaceList, or
SpaceHVAC object. Every list is one-entry SequentialLoad at cooling/heating
sequence `1/1`, so current fixtures expose only the trivial `M=0` topology.

CP263 changes no Rust target/state, support declaration, test, capability,
output, comparator, case, manifest, numerical claim, performance claim, or
conformance status. Counts become 32 algorithms and 267 routines, split 58
`state_mapped` plus 209 `source_mapped`, with 144 required. Domain-required
counts become heat-balance 88, HVAC 33, plant 1, and time/schedule 22, with
readiness `0/88`, `0/33`, `0/1`, and `0/22`. The IdealLoads parent remains
`scaffold` at claim level `none`.

CP264 next adds required source-mapped `routine.init_output_required`
immediately after `routine.init_system_output_required` and before
`routine.sim_purchased_air`. `initOutputRequired` is declared at
`ZoneEquipmentManager.hh` lines 190-196 and implemented completely at
`ZoneEquipmentManager.cc` lines 4292-4388.
`DistributeSystemOutputRequired` begins at source line 4390.

## CP264 `initOutputRequired` Demand and Sequence Reset Leaf

CP264 adds canonical required `routine.init_output_required` immediately
after `routine.init_system_output_required` and before
`routine.sim_purchased_air`, plus the same ordered HVAC project-contract
item. The lowercase leaf is declared at `ZoneEquipmentManager.hh` lines
190-196 and implemented completely at `ZoneEquipmentManager.cc` lines
4292-4388:

```cpp
void initOutputRequired(
    EnergyPlusData &state,
    int const ZoneNum,
    DataZoneEnergyDemands::ZoneSystemSensibleDemand &energy,
    DataZoneEnergyDemands::ZoneSystemMoistureDemand &moisture,
    bool const FirstHVACIteration,
    bool const ResetSimOrder,
    int spaceNum);
```

Only the header supplies `spaceNum = 0`. The two demand arguments are
mutable references and can identify a Zone pair, a Space pair, or arbitrary
caller-owned records. `ZoneNum` still controls every shared Zone lookup.
`spaceNum` is otherwise unused: zero permits the reset-order child, while
any nonzero value, including malformed negative input, suppresses it.

The exact source order begins with 12 unconditional scalar restores. The
six sensible writes are:

- `RemainingOutputRequired` and `UnadjRemainingOutputRequired` from
  `TotalOutputRequired`;
- `RemainingOutputReqToHeatSP` and its unadjusted counterpart from
  `OutputRequiredToHeatingSP`; and
- `RemainingOutputReqToCoolSP` and its unadjusted counterpart from
  `OutputRequiredToCoolingSP`.

The six moisture writes repeat the same pattern for total,
humidifying-setpoint, and dehumidifying-setpoint demand. No clamp,
finite-value check, sign normalization, multiplier, or unit conversion is
performed.

If `ResetSimOrder && spaceNum == 0`, CP264 then calls CP262
`SetZoneEquipSimOrder(state, ZoneNum)`. The normal CP263 Zone call passes
the same state demand record as `energy`, so the preceding total-to-
remaining copy determines CP262's cooling-versus-heating sign. An arbitrary
direct caller can pass a different sensible record; CP262 still reads the
state-owned Zone demand, and no identity check ties the two together.

Every sequence mutation is then gated only by

```cpp
allocated(energy.SequencedOutputRequired)
```

The other two sensible and all three moisture sequences are assumed to
exist, and a zero-length allocated main vector still passes. The gate does
not check companion allocation, nonempty slot one, equal extents, active
equipment count, or the correspondence between Zone/Space demand and
`ZoneNum`.

For an uncontrolled Zone or while `ZoneSizingCalc` is true, the leaf
broadcasts predictor totals across all six independent destination
extents: sensible total, heating setpoint, cooling setpoint, moisture total,
humidifying setpoint, and dehumidifying setpoint. This branch ignores
`FirstHVACIteration`, load-distribution scheme, design load, duct loss, and
`spaceNum`.

For a controlled, nonsizing first HVAC iteration, the source reads the
parent Zone equipment-list scheme once:

- Sequential and Uniform perform the same six predictor-value broadcasts.
- UniformPLR and SequentialUniformPLR broadcast a design value into each of
  the three sensible sequences and predictor values into the three moisture
  sequences.
- Invalid, `Num`, or any other cast value matches neither branch and writes
  no sequence element.

Each of the three PLR sensible broadcasts independently tests only
`energy.TotalOutputRequired >= 0.0`. A nonnegative total, including positive
or negative zero, selects the parent
`FinalZoneSizing(ZoneNum).DesHeatLoad`; a negative total selects
`-DesCoolLoad`. NaN makes the comparison false and therefore selects
negative cooling design load. Heating- and cooling-setpoint sequence seeds
do not use their own setpoint-demand signs or magnitudes.

A Space call uses the passed Space demand sign but still reads the parent
Zone's controlled flag, equipment-list scheme, `FinalZoneSizing`, and
deadband, plus shared duct-loss state. It never reads `FinalSpaceSizing`.
Full broadcasts
fill each vector's own complete extent, including any tail beyond an active
equipment prefix.

For a controlled, nonsizing later HVAC iteration, scheme is not read. The
leaf writes only index one of all six sequences from the predictor totals
and setpoint totals. When `DuctLossSimu` is true, it then adds the same
`SysSen` value to the three sensible cells and the same `SysLat` value to
the three moisture cells. The additions are raw: no finite, sign, or
magnitude guard exists. Every sequence element above one retains its prior
value.

Finally, after all sequence handling, CP264 always writes

```cpp
CurDeadBandOrSetback(ZoneNum) = DeadBandOrSetback(ZoneNum);
```

This is a Zone-level destination even when the demand references identify a
Space. Repeated or duplicate Space calls therefore rewrite the same parent
Zone flag; there is no per-Space deadband state in this leaf.

The body has 11 `if` tokens, including two `else if`, six `else` tokens,
and no loop, switch, return, break, continue, diagnostic, catch, cleanup,
transaction, or rollback. It contains 46 direct persistent mutation sites:
40 plain assignments and six compound additions. One separate local initialization establishes the distribution-scheme
value. Its 24 syntactic
calls/accessors comprise CP262, `allocated`, the Zone and equipment-list
lookups, six final-sizing accesses, 12 index-one sequence accesses, and two
deadband accesses.

Those sites address 19 direct destination families: 12 scalar demand
fields, six sequence vectors, and one Zone deadband field. Baseline
successful work executes 13 assignments. An allocated recognized
full-initialization path executes six more statements, for 19. A later
duct-off path also executes 19; duct-on executes 25 because each of the six
slot-one cells is written and then incremented. An invalid first-iteration
scheme or unallocated main gate remains at 13.

Let `L` be the sum of the six independent vector extents. A full broadcast
performs `L` sequence-element writes in six statements, for `13+L` direct
destination writes overall. Later duct-off touches six sequence cells;
duct-on performs 12 operations on those same six cells. If CP262 runs, its
`2N + 4U + 6S` scratch-mutation count is additional.

There are exactly three production call expressions. Sizing
`sizeZoneSpaceEquipmentPart1` line 339 passes first iteration true, reset
false, and the current Zone or Space demand pair. CP263 calls the leaf once
for the Zone and once per stored Space occurrence. The latter wrapper
forwards its first/reset flags and uses reset true at both executable
production call sites.

Let `I` indicate Space heat balance, `C` be controlled Zones, `M` their
stored Space occurrences, `K` be controlled Zones with return nodes, and
`M_K` their stored Space occurrences. A normal or sizing manager pass
therefore reaches

`C + K + I*(M + M_K)`

CP264 calls. Normal simulation gets `C+I*M` through CP263 before equipment
and `K+I*M_K` through leaving conditions afterward; all wrapper calls pass
reset true, so CP262 runs for the `C+K` Zone children. Sizing gets the first
group directly through Part1 with reset false, then the return-node group
through CP263; only its `K` valid Zone children run CP262. `ZoneSizingCalc`
selects full sequence broadcasts only when the main sensible sequence is
allocated, and the following distributor returns.

No C++ test calls the lowercase leaf directly. The audited lower-helper
census finds 82 executions: 75 through CP263 and seven through sizing
Part1. Fifty-six are first-iteration and 26 later; 58 pass reset true and
24 false. Total sensible signs are 23 positive, 43 negative, and 16 exact
zero. Sequence shapes are eight unallocated, 48 length-one, two
length-two, 20 length-three, and four length-four cases. The allocated
schemes comprise 58 Sequential, four Uniform, four UniformPLR, and eight
SequentialUniformPLR executions.

The 24 explicit wrapper calls strongly assert all three sensible sequence
families across four schemes, both load signs, and first/later behavior.
First-iteration PLR tests directly preserve and verify positive heating or
negative cooling design-load seeds. Seventeen reset-false calls explicitly
run distribution again, so many end-state assertions cannot isolate CP264.
The tests containing the 51 named-parent executions assert downstream
equipment, airflow, and return results rather than the leaf's immediate
destinations. The tests containing the seven sizing executions likewise
assert downstream load, DOAS, and node behavior.

Across all 82 audited executions, no Space-HB child runs and duct loss is
always disabled. Eight unallocated cases execute the gate but have no
sentinel oracle. There are zero assertions for the three sensible
unadjusted fields, moisture unadjusted fields, any moisture sequence, or
`CurDeadBandOrSetback`. Only three moisture remaining assertions read
zero-valued defaults. The sizing calls include nonzero moisture and five
true deadband sources, but only downstream descendants observe them and
the destination copy is never asserted.

Tests also omit an uncontrolled or sizing call with allocated arrays, an
invalid scheme, the zero/NaN PLR sign boundary, mixed total-versus-setpoint
signs, empty or mismatched companions, invalid Zone/Space identity, missing
final sizing, partial failure, rollback, and deliberate replay
reconstruction. Twenty-six leaving-condition replay executions occur (20
first-iteration and six later-iteration), but none deliberately corrupts
all destinations first or distinguishes repaired fields from retained
tails.

There is no up-front validation, result status, diagnostic, catch,
checkpoint, cleanup, transaction, or rollback. A failure after entry
retains the 12 restored scalars. CP262 failure retains that scalar prefix
and any scratch prefix already mutated; sequence or final-sizing failure
additionally retains completed CP262 work and any prior sequence writes. A final deadband lookup failure follows every earlier
mutation. Malformed later companion vectors can fail partway through the
six slot-one writes. An allocated zero-length main vector reaches that
later indexing path, which can throw under bounds checking or be undefined
without it.

With all mutable dependencies fixed, successful replay overwrites every
destination it rewrites. Duct-loss additions do not accumulate because
slot one is restored before `+=`. Full broadcasts can repair current
vector extents, while later, unallocated, and invalid-first paths preserve
some or all sequence tails. CP262's upper priority tail also remains
history-dependent. Allocation/extents, Zone control, sizing flag, scheme,
demand, design loads, duct state, and deadband are resampled on every call,
so the leaf has no canonical whole-state repair protocol.

Rust contains no exact or snake-case CP264 leaf, total/unadjusted demand,
six sequenced arrays, Zone/Space demand arenas, predictor/current deadband,
final Zone sizing design loads, duct-loss state, or operational
first-iteration, reset-order, or Zone-sizing flags. Exact source-name hits are confined to metadata/comments for selected
remaining and predictor-output fields; none implements CP264 state.

Rust `ZoneSysEnergyDemand` owns only a Zone ID and four heating, cooling,
humidifying, and dehumidifying setpoint-remaining scalars. The moisture
predictor can compute total and setpoint loads, but only two setpoint loads
enter this snapshot. `Deadband` is an IdealLoads calculation-result mode,
not the source predictor/current deadband pair, and
`hvac_iteration_count` is initialized and report-copied rather than used as
this branch discriminator.

The active runtime creates a fresh independent demand snapshot for each
IdealLoads system and calls prebound PurchasedAir directly. It never reads
the stored load-distribution scheme. Active data contain 30 equipment
lists, 30 connections, and 30 IdealLoads systems, with zero Space,
SpaceList, SpaceHVAC, Sizing:Zone, or duct-loss object. Every list is
one-entry SequentialLoad at sequence `1/1`, and all 61 active
SimulationControl objects disable Zone sizing.

CP264 changes no Rust target/state, support declaration, test, capability,
output, comparator, case, manifest, numerical claim, performance claim, or
conformance status. Counts become 32 algorithms and 268 routines, split 58
`state_mapped` plus 210 `source_mapped`, with 145 required. Domain-required
counts become heat-balance 88, HVAC 34, plant 1, and time/schedule 22, with
readiness `0/88`, `0/34`, `0/1`, and `0/22`. The IdealLoads parent remains
`scaffold` at claim level `none`.

CP265 next adds required source-mapped
`routine.distribute_system_output_required` immediately after
`routine.init_output_required` and before `routine.sim_purchased_air`.
`DistributeSystemOutputRequired` is declared at
`ZoneEquipmentManager.hh` line 198 and implemented completely at
`ZoneEquipmentManager.cc` lines 4390-4419. Its leaf
`distributeOutputRequired` begins at source line 4421.

## CP265 `DistributeSystemOutputRequired` Gate and Zone/Space Dispatcher

CP265 adds canonical required
`routine.distribute_system_output_required` immediately after
`routine.init_output_required` and before `routine.sim_purchased_air`,
plus the same ordered HVAC project-contract item. The wrapper is declared
at `ZoneEquipmentManager.hh` line 198 and implemented completely at
`ZoneEquipmentManager.cc` lines 4390-4419:

```cpp
void DistributeSystemOutputRequired(
    EnergyPlusData &state,
    int const ZoneNum,
    bool const FirstHVACIteration);
```

The header has no default argument and spells the two by-value parameters
without top-level `const`; the definition adds function-type-neutral
top-level `const` to both. There is no Space-number parameter. `ZoneNum`
selects the Zone and equipment-list priority, capacity, and fraction
context. CP266 separately consumes manager-global `PrioritySimOrder`
scratch whose correspondence to that Zone is an unchecked upstream
invariant.

The gate order is exact:

1. read `Zone(ZoneNum).IsControlled` and return when false;
2. read `ZoneSizingCalc` and return when true;
3. on the first HVAC iteration, return unless the Zone equipment-list
   scheme is Uniform or Sequential;
4. call lowercase `distributeOutputRequired` for the Zone sensible and
   moisture demand records; and
5. after that Zone child returns, read `doSpaceHeatBalance`, visit every
   stored `Zone(ZoneNum).spaceIndexes` occurrence in order, and call the
   same child for each Space demand pair.

Short-circuit evaluation matters. A later iteration performs no
wrapper-level equipment-list lookup. A first-iteration Uniform call reads
the list once because the first inequality is false. Sequential reads it
twice, first rejecting Uniform and then accepting Sequential. UniformPLR,
SequentialUniformPLR, Invalid, `Num`, and arbitrary other enum casts also
read it twice and return silently. The scheme is not cached in a local
snapshot.

This preserves the CP264 protocol. First-iteration Sequential and Uniform
calls enter CP266 and redistribute CP264 predictor broadcasts.
First-iteration UniformPLR and SequentialUniformPLR return so CP264's
design-load sequence seeds remain available for capacity discovery. An
unknown first-iteration scheme also returns, while CP264 has no matching
sequence-write branch, so prior sequence state can survive without a
diagnostic. Later iterations pass every scheme to CP266; its invalid
default is fatal. Uncontrolled and Zone-sizing calls likewise leave the
CP264 result untouched.

Define

```text
G = IsControlled
    && !ZoneSizingCalc
    && (!FirstHVACIteration
        || scheme == Uniform
        || scheme == Sequential)
```

For current Space flag `I` and `M` stored Space occurrences, a fully
successful wrapper call dispatches

```text
G * (1 + I*M)
```

lower-leaf executions. The Zone always precedes every Space. Duplicate or
cross-listed Space identities are not deduplicated, so the same demand
record can be revisited. A Space child receives its own demand records but
the unchanged parent `ZoneNum`; it therefore reuses the parent equipment
list with its priorities, learned capacities, and fraction schedules, plus
the current manager-global priority scratch and shared duct-loss state.
Scratch-to-parent correspondence is not validated. The flag is
`doSpaceHeatBalance`, not the narrower simulation-only flag. No Space
control, ownership, identity, or membership validation exists.

The wrapper has four `if` statements, one range-for, three `return`
statements, two `&&` tokens, and no `else`, switch, case, break, continue,
catch, diagnostic, or assignment operator. It has two lower-leaf call
sites and ten syntactic calls/accessors: those two children, two Zone
lookups, two equipment-list lookups, and four Zone/Space demand lookups.
There is no direct persistent mutation, result status, local recovery,
cleanup, checkpoint, transaction, or rollback.

All successful writes belong to CP266. In dependency context, that leaf
targets 12 demand families per selected record: six sequence vectors and
six adjusted remaining-demand scalars. It does not rewrite predictor
totals, unadjusted demand, CP264 deadband state, equipment-list data,
learned capacities, or priority scratch.

CP265 performs no allocation or extent check. Sequential CP266 needs
priority slot one in scratch corresponding to the current Zone, a valid
equipment pointer, and slot one in all six sequences. Nonsequential paths
need compatible list priority/capacity extents and demand-vector coverage
through the active equipment range;
their common tail reads slot one. CP264's sole main-sequence allocation
test governs only CP264's own writes and does not make CP265 safe.

The only production call expression is CP263
`InitSystemOutputRequired` at `ZoneEquipmentManager.cc` line 4289, after
the Zone initializer and every `doSpaceHeatBalance`-selected stored
Space-occurrence initializer return. A normal `SimZoneEquipment` pass
reaches CP265 once for each `ZoneEquipConfig.IsControlled` Zone before
equipment and again for each such Zone with a return node during leaving
conditions. CP265 independently gates on `Zone(ZoneNum).IsControlled`;
ordinary input processing aligns the two flags, but the wrapper does not
validate that invariant. With upstream-controlled count `C`, return-node
upstream-controlled count `K`, Space flag `I`, their stored occurrence
counts `M` and `M_K`, this is nominally `C+K` wrapper calls. A later
valid-scheme, control-aligned pass can dispatch

```text
C + K + I*(M + M_K)
```

lower leaves. A first pass includes only the Sequential/Uniform subsets.
Sizing Part1 calls CP264 directly and never CP265. Sizing can later reach
CP265 through the `K` leaving-condition wrappers, but `ZoneSizingCalc`
makes every one return before a lower Zone or Space call.

The audited C++ unit corpus executes the public wrapper exactly 92 times:
75 through CP263 and 17 through direct public call expressions. The 75
comprise 24 explicit CP263 calls plus 51 named-parent executions. Sixteen
direct calls immediately repeat the CP263 child; the seventeenth follows
CP263 plus an explicit `SetZoneEquipSimOrder`. No unit test calls lowercase
`distributeOutputRequired` directly.

The 92 public calls divide as follows:

- 55 first-iteration and 37 later-iteration calls;
- 60 Sequential, eight Uniform, eight UniformPLR, and 16
  SequentialUniformPLR schemes;
- 31 positive, 48 negative, and 13 exact-zero total sensible loads; and
- one unallocated main sequence, 48 length-one, two length-two, 36
  length-three, and five length-four shapes at public entry.

Ninety-one calls are controlled; the lone unallocated public entry is
uncontrolled. All 92 are outside Zone sizing, Zone-level, and have
`doSpaceHeatBalance=false`. Thus the corpus executes the uncontrolled
return once, never executes the sizing return, and dispatches no Space
child.

Eight first-iteration PLR calls return at the scheme gate: four CP263
children and their four direct repeats, covering heating/cooling
UniformPLR and SequentialUniformPLR cases. One additional first-iteration
call returns at the uncontrolled gate. The other 83 calls dispatch the
Zone leaf. Those lower calls divide into 46 first and 37 later calls, with
59 Sequential, eight Uniform, four UniformPLR, and 12
SequentialUniformPLR schemes. Their entry shapes are 48 length-one, two
length-two, 28 length-three, and five length-four records. Every later PLR
case computes a positive PLR; no `plr <= 0` no-write branch runs.

The lone unallocated public entry occurs in
`CZoeEquipmentManager_CalcZoneLeavingConditions_Test`.
`ZoneEquipConfig.IsControlled` is true, but
`Zone(1).IsControlled` remains default false, so CP265 returns at its first
gate before reading the scheme or reaching CP266. Its assertions read
return temperature, not CP265 demand preservation. It is an unasserted
uncontrolled-gate execution, not an unallocated lower-leaf dispatch.

Six explicit distribution tests contain exactly 222 sensible sequence
endpoint assertions:

- Sequential: 36;
- Uniform: 36;
- UniformPLR: 36;
- SequentialUniformPLR: 72;
- mixed Sequential equipment: 24; and
- mixed Sequential equipment with fractions: 18.

They strongly cover sensible formulas, both load signs, Uniform active
heating/cooling counts, UniformPLR capacities, and
SequentialUniformPLR selection of one, two, or three heating units and one
or two cooling units. The four first-PLR scenarios contribute 36
no-op-gate assertions: the positive heating or negative cooling CP264
design seeds survive both the internal CP265 call and its direct repeat.

The mixed-fraction test is the clearest Sequential mutation evidence. Its
positive heating fraction 0.4 scales slot one while the first-call tails
remain at full demand; a later update applies the second equipment's 0.6
fraction. A cooling fraction 0.3 is configured but no negative
mixed-fraction call exercises it.

Sixteen scenarios contain exactly 48 assertions for the three adjusted
sensible `Remaining*` fields. Twelve nonsequential scenarios reflect
CP265 slot-one distribution. Two basic Sequential later cases are
indistinguishable from CP264 because their fraction is one, and two mixed
later cases are observed only after `updateSystemOutputRequired`.

There are zero moisture-sequence assertions. Only three moisture
`Remaining*` assertions exist, all reading zero, and every CP265 input has
zero moisture predictor demand. The 51 named-parent executions have no
direct CP265 destination assertion; their host tests check downstream
equipment, airflow, and return behavior.

Repeated direct calls make the positive distribution evidence less
isolated. Uniform and later-PLR assertions follow a second deterministic
distribution, so they prove the aggregate replay-stable result but cannot
identify which invocation repaired a destination. All four repeated direct
first-PLR calls are no-ops. The corpus contains 13 direct distributing
replays and 26 leaving-condition wrapper replays (25 dispatching and one
returning as uncontrolled), but no test corrupts all six sequence vectors
plus all six adjusted Remaining destinations between calls.

The single uncontrolled return has no demand-state no-op oracle, and
coverage omits a `ZoneSizingCalc` return oracle, every Space traversal and
duplicate-membership case, nonzero moisture, every moisture sequence
ratio, and duct loss. Uniform has no zero-available
fallback case. PLR paths omit zero or wrong-sign capacity, `plr <= 0`,
zero total, NaN/Inf, inconsistent priority/capacity, and malformed active
counts. Sequential fraction coverage omits cooling, out-of-range,
negative, NaN, and schedule-failure fractions.

Tests also omit an invalid first-iteration silent return, a later invalid
fatal, allocated-zero-length or mismatched companion arrays, an active
missing-priority-scratch failure oracle, invalid Zone/Space identity,
isolated partial failure, rollback, and replay after changed scheme,
capacities, priorities, fractions, allocation, or Space topology. The one
unallocated uncontrolled call returns before sequence and priority
prerequisites are needed; without a demand sentinel it proves neither
no-op preservation nor active malformed-state behavior.

Gate returns occur before any wrapper-owned mutation and carry no result
status. For an active call, both Zone demand accessor arguments must be
evaluated before the Zone child begins, but their relative evaluation
order is not specified by the call syntax. A Zone-child failure prevents
the Space flag and membership traversal and retains the child's mutation
prefix. Failure while acquiring the membership after a completed Zone
child retains the complete Zone result. A Space accessor or lower-child
failure retains the Zone, every completed prior occurrence, and any
current-leaf prefix; later occurrences are skipped.

The first-iteration invalid-scheme path is silent. The corresponding later
path reaches CP266 `ShowFatalError`. CP265 has no catch, local diagnostic,
checkpoint, cleanup, recovery, or rollback around either case.

With scheme, priorities, capacities, fractions, demands, duct state, and
membership fixed, successful active replay is overwrite-idempotent for the
destinations CP266 rewrites. Sequential duct additions do not accumulate
because the child assigns before adding, and duplicate Space occurrences
repeat the same fixed calculation. This is not canonical whole-state
repair: wrapper skip paths, lower nonpositive-PLR no-write paths, and
sequence tails outside the active range retain history. Control and sizing
flags, first-iteration state, scheme, priorities, capacities, fractions,
demand, duct state, Space flag, and membership are all resampled on every
call.

Rust has no exact or snake-case CP265 wrapper or CP266 leaf. It lacks
source total/unadjusted/sequenced sensible and moisture state, Zone/Space
demand arenas, `PrioritySimOrder`, heating/cooling list priorities,
available-equipment counts, learned capacity caches, and operational
first-iteration, sizing, Space-HB, or duct-loss state.

Rust parses all four load-distribution enums plus heating/cooling sequences
and optional Sequential fraction schedules. The active runtime reads none
of the schemes or fraction schedules. Sequence values serve static graph
validation and binding order, while runtime visits each independently
bound IdealLoads system with a fresh four-setpoint-value demand snapshot.
Per-system maximum capacity limits are component caps, not the source
equipment-list learned capacities used by PLR distribution.

Active data contain 30 equipment lists, 30 connections, and 30 IdealLoads
systems. Every list has one SequentialLoad entry at heating/cooling
sequence `1/1` with blank fraction schedules. There are zero Space or
SpaceList objects, multi-equipment lists, non-Sequential schemes, active
fraction schedules, Sizing:Zone objects, or duct-loss cases. All 61 active
SimulationControl objects disable Zone sizing. These fixtures expose only
the trivial single-equipment topology and cannot establish CP265 parity.

The roadmap still requires real `FirstHVACIteration` semantics, multiple
ZoneHVAC equipment load distribution, equipment-list order and sequences,
availability, residual-load updates, and shared adaptive system-timestep
state. CP265 is source-only dependency evidence for that work, not an
implementation checkpoint.

CP265 changes no Rust target/state, support declaration, test, capability,
output, comparator, case, manifest, numerical claim, performance claim, or
conformance status. Counts become 32 algorithms and 269 routines, split 58
`state_mapped` plus 211 `source_mapped`, with 146 required.
Domain-required counts become heat-balance 88, HVAC 35, plant 1, and
time/schedule 22, with readiness `0/88`, `0/35`, `0/1`, and `0/22`. The
IdealLoads parent remains `scaffold` at claim level `none`.

## CP266 `distributeOutputRequired` Equipment Load Distribution Leaf

CP266 adds canonical required `routine.distribute_output_required`
immediately after `routine.distribute_system_output_required` and before
`routine.sim_purchased_air`, plus the same ordered HVAC project-contract
item. Lowercase `distributeOutputRequired` is declared at
`ZoneEquipmentManager.hh` lines 200-203 and implemented completely at
`ZoneEquipmentManager.cc` lines 4421-4715:

```cpp
void distributeOutputRequired(
    EnergyPlusData &state,
    int const ZoneNum,
    DataZoneEnergyDemands::ZoneSystemSensibleDemand &energy,
    DataZoneEnergyDemands::ZoneSystemMoistureDemand &moisture);
```

There is no default argument. `state`, `energy`, and `moisture` are mutable
references; `ZoneNum` is a const by-value selector. The leaf receives
neither `FirstHVACIteration` nor a Space identity. CP265 has already
selected whether to call it and may pass either a Zone demand pair or a
Space demand pair, but CP266 always reads equipment-list context through
the unchanged parent `ZoneNum`.

The leaf first binds `ZoneEquipList(ZoneNum)` and switches on its
`LoadDistScheme`. For the formulas below, define:

```text
Q, QH, QC = sensible predictor total, heating-setpoint, cooling-setpoint
W, WH, WC = moisture predictor total, humidifying-setpoint,
            dehumidifying-setpoint
N         = NumOfEquipTypes
E         = max(N, 0), the number of loop iterations
D         = duct-loss flag
SS, SL    = manager-global sensible and latent system duct loss
```

CP266 directly mutates only 12 persistent demand families: the three
sensible and three moisture sequence vectors, plus the six adjusted
`Remaining*` scalars corresponding to their slot-one values. It does not
write predictor totals, unadjusted Remaining demand, deadband/setback
state, list priorities, available counts, learned capacities, fraction
schedules, or manager-global priority scratch.

The exact four scheme branches are Sequential, Uniform, UniformPLR, and
SequentialUniformPLR.

### Sequential

Sequential ignores `N`, available-equipment counts, list priority values,
and learned capacities. It reads
`PrioritySimOrder(1).EquipPtr`, then unconditionally evaluates both that
equipment's heating and cooling fraction schedule getters before choosing
one result:

```text
rh = SequentialHeatingFraction(state, EquipPtr)
rc = SequentialCoolingFraction(state, EquipPtr)
r  = Q >= 0 ? rh : rc
```

Both schedule pointers therefore must be valid even though only one
sampled value is used. Positive zero and negative zero select heating;
NaN makes the comparison false and selects cooling. No finite, sign,
range, or clamp check is applied to either fraction.

The branch first assigns sensible slot one from the chosen raw fraction,
then conditionally adds sensible duct loss with the same fraction:

```text
energy.total[1] = r*Q
energy.heat[1]  = r*QH
energy.cool[1]  = r*QC

if D:
    energy.total[1] += r*SS
    energy.heat[1]  += r*SS
    energy.cool[1]  += r*SS
```

It copies those three values to the adjusted sensible Remaining fields,
then performs the analogous moisture writes and copies:

```text
moisture.total[1] = r*W
moisture.humid[1] = r*WH
moisture.dehum[1] = r*WC

if D:
    moisture.total[1] += r*SL
    moisture.humid[1] += r*SL
    moisture.dehum[1] += r*SL
```

Equivalently, each final slot-one value is
`r*(predictor + D*duct_loss)`. Assignment precedes addition, so a fixed
successful replay does not accumulate duct loss. Exactly the six
slot-one sequences and six adjusted Remaining fields are touched; every
higher sequence slot survives. The dynamic persistent mutation count is
`12 + 6D`.

### Uniform

Uniform computes both ratios before selecting a load sign:

```text
rh = NumAvailHeatEquip > 0 ? 1 / NumAvailHeatEquip : 1
rc = NumAvailCoolEquip > 0 ? 1 / NumAvailCoolEquip : 1
```

For every raw equipment index `1..N`, nonnegative `Q` selects the heating
priority and `rh`; negative or NaN `Q` selects the cooling priority and
`rc`. A selected priority greater than zero receives the corresponding
ratio times each of `Q`, `QH`, `QC`, `W`, `WH`, and `WC`. A nonpositive
priority receives six zeroes. The available count is not checked against
the number of positive priorities, so zero or negative counts use ratio
one and inconsistent positive counts simply over- or under-distribute.

Uniform does not read capacity, fractions, priority scratch, or duct
loss. After the loop, the shared non-Sequential tail copies the six
slot-one sequences into adjusted Remaining. Thus its dynamic persistent
mutation count is `6E + 6`. When `N <= 0`, the loop is empty but the common
tail still requires slot one in all six vectors.

### UniformPLR

UniformPLR uses the sign of sensible total demand only. For nonnegative
`Q`, it sums `HeatingCapacity(i)` for entries whose heating priority is
positive; for negative or NaN `Q`, it sums `CoolingCapacity(i)` for
entries whose cooling priority is positive:

```text
A = sum(active signed capacity)
p = Q/A
```

The division occurs only when the aggregate has the expected sign:
`A > 0` for heating and `A < 0` for cooling. Otherwise `p` remains zero.
There is no per-slot capacity-sign validation, finite check, or clamp to
one.

When `p <= 0`, the branch breaks from the switch without changing any
sequence. The common non-Sequential tail still copies the historical
slot-one values into all six adjusted Remaining fields, so the source
comment's no-change behavior applies only to sequences. This path performs
six persistent writes. A NaN `p` does not satisfy `p <= 0` and therefore
takes the full-write path.

On the full path, every index `1..N` is written. A nonpositive selected
priority gets six zeroes. For an active entry with selected capacity `C`,
all three sensible sequences receive the same value:

```text
energy.total[i] = C*p
energy.heat[i]  = C*p
energy.cool[i]  = C*p
```

The moisture direction follows the sensible sign rather than the moisture
load sign. In heating mode:

```text
if QH != 0:
    moisture.total[i] = W  * C*p/QH
    moisture.humid[i] = WH * C*p/QH
else:
    moisture.total[i] = W*p
    moisture.humid[i] = WH*p
moisture.dehum[i] = 0
```

Cooling is symmetric with denominator `QC`, values `W` and `WC`, and
`moisture.humid[i] = 0`. Only exact zero selects the fallback; NaN or
infinite denominators enter ordinary division. The full dynamic mutation
count, including the common slot-one copy, is `6E + 6`.

`Q=+0` or `Q=-0` selects heating but yields `p=0`, so sequences remain
unchanged and only the common tail writes. `Q=NaN` selects cooling. If the
cooling aggregate is negative, `p` becomes NaN and the full path can
write NaNs; if the aggregate fails its sign gate, `p=0` and only the
common tail runs.

### SequentialUniformPLR

SequentialUniformPLR also selects heating for nonnegative `Q` and cooling
otherwise, then scans every raw index `1..N` without early termination.
A heating entry is a candidate when `HeatingCapacity(i) > 0 && A < Q`; a
cooling entry is a candidate when
`CoolingCapacity(i) < 0 && A > Q`.

For every candidate, capacity contributes to `A` only when the matching
priority is positive, but `numOperating` increments regardless of that
priority. Consequently the count is neither necessarily the number of
capacity contributors nor the last candidate's raw index. The subsequent
distribution does not replay the candidate positions: it treats the raw
prefix `1..numOperating` as operating. Wrong-sign or inactive entries can
therefore make the scan set differ from the distributed prefix.

If the final aggregate has the expected sign, `p=Q/A`; otherwise
`p=0` and `numOperating=0`. There is again no clamp. When `p <= 0`, no
sequence changes and the common tail still performs six Remaining writes.
On the full path, the raw operating prefix uses exactly the UniformPLR
energy and moisture formulas, including its active-priority test and
exact-zero denominator fallback. Every index after the prefix through
`N` is explicitly zeroed in all six sequences. The scan visits `E`
indices and the distribution-plus-zero pass visits another `E`; the
full dynamic persistent mutation count remains `6E + 6`.

For signed zero, the heating threshold `A < Q` is initially false and the
branch takes its sequence no-write path. For NaN, the cooling threshold
`A > Q` is always false, so it also takes the no-write path. This differs
from UniformPLR's possible full NaN write. SequentialUniformPLR reads
neither available counts, fraction schedules, priority scratch, nor duct
loss.

### Fatal default and shared tail

The default calls:

```cpp
ShowFatalError(
    state,
    "DistributeSystemOutputRequired: Illegal load distribution scheme type.");
```

Under normal fatal semantics it terminates before demand mutation. The
following `break` is nominally unreachable; if the fatal helper ever
returned, execution would continue to the shared tail.

Sequential performs its six Remaining copies inside its case and skips
the shared tail. Every other case, including either PLR sequence no-write
path, reaches a six-assignment tail in this exact order: sensible total,
moisture total, sensible heating, moisture humidifying, sensible cooling,
and moisture dehumidifying. Each value is read from sequence slot one.
No unadjusted field is changed.

The complete leaf has 32 `if` tokens, 21 `else` tokens and no
`else if`, eight `for` loops, one switch, four cases, one default, seven
breaks, no return, two `&&` tokens, and one ternary. Its 136 plain
assignment tokens divide into 104 direct persistent writes and 32 local
writes. Ten `+=` tokens divide into six persistent Sequential duct
additions and four local capacity sums; ten `++` tokens are local loop or
operating-count increments.

There are 110 direct persistent mutation sites across the 12 destination
families: Sequential contributes 12 assignments plus six additions,
Uniform contributes 24 assignments, UniformPLR contributes 28,
SequentialUniformPLR contributes 34, and the shared tail contributes six.
The 151 syntactic calls/accessors comprise 110 sequence-vector accesses,
26 capacity accesses, ten priority accesses, two fraction getters, and one
each for `ZoneEquipList`, `PrioritySimOrder`, and `ShowFatalError`.

There is no up-front validation that:

- `ZoneNum` owns a list;
- priority and capacity arrays cover `1..N`;
- the six sequence vectors have mutually compatible extents;
- every non-Sequential vector owns slot one even when `N <= 0`;
- Sequential priority scratch slot one belongs to this Zone and holds a
  valid equipment pointer;
- both Sequential fraction-schedule arrays cover the equipment pointer and
  both referenced schedule pointers are valid;
- available counts match active priorities; or
- capacity signs, totals, ratios, and schedule values are finite.

Depending on build and container behavior, a malformed access can assert,
throw, or become undefined behavior. CP266 has no result status, catch,
local diagnostic except the fatal default, checkpoint, cleanup,
transaction, rollback, or recovery.

The only production lowercase call sites are CP265's Zone call at
`ZoneEquipmentManager.cc` lines 4408-4409 and stored-Space call at
lines 4413-4416. No C++ unit expression calls lowercase
`distributeOutputRequired` directly. A Space execution receives its own
mutable demand pair but still uses the parent Zone's list, priority arrays,
learned capacities, available counts, fraction schedules, and the same
manager-global priority scratch and duct-loss state. Demand identity and
`ZoneNum` correspondence are unchecked, and duplicate Space occurrences
can overwrite the same record repeatedly.

Failure preserves the exact prefix already written. A list lookup failure
precedes all persistent mutation. Sequential scratch or fraction sampling
also precedes mutation, but a later vector access can preserve a prefix of
sensible base assignments, sensible duct additions, sensible Remaining
copies, moisture base assignments, moisture duct additions, and moisture
Remaining copies. Uniform can retain all prior equipment plus the current
equipment's energy-then-moisture prefix. PLR capacity-scan failure occurs
before sequence writes; its sequence no-write path can still fail partway
through the six shared-tail copies. A PLR full-path failure preserves all
prior equipment and a current assignment prefix. SequentialUniformPLR can
also preserve a completed operating prefix followed by only part of the
zeroed tail. CP265 adds the already-completed Zone and prior-Space prefixes
around those leaf-local effects.

With every mutable dependency fixed, a successful full-write replay is
overwrite-idempotent on the destinations it rewrites. Sequential duct
loss does not accumulate because base assignment precedes addition. This
is not canonical whole-state repair: a PLR nonpositive path preserves all
old sequences, Sequential preserves every slot above one, and other
full-write paths preserve slots above `N`. Scheme, sign, totals, priorities,
available counts, learned capacities, fraction current values, priority
scratch, duct state, extents, and parent Zone/Space membership are
resampled on every parent call.

The audited C++ corpus executes CP266 exactly 83 times: 20 leaves from 24
explicit CP263 calls after four first-PLR gates, 13 direct-public
distributing replays, and 50 named-parent leaves from 51 public executions
after one uncontrolled return. Although the leaf receives no iteration
flag, its parent context divides those calls into 46 first and 37 later
iterations.

The scheme census is 59 Sequential, eight Uniform, four UniformPLR, and
12 SequentialUniformPLR. Total sensible signs are 27 positive, 44
negative, and 12 exact zero. By scheme they are:

- Sequential: 15 positive, 32 negative, and 12 zero;
- Uniform: four positive and four negative;
- UniformPLR: two positive and two negative; and
- SequentialUniformPLR: six positive and six negative.

All 83 calls use Zone demand records, `doSpaceHeatBalance=false`, valid
list/scratch prerequisites, duct loss disabled, and zero moisture
predictors. Their compatible sequence/list shapes are 48 one-entry, two
two-entry, 28 three-entry, and five four-entry records. There is no
unallocated, empty, mismatched, malformed, uncontrolled, or active Space
leaf execution.

Of the 59 Sequential leaves, 57 sample a selected fraction of one. Exactly
two positive leaves in the mixed-equipment fraction scenario select the
first equipment's heating fraction 0.4. A configured cooling fraction 0.3
is never selected, while the second equipment's heating fraction 0.6 is
consumed later by CP267 rather than CP266. Duct additions never execute.

All eight Uniform leaves have `N=3`. Four positive calls divide by three
available heating entries and write all three slots. Four negative calls
divide by two available cooling entries, write slots one and two, and
zero inactive slot three. Both positive-count ratio branches execute on
every call; neither ratio-one fallback does. Across 24 loop iterations,
12 are active heating, eight active cooling, and four inactive cooling.

The four later UniformPLR leaves also have `N=3` and all compute a
positive PLR. Heating runs twice with capacities `[2000, 1000, 500]`,
aggregate 3500, load 1000, and `p=2/7`. Cooling runs twice with capacities
`[-1200, -800, -500]`, only the first two priorities active, aggregate
-2000, load -1000, and `p=0.5`; the third slot is zeroed. The 12
assignment iterations comprise six active heating, four active cooling,
and two inactive cooling. No aggregate-sign failure or sequence no-write
path executes.

The 12 later SequentialUniformPLR leaves cover each of six scenarios
twice:

```text
Q =  1000: numOperating=1, A= 2000, p=0.5
Q =  2100: numOperating=2, A= 3000, p=0.7
Q =  3600: numOperating=3, A= 3500, p=36/35
Q = -1000: numOperating=1, A=-1200, p=5/6
Q = -1500: numOperating=2, A=-2000, p=0.75
Q = -2500: numOperating=3, A=-2000, p=1.25
```

The final negative scenario increments `numOperating` for the third
priority-zero candidate without adding its capacity, then processes that
raw third prefix slot as inactive and writes zero. It is direct evidence
for the unconditional operating-count increment. Across all 36 scans,
the operating loops perform 24 iterations, 22 active and two inactive,
and the remaining-tail loops perform 12 iterations. One-, two-, and
three-unit operating counts each occur four times. Four executions exceed
PLR one and confirm the absence of a clamp; every execution still avoids
the nonpositive-PLR branch.

The six explicit distribution blocks contain 222 sensible sequence
assertions, but only 186 follow an actual CP266 leaf: 78 Sequential, 36
Uniform, 18 later-UniformPLR, and 54 later-SequentialUniformPLR. The other
36 assert CP264 design seeds after first-PLR CP265 gate returns. Each of
the three sensible sequence families therefore has 62 post-CP266
assertions.

There are 48 assertions over the three adjusted sensible Remaining
fields. Twelve non-Sequential scenarios directly reflect CP266 slot one;
two fraction-one Sequential later cases are indistinguishable from the
CP264 value; and two mixed later cases are observed only after CP267 has
already updated the residual. No test asserts a moisture sequence.
Exactly three adjusted moisture Remaining assertions exist, all zero
after later cooling UniformPLR calls with zero moisture inputs.

The 50 named-parent leaves have no immediate CP266 destination assertion.
Thirteen direct distributing calls and 25 leaving-condition calls form 38
actual distributing replays, but none corrupts all 12 destinations
between invocations. The assertions therefore demonstrate many successful
aggregate formulas without isolating rollback, canonical repair, or
history-dependent tails.

Coverage omits the fatal default, active malformed identity or extent,
zero equipment, allocated-empty vectors, companion-vector mismatch,
priority/capacity mismatch, aliasing, partial failure, and rollback. It
also omits every Space leaf, duplicate or cross-Zone Space membership,
partial Zone/Space failure, duct loss, and meaningful moisture.

Uniform never exercises a nonpositive available-count fallback. PLR tests
omit zero or wrong-sign aggregate capacity, a nonpositive PLR, NaN or
infinite load, wrong-sign or extreme per-slot capacity, exact-zero
sensible-setpoint denominator fallback,
inconsistent active count, and capacity mutation between replays.
Sequential tests omit negative, out-of-range, or NaN fractions, selected
cooling fraction, either schedule failure, and fraction-scaled duct loss.
The 12 Sequential exact-zero leaves select heating, including two
UnitHeater contexts with nonzero setpoint loads, but no direct CP266 demand
oracle isolates that edge.

Tests also omit dependency mutation between retries, failure retry,
sentinel preservation on every no-write branch, and canonical repair of
retained sequence tails. The lower leaf's absence of a
`FirstHVACIteration` parameter means the first/later census proves only
parent-selected entry context, not a CP266-owned iteration decision.

Rust has no exact or snake-case CP266 leaf. Its by-value
`ZoneSysEnergyDemand` snapshot contains only four combined sensible and
moisture setpoint Remaining scalars. It owns no source-shaped Zone/Space sensible or
moisture arena, predictor totals, unadjusted or total adjusted Remaining
fields, six sequence vectors, manager-global priority scratch,
heating/cooling priority arrays, available-equipment counts, signed
first-iteration learned capacity caches, or duct-loss state.

The adjacent third-order moisture predictor can compute a transient
moisture total, but closed-loop and CLI paths copy only humidifying and
dehumidifying setpoint loads into the PurchasedAir snapshot. That value
is neither a persistent source-shaped moisture demand record nor CP266
distribution state.

Rust parses all four load-distribution enums and both optional Sequential
fraction schedule identifiers, but runtime reads neither the scheme nor
the fractions. Non-Sequential variants stop at parser arms. The compiler
requires positive unique heating/cooling sequence numbers, whereas source
CP266 treats nonpositive priorities as inactive and does not reject
duplicates. Graph helpers use static heating-first order, and active
compatibility execution visits each typed IdealLoads system with a fresh
full demand snapshot rather than shared residual demand. A multi-equipment
list is only marked diagnostic-only and remains dispatchable, so each
system would independently receive the full snapshot. Component maximum
capacities are authored caps, not the source list's signed learned
capacity arrays.

The active corpus remains 30 equipment lists, 30 connections, and 30
IdealLoads systems. Every list has one `SequentialLoad` entry at
heating/cooling sequence `1/1` with blank fraction schedules. It contains
zero Space, SpaceList, SpaceHVAC, multi-equipment, non-Sequential,
active-fraction, `Sizing:Zone`, or `Duct:Loss:*` cases. All 61
SimulationControl objects disable Zone sizing. This topology would reduce
source CP266 to a one-slot, fraction-one, duct-off Sequential transition,
but Rust still owns neither that slot nor its total/remaining distribution
state.

The roadmap still requires Rust-owned `ZoneSysEnergyDemand`, removal of
oracle demand injection, real first-iteration and adaptive system-timestep
state, multiple-equipment distribution, equipment-list order and
availability, residual-load updates, and shared lifecycle state. CP266 is
source-only dependency evidence for that work, not an implementation
checkpoint.

CP266 changes no Rust target/state, support declaration, test, capability,
output, comparator, case, manifest, numerical claim, performance claim,
or conformance status. Counts become 32 algorithms and 270 routines,
split 58 `state_mapped` plus 212 `source_mapped`, with 147 required.
Domain-required counts become heat-balance 88, HVAC 36, plant 1, and
time/schedule 22, with readiness `0/88`, `0/36`, `0/1`, and `0/22`. The
IdealLoads parent remains `scaffold` at claim level `none`.

## CP267 `updateSystemOutputRequired` System Residual Update Leaf

CP267 adds canonical required
`routine.update_system_output_required` immediately after
`routine.distribute_output_required` and before
`routine.sim_purchased_air`, plus the same ordered HVAC project-contract
item. The leaf is declared at `ZoneEquipmentManager.hh` lines 205-212 and
implemented completely at `ZoneEquipmentManager.cc` lines 4717-4908:

```cpp
void updateSystemOutputRequired(
    EnergyPlusData &state,
    int const ZoneNum,
    Real64 const SysOutputProvided,
    Real64 const LatOutputProvided,
    DataZoneEnergyDemands::ZoneSystemSensibleDemand &energy,
    DataZoneEnergyDemands::ZoneSystemMoistureDemand &moisture,
    int const EquipPriorityNum = -1);
```

The default appears only in the header. There is no `FirstHVACIteration`,
Space identity, sizing, or duct-loss argument. The mutable demand references
may designate either Zone or Space records, while `ZoneNum` always selects
the control type and, for a controlled call, the parent Zone equipment-list
context.

Let

```text
S, L       = provided sensible and latent output
U, UH, UC  = sensible unadjusted total, heating-SP, and cooling-SP residuals
R, RH, RC  = corresponding sensible adjusted residuals
V, VH, VD  = moisture unadjusted total, humidifying-SP, and dehumidifying-SP residuals
M, MH, MD  = corresponding moisture adjusted residuals
P          = EquipPriorityNum
Z          = energy.NumZoneEquipment
N          = selected Zone equipment list NumOfEquipTypes
q          = P + 1
HP(P),CP(P)= manager-global heating and cooling priorities at scratch slot P
```

For an uncontrolled Zone, the routine applies these mutations in exact
source order:

```text
U  -= S; R  = U
UH -= S; RH = UH
UC -= S; RC = UC
V  -= L; M  = V
VH -= L; MH = VH
VD -= L; MD = VD
```

It then recomputes the parent Zone's `CurDeadBandOrSetback` according to
`TempControlType(ZoneNum)`:

- Uncontrolled writes false;
- SingleHeat writes `R < 1.0` through the source expression
  `(R - 1.0) < 0.0`;
- SingleCool writes `R > -1.0` through `(R + 1.0) > 0.0`;
- SingleHeatCool and DualHeatCool write `RH < 0.0 && RC > 0.0`; and
- an unknown control type retains the prior flag.

All comparisons are strict, so the SingleHeat boundary `R == 1.0`, the
SingleCool boundary `R == -1.0`, and either dual-setpoint zero boundary are
outside deadband. A NaN comparison is false; the dual expression also
retains C++ short-circuit order.

Only when `P >= 0`, the uncontrolled path attempts three independently
gated sequence-pair writes. The total sensible/moisture pair uses slot
`q` when `q <= Z`. The heating/humidifying pair uses slot `HP(P)+1` when
that value is at most `Z`, and the cooling/dehumidifying pair analogously
uses `CP(P)+1`. The sole `return` follows these writes. The gates impose no
lower bound, scratch bound, vector-extent check, or moisture-side equipment
count check.

For a controlled Zone with Sequential distribution, the leaf first
subtracts `S` from all three sensible unadjusted residuals and `L` from all
three moisture unadjusted residuals. If

```text
P >= 0 && P < N
```

it treats `q=P+1` as the next priority slot, reads
`PrioritySimOrder(q).EquipPtr`, and lazily evaluates exactly one fraction:

```text
r = energy.TotalOutputRequired >= 0.0
      ? SequentialHeatingFraction(state, nextSystem)
      : SequentialCoolingFraction(state, nextSystem)
```

Unlike CP266, the unselected getter is not called. The discriminator is
the original predictor total, not any updated residual or provided output.
Positive and negative zero select heating, while NaN selects cooling. The
raw fraction is neither clamped nor checked for finiteness, range, schedule
validity, or consistency with the selected equipment.

The valid-next branch writes

```text
R  = r*U;  RH = r*UH; RC = r*UC
M  = r*V;  MH = r*VH; MD = r*VD
```

and then copies those six adjusted values into the six sensible/moisture
sequence families at slot `q`. It does not check `q` against `Z` or any
sequence extent. If the valid-next predicate is false, it instead copies
the six updated unadjusted residuals directly into their adjusted partners
and writes no sequence slot. Both Sequential paths then run the same
thermostat/deadband switch described above.

Sequential ignores duct-loss state, available-equipment counts, learned
capacities, list priority values other than the manager scratch lookup,
and every nonselected fraction. Its next-equipment test uses only raw `N`;
it does not establish allocation, extent, or identity agreement among the
six sequence vectors, `energy`, `moisture`, the selected Zone list, and the
manager-global scratch arena.

The three controlled non-Sequential schemes—Uniform, UniformPLR, and
SequentialUniformPLR—share one body. They ignore `S` and `L` completely.
When `P < 0`, the body is a no-op. Otherwise it independently copies at
most three sequence pairs into the six adjusted residuals:

```text
q <= Z:
    R = sensible total sequence(q)
    M = moisture total sequence(q)
HP(P)+1 <= Z:
    RH = sensible heating sequence(HP(P)+1)
    MH = moisture humidifying sequence(HP(P)+1)
CP(P)+1 <= Z:
    RC = sensible cooling sequence(CP(P)+1)
    MD = moisture dehumidifying sequence(CP(P)+1)
```

This body does not mutate unadjusted residuals, sequence vectors, or the
deadband flag. A skipped pair retains its historical adjusted values. It
again uses only upper-bound tests: `P=0` can complete the total slot-one
pair and then fail at scratch slot zero, a negative priority plus one can
pass the upper check, and signed `+1` overflow is not guarded.

Any other controlled load-distribution value calls

```cpp
ShowFatalError(
    state,
    "UpdateSystemOutputRequired: Illegal load distribution scheme type.");
```

before a direct demand mutation. Under the fatal helper contract, no
mutation follows.
NaN or infinity in `S` or `L` propagates through the subtracting
uncontrolled and Sequential paths but is ignored by the controlled
non-Sequential body. A selected NaN or infinite fraction propagates through
all six valid-next products. The routine performs no division. Deadband
comparisons against NaN produce false results; an unknown thermostat type
preserves history rather than normalizing it.

With `d=1` for a recognized thermostat case and zero otherwise, and with
`t`, `h`, and `c` denoting successful total, heating, and cooling pair
gates, the successful dynamic direct-mutation counts are

```text
uncontrolled:                 12 + d + 2(t+h+c)
controlled Sequential next:  18 + d
controlled Sequential tail:  12 + d
controlled non-Sequential:    2(t+h+c)
```

The static body contains 58 direct persistent mutation sites over 19
families:

- 12 sites for six unadjusted residual families, each appearing in the
  uncontrolled and controlled-Sequential branches;
- 24 sites for six adjusted residual families, each appearing in four
  branch locations;
- 12 sites for six sequence families, each appearing in two branch
  locations; and
- ten deadband assignments across the two thermostat switches.

Those sites divide into 12 compound subtractions and 46 plain assignments.
By branch location, 23 are uncontrolled, 29 are controlled Sequential, and
six are in the shared non-Sequential body. The complete body has ten `if`
tokens, one `else`, three switches, 14 cases, three defaults, 15 breaks,
one return, one ternary, five `&&` tokens, no `||`, no loop, and one unary
`!`. Its 49 plain `=` tokens comprise the 46 persistent assignments and
three local initializations.

Under the established audit convention that counts Objexx indexing as a
syntactic accessor, the body has 48 calls/accessors: one Zone lookup, two
temperature-control lookups, ten deadband accesses, one Zone equipment-list
lookup, 13 priority-scratch accesses, two fraction getters, 18 sequence
vector accesses, and one fatal call.

The leaf assumes a valid `ZoneNum`, a matching controlled equipment list,
valid `P`/`q` scratch positions, a valid next `EquipPtr`, a valid selected
fraction array and schedule value, and independently valid indices in all
six sequence vectors. `energy.NumZoneEquipment` is the only count used to
authorize moisture-vector access. Controlled Sequential instead trusts
list `N` to authorize all six slot-`q` writes and never compares it with
`Z`. The demand references may belong to a different Zone or Space, and a
Space call deliberately reuses its parent Zone's control type, equipment
list, priority scratch, and deadband destination.

There is no result status, local validation, catch, checkpoint, cleanup,
transaction, or rollback. A Zone lookup failure precedes all work. On the
uncontrolled path, a thermostat lookup or switch failure follows the 12
ordered residual mutations. Later failures can retain that prefix, a
new deadband value, and completed total, heating, then cooling sequence
pairs. `P=0` can write the total pair before failing on scratch slot zero.

For a controlled call, the list lookup precedes mutation. A Sequential
scratch or fraction failure follows the six unadjusted subtractions;
adjusted and sequence failures retain their source-ordered prefixes, and
the thermostat/deadband work occurs last. The shared non-Sequential body
can retain the total pair, then heating pair, then cooling-pair prefix.
The invalid-scheme fatal performs no direct demand write.

Uncontrolled and controlled Sequential calls are intrinsically
non-idempotent because they subtract the provided output on every replay.
For fixed `S` and `L`, after `k` successful subtracting calls,

```text
U_k = U_0 - k*S
V_k = V_0 - k*L
```

with analogous setpoint fields. Retrying after a partial failure therefore
cannot reconstruct the intended one-call state without an external reset.
The non-Sequential branch is overwrite-idempotent only for the adjusted
pairs whose fixed gates succeed; skipped fields retain history, and it
never repairs unadjusted demand, sequences, or deadband state. Every replay
can resample scheme, list counts, scratch identities and priorities,
fraction values, predictor sign, provided outputs, sequence contents and
extents, and Zone/Space membership.

There are four direct production call expressions:

- `sizeZoneSpaceEquipmentPart1` calls the leaf after each optional DOAS
  prefix at `ZoneEquipmentManager.cc` line 404, using the default `P=-1`;
- the same sizing helper calls it at its final tail at line 596, again with
  default priority;
- `SimZoneEquipment` calls it after each dispatched equipment slot at lines
  4108-4114 with explicit priority; and
- `ZoneEquipmentSplitter::distributeOutput` calls it for each Space at
  `DataZoneEquipment.cc` lines 2224-2230 with the parent Zone number,
  Space demand references, and explicit priority.

The splitter call can overwrite the parent Zone's
`CurDeadBandOrSetback(ZoneNum)` from a Space residual. It does not verify
that the Space belongs to that Zone or that the parent scratch/list state
matches the supplied demand records. Repeated Space occurrences repeat the
same cumulative subtraction.
The bounded, statically attributable C++ unit corpus executes the leaf 80
times. This count excludes unbounded repeated passes hidden inside complete
`ManageSimulation` runs:

- 65 calls follow individual `SimZoneEquipment` Zone slots;
- two tests call the lowercase leaf directly;
- three calls come from the splitter for Space demand; and
- ten sizing calls comprise seven final-tail calls plus three DOAS-prefix
  calls.

All ten sizing calls see an uncontrolled Zone and default `P=-1`. The other
70 calls are controlled Sequential calls with explicit priority. Of those,
18 take the valid-next branch—16 equipment-slot calls plus both direct
calls—and 52 take the fallback—49 last equipment slots plus the three
splitter calls whose selected list count is zero. No controlled Uniform,
UniformPLR, SequentialUniformPLR, or invalid-default execution occurs.
With recognized thermostat cases, the corpus therefore performs exactly

```text
10*13 + 18*19 + 52*13 = 1148
```

direct mutation-statement executions in this leaf.

The two direct unit calls are positive-heating, `P=1`, `N=4` cases. One
selects fraction one and the mixed-fraction case selects heating fraction
0.6. Together they make 18 sensible sequence assertions and six adjusted
sensible `Remaining*` assertions. Only six sequence assertions target the
three slot-two values freshly written by the two CP267 calls; the other 12
prove retention of other slots. There is no direct assertion for any
unadjusted field, moisture sequence or adjusted moisture field, or
`CurDeadBandOrSetback`.

The three splitter executions subtract nonzero Space sensible outputs but
assert no Space demand destination. Sizing tests inspect downstream sizing
results rather than the immediate residual fields. The named-parent
UnitHeater case adds one final sensible `RemainingOutputRequired == 0`
observation after two equipment slots, but it does not isolate CP267 from
its callers and equipment calculations.

Coverage therefore omits every non-Sequential branch, the invalid-scheme
fatal, unknown thermostat retention, direct deadband boundaries at `R=1`,
`R=-1`, and dual zero, meaningful latent output assertions, cooling
fraction selection, negative/out-of-range/NaN/infinite fractions, and
NaN/infinite provided output. It also omits `P=0`, `P<-1`, oversized or
overflowing priority, negative scratch priorities, mismatched Zone/demand
identity, allocated-empty or inconsistent sequence extents, Space-demand
destination assertions, partial failure, rollback, and a direct cumulative
replay/drift oracle.

Rust has no exact or snake-case CP267 function. Its copied
`ZoneSysEnergyDemand` snapshot contains a Zone identity plus only four
heating, cooling, humidifying, and dehumidifying setpoint-remaining values.
It owns no total or unadjusted demand, six sequence vectors,
`NumZoneEquipment`, `PrioritySimOrder`, temperature-control array, shared
`CurDeadBandOrSetback`, or mutable Zone/Space demand arenas.

The compatibility runtime constructs a fresh complete demand snapshot for
each compiled IdealLoads system from fixed run options, passes that snapshot
by value, and discards it after dispatch. Equipment sensible and latent
outputs are never subtracted from a shared residual and never feed the next
system. Humidistat logic changes only a local copied moisture snapshot; its
local deadband enum is not the source's shared Zone deadband flag.

Rust parses all four load-distribution enums, positive unique heating and
cooling sequence numbers, and optional heating/cooling fraction schedule
identities, but runtime consumes none of those distribution or fraction
fields. A multi-equipment topology is marked diagnostic-only yet remains
dispatchable, with every system receiving the same full input snapshot
rather than a sequenced residual. Rust's authored IdealLoads capacity limits
are not the source manager's signed learned capacity state.

The active fixture census remains 30 equipment lists, 30 equipment
connections, and 30 IdealLoads systems. Every list has one SequentialLoad
entry at heating/cooling sequence `1/1` with both fraction schedules blank.
There are no active Space, SpaceList, SpaceHVAC, multi-equipment,
non-Sequential, `Sizing:Zone`, or duct-loss objects, and all 61 active
SimulationControl records disable Zone sizing.

Even that one-equipment topology requires source CP267: the sole equipment
is the last Sequential slot, so the fallback subtracts `S` and `L`, copies
all six updated residuals, and recomputes deadband. Rust still omits that
transition. The roadmap continues to require Rust-owned Zone system demand,
removal of oracle demand injection, operational first-iteration and adaptive
system-timestep state, multiple-equipment distribution, list ordering and
availability, and residual-load feedback.

CP267 changes no Rust target or state, support declaration, test,
capability, output, comparator, case, manifest, numerical claim,
performance claim, or conformance status. Counts become 32 algorithms and
271 routines, split 58 `state_mapped` plus 213 `source_mapped`, with 148
required. Domain-required counts become heat-balance 88, HVAC 37, plant 1,
and time/schedule 22, with readiness `0/88`, `0/37`, `0/1`, and `0/22`.
The IdealLoads parent remains `scaffold` at claim level `none`.

## CP268 `adjustSystemOutputRequired` Zone/Sequence Ratio Leaf

CP268 adds canonical required
`routine.adjust_system_output_required` immediately after
`routine.update_system_output_required` and before
`routine.sim_purchased_air`, plus the same ordered HVAC project-contract
item. The leaf is declared at `ZoneEquipmentManager.hh` lines 214-219 and
implemented completely at `ZoneEquipmentManager.cc` lines 4910-4931:

```cpp
void adjustSystemOutputRequired(
    Real64 const sensibleRatio,
    Real64 const latentRatio,
    DataZoneEnergyDemands::ZoneSystemSensibleDemand &energy,
    DataZoneEnergyDemands::ZoneSystemMoistureDemand &moisture,
    int const equipPriorityNum);
```

The header and definition agree. All three scalar inputs are top-level
`const` by value, while the two demand records are mutable lvalue
references. There is no `EnergyPlusData`, Zone or Space identity, default
argument, `FirstHVACIteration`, or `noexcept` qualifier.

Let `s=sensibleRatio`, `l=latentRatio`, and `p=equipPriorityNum`. The body
contains exactly these 12 mutations in source order:

```text
energy.RemainingOutputRequired                  *= s
energy.RemainingOutputReqToHeatSP               *= s
energy.RemainingOutputReqToCoolSP               *= s
moisture.RemainingOutputRequired                *= l
moisture.RemainingOutputReqToHumidSP            *= l
moisture.RemainingOutputReqToDehumidSP          *= l
energy.SequencedOutputRequired(p)                *= s
energy.SequencedOutputRequiredToHeatingSP(p)     *= s
energy.SequencedOutputRequiredToCoolingSP(p)     *= s
moisture.SequencedOutputRequired(p)              *= l
moisture.SequencedOutputRequiredToHumidSP(p)     *= l
moisture.SequencedOutputRequiredToDehumidSP(p)   *= l
```

Thus `s` scales three adjusted sensible residuals and the matching three
sensible sequence cells, while `l` independently scales the three adjusted
moisture residuals and matching moisture sequence cells. The routine does
not touch predictor totals, any unadjusted residual, another sequence slot,
`NumZoneEquipment`, list or priority scratch, capacities, fractions,
deadband state, node state, or a saved demand copy.

The static and successful dynamic mutation counts are both 12, spanning 12
families. Every mutation is `*=`; there is no plain assignment, other
compound assignment, increment, local variable, branch, switch, loop,
ternary, logical operator, break, or explicit return. Each compound
assignment reads and writes its destination once. Under the established
audit convention, the only six calls/accessors are the six sequence-vector
indexing expressions.

The leaf uses raw `p` for every sequence vector. It performs no `+1`
conversion and consults no equipment count. It assumes all six vectors are
allocated and independently contain the same index, and does not validate
the index lower or upper bound, vector extent agreement, demand ownership,
or Zone/Space identity. A nonpositive or oversized index is not rejected;
a zero ratio does not skip indexing.

Ratios are likewise used raw, without sign, range, or finiteness checks,
clamping, division, or diagnostics. A negative ratio reverses signs. A
finite value times positive or negative zero becomes signed zero; repeated
negative-zero scaling can alternate a zero sign. NaN propagates through
that ratio's six destinations. Infinity can produce signed infinity, while
zero times infinity can produce NaN. Finite multiplication preserves the
platform's ordinary rounding, overflow, and underflow behavior. Because
both ratios are captured by value, earlier demand mutations cannot change
the later multiplier.

There is no local validation, result status, diagnostic, catch, checkpoint,
cleanup, transaction, or rollback. All six adjusted scalar multiplications
complete before the first indexed sequence access. An invalid first
sequence access therefore leaves those six scalar mutations. A later
malformed vector retains the same scalar prefix plus every sequence
multiplication already completed in the documented order. Under ordinary
IEEE behavior, NaN and infinity propagate as values rather than failures;
a trap-enabled environment can expose an earlier numeric prefix.

For fixed ratios, a fixed priority index, and successful calls, replay
compounds rather than reconstructing state. In ideal real arithmetic, after `k` calls each
sensible destination is its initial value times `s^k`, and each moisture
destination is its initial value times `l^k`. A retry after partial failure
therefore scales the retained prefix again and cannot recover the intended
one-call state. Unity ratios and some zero/NaN fixed points are incidental
special cases, not a replay guarantee.
The only direct production call expression is in
`ZoneEquipmentSplitter::adjustLoads` at `DataZoneEquipment.cc` lines
2180-2184. The transitive production entry is the `SimZoneEquipment`
equipment loop at `ZoneEquipmentManager.cc` lines 3740-3743. It calls
`adjustLoads` only when Space heat-balance simulation is active, sizing is
false, and the current equipment owns a nonnegative splitter index. The
priority-loop value `EquipTypeNum` is passed unchanged as `p`.

The caller initializes `s=l=1` and selects its ratio protocol from the
splitter thermostat-control enum:

- Ideal returns before saving demand or calling CP268;
- SingleSpace, when its configured control-Space fraction is positive,
  independently sets each ratio to
  `(selected Space total Remaining / Zone total Remaining) / fraction`
  when the corresponding Zone total Remaining is nonzero;
- Maximum scans stored Spaces in order for the greatest strictly positive
  value of `max(setptLo-T1, T1-setptHi)`, then applies the same independent
  total-Remaining ratios when the winning index and fraction are positive;
  and
- an unknown/default enum retains unity ratios and still calls CP268.

Neither total-derived ratio distinguishes the setpoint residuals that it
subsequently scales. The caller does not clamp or validate finiteness,
ratio sign, or magnitude. SingleSpace trusts its control-space ordinal and
identity. Maximum's strict comparison retains the first winner and leaves
unity ratios when no Space has a positive deviation.

Immediately before the leaf, `adjustLoads` copies the complete Zone sensible
and moisture demand records into splitter save storage. A later successful
`distributeOutput` call restores those copies before each Space update for
non-Ideal control. That is an external caller protocol, not CP268 recovery:
`adjustLoads` has no catch, and a failure in this leaf leaves both the saved
copy and the torn live demand state.

The bounded C++ unit corpus executes CP268 exactly twice, both through
`SpaceHVACSplitterTest` in `ZoneEquipmentManager.unit.cc`. Its three direct
`adjustLoads` calls classify as follows:

- line 5057 uses Ideal control and returns above the leaf;
- line 5074 uses SingleSpace control with `p=1`, `s=-0.2`, and `l=1`; and
- line 5153 uses Maximum control with `p=1`, `s=-0.9`, and `l=1`.

For SingleSpace, the configured second splitter entry has fraction 0.5 and
refers to Space index 3, whose total sensible Remaining is `+10`, while the
Zone total is `-100`:

```text
s = 10 / (-100 * 0.5) = -0.2
```

The Zone moisture total is zero, so the guarded latent calculation leaves
`l=1`. This execution reverses the six sensible destination signs. Lines
5076-5081 assert all three adjusted sensible residuals and all three
sensible slot-one sequence values.

The intervening `distributeOutput` restores the saved original Zone demand
before each Space update, but it also passes each Space demand through
`updateSystemOutputRequired`. In this test the Zone equipment list retains
its default `NumOfEquipTypes=0`, every Space's unadjusted sensible fields
retain zero, `sysOutputProvided=-90`, and the Sequential valid-next guard
fails for `p=1`. The fallback therefore overwrites all three sensible
Remaining fields for Space indices 1, 3, and 2 with `+18`, `+45`, and `+27`,
respectively, from `0 - (-90 * fraction)`. Latent values remain zero because
`latOutputProvided=0`.

For Maximum, Space index 2 wins with `T1=16` and fraction 0.3. Its total
sensible Remaining is now `+27`, not the originally seeded `-40`, producing

```text
s = 27 / (-100 * 0.3) = -0.9
```

The Zone moisture denominator is still zero, so `l=1`. Lines 5154-5159
assert the same six sensible destinations after this second sign-reversing
scale.
Together the two calls execute 24 leaf mutation statements and have 12
immediate sensible assertions.

All moisture scalars and sequences remain zero and are unasserted. The six
Ideal-case assertions prove only the caller's early return. Six later
restoration assertions prove the surrounding `distributeOutput` protocol,
not CP268 replay. No named `SimZoneEquipment` unit path activates a
SpaceHVAC splitter, so there is no other statically attributable leaf
execution.

Coverage omits a direct lowercase call, an observable nonunit latent ratio,
priority other than one, multi-slot isolation, positive nonunit, zero, or
unity sensible scaling, signed zero, NaN, infinity, and malformed or
mismatched sequence
vectors. It also omits caller fallbacks for zero Zone demand, nonpositive
fraction, Maximum ties or no positive deviation, the invalid enum, partial
failure, rollback, and a fixed-ratio compounding replay oracle.
Rust has no exact or snake-case CP268 function. It has no SpaceHVAC
splitter, splitter thermostat-control enum, control-Space ratio protocol,
mutable Zone/Space demand arena, six sequence vectors, or equipment-priority
indexed demand state. Its copied `ZoneSysEnergyDemand` contains a Zone
identity plus only four heating, cooling, humidifying, and dehumidifying
setpoint-remaining values; it has neither total adjusted residuals nor the
six sequenced destinations that CP268 scales.

The compatibility runtime constructs a fresh demand snapshot for each
compiled IdealLoads system from fixed run options and passes it by value.
It neither scales a shared Zone demand before equipment dispatch nor
restores a splitter-owned snapshot afterward. Humidistat code mutates only
a local copied demand. Static sequence numbers and parsed distribution
metadata therefore do not supply this runtime transition.

The active fixture census remains 30 equipment lists, 30 equipment
connections, and 30 IdealLoads systems. Every list has one SequentialLoad
entry at heating/cooling sequence `1/1` with blank fraction schedules.
There are no active Space, SpaceList, SpaceHVAC, multi-equipment,
non-Sequential, `Sizing:Zone`, or duct-loss objects, and all 61 active
SimulationControl records disable Zone sizing. The production guard for
CP268 is therefore inactive throughout the Rust fixture lane.

The roadmap still requires Rust-owned Zone and Space system demand,
SpaceHVAC topology and thermostat-control semantics, operational equipment
priority and sequence state, shared residual-load mutation, and removal of
oracle demand injection. A four-value by-value snapshot cannot establish
parity for this 12-destination in-place ratio leaf.

CP268 changes no Rust target or state, support declaration, test,
capability, output, comparator, case, manifest, numerical claim,
performance claim, or conformance status. Counts become 32 algorithms and
272 routines, split 58 `state_mapped` plus 214 `source_mapped`, with 149
required. Domain-required counts become heat-balance 88, HVAC 38, plant 1,
and time/schedule 22, with readiness `0/88`, `0/38`, `0/1`, and `0/22`.
The IdealLoads parent remains `scaffold` at claim level `none`.

## CP269 `CalcZoneMassBalance` Iterative Zone/Air-Loop Flow Solver

CP269 adds canonical required `routine.calc_zone_mass_balance` immediately
after `routine.adjust_system_output_required` and before
`routine.sim_purchased_air`, plus the same ordered HVAC project-contract
item. `CalcZoneMassBalance` is declared at `ZoneEquipmentManager.hh` line
221 and implemented completely at `ZoneEquipmentManager.cc` lines 4933-5283:

```cpp
void CalcZoneMassBalance(
    EnergyPlusData &state,
    bool FirstHVACIteration);
```

The definition adds top-level `const` to the by-value Boolean, which does not
change the C++ function type. There is no default argument, result status, or
`noexcept`. `state` supplies every mutable Zone, Space, node, air-loop, mixing,
infiltration, environment, sizing, and diagnostic dependency.

There are exactly two production call expressions:

- `SizeZoneEquipment` calls it unconditionally at
  `ZoneEquipmentManager.cc` line 675 with `true`, after the complete
  controlled-Zone and optional-Space Part1 sizing sweep and before
  `CalcZoneLeavingConditions(state, true)` at line 677 and every Part2 entry;
  and
- `SimZoneEquipment` calls it at line 4186 with its incoming
  `FirstHVACIteration`, after Zone exhaust controls and exhaust-system
  simulation at lines 4182-4184 and before leaving conditions, whole-system
  duct loss, and return-path simulation at lines 4188-4192.

`ManageZoneEquipment` selects sizing or simulation at lines 157-161. Neither
parent adds a mass-balance-specific guard, so sizing reaches CP269 even when
there is no controlled Zone.

The body defines `IterMax=25` and `ConvergenceTolerance=1.0e-5`. Its first
persistent write clears `ZoneMassBalanceHVACReSim`. Before entering the
iterative body it performs two complete source-ordered passes:

1. every `AirDistUnit` whose `AirLoopNum` is positive additively contributes
   `MassFlowRateSup` to `SupFlow`, `MassFlowRatePlenInd` to `RecircFlow`, and
   three leakage terms to `LeakFlow`; these targets are not cleared locally,
   and the source comment relies on prior `InitZoneEquipment`; then
2. every primary air loop marked `isAllOA` receives `MaxOutAir=SupFlow`,
   `OAFlow=SupFlow`, and `OAFrac=1`.

The `do/while` body at lines 4979-5277 executes the following ordered
building pass:

- When `EnforceZoneMassBalance` is true, it first clears each air loop's
  `ZoneRetFlow`, `SysRetFlow`, and `ExcessZoneExhFlow`. For every controlled
  Zone it clears `ZoneInfiltrationFlag`,
  `IncludeInfilToZoneMassBal`, mass-conservation `RetMassFlowRate`, and Zone
  `ExcessZoneExh`; with Space heat balance enabled it also clears
  `ExcessZoneExh` for controlled stored Spaces.
- It snapshots the prior building mixing and return totals and zeros the
  current local totals.
- The main Zone pass uses numeric order without enforcement and
  `ZoneReOrder(ZoneNum1)` with enforcement. Uncontrolled Zones are skipped.
  The reorder array is trusted without local range, uniqueness, or ownership
  validation.
- Each controlled Zone clears `TotExhaustAirMassFlowRate` and calls its
  `setTotalInletFlows`. Under `doSpaceHeatBalance`, controlled stored Spaces
  call their own inlet-flow child, while uncontrolled Spaces receive
  `scaleInletFlows` from the Zone node to the Space system node using raw
  `fracZoneVolume`. The source explicitly leaves mass balance at Zone level.
- Exhaust-node mass flow is accumulated only when the global
  `AirflowNetworkNumOfExhFan` is zero. Any nonzero global AFN exhaust-fan
  count suppresses that direct node summation for every Zone.
- If `ZoneMassBalanceFlag(ZoneNum)` is true, the routine sums return-node
  flows for positive node identities. Iteration zero, `AdjustReturnOnly`,
  and `AdjustReturnThenMixing` start from the stored incoming mixing flow;
  later mixing-first passes instead derive
  `max(0, return + exhaust - inlet + source mixing)`. It then calls
  `CalcZoneMixingFlowRateOfReceivingZone` and forms net mixing as receiving
  minus source mass flow.
- Standard return flow is
  `inlet + net mixing - (exhaust - balanced exhaust)`. Without enforcement,
  a negative value becomes positive `ExcessZoneExh` and the return target is
  clamped to zero; a nonnegative value clears the excess. With enforcement,
  excess is always zero and the target is clamped nonnegative.
- `calcReturnFlows` is then called once for every controlled Zone visit.
  Under enforcement, inlet and exhaust mass-conservation fields are
  overwritten and the selected adjustment mode adds work as follows:

| Adjustment | Extra receiving-mixing calls | Extra return-flow calls | Infiltration child |
|---|---:|---:|---:|
| `AdjustMixingOnly` | 0 | 0 | once |
| `AdjustMixingThenReturn` | 0 | 1 | once |
| `AdjustReturnOnly` | 0 | 1 | once |
| `AdjustReturnThenMixing` | 1 | 2 | once |
| any other value | 0 | 0 | once |

The table excludes the conditional initial receiving-mixing call and the
unconditional first return-flow call. Every enforced controlled-Zone visit
reaches exactly one of the three static `CalcZoneInfiltrationFlows` call
sites. Each return-derived adjustment uses
`max(0, inlet - exhaust + net mixing)` and, only outside `DoingSizing`,
applies raw `min(..., AirLoopDesSupply)` before delegating return allocation.

For any other adjustment value, enforcement has already zeroed the
mass-conservation `RetMassFlowRate`. Local `ZoneReturnAirMassFlowRate` also
starts at zero, but an independently true `ZoneMassBalanceFlag` first adds
current flows from positive return-node identities. The unconditional baseline
`calcReturnFlows` result is not copied into either value, so the building
return total adds zero. In ordinary freshly initialized
`NoAdjustReturnAndMixing` topology, `SetZoneMassConservationFlag` leaves the
Zone flag unset and the fallback infiltration child receives zero; an
inconsistent true flag plus an unrecognized adjustment can pass the pre-summed
return-node flow instead.

After each controlled Zone calculation, the routine accumulates building
mixing and return totals. Every positive return-air-loop identity receives
that node flow in `ZoneRetFlow`; when `TotAvailAirLoopOA > 0`, it also receives
the Zone excess exhaust in proportion to
`MaxOutAir / TotAvailAirLoopOA`.

The next primary-air-loop pass computes
`adjusted=max(0, ZoneRetFlow-ExcessZoneExhFlow)`. A strictly positive
`ZoneRetFlow` produces `ZoneRetFlowRatio=adjusted/ZoneRetFlow`; otherwise the
ratio is one. It then clears `ZoneRetFlow` for reconstruction. A second Zone
pass always uses numeric Zone order, skips uncontrolled Zones, and multiplies
the flow at each return node whose identity is positive by its air-loop ratio
when that air-loop identity is positive. It then rebuilds air-loop plus per-Zone
return totals. Aliased or
repeated node identities are neither deduplicated nor validated.

The imbalance-warning path runs only when all of these are true:

- Zone mass-balance enforcement is false;
- ordinary sizing and HVAC sizing simulation are both false;
- warmup is false;
- `FirstHVACIteration` is false; and
- the Zone's sticky `FlowError` latch is false.

It first applies the strict `HVAC::SmallMassFlow` threshold to unbalanced
system outflow. Only when that passes does it subtract outdoor-air,
ventilation, and incoming-mixing mass flow and require a second strict
`unbalancedFlow > HVAC::SmallMassFlow` comparison. Only then does it convert
the remaining imbalance to volume using the current psychrometric Zone
density and `StdRhoAir` and apply the strict `HVAC::SmallAirVolFlow`
threshold. A reported imbalance emits one warning,
one timestamp, and four continuation messages before setting
`FlowError=true`, so later calls suppress the warning for that Zone.

From `Iteration > 0`, convergence is the sum of absolute changes in building
mixing and return flow. Strict residual `< 1.0e-5` clears
`ZoneMassBalanceHVACReSim` and breaks; equality does not converge, while any
failed comparison sets the flag true. Non-enforced execution breaks after
one building pass. Enforced execution therefore performs between two and 25
passes; exhaustion or a NaN residual leaves the re-simulation request true.
After loop exit every primary air loop receives the unclamped
`SysRetFlow = ZoneRetFlow - RecircFlow + LeakFlow`.

`FirstHVACIteration` affects only warning suppression. It changes no solver,
iteration, airflow, mixing, infiltration, return, excess, or convergence
arithmetic.

The function contains 37 direct persistent mutation sites over 21 normalized
state-path families: 29 plain assignments, seven `+=` sites, and one `*=`
site. Those families cover the re-simulation flag; air-loop supply,
recirculation, leakage, outdoor-air, return, excess, ratio, and system-return
state; Zone infiltration and mass-conservation state; Zone/Space equipment
excess and Zone exhaust totals; return-node flow; and the warning latch.
Mutations inside the inlet, scaling, mixing, return, infiltration,
psychrometric, and diagnostic children are additional.

Lexically the body has 38 `if` tokens, seven `else` tokens including one
`else if`, 14 `for` loops split 12 indexed plus two range loops, one
`do/while`, two `break`, and four `continue`. Thus there are 15 loop
constructs in total. There is no switch, ternary, explicit return, result
status, or catch.

Under the established non-accessor convention, its 19 operational/service
call sites are two `setTotalInletFlows`, one `scaleInletFlows`, two receiving
mixing, four return-flow, three infiltration, one density, and six diagnostic
calls. Nine `max`, three `min`, two `abs`, and four formatting sites are
counted separately.

The routine performs no complete up-front validation of reorder, extent,
Zone/Space ownership, node or air-loop identity, aliasing, density, finite
arithmetic, or child state. It has no checkpoint, cleanup, transaction,
catch, rollback, or retry repair. Failure retains every completed ordered
prefix: the initial re-simulation clear, partial AirDistUnit additions,
all-OA overwrites, enforcement resets, completed Zone/Space and air-loop
passes, return-node scaling, child mutations, and any diagnostics.

A warning failure occurs before `FlowError=true`, so retry can repeat a
partial message sequence; successful completion makes the latch sticky. A
failure before the final system-return loop suppresses that tail, while a
failure within it preserves the completed air-loop prefix.

Same-state replay is generally non-idempotent. The AirDistUnit prefix adds
again, non-enforced excess/return aggregation can reuse prior air-loop state,
return nodes are multiplied in place, mixing and infiltration children own
additional state, and warning output is sticky. Normal parent flow calls
`InitZoneEquipment` first and externally zeros six air-loop aggregates, but
that is not CP269-local recovery. Enforced per-pass resets repair only a
selected subset and cannot roll back an interrupted call.

The C++ unit sources contain exactly 19 literal direct calls, all with
`FirstHVACIteration=false`: three warning-threshold calls in
`ZoneEquipmentManager_CalcZoneMassBalanceTest`, two return-basis calls in
`CalcZoneMassBalanceTest3`, one enforced no-adjust call in
`HeatBalanceManager_ZoneAirMassFlowConservationData2`, three calls each for
`AdjustMixingOnly`, `AdjustReturnOnly`, `AdjustReturnThenMixing`, and
`AdjustMixingThenReturn`, plus one additional source-and-receiving-Zone
`AdjustMixingOnly` call.

Those direct calls split into five non-enforced and 14 enforced entries. The
enforced modes are one no-adjust, four mixing-only, three return-only, three
return-then-mixing, and three mixing-then-return.

The 19 direct calls have 204 post-call `EXPECT` or `ASSERT` macros. All six
`AdjustReturnThenMixing` and `AdjustMixingThenReturn` calls immediately run
`CalcAirFlowSimple` before their 89 macros, split 47 plus 42, so those
outcomes are not CP269-isolated. Remaining assertions cover warning presence
but not exact text or the `FlowError` latch, return-node flow, conservation,
mixing state, and infiltration.
The enforced tests consume stored `ZoneReOrder` values, but each fixture
exercises only one ordering and no test perturbs or compares permutations.

The exact bounded route-representative unit census is 72 CP269 route
entries, not 72 literal dynamic function invocations:

- 19 direct leaf calls;
- six direct `SizeZoneEquipment` parents;
- 13 directly attributable `SimZoneEquipment` parents; and
- 17 effective `ManageSizing` contexts, each contributing one representative
  sizing route and one representative later simulation route.

The unit sources contain 18 lexical `ManageSizing` calls. The
`WaterToWaterSimple` call at source line 1538 performs plant sizing only and
reaches no CP269 route. In each of the other 17 contexts, the Zone-sizing
route can repeat inside design-day and timestep `ManageHeatBalance` cadence.
The bounded representative total is 51 true and 21 false
`FirstHVACIteration` routes, 14 enforced and 58 non-enforced; none activates
Space heat balance. Beyond that bound, 56 completing `ManageSimulation`
contexts include 55 with Zones and 34 that also perform Zone sizing, but
environment, design-day, timestep, and HVAC convergence cadence prevents an
exact dynamic invocation count.

Coverage omits a direct iteration-count or re-simulation-flag assertion,
exact tolerance equality, 25-pass exhaustion, nonconvergence and NaN,
`FirstHVACIteration=true` warning suppression, the `DoingSizing` supply-cap
bypass, AirDistUnit prefix, uncontrolled-Zone and controlled/uncontrolled
Space variants, AFN exhaust suppression, all-OA setup, positive-available-OA
excess allocation and `ZoneRetFlowRatio`, final `SysRetFlow`, and
`Add`, `No`, or `MixingSourceZonesOnly` infiltration modes. Every direct
enforced call uses `Adjust` plus `AllZones`; input-only enum tests do not call
CP269. Malformed topology, failure/rollback, and unchanged-input replay are
also uncovered.

Rust has no exact or snake-case CP269 function, re-simulation flag,
`ZoneRetFlowRatio`, excess-exhaust allocation, mass-conservation arena,
`ZoneReOrder`, or flow-adjustment enum. The compatibility runtime instead
iterates prebound IdealLoads systems directly, constructs a fresh four-value
by-copy `ZoneSysEnergyDemand`, and calls PurchasedAir. Its execution plan
contains metadata labels for `SimZoneEquipment` and `SimPurchasedAir`, but no
mass-balance execution step.

The diagnostic `simulate_ideal_loads_node_state_projection` is a superficially
similar but explicitly non-parity path. It seeds a design/default supply flow
and assigns that same fixed flow to Zone-air and return-node records; it owns
no inlet/exhaust/mixing/infiltration or AirLoop aggregates, adjustment mode,
iteration, convergence, re-simulation latch, first-iteration warning gate, or
return allocation.

The Rust demand record, diagnostic `NodeStateStore`, and static AirLoop graph
skeleton therefore do not supply a shared mutable CP269 solver. Rust lacks
operational `AirLoopFlow` and AirDistUnit aggregates, Zone equipment flow
topology, Space heat-balance allocation, and the mass-balance lifecycle.

Across 120 unique data models, the census includes 30 equipment
lists, 30 equipment connections, and 30 IdealLoads systems. Every list is
one-entry `SequentialLoad` at heating/cooling sequence `1/1` with blank
fraction schedules. The census has zero `ZoneAirMassFlowConservation`,
air-distribution-unit, air-terminal, mixing, cross-mixing, infiltration, ventilation,
AirflowNetwork, duct-loss, `Sizing:Zone`, Space, SpaceList, or SpaceHVAC objects, and all 61
SimulationControl records disable Zone sizing. Three AirLoopHVAC skeletons
exist only in diagnostic/nonclaim, run-blocked cases; they are not CP269
execution.

All 30 IdealLoads equipment connections do have one nonblank inlet and one
nonblank return node, with blank exhaust. Their EnergyPlus oracle runs CP269's
ordinary non-enforced one-pass inlet-to-return bookkeeping during simulation;
the 61 sizing-disabled controls remove only the separate sizing parent call.
Rust dispatch validates list edges and inlet nodes but does not consume return
or exhaust topology, then invokes PurchasedAir directly. Existing System Node
Mass Flow Rate coverage is supply-node-only and explicitly excludes broad HVAC
flow balancing, so that oracle activity is not Rust parity evidence.

The roadmap still requires Rust-owned shared node and air-loop flow state,
Zone/Space equipment topology, simple-airflow and mass-conservation arenas,
ordered mixing/return/infiltration adjustment, convergence, lifecycle, and
diagnostic parity. Static graph metadata and isolated node mass-flow fields
cannot establish this iterative transaction.

CP269 changes no Rust target or state, support declaration, test, capability,
output, comparator, case, manifest, numerical claim, performance claim, or
conformance status. Counts become 32 algorithms and 273 routines, split 58
`state_mapped` plus 215 `source_mapped`, with 150 required. Domain-required
counts become heat-balance 88, HVAC 39, plant 1, and time/schedule 22, with
readiness `0/88`, `0/39`, `0/1`, and `0/22`. The IdealLoads parent remains
`scaffold` at claim level `none`.

## CP270 `CalcZoneInfiltrationFlows` Mass-Conservation Infiltration Leaf

CP270 adds canonical required `routine.calc_zone_infiltration_flows`
immediately after `routine.calc_zone_mass_balance` and before
`routine.sim_purchased_air`, plus the same ordered HVAC project-contract
requirement. It changes no EnergyPlus source inventory.

The canonical declaration is `ZoneEquipmentManager.hh` lines 223-226 and the
complete definition is `ZoneEquipmentManager.cc` lines 5285-5340. CP269 ends
at source line 5283 and line 5284 is blank. The exact interface is:

```cpp
void CalcZoneInfiltrationFlows(
    EnergyPlusData &state,
    int const ZoneNum,
    Real64 const &ZoneReturnAirMassFlowRate);
```

The function returns no status. `state` is the only mutable aggregate;
`ZoneNum` is a const by-value identity and the passed total return flow is a
const reference. There is no Space identity, first-iteration argument,
sizing argument, default, result object, or `noexcept` boundary.

There are exactly three production call expressions, all inside CP269 after
its `EnforceZoneMassBalance` guard:

- line 5118 serves `AdjustMixingOnly` and `AdjustMixingThenReturn`;
- line 5154 serves `AdjustReturnOnly` and `AdjustReturnThenMixing`; and
- line 5158 serves every remaining return/mixing adjustment value.

Exactly one site executes for every controlled Zone visit in every enforced
CP269 solver pass. Non-enforced CP269 never invokes CP270. Before each
enforced pass, CP269 clears that Zone's `ZoneInfiltrationFlag` and
`IncludeInfilToZoneMassBal`, but it does not clear mass-conservation
`InfiltrationMassFlowRate` or the selected infiltration object's
`MassFlowRate`. Its two-to-25-pass solver can therefore repeat CP270 two to 25
times per controlled Zone.

CP269 does not consume a CP270 result status or include infiltration state in
its building mixing/return convergence residual. The leaf's state is instead
available to the later simple-airflow calculation. The passed return-flow
value already reflects the selected CP269 return/mixing adjustment branch.

For an eligible Zone the local signed residual is computed in this exact
order:

```text
R = MixingSourceMassFlowRate - MixingMassFlowRate
  + TotExhaustAirMassFlowRate + ZoneReturnAirMassFlowRate
  - TotInletAirMassFlowRate
```

The leaf uses `ConvergenceTolerance = 0.000010`. Every threshold is strict;
no branch uses an inclusive comparison.

The outer branch matrix is source-ordered as follows:

- Exact `InfiltrationFlow::No` performs no persistent write after acquiring
  `MassConservation(ZoneNum)`.
- For any other treatment, `InfiltrationPtr <= 0` writes only conservation
  `InfiltrationMassFlowRate = 0`.
- With a positive pointer, eligibility is exactly `IsOnlySourceZone` or
  `InfiltrationForZones == AllZones`.
- Eligible `Adjust` with `abs(R) > 1.0e-5` sets the Zone infiltration flag,
  stores signed `R` in conservation state, sets the include marker to one,
  writes `R` to the infiltration object, and then clamps only that object to
  `max(0, R)`.
- Eligible `Adjust` when the comparison fails zeros conservation and object
  mass flow, but does not locally clear the flag or include marker.
- Eligible `Add` with `R > 1.0e-5` sets the same flag, conservation value, and
  include marker, then performs `object.MassFlowRate += R` without a clamp.
- Eligible `Add` when the comparison fails zeros only conservation mass flow;
  the object, flag, and include marker are retained locally.
- An ineligible `Adjust` Zone copies the current infiltration-object flow into
  conservation state, while an ineligible `Add` Zone zeros conservation flow.
- With a positive pointer, an invalid or otherwise unmatched non-`No`
  treatment reaches no treatment-specific persistent write.

Under `MixingSourceZonesOnly`, only exact `IsOnlySourceZone` qualifies. A Zone
that is both a source and a receiver does not qualify through a separate
source-and-receiving flag. `AllZones` makes the classification irrelevant.

The body contains two syntactic inner `else if (... == No)` arms. Both are
unreachable because the unchanged treatment already passed the outer
`... != No` guard. A nonpositive pointer is handled before this inner dispatch,
so even an unmatched non-`No` enum zeros conservation flow on that path.

`Adjust` deliberately separates signed conservation state from the physical
object. A residual below `-1.0e-5` sets the flag and include marker, retains the
negative value in conservation state, briefly writes it to the object, and
then clamps only the object to zero. `Add` accepts only a strictly positive
residual and can accumulate onto any preexisting finite or nonfinite object
value.

Exact positive or negative tolerance equality takes the comparison-false
branch. A NaN also fails both ordered comparisons: `Adjust` zeros conservation
and object flow, whereas `Add` zeros conservation only. Neither path emits a
diagnostic. Positive or negative infinity is not rejected; it follows the
corresponding strict-comparison and arithmetic path.

The function contains 17 direct persistent mutation sites over four normalized
state-path families: 16 plain assignments and one `+=`. Two of the plain
assignments are the unreachable inner-`No` sites. The families are:

- per-Zone `ZoneInfiltrationFlag` at two sites;
- mass-conservation `InfiltrationMassFlowRate` at nine sites;
- mass-conservation `IncludeInfilToZoneMassBal` at two sites; and
- selected infiltration-object `MassFlowRate` at three assignments plus one
  compound addition.

Lexically the body has 11 `if` tokens and eight `else` tokens, including four
`else if` tokens. It has no loop, switch, ternary, explicit return statement,
`break`, `continue`, diagnostic, result status, or catch. Under the established
non-accessor convention it has zero operational child or service calls. One
`std::abs` and one `max` mathematical site are counted separately.

The indexed accessor census is one `MassConservation`, two `ZoneEquipConfig`,
two `ZoneInfiltrationFlag`, and six `Infiltration` sites. The leaf performs no
density, volume-flow, schedule, psychrometric, node, or air-loop update.

There is no complete validation of `ZoneNum`, allocation, pointer upper bound,
Zone/object ownership, enum validity, aliasing, or finite arithmetic. The
`MassConservation(ZoneNum)` reference is acquired before the outer treatment
guard, so an invalid Zone identity can fail even for treatment `No`. A positive
`InfiltrationPtr` is trusted without checking its allocated upper bound.

The routine has no checkpoint, cleanup, transaction, rollback, retry repair,
or local failure diagnostic. An abnormal exit retains every completed ordered
write. On the active `Adjust` path, flag, signed conservation flow, and include
state commit before the first object write; interruption after the raw object
write and before the clamp can expose a negative object flow. Its zero path can
clear conservation before a failing object access.

The active `Add` path likewise commits flag, conservation flow, and include
state before its final object `+=`. Failure there retains that prefix; failure
later in CP269 after a successful addition can cause a parent replay to add the
same residual again.

For fixed dependencies, a successful active `Adjust` replay overwrites the
fields it reaches. It is not canonical whole-state repair: treatment `No`,
comparison-false, pointerless, ineligible, and invalid-enum paths intentionally
leave selected flag, include, conservation, or object values untouched.
Positive `Add` replay is directly non-idempotent because it compounds object
flow, including across CP269 solver passes.

CP269's per-pass flag/include clear is external setup rather than CP270-local
recovery. It does not clear conservation or object infiltration flow. Direct
leaf calls, malformed lifecycles, and abnormal parent re-entry can therefore
observe state that the ordinary parent sequence would have reset only in part.

The C++ unit sources contain zero direct calls to the leaf. The established
72-entry bounded CP269 route-representative census has only 14 entries that
enable enforcement and can reach CP270; all are direct `CalcZoneMassBalance`
test calls with `FirstHVACIteration=false`. The other 58 routes are
non-enforced and skip all three CP270 call sites.

The 14 enforced parent entries split into one no-return/mixing-adjustment
case, four mixing-only, three return-only, three return-then-mixing, and three
mixing-then-return cases. Every one configures infiltration treatment
`Adjust` and Zone selection `AllZones`.

Across one solver pass their controlled-Zone footprint is 29 visits:

- two visits from the no-adjustment parent;
- nine from mixing-only parents;
- six from return-only parents;
- six from return-then-mixing parents; and
- six from mixing-then-return parents.

Twenty-seven visits have a positive infiltration pointer and two have zero.
The topology split is 14 source-only, one source-and-receiving, and 14
receiving-only Zones. CP269's guaranteed two-to-25 enforced passes bound these
successful test executions between 58 and 725 literal CP270 calls. The exact
iteration total is not instrumented.

The tests contain 28 post-parent infiltration assertions. Twenty-seven read
mass-conservation `InfiltrationMassFlowRate`, split six positive and 21 zero;
one reads an infiltration object's positive `MassFlowRate`. They contain zero
assertions on `ZoneInfiltrationFlag` or `IncludeInfilToZoneMassBal`, and zero
assertions on object zeroing or the negative-residual clamp.

Six return/mixing parent calls execute
`CalcAirFlowSimple(state, 0, true, true)` after CP269 and before 12 of those
conservation assertions. That child consumes CP270 flag/object state without
overwriting the conservation scalar, so the scalar checks remain observable
but the surrounding outcome is integration-level rather than leaf-isolated.
The two zero-pointer visits are also incidental and are not isolated from
other parent branches.

Behavioral coverage therefore omits direct leaf entry, `Add`, outer `No`, and
`MixingSourceZonesOnly`; the latter modes have input-enum tests only. It also
omits source-and-receiving exclusion under source-only scope, negative active
`Adjust`, exact positive and negative tolerance equality, near-zero clearing,
NaN, infinity, invalid modes, and an oversized positive pointer.

No test isolates flag/include lifetime, underlying-object clearing, additive
iteration accumulation, fixed-input replay, malformed Zone or object identity,
partial failure, rollback, or retry. The positive `Add` `+=` path and the
outer-`No` stale-state no-op have no runtime oracle.

Rust has no exact or snake-case CP270 function, typed
`ZoneAirMassFlowConservation`, mass-conservation arena, infiltration object
flow, `InfiltrationFlow`, `InfiltrationZoneType`, eligibility classification,
pointer, flag, or include-marker lifecycle. It also has no runtime residual
that combines mixing, exhaust, return, and inlet mass flow.

Typed `ZoneEquipmentConnection` metadata retains Zone-air, inlet, exhaust, and
return names, but compatibility dispatch validates and consumes only the
equipment-list edge and inlet before calling PurchasedAir with a fresh demand
snapshot. The diagnostic `NodeStateStore` projection assigns a fixed supply
flow to supply, Zone-air, and return records; it ignores exhaust and owns no
infiltration-balance transition.

`DesignSpecification:OutdoorAir` and demand-controlled-ventilation state are
PurchasedAir outdoor-air inputs, not mass-conservation infiltration state.
Run-blocked AirBoundary mixing metadata and the hard-coded zero
Zone outdoor-air-transfer report are also not CP270 implementations. Raw
`ZoneAirMassFlowConservation`, infiltration, mixing, cross-mixing, ventilation,
and AirflowNetwork inputs remain unsupported or run-blocked for arbitrary
runtime.

Across 120 unique data models, split 108 IDF and 12 epJSON, the census contains
zero `ZoneAirMassFlowConservation`, all three infiltration families, mixing,
cross-mixing, both ventilation families, and AirflowNetwork topology. Thirty
models contain one IdealLoads system, list, and equipment connection. Every
connection has nonblank inlet and return names and a blank exhaust name.

All 61 `SimulationControl` records disable Zone sizing, including all 30
IdealLoads models. More importantly, no model enables Zone mass-balance
enforcement. EnergyPlus therefore executes ordinary non-enforced CP269
bookkeeping during those simulations and reaches CP270 zero times. Rust's
direct PurchasedAir route supplies no alternative evidence.

The roadmap still requires Rust-owned Zone and infiltration identities,
mass-conservation state, mixing/exhaust/return/inlet aggregates, treatment and
Zone-selection enums, threshold behavior, additive lifecycle, and
failure/replay semantics. Static node metadata cannot establish this mutable
leaf.

CP270 changes no Rust target or state, support declaration, test, capability,
output, comparator, case, manifest, numerical claim, performance claim, or
conformance status. Counts become 32 algorithms and 274 routines, split 58
`state_mapped` plus 216 `source_mapped`, with 151 required. Domain-required
counts become heat-balance 88, HVAC 40, plant 1, and time/schedule 22, with
readiness `0/88`, `0/40`, `0/1`, and `0/22`. The IdealLoads parent remains
`scaffold` at claim level `none`.

## CP271 `CalcZoneLeavingConditions` Return-Node State Projection

CP271 adds canonical required `routine.calc_zone_leaving_conditions`
immediately after `routine.calc_zone_infiltration_flows` and before
`routine.sim_purchased_air`, plus the same ordered HVAC project-contract
requirement. It changes no EnergyPlus source inventory.

`CalcZoneLeavingConditions(EnergyPlusData &, bool FirstHVACIteration)` is
declared at `ZoneEquipmentManager.hh` line 240 and implemented completely at
`ZoneEquipmentManager.cc` lines 5342-5543. The definition adds only
function-type-neutral top-level `const` to the by-value Boolean. It has no
default argument, status result, or exception specification. CP270 ends at
source line 5340; CP272 `UpdateZoneEquipment` starts at line 5545.

There are exactly two executable production call expressions:

- `SizeZoneEquipment` calls the leaf unconditionally at line 677 with `true`,
  after the complete Part1 sizing sweep and CP269 mass balance and before any
  Part2 sizing entry; and
- `SimZoneEquipment` calls it unconditionally at line 4188 with its incoming
  `FirstHVACIteration`, after Zone exhaust controls, exhaust-system simulation,
  and CP269 and before whole-system duct loss and return-path simulation.

`ManageZoneEquipment` selects one of those parents. Neither caller adds a
controlled-Zone or return-topology guard. `FirstHVACIteration` is not sampled
by the leaving-state arithmetic; its only body use is forwarding the value to
the final demand-initialization child.

Before the numeric Zone pass, the leaf tests
`doSpaceHeatBalanceSimulation && !DoingSizing`. When true, it range-visits
every stored `zoneReturnMixer`, without filtering by controlled Zone or return
node, and invokes these three methods in order for each occurrence:

1. `setInletFlows`;
2. `setInletConditions`; and
3. `setOutletConditions`.

The following numeric Zone loop skips every uncontrolled configuration and
every controlled configuration with zero return nodes. A skipped Zone also
skips the final demand initializer. For each entered Zone, `ZoneMult` is the
product of `Multiplier` and `ListMultiplier`. Each return-node visit forms
`MassFlowRA` from return-node mass flow divided by `ZoneMult`, then adds the
full mapped exhaust-node mass flow only when the exhaust identity and its flow
are positive. The exhaust contribution is not divided by the multiplier.

The return-air base temperature is selected in this precedence order:

1. the already-written return-node temperature when Space heat-balance
   simulation is active, sizing is inactive, and
   `returnNodeSpaceMixerIndex > -1`;
2. allocated, active room-air-pattern `Tleaving` when `!BeginEnvrnFlag`; or
3. the Zone node temperature.

Every return-node visit first calls the sensible return-air convection-gain
sum. When the Zone reports an airflow-window return, the same visit then scans
all stored Zone Space identities and every Surface in each Space's inclusive
heat-transfer range. A Surface contributes only for positive current gap flow
and return-air destination. Its mass is
`rho(OutBaroPress, gap-outlet temperature, Zone-node humidity ratio)` times
current gap flow and Surface width; mass and mass-times-temperature are
accumulated to form a positive-flow mixture temperature.

That complete Zone-level airflow-window scan is inside the return-node loop.
Multiple return nodes therefore recompute and apply the same unpartitioned
aggregate once per node. There is no per-return-node window ownership, fraction, or filter, and no
sorting or deduplication. Density also uses Zone-node humidity even
when the base temperature came from a Space mixer or room-air pattern.

With `NoHeatToReturnAir=false`, the leaf computes moist-air specific heat from
Zone-node humidity. Positive `MassFlowRA` follows this ordered sensible path:

- positive airflow-window mass is blended with the selected base temperature
  when return mass is at least the window mass;
- when window mass exceeds return mass, return temperature becomes the window
  mixture and the excess window sensible term is added to
  `SysDepZoneLoads`;
- sensible return-air gain is divided by return mass and specific heat and
  added to the working return temperature;
- the return node is clamped to `HVAC::RetTempMin` and `HVAC::RetTempMax`;
  outside `ZoneSizingCalc`, but not during it, the clipped energy is added to
  `SysDepZoneLoads`; and
- with a positive mapped exhaust flow and positive sensible return gain, an
  exact `Shared` configuration adds only the gain temperature rise to the
  existing exhaust temperature, while every other configuration overwrites
  exhaust temperature with the unclamped working return temperature.

Nonpositive or NaN `MassFlowRA` takes the other branch. Positive window mass
contributes its signed sensible term, positive sensible return gain moves to
`SysDepZoneLoads`, and return
temperature is forced to Zone-node temperature rather than the selected Space
or room-air base. With `NoHeatToReturnAir=true`, return temperature is also
forced to the Zone value, but the locally computed sensible return gain and
window heat are not transferred by this branch.

Only exhaust-node temperature is changed. Exhaust pressure, humidity,
enthalpy, CO2, and generic contaminant are not synchronized. Return pressure
always copies Zone-node pressure, regardless of the heat branch.

Humidity handling follows temperature work. With heat-to-return enabled and
positive return mass, the leaf calls the node-specific latent return-gain sum,
computes water-vapor enthalpy, and sets return humidity ratio to Zone humidity
plus latent gain divided by vapor enthalpy and mass. Every other path copies
Zone humidity, adds the Zone's full `LatCaseCreditToHVAC` to
`LatCaseCreditToZone` without clearing the HVAC credit, calls the same latent
sum, and adds that result to the Zone heat-balance `latentGain`. These Zone
additions repeat for every return node.

The leaf always recomputes return enthalpy from final temperature and humidity.
It conditionally copies Zone-node CO2 and generic contaminant when their global
simulations are active. After all return nodes complete, it calls
`InitSystemOutputRequired(state, ZoneNum, FirstHVACIteration, true)` exactly
once for that entered Zone. The explicit final `true` requests simulation-order
reset; this child is the only consumer of `FirstHVACIteration` in CP271.

The body has 26 `if` tokens, 11 `else` tokens including two `else if` tokens,
three indexed and two range loops, and two `continue` statements. It has no
`while`, `switch`, ternary, explicit return, `break`, diagnostic, status,
checkpoint, catch, transaction, rollback, or cleanup.

There are 23 direct persistent mutation sites over nine normalized state
families: 13 plain assignments and ten compound additions. The families are
five `SysDepZoneLoads` additions; seven node-temperature sites across return
and exhaust roles; one return-pressure site; three return-humidity sites; two
refrigeration Zone-credit additions; two Zone latent-gain additions; and one
each for return enthalpy, CO2, and generic contaminant. Mixer and demand-child
mutations are additional.

Under the established census convention, the leaf owns 12 operational service
call sites: three mixer methods, four internal-gain queries, four
psychrometric functions, and one demand initializer. One allocation predicate
and 67 indexed accessors bring the complete syntactic call/accessor expression
count to 80.

No complete Zone, return, exhaust, mixer, Space, Surface, ownership, alias,
array-extent, finite-value, multiplier, or denominator validation precedes
mutation. A mixer failure retains completed methods and mixers and prevents
all Zone work. A Zone or return-node failure retains earlier Zones, nodes, and
the current-node prefix; temperature and load writes may survive without the
later humidity, enthalpy, or contaminant writes. A final demand-child failure
retains every leaving-node write for that Zone plus the reached child prefix.
There is no local repair protocol.

Same-state replay is generally non-idempotent. Ten `+=` sites can compound
system-dependent load, shared exhaust temperature, refrigeration Zone credit,
and Zone latent gain. `LatCaseCreditToHVAC` is not cleared after transfer, and multiple return nodes
can repeat the same Zone-level credit and
unpartitioned airflow-window aggregate. Repeated or aliased return, exhaust,
Space, and Surface identities are order-dependent. Plain node fields may
reconstruct some fixed paths, but optional exhaust and contaminant branches can
leave old values when disabled. Mixer and demand children add their own replay
semantics. The earlier `SimZoneEquipment` clear of `SysDepZoneLoads` is parent
setup, not CP271-local recovery.

The bounded C++ route-representative census contains 54 leaf entries: one
direct call, 23 sizing routes, and 30 simulation routes. It is composed of six
direct `SizeZoneEquipment` parents, 13 directly attributable simulation routes,
and 34 size/simulation projections from 17 effective `ManageSizing` contexts.
The 18th lexical sizing context is plant-only and does not reach Zone sizing.
The flags split 52 true and two false; both false entries are simulation
routes. Fifty-six completing `ManageSimulation` tests have runtime-dependent
HVAC cadence, so this bounded census does not invent an exact dynamic call
count.

All six direct sizing parents configure zero return nodes. They call CP271 but
skip its node and final-demand work. Other parent tests can contain return
nodes, availability, air-terminal mixer, or plenum behavior, but none owns a
CP271-specific complete return-node-state oracle. Their results are confounded
with equipment and parent simulation.

The C++ unit sources contain exactly one literal direct leaf call, in
`CZoeEquipmentManager_CalcZoneLeavingConditions_Test` at
`ZoneEquipmentManager.unit.cc` line 4480. It uses one controlled equipment
configuration, two positive-flow return nodes, one shared positive-flow
exhaust node, `NoHeatToReturnAir=false`, and 50 W then 100 W sensible return
gains. A non-Shared write followed by a `Shared` addition is checked with five
post-call expectations: preserved Zone temperature, two return temperatures,
one exhaust temperature, and a relation between the two return rises and the
exhaust rise.

That test does not activate a Space return mixer, room-air pattern,
airflow-window return, no/negative flow, clamp, refrigeration latent gain,
contaminant, or `NoHeatToReturnAir=true` branch. It asserts no return pressure,
humidity, enthalpy, latent state, system-dependent load, or demand reset. Its
`Zone.IsControlled` field remains false, so the final distribution wrapper
returns at its first gate; the meaningful first/later-iteration tail difference
is not isolated. No test covers malformed state, aliasing, partial failure,
rollback, replay, or repair.

Rust has no exact or snake-case CP271 function. Active compatibility classes
validate a connection and construct a fresh four-value sensible/moisture demand
snapshot before calling PurchasedAir directly. They never execute the leaving
projection or its reset-true demand tail and own no total, unadjusted, six-way
sequence, deadband, or Zone/Space demand arena.

Typed `ZoneEquipmentConnection` records retain inlet, exhaust, Zone-air,
return, return-fraction schedule, and return-basis names, and the compiler
registers those node references. Active dispatch resolves and consumes only
the supply/inlet edge. It does not use return/exhaust pairing, return basis or
schedule, Shared/Multi configuration, Space mixer identity, or Zone
multipliers for CP271 arithmetic.

`IdealLoadsNodeStateProjection` is a separate explicitly diagnostic,
non-parity path. Its `AirNodeState` contains temperature, humidity ratio, mass
flow, and optional temperature setpoint, but no pressure, enthalpy, CO2, or
generic contaminant. The projection copies design/default supply flow to Zone
and return records and assigns fixed default Zone temperature and humidity to
returns. It implements no exhaust, gain, airflow-window, room-air/mixer, clamp,
latent/refrigeration, contaminant, or demand-reset behavior. Active System Node
result output covers the supply node rather than a CP271-computed return node.

The CLI finite-limit and humidity evidence instead reads EnergyPlus same-call
return temperature and humidity and injects that oracle recirculation state
into the Rust case path. Across manifests that reference the 30 IdealLoads IDF
models, the broader return-node output census is 35 rows in 17 cases: 17
temperature, 17 humidity-ratio, and one mass-flow row. Three are baseline and
32 are diagnostic; zero is conformance-level. Those rows therefore do not
establish Rust CP271 numerics.

Across 120 unique data models, split 108 IDF and 12 epJSON, 30 models contain
one IdealLoads system, equipment list, and connection. Every connection has a
nonblank inlet and direct return and a blank exhaust. The corpus has no
`Sizing:Zone`, Space or SpaceList, SpaceHVAC return mixer, room-air model,
airflow window, Lights, refrigeration case/walk-in/air-chiller, return path,
or AirflowNetwork topology.

Ordinary EnergyPlus simulation enters CP271 for the controlled Zone and return
node in all 30 IdealLoads models, with no sizing route. The zonal-only setup
selects `NoHeatToReturnAir=true`, so the represented branch copies Zone
return temperature, humidity, and pressure, recomputes enthalpy, sees zero
return gain, and runs the demand tail. One CO2-DCV model also enables return
CO2 copying, but has no return-CO2 conformance output. Dynamic HVAC invocation
counts are not instrumented. Rust's direct PurchasedAir route and oracle
recirculation input supply no equivalent transition evidence.

The roadmap still requires Rust-owned return/exhaust node state including
pressure, enthalpy, and contaminants; Space and room-air topology; airflow
window and gain ownership; multiplier, clamp, latent, refrigeration, exhaust,
and demand-reset semantics; and complete failure/replay behavior. Static
connection names and diagnostic projection state cannot establish this mutable
leaf.

CP271 changes no Rust target or state, support declaration, test, capability,
output, comparator, case, manifest, numerical claim, performance claim, or
conformance status. Counts become 32 algorithms and 275 routines, split 58
`state_mapped` plus 217 `source_mapped`, with 152 required. Domain-required
counts become heat-balance 88, HVAC 41, plant 1, and time/schedule 22, with
readiness `0/88`, `0/41`, `0/1`, and `0/22`. The IdealLoads parent remains
`scaffold` at claim level `none`.

CP272 next adds required source-mapped `routine.update_zone_equipment`
immediately after `routine.calc_zone_leaving_conditions` and before
`routine.sim_purchased_air`. `UpdateZoneEquipment` is declared at
`ZoneEquipmentManager.hh` line 242 and implemented completely at
`ZoneEquipmentManager.cc` lines 5545-5568. `CalcAirFlowSimple` begins at
source line 5570.

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
