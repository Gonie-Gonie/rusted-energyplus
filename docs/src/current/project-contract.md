---
status: active
claim_level: none
owner: core
last_reviewed: 2026-07-14
---

# Project Contract

The locked oracle is EnergyPlus 26.1.0. The Rust core remains Rust-only and
does not change engineering algorithms in compatibility mode.

The machine-readable contract is `specs/project_contract.toml`.

## Mode Meanings

- `compatibility`: EnergyPlus source-order algorithm path. This is the only
  path that can produce compatibility evidence.
- `diagnostic`: compatibility functions plus extra instrumentation or probes.
  It may explain deltas, but it is not conformance evidence.
- `partial`: explicitly allowed supported subset execution. It is ad-hoc and
  never sets `conformance_claim=true`.
- `fast` and `experimental`: implementation experiments. Their results are not
  compatibility evidence.

## Allowed Optimization

Compatibility-safe optimization is limited to Rust representation, typed IDs,
precompute/cache, deterministic execution planning, output handles,
trace throttling, diagnostics, result storage, and numerical implementation
inside declared tolerance.

## Forbidden Compatibility Changes

Compatibility mode must not introduce new engineering algorithm variants,
timestep semantic changes, setpoint-manager timing changes, plant dispatch
semantic changes, or delta-tuned probes as candidate algorithms.

## Claim Requirements

A conformance claim requires:

```text
case manifest
+ declared variables or meters
+ tolerance rules
+ EnergyPlus oracle baseline
+ Rust result artifact
+ compare-summary.json
+ compare-report.md
+ blocking gate
```

Markdown wording, smoke tests, diagnostics, arbitrary IDF runs, and performance
results do not create compatibility claims.

## Full-Domain Claims

Heat-balance, HVAC, plant, and time full-domain claims use canonical
required-routine lists in `specs/project_contract.toml`. A domain can be
claimed only when its routine inventory is explicitly complete and every
listed routine is `family_gated` or `complete` in
`specs/algorithm_ledger.toml`. Limited algorithm conformance does not satisfy
this rule. Full runtime compatibility remains locked until all EnergyPlus
domains have complete inventories.

The canonical heat-balance inventory now includes
`set_pre_construction_input_parameters` immediately after the manager entry.
Its EnergyPlus boundary is the unconditional
`HeatBalanceManager::SetPreConstructionInputParameters` call from
`SimulationManager.cc` line 216 and the implementation at
`HeatBalanceManager.cc` lines 446-492. It remains `source_mapped` and required:
the current Rust dynamic construction-layer vectors and separate ordinary and
equivalent-layer limits do not implement the source's shared mutable maximum,
raw-object scans, input-buffer side effects, downstream allocation contract,
failure behavior, or lifecycle.

The canonical heat-balance inventory now also includes
`get_site_atmosphere_data` after the pre-construction-bound entry. Its
EnergyPlus boundary is the `GetHeatBalanceInput` line-264 call between project
controls and spectral/material input, the declaration at
`HeatBalanceManager.hh` line 100, and the implementation at
`HeatBalanceManager.cc` lines 1252-1317. It remains `source_mapped` and
required: Rust's Terrain-derived wind helper and fixed temperature-gradient
helper do not implement `Site:HeightVariation` intake, shared environment
mutation, diagnostics, EIO output, dependency side effects, or lifecycle.

The canonical heat-balance inventory includes `init_heat_balance` after the
required input/view-factor/internal-gain routines and before
`manage_surface_heat_balance`. Its EnergyPlus boundary is the unconditional
`HeatBalanceManager::InitHeatBalance` call at line 198 and the flag-driven
implementation at lines 2594-2821. It remains `source_mapped` and required:
the current Rust execution-plan stage, identity wrapper, and separately
bounded initialization state do not complete or promote this routine.

The inventory now also includes `allocate_zone_heat_bal_arrays` immediately
after `init_heat_balance`. Its EnergyPlus boundary is the first
`AllocateHeatBalArrays` action at line 2863, the declaration at
`HeatBalanceManager.hh` line 130, and the implementation at
`HeatBalanceManager.cc` lines 2824-2854, reached from the `InitHeatBalance`
BeginSim branch at lines 2617-2618. It remains `source_mapped` and required:
current Rust allocation and initialization shells do not implement the source
fallback bundle, exact Zone/Space and enclosure state, order, defaults,
partial-failure behavior, destructive re-entry, or clear/retry lifecycle.

The inventory now also includes `allocate_heat_bal_arrays` immediately after
`allocate_zone_heat_bal_arrays`. Its EnergyPlus boundary is the
`InitHeatBalance` BeginSim-only call at lines 2617-2618, the declaration at
`HeatBalanceManager.hh` line 132, and the implementation at
`HeatBalanceManager.cc` lines 2855-2963. It remains `source_mapped` and
required: Rust has no parent-routine analog for its ordered FanSystem,
contaminant, warmup-convergence, resilience, and report-array allocations,
defaults, conditional preservation, partial-failure state, or re-entry
semantics.

The inventory now also includes `init_conduction_transfer_functions`
immediately after the two allocation entries and before
`manage_surface_heat_balance`. Its EnergyPlus boundary is the
`InitHeatBalance` BeginSim branch at lines 2617-2622, which completes
`AllocateHeatBalArrays`, then under `AnyCTF || AnyEMPD` emits the initialization
display and calls the routine at line 2621; the canonical declaration is at
`HeatBalanceManager.hh` line 180 and the wrapper implementation is at
`HeatBalanceManager.cc` lines 6153-6202. It remains `source_mapped` and
required: Rust can consume EIO-seeded coefficients or a steady no-history
fallback, but has no native CTF/QTF generator, adaptive history/timestep
calculation, full Construction traversal and reporting, error lifecycle, or
re-entry semantics.

The inventory now also includes `init_surface_heat_balance` immediately after
`manage_surface_heat_balance`. Its EnergyPlus boundary is the unconditional
`HeatBalanceSurfaceManager::InitSurfaceHeatBalance` call at parent line 161
and the flag-driven implementation at lines 272-621. It remains
`source_mapped` and required. The existing Rust
`init_surface_heat_balance_stage` metadata and identity wrapper surround only
a limited outside-balance closure, remain intentionally absent from the
surface algorithm's target list, and do not implement or promote the complete
source routine, state, lifecycle, output, or numerical behavior.

The inventory now also includes `allocate_surface_heat_bal_arrays` immediately
after `init_surface_heat_balance` and before
`init_thermal_and_flux_histories`.
Its EnergyPlus boundary is the `InitSurfaceHeatBalance` line-350 call under the
lines-349-355 BeginSim branch, after the caller weather refresh and before the
`InterZoneWindow` reduction; the declaration is at
`HeatBalanceSurfaceManager.hh` line 101 and the implementation is at
`HeatBalanceSurfaceManager.cc` lines 1406-2206. It remains `source_mapped` and
required: Rust has no complete six-owner Surface allocation, CTF/master/source
history state, exact defaults and conditional preservation, or 78-site output
setup and re-entry lifecycle.

The inventory now also includes `init_thermal_and_flux_histories` immediately
after `allocate_surface_heat_bal_arrays` and before
`init_solar_heat_gains`. Its EnergyPlus boundary is the
`InitSurfaceHeatBalance` line-383 call inside the lines-379-384 BeginEnvrn
branch; the declaration is at `HeatBalanceSurfaceManager.hh` line 103 and the
implementation is at `HeatBalanceSurfaceManager.cc` lines 2208-2447. It
remains `source_mapped` and required: Rust's optional
`EnergyPlusSurfInitial` policy covers only configurable initial temperature,
typed boundary temperature, steady-`1/R` flux, and variable-length prior
histories, not the complete Zone/Space, fixed/master/source-history,
cavity/Kiva/OSCM, selective-reset, failure, or re-entry lifecycle.

