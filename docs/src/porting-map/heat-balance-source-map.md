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
- In those EIO rows, `#CTFs` is EnergyPlus `NumCTFTerms`, the number of
  transfer-function coefficient terms. It is not the per-surface history
  cadence; cadence is governed by the construction `CTFTimeStep` relative to
  `TimeStepZone` and EnergyPlus `NumHistories`. The official 1Zone FLOOR row
  reports `#CTFs=5` and `Time Step {hours}=0.250`, which matches the 15-minute
  zone timestep and therefore does not by itself imply a multi-zone-timestep
  master-history interpolation path.
- `Construction.cc::ConstructionProps::printReport` emits CTF coefficient
  rows in descending array-index order (`NumCTFTerms` down to `0`), but the row
  index is still the EnergyPlus CTF array index. `HeatBalanceSurfaceManager.cc`
  consumes history terms by looping `Term = 1..NumCTFTerms`, so Rust intentionally
  sorts parsed EIO rows by `time_index` before storing runtime history vectors.
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
- The diagnostic `CTF History First-Sample Deltas` table now carries the zero
  CTF coefficients and oracle/Rust inside/outside face temperatures next to the
  current/history term decomposition. In the active all-CTF interleaved
  longwave lane, `ZN001:FLR001` has `Z0=X0=58.085610 W/m2-K` and
  `Y0=0.72354869 W/m2-K`; first-sample face-temperature deltas of only
  `0.175831697 C` inside and `0.159122817 C` outside still expand to
  `2345.374002 W` inside-current and `2117.149523 W` outside-current CTF term
  deltas. The next floor work should therefore target adiabatic face
  temperature/source handoff as well as the history vector itself.
- An interleaved ScriptF longwave probe was added and rejected as an active
  candidate. With the same all-CTF, previous-inside quick-outside, twenty-pass
  interleaved coupling, replacing the grey direct-view-factor longwave exchange
  with the current Rust ScriptF helper raises the top RMSE from
  `108.672323` to `50142.610234`, led by `ZN001:ROOF001` inside net surface
  thermal radiation; `ZN001:FLR001` heat-storage RMSE rises to `6586.821302`.
  ScriptF therefore needs source-level EnergyPlus normalization/iteration
  parity work before it can replace the grey longwave active lane.
- A frozen-inside-convection fork of the same interleaved grey longwave lane
  was added as a source-order probe. EnergyPlus computes inside convection
  terms before the CTF inside loop and only re-evaluates them on the
  `ItersReevalConvCoeff` cadence, while the previous Rust interleaved lane
  recomputed TARP coefficients on every pass. Freezing the Rust coefficient
  map at timestep start lowers top floor storage RMSE from `108.672323` to
  `105.876226`, zone surface-convection RMSE from `10.438503` to `9.385594`,
  and `ZN001:FLR001` inside-convection-coefficient RMSE from `0.073182` to
  `0.031945`. It is a useful diagnostic candidate, but floor CTF
  face-temperature/history handoff remains the dominant bottleneck.
- A current-adiabatic fork of the frozen-hconv lane was added and rejected as
  an active candidate. The EnergyPlus interzone/adiabatic branch updates
  `SurfTempOut`/`SurfOutsideTempHist(1)` from the adjacent current inside
  temperature during `CalcHeatBalanceInsideSurf2CTFOnly`, so the probe lets the
  adiabatic outside face follow the current inside solve instead of the
  timestep-start previous-inside value. This lowers `ZN001:FLR001`
  first-sample CTF current/history term deltas (`inside_current` from
  `2332.481555 W` to `1904.486777 W`, `outside_current` from `2104.053664 W`
  to `1494.452520 W`, `inside_history` from `1869.921937 W` to
  `1432.798624 W`, and `outside_history` from `1760.206936 W` to
  `1253.666354 W`), but the annual dynamic lane regresses sharply: MAT RMSE
  rises from `0.116074` to `0.366845`, floor heat-storage RMSE rises from
  `105.876226` to `507.532350`, and floor outside-conduction RMSE rises from
  `45.144665` to `471.677285`. Treat the source clue as history/report-order
  work, not a direct current-inside adiabatic outside-face replacement in this
  coupled lane.
