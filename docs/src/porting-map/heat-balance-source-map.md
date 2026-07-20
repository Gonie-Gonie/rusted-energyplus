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
| global heat-balance data | `src/EnergyPlus/DataHeatBalance.hh` | `ep_model`, `ep_runtime::HeatBalanceState` |
| global heat-balance report registration | `src/EnergyPlus/DataHeatBalance.cc` | `ep_runtime::ResultStore`, conformance report metadata |
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
| material input | `Material::GetWindowGlassSpectralData` -> `Material::GetMaterialData` -> `Material::GetHysteresisData` | all 34 public base/overlay objects are inventoried in [the material-family source map](material-source-map.md); Regular, NoMass, AirGap, InfraredTransparent, RefractionExtinctionMethod, and EquivalentLayer plus only the `WindowMaterial:Glazing` `SpectralAverage` branch are typed, while equivalent-layer topology/ASHWAT consumers and full window behavior remain blocked |
| window frame/divider input | `GetFrameAndDividerData` | EnergyPlus places this routine after hysteresis and before construction input; Rust types the complete bounded object after base `parse_materials` and before `parse_constructions`, while its separate Hysteresis pass remains later, so complete pass-order parity is not claimed; every definition, including unused records, remains runtime-blocking |
| construction input | `GetConstructData` / `CreateFCfactorConstructions` / `CreateAirBoundaryConstructions` / `SetupComplexFenestrationStateInput` / `SearchWindow5DataFile` | the parent remains source-mapped; required ordinary names/layers, bounded opaque/fenestration validation, thermochromic first-state metadata, sole-layer SimpleGlazingSystem, the inline lexical InternalHeatSource overlay, the following WindowEquivalentLayer declaration state, and the final WindowDataFile request selector are typed; the F-then-C generator is state-mapped with private raw-count internal materials and exact formulas; AirBoundary is state-mapped as lexical-order zero-layer declaration state; the complex-state pass is state-mapped for bounded LBNLWINDOW/None graph state; every special construction definition/request is run-blocked while SearchWindow5DataFile expansion, equivalent-layer ASHWAT consumers, surface consumers, CTF/QTF/reporting, and window/ground/source-sink execution remain deferred |
| building/zone/space input | `GetBuildingData` -> `GetZoneData` -> `ProcessZoneData` / `GetZoneLocalEnvData` / `GetSpaceData` / `GetGeneralSpaceTypeNum` | both wrappers remain source-mapped; the complete public `Zone` declaration handled by `ProcessZoneData` is state-mapped, and bounded `GetZoneData` state now continues through nominal-control marking, ZoneList, ZoneGroup, local-environment input, and Space/SpaceList/default-Space declaration processing while the intervening ZonePreDefRep allocation remains deferred; every collection, local-environment, authored Space, SpaceList, and active raw surface-space assignment run-blocks, generated whole-zone defaults alone do not, and local-weather consumers, remainder-space/surface assignment, geometry realization, list consumers, outputs, and runtime convection remain deferred |
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
2. input acquisition through project controls, materials, frame-and-divider properties, constructions, then `GetBuildingData` in its `GetShadowingInput` -> `GetZoneData` -> `SetupZoneGeometry` order
3. `InitHeatBalance`
4. outside opaque surface balance
5. inside opaque surface balance
6. internal convective gain summation
7. zone air predictor/corrector update
8. output variable registration and sampling

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
  `HeatBalanceSurfaceManager::ManageSurfaceHeatBalance`, then
  `RecKeepHeatBalance`, then `ReportHeatBalance`. Warmup convergence is checked
  only at end-of-day after reporting, and `DayOfSim` is reset to `0` when the
  run-period warmup converges.
- `HeatBalanceSurfaceManager.cc::ManageSurfaceHeatBalance` calls
  `InitSurfaceHeatBalance`, `CalcHeatBalanceOutsideSurf`,
  `CalcHeatBalanceInsideSurf`, `HeatBalanceAirManager::ManageAirHeatBalance`,
  `UpdateFinalSurfaceHeatBalance`, `UpdateThermalHistories`, then
  `ReportSurfaceHeatBalance`.
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
all such definitions runtime-blocked, and no window execution is promoted.

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
- `CreateTCConstructions` child allocation and copying, source-formatted child names, surface-active construction switching, temperature-driven state selection, fenestration binding, optics, thermal calculations, daylighting, shading, nominal-U adjustment, EIO or other construction reporting, and runtime or conformance behavior

