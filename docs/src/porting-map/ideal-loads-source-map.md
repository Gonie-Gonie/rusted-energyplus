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
- no heat recovery
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
| `PurchasedAirManager::SimPurchasedAir` | `src/EnergyPlus/PurchasedAirManager.cc` | future orchestration around `ep_runtime::ideal_loads` |
| `PurchasedAirManager::GetPurchasedAir` | `src/EnergyPlus/PurchasedAirManager.cc` | `ep_compiler::objects::ideal_loads`; `ep_model::objects::ideal_loads` |
| `PurchasedAirManager::InitPurchasedAir` | `src/EnergyPlus/PurchasedAirManager.cc` | `crates/ep_runtime/src/ideal_loads/init.rs::IdealLoadsInitFlags` |
| `PurchasedAirManager::SizePurchasedAir` | `src/EnergyPlus/PurchasedAirManager.cc` | `ep_runtime::ideal_loads::size_ideal_loads_air_system_compat` |
| `PurchasedAirManager::CalcPurchAirLoads` | `src/EnergyPlus/PurchasedAirManager.cc` | `crates/ep_runtime/src/ideal_loads/calc.rs::calc_no_oa_no_limit_sensible_compat` |
| `PurchasedAirManager::UpdatePurchasedAir` | `src/EnergyPlus/PurchasedAirManager.cc` | `crates/ep_runtime/src/ideal_loads/update.rs::supply_node_update_from_result` |
| `PurchasedAirManager::ReportPurchasedAir` | `src/EnergyPlus/PurchasedAirManager.cc` | `crates/ep_runtime/src/ideal_loads/report.rs::IdealLoadsReportSnapshot` |
| `ZoneEquipmentManager::ManageZoneEquipment` | `src/EnergyPlus/ZoneEquipmentManager.cc` | `crates/ep_runtime/src/zone_equipment/mod.rs::ideal_loads_zone_equipment_stages` |
| `ZoneEquipmentManager::SimZoneEquipment` | `src/EnergyPlus/ZoneEquipmentManager.cc` | `crates/ep_runtime/src/zone_equipment/mod.rs::ZoneEquipmentCompatibilityStage` |
| `ZoneTempPredictorCorrector` predicted load state | `src/EnergyPlus/ZoneTempPredictorCorrector.cc` | `crates/ep_runtime/src/zone_equipment/mod.rs::ZoneSysEnergyDemand` |

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

## Finite Flow/Capacity Diagnostics

Finite no-OA flow and capacity limits are tracked diagnostically by:

- `ideal_loads_capacity_limit_diagnostic_001`
- `ideal_loads_flow_limit_diagnostic_001`
- `ideal_loads_flow_capacity_limit_diagnostic_001`

The Rust helper `calc_no_oa_sensible_with_limits_compat` covers the current
diagnostic reconstruction for numeric flow and capacity limits. The compare
lane resolves the EnergyPlus return/exhaust recirculation node and records
`ZONE ONE RETURN` `System Node Temperature` and `System Node Humidity Ratio`
as proof rows. The finite-limit reconstruction uses that same-call
recirculation state for the no-OA mixed-air and ReportPurchasedAir
calculations, matching the declared Detailed rate and supply-node rows in the
three fixtures.

The current evidence keeps finite limits diagnostic-only. Capacity-only and
flow-and-capacity cases now have zero tolerance failures across their declared
18 Detailed series. The flow-only case also has zero tolerance failures across
its declared 18 Detailed series. No finite-limit row joins the promoted
no-OA/no-limit conformance boundary.

## Humidity-Control Diagnostics

`ideal_loads_constant_shr_diagnostic_001` adds a diagnostic-only no-OA humidity
control lane for `ConstantSensibleHeatRatio`. The compare lane resolves
`ZONE ONE RETURN` as the recirculation/mixed-air proof node, preserves the
source-order pre-update zone state for the sensible demand calculation, and
passes EPW barometric pressure into the saturation clamp used by EnergyPlus
psychrometric routines.

The Rust reconstruction uses EnergyPlus `PsyHFnTdbW`/`PsyWFnTdbH` enthalpy
constants for the latent split and records zero tolerance failures for the
declared zone/supply latent and sensible rate rows plus supply humidity ratio.

`ideal_loads_constant_supply_humidity_diagnostic_001` adds the matching
diagnostic-only no-OA `ConstantSupplyHumidityRatio` lane. It uses the EnergyPlus
minimum cooling supply humidity ratio, allows the source's small latent-heating
report rows when heating availability is on during cooling, and keeps the same
return-node mixed-air and EPW barometric-pressure saturation proof path.

