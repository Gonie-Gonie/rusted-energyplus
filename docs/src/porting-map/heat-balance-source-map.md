---
status: active
claim_level: planning-guard
owner: runtime
last_reviewed: 2026-06-09
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
  standard no-pool branch. Rust now feeds the `SurfQdotRadIntGainsInPerArea`
  slot from `OtherEquipment` radiant fractions using the EnergyPlus
  inside-layer area-absorptance normalization while retaining outside-layer
  absorptance for exterior solar and longwave forcing. The other inside
  shortwave, additional heat source, HVAC radiant, and net longwave slots
  remain explicit per-surface source-map fields so future solar/radiation
  wiring can be isolated without changing the CTF face solver API again.
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
  EnergyPlus roughness multipliers. Rust now preserves explicit
  `SurfaceConvectionAlgorithm:Outside` objects in the typed model and uses the
  DOE-2 helper in the default exterior coefficient path when that setting is
  `DOE-2`; full exterior iteration parity remains diagnostic work.
- `DataSurfaces.cc::SetSurfaceWindSpeedAt` derives per-surface
  `SurfOutWindSpeed` from EPW wind speed, weather-station wind profile
  defaults, building terrain, and each surface centroid height. Rust now applies
  the same terrain profile and `WeatherManager.cc::interpolateWindDirection`
  timestep wind speed/direction values for diagnostic exterior convection
  instead of using raw hourly EPW wind directly, and keeps `NoWind` surfaces at
  zero local wind.
- `ConvectionCoefficients.cc::InitExtConvCoeff` also linearizes exterior
  longwave exchange into `SurfHSkyExt`, `SurfHGrdExt`, and `SurfHAirExt` using
  outside thermal absorptance, `ViewFactorSkyIR`, `ViewFactorGroundIR`, and
  `SurfAirSkyRadSplit = sqrt(0.5 * (1 + CosTilt))`. `CalcHeatBalanceOutsideSurf`
  then uses `(SurfHConvExt + SurfHAirExt) * TempExt + SurfHSkyExt * TSky +
  SurfHGrdExt * TGround` in the outside-face balance and reports
  `SurfQdotRadOutRepPerArea` from the same sky/air/ground terms. Rust now
  carries a diagnostic equivalent radiation coefficient/reference built from
  those three terms instead of the prior fixed exterior longwave coefficient,
  and the roof outside radiation/convection report rows share that helper.