inactive_branches:
- ordinary constructions without a thermochromic parent retain their existing effective material stacks and bounded opaque/fenestration classification; for these records the input and effective stacks are identical
- a sole-layer SimpleGlazingSystem construction retains its one source material ID and has no thermochromic metadata; no multi-layer source quirk is materialized
- when more than one thermochromic parent is present, every parent is first-state substituted but only the final parent owns the zero-based master metadata, preserving the source overwrite behavior without generating child constructions
- when no valid `ConstructionProperty:InternalHeatSource` targets an ordinary opaque construction, every construction retains absent internal-source metadata; a retained nonzero perpendicular position on a 1-D declaration has no active consumer
- when no `Construction:WindowEquivalentLayer` definition exists, the construction arena and graph gain no equivalent-layer state; when definitions exist, any one-to-eleven-layer contiguous family-only pack remains declaration data without topology repair or an executable window consumer
- when no `Construction:WindowDataFile` request exists, the request arena is empty; missing or blank file names retain the default selector, and explicit names remain inert retain-case text even when the referenced file does not exist

unsupported_active_branches:
- every typed thermochromic parent remains an all-definition runtime blocker through its existing parent-material capability rule, including an unused parent and a parent consumed by a valid ordinary `Construction`; direct-index structural lookup does not weaken that block
- every typed SimpleGlazingSystem definition remains an all-definition runtime blocker, including an unused definition and one consumed by a valid sole-layer ordinary `Construction`
- valid bounded fenestration constructions remain typed graph state only and do not enter the opaque runtime thermal cache or acquire window execution
- every valid `ConstructionProperty:InternalHeatSource` definition, including one attached only to an otherwise unused construction, is reported as `UnsupportedSurfaceBoundary` and `RunBlocked` with `RuntimeClass::None`; a `BuildingSurface:Detailed` may retain the still-ordinary opaque target identity, but no partial or compatibility runtime is admitted
- every valid `Construction:WindowEquivalentLayer` definition, including every unused definition, is reported as `UnsupportedSurfaceBoundary` and `RunBlocked` with `RuntimeClass::None`; supported equivalent-layer materials and graph edges do not admit surface binding, reporting, or partial window runtime
- every valid `Construction:WindowDataFile` request, including every unused, defaulted, or missing-file request, is reported as `UnsupportedSurfaceBoundary` and `RunBlocked` with `RuntimeClass::None` before file I/O; no synthesized construction identity or partial runtime is admitted

not_claimed_branches:
- complete `GetConstructData` parity, source case-collision and exact duplicate-key behavior, invalid-object and mark-used side effects, exact diagnostics/order/multiplicity, multi-layer or shaded SimpleGlazingSystem source quirks, thermochromic child generation/naming/state selection, `SearchWindow5DataFile` external-file expansion and generated state, broad InternalHeatSource target and recovery quirks, equivalent-layer topology repair and ASHWAT consumers, global flags, CTF/QTF calculations, nominal-U, EIO/SQLite and other reporting, window, air-boundary, ground, radiant, or source/sink physics, runtime numerics, and conformance
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
remain in EnergyPlus's shared Space array for later surface lookup. Authored
Spaces, all SpaceLists, and any nonempty or non-string raw
`BuildingSurface:Detailed.space_name` fail closed until partition consumers are
wired, while generated defaults alone add no runtime boundary. Remainder-space
creation, actual surface lookup/Zone validation, and all geometry realization
stay with the later `SetupZoneGeometry` checkpoint.

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
- every valid ZoneList or ZoneGroup is reported as `UnsupportedZoneGrouping`, every valid ZoneProperty:LocalEnvironment as `UnsupportedZoneLocalEnvironment`, and every authored Space, SpaceList, or active raw BuildingSurface:Detailed space_name as `UnsupportedSpacePartitioning`, with `RunBlocked` before arbitrary runtime execution because downstream collection, local-weather, and space-partition consumers are not wired

