---
status: active
claim_level: planning-guard
owner: runtime
last_reviewed: 2026-06-07
---

# Heat Balance Source Map

Reference version: EnergyPlus 26.1.0

Reference source root:

```text
.reference/energyplus-src/26.1.0/
```

Purpose: record the EnergyPlus source files, routines, data structures, and
call order that must be reviewed before any v0.8 heat-balance algorithm work is
promoted beyond diagnostics. This map is a planning guard, not a conformance
claim.

## Primary Source Files

| Area | EnergyPlus source | Rust target |
|---|---|---|
| heat-balance orchestration | `src/EnergyPlus/HeatBalanceManager.cc` | `ep_runtime::heat_balance` |
| heat-balance declarations | `src/EnergyPlus/HeatBalanceManager.hh` | `ep_runtime::heat_balance` |
| global heat-balance data | `src/EnergyPlus/DataHeatBalance.hh` | `ep_model`, `ep_runtime::HeatBalanceState` |
| surface heat balance | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `ep_runtime::surface_balance` |
| surface heat-balance declarations | `src/EnergyPlus/HeatBalanceSurfaceManager.hh` | `ep_runtime::surface_balance` |
| zone air predictor/corrector | `src/EnergyPlus/ZoneTempPredictorCorrector.cc` | `ep_runtime::zone_air` |
| zone air declarations | `src/EnergyPlus/ZoneTempPredictorCorrector.hh` | `ep_runtime::zone_air` |
| internal gains input summaries | `src/EnergyPlus/HeatBalanceInternalHeatGains.cc` | `ep_compiler`, `ep_runtime::internal_gains` |
| internal gains runtime sums | `src/EnergyPlus/InternalHeatGains.cc` | `ep_runtime::internal_gains` |
| output variable registration | `src/EnergyPlus/OutputProcessor.cc` | `ep_conformance`, `ep_runtime::ResultStore` |

## Required Routine Map

| Porting area | EnergyPlus routine or symbol | Current Rust status |
|---|---|---|
| heat-balance driver | `ManageHeatBalance` | mapped-not-ported |
| project heat-balance controls | `GetProjectControlData` | mapped-not-ported |
| material input | `Material::GetMaterialData` | typed subset exists; source map required before expansion |
| construction input | `GetConstructData` | typed opaque layer stack exists; CTF coefficients not ported |
| zone input | `GetZoneData` | typed geometry subset exists; source map required before expansion |
| heat-balance initialization | `InitHeatBalance` | diagnostic shell only |
| outside surface balance | `CalcHeatBalanceOutsideSurf` | CTF environmental balance helper exists; full call order not ported |
| inside surface balance | `CalcHeatBalanceInsideSurf` | CTF inside-face helper exists; full iteration/call order not ported |
| zone air updates | `ManageZoneAirUpdates` | diagnostic shell only |
| zone air correction | `correctZoneAirTemps` | mapped-not-ported |
| internal convective gains | `zoneSumAllInternalConvectionGains` | conformance trace exists for `internal_gains_001` only |
| space internal convective gains | `spaceSumAllInternalConvectionGains` | mapped-not-ported |

## Call Order Boundary

The first v0.8 heat-balance candidate must preserve this source-derived order
unless the deviation is documented in a case-specific waiver:

1. `ManageHeatBalance`
2. input acquisition through project controls, materials, constructions, and zones
3. `InitHeatBalance`
4. outside opaque surface balance
5. inside opaque surface balance
6. internal convective gain summation
7. zone air predictor/corrector update
8. output variable registration and sampling

## Data Structure Map

| EnergyPlus data | Rust target | Boundary |
|---|---|---|
| `DataHeatBalance::ZoneData` | `ep_model::Zone`, `ep_runtime::ZoneHeatBalanceState` | geometry is partial; heat capacity and histories are not conformance-ready |
| `DataSurface::SurfaceData` | `ep_model::Surface`, `ep_runtime::SurfaceHeatBalanceState` | opaque surface subset only; outside-layer roughness metadata is tracked for future exterior convection work |
| construction/material CTF data | `ep_model::Construction`, `ep_model::Material`, `ep_runtime::SurfaceCtfState` | ordered opaque layer stack, diagnostic EIO coefficient seeding for steady/no-mass rows, and CTF history advancement exist; mass-material coefficient generation and face-temperature CTF solving are not ported |
| zone predictor histories, sums, and coefficients such as `MAT`, `XMAT`, `DSXMAT`, `SumHA`, `SumHATsurf`, `SumHATref`, `TempDepCoef`, `TempIndCoef`, `AirPowerCap`, and `TempHistoryTerm` | `ep_runtime::ZoneHeatBalanceState`, `ep_runtime::ZoneAirTemperatureCoefficients`, and future `ep_runtime::zone_air` histories | diagnostic shell keeps MAT history, stores surface convection sums, and snapshots EnergyPlus-shaped zone-air coefficients for future predictor wiring; full predictor/corrector equations are not ported |
| internal gain sums such as `SumIntGain` | `simulate_zone_internal_convective_gains` and future state fields | convective trace conformance only for declared v0.26 case |