- The heat-balance digest now includes annual CTF derived current/history
  deltas, not just first-sample rows. In the frozen-hconv best lane, the mass
  floor dominates this latent decomposition: `ZN001:FLR001` has `8760` samples
  with inside-current/history RMSE `1122.846780 W`/`1122.029419 W` and
  outside-current/history RMSE `1122.860933 W`/`1122.261275 W`, while roof and
  wall history RMSEs are near zero. The floor heat-storage RMSE is much lower
  (`105.876226 W`) because the large current/history deltas cancel in the
  reported storage sum. This keeps the next solver target on mass-floor
  face/history cancellation parity rather than no-mass wall/roof history
  bookkeeping.
- A current-longwave fork of the frozen-hconv lane was added and rejected for
  the floor-focused active path. EnergyPlus calls interior radiation exchange
  with the current `SurfTempIn` vector inside the inside-surface iteration
  loop, so the probe disables Rust's first-pass previous-inside longwave
  temperature override while preserving the frozen hconv and adiabatic CTF
  handoff behavior. It is effectively neutral for zone air (`Surface
  Convection Rate` RMSE `9.385594` to `9.385137`, `Air Energy Storage Rate`
  `16.169222` to `16.168835`) but worsens the current top floor rows:
  `ZN001:FLR001` heat-storage RMSE `105.876226` to `105.890635`, inside
  conduction `61.293942` to `61.302300`, outside conduction `45.144665` to
  `45.150659`, and latent floor current/history RMSEs all rise by about
  `0.342 W`. Keep longwave source sampling as a secondary source-order detail;
  it is not the next floor-storage lever.
- A third-order zone-air correction fork was added on top of the frozen-hconv
  interleaved grey-longwave lane. It is the strongest floor/MAT probe so far:
  MAT RMSE falls from `0.116074 C` to `0.069817 C`, floor heat-storage RMSE
  from `105.876226 W` to `54.593582 W`, floor inside conduction from
  `61.293942 W` to `31.581604 W`, floor outside conduction from
  `45.144665 W` to `23.282797 W`, and floor inside longwave from
  `30.262635 W` to `16.615214 W`. It is not a clean promotion yet because the
  latent zone-air heat-balance rows regress: `Zone Air Heat Balance Surface
  Convection Rate` RMSE rises from `9.385594 W` to `29.623453 W`, and
  `Zone Air Heat Balance Air Energy Storage Rate` rises from `16.169222 W` to
  `29.666388 W`. EnergyPlus 26.1 confirms the third-order air-storage report
  uses `RhoAir * CpAir * Volume * (MAT - ZTM[0]) / TimeStepSysSec`, so the next
  target is coupled zone-air source ordering and moist-air capacitance ownership
  rather than changing the third-order storage report formula.
- A non-frozen-hconv sibling of the third-order interleaved grey-longwave lane
  was added to isolate that trade-off. It slightly improves MAT
  (`0.069817 C` to `0.069191 C`) and the latent zone-air rows (`29.623453 W` to
  `28.637227 W` for surface convection, `29.666388 W` to `28.446243 W` for air
  storage) relative to the frozen third-order probe, but worsens the top floor
  rows: heat storage rises from `54.593582 W` to `58.289839 W`, inside
  conduction from `31.581604 W` to `33.704368 W`, and outside conduction from
  `23.282797 W` to `24.970278 W`. Keep frozen-hconv third-order as the current
  floor/MAT candidate and treat non-frozen third-order as a rejected isolation
  probe, not a promotion path.
- A report-only weather-air-storage fork of the frozen third-order lane keeps
  MAT and the floor rows bit-identical to frozen third-order (`0.069817 C`,
  `54.593582 W`, `31.581604 W`, and `23.282797 W` RMSE for MAT/floor
  storage/inside/outside conduction), but drops `Zone Air Heat Balance Air
  Energy Storage Rate` RMSE from `29.666388 W` to `5.845285 W`. The remaining
  surface-convection row is unchanged at `29.623453 W`, so the air-storage
  regression is mostly report-capacity/humidity ownership while the surface
  convection regression is still source-order/coefficient timing.
