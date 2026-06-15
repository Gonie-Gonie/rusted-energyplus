---
status: active
claim_level: diagnostic-only
owner: runtime
last_reviewed: 2026-06-15
---

# IdealLoads Source Map

Reference version: EnergyPlus 26.1.0

Purpose: map the first `ZoneHVAC:IdealLoadsAirSystem` numerical-conformance
candidate to EnergyPlus source functions before any IdealLoads load,
air-node, availability, humidity, outdoor-air, sizing, fuel, or meter
conformance claim is promoted.

## Initial Claim Boundary

The first candidate case is
`ideal_loads_no_oa_sensible_conformance_001`. It remains
`comparison_class = "diagnostic-only"` and `conformance_claim = false` until
the manifest is explicitly promoted. The diagnostic compare lane now produces
timestamp-aligned Rust result-store artifacts for the declared variables and
requires zero tolerance failures before promotion.

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

The Rust compatibility path must preserve this ordering for the diagnostic
candidate before any variable is promoted to conformance.

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
It still requires upstream, source-order zone demand. The diagnostic report
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

## Required Proof Variables

The initial proof surface is:

- `Zone Thermostat Heating Setpoint Temperature`
- `Zone Thermostat Cooling Setpoint Temperature`
- `Zone Ideal Loads Zone Total Heating Rate`
- `Zone Ideal Loads Zone Total Cooling Rate`
- `Zone Ideal Loads Zone Sensible Heating Rate`
- `Zone Ideal Loads Zone Sensible Cooling Rate`
- `Zone Ideal Loads Supply Air Total Heating Rate`
- `Zone Ideal Loads Supply Air Total Cooling Rate`
- `Zone System Predicted Sensible Load to Setpoint Heat Transfer Rate`
- `System Node Temperature`
- `System Node Mass Flow Rate`

`System Node Humidity Ratio`, zone-air-node proof rows, heating/cooling
setpoint-distance proof rows, latent IdealLoads outputs, outdoor-air outputs,
heat-recovery outputs, economizer outputs, and meter outputs remain
diagnostic-only until their source-order branches are ported or explicitly
included in a promoted claim.

## Diagnostic Compare Artifacts

`scripts/dev.cmd compare-ideal-loads-no-oa-sensible-diagnostic` generates the
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

The current diagnostic run compares 16 Detailed series over 110 samples. All
series pass their draft tolerances, and `tolerance-failures.csv` is empty. This
is promotion evidence only; it does not create an IdealLoads conformance claim
while the manifest remains diagnostic-only.

## Promotion Requirements

The candidate can become an IdealLoads conformance claim only when all of these
exist:

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

Until then, the case is a diagnostic candidate and not an IdealLoads numerical
conformance result.
