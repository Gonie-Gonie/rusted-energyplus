---
status: active
claim_level: planning-guard
owner: runtime
last_reviewed: 2026-06-08
---

# Zone Air Update Map

Reference version: EnergyPlus 26.1.0

Purpose: define what must be ported before an official ExampleFile zone air
temperature series can be promoted with `conformance_claim=true`.

## Source Anchors

| EnergyPlus area | Source anchor | Rust target | Current status |
|---|---|---|---|
| zone predictor/corrector driver | `src/EnergyPlus/ZoneTempPredictorCorrector.cc` | `advance_heat_balance_state_one_timestep` successor | scalar shell only |
| mean air temperature histories | `MAT`, `XMAT`, `XM2T`, `XM3T`, `ZoneAirTemp` | `ZoneHeatBalanceState::previous_mean_air_temperatures_c` | placeholder history |
| air capacitance | zone volume, multipliers, air heat capacity | `ZoneHeatBalanceState::air_heat_capacity_j_per_k` | simple constant density/cp |
| internal convective gains | `InternalHeatGains.cc` | `simulate_zone_internal_convective_gains`, heat-balance gain input | convective gain case only |
| surface convection coupling | `HeatBalanceSurfaceManager.cc` | future surface convection aggregate | not ported |
| HVAC and infiltration coupling | zone equipment and air balance managers | future zone load inputs | not ported |

## Promotion Requirements

An official ExampleFile zone-air series may become conformance only after:

- Rust computes the hourly series without reading EnergyPlus ESO values.
- warmup exclusion/inclusion is explicit and matches the report contract.
- zone timestep count, hourly reporting timestamp, and run-period dates match.
- all heat inputs used in the promoted case have source-map entries.
- failure deltas are below declared max absolute and RMSE tolerances.
- the case has a blocking gate and `conformance_claim=true`.

## Current Boundary

`Zone Mean Air Temperature` is conformance only for the declared no-mass local
cases. Official `1ZoneUncontrolled` zone temperature is a baseline candidate
until this map is implemented beyond the scalar conductance shell.