history_state_ownership:
- TypedModel owns immutable nominal-control, ZoneList, ZoneGroup, ZoneLocalEnvironment, Space, SpaceList, SpaceType, and per-Zone list/local-node/space declaration state only; this checkpoint allocates no mutable local weather, geometry, sizing, reporting, equipment, surface, space-air, or zone-air history

unsupported_state:
- the pre-Zone-loop ZoneDaylight and resilience allocations and the post-`GetZoneLocalEnvData` ZonePreDefRep allocation
- ZoneList expansion for People, gains, thermostat, sizing, and every other Zone-or-ZoneList consumer, plus comprehensive `Zone.Multiplier * Zone.ListMultiplier` consumption across geometry, loads, HVAC flows, sizing, and reports
- OutdoorAir:Node condition inputs and node-connection metadata, remainder-space creation, surface-to-space assignment, Space-or-SpaceList target expansion, SetupZoneGeometry realization/correction, local weather and EMS state, space heat balance, reporting, runtime numerical behavior, and conformance evidence

inactive_branches:
- when no raw equipment connection names a Zone, its nominal-control flag remains false without a diagnostic; unmatched connection names mark no Zone in this bounded phase
- when no ZoneList, ZoneGroup, or ZoneProperty:LocalEnvironment exists, every Zone retains list multiplier one, no list-group identity, and no linked outdoor-air node, adding no boundary from those absent families
- a ZoneGroup multiplier omitted from valid input retains one but still records group membership exactly like an authored multiplier and remains inside the fail-closed grouping boundary
- a blank or missing local-environment node remains an explicit no-node declaration and does not clear an earlier nonblank link for the same Zone
- when no authored Space exists, GetSpaceData still creates one General whole-zone default per Zone but adds no space-partition runtime boundary unless an active raw surface space assignment is present

unsupported_active_branches:
- every valid ZoneList, including an otherwise unused definition, is typed but blocks arbitrary runtime execution until all Zone-or-ZoneList consumers expand it
- every valid ZoneGroup, including multiplier one, is typed but blocks arbitrary runtime execution until list multiplier semantics are comprehensively consumed
- every valid ZoneProperty:LocalEnvironment, including an otherwise unused definition or one with a blank node, is typed but blocks arbitrary runtime execution until local weather consumers are wired
- every authored Space and every SpaceList, including an otherwise unused or empty list, is typed but blocks arbitrary runtime execution until space partitioning and all downstream consumers are wired
- every nonempty or non-string raw BuildingSurface:Detailed space_name blocks defensively until the later surface-to-space lookup and same-Zone validation are ported; missing or exactly empty input adds no boundary
- the nominal-control scan does not validate or execute ZoneHVAC equipment; the existing later typed equipment-connection and IdealLoads boundaries remain authoritative

not_claimed_branches:
- complete GetZoneData parity, broad compiler pass-order parity, source partial-allocation and invalid-input recovery side effects, exact diagnostics/text/order/multiplicity, whitespace-preserving or case-colliding names, complete shared Zone/Space/ZoneList/SpaceList namespace behavior, full OutdoorAir:Node/NodeList and local-weather state, ZonePreDefRep, remainder Spaces, surface assignment, geometry, sizing, reporting, numerical parity, and conformance
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
- after all lists are processed, Zones are visited in typed order and each Zone without an authored Space receives one appended whole-zone default named after the Zone with AutoCalculate geometry, no tags, and the General space type; its late position keeps it unavailable to the preceding SpaceList lookup while later surface lookup remains deferred

write_state:
- a deterministic dense Space arena retains each fully validated authored declaration followed by generated whole-zone defaults; authored names alone enter the reference map, validation completes before SpaceId, SpaceTypeId, or Zone-link publication, and each Space retains its ZoneId, three AutoOrNumber selectors, normalized type name/id, ordered tags, and origin
- each Zone retains ordered SpaceIds and exits the routine with at least one Space; generated defaults reuse or append General through the separately bounded `GetGeneralSpaceTypeNum` helper and remain outside the authored-name map used by the preceding SpaceList phase
- a deterministic dense SpaceList arena and normalized name map retain each validated SpaceListId, lexical name, ordered authored SpaceIds, and maximum authored member-name length, including valid zero-member lists
- every authored Space, SpaceList, and nonempty or non-string raw BuildingSurface:Detailed space_name is reported as `UnsupportedSpacePartitioning` and `RunBlocked` before arbitrary runtime execution, while generated whole-zone defaults alone add no runtime boundary

