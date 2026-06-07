---
status: active
claim_level: planning-guard
owner: conformance
last_reviewed: 2026-06-07
---

# Output Variable Source Map

Reference version: EnergyPlus 26.1.0

Purpose: map the first heat-balance candidate output variables to EnergyPlus
source files and Rust result locations before promoting conformance claims.
`heat_balance_nomass_001` promotes the first zone variable in v0.8, and
`surface_temperature_nomass_001` promotes the first surface variables in v0.9.

## Candidate Variables

| Variable | Frequency | EnergyPlus source | Rust source or target | Current level |
|---|---|---|---|---|
| `Zone Mean Air Temperature` | hourly | `src/EnergyPlus/ZoneTempPredictorCorrector.cc` | `ResultStore` series from heat-balance trace | conformance for `heat_balance_nomass_001`; diagnostic otherwise |
| `Zone Total Internal Convective Heating Rate` | hourly | `src/EnergyPlus/InternalHeatGains.cc` | `simulate_zone_internal_convective_gains` | smoke |
| `Zone Air Heat Balance Internal Convective Heat Gain Rate` | hourly | `src/EnergyPlus/ZoneTempPredictorCorrector.cc`; `src/EnergyPlus/InternalHeatGains.cc` | future `ep_runtime::zone_air` report state | mapped-not-ported |
| `Zone Air Heat Balance Surface Convection Rate` | hourly | `src/EnergyPlus/ZoneTempPredictorCorrector.cc`; `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | future surface convection sum | mapped-not-ported |
| `Zone Air Heat Balance Air Energy Storage Rate` | hourly | `src/EnergyPlus/ZoneTempPredictorCorrector.cc` | future zone air storage term | mapped-not-ported |
| `Surface Inside Face Temperature` | hourly | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `ResultStore` series from heat-balance trace | conformance for `surface_temperature_nomass_001`; diagnostic otherwise |
| `Surface Outside Face Temperature` | hourly | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `ResultStore` series from heat-balance trace | conformance for `surface_temperature_nomass_001`; diagnostic otherwise |
| `Surface Inside Face Conduction Heat Transfer Rate` | hourly | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | future opaque conduction result | mapped-not-ported |
| `Surface Outside Face Conduction Heat Transfer Rate` | hourly | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | future opaque conduction result | mapped-not-ported |
| `Zone Thermostat Heating Setpoint Temperature` | hourly | thermostat setup/output registration in EnergyPlus HVAC/zone predictor code | future thermostat result state | baseline-only for `ideal_loads_thermostat_001` |
| `Zone Thermostat Cooling Setpoint Temperature` | hourly | thermostat setup/output registration in EnergyPlus HVAC/zone predictor code | future thermostat result state | baseline-only for `ideal_loads_thermostat_001` |
| `Zone Ideal Loads Zone Total Heating Rate` | hourly | IdealLoads HVAC component implementation and zone equipment managers | future IdealLoads result state | baseline-only for `ideal_loads_thermostat_001`; mapped-not-ported |
| `Zone Ideal Loads Zone Total Cooling Rate` | hourly | IdealLoads HVAC component implementation and zone equipment managers | future IdealLoads result state | baseline-only for `ideal_loads_thermostat_001`; mapped-not-ported |
| `System Node Temperature` | hourly | `node-state-source-map.md`; `NodeInputManager.cc`; `ZoneTempPredictorCorrector.cc`; `PurchasedAirManager.cc`; `ZoneEquipmentManager.cc` | future node-state result store | diagnostic-only for `air_side_node_diagnostic_001`; mapped-not-ported |
| `System Node Humidity Ratio` | hourly | `node-state-source-map.md`; `NodeInputManager.cc`; `ZoneTempPredictorCorrector.cc`; `PurchasedAirManager.cc`; `ZoneEquipmentManager.cc` | future node-state result store | diagnostic-only for `air_side_node_diagnostic_001`; mapped-not-ported |
| `System Node Mass Flow Rate` | hourly | `node-state-source-map.md`; `NodeInputManager.cc`; `DataZoneEquipment.cc`; `PurchasedAirManager.cc` | future node-state result store | diagnostic-only for `air_side_node_diagnostic_001`; mapped-not-ported |
| `System Node Setpoint Temperature` | hourly | `node-state-source-map.md`; `NodeInputManager.cc`; `ZoneTempPredictorCorrector.cc` | future node-state result store | future-gated; sentinel handling required |
| `Site Outdoor Air Drybulb Temperature` | hourly | `src/EnergyPlus/WeatherManager.cc` | EPW weather trace | smoke |
| `Schedule Value` | hourly | output processor plus schedule managers | schedule trace | smoke |

## Registration Boundary

- EnergyPlus output variables are registered through `SetupOutputVariable`.
- Rust output variables must be declared in case manifests before comparison.
- Rust values must be written to `ResultStore` or a successor output store with
  key, variable, frequency, class, source, and timestamp semantics.
- A console-only value is not a v0.8 conformance variable.

## Promotion Requirements

A variable can move from `diagnostic-only` or `smoke` to `conformance` only
when all of these exist:

- case manifest with the requested output
- EnergyPlus baseline artifact containing the requested variable
- Rust result artifact for the same key, variable, and frequency
- timestamp alignment rule
- tolerance policy
- compare-summary row with first divergence information
- blocking release gate

## Explicit Non-Claims

The current `Zone Mean Air Temperature` diagnostic report has
`tolerance_policy: none` and `status: extracted`. It is useful for locating
deltas, but it is not a zone heat-balance conformance result.

The v0.8 `heat_balance_nomass_001` report is a separate conformance result for
hourly `Zone Mean Air Temperature` only. It requires a case manifest,
zone-state tolerance, markdown/JSON report artifacts, and a blocking gate.

The v0.9 `surface_temperature_nomass_001` report is a separate conformance
result for hourly `Zone Mean Air Temperature`, `Surface Inside Face
Temperature`, and `Surface Outside Face Temperature` only. It requires
zone-state and surface-state tolerances, markdown/JSON report artifacts, and a
blocking gate.

The v0.10 `ideal_loads_thermostat_001` report is baseline-only smoke evidence
for thermostat and IdealLoads output availability plus typed graph coverage.
It is not an IdealLoads load-conformance claim and keeps
`tolerance_policy: none`.

The v0.11 `air_side_node_diagnostic_001` report is diagnostic-only node output
evidence. The v0.12 source map identifies where EnergyPlus registers and
updates those node fields, but it does not add Rust node-state samples,
tolerances, or node numerical conformance.