- A previous-MAT surface-convection report sibling of that weather-storage lane
  was added as a report-order rejection probe. It keeps MAT, floor rows, and
  weather-proxy air storage unchanged (`0.069817 C`, `54.593582 W`, and
  `5.845285 W` RMSE for MAT/floor storage/air storage), but worsens `Zone Air
  Heat Balance Surface Convection Rate` RMSE from `29.623453 W` to
  `104.589141 W`. EnergyPlus `CalcZoneComponentLoadSums` reports
  `SurfHConvInt * Area * (SurfTempInTmp - RefAirTemp)` after the corrected
  zone-air state, so the remaining surface-convection mismatch is not solved by
  using `ZTM[0]` as the report reference temperature.
- A balance-closure surface-convection sibling of the weather-storage lane was
  added as a lower-bound isolation probe. It leaves MAT, floor rows, and
  weather-proxy air storage unchanged, but reports the zone surface-convection
  row as `CzdTdt - SumIntGains` for this no-load/no-infiltration diagnostic and
  lowers `Zone Air Heat Balance Surface Convection Rate` RMSE from
  `29.623453 W` to `19.203798 W`. Keep this as evidence that part of the latent
  row mismatch is air-balance/report closure, not as a source-parity output:
  EnergyPlus still publishes `SumHADTsurfs` from the explicit
  `SurfTempInTmp`/`SurfHConvInt` surface sum.
- A frozen-reference-air sibling of the balance-closure lane was added to test
  whether EnergyPlus keeps the surface-solve `RefAirTemp` fixed while the
  inside surface loop iterates before the zone-air correction is committed. It
  improves MAT RMSE from `0.069817 C` to `0.031508 C`, floor inside/outside
  face-temperature RMSE from about `0.0534 C` to about `0.0322 C`, and
  aggregate inside-face conduction from `43.069343 W` to `27.427925 W`. Floor
  storage moves only slightly (`54.593582 W` to `54.561792 W`) and the trade-off
  is not promotion-ready: aggregate outside-face conduction worsens from
  `20.119228 W` to `29.132671 W`, zone surface convection from `19.203798 W` to
  `21.039586 W`, air storage from `5.845285 W` to `7.495999 W`, and floor inside
  longwave from `16.615214 W` to `31.074699 W`. Treat this as a strong source
  clue for surface reference-air cadence, but the next candidate must combine it
  with EnergyPlus inside longwave/source-order and outside aggregate reporting
  parity instead of promoting the probe directly.
- A current-longwave sibling of that frozen-reference-air probe was added after
  rechecking EnergyPlus 26.1.0's CTF-only inside loop, where
  `CalcInteriorRadExchange` receives the current `SurfTempIn` vector during the
  inside-surface iterations. In the current Rust shell this is effectively
  neutral relative to frozen-reference-air: MAT RMSE moves `0.031508 C` to
  `0.031507 C`, floor storage `54.561792 W` to `54.558577 W`, floor inside
  longwave `31.074699 W` to `31.072578 W`, and zone outside aggregate
  conduction `29.132671 W` to `29.131216 W`, while the latent zone-air rows
  remain at about `21.0396 W` surface convection and `7.4960 W` air storage.
  Keep current-pass longwave sampling as source-aligned bookkeeping, not the
  next bottleneck lever.
- Adding EnergyPlus' inside-surface convergence cutoff on top of the
  frozen-reference-air/current-longwave lane is a useful fixed-iteration
  correction. EnergyPlus uses `MaxAllowedDelTemp = 0.002 C`; with the same
  twenty-pass cap, stopping once the inside-face delta reaches that tolerance
  lowers floor heat-storage RMSE from `54.558577 W` to `52.022146 W`, floor
  inside/outside conduction from `31.672094 W`/`23.036538 W` to
  `30.201354 W`/`21.976058 W`, floor inside longwave from `31.072578 W` to
  `29.362310 W`, and zone inside/outside aggregate conduction from
  `27.426369 W`/`29.131216 W` to `26.355358 W`/`27.990507 W`. The latent
  zone-air rows still move the wrong way slightly (`21.039633 W` to
  `21.105254 W` surface convection, `7.496023 W` to `7.547299 W` air storage),
  so this is a stronger candidate cadence but not a conformance promotion.
