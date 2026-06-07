# Heat Balance Porting Map

Status: v0.9 first narrow surface-temperature conformance gate plus ongoing
porting map.

EnergyPlus reference version:

```text
EnergyPlus 26.1.0
.reference/energyplus-src/26.1.0
```

This document is the required map before replacing the current first-zone
diagnostic toy model with an EnergyPlus-aligned heat-balance subset. It does
not claim runtime conformance.

v0.7 splits the blocking source-map gate into companion maps:

- [Heat Balance Source Map](heat-balance-source-map.md)
- [Output Variable Source Map](output-variable-source-map.md)
- [Algorithm Porting Readiness](algorithm-porting-readiness.md)

Those documents are required before v0.8 heat-balance algorithm work can be
promoted toward conformance.

## Current Evidence

Already implemented gates:

- `compare schedule-value`: constant schedule ESO smoke comparison
- `compare weather-fields`: EPW weather field ESO smoke comparison
- `model geometry`: Rust zone geometry summary
- `compare geometry`: EIO `Zone Information` smoke comparison for zone count,
  surface count, floor area, volume, and exterior gross wall area
- `compare construction-materials`: EIO `Construction CTF` and
  `Material CTF Summary` smoke comparison for first-layer thermal conductance and
  material resistance/properties
- `compare internal-gains`: EIO `OtherEquipment Internal Gains Nominal` smoke
  comparison for schedule binding, zone binding, design level, W/m2, and gain fractions
- `compare internal-convective-gain`: ESO `Zone Total Internal Convective
  Heating Rate` smoke comparison for the typed `OtherEquipment` convective-gain trace

Still diagnostic-only:

- `run first-zone`
- `compare zone-temperature`

Those commands must stay `conformance_claim: false` unless a separate
case-specific tolerance-gated report exists.

v0.8 promoted case: `heat_balance_nomass_001`.

This case claims only hourly `Zone Mean Air Temperature` for one no-mass
adiabatic zone with no internal gains, windows, solar, infiltration, HVAC,
plant, or dynamic exterior heat-balance claim.

v0.9 promoted case: `surface_temperature_nomass_001`.

This case claims only hourly `Zone Mean Air Temperature`, `Surface Inside Face
Temperature`, and `Surface Outside Face Temperature` for the same no-mass
adiabatic surface equilibrium subset. It does not claim fenestration, solar
radiation, conduction-rate, or dynamic exterior heat-balance parity.

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

These modules can begin inside `crates/ep_runtime/src/runtime.rs` only if the
code stays small. Split once the implementation needs multiple state structs
or source-reference comments.

## Porting Order

1. Preserve current diagnostic command behavior.
2. Add heat-balance state structs with no solver changes. Implemented:
   `ep_runtime::initialize_heat_balance_state`.
3. Add report-only traces for EnergyPlus inputs already gated by EIO:
   geometry, material resistance, constructions, schedules, and
   `OtherEquipment`. Implemented EIO gates: geometry, construction/material
   thermal inputs, and `OtherEquipment` nominal gains.
4. Port internal convective gains as a separate runtime trace. Implemented:
   `ep_runtime::simulate_zone_internal_convective_gains`.
5. Add an EnergyPlus comparison for `Zone Total Internal Convective Heating
   Rate` before using it to claim zone air parity.
6. Add opaque surface heat-balance state for the first no-window one-zone case.
   Implemented state inputs: surface boundary condition, construction,
   outside-layer material, thermal resistance, optional area heat capacity, and
   surface conductance.
7. Add zone air predictor/corrector subset and compare `Zone Mean Air
   Temperature`. Implemented state advance and diagnostic trace: heat-balance
   timesteps update MAT history, internal convective gains, opaque surface heat
   gain, and zone mean air temperature, then `compare zone-temperature` reports
   EnergyPlus MAT deltas and can write diagnostic-only
   `compare-summary.json`/`compare-report.md` artifacts. The comparison remains
   diagnostic-only until a tolerance-gated report exists. The diagnostic MAT
   case is now represented by
   `data/conformance_cases/zone_temperature_diagnostic_001/case.toml` with
   `conformance_claim = false`.
8. Only after a tolerance-gated report exists, promote any case from
   diagnostic-only to conformance. Implemented first narrow promotion:
   `heat_balance_nomass_001` hourly `Zone Mean Air Temperature`.
9. Promote the first surface-state output only after the same case, variable,
   tolerance, report, and blocking-gate evidence exists. Implemented first
   narrow surface promotion: `surface_temperature_nomass_001` hourly `Surface
   Inside Face Temperature` and `Surface Outside Face Temperature`.

## First Declared Runtime Subset

Allowed:

- one or more `Zone` objects
- `BuildingSurface:Detailed` opaque floors, roofs, ceilings, and walls
- adiabatic opaque surfaces for the first v0.8 equilibrium gate
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

- `Zone Mean Air Temperature`: conformance only for `heat_balance_nomass_001`
- `Surface Inside Face Temperature`: conformance only for `surface_temperature_nomass_001`
- `Surface Outside Face Temperature`: conformance only for `surface_temperature_nomass_001`
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