The inventory now also includes `init_solar_heat_gains` immediately after
`init_thermal_and_flux_histories` and before
`init_int_solar_distribution`. Its EnergyPlus boundary is the unconditional
`InitSolarHeatGains(state)` call inside `InitSurfaceHeatBalance` line 457, the
declaration at `HeatBalanceSurfaceManager.hh` line 109, and the complete body
at `HeatBalanceSurfaceManager.cc` lines 2515-3776. It remains `source_mapped`
and required: Rust's incident-solar forcing diagnostic and bounded opaque
absorption do not implement the source's previous-solar latch and selective
reset lifecycle, enclosure/interzone distribution, scheduled/reflected solar,
window-model layers and shades, TDD/shelf/frame/divider paths, representative
averaging, reports/energy, or partial-failure and re-entry behavior.

The inventory now also includes `init_int_solar_distribution` immediately
after `init_solar_heat_gains` and before
`compute_int_thermal_absorp_factors` in source-definition order. Its
EnergyPlus boundary is the unconditional
`InitIntSolarDistribution(state)` call inside `InitSurfaceHeatBalance` line
468, the declaration at `HeatBalanceSurfaceManager.hh` line 111, and the
complete body at `HeatBalanceSurfaceManager.cc` lines 3778-4177. It remains
`source_mapped` and required: Rust has no Solar-enclosure/interzone
short-wave distribution, window/shade/frame/divider or movable-insulation
coupling, adjacent-window layer transfer, TDD transition-zone distribution,
matching report lifecycle, or additive failure/re-entry behavior.

The inventory now also includes `compute_int_thermal_absorp_factors`
immediately after `init_int_solar_distribution` and before
`compute_int_sw_absorp_factors` in source-definition order. Its EnergyPlus
boundary is the unconditional `ComputeIntThermalAbsorpFactors(state)` call
inside `InitSurfaceHeatBalance` line 427, the declaration at
`HeatBalanceSurfaceManager.hh` line 113, and the complete body at
`HeatBalanceSurfaceManager.cc` lines 4179-4295. The runtime call executes
before both solar routines despite the definition-order inventory. It remains
`source_mapped` and required: Rust's bounded typed-Zone thermal-radiant-gain
distribution has ordered `<= 0` gates that admit NaN and does not implement
radiant-enclosure recalculation gates,
window shade/blind/slat state, switchable glazing, frame/divider terms,
representative-surface topology, raw reciprocal/nonfinite behavior, or
matching failure and re-entry state.

The inventory now also includes `compute_int_sw_absorp_factors` immediately
after `compute_int_thermal_absorp_factors` and before
`compute_dif_sol_exc_zones_wiz_windows` in source-definition order.
Its EnergyPlus boundary is the unconditional
`ComputeIntSWAbsorpFactors(state)` call inside
`InitSurfaceHeatBalance` line 433, the declaration at
`HeatBalanceSurfaceManager.hh` line 115, and the complete body at
`HeatBalanceSurfaceManager.cc` lines 4297-4471. It remains `source_mapped` and
required: Rust has no Solar-enclosure multiplier or first-warning latch and no
production derivation of `inside_shortwave_absorbed_w_per_m2`. The full Solar
`radReCalc` gate and topology, active/base Construction and EQL split,
shade/screen/blind/switchable optics, frame/divider terms, strict 0.01 bad-sum
warning/zero branch, diagnostic side effects, and failure/re-entry lifecycle
remain source-only.

The inventory now also includes `compute_dif_sol_exc_zones_wiz_windows`
immediately after `compute_int_sw_absorp_factors` and before
`calc_heat_balance_outside_surf` in required-routine order. Its EnergyPlus
boundary is the `InterZoneWindow`-guarded
`ComputeDifSolExcZonesWIZWindows(state)` call inside
`InitSurfaceHeatBalance` line 439, the declaration at
`HeatBalanceSurfaceManager.hh` line 117, and the complete body at
`HeatBalanceSurfaceManager.cc` lines 4473-4644. It remains `source_mapped` and
required: Rust has adjacent-zone opaque heat-transfer state but no Solar
enclosure receiver/source matrix, bilateral diffuse-exchange transform,
fixed two-through-four-edge simple-path expansion, kickoff-reset lifecycle,
or matching failure and re-entry state.

The inventory now also includes `calc_heat_balance_outside_surf` immediately
after `compute_dif_sol_exc_zones_wiz_windows` in required-routine order. Its
EnergyPlus boundary is the
unconditional parent line-168 `CalcHeatBalanceOutsideSurf(state)` call, which
omits the optional zone-resimulation argument, and the implementation at lines
6951-7721. It remains `source_mapped` and required. Existing Rust
`calc_heat_balance_outside_surf_stage` metadata, the identity wrapper, and
bounded retained opaque CTF/environmental balance and report terms do not
implement or promote the complete Zone/Space/Surface traversal, exterior
boundary switch, child-call order, state, error behavior, or numerics.

The required inventory now places `calc_outside_surf_temp` after
`calc_heat_balance_outside_surf` and before `get_qdot_conv_out_per_area`,
matching its nesting in three exterior CTF/EMPD-or-TDD routes before the
parent common-tail convection store. Its EnergyPlus boundary is the
declaration at `HeatBalanceSurfaceManager.hh` lines 195-202, implementation
at `HeatBalanceSurfaceManager.cc` lines 9470-9763, and direct production
calls at lines 7355, 7419, and 7626. The parent retains complete and
optional-Zone traversal, boundary/algorithm gates, arguments, and the
immediate fatal check; CP175 owns the outside-temperature equations,
radiant-report write, source coefficients, and delayed ErrorFlag behavior.

CP175 preserves strict positive movable-insulation and strict
`CTFCross[0] > 0.01` quick-conduction decisions, surrounding-schedule then
ground-property temperature precedence, TDD coupling, no-OSCM and OSCM
slow/quick equations, and movable-insulation equations. A nonzero raw OSCM
pointer is indexed, the quick no-OSCM source branch omits the inside source
term, and the radiant-system divisor duplicates the surrounding coefficient.
Invalid source-plus-movable state commits outside history and radiation
reporting before four diagnostics, the final true flag, and a parent fatal
before CP171. Successful calls preserve a preexisting true flag, non-source
calls preserve old radiant coefficients, and no rollback repairs partial
writes.

The sole direct fixture proves only the invalid movable/source diagnostic
path. Four direct CTF parent fixtures execute CP175 incidentally without
isolating its outputs; a CondFD parent fixture skips it. Rust’s bounded
exterior CTF helpers do not cover the complete topology, branch set,
ownership, ErrorFlag/fatal order, or failure/re-entry semantics, so
`calc_outside_surf_temp` remains required and `source_mapped` without support
or conformance promotion.

The required inventory now also places `get_qdot_conv_out_per_area` after
`calc_outside_surf_temp` and before `calc_heat_balance_inside_surf`.
Its EnergyPlus boundary is the declaration at
`HeatBalanceSurfaceManager.hh` line 173, the implementation at
`HeatBalanceSurfaceManager.cc` lines 7723-7736, and the sole production call on
the right-hand side of the common-tail assignment in
`CalcHeatBalanceOutsideSurf` line 7717. It remains `source_mapped` and required.
A strictly positive `OSCMPtr` selects the modeled-other-side `HConv` and
`TConv` before rain is considered; otherwise rain selects the wet-bulb
reference and no rain selects dry bulb, both with `SurfHConvExt` and outside
temperature history term 1. The direct unit fixture covers only the default
zero-pointer wet and dry formulas. Rust's bounded exterior report helper shares
the final `-h * (surface - reference)` algebra and typed wet-weather context,
but it does not implement the raw OSCM-pointer precedence, exact history slot,
complete parent traversal and skips, caller-owned store, or failure/re-entry
lifecycle, so no support or conformance promotion follows.

