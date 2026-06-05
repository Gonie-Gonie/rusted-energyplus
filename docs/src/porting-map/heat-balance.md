# Heat Balance Porting Map

Status: planning guard before native heat-balance implementation.

EnergyPlus reference version:

```text
EnergyPlus 26.1.0
.reference/energyplus-src/26.1.0
```

This document is the required map before replacing the current first-zone
diagnostic toy model with an EnergyPlus-aligned heat-balance subset. It does
not claim runtime conformance.

## Current Evidence

Already implemented gates:

- `compare schedule-value`: constant schedule ESO parity
- `compare weather-drybulb`: EPW dry-bulb ESO parity
- `model geometry`: Rust zone geometry summary
- `compare geometry`: EIO `Zone Information` parity for zone count, surface
  count, floor area, volume, and exterior gross wall area
- `compare internal-gains`: EIO `OtherEquipment Internal Gains Nominal` parity
  for schedule binding, zone binding, design level, W/m2, and gain fractions
- `compare internal-convective-gain`: ESO `Zone Total Internal Convective
  Heating Rate` parity for the typed `OtherEquipment` convective-gain trace

Still diagnostic-only:

- `run first-zone`
- `compare zone-temperature`

Those commands must stay `conformance_claim: false` until this map is
implemented behind tolerance-gated reports.

## EnergyPlus Source Map

Primary files:

| Area | EnergyPlus file |
|---|---|
| global heat-balance data | `src/EnergyPlus/DataHeatBalance.hh` |
| heat-balance orchestration | `src/EnergyPlus/HeatBalanceManager.cc` |
| surface heat balance | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` |
| air heat balance setup | `src/EnergyPlus/HeatBalanceAirManager.cc` |
| internal gains input/reporting | `src/EnergyPlus/HeatBalanceInternalHeatGains.cc` |
| internal gains runtime sums | `src/EnergyPlus/InternalHeatGains.cc` |
| zone air predictor/corrector | `src/EnergyPlus/ZoneTempPredictorCorrector.cc` |
| predictor/corrector state | `src/EnergyPlus/ZoneTempPredictorCorrector.hh` |

Important routines identified in the reference source:

- `ManageHeatBalance`
- `GetProjectControlData`
- `GetMaterialData`
- `GetConstructData`
- `GetZoneData`
- `InitHeatBalance`
- `ManageZoneAirUpdates`
- `correctZoneAirTemps`
- `CalcZoneAirTempSetPoints`
- `zoneSumAllInternalConvectionGains`
- `spaceSumAllInternalConvectionGains`
- `CalcHeatBalanceOutsideSurf`
- `CalcHeatBalanceInsideSurf`

## Rust Target Shape

The current Rust runtime has:

- `SimulationModel`
- `ExecutionPlan`
- `SimulationState`
- `HeatBalanceState` shell
- `ResultStore`
- schedule traces
- EPW weather records
- geometry and internal-gains EIO gates

The first heat-balance implementation should add state without deleting the
diagnostic path:

```text
HeatBalanceState
  zones:
    MAT
    previous MAT histories
    air heat capacity
    convective internal gains
    surface convection sums
  surfaces:
    outside temperature inputs
    inside temperature histories
    construction/material thermal properties
  timestep:
    zone timestep index
    hour-ending time point
```

Recommended Rust module split:

| Rust area | Purpose |
|---|---|
| `ep_runtime::heat_balance` | zone/surface heat-balance state and timestep driver |
| `ep_runtime::internal_gains` | typed internal-gain evaluation and reporting sums |
| `ep_runtime::surface_balance` | exterior and interior opaque surface balance subset |
| `ep_runtime::zone_air` | zone air predictor/corrector subset |
| `ep_compare` | EIO/ESO parsers and tolerance summaries |

These modules can begin inside `crates/ep_runtime/src/lib.rs` only if the code
stays small. Split once the implementation needs multiple state structs or
source-reference comments.

## Porting Order

1. Preserve current diagnostic command behavior.
2. Add heat-balance state structs with no solver changes. Implemented:
   `ep_runtime::initialize_heat_balance_state`.
3. Add report-only traces for EnergyPlus inputs already gated by EIO:
   geometry, material resistance, constructions, schedules, and
   `OtherEquipment`.
4. Port internal convective gains as a separate runtime trace. Implemented:
   `ep_runtime::simulate_zone_internal_convective_gains`.
5. Add an EnergyPlus comparison for `Zone Total Internal Convective Heating
   Rate` before using it to claim zone air parity.
6. Add opaque surface heat-balance state for the first no-window one-zone case.
   Implemented state inputs: surface boundary condition, construction,
   outside-layer material, thermal resistance, optional area heat capacity, and
   surface conductance.
7. Add zone air predictor/corrector subset and compare `Zone Mean Air
   Temperature`. Implemented state advance: one heat-balance timestep updates
   MAT history, internal convective gains, opaque surface heat gain, and zone
   mean air temperature. EnergyPlus MAT comparison remains diagnostic-only
   until a tolerance-gated report exists.
8. Only after a tolerance-gated report exists, promote any case from
   diagnostic-only to conformance.

## First Declared Runtime Subset

Allowed:

- one or more `Zone` objects
- `BuildingSurface:Detailed` opaque floors, roofs, ceilings, and walls
- `Construction` with first-layer material resistance
- `Material` and `Material:NoMass`
- `Schedule:Constant`
- `Schedule:Compact` all-days `Until` subset
- `OtherEquipment`
- hourly weather dry-bulb input
- hourly output variables needed by the gate

Not in the first subset:

- windows and solar distribution parity
- infiltration and ventilation
- people/lights/electric equipment beyond `OtherEquipment`
- HVAC equipment, plant loops, and availability managers
- warmup convergence parity
- sizing periods
- moisture balance
- EMS/plugin callbacks

## Required Gates Before Heat-Balance Claim

A heat-balance claim needs all of these:

- case manifest with `conformance_claim = true`
- exact IDF and weather references
- EnergyPlus baseline artifacts
- Rust artifacts
- variable list
- tolerance policy
- report path
- blocking check script

Minimum first variables:

- `Zone Mean Air Temperature`
- `Zone Total Internal Convective Heating Rate`
- `Site Outdoor Air Drybulb Temperature`

Recommended supporting EIO rows:

- `Zone Information`
- `OtherEquipment Internal Gains Nominal`
- material/construction summaries once typed material layers are expanded

## Stop Conditions

Do not mark heat-balance compatibility if:

- the comparison is extraction-only
- tolerance policy is absent
- only first/last values are compared
- EnergyPlus warmup behavior is bypassed without a documented subset boundary
- any required input interpretation gate is failing