history_state_ownership:
- TypedModel owns immutable authored/default Space, SpaceList, SpaceType, and per-Zone SpaceId declaration topology only; this checkpoint allocates no mutable geometry, surface, space-air, zone-air, gain, HVAC, sizing, or reporting history

unsupported_state:
- the preceding ZonePreDefRep allocation and all predefined-report mutation
- SetupZoneGeometry remainder-space creation, surface-to-space assignment and reordering, calculated height/volume/floor area, floor/volume fractions, enclosure and surface ranges, and geometry correction/warnings
- Space-or-SpaceList target expansion for internal gains, infiltration, mixing, ventilation, sizing, outdoor air, HVAC, outputs, and every space heat-balance/reporting/runtime consumer

inactive_branches:
- with no authored Space, every Zone receives one General whole-zone default and no UnsupportedSpacePartitioning boundary is added unless an active raw surface space assignment is present
- a Zone with one or more authored Spaces receives no whole-zone default; if every Zone is covered and all authored types are non-General, the registry contains no General entry
- a SpaceList with a missing or empty member array is retained as a valid zero-member list but remains inside the all-definition runtime boundary

unsupported_active_branches:
- every authored Space, including one using only defaults and not referenced by a surface or load, blocks arbitrary runtime execution until space partition consumers are wired
- every SpaceList, including an empty or otherwise unused definition, blocks arbitrary runtime execution until all Space-or-SpaceList consumers expand it
- every nonempty or non-string raw BuildingSurface:Detailed space_name blocks defensively until SetupZoneGeometry resolves the shared Space arena and validates same-Zone membership; missing or exactly empty input remains inactive

not_claimed_branches:
- complete GetSpaceData parity, source allocation sizes and partial invalid Space/SpaceType/Zone/list side effects, exact diagnostics/text/order/multiplicity, source whitespace preservation or case-colliding same-family names, complete shared Zone/ZoneList/Space/SpaceList namespace behavior, remainder Spaces, surfaces, geometry, loads, space heat balance, HVAC, reporting, numerical parity, and conformance
<!-- routine-state-contract:v1 end get_space_data -->

### `GetGeneralSpaceTypeNum` state contract

<!-- routine-state-contract:v1 begin get_general_space_type_num -->
GetGeneralSpaceTypeNum

read_state:
- GetSpaceData calls `GetGeneralSpaceTypeNum` only while generating a whole-zone default; the helper searches the ordered type registry built from validated authored Spaces using case-insensitive General matching

write_state:
- the helper reuses the first existing General SpaceTypeId or appends one General entry at the end and returns that dense identity; every generated default in the same model reuses it

history_state_ownership:
- TypedModel owns the immutable ordered normalized SpaceType name registry and dense IDs only; the helper allocates no mutable simulation history

unsupported_state:
- EnergyPlus allocation capacity and global integer numSpaceTypes storage beyond the equivalent dense typed registry
- all downstream type-based grouping, gains, reporting, geometry, HVAC, and space heat-balance consumers

inactive_branches:
- when every Zone already has an authored Space, no default is generated and the helper is not called, so a non-General-only authored registry remains unchanged
- when General already exists from an authored Space, default creation reuses that identity without appending another type

unsupported_active_branches:
- generated default creation and General identity alone add no runtime boundary; raw authored Space and SpaceList definitions are blocked by the parent GetSpaceData contract

not_claimed_branches:
- complete helper parity, source one-based numeric identity, allocation/counter side effects, whitespace-preserving labels, exact diagnostics, downstream type consumers, numerical parity, and conformance
<!-- routine-state-contract:v1 end get_general_space_type_num -->

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
- Space declaration/default topology, nominal control, Zone collections, and local-environment node linkage belong to separate bounded GetSpaceData, GetZoneData, and GetZoneLocalEnvData contracts; remainder-space, surface-assignment, and geometry-realization semantics remain deferred

not_claimed_branches:
- complete `GetBuildingData` or `GetZoneData` parity, broad Rust compiler pass-order parity, source control-character restoration, source/native canonical enum-case behavior, whitespace-preserving or case-colliding names, shared Zone/Space/ZoneList/SpaceList namespace uniqueness, invalid-input recovery, exact diagnostics/order/multiplicity, downstream geometry correction and warnings, output registration, runtime convection, reporting, numerical parity, and conformance
<!-- routine-state-contract:v1 end process_zone_data -->