- Re-running that converged lane with the EnergyPlus `SurfInitialTemp`-shaped
  CTF initial history policy is bit-identical after the diagnostic warmup loop:
  the top floor storage, floor inside/outside conduction, MAT, zone-air
  surface-convection, air-storage, and latent floor current/history rows all
  remain unchanged. This rules out the pre-warmup CTF seed as the next active
  lever; the remaining floor current/history cancellation mismatch must come
  from warmup-to-run-period history evolution or same-timestep source/face
  ordering after histories have already been warmed.
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
  The EnergyPlus-shaped quick-outside probes now cache the exterior report
  terms from the outside balance itself so `SurfHConvExt`-like convection and
  `SurfH*Ext`-like radiation state are reused for reporting instead of being
  recomputed from the solved face temperature. EnergyPlus evaluates those
  exterior coefficients through `InitExtConvCoeff` before the same-timestep
  outside-face temperature solve, so the quick-outside diagnostic path now also
  freezes the coefficient evaluation temperature at the timestep-start
  `SurfOutsideTempHist(1)` analogue while still reporting heat gains from the
  solved outside face temperature.
- `WeatherManager.cc` sets timestep rain from interpolated liquid
  precipitation using `IsRainThreshold = 0.8 / TimeStepsInHour`, while
  `HeatBalanceSurfaceManager.cc::CalcHeatBalanceOutsideSurf` resets exposed wet
  exterior surfaces to `SurfHConvExt = 1000.0` and uses
  `SurfOutWetBulbTemp` as the convection reference. Rust now mirrors this
  exposed wet-surface branch for the diagnostic exterior balance and hourly
  exterior report rows by applying the EnergyPlus hourly interpolation weights
  to liquid precipitation, mixing the dry and wet convection terms, and using
  the EnergyPlus Psychrometrics wet-bulb formula with timestep-interpolated
  dry-bulb, relative humidity, and barometric pressure. The run-period and
  warmup timestep shells now pass a
  timestep-aware weather context for exterior forcing: dry-bulb follows
  EnergyPlus hourly interpolation, rain uses the current timestep flag,
  exterior convection uses timestep wind speed/direction, exterior longwave
  uses timestep-interpolated horizontal infrared radiation/sky temperature,
  and exterior solar balance/report terms use the same timestep solar
  interpolation helper that backs the hourly incident-solar diagnostic.
  `SolarShading.cc::AnisoSkyViewFactors` and
  `HeatBalanceSurfaceManager.cc` show that `SurfQRadSWOutIncident` uses
  direct beam, Perez anisotropic sky diffuse (`SurfAnisoSkyMult *
  DifSolarRad`), and ground-reflected beam/diffuse terms. Rust now mirrors
  that anisotropic sky multiplier and the default ground-reflectance term for
  unobstructed exterior opaque surfaces, keeps the EnergyPlus split between
  shadowing-period beam incidence and current timestep `SOLCOS` for Perez
  sky/ground diffuse terms, applies a shadowing-period 0/1 sunlit proxy to the
  Perez circumsolar term corresponding to EnergyPlus `SurfSunlitFrac`, and
  writes diagnostic beam, sky diffuse, and ground diffuse incident component
  rows next to the total incident solar row; detailed shadowing fractions and
  obstruction reflection factors remain outside the diagnostic claim boundary. At
  sunrise/sunset shadowing-period edges, Rust preserves the diffuse and
  ground-reflected solar terms when the current-day EnergyPlus sun-up test is
  true but the averaged shadowing-period beam position is still below the
  horizon. `WeatherManager.cc` derives weather day-of-year from the run-period
  calendar and the EPW leap-year allowance rather than from the source year
  printed on each TMY record, so Rust uses the deterministic non-leap
  run-period ordinal for solar position and shadowing-period coefficients.
  Surface temperatures, surface/zone conduction/source report rows, and
  latent zone-air heat-balance rate rows are averaged over the zone timesteps
  before being written as hourly diagnostics.
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
  Rust now has unit-checked helpers for the EnergyPlus moist-air capacitance
  formulas used by `AirPowerCap`
  (`PsyRhoAirFnPbTdbW` and `PsyCpAirFnW`), but they are deliberately not wired
  into the active dynamic diagnostic solver yet. A direct weather-context
  experiment that used timestep-interpolated outdoor humidity as the zone
  `airHumRat` proxy reduced the physical altitude shortcut but worsened the
  active floor heat-storage and aggregate conduction focus rows; porting the
  actual zone humidity ratio and predictor/corrector state ownership must come
  first.