The inventory now also includes `calc_heat_balance_inside_surf` immediately
after `get_qdot_conv_out_per_area` and before the CP173-mapped aggregation
helper, which in turn precedes the separately mapped general
`calc_heat_balance_inside_surf_2` child. Its EnergyPlus boundary is the
unconditional parent line-172 `CalcHeatBalanceInsideSurf(state)` call, which
omits the optional Zone-resimulation argument, and the canonical wrapper at
lines 7738-7813. It remains `source_mapped` and required. That wrapper owns
first-call and BeginEnvrn lifecycle, the CP173-mapped radiant-HVAC aggregation
call, complete versus partial and AllCTF versus general dispatch, MRT
calculation, and intermediate result updates. CP173 now owns the helper's
complete global-list aggregation and failure/re-entry boundary; CP172 owns the
complete general iteration and its
surface/window/moisture/Kiva topology, non-local partial-resimulation effects,
errors, and failure/re-entry behavior. CP174 now owns the shared checker's full
safety/diagnostic contract; this wrapper retains only the pass-by-value
warmup-counter reachability boundary shared with the distinct optimized child.
Existing Rust inside-balance stage metadata, its identity wrapper, bounded
surface passes, and the separate child mappings do not implement or promote
this complete canonical wrapper, state, lifecycle, dispatch, error behavior,
or numerics.

The required inventory now places `sum_surf_qdot_rad_hvac` immediately after
`calc_heat_balance_inside_surf` and before
`calc_heat_balance_inside_surf_2`, matching its wrapper execution before any
full, optimized, or partial solve dispatch. Its EnergyPlus boundary is the
header declaration at line 171, body at source lines 9277-9285, and sole
production call at wrapper line 7788. Every call traverses the complete raw
`allGetsRadiantHeatSurfaceList`, including optional-Zone resimulation, and
overwrites each listed `SurfQdotRadHVACInPerArea` with the exact
left-associated high-temperature radiant, hot-water baseboard, steam
baseboard, electric baseboard, and cooling-panel component sum. The five input
paths append without deduplication: an empty list and unlisted targets preserve
stale values, while duplicate entries repeat the overwrite rather than adding
the target again. All five literal wrapper calls in `HeatBalanceSurfaceManager.unit.cc` leave the
list default-empty and therefore exercise only CP173's no-op path. Four
`SizingManager.unit.cc` fixtures beginning at lines 2465, 2929, 3393, and 3818
include electric radiant baseboard input, populate the list, and indirectly
execute CP173 through `ManageSimulation`, but assert only downstream sizing.
No test calls CP173 directly, observes the aggregate, or isolates arithmetic,
order, duplicates, failure, or lifecycle. Raw indices, IEEE arithmetic, source lookup,
right-hand-side completion, later target lookup, failure prefixes, and retry
remain unchecked with no rollback. Failure at wrapper line 7788 blocks solver
dispatch, MRT, and intermediate results while already-cleared first-time or
BeginEnvironment flags can persist into parent retry. Rust has the bounded
`inside_radiant_hvac_w_per_m2` destination and consumers but no matching
five-source producer or global list lifecycle, so no support or conformance
promotion follows.

The required inventory places `calc_heat_balance_inside_surf_2` after
`sum_surf_qdot_rad_hvac` and before the distinct optimized
`calc_heat_balance_inside_surf_2_ctf_only`. Its EnergyPlus boundary is the
header declaration at lines 179-184, body at source lines 7815-8656, and the
wrapper's only two general calls at lines 7797 and 7809. An absent optional
Zone reaches it only when cached `AllCTF` is false; a present Zone always
reaches it. The four supplied Surface vectors independently define reference-
air/history, interzone, non-window, and Window work, while the optional Zone is
forwarded only to the radiation and convection children. The source zeros
exactly 13 selected Window fields, resets the global iteration count, samples
the global scheduled-source list, and can advance all Kiva instances before
its at-least-one-pass solve. Each pass snapshots the complete temperature
array, temporarily substitutes and restores all Kiva radiant temperatures,
executes the general opaque/Window/moisture branches, and commits histories
and interzone pairing. Regular Windows run only on pass one, TDD diffusers run
every pass, convection refreshes at pre-increment counts 30 through 480 on a
maximum-length run, and the strict post-increment limit breaks after 501
passes, warning outside warmup even if pass 501 first converged. The EMPD/HAMT
tail globally zeros every Zone moisture sum before rebuilding only the
supplied non-window subset. All five literal wrapper unit calls omit Zone and
retain cached `AllCTF = true`, including the Kiva-named fixture, so they select
the CTF-only child; CP172 has no direct or known exercised unit path. Raw list,
index, arithmetic, map-insertion, child-failure, diagnostic, and partial-state
semantics remain unchecked and have no rollback; retry resets only early
selected fields and the iteration count while inheriting other partial state.
Rust has no general solver, four-list/partial topology, non-CTF, Window,
Kiva, or moisture lifecycle, 501-pass behavior, or matching failure/re-entry
semantics,
so no support or conformance promotion follows.

The required inventory now places
`test_surf_temp_calc_heat_balance_inside_surf` after both inside-solver rows
and before `calculate_zone_mrt`, matching its nesting inside either solver
before the canonical wrapper tail. Its EnergyPlus boundary is the declaration
at `HeatBalanceSurfaceManager.hh` lines 192-193, implementation at
`HeatBalanceSurfaceManager.cc` lines 9287-9468, and sole production calls from
the CP172 general child at line 8481 and the optimized CTF-only child at lines
9194-9195. It remains `source_mapped` and required.

Both callers first apply the strict live-upper or fixed -100 C ordinary gate.
CP174 copies the raw Surface name, repeats that gate, and outside warmup emits
initial or recurring low/high diagnostics with a shared one-shot Zone-detail
latch. Zone details branch on floor area, AFN control, and controlled status.
The stored warmup counter is passed and incremented only by value: its zero
default and resets mean production reaches local one during warmup and zero
otherwise, leaving the coded enforced-reciprocity `> 3` and ordinary `> 10`
count fatals unreachable. Extra warnings can expose diagnostics but cannot
make those thresholds true.

The later strict live-upper-before-fatal or fixed -250 C gate fatals outside
warmup after an ordered diagnostic prefix; during warmup only values strictly
beyond +/-10000 C fatal. The extreme no-floor diagnostic preserves the source
division by `FloorArea`. All boundaries are strict, NaN fails the comparisons,
and failure can retain output, recurring-index, Zone-latch, and fatal prefixes
without rollback. The direct five-call unit fixture covers only in-range
warmup silence plus initial high/low positive-floor, default-AFN, controlled
diagnostics and already-latched Zone-detail suppression. It resets recurring
indices to zero and does not cover the true recurring-only, production-gate,
warmup-count, extreme, AFN-active, no-floor, uncontrolled, reciprocity,
nonfinite, malformed-index, failure, or retry paths. Rust has inside-face
temperature and iteration state but no matching checker, threshold/latch/
recurring lifecycle, fatal policy, or re-entry behavior, so no support or
conformance promotion follows.

