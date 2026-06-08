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
| outside surface balance | `CalcHeatBalanceOutsideSurf` | mapped-not-ported |
| inside surface balance | `CalcHeatBalanceInsideSurf` | mapped-not-ported |
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
| `DataSurface::SurfaceData` | `ep_model::Surface`, `ep_runtime::SurfaceHeatBalanceState` | opaque surface subset only |
| construction/material CTF data | `ep_model::Construction`, `ep_model::Material`, `ep_runtime::SurfaceCtfState` | ordered opaque layer stack, diagnostic EIO coefficient seeding for steady/no-mass rows, and CTF history advancement exist; mass-material coefficient generation and face-temperature CTF solving are not ported |
| zone predictor histories such as `MAT`, `XMAT`, and `DSXMAT` | future `ep_runtime::zone_air` histories | diagnostic shell only |
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

Current Rust boundary:

- `ep_model::Construction.layers` stores material IDs from outside to inside;
  `outside_layer` remains the outside-face compatibility field.
- `ep_compare` reads EIO `CTF` coefficient rows and associates them with the
  preceding `Construction CTF` row for coefficient-level oracle checks.
- `ep_runtime` sums layer thermal resistance and available areal heat capacity
  for the current simplified opaque surface state, stores CTF
  coefficient/history slots per surface, and can seed those slots from EIO rows
  during diagnostic-only heat-balance runs. The default CLI diagnostic seed is
  limited to steady/no-mass `#CTFs <= 1` constructions until CTF face-temperature
  solving is ported.
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