`ideal_loads_humidistat_dehumidification_diagnostic_001` adds a diagnostic-only
no-OA Humidistat dehumidification lane. The compare path reads EnergyPlus
`ZoneSysMoistureDemand` proof rows for the humidifying and dehumidifying
moisture transfer rates, uses the same-timestamp return node as the
source-order zone state for the first run-period sample, and matches the
Humidistat dehumidification supply mass flow, supply humidity ratio, and latent
cooling report rows with zero tolerance failures.

These remain diagnostic-only: humidistat humidification, outdoor-air humidity
control, humidification-side `ConstantSupplyHumidityRatio`, active economizer
or heat-recovery humidity interactions, finite-limit Humidistat behavior, and
broad humidity-control conformance are not promoted.

## Outdoor-Air Prerequisites

Outdoor-air IdealLoads conformance is not promoted yet. The current
preparatory Rust surface is:

- `DesignSpecification:OutdoorAir` typed intake with method, flow terms, and
  schedule references preserved in `TypedModel`
- `ModelGraph::ideal_loads_outdoor_air_specs` linking an IdealLoads system to
  its referenced outdoor-air design specification
- `calc_design_outdoor_air_volume_flow_m3_per_s` for supported
  `Flow/Person`, `Flow/Area`, `Flow/Zone`, `AirChanges/Hour`, `Sum`, and
  `Maximum` methods
- `calc_scheduled_outdoor_air_mass_flow_rate_kg_per_s` for applying the
  current OA schedule fraction and `StdRhoAir`
- `calc_outdoor_air_sensible_report_rates_compat` for the no-economizer,
  no-heat-recovery, no-humidity Flow/Zone OA report-rate and
  mixed-air state diagnostic

`ideal_loads_outdoor_air_design_flow_diagnostic_001` adds a diagnostic-only
Flow/Zone proof lane for the EnergyPlus report variables
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
`Zone Ideal Loads Mixed Air Humidity Ratio`, plus inactive
heat-recovery/economizer report variables. The compare lane derives
EnergyPlus `StdRhoAir` from `Site:Location`, applies the blank OA schedule as
always 1.0, and writes Rust `ResultStore` series for the 96 Detailed oracle
samples. The outdoor-air mass/volume, no-humidity latent, supply-air
mass/volume/humidity, and mixed-air rows are exact in this fixture; the
sensible/total report rows use a 1 W diagnostic tolerance, and supply-air
temperature uses 0.02 C because EnergyPlus sorts them from source-order
zone/OA state and report-rate mode gates. Inactive economizer/heat-recovery
rows are exact zeros.

Indoor air quality and proportional-control outdoor-air methods remain
unresolved, and no finite-limit, active economizer, active heat recovery,
active humidity-control, saturation-limit, or DCV output is part of the
promoted IdealLoads claim.

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
- `System Node Temperature`
- `System Node Mass Flow Rate`

The active signed `Zone System Predicted Sensible Load to Setpoint Heat
Transfer Rate`, `System Node Humidity Ratio`, zone-air-node proof rows,
heating/cooling setpoint-distance proof rows, ReportPurchasedAir energy rows,
blank and constant `Schedule:Constant` fuel energy/rate rows, active
humidity-control outdoor-air latent behavior, heat-recovery outputs,
economizer outputs, finite flow/capacity limits, adaptive system timestep,
broad meter conformance, and non-constant efficiency schedules remain
diagnostic-only or unsupported until their source-order branches are ported or
explicitly included in a promoted claim. `DistrictHeatingWater:Facility` and
`DistrictCooling:Facility` are hourly oracle-MTR vs Rust aggregated fuel-energy
diagnostics for the no-OA fixtures.
The no-OA `ConstantSensibleHeatRatio` and `ConstantSupplyHumidityRatio`
zone/supply latent and sensible rows, supply humidity ratio, and return-node
humidity proof rows have diagnostic evidence only in
`ideal_loads_constant_shr_diagnostic_001` and
`ideal_loads_constant_supply_humidity_diagnostic_001`.
The outdoor-air mass-flow, standard-density volume-flow, no-humidity
outdoor-air report-rate, supply-air state, mixed-air state, and inactive
economizer/heat-recovery outputs have diagnostic evidence only in
`ideal_loads_outdoor_air_design_flow_diagnostic_001`.
The finite flow/capacity limit fixtures have diagnostic evidence only in their
three finite-limit cases; those diagnostic lanes now have zero tolerance
failures for their declared Detailed rows.

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
for declared outputs; ReportPurchasedAir energy, blank/constant
Schedule:Constant fuel-efficiency energy/rate rows, and hourly facility meters
remain diagnostic.

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