The required inventory now places `calculate_zone_mrt` after
`test_surf_temp_calc_heat_balance_inside_surf` and before
`update_intermediate_surface_heat_balance_results`, preserving the canonical
inside-wrapper tail order. Its EnergyPlus boundary is the sole production call
at `CalcHeatBalanceInsideSurf` line 7811, the declaration at
`HeatBalanceSurfaceManager.hh` lines 140-141, and the complete body at
`HeatBalanceSurfaceManager.cc` lines 5583-5699. It remains `source_mapped` and
required. First-call area-times-inside-absorptance caches, optional-Zone
selection, monotonic enclosure recalculation flags, Zone and radiant-enclosure
MRT weighting and MAT fallbacks, warning and partial-failure behavior, and
re-entry remain source-only. Rust has bounded Surface area, inside thermal
absorptance, temperature, and radiant helpers but no Zone, Space, or enclosure
MRT state, cached weighting topology, fallback lifecycle, or equivalent
routine.

The following required inventory entry places
`update_intermediate_surface_heat_balance_results` after `calculate_zone_mrt`
and before
`manage_air_heat_balance`, preserving the canonical inside-balance
parent/general-or-optimized-child/tail grouping before the Air subtree. Its
EnergyPlus boundary is the sole production call after `CalculateZoneMRT` at
`CalcHeatBalanceInsideSurf` line 7812, the declaration at
`HeatBalanceSurfaceManager.hh` line 132, and the complete body at
`HeatBalanceSurfaceManager.cc` lines 4951-5020. It remains `source_mapped` and
required. Optional-zone bounds, additive exterior-Window Zone gain and
sign-selected reports, representative-surface child projection, inside
convection and solar-minus-lights assignments, the optional-zone-independent
global Kiva flux pass, nonfinite arithmetic, stale opposite-sign reports,
failure prefixes, and re-entry remain source-only. Rust's separate bounded
inside-convection report formula does not implement this orchestration,
report-state, representative-surface, or Kiva lifecycle.

The Air subtree now places required `get_air_heat_balance_input` immediately
after `manage_air_heat_balance` and before `manage_zone_air_updates`.
Its EnergyPlus boundary is the declaration at `HeatBalanceAirManager.hh` line
67, the implementation at `HeatBalanceAirManager.cc` lines 163-189, and the
sole production call under the manager-owned input latch at line 150. Every
entry resets a local aggregate false and calls `GetAirFlowFlag`,
`SetZoneMassConservationFlag`, and `GetRoomAirModelParameters` in order before
one final fatal decision. A returned first-child error does not skip later
children, the room-air child can only add an error, and the exact terminal
message is `GetAirHeatBalanceInput: Errors found in getting Air inputs`.

The parent clears its latch only after normal return. Thus final fatal or an
earlier child non-return leaves the latch true, blocks initialization,
calculation, and reporting on that attempt, and permits a same-state retry
with already committed child allocations, flags, registrations, diagnostics,
and EIO prefixes. Successful return makes later manager calls skip only this
input wrapper; direct calls do not alter the latch. CP184 itself owns no
persistent state, validation, catch, rollback, or transaction. One direct
PTAC setup call and broad integration paths exercise successful input, and
separate tests cover individual children, but no test proves aggregate child
continuation, fatal text, parent-latch transition, partial failure, or retry.
Rust's matching compatibility alias is only an identity closure around an
unrelated per-Zone coefficient calculation and owns neither the three-child
topology nor the latch. The routine remains `source_mapped` and required
without support or conformance promotion.

The next required Air-subtree entry is `get_air_flow_flag`, immediately after
`get_air_heat_balance_input` and before `manage_zone_air_updates`. Its source
boundary is `HeatBalanceAirManager.hh` line 69,
`HeatBalanceAirManager.cc` lines 191-214, and the sole production call as the
input wrapper's unconditional first child at line 179. It first overwrites
`AirFlowFlag` true, delegates complete simple-air-model parsing without
resetting or testing the shared error reference, then on normal child return
sums the five infiltration, ventilation, mixing, cross-mixing, and
refrigeration-door counts. A strictly positive total appends the exact
two-line `AirFlow Model, Simple` EIO summary even when the shared error is
true; a zero total writes no summary but still leaves the selector true.

The flag defaults false, CP185 is its sole source writer, and only
`HeatBalanceData::clear_state` restores it. It gates the later simple-mixing
initializer and `CalcAirFlowSimple` path, while the delegated parser owns the
Zone/Space reports, nine input families, counts, arrays, registrations,
diagnostics, and detailed EIO. A child non-return preserves the true flag and
its parser prefix; a later wrapper fatal leaves the parent latch true, so
same-state retry revisits non-transactional parser and output state. One
direct error fixture checks only the shared boolean, and one indirect
five-infiltration fixture asserts only later HVAC results; no test isolates
the flag write, exact EIO branch, error-plus-output behavior, or reset/retry.
Rust has no selector, parser, five totals, simple-airflow arenas, or matching
EIO. AirBoundary metadata remains run-blocked and Ideal Loads outdoor air is
a separate subsystem. The routine remains `source_mapped` and required
without support or conformance promotion.

The following required Air-subtree entry is
`set_zone_mass_conservation_flag`, after `get_air_flow_flag` and before
`manage_zone_air_updates`. Its source boundary is
`HeatBalanceAirManager.hh` line 71 and `HeatBalanceAirManager.cc` lines
216-233; CP184 calls it unconditionally at line 181 after CP185 returns and
before room-air input. A false mass-balance-enforcement control or exact
`NoAdjustReturnAndMixing` mode writes nothing. Otherwise the routine visits
only `ZoneMixing` records and sets each receiving Zone flag true before its
source Zone flag, never clearing a prior value or considering cross-mixing,
AirBoundary, refrigeration-door, infiltration, or ventilation records.

The flags begin false after normal heat-balance allocation and feed the
Zone mass-balance solver plus adjusted `CalcAirFlowSimple` mixing branches.
The routine owns no validation, error status, diagnostic, output, reset, or
rollback. In direct or externally mutated state, a malformed source endpoint
can retain the current receiving write, while repeated or disabled topology
can preserve stale true flags until the separate fan-system owner is cleared
and reallocated. Seven
direct fixtures call it, but none directly asserts the flag array; one covers
the enforced/no-adjust no-op, five exercise active downstream mass balance,
and one checks only parser-owned state. Rust has no mass-conservation controls,
typed ZoneMixing graph, endpoint flags, solver, or adjusted mixing consumer.
The routine remains `source_mapped` and required without support or
conformance promotion.

The next required Air-subtree entry is `get_simple_air_model_inputs`,
definition-ordered after `set_zone_mass_conservation_flag` and before
`manage_zone_air_updates`. Its source boundary is
`HeatBalanceAirManager.hh` line 73 and `HeatBalanceAirManager.cc` lines
235-4244; CP185 calls it unconditionally at line 207 immediately after setting
`AirFlowFlag` true. The routine allocates Zone plus sizing-or-simulation Space
reporting state, registers Zone plus simulation-only Space airflow outputs,
sizes shared parser buffers, and processes
exactly nine direct object schemas in order: Zone outdoor-air balance; three
infiltration families; two ventilation families; Mixing; CrossMixing; and
refrigeration-door Mixing. It expands count-driven records, creates
MassConservation and `ZoneReOrder` topology from Mixing, appends
AirBoundary-generated records to CrossMixing, registers output and EMS state,
writes nominal EIO rows, and stores per-Zone nominal and enforced-balance
state.