- `WeatherManager.cc` sets timestep rain from interpolated liquid
  precipitation using `IsRainThreshold = 0.8 / TimeStepsInHour`, while
  `HeatBalanceSurfaceManager.cc::CalcHeatBalanceOutsideSurf` resets exposed wet
  exterior surfaces to `SurfHConvExt = 1000.0` and uses
  `SurfOutWetBulbTemp` as the convection reference. Rust now approximates this
  exposed wet-surface branch for the diagnostic exterior balance and hourly
  exterior report rows by applying the EnergyPlus hourly interpolation weights
  to liquid precipitation, mixing the dry and wet convection terms, and using a
  bounded outdoor wet-bulb approximation until the full Psychrometrics wet-bulb
  routine is ported. The run-period and warmup timestep shells now pass a
  timestep-aware weather context for exterior forcing: dry-bulb follows
  EnergyPlus hourly interpolation, rain uses the current timestep flag,
  exterior convection uses timestep wind speed/direction, and exterior solar
  balance/report terms use the same timestep solar interpolation helper that
  backs the hourly incident-solar diagnostic. Surface temperatures and
  surface/zone conduction/source report rows are averaged over the zone
  timesteps before being written as hourly diagnostics; the latent zone-air
  heat-balance rows intentionally remain hour-end diagnostics until the full
  zone predictor/corrector source path is ported.
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
  allowing explicit analytical, analytical surface-first, coupled rebalance,
  previous-inside outdoor boundary, previous-inside quick outside-conduction
  boundary, previous-inside outdoor/adiabatic boundary, and third-order
  diagnostic probes. The
  default predictor equation itself remains the simplified diagnostic shell
  until all coefficient inputs are wired from source-mapped runtime state.
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
  order is ported as one coherent path. The tracked
  `official-dynamic-heat-balance-analytical-probe` currently regresses MAT and
  zone-air heat-balance terms in the same surface-state direction as the
  third-order probe. The analytical surface-first probe isolates the next
  call-order step by delaying zone-air correction until after the current
  surface pass. It lowers MAT and inside-face temperature RMSE relative to the
  default lane, but still regresses zone-air heat-balance rates and aggregate
  conduction, so it remains diagnostic-only until the full inside-surface,
  HVAC/air-correction, and history-update order is ported. The combined
  all-CTF analytical surface-first probe lowers the same air and surface
  focus metrics further while leaving floor inside conduction as the top
  bottleneck, which keeps mass-floor CTF face/history parity as the next
  source-mapped target. The analytical coupled probe adds a diagnostic
  same-timestep surface rebalance after the analytical MAT correction; it
  lowers floor and aggregate conduction relative to the combined surface-first
  lane, but remains behind the three-pass surface iteration lane and slightly
  worsens MAT/air-storage. Pairing the coupled rebalance with three surface
  passes moves the conduction and latent air-balance best-focus rows again:
  floor inside conduction drops to RMSE `924.427599`, floor outside conduction
  to `508.231496`, and zone aggregate conduction to `93.616120`, while MAT
  remains best in the one-pass all-CTF analytical surface-first lane. The
  previous-inside outdoor boundary probe then nudges floor inside conduction to
  RMSE `923.733908` and floor outside conduction to `507.588138`, but leaves
  zone aggregate conduction and the latent zone-air balance best rows with the
  coupled iter3 lane. A DOE-2 exterior-only sibling isolates the
  `SurfaceConvectionAlgorithm:Outside,DOE-2` coefficient impact without also
  enabling the quick-conduction outside-face branch: at three surface passes it
  lowers floor storage RMSE to `1120.518407`, floor inside conduction to
  `766.667596`, floor outside conduction to `373.650657`, and MAT to
  `2.186220`, while regressing zone aggregate conduction to `124.010025` RMSE.
  This made DOE-2 exterior convection a measured source term before it was wired
  to the default source-declared path. A direct quick-outside plus DOE-2
  three-pass sibling lowers
  the quick-only iter3 floor storage row to RMSE `771.500589`, floor inside
  conduction to `587.797421`, and floor outside conduction to `227.407205`, but
  raises zone aggregate conduction to `128.396815` and the latent air-balance
  rows (`90.988382` surface convection, `95.018026` air storage). This narrows
  the next target to coupled surface/zone source ordering rather than only the
  exterior coefficient expression. Adding the EnergyPlus advanced outside-face
  zone aggregate as a latent diagnostic row exposes the exterior side of that
  same bottleneck: after the explicit exterior longwave split plus timestep
  weather/solar/wind output alignment and EnergyPlus surface-local wind-speed
  profiling, the default lane has `1926.324353` RMSE, and quick-outside iter3
  lowers it to `584.195603`, matching the explicit quick-outside plus DOE-2
  iter3 lane. The active tracker now carries 41 rows by
  adding roof outside convection, net thermal radiation, and absorbed solar
  source diagnostics so the remaining outside aggregate movement can be tied to
  exterior source rows before runtime promotion. The same source alignment
  lowered the active rain-onset max spike and default roof outside source
  bottleneck: quick-outside iter3 roof net thermal radiation RMSE is
  `566.230481`, roof outside convection heat-gain RMSE is `602.238829`,
  roof outside convection coefficient RMSE is `0.079698`, and the default roof
  outside convection heat-gain RMSE is `7997.333666`. The active top
  quick-outside bottleneck has moved to `ZN001:FLR001` surface heat storage
  (`683.997518` RMSE), keeping floor mass CTF history/order parity and zone
  aggregate conduction as the next source-mapped target rather than exterior
  wind/convection alignment. A direct runtime candidate that preserved a
  separate adiabatic mass-CTF outside face/history instead of syncing it to the
  current inside face was tested and rejected for now: using the current zone
  boundary value made floor outside conduction the top bottleneck, while using
  the previous inside face left floor heat-storage RMSE essentially unchanged
  (`684.141484`). The EnergyPlus `SurfInitialTemp`/zero-flux CTF initial-history
  lane is a better isolated target: with five surface passes it lowers floor
  heat storage to `637.691788` RMSE, floor inside conduction to `530.085504`,
  floor outside conduction to `148.148684`, and zone outside aggregate
  conduction to `579.984742`, but it slightly regresses MAT (`2.107293`) and
  the latent air-storage row (`197.510852`). Keep it as a source-aligned probe
  lane until the zone-air/source-term ordering work can absorb those air-side
  regressions. The same quick-outside path with eight surface passes isolates
  surface-iteration sensitivity further: floor heat storage falls to
  `618.692718` RMSE and floor outside conduction to `136.513781`, while MAT
  (`2.125244`) and air storage (`203.462113`) continue to regress, so this lane
  is tracked as a convergence/ordering diagnostic rather than a default. An
  eight-pass interleaved surface/zone-air correction fork then lowers floor
  heat storage further to `607.029837`, floor inside conduction to
  `515.487716`, floor outside conduction to `134.641347`, and zone outside
  aggregate conduction to `573.076953`, while MAT remains slightly worse
  (`2.128169`). This confirms the next source-order target is the coupled
  inside-surface/zone-air correction loop, not only the number of surface
  passes.
  Extending the previous-inside path with the
  source-mapped EnergyPlus quick-conduction outside-face branch lowers floor
  inside conduction to RMSE
  `812.566220`, floor outside conduction to `397.351373`, floor heat storage
  to `1198.781640`, zone aggregate conduction to `84.217233`, and MAT to
  `2.573470`, becoming the current best focus lane for those rows plus the
  latent zone-air heat-balance rates. The five-pass quick-outside probe lowers
  the active floor/aggregate bottlenecks again (`800.087434` floor inside
  conduction RMSE, `386.128809` floor outside conduction RMSE, `1174.412273`
  floor heat-storage RMSE, and `78.393234` zone aggregate conduction RMSE) at
  the cost of a small MAT regression, so it is tracked as iteration-sensitivity
  evidence rather than a default promotion. The DOE-2 exterior-convection fork
  lowers floor storage to RMSE `752.765953`, floor inside conduction to
  `576.102819`, and floor outside conduction to `225.936049`, but regresses the
  zone aggregate row to `100.797367` and the latent air-balance rates. The grey
  interior-longwave fork lowers floor storage further to RMSE `579.551277`, but
  also gives back zone aggregate conduction (`122.199401` RMSE). Combining
  DOE-2 exterior convection and grey interior longwave improves MAT
  (`0.972533` RMSE), zone surface convection (`52.581726` RMSE), floor inside
  conduction (`293.417817` RMSE), and floor storage (`575.885599` RMSE), but
  raises floor outside conduction to `423.487145` RMSE and storage max-abs to
  `8287.121494`. The EnergyPlus `ViewFactorInfo` EIO probe for
  `1ZoneUncontrolled` now anchors the Script F factor orientation used by the
  Rust diagnostic. That source-aligned ScriptF interior-longwave lane is not a
  promotion candidate yet: it regresses floor storage and zone aggregate rows
  relative to the grey longwave forks, which indicates the remaining bottleneck
  is the broader coupled surface/zone/source-term path rather than only the
  ScriptF factor math. These forks narrow the next source-mapped target to
  coherent exterior radiation, interior longwave, quick/slow boundary branches,
  source coupling, surface iteration, zone-air correction, and CTF history
  commit order rather than a single post-correction surface feedback pass. The
  previous-inside
  outdoor/adiabatic boundary probe slightly lowers floor inside conduction
  again to RMSE `923.728787`, but does not improve floor heat storage
  (`1422.231349` versus `1422.193225`) or zone aggregate conduction.

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
  inside natural convection coefficient in the inside CTF balance. Rust warmup
  now forwards available EPW weather records into the same diagnostic exterior
  forcing path used by run-period timesteps, so solar/radiation boundary
  histories no longer use a dry-bulb-only warmup path. The compiler/runtime
  shell now honors explicit `SurfaceConvectionAlgorithm:Outside,DOE-2` for the
  exterior convection coefficient, applies EnergyPlus terrain/centroid
  wind-speed profiling plus timestep wind speed/direction interpolation before
  DOE-2/MoWITT forced-convection terms, and uses EnergyPlus-shaped
  sky/air/ground exterior longwave coefficients in the diagnostic outside
  balance/report path, with timestep-interpolated weather/solar context and
  hourly-averaged surface diagnostics. Full inside iteration order, zone
  predictor/corrector equations, and coupled radiation coefficient update order
  are not yet wired.
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