- `DataHeatBalance.cc::ZoneData::setUpOutputVars` registers `Zone Air Heat
  Balance Internal Convective Heat Gain Rate`, `Zone Air Heat Balance Surface
  Convection Rate`, and `Zone Air Heat Balance Air Energy Storage Rate`. Rust
  now emits diagnostic zone series with those EnergyPlus names from the current
  internal gain, `SumHA/SumHATsurf/SumHATref`, MAT, and air-capacity state. The
  air energy storage output follows EnergyPlus reporting semantics by using
  `TempIndCoef - TempDepCoef * MAT` for the analytical diagnostic lane and the
  timestep finite-difference expression for the third-order probe, then
  averaging the zone-timestep rate terms into the hourly diagnostic sample.
  Official dynamic reports can compare these latent air-balance terms before a
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
  iter3 lane. The active tracker now carries 65 rows by
  adding wall/roof outside convection, net thermal radiation, absorbed solar,
  incident solar, and wall outside-conduction diagnostics so the remaining
  outside aggregate movement can be tied to exterior source rows before runtime
  promotion. The same source alignment
  lowered the active rain-onset max spike and default roof outside source
  bottleneck. Adding EnergyPlus timestep interpolation for horizontal infrared
  radiation/sky temperature then lowers quick-outside iter3 roof net thermal
  radiation RMSE to `177.367681`, roof outside convection heat-gain RMSE to
  `214.438811`, and roof outside convection coefficient RMSE to `0.071865`;
  the default roof outside convection heat-gain RMSE remains `7997.333666`.
  The active top quick-outside bottleneck has moved back to `ZN001:FLR001`
  surface heat storage (`695.637088` RMSE), keeping floor mass CTF
  history/order parity and zone aggregate conduction as the next source-mapped
  target rather than exterior wind/convection alignment. A direct runtime
  candidate that preserved a separate adiabatic mass-CTF outside face/history
  instead of syncing it to the current inside face was tested and rejected for
  now: using the current zone boundary value made floor outside conduction the
  top bottleneck, while using the previous inside face left floor heat-storage
  RMSE essentially unchanged (`684.141484`). Re-testing the same idea after
  timestep-interpolated exterior longwave alignment on the interleaved
  twenty-pass lane also rejects it: freezing the adiabatic outside face at the
  timestep-start inside temperature lowers neither the active bottleneck nor
  the zone aggregate, raising floor heat-storage RMSE to `854.900255` and zone
  outside aggregate conduction RMSE to `871.940554`. The EnergyPlus
  InitHeatBalance-shaped CTF initial-history lane is a useful isolated target:
  with five surface passes and the corrected boundary outside-history/U-value
  flux seed, the first-sample floor history deltas drop to `462.011526 W`
  inside and `641.143796 W` outside, while floor heat storage lands at
  `611.120087` RMSE, floor inside conduction at `518.073223`, floor outside
  conduction at `133.967463`, and zone outside aggregate conduction at
  `553.316167`. The same lane badly exposes coupled source timing, however:
  floor inside longwave and convection rise to `1390.133963` and `926.220009`
  RMSE, MAT is `2.112462`, and the latent air-storage row is `167.005552`.
  Keep it as a source-aligned probe lane until the zone-air/source-term ordering
  work can absorb those air-side regressions. The same quick-outside path with
  eight surface passes isolates
  surface-iteration sensitivity further: floor heat storage falls to
  `629.603383` RMSE and floor outside conduction to `140.971525`, while MAT
  (`2.112893`) and air storage (`166.324263`) continue to regress relative to
  the simpler paths; roof outside convection and net thermal radiation are now
  down to `214.451575` and `177.821078`. This lane is tracked as a
  convergence/ordering diagnostic rather than a default. An
  eight-pass interleaved surface/zone-air correction fork then lowers floor
  heat storage further to `618.031709`, floor inside conduction to
  `520.860751`, floor outside conduction to `138.920627`, and zone outside
  aggregate conduction to `581.252181`, while MAT remains slightly worse
  (`2.115718`). Raising the same interleaved fork to twenty passes pushes the
  floor rows further (`578.427201` heat storage, `503.533184` inside
  conduction, and `115.570807` outside conduction) and lowers MAT to
  `2.147988` with air storage at `172.470431`; roof outside convection and net
  thermal radiation stay near `214.357183` and `178.357290`, so the top
  bottleneck is again floor heat storage. Adding a grey interior-longwave
  exchange update to the same twenty-pass interleaved fork first exposed a
  floor outside-face reporting/order trade-off: MAT fell to `0.484295`,
  surface-convection and air-storage RMSE to `21.126984` and `27.010902`,
  floor inside conduction to `90.441963`, floor heat storage to `369.424200`,
  and zone outside aggregate conduction to `328.987074`, but floor outside
  conduction regressed to `399.588084` and became the top bottleneck. A weaker
  per-pass previous-inside adiabatic boundary toggle was a no-op at that
  precision. Freezing only the adiabatic outside-face CTF balance at the
  timestep-start inside temperature across the interleaved passes matches the
  EnergyPlus `CalcHeatBalanceOutsideSurf` before `CalcHeatBalanceInsideSurf`
  and `UpdateThermalHistories` reporting order for regular adiabatic/partition
  surfaces. That accepted source-order probe moves the active lane again: MAT
  is `0.385438`, floor inside and outside face temperature RMSE are
  `0.267604` and `0.267603`, floor inside conduction is `69.374470`, floor
  outside conduction falls to `50.562260`, floor heat storage to `119.606076`,
  and zone outside aggregate conduction to `155.538581`. Roof outside
  convection and net thermal radiation become the new top source rows at
  `189.364767` and `171.066926`, so the next source-order target is exterior
  radiation/convection source coupling after the adiabatic floor CTF reporting
  order is no longer the top bottleneck. Freezing the quick-outside exterior
  convection/radiation coefficient evaluation temperature at the timestep-start
  outside face, matching the `InitExtConvCoeff` call before the TH11 solve,
  resolves the sharp post-rain roof spike: the active lane drops roof outside
  convection RMSE from `177.495366` to `57.796045`, max-abs from
  `6379.490036` to `609.232339`, and roof net thermal radiation RMSE from
  `161.732738` to `34.308908`. At the November 11 11:00 focus hour, roof
  outside face temperature moves from a `7.576803 C` delta to `0.000329 C`,
  and `SurfHConvExt` analogue moves from a `0.768933` delta to `0.000757`.
  The next active bottleneck is back on zone aggregate outside conduction,
  floor storage, and inside-face coupling rather than exterior HConv/source
  sampling.
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
- The diagnostic CTF seed report preserves each EIO construction's `#CTFs` and
  `Time Step {hours}` metadata so future runtime work can distinguish
  coefficient-term depth from actual history cadence before changing surface
  history advancement.