The passed error reference is transparent and monotonic: CP187 neither resets
nor tests it, and all later phases still run when it enters true. Two source
paths emit severe diagnostics without raising it—a blank induced-air schedule
for `ZoneAirBalance:OutdoorAir` and a source-only Mixing Zone lacking
Infiltration under enforced balance—while malformed continuation can instead
fail before return. There is no local latch, final barrier, rollback, or
repeat-safe cleanup. A same-state retry can meet already allocated arrays and
repeat registrations or EIO; manager-only or heat-balance-only clearing leaves
other owners stale, so whole-state reset plus normal reinitialization is the
clean domain-state boundary; output-clean replay additionally requires a fresh
or reset EIO stream. Fifteen direct unit calls cover selected infiltration,
design-flow ventilation, explicit CrossMixing, Mixing mass-balance, one
`ZoneReOrder` result, and output ordering, but do not directly cover
ZoneAirBalance, WindAndStack ventilation, refrigeration-door or generated
AirBoundary CrossMixing, either unflagged severe, exact nominal EIO, or
retry/reset behavior. Rust has none of the nine typed families, count/arena
state, topology, outputs, EMS, EIO, or consumers. The routine remains
`source_mapped` and required without support or conformance promotion.

The following required Air-subtree entry is `get_room_air_model_parameters`,
after `get_simple_air_model_inputs` and before `manage_zone_air_updates`. Its
source boundary is `HeatBalanceAirManager.hh` line 75 and
`HeatBalanceAirManager.cc` lines 4246-4492; CP184 calls it unconditionally at
line 184 as its third and final input child. It creates one default
Mixing/Direct/non-simulated room-air record per Zone even when no
`RoomAirModelType` object exists, then parses that sole direct schema's eight
model choices and Direct/Indirect coupling. Five model choices validate only
the matching Zone-name presence of their `RoomAirSettings:*` companion, while
AirflowNetwork checks only for `AirflowNetwork:SimulationControl`. Detailed
settings, RoomAir nodes and patterns, and intrazone AFN topology are later
dependencies.

The clean-entry duplicate check is ineffective because authored names are not
stored until the post-loop synthetic naming pass. Same-Zone declarations
therefore overwrite model and coupling in order while prior true simulation
and global-use flags can stick; the authored object name and `ZonePtr` are
never retained. Invalid model or coupling keys warn and fall back to Mixing or
Direct. Explicit input errors and non-Mixing Space-heat-balance conflicts
accumulate locally, but every normally reached path first writes a RoomAir EIO
header and one row per Zone; only afterward does CP188 emit its summary severe
and raise the shared flag. It never reads or clears an incoming true value.

Two direct unit calls cover missing and valid AirflowNetwork control cases, but
not default Mixing state/EIO, duplicate overwrite, other models, Indirect
coupling, Space incompatibility, state fields, repeat, or reset. Direct repeat
rebuilds Zone records but repeats EIO and preserves monotonic flags.
RoomAir-local clearing does not rearm all downstream room-air latches, the
parent latch is separate, and whole-state clearing does not reset EIO, so
end-to-end clean replay needs explicitly fresh/rearmed lifecycle and output
state. Rust has no typed room-air selector, companion settings, coupling,
flags, node topology, validation, dispatch, or EIO; all room-air inputs remain
run-blocking. The routine remains `source_mapped` and required without support
or conformance promotion.

The next required Air-subtree entry is `init_air_heat_balance`, after
`get_room_air_model_parameters` and before `manage_zone_air_updates`. Its
source boundary is `HeatBalanceAirManager.hh` line 77 and
`HeatBalanceAirManager.cc` lines 4494-4507. The wrapper has exactly one
executable statement: every entry passes the unchanged state once to
`InitSimpleMixingConvectiveHeatGains`. It owns no branch, persistent state,
status, diagnostic, output, catch, cleanup, or latch. The child remains CP190;
notably, its enforced mass-balance fractions use a global raw-`DesignLevel`
Mixing prefix rather than Zone receiving pointers or schedule/EMS-scaled flow.

Two production callers reach that wrapper. `ManageAirHeatBalance` line 154
calls it every ordinary Air-manager pass after the optional input block and
before Air calculation and reporting. If CP189 does not return, those later
children are skipped; the input latch is already false, so a same-state parent
retry skips input and calls CP189 again. `SimulationManager::Resimulate` line
2908 calls it only when `ResimHB` is true, after three Surface operations and
before refrigeration-rack work, the heat-balance iteration increment, and
forcing HVAC resimulation. That path can repeat without advancing simulation
time and deliberately bypasses the Air-manager input and calculation/report
siblings.

Five direct unit calls use CP189 only as setup for later airflow and
mass-balance assertions. None isolates delegation count, caller order,
resimulation, failure, retry, or reset; four child-direct calls bypass CP189.
Rust's `init_air_heat_balance_compat` is an identity closure whose sole call
passes an empty body. It has no simple-mixing initializer, associated flow
state, or demand-manager resimulation path. CP189 therefore remains
`source_mapped` and required without Rust state, support, output, numerical, or
conformance promotion.

The following required Air-subtree entry is
`init_simple_mixing_convective_heat_gains`, after `init_air_heat_balance` and
before `manage_zone_air_updates`. Its EnergyPlus boundary is
`HeatBalanceAirManager.hh` line 79 and `HeatBalanceAirManager.cc` lines
4509-4588; CP189 line 4506 is its sole production call. `AirFlowFlag = false`
preserves all targets. A true flag orders actual-vector Mixing schedule and
object-EMS refresh with a saved baseline, optional mass-conservation fraction
rebuild, actual-vector CrossMixing refresh without a saved baseline, and
Zone-indexed refrigeration-door zero/EMS initialization.

The fraction phase deliberately ignores each Zone's receiving pointers. It
uses that record's count `N` to zero fractions and normalize the global raw
`Mixing(1..N).DesignLevel` prefix only when its sum is strictly positive;
scheduled or EMS-overridden flow does not participate. `TotMixing` and
`TotCrossMixing` do not control their vector traversals, while
`TotRefDoorMixing` is only a positive gate. CP190 performs no infiltration,
ventilation, physical door-flow, heat-gain, allocation, validation,
diagnostic, output, or rollback work. Failures can retain an ordered prefix,
and repeat refreshes reached state while false or shortened gates preserve
untouched values.

Four direct calls in two fixtures cover only fraction no-op/zero/positive
cases and downstream CrossMixing effects. Mixing desired/saved values,
schedule and object EMS behavior, count/vector mismatches, the pointer quirk,
nonfinite arithmetic, every refrigeration-door branch, failure, repeat,
resimulation, and reset remain untested. Rust has no CP190 alias, typed simple
mixing records, fraction topology, or door state; the CP189 closure is empty,
`sum_mcp*` remains producerless here, and AirBoundary SimpleMixing metadata is
run-blocked. CP190 remains `source_mapped` and required without Rust state,
support, output, numerical, or conformance promotion.

The next required Air-subtree entry is `calc_heat_balance_air`, after
`init_simple_mixing_convective_heat_gains` and before
`manage_zone_air_updates`. Its EnergyPlus boundary is
`HeatBalanceAirManager.hh` line 83 and `HeatBalanceAirManager.cc` lines
4590-4604; `ManageAirHeatBalance` line 158 is its sole production call, after
CP189 returns and before `ReportZoneMeanAirTemp`.