## Data Structure Map

| EnergyPlus data | Rust target | Boundary |
|---|---|---|
| `DataHeatBalance::ZoneData` declaration and post-list/local-environment/space fields | `ep_model::Zone`, `ep_model::ZoneConvectionAlgorithm` | dense ID/name, north/origin, standard type, direct multiplier, raw auto geometry selectors, effective inherited-or-local convection algorithms, total-floor-area membership, nominal-control presence, list multiplier, source-shaped list identity, optional linked generic outdoor-air node, and ordered SpaceIds are retained; positive authored floor area is consumed, while local convection overrides, Zone grouping, local-environment declarations, and authored space partitions run-block before deferred geometry/report/runtime state |
| `DataHeatBalance::ZoneListData` and `ZoneGroupData` | `ep_model::ZoneList`, `ep_model::ZoneGroup`, and per-Zone `list_multiplier`/`list_group` | staged-IDF or native-epJSON ordered dense collections retain resolved membership, longest member name, list reference, positive multiplier, repeated-list and grouped-overlap validation, and member-Zone propagation; every definition run-blocks until list-target expansion and comprehensive list-multiplier consumption are wired |
| `DataHeatBalance::ZoneLocalEnvironmentData` and `ZoneData::LinkedOutAirNode` | `ep_model::ZoneLocalEnvironment` and `Zone::linked_outdoor_air_node` | staged-IDF or native-epJSON ordered dense declarations retain resolved ZoneId and optional one-member-NodeList or direct generic NodeId; last nonblank link wins and a later blank does not clear it; every definition run-blocks until OutdoorAir:Node condition state and local-weather consumers are wired |
| `DataHeatBalance::SpaceData`, `SpaceListData`, `spaceTypes`, and `ZoneData::spaceIndexes` | `ep_model::Space`, `ep_model::SpaceList`, `ep_model::SpaceOrigin`, `SpaceTypeId`, typed name maps, and `Zone::spaces` | lexical dense authored Spaces retain resolved Zone, numeric-or-Autocalculate geometry selectors, first-seen normalized type identity, ordered tags, and authored origin; lexical SpaceLists retain ordered authored-Space membership including valid empty lists; Zone-order General defaults ensure every Zone has a Space but remain outside the preceding SpaceList name map. Authored Spaces, every SpaceList, and active raw surface space assignments run-block; generated defaults alone do not, and remainder-space/surface/geometry/runtime consumers remain deferred |
| `DataSurface::SurfaceData` | `ep_model::Surface`, `ep_runtime::SurfaceHeatBalanceState` | opaque surface subset only; outside-layer roughness metadata is tracked for future exterior convection work |
| `DataSurfaces::FrameDividerProperties` | `ep_model::WindowFrameAndDivider`, `ep_model::WindowFrameProperties`, `ep_model::WindowDividerProperties`, `ep_model::WindowRevealProperties` | complete bounded immutable user-input descriptors and an independent normalized namespace are typed; fenestration binding, geometry, WINDOW 5 synthesis, shading mutation, window physics, NFRC calculations, reporting, and runtime remain blocked |
| `Construction::ConstructionProps::{Name, TotLayers, LayerPoint, isTCWindow, isTCMaster, TCMasterMatNum, TCLayerNum, TCGlassNum}` and construction/material CTF data | `ep_model::Construction`, optional immutable thermochromic master metadata, `ep_model::ModelGraph::construction_materials`, checked runtime direct-index construction/material lookup, and `ep_runtime::SurfaceCtfState` | ordinary input layers resolve into a bounded opaque/fenestration construction; every thermochromic parent contributes its first glazing state to the effective stack and only the final parent owns zero-based master metadata, while a sole SimpleGlazingSystem layer retains its original material identity and Fenestration kind. Graph edges follow the effective or retained IDs. The opaque runtime cache, static Regular/AirGap/IRT EIO evidence, diagnostic steady/no-mass coefficient seeding, and CTF histories do not enable thermochromic/window execution, multi-layer SimpleGlazing quirks, child construction generation, mass-material coefficient generation, or broad face-temperature solving |
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
