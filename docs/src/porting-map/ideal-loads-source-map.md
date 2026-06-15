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
the Rust runtime produces tolerance-gated results for the declared variables.

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
| `PurchasedAirManager::SimPurchasedAir` | `src/EnergyPlus/PurchasedAirManager.cc` | `ep_runtime::ideal_loads::sim_ideal_loads_air_system_compat` |
| `PurchasedAirManager::GetPurchasedAir` | `src/EnergyPlus/PurchasedAirManager.cc` | `ep_compiler::objects::ideal_loads`; `ep_model::objects::ideal_loads` |
| `PurchasedAirManager::InitPurchasedAir` | `src/EnergyPlus/PurchasedAirManager.cc` | `ep_runtime::ideal_loads::init_ideal_loads_air_system_compat` |
| `PurchasedAirManager::SizePurchasedAir` | `src/EnergyPlus/PurchasedAirManager.cc` | `ep_runtime::ideal_loads::size_ideal_loads_air_system_compat` |
| `PurchasedAirManager::CalcPurchAirLoads` | `src/EnergyPlus/PurchasedAirManager.cc` | `ep_runtime::ideal_loads::calc_ideal_loads_air_system_loads_compat` |
| `PurchasedAirManager::UpdatePurchasedAir` | `src/EnergyPlus/PurchasedAirManager.cc` | `ep_runtime::ideal_loads::update_ideal_loads_air_system_nodes_compat` |
| `PurchasedAirManager::ReportPurchasedAir` | `src/EnergyPlus/PurchasedAirManager.cc` | `ep_runtime::ideal_loads::report_ideal_loads_air_system_compat` |
| `ZoneEquipmentManager::ManageZoneEquipment` | `src/EnergyPlus/ZoneEquipmentManager.cc` | `ep_runtime::zone_equipment::manage_zone_equipment_compat` |
| `ZoneEquipmentManager::SimZoneEquipment` | `src/EnergyPlus/ZoneEquipmentManager.cc` | `ep_runtime::zone_equipment::simulate_zone_equipment_compat` |
| `ZoneTempPredictorCorrector` predicted load state | `src/EnergyPlus/ZoneTempPredictorCorrector.cc` | `ep_runtime::zone_equipment::ZoneSysEnergyDemand` |

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

The helper must still use the EnergyPlus formula order for:

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
- `System Node Temperature`
- `System Node Mass Flow Rate`

`System Node Humidity Ratio`, latent IdealLoads outputs, outdoor-air outputs,
heat-recovery outputs, economizer outputs, and meter outputs remain
diagnostic-only until their source-order branches are ported.

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