Every entry selects exactly one manager path. A configured
`externalHVACManager` first calls `initializeForExternalHVACManager` when
`externalHVACManagerInitialized` is false and then invokes the callback with
`&state`; a true flag skips only that initializer. With no callback, the flag
is irrelevant and CP191 calls `HVACManager::ManageHVAC`. The external route
therefore bypasses the standard HVAC manager. Neither CP191, its initializer,
nor runtime callback registration writes the initialized flag true, so the
default false value repeats the initializer call on each external-mode entry
unless outside code changes it. The child's separate one-time latch may skip
only its internal one-time block.

CP191 performs no direct mutation, validation, diagnostic, output, status,
catch, or rollback. Initializer failure prevents the callback; failure or
non-return from either branch preserves child effects and suppresses the
parent report and later Surface-manager work. `DataGlobal::clear_state` removes
the callback and resets the flag false, while Air-manager-local clear does
neither. No direct C++ test covers CP191, its initializer, or the external
callback API. Rust's `calc_heat_balance_air_compat` is an identity closure
around a bounded predictor/zone-temperature shell, not external-versus-standard
HVAC dispatch, and has no full `ManageHVAC` topology. CP191 remains
`source_mapped` and required without Rust state, support, output, numerical, or
conformance promotion.

The next required Air-subtree entry is `report_zone_mean_air_temp`, after
`calc_heat_balance_air` and before `manage_zone_air_updates`. Its EnergyPlus
boundary is `HeatBalanceAirManager.hh` line 85 and
`HeatBalanceAirManager.cc` lines 4615-4687; `ManageAirHeatBalance` line 160 is
its sole production caller and reaches it unconditionally after CP191 returns.

On the first completely returned call, CP193 scans the actual output-request
vector and numeric EMS sensor range for exact Zone or Space Wetbulb Globe
Temperature names. Stored empty Output:Variable keys fan out over the declared
Zone or, when Space simulation is active, Space count; nonempty keys use
literal name lookup. EMS keys are not uppercased here. Matches only set
`ReportWBGT` true, and the one-time latch clears after both scans. Every call
then visits Zones by `NumOfZones` and active Spaces by each Zone's stored
`spaceIndexes`, invoking the next source routine `calcMeanAirTemps` with the
owning Zone number for both Zone and Space operative-control metadata.

The child refreshes the ordinary mean-temperature, humidity, operative, and
dew-point fields, while thermostat-operative and WBGT values update only under
their own guards and otherwise remain retained. CP193 directly emits no output
stream and validates none of its counts, vectors, memberships, or report
arenas. A discovery non-return leaves a true-flag prefix and the latch true; a
later non-return leaves the latch false plus a completed Zone/Space and child
write prefix. Its report arrays, averaged inputs, requests, sensors, controls,
pressure, and scan latch have separate owners, so clean replay requires
coordinated reset and reconstruction.

No C++ unit test calls CP193 or its child directly or targets the WBGT discovery
fields. Rust's `report_zone_mean_air_temp_compat` is only an identity closure
around one bounded Zone MAT sample during run-period aggregation. Separate MAT
and humidity series plus a CLI trace label do not implement the one-time
request/EMS scan, Space traversal, report bundle, operative/dew-point/WBGT
work, failure lifecycle, or source call topology. CP193 remains `source_mapped`
and required without Rust state, support, output implementation, numerical, or
conformance promotion.

The final required Air-subtree definition entry is `calc_mean_air_temps`,
after `report_zone_mean_air_temp` and before `manage_zone_air_updates`. Its
EnergyPlus boundary is `HeatBalanceAirManager.hh` lines 87-92 and
`HeatBalanceAirManager.cc` lines 4689-4728, after which the namespace and file
end. CP193 is its only caller: line 4678 supplies each Zone's averaged air
temperature, averaged humidity ratio, MRT, report record, and Zone number;
lines 4682-4683 do the same for each active stored Space membership while
deliberately retaining the owning Zone number for control metadata.

Every entry first overwrites `MeanAirTemp` with the supplied `ZTAV` rather than
an instantaneous MAT, copies the raw averaged humidity ratio, writes the
ordinary 50/50 mean-air/MRT operative temperature, and then evaluates dew
point from that humidity and current outdoor barometric pressure. The
dew-point child floors only its local humidity input to `1e-5`, so
`MeanAirHumRat` remains unmodified. These four report fields update on every
normally returned call.

`ThermOperativeTemp` updates only when `AnyOpTempControl` is true, the owning
Zone is controlled, and its referenced control mode is not `None`. The Zone's
control index is read before the controlled guard, but the control array is
indexed only after that guard succeeds. Exact `Scheduled` mode reads the
schedule's current value, including an active EMS override; every other
non-`None` mode takes `FixedRadiativeFraction`. The routine performs no local
index, pointer, finite-value, or `[0,1]` fraction check, redundantly restores
the ordinary 50/50 operative value, and applies
`(1-f) * ZTAV + f * MRT` without clamping. A false guard leaves the
thermostat-operative field at its prior or default value.

Only `ReportWBGT = true` evaluates the W-input wet-bulb psychrometric child and
writes `WetbulbGlobeTemp` as 70 percent wet bulb plus 30 percent the ordinary
50/50 `OperativeTemp`. It uses the raw averaged humidity ratio and never the
thermostat-weighted operative value. A false report flag preserves the
previous/default WBGT. The default EnergyPlus build routes saturation and
wet-bulb work through quantized caches and can also update raw memo, warning,
and recurring-error state. Those dependencies can clamp or iterate and still
return a value, so the helper has no diagnostic-to-status conversion and its
numeric overwrite behavior is not a claim of pure whole-routine idempotence
or cold retry.

CP194 owns no allocation, bounds or null validation, status, latch, catch,
cleanup, rollback, or transaction. A dew-point non-return retains the first
three new writes; a schedule/index failure retains the first four; a wet-bulb
non-return retains all preceding reached writes while the old/default WBGT
survives. Normal repeat resamples schedule/EMS state and overwrites the first
four fields, while false conditional guards make the thermostat-operative and
WBGT fields sticky. Clean replay spans separately owned HeatBalance report
arenas, predictor/corrector averages, Zone controls, environment pressure, and
psychrometric/cache state.

No C++ unit test directly calls CP194 or CP193 or asserts any of the target
report fields. Focused dew-point and wet-bulb tests do not cover this report
composition, control guards, Space topology, partial effects, repeat, or
reset. Rust has no `calc_mean_air_temps` alias, integrated Air-report record,
MRT/operative/control/WBGT state, Space report path, or W-input cached
wet-bulb counterpart. Its isolated dew-point projection and RH-input outdoor
wet-bulb helper are not connected to this source call topology. CP194 remains
`source_mapped` and required without Rust state, support, output
implementation, numerical, or conformance promotion.

The predictor/corrector definition inventory now adds required
`get_zone_air_set_points` immediately after `manage_zone_air_updates` and
before `init_zone_air_set_points`. Its EnergyPlus boundary is the
declaration at `ZoneTempPredictorCorrector.hh` line 272 and the single
fall-through implementation at `ZoneTempPredictorCorrector.cc` lines
246-2174.

The routine orders ordinary thermostat and four temperature-setpoint
families; humidistat; thermal-comfort thermostat and four Fanger setpoint
families; unconditional HybridModel input; Zone capacitance assignment and
average EIO; operative/adaptive comfort; temperature-and-humidity overcool;
and staged-dual input before its accumulated-error fatal. It expands
ZoneLists, allocates and links control and schedule arenas, writes inverse
Zone indices and four capacitance multipliers, registers operative-temperature
outputs, and can invoke the following adaptive-comfort helpers. Several severe
paths deliberately do not set the local fatal flag, helper fatals can stop the
transaction early, the capacitance ZoneList loop omits its final member, and
the staged precheck consumes errors from every earlier phase.

