---
status: active
claim_level: planning-guard
owner: runtime
last_reviewed: 2026-07-16
---

# Heat Balance Source Map

Reference version: EnergyPlus 26.1.0

Reference source root:

```text
.reference/energyplus-src/26.1.0/
```

Local audit cache used for the June 2026 diagnostic pass:

```text
.runtime/energyplus-source/v26.1.0/src/EnergyPlus/
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
| Energy Management System dispatch | `src/EnergyPlus/EMSManager.cc::ManageEMS` | deferred; no bounded Rust target |
| global heat-balance data | `src/EnergyPlus/DataHeatBalance.hh` | `ep_model`, `ep_runtime::HeatBalanceState` |
| global heat-balance report registration | `src/EnergyPlus/DataHeatBalance.cc` | `ep_runtime::ResultStore`, conformance report metadata |
| surface heat balance | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `ep_runtime::surface_balance` |
| surface heat-balance declarations | `src/EnergyPlus/HeatBalanceSurfaceManager.hh` | `ep_runtime::surface_balance` |
| surface/material selection state | `src/EnergyPlus/DataSurfaces.cc::GetVariableAbsorptanceSurfaceList` | `ep_model::VariableAbsorptanceSurfaceBinding`, `ep_compiler` |
| per-Surface computed geometry | `src/EnergyPlus/DataSurfaces.cc::SurfaceData::set_computed_geometry` | `ep_model::SurfaceComputedGeometry`, `ep_compiler` |
| incident-solar multiplier request front end | `src/EnergyPlus/HeatBalanceManager.cc::GetIncidentSolarMultiplier` | `ep_model::SurfaceIncidentSolarMultiplierRequest`, `ep_compiler` |
| scheduled inside-solar input first phase | `src/EnergyPlus/HeatBalanceManager.cc::GetScheduledSurfaceGains` | `ep_model::SurfaceSolarIncident`, `ep_compiler` |
| scheduled complex-fenestration layer input second phase | `src/EnergyPlus/HeatBalanceManager.cc::GetScheduledSurfaceGains` | `ep_model::FenestrationSolarAbsorbedRequest`, `ep_compiler` |
| zone air predictor/corrector | `src/EnergyPlus/ZoneTempPredictorCorrector.cc` | `ep_runtime::zone_air` |
| zone air declarations | `src/EnergyPlus/ZoneTempPredictorCorrector.hh` | `ep_runtime::zone_air` |
| internal gains input summaries | `src/EnergyPlus/HeatBalanceInternalHeatGains.cc` | `ep_compiler`, `ep_runtime::internal_gains` |
| internal gains runtime sums | `src/EnergyPlus/InternalHeatGains.cc` | `ep_runtime::internal_gains` |
| output variable registration | `src/EnergyPlus/OutputProcessor.cc` | `ep_conformance`, `ep_runtime::ResultStore` |

## Required Routine Map

| Porting area | EnergyPlus routine or symbol | Current Rust status |
|---|---|---|
| heat-balance driver | `ManageHeatBalance` | mapped-not-ported |
| pre-construction maximum window-layer bound | `SetPreConstructionInputParameters`, called unconditionally from `SimulationManager.cc` line 216 before downstream construction allocation | CP146 adds required `routine.set_pre_construction_input_parameters` as source-mapped only. The source overwrites the shared maximum to 7, raises it to 10 on raw `Construction:ComplexFenestrationState` presence, and scans raw `Construction:WindowEquivalentLayer` objects for the largest positional alpha span. Rust's dynamic vectors and separate 8/11 validation limits do not implement this shared mutable bound, call order, input-buffer effects, downstream allocation contract, or failure behavior. |
| site atmospheric height variation | `GetSiteAtmosphereData`, called from `GetHeatBalanceInput` line 264 between project controls and spectral/material input | CP147 adds required `routine.get_site_atmosphere_data` as source-mapped only. Zero, one, and duplicate `Site:HeightVariation` counts preserve distinct Terrain-derived/default, numeric-prefix override, and diagnostic paths before an unconditional EIO header/row. Rust's Terrain helper and fixed temperature-gradient path do not implement this input routine, shared state, output, or lifecycle. |
| project heat-balance controls | `GetProjectControlData` | mapped-not-ported |
| material input | `Material::GetWindowGlassSpectralData` -> `Material::GetMaterialData` -> `Material::GetHysteresisData` | all 34 public base/overlay objects are inventoried in [the material-family source map](material-source-map.md); Regular, NoMass, AirGap, InfraredTransparent, RefractionExtinctionMethod, and EquivalentLayer plus only the `WindowMaterial:Glazing` `SpectralAverage` branch are typed, while equivalent-layer topology/ASHWAT consumers and full window behavior remain blocked |
| window frame/divider input | `GetFrameAndDividerData` | EnergyPlus places this routine after hysteresis and before construction input; Rust types the complete bounded object after base `parse_materials` and before `parse_constructions`, while its separate Hysteresis pass remains later, so complete pass-order parity is not claimed; every definition, including unused records, remains runtime-blocking |
| construction input | `GetConstructData` / `CreateFCfactorConstructions` / `CreateAirBoundaryConstructions` / `SetupComplexFenestrationStateInput` / `SearchWindow5DataFile` | the parent remains source-mapped; required ordinary names/layers, bounded opaque/fenestration validation, thermochromic first-state metadata, sole-layer SimpleGlazingSystem, the inline lexical InternalHeatSource overlay, the following WindowEquivalentLayer declaration state, and the final WindowDataFile request selector are typed; the F-then-C generator is state-mapped with private raw-count internal materials and exact formulas; AirBoundary is state-mapped as lexical-order zero-layer declaration state; the complex-state pass is state-mapped for bounded LBNLWINDOW/None graph state; every special construction definition/request is run-blocked while SearchWindow5DataFile expansion, equivalent-layer ASHWAT consumers, surface consumers, CTF/QTF/reporting, and window/ground/source-sink execution remain deferred |
| building/zone/space input | `GetBuildingData` -> `GetZoneData` -> `ProcessZoneData` / `GetZoneLocalEnvData` / `GetSpaceData` / `GetGeneralSpaceTypeNum` -> `SetupZoneGeometry` / `GetHTSurfaceData` / `CreateMissingSpaces` | the HeatBalanceManager wrappers plus `SetupZoneGeometry`, `GetSurfaceData`, and full `GetHTSurfaceData` remain source-mapped; bounded declaration state continues through Space/SpaceList/default-Space creation, then a CP97 composite bounds the GetHT-owned typed detailed-surface Space lookup and same-Zone validation before state-mapped `CreateMissingSpaces` consumes preclassified assignments, creates mixed-assignment remainders, and applies final fallback SpaceIds. Collections, local environments, authored Spaces, SpaceLists, and generated remainders run-block; sole whole-zone defaults remain inactive when no remainder is needed, including when every valid detailed surface explicitly references the default. Other surface families, opposite-surface generation, reordering, Space surface lists, geometry realization, list consumers, outputs, and runtime convection remain deferred. |
| variable-absorptance surface selection | `DataSurfaces::GetVariableAbsorptanceSurfaceList` immediately after `GetBuildingData` | CP98 state-maps the bounded retained `BuildingSurface:Detailed` subset: dense-order Outdoors surfaces whose construction outside layer owns a typed overlay receive immutable `VariableAbsorptanceSurfaceBinding` identities; non-Outdoors outside-layer uses warn without binding, followed by occurrence-local warnings for every typed construction layer after layer 1. Every overlay remains runtime-blocking. Full `AllHTSurfaceList` membership/reorder parity, other surface families, exact warning text/order/multiplicity outside typed arenas, `UpdateVariableAbsorptances`, runtime numerics, and conformance remain deferred. |
| incident-solar multiplier input | `GetIncidentSolarMultiplier` immediately after `DataSurfaces::GetVariableAbsorptanceSurfaceList` | CP99 types only immutable `SurfaceProperty:IncidentSolarMultiplier` request snapshots: dense typed ID, nonsemantic normalized declaration key, unresolved normalized window target, inclusive-[0,1] multiplier defaulting to 1.0, and optional resolved ScheduleId. Missing schedules and duplicate normalized targets fail closed, no source order is claimed, and every request run-blocks. The routine remains source-mapped because fenestration-surface identity, Window/exterior/construction/shade validation, per-surface mutation, source duplicate overwrite semantics, schedule evaluation, runtime, and conformance remain deferred. |
| scheduled surface gains input | `GetScheduledSurfaceGains` immediately after `GetIncidentSolarMultiplier` | CP100 types the first `SurfaceProperty:SolarIncidentInside` phase as immutable `SurfaceSolarIncident` records with a semantic normalized name, typed detailed-opaque SurfaceId, any typed ConstructionId, and required ScheduleId. CP101 types the following `ComplexFenestrationProperty:SolarAbsorbedLayers` phase as immutable requests with a semantic normalized name, unresolved fenestration target, complex-fenestration ConstructionId, and ordered ScheduleIds matching its solid optical layers. Both families allow duplicate names, fail closed on repeated resolved target/construction pairs because no source order is claimed, and run-block every definition. CP102 state-maps only a diagnostic tail slice of `CheckScheduledSurfaceGains`: an error-free Zone warns nonblockingly when its retained typed opaque subset already contains both an exact current-construction pair match and a miss. Empty, all-matched, and all-unmatched subsets stay silent. `GetScheduledSurfaceGains`, representative-surface mutation, full completeness, `SurfaceScheduledSolarInc`, `WindowScheduledSolarAbs`, schedule sampling, runtime, and conformance remain source-mapped or unsupported. |
| representative-surface assignment output barrier | inline `GetHeatBalanceInput` block immediately after `GetScheduledSurfaceGains` | mapped and deferred without a synthetic routine entry: when representative calculations are enabled, EnergyPlus writes one EIO header even for zero surfaces, then visits the complete global Surface array in order and writes only non-self representative assignments. Rust lacks the controlling project flag, complete Surface arena/order, representative/constituent mutation, and EIO writer needed for a truthful partial implementation. |
| thermochromic child construction projection | `CreateTCConstructions` immediately after the representative-surface EIO block | CP103 state-maps a bounded immutable series/child projection from CP85 master metadata and ordered thermochromic states. Master ConstructionId order, state order including the first state, effective-layer cloning, final-master-layer substitution, initial temperature, and source-shaped generated names are retained in separate arenas. Main ConstructionIds/names/counts/graph state, WINDOW5-relative global append order, deep-copy state, active switching, reporting, and runtime remain deferred. |
| no-Zone simulation validity diagnostic | inline `TotSurfaces > 0 && NumOfZones == 0` gate and `CheckValidSimulationObjects` immediately after `CreateTCConstructions` | CP104 state-maps a bounded diagnostic-only positive witness: absent prior Errors, an empty typed Zone arena plus raw `Shading:Site:Detailed` or `Shading:Building:Detailed` presence emits `InvalidSimulationWithoutZones` unless any one of the source's eight allowed collector/generator families is present in raw input. No model state is written. Full `TotSurfaces` parity, all other surface families, exact diagnostics/fatal sequencing, and collector/generator typing or runtime remain deferred. |
| positive construction-use evidence | `CheckUsedConstructions` immediately after the no-Zone gate | CP105 state-maps only monotonic positive evidence. Rust resolves retained typed-surface ConstructionIds plus the six source-ordered raw reference families, stores sorted/deduplicated known-used IDs, and separately stores sorted/deduplicated known-CTF-used IDs only for non-window GroundHeatExchanger and EMS references. Missing, blank, wrong-type, and unresolved raw references stay silent. Absence remains unknown: no `IsUsed=false`/`IsUsedCTF=false`, unused count/name/warning, `DisplayExtraWarnings`, CTF/CondFD selection, runtime, or support claim is added. |
| input-completion fatal barrier | inline `GetHeatBalanceInput` `ErrorsFound` check immediately after `CheckUsedConstructions` | mapped and deferred without a synthetic routine entry. EnergyPlus terminates immediately at source lines 311-313 before enclosure or internal-gain initialization. A failed Rust compile eventually returns `model = None`, which is a coarse fail-closed outcome, not exact parity for this short-circuit point, accumulated-diagnostic order, fatal text, or side effects. |
| solar enclosure view-factor initialization | `HeatBalanceIntRadExchange::InitSolarViewFactors` at the parent call on `HeatBalanceManager.cc` line 316 | source-mapped and required for the full domain. It depends on the `ViewFactorInfo` report option and EIO/debug writers, aligned user view factors, complete Solar enclosure and Space heat-transfer-surface lists, AirBoundary merging/exclusion, Surface pointers and global report order, area/azimuth/tilt/inside-solar-absorptance state, zero- and one-surface branches, approximate or user matrices, InternalMass detection, `FixViewFactors`, and warning/fatal/report side effects. Existing Rust approximate/fix helpers and 1Zone EIO evidence do not promote this routine. |
| internal-gain manager | `ManageInternalHeatGains(state, true)` at the parent call on `HeatBalanceManager.cc` line 320 | source-mapped and required for the full domain. CP107 does not implement the persistent one-time input flag, general `InitOnly` behavior, the non-init recurring branches, full internal-gain input, daylighting/reporting setup, or runtime gain updates. |
| bounded internal-gain input | `GetInternalHeatGainsInput` reached by the init-only manager call | CP107 state-maps only the direct-Zone People then OtherEquipment family slice. The wrapper returns on a pre-existing Error, but a People diagnostic created inside the pass does not prevent the OtherEquipment scan. The existing typed arenas and name maps are the only mapped state; all other families, target expansion, derived occupant/design-level state, reporting, runtime, and conformance remain deferred. |
| conditional Kiva instance setup | `if (AnyKiva) kivaManager.setupKivaInstances(state)` at `HeatBalanceManager.cc` lines 322-325 | CP108 source-maps the conditional call and its ignored boolean result only; it is not required for the full domain and has no Rust target. Foundation input, geometry, instance ownership, weather/ground algorithms, diagnostics, outputs, runtime, and conformance remain deferred. |
| sizing Space heat-balance mode override | inline `if (DoingSizing) doSpaceHeatBalance = doSpaceHeatBalanceSizing` at `ManageHeatBalance` lines 169-171 after `GetHeatBalanceInput` returns | CP109 maps and defers this caller branch without a synthetic routine or Rust helper. Input ownership, the sizing lifecycle, mutable mode/flag state, Space heat-balance consumers, and runtime remain unclaimed. |
| conditional Surface octree initialization | nested `TotSurfaces >= 100` and raw `Daylighting:Controls` count guards at `ManageHeatBalance` lines 173-180, then `SurfaceOctreeCube::init` | CP110 source-maps the complete-Surface call and octree structure only; it is not required for the full domain and has no Rust target. Complete surface identity/order, daylighting typing, mutable transparency, traversal, computed geometry, runtime, performance, and conformance remain deferred. |
| bounded per-Surface computed geometry | complete global Surface loop at `ManageHeatBalance` lines 182-184, then `SurfaceData::set_computed_geometry` | CP111 state-maps an error-free retained `BuildingSurface:Detailed` subset through `Compiler::set_bounded_surface_computed_geometry`: finite, coplanar, nondegenerate Triangles and conservative source-recognized Rectangles only. Derived shape category, Newell plane, axis projection, bounds, wrap edges, and rectangle side squares attach to each retained Surface without adding identity, count, graph, support, runtime, or conformance claims. |
| one-time heat-balance input-flag clear | inline `ManageHeatBalanceGetInputFlag = false` at `ManageHeatBalance` line 186 | CP112 maps and defers this inline lifecycle boundary without a synthetic routine or Rust target. The source flag defaults and resets to true, remains true through the once-only input tail, and is cleared only after successful reach past the computed-geometry loop; Rust owns no persistent equivalent flag or re-entry behavior. |
| pre-initialization EMS calling point | unconditional `ManageEMS(state, BeginZoneTimestepBeforeInitHeatBalance, anyRan, absent)` at `ManageHeatBalance` lines 189-194 | CP113 source-maps the canonical generic `ManageEMS` routine, not required for the full domain, with no Rust target. EMS setup, language execution, callbacks, optional manager selection, run flags, side effects, runtime behavior, and conformance remain deferred. |
| heat-balance initialization | unconditional `InitHeatBalance(state)` at `ManageHeatBalance` line 198, implemented at lines 2594-2821 | CP114 source-maps this canonical routine and makes it required for the full heat-balance domain. Existing Rust source-order metadata, identity wrapper, and bounded state construction do not implement the complete flag-driven EnergyPlus initialization lifecycle. |
| Zone/Space heat-balance core allocation | `AllocateZoneHeatBalArrays`, first action of `AllocateHeatBalArrays` and reached from the `InitHeatBalance` BeginSim branch | CP148 adds required `routine.allocate_zone_heat_bal_arrays` as source-mapped only. It conditionally backfills the internal-gain bundle, unconditionally reconstructs Zone/Space heat-balance records, and allocates/zeros eight solar-enclosure arrays. Existing Rust vectors and initialization shells do not implement this allocation order, defaults, partial-failure state, destructive re-entry, or lifecycle. |
| complete heat-balance array allocation | `AllocateHeatBalArrays`, reached from the `InitHeatBalance` BeginSim branch | CP149 adds required `routine.allocate_heat_bal_arrays` as source-mapped only. After the separate CP148 child, it dimensions FanSystem accumulators and warmup history, conditionally dimensions contaminant state, and allocates resilience/report matrices in exact source order. Existing Rust initialization shells do not implement the complete state, conditional preservation, defaults, partial-failure state, or re-entry semantics. |
| conduction transfer-function initialization | `InitConductionTransferFunctions`, reached after `AllocateHeatBalArrays` under `BeginSimFlag && (AnyCTF || AnyEMPD)` | CP153 adds required `routine.init_conduction_transfer_functions` as source-mapped only. The wrapper resets/calculates every Construction in array order, derives global simple/max state, performs used-only layer and optional detailed CTF/QTF reporting, and fatals last on accumulated errors. Rust consumes EIO-seeded coefficients or a steady no-history fallback but has no native generator. |
| post-initialization EMS calling point | unconditional `ManageEMS(state, BeginZoneTimestepAfterInitHeatBalance, anyRan, absent)` at `ManageHeatBalance` lines 199-200 | CP115 reuses the existing generic `routine.manage_ems` source mapping. The same caller-owned `anyRan` is passed again and overwritten; no routine, project-contract, Rust-state, support, or conformance entry is added. |
| surface heat-balance manager | unconditional `HeatBalanceSurfaceManager::ManageSurfaceHeatBalance(state)` at `ManageHeatBalance` line 209, implemented at `HeatBalanceSurfaceManager.cc` lines 145-230 | CP116 expands the existing required `routine.manage_surface_heat_balance` source mapping without a new row. The complete parent-driver order and gates are mapped, but the current Rust stage metadata, identity wrapper, and limited runtime path do not implement or match that complete order. |
| first-time surface initialization display | inline `if (ManageSurfaceHeatBalancefirstTime) DisplayString(state, "Initializing Surfaces")` at `ManageSurfaceHeatBalance` lines 158-160 | CP117 maps this inline lifecycle/output guard without a synthetic routine. The flag defaults and resets to true in `HeatBalanceSurfaceManager.hh`, remains true through all four progress guards, and is cleared only at the successfully reached parent tail; Rust owns no equivalent persistent flag or progress output. |
| surface heat-balance initialization | unconditional `InitSurfaceHeatBalance(state)` at `ManageSurfaceHeatBalance` line 161, implemented at `HeatBalanceSurfaceManager.cc` lines 272-621 | CP118 adds the canonical required `routine.init_surface_heat_balance` source mapping and project-contract entry. The existing Rust stage and identity wrapper are metadata dependencies around only a limited outside path; they are intentionally not added as an algorithm target or promoted to mapped state. |
| complete Surface heat-balance allocation and output setup | `AllocateSurfaceHeatBalArrays`, reached by `InitSurfaceHeatBalance` line 350 under `BeginSimFlag` | CP155 adds required `routine.allocate_surface_heat_bal_arrays` as source-mapped only. It defines 136 distinct allocation fields across six owners and issues 78 output-setup call sites for the complete conditional heat-transfer-Surface reporting family plus the site total; Rust has no complete allocation/output-setup or lifecycle analog. |
| initial thermal/flux histories and boundary seed state | `InitThermalAndFluxHistories`, reached by `InitSurfaceHeatBalance` line 383 under `BeginEnvrnFlag` | CP156 adds required `routine.init_thermal_and_flux_histories` as source-mapped only. It reconstructs Zone/Space air state, resets reachable Surface/report/window histories, then seeds base/master/source CTF histories and cavity, Kiva, and OSCM state in exact source order. Rust's optional `EnergyPlusSurfInitial` seed is only a bounded configurable-temperature, typed-boundary, steady-`1/R` analogue. |
| exterior movable-insulation evaluation | `EvalOutsideMovableInsulation`, first call in the `InitSurfaceHeatBalance` lines-387-390 `AnyMovableInsulation` block | CP157 adds non-required `routine.eval_outside_movable_insulation` as source-mapped only. Stored exterior-movable-Surface order and current/EMS schedule values select active-construction outside-layer properties or movable-material conductance and properties, with exact inactive preservation and partial-prefix behavior. Rust has no movable-insulation analog. |
| interior movable-insulation evaluation | `EvalInsideMovableInsulation`, second call in the `InitSurfaceHeatBalance` lines-387-390 `AnyMovableInsulation` block | CP158 adds non-required `routine.eval_inside_movable_insulation` as source-mapped only. Stored interior-movable-Surface order and current/EMS schedule values select active-construction inside absorptances or movable-material conductance and absorptances, preserving inactive `H` and exact `presentPrevTS`; unlike CP157 it writes no roughness. Rust has no movable-insulation analog. |
| predefined Surface/Construction report gathering | `GatherForPredefinedReport`, reached by `InitSurfaceHeatBalance` at line 481 under `BeginSimFlag` | CP154 adds non-required `routine.gather_for_predefined_report` as source-mapped only. It traverses the report-order Surface list, emits optional Construction EIO rows, and appends predefined Surface, fenestration, total, and count entries; Rust has no corresponding predefined-report or NFRC/fenestration topology implementation. |
| first-time outside surface-balance display | inline `if (ManageSurfaceHeatBalancefirstTime) DisplayString(state, "Calculate Outside Surface Heat Balance")` at `ManageSurfaceHeatBalance` lines 165-167 | CP119 maps this shared-flag output/lifecycle point after `InitSurfaceHeatBalance` returns and immediately before line-168 `CalcHeatBalanceOutsideSurf`. It adds no synthetic routine, project-contract entry, Rust state, output, support, count, or conformance promotion. |
| outside surface balance | unconditional `CalcHeatBalanceOutsideSurf(state)` at `ManageSurfaceHeatBalance` line 168, implemented at `HeatBalanceSurfaceManager.cc` lines 6951-7721 | CP120 adds the canonical required `routine.calc_heat_balance_outside_surf` source mapping and project-contract entry. The caller omits `ZoneToResimulate`, selecting the complete normal call rather than the optional resimulation subset. Existing Rust stage metadata, identity wrapper, and bounded retained opaque CTF/environmental balance and report terms do not implement or promote the complete routine topology, order, branches, state, or numerics. |
| first-time inside surface-balance display | inline `if (ManageSurfaceHeatBalancefirstTime) DisplayString(state, "Calculate Inside Surface Heat Balance")` at `ManageSurfaceHeatBalance` lines 169-171 | CP121 maps this shared-flag output/lifecycle point only after the unconditional outside-balance call returns and immediately before line-172 `CalcHeatBalanceInsideSurf`. It adds no synthetic routine, project-contract entry, Rust state, output, support, count, or conformance promotion. |
| inside surface balance | unconditional `CalcHeatBalanceInsideSurf(state)` at `ManageSurfaceHeatBalance` line 172, implemented at `HeatBalanceSurfaceManager.cc` lines 7738-7813 | CP122 adds canonical required `routine.calc_heat_balance_inside_surf` and its project-contract entry. The parent omits `ZoneToResimulate`, selecting the complete-building dispatch. The canonical wrapper's first-call/environment lifecycle, radiant-HVAC aggregation, optimized-AllCTF/general/partial dispatch, MRT and intermediate-result tail, dependency behavior, state, errors, and numerics remain only source-mapped. |
| first-time air heat-balance display | inline `if (ManageSurfaceHeatBalancefirstTime) DisplayString(state, "Calculate Air Heat Balance")` at `ManageSurfaceHeatBalance` lines 176-178 | CP123 maps this shared-flag output/lifecycle point only after the unconditional inside-balance call returns and immediately before line-179 `ManageAirHeatBalance`. It adds no synthetic routine, project-contract entry, Rust state, output, support, count, or conformance promotion. |
| air heat-balance manager | unconditional `HeatBalanceAirManager::ManageAirHeatBalance(state)` at `ManageSurfaceHeatBalance` line 179 | CP124 reuses the existing required canonical routine to map the caller, body order, and distinct one-time lifecycles without adding a duplicate routine or project-contract entry |
| final surface heat-balance update | unconditional `UpdateFinalSurfaceHeatBalance(state)` at `ManageSurfaceHeatBalance` line 184, implemented at `HeatBalanceSurfaceManager.cc` lines 5176-5219 | CP125 adds canonical required `routine.update_final_surface_heat_balance`. Seven averaged equipment-source updaters always run; if any reports a nonzero averaged source, the complete-building outside balance and then inside balance run again. Existing Rust stage metadata and bounded final-state wrapping are a scaffold target, not implementation parity. |
| thermal-history update | `AnyCTF || AnyEMPD`-guarded `UpdateThermalHistories(state)` at `ManageSurfaceHeatBalance` lines 186-189, implemented at `HeatBalanceSurfaceManager.cc` lines 5221-5581 | CP126 adds canonical required `routine.update_thermal_histories`. The source computes current CTF/EMPD flux and report state, then selects either the `SimpleCTFOnly && !AnyConstrOverridesInModel` fast shift or the normal capture, counter, rollover/interpolation, and optional internal-source history path. Existing Rust stage metadata and bounded vector-history lane are not full parity. |
| CondFD moisture-history update | independent `AnyCondFD`-guarded complete-Surface loop at `ManageSurfaceHeatBalance` lines 191-206, calling inline `SurfaceDataFD::UpdateMoistureBalance` at line 204; helper body in `HeatBalFiniteDiffManager.hh` lines 175-182 | CP127 adds non-required `routine.surface_data_fd_update_moisture_balance`. The parent skips only declared construction numbers at most zero, window constructions, and non-CondFD algorithms; each survivor copies `T` to `TOld`, `Rhov` to `RhovOld`, and `TDreport` to `TDOld`. No Rust/state/support/conformance claim is added. |
| thermal-comfort manager | unconditional `ThermalComfort::ManageThermalComfort(state, false)` at `ManageSurfaceHeatBalance` line 208, implemented at `ThermalComfort.cc` lines 105-164 | CP128 adds non-required `routine.manage_thermal_comfort`. The false argument disables the initialization-only early return, so after shared first-time setup and six-AM temperature maintenance the non-sizing/non-warmup comfort children run under their exact source gates. No Rust/state/support/conformance claim is added. |
| Surface heat-balance reporting | unconditional `ReportSurfaceHeatBalance(state)` at `ManageSurfaceHeatBalance` line 210, implemented at `HeatBalanceSurfaceManager.cc` lines 6605-6891 | CP129 adds required `routine.report_surface_heat_balance`. The source orders shading and representative projection, surface/window/opaque reporting, optional heat-emission and movable-insulation work, component-load snapshots, and advanced Zone accumulation. Existing Rust reporting stages remain bounded scaffolds; no support or conformance claim is promoted. |
| sizing Surface component-load gathering | `ZoneSizingCalc`-guarded `OutputReportTabular::GatherComponentLoadsSurface(state)` at `ManageSurfaceHeatBalance` lines 211-213, implemented at `OutputReportTabular.cc` lines 15064-15132 | CP130 adds non-required `routine.gather_component_loads_surface`. Entered calls still do work only when a component-load report is requested and pulse sizing is false; the routine resets and accumulates only the instantaneous fenestration-conduction sequences. No Rust/state/support/conformance claim is added. |
| thermal-resilience timestep calculation | unconditional `CalcThermalResilience(state)` at `ManageSurfaceHeatBalance` line 215, implemented at `HeatBalanceSurfaceManager.cc` lines 5707-5799 | CP131 adds non-required `routine.calc_thermal_resilience`. Shared first-time output setup and exact-request discovery precede independently gated Heat Index and Humidex calculations; inactive values are retained. No Rust/state/support/conformance claim is added. |
| thermal-resilience summary accumulation | independent `displayThermalResilienceSummary` guard at `ManageSurfaceHeatBalance` lines 217-219, calling `ReportThermalResilience(state)` at `HeatBalanceSurfaceManager.cc` lines 5801-6388 | CP132 adds non-required `routine.report_thermal_resilience`. Period electricity is accumulated before initialization and weather/warmup gating; annual and active-period safety, discomfort, Heat Index, Humidex, SET, and unmet-degree-hour state then follows the source's exact People/Zone cadence and retained quirks. No Rust/state/support/conformance claim is added. |
| CO2-resilience summary accumulation | independent `displayCO2ResilienceSummary` guard at `ManageSurfaceHeatBalance` lines 221-223, calling `ReportCO2Resilience(state)` at `HeatBalanceSurfaceManager.cc` lines 6390-6479 | CP133 adds non-required `routine.report_co2_resilience`. Its one-time allocation can permanently disable the summary when CO2 simulation is absent; weather/non-warmup calls then accumulate exact annual and active-period safe/caution/hazard hours plus period electricity, retaining the source's occupancy, period-allocation, lifecycle, and output-writer quirks. No Rust/state/support/conformance claim is added. |
| visual-resilience summary accumulation | independent `displayVisualResilienceSummary` guard at `ManageSurfaceHeatBalance` lines 225-227, calling `ReportVisualResilience(state)` at `HeatBalanceSurfaceManager.cc` lines 6481-6603 | CP134 adds non-required `routine.report_visual_resilience`. Its one-time allocation can permanently disable the summary when no daylight controls exist; weather/non-warmup calls then accumulate exact annual and active-period illuminance bins plus period electricity, retaining the source's occupancy, reference-point, period-allocation, lifecycle, and writer quirks. No Rust/state/support/conformance claim is added. |
| Surface-manager shared first-time tail | inline unconditional `ManageSurfaceHeatBalancefirstTime = false` at `ManageSurfaceHeatBalance` line 229 | CP135 maps the successful-parent-tail lifecycle boundary only. The bool defaults/resets true, gates four progress displays plus CP131 registration/request scanning, and becomes false only after every entered child returns. No synthetic routine, source, project-contract, Rust/state, support, count, or conformance claim is added. |
| pre-zone-reporting EMS calling point | unconditional `ManageEMS(state, EndZoneTimestepBeforeZoneReporting, anyRan, absent)` at `ManageHeatBalance` line 210 | CP136 reuses non-required `routine.manage_ems`. The generic routine overwrites the same caller-owned `anyRan`, and any actuator commit occurs after the Surface solve with no re-solve before line-211 record keeping. No duplicate routine, source, project-contract, Rust/state, support, count, or conformance claim is added. |
| heat-balance record keeping | unconditional `RecKeepHeatBalance(state)` at `ManageHeatBalance` line 211, declared at `HeatBalanceManager.hh` line 134 and implemented at `HeatBalanceManager.cc` lines 2971-3057 | CP137 adds required `routine.rec_keep_heat_balance`. It records Zone extrema and two-sample histories, optionally emits detailed warmup EIO, snapshots movable-insulation presence, and unconditionally updates non-BSDF Window face-temperature reports. Existing Rust execution-plan metadata remains scaffolding only. |
| non-BSDF Window face-temperature report handoff | `UpdateWindowFaceTempsNonBSDFWin`, the last executable action of `RecKeepHeatBalance` | CP150 adds required `routine.update_window_face_temps_non_bsdf_win` as source-mapped only. It trusts the stored `AllHTWindowSurfaceList`, uses each current mutable `Surface.Construction` to skip BSDF, and copies outside/inside history term 1 into the front-layer-1 and back-`TotLayers` report cells. Rust blocks fenestration runtime and has no analog. |
| external-shading CSV header | `OpenShadingFile`, guarded in `InitHeatBalance` by `BeginSimFlag && DoWeathSim && ReportExtShadingSunlitFrac` | CP151 adds non-required `routine.open_shading_file` as source-mapped only. It conditionally opens or suppresses the external-shading CSV, then writes the literal first field and every Surface name in numeric order with trailing commas. Rust has no corresponding report flag, file lifecycle, header writer, or all-Surface sunlit-fraction export. |
| daily storm-window control | `SetStormWindowControl`, called by `InitHeatBalance` only under `TotStormWin > 0 && BeginDayFlag` | CP152 adds non-required `routine.set_storm_window_control` as source-mapped only. It updates current/previous flags in StormWindow declaration order from inclusive/wrapping fixed-calendar dates with the input off date made exclusive, and latches the shared daily-change flag. The caller-owned `ChangeSet` cadence and active-construction remainder stay dependency context; Rust has no storm-window analog. |
| heat-balance reporting | unconditional `ReportHeatBalance(state)` at `ManageHeatBalance` line 217, declared at `HeatBalanceManager.hh` line 142 and implemented at `HeatBalanceManager.cc` lines 3321-3418 | CP138 adds required `routine.report_heat_balance`. Schedule reporting always runs before the mutually exclusive normal, warmup-reporting, external-interface fallback, or no-output paths. Existing Rust report-stage, composite-plan, prebinding, and bounded result-store metadata remain scaffolding only. |
| post-zone-reporting EMS calling point | unconditional `ManageEMS(state, EndZoneTimestepAfterZoneReporting, anyRan, absent)` at `ManageHeatBalance` line 219 | CP139 reuses non-required `routine.manage_ems`. The generic routine overwrites the same caller-owned `anyRan`; any entered actuator commit occurs only after CP138 reporting and cannot retroactively alter that already emitted output. No duplicate routine, source, project-contract, Rust/state, support, count, or conformance claim is added. |
| EMS trend-variable history update | unconditional `UpdateEMSTrendVariables(state)` at `ManageHeatBalance` line 221, declared at `EMSManager.hh` line 122 and implemented at `EMSManager.cc` lines 1444-1479 | CP140 adds non-required `routine.update_ems_trend_variables`. After two quick-return gates, valid positive pointer/depth entries shift their 1-based histories newest-first in declaration order. Input allocation, setup diagnostics, time arrays, and environment reset remain source-only dependencies. |
| Python plugin value/history update | unconditional `PluginManagement::PluginManager::updatePluginValues(state)` at `ManageHeatBalance` line 222, declared at `PluginManager.hh` line 198 and implemented at `PluginManager.cc` lines 1458-1467 | CP141 adds non-required `routine.update_plugin_values`. Only `LINK_WITH_PYTHON` builds execute the body, which directly visits stored-order plugin trends, pushes each current global value at index 0, and drops the oldest value. Rust plugin execution remains unsupported. |
| warmup convergence | `WarmupFlag && EndDayFlag` outer guard at `ManageHeatBalance` lines 224-226 calling `CheckWarmupConvergence(state)`, declared at `HeatBalanceManager.hh` line 136 and implemented at `HeatBalanceManager.cc` lines 3059-3226 | CP142 adds required `routine.check_warmup_convergence`. The source tests every Zone's daily temperature extrema and heating/cooling-load change, advances previous-day state, emits bounded maximum-day diagnostics, and updates `WarmupFlag`. Existing Rust warmup code uses a different temperature-only diagnostic loop and is not promoted. |
| post-warmup day-counter reset | inner `if (!WarmupFlag)` at `ManageHeatBalance` lines 227-229, assigning `DayOfSim = 0` and then `DayOfSimChr = "0"` | CP143 maps this inline mutation without a synthetic routine. The branch also follows zero-Zone and maximum-day forced exits, and its resets precede the CP144 EMS calling point. No Rust/state/support/conformance claim is promoted. |
| post-warmup EMS calling point | in-branch `ManageEMS(state, BeginNewEnvironmentAfterWarmUp, anyRan, absent)` at `ManageHeatBalance` line 231 | CP144 reuses non-required `routine.manage_ems`. Generic dispatch resets `anyRan`, initializes EMS, runs matching callbacks/plugins and managers, and conditionally commits actuators; the distinct `BeginNewEnvironment`-only initialization hooks do not run. No Rust/state/support/conformance claim is promoted. |
| warmup-convergence summary report | `!WarmupFlag && EndDayFlag && DayOfSim == 1 && !DoingSizing`-guarded `ReportWarmupConvergence(state)` at `ManageHeatBalance` lines 235-237, declared at `HeatBalanceManager.hh` line 138 and implemented at `HeatBalanceManager.cc` lines 3228-3301 | CP145 adds required `routine.report_warmup_convergence`. It writes a one-state-lifetime EIO header and Zone rows from first-nonwarmup-day samples, normalizing stored load differences in place and reporting population standard deviations. Existing Rust warmup summaries are not promoted. |
| zone air updates | `ManageZoneAirUpdates` | diagnostic shell only |
| zone air correction | `correctZoneAirTemps` | mapped-not-ported |
| internal convective gains | `zoneSumAllInternalConvectionGains` | conformance trace exists for `internal_gains_001` only |
| space internal convective gains | `spaceSumAllInternalConvectionGains` | mapped-not-ported |

## Call Order Boundary

The first v0.8 heat-balance candidate must preserve this source-derived order
unless the deviation is documented in a case-specific waiver:

1. `ManageHeatBalance`
2. input acquisition through project controls, materials, frame-and-divider properties, constructions, then `GetBuildingData` in its `GetShadowingInput` -> `GetZoneData` -> `SetupZoneGeometry` order, followed by `DataSurfaces::GetVariableAbsorptanceSurfaceList`, `GetIncidentSolarMultiplier`, `GetScheduledSurfaceGains`, the inline representative-surface EIO assignment barrier, `CreateTCConstructions`, the inline no-Zone validity gate with `CheckValidSimulationObjects`, `CheckUsedConstructions`, the immediate inline fatal barrier, `HeatBalanceIntRadExchange::InitSolarViewFactors` at line 316, `ManageInternalHeatGains(state, true)` at line 320, and conditional Kiva setup at lines 322-325; after `GetHeatBalanceInput` returns, the caller conditionally applies the sizing Space heat-balance mode at lines 169-171, conditionally initializes the Surface octree at lines 173-180, then visits the complete Surface array at lines 182-184 for `set_computed_geometry` before clearing `ManageHeatBalanceGetInputFlag` at line 186. CP100 and CP101 type the scheduled-gain routine's two public input families, CP102 bounds its diagnostic tail, CP103 bounds only an immutable thermochromic child projection while the intervening output block remains deferred, CP104 bounds only positive no-Zone invalidity witnesses while leaving the exact parent gate source-mapped, CP105 collects only sorted/deduplicated positive construction-use evidence without inferring any unused state, CP106 source-maps the fatal barrier plus `InitSolarViewFactors`, CP107 source-maps `ManageInternalHeatGains` while preserving only the bounded direct-Zone People-before-OtherEquipment input slice, CP108 source-maps only the conditional `setupKivaInstances` call, CP109 maps/defers only the inline sizing override, CP110 source-maps only the guarded `SurfaceOctreeCube::init`, CP111 state-maps only bounded retained detailed-opaque Triangle and conservative Rectangle computed geometry, CP112 maps/defers only the line-186 one-time flag clear without claiming persistent Rust lifecycle parity, CP113 source-maps the canonical generic `ManageEMS` routine at the unconditional `BeginZoneTimestepBeforeInitHeatBalance` caller checkpoint without adding a Rust target, CP114 makes the following unconditional `InitHeatBalance` call a required source mapping without promoting existing Rust initialization state, CP115 maps the second unconditional `ManageEMS` caller checkpoint at `BeginZoneTimestepAfterInitHeatBalance` by reusing `routine.manage_ems` without a new row, CP116 expands the existing required `routine.manage_surface_heat_balance` row for the unconditional line-209 call and complete source parent order, CP117 maps the inline first-time `Initializing Surfaces` display guard at `HeatBalanceSurfaceManager.cc` lines 158-160 without a synthetic routine, and CP118 adds the following unconditional line-161 `InitSurfaceHeatBalance(state)` call and lines 272-621 implementation as a new required source-mapped routine/project entry; CP119 maps the following first-time `Calculate Outside Surface Heat Balance` display guard at lines 165-167 without a synthetic routine; CP120 adds the unconditional line-168 `CalcHeatBalanceOutsideSurf(state)` call and lines 6951-7721 implementation as a new required source-mapped routine/project entry; CP121 maps the first-time inside-balance display at lines 169-171 without a synthetic routine; CP122 adds the unconditional line-172 `CalcHeatBalanceInsideSurf(state)` call and lines 7738-7813 canonical wrapper as a required source-mapped routine/project entry; CP123 maps the first-time air-balance display at lines 176-178 without a synthetic routine; CP124 maps the unconditional line-179 `ManageAirHeatBalance(state)` call by reusing the existing required routine; CP125 adds the unconditional line-184 `UpdateFinalSurfaceHeatBalance(state)` call and lines 5176-5219 implementation as a required source-mapped routine/project entry; CP126 adds the parent lines 186-189 `AnyCTF || AnyEMPD`-guarded `UpdateThermalHistories(state)` call and lines 5221-5581 implementation as a required source-mapped routine/project entry; CP127 adds the independent parent lines 191-206 `AnyCondFD` complete-Surface filtered moisture-update block and inline `SurfaceDataFD::UpdateMoistureBalance` helper as a non-required source-mapped routine; CP128 adds the unconditional line-208 `ManageThermalComfort(state, false)` call and `ThermalComfort.cc` lines 105-164 implementation as a non-required source-mapped routine; CP129 adds the unconditional line-210 `ReportSurfaceHeatBalance(state)` call and `HeatBalanceSurfaceManager.cc` lines 6605-6891 implementation as a required source-mapped routine/project entry; CP130 adds the lines 211-213 `ZoneSizingCalc`-guarded `GatherComponentLoadsSurface(state)` call and `OutputReportTabular.cc` lines 15064-15132 implementation as a non-required source-mapped routine; CP131 adds the unconditional line-215 `CalcThermalResilience(state)` call and `HeatBalanceSurfaceManager.cc` lines 5707-5799 implementation as a non-required source-mapped routine; CP132 adds the lines 217-219 `displayThermalResilienceSummary`-guarded `ReportThermalResilience(state)` call and lines 5801-6388 implementation as a non-required source-mapped routine; CP133 maps the lines 221-223 `displayCO2ResilienceSummary`-guarded `ReportCO2Resilience(state)` call; CP134 maps the lines 225-227 `displayVisualResilienceSummary`-guarded `ReportVisualResilience(state)` call; CP135 maps the parent-tail line-229 `ManageSurfaceHeatBalancefirstTime = false` assignment without a synthetic routine; CP136 maps the `HeatBalanceManager.cc` line-210 unconditional `EndZoneTimestepBeforeZoneReporting` `ManageEMS` call by reusing `routine.manage_ems`; CP137 maps the line-211 `RecKeepHeatBalance(state)` call as a required routine, declared at `HeatBalanceManager.hh` line 134 and implemented at `HeatBalanceManager.cc` lines 2971-3057; CP138 maps the line-217 unconditional `ReportHeatBalance(state)` call as a required routine, declared at header line 142 and implemented at source lines 3321-3418; CP139 maps the line-219 unconditional `EndZoneTimestepAfterZoneReporting` `ManageEMS` call by reusing `routine.manage_ems`; CP140 maps the line-221 unconditional `UpdateEMSTrendVariables(state)` call as non-required, declared at `EMSManager.hh` line 122 and implemented at `EMSManager.cc` lines 1444-1479; CP141 maps the line-222 unconditional `PluginManagement::PluginManager::updatePluginValues(state)` call as non-required, declared at `PluginManager.hh` line 198 and implemented at `PluginManager.cc` lines 1458-1467; CP142 maps the required outer `WarmupFlag && EndDayFlag` block at lines 224-226 and its `CheckWarmupConvergence(state)` call, declared at `HeatBalanceManager.hh` line 136 and implemented at `HeatBalanceManager.cc` lines 3059-3226; CP143 maps the inner line-227 `!WarmupFlag` branch and ordered line-228/229 `DayOfSim = 0` then `DayOfSimChr = "0"` mutations; CP144 maps the line-231 in-branch `ManageEMS(state, BeginNewEnvironmentAfterWarmUp, anyRan, absent)` call by reusing `routine.manage_ems`; CP145 maps the required lines 235-237 guarded `ReportWarmupConvergence(state)` call, declared at `HeatBalanceManager.hh` line 138 and implemented at `HeatBalanceManager.cc` lines 3228-3301; after `ManageHeatBalance` ends at line 238, CP146 maps required `SetPreConstructionInputParameters`, declared at header line 96, called unconditionally from `SimulationManager.cc` line 216, and implemented at source lines 446-492; CP147 maps required `GetSiteAtmosphereData`, declared at header line 100, called from `GetHeatBalanceInput` line 264 between project controls and spectral input, and implemented at source lines 1252-1317; CP148 maps required `AllocateZoneHeatBalArrays`, declared at header line 130, implemented at source lines 2824-2854, and called first by `AllocateHeatBalArrays` at line 2863 from the `InitHeatBalance` BeginSim chain; CP149 maps required `AllocateHeatBalArrays`, declared at header line 132 and implemented at source lines 2855-2963; CP150 maps required `UpdateWindowFaceTempsNonBSDFWin`, declared at header line 140, implemented at source lines 3303-3313, and called by `RecKeepHeatBalance` at line 3056; CP151 maps non-required `OpenShadingFile`, declared at header line 144, implemented at source lines 3422-3438, and called by `InitHeatBalance` at lines 2696-2698; CP152 maps non-required `SetStormWindowControl`, declared at header line 156, implemented at source lines 4595-4644, and called by `InitHeatBalance` at line 2669 under `TotStormWin > 0 && BeginDayFlag`; CP153 maps required `InitConductionTransferFunctions`, declared at header line 180, implemented at source lines 6153-6202, and called by `InitHeatBalance` at line 2621 under `BeginSimFlag && (AnyCTF || AnyEMPD)`; CP154 adds non-required `GatherForPredefinedReport`, declared at `HeatBalanceSurfaceManager.hh` line 99, implemented at `HeatBalanceSurfaceManager.cc` lines 623-1404, and called by `InitSurfaceHeatBalance` at line 481 under `BeginSimFlag`; CP155 adds required `AllocateSurfaceHeatBalArrays`, declared at header line 101, implemented at source lines 1406-2206, whose sole production `src/` call is `InitSurfaceHeatBalance` line 350 under its lines-349-355 BeginSim block; CP156 adds required `InitThermalAndFluxHistories`, declared at header line 103, implemented at source lines 2208-2447, whose sole production `src/` call is `InitSurfaceHeatBalance` line 383 inside its lines-379-384 BeginEnvrn block; CP157 adds non-required `EvalOutsideMovableInsulation`, declared at header line 105, implemented at source lines 2449-2481, whose sole production `src/` call is `InitSurfaceHeatBalance` line 388 as the first child of its lines-387-390 `AnyMovableInsulation` block; CP158 adds non-required `EvalInsideMovableInsulation`, declared at header line 107, implemented at source lines 2483-2513, whose sole production `src/` call is `InitSurfaceHeatBalance` line 389 as the second child of its lines-387-390 `AnyMovableInsulation` block; CP159 adds required source-mapped `routine.init_solar_heat_gains` and its project-contract entry immediately after `init_thermal_and_flux_histories` and before `calc_heat_balance_outside_surf`. It is declared at `HeatBalanceSurfaceManager.hh` line 109, implemented at `HeatBalanceSurfaceManager.cc` lines 2515-3776, and called only as unconditional `InitSolarHeatGains(state)` inside `InitSurfaceHeatBalance` line 457; the caller first-time flag gates only its preceding progress text, a CP159 failure blocks daylighting/internal gains/CP160, and only the successful caller tail clears that flag. Exactly four direct calls occur in three unit-test contexts: the incident-multiplier fixture calls twice under positive solar and checks only exact half-versus-whole transmitted solar, WindowFrameTest exercises BeginEnvironment reset with zero solar and asserts only a downstream heat-loss relation, and the CFS fixture calls under positive solar before mutating back-surface state/rerunning distribution and asserts only downstream beam absorption. Every call unconditionally zeros Zone window/opaque conduction rate and energy reports, every enclosure initial-diffuse-reflection term, stored opaque-range conduction/absorbed/report fields, stored Window-range frame/divider/shade/convection/gain/energy fields, and `SurfWinQRadSWwinAbs` through `CFSMAXNL + 1`. While first-time remains true it seeds both ground-reflection factors from each Surface ground view factor. It defines positive solar by `SunIsUp && signed beam+ground+diffuse > 0`, derives sunset and sun-up/no-radiation plus BeginEnvironment reset, then commits `PreviousSolRadPositive` before any broad reset; NaN makes the comparison false, and a failed sunset-only reset loses its edge on retry while the other two reset causes persist. Positive-or-reset calls clear all-Surface inside-beam/incident/reflection and sky/ground arrays, enclosure transmitted reports/energies, stored-Window shade/transmission/reveal/BSDF/absorbed-layer fields; reset-only calls additionally clear only `EnclSolQD`/`EnclSolQDforDaylight`, TDD transmittances and selected dome rate/layer fields, optional three reflection factors, and selected all-Surface reports/profile/system optics. They do not clear `EnclSolQSDifSol`, either complex-window ground split, `SurfSkyDiffReflFacGnd`, positive-only `IncSolMultiplier`, custom-only `GndReflSolarRad`, `AbsDiffWin*`, scheduled-mutated `SurfWinA`, external-library total energy, or all TDD dome W/energy state. The positive branch asserts reflection-table dimensions even when reflection calculation is disabled, computes `max(beam*SOLCOS(3)+diffuse,0)` with the expression first so NaN persists, samples/stores one multiplier for every numeric Surface using same-index request alignment, computes sky and ground incidence, and, when enabled, writes the complex ground split before interpolating the beam ground factor, then interpolates obstruction/ground factors and rebuilds main sky/ground incidence. It calls profile angles, optional reveal reflection, then either the external-library simplified opaque plus interior distribution path or ordinary interior distribution. It forms daylight and normal enclosure diffuse power, copies daylight power into `EnclSolQSDifSol`, adds only flagged off-diagonal interzone contributions, then applies diagonal fraction and `solVMULT`. Shading-range reports omit obstruction additions; stored AllExt reports add them; TDD diffuser overrides use active diffuser `TransDiff` and dome angle/sunlit while retaining earlier ground-component reports; shelf ground overrides happen after incident totals and do not refresh them. Opaque absorption uses base Construction and the first exact scheduled Surface/base-Construction pair, with unscheduled beam scaling but scheduled bare `SurfOpaqAI`; duplicate opaque ranges overwrite outside/report assignments but repeat inside `+=`. The Window loop dereferences SurfaceWindow before its solar/TDD guard and uses active Construction except base EQL layer count; Detailed optics cover shade/screen/blind horizontal sky-ground splitting and switchable interpolation, BSDF uses the first exact scheduled Surface/active-Construction pair and mutates `SurfWinA`, EQL uses base-system count with active optics, and external-library layers never write total energy. Shaded-construction array/member ownership differs unchecked. Common frame/divider work preserves projection, reveal, switchable, suspended-divider, ExtBlind/ExtShade/ExtScreen formulas; between-glass enters alternate incidence but no final absorber case. A zeroed local beam array is populated by earlier lists/overrides and consumed later, so malformed list/range alignment can yield zero beam; repeated windows reset Detailed/EQL totals locally but can accumulate BSDF/external totals. Final TDD dome absorption uses base Construction and writes energy inside its layer loop. Representative averaging uses raw constituent-area divides with no zero/topology guard, can consume earlier representatives in traversal order, uses active representative `TotGlassLayers` regardless model, and overwrites only per-area opaque/window-layer/frame/divider fields without recomputing Window layer W, totals, energy, or reports. CP159 emits no direct diagnostic and has no validation, catch, rollback, or cleanup; dimension/material assertions, unchecked blind casts, indices, TDD/area/reflection denominators, trig/nonfinite inputs, child failures, or allocation can leave an ordered prefix. Six mutated owners require coordinated clear: HeatBalance, HeatBalSurf, Surfaces, Environment, DaylightingDevices placement-new reconstruction plus `HeatBalSurfMgr::clear_state`; the latter rearms first-time and recreates scratch arrays, Environment restores the previous-solar latch false, and dependency-owner clears alone do not replay CP159. Rust incident-solar forcing diagnostics and bounded opaque absorption, plus run-blocked multiplier/scheduled-inside declarations, are not this latch/distribution/window/report lifecycle. CP159 adds no EnergyPlus source inventory, Rust target/code/state, test, support, capability, output, numerical, performance, or conformance promotion; the inventory becomes 32 algorithms and 168 routines, split 58 state-mapped plus 110 source-mapped, with 61 required. CP160 adds required source-mapped `routine.init_int_solar_distribution` and its project-contract entry immediately after `init_solar_heat_gains` and before `compute_int_thermal_absorp_factors` in source-definition order. It is declared at `HeatBalanceSurfaceManager.hh` line 111, implemented at `HeatBalanceSurfaceManager.cc` lines 3778-4177, and called only as unconditional `InitIntSolarDistribution(state)` inside `InitSurfaceHeatBalance` line 468; the first-time flag gates only preceding progress text, and a CP160 failure blocks interior convection and the successful parent-tail flag clear. The only direct unit context calls twice: with InterZone disabled and beam report 10 W, the night call observes an already-zero report remain zero, while the sun-up call checks only 10/6 = 1.666667 W/m2. SunIsUp gates only that AllHT, non-Shading beam-report traversal; at night CP160 still distributes internal short wave and leaves prior beam intensity/amount/energy untouched. Every call assigns each Solar enclosure raw `EnclSolQD + sum(space QLTSW)` and lights-only `sum(QLTSW)`; an InterZoneWindow receiver/source double loop requires both receive flags, excludes the diagonal, recomputes source QLTSW, adds raw matrix-weighted total/lights/diffuse power, and overwrites diffuse-report energy after each qualifying source while a no-source receiver gets no energy write. The enclosure values are then multiplied by `solVMULT` and, only for InterZoneWindow, the diagonal fraction; there is no normalization or range/finite guard. Sun-up beam reporting divides Zone beam W by raw enclosure total area, then multiplies by Surface area plus divider area and timestep seconds. Zone/stored-Space traversal visits inclusive opaque then Window ranges without sorting, deduplication, class checks, or bound validation. Opaque total short-wave absorption uses current `SurfAbsSolarInt`, lights-only absorption uses base Construction `InsideAbsorpSolar`, and initial diffuse is added last. Under global movable insulation, every reached opaque record marked present first writes `SurfQRadSWOutMvIns = prior outside absorption * current exterior absorptance / base outside-material absorptance`, then overwrites outside absorption through an optional `MaterialFen.Trans` and a second division by current absorptance; a failed cast supplies zero transmittance and both denominators are unchecked. Direct retry feeds the overwritten outside value back, while a false global/present gate leaves that value and `SurfQRadSWOutMvIns` stale. Every Window dereferences current SurfaceWindow, radiant/Solar enclosures, and active Construction with no exterior-solar or class gate. Both EQL and non-EQL overwrite Window internal radiant gain from enclosure long wave, `radThermAbsMult`, and current thermal absorptance; a load-component pulse adds exactly 0.01 times radiant-enclosure floor area, but common frame/divider formulas keep the unpulsed value plus HVAC radiation. Non-EQL has no separate external-library branch and uses active conventional optics: unshaded layers add back diffuse absorption; shade/screen layers use shaded back optics; only IntBlind/ExtBlind interpolate pane absorption with unclamped `Interp`, so BGBlind skips pane addition while still receiving the later any-blind shade absorption. Only interior shade/blind writes shade long-wave absorption; exterior shade/blind/screen scales shade short wave by glazed fraction. Switchable layers use `InterpSw`, which clamps ordered factors including infinities to [0,1] but propagates NaN, and dereferences the shaded Construction without a zero-index guard. Zero-index or conditional/invalid shading can skip main layer work and later fail in the Detailed initial-diffuse tail. Positive frame/divider areas add Solar-enclosure short wave plus unpulsed internal/HVAC long wave with raw projection corrections; suspended dividers assert a last-layer MaterialGlass and use an unchecked multiple-reflection denominator, while IntShade asserts MaterialFen and IntBlind applies current solar/IR transmission. EQL uses active EQL NL and back diffuse optics and omits frame/divider work. For every positive adjacent link, a conventional target uses the current/source active layer count but adjacent base `AbsDiff`; an EQL target uses adjacent base EQL NL but current/source active `AbsDiffFrontEQL`. The final Detailed/BSDF/EQL tail adds prior initial-diffuse layer values, and Detailed shade/screen/blind also adds initial shade absorption. All CP160 Window additions mutate only per-area `SurfWinQRadSWwinAbs` and never refresh CP159 layer W, total W, or total energy; frame/divider additions likewise have no separate W/energy derivation here. The unconditional final TDD child is declared at `DaylightingDevices.hh` line 95, implemented at `DaylightingDevices.cc` lines 1506-1559, called only by CP160 line 4176, and has no direct unit-test call. It uses diffuser base `TransDiff`, stale CP159 diffuser total W, normally prior CP159/dependency dome layer one, and newly CP160-mutated diffuser layer one; its raw formula applies diffuser area even to the dome term and divides by `TransDiff`. Zero-first `max(0, rawGain)` maps negative, negative infinity, and NaN report values to zero while retaining positive infinity, but transition-zone gains use unclamped signed/nonfinite rawGain times length divided by unchecked total length. The current pipe report is committed before its transition loop, so failure can retain that report and a partial current `TZoneHeatGain` prefix after complete earlier pipes. Preceding `ManageInternalHeatGains`/`FigureTDDZoneGains` can zero current transition gains on BeginEnvironment but not the pipe report, and its current internal-gain update completes before CP160 writes new transition values. CP160 has no direct diagnostic, validation, return status, catch, rollback, or cleanup; only the suspended-divider and IntShade casts assert, while invalid topology, arrays, indices, layer mismatches, denominators, and nonfinite inputs can leave an ordered prefix. Re-entry mixes assigned enclosure/radiant/TDD state with additive Zone diffuse, opaque, layer, adjacency, frame/divider, and shade state; Sun-down, no-source, existing no-longer-reached range members, reduced transition-zone counts, and changed gates can preserve stale values, and normal parent order merely relies on preceding CP159 resets. Four directly mutated owners clear by placement-new: HeatBalance, HeatBalSurf, Surfaces, and DaylightingDevices; HeatBalSurfMgr clear only rearms the caller display flag. Rust separately consumes initialized/supplied opaque inside-short-wave state and has a bounded typed-Zone radiant-gain distributor whose ordered `<= 0` gates admit NaN, but no CP160 Solar-enclosure/interzone, Window/shade/frame/divider, adjacency, movable-insulation, TDD, report, failure, or re-entry state machine. CP160 adds no EnergyPlus source inventory, Rust target/code/state, test, support, capability, output, numerical, performance, or conformance promotion; the inventory becomes 32 algorithms and 169 routines, split 58 state-mapped plus 111 source-mapped, with 62 required. CP161 adds required source-mapped `routine.compute_int_thermal_absorp_factors` and its project-contract entry immediately after `init_int_solar_distribution` and before `compute_int_sw_absorp_factors` in source-definition order. It is declared at `HeatBalanceSurfaceManager.hh` line 113, implemented at `HeatBalanceSurfaceManager.cc` lines 4179-4295, and called only by unconditional `InitSurfaceHeatBalance` line 427; runtime execution is after shading-status production and before CP162, CP159, and CP160. The caller first-time flag gates only progress text plus the separate preceding `InitInteriorRadExchange`, not CP161, and a CP161 failure blocks every later child and the successful caller-tail flag clear. Its sole direct unit test uses one 1 m2 `IntBlind` Window with 0.1 shade plus 0.1 glass effective emissivity and checks only `SurfAbsThermalInt = 0.2` and `radThermAbsMult = 5`. Pass one scans the full allocated `EnclRadInfo` container, skips false `radReCalc`, then visits stored enclosure Space Window ranges: `IntShade`/`IntBlind` assign effective shade-plus-glass emissivity and every other flag assigns active-Construction inside thermal absorptance. Pass two, independently of `radReCalc`, scans `SurfMovSlatsIndexList` only when `AnyMovableSlat` and rewrites only currently interior shade/blind entries. Pass three again scans the full allocated enclosure container and each flagged `SurfacePtr`: nonswitchable area uses current `SurfAbsThermalInt`, while switchable area uses clamped-factor `InterpSw` between declared base-Construction and `activeShadedConstruction` absorptance. Positive frames add area times `(1 + 0.5*projection)` times emissivity. Positive dividers start with divider emissivity, substitute base-Construction absorptance when suspended, and either use the projected term or, for an interior device with a declared layer, assert the active shaded Construction last material as `MaterialFen` and add shade effective emissivity plus divider absorptance times shade thermal transmittance or current blind back-IR transmittance; the shaded Construction is dereferenced before the layer-present test. This preserves deliberate active/base/runtime-optics asymmetries. The first pass visits full Window ranges, but normally initialized `SurfacePtr` holds representative surfaces only; CP161 ignores constituent-aggregated `EnclRadInfo.Area` and uses raw representative Surface/frame/divider areas, creating a representative-extent mismatch. The producer bulk paths use separate Solar and radiant enclosure counts, while its shading-change loop is Solar-count-bounded but reads same-index radiant `SurfacePtr`; CP161 range-for can see a retained radiant-container tail and never clears `EnclRadInfo.radReCalc`. CP162 reads the corresponding `EnclSolInfo.radReCalc`, not the same flag object. Normal movable slats are observed by the producer through `SurfacePtr`, while direct or malformed topology can still let the independent list update an unflagged enclosure. More importantly, CP158 can change opaque inside thermal absorptance without that movable-insulation transition itself setting `radReCalc`, leaving a normal-path multiplier stale absent another producer cause. Stored orders have no local sort, deduplication, class/model, membership, or bounds validation. `radThermAbsMult = 1/SUM1` is committed only after a complete flagged enclosure with no zero, sign, minimum, or finite guard: zero becomes positive infinity, signed infinity becomes signed zero, negative stays negative, and NaN propagates; `InterpSw` clamps ordered factors including infinities but preserves NaN and raw nonfinite endpoint arithmetic. CP161 has no direct diagnostic, return status, allocation, catch, rollback, or cleanup and only the divider material assertion. Failure can leave a first-pass Window prefix, then a movable-list prefix, or complete earlier enclosure multipliers while the current/later multipliers remain old; false `radReCalc`, no-longer-reached range/list entries, malformed independent slat updates, and direct retry can preserve selective stale state, while a nonpositive frame/divider area simply drops from a recomputed flagged sum. Exactly two direct owners have mutated targets: HeatBalSurf owns the written `SurfAbsThermalInt` and separately supplies the read-only movable-slat list, while ViewFactor owns the written `radThermAbsMult` and separately supplies read-only enclosure flags/topology; Surfaces, HeatBalance topology, Construction, Material, Window, SolarShading, and interior-radiation state are dependencies only. Rust has only a bounded typed-Zone radiant-gain distributor that applies `.max(0.0)` to absorptance but whose ordered `<= 0` gain/sum gates admit NaN, not this recalculation/shading/representative/frame-divider/raw-reciprocal state machine. CP161 adds no EnergyPlus source inventory, Rust target/code/state, test, support, capability, output, numerical, performance, or conformance promotion; the inventory becomes 32 algorithms and 170 routines, split 58 state-mapped plus 112 source-mapped, with 63 required. CP162 adds required source-mapped `routine.compute_int_sw_absorp_factors` and its project-contract entry immediately after `compute_int_thermal_absorp_factors` and before `compute_dif_sol_exc_zones_wiz_windows` in source-definition order. It is declared at `HeatBalanceSurfaceManager.hh` line 115, implemented at `HeatBalanceSurfaceManager.cc` lines 4297-4471, and called only by unconditional `InitSurfaceHeatBalance` line 433 immediately after CP161; the caller first-time flag gates only the preceding diffuse-solar progress text, and a CP162 failure blocks conditional interzone diffuse exchange, daylighting, interior long-wave exchange, CP159, CP160, and the successful caller-tail flag clear. There is no direct unit-test call. CP162 scans the full allocated `EnclSolInfo` container, skips false `radReCalc`, and accumulates each flagged enclosure over stored `SurfacePtr` order without local sort, deduplication, class, membership, index, or finite validation. Normal Solar topology contains every stored Space Surface except AirBoundary, including nonrepresentatives, whereas the shading-change producer is Solar-count-bounded but observes same-index representative-only radiant `SurfacePtr`; this records an observer/calculation-domain asymmetry without claiming a normal unflagged constituent. Only direct, malformed, representative-misaligned topology or count/container mismatch can turn that asymmetry into an unflagged Solar enclosure. Each Surface selects its active Construction and is opaque only when active `TransDiff <= 0`; opaque area uses current `SurfAbsSolarInt`, while NaN transmittance enters the Window branch. Conventional versus equivalent-layer selection then uses the declared base Construction `WindowTypeEQL`. Conventional Windows sum active-layer back absorptances and active diffuse transmission; only a nonzero storm-aware `SurfWinActiveShadedConstruction` enables shade/screen/blind optical substitution: shade/screen optics come from that Construction, blinds raw-linearly interpolate current slat endpoints, and a shade/blind flag with index zero silently retains bare active optics and zero shade absorption. Switchable glazing alone has two separate positive-shaded-Construction assertions and clamped-factor `InterpSw` for layer absorption and transmission. Their raw Surface-area term is `TransDiffWin + AbsDiffTotWin + DiffAbsShade`; positive frames add area times solar absorptance times `(1 + 0.5*projection)`. Positive dividers use current divider solar absorptance; suspended dividers assert the active Construction last layer as `MaterialGlass` and apply an unchecked glass/divider multiple-reflection denominator. Interior shade/blind dividers omit projection and add `DividerAbs + DiffAbsShade`; all others use `(1 + projection)*DividerAbs`. The base-EQL branch instead consumes the active Construction EQL pointer, layer count, diffuse transmission, and back absorptances and ignores shading, switching, frames, and dividers. After a complete enclosure, only strict `SUM1 > 0.01` writes `solVMULT = 1/SUM1`; positive infinity becomes positive zero, while exactly 0.01, smaller/negative values, negative infinity, and NaN enter the bad-sum branch, issue the exact misspelled `ComputeIntSWAbsorbFactors` warning only while `solAbsFirstCalc` is true, set that latch false after the warning returns, and write zero. A later good sum or BeginEnvironment does not rearm the latch. False recalculation preserves both targets; failures can leave earlier enclosure commits and the current/later multipliers and latch old, while warning-pipeline failure can leave partial diagnostic side effects before either target is committed. CP158 can change opaque `SurfAbsSolarInt` without itself producing a recalculation flag, giving the normal-path stale-multiplier case; direct or malformed Solar/radiant observer misalignment can also preserve stale state. ViewFactor is the sole direct mutation owner for `solVMULT` and `solAbsFirstCalc`; warning counters/streams/SQLite/callbacks are child diagnostic side effects, while Surface, HeatBalSurf, Construction, Material, equivalent-layer, Window, SolarShading, global, and topology state are dependencies. Rust has no `ComputeIntSWAbsorpFactors`, Solar multiplier, latch, or post-initialization production writer for `inside_shortwave_absorbed_w_per_m2`; its separate typed-Zone thermal-radiant helper has ordered `<= 0` gain/sum gates that admit NaN and is not this short-wave routine. CP162 adds no EnergyPlus source inventory, Rust target/code/state, test, support, capability, output, numerical, performance, or conformance promotion; the inventory becomes 32 algorithms and 171 routines, split 58 state-mapped plus 113 source-mapped, with 64 required. CP163 adds required source-mapped `routine.compute_dif_sol_exc_zones_wiz_windows` and its project-contract entry immediately after `compute_int_sw_absorp_factors` and before `calc_heat_balance_outside_surf` in source-definition order. It is declared at `HeatBalanceSurfaceManager.hh` line 117, implemented at `HeatBalanceSurfaceManager.cc` lines 4473-4644, and called only at `InitSurfaceHeatBalance` line 439 inside the lines-435-440 `InterZoneWindow` block; that parent flag gates both progress-and-call, while the nested first-time flag gates only the exact progress text. A guarded CP163 failure blocks daylighting, interior long-wave exchange, CP159, CP160, and the successful caller-tail flag clear. Exactly three direct calls occur in one unit fixture: with three enclosures and two reciprocal 1 m2, 0.1-transmittance Windows, the first call leaves receiver `HasInterZoneWindow` false and checks only identity diagonals plus all receive flags false; before the second call the fixture enables enclosures 1 and 2 and checks only flags true, true, false; before the third it enables `KickOffSimulation` and again checks only identity diagonals plus false flags. No test asserts a matrix coefficient or covers sizing kickoff, denominators, multi-enclosure paths, nonfinite values, allocation incoherence, or either assertion's failure. Allocation is gated solely by an unallocated `ZoneFractDifShortZtoZ`: that path allocates the target square matrix, receive-flag vector, then separate `DiffuseArray` scratch. Every call resets flags false, target identity, then scratch identity before either kickoff flag can return; already allocated dimensions are not repaired. Using the consumers' row-receiver/column-source convention, a stored-order `AllHTWindowSurfaceList` pass skips nonpositive adjacency, Surface-self adjacency, declared/base `TransDiff <= 0`, and a false receiver `HasInterZoneWindow`, then adds `A(receiver, source) += TransDiff * receiver.solVMULT * Area`; it does not use active/shaded/storm optics, validate the adjacent endpoint, sort, deduplicate, or validate indices, class, sign, range, or finiteness. The receiver flag is set when `solVMULT != 0`, independently of the coefficient. Same-enclosure paired Surfaces can modify raw diagonal state that the next transform discards. With raw matrix `A` frozen and identity scratch `D`, every distinct pair computes `D(R,S) = A(R,S)/(1-A(R,S)*A(S,R))`, and `D(N,N) = 1 + sum[M != N](A(N,M)*D(M,N))`; there is no denominator or finite guard. The copied target is followed by exactly two dimension assertions. A later scan only ORs flag `N` when some distinct receiver has `D(M,N) > 0`, making that scan a positive source-column witness rather than the direct pass's receiver witness. Frozen `D` then contributes every flagged, node-distinct simple path of exactly two, three, and four edges with products `D(J,K)*D(I,J)`, `D(K,L)*D(J,K)*D(I,J)`, and `D(L,M)*D(K,L)*D(J,K)*D(I,J)` into receiver/source cells `Z(I,K)`, `Z(I,L)`, and `Z(I,M)`; additions never feed later products, diagonals are not updated, node revisits and paths beyond four edges are omitted. Edge gates skip only exact signed zero, so negative, infinite, and NaN coefficients enter; ordered `> 0` flagging rejects negative and NaN but accepts positive infinity. CP163 emits no diagnostic and has no status, catch, rollback, cleanup, or local validation. Allocation, reset, direct-pass, pair-copy, flag-scan, or path failure can leave the corresponding ordered prefix; a coherent direct retry first erases all three objects, while kickoff commits false/identity state and returns. When production `InterZoneWindow` is false, CP163 is not entered and prior target/flag/scratch state remains dormant stale; CP159 and CP160 gate their normal interzone consumers on the same flag. Exactly two direct owners mutate: HeatBalSurf owns the target matrix and flags and clears them by placement-new and also supplies the read-only caller gate `InterZoneWindow`; CP163 never writes that gate and placement-new restores it false. HeatBalSurfMgr owns and explicitly clears scratch. Independent owner clears can desynchronize the single-guard allocation assumption in either direction. Rust has adjacent-zone opaque heat-transfer links and window-optical declarations but no Solar-enclosure exchange matrix/flags, bilateral transform, fixed simple-path expansion, kickoff reset, or matching failure/re-entry state. CP163 adds no EnergyPlus source inventory, Rust target/code/state, test, support, capability, output, numerical, performance, or conformance promotion; the inventory becomes 32 algorithms and 172 routines, split 58 state-mapped plus 114 source-mapped, with 65 required. CP164 next maps `InitEMSControlledSurfaceProperties`, declared at header line 119, implemented at source lines 4646-4720, and called only from `HeatBalanceManager.cc` line 2663 under `AnyEnergyManagementSystemInModel`; definition adjacency does not make it CP163's runtime successor, and no unit test calls it directly.
3. unconditional `ManageEMS(state, EMSCallFrom::BeginZoneTimestepBeforeInitHeatBalance, anyRan, absent)`
4. `InitHeatBalance`
5. unconditional `ManageEMS(state, EMSCallFrom::BeginZoneTimestepAfterInitHeatBalance, anyRan, absent)`
6. unconditional `ManageSurfaceHeatBalance(state)`, whose lines 145-230 parent body orders the CP117-mapped inline first-time initialization display, the CP118-mapped unconditional `InitSurfaceHeatBalance` call, the CP119-mapped first-time outside display, the CP120-mapped unconditional `CalcHeatBalanceOutsideSurf` call, the CP121-mapped first-time inside display, the CP122-mapped unconditional `CalcHeatBalanceInsideSurf`, the CP123-mapped first-time air display, the CP124-mapped `ManageAirHeatBalance`, the CP125-mapped `UpdateFinalSurfaceHeatBalance`, the CP126-mapped `AnyCTF || AnyEMPD`-guarded `UpdateThermalHistories`, the CP127-mapped `AnyCondFD`-guarded complete-Surface filtered moisture updates, the CP128-mapped unconditional `ManageThermalComfort(state, false)` call, the CP129-mapped unconditional `ReportSurfaceHeatBalance` call, the CP130-mapped `ZoneSizingCalc`-guarded `GatherComponentLoadsSurface`, the CP131-mapped unconditional `CalcThermalResilience`, the CP132-mapped thermal-summary-guarded `ReportThermalResilience`, the CP133-mapped CO2-summary-guarded `ReportCO2Resilience`, the CP134-mapped visual-summary-guarded `ReportVisualResilience`, and the CP135-mapped inline `ManageSurfaceHeatBalancefirstTime = false` tail
7. as mapped by CP136, unconditional `ManageEMS(state, EMSCallFrom::EndZoneTimestepBeforeZoneReporting, anyRan, absent)` at `HeatBalanceManager.cc` line 210
8. as mapped by CP137, unconditional `RecKeepHeatBalance(state)` at line 211, declared at `HeatBalanceManager.hh` line 134 and implemented at `HeatBalanceManager.cc` lines 2971-3057
9. as mapped by CP138, unconditional `ReportHeatBalance(state)` at line 217, declared at `HeatBalanceManager.hh` line 142 and implemented at `HeatBalanceManager.cc` lines 3321-3418
10. as mapped by CP139, unconditional `ManageEMS(state, EMSCallFrom::EndZoneTimestepAfterZoneReporting, anyRan, absent)` at line 219
11. as mapped by CP140, unconditional `UpdateEMSTrendVariables(state)` at line 221, declared at `EMSManager.hh` line 122 and implemented at `EMSManager.cc` lines 1444-1479
12. as mapped by CP141, unconditional `PluginManagement::PluginManager::updatePluginValues(state)` at line 222, declared at `PluginManager.hh` line 198 and implemented at `PluginManager.cc` lines 1458-1467
13. as mapped by CP142, the `WarmupFlag && EndDayFlag` outer block at lines 224-226 and its `CheckWarmupConvergence(state)` call, declared at `HeatBalanceManager.hh` line 136 and implemented at `HeatBalanceManager.cc` lines 3059-3226
14. as mapped by CP143, the inner `!WarmupFlag` branch at lines 227-229, assigning `DayOfSim = 0` and then `DayOfSimChr = "0"`
15. as mapped by CP144, `ManageEMS(state, EMSCallFrom::BeginNewEnvironmentAfterWarmUp, anyRan, absent)` at line 231, still inside the CP143 branch
16. as mapped by CP145, the lines 235-237 `!WarmupFlag && EndDayFlag && DayOfSim == 1 && !DoingSizing`-guarded `ReportWarmupConvergence(state)` call, declared at `HeatBalanceManager.hh` line 138 and implemented at `HeatBalanceManager.cc` lines 3228-3301
17. as mapped by CP146, unconditional `SetPreConstructionInputParameters(state)` at `SimulationManager.cc` line 216, declared at `HeatBalanceManager.hh` line 96 and implemented at `HeatBalanceManager.cc` lines 446-492
18. as mapped by CP147, `GetSiteAtmosphereData(state, ErrorsFound)` at `GetHeatBalanceInput` line 264, declared at `HeatBalanceManager.hh` line 100 and implemented at `HeatBalanceManager.cc` lines 1252-1317
19. as mapped by CP148, `AllocateZoneHeatBalArrays(state)`, declared at `HeatBalanceManager.hh` line 130, implemented at `HeatBalanceManager.cc` lines 2824-2854, and called first by `AllocateHeatBalArrays` at line 2863
20. as mapped by CP149, `AllocateHeatBalArrays(state)`, declared at `HeatBalanceManager.hh` line 132, implemented at `HeatBalanceManager.cc` lines 2855-2963, and called only under `InitHeatBalance`'s `BeginSimFlag` branch at lines 2617-2618; its first action is the separate CP148 child
21. as mapped by CP150, `UpdateWindowFaceTempsNonBSDFWin(state)`, declared at `HeatBalanceManager.hh` line 140, implemented at `HeatBalanceManager.cc` lines 3303-3313, and called as the last executable `RecKeepHeatBalance` action at line 3056
22. as mapped by CP151, `OpenShadingFile(state)`, declared at `HeatBalanceManager.hh` line 144, implemented at `HeatBalanceManager.cc` lines 3422-3438, and called by `InitHeatBalance` at lines 2696-2698 under `BeginSimFlag && DoWeathSim && ReportExtShadingSunlitFrac`
23. as mapped by CP152, `SetStormWindowControl(state)`, declared at `HeatBalanceManager.hh` line 156, implemented at `HeatBalanceManager.cc` lines 4595-4644, and called by `InitHeatBalance` at line 2669 under `TotStormWin > 0 && BeginDayFlag`
24. as mapped by CP153, `InitConductionTransferFunctions(state)`, declared at `HeatBalanceManager.hh` line 180, implemented at `HeatBalanceManager.cc` lines 6153-6202, and called by `InitHeatBalance` at line 2621 under `BeginSimFlag && (AnyCTF || AnyEMPD)`
25. as mapped by CP154, `GatherForPredefinedReport(state)`, declared at `HeatBalanceSurfaceManager.hh` line 99, implemented at `HeatBalanceSurfaceManager.cc` lines 623-1404, and called by `InitSurfaceHeatBalance` at line 481 under `BeginSimFlag`
26. as mapped by CP155, `AllocateSurfaceHeatBalArrays(state)`, declared at `HeatBalanceSurfaceManager.hh` line 101, implemented at `HeatBalanceSurfaceManager.cc` lines 1406-2206, whose sole production `src/` call is `InitSurfaceHeatBalance` line 350 under its lines-349-355 BeginSim block
27. as mapped by CP156, `InitThermalAndFluxHistories(state)`, declared at `HeatBalanceSurfaceManager.hh` line 103, implemented at `HeatBalanceSurfaceManager.cc` lines 2208-2447, whose sole production `src/` call is `InitSurfaceHeatBalance` line 383 inside its lines-379-384 BeginEnvrn block
28. as mapped by CP157, `EvalOutsideMovableInsulation(state)`, declared at `HeatBalanceSurfaceManager.hh` line 105, implemented at `HeatBalanceSurfaceManager.cc` lines 2449-2481, whose sole production `src/` call is `InitSurfaceHeatBalance` line 388 as the first child of its lines-387-390 `AnyMovableInsulation` block
29. as mapped by CP158, `EvalInsideMovableInsulation(state)`, declared at `HeatBalanceSurfaceManager.hh` line 107, implemented at `HeatBalanceSurfaceManager.cc` lines 2483-2513, whose sole production `src/` call is `InitSurfaceHeatBalance` line 389 as the second child of its lines-387-390 `AnyMovableInsulation` block
30. as mapped by CP159, required `InitSolarHeatGains(state)`, declared at `HeatBalanceSurfaceManager.hh` line 109, implemented at `HeatBalanceSurfaceManager.cc` lines 2515-3776, and called unconditionally by `InitSurfaceHeatBalance` line 457
31. as mapped by CP160, required `InitIntSolarDistribution(state)`, declared at `HeatBalanceSurfaceManager.hh` line 111, implemented at `HeatBalanceSurfaceManager.cc` lines 3778-4177, and called unconditionally by `InitSurfaceHeatBalance` line 468
32. as mapped by CP161, required `ComputeIntThermalAbsorpFactors(state)`, declared at `HeatBalanceSurfaceManager.hh` line 113, implemented at `HeatBalanceSurfaceManager.cc` lines 4179-4295, and called unconditionally by `InitSurfaceHeatBalance` line 427 before CP159 and CP160
33. as mapped by CP162, required `ComputeIntSWAbsorpFactors(state)`, declared at `HeatBalanceSurfaceManager.hh` line 115, implemented at `HeatBalanceSurfaceManager.cc` lines 4297-4471, and called unconditionally by `InitSurfaceHeatBalance` line 433 immediately after CP161
34. as mapped by CP163, required `ComputeDifSolExcZonesWIZWindows(state)`, declared at `HeatBalanceSurfaceManager.hh` line 117, implemented at `HeatBalanceSurfaceManager.cc` lines 4473-4644, and called only under `InterZoneWindow` by `InitSurfaceHeatBalance` line 439
35. CP164 next in source-definition order maps `InitEMSControlledSurfaceProperties`, declared at `HeatBalanceSurfaceManager.hh` line 119, implemented at `HeatBalanceSurfaceManager.cc` lines 4646-4720, and called only from `HeatBalanceManager.cc` line 2663 under `AnyEnergyManagementSystemInModel`; definition adjacency does not make it CP163's runtime successor, and no unit test calls it directly

## Current Blocker Ledger

`scripts/compare/official-dynamic-heat-balance-diagnostic.ps1` now requires the
diagnostic JSON and Markdown report to expose `top_blocker`,
`current_blockers`, `warmup_end_state_deltas`, and
`first_divergence_by_variable`. Resolved items stay in this source map with
`closed` status rather than disappearing from the trail.

| Blocker id | Status | Source-map note |
|---|---|---|
| `floor-storage-mismatch` | active | candidate top blocker row; keeps the floor storage delta separate from generic bottleneck ranking |
| `floor-face-temperature-current-inside-mismatch` | open | separate floor current-inside face-temperature row under the storage blocker |
| `ctf-current-term-delta` | open | per-surface CTF current-term row derived from inside/outside current RMSE |
| `ctf-history-temperature-term-delta` | open | per-surface CTF history temperature-term row |
| `ctf-history-flux-term-delta` | open | per-surface CTF history flux-term row |
| `longwave-radiation-source-delta` | open | per-surface inside longwave/radiation source row |
| `hconv-source-timing-delta` | open | per-surface hconv/reference-air source timing row |
| `warmup-end-state-mat-delta` | open | warmup final MAT delta row |
| `warmup-end-state-surface-temperature-delta` | open | first run-period surface-temperature delta used as warmup end-state evidence |
| `warmup-end-state-ctf-history-delta` | open | first run-period CTF history delta used as warmup end-state evidence |
| `ctf-coefficient-eio-seeding` | closed | EIO coefficient availability is resolved as diagnostic isolation; current blocker is source/history handoff, not coefficient presence |
| `source-order-wrapper-boundary` | closed | heat-balance source-order wrappers now exist on the runtime path; remaining rows track algorithm/state deltas behind those wrappers |
| `diagnostic-probe-conformance-aliasing` | closed | diagnostic probe metadata, selector matching, and compatibility source-order execution are separated; diagnostic selectors resolve to a probe-agnostic runtime config before compatibility code runs, so probe output cannot be promoted as conformance evidence |

<a id="diagnostic-probe-lifecycle-ledger"></a>

## Diagnostic Probe Lifecycle Ledger

The blocker ledger above records observed output and state deltas. Its active
and open rows are not, by themselves, source-state hypotheses and therefore do
not determine the number of active probes. A hypothesis in this registry must
name one scalar EnergyPlus state, one source ownership boundary, and one
falsifiable expected observation.

The active probe unit is a full executable lane: its selector, CTF and warmup
policies, iteration and convection settings, report settings, wrapper command,
and expected observation travel together. An enum variant by itself is not an
active probe. Historical probe selectors and wrappers are closed replay
artifacts; they are excluded from the default active suite. They remain
reproducible through the suite's explicit `-IncludeClosed` path and retained
direct wrapper commands. The recorded source-audit narrative below is retained
as their closure and replay evidence; it does not reopen a closed hypothesis or
make its lane active.

### Active Source-State Hypothesis Registry

<a id="diagnostic-probe-hypothesis:warmup-surftempin-first-sample-state-mismatch"></a>

#### `warmup-surftempin-first-sample-state-mismatch`

| Field | Contract |
|---|---|
| Status | `unresolved` |
| Algorithm / owner routine | `heat_balance_surface_manager_source_order` / `calc_heat_balance_inside_surf_2_ctf_only` |
| EnergyPlus ownership | `src/EnergyPlus/HeatBalanceSurfaceManager.cc::CalcHeatBalanceInsideSurf2CTFOnly` under the `ManageSurfaceHeatBalance` stage order |
| Scalar source state | `SurfTempIn` |
| Hypothesis | After repeated-day warmup, at least one first-run-period CTF surface has different EnergyPlus `SurfTempIn` and Rust inside-face state values at the reporting boundary. This is a direct state-mismatch hypothesis, not a claim that the mismatch's upstream cause has already been identified. |
| Expected observation | With forcing and the compatibility configuration held fixed, the observation-only lane finds the `ZN001:FLR001` first-run-period CTF row whose finite `oracle_inside_face_temperature_c` and `rust_inside_face_temperature_c` values differ by more than `1.0e-9 C`, with their absolute difference equal to `inside_face_temperature_delta_c`. The EnergyPlus `Surface Inside Face Temperature` report exposes `SurfTempIn`; the warmup day-end zone-air trace establishes the repeated-day context. If that direct state mismatch is absent, this hypothesis is rejected rather than converted into a compatibility branch. |
| Latest observation | 2026-07-14 active-lane run: 20 warmup day-end rows; `ZN001:FLR001` sample-`0` EnergyPlus `SurfTempIn` `-0.115727652883 C`, Rust inside-face state `-0.116329096525 C`, absolute delta `0.000601443642 C`. The direct mismatch remains present, so the hypothesis stays unresolved. |
| Active lane | `official-dynamic-heat-balance-warmup-surftempin-first-sample-probe` |

The locked source establishes the ownership boundary: the CTF-only inside solve
calculates `SurfTempInTmp` and copies it to the directly reported `SurfTempIn`,
while its caller continues through the air balance, final surface pass, and
`UpdateThermalHistories`. The hypothesis records only the directly observed
`SurfTempIn` mismatch at that boundary; it does not attribute the mismatch to
`SurfTempInTmp` or assert that the upstream defect has already been located.

## June 2026 Source-Audit Boundary

The official `1ZoneUncontrolled` dynamic promotion lane must keep the following
EnergyPlus 26.1.0 ownership boundaries explicit:

- `HeatBalanceManager.cc::ManageHeatBalance` calls `InitHeatBalance`, then
  `HeatBalanceSurfaceManager::ManageSurfaceHeatBalance`, then the CP136-mapped
  `EndZoneTimestepBeforeZoneReporting` EMS calling point, then CP137-mapped
  `RecKeepHeatBalance`, then CP138-mapped `ReportHeatBalance`, then the
  CP139-mapped `EndZoneTimestepAfterZoneReporting` EMS calling point, then
  CP140-mapped `UpdateEMSTrendVariables`, then CP141-mapped plugin-value update,
  then the CP142-mapped warmup/end-day convergence call and CP143-mapped
  numeric/text day-counter resets and CP144-mapped post-warmup EMS calling
  point and CP145-mapped guarded warmup-convergence report. CP146 separately
  maps the unconditional initialization-time
  `SetPreConstructionInputParameters` call and its shared maximum-layer bound.
  CP147 additionally maps required `GetSiteAtmosphereData` at the project-input
  head. CP148 additionally maps required `AllocateZoneHeatBalArrays` in the
  BeginSim allocation chain. CP149 adds required `AllocateHeatBalArrays`; CP150 adds required `UpdateWindowFaceTempsNonBSDFWin`; CP151 adds non-required `OpenShadingFile`; CP152 adds non-required `SetStormWindowControl`; CP153 adds required `InitConductionTransferFunctions`; CP154 adds non-required `GatherForPredefinedReport`; CP155 adds required `AllocateSurfaceHeatBalArrays`; CP156 adds required `InitThermalAndFluxHistories`; CP157 adds non-required `EvalOutsideMovableInsulation`; CP158 adds non-required `EvalInsideMovableInsulation`; CP159 adds required `InitSolarHeatGains`; CP160 adds required `InitIntSolarDistribution`; CP161 adds required `ComputeIntThermalAbsorpFactors`; CP162 adds required `ComputeIntSWAbsorpFactors`; CP163 adds required `ComputeDifSolExcZonesWIZWindows`; CP164 next in source-definition order maps `InitEMSControlledSurfaceProperties`.
  Warmup convergence is checked only at end-of-day.
- `HeatBalanceSurfaceManager.cc::ManageSurfaceHeatBalance` calls
  `InitSurfaceHeatBalance`, `CalcHeatBalanceOutsideSurf`,
  `CalcHeatBalanceInsideSurf`, `HeatBalanceAirManager::ManageAirHeatBalance`,
  `UpdateFinalSurfaceHeatBalance`, conditionally `UpdateThermalHistories`,
  conditionally updates CondFD moisture over the filtered complete Surface
  array, then calls `ManageThermalComfort`, `ReportSurfaceHeatBalance`,
  conditionally gathers sizing component loads, calculates thermal resilience,
  conditionally emits the three resilience reports, and clears its first-time
  flag. Four first-time progress displays precede initialization, outside,
  inside, and air work respectively.
- `CalcHeatBalanceInsideSurf2CTFOnly` freezes each surface's
  `SurfTempOutHist` from `SurfOutsideTempHist(1)`, stores
  `Surface::getInsideAirTemperature` in `RefAirTemp` and
  `SurfTempEffBulkAir`, builds `SurfTempTerm`, and only re-evaluates
  `Convect::InitIntConvCoeff` on the `ItersReevalConvCoeff` cadence.
- The CTF-only inside solve updates `SurfTempInTmp`, then copies it to
  `SurfTempIn`. For reporting, `SurfTempOut` is set from
  `SurfOutsideTempHist(1)`. Interzone surfaces are then synchronized by copying
  the paired inside history into `SurfOutsideTempHist(1)` and
  `SurfTempOutHist`; a direct current-inside adiabatic report sync is not the
  generic EnergyPlus reporting path.
- `UpdateThermalHistories` computes current inside flux from
  `SurfOutsideTempHist(1)`, current `SurfTempIn`, and
  `SurfCTFConstInPart`; computes current outside flux from the same current
  outside history slot and current `SurfTempIn`; writes report conduction
  variables; then shifts thermal histories. In `SimpleCTFOnly`, slot `1` is
  hard-copied to slot `2` after shifting.
- `ZoneTempPredictorCorrector.cc::calcSumHAT` builds solver coefficients from
  `SurfHConvInt * Area * SurfTempInTmp` plus `SurfTAirRef`-dependent
  `SumHA`/`SumHATref`. This is separate from the zone-air heat-balance report
  row.
- `ZoneTempPredictorCorrector.cc::CalcZoneComponentLoadSums` reports
  `SumHADTsurfs` by re-walking heat-transfer surfaces with
  `Surface::getInsideAirTemperature`, `SurfHConvInt`, and `SurfTempInTmp`.
  Therefore the zone `Surface Convection Rate` row is not required to equal the
  signed sum of individual surface inside convection report rows.
- `DataHeatBalance.cc::AirReportVars::setUpOutputVars` registers `Zone Mean Air
  Temperature` as `Zone/Average`, but registers `Zone Air Heat Balance Internal
  Convective Heat Gain Rate`, `Zone Air Heat Balance Surface Convection Rate`,
  and `Zone Air Heat Balance Air Energy Storage Rate` as `System/Average`.
  Rust keeps the default hourly average report mode and exposes a diagnostic
  `zone_air_report_sampling=last-system-state` probe only to isolate timing
  differences.
- `HeatBalanceSurfaceManager.cc::CalcHeatBalanceInsideSurf*` registers the
  advanced `Surface Inside Face Heat Balance Calculation Iteration Count` row
  with key `Simulation`, `TimeStepType::Zone`, and `StoreType::Sum`. It reports
  `InsideSurfIterations`, which is reset before the inside-surface convergence
  loop, increments after each pass, and triggers `InitIntConvCoeff` whenever
  `InsideSurfIterations % ItersReevalConvCoeff == 0`.
- The same source order keeps zone-air correction outside the
  inside-surface convergence loop: `CalcHeatBalanceInsideSurf*` converges
  `SurfTempIn`/`SurfTempInTmp` first, then the caller proceeds to air heat
  balance/reporting and final history update work. Both the broad
  `energyplus-heat-balance-compat-candidate` diagnostic wrapper and the
  promoted official compatibility candidate now pin
  `surface_loop_zone_air_correction=after-surface-loop` for this source-order
  parity point; `each-surface-iteration` remains available only as an explicit
  comparison probe.

## Bounded Window Frame And Divider Input Notes

EnergyPlus 26.1.0 calls `GetFrameAndDividerData` after
`Material::GetHysteresisData` and before `GetConstructData`. Rust instead
places `WindowProperty:FrameAndDivider` after base `parse_materials` and before
`parse_constructions`; its existing separate
`parse_material_phase_change_hystereses` pass remains later. Therefore this
checkpoint claims only the base-material/frame/construction relative order,
not Hysteresis-relative or complete `GetHeatBalanceInput` pass-order parity,
downstream surface binding, or any window calculation.

The bounded input owns an independent normalized name map and immutable frame,
divider, reveal, and NFRC descriptors. It accepts every source/schema numeric
field and both enums, applies only the source-effective corrections recorded
below, and validates every definition even when no surface references it. All
typed definitions are explicitly rejected at the runtime support boundary.

### `GetFrameAndDividerData` state contract

<!-- routine-state-contract:v1 begin get_frame_and_divider_data -->
GetFrameAndDividerData

read_state:
- EnergyPlus source places `GetFrameAndDividerData` after `Material::GetHysteresisData` and before `GetConstructData`; Rust deterministically reads `WindowProperty:FrameAndDivider` after base `parse_materials` and before `parse_constructions`, while its separate `parse_material_phase_change_hystereses` pass remains later; every definition is read eagerly, including an all-default, reveal-only, or unused record
- one required nonblank outer-key name in an independent normalized namespace; Divider Type defaults to `DividedLite` and accepts `DividedLite` or `Suspended`, while NFRC Product Type defaults to `CurtainWall` and accepts `CasementDouble`, `CasementSingle`, `DualAction`, `Fixed`, `Garage`, `Greenhouse`, `HingedEscape`, `HorizontalSlider`, `Jal`, `Pivoted`, `ProjectingSingle`, `ProjectingDual`, `DoorSidelite`, `Skylight`, `SlidingPatioDoor`, `CurtainWall`, `SpandrelPanel`, `SideHingedDoor`, `DoorTransom`, `TropicalAwning`, `TubularDaylightingDevice`, or `VerticalSlider`
- all 23 numeric fields as finite values with source-effective defaults and bounds: frame width [0,1] default 0, two frame projections [0,0.5] default 0, nonnegative frame conductance default 0 without upper bound, frame edge/center ratio (0,4] default 1, frame solar/visible absorptance [0,1] default 0.7, frame emissivity greater than 0 without upper bound default 0.9; divider width [0,0.5] default 0, two counts default 0, two divider projections [0,0.5] default 0, nonnegative divider conductance default 0 without upper bound, divider edge/center ratio (0,4] default 1, divider solar/visible absorptance [0,1] default 0, divider emissivity (0,1) default 0.9; three reveal absorptances [0,1] default 0 and two reveal depths [0,2] default 0
- horizontal and vertical divider counts accept finite values in [0,2147483648), truncate toward zero into nonnegative integer state, and have no cross-field validity requirement; no material, construction, surface, schedule, geometry, or usage dependency is read

write_state:
- a separate deterministic `WindowFrameAndDivider` arena and normalized name map; each record owns a typed ID, normalized name, nested `WindowFrameProperties`, `WindowDividerProperties`, and `WindowRevealProperties`, and one `WindowNfrcProductType`
- source correction order first clears both frame projections when frame width is zero, then clears both divider projections when divider width is zero or type is `Suspended`, then warns and resets a still-positive divider width when both effective counts are zero without revisiting the prior projection result, and finally warns and raises an inside sill shallower than the inside reveal to the reveal depth
- source-fixed frame and divider edge widths of 0.06355 m; no WINDOW 5 mullion orientation or synthesized frame/divider state is invented
- two nonblocking typed diagnostics identify the positive-divider-width/zero-count reset and the inside-sill-depth reset without claiming EnergyPlus warning text, severity, order, or multiplicity
- compile failure before typed ID or normalized-name reservation for a blank name, malformed or out-of-range numeric, count outside the finite [0,2147483648) boundary, invalid enum, or case-insensitive duplicate name; normalized duplicate rejection and invalid-enum rejection deliberately fail closed relative to source case-collision lookup and invalid-token fallback behavior

history_state_ownership:
- TypedModel owns immutable frame/divider/reveal descriptors; this checkpoint allocates no mutable window history

unsupported_state:
- fenestration-surface and legacy Window or GlazedDoor reference binding, first-match case-collision behavior, geometry and projected area state, frame/divider edge areas, reveal and sill geometry, and surface-local frame/divider pointers
- WINDOW 5 synthesized records and mullion orientation, between-glass shading mutation of shared divider width, window optical and thermal state, NFRC assembly calculations, reports, output variables, and runtime numerical state

inactive_branches:
- zero frame width silently leaves both frame projections at zero; zero divider width or `Suspended` silently leaves both divider projections at zero; a positive divider width with zero effective counts resets only the width after projection correction, so positive divided-lite projections intentionally remain possible beside the recovered zero width
- all-default, reveal-only, unused, non-fenestration-referenced, and names colliding with Material, Construction, or BuildingSurface namespaces remain valid typed records because this input has no cross-object dependency

unsupported_active_branches:
- every valid `WindowProperty:FrameAndDivider` definition is typed but reported as `UnsupportedSurfaceBoundary` and `RunBlocked`, including all-default and unused definitions; no partial run is allowed
- valid frame emissivity above 1, fractional divider counts truncated toward zero, source-recovered positive-width/zero-count input, and source-recovered shallow-sill input remain typed but do not activate a runtime or reporting consumer

not_claimed_branches:
- Hysteresis-relative Rust compiler pass order and complete `GetHeatBalanceInput` order parity, native-epJSON canonical enum casing, source case-colliding name lookup and declaration ordering, invalid-enum fallback, exact diagnostic text/severity/order/multiplicity, fenestration binding, geometry, WINDOW 5 synthesis, between-glass shading mutation, optics, thermal execution, NFRC assembly calculations, EIO/SQLite or other reporting, runtime numerics, and conformance
<!-- routine-state-contract:v1 end get_frame_and_divider_data -->

## Bounded Ordinary Construction Input Notes

EnergyPlus 26.1.0 calls `GetConstructData` after `GetFrameAndDividerData` and
before `GetBuildingData`. The routine reads all ordinary `Construction`
objects first, then calls the separately state-mapped F-factor/C-factor
generator, AirBoundary reader, and `SetupComplexFenestrationStateInput` before
entering its inline internal heat source overlay, equivalent-layer construction
family, and final WindowDataFile request branch. The parent routine therefore
remains `source_mapped`: the ordinary path and bounded F/C/Air/CFS/
InternalHeatSource/WindowEquivalentLayer/WindowDataFile request readers do not
imply completion of `SearchWindow5DataFile` or any downstream CTF/QTF, surface,
window, or source/sink consumer.

The bounded ordinary input retains its required construction name and ordered
material prefix. CP85 adds the source `GlassTCParent` handling: every typed
thermochromic parent is replaced in the effective stack by its first glazing
state, while the final parent encountered owns the source-shaped zero-based
master metadata. Effective model-graph edges follow the substituted glazing
IDs. Checked direct-index construction/material and opaque-cache lookups are
structural hardening only; the existing thermochromic parent capability keeps
all such definitions runtime-blocked, and no window execution is promoted. The
later CP103 `CreateTCConstructions` slice derives separate immutable series and
child snapshots from this metadata without changing this parent routine's
ConstructionIds, names, graph, or runtime boundary.

CP86 also accepts the intended whole-system SimpleGlazing form only when it is
the sole ordinary-`Construction` layer. The original SimpleGlazing material ID
is retained as the outside layer, sole ordered layer, and graph target, and the
construction is classified as fenestration. Multi-layer source-validation
holes and every window runtime consumer remain fail-closed.

CP90 types only the source-declaration overlay on an already materialized
ordinary opaque construction with at least two effective layers. It preserves
the direct epJSON map's lexical instance-key order, treats the required outer
name as a nonsemantic normalized diagnostic snapshot, and validates both
one-based layer interfaces plus the bounded dimension, spacing, and optional
perpendicular-position fields before attaching immutable metadata. The target
keeps the same identity, kind, layer stack, and material graph. Source recovery
quirks, broader construction-family targets, global source flags, and all
CTF/QTF and runtime consumers remain fail-closed.

CP91 types the following `Construction:WindowEquivalentLayer` declarations.
Staged IDF preserves family-local declaration order through the `getObjectItem`
order overlay, while native epJSON retains lexical instance-key order. A
required outside layer and optional layers 2 through 11 form a contiguous pack;
every reference must resolve to one of the six already typed equivalent-layer
material variants. GetConstructData itself does not impose solid/gap
alternation, so the bounded declaration retains one to eleven family-valid
layers without inventing downstream topology repair or ASHWAT behavior. Every
definition remains run-blocked.

CP92 types only the final `Construction:WindowDataFile` request surface. Its
required WINDOW5 entry name is normalized for matching, a missing or blank file
field retains the current-working-folder `Window5DataFile.dat` selector, and an
explicit file name remains retain-case text. Staged IDF uses the family order
overlay and native epJSON stays lexical. No file is opened, and the request
creates no construction name/ID, material, frame/divider, graph, or optical
state; all requests are blocked before I/O. The source's external-file expansion
remains mapped under `SearchWindow5DataFile`.

### `GetConstructData` bounded ordinary state contract

<!-- routine-state-contract:v1 begin get_construct_data -->
GetConstructData

read_state:
- EnergyPlus calls `GetConstructData` after `GetFrameAndDividerData` and before `GetBuildingData`; within it, every ordinary `Construction` is read before the separately state-mapped F-factor then C-factor generation pass, AirBoundary pass, and SetupComplexFenestrationStateInput pass, followed by the bounded InternalHeatSource overlay, bounded WindowEquivalentLayer family, and bounded request-only WindowDataFile front end whose external-file expansion remains deferred
- one required nonblank outer-key name in an independent normalized construction namespace, one required outside-layer material name, and optional layers 2 through 10 that must form a contiguous populated prefix; every populated layer resolves through the typed material namespace
- every typed `WindowMaterial:GlazingGroup:Thermochromic` parent encountered in layer order contributes its first ordered glazing state to the effective layer stack; the existing typed parent contract guarantees at least one state
- a `WindowMaterial:SimpleGlazingSystem` material is accepted only when it is the sole ordinary-Construction layer, matching the intended whole-system input while excluding EnergyPlus 26.1's multi-layer validation holes
- all layer references, contiguous-prefix rules, bounded opaque/fenestration family and topology checks, unsupported-consumer checks, and normalized duplicate checks complete before a `ConstructionId` or name is reserved
- the `ConstructionProperty:InternalHeatSource` epJSON instance map is read directly in case-sensitive lexical outer-key order and deliberately remains outside staged IDF declaration-order recovery; its required nonblank outer key is a normalized diagnostic snapshot without a typed ID or name map, so case variants may coexist when they target different constructions
- each internal-source record has a required nonblank case-insensitive shared-Construction reference restricted by the bounded contract to an ordinary opaque construction with at least two effective layers; both required one-based layer interfaces are positive integers in `1..TotLayers-1`, need not match, and have no ordering relationship
- internal-source CTF dimensions are exactly integer 1 or 2, required finite tube spacing is in `[0.01,1.0]` m and derives a retained perpendicular half-spacing, and the optional finite perpendicular temperature position defaults to 0 in `[0,1]` and remains retained but inactive for a 1-D declaration
- each `Construction:WindowEquivalentLayer` has a required nonblank name in the shared case-insensitive Construction namespace, one required outside layer, and optional layers 2 through 11 forming a contiguous one-to-eleven-layer prefix; every reference resolves case-insensitively to an already typed `MaterialFamily::EquivalentLayer` glazing, shade, drape, blind, screen, or gap definition
- EnergyPlus reads equivalent-layer constructions through `getObjectItem`; staged IDF therefore preserves family-local declaration order through the explicit order overlay, while native epJSON without that overlay retains case-sensitive lexical instance-key order
- each `Construction:WindowDataFile` request has a required nonblank WINDOW5 entry name and optional retain-case `file_name`; missing or blank input selects the explicit current-working-folder default `Window5DataFile.dat`, while nonblank text is retained without path resolution or file access
- EnergyPlus also reads WindowDataFile requests through `getObjectItem`; staged IDF preserves family-local declaration order through its explicit order overlay, while native epJSON without that overlay retains case-sensitive lexical instance-key order

write_state:
- the deterministic `Construction` arena and independent normalized name map retain each valid ID, normalized name, bounded opaque-or-fenestration kind, effective outside layer, and ordered effective outside-to-inside material stack
- every thermochromic parent layer is replaced by its first-state glazing `MaterialId`; the final parent encountered overwrites the source-shaped zero-based parent-material-layer and glazing-layer metadata slots while earlier parent substitutions remain in the effective stack
- a sole-layer SimpleGlazingSystem construction is retained as Fenestration with the original material ID in outside-layer, ordered-layer, and construction/material graph state; no detailed-glazing child is synthesized
- construction/material model-graph edges are emitted from the effective material stack, so thermochromic edges target first-state glazing materials rather than parent group descriptors
- runtime construction/material resolution and opaque construction thermal-cache building use checked direct-index lookups; these structural paths do not enable thermochromic or other window execution
- one immutable internal-source descriptor on each valid unique target retains its normalized diagnostic source name, one-based source and requested-temperature interfaces, 1-D or 2-D selector, authored tube spacing, derived perpendicular half-spacing, and perpendicular temperature position without changing Construction identity, kind, layers, other construction-family metadata, material state, or graph edges
- all internal-source fields and target constraints are validated before attachment; a malformed or missing earlier lexical record reserves no target, the first valid record attaches, and a later valid record for the same `ConstructionId` fails closed without overwriting the first descriptor
- one dedicated `ConstructionKind::WindowEquivalentLayer` record per valid equivalent-layer definition after the InternalHeatSource overlay, retaining an explicit outside material, ordered material stack and construction/material graph edges, plus a zero-based family source index corresponding to EnergyPlus's one-based `EQLConsPtr` ordinal
- all equivalent-layer fields, references, material families, contiguity, and shared-name duplication are validated before identity reservation; GetConstructData's family-only acceptance is preserved without inventing alternating solid/gap topology or downstream ASHWAT semantics
- one immutable `ConstructionWindowDataFileRequest` record per valid final-family input retains its normalized search-name snapshot, default-or-explicit retain-case file selector, and zero-based source ordinal in a separate arena
- request publication creates no `ConstructionId`, shared construction-name entry, material, frame/divider, graph edge, or optical state; name collisions remain request data until the deferred file parser can validate and materialize source identities

history_state_ownership:
- TypedModel owns immutable construction layer stacks, optional thermochromic master metadata, optional internal-source declaration metadata, equivalent-layer declaration ordinals, and WINDOW5 request selectors; this checkpoint creates no thermochromic child constructions, global source-presence flag, CTF/QTF or source-node history, equivalent-layer ASHWAT state, WINDOW5 synthesized state, active-state history, or mutable window/surface state

unsupported_state:
- multi-layer or shaded ordinary `Construction` consumption of `WindowMaterial:SimpleGlazingSystem`
- InternalHeatSource lookup or overlay of F-factor, C-factor, AirBoundary, window, complex-fenestration, later equivalent-layer, or WindowDataFile constructions; source warning recoveries for invalid layer interfaces, dimensions, spacing, or perpendicular position; and thermochromic child propagation
- `SetPreConstructionInputParameters` maximum-layer projection, `CheckAndFixCFSLayer`, `FinalizeCFS`, `SetEquivalentLayerWindowProperties`, ASHWAT optical and thermal calculations, nominal equivalent-layer resistance, U-factor/SHGC/rating state, and every equivalent-layer surface or window consumer
- `SearchWindow5DataFile` path lookup and fatal behavior, Unicode/header/EOF and entry matching, one- or two-glazing-system parsing, W5 glass/gas material and construction generation, nominal resistance and U state, angular optical arrays and polynomial fits, frame/divider and mullion synthesis, source collision behavior, and every generated surface, reporting, or runtime consumer
- the global `AnyInternalHeatSourceInInput` and `SimpleCTFOnly` flags, resistance-layer merging and source-node remapping, CTF/QTF generation and histories, source/sink heat fluxes and interior temperatures, radiant-system, ventilated-slab, surface-ground-heat-exchanger, representative-surface, and other downstream consumers
- `CreateTCConstructions` global child allocation and deep copying, ConstructionId/name/count/graph integration, surface-active construction switching, temperature-driven state selection, fenestration binding, optics, thermal calculations, daylighting, shading, nominal-U adjustment, EIO or other construction reporting, and runtime or conformance behavior; CP103's separate immutable projection is covered by its later dedicated contract

inactive_branches:
- ordinary constructions without a thermochromic parent retain their existing effective material stacks and bounded opaque/fenestration classification; for these records the input and effective stacks are identical
- a sole-layer SimpleGlazingSystem construction retains its one source material ID and has no thermochromic metadata; no multi-layer source quirk is materialized
- when more than one thermochromic parent is present, every parent is first-state substituted but only the final parent owns the zero-based master metadata; the later CP103 projection varies only that final retained parent while earlier occurrences stay at their first states
- when no valid `ConstructionProperty:InternalHeatSource` targets an ordinary opaque construction, every construction retains absent internal-source metadata; a retained nonzero perpendicular position on a 1-D declaration has no active consumer
- when no `Construction:WindowEquivalentLayer` definition exists, the construction arena and graph gain no equivalent-layer state; when definitions exist, any one-to-eleven-layer contiguous family-only pack remains declaration data without topology repair or an executable window consumer
- when no `Construction:WindowDataFile` request exists, the request arena is empty; missing or blank file names retain the default selector, and explicit names remain inert retain-case text even when the referenced file does not exist

unsupported_active_branches:
- every typed thermochromic parent remains an all-definition runtime blocker through its existing parent-material capability rule, including an unused parent and a parent consumed by a valid ordinary `Construction`; direct-index structural lookup and the later immutable child projection do not weaken that block
- every typed SimpleGlazingSystem definition remains an all-definition runtime blocker, including an unused definition and one consumed by a valid sole-layer ordinary `Construction`
- valid bounded fenestration constructions remain typed graph state only and do not enter the opaque runtime thermal cache or acquire window execution
- every valid `ConstructionProperty:InternalHeatSource` definition, including one attached only to an otherwise unused construction, is reported as `UnsupportedSurfaceBoundary` and `RunBlocked` with `RuntimeClass::None`; a `BuildingSurface:Detailed` may retain the still-ordinary opaque target identity, but no partial or compatibility runtime is admitted
- every valid `Construction:WindowEquivalentLayer` definition, including every unused definition, is reported as `UnsupportedSurfaceBoundary` and `RunBlocked` with `RuntimeClass::None`; supported equivalent-layer materials and graph edges do not admit surface binding, reporting, or partial window runtime
- every valid `Construction:WindowDataFile` request, including every unused, defaulted, or missing-file request, is reported as `UnsupportedSurfaceBoundary` and `RunBlocked` with `RuntimeClass::None` before file I/O; no synthesized construction identity or partial runtime is admitted

not_claimed_branches:
- complete `GetConstructData` parity, source case-collision and exact duplicate-key behavior, invalid-object and mark-used side effects, exact diagnostics/order/multiplicity, multi-layer or shaded SimpleGlazingSystem source quirks, thermochromic global child ConstructionId/name/count/graph integration and active-state selection beyond CP103's private projection, `SearchWindow5DataFile` external-file expansion and generated state, broad InternalHeatSource target and recovery quirks, equivalent-layer topology repair and ASHWAT consumers, global flags, CTF/QTF calculations, nominal-U, EIO/SQLite and other reporting, window, air-boundary, ground, radiant, or source/sink physics, runtime numerics, and conformance
<!-- routine-state-contract:v1 end get_construct_data -->

## Bounded F/C-Factor Construction Generation Notes

EnergyPlus allocates the common concrete and per-object fictitious insulation
materials from raw F/C counts during material input, then
`CreateFCfactorConstructions` creates every F-factor construction before every
C-factor construction. Rust preserves that family and staged-IDF declaration
order, keeps generated material names private, and retains the exact bounded
input formulas and graph. Surface pairing and ground runtime remain blocked.

### `CreateFCfactorConstructions` state contract

<!-- routine-state-contract:v1 begin create_fc_factor_constructions -->
CreateFCfactorConstructions

read_state:
- raw `Construction:FfactorGroundFloor` and `Construction:CfactorUndergroundWall` counts allocate source-ordered private material slots: public `Material`, common `~FC_Concrete`, public `Material:NoMass`, then one `~FC_Insulation_n` for every raw F object followed by every raw C object; staged IDF declaration-order overlays preserve ordinary, F-family, and C-family ordinals independently
- each F-factor record has a required nonblank shared Construction name plus required finite `f_factor` and `area` greater than zero and exact epJSON `perimeterexposed` greater than or equal to zero; positive perimeter derives `Reff = area/(perimeter*f_factor)-0.135-0.03`, while zero perimeter derives `Reff = 177`
- each C-factor record has a required nonblank shared Construction name plus required finite `c_factor` and `height` greater than zero; equivalent soil resistance is 0.12 at height less than or equal to 0.25 m, 0.92 at height greater than or equal to 2.5 m, and `0.0607+0.3479*height` between, with `Reff = 1/c_factor+soil resistance`
- the common MediumRough concrete has thickness 0.15 m, conductivity 1.95 W/m-K, density 2240 kg/m3, specific heat 900 J/kg-K, and default opaque absorptances; each MediumRough resistance-only insulation has all three absorptances zero and derives `Rfic = Reff-(0.15/1.95)`, which must be finite and greater than zero together with finite `Reff`

write_state:
- one typed opaque `Construction` per valid record after every ordinary Construction and in all-F-then-all-C order, with a distinct F-factor or C-factor metadata discriminant retaining source inputs, derived effective resistance, generated insulation resistance, and C-factor soil resistance where applicable
- each generated construction owns the outside-to-inside layer stack `[private insulation, private concrete]`; the construction/material graph emits both ordered edges and the raw-ordinal insulation slot receives the derived resistance
- internal materials live in the typed material arena but are deliberately absent from public `material_names` and every public attachment-target lookup; exact normalized generated-name collisions from user material families fail closed, while nonidentical names such as `~FC_Insulation_01` remain public
- invalid fields, nonpositive or nonfinite derived insulation resistance, and duplicate Construction names reserve no Construction ID or name; raw-count internal slots and later-family ordinals remain stable, while exact EnergyPlus invalid-object and collision side effects are not reproduced

history_state_ownership:
- TypedModel owns immutable generated ground-factor construction metadata and layer stacks; no surface-local ground-temperature, CTF, CondFD, HAMT, or mutable heat-balance history is allocated

unsupported_state:
- `GroundFCfactorMethod` surface-type pairing, detailed-surface and rectangular-surface boundary conversion, F area/perimeter and C height geometry checks, `Site:GroundTemperature:FCfactorMethod`, EPW fallback, monthly ground state, CTF/CondFD/HAMT calculation, and ground heat-balance execution
- public attachment targeting of private internal materials, EMS mutation, bounded-out `ConstructionProperty:InternalHeatSource` targeting of generated F/C constructions, nominal-U adjustment, generic material/construction EIO timing, envelope/SQLite reporting, exact diagnostics/order/multiplicity, and conformance evidence

inactive_branches:
- when no raw F-factor or C-factor definition exists, no internal concrete or insulation material is injected and ordinary material/construction behavior is unchanged
- F-factor exposed perimeter exactly zero uses the source 177 m2-K/W effective-resistance branch; C-factor heights exactly 0.25 m and 2.5 m use the lower and upper constant soil-resistance branches respectively

unsupported_active_branches:
- every valid F-factor or C-factor definition, including every unused definition, is reported as `UnsupportedSurfaceBoundary` and `RunBlocked`; no partial or compatibility runtime is admitted
- `BuildingSurface:Detailed` deliberately rejects a generated F/C-factor construction until the downstream `GroundFCfactorMethod` pairing and validation checkpoint is ported; reporting and runtime consumers likewise filter generated stacks from ordinary opaque constructions

not_claimed_branches:
- surface binding and geometry validation, ground-temperature and CTF/runtime behavior, internal-material public attachment behavior, source collision and invalid-object side effects, no-thermal-mass flags, nominal-U and reporting parity, exact diagnostics/order/multiplicity, runtime numerics, EIO/SQLite, and conformance
<!-- routine-state-contract:v1 end create_fc_factor_constructions -->

## Bounded Air-Boundary Construction Input Notes

EnergyPlus calls `CreateAirBoundaryConstructions` after ordinary construction
input and F/C-factor generation. Unlike those preceding families, it iterates
the epJSON object map directly, so the bounded Rust pass deliberately retains
case-sensitive lexical instance-key order instead of adding an IDF declaration
order overlay. Rust waits until the typed top-level schedule namespace is
complete before materializing these records, while still appending them after
all ordinary, F-factor, and C-factor constructions. This compiler pass-order
deviation is explicit and does not claim complete `GetConstructData` parity.

The checkpoint owns declaration state only. Official AirBoundary EIO evidence
depends on deferred surface conversion, enclosure grouping, and generated
cross-mixing, so no report comparator or numerical conformance claim is added.

### `CreateAirBoundaryConstructions` state contract

<!-- routine-state-contract:v1 begin create_air_boundary_constructions -->
CreateAirBoundaryConstructions

read_state:
- the `Construction:AirBoundary` epJSON object map after every ordinary, F-factor, and C-factor construction; EnergyPlus iterates that map directly, so records materialize in case-sensitive lexical key order rather than staged source-IDF declaration order, and Rust preserves the same family-local order
- one required nonblank outer-key name in the shared case-insensitive Construction namespace; `air_exchange_method` missing or blank defaults to `None` and otherwise accepts case-insensitive `None` or `SimpleMixing` within the Rust parser convention
- a supplied nonblank `simple_mixing_air_changes_per_hour` must be finite and nonnegative without an upper bound; `SimpleMixing` applies the source default 0.5 when it is missing or blank, while `None` retains source-effective zero mixing state
- only `SimpleMixing` consumes `simple_mixing_schedule_name`: missing or Rust-normalized blank input selects an explicit always-on sentinel, while a nonblank name resolves case-insensitively through the existing typed top-level schedule namespace after Rust schedule materialization; `None` ignores the schedule field and creates no schedule dependency

write_state:
- one dedicated zero-layer air-boundary construction descriptor per valid record, appended after every C-factor construction with a shared `ConstructionId` and normalized Construction name but no outside material, material layer, thermochromic or ground-factor metadata, or construction/material graph edge
- `None` retains false simple-mixing state, zero air changes per hour, and no schedule selector; inactive supplied mixing fields do not create executable state
- `SimpleMixing` retains true mixing state, the effective nonnegative air-changes-per-hour value, and either a resolved user `ScheduleId` or an explicit always-on selector without synthesizing another typed Schedule object
- blank names, malformed or out-of-range numeric input, invalid enums, unresolved active schedule references, and normalized duplicate Construction names fail before Rust ID or name reservation; this deliberately does not reproduce source partial-slot and name-reservation side effects after an active schedule lookup failure

history_state_ownership:
- TypedModel owns immutable zero-layer air-boundary and optional simple-mixing descriptors; no surface pair, enclosure, cross-mixing object, schedule current value, or mutable heat-balance history is allocated

unsupported_state:
- base-surface or fenestration-surface binding, `Surface` or `Zone` outside-boundary validation, paired nonadiabatic interzone checks, child-fenestration exclusion, `AirBoundaryNoHT` surface conversion, and surface heat-transfer suppression
- solar, radiant, and daylighting enclosure grouping, `AnyAirBoundary`, view-factor and enclosure remapping, and every geometry-dependent surface mutation
- space-pair deduplication, minimum-space-volume times air-changes-per-hour divided by 3600 flow derivation, AirflowNetwork suppression, generated `ZoneCrossMixing`, schedule current-value consumption, zone mixing calculations, and mixing output variables
- nominal-U and CTF skip behavior beyond the typed zero-layer flag, construction or surface EIO/SQLite reporting, exact diagnostics, and conformance evidence

inactive_branches:
- when no `Construction:AirBoundary` definition exists, ordinary, F-factor, and C-factor construction behavior is unchanged
- `None` leaves simple mixing disabled with zero air changes and no schedule selector even when inactive mixing fields are supplied; their schedule name is not resolved
- `SimpleMixing` with no effective schedule name retains only the explicit always-on selector; no schedule sampling or downstream mixing state is activated

unsupported_active_branches:
- every valid `Construction:AirBoundary` definition, including every unused `None` or `SimpleMixing` definition, is reported as `UnsupportedSurfaceBoundary` and `RunBlocked`; no partial or compatibility runtime is admitted
- `BuildingSurface:Detailed` deliberately rejects an air-boundary construction until interzone pairing, enclosure grouping, no-heat-transfer surface conversion, and optional simple-mixing consumers are ported; a supported referenced schedule does not weaken this block

not_claimed_branches:
- complete `GetConstructData` or Rust compiler pass-order parity, source case-collision lookup and invalid-after-reservation side effects, explicit-empty native-epJSON behavior, canonical enum casing, exact diagnostics/order/multiplicity, surface and enclosure behavior, schedule sampling, AirflowNetwork interaction, zone mixing, reporting, runtime numerics, EIO/SQLite, and conformance
<!-- routine-state-contract:v1 end create_air_boundary_constructions -->

## Bounded Complex-Fenestration State Input Notes

EnergyPlus calls `SetupComplexFenestrationStateInput` after AirBoundary input
and before `ConstructionProperty:InternalHeatSource`. The source uses
`getObjectItem` for complex states, so staged IDF input retains family-local
declaration order; native epJSON without the overlay follows its object-map
order. `ConvertInputFormat` can reorder that map, so broad IDF/native-epJSON
order parity is not claimed.

This checkpoint owns only the bounded LBNLWINDOW/None declaration graph. A CFS
presence activates all-definition validation of raw `WindowThermalModel:Params`
helpers. `Matrix:TwoDimension` remains lazy until at least one nonblank CFS
name survives the shared Construction-name collision gate; the first surviving
candidate then validates every matrix definition, including unused ones.
Thermal and matrix identities resolve through normalized case-insensitive keys,
while retained matrix snapshots preserve their original spelling. Neither raw
helper family enters public typed object coverage.

The only EnergyPlus EIO-observable complex-window rows require deferred
fenestration-surface, BSDF optical, or complex thermal consumers. No manifest,
comparator, proof variable, numerical runtime, or conformance claim is added.

### `SetupComplexFenestrationStateInput` state contract

<!-- routine-state-contract:v1 begin setup_complex_fenestration_state_input -->
SetupComplexFenestrationStateInput

read_state:
- the `Construction:ComplexFenestrationState` family after every ordinary, F-factor, C-factor, and AirBoundary construction; staged IDF input preserves family-local declaration order, while native epJSON without that overlay retains lexical object-map order
- one required nonblank name in the shared case-insensitive Construction namespace; missing or blank basis fields default to `LBNLWINDOW` and `None`, and only that combination is admitted while `UserDefined` and `Axisymmetric` fail closed
- when at least one complex state exists, every raw-only `WindowThermalModel:Params` definition is validated before state inspection; `Matrix:TwoDimension` loading remains lazy until one nonblank state name passes the shared Construction-name collision gate, then every matrix definition including unused entries is validated before any surviving state identity is materialized
- positive finite matrix shapes and effective row-major prefixes drive a one- or two-column LBNL basis, four square global optical matrices, and one-row front/back absorptance matrices per solid; layers form a contiguous alternating one-to-five-solid and zero-to-four-gap pack ending in a solid, accepting only SpectralAverage glazing or ComplexShade at solid positions and WindowMaterial:Gap at gap positions with reserved gap matrix fields blank

write_state:
- one `ConstructionKind::ComplexFenestration` record per valid definition after every AirBoundary record, with shared `ConstructionId`, normalized name, outside layer, ordered material stack, no thermochromic, ground-factor, or air-boundary metadata, and staged-IDF family order when available
- immutable `ConstructionComplexFenestrationState` metadata retains LBNL/None selectors, the resolved thermal-model snapshot, basis matrix and derived basis length, four global optical matrix snapshots, and ordered solid-layer front/back absorptance snapshots; thermal and matrix lookup plus duplicate detection are normalized case-insensitively, retained matrix snapshots preserve original spelling, and raw helper families remain outside public typed object coverage
- ordered construction/material graph edges cover every solid and gap layer, and material-family validation prevents ordinary, equivalent-layer, simple-glazing, thermochromic, or unsupported BSDF glazing variants from entering the pack
- all helper catalogs, references, matrix dimensions, layer topology, reserved fields, and shared-name duplication are validated before identity reservation; an invalid earlier state does not consume the shared name needed by a later valid case-variant definition

history_state_ownership:
- TypedModel owns immutable declaration, helper snapshots, and ordered layer graph state; no BSDF basis cache, surface-local optical/thermal state, deflection history, TARCOG/WCE state, or mutable window history is allocated

unsupported_state:
- `UserDefined` bases, `Axisymmetric` symmetry, regular-glazing `BSDF`, Spectral and SpectralAndAngle glazing consumers, nonblank reserved gap matrix fields, unsupported solid/gap material families, and any layer topology outside the bounded alternating pack
- fenestration-surface binding, complex-window initialization, basis expansion beyond retained input snapshots, BSDF optical calculations, deflection and support-pillar algorithms, TARCOG/WCE thermal calculations, shading, ratings, daylighting, nominal-U or CTF state, output variables, and EIO/SQLite or other reporting

inactive_branches:
- when no `Construction:ComplexFenestrationState` definition exists, raw thermal-model and matrix helpers are not parsed by this pass; when definitions exist but every nonblank name collides with the shared Construction namespace, thermal helpers are still validated but the matrix catalog remains untouched
- a one-column basis uses its row count directly; a two-column basis uses the EnergyPlus count projection retained by the bounded LBNL branch, while extra finite matrix values beyond rows times columns are ignored
- a single solid needs no gap, while each higher populated position must preserve solid-gap alternation and every omitted trailing layer leaves no typed or graph state

unsupported_active_branches:
- every valid `Construction:ComplexFenestrationState` definition, including every unused definition, is reported as `UnsupportedSurfaceBoundary` and `RunBlocked` with `RuntimeClass::None`; no partial or compatibility runtime is admitted
- `BuildingSurface:Detailed` deliberately rejects a complex-fenestration construction until the dedicated fenestration-surface, BSDF optical, and complex-window thermal consumers are ported; supported material and helper declarations do not weaken that block

not_claimed_branches:
- complete `GetConstructData` parity, source partial-slot allocation and invalid-object side effects, broad IDF/native-epJSON order parity, source case-collision behavior, exact diagnostics/order/multiplicity, custom or axisymmetric bases, broad matrix semantics, surface and window behavior, optics, thermal calculations, ratings, daylighting, reporting, runtime numerics, EIO/SQLite, and conformance
<!-- routine-state-contract:v1 end setup_complex_fenestration_state_input -->

## Bounded Internal Heat Source Overlay Notes

EnergyPlus reads the `ConstructionProperty:InternalHeatSource` epJSON object
map directly after complex-fenestration states and before the following typed
equivalent-layer family and deferred WindowDataFile branch. The bounded Rust
overlay therefore uses case-sensitive lexical outer-key order without an IDF
declaration-order overlay. Its required outer key is retained only as a
normalized diagnostic source-name snapshot, not as a typed identity or lookup
namespace; case-colliding source names may consequently attach to different
targets.

The admitted target is an already materialized ordinary opaque construction
with at least two effective layers. Both one-based layer interfaces must be in
`1..TotLayers-1`; CTF dimensions must be exactly 1 or 2, tube spacing must be
finite in `[0.01,1.0]` m, and the optional finite perpendicular temperature
position defaults to 0 and must remain in `[0,1]`. Every field and target
constraint is validated before attachment, so an invalid earlier lexical
record does not reserve the target, the first valid attachment wins, and a
later valid record for the same construction fails closed without replacement.
The overlay changes no Construction identity, kind, layer stack, material
state, or graph edge.

Every valid definition remains an exact `UnsupportedSurfaceBoundary` /
`RunBlocked` declaration with `RuntimeClass::None`, including an unused target.
No manifest, comparator, proof variable, runtime, reporting, or conformance
claim is added. Broader F/C-factor, AirBoundary, window, complex-fenestration,
later equivalent-layer, and WindowDataFile targets; source warning recoveries;
thermochromic-child propagation; global source-presence flags; resistance-layer
remapping; CTF/QTF generation and histories; and every surface, radiant, or
source/sink consumer remain deferred.

## Bounded Equivalent-Layer Window Construction Notes

EnergyPlus reads `Construction:WindowEquivalentLayer` after the internal heat
source overlay through `getObjectItem`, so staged IDF declaration order is
semantic for the family. Rust adds that object to the existing order overlay;
native epJSON without the overlay continues to follow its case-sensitive
lexical instance-key order. The required name shares the normalized
Construction namespace. The required outside layer and optional layers 2
through 11 must form a contiguous prefix and resolve case-insensitively to the
typed EquivalentLayer material family.

Each definition becomes a dedicated `ConstructionKind::WindowEquivalentLayer`
with its ordered layer stack, construction/material graph edges, and zero-based
family ordinal corresponding to EnergyPlus's one-based `EQLConsPtr`. All
fallible validation precedes identity reservation. The source input routine
accepts family-valid packs without enforcing solid/gap alternation, and Rust
preserves that declaration behavior rather than projecting rules from later
window initialization.

Every equivalent-layer construction, including an unused definition, is an
exact `UnsupportedSurfaceBoundary` / `RunBlocked` declaration with
`RuntimeClass::None`. `SetPreConstructionInputParameters` maximum-layer state,
`CheckAndFixCFSLayer`, `FinalizeCFS`, `SetEquivalentLayerWindowProperties`,
ASHWAT optical/thermal calculations, ratings, surfaces, runtime execution,
reporting, and conformance remain deferred. The following WindowDataFile request
is typed separately, but its external-file expansion remains unported, so
`GetConstructData` stays source-mapped.

## Bounded WINDOW5 Data-File Request Notes

EnergyPlus reads `Construction:WindowDataFile` through `getObjectItem` after
every equivalent-layer construction. Rust therefore adds it to the staged-IDF
order overlay; native epJSON without the overlay retains lexical instance-key
order. The required object name is a normalized WINDOW5 entry search snapshot.
The optional retain-case file field becomes either an explicit
`DefaultWorkingDirectory` selector for `Window5DataFile.dat` or an exact
nonblank string. The IDD's 100-character text is a note, not a schema/source
validation rule, so the bounded compiler does not invent a length limit.

Requests live in a separate `ConstructionWindowDataFileRequest` arena with a
zero-based source ordinal. They deliberately reserve no `ConstructionId` or
shared name, and produce no generated material, construction/material edge,
frame/divider, or optical state. Consequently request case collisions and
collisions with existing construction names remain inert request data until the
file-expansion routine is ported; a surface cannot resolve a request as a
construction.

Every request, including a defaulted, unused, or nonexistent-file request, is
an exact `UnsupportedSurfaceBoundary` / `RunBlocked` declaration with
`RuntimeClass::None` before file access. `SearchWindow5DataFile` remains only
source-mapped: path search/fatal behavior, Unicode/header/EOF handling, entry
matching, one- or two-system expansion, W5 glass/gas material generation,
nominal resistance/U state, angular optics and fits, `:2` construction naming,
frame/divider and mullion synthesis, surfaces, reporting, runtime, and
conformance are deferred.

## Bounded Zone Declaration Notes

EnergyPlus calls `GetBuildingData` immediately after `GetConstructData` and
orders `GetShadowingInput`, `GetZoneData`, then `SetupZoneGeometry`.
`GetZoneData` sizes the Zone, ZoneDaylight, and resilience arrays before reading
every `Zone` through `getObjectItem` and delegating its public fields to
`ProcessZoneData`. After that loop it derives nominal equipment-control flags,
processes ZoneList and ZoneGroup, reads local-environment data, allocates
ZonePreDefRep, and calls `GetSpaceData`. Rust keeps both wrappers source-mapped,
retains the complete immutable `ProcessZoneData` declaration state, and now
continues the bounded `GetZoneData` state through nominal-control marking,
ZoneList, ZoneGroup, `GetZoneLocalEnvData`, and `GetSpaceData` while explicitly
omitting the intervening ZonePreDefRep reporting allocation.

The staged-IDF order overlay includes `Zone`, `ZoneList`, `ZoneGroup`, and
`ZoneProperty:LocalEnvironment`, so those dense arenas follow family-local IDF
declaration order. Native epJSON without that overlay uses lexical instance-key
order. `Space` and `SpaceList` deliberately do not use the overlay: EnergyPlus
directly iterates their JSON object maps, so both staged IDF and native epJSON
use lexical outer-key order. All 12 Zone public fields plus the required
object-key name are validated before the dense ID and normalized name are
published. A positive authored floor area is preferred by current area
consumers; an authored local convection selector is retained but blocks
arbitrary execution until the zone-local coefficient path consumes it.

After the Zone arena is complete, the bounded equipment-connection scan sets
each Zone's nominal-control flag solely from case-insensitive raw `zone_name`
presence. ZoneList then retains nonempty, ordered, resolved Zone membership and
the longest authored member name. ZoneGroup retains its resolved list and
positive multiplier, rejects repeated or overlapping grouped lists, and writes
the multiplier plus the source-shaped ZoneList identity to every member Zone.
Every ZoneList and ZoneGroup definition remains fail-closed because current
Zone-or-ZoneList consumers and comprehensive list-multiplier runtime paths are
not yet wired.

`GetZoneLocalEnvData` retains each resolved Zone and its optional generic node.
A nonblank node can name a one-member NodeList alias or a direct node that is
registered or reused. Ordered nonblank definitions overwrite the linked node
on their Zone, while a later blank node does not clear an earlier link. The
EnergyPlus 26.1 source checks outdoor-node membership only behind a
short-circuited zero-node condition, so this bounded declaration phase does not
require a direct node to have an `OutdoorAir:Node` definition. NodeList parsing
therefore moves immediately before this phase, matching the source routine's
lazy single-node lookup dependency without otherwise changing node order.
Every local-environment definition remains fail-closed until its mutable local
weather state and downstream consumers are wired.

`GetSpaceData` next retains authored Spaces in lexical order, including their
resolved Zone, three numeric-or-Autocalculate geometry selectors, first-seen
case-insensitive Space type identity, and ordered tags. It then retains lexical
SpaceLists whose member arrays resolve authored Spaces in authored array order;
missing and empty arrays are valid. Finally, it visits Zones in typed order and
adds one General whole-zone default to every Zone without an authored Space.
Those late defaults are unavailable to the preceding SpaceList lookup but do
remain in EnergyPlus's shared Space array for later surface lookup. The bounded
CP97 `GetHTSurfaceData` cross-section resolves optional typed
`BuildingSurface:Detailed.space_name` values through that full pre-remainder
arena and validates same-Zone membership while the full routine remains
`source_mapped`. State-mapped `CreateMissingSpaces` then consumes those
preclassified assignments and gives every retained detailed opaque surface a
final SpaceId. A Zone mixing explicit and implicit assignments receives one
General `AutoZoneRemainder`; an all-implicit Zone uses its existing last Space
and an all-explicit Zone creates no remainder. Authored Spaces, all SpaceLists,
and generated remainders fail closed, while a sole whole-zone default remains
inactive when no remainder is needed, including when every valid surface
explicitly references it.
Other surface families, opposite-surface generation, reordering, Space surface
lists, and all derived geometry remain with later checkpoints.

### `GetZoneData` bounded collection state contract

<!-- routine-state-contract:v1 begin get_zone_data -->
GetZoneData

read_state:
- after all `ProcessZoneData` calls complete, EnergyPlus derives nominal control for every Zone, processes every ZoneList and ZoneGroup, calls `GetZoneLocalEnvData`, allocates ZonePreDefRep, then calls `GetSpaceData`; bounded Rust preserves those declaration phases while explicitly omitting the intervening reporting allocation
- nominal control is a case-insensitive field-value existence scan independent of full equipment-connection field parsing; every Zone is explicitly retained as true for a matching raw zone name or false otherwise
- each ZoneList has one required nonblank outer-key name and a nonempty `zones` array whose ordered entries each require one case-insensitively resolved Zone name; staged IDF lists use recovered declaration order while native epJSON lists use lexical outer-key order
- each ZoneGroup has one required nonblank outer-key name, one required case-insensitively resolved ZoneList name, and an integer multiplier defaulting to one in the source-compatible positive signed-integer range; staged IDF groups use recovered declaration order while native epJSON groups use lexical outer-key order
- each ZoneProperty:LocalEnvironment has one required nonblank outer-key name, one required case-insensitively resolved nonblank Zone name, and an optional outdoor-air node name; bounded Rust initializes NodeList declarations immediately before this phase because EnergyPlus `GetOnlySingleNode` lazily initializes them
- GetSpaceData reads authored Space then SpaceList outer keys in lexical order for both staged IDF and native epJSON, preserves each tag/member array order, and finally visits Zones in typed Zone order to create whole-zone defaults

write_state:
- each typed Zone retains `is_nominal_controlled`, default false and set true solely by the bounded raw equipment-connection scan without claiming full connection validity
- a deterministic dense ZoneList arena and independent normalized name map retain each validated `ZoneListId`, normalized name, ordered ZoneId members, and maximum authored member-name length; empty, malformed, unresolved, or duplicate members fail before identity publication, while a Zone-name collision emits a nonblocking warning
- a deterministic dense ZoneGroup arena and independent normalized name map retain each validated `ZoneGroupId`, normalized name, ZoneListId, and multiplier; repeated list use or grouped-list Zone overlap fails before publication, while every valid member Zone receives the multiplier and the source-shaped ZoneList identity in `list_group`
- a deterministic dense ZoneLocalEnvironment arena and independent normalized name map retain each validated declaration, its ZoneId, and optional generic NodeId; each nonblank node resolves a one-member NodeList alias or registers/reuses a direct node, and ordered nonblank links overwrite the Zone link while a later blank node does not clear it
- deterministic Space, SpaceList, and first-seen SpaceType state retain validated authored declarations, ordered Zone/member links, then one whole-zone default for each Zone still without a Space
- every valid ZoneList or ZoneGroup is reported as `UnsupportedZoneGrouping`, every valid ZoneProperty:LocalEnvironment as `UnsupportedZoneLocalEnvironment`, and every authored Space or SpaceList as `UnsupportedSpacePartitioning`; the later CreateMissingSpaces contract applies that partition boundary to generated remainders while leaving sole whole-zone defaults inactive

history_state_ownership:
- TypedModel owns immutable nominal-control, ZoneList, ZoneGroup, ZoneLocalEnvironment, Space, SpaceList, SpaceType, and per-Zone list/local-node/space declaration state only; this checkpoint allocates no mutable local weather, geometry, sizing, reporting, equipment, surface, space-air, or zone-air history

unsupported_state:
- the pre-Zone-loop ZoneDaylight and resilience allocations and the post-`GetZoneLocalEnvData` ZonePreDefRep allocation
- ZoneList expansion for People, gains, thermostat, sizing, and every other Zone-or-ZoneList consumer, plus comprehensive `Zone.Multiplier * Zone.ListMultiplier` consumption across geometry, loads, HVAC flows, sizing, and reports
- OutdoorAir:Node condition inputs and node-connection metadata, Space-or-SpaceList target expansion, SetupZoneGeometry behavior beyond the separately bounded CP97 GetHTSurfaceData/CreateMissingSpaces surface-membership composite, local weather and EMS state, space heat balance, reporting, runtime numerical behavior, and conformance evidence

inactive_branches:
- when no raw equipment connection names a Zone, its nominal-control flag remains false without a diagnostic; unmatched connection names mark no Zone in this bounded phase
- when no ZoneList, ZoneGroup, or ZoneProperty:LocalEnvironment exists, every Zone retains list multiplier one, no list-group identity, and no linked outdoor-air node, adding no boundary from those absent families
- a ZoneGroup multiplier omitted from valid input retains one but still records group membership exactly like an authored multiplier and remains inside the fail-closed grouping boundary
- a blank or missing local-environment node remains an explicit no-node declaration and does not clear an earlier nonblank link for the same Zone
- when no authored Space exists, GetSpaceData still creates one General whole-zone default per Zone; that default adds no space-partition runtime boundary, including when every valid typed detailed surface explicitly references the sole default and no remainder is needed

unsupported_active_branches:
- every valid ZoneList, including an otherwise unused definition, is typed but blocks arbitrary runtime execution until all Zone-or-ZoneList consumers expand it
- every valid ZoneGroup, including multiplier one, is typed but blocks arbitrary runtime execution until list multiplier semantics are comprehensively consumed
- every valid ZoneProperty:LocalEnvironment, including an otherwise unused definition or one with a blank node, is typed but blocks arbitrary runtime execution until local weather consumers are wired
- every authored Space and every SpaceList, including an otherwise unused or empty list, is typed but blocks arbitrary runtime execution until space partitioning and all downstream consumers are wired; generated remainder Spaces are blocked by the separate CreateMissingSpaces contract
- typed BuildingSurface:Detailed lookup and same-Zone validation belong to the bounded GetHTSurfaceData cross-section, while final fallback assignment and mixed-assignment remainder generation belong to the state-mapped CreateMissingSpaces contract; the composite does not promote full GetHTSurfaceData or this parent routine
- the nominal-control scan does not validate or execute ZoneHVAC equipment; the existing later typed equipment-connection and IdealLoads boundaries remain authoritative

not_claimed_branches:
- complete GetZoneData parity, broad compiler pass-order parity, source partial-allocation and invalid-input recovery side effects, exact diagnostics/text/order/multiplicity, whitespace-preserving or case-colliding names, complete shared Zone/Space/ZoneList/SpaceList namespace behavior, full OutdoorAir:Node/NodeList and local-weather state, ZonePreDefRep, SetupZoneGeometry beyond the separate bounded surface-membership slice, sizing, reporting, numerical parity, and conformance
<!-- routine-state-contract:v1 end get_zone_data -->

### `GetZoneLocalEnvData` state contract

<!-- routine-state-contract:v1 begin get_zone_local_env_data -->
GetZoneLocalEnvData

read_state:
- EnergyPlus calls `GetZoneLocalEnvData` after ZoneGroup processing and before ZonePreDefRep allocation and `GetSpaceData`; bounded Rust moves existing NodeList parsing immediately before this phase because `GetOnlySingleNode` lazily initializes NodeLists
- each ZoneProperty:LocalEnvironment has a required nonblank outer-key name and case-insensitively resolved nonblank `zone_name`; `outdoor_air_node_name` is optional, and blank or missing input retains no node; staged IDF instances use recovered declaration order while native epJSON instances use lexical outer-key order
- a nonblank node first resolves a typed NodeList alias, accepting exactly one member and rejecting multiple members; otherwise an existing generic NodeId is reused or a new generic NodeId is registered
- EnergyPlus 26.1 checks OutdoorAir:Node membership only inside a short-circuited `NodeNum == 0` condition, so bounded declaration state likewise does not require a nonblank direct node to have an OutdoorAir:Node definition
- ordered declarations for the same Zone are allowed; each nonblank resolved node overwrites that Zone's link while a later blank node does not clear the previous link, so the last nonblank node wins

write_state:
- a deterministic dense ZoneLocalEnvironment arena and independent normalized name map retain each fully validated ZoneLocalEnvironmentId, normalized name, ZoneId, and optional generic NodeId; validation completes before identity, node, or Zone-link publication, so an invalid earlier record consumes none of those bounded identities or side effects
- each Zone retains an optional linked outdoor-air NodeId; a one-member NodeList stores its member node, a direct name registers or reuses that generic node, and blank input stores no node
- every valid ZoneProperty:LocalEnvironment definition is reported as `UnsupportedZoneLocalEnvironment` and `RunBlocked` before arbitrary runtime execution because mutable local-weather conditions and their consumers are not wired

history_state_ownership:
- TypedModel owns immutable local-environment declaration and per-Zone optional generic-node linkage only; this checkpoint allocates no mutable dry-bulb, wet-bulb, wind, psychrometric, EMS, surface, load, or zone-air history

unsupported_state:
- complete OutdoorAir:Node condition input including height, dry-bulb/wet-bulb/wind schedules, wind-pressure-coefficient curve and symmetry/angle controls, plus NodeConnection metadata
- AnyLocalEnvironmentsInModel and mutable local dry-bulb, wet-bulb, humidity, pressure, wind, psychrometric, EMS, weather, surface, infiltration, load, and zone-air consumer state
- source-sized partial records and invalid-input side effects and the post-routine ZonePreDefRep allocation; GetSpaceData declaration/default state belongs to a separate bounded contract

inactive_branches:
- when no ZoneProperty:LocalEnvironment exists, the arena stays empty, every Zone node link stays absent, and no local-environment runtime boundary is added
- a blank or missing node is a valid no-node declaration and does not clear an earlier nonblank link for the same Zone
- multiple declarations may reference one Zone; only ordered declarations with a nonblank resolved node overwrite its link

unsupported_active_branches:
- every valid definition, including an otherwise unused record or a record with no node, blocks arbitrary runtime execution until local-weather consumers are wired
- a nonblank direct generic NodeId does not claim an OutdoorAir:Node declaration or any local environmental conditions

not_claimed_branches:
- complete GetZoneLocalEnvData parity, source preallocation and partial invalid-record/node/link side effects, the source multi-member NodeList first-node side effect, exact diagnostics/text/order/multiplicity, whitespace-preserving or case-colliding names, OutdoorAir:Node condition state, local-weather consumers, ZonePreDefRep, geometry, reporting, numerical parity, and conformance
<!-- routine-state-contract:v1 end get_zone_local_env_data -->

### `GetSpaceData` state contract

<!-- routine-state-contract:v1 begin get_space_data -->
GetSpaceData

read_state:
- EnergyPlus calls `GetSpaceData` after `GetZoneLocalEnvData` and ZonePreDefRep allocation and before `SetupZoneGeometry`; bounded Rust calls its declaration/default phase immediately after local-environment parsing while leaving that intervening reporting allocation deferred
- authored Space outer keys are always lexical for both staged IDF and native epJSON because the source directly iterates the JSON object map; each Space requires a nonblank name and resolved nonblank Zone, while ceiling height, volume, and floor area preserve every finite number or default missing, blank, and Autocalculate input to AutoCalculate
- space type defaults to General for missing or blank input and joins a case-insensitive first-seen lexical-order registry; tag objects preserve array order, normalized text, duplicates, and empty or missing tag strings
- SpaceList outer keys are likewise always lexical while member arrays preserve authored order; missing or empty member arrays are valid, each present member resolves an authored Space, duplicate resolved members fail, and list names colliding with a Zone or authored Space fail
- after all lists are processed, Zones are visited in typed order and each Zone without an authored Space receives one appended whole-zone default named after the Zone with AutoCalculate geometry, no tags, and the General space type; its late position keeps it unavailable to the preceding SpaceList lookup while the later bounded GetHTSurfaceData cross-section can resolve it through the full pre-remainder Space arena

write_state:
- a deterministic dense Space arena retains each fully validated authored declaration followed by generated whole-zone defaults; authored names alone enter the reference map, validation completes before SpaceId, SpaceTypeId, or Zone-link publication, and each Space retains its ZoneId, three AutoOrNumber selectors, normalized type name/id, ordered tags, and origin
- each Zone retains ordered SpaceIds and exits the routine with at least one Space; generated defaults reuse or append General through the separately bounded `GetGeneralSpaceTypeNum` helper and remain outside the authored-name map used by the preceding SpaceList phase
- a deterministic dense SpaceList arena and normalized name map retain each validated SpaceListId, lexical name, ordered authored SpaceIds, and maximum authored member-name length, including valid zero-member lists
- every authored Space and SpaceList is reported as `UnsupportedSpacePartitioning` and `RunBlocked` before arbitrary runtime execution; the separate CreateMissingSpaces contract applies the same boundary to generated remainder Spaces, while sole generated whole-zone defaults remain inactive even when valid detailed surfaces explicitly reference them

history_state_ownership:
- TypedModel owns immutable authored/default Space, SpaceList, SpaceType, and per-Zone SpaceId declaration topology only; this checkpoint allocates no mutable geometry, surface, space-air, zone-air, gain, HVAC, sizing, or reporting history

unsupported_state:
- the preceding ZonePreDefRep allocation and all predefined-report mutation
- SetupZoneGeometry behavior beyond the separately bounded CP97 composite of GetHTSurfaceData typed detailed-surface lookup/same-Zone validation and CreateMissingSpaces remainder/fallback finalization: other surface families, opposite-surface generation, reordering, Space surface lists, calculated height/volume/floor area, floor/volume fractions, enclosure and surface ranges, and geometry correction/warnings
- Space-or-SpaceList target expansion for internal gains, infiltration, mixing, ventilation, sizing, outdoor air, HVAC, outputs, and every space heat-balance/reporting/runtime consumer

inactive_branches:
- with no authored Space, every Zone receives one General whole-zone default and no UnsupportedSpacePartitioning boundary is added when detailed surfaces are all implicit or all validly explicit to that sole default
- a Zone with one or more authored Spaces receives no whole-zone default; if every Zone is covered and all authored types are non-General, the registry contains no General entry
- a SpaceList with a missing or empty member array is retained as a valid zero-member list but remains inside the all-definition runtime boundary

unsupported_active_branches:
- every authored Space, including one using only defaults and not referenced by a surface or load, blocks arbitrary runtime execution until space partition consumers are wired
- every SpaceList, including an empty or otherwise unused definition, blocks arbitrary runtime execution until all Space-or-SpaceList consumers expand it
- typed BuildingSurface:Detailed lookup and same-Zone validation belong to the bounded GetHTSurfaceData cross-section, while final fallback assignment and mixed-assignment remainder generation belong to the state-mapped CreateMissingSpaces contract; this declaration routine does not claim either part

not_claimed_branches:
- complete GetSpaceData parity, source allocation sizes and partial invalid Space/SpaceType/Zone/list side effects, exact diagnostics/text/order/multiplicity, source whitespace preservation or case-colliding same-family names, complete shared Zone/ZoneList/Space/SpaceList namespace behavior, SetupZoneGeometry beyond the separately bounded typed detailed-surface membership slice, loads, space heat balance, HVAC, reporting, numerical parity, and conformance
<!-- routine-state-contract:v1 end get_space_data -->

### `GetGeneralSpaceTypeNum` state contract

<!-- routine-state-contract:v1 begin get_general_space_type_num -->
GetGeneralSpaceTypeNum

read_state:
- GetSpaceData calls `GetGeneralSpaceTypeNum` while generating a whole-zone default and CreateMissingSpaces calls it while generating a mixed-assignment remainder; the helper searches the ordered type registry built from validated authored Spaces and any earlier generated default using case-insensitive General matching

write_state:
- the helper reuses the first existing General SpaceTypeId or appends one General entry at the end and returns that dense identity; every generated whole-zone default and remainder in the same model reuses it

history_state_ownership:
- TypedModel owns the immutable ordered normalized SpaceType name registry and dense IDs only; the helper allocates no mutable simulation history

unsupported_state:
- EnergyPlus allocation capacity and global integer numSpaceTypes storage beyond the equivalent dense typed registry
- all downstream type-based grouping, gains, reporting, geometry, HVAC, and space heat-balance consumers

inactive_branches:
- when every Zone already has an authored Space and no mixed explicit/blank surface assignment requires a remainder, the helper is not called, so a non-General-only authored registry remains unchanged
- when General already exists from an authored Space or earlier generated default, later default or remainder creation reuses that identity without appending another type

unsupported_active_branches:
- generated whole-zone default creation and General identity alone add no runtime boundary; authored Space and SpaceList definitions are blocked by the parent GetSpaceData contract, while generated remainders are blocked by the separate CreateMissingSpaces contract

not_claimed_branches:
- complete helper parity, source one-based numeric identity, allocation/counter side effects, whitespace-preserving labels, exact diagnostics, downstream type consumers, numerical parity, and conformance
<!-- routine-state-contract:v1 end get_general_space_type_num -->

### CP97 `GetHTSurfaceData` / `CreateMissingSpaces` bounded partition link

EnergyPlus enters `SetupZoneGeometry` immediately after `GetZoneData`, and its
`GetSurfaceData` child calls `CreateMissingSpaces` only after every source
surface family has been read, the fatal input-error gate has passed, adjacent
Zone/Space surfaces have been generated, and base links have been reconciled.
The CP97 Rust composite deliberately maps only the existing typed
`BuildingSurface:Detailed` cross-section: `GetHTSurfaceData` owns bounded raw
Space lookup and same-Zone validation while its full routine remains
`source_mapped`, and state-mapped `CreateMissingSpaces` consumes the resolved,
preclassified assignments for remainder/fallback finalization. The canonical
finalizer state contract is in
[the geometry source map](geometry-source-map.md#createmissingspaces-create_missing_spaces).
The parent `SetupZoneGeometry`, `GetSurfaceData`, and full `GetHTSurfaceData`
routines remain `source_mapped`.

### CP98 `GetVariableAbsorptanceSurfaceList` bounded selection

After `GetBuildingData` returns, EnergyPlus immediately calls
`DataSurfaces::GetVariableAbsorptanceSurfaceList`. Bounded Rust preserves that
relative barrier by calling
`Compiler::build_variable_absorptance_surface_list` after the retained detailed
surface pass has resolved ConstructionIds and only when that pass added no
error. The pass scans retained surfaces in dense SurfaceId order. An `Outdoors`
surface whose construction outside-layer MaterialId owns a typed
`MaterialProperty:VariableAbsorptance` overlay receives one immutable
`VariableAbsorptanceSurfaceBinding`; the same outside-layer use on a
non-Outdoors surface produces a warning and no binding. Rust then scans every
typed construction and every source-effective layer after layer 1 in dense
construction/layer order, producing an independent warning for each overlay
occurrence, without deduplication.

This immutable selection does not weaken the existing all-definition
`UnsupportedSurfaceBoundary` / `RunBlocked` boundary. The full source
`AllHTSurfaceList` and source surface reorder, other surface families,
`UpdateVariableAbsorptances`, exact warning text/punctuation/order/multiplicity
outside the typed arenas, surface absorptance mutation, runtime numerics, and
conformance remain deferred. The canonical contract is in
[the material source map](material-source-map.md#getvariableabsorptancesurfacelist-state-contract).

### CP99 `GetIncidentSolarMultiplier` request-only front end

EnergyPlus calls `GetIncidentSolarMultiplier` immediately after
`DataSurfaces::GetVariableAbsorptanceSurfaceList` and before
`GetScheduledSurfaceGains`. CP99 preserves that relative compiler barrier but
types only the complete public field surface of
`SurfaceProperty:IncidentSolarMultiplier`; the source routine remains
`source_mapped`. A deterministic request arena owns one dense
`SurfaceIncidentSolarMultiplierRequestId` and immutable
`SurfaceIncidentSolarMultiplierRequest` per valid declaration. Each record
retains `declaration_name`, a normalized diagnostic snapshot of the
nonsemantic outer epJSON key, plus the required nonblank normalized
`surface_name`, a finite inclusive-[0,1] `multiplier` defaulting to 1.0, and an
optional `ScheduleId` resolved case-insensitively from a nonblank schedule
name. Missing or blank schedule input retains `None`.

The IDD object has no semantic Name field. The declaration key therefore owns
no name map and creates no surface identity; CP99 claims neither staged-IDF
declaration order nor native-epJSON source order. The window target is also
deliberately unresolved because the current typed `SurfaceId` and
`surface_names` arena covers `BuildingSurface:Detailed`, not
`FenestrationSurface:Detailed`. A second normalized surface target fails
closed before request publication rather than reproducing the source's
order-sensitive overwrite. A missing nonblank schedule also fails closed:
EnergyPlus emits a Severe item-not-found diagnostic but omits
`ErrorsFound = true` in this branch, and Rust does not preserve that null
schedule state.

Full routine behavior requires the deferred global surface arena. EnergyPlus
looks up the target, requires `SurfaceClass::Window` and
`ExternalEnvironment`, inspects the construction's innermost material for
Shade, Blind, Screen, or equivalent-layer shade families, and rejects a
surface with `HasShadeControl`. A successful source record mutates
`Surface::hasIncSolMultiplier` and a surface-indexed
`SurfIncSolMultiplier` slot. Duplicate declarations can overwrite the scalar;
a later nonblank schedule replaces or clears the schedule pointer, while a
later blank schedule leaves the earlier pointer in place. CP99 performs none
of those lookups, validations, mutations, or duplicate reductions. Schedule
sampling, solar and visible transmittance effects, window optics and thermal
behavior, reports, exact diagnostics/order/multiplicity, runtime numerics, and
conformance remain deferred. Every request, including one whose target window
is absent from the typed model, is `UnsupportedSurfaceBoundary`, `RunBlocked`,
and `RuntimeClass::None`; no graph edge, manifest, comparator, or proof
variable is added.

### CP100 `GetScheduledSurfaceGains` bounded first phase

EnergyPlus calls `GetScheduledSurfaceGains` immediately after
`GetIncidentSolarMultiplier`. The source routine first reads
`SurfaceProperty:SolarIncidentInside`, then reads
`ComplexFenestrationProperty:SolarAbsorbedLayers`, and finally calls
`CheckScheduledSurfaceGains` for every Zone when either family is present.
CP100 preserves the parent compiler barrier but types only the complete public
field surface of the first family; `GetScheduledSurfaceGains` remains
`source_mapped`.

Each valid first-phase declaration owns a dense `SurfaceSolarIncidentId` and
an immutable `SurfaceSolarIncident` in
`TypedModel::surface_solar_incidents`. The record stores the required semantic
outer-key name as a `NormalizedName`, resolves the required surface name only
through the typed `BuildingSurface:Detailed` namespace to a `SurfaceId`,
resolves the required construction name to any available typed
`ConstructionId`, and resolves the required inside-solar schedule name to a
`ScheduleId`. The schedule is retained as an identity only; its W/m2 values
are not sampled or constrained by this pass.

The semantic declaration name deliberately has no name map. Case-insensitive
duplicate names therefore remain valid, and two records may share a SurfaceId
or a ConstructionId independently. EnergyPlus does not require the referenced
construction to be the surface's current construction, so Rust preserves such
an intentionally different pair. A second record resolving to the same
SurfaceId/ConstructionId pair fails closed before publication: downstream
`SurfaceScheduledSolarInc` returns the first matching source record, while
CP100 claims neither staged-IDF declaration order nor native-epJSON source
order for this family. Blank or missing required fields, a surface outside the
typed detailed-opaque namespace, an unresolved construction or schedule, and
a repeated resolved pair likewise publish no typed identity.

When representative-surface calculations are active, EnergyPlus input
processing can remove a targeted surface from its representative's constituent
list and reset it to represent itself. CP100 performs no representative or
constituent-list mutation. CP100 itself leaves the complex-fenestration second
phase deferred; CP101 below types its bounded request state but not
fenestration-surface binding or full `CheckScheduledSurfaceGains` mixed-zone
completeness. CP102 adds only a nonblocking monotonic warning when the retained
typed opaque subset already contains both an exact current-construction pair
match and a miss; it writes no model state.
Runtime pair lookup, schedule sampling, inside-face incident-solar replacement,
window-layer absorption, exact diagnostics/order/multiplicity, reports,
numerical behavior, and conformance are unclaimed. Every typed first-phase
definition, including an unused or construction-mismatched pair, is
`UnsupportedSurfaceBoundary`, `RunBlocked`, and `RuntimeClass::None`; no model
graph edge, manifest, comparator, or proof variable is added.

### CP101 `GetScheduledSurfaceGains` bounded second phase

After `SurfaceProperty:SolarIncidentInside`, EnergyPlus reads
`ComplexFenestrationProperty:SolarAbsorbedLayers` into
`FenestrationSolarAbsorbed::{Name, SurfPtr, ConstrPtr, NumOfSched, scheds}`.
CP101 preserves this second-family compiler barrier and types its complete
public field surface as immutable request state; the enclosing
`GetScheduledSurfaceGains` routine remains `source_mapped`.

Each valid declaration owns a dense `FenestrationSolarAbsorbedRequestId` and
an immutable `FenestrationSolarAbsorbedRequest` in
`TypedModel::fenestration_solar_absorbed_requests`. Its required semantic
outer-key name is retained as a `NormalizedName` without a name map. The
required fenestration-surface field is also retained as a nonblank normalized
name, but remains unresolved because the current typed `SurfaceId` arena does
not include `FenestrationSurface:Detailed`. The required construction resolves
only to a typed `Construction:ComplexFenestrationState` `ConstructionId`; an
ordinary, equivalent-layer, or otherwise non-complex construction is rejected.

The construction's `complex_fenestration.optical_layers` length supplies the
bounded solid-layer count. CP101 requires one nonblank, resolved `ScheduleId`
for each solid optical layer in outside-to-inside order, rejects a missing or
blank slot and any layer field present beyond that count, including an explicit
blank or malformed value, and retains the exact ordered vector. Schedule values
are neither sampled nor constrained by a schedule type or numerical range; the
IDD's W/m2 text is a note rather than an input limit. A semantic declaration
name may repeat case-insensitively. The same normalized fenestration target with
a different complex-fenestration ConstructionId remains valid, as does a
different target with the same ConstructionId; only a repeated
target/ConstructionId pair fails closed before publication. EnergyPlus's
`WindowScheduledSolarAbs` returns
the first matching source record, while Rust claims neither staged-IDF
declaration order nor native-epJSON source order for this family.

EnergyPlus derives its scheduled-layer count from the per-object
`getObjectItem` result `NumAlpha - 3` and compares that value with
`TotSolidLayers`. The staged IDF-to-`RawModel` path does not preserve the
original positional extent of trailing blank layer fields, so CP101 does not
claim source `NumAlpha` or trailing-blank positional parity. Rust also resolves
the construction before accessing it instead of reproducing the source's
comment-marked `Construct(ConstrNum)` access before the `ConstrNum == 0` check.
For a failed schedule lookup, Rust diagnoses the actual failing layer field;
the source loop instead indexes the diagnostic field and value with
`NumOfScheduledLayers + 3` for every failure. EnergyPlus can allocate the
per-object schedule array and continue after a layer-count mismatch before its
final `ErrorsFound` termination, whereas any CP101 diagnostic prevents the
transactional request arena from being published.

Fenestration identity and eligibility, surface/current-state construction
pairing, full `CheckScheduledSurfaceGains` completeness,
`WindowScheduledSolarAbs` lookup, schedule sampling, BSDF layer absorption,
transmitted-solar replacement, exact diagnostics/order/multiplicity,
reporting, runtime numerics, and conformance remain deferred. Every request,
including an otherwise unused one, is `UnsupportedSurfaceBoundary`,
`RunBlocked`, and `RuntimeClass::None`; no model graph edge, manifest,
comparator, or proof variable is added. CP102's bounded warning observes only
current-construction pairs on retained typed opaque surfaces; it cannot consume
these unresolved complex-fenestration requests.

### CP102 `CheckScheduledSurfaceGains` monotonic typed-subset warning

`CheckScheduledSurfaceGains` is the diagnostic tail helper inside
`GetScheduledSurfaceGains` in `HeatBalanceManager.cc`. EnergyPlus calls it in
Zone order only when at least one of the two scheduled-gain input families is
present. The helper walks each Zone's `spaceIndexes`, each Space's inclusive
heat-transfer-surface range, and each surface's current Construction. Windows
query `WindowScheduledSolarAbs`; every other surface queries
`SurfaceScheduledSolarInc`. The source warns when that complete set contains
both a scheduled and an unscheduled surface, then emits a continuation for
every unscheduled surface. After the parent returns, the next top-level
`GetHeatBalanceInput` block writes representative-surface assignments to EIO
when representative calculations are enabled.

CP102 state-maps only a diagnostic subset in
`Compiler::check_scheduled_surface_gains_typed_subset`. It runs after final
Space assignments and both CP100/CP101 parsers, and only when the relevant
surface and scheduled-input phases added no error. Zone membership comes from
final `Zone::spaces` plus each retained `BuildingSurface:Detailed` record's
final SpaceId; this is not a claim that source Space surface ranges or lists
have been reproduced. Each retained opaque surface is matched against the
CP100 arena by its exact SurfaceId and its current resolved ConstructionId.
CP101 targets remain unresolved fenestration names and therefore cannot match
this subset.

A Zone receives one nonblocking compiler warning only if its retained typed
opaque subset already contains at least one exact-pair match and at least one
miss. This predicate is monotonic: adding any missing heat-transfer surfaces
cannot make an already mixed set uniformly matched or uniformly unmatched.
The diagnostic names only the known unmatched retained surfaces. An empty,
all-matched, or all-unmatched typed subset is deliberately silent because
omitted surfaces could make the complete source set mixed; a Zone with no
typed surfaces is also silent rather than reproducing the source's
`firstZoneSurface` empty-Zone flag quirk.

This pass writes no model, graph, schedule, surface, or lookup state. Windows,
other fenestration, legacy detailed surfaces, `InternalMass`, generated
opposite surfaces, representative/constituent surfaces, source reordering,
active-construction behavior, and complete Zone/Space heat-transfer-surface
membership remain deferred. Exact warning text, continuation structure,
ordering, punctuation, and multiplicity are unclaimed. The parent
`GetScheduledSurfaceGains` remains `source_mapped`; `SurfaceScheduledSolarInc`
and `WindowScheduledSolarAbs` lookup/consumption remain source-mapped or
unsupported, as do schedule sampling, solar replacement, BSDF absorption,
reporting, and runtime. The warning does not weaken either input family's
all-definition `UnsupportedSurfaceBoundary` / `RunBlocked` boundary and adds
no object, capability, manifest, comparator, proof variable, or conformance
claim.

### `CheckScheduledSurfaceGains` state contract

<!-- routine-state-contract:v1 begin check_scheduled_surface_gains -->
CheckScheduledSurfaceGains

read_state:
- the internal tail helper called by `GetScheduledSurfaceGains` for every EnergyPlus Zone only when either scheduled-gain input family has at least one source record; the enclosing routine remains source-mapped, and its next top-level sibling is the representative-surface EIO assignment block
- only the error-free retained typed `BuildingSurface:Detailed` subset after final Space assignment: deterministic `Zone::spaces`, each retained surface's final SpaceId and current resolved ConstructionId, and the immutable `SurfaceSolarIncident` exact SurfaceId/ConstructionId pairs
- for each retained typed opaque surface, a pair is scheduled only when its current construction exactly matches one typed first-family record; complex-fenestration requests remain unresolved and cannot match this subset

write_state:
- no model state is written; a Zone receives one nonblocking compile warning only when its retained typed opaque subset already contains both at least one exact-pair match and at least one miss
- the bounded diagnostic identifies the Zone and lists only the known retained typed surface names whose current SurfaceId/ConstructionId pair is unscheduled; it does not invent names or outcomes for deferred surfaces
- the warning predicate is monotonic under completion of the full EnergyPlus heat-transfer surface set: once a retained subset contains both a match and a miss, adding omitted surfaces cannot make the full set uniformly scheduled or uniformly unscheduled
- any relevant typed surface-input or scheduled-surface-gain input error suppresses the entire check so diagnostics are not inferred from partial compiler arenas

history_state_ownership:
- the compiler owns no new persistent state for this diagnostic-only pass; it allocates only local match/miss observations and warning text, with no schedule values, surface flags, lookup cache, or runtime history

unsupported_state:
- the complete EnergyPlus heat-transfer surface population: windows and other fenestration, legacy detailed surface families, InternalMass, generated opposite surfaces, source surface reordering, and representative/constituent-surface mutation
- EnergyPlus `WindowScheduledSolarAbs` matching, complex-fenestration current or active construction selection, `SurfaceScheduledSolarInc` and `WindowScheduledSolarAbs` runtime consumers, schedule sampling, solar replacement, BSDF absorption, and reporting

inactive_branches:
- an empty retained subset, an all-matched retained subset, or an all-unmatched retained subset emits no warning because omitted full-domain surfaces could change those classifications; this also avoids claiming the source empty-Zone flag quirk
- a Zone with no typed detailed opaque surfaces emits no warning, and unresolved CP101 fenestration requests cannot create a typed-subset match
- surface or scheduled-input errors suppress the pass rather than emitting warnings from partially published state

unsupported_active_branches:
- the bounded warning is diagnostic-only and nonblocking; it does not weaken the all-definition `UnsupportedSurfaceBoundary` and `RunBlocked` boundary for either scheduled-gain object family
- Zones whose full source surface set is mixed but whose retained typed subset is empty, all matched, or all unmatched intentionally receive no warning until full completeness is available

not_claimed_branches:
- complete Zone/Space heat-transfer surface membership, windows, legacy/InternalMass/generated/representative surfaces, active-construction semantics, full scheduled-gain completeness, or the EnergyPlus empty-Zone behavior
- exact EnergyPlus warning severity text, continuation text, surface text, order, punctuation, or multiplicity; pair-lookup routines, schedule evaluation, runtime numerics, reporting, graph edges, manifests, comparators, proof variables, and conformance
<!-- routine-state-contract:v1 end check_scheduled_surface_gains -->

### CP103 representative-output barrier and thermochromic child projection

After `GetScheduledSurfaceGains` returns, EnergyPlus evaluates the inline
representative-surface assignment output block before calling
`CreateTCConstructions`. This block is output-only but is not safely separable:
when `UseRepresentativeSurfaceCalculations` is true it writes the EIO header
once even when `TotSurfaces` is zero, visits the complete global Surface array
in numeric order, and writes a row only when a surface's
`RepresentativeCalcSurfNum` is not itself. Those identities include the
representative/constituent mutation performed during full surface processing,
including scheduled-gain-driven constituent removal. Rust has neither the
`PerformancePrecisionTradeoffs` control, the complete ordered Surface arena,
the representative/constituent relationship, nor the applicable EIO writer.
CP103 therefore records this exact inline barrier as mapped and deferred and
does not invent a routine-ledger entry for it.

The following EnergyPlus `CreateTCConstructions` pass first counts every state
of every thermochromic master in the pre-existing Construction range, expands
the global construction and nominal-property arrays, then visits the original
master range again. It uses the first state for the master specification
temperature and creates one child for every ordered state, including the first.
Each child is a deep copy with the final retained thermochromic parent layer
replaced by that state's glazing. The bounded name helper follows the shape of
EnergyPlus's custom `{:.0R}` formatter rather than generic Rust decimal
rounding and pins `40 -> 40`, `-999.5 -> -1000`, either signed zero to `0`,
`0.05 -> 5E-002`, and finite-limit guards. Exhaustive fmt 8.0.1 versus Rust
finite-`f64` serialization equivalence remains unclaimed.

CP103 state-maps only an immutable structural projection in
`Compiler::create_thermochromic_construction_projections`. One series record is
emitted for each CP85 master in dense `ConstructionId` order and addresses a
contiguous child range. Children retain dense projection-only identities,
master identity, zero-based input-state index, the EnergyPlus-formatted name,
specification temperature, outside layer, and the cloned effective stack with
only the final master's retained layer slot changed. Earlier thermochromic
parents remain fixed at their already substituted first states. Duplicate
temperatures, glazing references, and generated names are neither sorted,
deduplicated, nor rejected; an unused parent group produces no series, while a
group used by multiple masters produces one independent series per master.

These projections deliberately do not enter `TypedModel::constructions`, the
shared construction name map, object counts, or model-graph edges. EnergyPlus
can synthesize WINDOW5 constructions inside `GetConstructData` before this
later append, so assigning global `ConstructionId`s without that deferred
expansion would assert a false source order. Full `ConstructionProps` deep
copies, `TotConstructs`, nominal R/U and adjustment arrays, generated-name
lookup/collision behavior, master child-index mutation, surface-active
construction switching, nearest-temperature selection, optics, thermal
calculations, daylighting, shading, EIO/reporting, runtime, and conformance
remain deferred. The existing all-definition thermochromic parent blocker is
unchanged, and this derived state adds no object, capability, manifest,
comparator, proof variable, or numerical claim.

### `CreateTCConstructions` state contract

<!-- routine-state-contract:v1 begin create_tc_constructions -->
CreateTCConstructions

read_state:
- EnergyPlus calls this routine immediately after the inline representative-surface EIO assignment block; bounded Rust enters after the CP102 scheduled-gain check and reads only immutable typed-model state, while the intervening output block remains mapped and deferred
- every existing typed Construction is visited once in dense ConstructionId order; only records with CP85 `ConstructionThermochromicMaster` metadata are selected, and that descriptor's parent MaterialId plus zero-based layer index address the final thermochromic parent and its slot in the already first-state-substituted effective stack
- each selected parent resolves to a typed `WindowGlazingThermochromicGroupMaterial` and its nonempty contiguous state-arena slice; every ordered state, including the first, supplies its retained optical-data temperature and resolved glazing MaterialId without sorting, deduplication, or duplicate rejection

write_state:
- one immutable `ConstructionThermochromicSeries` per selected master retains the master ConstructionId, the first state's initial specification temperature, and a contiguous first-child/count descriptor
- one immutable `ConstructionThermochromicChild` per ordered state retains a dense projection-only child ID, master ConstructionId, zero-based state index, source-shaped `{:.0R}` normalized name with the pinned 40, -999.5, signed-zero, 0.05, and finite-limit boundaries, specification temperature, outside layer, and cloned effective layer stack with only the final master's retained layer slot replaced by that state's glazing MaterialId
- output order is master ConstructionId then input state; earlier thermochromic parents in a multiple-parent construction remain fixed at their first states, layer-zero replacement also updates the child outside layer, duplicate temperatures/materials/names are preserved, and a shared group produces a separate series for each master
- both projection arenas are built transactionally and published together only after every selected parent descriptor, state range, effective first-state layer, finite temperature, layer index, and dense child range validates; prior compiler errors suppress the pass rather than publishing projections from partial upstream arenas
- the main Construction arena and name map, ConstructionIds, object counts, material arena, model-graph edges, surfaces, and runtime state are unchanged

history_state_ownership:
- TypedModel owns only immutable thermochromic series and child projection arenas; no master mutation, active-state pointer, temperature history, surface state, optical/thermal cache, or mutable simulation history is allocated

unsupported_state:
- appending deep-copied children to the global Construction arena, assigning source-global ConstructionIds after any WINDOW5-generated constructions, updating `TotConstructs`, and extending construction names, object counts, or construction/material graph edges
- full `ConstructionProps` deep copy, master `TCChildConstrs` allocation, child-to-master global construction links, and nominal R/U, pre-adjusted U, or coefficient-adjustment arrays
- source linear lookup and first-match behavior for generated-name collisions, including collisions between rounded temperatures, different masters, existing constructions, or deferred WINDOW5 children
- active surface-construction mutation, nearest-temperature state selection and history, fenestration binding, window optics and thermal calculations, daylighting, shading, ratings, EIO/SQLite or other reporting, and runtime behavior

inactive_branches:
- with no retained thermochromic master construction, both projection arenas remain empty; an otherwise valid but unused thermochromic group produces no series or child
- reusing one thermochromic group from multiple masters produces an independent series and child range for each master without sharing projection identities
- the first state always produces a child even though its layers equal the master's already first-state-substituted effective stack; duplicate temperatures, glazing references, and formatted names remain distinct children in input order

unsupported_active_branches:
- every typed thermochromic parent remains `UnsupportedSurfaceBoundary` and `RunBlocked`, including an unused parent and one used by one or more projected master constructions; projection materialization never admits partial runtime
- series and child identities are private derived state, not public ConstructionIds or object definitions, and do not change object, unsupported-object, capability, manifest, comparator, proof-variable, or conformance counts

not_claimed_branches:
- complete source ordering relative to WINDOW5-synthesized constructions, source-global child identity and name lookup, exhaustive fmt 8.0.1 versus Rust generated-name equivalence for arbitrary finite `f64` values beyond the pinned boundaries, invalid-upstream partial allocation or assertion behavior, exact `ErrorsFound` side effects, and full deep-copy field parity
- representative-surface assignment output, construction/surface mutation, thermochromic state selection, optics, thermal calculations, daylighting, shading, ratings, EIO/SQLite or other reporting, runtime numerics, exact diagnostics/order/multiplicity, and conformance
<!-- routine-state-contract:v1 end create_tc_constructions -->

### CP104 bounded no-Zone validity diagnostic and CP105 positive construction-use evidence

Immediately after `CreateTCConstructions`, EnergyPlus evaluates the inline
`TotSurfaces > 0 && NumOfZones == 0` condition. Only inside that gate does it
call `CheckValidSimulationObjects`; a false return emits a severe
`GetHeatBalanceInput` diagnostic and sets the caller's `ErrorsFound` flag. The
source helper returns true when raw input contains any one of, in order,
`SolarCollector:FlatPlate:Water`, `Generator:Photovoltaic`,
`Generator:InternalCombustionEngine`, `Generator:CombustionTurbine`,
`Generator:FuelCell`, `Generator:MicroCHP`, `Generator:MicroTurbine`, or
`Generator:WindTurbine`. These are input-object count checks, not field,
reference, or executability checks. The caller next runs
`CheckUsedConstructions`, then terminates fatally if `ErrorsFound` is set.

CP104 invokes
`Compiler::check_valid_simulation_objects_bounded` immediately after the CP103
projection but state-maps only a monotonic positive diagnostic slice. Any
prior compile Error suppresses it. Otherwise, the typed Zone arena must be
empty and raw `Shading:Site:Detailed` or `Shading:Building:Detailed` presence
must positively witness a source Surface. If all eight allowed raw families
are absent, Rust emits one blocking `InvalidSimulationWithoutZones` Error on
object type `GetHeatBalanceInput`, without a name or field and without writing
model state. Either detailed shading family is sufficient; absence of both is
inconclusive and stays silent. Legacy rectangular `Shading:Site` and
`Shading:Building`, Zone shading, heat-transfer/fenestration families, and
generated surfaces are deliberately not used for negative inference. Adding
any omitted surface cannot invalidate an already positive `TotSurfaces`
witness.

Presence of any one allowed family suppresses only this bounded diagnostic.
It does not type, validate, run-enable, or otherwise support that collector or
generator object. Exact full-surface and invalid-input recovery behavior,
source severe/fatal text and sequencing, and the complete parent
`GetHeatBalanceInput` condition remain source-mapped.

The following `CheckUsedConstructions` routine reads `construction_name` from
`Pipe:Indoor`, `Pipe:Outdoor`, `Pipe:Underground`, and
`GroundHeatExchanger:Surface`; `construction_name` from alpha field 4 of
`DaylightingDevice:Tubular`; and `construction_object_name` from
`EnergyManagementSystem:ConstructionIndexVariable`, in that source order. A
resolved reference sets the global Construction's `IsUsed`.
GroundHeatExchanger and EMS references also set `IsUsedCTF` when the resolved
construction is not a window. The source then counts every global construction
still not used and emits either a summary plus the `DisplayExtraWarnings`
instruction or the summary followed by each unused construction name.

CP105 invokes `Compiler::collect_known_construction_use_evidence` after the
CP104 validity check and before the existing fatal barrier, but preserves only
monotonic positive metadata. Every retained typed `BuildingSurface:Detailed`
contributes its resolved ConstructionId. Each nonblank string from the six raw
fields above contributes only when it case-insensitively resolves to a typed
ConstructionId; missing, blank, wrong-typed, and unresolved fields stay silent.
The resulting `TypedModel::known_used_constructions` vector is sorted and
deduplicated. A separate sorted/deduplicated
`known_ctf_used_constructions` vector accepts only GroundHeatExchanger and EMS
references to typed Opaque, including F/C variants, or AirBoundary
constructions. Fenestration, ComplexFenestration, and WindowEquivalentLayer
are the exact excluded window kinds. This vector mirrors only the source
non-window `IsUsedCTF` mark; an AirBoundary ID does not assert that the
construction owns or requires CTF coefficient state. Pipe,
tubular-daylighting, and retained surface evidence never imply CTF use. If any prior compile Error exists,
including one just emitted by CP104, the collector transactionally leaves both
vectors empty.

These vectors are positive evidence sets, not reconstructed mutable source
flags. An absent ID means unknown, never `IsUsed=false` or `IsUsedCTF=false`.
CP105 therefore adds no unused count, construction name, summary/detail
warning, `DisplayExtraWarnings` behavior, CTF/CondFD selection, runtime or
conformance claim. It also adds no ConstructionId, public object, object count,
name-map entry, graph edge, or support for any of the six raw referring
families. Complete post-WINDOW5/post-thermochromic Construction identity and
all other use paths remain deferred.

### CP106 input-completion barrier and solar view-factor initialization map

Immediately after `CheckUsedConstructions`, `GetHeatBalanceInput` checks its
accumulated `ErrorsFound` flag at `HeatBalanceManager.cc` lines 311-313 and
calls `ShowFatalError` before any later initialization when the flag is set.
This is an inline input-completion barrier, not a source routine, so CP106 does
not invent a routine-ledger row for it. A Rust compile containing any Error
eventually returns `CompileResult { model: None, .. }`, which is a coarse
fail-closed boundary. It does not reproduce the exact line-311 short circuit,
the ordering of all diagnostics accumulated before that point, the EnergyPlus
fatal diagnostic text, or fatal/reporting side effects.

On successful input completion, the next parent call is
`HeatBalanceIntRadExchange::InitSolarViewFactors(state)` at
`HeatBalanceManager.cc` line 316. The routine first scans the `ViewFactorInfo`
report option and may write EIO headings, then aligns any
`ZoneProperty:UserViewFactors:BySurfaceName` input before visiting every Solar
enclosure. Its prerequisites include the complete Solar enclosure collection,
each enclosure's ordered Space membership, and every Space's complete
heat-transfer-surface list. Those lists must already reflect AirBoundary-driven
enclosure merging while AirBoundary surfaces themselves are excluded from the
matrix population.

For each enclosure the source constructs Surface pointer and back-pointer
state, preserves global `AllSurfaceListReportOrder`, and gathers Surface area,
azimuth, tilt, and Construction inside solar absorptance. Zero surfaces emit a
severe error and set the local fatal flag. A one-surface enclosure takes its
dedicated no-distribution branch. Larger enclosures consume an aligned user
matrix when supplied or call `CalcApproximateViewFactors`, optionally preserve
the pre-fix matrix for reports, detect InternalMass, and call
`FixViewFactors`. That fixer owns the `N <= 3` warnings and Zone
`EnforcedReciprocity` mutation, iterative reciprocity/completeness repair, the
401st-iteration fallback and its warning/severe paths, and possible fatal
termination. `InitSolarViewFactors` also owns EIO/debug report order and its
final fatal-on-errors side effect.

Rust already contains diagnostic helpers
`energyplus_approximate_view_factors` and
`fix_energyplus_approximate_view_factors`, plus a
`approximate_view_factors_match_energyplus_1zone_eio` evidence test. CP106 does
not promote those isolated calculations or the 1Zone EIO fixture to
`InitSolarViewFactors`: Rust lacks complete Solar enclosure topology, complete
surface-family and Space surface-list state, AirBoundary merging/exclusion,
Surface/back-pointer and global report-order mutation, user-factor alignment,
the zero/one/at-most-three-surface branches and warnings,
`EnforcedReciprocity`, the 401st-iteration/fatal behavior, and the complete EIO
and debug side effects. The routine is therefore `source_mapped` and remains
required for full-domain support.

The source-order tail recorded by CP106 continues through
`ManageInternalHeatGains(state, true)` at line 320 and conditional
`kivaManager.setupKivaInstances(state)` at lines 322-325 when `AnyKiva` is
true. CP107 maps the first call as described below, and CP108 source-maps the
conditional Kiva call. After `GetHeatBalanceInput` returns, the caller's
`DoingSizing` assignment from `doSpaceHeatBalanceSizing` into
`doSpaceHeatBalance` is mapped/deferred by CP109. CP110 source-maps the
following conditional surface-octree initialization. CP111 then state-maps a
bounded retained detailed-opaque slice of the complete Surface
computed-geometry loop; the one-time input-flag clear is next.

### CP107 bounded internal-gain input map

`ManageInternalHeatGains(state, true)` is the next parent call after solar
view-factor initialization. In `InternalHeatGains.cc`, the manager's one-time
input branch calls `GetInternalHeatGainsInput`; because this parent call passes
`InitOnly=true`, the source returns after input acquisition. Other manager
entries may bypass input once the persistent flag is clear or continue into
recurring radiation, convection, latent, contaminant, reporting, and
daylighting work. CP107 therefore records `ManageInternalHeatGains` as required
but only `source_mapped`.

Within `GetInternalHeatGainsInput`, the source scans People first, then Lights,
ElectricEquipment, GasEquipment, HotWaterEquipment, SteamEquipment,
OtherEquipment, ElectricEquipment:ITE:AirCooled,
ZoneBaseboard:OutdoorTemperatureControlled, and
ZoneContaminantSourceAndSink:CarbonDioxide before later setup and registration.
The bounded Rust wrapper preserves only two monotonic family points from that
sequence: direct-Zone People first and direct-Zone OtherEquipment second. It
returns without invoking either existing parser when any Error was already
present on entry. Once admitted, it deliberately performs both scans; an Error
added by People does not suppress OtherEquipment, matching the source's
accumulating input-error behavior within this bounded slice.

The mapped state is exactly the existing `TypedModel::people` plus
`people_names` and `TypedModel::other_equipment` plus
`other_equipment_names` declaration state. No new model identity, public-object
count, name map, graph edge, capability, manifest, proof variable, runtime
history, or conformance evidence is introduced. Direct typed Zone resolution
is the complete target boundary here. ZoneList, Space, and SpaceList expansion,
source-generated instances, declaration order parity, Space and Zone occupant
totals, `setupIHGZonesAndSpaces`, `setDesignLevel`, floor-area/person-derived
design levels, schedule-minimum validation, nominal min/max arrays, all omitted
families, reporting/output variables/EIO/meters, contaminant coupling, EMS,
component-load reporting, runtime gain updates, persistent input flags,
re-entry behavior, and exact global compiler short-circuit behavior after an
Error created inside this pass remain unclaimed.

### `GetInternalHeatGainsInput` state contract

<!-- routine-state-contract:v1 begin get_internal_heat_gains_input -->
GetInternalHeatGainsInput

read_state:
- the line-320 `ManageInternalHeatGains(state, true)` parent call after `InitSolarViewFactors`; bounded Rust invokes `Compiler::parse_bounded_internal_heat_gains_input` after the CP105 construction-use collector, returns before either family when an Error already exists on entry, otherwise scans People before OtherEquipment, and continues into OtherEquipment even when People adds an Error
- every `People` definition whose required `zone_or_zonelist_or_space_or_spacelist_name` resolves directly to an existing typed Zone; an optional number-of-people schedule resolves through the existing typed schedule namespace, the calculation method defaults to People and accepts People, People/Area or PeoplePerArea, and Area/Person or AreaPerPerson, and the three finite sizing scalars default to zero and must be nonnegative
- every `OtherEquipment` definition whose same target field resolves directly to an existing typed Zone; an optional schedule resolves through the existing typed schedule namespace, fuel type defaults to normalized None, design-level method defaults to EquipmentLevel and accepts the bounded Watts/Area, Power/Area, WattsPerZoneFloorArea, Watts/Person, Power/Person, or WattsPerPerson aliases, and all existing finite numeric/default/range validation remains in force
- OtherEquipment retains finite design level and carbon-dioxide generation rate with zero defaults, nonnegative power-per-floor-area and power-per-person with zero defaults, and latent, radiant, and lost fractions each in [0,1] with zero defaults; a sum above one emits the existing blocking typed diagnostic without suppressing later definitions

write_state:
- the existing deterministic People arena and normalized people name map, with dense InternalGainId, normalized name, direct ZoneId, optional ScheduleId, calculation-method enum, number of people, people per floor area, and floor area per person
- the existing deterministic OtherEquipment arena and normalized other-equipment name map, with dense InternalGainId, normalized name and fuel type, direct ZoneId, optional ScheduleId, design-level-method enum, design level, power-per-floor-area, power-per-person, latent/radiant/lost fractions, and carbon-dioxide generation rate
- family-level diagnostic projection ordered People before OtherEquipment for an error-free entry; validation remains definition-local, so a malformed People object does not prevent the bounded OtherEquipment family from being scanned in the same wrapper invocation

history_state_ownership:
- TypedModel owns immutable bounded People and OtherEquipment declaration state already used by downstream Rust paths; CP107 adds no mutable EnergyPlus internal-gain history, one-time input flag, runtime rate, schedule sample, meter, output, or EMS state

unsupported_state:
- Lights, ElectricEquipment, GasEquipment, HotWaterEquipment, SteamEquipment, ElectricEquipment:ITE:AirCooled, ZoneBaseboard:OutdoorTemperatureControlled, and ZoneContaminantSourceAndSink:CarbonDioxide source-family input and all of their dependent state
- ZoneList, Space, and SpaceList target expansion; source-generated gain instances and declaration order; Space and Zone occupant totals; `setupIHGZonesAndSpaces`, `setDesignLevel`, floor-area/person-derived design-level materialization, schedule-minimum checks, nominal min/max gain arrays, and Space heat-balance topology
- ManageInternalHeatGains runtime branches and recurring summation/update behavior, daylighting order beyond the mapped parent call, reporting variables, EIO, meters, contaminant coupling, EMS actuators/internal variables, component-load reporting, and all numerical runtime state

inactive_branches:
- any compile Error present on wrapper entry suppresses both bounded family scans and publishes no additional People or OtherEquipment state from this pass
- when neither bounded family is present, the wrapper is an empty no-op and adds no identity, count, capability, runtime state, or diagnostic
- a missing optional schedule reference retains no ScheduleId; absent optional/defaulted numeric, fuel, and method fields retain their existing typed defaults without creating schedule or derived design-level state

unsupported_active_branches:
- a People Error created after entry does not suppress the OtherEquipment scan, but no claim follows for exact source recovery state, complete diagnostic ordering, or arbitrary compiler passes after this bounded wrapper
- a valid direct-Zone People or OtherEquipment definition retains its pre-existing typed support and downstream bounded uses only; CP107 does not promote full internal-gain runtime execution or any omitted family

not_claimed_branches:
- complete `ManageInternalHeatGains` or `GetInternalHeatGainsInput` parity, source `GetInputFlag` persistence and re-entry behavior, `InitOnly` control beyond this compile-time projection, exact object/declaration ordering within either family, partial-allocation recovery, exact diagnostics/text/severity/order/multiplicity, or exact global compiler short-circuit behavior after Errors created inside the pass
- ZoneList/Space/SpaceList expansion, generated instances, Space/Zone occupant totals, every omitted internal-gain family, setup and design-level derivation, schedule minima, reporting, meters, EMS, runtime gains, numerical parity, capability expansion, object-count or graph expansion, and conformance
<!-- routine-state-contract:v1 end get_internal_heat_gains_input -->

### CP108 conditional Kiva setup map

At `HeatBalanceManager.cc` lines 322-325, immediately after
`ManageInternalHeatGains(state, true)`, EnergyPlus tests
`state.dataHeatBal->AnyKiva` and conditionally calls
`state.dataSurfaceGeometry->kivaManager.setupKivaInstances(state)`. The setup
routine returns its local `ErrorsFound` boolean, but this caller does not store,
test, or otherwise consume the result. CP108 records only that guard, call, and
ignored return boundary. `setupKivaInstances` is `source_mapped`, is not
required for the full domain, and has no Rust implementation target.

The source routine installs Kiva diagnostic callbacks, conditionally acquires
Zone air setpoints, reads weather state, and consumes the complete Surface,
Construction, regular-material, Zone, and Foundation inputs. It selects Kiva
floor surfaces and associated walls, requires
`SurfaceProperty:ExposedFoundationPerimeter`, walks detailed vertices and wall
heights/construction groupings, materializes `Foundation:Kiva` and
`Foundation:Kiva:Settings`-derived layers/blocks and deep-ground state, and
owns Kiva instances, floor/wall aggregators, surface maps, mesh state, and EIO
foundation rows. Those are dependency observations only, not mapped Rust
state.

CP108 claims no `Foundation:Kiva` or settings/material input support, complete
Foundation outside-boundary topology, exposed-perimeter or coplanar geometry,
Kiva instance/aggregator/surface-map ownership, ground/weather/site boundary
processing, inside/outside convection-algorithm coupling, Zone setpoint or
thermal-comfort interaction, mesh/solver initialization, EIO/output variables,
EMS behavior, diagnostic callback text/order/multiplicity, early-return or
returned-error parity, subsequent initialization/calculation/reporting, runtime
numerics, or conformance. Existing typed
`OutsideBoundaryCondition::Foundation` state and ground-like Rust runtime
handling are not Kiva instance setup and do not promote this routine; no
support-gate repair is mixed into CP108.

On return from `GetHeatBalanceInput`, `ManageHeatBalance` next conditionally
copies `doSpaceHeatBalanceSizing` into `doSpaceHeatBalance` when `DoingSizing`
is true. CP109 maps that inline sizing override below without inventing a
routine row. CP110 source-maps the following conditional surface-octree
initialization, CP111 state-maps the following bounded computed-geometry
slice, CP112 maps the one-time input-flag clear, and CP113 source-maps the
following generic EMS dispatch. CP114 makes the unconditional
`InitHeatBalance` call a required source mapping, and CP115 maps the
post-initialization EMS caller by reusing the generic row. CP116 expands the
existing required `routine.manage_surface_heat_balance` row for the following
unconditional parent call and its complete driver order. CP117 maps the
parent's inline first-time `Initializing Surfaces` display guard, and CP118
adds the following unconditional `InitSurfaceHeatBalance(state)` as a required
source mapping. CP119 maps the following first-time outside-balance display,
and CP120 adds the unconditional `CalcHeatBalanceOutsideSurf(state)` required
source mapping. CP121 maps the following first-time inside-balance display,
and CP122 adds the unconditional `CalcHeatBalanceInsideSurf(state)` as a
required source mapping. CP123 maps the following first-time air-balance
display, and CP124 maps the unconditional `ManageAirHeatBalance(state)` call
by reusing its existing required routine. CP125 adds the following final
Surface heat-balance update as a required source mapping. CP126 adds the
guarded thermal-history update as a required source mapping. CP127 adds the
inline CondFD moisture-history helper as a non-required source mapping. CP128
adds the unconditional ThermalComfort call as a non-required source mapping.
CP129 adds Surface reporting as a required source mapping. CP130 adds the
sizing-only component-load gathering call as a non-required source mapping.
CP131 adds the unconditional thermal-resilience calculation as a non-required
source mapping. CP132 adds its independently guarded summary report as a
non-required source mapping. CP133 adds the independently guarded CO2 summary
as a non-required source mapping. CP134 adds the independently guarded visual
summary as a non-required source mapping. CP135 maps the parent-tail
first-time-flag clear without a synthetic routine or count change. CP136 maps
the `EndZoneTimestepBeforeZoneReporting` EMS calling point by reusing the
existing generic routine without a row or count change. CP137 adds
`RecKeepHeatBalance` as a required source mapping; CP138 adds
`ReportHeatBalance` as a required source mapping; CP139 maps the
`EndZoneTimestepAfterZoneReporting` EMS calling point by reusing the generic
routine; CP140 adds non-required `UpdateEMSTrendVariables`; CP141 adds the
non-required plugin-value update; CP142 adds required
`CheckWarmupConvergence`; CP143 maps the inner day-counter resets; CP144 maps
the in-branch post-warmup EMS call; CP145 adds required
`ReportWarmupConvergence`; CP146 adds required
`SetPreConstructionInputParameters`; CP147 adds required
`GetSiteAtmosphereData`; CP148 adds required `AllocateZoneHeatBalArrays`;
CP149 adds required `AllocateHeatBalArrays`; CP150 adds required `UpdateWindowFaceTempsNonBSDFWin`; CP151 adds non-required `OpenShadingFile`; CP152 adds non-required `SetStormWindowControl`; CP153 adds required `InitConductionTransferFunctions`; CP154 adds non-required `GatherForPredefinedReport`; CP155 adds required `AllocateSurfaceHeatBalArrays`; CP156 adds required `InitThermalAndFluxHistories`; CP157 adds non-required `EvalOutsideMovableInsulation`; CP158 adds non-required `EvalInsideMovableInsulation`; CP159 adds required `InitSolarHeatGains`; CP160 adds required `InitIntSolarDistribution`; CP161 adds required `ComputeIntThermalAbsorpFactors`; CP162 adds required `ComputeIntSWAbsorpFactors`; CP163 adds required `ComputeDifSolExcZonesWIZWindows`; CP164 next in source-definition order maps `InitEMSControlledSurfaceProperties`.

### CP109 inline sizing Space heat-balance mode map

After the once-only `GetHeatBalanceInput(state)` call returns,
`ManageHeatBalance` lines 169-171 test `state.dataGlobal->DoingSizing`. Only
when that flag is true does the caller copy
`state.dataHeatBal->doSpaceHeatBalanceSizing` into the mutable active
`state.dataHeatBal->doSpaceHeatBalance` mode. CP109 records this exact
conditional assignment as an inline mapped/deferred barrier. It creates no
synthetic routine-ledger row, Rust helper, model field, or executable claim.

The branch is nested inside the outer
`ManageHeatBalanceGetInputFlag` one-time-input block. That outer flag is still
true during the sizing assignment, the following conditional surface-octree
setup, and the complete Surface `set_computed_geometry` loop; EnergyPlus clears
it only later at line 186. CP109 does not map that clear, persistence across
calls, or re-entry behavior. CP110 maps the conditional surface-octree call
below, CP111 maps the bounded computed-geometry slice after it, and CP112 maps
the flag clear below without adding persistent Rust state.

`doSpaceHeatBalanceSizing` originates in `GetProjectControlData`, where
`ZoneAirHeatBalanceAlgorithm` alpha field 2 selects Space heat balance during
sizing and retains its false default when blank or absent. This origin is a
source dependency only: CP109 does not add or widen object typing, defaults,
Yes/No parsing, invalid-input recovery, diagnostics, or EIO reporting. The
separate `SimulationManager` post-sizing assignment that later copies
`doSpaceHeatBalanceSimulation` into `doSpaceHeatBalance` is not this checkpoint
and receives no claim here.

Sizing-environment lifecycle, mutable active-mode ownership and consumption,
outer-flag clearing/re-entry, Space heat-balance arrays and allocation, Zone or
Space loads, HVAC sizing and results, the following surface octree and computed
geometry state, EMS calling points, runtime heat-balance numerics, reporting,
and conformance all remain deferred.

### CP110 conditional Surface octree initialization map

Inside the still-active once-only `ManageHeatBalanceGetInputFlag` block,
`ManageHeatBalance` lines 173-180 first requires the complete source
`TotSurfaces` count to be at least `Dayltg::octreeCrossover`, whose EnergyPlus
26.1 value is exactly 100. It then requires the raw input processor count for
`Daylighting:Controls` to be greater than zero. Only when both guards pass does
the caller provide the complete mutable `Surface` array to
`surfaceOctree.init`. CP110 records this double guard and the source octree
routine as `source_mapped`, not required for the full domain, with no Rust
implementation target.

`SurfaceOctreeCube::init` clears its current root surface list and retains live
Surface pointers only for entries having at least three vertices and
`IsTransparent == false`. If none survive, lower corner, upper corner, center,
width, and radius state are all zeroed. Otherwise it finds componentwise
minimum and maximum vertices, centers that bounding box, and expands it to a
uniform cube whose side length is the largest axis span. A node branches only
when it holds more than 10 surfaces and its depth is below the maximum 255.
Each surface descends only when its complete bounding box fits one child;
boundary-spanning surfaces stay in the parent, and occupied children recurse.

CP110 does not claim complete Surface counts, global order, omitted families,
or generated surfaces; `Daylighting:Controls` typing or semantics; transparent
surface schedules or EMS/plugin mutation of transparency; octree ownership,
clear/rebuild timing, pointer/reference validity, or repeated calls; traversal,
line/cube or ray/surface intersections, obstruction queries, or
`PierceSurface`; complete computed geometry outside the following bounded
CP111 slice; daylighting,
shading, reflection, or solar algorithms; runtime numerical behavior,
performance/scaling, or conformance. Existing daylighting run-blocking and
typed opaque detailed surfaces do not promote the complete guards or octree,
and CP110 adds no object support, identity, graph edge, capability, runtime
admission, manifest, comparator, or proof variable. CP111 state-maps the next
bounded computed-geometry slice below, and CP112 maps the line-186 flag clear.

### CP111 bounded per-Surface computed geometry map

Immediately after the conditional octree block, `ManageHeatBalance` lines
182-184 visits the complete mutable source Surface array in its current order
and calls `SurfaceData::set_computed_geometry` for every entry. The source
method skips surfaces with fewer than three vertices; for every other entry it
sets `shapeCat`, then the Newell `plane`, then `surface2d` in that order.
EnergyPlus clears `ManageHeatBalanceGetInputFlag` only after this loop at line
186. CP111 state-maps the bounded computation, and CP112 maps that following
inline lifecycle boundary below.

Bounded Rust invokes
`Compiler::set_bounded_surface_computed_geometry` after the CP107 internal-gain
input slice, with the intervening CP108 Kiva, CP109 sizing-mode, and CP110
octree checkpoints remaining mapped or deferred. Any compile Error already
present on entry suppresses the entire pass, matching the source fact that the
earlier `GetHeatBalanceInput` fatal barrier prevents this caller loop from
being reached. Otherwise each retained `BuildingSurface:Detailed` is handled
independently and receives derived state only for a finite, coplanar,
nondegenerate three-vertex Triangle or a four-vertex Rectangle that also
passes the conservative source predicate. An excluded surface remains
`computed_geometry = None` without a new diagnostic and does not suppress
eligible later surfaces.

The input vertices are the already canonicalized world vertices stored on the
typed Surface. `SurfaceGeometry.cc::GetVertices` world-coordinate assignment,
`ProcessSurfaceVertices` shape assignment, and
`SurfaceGeometry.cc::isRectangle` are source dependencies, not separately
promoted routines. Rust preserves the source diagonal-difference threshold
`abs(d1 - d2) < 0.020` and inclusive adjacent-unit-edge test
`abs(dot) <= cos(89 degrees)`, while additionally requiring every wrap edge to
have positive finite length. It requires a finite nonzero Newell normal, a
finite nonzero projected signed area, and conservative coplanarity at distance
`1.0e-9 * max(1, max(abs(world coordinate)))`. These extra rejection gates
avoid publishing source-looking state without the omitted upstream geometry
validation and recovery lifecycle.

For each admitted surface, the Newell coefficients follow source vertex and
wrap order, and `d` uses the accumulated vertex center divided by the vertex
count; the plane is not normalized. The projection removes the largest
absolute normal component, with X winning any tie that includes X and Y
winning the remaining Y/Z tie. X projects `(y,z)`, Y projects `(x,z)`, and Z
projects `(x,y)`. Bounds span the projected input vertices. A negative
twice-signed area reverses only projected vertices 2 through N, retaining the
first vertex; wrap edges are then formed from that stored order. Triangles
retain zero rectangle side squares, while Rectangles store the squared lengths
of projected edges 1 and 4 in the source-shaped side-1 and side-3 fields.

The complete result attaches to `Surface::computed_geometry` as
`Option<SurfaceComputedGeometry>`. It retains `SurfaceShapeCategory`, the
four-coefficient `plane`, `SurfaceProjectionAxis`, projected vertices/bounds
and wrap edges as `SurfaceProjectedPoint`, and
`rectangle_side_1_squared_m2`/`rectangle_side_3_squared_m2`. This is derived
Surface state only: no object identity, name map, public object count, model
graph edge, capability, support-gate admission, runtime consumer, comparator,
proof variable, numerical-conformance claim, or external result is added.

Complete Surface array identity/order and all omitted or generated families;
fenestration, shading, InternalMass, and other no-vertex surfaces; source
Convex/Nonconvex category and `IsConvex`; nonrectangular quadrilaterals,
polygons, slab construction, and the `n >= 20` path; full `GetVertices`,
`CheckConvexity`, collinear/coincident deletion, planarity diagnostics,
`ProcessSurfaceVertices`, and `isRectangle` recovery semantics; exact source
partial mutation/default state; octree, `PierceSurface`, intersection and all
other consumers; runtime numerics, performance, reporting, and conformance
remain deferred.

### `set_computed_geometry` state contract

<!-- routine-state-contract:v1 begin set_computed_geometry -->
set_computed_geometry

read_state:
- the complete source loop at `ManageHeatBalance` lines 182-184, after sizing-mode and octree work but before the line-186 one-time flag clear; bounded Rust invokes `Compiler::set_bounded_surface_computed_geometry` after the CP107 executable slice while CP108-CP110 remain mapped or deferred, and returns without mutation when any compile Error already exists
- every retained typed `BuildingSurface:Detailed` Surface in dense typed order, reading only its already canonicalized world-coordinate `Point3` vertices; source `SurfaceGeometry.cc::GetVertices` world-coordinate assignment plus `ProcessSurfaceVertices` shape assignment and `isRectangle` are dependencies rather than promoted routines
- a positive-only candidate gate accepting exactly three or four vertices, all finite; a finite nonzero source-ordered Newell normal and finite plane; finite nonzero projected twice-signed area; and conservative coplanarity whose point-to-plane distance is at most `1.0e-9 * max(1, max(abs(world coordinate)))`
- for four-vertex candidates only, positive finite length for every wrap edge, source diagonal lengths 1-3 and 2-4 differing by strictly less than 0.020 m, and the absolute dot product of normalized source edges 3-2 and 2-1 no greater than `cos(89 degrees)`; three-vertex candidates map directly to `SurfaceShapeCategory::Triangular`, while admitted four-vertex candidates map to `Rectangular`
- source-ordered Newell accumulation without normal normalization; projection-axis selection from the largest absolute plane-normal component with X-over-all and then Y-over-Z tie precedence; X-to-(y,z), Y-to-(x,z), or Z-to-(x,y) projection; projected bounds; strict negative-area reversal of vertices 2 through N only; then wrap-edge construction and Rectangle-only projected edge-1/edge-4 squared lengths

write_state:
- each admitted Surface receives `Some(SurfaceComputedGeometry)` containing its `SurfaceShapeCategory`, source-shaped four-element plane, `SurfaceProjectionAxis`, ordered `projected_vertices`, `projected_lower_bound`, `projected_upper_bound`, ordered `projected_edges`, and `rectangle_side_1_squared_m2` plus `rectangle_side_3_squared_m2`
- a negative projected signed area reverses only the copied projected tail before edges are written and never mutates the source 3D vertices or plane orientation; a Triangle stores exact zero for both rectangle side-square fields
- every excluded retained Surface receives `computed_geometry = None` without a diagnostic, identity, name-map entry, object-count increment, graph edge, support/capability change, manifest, comparator, proof variable, or runtime state; exclusion of one Surface does not suppress eligible later entries

history_state_ownership:
- `TypedModel::surfaces` owns immutable compile-time derived `Surface::computed_geometry` attachments only; CP111 allocates no mutable source input flag, octree pointer, intersection cache, Surface history, timestep state, reporting state, or runtime geometry consumer

unsupported_state:
- the complete EnergyPlus Surface array and its source order, all generated/opposite surfaces, FenestrationSurface:Detailed, shading, InternalMass, doors/windows, overhangs/fins, and every other omitted Surface family or source shape
- source `Shape`, `IsConvex`, Convex and Nonconvex categories, nonrectangular quadrilaterals, polygons, `Surface2DSlab`, unique slab-Y state, inverse edge slopes, and the convex `n >= 20` slab switch
- complete `GetVertices` canonicalization and recovery, `CheckConvexity`, coincident/collinear deletion, planarity checking and diagnostics, `ProcessSurfaceVertices`, full `isRectangle` lifecycle, source partial writes/default objects, one-time flag ownership and clearing, octree/PierceSurface/intersection consumers, runtime, reporting, performance, and numerical conformance

inactive_branches:
- any compile Error present on pass entry preserves the pre-pass computed-geometry state for every retained Surface and emits no CP111 diagnostic
- an empty Surface arena is a no-op; a retained Surface with a vertex count other than three or four, any nonfinite or overflow-derived value, a zero Newell normal or signed projected area, conservative noncoplanarity, or a rejected four-vertex rectangle remains `None` and does not affect another Surface
- strict source boundaries are retained for rectangle diagonal difference below 0.020 m, the inclusive cosine threshold, X-then-Y projection ties, and signed-area reversal only below zero; no tolerance is added to those branch comparisons beyond the separately declared conservative coplanarity gate

unsupported_active_branches:
- valid computed geometry remains metadata only and grants no new object support, runtime admission, daylighting, shading, solar, octree, ray-intersection, or heat-balance behavior
- a four-vertex input that source local `isRectangle` could accept only through zero-vector normalization is deliberately rejected, as are finite source inputs whose derived state overflows or whose missing upstream validation cannot be conservatively established
- typed Surface order is preserved for the bounded pass, but this does not claim complete global Surface count/order, generated-family placement, source reordering, or exact loop side effects outside the retained arena

not_claimed_branches:
- complete `SurfaceData::set_computed_geometry`, `computed_shapeCat`, `computed_plane`, `computed_surface2d`, `Surface2D` construction, `GetVertices`, `ProcessSurfaceVertices`, `CheckConvexity`, or `isRectangle` parity; source invalid-input recovery, partial mutation/default retention, exact diagnostic text/severity/order/multiplicity, or one-time flag clearing and re-entry
- Convex/Nonconvex and slab state, complete Surface families/order, octree/PierceSurface and every computed-geometry consumer, exact cross-language floating-point behavior, runtime numerical behavior or performance, output/report serialization, capability/support expansion, conformance cases, or numerical conformance
<!-- routine-state-contract:v1 end set_computed_geometry -->

### CP112 inline one-time heat-balance input-flag clear map

The outer input block in `ManageHeatBalance` begins only while
`state.dataHeatBalMgr->ManageHeatBalanceGetInputFlag` is true. The owning
`HeatBalanceMgrData` member defaults to true, and its `clear_state()` reset also
restores true. That value remains true through `GetHeatBalanceInput`, the CP109
sizing-mode branch, the CP110 conditional octree setup, and the CP111 complete
Surface computed-geometry loop. Only after those steps successfully reach
`HeatBalanceManager.cc` line 186 does EnergyPlus assign the flag false and exit
the block. Later `ManageHeatBalance` calls with the same state therefore skip
that entire input tail.

CP112 maps and defers only this inline assignment and source-order lifecycle
boundary. It intentionally adds no synthetic routine-ledger row, Rust helper,
model field, capability, support-gate admission, manifest, comparator, proof
variable, result, or conformance claim. The Rust compiler's ordinary single
execution and its final `model = None` failure result are not persistent
per-state input-flag ownership or re-entry parity.

The source's exact flag ownership, construction default, `clear_state()` reset,
subsequent-call suppression, and successful-path position are recorded here as
dependencies, not executable Rust state. If an earlier EnergyPlus fatal or
abnormal path prevents control from reaching line 186, this assignment does
not occur; CP112 does not claim exact fatal/exception behavior, partially
completed side effects, diagnostic order/text/multiplicity, or recovery.
Sizing and simulation-environment transitions, repeated calls and explicit
state resets, the exact side effects and ordering of the skipped input tail,
the following EMS calling point, `InitHeatBalance`, timestep/warmup/runtime
behavior, reporting, numerical parity, performance, and conformance all remain
deferred. CP113 source-maps the unconditional generic `ManageEMS` call
immediately after this once-only block; the following `InitHeatBalance` call
is the next CP114 checkpoint.

### CP113 pre-`InitHeatBalance` EMS calling-point map

After the CP112 once-only input block has closed, `ManageHeatBalance` line 189
declares the caller-owned `anyRan` boolean. Lines 191-194 then
unconditionally call
`EMSManager::ManageEMS(state,
EMSCallFrom::BeginZoneTimestepBeforeInitHeatBalance, anyRan,
ObjexxFCL::Optional_int_const())`. The fourth argument is an explicitly absent
optional program-manager index. This call occurs on every caller path that
reaches this point, whether the once-only input block ran on that invocation or
was skipped, and immediately precedes the unconditional `InitHeatBalance` call
at line 198.

CP113 adds one canonical generic `routine.manage_ems` source mapping to
`src/EnergyPlus/EMSManager.cc::ManageEMS`. It is not required for the full
domain and has no Rust target, project-contract routine, executable wrapper,
model or runtime state, capability, support-gate admission, manifest,
comparator, proof variable, result, or conformance promotion. The routine body
first writes its caller-provided run flag false, may return when the model owns
no EMS, and otherwise performs calling-point-dependent initialization,
callback/plugin dispatch, program-manager selection, Erl evaluation, actuator
application, and reporting; those are dependency observations only.

Complete EMS input and setup; the runtime language, program-call managers,
programs, sensors, actuators, internal variables, and trends; Erl evaluation,
plugins, external interfaces, and registered callback dispatch; calling-point
gating and the optional program-manager-index branch; `anyRan`,
`anyProgramRan`, and other run-flag ownership or values; actuator/reporting
side effects; diagnostic text, severity, order, multiplicity, and recovery;
exact execution order inside `ManageEMS`; other EMS calling points; repeated
calls, sizing or simulation environments, timestep and warmup lifecycle,
runtime numerics and performance, output behavior, and conformance all remain
deferred. CP114 source-maps the following unconditional
`InitHeatBalance(state)` call at `HeatBalanceManager.cc` line 198, and CP115
maps the post-initialization EMS caller by reusing this same generic
`routine.manage_ems` row. CP116 maps the following `ManageSurfaceHeatBalance`
caller and source parent body, and CP117 maps its first-time initialization
display guard. CP118 maps the unconditional `InitSurfaceHeatBalance` call and
source routine. CP119 maps the following first-time outside-balance display,
and CP120 adds the unconditional outside-balance call as a required routine.
CP121 maps the following first-time inside-balance display, and CP122 adds the
unconditional inside-balance call as a required routine. CP123 maps the
following first-time air-balance display, and CP124 maps the unconditional
Air-manager call by reusing its existing required routine. CP125 adds the
final Surface heat-balance update as a required source mapping. CP126 adds the
guarded thermal-history update as a required source mapping. CP127 adds the
inline CondFD moisture-history helper as a non-required source mapping. CP128
adds the unconditional ThermalComfort call as a non-required source mapping.
CP129 adds Surface reporting as a required source mapping. CP130 adds the
sizing-only component-load gathering call as a non-required source mapping.
CP131 adds the unconditional thermal-resilience calculation as a non-required
source mapping. CP132 adds its independently guarded summary report as a
non-required source mapping. CP133 adds the independently guarded CO2 summary
as a non-required source mapping. CP134 adds the independently guarded visual
summary as a non-required source mapping. CP135 maps the parent-tail
first-time-flag clear without a synthetic routine or count change. CP136 maps
the `EndZoneTimestepBeforeZoneReporting` EMS calling point by reusing the
existing generic routine without a row or count change. CP137 adds
`RecKeepHeatBalance` as a required source mapping; CP138 adds
`ReportHeatBalance` as a required source mapping; CP139 maps the
`EndZoneTimestepAfterZoneReporting` EMS calling point by reusing the generic
routine; CP140 adds non-required `UpdateEMSTrendVariables`; CP141 adds the
non-required plugin-value update; CP142 adds required
`CheckWarmupConvergence`; CP143 maps the inner day-counter resets; CP144 maps
the in-branch post-warmup EMS call; CP145 adds required
`ReportWarmupConvergence`; CP146 adds required
`SetPreConstructionInputParameters`; CP147 adds required
`GetSiteAtmosphereData`; CP148 adds required `AllocateZoneHeatBalArrays`;
CP149 adds required `AllocateHeatBalArrays`; CP150 adds required `UpdateWindowFaceTempsNonBSDFWin`; CP151 adds non-required `OpenShadingFile`; CP152 adds non-required `SetStormWindowControl`; CP153 adds required `InitConductionTransferFunctions`; CP154 adds non-required `GatherForPredefinedReport`; CP155 adds required `AllocateSurfaceHeatBalArrays`; CP156 adds required `InitThermalAndFluxHistories`; CP157 adds non-required `EvalOutsideMovableInsulation`; CP158 adds non-required `EvalInsideMovableInsulation`; CP159 adds required `InitSolarHeatGains`; CP160 adds required `InitIntSolarDistribution`; CP161 adds required `ComputeIntThermalAbsorpFactors`; CP162 adds required `ComputeIntSWAbsorpFactors`; CP163 adds required `ComputeDifSolExcZonesWIZWindows`; CP164 next in source-definition order maps `InitEMSControlledSurfaceProperties`.

### CP114 `InitHeatBalance` source map

`ManageHeatBalance` calls `InitHeatBalance(state)` unconditionally at line
198, after the CP113
`BeginZoneTimestepBeforeInitHeatBalance` EMS dispatch and immediately before
the second `ManageEMS` call at lines 199-200. The implementation at
`HeatBalanceManager.cc` lines 2594-2821 is the source's flag-driven
heat-balance initialization driver. CP114 records it as `source_mapped` and
required for the full heat-balance domain; it adds no new Rust implementation
or mapped state.

On `BeginSimFlag`, the source first allocates heat-balance arrays, conditionally
initializes CTF/EMPD response factors, then initializes surface-property view
factors, equivalent-layer and detailed-window optics, daylighting devices, and
solar/shadowing state in that order. `BeginEnvrnFlag` resets prior-day load
and temperature extrema, warmup differences and report counters, and complete
window face/effective inside temperatures. Every EMS-active call then
initializes EMS-controlled constructions and surface properties.

Storm-window state and active constructions follow their daily `ChangeSet`
lifecycle. Simulation-start reporting may open the exterior-shading file;
begin-day and detailed-timestep controls select solar recalculation and
sunlit-fraction reporting. The tail updates Zone outdoor dry-bulb, wet-bulb,
wind speed and direction, applies linked outdoor-air-node schedules and
values, then applies EMS overrides. A final simulation-start loop defaults
non-BSDF/non-equivalent-layer window model types to Detailed. This sequence is
documented only as source dependency and ordering, not Rust parity.

Rust already exposes `init_heat_balance_stage` as execution-plan metadata and
`init_heat_balance_source_order_path` as an identity wrapper. The current
limited runtime invokes that wrapper around its own internal-gain schedule
cache construction; bounded typed Zone and retained opaque-surface state;
construction thermal/CTF state; initial temperature and humidity state; and
selected surface-history seeding. Those existing mechanisms support only
their separately declared limited lanes. They do not implement the EnergyPlus
`InitHeatBalance` lifecycle, and CP114 changes no Rust target, state,
capability, support gate, comparator, proof variable, result, or conformance
boundary.

Complete first-call, simulation-start, environment-start, day, timestep,
warmup, weather-simulation, solar-integration, and report flags; all complete
Zone, Space, Surface, window, enclosure, mass-balance, conduction,
response-factor, and thermal-history allocations and resets; daylighting,
window optical/storm-window, ground and Kiva state; local-environment and EMS
coupling; every initialization child call and its exact ordering, re-entry,
partial side effects, and error behavior; progress messages, files, diagnostics
and reporting; runtime numerical behavior, performance, and conformance remain
deferred. CP115 maps the following unconditional
`ManageEMS(state, BeginZoneTimestepAfterInitHeatBalance, anyRan, absent)` call
at lines 199-200 by reusing the generic `routine.manage_ems` source mapping.
CP116 maps the line-209 Surface manager call and complete source parent body.
CP117 maps its inline first-time initialization display guard, and CP118 maps
the following unconditional `InitSurfaceHeatBalance` call and source routine.
CP119 maps the following first-time outside-balance display, and CP120 adds
the unconditional outside-balance call as a required routine. The first-time
inside-balance display follows, and CP121 maps it. CP122 adds the unconditional
inside-balance call as a required routine. CP123 maps the following first-time
air-balance display, and CP124 maps the unconditional Air-manager call by
reusing its existing required routine. CP125 adds the final Surface
heat-balance update as a required source mapping. CP126 adds the guarded
thermal-history update as a required source mapping. CP127 adds the inline
CondFD moisture-history helper as a non-required source mapping. CP128 adds
the ThermalComfort manager as a non-required source mapping. CP129 adds the
Surface reporting routine as a required source mapping. CP130 adds the
sizing-only component-load gathering call as a non-required source mapping.
CP131 adds the unconditional thermal-resilience calculation as a non-required
source mapping. CP132 adds its independently guarded summary report as a
non-required source mapping. CP133 adds the independently guarded CO2 summary
as a non-required source mapping. CP134 adds the independently guarded visual
summary as a non-required source mapping. CP135 maps the parent-tail
first-time-flag clear without a synthetic routine or count change. CP136 maps
the `EndZoneTimestepBeforeZoneReporting` EMS calling point by reusing the
existing generic routine without a row or count change. CP137 adds
`RecKeepHeatBalance` as a required source mapping; CP138 adds
`ReportHeatBalance` as a required source mapping; CP139 maps the
`EndZoneTimestepAfterZoneReporting` EMS calling point by reusing the generic
routine; CP140 adds non-required `UpdateEMSTrendVariables`; CP141 adds the
non-required plugin-value update; CP142 adds required
`CheckWarmupConvergence`; CP143 maps the inner day-counter resets; CP144 maps
the in-branch post-warmup EMS call; CP145 adds required
`ReportWarmupConvergence`; CP146 adds required
`SetPreConstructionInputParameters`; CP147 adds required
`GetSiteAtmosphereData`; CP148 adds required `AllocateZoneHeatBalArrays`;
CP149 adds required `AllocateHeatBalArrays`; CP150 adds required `UpdateWindowFaceTempsNonBSDFWin`; CP151 adds non-required `OpenShadingFile`; CP152 adds non-required `SetStormWindowControl`; CP153 adds required `InitConductionTransferFunctions`; CP154 adds non-required `GatherForPredefinedReport`; CP155 adds required `AllocateSurfaceHeatBalArrays`; CP156 adds required `InitThermalAndFluxHistories`; CP157 adds non-required `EvalOutsideMovableInsulation`; CP158 adds non-required `EvalInsideMovableInsulation`; CP159 adds required `InitSolarHeatGains`; CP160 adds required `InitIntSolarDistribution`; CP161 adds required `ComputeIntThermalAbsorpFactors`; CP162 adds required `ComputeIntSWAbsorpFactors`; CP163 adds required `ComputeDifSolExcZonesWIZWindows`; CP164 next in source-definition order maps `InitEMSControlledSurfaceProperties`.

### CP115 post-`InitHeatBalance` EMS calling-point map

Immediately after CP114's unconditional `InitHeatBalance(state)` return,
`ManageHeatBalance` lines 199-200 unconditionally call
`EMSManager::ManageEMS(state,
EMSCallFrom::BeginZoneTimestepAfterInitHeatBalance, anyRan,
ObjexxFCL::Optional_int_const())`. The fourth argument is again an explicitly
absent optional program-manager index. Every `ManageHeatBalance` invocation
that reaches this point makes the call, including later invocations whose
once-only input block was skipped.

The caller reuses the same `anyRan` variable declared at line 189 and passed
to the CP113 pre-initialization call. `ManageEMS` writes that referenced
result anew for the post-initialization calling point, so the earlier call's
value is overwritten rather than combined by `ManageHeatBalance`. CP115 maps
only that caller identity, order, argument, and overwrite boundary. It makes no
claim about which EMS programs, plugins, or callbacks run before or after
initialization, or about how they observe or mutate the state initialized by
CP114.

The canonical `routine.manage_ems` row added by CP113 covers the generic
source routine, so CP115 adds no second routine row, source file,
project-contract entry, Rust target or wrapper, model/runtime state,
capability, support-gate admission, manifest, comparator, proof variable,
result, count, or conformance promotion.

Complete EMS input/setup, Erl programs and program-call managers, sensors,
actuators, internal variables, trends, plugins, registered callbacks, and
external interfaces; calling-point gates and optional-index behavior; exact
`anyRan` or internal run-flag values; the semantic relationship between the
pre- and post-initialization calls; observation or mutation of initialized
state; actuator/reporting side effects, diagnostics and exact execution order;
other calling points, repeated-call/environment/timestep/warmup lifecycle,
runtime numerics, performance, output behavior, and conformance remain
deferred. CP116 maps the following unconditional
`HeatBalanceSurfaceManager::ManageSurfaceHeatBalance(state)` call at line 209
and expands the existing required `routine.manage_surface_heat_balance`
mapping rather than creating a duplicate row.

### CP116 `ManageSurfaceHeatBalance` source map

After the CP115 post-initialization EMS call, the comments at
`HeatBalanceManager.cc` lines 202-208 require the Surface manager to solve the
zone heat balance, call the Air manager from within that Surface-manager
sequence, and avoid record keeping before HVAC has run because radiant systems
may require HVAC/zone iteration. Line 209 then unconditionally calls
`HeatBalanceSurfaceManager::ManageSurfaceHeatBalance(state)`. Every
`ManageHeatBalance` invocation that reaches line 209 makes this call.

`HeatBalanceSurfaceManager.cc::ManageSurfaceHeatBalance` lines 145-230 own the
following parent-driver order:

1. as mapped by CP117, when `ManageSurfaceHeatBalancefirstTime` is true, display `Initializing Surfaces`;
2. as mapped by CP118, call `InitSurfaceHeatBalance(state)`;
3. as mapped by CP119, under the same first-time flag, display `Calculate Outside Surface Heat Balance`;
4. as mapped by CP120, call `CalcHeatBalanceOutsideSurf(state)`;
5. as mapped by CP121, under the same first-time flag, display `Calculate Inside Surface Heat Balance`;
6. as mapped by CP122, call `CalcHeatBalanceInsideSurf(state)`;
7. as mapped by CP123, under the same first-time flag, display `Calculate Air Heat Balance`;
8. as mapped by CP124, call `HeatBalanceAirManager::ManageAirHeatBalance(state)`;
9. as mapped by CP125, call `UpdateFinalSurfaceHeatBalance(state)`;
10. as mapped by CP126, call `UpdateThermalHistories(state)` only when
    `AnyCTF || AnyEMPD`;
11. when `AnyCondFD` is true, visit every `SurfNum` from 1 through
    `TotSurfaces`, skip non-heat-transfer surfaces whose construction is at
    most zero, skip window constructions, skip surfaces whose heat-transfer
    algorithm is not `CondFD`, and call
    `SurfaceFD(SurfNum).UpdateMoistureBalance()` for each survivor;
12. as mapped by CP128, call
    `ThermalComfort::ManageThermalComfort(state, false)`;
13. as mapped by CP129, call `ReportSurfaceHeatBalance(state)`;
14. as mapped by CP130, when `ZoneSizingCalc` is true, call
    `OutputReportTabular::GatherComponentLoadsSurface(state)`;
15. as mapped by CP131, call `CalcThermalResilience(state)`;
16. as mapped by CP132, when `displayThermalResilienceSummary` is true, call
    `ReportThermalResilience(state)`;
17. as mapped by CP133, when `displayCO2ResilienceSummary` is true, call
    `ReportCO2Resilience(state)`;
18. as mapped by CP134, independently call `ReportVisualResilience` when its
    summary display flag is true; and
19. as mapped by CP135, set `ManageSurfaceHeatBalancefirstTime = false`.

Rust already publishes `manage_surface_heat_balance_stage` as execution-plan
metadata and `manage_surface_heat_balance_source_order_path` as an identity
wrapper. In the current limited timestep path, that wrapper encloses only the
limited `InitSurfaceHeatBalance`/`CalcHeatBalanceOutsideSurf` path and closes
before the Rust Air-manager shell. The Air-manager shell runs before the
separately wrapped inside-surface pass, while final-surface and thermal-history
wrappers run later and surface reporting is wrapped from the run-period path.
That actual Rust nesting and order do not match or close the EnergyPlus parent
driver above. Existing retained opaque-surface, CTF, face-temperature, zone-air,
history, and report implementations remain bounded by their own documented
lanes and proof variables; CP116 neither broadens nor removes those narrow
implementations or claims.

CP116 adds no routine row, source file, project-contract routine, Rust target
or wrapper, model/runtime state, capability, support-gate admission, manifest,
comparator, proof variable, result, count, or conformance promotion. Complete
first-time/display lifecycle; initialization; outside-, inside-, air-, final-,
and history-balance implementations; radiant-system and HVAC iteration;
complete Surface, Construction, CTF, EMPD, CondFD, finite-difference moisture,
window, and heat-transfer-algorithm state; thermal comfort, surface reporting,
component-load gathering, resilience calculation/reporting, exact side
effects and error behavior, runtime numerics, performance, outputs, and broad
conformance remain deferred. CP117 maps the following inline
`ManageSurfaceHeatBalancefirstTime`-guarded `Initializing Surfaces` display at
lines 158-160 without a synthetic routine row.

### CP117 first-time `Initializing Surfaces` display map

At `HeatBalanceSurfaceManager.cc` lines 158-160,
`ManageSurfaceHeatBalance` tests
`state.dataHeatBalSurfMgr->ManageSurfaceHeatBalancefirstTime` and, only when
it is true, calls `DisplayString(state, "Initializing Surfaces")`. The guard
and output are inline parent behavior rather than a separately named source
routine. The unconditional `InitSurfaceHeatBalance(state)` call follows
immediately at line 161 whether or not this message was displayed.

`HeatBalanceSurfaceManager.hh` owns the lifecycle state: the
`ManageSurfaceHeatBalancefirstTime` member defaults to true at line 239 and
`clear_state()` resets it to true at line 291. The same flag remains true
through the parent's initialization, outside-balance, inside-balance, and
air-balance progress guards. None of those guards clears it. Only successful
reach of the parent tail at line 229 writes it false, so normal later parent
invocations skip all four messages. An abnormal exit before that tail can
leave the flag true, allowing this message and the other subsequently reached
first-time messages to repeat if the same state is invoked again.

Rust has no equivalent persistent `ManageSurfaceHeatBalancefirstTime` field,
reset lifecycle, or `DisplayString` progress output. The current
`manage_surface_heat_balance_source_order_path` identity wrapper executes its
closure without owning persistent first-time state; its limited one-closure
invocation and current nesting therefore are not parity for the source guard,
four-message lifetime, successful-tail clear, abnormal-exit behavior, or
re-entry. Adding `HeatBalanceSurfaceManager.hh` to the surface-manager
algorithm's EnergyPlus source list records only the declaration/reset owner.

CP117 adds no synthetic routine row, project-contract routine, Rust helper or
target, model/runtime state, capability, support-gate admission, manifest,
comparator, proof variable, result, count, output implementation, or
conformance promotion. Exact `DisplayString` stream routing, formatting,
buffering, callback behavior, message visibility and ordering outside the
mapped parent sequence; lifecycle behavior after fatal, exception, or other
abnormal control flow; the still-unmapped inside- and air-balance first-time
guards and messages; implementation of the outside-balance guard mapped by
CP119; child routine effects, runtime numerics, performance, outputs, and
broad conformance remain deferred. CP118 maps the following unconditional
`InitSurfaceHeatBalance(state)` call at line 161 and adds its canonical
required routine and project-contract entries.

### CP118 `InitSurfaceHeatBalance` source map

`ManageSurfaceHeatBalance` line 161 calls `InitSurfaceHeatBalance(state)`
unconditionally, immediately after the CP117 first-time display guard and
before the next outside-balance display guard. The implementation at
`HeatBalanceSurfaceManager.cc` lines 272-621 is a flag-driven surface
initialization driver. CP118 records it as `source_mapped` and required for
the full heat-balance domain; it does not add executable Rust state or claim
that any source child is complete.

The source routine preserves this major order and gating:

1. under its own first-time flag, display the outdoor-environment
   initialization message, then populate per-Surface outdoor
   dry-bulb, wet-bulb, wind-speed, and wind-direction state; apply linked
   outdoor-air-node values when local environments exist; and then apply EMS
   overrides for surface weather and ground view factor;
2. on `BeginSimFlag`, allocate surface heat-balance arrays and derive
   `InterZoneWindow` from the solar-enclosure view-factor state;
3. on `BeginSimFlag || AnySurfPropOverridesInModel`, walk Zone/Space heat
   transfer surfaces and copy inside/outside solar and thermal absorptance plus
   outside roughness from each active construction;
4. on every call, run `UpdateVariableAbsorptances(state)`;
5. on `BeginEnvrnFlag`, optionally display the temperature/flux-history
   message and call `InitThermalAndFluxHistories(state)`;
6. when movable insulation exists, evaluate outside then inside movable
   insulation; then, on every call, update ground-surface reflectance averages
   and initialize complex fenestration for the timestep;
7. when the sun is up with positive diffuse solar, compute anisotropic-sky
   view factors, otherwise zero the anisotropic multiplier; then perform the
   first-time window-shading display, `WindowShadingManager`, and glazing
   shading-status check;
8. perform the first-time interior-absorption display, first-time
   `InitInteriorRadExchange`, and per-call thermal absorption factors; then
   perform the first-time diffuse-solar display and short-wave absorption
   factors, including the guarded first-time interzone-window display and
   diffuse solar exchange when `InterZoneWindow` is true;
9. initialize daylighting with the routine's first-time flag, calculate
   interior long-wave radiation from inside temperature history, and, when
   airflow windows exist, run window-gap airflow control;
10. in order, perform the first-time solar-gain display and
    `InitSolarHeatGains`; manage daylighting; perform the first-time internal
    gain display and `ManageInternalHeatGains(state, false)`; perform the
    first-time interior solar-distribution display and
    `InitIntSolarDistribution`; then perform the first-time interior-convection
    display and initialize inside convection coefficients from
    `SurfTempInTmp`;
11. on `BeginSimFlag`, optionally display the predefined-report message and
    call `GatherForPredefinedReport(state)`;
12. when `AnyCondFD` is true, call `InitHeatBalFiniteDiff(state)`;
13. walk the Zone/Space opaque or internal-mass surfaces using CTF or EMPD and
    accumulate inside/outside CTF history constants from construction
    coefficients and temperature/flux histories; when internal heat-source
    input exists, make the following source/sink pass and add its CTF source,
    user-temperature, and source-temperature history terms;
14. over the Zone/Space heat-transfer-surface ranges, reset the listed
    radiant-system, PV, pool, and associated heat-balance coefficient/source
    arrays, then reset every supported
    radiant-HVAC source category over `allGetsRadiantHeatSurfaceList`;
15. when `ZoneSizingCalc` is true, call
    `GatherComponentLoadsSurfAbsFact(state)`; finally, under the routine's
    first-time flag, display completion, then at the successfully reached tail
    set `InitSurfaceHeatBalancefirstTime = false`.

This order depends on complete Zone, Space, Surface, active-Construction,
solar-enclosure, loop-node, local-environment, EMS-override, material, window,
daylighting, solar-shading, internal-gain, convection, CTF/EMPD/CondFD,
history, radiant-system, sizing, and reporting state, together with
`BeginSimFlag`, `BeginEnvrnFlag`, solar/weather, feature-presence, and
first-time lifecycle flags.

Rust already exposes `init_surface_heat_balance_stage` as source-order
metadata and `init_surface_heat_balance_source_order_path` as an identity
wrapper. The current limited timestep path nests that wrapper inside the
surface-manager wrapper only around the limited outside-balance closure. It
does not execute the source initialization driver or reproduce its flags,
children, state writes, or order. Following the existing metadata-only
precedent, CP118 intentionally leaves the surface algorithm's `rust_target`
list unchanged; listing the stage now would imply a promotion that did not
occur. Existing bounded opaque-surface, CTF, face-temperature, zone-air,
history, and reporting implementations and their narrow claims are unchanged.

Complete child-routine behavior and exact call nesting; first-time,
simulation-start, environment-start, local-environment, EMS, weather, solar,
movable-insulation, fenestration/window, daylighting, airflow-window,
internal-gain, CTF, EMPD, CondFD, source/sink, radiant/HVAC, sizing, and report
state; allocation, mutation, reset, and re-entry behavior; progress/output
messages, files and diagnostics; error and partial-side-effect behavior;
runtime numerics, performance, outputs, and conformance remain deferred.
CP118 adds no Rust target or wrapper, mapped model/runtime state, capability,
support-gate admission, manifest, comparator, proof variable, result, or
conformance promotion. CP119 maps the following
`ManageSurfaceHeatBalancefirstTime`-guarded
`Calculate Outside Surface Heat Balance` display at parent lines 165-167.

### CP119 first-time `Calculate Outside Surface Heat Balance` display map

At `HeatBalanceSurfaceManager.cc` lines 165-167,
`ManageSurfaceHeatBalance` tests the same
`state.dataHeatBalSurfMgr->ManageSurfaceHeatBalancefirstTime` flag mapped by
CP117 and, only when it is true, calls
`DisplayString(state, "Calculate Outside Surface Heat Balance")`. This inline
parent output block is reached only after the unconditional line-161
`InitSurfaceHeatBalance(state)` call returns. It is immediately followed by
the unconditional line-168 `CalcHeatBalanceOutsideSurf(state)` call.

CP119 introduces no new lifecycle state. As already mapped by CP117,
`HeatBalanceSurfaceManager.hh` defaults the shared flag to true at line 239,
`clear_state()` resets it to true at line 291, and only successful reach of
the parent tail at line 229 clears it. An abnormal exit before lines 165-167
suppresses this message for that invocation; in particular, if
`InitSurfaceHeatBalance` does not return normally, this display is not
reached. An abnormal exit after the display but before the parent-tail clear
can leave the flag true, so the message can repeat when the same state
re-enters the parent. After a normally reached tail, later invocations skip
this and the other three first-time messages.

Rust has no persistent `ManageSurfaceHeatBalancefirstTime` field, reset
lifecycle, or progress-output implementation. The existing Surface-manager,
initialization, and outside-balance stage metadata and identity wrappers do
not own this flag or emit this message, so their current nesting is not parity
for the source guard, output, abnormal-exit behavior, or re-entry.

CP119 adds no synthetic routine or source row, project-contract entry, Rust
helper, target, wrapper, model/runtime state, capability, support-gate
admission, manifest, comparator, proof variable, result, count, output
implementation, or conformance promotion. Exact `DisplayString` routing,
formatting, buffering, callbacks, visibility, and ordering outside the mapped
parent sequence; `InitSurfaceHeatBalance` failure and partial-side-effect
behavior; the following child routine, later progress guards, parent-tail
lifecycle, runtime numerics, performance, outputs, and broad conformance
remain deferred. CP120 maps the unconditional line-168
`CalcHeatBalanceOutsideSurf(state)` call as a canonical required routine and
project-contract entry.

### CP120 `CalcHeatBalanceOutsideSurf` source map

`ManageSurfaceHeatBalance` line 168 calls
`CalcHeatBalanceOutsideSurf(state)` unconditionally, after the CP119
first-time outside-balance display guard and immediately before the next
inside-balance display guard. The caller omits the optional
`ZoneToResimulate` argument, so this checkpoint selects the complete normal
call rather than a zone-resimulation subset. The implementation at
`HeatBalanceSurfaceManager.cc` lines 6951-7721 is recorded as
`source_mapped` and required for the full heat-balance domain.

The source routine preserves this major order and gating:

1. calculate average temperatures for ground surfaces and scheduled
   surrounding surfaces;
2. when any input construction has an internal heat source, visit the
   complete Surface array and transfer radiant-system plus integrated-PV
   sources into per-area source history for positive-area surfaces;
3. calculate interior long-wave exchange for the outside face. The source
   routine can forward `ZoneToResimulate`, but the parent call mapped here
   omits it and therefore takes the complete no-zone-filter form;
4. sample scheduled outside additional-heat-source terms for
   `allOutsideSourceSurfaceList`;
5. visit Zone order, each Zone's Space indexes, and each Space's
   `HTSurfaceFirst..HTSurfaceLast` range; skip Window-class surfaces. The
   optional Zone/adjacent-Zone filter exists in the reusable routine but is
   inactive for this parent call. For each survivor, select the active
   Construction and reset exterior convection, air/sky/ground/surrounding
   radiation coefficients and surrounding-surface long-wave flux;
6. dispatch exactly by exterior boundary condition in this order:
   `Ground`, `GroundFCfactorMethod`, `OtherSideCoefNoCalcExt`,
   `OtherSideCoefCalcExt`, `OtherSideCondModeledExt`,
   `ExternalEnvironment`, `KivaFoundation`, then the default
   interior/other branch. These branches set outside-face histories and
   radiant-source coefficients; derive other-side conditions and limits;
   initialize exterior convection/radiation; populate HAMT and CondFD
   moisture boundary state; run modeled exterior conditions and vented
   cavities; handle movable insulation, wet/dry/no-wind weather, EcoRoof,
   scheduled surrounding-surface long-wave exchange, and Kiva convection;
   solve eligible CTF, EMPD, or TDD-dome outside temperatures; and terminate
   through the source fatal paths when movable-insulation temperature solving
   reports an error. The default branch distinguishes self-referencing
   interior mass from interzone partitions and copies the corresponding
   inside temperature, vapor, and convection state;
7. when a non-Window surface's boundary branch reaches the common tail, store
   `SurfQdotConvOutPerArea` through `GetQdotConvOutPerArea`. The
   `ExternalEnvironment` EcoRoof path instead executes the line-7440 early
   `continue` and skips this common-tail store. For every path that reaches
   it, the helper at lines 7723-7736 uses modeled-other-side convection first,
   otherwise the rain wet-bulb reference, otherwise the dry-bulb reference.
   CP120 records that helper as a dependency of the canonical routine, not as
   a separate routine-inventory row.

This order depends on the complete Zone/Space heat-transfer-surface topology,
complete Surface order and families, active Construction and material state,
ground and surrounding-surface schedules, internal radiant and PV sources,
interior long-wave exchange, outside source schedules, weather and local
surface conditions, convection algorithms, OSC/OSCM, Kiva, EcoRoof, vented
cavities, movable insulation, CTF/EMPD/CondFD/HAMT, moisture and radiant-system
arrays, and fatal/error side effects. The optional zone-resimulation entry
path additionally depends on owning and adjacent Zone identity even though
the mapped parent call does not activate it.

Rust already lists `calc_heat_balance_outside_surf_stage` as source-order
metadata and owns `calc_heat_balance_outside_surf_source_order_path` as an
identity wrapper. Its retained opaque-surface lane also computes bounded CTF
outside environmental balances and selected exterior convection, radiation,
long-wave, and report terms. Those narrow pieces do not reproduce the source
Zone/Space/Surface traversal, full boundary switch, child-call topology and
order, optional resimulation semantics, state mutation, error behavior, or
numerical result, and CP120 does not promote them.

Complete branch and child behavior, complete topology and state ownership,
all flag and schedule lifecycles, optional resimulation, mutation/reset and
partial-side-effect behavior, diagnostics and fatal paths, runtime numerics,
performance, outputs, and broad conformance remain deferred. CP120 adds no
Rust target, wrapper, model/runtime state, capability, support-gate admission,
manifest, comparator, proof variable, result, or conformance promotion; the
existing surface algorithm target list and narrow claims remain unchanged.
CP121 maps the parent lines 169-171 first-time
`Calculate Inside Surface Heat Balance` display guard.

### CP121 first-time `Calculate Inside Surface Heat Balance` display map

At `HeatBalanceSurfaceManager.cc` lines 169-171,
`ManageSurfaceHeatBalance` tests the same
`state.dataHeatBalSurfMgr->ManageSurfaceHeatBalancefirstTime` flag mapped by
CP117 and, only when it is true, calls
`DisplayString(state, "Calculate Inside Surface Heat Balance")`. This inline
parent output block is reached only after the unconditional line-168
`CalcHeatBalanceOutsideSurf(state)` call mapped by CP120 returns. It is
immediately followed by the unconditional line-172
`CalcHeatBalanceInsideSurf(state)` call.

CP121 introduces no new lifecycle state. `HeatBalanceSurfaceManager.hh`
defaults the shared flag to true at line 239 and `clear_state()` resets it to
true at line 291. None of the parent's four first-time guards clears the flag;
only successful reach of the parent tail at line 229 writes it false. An
abnormal exit from `CalcHeatBalanceOutsideSurf` prevents this message from
being reached for that invocation. An abnormal exit after this display but
before the parent-tail clear can leave the flag true, so this message can
repeat when the same state re-enters the parent. After the tail is reached
normally, later invocations skip this and the other first-time messages.

Rust has no persistent `ManageSurfaceHeatBalancefirstTime` field, reset
lifecycle, or progress-output implementation. The existing Surface-manager
and inside-balance stage metadata and identity wrappers neither own this flag
nor emit this message, so their current execution and nesting are not parity
for the source guard, output, abnormal-exit behavior, or re-entry.

CP121 adds no synthetic routine or source row, project-contract entry, Rust
helper, target, wrapper, model/runtime state, capability, support-gate
admission, manifest, comparator, proof variable, result, count, output
implementation, or conformance promotion. Exact `DisplayString` routing,
formatting, buffering, callbacks, visibility, and ordering outside the mapped
parent sequence; outside-balance failure and partial-side-effect behavior;
the following child routine, later progress guard, parent-tail lifecycle,
runtime numerics, performance, outputs, and broad conformance remain deferred.
CP122 maps the unconditional line-172 `CalcHeatBalanceInsideSurf(state)` call
as a canonical required routine and project-contract entry. CP123 maps the
following first-time air-balance display guard, and CP124 maps the
unconditional Air-manager caller by reusing its existing required routine.
CP125 adds the final Surface heat-balance update as a required source mapping;
CP126 adds the guarded thermal-history update as a required source mapping.
CP127 adds the inline CondFD moisture-history helper as a non-required source
mapping. CP128 adds the ThermalComfort manager as a non-required source
mapping. CP129 adds the Surface reporting routine as a required source
mapping. CP130 adds the sizing-only component-load gathering call as a
non-required source mapping. CP131 adds the unconditional thermal-resilience
calculation as a non-required source mapping. CP132 adds its independently
guarded summary report as a non-required source mapping. CP133 adds the
independently guarded CO2 summary as a non-required source mapping. CP134 adds
the independently guarded visual summary as a non-required source mapping.
CP135 maps the parent-tail first-time-flag clear without a synthetic routine or
count change. CP136 maps the `EndZoneTimestepBeforeZoneReporting` EMS calling
point by reusing the existing generic routine without a row or count change.
CP137 adds `RecKeepHeatBalance` as a required source mapping, CP138 adds
`ReportHeatBalance` as a required source mapping, CP139 maps the
`EndZoneTimestepAfterZoneReporting` EMS calling point by reusing the generic
routine, CP140 adds non-required `UpdateEMSTrendVariables`, CP141 adds the
non-required plugin-value update, CP142 adds required
`CheckWarmupConvergence`, CP143 maps the inner day-counter resets, CP144 maps
the in-branch post-warmup EMS call, CP145 adds required
`ReportWarmupConvergence`, CP146 adds required
`SetPreConstructionInputParameters`, CP147 adds required
`GetSiteAtmosphereData`, CP148 adds required `AllocateZoneHeatBalArrays`, and
CP149 adds required `AllocateHeatBalArrays`; CP150 adds required `UpdateWindowFaceTempsNonBSDFWin`; CP151 adds non-required `OpenShadingFile`; CP152 adds non-required `SetStormWindowControl`; CP153 adds required `InitConductionTransferFunctions`; CP154 adds non-required `GatherForPredefinedReport`; CP155 adds required `AllocateSurfaceHeatBalArrays`; CP156 adds required `InitThermalAndFluxHistories`; CP157 adds non-required `EvalOutsideMovableInsulation`; CP158 adds non-required `EvalInsideMovableInsulation`; CP159 adds required `InitSolarHeatGains`; CP160 adds required `InitIntSolarDistribution`; CP161 adds required `ComputeIntThermalAbsorpFactors`; CP162 adds required `ComputeIntSWAbsorpFactors`; CP163 adds required `ComputeDifSolExcZonesWIZWindows`; CP164 next in source-definition order maps `InitEMSControlledSurfaceProperties`.

### CP122 `CalcHeatBalanceInsideSurf` source map

`ManageSurfaceHeatBalance` line 172 calls
`CalcHeatBalanceInsideSurf(state)` unconditionally, after the CP121
first-time inside-balance display guard and before the next first-time
air-balance display guard. The caller omits the optional
`ZoneToResimulate` argument, so this checkpoint selects the complete-building
call. The canonical wrapper at `HeatBalanceSurfaceManager.cc` lines 7738-7813
is recorded as `source_mapped` and required for the full heat-balance domain.
The conditional full-building re-entry from
`UpdateFinalSurfaceHeatBalance` lines 5213-5217 and the separate radiant,
baseboard, pool, slab, and simulation-manager call sites remain dependency
context rather than additional routine rows at this checkpoint.

The canonical wrapper preserves this major order and gating:

1. on `calcHeatBalInsideSurfFirstTime`, raise `MinIterations` to
   `MinEMPDIterations` when `AnyEMPD`; optionally register the advanced
   zone-timestep/sum inside-surface iteration-count output; scan every Zone,
   its Spaces, and their heat-transfer surfaces for CondFD, HAMT, or Kiva to
   populate `Zone_has_mixed_HT_models`; then clear the first-time flag;
2. on the first `BeginEnvrnFlag` call, initialize `SurfTempInsOld`,
   `RefAirTemp`, and `SurfTempEffBulkAir` to 23 C, reset
   `calcHeatBalInsideSurfWarmupErrCount`, conditionally initialize Kiva
   instances, and clear the environment flag. A later call with
   `BeginEnvrnFlag` false rearms that flag;
3. call `sumSurfQdotRadHVAC` at line 7788. Its lines 9277-9285 helper visits
   `allGetsRadiantHeatSurfaceList` and sums the high-temperature radiant,
   hot-water, steam, electric-baseboard, and cooling-panel components into
   `SurfQdotRadHVACInPerArea`;
4. when no optional Zone is present, dispatch `AllCTF` state to the existing
   `CalcHeatBalanceInsideSurf2CTFOnly` implementation at lines 8658-9275 with
   Zones 1 through `NumOfZones` and `AllIZSurfaceList`; otherwise dispatch the
   complete HT, interzone, non-window, and window lists to the general
   `CalcHeatBalanceInsideSurf2` implementation at lines 7815-8656. When an
   optional Zone is present, always use the general implementation with that
   Zone's four lists and forward the Zone argument, because adjacent interzone
   surfaces make the optimized CTF-only path invalid for partial resimulation;
5. after either child returns, forward the optional Zone to
   `CalculateZoneMRT` at lines 5583-5699 and then
   `UpdateIntermediateSurfaceHeatBalanceResults` at lines 4951-5020.

The general child first derives reference-air temperatures while skipping
TDD domes, resets window heat-gain/report accumulators, resets
`InsideSurfIterations`, samples scheduled inside additional-heat-source terms,
and conditionally advances Kiva instances according to their hourly or
timestep setting outside warmup. Each convergence iteration then snapshots
inside temperatures; temporarily substitutes Kiva radiant temperatures for
interior long-wave exchange; restores the Kiva inside temperatures; and
re-evaluates inside convection after iteration zero on the
`ItersReevalConvCoeff` cadence of 30. EMPD/HAMT vapor and mass-transfer
boundaries follow before the non-window solve.

The non-window loop filters non-representative surfaces and distinguishes
adiabatic from standard/interzone boundaries. It dispatches CTF/EMPD,
CondFD/HAMT, and Kiva algorithms; applies pool, embedded-source, mixed-model
temperature-limit, and movable-insulation branches; and establishes the
radiant-system coefficients and paired interzone coefficients owned by each
branch. The following window loop filters representatives, solves TDD
diffusers on every pass, and evaluates regular windows only on iteration zero,
including user exterior convection, wind/rain or no-wind exterior
coefficients, interior/interzone coefficients, shading/equivalent-layer
emissivity, and `Window::CalcWindowHeatBalance`. The child then commits
inside/outside report temperatures and TDD coupling, checks surface
temperature bounds, synchronizes interzone outside temperatures from their
paired current inside histories, and increments the iteration count.

Convergence uses the largest non-window inside-temperature change and, for
CondFD surfaces, the largest internal-node change. Global `AnyCondFD` selects
`MaxAllowedDelTempCondFD`; otherwise `MaxAllowedDelTemp` applies. CondFD
relaxation resets to its input value through the first iteration and, after
`IterationsForCondFDRelaxChange`, falls by a factor of 0.9 while unconverged
with a floor of 0.1. The source enforces the current `MinIterations`, raised to
four by the wrapper for any EMPD input, and warns or records a recurring
warning outside warmup before breaking when `InsideSurfIterations` exceeds
`MaxIterations` 500. After the loop, an EMPD/HAMT gate zeros every Zone's
moisture sums and then updates HAMT or EMPD surfaces in the supplied
non-window list, including their psychrometric accumulation terms.

The optional-Zone path is therefore not a pure local-side-effect boundary.
The wrapper's first-call scan, environment initialization, and radiant-HVAC
aggregation are global. The general child samples
`allInsideSourceSurfaceList`, can advance all Kiva instances, and globally
zeros Zone moisture sums before repopulating only the supplied surface subset.
`CalculateZoneMRT` performs global first-call allocation and enclosure setup;
`UpdateIntermediateSurfaceHeatBalanceResults` scopes its main Zone/Space loops
but still updates `AllHTKivaSurfaceList`. CP122 records these exact source
semantics without claiming that Rust implements or supports partial
resimulation.

The general child fatally rejects interior movable insulation combined with
an embedded construction source or sink at lines 8265-8278. Failure to
converge within the iteration limit warns or records a recurring warning only
outside warmup; every such path then breaks without a convergence fatal. Both
child implementations call
`TestSurfTempCalcHeatBalanceInsideSurf`, lines 9287-9468, after a normal
surface-temperature bound is crossed. Outside warmup, crossing the separate
before-fatal bounds is fatal; during warmup that branch becomes fatal only for
a temperature below -10000 C or above 10000 C.

The checker's warmup-count fatal thresholds have a narrower reachable
boundary than their code shape suggests. Both child call sites pass
`calcHeatBalInsideSurfWarmupErrCount` by value, the helper receives an `int`
by value, and line 9293 increments only that local copy. The stored member is
reset but never incremented anywhere in this source. Thus a production helper
invocation reaches at most local value one during warmup and value zero
outside warmup: the `WarmupSurfTemp > 3` enforced-reciprocity fatal and the
`WarmupSurfTemp > 10` ordinary fatal at lines 9388-9395 are unreachable from
these call sites. `DisplayExtraWarnings` can expose the diagnostic block
during warmup but cannot make either threshold true. CP122 records this
code-structure/reachability distinction and does not turn unreachable
thresholds into claimed runtime parity.

The existing surface algorithm already lists
`calc_heat_balance_inside_surf_stage` and owns
`calc_heat_balance_inside_surf_source_order_path`; the latter is an identity
wrapper around bounded Rust surface passes. Those targets and the existing
required `routine.calc_heat_balance_inside_surf_2_ctf_only` row remain
unchanged. They do not implement or promote the canonical wrapper's
first-call/environment lifecycle, exact optimized/general/partial dispatch,
complete non-CTF topology, Kiva/EMPD/HAMT/window behavior, optional
resimulation side effects, MRT/intermediate-result tail, errors, or numerical
results. CP122 adds one required `source_mapped` routine and project-contract
entry only: no EnergyPlus source, Rust target/state, capability, support-gate
admission, manifest, comparator, proof variable, result, family gate, or
conformance claim changes. CP123 maps the parent lines 176-178 first-time
`Calculate Air Heat Balance` display guard.

### CP123 first-time `Calculate Air Heat Balance` display map

At `HeatBalanceSurfaceManager.cc` lines 176-178,
`ManageSurfaceHeatBalance` tests the same
`state.dataHeatBalSurfMgr->ManageSurfaceHeatBalancefirstTime` flag mapped by
CP117 and, only when it is true, calls
`DisplayString(state, "Calculate Air Heat Balance")`. This inline parent
output block is reached only after the unconditional line-172
`CalcHeatBalanceInsideSurf(state)` call mapped by CP122 returns. The comments
at lines 174-175 require the Air heat balance to run before thermal-history
updates because a radiant system can be present. The display is immediately
followed by the unconditional line-179
`HeatBalanceAirManager::ManageAirHeatBalance(state)` call.

The physical source sequence through this part of the parent is outside
surface balance, inside surface balance, Air heat balance, final Surface heat
balance, then thermal-history update. CP123 maps only the progress guard inside
that sequence. The current Rust path does not reproduce the complete nesting
or order: the Surface-manager wrapper encloses the limited initialization and
outside-balance path and then closes; the Air-manager shell executes before a
separately wrapped inside-balance pass; final-balance and history work occur
later. Existing stage identities and narrow numerical claims therefore do not
promote this source order.

CP123 introduces no new lifecycle state. `HeatBalanceSurfaceManager.hh`
defaults the shared flag to true at line 239 and `clear_state()` resets it to
true at line 291. None of the four progress guards clears it; only successful
reach of the parent tail at line 229 writes it false. An abnormal exit from
`CalcHeatBalanceInsideSurf` prevents this message from being reached for that
invocation. An abnormal exit after this display but before the parent-tail
clear can leave the flag true, so this message can repeat when the same state
re-enters the parent. After a normally reached tail, later invocations skip
this and the other first-time messages.

Rust has no persistent `ManageSurfaceHeatBalancefirstTime` field, reset
lifecycle, or progress-output implementation. Existing Surface-manager,
inside-balance, and Air-manager metadata or identity wrappers neither own this
flag nor emit this message, so they are not parity for the source guard,
output, abnormal-exit behavior, or re-entry.

CP123 adds no synthetic routine or source row, project-contract entry, Rust
helper, target, wrapper, model/runtime state, capability, support-gate
admission, manifest, comparator, proof variable, result, count, output
implementation, or conformance promotion. Exact `DisplayString` routing,
formatting, buffering, callbacks, visibility, and ordering outside the mapped
parent sequence; inside-balance failure and partial-side-effect behavior; the
following Air-manager implementation, later parent stages, parent-tail
lifecycle, runtime numerics, performance, outputs, and broad conformance
remain deferred. CP124 maps the unconditional line-179
`HeatBalanceAirManager::ManageAirHeatBalance(state)` caller checkpoint below
by reusing the existing canonical routine and project-contract entry.

### CP124 `ManageAirHeatBalance` source map

After the CP123 first-time Air-balance display block,
`HeatBalanceSurfaceManager.cc::ManageSurfaceHeatBalance` line 179
unconditionally calls
`HeatBalanceAirManager::ManageAirHeatBalance(state)`. The display's
`ManageSurfaceHeatBalancefirstTime` guard does not enclose this call. Every
parent invocation that reaches line 179 therefore enters the Air manager,
after the unconditional inside Surface balance has returned and before the
comments at lines 181-183 and unconditional line-184
`UpdateFinalSurfaceHeatBalance(state)` call.

`HeatBalanceAirManager.cc::ManageAirHeatBalance` has signature
`void ManageAirHeatBalance(EnergyPlusData &state)` at line 135 and ends at
line 161. Its source order is:

1. at lines 149-152, when
   `state.dataHeatBalAirMgr->ManageAirHeatBalanceGetInputFlag` is true, call
   `GetAirHeatBalanceInput(state)` and only after that child returns write the
   flag false;
2. at line 154, unconditionally call `InitAirHeatBalance(state)`;
3. at line 158, unconditionally call `CalcHeatBalanceAir(state)`; and
4. at line 160, unconditionally call `ReportZoneMeanAirTemp(state)`.

The one-time input child at lines 163-189 initializes a local
`ErrorsFound = false`, then calls `GetAirFlowFlag`,
`SetZoneMassConservationFlag`, and `GetRoomAirModelParameters` in that order,
and calls `ShowFatalError` when errors remain. `HeatBalanceAirManager.hh` line
101 defaults `ManageAirHeatBalanceGetInputFlag` to true, and `clear_state()`
resets it true at line 114. Because the manager's false assignment follows the
child call, a fatal or other non-return from input leaves the flag true; if the
same state can re-enter, the input child is attempted again. After a normal
return and clear, later manager invocations skip only input acquisition and
still execute all three unconditional children.

The unconditional initialization child spans lines 4494-4507 and performs its
every-timestep work through `InitSimpleMixingConvectiveHeatGains(state)`. The
Air-calculation child spans lines 4590-4604: it initializes and invokes the
external HVAC manager when that callback is configured, otherwise it calls
`HVACManager::ManageHVAC(state)`. The reporting child spans lines 4615-4687.
It owns a different `CalcExtraReportVarMyOneTimeFlag`, defaulted true at
`HeatBalanceAirManager.hh` line 102 and reset true at line 115; the child scans
requested output variables and EMS sensors under that flag, clears it at line
4674, then updates Zone and optional Space mean-air report state on every
call. Neither Air-manager flag is the Surface-manager progress flag mapped by
CP117/CP123.

The existing required
`heat_balance_air_manager_source_order.routine.manage_air_heat_balance` row
remains the canonical record, so CP124 does not create a duplicate routine or
project-contract entry. The algorithm source inventory now includes
`HeatBalanceAirManager.hh` to ground the two flag defaults and resets. The
child ranges above define the manager's call topology and lifecycle boundary;
they are not separate completed routine rows and do not promote their complete
input families, mixing/room-air state, HVAC execution, reporting state, error
behavior, or numerical results.

Rust already exposes `manage_air_heat_balance_stage`, an identity source-order
wrapper, and compatibility aliases for the manager and its three children.
Those names do not reproduce this source driver. The current timestep closes
the Surface-manager wrapper after limited Init/Outside work, runs and closes
the Air-manager/Air-calculation shell before the separately wrapped Inside
pass, and invokes the report compatibility wrapper later from the run-period
reporting path. Rust owns neither source one-time flag nor either flag's
reset/failure/re-entry lifecycle. Existing
bounded zone-air calculations and outputs therefore remain unchanged and do
not establish exact manager or parent ordering.

CP124 changes no routine or project-contract count, Rust target/state,
capability, support-gate admission, manifest, comparator, proof variable,
result, family gate, output implementation, or conformance claim. The ledger
remains 32 algorithms and 138 routines, split into 58 `state_mapped` and 80
`source_mapped` routines. Complete child implementations, flags and partial
side effects, external and standard HVAC behavior, runtime numerics,
performance, outputs, and broad compatibility remain deferred. CP125 maps the
unconditional parent line-184 `UpdateFinalSurfaceHeatBalance(state)` call and
its canonical implementation below.

### CP125 `UpdateFinalSurfaceHeatBalance` source map

After the CP124 Air manager returns, the comments at
`HeatBalanceSurfaceManager.cc` lines 181-183 explain that a final average
surface heat-balance pass may be necessary when a radiant system ran for part
or all of the zone timestep. Parent line 184 then unconditionally calls
`UpdateFinalSurfaceHeatBalance(state)`. No first-time, radiant-system, or other
parent guard encloses the call. The canonical implementation spans lines
5176-5219 and is recorded as required and `source_mapped`.

The routine always calls these seven averaged-source updaters in exact order:

1. `LowTempRadiantSystem::UpdateRadSysSourceValAvg`;
2. `HighTempRadiantSystem::UpdateHTRadSourceValAvg`;
3. `HWBaseboardRadiator::UpdateBBRadSourceValAvg`;
4. `SteamBaseboardRadiator::UpdateBBSteamRadSourceValAvg`;
5. `ElectricBaseboardRadiator::UpdateBBElecRadSourceValAvg`;
6. `CoolingPanelSimple::UpdateCoolingPanelSourceValAvg`; and
7. `SwimmingPool::UpdatePoolSourceValAvg`.

Each child receives one of the seven local `bool` variables by reference,
initializes that flag false, and can return immediately when its equipment
family is absent. Otherwise it transfers the family's zone-timestep average
source into current heat-balance state and, where applicable, redistributes
radiant gains or reconciles interzone source state. A child sets its flag true
when at least one relevant averaged source is exactly nonzero. Consequently,
even when every final flag is false, these unconditional updater calls can
still copy, clear, or redistribute average state; a false rerun gate does not
make the whole routine side-effect free. The seven children and their deeper
distribution helpers are dependency context only. CP125 adds no separate
routine rows or EnergyPlus source inventory for them.

After all seven updaters return, lines 5213-5217 test the exact logical OR of
`LowTempRadSysOn`, `HighTempRadSysOn`, `HWBaseboardSysOn`,
`SteamBaseboardSysOn`, `ElecBaseboardSysOn`, `CoolingPanelSysOn`, and
`SwimmingPoolOn`. If none is true, the function returns after the updater side
effects. If any is true, it calls `CalcHeatBalanceOutsideSurf(state)` and then
`CalcHeatBalanceInsideSurf(state)` in that actual code order. Both calls omit
the optional `ZoneToResimulate` argument and therefore select the complete
building paths mapped by CP120 and CP122. The final update does not rerun
`InitSurfaceHeatBalance`, `ManageAirHeatBalance`, or
`UpdateThermalHistories`; after it returns, the parent separately evaluates
the CP126-mapped history guard.

Rust already publishes `update_final_surface_heat_balance_stage`, includes it
after `ManageZoneAirUpdates` in the compatibility stage list, and uses
`update_final_surface_heat_balance_source_order_path` in the timestep path.
CP125 therefore adds the existing stage to the Surface-manager algorithm's
`rust_target` list between the inside-balance and thermal-history stages. This
is scaffold-target recognition only. The current wrapper owns only bounded
adiabatic outside-face synchronization and report/history snapshot handling;
it does not call any of the seven equipment-source updaters or conditionally
repeat the complete outside and inside source routines. The execution-plan
stage identity, dependency metadata, wrapper, and existing narrow surface
calculations do not promote source routine state, equipment support, exact
side effects, ordering, errors, numerics, outputs, performance, or
conformance.

CP125 adds one required `source_mapped` routine and project-contract entry,
without a Rust code edit, state map, capability, support-gate admission,
manifest, comparator, proof variable, result, family gate, output, or
conformance promotion. The inventory becomes 32 algorithms and 139 routines,
split into 58 `state_mapped` and 81 `source_mapped` routines. The following
CP126 section maps the parent lines 186-189 `AnyCTF || AnyEMPD` guard and
`UpdateThermalHistories(state)` call, whose canonical implementation spans
lines 5221-5581.

### CP126 `UpdateThermalHistories` source map

Immediately after the CP125 final Surface update returns,
`HeatBalanceSurfaceManager.cc::ManageSurfaceHeatBalance` lines 186-189 test
the exact logical OR `state.dataHeatBal->AnyCTF ||
state.dataHeatBal->AnyEMPD`. Only a true result calls
`UpdateThermalHistories(state)`; a false result skips the complete routine and
continues to the following CondFD block. The call is therefore after the Air
subtree and final Surface update and before finite-difference moisture work.
The canonical routine spans lines 5221-5581 and is recorded as required and
`source_mapped`.

The routine first owns a separate one-time lifecycle. Its
`UpdateThermalHistoriesFirstTimeFlag` defaults to true at
`HeatBalanceSurfaceManager.hh` line 241 and is reset true by `clear_state()` at
line 293. On the first entered call, lines 5245-5256 dimension `QExt1`, `QInt1`,
`TempInt1`, `TempExt1`, and `SumTime` over `TotSurfaces`; when
`AnyInternalHeatSourceInInput` is true they also dimension `Qsrc1`, `Tsrc1`,
and `Tuser1`. The flag is then cleared. If the parent CTF/EMPD guard is false,
none of this allocation or flag mutation occurs.

The first Zone -> Space -> `OpaqOrIntMassSurface` traversal accepts only CTF
or EMPD surfaces, reads each declared `surface.Construction`, and skips a
construction whose `NumCTFTerms` is zero. For every survivor it derives the
current inside flux from current outside and inside temperatures, the CTF
constant part, and any embedded source term; stores the inside area-rate,
per-area report flux, and term-1 inside-flux history; and, when a source/sink
exists, updates the source and user-location temperatures. It then computes
term-1 outside flux with the optional source contribution and stores the
outside report flux with a sign reversal plus its area-rate.

Lines 5332-5356 then select a whole-routine fast path when
`SimpleCTFOnly && !AnyConstrOverridesInModel`. That path recycles the final
temperature and flux history-array objects, shifts terms from
`MaxCTFTerms + 1` down through term 3, copies all four term-1 arrays into term
2, and returns immediately. It does not enter the normal per-surface counter,
master-history interpolation, or embedded-source-history passes below.

The normal path first captures term-1 outside/inside temperature and flux
samples into `TempExt1`, `TempInt1`, `QExt1`, and `QInt1` whenever a CTF/EMPD
surface's `SurfCurrNumHist` is zero. Under the global internal-source gate, a
second traversal likewise captures `Tsrc1`, `Tuser1`, and `Qsrc1`. Unlike the
initial current-flux pass, these capture loops and the following counter loop
do not repeat the `NumCTFTerms == 0` skip; they filter only on CTF/EMPD. The main
history traversal then increments each such surface's counter and sets
`SumTime = SurfCurrNumHist * TimeStepZone`. When the counter reaches the
declared construction's `NumHistories`, it resets the counter, shifts both
master and working temperature/flux histories through `NumCTFTerms + 1`, and seeds master
and working term 2 from the captured first samples. Otherwise it interpolates
the working histories between master terms and those captured samples using
the exact `SumTime / CTFTimeStep` fraction.

When `AnyInternalHeatSourceInInput` is true, the final traversal restricts the
same topology to CTF/EMPD constructions with `SourceSinkPresent`, again without
a `NumCTFTerms == 0` skip. It observes the counter already updated by the main
history pass: counter zero performs
the analogous source/user-temperature and source-flux master rollover and term
2 seed, while a nonzero counter interpolates those working histories with the
same fraction. `UpdateThermalHistories` calls no named EnergyPlus child
routine and emits no warning, severe, fatal, or other diagnostic; its effects
are the allocations, current report/source state, counters, and history-array
mutations above.

Rust already lists `update_thermal_histories_stage` in the Surface-manager
algorithm and enters `update_thermal_histories_source_order_path` around a
bounded retained-surface vector-history snapshot/push lane, including several
diagnostic outside-temperature override variants. CP126 does not change that
target. The lane does not implement the parent `AnyCTF || AnyEMPD` call gate,
complete Zone/Space/opaque topology and declared-construction selection,
one-time scratch lifecycle, current source/report assignments, `SimpleCTFOnly` array
recycling, per-construction `NumHistories` counters and `SumTime`, master
rollover/interpolation, or internal source/user histories. Existing narrow CTF
numerics and report evidence therefore remain unchanged and do not establish
full routine parity.

CP126 adds one required `source_mapped` routine and project-contract entry,
without an EnergyPlus source-file addition, Rust target or code edit, mapped
state, capability, support-gate admission, manifest, comparator, proof
variable, result, family gate, output, or conformance promotion. The inventory
becomes 32 algorithms and 140 routines, split into 58 `state_mapped` and 82
`source_mapped` routines. The following CP127 section maps the parent lines
191-206 `AnyCondFD` guard and inline filtered calls to
`SurfaceDataFD::UpdateMoistureBalance`.

### CP127 CondFD moisture-history update source map

After the CP126 `AnyCTF || AnyEMPD` block closes,
`HeatBalanceSurfaceManager.cc::ManageSurfaceHeatBalance` lines 191-206 use a
separate `if (state.dataHeatBal->AnyCondFD)` condition. It is not nested under
the thermal-history guard: any CTF/EMPD result leaves this CondFD decision
independent. A false CondFD guard skips the complete loop, while a true guard
visits every numeric `SurfNum` from 1 through `TotSurfaces` in complete Surface
array order. After the block, line 208 unconditionally calls the CP128-mapped
`ThermalComfort::ManageThermalComfort(state, false)` routine.

For each Surface, the parent reads the declared `surface.Construction` into
`ConstrNum`. It skips exactly these three cases, in this order:

1. `ConstrNum <= 0`, before any Construction-array lookup;
2. `Construct(ConstrNum).TypeIsWindow`; and
3. `surface.HeatTransferAlgorithm != CondFD`.

Every survivor invokes
`state.dataHeatBalFiniteDiffMgr->SurfaceFD(SurfNum).UpdateMoistureBalance()`
with no arguments. The loop adds no representative-surface, Zone/Space,
surface-class, heat-transfer flag, construction-history-term, source/sink, or
other filter. Its complete numeric ordering and selection remain inline
`ManageSurfaceHeatBalance` ownership; CP127 does not invent a separate parent
loop routine.

The invoked inline member is defined entirely in
`HeatBalFiniteDiffManager.hh` lines 175-182. It performs exactly three
whole-array assignments on the selected `SurfaceDataFD` instance, in this
order: `TOld = T`, `RhovOld = Rhov`, and `TDOld = TDreport`. Thus the current
temperature and vapor-density node arrays become their old snapshots, while
the old `TD` snapshot comes from the reported `TDreport` array rather than
from `TD`. The helper has no arguments, branch, return value, diagnostic, or
named child call.

`HeatBalFiniteDiffManager.cc::InitHeatBalFiniteDiff` supplies later dependency
context at lines 549-553: its every-timestep initialization assigns `T = TOld`,
`Rhov = RhovOld`, and `TD = TDOld`, then separately sets `TDT = TDreport` and
`TDTLast = TDOld`. Those restoration/current-iteration assignments are not
effects of `UpdateMoistureBalance`. The inline helper itself does not mutate
`T`, `Rhov`, `TD`, `TDT`, `TDTLast`, enthalpy, phase-change, boundary,
reporting, or any other FiniteDiff state beyond its three destination arrays.
The `.cc` implementation remains dependency context only and is not added to
this algorithm's source inventory or as another routine row.

CP127 adds `HeatBalFiniteDiffManager.hh` to the Surface-manager algorithm's
EnergyPlus source list and adds non-required `source_mapped`
`routine.surface_data_fd_update_moisture_balance`. It adds no project-contract
routine, Rust target or code, mapped state, capability, support-gate admission,
manifest, comparator, proof variable, result, output, numerical, performance,
or conformance promotion. Existing Surface-manager and CTF-history stage names
do not implement or promote this CondFD moisture-history loop or helper.

The inventory becomes 32 algorithms and 141 routines, split into 58
`state_mapped` and 83 `source_mapped` routines; the required-routine total
remains 47. The following CP128 section maps the unconditional parent line-208
`ThermalComfort::ManageThermalComfort(state, false)` call.

### CP128 `ManageThermalComfort(state, false)` source map

After the independent CP127 CondFD block closes,
`HeatBalanceSurfaceManager.cc::ManageSurfaceHeatBalance` line 208
unconditionally calls
`ThermalComfort::ManageThermalComfort(state, false)`. The parent supplies no
People, comfort-model, first-time, sizing, warmup, or environment guard. Every
parent invocation that reaches this point enters the manager after any CondFD
moisture snapshots and before the line-210 `ReportSurfaceHeatBalance(state)`
call. The canonical manager body is `ThermalComfort.cc` lines 105-164.

The manager first observes the shared ThermalComfort lifecycle flag.
`ThermalComfortsData::FirstTimeFlag` defaults true in `ThermalComfort.hh` line
253, and `clear_state()` placement-news a fresh `ThermalComfortsData` at lines
387-390, recreating that default. When the flag is true, manager lines 112-115
call `InitThermalComfort(state)` and clear the flag only after that call
returns. The literal `false` at this Surface-manager call site does not skip
that initialization; it only prevents the later `InitializeOnly` early
return.

`InitThermalComfort` spans lines 166-497. It allocates one
`ThermalComfortData` entry per People object and registers the People-keyed
Fanger, Pierce, KSU, shared Fanger/Pierce/KSU, adaptive ASHRAE 55, adaptive CEN
15251, cooling-effect, and ankle-draft outputs under their exact per-People
model gates. It then allocates per-Zone ASHRAE 55 state, propagates each
People object's `Show55Warning` to its Zone, registers the Zone and Facility
simple-model outputs, allocates per-Zone setpoint-not-met state, and registers
the Zone and Facility setpoint-not-met outputs. Finally it calls
`GetAngleFactorList(state)` and dimensions `ZoneOccHrs` over `NumOfZones` with
zeroes. These setup and dependency operations are initialization context, not
separate CP128 routine rows.

`ManageThermalComfort` and `InitThermalComfort` contain no direct warning,
severe, or fatal emission statements. Their output-registration,
`GetAngleFactorList`, and later comfort-calculation dependencies can emit
diagnostics, so CP128 does not claim a diagnostic-free subtree. In particular,
the manager invokes `CalcThermalComfortFanger(state)` without its optional
`PNum`, `Tset`, or `PMVResult` arguments. The Fanger air-velocity warning at
lines 633-649 is guarded by `present(PNum)` and is therefore unreachable from
this call, while other diagnostics in Fanger and the other comfort children
remain dependency behavior.

After possible first-time setup, lines 117-131 maintain
`TemporarySixAMTemperature` before testing `InitializeOnly`. On simulation day
1 while `HourOfDay < 7`, every manager invocation repeats the exact
`1.868132` assignment. On day 1 at `HourOfDay == 7 && TimeStep == 1`, it
instead captures `OutDryBulbTemp`; on every later simulation day, that same
hour-7, timestep-1 condition captures `OutDryBulbTemp`. All other day/hour/
timestep combinations leave the value unchanged. The field name describes a
six-AM temperature, but CP128 preserves these exact source predicates rather
than translating them to an `HourOfDay == 6` test.

Lines 133-135 return only when `InitializeOnly` is true. The line-208 caller's
false argument therefore continues to the `BeginEnvrnFlag` test, which resets
the complete `ZoneOccHrs` array to zero at lines 137-139, and then to the
calculation gate. Only when both `DoingSizing` and `WarmupFlag` are false do
lines 141-163 invoke the comfort children in this exact order:

1. always `CalcThermalComfortFanger(state)`, with no optional arguments;
2. `CalcThermalComfortPierceASHRAE(state)` only when
   `AnyThermalComfortPierceModel` is true;
3. `CalcThermalComfortKSU(state)` only when `AnyThermalComfortKSUModel` is
   true;
4. `CalcThermalComfortCoolingEffectASH(state)` only when
   `AnyThermalComfortCoolingEffectModel` is true;
5. `CalcThermalComfortAnkleDraftASH(state)` only when
   `AnyThermalComfortAnkleDraftModel` is true;
6. always `CalcThermalComfortSimpleASH55(state)`;
7. always `CalcIfSetPointMet(state)`;
8. `CalcThermalComfortAdaptiveASH55(state, false)` only when
   `AdaptiveComfortRequested_ASH55` is true; and
9. `CalcThermalComfortAdaptiveCEN15251(state, false)` only when
   `AdaptiveComfortRequested_CEN15251` is true.

The two adaptive-call false arguments mean `initiate = false`; they are
preserved independently of the manager's `InitializeOnly = false`. During
sizing or warmup, the complete nine-child block is skipped, but first-time
setup, six-AM-temperature maintenance, and the post-early-return
`BeginEnvrnFlag` reset have already occurred in source order.

Another production caller appears at
`ZoneTempPredictorCorrector.cc` lines 5986-5988. Under its own
`CalcZoneAirComfortSetPointsFirstTimeFlag`, it calls
`ManageThermalComfort(state, true)` and then clears its caller-local flag.
Whichever production caller reaches the shared ThermalComfort flag first can
therefore perform `InitThermalComfort`; the true caller still performs setup
and six-AM-temperature maintenance before returning ahead of the environment
reset and calculation block. This caller establishes lifecycle context only:
CP128 does not add `ZoneTempPredictorCorrector.cc` or its header to the Surface
algorithm's source inventory.

CP128 adds `ThermalComfort.cc` and `ThermalComfort.hh` to that inventory and
adds non-required `source_mapped` `routine.manage_thermal_comfort`. It adds no
rows or source inventory entries for `InitThermalComfort`, the nine comfort
children, or their dependencies, and adds no project-contract routine, Rust
target or code, mapped state, capability, support-gate admission, manifest,
comparator, proof variable, result, output, numerical, performance, or
conformance promotion. The inventory becomes 32 algorithms and 142 routines,
split into 58 `state_mapped` and 84 `source_mapped` routines; the
required-routine total remains 47. The following CP129 section maps the
unconditional parent line-210 `ReportSurfaceHeatBalance(state)` call and its
`HeatBalanceSurfaceManager.cc` lines 6605-6891 implementation.

### CP129 `ReportSurfaceHeatBalance` source map

Immediately after the CP128 thermal-comfort manager returns,
`HeatBalanceSurfaceManager.cc::ManageSurfaceHeatBalance` line 210
unconditionally calls `ReportSurfaceHeatBalance(state)`. There is no parent
sizing, warmup, reporting-request, representative-surface, or other guard at
this call site. The canonical body spans lines 6605-6891 and has no
routine-wide guard or early return, so every entered call reaches its ordered
unconditional work even when individual report blocks are disabled.

The first operation is the unconditional
`SolarShading::ReportSurfaceShading(state)` dependency, whose canonical body
is `SolarShading.cc` lines 11533-11619. It visits numeric Surface indices 1
through `TotSurfaces`, copies the current `HourOfDay`/`TimeStep` sunlit
fraction from the heat-balance array, and multiplies by Surface area for the
sunlit-area report. On March 21, June 21, or December 21, only hours 9, 12,
and 15 at `TimeStep == 4` select a predefined-report column; when one is
selected, a second complete-Surface pass writes the sunlit fraction for
Window-class Surfaces. The dependency remains context and does not add a
SolarShading source file or separate routine row to this Surface-manager
checkpoint.

When `UseRepresentativeSurfaceCalculations` is true, lines 6617-6619 invoke
the local lines 6893-6930 `ReportNonRepresentativeSurfaceResults` child. In
Zone -> Space -> `HTSurfaceFirst..HTSurfaceLast` order, each nonrepresentative
Surface copies the inside- and outside-convection report classes from its
`RepresentativeCalcSurfNum`. Under the child's independent
`DisplayAdvancedReportVariables` gate, its Window-range pass also copies the
representative Window convective gain, infrared gain, and shortwave-loss
reports. The coded scale at line 6918 is exactly
`surface.Area / Surface(surfNum).Area`: both operands refer to the same current
Surface, so it is ordinarily 1.0. CP129 preserves that self-area ratio and
does not reinterpret the denominator as the representative Surface's area.

The first main Zone -> Space -> `OpaqOrWinSurfaceFirst..Last` traversal then
converts prior per-area inside convection, inside net longwave, internal
radiant gain, radiant-HVAC, outside convection, and outside-radiation values
to Surface rates with `surface.Area`, and converts the corresponding rates to
energies with `TimeStepZoneSec`. It also computes
`SurfQAirExtReport = Area * SurfHAirExt * (SurfTempOut -
SurfOutDryBulbTemp)`. Because positive `SurfQdotConvOutRep` uses the opposite
direction convention, the heat-emission report is exactly
`SurfQAirExtReport - SurfQdotConvOutRep`.

Only when `displayHeatEmissionsSummary` is true do lines 6669-6677 reset
`SumSurfaceHeatEmission` to zero and traverse all numeric Surfaces, adding
`SurfQHeatEmiReport * TimeStepZoneSec` only for
`ExtBoundCond == ExternalEnvironment`. When that flag is false, this routine
does not reset or update the summary accumulator; the already calculated
per-Surface emission reports remain available.

The Window traversal next stores initial diffuse transmission times area and
selects the absorbed-shortwave layer count from the active model state in
this exact precedence: EQL uses `CFS(thisConstruct.EQLConsPtr).NL`; BSDF uses
the active construction's `TotSolidLayers`; an unshaded or
`SwitchableGlazing` Window uses the active construction's `TotGlassLayers`;
and every other active shade, screen, or blind uses the active shaded
construction's `TotGlassLayers`. After zeroing the three per-Window absorbed
shortwave report sums, the layer loop adds the initial diffuse absorbed,
inside total shortwave absorbed, and all-solid-layer shortwave absorbed terms,
each multiplied by Surface area.

For each Window, nonnegative `SurfWinHeatGain` writes the gain rate and gain
energy, while a negative value writes a positive loss rate and loss energy.
`SurfWinHeatTransferRepEnergy` always retains the signed heat-gain value times
`TimeStepZoneSec`. A Surface whose `OriginalClass` is `TDD_Diffuser` transfers
its current gain and loss reports to the referenced `TDDPipe`. The body does
not add another Window-model validation or clear the opposite per-Surface
gain/loss branch here; those report arrays and active construction/model
inputs are precomputed dependency state.

When `AnyMovableInsulation` is true, lines 6740-6742 invoke the local lines
6932-6940 `ReportIntMovInsInsideSurfTemp` child. It first copies the complete
`SurfTempIn` array into `SurfTempInMovInsRep`, then replaces only entries in
`intMovInsulSurfNums` whose current insulation is present with
`SurfTempInTmp`. A false guard skips both the complete-array copy and these
selected replacements.

The opaque reporting block uses Zone -> Space ->
`OpaqOrIntMassSurfaceFirst..Last` order and two passes per Space. Its first
pass stores outside absorbed shortwave, inside solar and lights radiation,
initial diffuse and total inside absorbed shortwave, and inside/outside face
conduction rates and energies. Each face-conduction rate clears both its gain
and loss report before placing a nonnegative value in gain or the magnitude
of a negative value in loss. The second pass derives average conduction as
`(inside - outside) / 2` for both rate and flux, derives storage as
`-(inside + outside)`, multiplies both rates by `TimeStepZoneSec` for energy,
and likewise clears then selects the corresponding gain or loss report.

The component-load snapshot runs only under the exact triple guard
`ZoneSizingCalc && CompLoadReportIsReq && !WarmupFlag`. It derives the current
timestep-in-day and selects the current overall sizing-day Surface array,
then records lights shortwave and fenestration solar sequences for each
opaque/internal-mass Surface. The following opaque-or-Window pass repeats an
inner `!WarmupFlag` test even though the outer guard already established it:
under pulse sizing it stores `loadConvectedWithPulse`; otherwise it stores
`loadConvectedNormal` and net Surface longwave. CP129 retains that redundant
test and does not collapse the three outer predicates.

Finally, `DisplayAdvancedReportVariables` gates the advanced Zone
accumulation. For every Space, the routine adds each opaque/internal-mass
Surface's inside and outside conduction rate to the Zone accumulators with
`+=`, then immediately updates that Zone's sign-selected gain/loss rates and
energies before advancing to the next Space. This routine performs no local
reset of those Zone accumulators and writes only the currently selected sign
branch. It depends on the earlier unconditional `InitSolarHeatGains` lines
2558-2575 reset of both Zone accumulators and both gain/loss report branches.
Without that cadence, re-entry can accumulate prior values, and a sign change
across successive Space updates can leave the previously selected opposite
branch stale; CP129 records rather than repairs those source semantics.

`ReportSurfaceHeatBalance`, `ReportNonRepresentativeSurfaceResults`,
`ReportIntMovInsInsideSurfTemp`, and `ReportSurfaceShading` contain no direct
warning, severe, fatal, or display diagnostic statements. Their predefined
table writer and all precomputed topology, construction, shading, solar,
temperature, flux, sizing, and report state remain dependencies whose wider
behavior is outside this routine row. The three child routines add no rows,
and `SolarShading.cc` or its header is not added to the Surface algorithm's
EnergyPlus source inventory.

Rust already lists `report_surface_heat_balance_stage` as a Surface-manager
target and uses `report_surface_heat_balance_source_order_path` as an identity
wrapper around bounded retained-Surface report/trace work in the run-period
path, with limited result-store reporting elsewhere. CP129 does not alter
that target or any Rust code. The current paths do not implement the exact
parent nesting, complete Zone/Space/Surface topology, shading and predefined
tables, representative projection, complete Window/EQL/BSDF/shading and TDD
state, movable insulation, emissions summary, sizing-day component loads, or
advanced accumulator/reset cadence, so they do not establish canonical
routine parity.

CP129 adds required `source_mapped`
`routine.report_surface_heat_balance` and its heat-balance project-contract
entry. `HeatBalanceSurfaceManager.cc` is already in the algorithm source
inventory, so no EnergyPlus source, Rust target or code, mapped state,
capability, support-gate admission, manifest, comparator, proof variable,
result, output, numerical, performance, or conformance promotion is added.
The inventory becomes 32 algorithms and 143 routines, split into 58
`state_mapped` and 85 `source_mapped` routines; the required-routine total
becomes 48. The following CP130 section maps parent lines 211-213, whose
`ZoneSizingCalc` guard calls
`OutputReportTabular::GatherComponentLoadsSurface(state)`, and the
canonical `OutputReportTabular.cc` lines 15064-15132 child body.

### CP130 `GatherComponentLoadsSurface` source map

After the CP129 Surface report returns,
`HeatBalanceSurfaceManager.cc::ManageSurfaceHeatBalance` lines 211-213 call
`OutputReportTabular::GatherComponentLoadsSurface(state)` only when
`ZoneSizingCalc` is true. A false parent guard skips the call and every child
effect. The canonical implementation is `OutputReportTabular.cc` lines
15064-15132, declared at `OutputReportTabular.hh` line 869.

An entered call still performs its complete body only under the independent
inner condition `CompLoadReportIsReq && !isPulseZoneSizing`. The effective
work condition is therefore
`ZoneSizingCalc && CompLoadReportIsReq && !isPulseZoneSizing`. There is no
`WarmupFlag` test. In particular, pulse Zone sizing reaches the call but is a
complete CP130 no-op, whereas CP129's preceding Surface-report body can store
`loadConvectedWithPulse` when its separate non-warmup component-load guard is
true. CP130 does not merge those distinct pulse semantics.

Inside the work gate, the routine derives
`timeStepInDayGCLS = (HourOfDay - 1) * TimeStepsInHour + TimeStep`, selects
`znCompLoads[CurOverallSimDay - 1]`, and then selects
`ts[timeStepInDayGCLS - 1]`. It visits every `spacezone` entry in that current
Zone day/timestep slice and resets only `feneCondInstantSeq` to zero. When
`doSpaceHeatBalanceSizing` is true, it selects the analogous `spCompLoads`
slice and resets only that field for every Space entry. A false Space-sizing
flag leaves the Space component-load arrays untouched; no other Zone or Space
component-load field is reset here.

The next loop visits every numeric Surface from 1 through `TotSurfaces`. It
skips exactly a Surface whose `Zone` is zero and then a Surface whose `Class`
is not `Window`. The source's controlled-Zone test is commented out, so it is
not an active filter. For every surviving Window it calculates the exact sum

```text
SurfWinGainConvGlazToZoneRep
+ SurfWinConvHeatFlowNatural
+ SurfWinGainConvShadeToZoneRep
+ SurfWinGainFrameDividerToZoneRep
```

and adds that value to the owning Zone's `feneCondInstantSeq`. Under Space
sizing it also reads the Window's `spaceNum` and adds the same value to
`spCompLoads...spacezone[spaceNum - 1]`. The routine performs no positive or
in-range validation of that Space index and relies on previously established
Surface/Zone/Space topology. Its immediate-solar calculation is only a source
comment: CP130 writes no `feneSolarInstantSeq` value.

After the Surface pass, the routine visits every Zone and adds tubular
daylighting convective gain to the same Zone sequence. The type selector is
the `OutputReportTabular.hh` line-255 singleton
`IntGainTypesTubularGCLS = {DaylightingDeviceTubular}`. The
`InternalHeatGains.cc` lines 8170-8213
`SumInternalConvectionGainsByTypes` dependency receives no explicit Space for
the Zone call, so it traverses that Zone's Space list and adds
`ConvectGainRate` only for matching device types. When Space sizing is true,
a final complete Space pass obtains each Space's `zoneNum`, calls the same
helper with the explicit Space index, and adds only that Space's matching
tubular gain to its sequence.

The routine assumes `AllocateLoadComponentArrays` lines 14856-14919 has
already sized the design-day/run-design-period by timestep Zone arrays and,
when enabled, the Space arrays. It likewise trusts `CurOverallSimDay`, the
hour/timestep indices, `NumOfZones`, `numSpaces`, Surface ownership and class,
the four precomputed Window convective report terms, Zone/Space topology, and
`spaceIntGainDevices`. CP129 and the preceding window and inside-balance work
provide report state used here, but none of those dependencies becomes a
separate CP130 claim.

Neither `GatherComponentLoadsSurface` nor
`SumInternalConvectionGainsByTypes` emits a direct warning, severe, fatal, or
display diagnostic. Neither validates the day/timestep or Zone/Space vector
bounds it indexes. The internal-gain helper remains dependency context only:
CP130 adds no helper row and does not add `InternalHeatGains.cc` or its header
to this algorithm's source inventory.

CP130 adds `OutputReportTabular.cc` and `OutputReportTabular.hh` to the
Surface-manager algorithm source inventory and adds non-required
`source_mapped` `routine.gather_component_loads_surface`. It adds no
project-contract entry, Rust target or code, mapped state, capability,
support-gate admission, manifest, comparator, proof variable, result, output,
numerical, performance, or conformance promotion. The inventory becomes 32
algorithms and 144 routines, split into 58 `state_mapped` and 86
`source_mapped` routines; the required-routine total remains 48. The following
CP131 section maps the unconditional parent line-215
`CalcThermalResilience(state)` call and its `HeatBalanceSurfaceManager.cc`
lines 5707-5799 implementation.

### CP131 `CalcThermalResilience` source map

After the CP130 branch closes, whether or not that branch called or performed
component-load work, `HeatBalanceSurfaceManager.cc::ManageSurfaceHeatBalance`
line 215 unconditionally calls `CalcThermalResilience(state)`. The canonical
body spans lines 5707-5799 and is declared at
`HeatBalanceSurfaceManager.hh` line 155. There is no parent or routine-wide
warmup, sizing, environment, occupancy, or reporting-request guard.

The routine shares the parent Surface manager's first-time lifecycle rather
than owning a separate flag. `ManageSurfaceHeatBalancefirstTime` defaults true
at `HeatBalanceSurfaceManager.hh` line 239 and is restored true by
`clear_state()` at line 291; `reportVarHeatIndex` and `reportVarHumidex`
default false at lines 244-245 and are restored false at lines 296-297. While
the shared flag is true, CP131 registers `Zone Heat Index` with units C and
`Zone Humidity Index` with units None as Zone-timestep Average variables for
every Zone, regardless of whether either variable was requested.

It then scans the complete `dataOutputProcessor->reqVars` list. Exact name
equality with `Zone Heat Index` sets `reportVarHeatIndex` true; the mutually
exclusive `else if` exact match with `Zone Humidity Index` sets
`reportVarHumidex` true. CP131 does not clear the shared first-time flag. Only
the successfully reached `ManageSurfaceHeatBalance` tail at line 229 writes it
false. Consequently, an abnormal exit after this routine is reached but
before that tail can cause same-state re-entry to repeat both output
registration and request scanning.

The Zone `Resilience` array is dependency-owned rather than allocated here.
`HeatBalanceManager.cc` lines 1853-1856 always allocate it with
`NumOfZones` during Zone input because the two output variables require the
structure; its Heat Index and Humidex values have zero defaults. CP131 does
not allocate or globally reset the array and overwrites only whichever
calculation is active. An inactive calculation therefore retains its prior or
default value.

Heat Index has the independent gate
`reportVarHeatIndex || displayThermalResilienceSummary`. For every Zone it
reads ordinary Zone-average `ZTAV` and `airHumRatAvg`, not the comfort-only
state, computes relative humidity as
`PsyRhFnTdbWPb(ZoneT, ZoneW, OutBaroPress) * 100`, and converts temperature to
`ZoneTF = ZoneT * 9 / 5 + 32`. When `heatIndexMethod == Simplified`, a
Fahrenheit temperature below 80 uses exactly

```text
HI_F = 0.5 * (ZoneTF + 61
              + 1.2 * (ZoneTF - 68)
              + 0.094 * ZoneRH)
```

At 80 F or above it uses the nine-term Rothfusz expression

```text
HI_F = -42.379
       + 2.04901523 * T
       + 10.14333127 * RH
       - 0.22475541 * T * RH
       - 0.00683783 * T^2
       - 0.05481717 * RH^2
       + 0.00122874 * T^2 * RH
       + 0.00085282 * T * RH^2
       - 0.00000199 * T^2 * RH^2
```

and then subtracts `(13 - RH) / 4 * sqrt((17 - abs(T - 95)) / 17)` when
`RH < 13 && T < 112`; only the `else if` case `RH > 85 && T < 87` adds
`(RH - 85) / 10 * (87 - T) / 5`. Both simplified branches finally convert
with `(HI_F - 32) * 5 / 9` and store `ZoneHeatIndex` in C.

The source tests only equality with `Simplified`, so every other enum value
takes the extended path. That path calls
`ExtendedHI::heatindex(state, ZoneT + Kelvin, ZoneRH / 100) - Kelvin`, using
Kelvin temperature and fractional relative humidity. The
`ExtendedHeatIndex.cc` lines 538-557 dependency temporarily replaces the
global root algorithm with `ShortBisectionThenRegulaFalsi`, performs its
extended calculations and root solves, and restores the saved algorithm
before returning. It is dependency context rather than another CP131 routine
or source entry.

Humidex has its own independent gate
`reportVarHumidex || displayThermalResilienceSummary`. For every Zone it uses
the same ordinary `airHumRatAvg`, `ZTAV`, and outdoor barometric pressure and
computes exactly

```text
TDewPointK = PsyTdpFnWPb(ZoneW, OutBaroPress) + Kelvin
e = 6.11 * exp(5417.7530 * (1 / 273.16 - 1 / TDewPointK))
h = 5 / 9 * (e - 10)
ZoneHumidex = ZoneT + h
```

Although this result is temperature-based, its registered `Zone Humidity
Index` unit is exactly None. Enabling `displayThermalResilienceSummary`
activates both the Heat Index and Humidex passes even when neither output
variable was requested; otherwise each false request/summary gate leaves that
quantity unchanged.

`CalcThermalResilience` contains no direct warning, severe, fatal, or display
diagnostic statement. Output registration, Psychrometrics, ExtendedHI, and
the root solver remain dependencies. The psychrometric functions bound very
small humidity ratios and relative humidity, and diagnostic-enabled
psychrometric paths can report large range errors; ExtendedHI does not return
its root-solver flags through CP131. These deeper behaviors add no routine row
or source inventory entry for ExtendedHI, Psychrometrics, or DataHeatBalance.

CP131 adds non-required `source_mapped`
`routine.calc_thermal_resilience`. Its implementation and lifecycle header are
already in the Surface-manager source inventory, so it adds no EnergyPlus
source, project-contract entry, Rust target or code, mapped state, capability,
support-gate admission, manifest, comparator, proof variable, result, output,
numerical, performance, or conformance promotion. The inventory becomes 32
algorithms and 145 routines, split into 58 `state_mapped` and 87
`source_mapped` routines; the required-routine total remains 48. The following
CP132 section maps the parent lines 217-219
`displayThermalResilienceSummary` guard and its
`ReportThermalResilience(state)` call, whose canonical body spans
`HeatBalanceSurfaceManager.cc` lines 5801-6388.

### CP132 `ReportThermalResilience` source map

After CP131 returns, `HeatBalanceSurfaceManager.cc::ManageSurfaceHeatBalance`
lines 217-219 independently test `displayThermalResilienceSummary` and call
`ReportThermalResilience(state)` only when it is true. A false guard skips every
CP132 effect. The call precedes the separate CO2- and visual-resilience guards.
The canonical body spans lines 5801-6388 and is declared at
`HeatBalanceSurfaceManager.hh` line 157.

The body first creates its active thermal-report-period flags. It dimensions
that array only under `TotReportPers > 0`, but the dimension and all following
loops use `TotThermalReportPers`. For every active thermal period it reads
`meterNumTotalsBEPS(1)` and adds `GetCurrentMeterValue` to that period's
`totalElectricityUse`. This electricity accumulation occurs before the
routine's first-time block and before the RunPeriodWeather/non-warmup gate, so
an entered sizing, warmup, or other-environment call can still add it.

The routine owns a distinct `reportThermalResilienceFirstTime` flag, defaulted
true at `HeatBalanceSurfaceManager.hh` line 243 and restored true by
`clear_state()` at line 295. On the first entered call it establishes exact
array widths of five for Heat Index, Humidex, low/high SET, cold safety, and
heat safety, six for unmet degree hours, and four for each PMV-discomfort
array. It zeros every Zone/thermal-period array plus the period and annual
low/high-SET longest-duration and start trackers, then clears the flag. If
there are no People, or if any People object does not request Pierce comfort,
`hasPierceSET` becomes false and suppresses SET arrays and SET accumulation for
all Zones. The annual crossing flags default false with each Zone Resilience
record, while the report-period crossing arrays are allocated and cleared by
`HeatBalanceManager`; CP132 itself resets neither family. There is no
BeginEnvrn reset, so first-time state, annual bins, crossings, and longest-event
state can span multiple RunPeriodWeather environments in one EnergyPlus state.

All remaining aggregation runs only when
`KindOfSim == RunPeriodWeather && !WarmupFlag`, using `TimeStepZone` directly;
the routine performs no hourly or daily reset. At each entered Zone timestep it
sets only the per-Zone PierceSET and PMV comparison scratch values to -999.
The People loop then writes, rather than sums,
`ZoneNumOcc = NumberOfPeople * schedule value`; the last People object for a
Zone wins, and a Zone untouched by the loop is not reset and can retain stale
occupancy. It also copies `ZonePierceSET` into `ZonePierceSETLastStep` before
each People update, so multiple People objects can make the apparent prior
step be an earlier People object in the same timestep.

For each People object, cold is safe only when Zone `ZTAV` is strictly greater
than that object's cold-stress threshold and dangerous at equality or below;
heat is safe only when strictly below the heat-stress threshold and dangerous
at equality or above. Each five-slot safety vector stores safe hours before
the first crossing, the encoded first-crossing timestamp, dangerous hours,
dangerous occupant-hours, and dangerous occupied-hours. Normal crossings use
minute `TimeStepZone * (TimeStep - 1) * 60`. Annual and every active-period
copy repeat this work inside the People loop, so raw Zone hours as well as
occupant/occupied contributions can be duplicated when a Zone has multiple
People objects.

PMV degree-hour bins use four independent tests at `< -3`, `< -0.7`, `> 0.7`,
and `> 3`. Because these are not an `else if` chain, an extreme PMV contributes
both to the extreme bin and its adjacent cool or warm bin. The same per-People
duplication and active-period repetition apply. Exact floating inequality,
with no tolerance, compares PierceSET and PMV across People in a Zone. The only
two direct diagnostics are recurring warnings with exact messages
`Zone {} has multiple people objects with different PierceSet.` and
`Zone {} has multiple people objects with different PMV.`; each warning family
uses one global recurring index shared across Zones. CP132 contains no direct
severe, fatal, or display diagnostic.

The following Zone loop bins the CP131 values with exact Heat Index boundaries
`<= 26.7`, `(26.7, 32.2]`, `(32.2, 39.4]`, `(39.4, 51.7]`, and `> 51.7`, and
Humidex boundaries `<= 29`, `(29, 40]`, `(40, 45]`, `(45, 50]`, and `> 50`.
Each family accumulates elapsed hours, occupant-hours, and occupied-hours from
the potentially overwritten or stale Zone occupancy. Cooling and heating
unmet degree hours require a positive respective setpoint and accumulate raw,
person-weighted, and occupied variants.

When global `hasPierceSET` remains true, SET at or below 12.2 C accumulates
three low degree-hour variants and SET above 30 C accumulates the analogous
high variants. Annual and report-period trackers retain the longest occupied
low/high excursion duration and encoded start, and zero a continuing duration
when occupancy is zero. A normal new excursion uses minute
`(TimeStep - 1) * TimeStepZone * 60`. If a low-SET excursion is already in
progress at the beginning of an active report period, the continuation start
instead uses `TimeStepZone * TimeStep * 60`; source lines 6314-6315 then
erroneously reset that period's **high**-SET longest duration and start rather
than the low-SET trackers. Exact SET comparisons and this defect are part of
the mapped boundary. Every active thermal period repeats the Heat Index,
Humidex, SET, and unmet-degree-hour accumulation, so overlapping periods all
accrue independently.

The routine writes no table directly; its report payload remains accumulated
state. Annual
`WriteThermalResilienceTables` and report-period
`WriteThermalResilienceTablesRepPeriod` in `OutputReportTabular.cc` later
consume it, including period `totalElectricityUse` in their time/consumption
presentation; their table layout and any dependency-level request warning are
context, not CP132 child rows or direct diagnostics. `findReportPeriodIdx`,
`EncodeMonDayHrMin`, `isReportPeriodBeginning`, meter access, People schedules,
ThermalComfort PierceSET/FangerPMV, Zone temperatures/setpoints, and CP131 Heat
Index/Humidex values likewise remain dependencies.

CP132 adds non-required `source_mapped`
`routine.report_thermal_resilience`. Its implementation, declaration, and
table-writer dependencies are already represented by existing source
inventories, so it adds no EnergyPlus source, project-contract entry, Rust
target or code, mapped state, capability, support-gate admission, manifest,
comparator, proof variable, result, output, numerical, performance, or
conformance promotion. The inventory becomes 32 algorithms and 146 routines,
split into 58 `state_mapped` and 88 `source_mapped` routines; the
required-routine total remains 48. The following CP133 section maps parent
lines 221-223 `displayCO2ResilienceSummary` guard and
`ReportCO2Resilience(state)`, whose canonical body spans
`HeatBalanceSurfaceManager.cc` lines 6390-6479.

### CP133 `ReportCO2Resilience` source map

After CP132's independent guard, `ManageSurfaceHeatBalance` lines 221-223
independently test `displayCO2ResilienceSummary` and call
`ReportCO2Resilience(state)` only when it is true. A false guard skips every
CP133 effect. The call precedes the separate visual-resilience guard and the
parent's final first-time-flag clear. The canonical body spans
`HeatBalanceSurfaceManager.cc` lines 6390-6479 and is declared at
`HeatBalanceSurfaceManager.hh` line 159.

The routine owns `reportCO2ResilienceFirstTime`, defaulted true at header line
247 and restored true by `clear_state()` at line 299. On the first entered
call it gives each Zone and each CO2 report period three-bin elapsed-hour,
occupant-hour, and occupied-hour vectors and clears the flag. The enclosing
Zone-by-period arrays were allocated only when `TotCO2ReportPers > 0` by
`HeatBalanceManager.cc` lines 2940-2944; annual three-bin arrays instead have
zero member defaults in each Zone Resilience record. CP133 contains no
BeginEnvrn reset, so its first-time state and accumulated annual/period values
can span multiple RunPeriodWeather environments in one EnergyPlus state. A
first entered call can occur during sizing or warmup because initialization
precedes the later weather/non-warmup gate.

Still inside the first-time block, a false
`Contaminant.CO2Simulation` condition emits the routine's only direct
diagnostic, and only when `displayCO2ResilienceSummaryExplicitly` is true: a
warning that the Zone Air CO2 Concentration output is required but no
`ZoneAirContaminantBalance` object is defined. The routine then sets
`displayCO2ResilienceSummary = false` and returns. The parent continues to its
visual guard and tail; subsequent parent calls skip CP133 entirely. Thus the
three-bin period initialization can occur before CO2 absence permanently
disables this report.

All remaining work requires
`KindOfSim == RunPeriodWeather && !WarmupFlag`, uses `TimeStepZone` directly,
and has no hourly or daily reset. Before any Zone binning, the People loop
writes rather than sums `ZoneNumOcc = NumberOfPeople * schedule value`; the
last People object for a Zone wins, and a Zone untouched by the loop is not
reset and can retain stale occupancy. This shared Resilience field is the same
one used by CP132.

The active-period flag array is declared only inside this weather/non-warmup
gate. Its allocation is guarded by `TotReportPers > 0`, while the dimension,
lookup, and following loops use `TotCO2ReportPers`. Every active CO2 period
then reads `meterNumTotalsBEPS(1)` and adds `GetCurrentMeterValue` to that
period's `totalElectricityUse`. The value is added once per entered Zone
timestep with no `TimeStepZone` multiplication and no environment reset;
overlapping periods each receive the same current meter value. This ordering
differs from CP132, whose corresponding electricity accumulation precedes its
weather/warmup gate.

For each Zone, `ZoneAirCO2Avg <= 1000` selects bin zero, values greater than
1000 and at most 5000 select bin one, and every remaining value selects bin
two. The selected annual vector entries add elapsed `TimeStepZone`,
`ZoneNumOcc * TimeStepZone`, and `static_cast<Real64>(ZoneNumOcc > 0) *
TimeStepZone`; they do not multiply by the concentration exceedance. Every
active CO2 period repeats the same exact test and three additions into its
period vectors, so overlapping periods accrue independently.

The routine writes no table itself. Annual `WriteCO2ResilienceTables` can
consume the annual bins. Report-period
`WriteCO2ResilienceTablesRepPeriod` at `OutputReportTabular.cc` lines
13686-13753 is called only inside the outer `WriteTabularFiles` true block,
but its lines 13689-13692 inverted guard returns when `WriteTabularFiles` is
true. Its period CO2 bin tables and per-period header are therefore
unreachable in the production call path. The separate generic Reporting
Period Time and Consumption table remains reachable and can display the
accumulated CO2-period electricity after dividing joules by 3,600,000 for
kWh. This writer defect and table layout are mapped dependency context, not a
child row or Rust/output claim.

`findReportPeriodIdx`, meter access, People schedules, ContaminantBalance
state, Zone-average CO2, annual/period array allocation, and the tabular
writers remain dependencies. Apart from the explicit no-CO2 warning above,
the body emits no recurring warning, severe, fatal, or display diagnostic.

CP133 adds non-required `source_mapped`
`routine.report_co2_resilience`. Its implementation, declaration, and writer
dependencies are already represented by existing source inventories, so it
adds no EnergyPlus source, project-contract entry, Rust target or code, mapped
state, capability, support-gate admission, manifest, comparator, proof
variable, result, output, numerical, performance, or conformance promotion.
The inventory becomes 32 algorithms and 147 routines, split into 58
`state_mapped` and 89 `source_mapped` routines; the required-routine total
remains 48. The following CP134 section maps parent lines 225-227
`displayVisualResilienceSummary` guard and `ReportVisualResilience(state)`,
whose canonical body spans `HeatBalanceSurfaceManager.cc` lines 6481-6603.

### CP134 `ReportVisualResilience` source map

After CP133's independent guard, `ManageSurfaceHeatBalance` lines 225-227
independently test `displayVisualResilienceSummary` and call
`ReportVisualResilience(state)` only when it is true. A false guard skips every
CP134 effect. This is the parent's final child call before its line-229
first-time-flag clear. The canonical body spans
`HeatBalanceSurfaceManager.cc` lines 6481-6603 and is declared at
`HeatBalanceSurfaceManager.hh` line 161.

The routine owns `reportVisualResilienceFirstTime`, defaulted true at header
line 248 and restored true by `clear_state()` at line 300. On the first entered
call it gives every Zone and visual report period four-bin elapsed-hour,
occupant-hour, and occupied-hour vectors, then clears the flag. The enclosing
Zone-by-period arrays were allocated only when `TotVisualReportPers > 0` by
`HeatBalanceManager.cc` lines 2946-2952; annual four-bin arrays instead have
zero member defaults in each Zone Resilience record. CP134 contains no
BeginEnvrn reset, so its first-time state and accumulated annual/period values
can span multiple RunPeriodWeather environments in one EnergyPlus state. A
first entered call can occur during sizing or warmup because initialization
precedes the later weather/non-warmup gate.

Still inside the first-time block, an empty global `daylightControl` arena
emits the routine's only direct diagnostic, and only when
`displayVisualResilienceSummaryExplicitly` is true: a warning that Zone
Average Daylighting Reference Point Illuminance output is required but no
Daylighting Control object is defined. The routine then sets
`displayVisualResilienceSummary = false` and returns. The parent continues to
its line-229 tail; subsequent parent calls skip CP134 entirely. Thus
four-bin period initialization can occur before global daylight-control
absence permanently disables this report.

All remaining work requires
`KindOfSim == RunPeriodWeather && !WarmupFlag`, uses `TimeStepZone` directly,
and has no hourly or daily reset. The People loop first writes rather than sums
`ZoneNumOcc = NumberOfPeople * schedule value`; the last People object for a
Zone wins, and a Zone untouched by the loop is not reset and can retain stale
occupancy. The routine then resets `zoneAvgIllumSum` to zero for every Zone on
each eligible call and accumulates every daylight control in control-array
order. When a control's `PowerReductionFactor > 0`, each declared reference
point contributes its `illumSetPoint`; otherwise it contributes
`lums[(int)DataSurfaces::Lum::Illum]`. The factor is only a branch condition,
not a multiplier. Multiple controls targeting one Zone all add to the same
sum.

The active-period flag array is declared inside the weather/non-warmup gate.
Its allocation is guarded by generic `TotReportPers > 0`, while its dimension,
`findReportPeriodIdx` lookup, and following loops use `TotVisualReportPers`.
After occupancy and illuminance accumulation, every active visual period reads
`meterNumTotalsBEPS(1)` and adds `GetCurrentMeterValue` to that period's
`totalElectricityUse`. The value is added once per entered Zone timestep with
no `TimeStepZone` multiplication and no environment reset; overlapping periods
each receive the same current meter value. This work occurs before the later
per-Zone zero-reference-point skip.

Each Zone with `ZoneDaylight.totRefPts == 0` is skipped without annual or
period-bin accumulation. Otherwise `zoneAvgIllumSum` is divided by that
Zone-level reference-point count. Average illuminance at most 100 lux selects
bin zero, values greater than 100 and at most 300 select bin one, values
greater than 300 and at most 500 select bin two, and every remaining value
selects bin three. The selected annual entries add elapsed `TimeStepZone`,
`ZoneNumOcc * TimeStepZone`, and
`static_cast<Real64>(ZoneNumOcc > 0) * TimeStepZone`; every active visual
period repeats the same exact test and three additions, so overlapping periods
accrue independently.

The routine writes no table itself. Annual `WriteVisualResilienceTables` is
called only while `displayVisualResilienceSummary` remains true and pure-load
calculation is false. That writer issues an explicit-request-only dependency
warning for each Zone whose `totRefPts` is zero, but after a positive Zone
count it still writes all three annual tables, including zero bins for skipped
Zones. Report-period `WriteVisualResilienceTablesRepPeriod` at
`OutputReportTabular.cc` lines 13798-13866 has the correct
`if (!WriteTabularFiles) return` guard. The production tabular path calls it
for every visual period regardless of the display flag, so after the
first-call global no-control disable it can still emit initialized all-zero
period bin tables and a zero-electricity period header. The separate generic
Reporting Period Time and Consumption table can likewise show accumulated
visual-period electricity after dividing joules by 3,600,000 for kWh. These
writer behaviors and table layouts are dependency context, not child rows or
Rust/output claims.

`findReportPeriodIdx`, meter access, People schedules, daylight-control and
reference-point state, Zone daylight aggregates, annual/period allocation,
and tabular writers remain dependencies. Apart from the explicit global
no-control warning above, the body emits no recurring warning, severe, fatal,
or display diagnostic; per-Zone writer warnings are not body diagnostics.

CP134 adds non-required `source_mapped`
`routine.report_visual_resilience`. Its implementation, declaration, and
writer dependencies are already represented by existing source inventories,
so it adds no EnergyPlus source, project-contract entry, Rust target or code,
mapped state, capability, support-gate admission, manifest, comparator, proof
variable, result, output, numerical, performance, or conformance promotion.
The inventory becomes 32 algorithms and 148 routines, split into 58
`state_mapped` and 90 `source_mapped` routines; the required-routine total
remains 48. The following CP135 section maps only the parent-tail line-229
`ManageSurfaceHeatBalancefirstTime = false` assignment, without a synthetic
routine row.

### CP135 shared Surface-manager first-time tail map

`HeatBalanceSurfaceManager.cc::ManageSurfaceHeatBalance` reaches the
unconditional line-229 assignment
`ManageSurfaceHeatBalancefirstTime = false` only after the CP134 visual guard
and every earlier entered child have returned. The member defaults true at
`HeatBalanceSurfaceManager.hh` line 239 and `clear_state()` restores true at
line 291. The source has no BeginEnvrn or per-environment reset for this shared
flag.

Its only reads are the four parent progress-display guards at lines 158, 165,
169, and 176 plus the `CalcThermalResilience` first-time block at line 5721;
the line-229 assignment is its only false write. On a normal first call, the
source therefore displays `Initializing Surfaces`, returns from
`InitSurfaceHeatBalance`, displays and runs the outside balance, displays and
runs the inside balance, displays and runs the Air balance, completes the
remaining final/history/comfort/reporting/resilience order, lets
`CalcThermalResilience` register its two Zone outputs and scan exact requests,
passes the three resilience guards, and only then clears the shared flag.

The CP133 no-CO2 and CP134 no-daylight-controls early returns are local to
their child routines. They can clear their own first-time flags, disable their
own display flags, and return, but control resumes in
`ManageSurfaceHeatBalance`, so an otherwise successful parent call still
reaches line 229. In contrast, a fatal path, exception, or other non-return in
any earlier entered child prevents the parent-tail assignment and preserves
the shared flag as true. A same-state re-entry can then repeat all four
progress messages and the CP131 output-registration/request scan even when one
or more child-local first-time flags have already become false.

After any later call successfully reaches the tail, subsequent calls skip
only those five shared-first-time effects: the four progress displays and the
`CalcThermalResilience` registration/request scan. The parent still executes
its normal children and the non-first-time calculation/report work under their
own guards. The inline assignment calls no child and emits no diagnostic; its
complete mapped state is the single source bool and its default/reset/tail
lifecycle.

CP135 adds no synthetic routine, EnergyPlus source, project-contract entry,
Rust target or code, mapped Rust state, capability, support-gate admission,
manifest, comparator, proof variable, result, output, numerical, performance,
or conformance promotion. The inventory remains 32 algorithms and 148
routines, split into 58 `state_mapped` and 90 `source_mapped` routines; the
required-routine total remains 48. The following CP136 section maps the
unconditional `HeatBalanceManager.cc` line-210
`ManageEMS(state, EMSCallFrom::EndZoneTimestepBeforeZoneReporting, anyRan,
absent)` caller checkpoint by reusing `routine.manage_ems`.

### CP136 pre-zone-reporting EMS calling-point map

After `ManageSurfaceHeatBalance(state)` successfully returns at
`HeatBalanceManager.cc` line 209, line 210 unconditionally calls
`EMSManager::ManageEMS(state,
EMSCallFrom::EndZoneTimestepBeforeZoneReporting, anyRan,
ObjexxFCL::Optional_int_const())`. The fourth argument is an explicitly absent
program-manager index. There is no caller guard. The call immediately precedes
`RecKeepHeatBalance(state)` at line 211.

The caller passes the same `anyRan` local declared at line 189 and previously
passed to the CP113 and CP115 calling points. Canonical `ManageEMS` begins by
writing its referenced `anyProgramRan` argument false at `EMSManager.cc` line
263, so CP136 overwrites the CP115 result rather than accumulating it. The
caller never branches on or otherwise consumes the resulting value.

CP113's existing non-required `routine.manage_ems` row covers the generic
`EMSManager.cc` lines 248-374 body, declared at `EMSManager.hh` lines 99-103.
After resetting the run flag, a model with
`AnyEnergyManagementSystemInModel == false` returns immediately. Otherwise the
routine calls `InitEMS` for this calling point, dispatches registered callbacks
and plugins because this is not `UserDefinedComponentModel`, and visits every
program-call manager whose `CallingPoint` exactly equals
`EndZoneTimestepBeforeZoneReporting`, evaluating its Erl programs in stored
order. The absent optional manager index is inspected only by the separate
`UserDefinedComponentModel` path and is therefore unused at CP136.

A callback, plugin, or matching Erl program can set `anyProgramRan` true. If
none ran, the generic routine returns before actuator application and
`ReportEMS`; if any ran, it commits every usable EMS/external-interface
actuator value and then calls `ReportEMS`. The caller still ignores `anyRan`.
Because this calling point follows the complete Surface-manager solve and the
next statement is record keeping, EnergyPlus performs no intervening Surface
heat-balance re-solve before `RecKeepHeatBalance`; CP136 does not infer when a
late actuator mutation becomes numerically visible.

Complete EMS setup and initialization, callbacks, plugins, Erl managers and
programs, sensors, actuators, external interfaces, reporting, diagnostics,
failure behavior, and every state mutation remain dependency context already
bounded by `routine.manage_ems`. CP136 maps only this caller identity, exact
arguments, unconditional order, run-flag overwrite, and no-re-solve boundary.

CP136 adds no duplicate routine row, EnergyPlus source, project-contract
entry, Rust target or code, mapped Rust state, capability, support-gate
admission, manifest, comparator, proof variable, result, output, numerical,
performance, or conformance promotion. The inventory remains 32 algorithms
and 148 routines, split into 58 `state_mapped` and 90 `source_mapped` routines;
the required-routine total remains 48. The following CP137 section maps the
unconditional `RecKeepHeatBalance(state)` call at `HeatBalanceManager.cc` line
211, declared at `HeatBalanceManager.hh` line 134 and implemented at
`HeatBalanceManager.cc` lines 2971-3057.

### CP137 `RecKeepHeatBalance` source map

After the CP136 EMS call returns, `ManageHeatBalance` line 211 calls
`RecKeepHeatBalance(state)` unconditionally. The routine has no body-wide
guard and returns before the next unconditional `ReportHeatBalance(state)`
call at line 217. CP136 performs no intervening Surface re-solve; CP137 records
the Zone, demand, Surface, and Window state as it exists after that late EMS
calling point without claiming that any particular actuator mutation is
already visible in those values.

For every Zone in numeric order, source lines 2985-3007 compare `ZTAV` against
the stored maximum and minimum temperatures and compare raw
`airSysHeatRate` and raw `airSysCoolRate` against their respective stored
maximum loads. The cooling-extrema comparison does not apply absolute value.
The routine then shifts `TempZonePrevDay` into `TempZoneSecPrevDay` and
`LoadZonePrevDay` into `LoadZoneSecPrevDay`, shifts the prior current
temperature/load into the `PrevDay` slots, writes current `ZTAV`, and defines
the current load as
`max(airSysHeatRate, abs(airSysCoolRate))`.

Only under the exact gate `!WarmupFlag && DayOfSim == 1 && (!DoingSizing ||
DoPureLoadCalc)` does the routine calculate its detailed warmup-report
samples. The temperature and load differences are absolute differences
between the just-shifted second-previous and previous values; the newly written
current values do not enter those two differences. Zone 1 alone increments
`CountWarmupDayPoints`, after which every Zone writes that shared index in
`TempZoneRpt`, `LoadZoneRpt`, and `MaxLoadZoneRpt`, the last receiving the
current combined load. Those arrays are allocated as `NumOfZones` by
`TimeStepsInHour * 24`; the body performs no additional bounds check.

When `ReportDetailedWarmupConvergence` is true inside that gate, the body emits
EIO output. `FirstWarmupWrite` defaults true at `HeatBalanceManager.hh` line
190 and `clear_state()` restores true at line 245; `BeginEnvrn` does not reset
it. The first qualifying detailed sample writes the one header and clears the
flag, and every qualifying Zone sample writes Zone name, timestep, hour,
temperature difference, and load difference. This optional EIO stream is the
routine's only direct message; it emits no warning, recurring warning, severe,
or fatal diagnostic.

`AllocateHeatBalArrays` lines 2891-2909 dimension the extrema, history,
difference, and report arrays and initializes the report arrays to zero.
`InitHeatBalance`'s `BeginEnvrn` block at lines 2633-2652 resets previous-day
extrema, current extrema, current/previous temperature and load state,
temperature second-previous state, differences, report arrays, and the shared
count. It deliberately does not reset `LoadZoneSecPrevDay`; the first
`RecKeepHeatBalance` shift overwrites that slot from the reset
`LoadZonePrevDay` before any gated difference is calculated. This distinction,
and the lack of a per-environment `FirstWarmupWrite` reset, are part of the
mapped lifecycle.

After all Zones, an `AnyMovableInsulation` guard visits
`intMovInsulSurfNums` in stored order and snapshots each corresponding
`intMovInsuls(surfNum).present` into `presentPrevTS`. The routine then
unconditionally calls local `UpdateWindowFaceTempsNonBSDFWin`, whose body at
lines 3303-3313 traverses `AllHTWindowSurfaceList`, skips only constructions
with `WindowTypeBSDF`, writes layer 1 front temperature from
`SurfOutsideTempHist(1)(SurfNum)`, and writes the construction's `TotLayers`
back temperature from `SurfInsideTempHist(1)(SurfNum)`. No window report update
occurs for the skipped BSDF entries.

Existing Rust `rec_keep_heat_balance_stage`, its placement in the composite
heat-balance execution-plan list, Zone prebinding, and declared execution-plan
dependencies are compatibility scaffolding only. They do not implement the
complete source loop, extrema/history lifecycle, warmup arrays/EIO, movable
insulation snapshot, Window helper, exact source order, diagnostics, or
numerics, and are intentionally not added as a ledger Rust target.

CP137 adds required `source_mapped` `routine.rec_keep_heat_balance` and the
matching heat-balance project-contract requirement. `HeatBalanceManager.cc`
and `.hh` are already in the algorithm source inventory, so it adds no source
file, Rust target or code, mapped Rust state, capability, support-gate
admission, manifest, comparator, proof variable, result, numerical,
performance, or conformance promotion. The inventory becomes 32 algorithms
and 149 routines, split into 58 `state_mapped` and 91 `source_mapped` routines;
the required-routine total becomes 49. The following CP138 section maps the
unconditional `ReportHeatBalance(state)` call at `HeatBalanceManager.cc` line
217, declared at `HeatBalanceManager.hh` line 142 and implemented at
`HeatBalanceManager.cc` lines 3321-3418.

### CP138 `ReportHeatBalance` source map

After CP137 record keeping returns, `ManageHeatBalance` line 217 calls
`ReportHeatBalance(state)` unconditionally. The implementation at
`HeatBalanceManager.cc` lines 3321-3418 has no routine-wide guard or early
return. A successful call reaches the next unconditional
`EndZoneTimestepAfterZoneReporting` EMS calling point at parent line 219,
before `UpdateEMSTrendVariables(state)` and the plugin-value update at lines
221-222.

`Sched::ReportScheduleVals(state)` is always first, even when all later output
branches are inactive. Its own reporting setup and `UpdateScheduleVals` work
are dependency context; CP138 adds no Schedule-manager child row or source.
The body then selects exactly one of four remaining paths through one
`if`/`else if` chain.

The normal path requires the exact gate `!WarmupFlag && DoOutputReporting`.
It calls `Node::CalcMoreNodeInfo(state)` only when `!DoingSizing`, always calls
`UpdateDataandReport(state, TimeStepType::Zone)`, and updates the HVAC sizing
manager's Zone-step logs only when `KindOfSim` is `HVACSizeDesignDay` or
`HVACSizeRunPeriodDesign` and the manager object exists. It then, in order,
calls `UpdateTabularReports(state, TimeStepType::Zone)` and
`EconomicTariff::UpdateUtilityBills(state)`.

The second branch's written condition is `!KickOffSimulation &&
DoOutputReporting && ReportDuringWarmup`; because it is an `else if` after the
normal branch, its effective entered gate also requires `WarmupFlag`. At the
first `BeginDayFlag` call not already marked by
`PrintEnvrnStampWarmupPrinted`, it sets both warmup-stamp flags. A non-begin-day
call clears only the printed latch. When `PrintEnvrnStampWarmup` is set, the
routine optionally writes `End of Data Dictionary` to both ESO and MTR and
clears `PrintEndDataDictionary`, then writes the warmup environment stamp to
both streams and clears `PrintEnvrnStampWarmup`. It next performs the same
non-sizing-only node update, unconditional Zone-step `UpdateDataandReport`,
and guarded HVAC-sizing log update as the normal path. It does not call the
tabular or utility reporters.

If both reporting branches miss, the third branch tests only
`UpdateDataDuringWarmupExternalInterface`. Despite its name, it has no direct
`WarmupFlag` test. It calls Zone-step `UpdateDataandReport` and the same
simulation-kind/manager-guarded sizing-log update, but performs no node,
dictionary, environment-stamp, tabular, or utility work. If this fallback is
also false, no work follows the already completed schedule reporting.

The body contains no direct warning, recurring warning, severe, or fatal
diagnostic. ESO/MTR writes and every called reporting, schedule, node, sizing,
tabular, and utility subsystem own their diagnostics and failure behavior as
dependencies. A dependency that does not return prevents the parent from
reaching CP139; every successfully completed branch, including the no-further-
output path, reaches it. These children add no routine rows or EnergyPlus
source files at CP138.

Existing Rust `report_heat_balance_stage`, composite execution-plan placement,
output-handle prebindings, dependencies, and bounded result-store/reporting
work remain compatibility scaffolding. They do not implement the complete
source setup, branch order and gates, stamp/dictionary lifecycle, ESO/MTR,
node, sizing-log, tabular, utility, external-interface, diagnostic, failure,
or numerical behavior and are intentionally absent from the new routine's
ledger Rust target.

CP138 adds required `source_mapped` `routine.report_heat_balance` and the
matching heat-balance project-contract requirement. `HeatBalanceManager.cc`
and `.hh` are already in the algorithm source inventory, so it adds no source
file, Rust target or code, mapped Rust state, capability, support-gate
admission, manifest, comparator, proof variable, result, output, numerical,
performance, or conformance promotion. The inventory becomes 32 algorithms
and 150 routines, split into 58 `state_mapped` and 92 `source_mapped` routines;
the required-routine total becomes 50. The following CP139 section maps the
unconditional `ManageEMS(state,
EMSCallFrom::EndZoneTimestepAfterZoneReporting, anyRan, absent)` call at
`HeatBalanceManager.cc` line 219 by reusing `routine.manage_ems` without a
duplicate row.

### CP139 post-zone-reporting EMS calling-point map

After `ReportHeatBalance(state)` successfully returns at
`HeatBalanceManager.cc` line 217, line 219 unconditionally calls
`EMSManager::ManageEMS(state,
EMSCallFrom::EndZoneTimestepAfterZoneReporting, anyRan,
ObjexxFCL::Optional_int_const())`. The fourth argument is an explicitly absent
program-manager index. There is no caller guard. A successful call immediately
reaches `UpdateEMSTrendVariables(state)` at line 221 and then
`PluginManagement::PluginManager::updatePluginValues(state)` at line 222.

The caller passes the same `anyRan` local declared at line 189 and most
recently written by the CP136 pre-reporting EMS call. Canonical `ManageEMS`
begins by writing its referenced `anyProgramRan` argument false at
`EMSManager.cc` line 263, so CP139 overwrites the CP136 result rather than
accumulating it. `ManageHeatBalance` never branches on or otherwise consumes
the resulting value.

The existing non-required `routine.manage_ems` row covers the generic
`EMSManager.cc` lines 248-374 body, declared at `EMSManager.hh` lines 99-103.
After resetting the run flag, a model with
`AnyEnergyManagementSystemInModel == false` returns immediately. Otherwise the
routine calls `InitEMS` for this calling point, dispatches registered callbacks
and plugins because this is not `UserDefinedComponentModel`, and visits every
program-call manager whose `CallingPoint` exactly equals
`EndZoneTimestepAfterZoneReporting`, evaluating its Erl programs in stored
order. The absent optional manager index is inspected only by the separate
`UserDefinedComponentModel` path and is unused at CP139.

A callback, plugin, or matching Erl program can set `anyProgramRan` true. If
none ran, the generic routine returns before actuator application and
`ReportEMS`; if any ran, it commits every usable EMS/external-interface
actuator value and then calls `ReportEMS`. The caller still ignores `anyRan`.
Because CP138 has already completed schedule, Zone, sizing-log, tabular,
utility, and any warmup ESO/MTR reporting before CP139 is entered, an actuator
commit here cannot retroactively alter those outputs or cause
`ReportHeatBalance` to rerun. CP139 does not infer when the changed value first
becomes numerically, trend, or plugin-visible.

Complete EMS setup and initialization, callbacks, plugins, Erl managers and
programs, sensors, actuators, external interfaces, `ReportEMS`, diagnostics,
failure behavior, and every state mutation remain dependency context already
bounded by `routine.manage_ems`. A non-return prevents line-221 trend updating;
a successful quick return or entered execution reaches it. CP139 maps only the
caller identity, exact arguments and order, run-flag overwrite, call-specific
selection, and reporting non-retroactivity.

CP139 adds no duplicate routine row, EnergyPlus source, project-contract
entry, Rust target or code, mapped Rust state, capability, support-gate
admission, manifest, comparator, proof variable, result, output, numerical,
performance, or conformance promotion. The inventory remains 32 algorithms
and 150 routines, split into 58 `state_mapped` and 92 `source_mapped` routines;
the required-routine total remains 50. The following CP140 section maps the
unconditional `UpdateEMSTrendVariables(state)` call at
`HeatBalanceManager.cc` line 221, declared at `EMSManager.hh` line 122 and
implemented at `EMSManager.cc` lines 1444-1479.

### CP140 `UpdateEMSTrendVariables` source map

After the CP139 post-reporting EMS call successfully returns,
`ManageHeatBalance` line 221 unconditionally calls
`EMSManager::UpdateEMSTrendVariables(state)`. A successful call reaches the
unconditional `PluginManagement::PluginManager::updatePluginValues(state)`
call at line 222. The parent adds no warmup, sizing, output, or environment
guard around either call. `UpdateEMSTrendVariables` is declared at
`EMSManager.hh` line 122 and implemented at `EMSManager.cc` lines 1444-1479.

The implementation first returns immediately when
`AnyEnergyManagementSystemInModel` is false and then returns when
`NumErlTrendVariables == 0`. Otherwise it visits trend variables in their
1-based declaration order. For each entry it reads `ErlVariablePointer` and
`LogDepth`; a pointer at most zero or depth at most zero silently skips that
entry without affecting later declarations.

For each positive pointer/depth pair, the routine reads
`ErlVariable(ErlVariablePointer).Value.Number` without inspecting the Value
type, copies the complete `TrendValARR` into `tempTrendARR`, writes that
current number at `TrendValARR(1)`, and copies old indices
`1..LogDepth-1` into new indices `2..LogDepth`. For `LogDepth == 1` that tail
slice is empty, so only index 1 is overwritten. The newest value is therefore
always at 1 and the oldest retained value falls off the end. The body locally
validates neither an upper bound on `ErlVariablePointer` nor that `LogDepth`
fits the allocated arrays; it relies on prior setup invariants.

Trend-object input processing owns the declaration count/order and Erl
variable resolution. It floors the numeric requested depth into `LogDepth`
and then requires that stored depth to be greater than zero, so a request of
`0.5` fails rather than allocating a zero-depth history. It allocates
equal-depth `TrendValARR`, `tempTrendARR`, and `TimeARR` arrays, zeroes both
value arrays, and fills each fixed time entry as `-n * TimeStepZone` for
1-based history index `n`. All input/setup diagnostics are emitted there
rather than by CP140.

EMS BeginEnvrn initialization clears `TrendValARR` only. It does not clear
`tempTrendARR`, but CP140 overwrites that complete temporary array from the
current history before using it; the fixed `TimeARR` is not rebuilt. There is
no separate BeginDay or warmup-completion reset. Because the parent call is
unconditional, warmup and sizing invocations that reach line 221 advance the
same environment history. CP140 itself neither allocates nor validates trend
storage, mutates `TimeARR`, nor owns an environment reset.

CP140 samples whatever numeric Erl-variable state exists after CP139. A CP139
callback, plugin, or matching Erl program may have changed such state before
this sample, but CP140 does not attribute a value to a particular producer.
The separate plugin-value update at line 222 is later in source order and
cannot retroactively change the sample just pushed; CP140 makes no claim about
when that later update becomes visible to any future EMS execution.

The body emits no direct warning, recurring warning, severe, fatal, or output
message. Invalid positive pointers, inconsistent positive depths, allocation
failures, and any failure to return are governed by setup/runtime dependencies;
a non-return prevents the line-222 plugin update. No dependency receives a
child routine row or added source at this checkpoint.

Existing Rust composite heat-balance stage metadata includes the surrounding
post-report EMS checkpoint and later warmup-convergence checkpoint but has no
trend-update stage or EMS trend-history implementation. Existing execution
plan, prebinding, reporting, and result-store scaffolds do not implement this
routine's input ownership, current Erl-value dereference, 1-based history
shift, lifecycle, diagnostics, failure behavior, or source order, and are not
listed as a ledger Rust target.

CP140 adds non-required `source_mapped`
`routine.update_ems_trend_variables`. `EMSManager.cc` is already in the
Heat Balance Manager algorithm source inventory, so it adds no source file,
project-contract requirement, Rust target or code, mapped Rust state,
capability, support-gate admission, manifest, comparator, proof variable,
result, output, numerical, performance, or conformance promotion. The
inventory becomes 32 algorithms and 151 routines, split into 58
`state_mapped` and 93 `source_mapped` routines; the required-routine total
remains 50. The following CP141 section maps the unconditional plugin-value
update at `HeatBalanceManager.cc` line 222, declared at `PluginManager.hh`
line 198 and implemented at `PluginManager.cc` lines 1458-1467.

### CP141 `PluginManager::updatePluginValues` source map

After CP140 EMS trend updating successfully returns, `ManageHeatBalance` line
222 unconditionally calls
`PluginManagement::PluginManager::updatePluginValues(state)`. The parent has
no warmup, sizing, environment, output, plugin-presence, or build-mode guard.
The next block begins with `WarmupFlag && EndDayFlag` at line 224, whose
`CheckWarmupConvergence(state)` call is at line 226. The plugin routine
is declared at `PluginManager.hh` line 198 and implemented at
`PluginManager.cc` lines 1458-1467.

The implementation body is compiled only under `#if LINK_WITH_PYTHON`. A
build without Python linkage therefore keeps the unconditional parent call
but executes an empty no-op body. The linked body has no runtime manager or
plugin-presence guard: it directly visits `state.dataPluginManager->trends` in
stored order, and an empty vector naturally performs no work. For each trend
it obtains the linked current plugin-global value through the manager getter,
calls
`values.push_front(currentValue)`, and then `values.pop_back()`. The deque
keeps its preexisting length, places the newest sample at zero-based index 0,
shifts retained older values back, and drops the oldest. Setup owns the
positive history length and fixed time offsets; this routine validates neither
history length nor deque size and does not change the offsets.

PluginManager construction returns before input population when no retained
`PythonPlugin:Instance` exists. On the retained-instance path, setup owns
plugin-global/trend allocation, name/handle resolution, positive history
length, fixed offsets, and diagnostics. BeginEnvironment zeros both plugin
globals and every trend's `values` while retaining `times`. Missing handles
fatal during valid setup; the getter fatals when the global vector is empty,
while a nonempty invalid handle reaches unchecked `[]` and is not reliably
caught. CP141 allocates, resolves, resets, or validates none of this state.

The parent calls CP141 during warmup as well as ordinary and sizing Zone
timesteps. `updatePluginValues` does not test an individual plugin instance's
run-during-warmup flag, so each populated trend records the current global
during warmup even when its instance does not execute then. BeginEnvironment
is the relevant reset; CP141 adds no warmup-completion reset.

CP140 always precedes CP141 and updates only EMS/Erl trend storage. CP141 then
samples the current plugin globals into the separate plugin histories; it does
not revise the just-pushed EMS trend values or rerun the CP139 calling point.
A successful no-Python no-op or Python update reaches the line-224 warmup
block. Setup, getter, container, and Python-runtime failures remain
dependencies, and any non-return prevents it. The update body itself emits no
direct warning, recurring warning, severe, fatal, or output message.

Rust has no Python Plugin manager, plugin-global arena, plugin-trend history,
callback execution, or `updatePluginValues` stage. Existing heat-balance
source-order metadata proceeds from the post-report EMS stage directly toward
warmup convergence and does not implement this call. Existing input handling
continues to reject or run-block active Python plugin semantics; CP141 does not
change support admission and adds no Rust target.

CP141 adds non-required `source_mapped` `routine.update_plugin_values` and
adds `PluginManager.cc` plus `.hh` to the Heat Balance Manager algorithm
source inventory. It adds no project-contract requirement, Rust target or
code, mapped Rust state, capability, support-gate admission, manifest,
comparator, proof variable, result, output, numerical, performance, or
conformance promotion. The inventory becomes 32 algorithms and 152 routines,
split into 58 `state_mapped` and 94 `source_mapped` routines; the
required-routine total remains 50. The following CP142 section maps the
`WarmupFlag && EndDayFlag` block beginning at `HeatBalanceManager.cc` line
224, including the line-226 `CheckWarmupConvergence(state)` call.

### CP142 `CheckWarmupConvergence` source map

After CP141 returns, `ManageHeatBalance` lines 224-226 call
`CheckWarmupConvergence(state)` only when both `WarmupFlag` and `EndDayFlag`
are true. The routine is declared at `HeatBalanceManager.hh` line 136 and
implemented at `HeatBalanceManager.cc` lines 3059-3226. CP142 ends when that
call returns; the following CP143 section maps the inner `!WarmupFlag` branch
and `DayOfSim`/`DayOfSimChr` resets at lines 227-229 only.

`NumOfZones <= 0` immediately clears `WarmupFlag` without reading Zone arrays
or emitting diagnostics. Otherwise a false local `ConvergenceChecksFailed`
latch is shared across the source-ordered 1-based Zone loop. Each Zone always
stores `abs(previous maximum temperature - current maximum temperature)` and
the corresponding minimum-temperature difference. A value `<=`
`TempConvergTol` passes inclusively with `PassFlag` 1 or 2 set to 2; a larger
value sets that flag to 1 and latches failure for the whole building.

Heating and cooling are independent. Each load test is active only when its
current daily maximum is strictly `> 1.0e-4`; equality, negative values, and
smaller values take the no-load pass. An active test mutates both current and
previous loads in place to `abs(max(value, 100.0))`, then stores
`abs((current - previous) / current)` using the clamped current value as the
denominator. `<= LoadsConvergTol` is an inclusive pass; a larger ratio fails.
The no-load path writes only pass value 2 and does not overwrite its stored
load-test value, so that field can retain its constructor zero or a prior day
or environment's active-test result. The routine validates neither array
extent nor finite arithmetic locally.

At `DayOfSim >= MaxNumberOfWarmupDays` while `WarmupFlag` is true, each Zone
whose four pass values do not sum to 8 emits a severe nonconvergence
diagnostic, an environment-type and name continuation, and all four stored
comparisons. Thus a displayed
no-load comparison can be stale even though its pass flag is 2. Shared
`WarmupConvergenceWarning` and `SizingWarmupConvergenceWarning` latches allow
the extra normal-run or sizing guidance only once per module state; they are
not BeginEnvironment resets. Every failing Zone still receives its own severe,
environment, and comparison lines.

Before advancing to the next Zone, current heat, cooling, maximum-temperature,
and minimum-temperature values are copied to their previous-day arrays in
that order; active load values have already been clamped. Current heat,
cooling, and maximum temperature are then reset to `-9999.0`, and current
minimum temperature to `1000.0`. After the loop, a still-failing maximum-day
run emits a separate insufficient-user-maximum severe plus a suggestion only
when the configured maximum is below the default 25 days.

If every check passed, `DayOfSim >= MinNumberOfWarmupDays` clears
`WarmupFlag`; before the minimum it explicitly retains true. Any failure
otherwise leaves the caller-entered true value unchanged until the final
`DayOfSim >= MaxNumberOfWarmupDays` cap forces it false. The routine issues no
fatal itself. Diagnostic formatting/output, Zone and environment names, and
any non-return are dependencies; a non-return can prevent later Zone
copy/reset work and the CP143 branch.

`AllocateHeatBalArrays` dimensions previous values at zero, current extrema at
the four sentinels, and one convergence record per Zone whose four pass flags
default to 2 and test values to zero. BeginEnvironment resets previous values
and current extrema but does not recreate the convergence records or reset the
two warning latches; `clear_state()` clears the arrays and resets both latches.
CP137 `RecKeepHeatBalance` accumulates the daily current extrema. Project
control input owns positive tolerances, normalized minimum/maximum day counts,
and their diagnostics. These are dependencies, not CP142 allocations.

Rust exposes a `check_warmup_convergence_stage` and generic prebound/dependency
labels, but those are source-order metadata. Its separate diagnostic
`run_heat_balance_run_period_warmup` loop uses only the maximum absolute
change of daily Zone temperature extrema, requires a locally retained prior
day, reports one aggregate delta, and merely carries its load-tolerance field
into the summary. It has no canonical load tests, four-value pass state,
100 W mutation, source `-9999.0`/`1000.0` persistent sentinel/copy-reset
lifecycle, warning latches, Zone diagnostics, global `WarmupFlag`/`DayOfSim`
behavior, or exact parent gate. It is not a Rust target
for this row and is not promoted.

CP142 adds required `source_mapped` `routine.check_warmup_convergence` and the
matching heat-balance project-contract entry. `HeatBalanceManager.cc` and
`.hh` are already inventoried, so it adds no source file, Rust target or code,
mapped Rust state, support-gate admission, capability, output, numerical,
performance, or conformance promotion. The inventory becomes 32 algorithms
and 153 routines, split into 58 `state_mapped` and 95 `source_mapped`
routines; required routines become 51. The following CP143 section maps only
the inner `!WarmupFlag` branch and `DayOfSim`/`DayOfSimChr` resets at lines
227-229.

### CP143 post-warmup day-counter reset map

After CP142 `CheckWarmupConvergence(state)` returns inside the outer
`WarmupFlag && EndDayFlag` block, `ManageHeatBalance` line 227 tests the
updated `!WarmupFlag`. When true, line 228 first assigns numeric `DayOfSim =
0`; line 229 then assigns textual `DayOfSimChr = "0"`. CP143 maps that exact
order as inline caller state and creates no synthetic routine.

The line-228 comment describes convergence, but the predicate is broader.
CP142 can clear the flag after ordinary convergence at or after the minimum
day, immediately for `NumOfZones <= 0`, or forcibly at the maximum day even
when convergence checks failed. All three outcomes enter CP143. A failed
check below the maximum leaves `WarmupFlag` true and does not enter; neither
does a successful check before the minimum day.

The following CP144 `ManageEMS(state,
EMSCallFrom::BeginNewEnvironmentAfterWarmUp, anyRan, absent)` call at line 231
remains inside the same branch and therefore observes both reset values. The
mutations occur before that dependency, so an EMS fatal or other non-return
does not roll them back. Conversely, a CP142 non-return prevents both resets.
On the same `ManageHeatBalance` invocation, the later CP145 line-235 gate
cannot satisfy `DayOfSim == 1` after CP143 has written zero; CP143 does not map
that later report gate.

Existing Rust warmup options, summary, stage metadata, and diagnostic loop do
not implement these global numeric/text mutations or their exact parent
nesting. CP143 adds no routine row, EnergyPlus source, project-contract entry,
Rust target or code, mapped Rust state, support-gate admission, capability,
output, numerical, performance, or conformance promotion. The inventory
remains 32 algorithms and 153 routines, split into 58 `state_mapped` and 95
`source_mapped` routines, with 51 required. The following CP144 section maps
the exact `BeginNewEnvironmentAfterWarmUp` `ManageEMS` call at line 231.

### CP144 post-warmup EMS calling-point map

After CP143 sets `DayOfSim = 0` and then `DayOfSimChr = "0"`, line 231 calls
`EMSManager::ManageEMS(state,
EMSCallFrom::BeginNewEnvironmentAfterWarmUp, anyRan,
ObjexxFCL::Optional_int_const())` inside the same `!WarmupFlag` branch. The
fourth argument is absent. The same caller-owned `anyRan` was last written by
CP139; CP140 through CP143 do not touch it.

The existing non-required `routine.manage_ems` covers `EMSManager.cc` lines
248-374. It first overwrites `anyRan` false and quick-returns when no EMS is
present. Otherwise `InitEMS` receives this exact calling point, registered
callbacks/plugins run, and only program-call managers whose `CallingPoint`
equals `BeginNewEnvironmentAfterWarmUp` evaluate their stored Erl programs.
Any callback, plugin, or program sets the result true, which gates actuator
commit and `ReportEMS`; when none runs, both are skipped.

The runtime-language initialization and `PluginManagement::onBeginEnvironment`
hooks at `EMSManager.cc` lines 268-271 run only for the distinct
`BeginNewEnvironment` calling point, not CP144. CP144 sensors, callbacks,
plugins, and Erl code observe `WarmupFlag == false`, numeric day zero, and
textual day zero. The absent manager index is irrelevant outside the
`UserDefinedComponentModel` path.

The caller ignores the resulting `anyRan`. CP143 resets precede this call and
persist if an EMS dependency fatals or otherwise does not return; such a
non-return prevents CP145. On successful return, the same-invocation CP145
line-235 gate still fails `DayOfSim == 1` because CP143 wrote zero.

CP144 adds no duplicate routine, EnergyPlus source, project-contract entry,
Rust target or code, mapped Rust state, support-gate admission, capability,
output, numerical, performance, or conformance promotion. The inventory
remains 32 algorithms and 153 routines, split into 58 `state_mapped` and 95
`source_mapped` routines, with 51 required. The following CP145 section maps
the lines 235-237 guarded `ReportWarmupConvergence(state)` call.

### CP145 `ReportWarmupConvergence` source map

After the CP142-CP144 outer block, `ManageHeatBalance` lines 235-237 call
`ReportWarmupConvergence(state)` only under the four-way guard
`!WarmupFlag && EndDayFlag && DayOfSim == 1 && !DoingSizing`. The routine is
declared at `HeatBalanceManager.hh` line 138 and implemented at
`HeatBalanceManager.cc` lines 3228-3301. The convergence-ending invocation
cannot enter because CP143 wrote day zero; normal day progression makes this
the end of the first actual non-warmup day. The parent ends at line 238.

The body has its own `!WarmupFlag` guard and otherwise performs a complete
no-op. On entry, a true
`ReportWarmupConvergenceFirstWarmupWrite` together with `NumOfZones > 0`
writes the EIO header and clears that flag. The flag defaults and resets true
only with module-state construction/`clear_state`, not BeginEnvironment, so
the header is one per state rather than one per environment. A zero-Zone call
writes no header and leaves the flag true.

The routine zeros the complete temperature and load standard-deviation
scratch arrays, then chooses `RunPeriod:` when `RunPeriodEnvironment` is true
and `SizingPeriod:` otherwise. This label selection is independent of the
caller's `!DoingSizing` condition. Zones are visited in 1-based source order.
For each Zone, the temperature mean is the sum of samples `1..N` divided by
`N`, where `N = CountWarmupDayPoints`.

Before computing the load mean, each stored `LoadZoneRpt(ZoneNum, sample)` is
mutated in place. If the matching `MaxLoadZoneRpt` is strictly `> 1.0e-4`,
the load difference is divided by that maximum; equality, smaller, or
negative maxima replace it with zero. The load mean then sums these normalized
values over `1..N` and divides by `N`. Temperature and load scratch entries
receive squared deviations, and each reported standard deviation is the
population value `sqrt(sum(squared deviation) / N)`, not the sample `N - 1`
form.

Each EIO row writes, in order, Zone name; the selected environment label plus
a space and environment name; temperature mean and standard deviation;
`PassFail` for CP142 maximum- and minimum-temperature flags; normalized load
mean and standard deviation; then `PassFail` for heating- and cooling-load
flags. The four reals use `{:.10R}`. The header nevertheless labels the two
load statistics in W even though the body has converted them to ratios.

Because normalization mutates `LoadZoneRpt` but not `MaxLoadZoneRpt`, a
repeated call divides already normalized positive-threshold samples again;
the report is not idempotent. The header remains suppressed after its first
write, while scratch arrays are cleared and rows recomputed on every call.
The routine validates neither `N > 0`, sample-array capacity, aligned Zone
extents, nor finite division locally, and emits no warning, severe, fatal, or
other direct diagnostic; EIO, formatting, array, and `PassFail` failures are
dependencies.

`AllocateHeatBalArrays` sizes the three Zone/sample arrays to `NumOfZones` by
`TimeStepsInHour * 24`, allocates matching one-day scratch arrays, and zeros
the shared count. BeginEnvironment zeros the three sample arrays and
`CountWarmupDayPoints`. During the first non-warmup day, CP137
`RecKeepHeatBalance` increments the shared count only for Zone 1 once per
timestep, then stores every Zone's temperature difference, raw load
difference, and maximum load in that common sample slot. CP145 allocates,
resets, and bounds none of this producer state.

Existing Rust warmup options, temperature-extrema loop, day-end snapshots,
summary, and source-order metadata own no equivalent EIO header/rows,
first-day sample arena/count, four `PassFail` values, in-place normalized-load
mutation, population-statistics report, or exact caller gate. They are not a
Rust target for this routine and are not promoted.

CP145 adds required `source_mapped` `routine.report_warmup_convergence` and
the matching heat-balance project-contract entry. `HeatBalanceManager.cc` and
`.hh` are already inventoried, so it adds no source file, Rust target or code,
mapped Rust state, support-gate admission, capability, numerical, performance,
output, or conformance promotion. The inventory becomes 32 algorithms and 154
routines, split into 58 `state_mapped` and 96 `source_mapped` routines, with
52 required. The following CP146 section maps
`SetPreConstructionInputParameters`, declared at header line 96 and
implemented at source lines 446-492.

### CP146 `SetPreConstructionInputParameters` source map

`SimulationManager.cc` line 216 unconditionally calls
`SetPreConstructionInputParameters(state)` after constant/state initialization,
the mismatched-environment and requested-reporting checks, and predefined-table
setup at lines 210-215. The call establishes construction array bounds before
the Zone and System `SetupTimePointers` calls at lines 218-220. The routine is
declared at `HeatBalanceManager.hh` line 96 and implemented at
`HeatBalanceManager.cc` lines 446-492. This caller is cross-domain source-order
context only; CP146 reuses the existing HeatBalanceManager source inventory and
adds no `SimulationManager.cc` inventory entry.

`HeatBalanceData::MaxSolidWinLayers` has a member default of zero at
`DataHeatBalance.hh` line 1787, and `clear_state()` reconstructs the complete
module state at lines 2051-2054, restoring that default. Every routine entry
then overwrites any previous value with 7 at source line 464. The adjacent
line-462 comment still says to start at 5, so the executable assignment, not
the stale comment, owns the mapped baseline. Consequently re-entry recomputes
from 7 rather than accumulating a prior call's maximum.

Lines 466-469 query only the raw occurrence count of
`Construction:ComplexFenestrationState`. Any positive count raises the shared
maximum to at least 10. This branch retrieves no item and therefore inspects
no complex-state identity, layer fields, validity, declaration position, or
resolved graph; raw presence alone is sufficient. The count query does not
erase any complex-state record from InputProcessor's `unusedInputs`, so CP146
does not mark those objects used.

Lines 471-489 then obtain the raw
`Construction:WindowEquivalentLayer` count and request items in 1-based input
order. Each `getObjectItem` call fills the shared `dataIPShortCut` alpha and
numeric value arrays, alpha and numeric blank-marker arrays, and alpha and
numeric field-name arrays, while also returning local `NumAlpha`, `NumNumber`,
and `IOStat`. The routine uses only `NumAlpha - 1`. This is the positional
alpha-field span after the construction name, including any intervening blank
or hole positions represented by the returned span; it is not a count of
nonblank or successfully resolved layer references. Alpha contents, numeric
values, `NumNumber`, `IOStat`, every blank marker, and every field name are
otherwise ignored.

Each returned positional span is folded into the shared maximum with `max`.
Under the EnergyPlus 26.1 schema the normal successful result is therefore in
the inclusive range 7 through 11: baseline 7, possible raw complex-state
presence raising it to 10, and up to eleven equivalent-layer layer positions.
Within `getObjectItem`, `NumAlpha` and `NumNumber` are zeroed and `IOStat` is
set to -1, the shared arrays/blank markers/field names are cleared, and the
requested equivalent-layer object is erased from InputProcessor's
`unusedInputs` before field traversal. Only a fully completed traversal sets
`IOStat` to 1. CP146 tests none of that status or usage state. Successful item
order cannot change the final maximum, but retrieval still occurs in source
input order and leaves the last completed item's shared values and markers. If
a dependency failure does not return, the maximum exposes only the already
applied baseline, complex-state presence, and successfully returned item
prefix; however, the current item may already be marked used and the shared
buffers may contain its partial field traversal rather than the prior complete
item. Later items and downstream initialization are then not reached.

The routine deliberately ignores opaque
`Construction:CfactorUndergroundWall` and
`Construction:FfactorGroundFloor` objects. It performs no local validation of
layer references, alternating equivalent-layer topology, alpha holes or
blankness, returned counts, or the schema maximum, and it emits no direct
warning, severe, fatal, or output row. InputProcessor/schema validation,
diagnostics, and any non-return behavior are dependencies rather than CP146
child rows.

The resulting integer is an early shared allocation/loop bound. Construction
input calls `ConstructionProps::setArraysBasedOnMaxSolidWinLayers` at
`HeatBalanceManager.cc` lines 1418-1419, later generated Window5 constructions
repeat that allocation at lines 4228-4231, and source consumers include the
line-4312 construction loop, `HeatBalanceSurfaceManager.cc` line 2757,
`SolarShading.cc` lines 1027-1029 and 7668, `SurfaceGeometry.cc` lines 121,
13505, 13585, 13722, and 13787, and `WindowManager.cc` line 1680. Those arrays,
loops, construction/window state, and their consumers remain dependencies;
CP146 maps the producer contract only.

Rust defines `MAX_WINDOW_CONSTRUCTION_LAYERS = 8` and
`MAX_WINDOW_EQUIVALENT_LAYER_CONSTRUCTION_LAYERS = 11`, and its ordinary and
equivalent-layer construction paths store layers in dynamic `Vec`s. Those
validation/capacity limits do not implement the mutable shared
`MaxSolidWinLayers`, raw full-family scans, line-216 call order, shared input
buffers, partial-failure prefix, downstream source allocations, or reset and
re-entry lifecycle. They are not targets or parity evidence for this routine.

CP146 adds required `source_mapped`
`routine.set_pre_construction_input_parameters` and the matching heat-balance
project-contract entry. It adds no new EnergyPlus source inventory, Rust target
or code, mapped Rust state, support-gate admission, capability, output,
numerical, performance, or conformance promotion. The inventory becomes 32
algorithms and 155 routines, split into 58 `state_mapped` and 97
`source_mapped` routines, with 53 required. The following CP147 section maps
`GetSiteAtmosphereData`, declared at header line 100 and implemented at source
lines 1252-1317.

### CP147 `GetSiteAtmosphereData` source map

`GetHeatBalanceInput` calls `GetProjectControlData(state, ErrorsFound)` at
`HeatBalanceManager.cc` line 262, `GetSiteAtmosphereData(state, ErrorsFound)`
at line 264, and `Material::GetWindowGlassSpectralData` at line 266. CP147 is
not gated by an already-true shared `ErrorsFound`. Its declaration is at
`HeatBalanceManager.hh` line 100 and body at `HeatBalanceManager.cc` lines
1252-1317.

The routine allocates local `AlphArray(1)` and `NumArray(3)`, overwrites
`HeatBalanceMgrData::CurrentModuleObject` with `Site:HeightVariation`, and asks
InputProcessor for the raw object count. Module `clear_state()` restores the
current-object string to empty; each re-entry overwrites it again. Environment
state defaults to wind exponent 0.22, boundary-layer height 370.0 m, and
temperature gradient 0.0065 K/m, and `EnvironmentData::clear_state()` restores
those defaults.

The preceding project-control routine maps Building Terrain to wind state:
Country 0.14/270, Suburbs or Urban 0.22/370, City 0.33/460, and Ocean 0.10/210;
its invalid-Terrain recovery uses 0.14/270 and sets `ErrorsFound`. Thus CP147
enters with Terrain-derived wind values and the existing gradient, even when a
prior project-control diagnostic already set the shared error flag.

For exactly one raw object, lines 1272-1287 request item 1 into the local value
arrays while passing the shared numeric/alpha blank-marker and field-name
arrays. `NumNums` controls three independent prefix copies: `> 0` writes
`SiteWindExp`, `> 1` writes `SiteWindBLHeight`, and `> 2` writes
`SiteTempGradient`. A zero-length numeric span preserves all predecessor state;
one or two positions can therefore produce a hybrid of explicit/defaulted
prefix values with retained Terrain wind or retained gradient. A blank or
omitted position within a returned prefix receives its schema default before
the copy; omitted trailing positions beyond `NumNums` are not copied even
though the local array has three slots.

EnergyPlus 26.1 declares `Site:HeightVariation` unique. N1 wind exponent
defaults to 0.22 with inclusive minimum 0.0; N2 boundary-layer thickness
defaults to 370.0 m with strict minimum above 0.0; N3 temperature gradient
defaults to 0.0065 K/m with inclusive minimum 0.0. None has a schema maximum.
CP147 performs no local numeric, finite, cross-field, or returned-count
validation and does not inspect `AlphArray`, `NumAlphas`, blank markers, field
names, or `IOStat`; schema/InputProcessor validation owns those effects.

For a found item, `getObjectItem` zeros both returned counts, sets `IOStat` to
-1, clears the local values and shared metadata arrays, and erases the requested
record from InputProcessor `unusedInputs` before field traversal. Only full
completion sets `IOStat` to 1, which CP147 ignores. A non-return can therefore
leave the object marked used and shared metadata or local arrays partially
filled while none of CP147's three environment copies or EIO writes has run.

More than one raw object takes no item read, marks no height-variation object
used, and changes none of the three environment values. It emits exactly
`Too many Site:HeightVariation objects, only 1 allowed.` and then sets the
caller-owned `ErrorsFound = true`; the assignment is monotonic because CP147
never clears the flag. A non-returning diagnostic prevents that later flag
assignment and all output. With zero objects, no item is read, Terrain-derived
wind exponent/height are retained, and only the gradient is reset to 0.0065.

After every normally returning zero-, one-, or duplicate-object branch, lines
1311-1316 unconditionally write the EIO header
`! <Environment:Site Atmospheric Variation>,Wind Speed Profile Exponent {},Wind Speed Profile Boundary Layer Thickness {m},Air Temperature Gradient Coefficient {K/m}`
and one `Environment:Site Atmospheric Variation` row using `{:.3R}`, `{:.3R}`,
and `{:.6R}`. There is no one-time guard: direct re-entry repeats both records.
The duplicate path reports retained predecessor values despite its error. A
failure in the header or row output preserves prior mutations, may leave only
the header, and prevents return to the line-266 material call.

Downstream source formulas consume the three shared values. `OutDryBulbTempAt`
and `OutWetBulbTempAt` use the gradient, weather-file temperature modifier,
Earth radius, and height at `DataEnvironment.cc` lines 100-107 and 143-150;
`WindSpeedAt` uses the zero-height/zero-exponent branches and
`WindSpeed * WeatherFileWindModCoeff * pow(Z / SiteWindBLHeight, SiteWindExp)`
at lines 179-188. Bulk Surface temperature/wind paths and Kiva wind state are
additional dependencies, not CP147 child routines or promoted behavior.

Rust retains Building Terrain and hard-codes Terrain-to-wind pairs in
`energyplus_site_wind_profile`; its wind helper derives from that enum, while
its temperature helper uses a fixed default gradient and a different fixed
linear formula. `Site:HeightVariation` remains raw-only. These helpers do not implement
the source object intake, prefix/default hybrid state, mutable shared
environment, diagnostics, InputProcessor usage/metadata effects, EIO records,
or reset/re-entry behavior, so none is a CP147 target or parity claim.

CP147 adds required `source_mapped` `routine.get_site_atmosphere_data` and the
matching project-contract entry. Existing HeatBalanceManager sources cover the
routine, so no source inventory, Rust target/code/state, support, capability,
output, numerical, performance, or conformance claim is added. The inventory
becomes 32 algorithms and 156 routines, split into 58 `state_mapped` and 98
`source_mapped`, with 54 required. The following CP148 section maps
`AllocateZoneHeatBalArrays`, declared at header line 130 and implemented at
source lines 2824-2854.

### CP148 `AllocateZoneHeatBalArrays` source map

`InitHeatBalance` lines 2617-2618 call `AllocateHeatBalArrays(state)` only when
`BeginSimFlag` is true. After comments, the parent routine's first executable
action is the unconditional `AllocateZoneHeatBalArrays(state)` call at line
2863. CP148 is declared at `HeatBalanceManager.hh` line 130 and implemented at
`HeatBalanceManager.cc` lines 2824-2854. Direct unit-test calls are also
possible, as the source comment notes.

The first branch uses only `ZoneIntGain.allocated()` as a sentinel. When false,
`DataHeatBalance::AllocateIntGains` performs this exact fallback bundle in
`DataHeatBalance.cc` lines 1044-1047: `ZoneIntGain` to `NumOfZones`, then
`spaceIntGain` to `numSpaces`, then `spaceIntGainDevices` to `numSpaces`, then
`spacePowerReductionFactor` to `numSpaces` with every value 1.0.
`SpaceZoneSimData` elements default `NOFOCC` and `QLTSW` to zero;
`SpaceIntGainDeviceData` defaults both counts to zero with its device array
unallocated. The sole Zone sentinel does not verify any companion's allocation,
extent, or content and skips the entire bundle when already true.

On the ordinary simulation path, `GetHeatBalanceInput` line 320 has already
called `ManageInternalHeatGains(state, true)`. Its input path checks the same
sentinel and normally calls `AllocateIntGains` at `InternalHeatGains.cc` lines
291-293 before CP148 is later reached through BeginSim initialization. CP148
therefore ordinarily preserves an already populated internal-gain bundle; its
fallback chiefly serves direct unit-test and alternate call paths.

Lines 2832-2834 then unconditionally allocate `zoneHeatBalance` to
`NumOfZones` and `spaceHeatBalance` to `numSpaces`. Space records are allocated
even when `doSpaceHeatBalance` is false because they gather some Zone totals.
Both are `EPVector`s: `allocate` sets its allocated flag before `resize`, then
fills every element with `T{}`. The inherited `ZoneSpaceHeatBalanceData`
defaults therefore include 23 C for MAT, MRT, ZTAV, ZT, ZTAVComf, XMPT, TMX,
TM2, MixingMAT, and all four XMAT and DSXMAT slots; 0.01 for airHumRat,
airHumRatAvg, airHumRatTemp, airHumRatAvgComf, and MixingHumRat; and zero for
every remaining member, including T1, ZTM, both W history families,
coefficients, gains, flows, and loads. CP148 invokes no later environment
initializer and applies no Zone/Space identity or topology check locally.

Eight `Array1D<Real64>` solar-enclosure allocations follow, each dimensioned
exactly to `NumOfSolarEnclosures`, in source order:
`EnclSolQSDifSol`, `EnclSolQD`, `EnclSolQDforDaylight`, `EnclSolDB`,
`EnclSolDBSSG`, `EnclSolDBIntWin`, `EnclSolQSWRad`, then
`EnclSolQSWRadLights`. A 1-based enclosure loop then writes zero in a different
exact order: `EnclSolQSDifSol`, `EnclSolQD`, `EnclSolQDforDaylight`,
`EnclSolQSWRad`, `EnclSolQSWRadLights`, `EnclSolDB`, `EnclSolDBSSG`, and
`EnclSolDBIntWin`.

Zero Zone, Space, or enclosure counts are not rejected. EPVector zero-size
allocation still sets its allocated sentinel, companion dimensions are
requested at zero extent, and the enclosure zeroing loop naturally has no
iterations. A negative `NumOfZones` or `numSpaces` converts to EPVector's
unsigned `size_type`; `allocate` first sets the sentinel true and then generally
does not return from the resulting huge `resize`. In contrast, a negative
`NumOfSolarEnclosures` gives each Objexx array the empty 1:0 extent and the
1-based zeroing loop skips, so that tail can return. Oversized, mutually
inconsistent, and topology-inconsistent counts are also unchecked. Container
allocation/failure behavior is a dependency; CP148 emits no warning, severe,
fatal, output, or status result.

All calls are sequential with no transaction or rollback. Because EPVector
marks itself allocated before `resize`, even a failure during the first
`ZoneIntGain` resize can leave the sentinel true; a direct retry then skips the
whole fallback bundle. Failure after any companion or Zone/Space/enclosure
allocation exposes that completed prefix, may expose an allocated-before-resize
EPVector, and prevents later allocations/zero writes and the parent remainder.
An enclosure-loop failure exposes a prefix of zeroed indices in its exact
per-index write order.

On successful direct re-entry without a clear, the true `ZoneIntGain` sentinel
preserves the entire existing internal-gain bundle and its accumulated values.
In contrast, both Zone/Space heat-balance EPVectors are unconditionally resized
and filled with fresh `T{}` records, destroying their prior temperatures,
humidity, histories, loads, and coefficients; the eight enclosure arrays are
again allocated and zeroed. Owning clears are nonuniform:
`HeatBalanceData::clear_state()` resets the `ZoneIntGain` sentinel, its Space
companions, and the enclosure arrays; predictor/corrector clear resets the
Zone/Space heat-balance vectors; and ViewFactor clear resets its enclosure
count. `DaylightingManagerData::clear_state()` does not clear
`spacePowerReductionFactor`. Nevertheless, the reset false Zone sentinel makes
the next fallback re-dimension that retained factor to `numSpaces` and overwrite
every value with 1.0. A partial-failure retry without the relevant owning clears
is observably different; CP148 does not claim that all participating state is
uniformly fresh or unallocated after clear.

Only a successful CP148 return lets `AllocateHeatBalArrays` continue with its
line-2864-through-2962 FanSystem, contaminant, warmup, resilience, and report
arrays. That parent remainder, the BeginSim flag lifecycle, and all later
initialization are dependencies rather than CP148 child rows or promoted state.

The Zone/Space records feed predictor/corrector, convection, HVAC, moisture,
comfort, and reporting paths. `SolarShading` produces the beam/interior-window
enclosure factors, while `HeatBalanceSurfaceManager` derives daylighting and
diffuse/short-wave quantities and surface absorption from the eight arrays.
Internal-gain and daylighting consumers use the fallback bundle. Their complete
state, equations, order, diagnostics, and outputs remain downstream dependencies.

Rust has a `Vec<ZoneHeatBalanceState>` and bounded initialization/state-shell
helpers, but no equivalent Space arena, four-member fallback sentinel bundle,
eight-array solar-enclosure allocation, EPVector allocated-before-resize
semantics, source defaults/order, partial failure, destructive re-entry, or
clear/retry lifecycle. These conceptual shells are not a CP148 target or parity
evidence.

CP148 adds required `source_mapped` `routine.allocate_zone_heat_bal_arrays` and
the matching heat-balance project-contract entry. The existing
HeatBalanceManager inventory covers it; `AllocateIntGains` and every consumer
remain dependency context, so no source inventory, Rust target/code/state,
support, capability, output, numerical, performance, or conformance claim is
added. The inventory becomes 32 algorithms and 157 routines, split into 58
`state_mapped` and 99 `source_mapped`, with 55 required. The following CP149
section maps the complete `AllocateHeatBalArrays` parent, declared at
`HeatBalanceManager.hh` line 132 and implemented at `HeatBalanceManager.cc`
lines 2855-2963.

### CP149 `AllocateHeatBalArrays` source map

`InitHeatBalance` lines 2617-2618 contain the sole production `src/` call to
`AllocateHeatBalArrays(state)`, guarded only by `BeginSimFlag`; unit tests can
and do call the routine directly. It is declared at `HeatBalanceManager.hh`
line 132 and implemented at `HeatBalanceManager.cc` lines 2855-2963. Its first
executable action is the line-2863 unconditional CP148
`AllocateZoneHeatBalArrays(state)` call; CP149 retains that child as its own
required row rather than folding its internal-gain, Zone/Space, and
solar-enclosure effects into the parent row.

After CP148 returns, lines 2864-2878 perform this exact unconditional order for
`N = NumOfZones`. Below, `T`, `C`, and `V` denote the executable extents
`state.dataWeather->TotThermalReportPers`, `TotCO2ReportPers`, and
`TotVisualReportPers`, respectively:

1. dimension `SumConvHTRadSys`, `SumLatentHTRadSys`, `SumConvPool`,
   `SumLatentPool`, `ZoneQdotRadHVACToPerson`,
   `ZoneQHTRadSysToPerson`, `ZoneQHWBaseboardToPerson`,
   `ZoneQSteamBaseboardToPerson`, `ZoneQElecBaseboardToPerson`, and
   `ZoneQCoolingPanelToPerson`, in that order, to `N` values of 0.0;
2. call `ZoneReOrder.allocate(N)`;
3. dimension `ZoneMassBalanceFlag` and then `ZoneInfiltrationFlag` to `N`
   values of false;
4. assign zero to the complete `ZoneReOrder` array; and
5. dimension `TempTstatAir` to `N` values of the 23 C initial Zone
   temperature.

The two following contaminant branches are independent. If
`CO2Simulation` is true, line 2880 samples `CO2OutdoorSched->getCurrentVal()`
once into `OutdoorCO2`, then dimensions `ZoneAirCO2`, `ZoneAirCO2Temp`, and
`ZoneAirCO2Avg`, in that order, to `N` copies of the sampled value. If
`GenericContamSimulation` is true, line 2886 independently samples
`genericOutdoorSched->getCurrentVal()` once into `OutdoorGC`, then dimensions
`ZoneAirGC`, `ZoneAirGCTemp`, and `ZoneAirGCAvg` in that order. Each sample is
the schedule's current value, including the current EMS-overridden value when
applicable; the routine does not resample between the three dimensions. A
false branch performs no write and therefore preserves any prior scalar and
array state on same-state re-entry.

Lines 2891-2909 next dimension the complete warmup bundle in exact order:
`MaxTempPrevDay`, `MinTempPrevDay`, `MaxHeatLoadPrevDay`, and
`MaxCoolLoadPrevDay` to 0.0; `MaxHeatLoadZone`, `MaxCoolLoadZone`, and
`MaxTempZone` to -9999.0; `MinTempZone` to 1000.0; then
`TempZonePrevDay`, `LoadZonePrevDay`, `TempZoneSecPrevDay`,
`LoadZoneSecPrevDay`, `WarmupTempDiff`, `WarmupLoadDiff`, `TempZone`, and
`LoadZone` to 0.0. Each is dimensioned to `N`. Finally `TempZoneRpt`,
`LoadZoneRpt`, and `MaxLoadZoneRpt` are dimensioned, in that order, to
`[N, TimeStepsInHour * 24]` with every value 0.0.

The convergence records and work arrays follow without explicit fill values:
`WarmupConvergenceValues.allocate(N)`, then
`TempZoneRptStdDev.allocate(TimeStepsInHour * 24)`, then
`LoadZoneRptStdDev.allocate(TimeStepsInHour * 24)`. On a fresh allocation, each
`WarmupConvergence` record receives its constructor defaults: four
`PassFlag` entries initialized to 2 and its four stored temperature/load test
values initialized to 0.0. The two arithmetic standard-deviation arrays have
no explicit initialization in CP149.

Lines 2915-2918 then allocate
`CrossedColdThreshRepPeriod[N, T]`, allocate
`CrossedHeatThreshRepPeriod` to the same shape, assign the complete cold array
false, and only then assign the complete heat array false. The explicit
assignments reset both arrays regardless of whether the preceding no-value
allocation reused storage.

When `T > 0`, lines 2919-2937 allocate these eleven `[N, T]` report matrices in exact
order: `ZoneHeatIndexHourBinsRepPeriod`,
`ZoneHeatIndexOccupiedHourBinsRepPeriod`,
`ZoneHeatIndexOccuHourBinsRepPeriod`, `ZoneHumidexHourBinsRepPeriod`,
`ZoneHumidexOccupiedHourBinsRepPeriod`,
`ZoneHumidexOccuHourBinsRepPeriod`, `ZoneColdHourOfSafetyBinsRepPeriod`,
`ZoneHeatHourOfSafetyBinsRepPeriod`, `ZoneUnmetDegreeHourBinsRepPeriod`,
`ZoneDiscomfortWtExceedOccuHourBinsRepPeriod`, and
`ZoneDiscomfortWtExceedOccupiedHourBinsRepPeriod`. When `C > 0`, lines
2940-2944 allocate
`ZoneCO2LevelHourBinsRepPeriod`, `ZoneCO2LevelOccuHourBinsRepPeriod`, and
`ZoneCO2LevelOccupiedHourBinsRepPeriod` to `[N, C]`. When `V > 0`, lines
2946-2952 allocate
`ZoneLightingLevelHourBinsRepPeriod`,
`ZoneLightingLevelOccuHourBinsRepPeriod`, and
`ZoneLightingLevelOccupiedHourBinsRepPeriod` to `[N, V]`. A false condition skips its complete
family and preserves any prior allocation and contents on re-entry.

Regardless of those three conditions, lines 2955-2960 allocate six
`[N, T]` arrays in this order:
`ZoneLowSETHoursRepPeriod`, `ZoneHighSETHoursRepPeriod`,
`lowSETLongestHoursRepPeriod`, `highSETLongestHoursRepPeriod`,
`lowSETLongestStartRepPeriod`, and `highSETLongestStartRepPeriod`. Thus even a
zero thermal-report count reaches all six empty-extent allocation calls.
`CountWarmupDayPoints = 0` at line 2962 is the routine's last executable
mutation.

Objexx `dimension(extent, value)` writes the supplied value throughout on every
call, including equal-shape re-entry. In contrast, no-value `allocate` routes
through `dimension_real`. In an ordinary release build, existing linear cells
survive whenever `resize(total_size)` reports no reallocation: equal extents
are the common case, while changed bounds or a changed two-dimensional shape
with the same product reinterpret those cells under the new indices. A changed
element count can also avoid reallocation when reserved capacity suffices;
that container edge remains a dependency. When `resize` reports reallocation,
`std::vector<Real64>` report cells default-construct as empty vectors, while
fresh arithmetic cells, including standard-deviation and longest/start arrays,
have no CP149 value initialization and may be indeterminate until a producer
writes them. With `OBJEXXFCL_ARRAY_INIT`/debug initialization enabled, the
no-reallocation path instead `assign()`s the configured trait/debug values
rather than preserving cells, and fresh arithmetic cells can receive a debug
sentinel. In every build, the post-allocation assignments still zero
`ZoneReOrder` and both crossed-threshold arrays, and every value-bearing
`dimension` still resets its complete extent. This distinction forbids
treating every allocation as a blanket zero initialization.

All operations are sequential and CP149 has no local validation, diagnostic,
status result, transaction, or rollback. Failure to return from CP148 prevents
every parent-tail effect. Thereafter a failure exposes the already completed
allocation/fill prefix and prevents every later branch and the final count
reset. A contaminant schedule dependency can fail before its scalar assignment;
after a successful sample the scalar is already changed if a following
dimension fails. Failures in a later conditional report family leave earlier
families and cells intact. Two delayed fills are especially observable:
failure in either flag dimension occurs after `ZoneReOrder.allocate` but before
its zero assignment, and failure in the heat-threshold allocation occurs after
the cold-threshold allocation but before either false assignment. The owning
container's extent and allocation-failure behavior, schedule pointer validity,
and consistency of Zone, timestep, and report counts are dependencies rather
than checks performed here.

Zero `N` still reaches enabled contaminant schedule samples and scalar
assignments and then executes the complete tail. Zone-shaped arrays are empty,
but the standard-deviation arrays remain `TimeStepsInHour * 24` long when that
value is positive. Negative `N` normally cannot get past CP148 because its
EPVector resize converts the count to a huge unsigned size. Nonpositive `T`,
`C`, or `V` skips the corresponding conditional family; crossed-threshold and
all six SET-family allocations still execute for nonpositive `T`, with negative
Objexx extents yielding empty ranges. Nonpositive `TimeStepsInHour` yields
empty report second extents and standard-deviation arrays; signed
multiplication overflow remains a dependency. None of these cases is locally
validated or diagnosed.

After CP149, the same BeginSim branch still runs its optional CTF, view-factor,
equivalent-layer/window-optics, daylighting, and solar initialization children
at lines 2619-2630. A non-return there prevents the later line-2633
`BeginEnvrnFlag`-only block from resetting CP149 state. If reached, lines
2633-2652 reset previous-day extrema to 0.0; `MaxHeatLoadZone`,
`MaxCoolLoadZone`, and `MaxTempZone` to -9999.0; `MinTempZone` to 1000.0;
`TempZone` and `LoadZone` to -9999.0;
`TempZonePrevDay` to 1000.0; `LoadZonePrevDay` and `TempZoneSecPrevDay` to
-9999.0; both warmup-difference arrays and the three report matrices to 0.0;
and `CountWarmupDayPoints` to zero. They do not reset `LoadZoneSecPrevDay`, any
member of `WarmupConvergenceValues`, `TempZoneRptStdDev`, or
`LoadZoneRptStdDev`. Later environments normally take this reset with
`BeginSimFlag` false and therefore do not reallocate retained arrays; a direct
`BeginSimFlag`-true/`BeginEnvrnFlag`-false path allocates without this reset.
The separate HeatBalFanSys, ContaminantBalance, and HeatBalanceManager owning
`clear_state()` paths remove or reset all CP149 arrays/scalars, so a fully
cleared retry is fresh, unlike ordinary same-state storage preservation and
CP148's separate cross-owner clear asymmetry. Contaminant BeginEnvironment
logic can independently overwrite the CO2/GC initial state later.

The first accumulators feed radiant systems, baseboards, pools, person gains,
HVAC, comfort, and air-balance work. `ZoneReOrder`, the mass-balance and
infiltration flags, and `TempTstatAir` feed ZoneEquipmentManager, Zone air
balance, room-air, and predictor/corrector paths. The CO2/GC triples feed the
contaminant predictor, HVAC, and reporting paths. Warmup and resilience
producer/consumer paths, especially CP137, CP142, CP145, and CP132-CP134,
remain dependency context rather than CP149 children or promoted sources.

Rust has no parent `AllocateHeatBalArrays` analog. Existing Zone vectors,
initialization stages, warmup metadata, contaminant shells, and resilience
reporting code do not implement this complete cross-module state bundle, exact
order and shapes, constructor/default versus indeterminate values, conditional
preservation, schedule sampling, sequential partial effects, BeginSim and
BeginEnvironment lifecycle, or resize/reallocation-dependent re-entry
semantics. None becomes a
CP149 target or parity claim.

CP149 adds required `source_mapped` `routine.allocate_heat_bal_arrays` and the
matching heat-balance project-contract entry immediately after its CP148 child.
Existing HeatBalanceManager sources already cover the parent; all schedule,
container, producer, and consumer details remain dependencies, so no source
inventory, Rust target/code/state, support, capability, output, numerical,
performance, or conformance claim is added. The inventory becomes 32 algorithms
and 158 routines, split into 58 `state_mapped` and 100 `source_mapped`, with 56
required. The following CP150 section maps
`UpdateWindowFaceTempsNonBSDFWin`, declared at `HeatBalanceManager.hh` line 140
and implemented at `HeatBalanceManager.cc` lines 3303-3313.

### CP150 `UpdateWindowFaceTempsNonBSDFWin` source map

`RecKeepHeatBalance` line 3056 is the sole production `src/` caller and invokes
`UpdateWindowFaceTempsNonBSDFWin(state)` unconditionally as its last executable
action. The helper returns at line 3313 and then its parent returns at line
3057. At the complete `ManageHeatBalance` level, the line-210 pre-reporting EMS
call precedes the line-211 `RecKeepHeatBalance` call, and successful record
keeping precedes `ReportHeatBalance` at line 217. Thus successfully copied
endpoint state is available to same-Zone-timestep reporting. The helper has no
warmup, sizing, output-request, kickoff, or other body-wide guard. Its public
declaration also supports the direct unit-test call in
`HeatBalanceManager.unit.cc`, where the test manually assembles only the list,
constructions, histories, and destination matrices needed by the helper.

`AllHTWindowSurfaceList` is a `std::vector<int>` declared at
`DataSurfaces.hh` line 1525. Its normal clean-state producer in
`SurfaceGeometry.cc::GetSurfaceData` reserves `TotWindows` capacity at line
2633, then scans `SurfNum = 1..TotSurfaces` at line 2640 and appends an index
only inside the line-2647 `HeatTransSurf` branch when `Class == Window` at
lines 2654-2655. Normal membership is therefore exactly heat-transfer Window
surfaces in ascending Surface index, without a BSDF or exterior-solar filter.
`reserve` creates no entries or defaults, and that builder block does not clear
the vector first; an abnormal same-state producer re-entry can append duplicate
runs. CP150 consumes the stored vector exactly as found and performs no class,
heat-transfer, window-construction, exterior-solar, sorting, uniqueness, or
membership recheck.

For each stored `SurfNum`, line 3306 first reads
`Surface(SurfNum).Construction` and dereferences that current mutable entry in
`Construct`. This is not necessarily the originally entered construction:
other source paths can change `Surface.Construction`. CP150 does not directly
consult `ConstructionStoredInputValue`, `SurfActiveConstruction`,
`SurfWinActiveShadedConstruction`, `activeShadedConstruction`, or
`SurfWinStormWinConstr`. A true `thisConstruction.WindowTypeBSDF` is the sole
per-entry skip and occurs only after both the Surface and Construction lookups.

Every non-BSDF entry then performs exactly two assignments in this order:

1. `SurfWinFenLaySurfTempFront(SurfNum, 1) =
   SurfOutsideTempHist(1)(SurfNum)` at line 3310; and
2. `SurfWinFenLaySurfTempBack(SurfNum, thisConstruction.TotLayers) =
   SurfInsideTempHist(1)(SurfNum)` at line 3311.

`TotLayers` is the Construction's total material-layer count, not
`TotSolidLayers`. The two writes target different front/back arrays even when
`TotLayers == 1`. CP150 performs no interpolation, averaging, unit conversion,
clamp, finite-value test, or arithmetic, so finite values, infinities, and NaNs
are copied as supplied. It writes no intermediate front/back layer, solar,
flux, Construction, Surface, history, or BSDF state.

Destination allocation is dependency-owned.
`SolarShading::AllocateModuleArrays` dimensions both
`SurfWinFenLaySurfTempFront` and `SurfWinFenLaySurfTempBack` to
`[TotSurfaces, MaxSolidWinLayers]` with 0.0 at `SolarShading.cc` lines
1028-1029. The term-1 sources are the current-time histories declared at
`DataHeatBalSurface.hh` lines 211-212.
`HeatBalanceSurfaceManager::AllocateSurfaceHeatBalArrays` allocates their
outer arrays to `Construction::MaxCTFTerms` and dimensions every inner Surface
array to zero at lines 1455-1463; initialization later writes the 23 C
`SurfInitialTemp` to every history term for ordinary heat-transfer Surfaces at
lines 2323-2332, and subsequent Surface-balance work owns their current values.
CP150 itself allocates, dimensions, defaults, registers, or clears none of this
state and does not verify either matrix's extents.

The associated face-temperature output registration is narrower than the
helper loop. `SolarShading.cc` lines 1396-1450 first require a heat-transfer
Window and then `ExtSolar`. For a non-BSDF construction they set
`NumOfLayers = TotLayers` and register only the front-face variable for layer 1
and the back-face variable for layer `NumOfLayers`; BSDF constructions instead
register all eligible layer faces and rely on their own producers. CP150 still
copies endpoint cells for non-BSDF entries without testing `ExtSolar` or an
output request. Output registration, request selection, aggregation, and the
BSDF solvers remain dependencies, not CP150 effects or promoted output claims.

An empty stored list is a natural no-op. Valid BSDF entries perform their
Surface and Construction lookups and then preserve every destination cell.
Duplicate non-BSDF entries repeat the same two writes in stored order. A valid
manually inserted non-Window or non-heat-transfer Surface is processed because
the loop trusts list membership. Invalid Surface or Construction indices fail
before the BSDF decision; even an intended BSDF entry cannot skip an invalid
Construction lookup. Missing term 1, absent or undersized destination arrays,
and invalid Surface extents are likewise unchecked dependency failures.

All effects are sequential, with no diagnostic, status result, transaction, or
rollback. Earlier entries remain updated when a later lookup or write does not
return. Within one non-BSDF entry, a successful front write precedes the back
history/access/write; `TotLayers <= 0` or above the destination's second extent
can therefore fail at the back target after the front endpoint was updated.
That failure preserves the completed prefix, prevents later list entries,
prevents `RecKeepHeatBalance` from returning, and consequently prevents the
same invocation's line-217 reporting call. The routine emits no warning,
severe, fatal, recurring diagnostic, log record, or direct output of its own.

Successful re-entry overwrites the two currently selected cells rather than
accumulating, and is idempotent when the list, current Construction metadata,
and term-1 histories are unchanged. New history values replace the endpoints.
If `Surface.Construction` or its `TotLayers` changes, the newly selected back
cell is written but the obsolete back cell is not cleared. A construction that
becomes BSDF preserves prior non-BSDF values; removed or newly unlisted
surfaces and all untouched intermediate cells likewise retain old values.
`InitHeatBalance`'s BeginEnvironment block does not explicitly reset these
destination matrices. A later BeginSim `AllocateModuleArrays` dimension resets
them to zero, while the HeatBalanceData, HeatBalSurfData, SurfacesData, and
ConstructionData owner clears respectively reconstruct the destinations,
histories, list/Surface table, and Construction table. A coherently cleared
state has an empty list and no work; a manually repopulated list with otherwise
cleared dependencies can fail at the first unchecked lookup.

Rust has no `AllHTWindowSurfaceList`, fenestration face-report matrices, or
window-history handoff analog, and supported execution deliberately blocks
fenestration Surfaces. Existing construction typing, stage/result metadata,
opaque Surface histories, and report scaffolding do not implement this helper,
its mutable Construction selection, stored-order traversal, endpoint state,
failure prefix, or lifecycle. None becomes a CP150 target or parity claim.

CP150 adds required `source_mapped`
`routine.update_window_face_temps_non_bsdf_win` and its heat-balance
project-contract entry immediately after `rec_keep_heat_balance` and before
`report_heat_balance`. `HeatBalanceManager.cc` already covers the routine; list
production, allocations, producers, and output setup remain dependency context,
so no source inventory, Rust target/code/state, support, capability, output,
numerical, performance, or conformance claim is added. The inventory becomes
32 algorithms and 159 routines, split into 58 `state_mapped` and 101
`source_mapped`, with 57 required. The following CP151 section maps
`OpenShadingFile`, declared at `HeatBalanceManager.hh` line 144 and implemented
at `HeatBalanceManager.cc` lines 3422-3438.

### CP151 `OpenShadingFile` source map

`InitHeatBalance` lines 2696-2698 contain the sole production `src/` call. The
exact three-way caller gate is `BeginSimFlag && DoWeathSim &&
ReportExtShadingSunlitFrac`; a false conjunct skips the complete routine,
including file opening. Before this point, the required BeginSim block has
completed its allocations and optics/daylighting/solar setup, followed by any
entered BeginEnvironment reset, EMS construction-property, and
storm-window/active-construction branches at lines 2633-2694. A successful
CP151 return reaches BeginDay solar work at lines 2700-2716 and the later
shading-row writer at lines 2718-2739.

`ReportExtShadingSunlitFrac` defaults and resets false in
`DataSystemVariables.hh` and is populated by ShadowCalculation input before
this initialization path. `DoWeathSim` is the independent global decision that
a weather simulation will be performed. The physical-output selector is
separate: `state.files.outputControl.extshd` defaults true and can be disabled
by `OutputControl:Files` `output_extshd`. CP151 has no own first-call flag; its
normal one-header lifecycle relies on the caller's `BeginSimFlag`, and its
public declaration also supports direct calls. The direct
`SolarShading.unit.cc` test opens the shade stream as a stringstream before
calling CP151 at line 4151.

Line 3432 first calls
`state.files.shade.ensure_open(state, "OpenOutputFiles",
state.files.outputControl.extshd)`. The `shade` `InputOutputFile` defaults to
`eplusshading.csv`; command-line output-prefix and suffix selection can replace
that path by combining the configured prefix with legacy `shading.csv`, dash
`-shading.csv`, or capitalized `Shading.csv` suffixes. This CSV stream is distinct from SolarShading's
separate diagnostic `.shd` output.

`InputOutputFile::ensure_open` at `IOFiles.cc` lines 212-221 opens only when
the current stream is not `good()`. If physical output is enabled, its
`open(false, true)` path uses text-mode `in | out | trunc`, creating or
truncating the selected file. An unsuccessful open does not return after the
exact fatal template `OpenOutputFiles: Could not open file {filePath} for
output (write).`. If `extshd` is false, `open(false, false)` instead installs a
dev-null stream that `good()` deliberately treats as usable; CP151 still
executes and evaluates all following name accesses and print calls, but no
physical file receives them.

An already-good physical stream, stringstream, or dev-null stream is not
reopened. The current stream, mode, path, and put position therefore stay
latched even if `filePath` or `extshd` has changed. A direct successful re-entry
normally writes another header at the current position. In contrast, a closed
or failed physical stream is reopened with truncation and can discard its old
contents; a good dev-null stream remains dev-null even if physical output is
later enabled. These are `ensure_open` dependency effects rather than CP151
flags or validation.

After opening, the body emits this exact logical record in source order:

1. literal `Surface Name,`;
2. for `SurfNum = 1..TotSurfaces`, each raw
   `Surface(SurfNum).Name` formatted as `{},`; and
3. one `\n` newline.

Every complete Surface participates in numeric index order, including
non-heat-transfer, interior, Window, shading, or otherwise unsupported
classes. There is no sorting, deduplication, class/exposure filter, alternate
report order, quoting, escaping, or comma/newline sanitization. Blank names add
an empty field; embedded CSV delimiters or line breaks are written raw. Every
field, including the last Surface, has a trailing comma. Zero or negative
`TotSurfaces` skips the loop but still produces logical `Surface Name,\n`. The
direct stringstream unit test fixes the two-Surface LF representation exactly
as `Surface Name,ZN001:WALL001,ZN001:WALL002,\n`; a physical text stream's
newline representation remains platform dependent.

CP151 never flushes or closes the stream, computes a sunlit fraction, mutates a
Surface, or returns a status. Normal simulation close occurs later in
`SimulationManager`, while `EnergyPlusData` clear also closes the shade stream;
the helper itself neither deletes the file nor checks a final stream status.
The already-mentioned inline parent block later writes daily data under the
separate `BeginDayFlag && !WarmupFlag && KindOfSim == RunPeriodWeather &&
ReportExtShadingSunlitFrac` gate; this is not a refinement of CP151's caller
gate because it omits `BeginSimFlag` and `DoWeathSim`. After
`PerformSolarCalculations`, a normal validated positive `TimeStepsInHour`
produces `24 * TimeStepsInHour` timestamped rows and every numeric-index
`SurfSunlitFrac`, again with trailing commas. Those calculations, allocations,
formats, and row writes are downstream dependency context, not CP151 body
effects or child rows. The header can consequently exist without any data row,
including when it is opened during earlier design/sizing activity for a model
that also requests a later weather run.

All opening and printing is sequential with no transaction or rollback. A
physical-open failure prevents the literal header. After a successful open, an
invalid positive `TotSurfaces`/Surface-array relationship can leave
`Surface Name,` plus a prefix of names before the failed lookup. A stream error
during a print can likewise leave a partial buffered/file prefix; the body has
no post-write `good()` check, local warning, severe, recurring diagnostic, or
recovery. If it returns despite a newly failed stream, later parent work still
runs. Re-entry after that failed state can cause `ensure_open` to replace the
stream with a truncating reopen. The initial physical open is the sole locally
reachable exact fatal diagnostic.

Rust has no `OpenShadingFile`, `ReportExtShadingSunlitFrac`, `extshd` output
selector, `eplusshading.csv` lifecycle, all-Surface `SurfSunlitFrac` export, or
equivalent header/row writer. Existing `solar.rs`
`SurfaceIncidentSolarComponents` values consumed by `surface_balance.rs` are
bounded incident-solar calculation/result state, not this all-Surface CSV
export. `Schedule:File:Shading` support is the opposite input direction
and explicitly does not establish Surface sunlit-fraction consumption or heat
balance parity. No current Rust stage, result store, or file writer is a CP151
target.

CP151 adds non-required `source_mapped` `routine.open_shading_file`.
`HeatBalanceManager.cc` already covers the body, while IO, input, Surface,
solar-calculation, row-writing, and shutdown behavior remains dependency
context. It adds no project-contract requirement, source inventory, Rust
target/code/state, support, capability, output, numerical, performance, or
conformance claim. The inventory becomes 32 algorithms and 160 routines, split
into 58 `state_mapped` and 102 `source_mapped`, with 57 required. After the
already mapped `GetFrameAndDividerData` and `SearchWindow5DataFile` declarations,
the following CP152 section maps `SetStormWindowControl`, declared at
`HeatBalanceManager.hh` line 156, implemented at `HeatBalanceManager.cc` lines
4595-4644, and called by `InitHeatBalance` at line 2669 under
`TotStormWin > 0 && BeginDayFlag`.

### CP152 `SetStormWindowControl` source map

`InitHeatBalance` lines 2666-2694 own the sole production `src/` call. The
outer `TotStormWin > 0` gate encloses the complete storm-window block; its
inner `BeginDayFlag` branch calls CP152 at line 2669. Only after a successful
return does line 2670 set caller-owned `ChangeSet = false`. The call follows
the BeginSim and BeginEnvironment work plus EMS-controlled construction and
Surface-property initialization, and precedes the caller's active-construction
remainder, CP151 file opening, and solar work. The public declaration permits a
direct call, but no dedicated EnergyPlus unit test or second production caller
was found.

The CP152 body performs these effects in exact source order:

1. set shared `StormWinChangeThisDay = false` before any StormWindow lookup;
2. visit `StormWindow(1..TotStormWin)` in numeric input order, read its
   `BaseWindowNum`, and copy that Surface's current `SurfWinStormWinFlag` into
   `SurfWinStormWinFlagPrevDay`;
3. compute a local `DateOff = StormWindow.DateOff - 1`, replacing only zero
   with 366;
4. call `General::BetweenDates(DayOfYear_Schedule, DateOn, DateOff)` and choose
   local/current flag 1 for true or 0 for false;
5. write that current flag, then, only when `BeginSimFlag` is true, overwrite
   the previous-day flag with the same new value; and
6. compare current with previous and monotonically latch
   `StormWinChangeThisDay = true` on any mismatch.

`GetStormWindowData` creates both input dates with
`OrdinalDay(month, day, 1)`, and WeatherManager similarly creates
`DayOfYear_Schedule` with `OrdinalDay(Month, DayOfMonth, 1)` regardless of the
actual year's leap status. The normal dependency therefore supplies fixed
leap-shaped 1..366 ordinals. `BetweenDates` includes both supplied endpoints:
when start is at most end it tests one closed interval, and when start exceeds
end it tests the inclusive year-wrapping union. Subtracting one before that
call therefore makes `DateOn` inclusive and the original input `DateOff`
exclusive. An input off date of January 1 first becomes zero and is explicitly
remapped to 366, preserving the wrap through December 31. Equal on/off dates
are rejected by input processing, not CP152; if malformed equal valid ordinals
reach this body directly, the adjusted inclusive interval/wrap covers all 366
dates and leaves the storm window on all year. CP152 does not inspect months,
validate the test/date range, account for a different calendar shape, or emit a
diagnostic.

After a normal CP152 return, the caller's false `ChangeSet` marks its one-time
post-BeginDay clear/snapshot as pending, whether or not a flag changed. On the
first later call with `BeginDayFlag == false`, lines
2671-2677 clear `StormWinChangeThisDay`, copy every referenced current flag to
its previous-day cell in StormWindow order, and set `ChangeSet = true`; later
non-BeginDay calls skip that one-time copy. Independently of the BeginDay and
`ChangeSet` branches, every call inside the positive-total outer gate then
visits Zones in numeric order, each Zone's stored `spaceIndexes`, and every
index in each Space's inclusive `WindowSurfaceFirst..WindowSurfaceLast` range,
without restricting the pass to StormWindow-record references. Normal ranges
can include Window, GlassDoor, and TDD_Diffuser Surfaces. It selects the
dependency-synthesized `SurfWinStormWinConstr` only for current flag 1 and
`WindowModel::Detailed`; all other visited indices receive the current mutable
`Surface(SurfNum).Construction` in `SurfActiveConstruction`. This can overwrite
the immediately preceding EMS-controlled active-construction decision, while
still using the EMS-mutated current `Surface.Construction` on the fallback
path. Those caller mutations are ordered downstream dependency effects, not
CP152 body writes or an additional routine row.

The routine does not sort, filter, deduplicate, validate, diagnose, return a
status, transact, or roll back. A direct call with nonpositive `TotStormWin`
still clears the shared change flag and otherwise returns. Duplicate
`BaseWindowNum` entries repeatedly target one Surface: the last record's
current flag wins, each later duplicate copies the earlier duplicate's newly
written current value into previous, and any intermediate mismatch remains
latched globally. Under `BeginSimFlag`, each record overwrites previous with
its own new current value before comparison, suppressing that record's change.
For normal unique BaseWindow references, a successful direct or repeated
same-BeginDay re-entry first clears the global latch, copies the already-current
value to previous, and recomputes the same date value, thereby erasing the
earlier transition; when `BeginSimFlag` is false, conflicting duplicate
schedules can instead relatch it during their intermediate writes.

All mutations are sequential. An invalid positive count, StormWindow extent,
`BaseWindowNum`, or flag-array extent can fail after the entry clear, after a
record's previous copy, or after any completed record. The prefix remains, no
rollback or local diagnostic occurs, and later records are skipped. A failure
also prevents the caller's `ChangeSet = false` and its active-construction
remainder. Conversely, a failure in that remainder after CP152 returns leaves
the CP152 writes, `ChangeSet = false`, and any completed active-construction
prefix in place while preventing CP151 and later solar work. A failure in the
separate non-BeginDay clear/snapshot loop occurs after the event flag was
cleared but before `ChangeSet = true`; it leaves a previous-day prefix and
`ChangeSet = false`, so a retry repeats that branch. If caller topology instead
changes `TotStormWin` to zero or negative, the complete outer block is skipped
and existing event, `ChangeSet`, flag, and active-construction state can remain
stale; this differs from a direct nonpositive-total CP152 call, which clears the
event flag. Re-entry consequently observes whatever completed or stale state
was left rather than a clean transaction.

Fresh `SurfacesData` owns `TotStormWin = 0`, an empty StormWindow array, and
unallocated flag arrays; SurfaceGeometry dependency allocation dimensions both
flag arrays to zero for all Surfaces. Fresh/cleared `HeatBalanceData` owns
`StormWinChangeThisDay = false`, while fresh/cleared `HeatBalanceMgrData` owns
`ChangeSet = true`. Their respective owner clears restore those values and
containers. The BeginEnvironment block does not independently clear this
bundle; daily caller cadence and `BeginSimFlag` establish its ordinary runtime
history.

Downstream source code consumes this state rather than extending CP152. The
caller immediately publishes `SurfActiveConstruction`; window starting-face
temperature logic compares current and previous flags and selects a fresh
resistance-network glass-face temperature guess at BeginEnvironment or across
add/remove transitions, otherwise reusing previous-timestep face temperatures;
SolarShading can recompute
daylighting coefficients when the shared daily-change latch survives its own
guards and selects storm variants of shaded constructions when the current
flag is 1; divider heat transfer is skipped while a storm window is on; and the
current flag backs `Surface Storm Window On Off Status`. Construction creation,
input validation, output registration, daylighting, shading control, optical
and thermal calculations, and all such consumers remain dependency context.

Rust has no typed `WindowProperty:StormWindow`, fixed-calendar on/off dates,
`TotStormWin`/StormWindow arena, current/previous storm flags,
`StormWinChangeThisDay`, `ChangeSet`, storm construction, active-construction
switch, or fenestration runtime. Existing calendar helpers, Surface and
Construction typing, execution-stage metadata, and result/report scaffolding
do not implement this controller or any consumer and are not CP152 targets.

CP152 adds non-required `source_mapped`
`routine.set_storm_window_control`. `HeatBalanceManager.cc` already covers the
body, so CP152 adds no project-contract requirement, source inventory, Rust
target/code/state, test, support, capability, output, numerical, performance,
or conformance claim. The inventory becomes 32 algorithms and 161 routines,
split into 58 `state_mapped` and 103 `source_mapped`, with 57 required. The
following CP153 section maps `InitConductionTransferFunctions`, declared at
`HeatBalanceManager.hh` line 180, implemented at `HeatBalanceManager.cc` lines
6153-6202, and called by `InitHeatBalance` at line 2621 under
`BeginSimFlag && (AnyCTF || AnyEMPD)`.

### CP153 `InitConductionTransferFunctions` source map

`InitHeatBalance` lines 2617-2622 contain the sole production `src/` call. A
false `BeginSimFlag` skips the allocation and CP153 chain completely. When it
is true, line 2618 must first return from `AllocateHeatBalArrays`; the caller
then enters CP153 only when `AnyCTF || AnyEMPD`, emits exact progress text
`Initializing Response Factors`, and calls the routine at line 2621. Thus an
EMPD model uses this same thermal CTF generator even though its moisture state
is owned elsewhere. A false inner condition skips both display and call. A
successful CP153 return reaches `InitSurfacePropertyViewFactors` and the rest
of BeginSim initialization; any non-return prevents those later effects.

The wrapper starts local `ErrorsFound` and `DoCTFErrorReport` as false, then
performs these ordered passes:

1. visit every entry of `dataConstruction->Construct` in array order, including
   entries not used by CTF, and call its `calculateTransferFunction` method;
2. after each returned child, set shared `SimpleCTFOnly = false` when that
   construction's `NumHistories > 1`, and raise shared `MaxCTFTerms` only when
   its `NumCTFTerms` exceeds the retained maximum;
3. after the complete first pass, independently set `SimpleCTFOnly = false`
   when `AnyInternalHeatSourceInInput` is true;
4. make a second complete Construction pass, skipping only `!IsUsedCTF`, and
   call `reportLayers` for each survivor;
5. call `ScanForReports(state, "Constructions", InitCTFDoReport,
   "Constructions")`; when the request is present or a child forced
   `DoCTFErrorReport`, write the four Construction/material/CTF EIO headers,
   increment a one-based `cCounter` for every Construction including skipped
   ones, and call `reportTransferFunction` only for `IsUsedCTF` entries; and
6. after all reachable reporting, fatal on accumulated errors with exact text
   `Program terminated for reasons listed (InitConductionTransferFunctions)`.

`ConstructionProps::calculateTransferFunction` in `Construction.cc` lines
59-1103 is child dependency context rather than a CP153 routine row. On every
entry it zero-fills the fixed regular CTF arrays (`Outside`, `Cross`, `Inside`,
and `Flux`), source/sink QTF arrays, source-location and user-location
temperature-QTF arrays, and resets `CTFTimeStep`, `NumHistories`,
`NumCTFTerms`, and `UValue` to zero. An unused Construction returns only after
that destructive reset, so stale coefficients on a newly unused entry are not
preserved.

Each used child starts its timestep at `TimeStepZone`, reads every declared
layer's material pointer in layer order, and classifies resistance-only layers.
For a source/sink Construction, a too-thin or too-diffusive material can emit a
one-material-lifetime severe/continuation diagnostic and set
`WarnedForHighDiffusivity`, but that branch does not set the shared error flag.
Thickness above 3 m and resistance below `1.0e-3 m2-K/W` do set
`ErrorsFound`. Valid boundary resistance-only layers, and any resistive layer
whose material is not `ROnly`, are converted to equivalent air properties;
interior `ROnly` layers instead retain exact massless treatment. The child
scans the full layer list, then returns before all generation whenever the
shared `ErrorsFound` is already true. Consequently one earlier Construction
error makes each later used child reset and scan its layers but leave its
generation outputs zero, even when that later Construction has no own error.

Before generation, adjacent interior massless layers are merged in local work
arrays. Every successful merge reduces the local layer/resistance counts and,
when a source is present, also decrements the Construction's persistent
`SourceAfterLayer` and `TempAfterLayer` fields. The routine then converts its
work properties to the legacy English-unit calculation basis, sums series
resistance, and derives conductance. This merge mutation is not restored at
return and is therefore relevant to re-entry.

The child next selects one of three branches:

- An all-resistance Construction uses `TimeStepZone`, one history, and one CTF
  term. Its current outside, cross, and inside coefficients and `UValue` become
  the inverse total resistance, while history/flux terms are zero. A source or
  sink on this branch emits a severe error and sets `ErrorsFound`, but the
  steady coefficients are still copied before the child returns.
- A source-free Construction whose declared layer pointers exactly reverse an
  earlier array entry with the same layer count can reuse that entry only when
  the earlier Construction is `IsUsedCTF`. It copies timestep and counts,
  swaps inside/outside coefficients, retains cross and nonzero-index flux
  coefficients, and derives its own `UValue`; source-bearing constructions
  never take this shortcut.
- Every other massive Construction creates a finite-difference node grid:
  interior resistance-only layers use one node, massive/equivalent boundary
  layers clamp to the source minimum/maximum node rules, optional 2-D geometry
  expands the grid, and source and requested-temperature nodes are mapped.
  The adaptive loop builds the 1-D or 2-D state matrices, emits
  `Calculating CTFs for "<name>"` on every attempt, then calls
  `calculateExponentialMatrix`, `calculateInverseMatrix`, `calculateGammas`,
  and `calculateFinalCoefficients` in that exact order.

The final-coefficient dependency stops its term series when the absolute last
to first flux ratio is below `1.0e-13` or its first flux term is zero, and
symmetrizes paired cross terms. Back in the adaptive loop, more than 18 terms
or a greater-than-1% mismatch between absolute outside/cross/inside series sums
increments `NumHistories`, adds one `TimeStepZone` to `CTFTimeStep`, and retries
the complete matrix/coefficient sequence. A zero largest series sum fatals
immediately with `Illegal construction definition, no CTFs calculated for
<name>`. Any attempt reaching a CTF timestep of at least 7 hours emits the
ordered convergence/material guidance, forces the Construction report, sets
the shared error, and breaks. Singular, overflow, and nonfinite matrix results
have no separate local validation contract. A normally returning new-massive
path copies current/history CTFs and all applicable QTFs back to fixed storage,
sets `UValue`, and deallocates its matrix work arrays.

`reportLayers` at `Construction.cc` lines 1973-1984 writes each used
Construction's material names to already-created predefined columns only when
that column vector is nonempty. If detailed reporting is selected,
`reportTransferFunction` at lines 1908-1971 writes the Construction summary
with the gap-preserving all-Construction `cCounter`, then one material-summary
record per declared layer, regular CTF terms from highest index down through
zero, and, when
applicable, source/sink plus source-location and user-location QTF series. These
child calculations and report writers remain dependency context; CP153 adds no
separate row or `Construction.cc`/`.hh` source-inventory entry.

All effects are sequential and nontransactional. A child convergence error can
leave earlier Constructions fully generated, the failing Construction's latest
coefficients, and later used Constructions reset to zero; wrapper layer reports
and a forced or requested detailed report still run before the final fatal.
Ordinary accumulated errors that do not set `DoCTFErrorReport` produce detailed
CTF output only when independently requested. An immediate zero-series or
resistance-merge fatal, a material/assert/matrix dependency failure, or an
output failure bypasses the remaining passes and wrapper fatal while retaining
the completed prefix. There is no rollback, status result, or cleanup of
already published Construction/global/report state.

Successful direct re-entry resets and recomputes each Construction's coefficient
bundle, and reverse reuse sees the freshly processed earlier entry in the same
pass. The wrapper does not first restore `SimpleCTFOnly = true` or
`MaxCTFTerms = 0`; it can only turn the former false and raise the latter, so a
simpler mutated model can retain stale global state. Its two local error/report
flags restart false, while material high-diffusivity warning latches persist
and successful reports append again. Repeated adjacent-resistance merging can
decrement persistent `SourceAfterLayer`/`TempAfterLayer` again, making that
source-bearing path non-idempotent. `ConstructionData`, `HeatBalanceData`, and
`MaterialData` owner clears respectively reconstruct coefficient/Construction
state, `SimpleCTFOnly`/`MaxCTFTerms` and related flags, and the material
`WarnedForHighDiffusivity` latch/material state. BeginEnvironment alone does
not rerun CP153.

Rust's `ConstructionThermalDataCache` can label coefficients
`EnergyPlusEioSeeded` and feed them into bounded CTF histories, balances,
updates, and reports. Without imported rows, `RustGeneratedSteady` supplies only
the no-history `Outside0 = Cross0 = Inside0 = 1/R` fallback. Rust has no native
finite-difference grid, SI/English conversion path, reverse reuse, matrix
exponential/inversion/gamma/final-coefficient generator, adaptive timestep and
history loop, QTF generation, exact diagnostics/report lifecycle, global
simple/max mutation, or re-entry behavior. Oracle-seeded limited conformance
therefore does not prove a native CP153 implementation.

CP153 adds required `source_mapped`
`routine.init_conduction_transfer_functions` and its heat-balance
project-contract requirement immediately after the two allocation children and
before `manage_surface_heat_balance`. `HeatBalanceManager.cc` already covers
the canonical wrapper; all Construction methods remain dependencies. CP153
adds no source inventory, Rust target/code/state, test, support, capability,
output, numerical, performance, or conformance promotion. The inventory
becomes 32 algorithms and 162 routines, split into 58 `state_mapped` and 104
`source_mapped`, with 58 required. The following CP154 section maps
`GatherForPredefinedReport` before CP155 continues the Surface-manager source
order.

### CP154 `GatherForPredefinedReport` source map

The canonical declaration is `HeatBalanceSurfaceManager.hh` line 99 and the
body is `HeatBalanceSurfaceManager.cc` lines 623-1404. Its sole production
`src/` caller is `InitSurfaceHeatBalance` lines 475-482. Only
`BeginSimFlag` gates the call: `InitSurfaceHeatBalancefirstTime` gates the exact
`Gathering Information for Predefined Reporting` display at lines 478-480 but
does not gate line 481. A false BeginSim flag skips both display and gather; a
true flag always gathers even when no predefined report will be shown. The
independent `Output:Constructions`-derived `dataGeneral->Constructions` flag
controls only Construction-related EIO headers and rows. In caller order,
`InitIntSolarDistribution` and then `InitIntConvCoeff` have already returned;
a successful gather continues to the `AnyCondFD` initialization and following
conduction-history work, while a non-return blocks that remainder. Three direct
EnergyPlus unit slices exercise azimuth classification, interior envelope
reporting, and exterior shaded-state reporting, but do not close the complete
routine contract.

Entry state is rebuilt locally in a fixed order. Both Surface-class count
arrays and all exterior/interior fenestration totals start at zero;
`computedNetArea` is allocated to `TotSurfaces` and zero-filled. When at least
one frame/divider exists and Construction EIO output is enabled, the routine
writes the `FenestrationAssembly` header before traversing any Surface. Local
construction/frame, shaded-construction/frame, and unframed-shaded-construction
deduplication vectors plus the two shaded-state header latches begin empty on
every call. It then makes its first pass in stored
`AllSurfaceListReportOrder`, without adding a heat-transfer, used, validity, or
report-selection filter.

A Surface is exterior only when `ExtBoundCond` is exactly
`ExternalEnvironment`, `Ground`, `GroundFCfactorMethod`, or `KivaFoundation`;
every other value takes the interior branch. Exterior Wall, Floor, and Roof
rows append Construction, Zone, Space, outside solar reflectance
`1 - OutsideAbsorpSolar`, no-film `NominalU`, multiplied gross area, azimuth,
tilt, and possible cardinal direction. Interior opaque rows add `IntMass` to
those three classes and also append the adjacent-surface name. Opaque and Door
areas use only `Zone.Multiplier * Zone.ListMultiplier`; Window and `TDD_Dome`
areas additionally use `Surface.Multiplier`. The routine adds each multiplied
opaque gross area to that Surface's net-area slot and subtracts each multiplied
opening from `computedNetArea(BaseSurf)`, with no lower clamp or validation.

Azimuth is rounded to two decimals before both storage and classification.
Cardinal output exists only for `60 <= Tilt < 180`: rounded azimuth at least
315 or below 45 is north, `[45, 135)` east, `[135, 225)` south, and
`[225, 315)` west. Exterior fenestration sets its separate north flag only in
that first range; every other orientation, including horizontal and
non-cardinal tilt, contributes to non-north totals.

For every exterior Window or `TDD_Dome`, the routine first appends
Construction/Zone/Space identity and obtains nominal optical state. A nonzero
cached `Construction.SummerSHGC` supplies SHGC and visible transmittance.
Otherwise a non-equivalent-layer construction calls
`Window::CalcNominalWindowCond` and stores its returned summer SHGC, visible
transmittance, and solar transmittance into the Construction; the returned
`errFlag` is ignored. An equivalent-layer construction with a zero cache does
not call that helper. The local SHGC/transmittance outputs have no entry
initializers, so a dependency error or that zero-cache equivalent-layer path
can publish indeterminate or previously occupied values; the non-equivalent
dependency path also stores its returned values into the exterior cache.
A normally returning `CalcNominalWindowCond` additionally rewrites
`SurfWinCoeffAdjRatio` across the complete exterior-Window list, not just the
current Surface. Normal setup relies on the earlier Construction glass-report
path to have produced valid nominal state.

Exterior opening geometry begins with `GrossArea`. A frame adds
`(Height + 2 * FrameWidth) * (Width + 2 * FrameWidth) - Height * Width`; the
divider contribution is
`DividerWidth * (HorDividers * Width + VertDividers * Height - HorDividers *
VertDividers * DividerWidth)`. Frame name/conductance and divider conductance
are appended, while the full gross-plus-frame opening receives every
multiplier, is subtracted from the parent net area, and weights the aggregate
U/SHGC/visible-transmittance numerators. The one-opening area, frame area,
divider area, derived glass area, multiplied area, nominal values, parent,
rounded orientation, and shading information are then appended in source
order.

A framed opening indexes fixed NFRC product-name, width, height, and vision
tables and calls `GetWindowAssemblyNfrcForReport` for assembly U-factor, SHGC,
and visible transmittance. Its NFRC factory can substitute
`SurfWinActiveShadedConstruction` according to the live `SurfWinShadingFlag`,
independently of the construction argument supplied by this routine. An
external-library window model additionally overwrites `NominalU` and the local
SHGC through the 90-degree NFRC rating helpers before the ordinary
fenestration row and weighted totals. When Construction EIO output is enabled,
only the first locally seen `(Construction, FrameDivider)` pair writes a
`FenestrationAssembly` row; the predefined per-Surface rows are not
deduplicated.

Shaded-state iteration count comes from
`windowShadingControlList.size()`, while each matching
`shadedConstructionList[i]` is read unchecked. For framed states, NFRC glass
helpers execute before the local `(shaded Construction, FrameDivider)` dedup
test; the first pair then appends glass and assembly state rows and may write
EIO. Unframed states use a separate construction-only dedup path and the NFRC
width/height stored at table index zero. The same shaded Construction paired
with different frames passes pair dedup more than once, yet its predefined rows
use only the Construction name as object key; framed and unframed dedup domains
can likewise converge on that key. Framed and unframed EIO each own a lazy
local header latch, so one invocation may emit two `FenestrationShadedState`
headers with the same tag but different schemas.
When `HasShadeControl` is true, the unchecked active WindowShadingControl
indexes its name, shading type, control type, and glare flag, and the routine
joins every shaded and shaded-storm Construction name in their stored list
orders. A false flag writes only `Switchable = No`.

Exterior Doors append Construction/Zone/Space, no-film U-factor, multiplied
gross area, and parent, then subtract that area from the parent net slot.
Interior Window and `TDD_Dome` rows use the same gross-plus-frame and complete
multiplier basis, but a literal case-sensitive `iz-` name prefix skips all
individual rows, parent subtraction, and interior fenestration totals for a
generated mirror. The Surface still reaches class counting. Interior
`AirBoundary` constructions contribute area and U-factor to the denominator
and U numerator, but intentionally append no SHGC/visible values and add zero
to those numerators, diluting the resulting optical averages. An interior
zero-cache legacy calculation does not write its returned values back to the
Construction. Interior Doors follow the opaque multiplier basis and subtract
their gross area.

After each first-pass Surface, the routine increments its actual class bucket
and, for exterior entries, the matching exterior bucket. An actual Window whose
`OriginalClass` is `GlassDoor` or `TDD_Diffuser` also increments that original
bucket, so it is counted in both categories. The Overhang and Fin buckets are
then overwritten, not incremented, by raw input-object totals for their base
and `:Projection` families. A second complete report-order pass appends net
areas only for exterior or interior Wall, Floor, and Roof; `IntMass` can
accumulate a net slot in the first pass but receives no second-pass net row.

The tail appends exterior overall, north, and non-north fenestration area plus
area-weighted U-factor, SHGC, and visible transmittance, then the analogous
interior totals. A nonpositive denominator writes literal `-` for all three
averages. It finally appends fourteen fixed Surface-class labels to both total
and exterior count columns: Wall, Floor, Roof, Internal Mass, two detached
shading classes, Window, Door, Glass Door, Shading, Overhang, Fin, TDD Dome,
and TDD Diffuser. Even an empty report-order list therefore appends all total
and count rows, and the early assembly header can exist without a later row.

Every `PreDefTableEntry` call appends a new entry; it neither searches nor
overwrites an existing cell. Real entries retain formatted text, original SI
value, and significant-digit metadata for later unit-aware report writing.
That writer groups objects by first-seen object-name order and lets the last
duplicate cell win, while `RetrievePreDefTableEntry` scans forward and returns
the first matching cell. Re-entry can therefore make rendered reports and
programmatic retrieval observe different generations; the shaded-state
construction-key collisions above can create that disagreement within one
invocation. EIO rows remain SI-only and are controlled solely by the
Construction-output flag.

Effects are sequential and nontransactional. Invalid Surface, BaseSurf,
Zone/Space, Construction, FrameDivider, NFRC enum, shading-list, or active
control topology is unchecked locally; assertions and dependency failures can
leave a predefined-entry prefix, mutated nominal caches, and an EIO prefix.
Duplicate report-order indices or names are not deduplicated: they repeat
entries, class counts, total weights, and opening subtraction, can drive parent
net area farther below zero, and expose the writer-last versus retrieval-first
duplicate-cell disagreement.
The ignored nominal-window error and uninitialized optical locals are the
critical ordinary edge. CP154 itself emits no local diagnostic or status
result, rollback, or cleanup, and a failure prevents the parent initialization
remainder even when a dependency diagnosed the cause.

On successful direct re-entry, count/totals/dedup/header locals restart, but
the predefined `tableEntry` arena persists and every row is appended again;
enabled EIO headers and rows also repeat. A nonzero exterior SummerSHGC cache
skips the legacy calculation on the next call, while a physically zero value
recomputes; external-window-model `NominalU`, nominal caches, and dependency
scratch state persist. `HeatBalanceSurfaceManager` clear resets several of its
own arrays and lifecycle flags but owns no CP154 table/EIO state, so it does not
remove duplicate predefined entries or emitted output. `OutputReportPredefined`
clear removes table definitions, column identifiers, and entries, so
`SetPredefinedTables` must rebuild them before retry; Surface/Construction/
Window owner clears rebuild topology and rating caches, and General-data clear
resets the Construction-output flag. EIO stream rewind or truncation belongs to
separate output-file lifecycle, outside both this routine and those report
clears.

Rust's bounded opaque computed geometry, typed frame/divider input, and static
report comparator are separate narrow features, not CP154. Rust has no
predefined-table arena, complete report-order Surface/fenestration topology,
NFRC or external-window-library reporting, shaded-state/control traversal,
net-area/totals/count gatherer, EIO lifecycle, duplicate semantics, or this
failure/re-entry behavior. CP154 therefore adds non-required `source_mapped`
`routine.gather_for_predefined_report` under the existing Surface-manager
algorithm and adds no project contract, source inventory, Rust target/code or
state, test, support, output, numerical, performance, or conformance
promotion. The inventory becomes 32 algorithms and 163 routines, split into
58 `state_mapped` and 105 `source_mapped`, with 58 required. The following
CP155 section maps `AllocateSurfaceHeatBalArrays` before CP156 continues the
Surface-manager source order.

### CP155 `AllocateSurfaceHeatBalArrays` source map

The canonical declaration is `HeatBalanceSurfaceManager.hh` line 101 and the
implementation is `HeatBalanceSurfaceManager.cc` lines 1406-2206. The sole
production `src/` call is `InitSurfaceHeatBalance` line 350, inside its
lines-349-355 `BeginSimFlag` block. The caller has already refreshed each
Surface's outdoor/local-node weather and applied EMS overrides. A false flag
skips both CP155 and the following `InterZoneWindow` reduction. On a true flag,
CP155 must return before lines 351-354 scan solar enclosures; an abnormal exit
also blocks every later initialization and the parent line-620 first-time-flag
clear. Seven EnergyPlus unit-test files make 35 direct calls as downstream
setup, but there is no dedicated zero-Surface or re-entry contract test.

Let `S = TotSurfaces`, `H = Construction::MaxCTFTerms = 19`, and
`L = DataWindowEquivalentLayer::CFSMAXNL = 6`. The allocation phase at lines
1417-1605 touches 136 distinct fields across six owners: 100 in
`HeatBalSurfData`, 15 in `HeatBalFanSysData`, 12 in `MoistureBalanceData`, six
in `HeatBalanceData`, two in `SurfacesData`, and one (`RefAirTemp`) in
`HeatBalanceSurfaceManagerData`. Those fields split into 121 unconditional and
15 conditional fields. The source contains 144 syntactic `allocate`/`dimension`
sites, not 136: each of eight jagged histories has a separate outer allocation
and inner-element dimension site. Thus the syntactic split is 125
unconditional and 19 conditional sites; loop execution expands the inner sites
again for every history term.

The 136 field shapes are 119 ordinary per-Surface arrays, eight rectangular
arrays, eight jagged history arrays, and one EPVector. The two unconditional
window rectangles are `SurfWinQRadSWwinAbs[S, 7]` and
`SurfWinInitialDifSolwinAbs[S, 6]`. Six source/sink rectangles are
`SurfTsrcHist`, `SurfTuserHist`, `SurfQsrcHist` and their three master variants,
all `[S, 19]`. The four unconditional working inside/outside temperature/flux
histories and four conditional master histories each allocate an outer
19-element array, then dimension every inner array to `S`. The EPVector is
`surfQRadFromHVAC[S]`.

Exactly 133 distinct fields receive zero or zero-valued default construction.
The exceptions are `SurfTempEffBulkAir = 23 C`,
`TCondFDSourceNode = 15 C`, and `SurfRoughnessExt = Invalid`, whose enum value
is -1. In source order the routine initializes CTF constants/flags, optional
source constants, bulk-air and convection state, face temperatures and short-
wave state, working and optional master histories, the full reporting and
opaque-conduction bundle, optional source histories, radiant/HVAC/pool/PV
state, moisture arrays, sky/ground incident flags, and exterior/interior
absorptance state. It does not derive values from Surface topology or validate
the counts during this phase.

`AnyInternalHeatSourceInInput` alone gates ten distinct fields: FanSystem
`CTFTsrcConstPart`/`CTFTuserConstPart`, Surface source/user temperatures, and
the six `[S, 19]` source/QTF histories. Independently,
`!SimpleCTFOnly || AnyEnergyManagementSystemInModel` gates
`SurfCurrNumHist` plus the four master jagged histories. Neither false branch
deallocates, clears, or validates a previously allocated field. The four
working jagged histories are always allocated and dimensioned.
`AnyInternalHeatSourceInInput` is evaluated separately at lines 1429 and 1555;
the SimpleCTF/EMS expression is evaluated once at line 1466. No intervening
ordinary CP155 dependency mutates those flags, but the source performs the two
internal-source reads rather than retaining one snapshot.

After the reached allocation phase completes, including for `S == 0`, line
1607 unconditionally displays `Setting up Surface Reporting Variables`. The
routine then visits numeric Surface indices `1..S` and skips only
`!HeatTransSurf`. It adds no Zone, Space, class, construction-validity,
representative, algorithm, used, or report-request filter before the dependency
calls. There are 78 syntactic
`SetupOutputVariable` sites: 77 in the per-Surface union and the unconditional
site-total setup call after the loop. Every call uses Zone timestep; 64 sites
specify `Average` and 14 specify `Sum`. The output dictionary deduplicates by
name and units, while setup counters increment on every call; these no-meter
calls append an active `OutVar` only for a requested name/key match. On the
first call while `OutputInitialized` is false, OutputProcessor initialization
runs before request checking and dictionary insertion. Per-Surface keys are
`Surface.Name`; the final Sum key is `Environment` for `Site Total Surface
Heat Emission to Air`, pointing to the existing `SumSurfaceHeatEmission`
scalar rather than resetting it here.

For every retained heat-transfer Surface, 21 sites are unconditional: inside
face and movable-insulation temperatures; adjacent-air temperature; inside
convection coefficient plus rate/per-area/energy; inside net surface-radiation
rate/per-area/energy; internal-gain and HVAC-radiation rate/per-area/energy;
three inside-convection classification/model/reference-air indices; and the
inside/outside additional heat-source rates per area. The remaining sites use
only these exact source gates:

- `ExtBoundCond != KivaFoundation` adds outside-face temperature (one site).
- `Class != Window` adds inside-face solar and lights rate/per-area/energy
  triples (six sites).
- `ExtBoundCond == ExternalEnvironment || DisplayAdvancedReportVariables`
  adds four outdoor weather values, four outside-convection values, three net
  thermal-radiation values, three air/sky/ground long-wave coefficients, the
  radiation-to-air rate, and heat-emission rate (16 sites). A nested
  `Class != Window` adds the outside absorbed-solar triple (three sites).
- Class in Floor, Wall, `IntMass`, Roof, or Door adds the inside-conduction
  rate/gain/loss/per-area/energy quintet plus inside beam-solar rate (six
  sites). Nested `ExtBoundCond != KivaFoundation` adds outside-conduction,
  average-face conduction, and storage quintets (15 sites).
- Exact `Construct(surface.Construction).SourceSinkPresent` adds
  source-location and user-location temperatures (two sites), independent of
  the earlier global allocation guard.
- `Class == Window` adds shading-on fraction, storm-window status, and blind
  slat angle (three sites).
- Exact `ExtBoundCond == ExternalEnvironment` adds three outside-convection
  classification/forced/natural equation indices; advanced-report display
  alone adds the final Construction index (one site).

The fourteen Sum call sites are the energy member of each applicable
inside convection, inside net radiation, inside solar, inside lights, internal
gain radiation, HVAC radiation, outside convection, outside net radiation,
outside solar, inside/outside/average conduction, and storage group, plus the
site total. All other sites are Average. Comments mentioning advanced opaque
variables do not add a guard: the three inside convection indices are always
given setup calls for retained heat-transfer Surfaces.

Allocation and output-setup effects are sequential and nontransactional.
CP155 itself performs no local count, topology, flag-consistency, allocation,
or output validation and returns no status. A positive inconsistent `S`, an
invalid Surface/Construction, a `SourceSinkPresent` construction with globally
suppressed source arrays, allocation failure, display failure, or output-
processor failure can retain a prefix spanning any of the six owners and any
completed output-setup effects. A negative `S` can reach the EPVector call only
after many Objexx arrays have already mutated; its argument then converts to an
enormous unsigned size at the call boundary, EPVector sets its allocated
sentinel, and the resize can fail. There is no diagnostic, rollback, or cleanup
owned by CP155; dependencies may diagnose or throw while the caller remainder
stays blocked.

With `S == 0`, ordinary/rectangular inner dimensions are empty but all eight
jagged outer arrays still allocate 19 elements when their gates are active, and
`surfQRadFromHVAC.allocate(0)` leaves the empty EPVector's allocated sentinel
true; the Surface loop is empty, yet the progress display and site-total setup
call still occur. On successful direct re-entry, every reached value-bearing
`dimension` refills its array and the unconditional history inners are
redimensioned; a newly false conditional preserves its prior allocation and
contents because there is no else/deallocate branch. Changing `S` can relocate
backing storage before the routine performs output setup again. Dictionary
entries remain deduplicated, but setup counters still advance; for requested
name/key matches, repeated calls can append duplicate active records and leave
earlier requested records pointing at pre-redimension storage while new
requested records point at current cells.

`HeatBalSurfData`, `HeatBalFanSysData`, `MoistureBalanceData`,
`HeatBalanceData`, and `SurfacesData` owner clears reconstruct their objects and
therefore release their CP155 arrays; the HeatBalance clear also restores
`AnyInternalHeatSourceInInput = false` and `SimpleCTFOnly = true`.
`HeatBalanceSurfaceManagerData::clear_state` explicitly clears `RefAirTemp` and
rearms its first-time flags. Global-data clear owns the EMS/BeginSim/advanced-
report flags. OutputProcessor clear is separately required to remove output
registrations; clearing only the six allocation owners does not make those
records or their stored references safe. BeginEnvironment alone neither calls
CP155 nor provides a complete allocation/registration reset.

The allocated state is consumed downstream by Surface initialization,
CTF/EMPD/CondFD and moisture histories, outside/inside balances, radiant/HVAC/
pool/PV coupling, solar distribution, and reporting. Those consumers and
`SetupOutputVariable` are dependency context, not additional CP155 rows.
Rust's bounded Surface vectors, opaque CTF state, frame typing, result store,
and static report comparator do not implement the complete six-owner arena,
fixed/jagged shapes, feature-gated preservation, output registry, pointer
lifecycle, partial failure, or re-entry behavior.

CP155 adds required `source_mapped`
`routine.allocate_surface_heat_bal_arrays` under the existing Surface-manager
algorithm and its project-contract requirement immediately after
`init_surface_heat_balance`. `HeatBalanceSurfaceManager.cc`/`.hh` are already
in the source inventory. CP155 adds no Rust target/code/state, test, support,
capability, output, numerical, performance, or conformance promotion. The
inventory becomes 32 algorithms and 164 routines, split into 58
`state_mapped` and 106 `source_mapped`, with 59 required. The following
CP156 section maps `InitThermalAndFluxHistories` before CP157 continues the
Surface-manager source order.

### CP156 `InitThermalAndFluxHistories` source map

The canonical declaration is `HeatBalanceSurfaceManager.hh` line 103 and the
implementation is `HeatBalanceSurfaceManager.cc` lines 2208-2447. Its sole
production `src/` call is `InitSurfaceHeatBalance` line 383 inside the
lines-379-384 `BeginEnvrnFlag` block; no EnergyPlus unit test calls CP156
directly. Before that block the caller has refreshed outdoor/local-node weather
and EMS overrides, conditionally run CP155 under `BeginSimFlag`, refreshed
construction absorptances under `BeginSimFlag || AnySurfPropOverridesInModel`,
and unconditionally called `UpdateVariableAbsorptances`. The caller's
`InitSurfaceHeatBalancefirstTime` flag gates only the exact line-381 display
`Initializing Temperature and Flux Histories`; CP156 itself has no output or
diagnostic. A false `BeginEnvrnFlag` skips both display and routine. A
successful return next reaches the `AnyMovableInsulation` block, beginning
with CP157 at line 388; an abnormal exit blocks that remainder and the parent
line-620 first-time clear.

Let `ZI = DataHeatBalance::ZoneInitialTemp = 23 C`,
`SI = DataHeatBalance::SurfInitialTemp = 23 C`,
`HC = DataHeatBalance::SurfInitialConvCoeff = 3.076 W/m2-K`, and
`H = Construction::MaxCTFTerms = 19`. CP156 first visits numeric Zones
`1..NumOfZones`. Placement-new reconstructs each existing
`ZoneHeatBalanceData`: `MAT`, `MRT`, `ZTAV`, `ZT`, `ZTAVComf`,
`XMPT`, all four `XMAT`, all four `DSXMAT`, `TMX`, `TM2`, and
`MixingMAT` take `ZI`; `airHumRat`, `airHumRatAvg`, `airHumRatTemp`,
`airHumRatAvgComf`, and `MixingHumRat` first take 0.01; and the remaining
members use zero or zero-valued default construction. The routine then
overwrites only `airHumRatAvg` and `airHumRat` with current
`OutHumRat`, and separately writes Zone `TempTstatAir = ZI`.

Next every existing `EnclRadInfo` record receives `MRT = ZI`. Independently
of `doSpaceHeatBalance`, every existing `spaceHeatBalance` record is
placement-new reconstructed with the same base defaults, then only its
`airHumRatAvg` and `airHumRat` are overwritten by `OutHumRat`. This
enclosure/Space traversal is over the allocated containers rather than the
numeric Zone membership visited by the first loop.

The first Surface phase traverses numeric Zone order, each Zone's stored
`spaceIndexes` order, and every inclusive
`space.HTSurfaceFirst..HTSurfaceLast` index without an additional Surface
predicate. It performs exactly 34 assignments for each reached index:

- `SurfTempEffBulkAir`, `SurfTempIn`, and `SurfTempInTmp` receive `SI`;
  `SurfHConvInt` receives `HC`.
- Thirty fields receive zero: the five
  `SurfHConvExt`/`SurfHAirExt`/`SurfHSkyExt`/`SurfHGrdExt`/
  `SurfHSrdSurfExt` coefficients; `SurfTempOut` and
  `SurfTempInMovInsRep`; the inside-convection triplet; inside net-radiation
  pair; inside solar triplet; lights pair; internal-gain pair; HVAC triplet;
  outside-convection triplet; outside-radiation triplet; and
  `SurfQAirExtReport` plus `SurfQHeatEmiReport`.

Within the same Zone/Space traversal, a nonnegative
`OpaqOrIntMassSurfaceFirst` enables its inclusive range and zeros
`SurfOpaqInsFaceCond`, `SurfOpaqInsFaceCondFlux`,
`SurfOpaqInsFaceCondEnergy`, and
`SurfOpaqInsFaceBeamSolAbsorbed`. A nonnegative `WindowSurfaceFirst`
enables its inclusive range: current/old/outer frame temperatures and
current/old/outer divider temperatures receive `SI`, while
`SurfWinExtIntShadePrevTS` and `SurfWinShadingFlag` receive `NoShade`.
Neither guarded range adds a class check.

The base fixed-history phase again follows Zone, stored Space, then inclusive
heat-transfer range order. Within each Space it places term `1..H` outside
the Surface loop, setting `SurfInsideTempHist` and `SurfOutsideTempHist` to
`SI` and both base flux histories to zero. The first evaluation at line 2339
of `!SimpleCTFOnly || AnyEnergyManagementSystemInModel` conditionally resets
each reached `SurfCurrNumHist` to zero, then uses the same term-outer order to
set both master temperature histories to `SI` and both master flux histories
to zero.

`AnyInternalHeatSourceInInput` is evaluated independently at line 2359. When
true, the loop order is Zone, stored Space, reached Surface, then term
`1..H`: `SurfTsrcHist`, `SurfTsrcHistM`, `SurfTuserHist`, and
`SurfTuserHistM` receive `SI`; `SurfQsrcHist` and `SurfQsrcHistM`
receive zero. Either false feature condition preserves any prior corresponding
allocation and contents. CP156 next copies
`CondFDRelaxFactorInput` into working `CondFDRelaxFactor` unconditionally.

The boundary-dependent base-history loop puts term
`1..MaxCTFTerms + 1` outside stored `AllHTSurfaceList` order. For every
reached Surface and term, `ExternalEnvironment` or
`OtherSideCondModeledExt` replaces outside temperature history with that
Surface's already refreshed local dry-bulb value; `Ground` uses the
BuildingSurface ground temperature; `GroundFCfactorMethod` uses the FCFactor
ground temperature; every other boundary retains its prior `SI` under normal
reachable topology. It then assigns
`q = Construct(surface.Construction).UValue * (OutsideTempHist(1) -
InsideTempHist(1))` to that term's outside flux and copies
`OutsideFluxHist(2)` into that term's inside flux.

The term-outer ordering is observable. Under the preceding zero initialization,
term 1 writes its outside `q` but reads still-zero outside slot 2, so inside
term 1 remains zero. Term 2 first writes outside slot 2 to `q`, after which
inside terms 2 and higher receive that same slot-2 `q`. The calculation
always reads temperature slot 1, not the current term. Normal global
`MaxCTFTerms` is 0 through 18; zero still executes term 1 and reads fixed
slot 2, while 18 reaches all 19 slots. Stored list duplicates repeat writes.

CP156 then traverses `AllHTSurfaceList` once. Each entry whose
`SurfExtCavityPresent` is true resolves `SurfExtCavNum` and overwrites only
that cavity's `TbaffleLast` and `TairLast` with 20 C; duplicate Surface or
cavity references repeat writes. It next traverses stored
`AllHTKivaSurfaceList`. `surfaceConvMap[SurfNum]` inserts a missing default
record, then assigns `.in = KIVA_CONST_CONV(3.076)`, `.f = KIVA_HF_DEF`,
and `.out = KIVA_CONST_CONV(0.0)` in that order.

The SimpleCTF/EMS expression is evaluated a second time at line 2413 rather
than retained from line 2339. When true, each stored all-heat-transfer Surface
is visited. The three special boundary families overwrite all 19 master
outside-temperature slots with their same local dry-bulb or selected ground
temperature; other boundaries retain the earlier `SI`. Terms
`2..Construct(surface.Construction).NumCTFTerms + 1` then copy base
`OutsideFluxHist(2)` into both master flux histories. Master term 1 and terms
above that Construction-specific bound retain zero under normal preceding
initialization; all master inside temperatures remain `SI`.

Finally, only `TotOSCM >= 1` enables numeric `OSCM(1..TotOSCM)` traversal.
Each reached record gets `TConv = 20 C`, `HConv = 4 W/m2-K`,
`TRad = 20 C`, and `HRad = 4 W/m2-K`. CP156 preserves its EMS override
flags and values and every other OSCM member.

CP156 is a selective reset rather than a fresh allocation. Direct re-entry
destructively reconstructs every reached Zone/Space record and rewrites the
reachable ranges, histories, working CondFD factor, listed cavity/Kiva state,
and positive-count OSCMs. A newly false feature condition preserves master or
source histories; Surfaces removed from Zone/Space ranges or stored lists and
map/list entries no longer reached preserve prior state. Changed topology can
therefore leave mixed fresh and stale cells. Any CP155-owned field not listed
above also retains its existing/default value.

With zero Zones, the Zone/range/history phases are empty, but every existing
enclosure and Space heat-balance record is still reset, the CondFD factor is
still copied, and the all-heat-transfer, cavity, Kiva, and OSCM list/count
phases still run. Conventional empty ranges such as `0..-1` execute no
iterations. The routine assumes prior allocation and validated counts,
memberships, ranges, Construction indices, cavity indices, and history bounds.
Normal per-Construction `NumCTFTerms + 1` is at most 19; oversized terms,
invalid indices, inconsistent lists, and missing allocations are unchecked.

There is no return value, local validation, warning, status, catch, rollback,
or cleanup. Nonfinite `OutHumRat`, local dry-bulb, selected ground
temperature, or Construction U-value propagates into reached state.
The concrete Zone/Space placement-new default construction is nonthrowing.
Unchecked container/index access can assert, terminate, or produce undefined
behavior after earlier writes persist. Kiva `operator[]` insertion can throw
on allocation, and `std::function` construction/assignment can throw. A Kiva
failure can leave a newly inserted key and, depending on the failing
assignment, only `.in` or `.in` plus `.f` committed. Any failure blocks
movable-insulation initialization and the parent first-time tail.

The participating reset owners span
`ZoneTempPredictorCorrectorData`, `DataViewFactorInformation`,
`HeatBalanceData`, `HeatBalSurfData`, `HeatBalFanSysData`,
`SurfacesData`, and SurfaceGeometry's Kiva manager. Their respective
`clear_state` paths reconstruct or release their records, arrays, lists,
OSCMs, cavities, and map; no one partial owner clear reproduces CP156. A
successfully reached BeginEnvironment call is the ordinary CP156 reset/rerun
boundary, while the caller first-time flag controls only progress display.

Rust's optional `HeatBalanceCtfInitialHistoryPolicy::EnergyPlusSurfInitial`
seed accepts a configurable initial Surface temperature, selects a typed
boundary temperature, computes steady `1/R * (Tout - Tin)`, and fills
variable-length retained base histories. It gives both inside and outside flux
histories that same value. EnergyPlus instead owns a fixed 19-slot shell with
slot 1 as current and slots 2..19 as prior history, the base inside-slot-1 zero
effect, and the Zone/Space/report/window, master/source, cavity, Kiva, OSCM,
selective-reset, partial-failure, and re-entry behavior above. The bounded Rust
helper is therefore not routine parity.

CP156 adds required `source_mapped`
`routine.init_thermal_and_flux_histories` under the existing Surface-manager
algorithm and its project-contract requirement immediately after
`allocate_surface_heat_bal_arrays`. The source files are already inventoried.
CP156 adds no Rust target/code/state, test, support, capability, output,
numerical, performance, or conformance promotion. The inventory becomes 32
algorithms and 165 routines, split into 58 `state_mapped` and 107
`source_mapped`, with 60 required. The following CP157 section maps
`EvalOutsideMovableInsulation` before CP158 continues the Surface-manager
source order.

### CP157 `EvalOutsideMovableInsulation` source map

The canonical declaration is `HeatBalanceSurfaceManager.hh` line 105 and the
implementation is `HeatBalanceSurfaceManager.cc` lines 2449-2481. Its sole
production `src/` call is `InitSurfaceHeatBalance` line 388, the first child
inside the shared lines-387-390 `AnyMovableInsulation` block. The caller has
already refreshed Surface weather/EMS state, conditionally completed CP155 and
CP156, refreshed active-construction absorptances when required, and always
called `UpdateVariableAbsorptances`. CP157 must return before CP158
`EvalInsideMovableInsulation` runs at line 389. One EnergyPlus unit test calls
CP157 three times with an always-on Schedule: the Regular case checks solar
absorptance, `H`, thermal absorptance, and roughness, while Glass and GlassEQL
check only solar absorptance. It does not exercise the inactive branch.

CP157 traverses `extMovInsulSurfNums` in stored order without sorting,
deduplication, a Surface-class/use/range predicate, or a local count check.
For each raw `SurfNum`, it resolves `extMovInsuls(SurfNum)` and calls the
stored non-null-assumed `sched->getCurrentVal()` exactly once. That getter
returns `EMSVal` when `EMSActuatedOn` is true and otherwise returns the
Schedule's cached `currentVal`; CP157 performs no schedule-type, range, or
finite-value validation.

For `MovInsulSchedVal <= 0.0`, including either signed zero, negative finite
values, and negative infinity, the routine performs this exact inactive order:

1. set `movInsul.present = false`;
2. read current `SurfActiveConstruction(SurfNum)`;
3. read that Construction's exterior `LayerPoint(1)` material;
4. copy the material's raw `AbsorpSolar`, then `AbsorpThermal`, then
   `Roughness` into `SurfAbsSolarExt`, `SurfAbsThermalExt`, and
   `SurfRoughnessExt`; and
5. continue to the next stored list entry.

The inactive branch deliberately does not clear or recompute `movInsul.H`
and does not touch exact field `movInsul.presentPrevTS`. It uses the current
active Construction, including an earlier EMS selection, rather than a stored
original Construction or `movInsul.matNum`. Its copied solar absorptance is
raw even when that active outside layer is a glass-family material.

Every other schedule value takes the active branch; because the only test is
`<= 0.0`, a NaN also reaches this branch. CP157 first resolves
`materials(movInsul.matNum)`, then sets `present = true`, then writes
`H = 1.0 / (MovInsulSchedVal * mat->Resistance)`. It neither clips nor
validates Schedule value or resistance, so positive/negative/zero/infinite/NaN
resistance and positive-infinite/NaN Schedule arithmetic follows the raw
floating-point result.

For material group Glass or GlassEQL, CP157 next `dynamic_cast`s to
`MaterialFen const *`, asserts non-null, and writes
`SurfAbsSolarExt = max(0.0, 1.0 - Trans - ReflectSolBeamFront)`. There is no
upper clamp. With the comparison-style maximum and zero as its first argument,
a finite negative result and negative infinity select zero, positive infinity
is retained, and a NaN optical expression selects zero. Every other material
group copies raw `mat->AbsorpSolar`. Both material paths then copy raw
`AbsorpThermal` and `Roughness` in that order. The active branch also leaves
`presentPrevTS` unchanged.

Both successful branches therefore overwrite the caller's immediately prior
construction and variable-absorptance values for the three exterior Surface
properties. CP157 does not mutate the active Construction, Schedule,
`matNum`, material data, list, `presentPrevTS`, or any inside movable-
insulation state. A false global `AnyMovableInsulation` skips CP157 and CP158
together; a true flag with an empty exterior list makes CP157 a no-op but still
allows CP158.

Effects are stored-order and nontransactional. Duplicate Surface indices repeat
the Schedule read and all reached writes, with the last completed visit
remaining. On direct re-entry, active values recompute `H` and the three
properties; an active-to-inactive transition clears only `present` and
restores current active-construction exterior properties while preserving the
last `H`; an inactive-to-active transition overwrites both. Entries removed
from the stored list receive no CP157 mutation, so their `present`, `H`,
`presentPrevTS`, and CP157 property state persist until another owner/caller
changes them.

The preserved inactive `H` is observable dependency state. The CondFD path
tests only global `AnyMovableInsulation` before reading
`extMovInsuls(Surf).H`, without checking that Surface's `present`; it can
therefore consume a stale value for an inactive or even non-movable Surface.
In particular, an active-to-inactive transition can preserve a positive `H`
that still selects CondFD's movable-insulation branch. Other consumers may add
their own presence guard, but CP157 establishes no global invariant that an
inactive record has zero conductance.

CP157 has no return value, local validation, diagnostic, warning, catch,
rollback, or cleanup. Invalid list indices, unallocated arrays, a null Schedule,
invalid active Construction/layer/material indices, or a mismatched glass
dynamic type can assert, terminate, or produce undefined behavior. A dependency
failure/non-return can preserve completed earlier entries and a current-entry
prefix: inactive `present = false` can survive without all three property
writes, while active `present = true` and `H` can survive before optical,
thermal, or roughness completion. Failure blocks CP158 and the rest of
`InitSurfaceHeatBalance`; conversely, a later CP158 failure leaves completed
CP157 exterior effects committed.

`SurfacesData::clear_state` reconstructs the exterior movable-insulation
records and stored list, restoring `present = false`, exact
`presentPrevTS = false`, and `H = 0.0`; it also rebuilds the active-
construction and Surface-side ownership. `HeatBalSurfData::clear_state`
releases the three copied exterior-property arrays. Material, Construction, and
Schedule owner clears separately rebuild CP157 dependencies. A partial owner
clear is not a CP157 replay, while ordinary caller execution reevaluates the
Schedule whenever the shared global movable-insulation gate is true.

Rust has no exterior movable-insulation input/state record, stored traversal,
current/EMS Schedule controller, active-construction outside-layer restoration,
glass optical branch, conductance lifecycle, CondFD handoff, or matching
failure/re-entry semantics. Existing typed constructions, materials, schedules,
variable absorptance, and bounded Surface heat-balance state are dependencies,
not an implementation analog.

CP157 adds non-required `source_mapped`
`routine.eval_outside_movable_insulation` under the existing Surface-manager
algorithm. It adds no project-contract requirement, source inventory, Rust
target/code/state, test, support, capability, output, numerical, performance,
or conformance promotion. The inventory becomes 32 algorithms and 166
routines, split into 58 `state_mapped` and 108 `source_mapped`, with 60
required. The following CP158 section maps `EvalInsideMovableInsulation`
before CP159 continues the Surface-manager source order.

### CP158 `EvalInsideMovableInsulation` source map

The canonical declaration is `HeatBalanceSurfaceManager.hh` line 107 and the
implementation is `HeatBalanceSurfaceManager.cc` lines 2483-2513. Its sole
production `src/` call is `InitSurfaceHeatBalance` line 389, the second child
inside the shared lines-387-390 `AnyMovableInsulation` block. CP157 must
return before CP158 begins, so a CP158 failure leaves completed exterior
movable-insulation effects while blocking the caller's ground, shading,
radiation, solar-gain, and later initialization remainder. A successful CP158
return resumes the intervening caller work; only if those dependencies also
return does the caller reach unconditional CP159 `InitSolarHeatGains` at line
457.

One EnergyPlus unit test makes three positive always-on calls while reusing the
same concrete `MaterialShade` record and mutating only its `group` tag from
Regular to Glass to GlassEQL. The first call checks inside solar absorptance,
`H`, `present`, and inside thermal absorptance; the later two calls check
only solar absorptance. It does not exercise the inactive branch, roughness,
`presentPrevTS`, duplicate ordering, invalid state, or re-entry transitions.

CP158 traverses `intMovInsulSurfNums` in stored order without sorting,
deduplication, a Surface-class/use/range predicate, or a local count check.
For each raw `SurfNum`, it resolves `intMovInsuls(SurfNum)` and calls the
stored non-null-assumed `sched->getCurrentVal()` exactly once. That getter
returns `EMSVal` when `EMSActuatedOn` is true and otherwise the Schedule's
cached `currentVal`; CP158 performs no schedule-type, range, or finite-value
validation.

For `MovInsulSchedVal <= 0.0`, including either signed zero, negative finite
values, and negative infinity, the routine performs this exact inactive order:

1. set `movInsul.present = false`;
2. read current `SurfActiveConstruction(SurfNum)`;
3. copy that Construction's `InsideAbsorpSolar` into
   `SurfAbsSolarInt(SurfNum)`; then
4. copy `InsideAbsorpThermal` into `SurfAbsThermalInt(SurfNum)` and continue.

The inactive branch deliberately does not clear or recompute `movInsul.H`
and does not touch exact field `movInsul.presentPrevTS`. It uses the current
active Construction, including an earlier EMS selection, rather than base
`Surface.Construction` or `movInsul.matNum`. Unlike CP157, it performs no
outside-layer material lookup and writes no roughness.

Every other schedule value takes the active branch; because the only test is
`<= 0.0`, a NaN also reaches this branch. CP158 first resolves
`materials(movInsul.matNum)`, then sets `present = true`, then writes
`H = 1.0 / (MovInsulSchedVal * mat->Resistance)`. It neither clips nor
validates Schedule value or resistance, so zero, negative, infinite, or NaN
resistance and positive-infinite or NaN Schedule inputs follow raw
floating-point arithmetic.

For material group Glass or GlassEQL, CP158 next `dynamic_cast`s to
`MaterialFen const *`, asserts non-null, and writes
`SurfAbsSolarInt = max(0.0, 1.0 - Trans - ReflectSolBeamFront)`. There is no
upper clamp. Objexx's comparison-style maximum uses zero as its first argument,
so a finite negative result and negative infinity select zero, positive
infinity is retained, and a NaN optical expression selects zero. Every other
group copies raw `mat->AbsorpSolar`. Both material paths then copy raw
`mat->AbsorpThermal` into `SurfAbsThermalInt`. The active branch also leaves
`presentPrevTS` unchanged and writes no roughness.

Both successful branches overwrite the two current inside Surface-property
values, including the construction-derived refresh under the caller's earlier
`BeginSimFlag || AnySurfPropOverridesInModel` gate. The intervening
`UpdateVariableAbsorptances` writes only exterior arrays and contributes no
inside value. CP158
does not mutate active/base Construction identity, Schedule, `matNum`,
material data, the stored list, `presentPrevTS`, any exterior movable-
insulation state, or either roughness field. A false global
`AnyMovableInsulation` skips CP157 and CP158 together; a true flag with an
empty interior list makes CP158 a no-op after CP157.

Effects are stored-order and nontransactional. Duplicate Surface indices repeat
the Schedule read and all reached writes, with the last completed visit
remaining. On direct re-entry, active values recompute `H` and both
absorptances; an active-to-inactive transition clears only `present` and
restores current active-Construction inside absorptances while preserving the
last `H`; an inactive-to-active transition overwrites both. Entries removed
from the stored list receive no CP158 mutation, so their `present`, `H`,
`presentPrevTS`, and CP158 absorptance state persist until another owner or
caller changes them.

Unlike CP157's unguarded CondFD handoff, the standard and optimized inside
Surface-balance paths gate their interior movable-insulation `H` use on the
global flag plus `intMovInsuls(SurfNum).present`, though not on list
membership. A stale inactive `H` alone is suppressed; a removed formerly
active entry that retains `present = true` and `H` can still be consumed
while the global flag remains true. The change flag is observed later in the
same caller, after line-417
`SolarShading::CheckGlazingShadingStatusChange`. Caller line 444 passes
`SurfIterations = 0` into
`HeatBalanceIntRadExchange::CalcInteriorRadExchange`; when not in
BeginEnvironment, each reached non-window enclosure Surface can call
`UpdateMovableInsulationFlag`, which compares `present != presentPrevTS`
and then tests the absolute thermal-absorptance difference between base
`Surface.Construction` and `movInsul.matNum` against strict `> 0.01`. A
NaN difference does not pass that comparison. BeginEnvironment forces the
radiation recalculation separately. `RecKeepHeatBalance` later visits stored
interior-movable order and copies `present` into `presentPrevTS` at line
3051.

CP158 has no return value, local validation, diagnostic, warning, catch,
rollback, or cleanup. Invalid list/array/Construction/material state, a null
Schedule, or a mismatched glass dynamic type can assert, terminate, or produce
undefined behavior. A dependency failure/non-return can retain completed
earlier entries and a current-entry prefix: inactive `present = false` and
solar can survive without thermal completion, while active `present = true`
and `H` can survive before solar or thermal completion. No CP158-owned
diagnostic identifies that prefix.

`SurfacesData::clear_state` reconstructs the interior movable-insulation
records and stored list, restoring `present = false`, exact
`presentPrevTS = false`, and `H = 0.0`; it also rebuilds active-Construction
and Surface ownership. `HeatBalSurfData::clear_state` releases the two copied
inside-absorptance arrays. Material, Construction, and Schedule owner clears
separately rebuild dependencies. A partial owner clear is not a CP158 replay,
while ordinary caller execution reevaluates the Schedule whenever the shared
global movable-insulation gate is true.

Rust has no interior movable-insulation input/state record, stored traversal,
current/EMS Schedule controller, active-Construction inside-absorptance
restoration, glass optical branch, conductance/inside-balance handoff,
long-wave change detection, or matching failure/re-entry lifecycle. Existing
typed constructions, materials, schedules, variable absorptance, and bounded
Surface heat-balance state are dependencies, not an implementation analog.

CP158 adds non-required `source_mapped`
`routine.eval_inside_movable_insulation` under the existing Surface-manager
algorithm. It adds no project-contract requirement, source inventory, Rust
target/code/state, test, support, capability, output, numerical, performance,
or conformance promotion. The inventory becomes 32 algorithms and 167
routines, split into 58 `state_mapped` and 109 `source_mapped`, with 60
required.

### CP159 `InitSolarHeatGains` source map

The canonical declaration is `HeatBalanceSurfaceManager.hh` line 109 and the
complete implementation is `HeatBalanceSurfaceManager.cc` lines 2515-3776.
Its sole production `src/` call is `InitSolarHeatGains(state)` inside
`InitSurfaceHeatBalance` at line 457, and that call is unconditional. The
caller's lines 453-455
`InitSurfaceHeatBalancefirstTime` guard controls only exact progress text
`Initializing Solar Heat Gains`; it does not gate CP159. CP159 returns before
`Dayltg::manageDaylighting` line 459, internal-gain management line 464, and
CP160 `InitIntSolarDistribution` line 468. A CP159 non-return therefore leaves
its completed prefix committed and blocks all of that caller remainder.

There are exactly four direct unit-test calls in three contexts. The
`HeatBalanceSurfaceManager_IncSolarMultiplier` fixture calls CP159 twice at
lines 4959 and 4962 with positive solar and scaler 0.5 then 1.0; its sole
direct result assertion is exact
`transmittedSolarHalf == 0.5 * transmittedSolarWhole` at line 4964.
`WindowFrameTest` calls at line 266 with `I_s = 0`, default-false `SunIsUp`,
and true BeginEnvironment, then separately runs interior distribution and the
window balance; only downstream heat loss greater than heat gain is asserted
at line 318. `CFS_InteriorSolarDistribution_Test` calls at line 7694 with
`I_s = 20`, `SunIsUp = true`, and full interior/exterior distribution, then
mutates a back-surface entry, reruns the child distribution, and checks only
`SurfWinAbsBeam(1)` and `(2)` are zero at lines 7700-7701. These tests do not
directly cover the ordinary-night/sunset split, reflected-solar interpolation,
scheduled inside incidence, BSDF/EQL/external-library branches, TDDs, shelves,
frame/divider optics, representative averaging, invalid state, nonfinite
inputs, failure prefixes, or caller re-entry.

Every call first performs an unconditional reporting/reset prefix, independent
of sun state:

1. Numeric Zones `1..NumOfZones` zero six energy fields
   `ZoneWinHeatGainRepEnergy`, `ZoneWinHeatLossRepEnergy`,
   `ZnOpqSurfInsFaceCondGnRepEnrg`, `ZnOpqSurfInsFaceCondLsRepEnrg`,
   `ZnOpqSurfExtFaceCondGnRepEnrg`, and
   `ZnOpqSurfExtFaceCondLsRepEnrg`, followed by nine rate/report fields
   `ZoneWinHeatGain`, `ZoneWinHeatGainRep`, `ZoneWinHeatLossRep`, and the three
   inside plus three outside opaque conduction total/gain/loss fields.
2. Numeric Solar enclosures zero `EnclSolInitialDifSolReflW`.
3. The routine revisits each Zone's stored `spaceIndexes`. Every inclusive
   opaque/IntMass range, without a local nonnegative-first guard or class
   recheck, zeros `SurfOpaqInsFaceCondGainRep`,
   `SurfOpaqInsFaceCondLossRep`, `SurfOpaqQRadSWInAbs`,
   `SurfQdotRadLightsInPerArea`, `SurfOpaqQRadSWOutAbs`,
   `SurfOpaqInitialDifSolInAbs`, `SurfOpaqInsFaceBeamSolAbsorbed`,
   `SurfOpaqSWOutAbsTotalReport`, and `SurfOpaqSWOutAbsEnergyReport`.
4. Each inclusive Window range is traversed three times. The first pass zeros
   frame/divider outside and inside absorbed fluxes, shade short-/long-wave
   absorption, natural/gain/return convective flows, and divider heat gain.
   The second zeros the nine glazing/short-wave/frame-divider/shade/gap gain
   reports through `SurfWinSysSolTransmittance`; the third zeros seven
   heat-gain/loss and energy reports through
   `SurfWinShadingAbsorbedSolarEnergy`. A layer-outer
   `1..CFSMAXNL + 1` pass then zeros `SurfWinQRadSWwinAbs` for the same range.

If `InitSurfaceHeatBalancefirstTime` is true, CP159 next visits all numeric
Surfaces and seeds both `SurfBmToDiffReflFacGnd` and
`SurfSkyDiffReflFacGnd` from `Surface.ViewFactorGround`. CP159 never clears the
flag. The parent clears it only at its successfully reached line-620 tail, so
direct tests and a failed parent re-entry can repeat this seed. A later
reset-only block can overwrite the beam-to-diffuse factor back to zero when
`CalcSolRefl` is true, but does not symmetrically clear the sky factor.

The solar-state latch is then evaluated in exact order:

- `currSolRadPositive = SunIsUp && (BeamSolarRad + GndSolarRad +
  DifSolarRad > 0.0)`;
- `sunset = !currSolRadPositive && PreviousSolRadPositive` and
  `sunIsUpNoRad = SunIsUp && !currSolRadPositive`;
- `resetSolar = BeginEnvrnFlag || sunIsUpNoRad || sunset`; then
- `PreviousSolRadPositive = currSolRadPositive` is committed before either
  reset block or any active-solar calculation.

The test is on the signed raw sum, not each component; either signed zero, a
nonpositive finite sum, negative infinity, or NaN is inactive, while positive
infinity is active.
An ordinary sun-down call with previous latch false and BeginEnvironment false
therefore performs only the unconditional prefix. BeginEnvironment and
sun-up/no-radiation remain retry-stable, but sunset is edge-triggered: if a
sunset-only call fails after the latch assignment, a same-state retry no longer
sees `sunset` and can skip completion of the broad reset.

When `currSolRadPositive || resetSolar`, the routine zeros a second, wider
solar-owned set:

- all numeric Surfaces: five inside-beam intensity/amount/energy reports, the
  total/beam/sky/ground incident reports, five ground/obstruction reflected
  incident components, and `SurfSkySolarInc` plus `SurfGndSolarInc`;
- all Solar enclosures: `ZoneTransSolar`, four exterior/interior beam/diffuse
  reports, and their five energy counterparts;
- every stored Window range: ten shade/total/initial-transmission fields,
  thirteen blind/screen/glass transmission fields, nine interior-window and
  reveal working fields, six reveal reports plus three energies, and thirteen
  total/beam/diffuse transmission and BSDF direction/theta/phi reports;
- layer-outer `1..MaxSolidWinLayers`: `SurfWinQRadSWwinAbsLayer`; and
  layer-outer `1..CFSMAXNL`: `SurfWinInitialDifSolwinAbs`.

The thirteen transmission/BSDF fields in the last Window pass include
`SurfWinTransSolar`, `SurfWinBmSolar`, `SurfWinBmBmSolar`,
`SurfWinBmDifSolar`, `SurfWinDifSolar`, their five energy forms,
`SurfWinBSDFBeamDirectionRep`, `SurfWinBSDFBeamThetaRep`, and
`SurfWinBSDFBeamPhiRep`. This wider reset still does not clear
`SurfWinSkyGndSolarInc` or `SurfWinBmGndSolarInc`; a reset-only call can leave
those complex-window ground-split values stale while the main incident arrays
are zero. It also reaches only stored Zone/Space Window ranges, so TDD domes
outside those ranges need the separate handling below.

When `resetSolar` is true, numeric Solar enclosures additionally zero
`EnclSolQD` and `EnclSolQDforDaylight`, but not positive-only
`EnclSolQSDifSol`. Each stored TDD pipe then zeros its
four solar/visible transmittances and `TransmittedSolar`, zeros the dome's
`SurfWinTransSolar`, total outside incident rate, and total absorbed-window
rate, and zeros dome `SurfWinQRadSWwinAbs` layers `1..CFSMAXNL + 1`. It does
not zero the dome's per-layer W reports or total absorbed energy. If
`CalcSolRefl` is true, all numeric Surfaces zero
`SurfBmToBmReflFacObs`, `SurfBmToDiffReflFacObs`, and
`SurfBmToDiffReflFacGnd`. A final all-Surface pass zeros initial-diffuse and
inside-short-wave absorption reports, reported incidence cosine, horizontal
and vertical profile angles, and system solar reflectance/absorptance. It does
not clear `SurfSkyDiffReflFacGnd`, either complex-window ground split,
positive-only `Surface.IncSolMultiplier`, custom-only
`Surface.GndReflSolarRad`, the three `AbsDiffWin*` scratch arrays,
scheduled mutations of `SurfWinA`, or all TDD dome W/energy state.

Only `currSolRadPositive` enters the calculation branch. Its first two actions
assert equal dimensions between the beam-to-beam obstruction reflection table
and each beam-to-diffuse obstruction/ground table, even when `CalcSolRefl` is
false. It computes
`GndSolarRadInc = max(BeamSolarRad * SOLCOS(3) + DifSolarRad, 0.0)`.
Here the expression is Objexx `max`'s first operand, so NaN and positive
infinity are retained; finite negative values and negative infinity select
zero.

CP159 then visits every numeric Surface, not only solar/exterior lists, and
stores `Surface.IncSolMultiplier = GetSurfIncidentSolarMultiplier(...)`.
That helper returns 1.0 when `hasIncSolMultiplier` is false, otherwise returns
`Scaler`, multiplied by one current Schedule read when the stored Schedule
pointer is non-null. It directly indexes
`SurfIncSolMultiplier(SurfNum)` rather than following a separately stored
Surface identity, assuming those arrays are aligned. CP159 adds no finite/range
clamp. A second all-Surface
pass writes `SurfSkySolarInc = DifSolarRad * multiplier * SurfAnisoSkyMult`.
For a custom ground-reflectance property it computes and stores
`Surface.GndReflSolarRad = GndSolarRadInc * multiplier * SurfsReflAvg`; the
default path instead uses `GndSolarRad * multiplier` only as a local and leaves
the persistent Surface cache unchanged. Both paths write
`SurfGndSolarInc = local * ViewFactorGround`, copy that result to
`SurfWinSkyGndSolarInc`, and set `SurfWinBmGndSolarInc = 0`.

Under `CalcSolRefl`, the routine forms linear current-hour and previous-hour
offsets and visits all numeric Surfaces. It selects custom or environment
ground reflectance and scales current beam/diffuse by the stored multiplier.
It first writes the complex-window sky-ground split from
`SurfReflFacSkySolGnd` and writes the beam-ground split using the existing
`SurfBmToDiffReflFacGnd`; only afterward does it interpolate
`SurfBmToBmReflFacObs`, `SurfBmToDiffReflFacObs`, and
`SurfBmToDiffReflFacGnd` with `WeightNow` and `WeightPreviousHour`. Thus the
beam-ground split can use the prior factor while the subsequently recomputed
main `SurfGndSolarInc` uses the new factor. It adds interpolated beam
specular/diffuse obstruction terms and sky-diffuse obstruction reflection to
`SurfSkySolarInc`, overwrites `SurfGndSolarInc` with beam- plus sky-ground
reflection, and copies `SurfReflFacSkySolGnd` to
`SurfSkyDiffReflFacGnd`. The two complex-window split values are not updated
again from that recomputed main ground total.

Dependency order is then exact: unconditional
`SolarShading::CalcWindowProfileAngles`; optional
`CalcBeamSolarOnWinRevealSurface`; and either, for an external window library
plus simplified optical model, `CalcAbsorbedOnExteriorOpaqueSurfaces` followed
by `CalcInteriorSolarDistributionWCESimple`, or otherwise
`CalcInteriorSolarDistribution`. The simplified branch redundantly retests
`isSimplifiedModel()` before its second call. These children own much of the
beam distribution, `SurfWinA`, transmitted-solar, reveal, and enclosure input
state consumed below; their diagnostics and non-return behavior remain
dependency effects rather than CP159-local diagnostics.

After those calls, every Solar enclosure receives the ordered formulas

`EnclSolQDforDaylight = (EnclSolDB - EnclSolDBIntWin) * BeamSolarRad +
EnclSolDBSSG + EnclSolInitialDifSolReflW`

and

`EnclSolQD = EnclSolDB * BeamSolarRad + EnclSolDBSSG +
EnclSolInitialDifSolReflW`.

`EnclSolQSDifSol` first copies the daylight form. If interzone windows exist,
only a receiver with `EnclSolRecDifShortFromZ` true adds off-diagonal
`ZoneFractDifShortZtoZ(receiver, source) * source.QDforDaylight`, and only from
other flagged enclosures. A final all-enclosure pass multiplies by
`solVMULT`; the interzone case additionally multiplies by the diagonal
`ZoneFractDifShortZtoZ(enclosure, enclosure)`. There is no normalization,
range check, or finite-value guard in CP159.

Exterior incident reporting follows four distinct traversals and preserves
their overwrite asymmetries:

1. If any Building, Fixed, or Attached shading count is nonzero, CP159 visits
   the full inclusive `ShadingSurfaceFirst..ShadingSurfaceLast` range without
   another count/class filter. With `B = BeamSolarRad * multiplier`,
   `D = DifSolarRad * multiplier`, `F = SurfSunlitFrac`, and the tabulated
   cosine `C`, it reports beam `B*F*C`, sky diffuse
   `D*SurfAnisoSkyMult`, aggregate ground from `SurfGndSolarInc`, and the
   beam-/sky-to-diffuse ground components. Total incident is beam + sky + the
   two ground components; obstruction-reflection components are not added in
   this shading traversal even when `CalcSolRefl` is true.
2. A freshly zeroed local `currBeamSolar(TotSurfaces)` is populated in stored
   `AllExtSolarSurfaceList` order. Each visit repeats the same basic reports,
   then, under `CalcSolRefl`, writes three obstruction-reflection reports and
   adds them to total incident. Duplicate list entries simply overwrite their
   prior visit. An overlap with the shading range receives this later regular
   calculation.
3. Each TDD pipe rewrites its diffuser using the diffuser's active
   Construction but the dome's cosine and sunlit fraction. It divides beam
   and anisotropic-sky `TransTDD` products, and dome ground times
   `TransSolIso`, by the active diffuser `TransDiff` with no zero guard. It
   overwrites diffuser cosine, local beam, sky, ground, beam report, sky report,
   and total. That total combines the new beam/sky with the earlier
   beam-/sky-ground component reports; CP159 does not rebuild those components,
   aggregate ground report, or obstruction terms for the diffuser.
4. Every daylighting shelf uses its Window as target and indexes `OutSurf`
   without a local `> 0` guard. It first copies the target's tabulated cosine
   to its reported incidence cosine and overwrites the target's local beam with
   `BeamSolarRad * target multiplier`. Shelf radiation is
   `(B * sunlit(out) * cosine(out) + D * anisotropic(out)) * OutReflectSol`.
   The target Window ground becomes default `GndSolarRad * multiplier` or its
   custom-only cached `Surface.GndReflSolarRad`, times target ground view
   factor, plus shelf radiation times shelf view factor. Duplicate shelves for
   one target overwrite rather than accumulate that ground assignment. This
   occurs after outside incident reports and does not recompute their ground
   components or total; later absorption consumes the shelf-overwritten ground
   and local-beam values.

The absorbed-short-wave phase again follows Zone then stored Space ranges.
For each opaque/IntMass entry CP159 recomputes local beam from that Surface's
stored multiplier. An `ExtSolar` Surface gets
`SurfOpaqQRadSWOutAbs = SurfOpaqAO * beam + SurfAbsSolarExt *
(SurfSkySolarInc + SurfGndSolarInc)`. If base `Surface.Construction > 0`, the
base Surface/Construction pair is searched by `SurfaceScheduledSolarInc`.
That helper scans numeric entries `1..TotSurfIncSolSSG` and returns the first
exact Surface/Construction pair. `WindowScheduledSolarAbs` below similarly
scans `1..TotFenLayAbsSSG` and returns the first exact pair, but receives the
active Construction. With no scheduled opaque pair and base
`Construct.TransDiff <= 0`, inside absorption
adds `SurfOpaqAI * beam`; its W report multiplies by Surface area, except an
identified inside shelf uses half the already doubled area. With a scheduled
pair, the routine instead adds bare `SurfOpaqAI` without beam scaling and
leaves the beam-absorbed W report at its reset value. This path uses base, not
active, Construction identity for both lookup and opacity. Opaque range
overlap repeats the calculation: outside and beam-W assignments overwrite,
while the inside absorbed-rate `+=` operation accumulates each visit.

The Window loop first dereferences `Surface(SurfNum)` and
`SurfaceWindow(SurfNum)` for every range entry, then processes optical/frame
work only for `ExtSolar` or original class `TDD_Diffuser`. It normally uses
`SurfActiveConstruction`, previously populated `currBeamSolar`, current
sky/ground incidence, and current shading flag. An otherwise reached,
unrecognized nonexternal model performs no layer writes but still continues
to common frame/divider work:

- Detailed non-external windows copy active unshaded `AbsDiff` for each glass
  layer. An active shade/screen replaces layer diffuse absorptance from
  `SurfWinActiveShadedConstruction` and computes shade diffuse absorption. An
  active blind interpolates glass total/ground/sky diffuse absorptances and
  blind absorptance across low/high slat indices; horizontal slats replace the
  total with separate sky/ground formulas weighted by
  `0.5 * abs(CosTilt)`. Interior shade/blind diffuse absorption is multiplied
  by `glazedFrac` when divider area is positive. Switchable glazing
  interpolates each layer's unshaded and shaded diffuse absorptance. The branch
  resets total W locally, writes each layer as diffuse absorption times
  `(sky + ground)` plus `SurfWinA * beam`, repeats the split sky/ground formula
  for horizontal-blind glass, writes layer W as rate times Surface area,
  accumulates total W, and writes total energy after the loop.
- BSDF uses the active Construction's solid-layer count and current complex
  state. The first matching `WindowScheduledSolarAbs` pair samples every layer
  Schedule, mutates `SurfWinA`, and copies that scheduled value directly into
  layer absorption. Otherwise the layer formula is current-state sky
  absorptance times sky + ground absorptance times ground +
  `SurfWinA * beam + SurfWinACFOverlap * beam`. It writes layer W, accumulates
  total W, and then energy. This branch relies on the earlier broad reset and
  has no branch-local total zero.
- EQL resets total W locally. Its layer count comes from the base
  `Surface.Construction` EQL system, while each layer's
  `AbsDiffFrontEQL` comes from the active Construction; absorption is
  `SurfWinA * beam + AbsDiffFrontEQL * (sky + ground)`, followed by layer W,
  total W, and energy.
- The external-library branch substitutes the dome only for an original TDD
  diffuser's incoming-angle calculation, obtains the active Construction's
  Solar equivalent layer, and uses front diffuse layer absorptance times
  `(sky + ground) + SurfWinA * beam`. It writes layer W and accumulates total
  W, but never writes `SurfWinQRadSWwinAbsTotEnergy`. Like BSDF, it relies on
  the earlier range reset rather than a local total zero.

The detailed branch initially resolves shaded optics through array
`SurfWinActiveShadedConstruction(SurfNum)`, while horizontal-blind
recomputation and frame/divider switchable or ExtShade paths use member
`Surface(SurfNum).activeShadedConstruction`; CP159 does not validate that
those two owners agree. Likewise, window absorption consumes the earlier local
beam population rather than recomputing it. A malformed list/range mismatch
can therefore leave zero local beam. Overlapping Window ranges repeat work:
Detailed and EQL reset totals per occurrence, while BSDF and external-library
totals can accumulate because their reset occurred only before the ranges.

Frame and divider absorption follows every window-model branch. For positive
frame or divider area CP159 sets face beam `B * sunlit * cosine` and diffuse
face incidence `sky + ground`. Only positive sunlit fraction plus a positive
projection computes horizontal and vertical projection cosines from Surface
normal and `SOLCOS` via `asin`, `atan2`, and absolute dot-product forms.

For positive frame area, outside incidence begins with face beam, adds
horizontal/vertical projection beam times outside projection, then diffuse
times `(1 + 0.5 * SurfWinProjCorrFrOut)`. Inside incidence and the local diffuse
transmittance start at zero. Only when `FrProjIn > 0` inside the any-positive-
projection block does the source evaluate the active beam polynomial, assign
active `TransDiff`, and form the inside beam contribution; otherwise the inside
projection contribution remains zero. Switchable glazing interpolates both
transmittances with the active shaded Construction under the same guard.
Outside and inside results
are multiplied by frame solar absorptance, then their respective reveal-
diffuse beam terms are added with the same absorptance. A shade/blind is not
treated as covering the frame.

For positive divider area, a suspended divider first replaces its raw
absorptance using outer-glass `Trans`, front reflectance, and
`Abs = 1 - Trans - Refl`; switchable glazing interpolates those three optical
values. Effective divider absorptance is
`AbsGl + TransGl * (DividerAbs + DividerRefl * AbsGl) /
(1 - DividerRefl * ReflGl)` without a denominator guard. Outside projection
beam is calculated only without exterior shade/blind/screen. With neither
exterior nor between-glass shading, divider outside beam/diffuse use face plus
projection and `(1 + projection-correction)` forms, and positive inside
projection uses active beam/diffuse transmittance. Exterior or between-glass
shading instead uses projection-corrected face beam/diffuse on both sides and
active `TransDiff` inside.

The final divider absorber is selected only by these cases: no exterior or
between-glass device uses raw effective absorptance; ExtBlind multiplies beam
by interpolated blind beam-diffuse plus beam-beam transmittance and diffuse by
front diffuse transmittance; ExtShade multiplies all by the first shaded-layer
material transmittance; ExtScreen obtains relative phi/theta and uses either
the compile-time bilinear table path or `CalcScreenTransmittance`, multiplying
by beam-beam plus beam-diffuse transmittance. A between-glass flag enters the
alternate incidence formulas but matches none of these final absorption cases,
so its values remain at the earlier reset. Material dynamic types and the two
screen build variants are not normalized by CP159.

After all Zone/Space windows, each TDD dome is recalculated from the dome's
base `Surface.Construction`, unlike the active diffuser Construction used
earlier. It resets dome total W, copies base diffuse absorptance, computes each
glass layer from dome sky/ground plus the prepopulated local beam, writes layer
W, accumulates total W, and writes total energy inside the layer loop. Zero
glass layers therefore perform no energy write; multiple layers expose a
partially updated energy after each completed layer.

The positive branch ends with optional representative-surface averaging.
Every stored opaque/IntMass representative having more than one constituent,
without an `ExtSolar` test, replaces only its outside and inside per-area
absorbed fluxes by raw area-weighted constituent averages. Every `ExtSolar`
Window representative with more than one constituent uses the representative
active Construction's `TotGlassLayers`, regardless of BSDF/EQL/external model,
and replaces only per-area `SurfWinQRadSWwinAbs` for those layers. If it owns
a frame/divider definition
and its representative frame or divider area is positive, the corresponding
outside and inside per-area values are averaged with constituent frame or
divider areas. Duplicate, self, invalid, or heterogeneous constituent topology
is unchecked, and stored Zone/Space order permits a later representative to
consume a constituent representative already averaged earlier in the same
traversal. Every division uses raw summed constituent area with no local zero
guard. Crucially, this tail does not recompute Window per-layer W,
`SurfWinQRadSWwinAbsTot`, total energy, or any other report after replacing
per-area values, so those reports can remain inconsistent with the final
representative fluxes.

CP159 has no return value, direct `Show*` diagnostic, local validation, catch,
rollback, or cleanup. It assumes all counts, ranges, stored lists, enclosure
matrices, layer limits, Construction/material/state indices, Schedule pointers,
TDD/shelf links, and output arrays are valid. The source asserts reflection-
table dimensions and the suspended/switchable glass, ExtShade, and ExtScreen
material casts; blind-material casts are dereferenced without an assertion.
Unchecked divisions
include TDD `TransDiff`, positive representative sums that can still have zero
constituent area, frame/divider area expressions, and the suspended-divider
multiple-reflection denominator. Invalid trigonometric inputs, nonfinite
optics/radiation/weights/areas, dependency diagnostics, allocation failure,
unchecked indexing, or a failed assertion can propagate, throw, terminate, or
leave undefined behavior after an ordered mutation prefix.

Re-entry is selective rather than transactional. The unconditional prefix
always repeats; broad solar resets depend on the new latch predicates; positive
calls reevaluate every multiplier and child distribution. Removed list/range
members and newly false gates can preserve stale fields. In addition to the
sunset retry edge, persistent selective state includes reset-missed
`EnclSolQSDifSol`, complex ground splits and sky factor, positive-only
multipliers, custom-only ground cache, scratch diffuse arrays,
scheduled-mutated `SurfWinA`, external-library energy, and TDD dome W/energy.
Failure after line 2651 advances the radiation
latch before later state is complete; failure in any child or later loop leaves
the completed Zone/Surface/enclosure prefix and blocks caller daylighting and
CP160.

Six directly mutated state owners define the full clear boundary.
`HeatBalanceData::clear_state`, `HeatBalSurfData::clear_state`,
`SurfacesData::clear_state`, `EnvironmentData::clear_state`, and
`DataDaylightingDevicesData::clear_state` reconstruct their records by
placement-new. Environment reconstruction restores
`PreviousSolRadPositive = false`; Daylighting reconstruction removes/redefaults
TDD state; the Surface and heat-balance owners release/rebuild the arrays,
lists, ranges, enclosure and report state. `HeatBalSurfMgr::clear_state`
restores `InitSurfaceHeatBalancefirstTime = true` and recreates
`AbsDiffWin`, `AbsDiffWinGnd`, and `AbsDiffWinSky` at `CFSMAXNL`.
Construction, Material, Schedule, SolarShading, ViewFactor, and WindowManager
clears separately rebuild dependencies. No partial owner clear is a CP159
replay, and BeginEnvironment alone is only one reset predicate.

Rust has no `InitSolarHeatGains` target or state machine. The typed incident-
solar multiplier and scheduled-inside-solar declarations remain run-blocked
input snapshots. `ep_runtime::heat_balance::solar` explicitly labels its
sun-exposed outdoor-Surface incident series a forcing diagnostic rather than a
full distribution/shadowing claim, and `surface_balance` has only bounded,
clamped opaque exterior absorption plus separately supplied inside-short-wave
state. Those helpers do not implement the CP159 latch/reset topology,
Zone/Space/Solar-enclosure exchange, Schedule multipliers, obstacle/ground
reflection state, window layers/shades/BSDF/EQL/external optics, TDDs, shelves,
frames/dividers, representative overwrites, report/energy lifecycle, or
failure/re-entry semantics.

CP159 adds required `source_mapped` `routine.init_solar_heat_gains` under the
existing Surface-manager algorithm and the matching project-contract
requirement immediately after `init_thermal_and_flux_histories` and before
`calc_heat_balance_outside_surf`. It adds no EnergyPlus source inventory, Rust
target/code/state, test, support, capability, output, numerical, performance,
or conformance promotion. The inventory becomes 32 algorithms and 168
routines, split into 58 `state_mapped` and 110 `source_mapped`, with 61
required.

### CP160 `InitIntSolarDistribution` source map

The canonical declaration is `HeatBalanceSurfaceManager.hh` line 111 and the
complete implementation is `HeatBalanceSurfaceManager.cc` lines 3778-4177.
Its sole production `src/` call is unconditional
`InitIntSolarDistribution(state)` inside `InitSurfaceHeatBalance` line 468.
The caller's lines-465-467 `InitSurfaceHeatBalancefirstTime` guard controls
only exact progress text `Initializing Interior Solar Distribution`; it does
not gate CP160. Runtime order is CP159, daylighting, internal-gain management,
then CP160. A CP160 non-return therefore preserves all completed predecessor
and CP160 mutations, blocks `InitIntConvCoeff` line 473 and the caller
remainder, and prevents the successful line-620 first-time-flag clear.

There are exactly two direct unit-test calls, both in
`HeatBalanceSurfaceManager_TestInitHBInterzoneWindow`. After two full
`InitSurfaceHeatBalance` calls establish a one-enclosure, six-square-metre
fixture, the test disables `InterZoneWindow`, sets
`ZoneBmSolFrIntWinsRep(1) = 10 W`, and calls CP160 at line 4880 with
`SunIsUp = false`. It observes only that an already-zero
`SurfIntBmIncInsSurfIntensRep(1)` remains zero. It then sets
`SunIsUp = true`, calls again at line 4883, and checks only
`10 / 6 = 1.666667 W/m2` within tolerance. The night call does not bypass
the routine: only the interior-window beam-report block is gated by
`SunIsUp`; all enclosure, internal-short-wave, opaque, window, and TDD work
still executes. No direct test covers nonzero `QLTSW`, off-diagonal
interzone transfer, absorbed opaque or window flux, shading, movable
insulation, adjacency, EQL/BSDF/external-window optics, TDDs, nonfinite or
invalid state, failure prefixes, or additive re-entry.

Every call first rebuilds two Solar-enclosure working fluxes without a sun
gate. For each numeric enclosure, CP160 visits stored
`EnclSolInfo(enclosure).spaceNums` in order and sums each
`spaceIntGain(space).QLTSW`. It assigns

`EnclSolQSWRad = EnclSolQD + sum(QLTSW)`

and

`EnclSolQSWRadLights = sum(QLTSW)`.

Those inputs are powers in W. Empty membership gives a zero lights sum;
duplicates count repeatedly. The two targets are assigned, not accumulated,
at this stage.

When `InterZoneWindow` is true, each receiver whose
`EnclSolRecDifShortFromZ(receiver)` flag is true scans every different
enclosure whose same flag is also true. The source `QLTSW` sum is recomputed
for every qualifying receiver/source pair. In matrix index order CP160 adds

- `ZoneFractDifShortZtoZ(receiver, source) * (source.EnclSolQD +
  source.sumQLTSW)` to receiver `EnclSolQSWRad`;
- the same fraction times `source.sumQLTSW` to receiver
  `EnclSolQSWRadLights`; and
- the same fraction times `source.EnclSolQD` to
  `ZoneDifSolFrIntWinsRep(receiver)`.

Immediately after each qualifying source, not after the complete source loop,
it assigns `ZoneDifSolFrIntWinsRepEnergy = cumulative report *
TimeStepZoneSec`. CP160 does not reset either Zone diffuse report. A receiver
with no qualifying source receives no report-energy write, and there is no
fraction normalization, range check, or solar-state gate.

Only when `SunIsUp` is true does CP160 visit stored
`AllHTSurfaceList` order for beam reporting. It skips only a Surface whose
current class is `Shading`. For every other entry it uses the Surface Solar
enclosure to assign

- intensity = `ZoneBmSolFrIntWinsRep / EnclSolInfo.TotalSurfArea` in W/m2;
- amount = intensity times `(Surface.Area + SurfWinDividerArea)` in W; and
- energy = amount times `TimeStepZoneSec` in J.

Sun-down leaves all three prior values untouched rather than zeroing them.
The total-area division is unchecked.

CP160 next converts each enclosure's just-built powers to absorbed flux
density. With interzone windows, both working values are multiplied by the
diagonal `ZoneFractDifShortZtoZ(enclosure, enclosure)` and
`solVMULT`; otherwise they are multiplied only by `solVMULT`.
`solVMULT` is the precomputed reciprocal of summed
inside-area-times-absorptance, so the resulting
`EnclSolQSWRad` and lights-only value are W/m2. This scaling does not alter
the Zone diffuse report or the earlier beam report.

Radiant absorption then traverses numeric Zones, each stored
`Zone.spaceIndexes`, and each Space's inclusive opaque/IntMass range followed
by its inclusive Window range. CP160 does not sort, deduplicate, validate
first/last bounds, or recheck membership or class; repeated or overlapping
ranges repeat mutations.

For every opaque/IntMass entry CP160 resolves the base
`Surface.Construction`. It adds enclosure total flux times the current
`SurfAbsSolarInt` to `SurfOpaqQRadSWInAbs`, but adds lights-only flux times
the base Construction's `InsideAbsorpSolar` to
`SurfQdotRadLightsInPerArea`. Thus an earlier inside movable-insulation
absorptance can affect total short wave while the lights-only path continues
to use the base Construction value.

Under global `AnyMovableInsulation`, every reached opaque entry also
dereferences `extMovInsuls(SurfNum)`, regardless of exterior-list membership.
If that record is present, let `M` be the base Construction's first-layer
material and `Aext` the current `SurfAbsSolarExt`. The exact ordered
overwrite is

`SurfQRadSWOutMvIns = SurfOpaqQRadSWOutAbs * Aext / M.AbsorpSolar`

then

`SurfOpaqQRadSWOutAbs = Tmov * SurfQRadSWOutMvIns *
((M.AbsorpSolar / Aext) + (1 - M.AbsorpSolar))`.

`Tmov` is the movable material's `MaterialFen.Trans` when that dynamic cast
succeeds and zero otherwise. Both divisions are raw and zero-prone; the
nullable cast is not asserted. This can overwrite CP159's exterior absorbed
solar after the two inside additions. A direct retry feeds that overwritten
outside value back into the next `SurfQRadSWOutMvIns` calculation. A false
global gate or no-longer-present record skips both assignments and can leave
the prior `SurfQRadSWOutMvIns` and transformed outside value stale.
Regardless of that gate, CP160 next adds `SurfOpaqInitialDifSolInAbs` to the
opaque inside absorbed flux; when the movable path is reached, this addition
occurs only after it returns.

Each Window entry immediately dereferences `Surface`, `SurfaceWindow`,
its radiant and Solar enclosure indices, and its current
`SurfActiveConstruction`; there is no exterior-solar or class gate. The
`WindowModel` enum has Detailed, BSDF, and EQL values. CP160 has no separate
external-window-library branch: any such window represented as Detailed
follows the same conventional path. An invalid enum also satisfies the
non-EQL test but misses the later model-specific initial-diffuse tail.

Both the non-EQL and EQL paths overwrite
`SurfQdotRadIntGainsInPerArea`. Normally the value is
`radQThermalRad * radThermAbsMult * SurfAbsThermalInt`. During
`doLoadComponentPulseNow`, CP160 first adds
`0.01 * EnclRadInfo.FloorArea` to raw `radQThermalRad`, then applies the
same multiplier and absorptance. This duplicate calculation follows
`ManageInternalHeatGains`; it is not additive. Frame and divider formulas
below still use the unpulsed `radQThermalRad`.

For a non-EQL window, the active Construction supplies
`TotGlassLayers` and unshaded optics, while
`SurfWinActiveShadedConstruction` supplies the shaded Construction index:

- `NoShade` or `ShadeOff` adds
  `EnclSolQSWRad * active.AbsDiffBack(layer)` to every active glass layer.
- With a nonzero shaded Construction and any other nonswitchable flag,
  shade/screen flags add its `AbsDiffBack` to each shaded glass layer.
  `IntBlind` and `ExtBlind` instead linearly interpolate the low/high slat
  layer back-diffuse absorptances using raw, unclamped
  `slatAngInterpFac`. `BGBlind` is not included in this glass-layer blind
  condition.
- `IntShade` assigns interior-shade long-wave absorption from unpulsed
  `radQThermalRad * ShadeAbsorpThermal * radThermAbsMult`; `IntBlind`
  assigns the analogous value using `effShadeEmi`. Other shade positions do
  not assign that long-wave field.
- Any shade/screen assigns
  `SurfWinIntSWAbsByShade = EnclSolQSWRad * AbsDiffBackShade`; any blind,
  including `BGBlind`, assigns it from the raw low/high slat interpolation of
  back diffuse blind absorption. Exterior shade/blind/screen then multiplies
  that value by `SurfaceWindow.glazedFrac`; between-glass and interior
  devices do not.
- Switchable glazing dereferences its shaded Construction even when the index
  is zero and adds `EnclSolQSWRad * InterpSw(switching factor,
  active.AbsDiffBack, shaded.AbsDiffBack)` over the active layer count.
  `InterpSw` uses `std::clamp`: ordered values, including infinities, clamp
  to [0,1], while NaN survives and propagates. Blind `Interp` has no clamp.

A nonswitchable shaded or conditional/invalid flag with a zero shaded index
performs no main glass/shade addition. With a nonzero index, flags that match
none of the explicit shade/screen or Int/ExtBlind predicates can still enter
the branch while doing no layer work. Common frame/divider work nevertheless
continues.

For positive frame area, CP160 adds

`(QS * frameSolarAbs + (radQ * radThermAbsMult + HVAC) * frameEmissivity) *
(1 + 0.5 * insideFrameProjectionCorrection)`

to the CP159-initialized inside frame absorbed flux. For positive divider
area it begins with divider solar absorptance and emissivity. A suspended
divider dynamic-casts the active Construction's last layer to
`MaterialGlass`, asserts success, replaces thermal absorptance with back
thermal absorptance, and replaces solar absorptance by

`AbsGlass + TransGlass * (AbsDivider + ReflDivider * AbsGlass) /
(1 - ReflDivider * ReflGlass)`.

An interior shade then asserts a last-layer `MaterialFen` and multiplies
solar and thermal divider absorptance by its solar and thermal transmittance.
An interior blind instead multiplies them by current back diffuse solar and
infrared transmittance. No exterior or between-glass device applies this
correction. The resulting divider addition is

`(QS * dividerSolarAbs + (radQ * radThermAbsMult + HVAC) *
dividerThermalAbs) * (1 + insideDividerProjectionCorrection)`.

Those frame/divider updates are `+=`, use raw projection factors, and are not
part of the load-component pulse. EQL windows execute neither frame nor
divider work.

For an EQL window, the active Construction's `EQLConsPtr` selects
`CFS(EQLNum).NL`. Each layer receives
`EnclSolQSWRad * active.AbsDiffBackEQL(layer)` after the duplicated thermal
radiant-source overwrite. There is no EQL frame implementation here.

After its own model branch, every Window whose `ExtBoundCond > 0` adds
current-enclosure short wave to the corresponding adjacent Window:

- when the adjacent Window is non-EQL, the loop count is the current/source
  active Construction's `TotGlassLayers`, but absorptance comes from the
  adjacent Surface's base Construction `AbsDiff(layer)`; and
- when the adjacent Window is EQL, the loop count comes from the adjacent
  base Construction's EQL system, but absorptance is the current/source active
  Construction's `AbsDiffFrontEQL(layer)`.

These deliberate front-incidence formulas are also topology asymmetries:
neither branch reconciles source and target layer counts, active/base identity,
or model compatibility. CP160 does not validate positive links as reciprocal
Window links, and every transfer is additive.

The current Window then receives initial diffuse absorption calculated by an
earlier distribution dependency. Detailed no-shade and switchable cases add
`SurfWinInitialDifSolwinAbs` over the active layer count. Other Detailed
shading flags dereference the shaded Construction and use its layer count;
shade/screen/blind flags also add
`SurfWinInitialDifSolAbsByShade` to the shade field. Thus a zero shaded index
that skipped the main nonswitchable branch can still fail here. BSDF uses the
active glass-layer count, EQL uses the active EQL system's `NL`, and an
invalid model receives no initial-diffuse addition.

All own-window, adjacent-window, and initial-diffuse layer mutations update
only per-area `SurfWinQRadSWwinAbs`. CP160 never recomputes CP159's
`SurfWinQRadSWwinAbsLayer` W values, all-layer
`SurfWinQRadSWwinAbsTot`, or `SurfWinQRadSWwinAbsTotEnergy`. Frame and
divider additions also remain only in their per-area absorbed-flux fields,
with no corresponding W/energy derivation here. The Window layer W/energy
reports can therefore disagree with the final CP160-augmented per-area state.

The final action is unconditional
`Dayltg::DistributeTDDAbsorbedSolar(state)` at line 4176. That child is
declared at `DaylightingDevices.hh` line 95, implemented completely at
`DaylightingDevices.cc` lines 1506-1559, has this sole production `src/`
call, and has no direct unit-test call. For every stored TDD pipe it uses the
diffuser's base Construction `TransDiff`, not an active Construction. Let
`A` be diffuser area and define the exact raw reflection term

`QRefl = (SurfQRadSWOutIncident(diffuser) -
SurfWinQRadSWwinAbsTot(diffuser)) * A - SurfWinTransSolar(diffuser)
+ EnclSolQSWRad(diffuser enclosure) * A * TransDiff`.

The declared units are asymmetric in that subtraction:
`SurfQRadSWOutIncident` is W/m2 while the stale CP159
`SurfWinQRadSWwinAbsTot` is W. Under normal topology CP160 has augmented the
diffuser per-area layer value but not that total; the TDD dome is outside
stored Zone Window ranges, so its layer-one value remains CP159/dependency
state unless malformed topology reaches it through another path. The child
then computes

`rawGain = SurfWinTransSolar(dome) -
SurfQRadSWOutIncident(diffuser) * A +
QRefl * (1 - TransSolIso / TransDiff) +
SurfWinQRadSWwinAbs(dome, 1) * A / 2 +
SurfWinQRadSWwinAbs(diffuser, 1) * A / 2`.

The diffuser term consumes the newly CP160-mutated per-area value, while the
dome term normally consumes its prior CP159/dependency value; both, including
the dome term, are multiplied by diffuser area. The report
`PipeAbsorbedSolar = max(0, rawGain)` uses zero as the first operand, mapping
negative finite values, negative infinity, and NaN to zero while retaining
positive infinity. Each transition-zone `TZoneHeatGain`, however, receives
the unclamped signed/nonfinite
`rawGain * TZoneLength / TotLength`. `TransDiff` and `TotLength`
divisions are unchecked.

CP160 has no return value, direct diagnostic, local validation, catch,
rollback, or cleanup. It assumes valid enclosure/Space/range topology,
matrices and flags, Surface and Construction links, material types, active and
shaded layer counts, EQL systems, adjacency, TDD arrays, and all target
allocations. Its only local assertions are the suspended-divider glass and
interior-shade material casts. Unchecked total-area, movable-material,
suspended-divider, TDD-transmittance, and TDD-length denominators; unchecked
indices and nullable state; invalid ranges or layer mismatches; and raw
nonfinite inputs can propagate NaN/infinity, assert, terminate, or produce
undefined behavior after an ordered mutation prefix. The final TDD
child can fail only after all preceding enclosure, opaque, and Window changes,
and can leave completed earlier pipes. Within the current pipe its report is
assigned before transition-zone traversal, so a later failure leaves the
current `PipeAbsorbedSolar` plus only the completed `TZoneHeatGain` prefix.

Re-entry is additive and selective rather than transactional. Each call
reassigns then rescales the two enclosure working fluxes and overwrites Window
thermal-radiant source terms, but Zone diffuse reports, opaque absorbed terms,
Window layers, adjacent transfers, frame/divider fields, and many shade paths
use `+=`. The main shade short-wave field is assigned before its
initial-diffuse addition; TDD fields are assigned for reached pipes. A
sun-down call preserves prior beam reports, no eligible interzone source
preserves report energy, and existing no-longer-reached
enclosures/Surfaces/range members retain state. Reducing a pipe's
`NumOfTZones` preserves unvisited `TZoneHeatGain` tail slots. Normal parent
order relies on CP159 to clear many, but not all, inputs and targets before
CP160; CP160 has no independent reset guarantee. A direct retry repeats
additions and the movable outside-value feedback, while a parent retry also
repeats predecessor calls and keeps the first-time display flag true until the
complete parent tail succeeds. Newly false shade/model/positive-area gates
likewise preserve their CP160 direct-call targets until CP159 or an owner clear
resets them.

The caller's preceding `ManageInternalHeatGains` reaches
`FigureTDDZoneGains`. On the first BeginEnvironment call guarded by its own
`MyEnvrnFlag`, that helper zeros current `TZoneHeatGain` arrays but not
`PipeAbsorbedSolar`. Its current `UpdateInternalGainValues` pass has already
occurred before CP160 writes the new transition-zone values, so these new
values are not part of that already-completed internal-gain update.

Four directly mutated owners define the body clear boundary:
`HeatBalanceData::clear_state`, `HeatBalSurfData::clear_state`,
`SurfacesData::clear_state`, and
`DataDaylightingDevicesData::clear_state` each reconstruct their record by
placement-new. `HeatBalSurfMgr::clear_state` does not own CP160 numerical
state; it only rearms the caller's shared first-time display lifecycle.
ViewFactor, Construction, Material, WindowEquivalentLayer, Environment,
Global, internal-gain, and daylighting setup state are read dependencies whose
separate clears do not replay CP160. The ordinary operational reset is the
preceding source-order initialization, not any CP160-local clear.

Rust has no `InitIntSolarDistribution` target or state machine.
`inside_shortwave_absorbed_w_per_m2` is initialized or explicitly supplied
and consumed by the inside-face balance, but no runtime path derives it from
Solar enclosures. The bounded internal-gain schedule helper uses ordered
`<= 0` checks for typed-Zone radiant gain and the area-absorptance sum, so NaN
passes those gates, while each thermal absorptance uses `.max(0.0)`. The
incident-solar helper remains an exterior forcing diagnostic. They do not
implement `QLTSW`, Solar/radiant enclosure multipliers, interzone matrices
and reports, night-stale beam reporting, movable exterior transforms, Window
layers/shades/frames/dividers, adjacent-window transfer, initial diffuse
addition, TDD transition-zone gains, report inconsistency, or CP160
failure/re-entry semantics.

CP160 adds required `source_mapped`
`routine.init_int_solar_distribution` under the existing Surface-manager
algorithm and the matching project-contract requirement immediately after
`init_solar_heat_gains` and before
`compute_int_thermal_absorp_factors` in source-definition order. It adds no
EnergyPlus source inventory, Rust target/code/state, test, support,
capability, output, numerical, performance, or conformance promotion. The
inventory becomes 32 algorithms and 169 routines, split into 58
`state_mapped` and 111 `source_mapped`, with 62 required.

### CP161 `ComputeIntThermalAbsorpFactors` source map

The canonical declaration is `HeatBalanceSurfaceManager.hh` line 113 and the
complete implementation is `HeatBalanceSurfaceManager.cc` lines 4179-4295.
Its sole production `src/` call is unconditional
`ComputeIntThermalAbsorpFactors(state)` inside `InitSurfaceHeatBalance` line
427. This checkpoint follows CP160 only in declaration and source-definition
inventory order: at runtime line 427 executes before CP162 and well before
CP159 line 457 and CP160 line 468. The caller's lines-421-423
`InitSurfaceHeatBalancefirstTime` guard controls only exact progress text
`Computing Interior Absorption Factors`; the separate lines-424-426 guard
calls `InitInteriorRadExchange` only while first-time is true. Neither guard
controls CP161. A CP161 non-return preserves completed predecessor and CP161
mutations, blocks CP162, interzone diffuse exchange, daylighting and interior
long-wave exchange, CP159 and CP160, the caller remainder, and the successful
line-620 first-time-flag clear.

There is exactly one direct unit-test call, in
`HeatBalanceSurfaceManager_ComputeIntThermalAbsorpFactors` at line 321. Its
single one-square-metre Window has `IntBlind`, effective shade and glass
emissivities 0.1 each, base Construction inside thermal absorptance 0.9, one
flagged radiant enclosure, one stored Space Window range, and one
`SurfacePtr` entry. The test checks only that CP161 writes
`SurfAbsThermalInt = 0.1 + 0.1 = 0.2` and
`radThermAbsMult = 1 / (1 * 0.2) = 5`. It does not cover a false
`radReCalc`, ordinary or switchable glazing, opaque movable insulation,
movable-slat list behavior, frames, dividers, representative surfaces,
duplicate or invalid topology, nonfinite or zero sums, failure prefixes, or
re-entry.

Immediately before CP161, `WindowShadingManager` updates shading state and
`CheckGlazingShadingStatusChange` produces radiant and Solar enclosure
`radReCalc` flags. BeginSimulation always-recalculate modes,
BeginEnvironment, and construction or surface-property overrides can flag all
counted enclosures; otherwise the producer first clears counted flags and can
set an enclosure again for a shading flag, active shaded Construction, or
movable-blind status change. On first entry the separately guarded
`InitInteriorRadExchange` constructs radiant `SurfacePtr` topology before
CP161. CP161 neither sets nor clears `EnclRadInfo.radReCalc`. CP162 instead reads
the corresponding `EnclSolInfo.radReCalc` produced alongside the radiant
flag, not the same state object. The producer's bulk set/reset paths use
separate `NumOfSolarEnclosures` and `NumOfRadiantEnclosures` bounds. Its
shading-change scan iterates `1..NumOfSolarEnclosures` but reads the same-index
`EnclRadInfo(enclosureNum).SurfacePtr` before setting both corresponding
Solar and radiant flags. CP161's first and third passes instead range over the
entire allocated `EnclRadInfo` container. A malformed or reduced count with a
retained flagged tail can therefore be processed by CP161 without having been
reset or set by that producer.

CP161 performs three complete-pass phases. In the first phase, every stored
radiant enclosure whose `radReCalc` is false is skipped. For each flagged
entry it traverses stored `spaceNums` and every numeric Surface in each
Space's inclusive `WindowSurfaceFirst..WindowSurfaceLast` range. `IntShade`
and `IntBlind` are the only flags accepted by
`ANY_INTERIOR_SHADE_BLIND`; either assigns

`SurfAbsThermalInt = surfShade.effShadeEmi + surfShade.effGlassEmi`.

Every other flag, including switchable glazing and exterior or between-glass
devices, assigns the current `SurfActiveConstruction`'s
`InsideAbsorpThermal`. This pass does not inspect Surface class, Window model,
heat-transfer status, enclosure membership beyond the stored ranges, or
bounds. It does not sort or deduplicate enclosures, Spaces, or ranges;
overlaps and duplicates repeat assignments, while a false gate or a Window
that is no longer reached preserves its prior value.

The second phase is independent of `radReCalc`. Only when global
`AnyMovableSlat` is true, it walks stored `SurfMovSlatsIndexList` order. A
listed Surface currently flagged `IntShade` or `IntBlind` receives the same
effective-emissivity sum; every other listed entry receives no write. The
phase has no enclosure, Space, Window-class, membership, uniqueness, or bounds
check. It can therefore refresh a Surface inside an unflagged enclosure
without refreshing that enclosure's multiplier. A false global gate or a
listed Surface whose interior-device flag became false performs no
second-phase write; only when the first phase did not refresh that Surface can
a prior absorptance remain stale. Duplicate entries repeat the assignment.

The third phase again scans the full allocated radiant-enclosure container and
skips each false `radReCalc`. It starts local `SUM1 = 0` for every flagged
enclosure and visits stored `SurfacePtr` order. Each entry immediately reads
the Surface's declared base `Construction` and its Window shading flag,
without a Surface-class or model guard. A nonswitchable entry adds

`Surface.Area * SurfAbsThermalInt`.

A switchable-glazing entry ignores that cached current absorptance and adds

`Surface.Area * InterpSw(SurfWinSwitchingFactor,
base Construction.InsideAbsorpThermal,
activeShadedConstruction.InsideAbsorpThermal)`.

Thus the first pass uses `SurfActiveConstruction` for every non-interior-device
Window, while the switchable denominator instead combines the declared base
`Surface.Construction` with `surface.activeShadedConstruction`. Ordinary
opaque and nonswitchable entries consume whichever current
`SurfAbsThermalInt` survived preceding initialization, variable or movable
insulation, and the first two CP161 phases.

`InitInteriorRadExchange` normally excludes AirBoundary Surfaces and places
only representative calculation Surfaces in `SurfacePtr`. It separately adds
nonrepresentative constituent areas into `EnclRadInfo.Area`, but CP161 never
uses that aggregate array: it uses only the representative's raw
`Surface.Area` and its own frame/divider areas. The initial Window-range pass
still visits full stored ranges, including nonrepresentatives. Representative
Surface absorptance and denominator area can therefore describe different
physical extents. CP161 performs no local validation of that topology, and a
malformed `SurfacePtr` can also contain duplicates or arbitrary classes.

Every `SurfacePtr` entry with positive frame area next adds

`SurfWinFrameArea * (1 + 0.5 * SurfWinProjCorrFrIn) * SurfWinFrameEmis`.

NaN or nonpositive frame area skips the branch; positive infinity enters it.
There is no Window-class guard and projection and emissivity are raw.

For positive divider area, CP161 starts `DividerThermAbs` from
`SurfWinDividerEmis`. A suspended divider replaces it with the declared base
Construction's `InsideAbsorpThermal`. If the current flag is neither
`IntShade` nor `IntBlind`, the contribution is

`SurfWinDividerArea * (1 + SurfWinProjCorrDivIn) * DividerThermAbs`.

The same projected formula applies to an interior shade/blind when
`SurfWinHasShadeOrBlindLayer` is false. However, CP161 dereferences
`surface.activeShadedConstruction` before checking that layer flag, so even
this false branch requires a valid shaded Construction index. When the layer
flag is true, the shaded Construction's last layer is dynamic-cast to
`MaterialFen` and asserted non-null. `TauShIR` starts as that material's
`TransThermal`; `IntBlind` replaces it with current
`surfShade.blind.TAR.IR.Bk.Tra`. The exact contribution is then

`SurfWinDividerArea * (surfShade.effShadeEmi +
DividerThermAbs * TauShIR)`.

That shade/blind-layer branch intentionally omits the divider projection
correction. It uses effective shade emissivity but not effective glass
emissivity. These branches mix the declared base Construction for suspended
divider absorptance, the active shaded Construction for its last material,
and current blind runtime optics.

Only after all entries in a flagged enclosure complete does CP161 assign

`radThermAbsMult = 1 / SUM1`.

There is no minimum, sign, finite, or zero check and no CP162-style warning.
Positive zero produces positive infinity; a negative finite sum produces a
negative multiplier; positive or negative infinity produces signed zero; and
NaN produces NaN. `InterpSw` clamps ordered switching factors, including
negative and positive infinity, into [0,1], while a NaN factor remains NaN.
Its raw `(1-f)*A + f*B` arithmetic can still propagate nonfinite A or B and
can form NaN through zero times infinity even at an endpoint. Negative,
nonfinite, or oversized areas, absorptances, emissivities, transmissions, and
projection factors otherwise flow directly into the sum.

CP161 has no return value, direct diagnostic, local validation, catch,
rollback, allocation, or cleanup. Its only assertion is the interior
shade/blind divider material cast. Invalid arrays, indices, Construction or
material links, range/list topology, and nonfinite data can assert, terminate,
produce undefined behavior, or publish raw nonfinite state. A failure in the
first phase leaves only its completed Window-absorptance prefix. A failure in
the movable-slat phase follows the complete first phase and leaves a partial
list prefix. A failure during a current third-pass enclosure follows both
complete absorptance phases and all previously committed enclosure
multipliers, but the current and later enclosure multipliers retain their old
values because the current write occurs only after its full `SurfacePtr` loop.

Re-entry is selective rather than transactional. Flagged, reached Window
absorptances and completed enclosure multipliers are overwritten, not
accumulated. A false `radReCalc` preserves that enclosure's targets; removed
Spaces/ranges/list members, an independent movable-slat entry outside a
flagged/reached Window range, and an abnormal prefix can preserve individual
absorptances or old current/later multipliers. A frame or divider area that
becomes nonpositive contributes nothing to a newly recomputed flagged sum and
owns no separate CP161 target, so that gate alone is not stale state. CP161
does not consume its recalculation flags. The
preceding CP158 can change an opaque Surface's `SurfAbsThermalInt` through
movable insulation without itself causing `CheckGlazingShadingStatusChange`
to set `radReCalc`; absent another producer cause, CP161 can therefore retain
a multiplier normalized against the old opaque absorptance. In normal topology
the producer sees `surfShade.blind.movableSlats` through `SurfacePtr` and
flags that enclosure, so the independent movable-slat phase creates an
analogous unflagged absorptance/multiplier mismatch only under direct calls,
malformed or representative-misaligned topology, or count/container mismatch.

Exactly two directly mutated owners define the clear boundary. The only
HeatBalSurf target written by CP161 is `SurfAbsThermalInt`; the same owner
supplies read-only `SurfMovSlatsIndexList`. The only ViewFactor target written
is `EnclRadInfo.radThermAbsMult`; the same owner supplies read-only enclosure
flags and topology. `HeatBalSurfData::clear_state` placement-new reconstructs
its absorptance array and list, while `ViewFactorInfoData::clear_state` clears
both enclosure containers and all contained multiplier and flag state.
`SurfacesData`, HeatBalance Space/Zone topology, Construction, Material,
Window-manager optics, global flags, SolarShading, and
HeatBalanceIntRadExchange are read or producer dependencies, not CP161 direct
mutation owners. Clearing a dependency alone neither rebuilds both CP161
target families nor replays the routine.

Rust has no `ComputeIntThermalAbsorpFactors` target or matching persistent
multiplier state. Its bounded internal-gain helper resets retained Surface
radiant source terms, skips a typed-Zone gain or area-absorptance sum only when
its ordered comparison is `<= 0` so NaN passes both gates, applies `.max(0.0)`
to each inside thermal absorptance, and directly distributes W/m2. It does not
implement radiant-enclosure
`radReCalc`, Space Window ranges, interior shade/blind or movable-slat
emissivity, switchable glazing, representative `SurfacePtr` topology,
frame/divider terms, raw reciprocal/nonfinite behavior, or CP161
failure/re-entry semantics.

CP161 adds required `source_mapped`
`routine.compute_int_thermal_absorp_factors` under the existing
Surface-manager algorithm and the matching project-contract requirement
immediately after `init_int_solar_distribution` and before
`compute_int_sw_absorp_factors` in source-definition order. It adds no
EnergyPlus source inventory, Rust target/code/state, test, support,
capability, output, numerical, performance, or conformance promotion. The
inventory becomes 32 algorithms and 170 routines, split into 58
`state_mapped` and 112 `source_mapped`, with 63 required.

### CP162 `ComputeIntSWAbsorpFactors` source map

The canonical declaration is `HeatBalanceSurfaceManager.hh` line 115 and the
complete implementation is `HeatBalanceSurfaceManager.cc` lines 4297-4471.
Its sole production `src/` call is unconditional
`ComputeIntSWAbsorpFactors(state)` inside `InitSurfaceHeatBalance` line 433,
immediately after CP161 line 427. The caller's lines-430-432
`InitSurfaceHeatBalancefirstTime` guard controls only exact progress text
`Computing Interior Diffuse Solar Absorption Factors`; it does not control
CP162. A CP162 non-return preserves completed predecessor and CP162 prefixes,
blocks the conditional CP163 interzone-diffuse child, daylighting, interior
long-wave exchange, CP159 line 457, CP160 line 468, the caller remainder, and
the successful line-620 first-time-flag clear. No unit test calls CP162
directly; its sole production call is line 433.

The immediately preceding `CheckGlazingShadingStatusChange` produces separate
Solar and radiant `radReCalc` flags. BeginSimulation always-recalculate mode,
BeginEnvironment, and construction or surface-property overrides bulk-flag
Solar and radiant enclosures with their separate
`NumOfSolarEnclosures`/`NumOfRadiantEnclosures` bounds. Otherwise it clears
both counted sets and, only when shading controls exist, loops
`1..NumOfSolarEnclosures` while inspecting the same-index radiant enclosure's
`SurfacePtr`; a shading flag or active shaded Construction change, or a
blind marked `movableSlats`, sets both flags and stops that enclosure scan.
CP161 consumes the
radiant `EnclRadInfo.radReCalc` value, while CP162 reads the corresponding
`EnclSolInfo.radReCalc`; neither routine clears its flag.

Those producer observations and CP162's consumption have different topology.
`InitSolarViewFactors` normally builds Solar `SurfacePtr` from every stored
Space Surface except AirBoundary Surfaces, including nonrepresentative
calculation Surfaces. `InitInteriorRadExchange` normally builds the
corresponding radiant pointer list from representatives only. Thus CP162's
Solar calculation visits every stored constituent directly and has no
CP161-style representative-area omission, while the shading-change producer
observes the representative-only radiant list. This records an
observer/calculation-domain asymmetry, not a normal unflagged constituent:
`HasShadeControl` Windows are normally made unique representatives. Only a
direct call, malformed or representative-misaligned topology, or a
count/container mismatch can turn the asymmetry into an unflagged Solar
calculation entry. The producer's Solar-count-bound radiant access also
assumes aligned containers and indices. CP162 instead range-scans the entire
allocated `EnclSolInfo` container, so a malformed or reduced count can leave a
retained Solar tail whose flag was not reset or set but is still consumed.

CP162 defines `SmallestAreaAbsProductAllowed = 0.01`. For every stored Solar
enclosure, false `radReCalc` skips all work and preserves both persistent
CP162 targets. A flagged enclosure starts local `SUM1 = 0` and visits stored
`SurfacePtr` order without sorting or deduplication. Every entry immediately
reads the Surface, its current `SurfActiveConstruction`, and that active
Construction. Active `TransDiff <= 0` selects the opaque path and adds

`Surface.Area * SurfAbsSolarInt`.

The area and current inside-solar absorptance are raw. Negative and nonfinite
values flow into the sum. NaN `TransDiff` makes the ordered `<= 0` comparison
false and therefore selects the Window path; positive transmission alone does
not establish a Window class. CP162 performs no local heat-transfer, class,
model, enclosure-membership, active-index, or bounds validation.

Inside the Window path, the declared base
`Construct(Surface.Construction).WindowTypeEQL` selects conventional versus
equivalent-layer handling, even though the path and all optical values are
otherwise based on the active Construction. For a conventional Window, the
routine loops `1..thisConstruct.TotGlassLayers`. Each layer starts with the
active Construction's `AbsDiffBack(Lay)`. Only when the storm-aware
`SurfWinActiveShadedConstruction` array is nonzero can a shade, screen, or
blind substitute shaded optics. Any shade or screen then replaces the layer
value with that shaded Construction's `AbsDiffBack(Lay)`; any blind instead
uses the current low/high slat indices and raw, unclamped
`Interp(Lower, Upper, slatAngInterpFac)` over
`layerSlatBlindDfAbs(Lay)` back-diffuse solar absorptance. A shade/blind flag
paired with a zero shaded-Construction index silently retains the bare active
layer value. Both optical blocks dereference any nonzero shaded-Construction
index before testing shade/screen/blind helpers, so a stale or invalid nonzero
index can fail even when the current flag requests no substitution. A
switchable-glazing flag separately asserts the shaded
Construction index is positive and applies

`InterpSw(SurfWinSwitchingFactor, current layer value,
shaded Construction.AbsDiffBack(Lay))`.

`InterpSw` clamps ordered factors, including infinities, to [0,1], preserves a
NaN factor, and can still create NaN through raw nonfinite endpoint
arithmetic. The ordinary blind `Interp` is the unbounded
`Lower + factor * (Upper - Lower)` expression. All layer results accumulate
in `AbsDiffTotWin`.

`TransDiffWin` separately starts at active `TransDiff`, while `DiffAbsShade`
starts at zero. Under the same nonzero storm-aware shaded-Construction gate,
any shade or screen replaces these with shaded `TransDiff` and
`AbsDiffBackShade`; any blind raw-linearly interpolates current slat endpoints
for front-diffuse solar transmittance and back-diffuse solar absorptance. A
shade/blind flag paired with zero again retains bare active transmission and
zero shade absorptance. Switchable glazing performs a second independent
positive-shaded-Construction assertion and applies `InterpSw` to
transmission. The main conventional contribution is exactly

`Surface.Area * (TransDiffWin + AbsDiffTotWin + DiffAbsShade)`.

No term is normalized or range/finite checked. Shade/screen, blind, and
switchable paths deliberately consume different shaded-Construction members
and current runtime optics through the same storm-aware index.

A positive `SurfWinFrameArea` next adds

`SurfWinFrameArea * SurfWinFrameSolAbsorp *
(1 + 0.5 * SurfWinProjCorrFrIn)`.

NaN or nonpositive frame area skips the branch; positive infinity enters it,
and absorptance and projection remain raw. A positive divider area starts
`DividerAbs` from `SurfWinDividerSolAbsorp`. For a suspended divider, the
active Construction's final layer is dynamic-cast and asserted as
`MaterialGlass`; with its `Trans`, back-beam solar reflectance, derived
`AbsGl = 1 - TransGl - ReflGl`, and
`DividerRefl = 1 - DividerAbs`, the replacement is

`AbsGl + TransGl * (DividerAbs + DividerRefl * AbsGl) /
(1 - DividerRefl * ReflGl)`.

That denominator has no zero, sign, or finite guard. An interior shade or
blind then adds `SurfWinDividerArea * (DividerAbs + DiffAbsShade)` and omits
the divider projection correction. Every other flag adds
`SurfWinDividerArea * (1 + SurfWinProjCorrDivIn) * DividerAbs`. Unlike CP161,
there is no `SurfWinHasShadeOrBlindLayer` gate and no interior material cast;
the suspended-divider glass cast is the third assertion site after the two
switchable-glazing index assertions.

When the declared base Construction is EQL, CP162 uses the active
Construction's `TransDiff`, `EQLConsPtr`, equivalent-layer `NL`, and
`AbsDiffBackEQL(1..NL)`, then adds

`Surface.Area * (active TransDiff + sum(active AbsDiffBackEQL))`.

This branch ignores shading, switching, frames, and dividers entirely. A
base/active EQL mismatch can therefore route active conventional data through
EQL indices or active EQL data through conventional layer arrays; no local
compatibility check repairs it.

Only after every Surface completes does CP162 test the raw sum. Strict
`SUM1 > 0.01` assigns `solVMULT = 1 / SUM1`; positive infinity passes and
produces positive zero. Exactly 0.01, positive values below it, zero, negative
finite values, negative infinity, and NaN all enter the bad-sum branch. While
`solAbsFirstCalc` is true, that branch calls `ShowWarningError` with exact
misspelled routine prefix

`ComputeIntSWAbsorbFactors: Sum of area times inside solar absorption for all surfaces is zero in Enclosure: {Name}`.

Only after the warning helper returns does it set `solAbsFirstCalc = false`;
it then assigns `solVMULT = 0`. Subsequent bad sums in the same enclosure
lifetime remain silent. A later good sum, BeginEnvironment, or ordinary
recalculation does not rearm the latch, and a false `radReCalc` preserves both
the multiplier and latch. Only enclosure destruction/reconstruction restores
the member defaults `solAbsFirstCalc = true` and `solVMULT = 0`. Despite a
legacy purpose comment, the body performs no `VCONV` write.

CP162 has no return value, body-owned container allocation, catch, rollback,
cleanup, or validation diagnostic beyond that bad-sum warning. Warning
formatting and diagnostic machinery can allocate outside the numeric body.
Its three assertion sites,
unchecked indices and topology, active/base model mismatch, blind arrays,
material links, and raw arithmetic can assert, terminate, produce undefined
behavior, or yield a bad sum. Because all Surface work is local accumulation,
a failure before the tail leaves the current enclosure's multiplier and latch
unchanged after all earlier enclosures may already have committed. A failure
inside the warning pipeline can expose partial counter, message-stream,
SQLite, or callback side effects while leaving the latch true and multiplier
old. Once the warning returns, the two scalar target assignments have no
intervening child call.

Re-entry overwrites, rather than accumulates, a completed flagged enclosure's
multiplier. A good recomputation does not alter the latch; a bad one writes
zero and conditionally consumes it. False recalculation, a direct or malformed
Solar/radiant observer mismatch, or retained count/container tails can
preserve old targets. The preceding CP158 can change an opaque Surface's
current `SurfAbsSolarInt` through interior movable insulation without that
transition itself setting a Solar `radReCalc`; absent another producer cause,
CP162 can therefore retain a multiplier normalized against the old
absorptance on the normal parent path. Positive frame/divider gates own no
separate CP162 state: if a flagged recomputation sees an area become
nonpositive, that term simply drops from the new sum.

ViewFactor is the one direct mutation owner: CP162 writes only
`EnclSolInfo.solVMULT` and `EnclSolInfo.solAbsFirstCalc`, while the same owner
supplies the read-only flags, names, and topology. Its `clear_state` zeros both
enclosure counts and clears both containers; reconstructed enclosure records
restore the member defaults. The warning helper separately mutates diagnostic
counters and output/SQLite/callback dependencies, but those are child side
effects rather than another CP162 direct target owner. Surfaces, HeatBalSurf,
Construction, Material, WindowEquivalentLayer, SolarShading, Window-manager,
HeatBalance topology, and Global state are read or producer dependencies;
clearing one does not replay CP162 or consistently rebuild its targets.

Rust has no `ComputeIntSWAbsorpFactors`, Solar-enclosure `solVMULT`, warning
latch, Window/shade/frame/divider normalization, or matching failure/re-entry
state. Production initializes `inside_shortwave_absorbed_w_per_m2` to zero and
reads it in the inside-face balance and reports, but no production path
derives or updates it afterward; tests can assign it directly.
The separate bounded typed-Zone thermal-radiant helper resets its own output,
applies `.max(0.0)` to inside thermal absorptance, and skips gain and sum only
when ordered `<= 0` comparisons are true, so NaN passes those gates. That is
not CP162's short-wave multiplier, whose ordered `SUM1 > 0.01` comparison sends
NaN to the warning/zero branch.

CP162 adds required `source_mapped`
`routine.compute_int_sw_absorp_factors` under the existing Surface-manager
algorithm and the matching project-contract requirement immediately after
`compute_int_thermal_absorp_factors` and before
`compute_dif_sol_exc_zones_wiz_windows` in source-definition order. It adds no
EnergyPlus source inventory, Rust target/code/state, test, support,
capability, output, numerical, performance, or conformance promotion. The
inventory becomes 32 algorithms and 171 routines, split into 58
`state_mapped` and 113 `source_mapped`, with 64 required. CP163 maps
`ComputeDifSolExcZonesWIZWindows`, declared at
`HeatBalanceSurfaceManager.hh` line 117, implemented at
`HeatBalanceSurfaceManager.cc` lines 4473-4644, and called only by the
`InitSurfaceHeatBalance` lines-435-439 `InterZoneWindow` branch. Its three
direct unit-test calls at lines 3173, 3186, and 3193 are mapped below.

### CP163 `ComputeDifSolExcZonesWIZWindows` source map

The canonical declaration is `HeatBalanceSurfaceManager.hh` line 117 and the
complete implementation is `HeatBalanceSurfaceManager.cc` lines 4473-4644.
Its sole production `src/` call is
`ComputeDifSolExcZonesWIZWindows(state)` at `InitSurfaceHeatBalance` line
439. The parent lines-435-440 `InterZoneWindow` guard controls the entire
display-and-call block; within it, the lines-436-438
`InitSurfaceHeatBalancefirstTime` guard controls only exact progress text
`Computing Interior Diffuse Solar Exchange through Interzone Windows`.
CP163 itself does not recheck the parent `InterZoneWindow` flag. A guarded
CP163 non-return preserves CP161 and CP162 state, blocks daylighting, interior
long-wave exchange, CP159, CP160, the caller remainder, and the successful
line-620 first-time-flag clear.

Exactly three direct calls occur in one unit-test fixture at lines 3173, 3186,
and 3193. The fixture has three Solar enclosures and two reciprocal 1 m2
Windows whose declared diffuse transmittance is 0.1 and whose enclosure
`solVMULT` values are 1. The first call leaves both receiver
`HasInterZoneWindow` flags false and checks only all three matrix diagonal
entries equal one plus all three receive flags false. The second sets those
flags true for enclosures 1 and 2 before calling and checks only receive flags
true, true, false; it does not assert any matrix coefficient. Before the third
call the fixture sets `KickOffSimulation` true, then again checks only unit
diagonal entries and all receive flags false. No test covers
`KickOffSizing`, nonfinite values, allocation incoherence, pair denominators,
multi-enclosure paths, or either assertion's failure.

CP163 snapshots `NumOfSolarEnclosures` as its matrix extent. Allocation is
controlled solely by whether
`HeatBalSurf::ZoneFractDifShortZtoZ` is unallocated. When it is unallocated,
the routine allocates that square target matrix, then
`EnclSolRecDifShortFromZ`, then the separate
`HeatBalSurfMgr::DiffuseArray` square scratch matrix. It does not
independently verify allocation or dimensions of the latter two objects and
does not resize already allocated state after an enclosure-count change.

Every call then performs this order before inspecting either kickoff flag:

1. assign every `EnclSolRecDifShortFromZ` entry false;
2. replace `ZoneFractDifShortZtoZ` with an identity matrix;
3. replace `DiffuseArray` with an identity matrix;
4. return if `KickOffSimulation || KickOffSizing`.

Thus a kickoff call allocates when needed and destructively resets CP163 state
before returning; it does not preserve a prior exchange solution. With
coherent dimensions the third unit call proves the diagonal/flag part of that
ordering. A partially allocated owner pair or changed enclosure count can
fail during these operations or retain mismatched extents before the later
assertions are reachable.

For the direct pass, write `A(receiver, source)` for the current
`ZoneFractDifShortZtoZ` matrix. CP160 and the daylighting consumer use this
row-receiver, column-source orientation. CP163 visits
`AllHTWindowSurfaceList` in stored order without local sorting,
deduplication, Window-class, heat-transfer, reciprocal-link, enclosure-index,
construction-index, or bounds validation. An entry is skipped when its
`ExtBoundCond <= 0`, when it points to itself by Surface number, when its
declared/base `Surface.Construction.TransDiff <= 0`, or when the receiver
Solar enclosure's `HasInterZoneWindow` is false. The adjacent Surface need
not advertise `HasInterZoneWindow`.

For each remaining entry, the receiver is the current Surface's
`SolarEnclIndex` and the source is the adjacent Surface's
`SolarEnclIndex`. The exact update is

`A(receiver, source) += base TransDiff * receiver.solVMULT * Surface.Area`.

The routine deliberately does not use `SurfActiveConstruction`, a shaded or
storm Construction, or adjacent-surface optics. Repeated list entries and
multiple Windows accumulate. After the update it sets the receiver's
`EnclSolRecDifShortFromZ` flag whenever receiver `solVMULT != 0`; this flag
test is independent of the actual coefficient, area, and accumulated sign.
Both signed zero values fail `!= 0`, while negative finite values, either
infinity, and NaN pass it.

The direct-pass gates are ordered raw comparisons. A NaN `TransDiff` passes
the `<= 0` skip and enters arithmetic; a positive transmittance can still be
multiplied by zero, negative, infinite, or NaN area/multiplier. There is no
range, normalization, conservation, sign, or finite check. The self-boundary
test compares Surface numbers, not Solar enclosure identities. A paired
Surface in the same Solar enclosure can therefore add to raw
`A(receiver, receiver)`, but that raw diagonal addition is discarded by the
following transform because its scratch diagonal starts at one and the pair
loop skips equal enclosure indices.

The pair transform keeps `A` frozen and writes scratch matrix `D`. For
every distinct receiver/source pair it computes exactly

`D(R, S) = A(R, S) / (1 - A(R, S) * A(S, R))`.

For each enclosure `N`, its scratch diagonal then becomes

`D(N, N) = 1 + sum[M != N](A(N, M) * D(M, N))`.

This algebraically folds reciprocal two-enclosure returns into the direct
pair and diagonal without checking denominator sign, zero, magnitude, or
finiteness. A zero denominator, overflow, infinity, or NaN can therefore
produce signed infinity, signed zero, or NaN. The completed scratch matrix is
assigned wholesale to `ZoneFractDifShortZtoZ`; only afterward do exactly two
assertions check its first and second dimensions against
`NumOfSolarEnclosures`. There is no corresponding scratch or flag-vector
assertion, and release builds may omit both checks.

The subsequent flag scan never clears an earlier true value. For each column
`N`, it sets flag `N` true when any distinct receiver `M` has
`D(M, N) > 0`. Under the consumer orientation this is a positive
outgoing/source-column witness, whereas the direct pass set the current
receiver flag. Ordinary reciprocal pairs normally make those sets coincide,
but malformed or asymmetric inputs need not. Positive infinity passes this
ordered test; zero, negative values, negative infinity, and NaN do not.

The final fixed-depth expansion reads only frozen pair matrix `D` and adds
to the already copied target. It requires every visited node's mixed-purpose
receive flag true and all nodes in a path distinct. For distinct
`I, J, K, L, M`, the exact additions are

- two edges: `Z(I, K) += D(J, K) * D(I, J)`;
- three edges: `Z(I, L) += D(K, L) * D(J, K) * D(I, J)`;
- four edges: `Z(I, M) += D(L, M) * D(K, L) * D(J, K) * D(I, J)`.

With receiver rows and source columns these represent the ordered simple
paths `K -> J -> I`, `L -> K -> J -> I`, and
`M -> L -> K -> J -> I`. Each edge gate skips only exact
`D(edge) == 0`, so either signed zero skips while negative, infinite, and
NaN values enter multiplication. Newly added target values never feed later
products. The routine enumerates no path longer than four edges, permits no
node revisit in these multi-enclosure paths, and never updates a diagonal in
this phase. The bilateral pair denominator is the only separate repeated
two-node-return treatment.

CP163 returns no status and emits no diagnostic. It has no catch, rollback,
cleanup, or local topology/numeric validation; its only explicit checks are
the two post-copy dimension assertions. Allocation failure can leave a
three-object prefix. Invalid arrays or indices can fail during reset or the
direct pass. A later failure can leave raw direct coefficients in the target
with a partially transformed scratch matrix, a complete pair matrix in the
target with partial flags, or a prefix of additive two-, three-, and
four-edge paths. A direct retry with coherent arrays starts by erasing all
three objects to false/identity and normally recomputes rather than
accumulating across calls. Kickoff retry instead commits the reset state and
returns. A direct unit or malformed caller bypasses the production
`InterZoneWindow` guard.

When production `InterZoneWindow` is false, the caller does not enter CP163,
so prior target, flag, and scratch values receive no reset and remain dormant
stale. CP159 and CP160 gate their normal interzone-exchange consumers on the
same `InterZoneWindow` value, so that skipped-path state is not consumed by
those normal paths while the gate remains false.

Exactly two direct owners hold CP163 mutations. `HeatBalSurf` owns
`ZoneFractDifShortZtoZ` and `EnclSolRecDifShortFromZ` and clears both by
placement-new reconstruction. The same owner also supplies the surrounding
caller's read-only `InterZoneWindow` gate; CP163 never writes it, and
placement-new restores it false. `HeatBalSurfMgr` owns scratch
`DiffuseArray` and clears it explicitly. Because the single allocation
guard observes only the first owner's target matrix, clearing either owner
alone breaks the assumed all-three allocation coherence: a manager-only
clear can leave the target allocated while scratch is absent, and a
HeatBalSurf-only clear can make the routine allocate against an already
allocated scratch object. Global, Surface, Construction, and ViewFactor
state are read-only dependencies; clearing them does not replay CP163.

Rust models adjacent-zone opaque heat-transfer links and separately parses
window optical inputs, but it has no
`ComputeDifSolExcZonesWIZWindows`, Solar-enclosure receiver/source exchange
matrix or flags, bilateral denominator transform, fixed simple-path
expansion, kickoff reset, or matching failure/re-entry lifecycle. Those
adjacent-zone and optical declarations do not implement CP163.

CP163 adds required `source_mapped`
`routine.compute_dif_sol_exc_zones_wiz_windows` under the existing
Surface-manager algorithm and the matching project-contract requirement
immediately after `compute_int_sw_absorp_factors` and before
`calc_heat_balance_outside_surf` in source-definition order. It adds no
EnergyPlus source inventory, Rust target/code/state, test, support,
capability, output, numerical, performance, or conformance promotion. The
inventory becomes 32 algorithms and 172 routines, split into 58
`state_mapped` and 114 `source_mapped`, with 65 required.

CP164 next maps `InitEMSControlledSurfaceProperties`, declared at
`HeatBalanceSurfaceManager.hh` line 119 and implemented at
`HeatBalanceSurfaceManager.cc` lines 4646-4720. Its sole production call is
`HeatBalanceManager.cc` line 2663 under
`AnyEnergyManagementSystemInModel`; definition adjacency does not make it
CP163's runtime successor. No unit test calls CP164 directly.

### `CheckValidSimulationObjects` state contract

<!-- routine-state-contract:v1 begin check_valid_simulation_objects -->
CheckValidSimulationObjects

read_state:
- the inline `GetHeatBalanceInput` gate immediately after `CreateTCConstructions`: EnergyPlus calls `CheckValidSimulationObjects` only when `TotSurfaces > 0 && NumOfZones == 0`, then calls `CheckUsedConstructions`; the parent routine and exact gate remain source-mapped
- bounded Rust enters immediately after the CP103 projection, suppresses the check after any prior compile Error, requires an empty typed Zone arena, and accepts only raw `Shading:Site:Detailed` or `Shading:Building:Detailed` presence as a monotonic positive witness that at least one source Surface exists
- validity is raw presence only across EnergyPlus's ordered eight-family test: `SolarCollector:FlatPlate:Water`, `Generator:Photovoltaic`, `Generator:InternalCombustionEngine`, `Generator:CombustionTurbine`, `Generator:FuelCell`, `Generator:MicroCHP`, `Generator:MicroTurbine`, or `Generator:WindTurbine`; fields, names, references, typing, and runtime support are not inspected by this check

write_state:
- no model state is written; when the bounded surface witness exists, the typed Zone arena is empty, no prior Error exists, and all eight allowed raw families are absent, the compiler emits one blocking `InvalidSimulationWithoutZones` Error attached to object type `GetHeatBalanceInput` with no object name or field
- a prior Error, any typed Zone, absence of both bounded surface witnesses, or presence of any one allowed raw family suppresses this diagnostic without publishing a flag, identity, graph edge, capability, manifest, comparator, proof variable, or runtime state
- allowed-family presence affects only this bounded validity diagnostic and never promotes that family to typed input support or runtime execution

history_state_ownership:
- the compiler owns no new persistent state for this diagnostic-only check; it reads immutable typed and raw input snapshots and may append one compile Error, with no surface, Zone, generator, collector, or runtime history

unsupported_state:
- the complete EnergyPlus `TotSurfaces` population and order, including legacy rectangular shading, Zone shading, heat-transfer and fenestration surfaces, generated opposite or multiplier surfaces, and every other surface family omitted from the two bounded positive witnesses
- input parsing, field/reference validation, simulation state, electrical or thermal behavior, and runtime support for every allowed collector/generator family; raw count presence is not a support claim
- the caller's mutable `ErrorsFound` flag, later fatal termination, full input-processor recovery behavior, and interaction with diagnostics or state outside the bounded compiler pass

inactive_branches:
- any prior compile Error suppresses the check so a diagnostic is not inferred from partially published typed arenas
- a nonempty typed Zone arena or no raw detailed Site/Building shading witness is silent; the latter is inconclusive rather than proof that EnergyPlus `TotSurfaces` is zero
- presence of any one of the eight raw allowed families is silent regardless of whether that family has typed or executable Rust support

unsupported_active_branches:
- an allowed raw collector/generator witness only suppresses `InvalidSimulationWithoutZones`; its own unsupported-object, malformed-input, or run-blocking treatment remains independent and unchanged
- a detected no-Zone invalid-simulation witness is compile-blocking and adds no partial runtime admission or simulation state

not_claimed_branches:
- exact parity for `TotSurfaces > 0 && NumOfZones == 0`, negative inference from absent detailed Site/Building shading, any other surface family, legacy `Shading:Site` or `Shading:Building`, generated surfaces, or invalid/raw Zone recovery
- exact EnergyPlus severe/fatal text, punctuation, severity mapping, order, multiplicity, `ErrorsFound` timing, or downstream termination; negative construction-use flags and unused warnings beyond CP105 positive metadata, collector/generator semantics, runtime, reporting, and conformance
<!-- routine-state-contract:v1 end check_valid_simulation_objects -->

### `CheckUsedConstructions` state contract

<!-- routine-state-contract:v1 begin check_used_constructions -->
CheckUsedConstructions

read_state:
- the source call immediately after the inline no-Zone validity gate and before the caller's fatal `ErrorsFound` barrier; bounded Rust calls `Compiler::collect_known_construction_use_evidence` after `check_valid_simulation_objects_bounded` in the same relative order
- every retained typed `BuildingSurface:Detailed` contributes its already resolved ConstructionId as monotonic known-used evidence; this neither claims the complete EnergyPlus Surface arena nor interprets an absent typed surface as proof of non-use
- the six source-ordered raw reference scans and exact fields: `Pipe:Indoor.construction_name`, `Pipe:Outdoor.construction_name`, `Pipe:Underground.construction_name`, `GroundHeatExchanger:Surface.construction_name`, `DaylightingDevice:Tubular.construction_name` corresponding to legacy alpha field 4, then `EnergyManagementSystem:ConstructionIndexVariable.construction_object_name`
- raw references contribute only when the field is a nonblank string that case-insensitively resolves to an existing typed ConstructionId; missing fields, blanks, wrong JSON value types, and unresolved names are silent and publish no diagnostic or placeholder
- only resolved GroundHeatExchanger and EMS references are candidates for known-CTF-use evidence, and only when the typed construction kind is Opaque, including ordinary and F/C generated variants, or AirBoundary; Fenestration, ComplexFenestration, and WindowEquivalentLayer are the exact excluded window kinds
- known-CTF-use is only positive metadata mirroring the source non-window `IsUsedCTF` mark; inclusion of an AirBoundary ConstructionId does not assert that the construction owns or requires CTF coefficient state
- any compile Error already present when the collector begins, including one emitted by the preceding CP104 check, makes the collection transactionally inactive and leaves both evidence vectors empty

write_state:
- `TypedModel::known_used_constructions` is a sorted and deduplicated positive-only ConstructionId set containing the union of retained typed-surface references and all resolved references from the six raw families
- `TypedModel::known_ctf_used_constructions` is a separately sorted and deduplicated positive-only ConstructionId subset containing only qualifying non-window GroundHeatExchanger and EMS references; Pipe, tubular daylighting, and retained typed-surface references never imply CTF evidence in this checkpoint
- the two vectors add no public object identity, ConstructionId, construction name, object count, graph edge, support row, capability, manifest, comparator, proof variable, diagnostic, or runtime state

history_state_ownership:
- TypedModel owns two immutable positive-evidence ConstructionId vectors; no mutable `IsUsed`/`IsUsedCTF` flag, surface history, construction history, CTF/CondFD history, or runtime selection state is allocated

unsupported_state:
- the complete EnergyPlus global Construction arena after WINDOW5 and thermochromic child append, generated identities absent from the typed ConstructionId arena, all Surface families beyond the retained typed detailed opaque subset, and any other use-marking path outside the six bounded raw scans
- negative `IsUsed` or `IsUsedCTF` state, the source count of constructions lacking use, summary and per-name unused-construction warnings, the `DisplayExtraWarnings` branch, and exact warning text/severity/order/multiplicity
- downstream CTF, CondFD, HAMT, window, daylighting, pipe, ground-heat-exchanger, EMS, and surface consumers; the raw six-family scan validates no referenced object's own fields, dependencies, or executability

inactive_branches:
- any prior compile Error leaves both evidence vectors empty so partially published typed arenas cannot yield positive metadata
- missing, blank, wrong-typed, or unresolved raw construction references remain silent and add no evidence; they are not reclassified as malformed typed definitions by this routine
- duplicate evidence from multiple surfaces, repeated raw records, or overlap between typed-surface and raw references collapses to one ConstructionId in each independently sorted vector
- a resolved Fenestration, ComplexFenestration, or WindowEquivalentLayer GroundHeatExchanger/EMS reference adds known-used evidence but no known-CTF-used evidence; the same reference from Pipe or tubular daylighting never adds CTF evidence regardless of construction kind

unsupported_active_branches:
- raw Pipe, GroundHeatExchanger, DaylightingDevice, and EMS object presence remains outside typed object coverage and runtime support; a resolved construction-reference string contributes metadata only and never promotes the referring family
- known-used or known-CTF-used evidence does not remove any existing all-definition run blocker, admit partial runtime, choose a heat-transfer algorithm, or prove that another construction is unused

not_claimed_branches:
- complete source `IsUsed`/`IsUsedCTF` mutation parity, false-state parity, complete Construction or Surface coverage/order, source invalid-input recovery, WINDOW5 or thermochromic-child identity, and negative inference from either evidence vector
- unused construction counts or names, `DisplayExtraWarnings`, exact diagnostics/order/multiplicity, CTF/CondFD selection or initialization, pipe/ground-heat-exchanger/daylighting/EMS semantics, runtime, reporting, numerical parity, capability support, and conformance
<!-- routine-state-contract:v1 end check_used_constructions -->

### `ProcessZoneData` state contract

<!-- routine-state-contract:v1 begin process_zone_data -->
ProcessZoneData

read_state:
- EnergyPlus calls `GetBuildingData` after `GetConstructData` and orders `GetShadowingInput`, `GetZoneData`, then `SetupZoneGeometry`; bounded Rust state covers only each Zone declaration processed at the start of `GetZoneData`, while both wrappers remain source-mapped
- one required nonblank outer-key name; finite relative-north and x/y/z origins default to zero without numeric bounds, Type defaults to and must equal integer 1, and Multiplier defaults to integer 1 with source-compatible positive signed-integer range
- Ceiling Height, Volume, and Floor Area retain every finite authored number including zero or negative values, while missing, blank, or `Autocalculate` input retains an explicit `AutoCalculate` selector
- inside overrides accept Simple, TARP, CeilingDiffuser, TrombeWall, AdaptiveConvectionAlgorithm, or ASTMC1340; outside overrides accept SimpleCombined, TARP, MoWiTT, DOE-2, or AdaptiveConvectionAlgorithm; blank or missing values inherit already parsed global selections, defaulting to TARP and DOE-2 when no global object exists
- Part of Total Floor Area defaults to Yes and retains Yes as true or No as false; staged IDF Zone instances use recovered declaration order while native epJSON instances use lexical key order

write_state:
- a deterministic dense `Zone` arena and normalized name map retain each validated `ZoneId`, normalized name, relative-north angle, origin, source-forced standard Type value 1, positive multiplier, three `AutoOrNumber` selectors, and total-floor-area membership flag
- each convection selection retains both its effective enum and whether it was inherited or authored locally; all fields and normalized duplicate checks complete before ID/name publication, so an invalid earlier record does not consume a dense identity
- positive authored Floor Area is preferred by shared runtime geometry, schedule/internal-gain, CLI IdealLoads, and typed IdealLoads outdoor-air consumers; nonpositive or autocalculated values fall through to the available geometry path rather than being normalized during declaration parsing
- any zone-local convection override is reported as `UnsupportedZoneConvectionOverride` and `RunBlocked` before arbitrary runtime execution because the current coefficient paths do not consume zone-local selectors

history_state_ownership:
- TypedModel owns immutable Zone declaration descriptors only; this checkpoint allocates no mutable zone-air, surface, weather, equipment, geometry, reporting, or predictor/corrector history

unsupported_state:
- the remaining `GetZoneData` stages and allocations outside this routine: pre-loop ZoneDaylight and resilience arrays, the separately bounded post-Zone collection, local-environment, and Space declaration/default contracts, plus post-local-environment ZonePreDefRep allocation
- `SetupZoneGeometry` realization of ceiling height, volume, and floor area; five-percent comparison warnings, space-area adjustment, nonpositive-volume recovery, coordinate-system warnings, centroids, bounds, and other surface-derived state
- four zone outdoor-air output registrations, weather and EMS updates, actual inside/outside convection coefficient selection, CeilingDiffuser recovery, building floor-area reporting, EIO/SQLite output, and conformance evidence

inactive_branches:
- missing or blank local convection fields inherit the effective project setting and do not add a runtime boundary; when the project object is absent the retained effective defaults are TARP inside and DOE-2 outside
- missing, blank, or `Autocalculate` geometry selectors remain explicit auto state; authored zero or negative values remain finite declaration values but are treated as geometry selectors requiring downstream realization
- Part of Total Floor Area equal to No is retained but has no current building-total reporting consumer; Type always materializes as the standard value 1

unsupported_active_branches:
- every authored zone-local inside or outside convection override, including one equal to the inherited effective value, is typed but blocks arbitrary runtime execution until the zone-local coefficient consumer is wired
- Space declaration/default topology, nominal control, Zone collections, and local-environment node linkage belong to separate bounded GetSpaceData, GetZoneData, and GetZoneLocalEnvData contracts; typed detailed-surface lookup/same-Zone validation and remainder/fallback assignment belong to the bounded CP97 GetHTSurfaceData/CreateMissingSpaces composite, while full GetHTSurfaceData and broader geometry realization remain deferred

not_claimed_branches:
- complete `GetBuildingData` or `GetZoneData` parity, broad Rust compiler pass-order parity, source control-character restoration, source/native canonical enum-case behavior, whitespace-preserving or case-colliding names, shared Zone/Space/ZoneList/SpaceList namespace uniqueness, invalid-input recovery, exact diagnostics/order/multiplicity, downstream geometry correction and warnings, output registration, runtime convection, reporting, numerical parity, and conformance
<!-- routine-state-contract:v1 end process_zone_data -->

## Data Structure Map

| EnergyPlus data | Rust target | Boundary |
|---|---|---|
| `DataHeatBalance::ZoneData` declaration and post-list/local-environment/space fields | `ep_model::Zone`, `ep_model::ZoneConvectionAlgorithm` | dense ID/name, north/origin, standard type, direct multiplier, raw auto geometry selectors, effective inherited-or-local convection algorithms, total-floor-area membership, nominal-control presence, list multiplier, source-shaped list identity, optional linked generic outdoor-air node, and ordered authored/default/remainder SpaceIds are retained; positive authored floor area is consumed, while local convection overrides, Zone grouping, local-environment declarations, authored partitions, and generated remainders run-block before deferred geometry/report/runtime state |
| `DataHeatBalance::ZoneListData` and `ZoneGroupData` | `ep_model::ZoneList`, `ep_model::ZoneGroup`, and per-Zone `list_multiplier`/`list_group` | staged-IDF or native-epJSON ordered dense collections retain resolved membership, longest member name, list reference, positive multiplier, repeated-list and grouped-overlap validation, and member-Zone propagation; every definition run-blocks until list-target expansion and comprehensive list-multiplier consumption are wired |
| `DataHeatBalance::ZoneLocalEnvironmentData` and `ZoneData::LinkedOutAirNode` | `ep_model::ZoneLocalEnvironment` and `Zone::linked_outdoor_air_node` | staged-IDF or native-epJSON ordered dense declarations retain resolved ZoneId and optional one-member-NodeList or direct generic NodeId; last nonblank link wins and a later blank does not clear it; every definition run-blocks until OutdoorAir:Node condition state and local-weather consumers are wired |
| `DataHeatBalance::SpaceData`, `SpaceListData`, `spaceTypes`, and `ZoneData::spaceIndexes` | `ep_model::Space`, `ep_model::SpaceList`, `ep_model::SpaceOrigin`, `SpaceTypeId`, typed name maps, and `Zone::spaces` | lexical dense authored Spaces retain resolved Zone, numeric-or-Autocalculate geometry selectors, first-seen normalized type identity, ordered tags, and authored origin; lexical SpaceLists retain ordered authored-Space membership including valid empty lists; Zone-order General defaults ensure every Zone has a Space but remain outside the preceding SpaceList name map. A later bounded pass appends General `AutoZoneRemainder` entries for mixed detailed-surface assignments. Authored Spaces, every SpaceList, and remainders run-block; sole whole-zone defaults do not, while Space surface lists/geometry/runtime consumers remain deferred |
| `DataSurface::SurfaceData::{Zone, spaceNum}` | `ep_model::Surface::{zone, space}` and `ep_model::SpaceOrigin` | the typed detailed opaque subset resolves optional Space names through the full pre-remainder arena with same-Zone validation, retains explicit targets, assigns all-implicit Zones to their existing last Space, and redirects mixed implicit surfaces to a Zone-order remainder; other surface families, opposite-surface generation, reordering, per-Space surface lists, geometry, and runtime consumers remain deferred |
| `DataSurface::AllVaryAbsOpaqSurfaceList` selected by `GetVariableAbsorptanceSurfaceList` | `TypedModel::variable_absorptance_surface_bindings` and `ep_model::VariableAbsorptanceSurfaceBinding` | after the retained detailed-surface pass, dense-order Outdoors surfaces whose construction outside layer owns a typed overlay retain a SurfaceId-to-MaterialVariableAbsorptanceId binding; non-Outdoors outside-layer uses and each typed inside-layer occurrence produce bounded warnings only. Full `AllHTSurfaceList` membership/reorder parity, other surface families, runtime updates, exact warning text/order/multiplicity, numerics, and conformance remain deferred, and every overlay still run-blocks |
| `SurfaceProperty:IncidentSolarMultiplier` request fields before `Surface::hasIncSolMultiplier` and `SurfIncSolMultiplier` mutation | `TypedModel::surface_incident_solar_multiplier_requests`, `ep_model::SurfaceIncidentSolarMultiplierRequestId`, and `ep_model::SurfaceIncidentSolarMultiplierRequest` | a dense request arena retains the nonsemantic normalized declaration key, unresolved normalized window target, inclusive-[0,1] default-1 multiplier, and optional resolved ScheduleId without creating or mutating a SurfaceId. Duplicate targets and missing schedules fail closed; source order, fenestration lookup and eligibility, construction/shade checks, per-surface overwrite state, schedule evaluation, runtime, reporting, and conformance remain deferred, and every request run-blocks |
| `SurfaceProperty:SolarIncidentInside` first-phase `SurfaceSolarIncident::{Name, SurfPtr, ConstrPtr, sched}` state | `TypedModel::surface_solar_incidents`, `ep_model::SurfaceSolarIncidentId`, and `ep_model::SurfaceSolarIncident` | each dense record retains a normalized semantic name without a name map plus one typed detailed-opaque SurfaceId, any typed ConstructionId, and required ScheduleId. Duplicate names and construction-mismatched surface pairs remain valid, repeated resolved pairs fail closed, and no source order is claimed. CP101 separately types the following complex-fenestration request family, while representative-surface mutation, full completeness, pair lookup, schedule sampling, solar replacement, runtime, reporting, and conformance remain deferred. CP102 only emits a nonblocking monotonic warning when the retained typed opaque subset is already mixed by exact current-construction pair matching; every definition run-blocks and no graph edge is added |
| `ComplexFenestrationProperty:SolarAbsorbedLayers` second-phase `FenestrationSolarAbsorbed::{Name, SurfPtr, ConstrPtr, NumOfSched, scheds}` request state | `TypedModel::fenestration_solar_absorbed_requests`, `ep_model::FenestrationSolarAbsorbedRequestId`, and `ep_model::FenestrationSolarAbsorbedRequest` | each dense request retains a normalized semantic name without a name map, unresolved normalized fenestration target, typed complex-fenestration ConstructionId, and outside-to-inside ScheduleIds whose count exactly matches `complex_fenestration.optical_layers`. Duplicate names remain valid and duplicate target/construction pairs fail closed because no source order is claimed. Source `NumAlpha`/trailing-blank positional parity, source indexing defects, fenestration binding, full completeness, pair lookup, schedule sampling and value/type limits, BSDF layer absorption, runtime, reporting, and conformance remain deferred; every request run-blocks and no graph edge is added |
| `CheckScheduledSurfaceGains` per-Zone `ZoneScheduled`/`ZoneUnscheduled` diagnostic state | `Compiler::check_scheduled_surface_gains_typed_subset` and compile warnings only | after error-free final Space assignment, a Zone warns nonblockingly only when retained typed detailed opaque surfaces already include both an exact current SurfaceId/ConstructionId pair match and a miss; only known misses are named. Empty, all-matched, and all-unmatched subsets are silent. No model state is added; full surface completeness, windows, other surface families, active constructions, pair lookups, exact diagnostics, and runtime remain deferred |
| inline representative-surface EIO header and assignment rows | mapped/deferred barrier only | the exact condition, unconditional-when-enabled header, complete global Surface order, non-self assignment filter, and representative-name payload are documented without adding a synthetic routine or Rust output state; the project flag, full Surface population/order, representative/constituent mutation, and EIO writer remain absent |
| `CreateTCConstructions` master `specTemp`/`TCChildConstrs` and child layer/name/temperature projection | `TypedModel::construction_thermochromic_series`, `TypedModel::construction_thermochromic_children`, `ep_model::ConstructionThermochromicSeries`, `ep_model::ConstructionThermochromicChild`, and `ep_model::ThermochromicConstructionChildId` | dense master-ConstructionId then ordered-state projection retains the initial first-state temperature and one child per state, including the first; each child clones the effective stack, replaces only the final retained TC layer, derives the outside layer, and uses source-shaped `{:.0R}` naming with pinned boundary examples. Projection IDs are not ConstructionIds; arbitrary-finite exact formatter equivalence, global append/count/name/graph state, WINDOW5-relative ordering, deep-copy fields, collision lookup, switching, reporting, and runtime remain deferred |
| inline no-Zone gate and `CheckValidSimulationObjects` return consumed as caller diagnostics | `Compiler::check_valid_simulation_objects_bounded` and compile diagnostics only | after the CP103 projection and absent prior Errors, an empty typed Zone arena plus raw detailed Site/Building shading presence is a positive Surface witness; absence of all eight raw allowed collector/generator families emits one blocking `InvalidSimulationWithoutZones` Error. No model state is added. Full `TotSurfaces` parity, other surface families, exact severe/fatal behavior, allowed-family semantics, runtime, and conformance remain deferred; raw allowed presence is not a typing or support claim |
| `CheckUsedConstructions` retained-surface plus six-family reference scan before `ConstructionProps::{IsUsed, IsUsedCTF}` and unused warnings | `TypedModel::known_used_constructions`, `TypedModel::known_ctf_used_constructions`, and `Compiler::collect_known_construction_use_evidence` | after the CP104 validity check and before the fatal barrier, error-free input produces sorted/deduplicated positive-only ConstructionId vectors. Known used is the union of retained typed surfaces and resolved Pipe Indoor/Outdoor/Underground, GroundHeatExchanger Surface, DaylightingDevice Tubular, and EMS ConstructionIndexVariable references in source-family order. Known CTF use contains only Opaque, including F/C, or AirBoundary IDs reached from the GroundHeatExchanger/EMS pair; all three window kinds are excluded. It mirrors only the source mark and does not assert AirBoundary CTF coefficient state. Missing/blank/wrong/unresolved fields and prior-error input publish nothing. Absence is unknown: false flags, unused count/names/warnings, `DisplayExtraWarnings`, complete generated/global identity, CTF/CondFD selection, runtime, support promotion, object counts, and graph edges remain deferred |
| inline post-input `ErrorsFound` fatal barrier | final `CompileResult::model = None` on any Error, as a coarse fail-closed analogue only | EnergyPlus stops immediately after `CheckUsedConstructions` at lines 311-313, before `InitSolarViewFactors`; Rust does not model that exact checkpoint, source diagnostic ordering, fatal message, or side effects, and this inline barrier has no synthetic routine entry |
| `ViewFactorInformation`, Solar enclosure `spaceNums`/`NumOfSurfaces`/`F`/`Area`/`SolAbsorptance`/`Azimuth`/`Tilt`/`SurfacePtr`/`SurfaceReportNums`, Surface Solar/Radiant enclosure back-pointers, and Zone `EnforcedReciprocity` | source-mapped `InitSolarViewFactors`; isolated `ep_runtime::heat_balance::radiation::{energyplus_approximate_view_factors, fix_energyplus_approximate_view_factors}` diagnostic helpers and the `approximate_view_factors_match_energyplus_1zone_eio` evidence test | the parent call is `HeatBalanceManager.cc` line 316. Full support requires report-option and EIO/debug behavior, user-factor alignment, complete merged Solar enclosure/Space/surface topology with AirBoundary exclusion, pointer/global-report ordering, geometric and inside-absorptance inputs, zero/one/at-most-three-surface branches, approximate/user matrices, InternalMass-aware fixing, `EnforcedReciprocity`, 401st-iteration and fatal paths, and all report side effects. Existing helpers and one fixture prove only bounded calculations and do not promote the routine |
| `DataSurfaces::FrameDividerProperties` | `ep_model::WindowFrameAndDivider`, `ep_model::WindowFrameProperties`, `ep_model::WindowDividerProperties`, `ep_model::WindowRevealProperties` | complete bounded immutable user-input descriptors and an independent normalized namespace are typed; fenestration binding, geometry, WINDOW 5 synthesis, shading mutation, window physics, NFRC calculations, reporting, and runtime remain blocked |
| `Construction::ConstructionProps::{Name, TotLayers, LayerPoint, isTCWindow, isTCMaster, TCMasterMatNum, TCLayerNum, TCGlassNum}` and construction/material CTF data | `ep_model::Construction`, optional immutable thermochromic master metadata, separate immutable thermochromic series/child projections, `ep_model::ModelGraph::construction_materials`, checked runtime direct-index construction/material lookup, and `ep_runtime::SurfaceCtfState` | ordinary input layers resolve into a bounded opaque/fenestration construction; every thermochromic parent contributes its first glazing state to the effective stack and only the final parent owns zero-based master metadata, while a sole SimpleGlazingSystem layer retains its original material identity and Fenestration kind. CP103 derives private ordered child snapshots without mutating the global construction/name/graph arenas or granting child ConstructionIds. Existing graph edges follow the effective or retained master IDs. The opaque runtime cache, static Regular/AirGap/IRT EIO evidence, diagnostic steady/no-mass coefficient seeding, and CTF histories do not enable thermochromic/window execution, multi-layer SimpleGlazing quirks, global child construction integration, mass-material coefficient generation, or broad face-temperature solving |
| F/C-factor construction flags, source dimensions/factors, `NominalR`, and generated material layer points | `ep_model::ConstructionGroundFactor`, private generated entries in `TypedModel::materials`, and `ModelGraph::construction_materials` | exact bounded generation formulas, ordinary-then-F-then-C ordering, private names, raw ordinals, and two graph edges are retained; surface pairing, ground temperatures, CTF/runtime, reporting, and public attachment targeting remain blocked |
| `ConstructionProps::{TypeIsAirBoundary, TypeIsAirBoundaryMixing, AirBoundaryACH, AirBoundaryMixingSched}` with zero `TotLayers` | `ep_model::ConstructionKind::AirBoundary`, `ConstructionAirBoundary`, `AirBoundaryAirExchange`, and `AirBoundaryMixingSchedule` | lexical-order zero-layer descriptors retain `None` or `SimpleMixing` input state with an optional typed schedule identity or explicit always-on selector and emit no construction/material edge; surface pairing, enclosure remapping, generated cross-mixing, schedule sampling, reporting, and runtime remain blocked |
| `ConstructionProps::BSDFInput`, complex-state layer points, `WindowThermalModel:Params`, and `Matrix:TwoDimension` snapshots | `ep_model::ConstructionKind::ComplexFenestration`, `ConstructionComplexFenestrationState`, `WindowThermalModelParameters`, `ComplexFenestrationMatrix`, `ComplexFenestrationOpticalLayer`, and `ModelGraph::construction_materials` | the bounded LBNLWINDOW/None declaration state retains normalized helper identities, original matrix spelling, derived basis length, dimension-checked global/solid optical snapshots, an alternating SpectralAverage-or-ComplexShade/Gap layer pack, and every ordered graph edge; helper families remain raw-only, while surfaces, BSDF/TARCOG/WCE calculations, reporting, runtime, and conformance remain blocked |
| `ConstructionProps::{SourceSinkPresent, SourceAfterLayer, TempAfterLayer, SolutionDimensions, ThicknessPerpend, userTemperatureLocationPerpendicular}` | optional `ep_model::ConstructionInternalHeatSource` metadata on `ep_model::Construction` | lexical epJSON overlay state is validated before attaching to one ordinary opaque construction with at least two layers; it retains the normalized diagnostic source name, strict one-based interfaces, dimension selector, authored spacing, derived half-spacing, and perpendicular temperature position without changing construction/material identity, layers, or graph edges. Source recovery, broader targets, global flags, CTF/QTF state, runtime consumers, reporting, and conformance remain blocked |
| equivalent-layer `ConstructionProps::{WindowTypeEQL, EQLConsPtr, TotLayers, LayerPoint}` | `ep_model::ConstructionKind::WindowEquivalentLayer`, `ConstructionWindowEquivalentLayer`, and `ModelGraph::construction_materials` | a staged-IDF-ordered or native-epJSON-lexical declaration retains one to eleven contiguous EquivalentLayer-family material identities, every graph edge, and a zero-based source ordinal; downstream topology repair, ASHWAT state, surfaces, reporting, runtime, ratings, and conformance remain blocked |
| `Construction:WindowDataFile` request name/file selector before `SearchWindow5DataFile` | `ep_model::ConstructionWindowDataFileRequest` and `WindowDataFileSource` | staged-IDF or native-epJSON order, normalized search name, default-versus-explicit retain-case file source, and zero-based request ordinal are retained without creating a construction/material/frame/graph identity; all external parsing, synthesized state, surfaces, reporting, runtime, and conformance remain blocked |
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
- The active ScriptF-flat/current-longwave lane now preserves the previous
  surface inside-face temperature at timestep entry for EnergyPlus-shaped
  first-pass longwave work instead of resetting `SurfTempIn` to zone MAT.
  EnergyPlus enters `CalcHeatBalanceInsideSurf2CTFOnly` with the carried
  `SurfTempIn` vector, copies it to `SurfTempInsOld`, and then calls
  `CalcInteriorRadExchange`; Rust therefore keeps the previous surface solve
  value as the "current" iteration-zero surface temperature for this lane.
  This sharply reduces the dominant floor current/history cancellation error:
  active `1ZoneUncontrolled` MAT RMSE falls from `0.022448 C` to `0.009403 C`,
  floor heat-storage RMSE from `21.090951 W` to `7.549255 W`, floor inside
  conduction from `12.263226 W` to `4.415549 W`, and floor inside-solve
  history RMSE from `293.529881 W` to `85.417533 W`. The top remaining rows are
  now exterior roof/wall convection and radiation, so the next source target is
  exterior coefficient/report timing rather than mass-floor history seeding.
- EnergyPlus iterates inside/outside surface balances before committing CTF
  histories for the timestep. Rust default diagnostics still use one pass, but
  `RUSTED_ENERGYPLUS_HEAT_BALANCE_SURFACE_ITERATIONS` and the all-CTF
  surface-iter3 probe can repeat the face-balance pass while advancing histories
  once at timestep end.
- `DataHeatBalance::SurfInitialConvCoeff = 3.076 W/m2-K` initializes inside
  convection coefficients before the selected inside convection algorithm is
  evaluated. `DataHeatBalance::LowHConvLimit = 0.1 W/m2-K` and
  `HighHConvLimit = 1000 W/m2-K` bound calculated convection coefficients.
- `DataHeatBalSurface.hh::ItersReevalConvCoeff = 30` is the EnergyPlus
  inside-convection re-evaluation cadence inside `CalcHeatBalanceInsideSurf*`.
  The ScriptF-flat official lane now has an explicit
  `hconv-reeval30-iter20` wrapper using the same cadence and the
  `energyplus-surf-initial` CTF history seed. With the active twenty-pass cap,
  this source-aligned cadence is behaviorally neutral: MAT, zone surface
  convection, air storage, inside-surface iteration count, floor storage, floor
  inside/outside conduction, floor inside hconv, floor inside convection, and
  roof outside convection RMSEs all match the no-override active lane
  bit-for-bit. The `hconv-reeval2` wrapper remains a rejected compensation
  probe: it lowers overall/floor storage RMSE (`28.786920 W` to
  `27.005834 W`) and iteration-count RMSE (`10.643041` to `8.639204`), but it
  worsens MAT (`0.037329 C` to `0.037718 C`), air storage (`9.127258 W` to
  `9.576803 W`), floor inside hconv (`0.025744` to `0.037803 W/m2-K`), and
  floor inside convection (`13.602803 W` to `17.038813 W`).
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
  Perez circumsolar term corresponding to EnergyPlus `SurfSunlitFrac` for all
  exterior opaque surfaces including horizontal roofs, and writes diagnostic
  beam, sky diffuse, and ground diffuse incident component rows next to the
  total incident solar row; detailed shadowing fractions and obstruction
  reflection factors remain outside the diagnostic claim boundary. At
  sunrise/sunset shadowing-period edges, current timestep `SOLCOS` still drives
  Perez coefficients, but circumsolar brightening is gated off when the
  averaged shadowing-period position is below the horizon, matching EnergyPlus
  lower sky-diffuse reports at those edges. `WeatherManager.cc` derives
  weather day-of-year from the run-period
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
  and third-order zone-air temperature helpers. Heat-balance runtime selection
  now exposes `CompatibilityHeatBalanceAlgorithm`, `DiagnosticHeatBalanceProbe`,
  and the probe-agnostic `HeatBalanceRuntimeConfig` so compatibility and probe
  lanes have separate typed APIs. `DiagnosticHeatBalanceProbe`, the legacy
  `HeatBalanceZoneAirAlgorithm` selector, and all long-form probe matching live
  under the `diagnostic_probes` module boundary. That boundary resolves each
  selector to `HeatBalanceRuntimeConfig` before compatibility runtime code is
  entered; the CLI delegates selector parsing and display names to the same
  module instead of matching the diagnostic variants itself. The default
  predictor equation itself remains the simplified diagnostic shell
  until all coefficient inputs are wired from source-mapped runtime state.
  Rust now has unit-checked helpers for the EnergyPlus moist-air capacitance
  formulas used by `AirPowerCap`
  (`PsyRhoAirFnPbTdbW` and `PsyCpAirFnW`), and the active dynamic diagnostic
  solver updates `ZoneHeatBalanceState::air_heat_capacity_j_per_k` from the
  timestep weather-context pressure/RH proxy immediately before zone-air
  coefficient construction. This source-order wiring lowered the active
  `1ZoneUncontrolled` MAT RMSE to `0.022407 C` and the coefficient-level
  surface-convection RMSE to `4.277641 W`. The actual EnergyPlus-owned
  `ZoneAirHumRat` path is still a promotion blocker.
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
- The compact digest and markdown report also carry Rust-only
  `zone_air_first_sample_trace` and `surface_first_sample_trace` tables for the
  first reported hour. The zone-air table records MAT, previous MAT history,
  EnergyPlus-style third-order coefficients, denominator, and reconstructed
  solution for each first-hour substep. In the active all-CTF, ScriptF-flat,
  20-iteration lane this proves the Rust third-order solution is internally
  consistent to about `2.5e-7 C` on the first substep, while a
  timestep-frequency EnergyPlus oracle probe still places the first-substep
  MAT about `0.0065655 C` lower; the remaining blocker is therefore upstream of
  hourly averaging and downstream of the basic third-order formula. The surface
  table exposes the timestep dry-bulb sequence and the matching per-surface
  outside balance terms before hourly averaging. EnergyPlus `WeatherManager`
  seeds the first weather-day
  interpolation from either Hour 1 or Hour 24 of the first run-period weather
  day via `firstHrInterpUseHr1` in `ReadWeatherForDay`, with the RunPeriod field
  `First Hour Interpolation Starting Values` parsed in `GetRunPeriodData`.
  Rust now preserves this RunPeriod enum and defaults to EnergyPlus `Hour24`
  instead of wrapping first-hour interpolation to the final EPW record. For
  `1ZoneUncontrolled`, the active lane's first-hour dry-bulb trace moved from
  the file-wrap sequence `2.25, 0.50, -1.25, -3.00 C` to the source-aligned
  `-6.00, -5.00, -4.00, -3.00 C`; first-sample zone outside opaque conduction
  delta fell from `821.886055 W` to `3.529915 W`, and floor storage max-abs
  fell from `701.082304 W` to `242.511509 W`.
- EnergyPlus `UpdateThermalHistories` first computes current CTF inside and
  outside fluxes into `SurfInsideFluxHist(1)` and `SurfOutsideFluxHist(1)`,
  flips the outside flux into `SurfOpaqOutFaceCondFlux` for reporting, then
  shifts the current temperature/flux slots into history slot 2 for the next
  timestep in the `SimpleCTFOnly` path. The Rust history vectors intentionally
  represent EnergyPlus history slot 2 and later, not the current slot 1; the
  remaining mass-floor storage work should therefore target the warmup/run-period
  history handoff and coupled source update order rather than another outside
  report sign flip.
- `ExecutionPlan.stages` now uses the EnergyPlus heat-balance source-order
  contract from `GetHeatBalanceInput` through `CheckWarmupConvergence` as the
  actual planning barrier sequence, including EMS callback barriers and the
  nested `ManageSurfaceHeatBalance`/`ManageAirHeatBalance`/
  `UpdateThermalHistories` sequence. `ExecutionPlan.compatibility_stages`
  keeps the same heat-balance contract for reports. This is an ordering and
  trace scaffold only; it does not promote the active official dynamic case
  until the numerical bottlenecks below pass.
- The active 1Zone dynamic lane now emits signed inside-solve CTF current and
  history source splits at each surface storage max sample. For `ZN001:FLR001`
  sample 2435, the aggregate history delta (`+312.812546 W`) decomposes as
  inside conduction signed delta (`-150.334642 W`) minus current CTF signed
  delta (`-463.147188 W`). The current CTF split shows only `+4.714888 W`
  outside-current mismatch but `-467.862076 W` inside-current mismatch, so the
  next floor-storage work should inspect inside current-term temperature
  alignment/update timing before treating the remaining error as pure history
  handoff.
- The compact diagnostic also emits annual CTF current/history split RMSE by
  surface. In the active all-CTF, ScriptF-flat, warmup-20, 20-surface-iteration
  lane, `ZN001:FLR001` has `inside_current_inside_term_delta.rmse_delta_c =
  231.720702 W` and `inside_current_outside_term_delta.rmse_delta_c =
  2.886563 W`; the max current-inside split is `717.882711 W`. Roof/wall
  current-inside RMSE stays below `0.8 W`. This confirms the next numerical
  bottleneck is floor inside-face current-term timing/source alignment, not the
  outside current term or a broad coefficient mismatch.
- The same annual CTF table now exposes Rust's history temperature/flux split
  magnitudes beside the aggregate oracle-vs-Rust history mismatch. Because
  EnergyPlus hourly output only exposes total history contribution, these
  split columns are zero-baseline Rust magnitudes rather than independent
  oracle split deltas. In the active lane, `ZN001:FLR001`
  `inside_history_delta.rmse_delta_c = 228.537244 W`, while the Rust split
  magnitudes are `in_hist_temp_rms_w = 231910.143809 W` and
  `in_hist_flux_rms_w = 864.878867 W`; roof/wall history split magnitudes are
  zero in this diagnostic because their CTF history slots remain effectively
  steady/no-mass. The next floor-storage probe should therefore focus on
  floor temperature-history state alignment and the current inside-face source
  timing before treating the residual as a flux-history handoff problem.
- The diagnostic now also emits annual inside-solve source deltas for each
  surface, using signed zero-baseline source mismatches. In the active lane,
  `ZN001:FLR001` has `inside_face_temperature_delta.rmse_delta_c =
  0.017176 C`, `implied_solve_numerator_delta.rmse_delta_c = 293.827236 W`,
  `tracked_solve_source_delta.rmse_delta_c = 275.530580 W`, and
  `solve_source_residual_delta.rmse_delta_c = 30.271817 W`. The tracked
  source split is dominated by `inside_history_delta.rmse_delta_c =
  228.537244 W` and `reference_air_coefficient_source_delta.rmse_delta_c =
  117.937364 W`; `reference_air_temperature_source_delta.rmse_delta_c =
  15.537367 W`, `inside_net_longwave_delta.rmse_delta_c = 10.514138 W`, and
  `outside_temperature_source_delta.rmse_delta_c = 0`. This makes the next
  numerical work a floor history-state plus inside reference-air coefficient
  alignment problem, not an untracked solve residual problem.
- Re-running the `hconv-reeval2` candidate with the signed split lowers the
  active top RMSE from `28.786920` to `27.005834` W on floor storage, but it is
  not promotion-ready: MAT RMSE rises from `0.037329` to `0.037718 C`, floor
  inside-convection RMSE rises from `13.602803` to `17.038813 W`, and the
  max-sample history/current split shifts to sample 1091 with reference-air
  cancellation (`-2.107333 W` total from `79.069518 W` absolute split) while
  current-inside mismatch remains dominant (`-542.690944 W`). Treat this as a
  diagnostic lane for hconv/source coupling, not the next active algorithm.
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
  A follow-up commit probe that also writes that same inside-CTF outside
  snapshot into outdoor CTF history slots is a verified no-op: all 99 compared
  series and all Rust hourly sample rows match the active inside-CTF
  outside-history lane exactly, including floor storage RMSE `45.539704`, floor
  inside conduction `26.437580`, floor outside conduction `19.262430`,
  aggregate outside conduction `25.267733`, zone surface convection
  `21.080512`, and air storage `7.486249`. This rules out a separate outdoor
  history-commit cadence mismatch; the persistent floor storage gap is in the
  inside CTF solve inputs/coupling itself rather than the subsequent outdoor
  history push.
  Replacing that lane's grey direct-view-factor interior longwave exchange with
  the current ScriptF implementation is rejected: floor storage RMSE jumps from
  `45.539704` to `6293.390244`, floor inside conduction from `26.437580` to
  `3666.401318`, floor outside conduction from `19.262430` to `2659.919883`,
  and floor inside net longwave from `24.877762` to `3613.270767`. This keeps
  ScriptF as a source-porting target, but not as a drop-in improvement until the
  surrounding EnergyPlus coupled radiation coefficient/update order is mapped.
  A narrower ScriptF flat-access follow-up keeps the same Hottel ScriptF
  coefficient generation but applies the matrix with EnergyPlus'
  `lSR = RecSurf * NumSurfaces + SendSurf` flat access order from
  `CalcInteriorRadExchange`. This becomes the strongest current 1Zone dynamic
  candidate: floor storage RMSE falls from `45.539704` to `29.868130`, floor
  inside conduction from `26.437580` to `17.354885`, floor outside conduction
  from `19.262430` to `12.667232`, aggregate inside conduction from
  `23.838450` to `18.204288`, aggregate outside conduction from `25.267733` to
  `14.690801`, and floor inside longwave drops out of the top eight
  bottlenecks. The trade-off is still visible in the latent air/report rows:
  MAT RMSE rises from `0.031384 C` to `0.038412 C`, zone surface convection
  from `21.080512 W` to `22.090516 W`, and air storage from `7.486249 W` to
  `9.195009 W`. Treat flat ScriptF access as the new source-aligned promotion
  candidate, with the next probe focused on zone-air/source reporting coupling
  and first-sample floor storage max-abs alignment rather than reverting to the
  grey direct-view-factor lane.
  A live-reference-air follow-up keeps that ScriptF flat-access path, inside-CTF
  outside-history snapshots, current longwave, converged surface cutoff, and
  20-day warmup, but thaws the timestep-start frozen surface reference air
  during the interleaved surface solve. It improves the latent zone
  surface-convection row (`22.062956` to `19.657827 W` RMSE, max `212.520279`
  to `177.745381 W`), but it is rejected for promotion because it regresses the
  dominant floor CTF rows: floor storage RMSE rises from `28.786920` to
  `81.570933` and max from `242.511509` to `412.045918`, floor inside
  conduction RMSE rises from `16.729618` to `47.254164`, and aggregate inside
  conduction rises from `18.143612` to `54.154851`. This keeps the frozen
  surface reference air in the current active floor solve and shifts the next
  zone-convection work toward report/coupling timing or EnergyPlus source-map
  refinement rather than thawing the surface reference-air input.
  A surface-reference-air report follow-up keeps that active ScriptF-flat,
  frozen-reference-air solve but reports inside convection from each surface's
  stored inside-solve reference-air snapshot. The compatibility-candidate alias
  now resolves its report flags through the execution variant, so the active
  lane applies the ScriptF-flat surface reference-air snapshot to individual
  `SurfQdotConvInRep` rows without changing zone-air `SumHADTsurfs`. Floor
  inside-convection RMSE drops from `20.828820` to `0.021677 W`, roof from
  `18.955600` to `0.044044 W`, and wall rows below `0.018 W`, while MAT
  (`0.006500 C`), zone surface convection (`0.063018 W`), floor storage
  (`0.175929 W`), and surface conduction stay unchanged. This is now promoted
  as report alias resolution, not as a zone-air source swap.
  A final-hconv report sibling recomputes TARP inside convection from final
  reported surface temperatures and report reference air while leaving the
  frozen-hconv solver untouched. It is still rejected under the current all-EIO,
  EnergyPlus-surf-initial compatibility setup: floor storage worsens from
  `0.175929` to `7.535715 W` RMSE and zone surface convection from `0.063018`
  to `11.729318 W`. This rules out a simple final `SurfTempIn` TARP-report
  recompute as the missing EnergyPlus hconv/report timing; keep the frozen
  solver coefficient path until a source-level `InitIntConvCoeff`/
  inside-iteration cadence probe can be isolated.
  A live-hconv solve sibling then keeps the same ScriptF-flat, frozen-reference
  air, current-longwave, inside-CTF outside-history, and 20-iteration path, but
  refreshes TARP inside convection coefficients during interleaved solves. It
  confirms the trade-off expected from EnergyPlus' sparse `InitIntConvCoeff`
  cadence, but it is rejected for the current all-EIO compatibility gate:
  individual inside-convection rows improve, while promoted zone/source rows
  regress. Floor inside-convection RMSE drops from `20.828820` to `1.674260 W`
  and roof from `18.955600` to `1.230614 W`, but zone surface convection rises
  from `0.063018` to `4.500161 W`, floor storage from `0.175929` to
  `7.581421 W`, and floor outside conduction from `0.075458` to `3.321042 W`.
  Keep frozen inside convection in the active floor solve; the remaining
  convection work should map EnergyPlus' exact initialization/report timing
  instead of live-updating hconv every interleaved pass.
  The active inside-solve max-sample decomposition now splits the reference-air
  source delta into hconv-coefficient and reference-air-temperature components.
  At the floor storage max sample, the implied numerator delta is
  `646.261894 W`; tracked source coverage is `557.721918 W` (`86.299676%`).
  The reference-air source contributes `156.447929 W`, but `153.850543 W` of
  that comes from the hconv coefficient and only `2.597386 W` from the inferred
  reference-air temperature. The same row splits the implied numerator into
  `508.476696 W` of inside-temperature movement and `137.785198 W` of solve
  denominator movement. This makes `InitIntConvCoeff`/inside-iteration cadence
  the next hconv target; it does not justify thawing the surface reference-air
  or live-updating hconv through every interleaved solve pass.
  Adding the EnergyPlus advanced
  `Surface Inside Face Heat Balance Calculation Iteration Count` row now shows
  the first-hour cadence is aligned on the active source-order compatibility lane:
  the first ten hourly counts are `18/18`, `15/15`, `24/24`, `21/21`, `11/11`,
  `17/17`, `18/18`, `31/31`, `34/34`, and `37/37` for oracle/Rust. The broad
  annual row still shows occasional one-iteration differences, but this no
  longer explains the first-substep MAT/floor offset. Keep the next solver work
  on repeated-day surface/zone source fixed-point alignment.
  An inside-CTF report probe then tests whether EnergyPlus report/source
  conduction should use the outside temperature snapshot consumed by the last
  inside CTF solve (`SurfOutsideTempHist(1)` shape) rather than the reported
  outside face temperature. It is rejected as a promotion path: top floor
  storage and individual floor inside/outside conduction remain unchanged
  (`28.786920`, `16.729618`, and `12.216935 W` RMSE), while zone opaque
  aggregate conduction regresses from `18.143612` to `22.208305 W` inside and
  from `11.590547` to `12.785602 W` outside. This keeps the current individual
  surface conduction report path intact and shifts the aggregate conduction
  question back to EnergyPlus advanced report-variable timing rather than the
  inside-CTF outside snapshot alone.
  A follow-on EnergyPlus source recheck of `UpdateThermalHistories` confirmed
  that the advanced zone aggregate rows are sums of the per-surface opaque
  report terms (`SurfOpaqInsFaceCond`/`SurfOpaqOutFaceCond`). A diagnostic Rust
  probe therefore made the zone aggregate accumulator sum the same per-surface
  report helpers used by individual surface conduction outputs. It is a no-op
  against the active ScriptF-flat lane: MAT, surface convection, air storage,
  zone opaque inside/outside aggregate conduction, floor inside/outside
  conduction, and floor storage all keep identical RMSE values (`0.037329`,
  `22.062956`, `9.127258`, `18.143612`, `11.590547`, `16.729618`,
  `12.216935`, and `28.786920`). This rules out zone-state-vs-surface-report
  accumulation as the next bottleneck; keep the remaining aggregate work on the
  upstream surface/source/history values and EnergyPlus report timing.
  The dynamic probe summary also carries a zone surface-convection report
  closure check against the signed sum of individual
  `Surface Inside Face Convection Heat Gain Rate` rows (`zone + surface_sum`).
  After alias-resolution, individual surface report rows are near oracle but
  `SumHADTsurfs` remains an independent EnergyPlus report source, so EnergyPlus
  `CalcZoneComponentLoadSums`/`SumHADTsurfs` cannot be approximated as a direct
  negative of the individual surface report rows. Keep the remaining work on
  `SurfTempInTmp`, reference-air timing, and hconv/source ownership. A
  2026-06-20 warmup day-end trace adds the decisive boundary evidence:
  EnergyPlus `Warmup {20} RUN PERIOD 1` ends at `ZONE ONE` MAT
  `-0.606928229693 C`, while Rust day 20 and the Rust run-period initial state
  end at `-0.600360486247 C`. The same `~0.00656 C` offset appears in the three
  previous-MAT history slots, so the current blocker is not the copy into the
  run period or hourly reporting; it is the warmup fixed point produced by the
  repeated-day surface/zone source model.
  The 2026-06-20 weather/sky and exterior-radiation split probe then removes
  another candidate cause: dry-bulb, sky temperature, horizontal infrared, and
  rain status are exact; wet-bulb max delta is `0.000009927010 C`; roof
  exterior radiation-to-air and radiation-to-ground coefficients are exact; and
  the roof radiation-to-sky coefficient RMSE is only `0.000329165799 W/m2-K`.
  Re-running the same active lane with EnergyPlus' documented 30-iteration
  inside-convection re-evaluation cadence is also a no-op: MAT, floor
  inside-face temperature, floor storage, roof outside-face temperature, roof
  net exterior radiation, and the advanced inside-surface iteration count all
  remain bit-for-bit identical to the no-reevaluation diagnostic. This keeps the
  next target on the coupled warmup fixed point, `SurfTempIn`/`SurfTempInTmp`
  source ownership, and CTF history state, rather than weather forcing,
  exterior longwave coefficients, or the 30-pass hconv cadence.
  A 2026-06-20 adjacent-air diagnostic now exposes EnergyPlus
  `SurfTempEffBulkAir` as `Surface Inside Face Adjacent Air Temperature` and
  compares it directly with Rust's inside-solve reference-air snapshot. All six
  official `1ZoneUncontrolled` surfaces carry the same annual adjacent-air
  delta shape: floor first sample oracle `-0.621792349810 C`, Rust
  `-0.615242190505 C`, first delta `0.006550159304 C`, RMSE
  `0.006201912249 C`, and max `0.007134902342 C`. The zone-air surface
  coefficient decomposition reports the same floor reference-air-temperature
  RMSE (`0.006202000197 C`) and floor inside-face-temperature RMSE
  (`0.006212567326 C`). This rejects a separate surface-reference-air reporting
  bug as the current blocker; the reference air is following the same warmup
  MAT fixed-point offset that later amplifies through `SumHATref`, inside face
  temperatures, and CTF storage.
  ScriptF-flat adiabatic report/history split probe also rejects syncing
  adiabatic outside faces to current inside faces for
  report-only state: it preserves MAT (`0.037329 C` RMSE), zone surface
  convection (`22.062956 W` RMSE), and air storage (`9.127258 W` RMSE), but
  regresses floor outside conduction from `12.216935 W` to `747.544527 W` RMSE
  and floor heat storage from `28.786920 W` to `732.801403 W` RMSE. This keeps
  `CalcHeatBalanceOutsideSurf`/`UpdateThermalHistories` adiabatic parity on the
  pre-sync outside snapshot path rather than a report-only current-inside sync.
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
  digest now carries first reported sample bottlenecks and Rust-only
  first-sample CTF component rows. After the RunPeriod first-hour weather seed
  fix, the active lane's first-sample outside opaque aggregate delta is
  `3.529915 W`, and the mass floor first sample has Rust current/history terms
  much closer to the oracle-inferred decomposition: inside current
  `1751.360499 W` versus oracle `1546.858233 W`, inside history
  `-852.644209 W` versus oracle `-650.814857 W`, outside current
  `1338.455829 W` versus oracle `1136.823976 W`, and outside history
  `-673.309511 W` versus oracle `-471.682586 W`. The remaining first-sample
  deltas are now roughly `200 W` per floor CTF component rather than the prior
  multi-kW file-wrap weather artifact, so the next EnergyPlus source-porting
  target shifts back to mass-floor face-temperature/history alignment and the
  coupled warmup/run-period source handoff. The digest also emits Rust
  run-period initial CTF history slots captured after warmup and before the
  first reported timestep; in the same active lane, the floor run-period initial
  slot sum is `-209.595019 W` inside and `-4.314343 W` outside before the first
  hour averages to `-852.644209 W` inside and `-673.309511 W` outside. Full
  inside iteration order, zone predictor/corrector equations, detailed
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
- `material_opaque_variants_001`: nonblocking static grouped-EIO layer evidence
  for the exact Regular/AirGap/IRT construction fixture

The heat-balance cases may remain diagnostic-only until v0.8 declares
tolerances and blocking gates. `material_opaque_variants_001` remains a
nonblocking static smoke and does not supply dynamic heat-balance evidence.

## Stop Rule

No heat-balance algorithm change may be promoted as conformance work unless the
changed behavior has a source-map entry in this document, an output-variable
entry in `output-variable-source-map.md`, an algorithm entry in
`specs/algorithm_ledger.toml`, and the port-ticket fields required by
`docs/src/current/project-contract.md`.