## CTF Porting Notes

EnergyPlus 26.1.0 anchors for opaque conduction:

- `Construction.hh` defines `MaxLayersInConstruct`, `ConstructionProps::TotLayers`,
  `LayerPoint`, and CTF arrays `CTFOutside`, `CTFCross`, `CTFInside`, and
  `CTFFlux`.
- `Construction.cc::ConstructionProps::calculateTransferFunction` consumes the
  material layer physical properties, handles all-resistive, reversed, and
  state-space paths, and emits the EIO `Construction CTF` rows.
- `HeatBalanceSurfaceManager.cc` builds `SurfCTFConstInPart` and
  `SurfCTFConstOutPart` from temperature and flux histories before calculating
  current inside/outside conduction fluxes and face temperatures.
- `CalcHeatBalanceInsideSurf2CTFOnly` uses `IterDampConst = 5.0`, subtracts
  `CTFCross[0]` from the inside denominator for adiabatic surfaces, and uses
  `CTFCross[0] * SurfTempOutHist(1)` for standard opaque surfaces.
- `CalcHeatBalanceInsideSurf2CTFOnly` builds `SurfTempTerm` from
  `SurfCTFConstInPart`, `SurfQdotRadIntGainsInPerArea`, `SurfOpaqQRadSWInAbs`,
  `SurfQAdditionalHeatSourceInside`, `HConvInt * RefAirTemp`, and
  `SurfQdotRadHVACInPerArea`, then adds `SurfQdotRadNetLWInPerArea` in the
  standard no-pool branch. Rust now exposes zero-initialized per-surface slots
  for those inside radiant/source terms so future solar/radiation wiring can be
  isolated without changing the CTF face solver API again.
- `CalcHeatBalanceInsideSurf2CTFOnly` keeps the previous inside surface
  temperature in `SurfTempInsOld` for the iterative damping term; Rust now
  preserves the previous per-surface inside-face temperature before its
  zone-air predictor pass overwrites the current face estimate.
- EnergyPlus iterates inside/outside surface balances before committing CTF
  histories for the timestep. Rust default diagnostics still use one pass, but
  `RUSTED_ENERGYPLUS_HEAT_BALANCE_SURFACE_ITERATIONS` and the all-CTF
  surface-iter3 probe can repeat the face-balance pass while advancing histories
  once at timestep end.
- `DataHeatBalance::SurfInitialConvCoeff = 3.076 W/m2-K` initializes inside
  convection coefficients before the selected inside convection algorithm is
  evaluated. `DataHeatBalance::LowHConvLimit = 0.1 W/m2-K` and
  `HighHConvLimit = 1000 W/m2-K` bound calculated convection coefficients.
- `ConvectionCoefficients.cc::CalcASHRAEDetailedIntConvCoeff` dispatches
  inside `SurfaceConvectionAlgorithm:Inside,TARP` surfaces through
  `CalcASHRAETARPNatural(SurfTempIn, RefAirTemp, -CosTilt)`, using ASHRAE
  vertical-wall and Walton stable/unstable horizontal-or-tilt correlations.
- `CalcHeatBalanceOutsideSurf` solves the no-movable-insulation exterior face
  temperature with `-SurfCTFConstOutPart`, current `CTFCross[0] * SurfTempIn`,
  absorbed outside source terms, and exterior convection/radiation coefficients.
- `ConvectionCoefficients.cc::InitExtConvCoeff` dispatches
  `SurfaceConvectionAlgorithm:Outside,DOE-2` through the DOE-2 branch:
  windward/leeward MoWITT forced terms, ASHRAE TARP natural convection, and
  EnergyPlus roughness multipliers. Rust has a source-anchored DOE-2 helper for
  this expression, but the official dynamic diagnostic keeps the existing
  exterior balance coefficient until DOE-2 wiring can be paired with the full
  exterior radiation and iteration path; the isolated helper probe improved some
  wall/roof rows but regressed MAT and zone aggregate conduction.