`ManageZoneAirUpdates`, conditional Kiva setup, and
`VerifyThermostatInZone` are the three production call sites. Each tests the
same `GetZoneAirStatsInputFlag` and clears it only after normal return, so the
first successful caller owns input acquisition. A non-return preserves the
ordered allocation, mutation, output, EIO, and diagnostic prefix plus the true
latch. The routine has no status, catch, cleanup, rollback, or safe same-state
retry; clean replay requires coordinated ZoneControls, predictor, Zone,
Hybrid/RoomAir, input, schedule, weather, output, file, and diagnostic reset.

Nine direct C++ unit calls cover selected ordinary control values, downstream
fixture setup, a schedule/control mismatch fatal, and a missing setpoint
reference fatal. They do not cover the complete family order, latch timing,
valid staged, overcool, positive ZoneList, capacitance-output, partial-failure,
retry, or reset paths. Rust's same-purpose identity closure only executes an
arbitrary caller body. Its compiler types a bounded direct-Zone subset of
DualSetpoint thermostat and humidistat state, with different ZoneList,
humidistat-default, validation, diagnostic, and failure semantics. That
adjacent subset, execution-plan metadata, and IdealLoads consumer do not
implement CP196. The routine remains `source_mapped` and required without a
new Rust target, mapped state, support, output, numerical, or conformance
promotion.

The next required predictor/corrector definition entry is
`init_zone_air_set_points`, after `get_zone_air_set_points` and before
`zone_space_heat_balance_begin_environment_init`. Its EnergyPlus boundary is the declaration
at `ZoneTempPredictorCorrector.hh` line 274 and implementation at
`ZoneTempPredictorCorrector.cc` lines 2350-2816. Required
`ManageZoneAirUpdates` calls it at line 220 for every selector after optional
input acquisition and before dispatch. The external-HVAC initializer also
calls it directly at `HeatBalanceAirManager.cc` line 4612 without that input
prefix.

The default-true one-time block sizes thermostat/control/report, optional
comfort/Fanger, deadband/setback, ZoneList/ZoneGroup, hybrid-history, and Zone
plus conditional Space demand state. It warning-checks each Zone's surface
reference-air consistency, registers Zone/Space heat-balance and
sensible/moisture demand bundles, direct thermostat/correction, comfort,
ZoneList, and ZoneGroup outputs, and clears its latch only after every setup
returns. Output identities, staged/latent branches, meter attachment,
multipliers, Space membership, surface ranges, and cross-owner array shape are
trusted source dependencies.

The begin-environment gate invokes the following
`ZoneSpaceHeatBalanceData::beginEnvironmentInit` child for all Zones and active
Spaces, resets four current thermostat setpoints, load correction, ordinary
control type, demand helpers, `DeadBandOrSetback`, `NoHeatToReturnAir`, and
hybrid histories, then clears its latch. It does not directly reset averaged
setpoints, ordinary control report, comfort state, `Setback`,
`CurDeadBandOrSetback`, or ZoneList/ZoneGroup totals. The begin-day phase
changes only its own latch.

Every call then verifies ordinary temperature controls followed by comfort
controls once Zone-equipment input is ready, and independently applies
demand-limit setpoint changes. Ordinary branches use strict `>`/`<`
comparisons; comfort branches use inclusive `>=`/`<=` comparisons and can
rewrite the ordinary control type/report after the ordinary loop. Missing Zone
equipment configuration sets a routine-owned sticky error after Severe and
Continue diagnostics. Its fatal follows both loops and all reached clamp
writes, while `ControlledZonesChecked` is committed only after that fatal point
on normal input-filled return.

A failure can therefore retain allocation/output, environment-reset,
diagnostic, or demand-clamp prefixes with different one-time/environment latch
states. `ErrorsFound` is never cleared locally, and no catch, cleanup, rollback,
or safe isolated retry exists. Clean replay spans predictor/corrector,
HeatBalFanSys, ZoneEnergyDemand, HeatBalance, ZoneControls, ZoneEquipment,
Surface, environment, and OutputProcessor owners.

Four direct C++ calls use the routine only as setup and assert none of these
states or latches. Rust's `init_zone_air_set_points_compat` passes one closure
through only in its Predict scaffold; the Correct shell omits it. Separately
constructed constant-DualSetpoint heating/cooling output series and limited
`ZoneSysEnergyDemand` snapshots do not implement this allocation, registration,
environment, verification, demand-limiting, failure, or source caller
topology. CP199 remains `source_mapped` and required without new Rust state,
support, output implementation, numerical, or conformance promotion.

The following required predictor/corrector definition entry is
`zone_space_heat_balance_begin_environment_init`, after
`init_zone_air_set_points` and before `update_final_surface_heat_balance`.
Its EnergyPlus boundary is
`ZoneSpaceHeatBalanceData::beginEnvironmentInit(EnergyPlusData &state)`,
declared at `ZoneTempPredictorCorrector.hh` line 213 and implemented at
`ZoneTempPredictorCorrector.cc` lines 2818-2836.

CP199 calls the member only inside its begin-environment gate: every stored
Zone record runs first, followed by every stored Space record only when current
`doSpaceHeatBalance` is true. For each fixed index 0 through 3, CP200 zeros
`ZTM` and `WPrevZoneTSTemp` while copying current `OutHumRat` into
`WPrevZoneTS` and `DSWPrevZoneTS`. It next copies that humidity into
`WTimeMinusP`, `W1`, `WMX`, and `WM2`, then zeros `airHumRatTemp`,
`tempIndLoad`, `tempDepLoad`, `airRelHum`, `AirPowerCap`, and `T1`. These 26
overwrites leave all unlisted record fields unchanged.

All indexed targets are fixed four-element arrays. Outdoor humidity is copied
without finite, sign, range, or consistency validation, so negative and
nonfinite values propagate to the 12 humidity targets without a diagnostic.
The helper owns no latch, allocation, checked access, child call, status,
catch, cleanup, or rollback and has no ordinary catchable failure path for
valid state. Repeating with unchanged outdoor humidity is overwrite-idempotent;
only CP199's outer latch limits normal environment execution. A Space mode
enabled after that latch clears in the same uninterrupted environment interval
does not replay skipped Space records.

No C++ unit test calls CP200 directly or asserts one of its targets. Fifty-six
active full-simulation tests transitively exercise the Zone path; seven
`SizingManager` tests reach sizing-Space and one `HeatBalanceAirManager` test
reaches simulation-Space, but their assertions are downstream.

Rust constructs only Zone run state with different three-slot histories. When
weather data exists, it seeds current and averaged humidity plus both histories
once from the first weather sample. That is not CP200's current-`OutHumRat`
four-slot per-environment Zone/Space reset and touches a different field set.
Rust has no Space record, exact 26-field boundary, or environment gate, and its
coefficient representation is computed through a different initialization
path. These adjacent histories and coefficients are not this member
implementation. CP200 remains `source_mapped` and required without new Rust
state, support, output implementation, numerical, or conformance promotion.

The inventory now also includes `update_final_surface_heat_balance` after
`zone_space_heat_balance_begin_environment_init`, preserving the completed
predictor/corrector definition slice before
the Surface manager's final update. Its EnergyPlus boundary is the
unconditional parent line-184 `UpdateFinalSurfaceHeatBalance(state)` call and
the implementation at lines 5176-5219. The routine always invokes seven
averaged radiant, baseboard, cooling-panel, and swimming-pool source updaters;
if any child reports an active averaged source, it reruns the complete-building
outside balance and then inside balance, without rerunning initialization, Air
balance, or histories. It remains `source_mapped` and required. The existing
Rust `update_final_surface_heat_balance_stage`, now listed as a Surface-manager
algorithm target, and its bounded adiabatic synchronization/snapshot wrapper
do not implement the seven equipment-source updates or conditional full
two-pass replay and do not promote state, support, or conformance.