- Heat-balance report generation writes `compare-digest.json` alongside the
  full `compare-summary.json` and markdown report. The digest keeps manifest,
  warmup, CTF seed, bottleneck, and series-level delta metadata but omits full
  hourly `sample_rows`, so diagnostic gates can validate large official dynamic
  lanes without repeatedly parsing the full trace payload.
- EnergyPlus `UpdateThermalHistories` first computes current CTF inside and
  outside fluxes into `SurfInsideFluxHist(1)` and `SurfOutsideFluxHist(1)`,
  flips the outside flux into `SurfOpaqOutFaceCondFlux` for reporting, then
  shifts the current temperature/flux slots into history slot 2 for the next
  timestep in the `SimpleCTFOnly` path. The Rust history vectors intentionally
  represent EnergyPlus history slot 2 and later, not the current slot 1; the
  remaining mass-floor storage work should therefore target the warmup/run-period
  history handoff and coupled source update order rather than another outside
  report sign flip.
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
  balance/report path, with timestep-interpolated weather/solar/horizontal-IR
  context, EnergyPlus Perez anisotropic sky diffuse for exterior incident
  solar, and hourly-averaged surface diagnostics. Ground boundary surfaces use
  the EnergyPlus default `Site:GroundTemperature:BuildingSurface` value of
  `18.0 C` until explicit `Site:GroundTemperature:*` parsing/model selection
  is ported. The active grey interior-longwave diagnostic now uses
  EnergyPlus-style fixed direct surface view factors, rather than zone-area
  weighting, before applying the grey-pair exchange emissivity. This lowers the
  newly exposed floor inside longwave RMSE from `137.147093` to `27.742006`,
  the floor inside convection RMSE from `123.066168` to `41.950371`, MAT RMSE
  from `0.323407` to `0.117536`, and the zone outside opaque conduction RMSE
  from `84.712495` to `38.774428`; floor storage remains the top active
  diagnostic row at `108.672323` RMSE. Freezing inside convection coefficients
  at timestep start on the same lane modestly improves the analytical
  diagnostic candidate: floor storage RMSE falls to `105.876226`, MAT RMSE to
  `0.116074`, zone surface-convection RMSE to `9.385594`, and floor inside
  convection RMSE to `39.128925`. A coupled third-order zone-air correction on
  that frozen-hconv lane then cuts the floor-focused top rows again
  (`54.593582` floor storage, `31.581604` floor inside conduction,
  `23.282797` floor outside conduction, and `0.069817` MAT RMSE), but exposes a
  latent zone-air report/source-order trade-off because surface-convection and
  air-storage RMSE rise to `29.623453` and `29.666388`. Removing the hconv
  freeze from that third-order lane slightly improves MAT and those latent
  air-balance rows (`0.069191`, `28.637227`, and `28.446243` RMSE), but worsens
  the floor storage/inside/outside conduction rows to `58.289839`, `33.704368`,
  and `24.970278`, so it is only an isolation probe. A report-only
  weather-proxy moist-air storage fork keeps the frozen third-order MAT/floor
  rows unchanged while lowering air-storage RMSE to `5.845285`; the surface
  convection row stays at `29.623453`, keeping source-order/coefficient timing
  as the next zone-air target. A previous-MAT surface-convection report sibling
  rejects the report-reference-temperature hypothesis: MAT/floor/storage rows
  are unchanged, but the zone surface-convection RMSE rises to `104.589141`.
  A balance-surface-convection sibling that syncs self-adiabatic outside faces
  back to the current inside face immediately before CTF history/report commit
  is also rejected in the current Rust interleaving shell: floor storage RMSE
  worsens from `54.593582` to `453.783584`, floor outside conduction from
  `23.282797` to `446.456057`, and MAT from `0.069817` to `0.335157`. This
  rules out a simple post-inside-solve adiabatic outside-history sync as the
  next promotion path; the remaining floor CTF error needs the broader
  EnergyPlus inside/outside/air-balance iteration cadence, not just a final
  history-slot assignment.
  A narrower converged-lane follow-up that preserved reported outside-face
  state but committed adiabatic CTF history temperature/flux from the current
  inside face also regresses, so the rejection is not just a report-state side
  effect: floor storage RMSE rises from `52.022146` to `500.413170`, floor
  outside conduction from `21.976058` to `456.564008`, and the annual floor
  current/history RMSEs jump from about `401 W` to roughly `3800 W` to
  `3909 W`.
  This rules out a history-only current-inside adiabatic commit as the next
  floor-storage lever.
  A converged-lane frozen-outside snapshot probe then holds the outside
  boundary-balance temperature and exterior report terms from the first
  inside-surface pass through subsequent surface iterations. This improves the
  floor CTF/storage cancellation rows relative to the active best
  (`52.022146` to `45.972185` floor storage RMSE, `30.201354` to
  `26.687843` inside conduction, `21.976058` to `19.445141` outside
  conduction, and `27.990507` to `20.835446` aggregate outside conduction),
  but it also regresses `ZN001:ROOF001 / Surface Outside Face Convection Heat
  Gain Rate` from `19.325833` to `67.850650` RMSE. This points to a real
  EnergyPlus outside-snapshot cadence lever for floor CTF cancellation, but
  rejects freezing exterior report terms wholesale; the next narrower probe
  should separate the inside CTF solve's `SurfTempOutHist` snapshot from
  current outside-face report-state generation.
  That narrower converged-lane probe is now wired as an inside-CTF
  outside-history snapshot: the current outside-face balance/report state is
  recalculated each surface pass, but the inside CTF solve's current outside
  temperature term reuses the first-pass outside-face snapshot. It keeps the
  broad frozen-outside floor benefit while avoiding the roof exterior-report
  regression: top floor storage RMSE improves from `52.022146` to `45.539704`,
  floor inside conduction from `30.201354` to `26.437580`, floor outside
  conduction from `21.976058` to `19.262430`, aggregate inside conduction from
  `26.355358` to `23.838450`, aggregate outside conduction from `27.990507` to
  `25.267733`, zone surface convection from `21.105254` to `21.080512`, and
  air storage from `7.547299` to `7.486249`. The roof outside convection RMSE
  only moves from `19.325833` to `19.473624`, so the next runtime promotion
  candidate should carry the inside-CTF-only outside snapshot and leave exterior
  report-state generation current.
  Rechecking the active
  analytical lane with the
  EnergyPlus InitHeatBalance-shaped CTF initial-history seed produces identical
  floor RMSE rows and identical first-sample floor history deltas
  (`1880.111844`/`1769.027186 W`), so the active warmup path washes out that
  initial seed difference. Forcing the Rust warmup to the oracle's 20 run-period
  warmup days only moves top RMSE from `108.672323` to `108.671673` while
  reducing the floor first-sample inside/outside history deltas from
  `1880.111844`/`1769.027186 W` to `1824.704274`/`1713.508910 W`. Raising the
  same interleaved grey-longwave
  lane from 20 to 100 surface iterations slightly worsens the top floor storage
  RMSE (`108.672323` to `108.676973`). A trial final surface pass after the
  active interleaved zone-air correction also worsened top RMSE to `108.674004`
  while barely changing the first-sample floor history deltas, so the active
  mismatch is not explained by a missing final post-correction surface pass. A
  non-interleaved grey-longwave twenty-pass probe reduces the first-sample floor
  history deltas to `1422.936503`/`1243.804233 W`, but it worsens top RMSE to
  `505.688631`, floor outside conduction RMSE to `470.726229`, and floor storage
  RMSE to `505.688631`; the active lane therefore still needs interleaved
  surface/zone coupling while the remaining CTF history mismatch is isolated
  elsewhere in the coupled update path. A
  trial that froze the grey interior-longwave source after the first interleaved
  surface pass was also rejected: top RMSE jumped to `20311.728529`, and the
  floor first-sample inside/outside history deltas jumped to
  `226264.768901`/`226437.862641 W`, so the EnergyPlus `InitSurfaceHeatBalance`
  longwave timing cannot be approximated by simply holding the first-pass source
  while keeping the current Rust interleaving loop. A one-pass full ScriptF
  source still diverges in the current simplified coupling shell. The Rust fixed approximate
  view-factor generation and ScriptF orientation are now unit-checked against
  the `1ZoneUncontrolled` EIO final view-factor/ScriptF values, so the remaining
  ScriptF gap is expected to live in the coupled surface/zone iteration timing
  rather than in the grey interchange matrix itself. The compact diagnostic
  digest now carries first reported sample bottlenecks; in the active lane the
  first-sample outside opaque aggregate delta is `871.308445 W`, driven by
  floor storage/conduction plus underpredicted wall/roof outside conduction,
  while the floor inside net longwave first-sample delta remains
  `404.796794 W`. The digest now also emits Rust-only first-sample CTF
  component rows. In the active lane the mass floor's first sample is dominated
  by CTF history terms (`1229.296987 W` inside and `1297.344600 W` outside),
  while roof/wall no-mass rows have zero history terms and cancel inside/outside
  conduction. The companion oracle-inferred first-sample table derives
  `1546.858233 W` inside and `1136.823976 W` outside floor current zero-term
  values plus `-650.814857 W` inside and `-471.682586 W` outside floor history
  terms from oracle temperatures/rates and EIO zero CTF coefficients. Rust's
  corresponding first-sample current terms are `-798.515769 W` inside and
  `-980.325547 W` outside, so the current-term deltas
  (`2345.374002`/`2117.149523 W`) are even larger than the history deltas
  (`1880.111844`/`1769.027186 W`). That shifts the next EnergyPlus
  source-porting target from history-vector contents alone to the combined
  mass-floor face-temperature/current-term alignment and coupled
  warmup/run-period source handoff. The digest now also emits Rust run-period
  initial CTF history slots captured after warmup and before the first reported
  timestep. In the same active lane, the floor
  run-period initial slot sum is already `1393.986801 W` inside and
  `1607.011644 W` outside before the first hour averages to `1229.296987 W`
  inside and `1297.344600 W` outside, so the current mismatch is present at the
  warmup/run-period handoff rather than being introduced only by first-hour
  averaging. Full inside iteration order, zone predictor/corrector equations,
  detailed
  shadowing/reflection, and coupled radiation coefficient update order are not
  yet wired.
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