- `ZoneTempPredictorCorrector.cc::ZoneSpaceHeatBalanceData::predictSystemLoad`
  builds `TempDepCoef` and `TempIndCoef` from `SumHA`, `SumHATsurf`,
  `SumHATref`, internal gains, air-exchange terms, and third-order history
  terms. `calcZoneOrSpaceSums`/`calcSumHAT` are the source anchors for the
  surface convection sums. Rust now stores the zone-level `SumHA`,
  `SumHATsurf`, and `SumHATref` diagnostic state from current inside
  convection coefficients and surface temperatures, snapshots
  `TempDepCoef`, `TempIndCoef`, `AirPowerCap`, and `TempHistoryTerm` in
  `ZoneAirTemperatureCoefficients`, and exposes EnergyPlus-shaped analytical
  and third-order zone-air temperature helpers. `HeatBalanceZoneAirAlgorithm`
  keeps the default trace on the existing simplified analytical shell while
  allowing an explicit third-order diagnostic probe. The default predictor
  equation itself remains the simplified diagnostic shell until all coefficient
  inputs are wired from source-mapped runtime state.
- `DataHeatBalance.cc::ZoneData::setUpOutputVars` registers `Zone Air Heat
  Balance Internal Convective Heat Gain Rate`, `Zone Air Heat Balance Surface
  Convection Rate`, and `Zone Air Heat Balance Air Energy Storage Rate`. Rust
  now emits diagnostic zone series with those EnergyPlus names from the current
  internal gain, `SumHA/SumHATsurf/SumHATref`, MAT, and air-capacity state. The
  air energy storage output follows EnergyPlus reporting semantics by using
  `TempIndCoef - TempDepCoef * MAT` for the analytical diagnostic lane and the
  timestep finite-difference expression for the third-order probe. Official
  dynamic reports can compare these latent air-balance terms before a
  conformance claim is attempted.
- `SimulationManager.cc` documents the high-level order for the relevant
  timestep path as `ManageSurfaceHeatBalance` ->
  `CalcHeatBalanceOutsideSurf` -> `CalcHeatBalanceInsideSurf` ->
  `ManageAirHeatBalance` -> `CalcHeatBalanceAir` -> `ManageHVAC` ->
  `ManageZoneAirUpdates(PREDICT)`. `HeatBalanceAirManager.cc` confirms that
  `CalcHeatBalanceAir` delegates to `HVACManager::ManageHVAC`, and
  `ZoneTempPredictorCorrector.cc::ZoneSpaceHeatBalanceData::correctAirTemp`
  applies the `ThirdOrder`, `AnalyticalSolution`, or `EulerMethod` branch from
  freshly calculated `SumHA/SumHATsurf/SumHATref` terms. Rust still updates the
  default MAT through the simplified zone-air shell before the current surface
  CTF pass, then records the EnergyPlus-shaped coefficients afterward. Any
  future analytical/coefficient probe should therefore be isolated from the
  default lane until the inside-surface, zone-air correction, and history-update
  order is ported as one coherent path.

Current Rust boundary:

- `ep_model::Construction.layers` stores material IDs from outside to inside;
  `outside_layer` remains the outside-face compatibility field.
- `ep_compare` reads EIO `CTF` coefficient rows and associates them with the
  preceding `Construction CTF` row for coefficient-level oracle checks.
- `ep_model` and `ep_compiler` preserve material surface roughness names using
  EnergyPlus roughness categories so future DOE-2/TARP exterior convection
  ports can use the selected outside layer metadata directly.
- `ep_runtime` sums layer thermal resistance and available areal heat capacity
  for the current simplified opaque surface state, stores outside-layer
  roughness plus CTF coefficient/history slots per surface, and can seed those
  slots from EIO rows during diagnostic-only heat-balance runs. The default CLI
  diagnostic seed is
  limited to steady/no-mass `#CTFs <= 1` constructions while mass-material CTF
  temperature histories are isolated from the simplified timestep shell. Runtime
  helpers now encode the EnergyPlus-shaped CTF inside and outside
  face-temperature equations, and the timestep shell uses the EnergyPlus TARP
  inside natural convection coefficient in the inside CTF balance. A DOE-2
  outside convection helper exists for future wiring, but full inside iteration
  order, exterior DOE-2/radiation coupling, zone predictor/corrector
  equations, and radiation coefficient updates are not yet wired.
- EnergyPlus mass-material CTF coefficient generation, source/sink terms, and
  timestep-dependent transfer-function validation are still unmapped runtime
  work.

## Required Cases Before Porting

- `heat_balance_uncontrolled_001`: one-zone uncontrolled, no HVAC, opaque
  surfaces only
- `heat_balance_nomass_001`: `Material:NoMass` variant
- `heat_balance_mass_001`: simple mass material variant

These cases may remain diagnostic-only until v0.8 declares tolerances and
blocking gates.

## Stop Rule

No heat-balance algorithm change may be promoted as conformance work unless the
changed behavior has a source-map entry in this document, an output-variable
entry in `output-variable-source-map.md`, and a readiness note in
`algorithm-porting-readiness.md`.