The next required inventory entry is `update_thermal_histories`, after
`update_final_surface_heat_balance`; together with the existing preceding
`manage_air_heat_balance` and nested `manage_zone_air_updates` /
`get_zone_air_set_points` / `init_zone_air_set_points` /
`zone_space_heat_balance_begin_environment_init` entries, this
preserves completion of the Air subtree before the Surface manager's final and
history stages. The EnergyPlus parent calls the routine at lines 186-189 only
when `AnyCTF || AnyEMPD`, and the canonical body spans lines 5221-5581. It owns
one-time scratch allocation, current CTF/EMPD flux and report updates, the
`SimpleCTFOnly && !AnyConstrOverridesInModel` fast shift, and the normal
first-sample capture, per-surface history counter, master rollover or
interpolation, and embedded-source history paths. It remains `source_mapped`
and required. The existing Rust `update_thermal_histories_stage`, identity
wrapper, and bounded vector-history push do not implement the complete parent
gate, topology, first-time/master state, interpolation cadence, current report
terms, or internal-source histories and do not promote support or conformance.

The next required inventory entry is `report_surface_heat_balance`, directly
after `update_thermal_histories`; the intervening CP127 CondFD moisture helper
and CP128 thermal-comfort manager remain non-required and are intentionally
absent from this required list. Its EnergyPlus boundary is the unconditional
parent line-210 `ReportSurfaceHeatBalance(state)` call and the canonical body
at lines 6605-6891. That body orders shading and representative-surface
projection; opaque-or-Window, Window, movable-insulation, and two-pass opaque
report state; the guarded heat-emission summary and sizing component loads;
and advanced Zone accumulation that depends on prior initialization. It
remains `source_mapped` and required. The existing Rust
`report_surface_heat_balance_stage`, identity wrapper, bounded run-period
Surface report/trace path, and limited result-store outputs do not implement
the canonical parent nesting, complete topology, dependencies, report flags,
sizing arrays, or accumulator cadence and do not promote support or
conformance.

The canonical heat-balance inventory now also includes
`rec_keep_heat_balance`, directly after `report_surface_heat_balance` and
before `report_heat_balance`. Its EnergyPlus boundary is the unconditional
parent line-211 `RecKeepHeatBalance(state)` call and the implementation at
lines 2971-3057. The routine records Zone load and temperature extrema; shifts
the two-sample temperature and combined-load histories; and, under
`!WarmupFlag && DayOfSim == 1 && (!DoingSizing || DoPureLoadCalc)`, stores
warmup-convergence differences through the Zone-1-owned shared point counter
and optionally writes the detailed EIO header and Zone rows. It also snapshots
movable-insulation presence and unconditionally refreshes non-BSDF window-face
temperatures through `UpdateWindowFaceTempsNonBSDFWin` at lines 3303-3313.
It remains `source_mapped` and required. Existing Rust
`rec_keep_heat_balance_stage` source-order and execution-plan metadata are
scaffolding only and do not implement or promote the complete state,
lifecycle, reporting, or window-history behavior.

The canonical heat-balance inventory now also includes
`update_window_face_temps_non_bsdf_win`, immediately after
`rec_keep_heat_balance` and before `report_heat_balance`. Its EnergyPlus
boundary is the last executable `RecKeepHeatBalance` action at line 3056, the
declaration at `HeatBalanceManager.hh` line 140, and the implementation at
`HeatBalanceManager.cc` lines 3303-3313. It remains `source_mapped` and
required: Rust has no analog for the stored-order, current
`Surface.Construction`-driven copy from outside/inside history term 1 into the
non-BSDF window report
array endpoints, including its unchecked indices, partial effects, and retained
state lifecycle.

The canonical heat-balance inventory now also includes
`report_heat_balance`, immediately after
`update_window_face_temps_non_bsdf_win` and before the post-reporting EMS
calling point. Its EnergyPlus boundary is the
unconditional parent line-217 `ReportHeatBalance(state)` call and the
implementation at lines 3321-3418. Schedule reporting always runs first; the
remaining work selects the normal non-warmup output path, the requested
warmup-reporting path, the external-interface warmup fallback, or no further
reporting. The entered paths own Zone-step output, conditional node and HVAC
sizing-log updates, and, only on the normal path, tabular and utility reports.
It remains `source_mapped` and required. Existing Rust report-stage,
composite-plan, prebinding, and bounded result-store metadata are scaffolding
only and do not implement or promote the complete source branching, output
state, dependencies, failure behavior, or numerics.

The canonical heat-balance inventory now also includes
`check_warmup_convergence`, after `report_heat_balance` and the intervening
non-required EMS, EMS-trend, and plugin-history checkpoints. Its EnergyPlus
boundary is the `WarmupFlag && EndDayFlag` outer guard at
`HeatBalanceManager.cc` lines 224-226 and the required
`CheckWarmupConvergence` body at lines 3059-3226, declared at header line 136.
It remains `source_mapped` and required. Existing Rust source-order metadata
and its separate temperature-only diagnostic warmup loop do not implement or
promote the canonical four-test Zone state, load normalization, lifecycle,
diagnostics, latches, maximum-day behavior, or parent gating. CP143 separately
maps the inner `!WarmupFlag` branch and `DayOfSim`/`DayOfSimChr` resets at
lines 227-229, while CP144 reuses the non-required generic EMS row for the
following post-warmup calling point.

The required inventory now also includes `report_warmup_convergence` after
`check_warmup_convergence` and the intervening inline reset and non-required
post-warmup EMS checkpoint. Its caller is the
`!WarmupFlag && EndDayFlag && DayOfSim == 1 && !DoingSizing` guard at
`HeatBalanceManager.cc` lines 235-237; the routine is declared at header line
138 and implemented at source lines 3228-3301. It remains `source_mapped` and
required. Existing Rust warmup options, execution metadata, summaries, and
diagnostic temperature-only loop do not implement or promote the canonical
sample ownership, in-place load normalization, EIO lifecycle, Zone rows,
parent gate, outputs, or repeated-call behavior.

The canonical time-domain inventory includes `get_project_data`,
`process_schedule_input`, `process_interval_fields`,
`day_schedule_populate_from_minute_vals`, and
`schedule_detailed_get_hr_ts_val` as required routines. Their EnergyPlus
boundaries are `GetProjectData`, `ProcessScheduleInput`,
`ProcessIntervalFields`, `DaySchedule::populateFromMinuteVals`, and
`ScheduleDetailed::getHrTsVal`.
They remain `source_mapped`; these inventory additions do not promote
`calendar_time_state`, `Sched::UpdateScheduleVals`, or broad detailed-schedule
conformance.

The root marker `routine_completion_schema = "routine_completion.v1"` records
the one-time introduction of routine-level completion metadata. The PR-ticket
bootstrap exemption applies only while this marker changes from absent to
present and only to the explicit governance and documentation file allowlist;
after that transition, routine promotions use the normal Algorithm Port Ticket.

## Run States

Arbitrary runs return one of three support states:

- `run_blocked`: unsupported active semantics prevent Rust execution.
- `partial_supported_run`: unsupported or inactive items were ignored by an
  explicit rule and the result is ad-hoc only.
- `supported_compatibility_run`: all active objects and algorithms match
  declared capabilities and the run completed inside compatibility mode.
