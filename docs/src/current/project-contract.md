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

No C++ unit test calls CP200 directly or asserts one of its targets. Of 56
active full-simulation tests that reach CP199, 55 execute at least one Zone
CP200 call; `WeatherManager_SetRainFlag` has zero Zones and executes none.
Seven `SizingManager` tests reach sizing-Space and one `HeatBalanceAirManager`
test reaches simulation-Space, but their assertions are downstream.

Rust constructs only Zone run state with different three-slot histories. When
weather data exists, it seeds current and averaged humidity plus both histories
once from the first weather sample. That is not CP200's current-`OutHumRat`
four-slot per-environment Zone/Space reset and touches a different field set.
Rust has no Space record, exact 26-field boundary, or environment gate, and its
coefficient representation is computed through a different initialization
path. These adjacent histories and coefficients are not this member
implementation. CP200 remains `source_mapped` and required without new Rust
state, support, output implementation, numerical, or conformance promotion.

The following required predictor/corrector definition entry is
`zone_space_heat_balance_set_up_output_vars`, after
`zone_space_heat_balance_begin_environment_init` and before
`predict_system_loads`. Its EnergyPlus boundary is
`ZoneSpaceHeatBalanceData::setUpOutputVars(EnergyPlusData &state,
std::string_view prefix, std::string const &name)`, declared at
`ZoneTempPredictorCorrector.hh` line 215 and implemented at
`ZoneTempPredictorCorrector.cc` lines 2838-2868.

CP199's one-time Zone loop is the only production caller. It invokes CP201 for
every Zone, then for that Zone's stored Spaces only when
`doSpaceHeatBalanceSimulation` is true, before continuing with the rest of that
Zone's demand and thermostat outputs. Sizing-only Space mode does not call
CP201. Production prefixes are `Zone` and `Space`, and keys are the
corresponding stored names.

Each call registers four values in fixed order: air temperature binds `ZT`
with C/System/Average metadata; air humidity ratio binds `airHumRat` with
Units::None/System/Average; air relative humidity binds `airRelHum` with
percent/System/Average; and mean radiant temperature binds `MRT` with
C/Zone/Average. The formatted names are the prefix followed by `Air
Temperature`, `Air Humidity Ratio`, `Air Relative Humidity`, or `Mean Radiant
Temperature`. Meter metadata stays invalid or empty, multipliers stay one, the
SQL index stays -999, and Hour is only the default report-frequency argument;
a matching request can replace the concrete output entry's frequency and
schedule.

CP201 does not change those numeric members. It hands their addresses to
OutputProcessor, which initializes output state when necessary, parses
requests, creates or reuses dictionary entries, and advances setup/total
counters. A row with neither a report request nor a DataOutputs variable-list
match retains its dictionary entry but creates no `OutVarReal`. A list-only
match creates a keyed dummy pointer, link, and report identifier without a
dictionary-sink write; a report request additionally marks the entry for
reporting and emits the applicable rows. Dictionary reuse is case-insensitive
name-plus-units only. The helper has no re-entry guard, so repeated calls can
duplicate whichever counter, dummy or requested entry, link, identifier, and
sink effects the same match state selects.

There is no local identity, membership, prefix, key, shape, lifecycle, status,
catch, cleanup, or rollback. A formatting, initialization, request-input,
allocation, or report-write non-return can retain an arbitrary completed
prefix, while CP199 does not clear its one-time latch until its entire output
phase returns. Predictor/corrector reset alone can invalidate stored member
pointers without clearing OutputProcessor; OutputProcessor reset alone does
not reconstruct the owner records or parent latch. Clean replay therefore
requires coordinated ownership reset, and already emitted external rows are
not transactional.

No unit test calls CP201 directly or positively asserts a registered name,
unit, key, timestep, store, or field binding. Four direct CP199 fixtures and 55
active full-simulation tests execute at least one Zone setup indirectly. One
additional no-Zone `WeatherManager_SetRainFlag` simulation reaches CP199 but
executes no CP201 call. Only
`HeatBalanceAirManager_GetMixingAndCrossMixing` reaches simulation-Space setup,
for three Spaces; seven sizing-Space tests do not enter CP201. Rust's runtime
registry exposes `Zone Mean Air Temperature`, and its heat-balance ResultStore
also carries adjacent `Zone Mean Air Temperature` and `Zone Mean Air Humidity
Ratio` series. Those are different identities and bindings. Rust has no exact
CP201 Zone/Space output set, relative-humidity or MRT registration, Space
heat-balance state, System-versus-Zone timestep distinction, or this pointer
and lifecycle contract. The IdealLoads CLI's exact `Zone Air Temperature` and
`Zone Air Humidity Ratio` names are ESO input identities; the humidity name
also labels a diagnostic comparison, but neither is a Rust production output.
CP201 remains `source_mapped` and required without new Rust
state, support, output implementation, numerical, or conformance promotion.

The next required predictor/corrector definition entry is
`predict_system_loads`, after
`zone_space_heat_balance_set_up_output_vars` and before
`zone_space_heat_balance_predict_system_load`. Its source boundary is
`PredictSystemLoads(EnergyPlusData &state, bool ShortenTimeStepSys,
bool UseZoneTimeStepHistory, Real64 PriorTimeStep)`, declared at
`ZoneTempPredictorCorrector.hh` lines 276-280 and implemented at
`ZoneTempPredictorCorrector.cc` lines 2870-3145.

The only production call expression is CP195 `ManageZoneAirUpdates` line 227
under `PredictStep`, after its optional CP196 input and unconditional CP199
initialization return. `HVACManager` enters that selector before its initial
`SimHVAC` and again inside shortened system-timestep work; the latter can run
once with `ShortenTimeStepSys = true` and later substeps with it false.
`SimulationManager::Resimulate` provides the third entrance with false
shortening, the current history selector, and zero prior timestep. CP202 itself
does not receive or alter caller-owned `ZoneTempChange`.

CP202 first walks staged controls in stored order. It samples the heating and
cooling base schedules on every call and uses Zone `MAT`, or `XMPT` during a
shortened call, as the comparison temperature. Heating not below cooling
increments persistent staged-error state, emits a first warning or later
recurring warning, and replaces heating with cooling minus 0.1 C. Cooling
selects the last qualifying negative stage; heating selects the last
qualifying positive stage; deadband writes stage zero. The resulting
thermostat setpoint uses the applicable half throttling range, except that the
source heating decision compares against half `CoolThroRange` while the value
written uses half `HeatThroRange`. When `doSpaceHeatBalance` is true, the Zone
stage is copied to every stored Space demand membership.

A second, `NumOnOffCtrZone`-gated pass scans all temperature controls and acts
only on positive `DeltaTCutSet`. Nonshortened calls save current heat/cool
last-mode memory; shortened calls restore it. Both off flags are then cleared.
ThirdOrder uses `MAT` or shortened `XMPT`, while every other solution algorithm
uses `T1` regardless of shortening. SingleHeat, SingleCool, and the two
independent DualHeatCool halves apply the literal strict comparisons around a
fixed 0.02 C tolerance, followed by last-mode overrides. `SingleHeatCool`,
Uncontrolled, and other switch values make no setpoint write in this phase. If
the revised DualHeatCool lower setpoint is greater than or equal to its upper
setpoint, CP202 emits severe, timestamp, and setpoint diagnostics and fatals
before any Zone load child.

The main loop always calls the still-source-mapped CP203
`ZoneSpaceHeatBalanceData::predictSystemLoad` once for every Zone, forwarding
all three timestep arguments and the Zone identity. For each stored Space
membership, active Space heat balance calls the same member with both
identities. When Space heat balance is inactive, only a shortened call copies
the already processed Zone `MAT` and `airHumRat` to the Space; a nonshortened
call leaves the Space unchanged. `UseZoneTimeStepHistory` and `PriorTimeStep`
have no other local use. CP202 therefore dispatches, but does not itself claim,
CP203's temperature update, coefficient, sensible-load, and moisture-load
equations.

After all children return, a final positive-cutout pass sets cooling last-mode
memory only when `CoolOffFlag` and `TotalOutputRequired >= 0`, and heating
last-mode memory only when `HeatOffFlag` and `TotalOutputRequired <= 0`. Zero
can set both if both off flags are true; a NaN load clears both. Schedule
pointers, counts, actual Zone indices, stage arrays, Zone/Space membership,
numeric ranges, and child state are trusted. There is no local latch, status,
catch, cleanup, or rollback. A dual fatal or child non-return preserves the
completed staged, diagnostic, setpoint, save/restore, demand, child, or
Space-copy prefix and suppresses the unreached tail. Same-state retry resamples
schedules, advances staged recurrence, and repeats child work. Clean replay
requires coordinated predictor/corrector, ZoneControls, HeatBalFanSys,
ZoneEnergyDemand, HeatBalance topology, HVAC history, diagnostics, and CP203
dependency reset.

Two C++ fixtures call CP202 directly 16 times and make 24 setpoint assertions.
They cover selected Euler nonshortened and ThirdOrder normal/shortened
SingleHeat, SingleCool, SingleHeatCool, and DualHeatCool cases. Both leave
global `NumOfZones` at zero, always pass false history selection and 0.01 prior
time, and assert no staged state, Zone/Space child, forwarding, off/final-mode
state, fatal boundary, partial failure, retry, or reset. Of 56 active
full-simulation tests that reach CP202, 55 execute a Zone child. One
simulation-Space fixture and seven sizing-Space fixtures enable Space children,
but only assert downstream mixing or sizing results. None supplies a staged
thermostat or positive cutout delta.

Rust parses and retains the cutout delta but never consumes it at runtime.
`predict_system_loads_compat` and `predict_step_source_order_path` are identity
closures around a bounded Zone-only temperature/history update; the only
related wrapper test calls the latter path directly and checks only a string
call order. Rust has no staged-control state,
`StageNum`, source thermostat setpoint arena, off/last-mode memory, sensible
`ZoneSysEnergyDemand::TotalOutputRequired` field, Space heat-balance record, or
CP203 dispatch.
Its IdealLoads `ZoneSysEnergyDemand` snapshot is fed by EnergyPlus ESO oracle
loads and is not a production predictor. CP202 remains `source_mapped` and
required without new Rust target, state, support, output, numerical, or
conformance promotion.

The following required predictor/corrector definition entry is
`zone_space_heat_balance_predict_system_load`, after `predict_system_loads`
and before `calc_zone_air_temp_set_points`. Its source boundary is
`ZoneSpaceHeatBalanceData::predictSystemLoad(EnergyPlusData &state, bool
shortenTimeStepSys, bool useZoneTimeStepHistory, Real64 priorTimeStep, int
zoneNum, int spaceNum)`, declared at `ZoneTempPredictorCorrector.hh` lines
217-222 and implemented at `ZoneTempPredictorCorrector.cc` lines 3146-3257.

The only production call expressions are CP202 line 3116 for every Zone and
lines 3119-3120 for every stored Space while `doSpaceHeatBalance` is true.
After the positive-Zone debug assertion, CP203 first delegates all three
timestep/history arguments and both identities to `updateTemperatures`. It then
chooses Space volume only for a positive Space identity, otherwise Zone volume,
and writes `AirPowerCap` from that volume, the parent Zone's sensible
capacitance multiplier, current outdoor pressure, this record's `MAT` and
`airHumRat`, source psychrometric density and heat capacity, and unguarded
system timestep seconds.

After `calcZoneOrSpaceSums(state, false, zoneNum, spaceNum)` returns, CP203
handles one hybrid-model exception only for the Zone record: it first clears
`SumIntGainExceptPeople`, then replaces it through
`SumAllInternalConvectionGainsExceptPeople`. It writes `TempDepCoef` from
`SumHA + SumMCp`, `TempIndCoef` from internal, surface, reference-air,
air-exchange, and `SysDepZoneLoadsLagged` terms, and `TempHistoryTerm` from
`AirPowerCap * (3 * ZTM[0] - 1.5 * ZTM[1] + ZTM[2] / 3)`. The initial
`tempDepLoad` and `tempIndLoad` receive the corresponding ThirdOrder forms.
Predictor `SumSysMCp` and `SumSysMCpT` remain excluded.

A used AirflowNetwork Zone under the state-wide nonmixing flag first calls
`LoadPredictionRoomAirModelAFN` for its control node, then replaces the direct
coefficients, capacity, history term, and two load scalars from that node's
sums and air properties. If the node has assigned HVAC,
`HVAC(1).SupplyFraction` becomes the local `RAFNFrac` passed to both final
demand children; otherwise it remains zero. This branch is keyed by the parent
Zone and is not suppressed merely because the current record is a Space.

CP203 then writes shared `ShortenTimeStepSysRoomAir = false`. ThirdOrder leaves
this record's `T1` and `W1` untouched. For every other solution algorithm, a
shortened system step below the Zone timestep chooses `TM2`/`WM2` when shared
`PreviousTimeStep` is below the Zone timestep, otherwise `TMX`/`WMX`, copies
the corresponding T2 or TX histories to every AirflowNetwork node, and sets the
shared flag true. The other path copies current `ZT` and `airHumRat` plus
current AFN-node values while leaving the flag false. These later AFN-node
copies test only the Zone model type, not the earlier nonmixing and `IsUsed`
gates. Non-ThirdOrder then replaces both load scalars with the plain
coefficients. The direct choice reads shared `PreviousTimeStep`; the argument
`priorTimeStep` is consumed only by the earlier temperature-update child.

The final ordered calls are `calcPredictedSystemLoad` followed by
`calcPredictedHumidityRatio`, both with `RAFNFrac`, `zoneNum`, and `spaceNum`.
CP203 owns their order and inputs but not their setpoint, sensible-load,
humidity-control, moisture-load, diagnostic, or reporting equations. The same
dependency boundary applies to temperature updating, sum assembly,
psychrometrics, hybrid gain collection, and AFN load prediction.

Apart from its debug assertion, counts, upper bounds, Space membership,
volumes, timestep values, histories, node identities, supply fraction, and all
numeric inputs are trusted. CP203 has no local diagnostic, latch, status,
catch, cleanup, or rollback. A child failure can retain any completed update,
capacity, hybrid clear, coefficient, AFN, shared-flag, record-history,
node-history, or demand prefix. A humidity-child failure occurs after the
sensible child has already returned. Same-state retry repeats every child and
direct write; a clean replay requires coordinated reset of the Zone/Space
record, HVAC timestep and global room-air-shortening state, HeatBalFanSys and
nodes, HeatBalance topology, RoomAir/AFN, HybridModel/internal gains,
sensible/moisture demand, diagnostics, and every child owner.

No C++ test calls CP203 directly. CP202's two focused fixtures make 16 wrapper
calls but retain zero global Zones, so none reaches this member. Of 56 active
full-simulation tests that reach CP202, 55 execute at least one Zone call and
eight enable Space calls; their assertions are downstream and do not isolate
CP203's coefficient/history sequence, hybrid/AFN path, ordered children,
failure prefix, retry, or reset. No active full-simulation block exercises the
RoomAir AFN override.

Rust's `predict_system_loads_compat` remains an identity wrapper around a
different Zone-only temperature/history update. Existing coefficient,
moist-air-capacitance, analytical/ThirdOrder, interpolation, and bounded
no-outdoor-air ThirdOrder moisture helpers cover adjacent formulas under
different guards and ownership. They do not implement CP203's current-state
Zone/Space capacity transaction, lagged/hybrid/AFN overrides, shared
`ShortenTimeStepSysRoomAir`, exact `T1`/`W1` and AFN-node histories, or ordered
sensible-then-moisture demand dispatch. CP203 therefore remains required
`source_mapped` and adds no Rust target, state, support, output, numerical, or
conformance claim. The inventory becomes 32 algorithms and 211 routines,
split 58 `state_mapped` plus 153 `source_mapped`, with 88 required; the
heat-balance project list becomes 57.

The following required predictor/corrector definition entry is
`calc_zone_air_temp_set_points`, after
`zone_space_heat_balance_predict_system_load` and before
`zone_space_heat_balance_calc_predicted_humidity_ratio`. Its source boundary
is
`CalcZoneAirTempSetPoints(EnergyPlusData &state)`, declared at
`ZoneTempPredictorCorrector.hh` line 282 and implemented at
`ZoneTempPredictorCorrector.cc` lines 3259-3460.

CP195 line 224 is the only direct production call expression and selects it
only for `GetZoneSetPoints`. The ordinary `HVACManager` timestep entrance and
`SimulationManager::Resimulate` both reach CP204 after CP196 input and CP199
initialization. The routine has no Space, timestep, history, or
`ZoneTempChange` argument.

Every call resets the complete `TempControlType` array to Uncontrolled,
allocates the occupied heating/cooling arrays only if absent, and fills their
existing entries with zero and 100 respectively. It does not resize those
arrays or globally clear `TempControlTypeRpt` or thermostat setpoint members.
The local `DeltaT` is assigned zero but never read. Each stored ordinary
temperature-control record then samples and casts its control-type schedule,
writes the enum and integer report for the trusted actual Zone, and dispatches
its setpoint family.

Uncontrolled preserves prior setpoint fields. SingleHeat alone checks
`isUsed`, snapshots the raw heating value, applies operative control, and
writes generic plus low setpoints while preserving high. SingleCool snapshots
the raw cooling value, optionally applies and snapshots adaptive comfort,
applies operative control, writes generic plus high, and then invokes
temperature-and-humidity overcool while preserving low. SingleHeatCool applies
adaptive and operative adjustment to its heat schedule and writes both bounds.
Its optimum-start branch samples the `SingleHeat` day array at
`(ceil(OccStartTime) + 1) * TimeStepsInHour` into generic setpoint, then an
independent flag may copy it to both bounds. DualHeatCool processes cooling
and adaptive/operative adjustment before heating and operative adjustment,
optionally replaces both bounds from globally reset occupied day-array values,
then invokes overcool. It does not directly refresh generic setpoint.

An invalid control value emits Severe and continues. After every ordinary
record, the enabled thermostat-fault scan stops at the first name match even
when its availability is off; an active match subtracts severity times offset
from generic, low, and high values. This can repeatedly offset fields not
rewritten by the selected branch. After all ordinary records, comfort
calculation can overwrite their results, and the unconditional EMS override
has final source precedence. All helper formulas, diagnostics, latches, and
actuator state remain dependency behavior rather than CP204 implementation.

CP204 trusts counts, array shapes, Zone identities, pointers, casts, schedule
and setpoint-family consistency, optimum-start topology/indexes, fault state,
and numeric values. It has no local latch, status, catch, cleanup, rollback,
or allocation reconciliation. Any non-return retains the completed reset,
allocation, record, fault, comfort, or EMS prefix. Same-state retry resamples
schedules and repeats helpers and diagnostics; it is not generally idempotent
because several branch fields remain stale across fault subtraction. Clean
replay requires coordinated reset of ZoneControls, HeatBalFanSys
control/report/setpoint state, schedules, Availability optimum-start,
FaultsManager, environment/weather and Zone MRT/RH inputs, thermal comfort,
EMS, diagnostics, and child owners.

Four C++ fixtures contain 21 direct calls and 33 thermostat-field assertions:
one/two in the optimum-start fixture, four/seven in the reporting fixture, and
eight/twelve in each of two CP202-composed cutout fixtures. Only the first two
assertions follow CP204 without a later load-prediction child. Helper-only
tests exercise adaptive adjustment and EMS override, while operative
adjustment, overcool, and comfort calculation have no direct helper call.
Fifty-six of 57 active full simulations reach CP204, 38 contain thermostat
declarations by static evidence, and none directly asserts CP204 output or
contains positive comfort, operative-temperature, overcool, optimum-start, or
thermostat-fault input.

Rust's `calc_zone_air_temp_set_points_compat` is an untested identity closure
around an empty body. Its sole live call is incorrectly nested in a hard-coded
Predict scaffold rather than the source `GetZoneSetPoints` selector. Rust
retains only direct-Zone DualSetpoint thermostat input. A separate IdealLoads
diagnostic helper ignores the control-type schedule and repeats the first
control's constant heat/cool values; it is not a heat-balance evaluator. Rust
has no CP204 setpoint/control state, Single branches, schedule selection,
operative/adaptive or thermal comfort, overcool, optimum start, fault, EMS
override, partial-effect, or lifecycle implementation.

CP204 remains required `source_mapped` and adds no Rust target, state, test,
support, output, numerical, or conformance claim. The inventory becomes 32
algorithms and 212 routines, split 58 `state_mapped` plus 154
`source_mapped`, with 89 required; the heat-balance project list becomes 58.

The following required predictor/corrector definition entry is
`zone_space_heat_balance_calc_predicted_humidity_ratio`, after
`calc_zone_air_temp_set_points` and before `correct_zone_air_temps`. Its
source boundary is
`ZoneSpaceHeatBalanceData::calcPredictedHumidityRatio(EnergyPlusData &state,
Real64 RAFNFrac, int zoneNum, int spaceNum = 0)`, declared at
`ZoneTempPredictorCorrector.hh` line 243 and implemented at
`ZoneTempPredictorCorrector.cc` lines 3462-3815.

Its only production call is CP203 line 3256, after sensible-load prediction.
CP202/CP203 therefore run it for each Zone and each active Space. The routine
selects Zone-keyed humidifying and dehumidifying RH schedules, then applies
the two EMS overrides. Outside warmup, sizing, and kickoff it stops at the
first matching humidistat fault. Independent faults subtract their available
severity-scaled offset and clamp RH; thermostat-dependent faults locate the
referenced thermostat fault and, only for a nonzero available offset, transform
both RH values through humidity ratio at this record's offset `MAT` before
clamping. A missing referenced thermostat fatals. Reversed RH values warn
through the control-owned recurrence index and are collapsed to the
dehumidifying value.

Without a humidistat, latent sizing examines only the first controlled Zone
equipment configuration, does not match it to `zoneNum`, and uses its matching
`Sizing:Zone` input or the first sizing input as fallback. When latent sizing
is enabled there, that one selection can control every otherwise uncontrolled
Zone and Space invocation.

Controlled calculation combines record latent gain with Zone radiant and pool
latent gains, forms moisture coefficients from the active ordinary-airflow or
AirflowNetwork path, and uses Space volume only for a positive `spaceNum`.
A parent Zone RoomAir AirflowNetwork model replaces those coefficients from
its control node without CP203's nonmixing or node-use guards, including for a
Space call. ThirdOrder uses three humidity-history values; Analytical uses its
exact-zero or exponential branch; Euler uses the one-step balance. A positive
`RAFNFrac` divides each load. Exact-equal setpoints select humidifying load;
otherwise the signed humidifying/dehumidifying matrix selects humidification,
dehumidification, or zero and fatals on every remaining combination.

Controlled output selects Zone or Space moisture demand and delegates to
`reportMoistLoadsZoneMultiplier`, which stores raw predicted values, Zone-
multiplied public demand, and conditionally sequenced equipment demand.
Uncontrolled output zeros only the three public demand fields, leaving raw,
sequenced, remaining, unadjusted, and report state stale. There is no local
validation, latch, status, catch, cleanup, or rollback beyond one debug
assertion. A diagnostic, psychrometric, allocation, or report-child failure
can preserve warnings, recurrence state, or a partial demand prefix; Zone
demand can commit before a later Space failure. Retry resamples schedules and
faults and repeats all writes.

No C++ test calls CP205 directly. One report-helper fixture covers only Zone
raw and multiplied values. Fifty-five active full simulations execute Zone
CP205 and eight also execute Space CP205, but none supplies a humidistat,
humidistat fault/EMS override, or RoomAir AFN. Only one ThirdOrder
latent-sizing case enters controlled equations, with downstream assertions;
six Analytical cases remain uncontrolled and no Euler case is reached.

Rust has no heat-balance CP205 wrapper, call site, Zone/Space moisture-demand
owner, or runtime mutation. The separate
`calc_no_oa_third_order_moisture_demand_compat` and fixed-one-step IdealLoads
Humidistat loop cover a guarded Zone-only, no-outdoor-air, `A = 0` ThirdOrder
subset. They reject invalid input, always clamp RH, and return only multiplied
loads. They omit the source schedules, EMS, faults, sizing selection,
radiant/pool and airflow terms, AFN/RoomAir/RAFN, Space, Analytical/Euler,
diagnostics and partial effects, raw predicted values, uncontrolled writes,
and sequenced demand.

CP205 remains required `source_mapped` and adds no Rust target, state, test,
support, output, numerical, or conformance claim. The inventory becomes 32
algorithms and 213 routines, split 58 `state_mapped` plus 155
`source_mapped`, with 90 required; the heat-balance project list becomes 59.

The following required predictor/corrector definition entry is
`correct_zone_air_temps`, after
`zone_space_heat_balance_calc_predicted_humidity_ratio` and before
`zone_space_heat_balance_correct_air_temp`. Its source boundary is
`correctZoneAirTemps(EnergyPlusData &state,
bool useZoneTimeStepHistory)`, declared at
`ZoneTempPredictorCorrector.hh` lines 289-291 and implemented at
`ZoneTempPredictorCorrector.cc` lines 3817-3861.

Its only production direct call is CP195 line 230. The `CorrectStep` arm
assigns the returned maximum to caller-owned `ZoneTempChange` only after
CP206 returns. Initial HVAC Get/Predict/simulation precedes Correct. The caller
selects adaptive downstepping only when that first maximum exceeds
`MaxZoneTempDiff` and `KickOffSimulation` is false; otherwise it uses one
system step and Zone-timestep history. Fine steps repeat Predict, HVAC
simulation, Correct, contaminant correction, and system-history push without
reselecting their count from later returns. Demand resimulation does not call
Correct.

CP206 starts its maximum at zero. For each Zone it calls the Zone
`correctAirTemp` child first and saves its result, then visits every stored
Space. Simulation Space HB outside sizing calls the Space child and folds its
delta immediately. Every other case optionally copies controlled sizing Zone
node temperature, humidity ratio, and enthalpy to the Space node, then always
copies Zone `ZT`, `ZTM`, `MAT`, `airHumRat`, and `airRelHum` to the
Space record. The Zone delta is folded only after its Spaces.

The wrapper then always calls `CalcZoneComponentLoadSums` for the Zone and,
when simulation Space HB is enabled, calls it for every Space even during
sizing. The wrapper owns only the optional three node and five Space-record
writes; the correction and report equations remain child dependencies.
Starting from zero and passing each child value as the second `std::max`
argument ignores negative and NaN candidates and can retain positive infinity.
A nonpositive Zone count returns zero.

CP206 has no local assertion, validation, diagnostic, status, latch, catch,
cleanup, transaction, or rollback. Zone or Space child failure retains its
completed prefix and suppresses the remaining traversal; mirror and report
failure can retain direct-write or report prefixes. A failure loses the local
maximum and leaves the caller's old `ZoneTempChange` because CP195's
assignment did not complete. Retry restarts from Zone one and repeats all
children, copies, and reports. Clean replay requires coordinated
predictor/corrector, Zone/Space heat balance, nodes, topology, RoomAir, HVAC,
report, diagnostics, and child reset.

One HybridModel fixture makes five direct calls with Zone history true, one
Zone, one stored Space, and false Space-HB flags. It reaches Zone correction
and Space record mirroring, but its five assertions inspect only child-owned
hybrid effects. Of 57 active full simulations, one expected EMS fatal stops
before CP206, one zero-Zone case reaches the zero return, and 55 correct at
least one Zone. One Analytical configuration reaches active Space correction
and reporting for two Zones and three Spaces. Seven sizing configurations
reach controlled node and record mirroring for one Zone and three Spaces each.
Their assertions are downstream; no test isolates the maximum, fold order,
adaptive selection, component-report dispatch, failure, retry, or reset.

Rust's `correct_zone_air_temps_compat` is an identity alias with one live call
and no direct test. Its unit-returning closure performs an all-Zone temperature
pass, then an all-Zone humidity pass, then project-specific per-Zone adaptive
correction or local history synchronization. Rust has no Space HB or Space
node owner, AirReportVars, source component-report child, functional history
selector, or returned global maximum. Its independent per-Zone substep choice
does not reproduce the source global maximum or full Predict-HVAC-Correct
retry. Adjacent temperature formulas and bounded `1ZoneUncontrolled` output
evidence belong to the delegated child and case-specific results, not CP206
wrapper parity; Space execution and sizing remain run-blocked.

CP206 remains required `source_mapped` and adds no Rust target, state, test,
support, output, numerical, or conformance claim. The inventory becomes 32
algorithms and 214 routines, split 58 `state_mapped` plus 156
`source_mapped`, with 91 required; the heat-balance project list becomes 60.

The following required predictor/corrector definition entry is
`zone_space_heat_balance_correct_air_temp`, after
`correct_zone_air_temps` and before `update_final_surface_heat_balance`. Its
source boundary is
`ZoneSpaceHeatBalanceData::correctAirTemp(EnergyPlusData &state,
bool useZoneTimeStepHistory, int zoneNum, int spaceNum = 0)`, declared at
`ZoneTempPredictorCorrector.hh` lines 236-239 and implemented at
`ZoneTempPredictorCorrector.cc` lines 3863-4165.

Its only production expressions are CP206 lines 3825 and 3831. The first calls
every Zone with default zero Space identity; the second calls positive active
Spaces only outside sizing. Each call selects complete Zone- or system-step
temperature and humidity histories, computes `AirPowerCap` from positive Space
or fallback Zone volume, the parent sensible-capacitance multiplier,
pre-correction `MAT`/humidity, pressure, and unguarded system seconds, calls
RoomAir only for exact Zone identity, then runs correction-step sum assembly.
`FlagHybridModel_PC` can write the parent Zone's except-People aggregate into
a Space record because there is no Space gate.

A positive selected system node, not `Zone.IsControlled`, chooses the
controlled coefficients. They include system flow, divided non-air response,
lagged system load, and optional Zone-indexed AFN and duct additions.
Uncontrolled coefficients omit the system, non-air, and lagged terms but retain
those optional additions. Both paths use unguarded ThirdOrder, exact-zero
Analytical, or Euler equations; an unknown enum retains stale `ZT` and
continues.

Controlled mixed paths update the selected node, Zone-only thermostat, and
Zone-shared load correction. Displacement, UFAD, UserDefined, and one-node
displacement paths can instead derive a [-3,3] correction factor from supply
and existing-node temperatures; AirflowNetwork can replace `ZT` from the
parent control node. An active Space uses its own node with parent RoomAir
flags and can overwrite the Zone-shared correction factor without writing the
thermostat. The controlled sensible load uses the selected node, Zone
multiplier, non-air response, and lagged load; uncontrolled load stays zero.

An enabled exact-Zone hybrid inverse runs after node and sensible-load
calculation and can replace `ZT` with measured temperature. CP207 then writes
`MAT`, reports Zone or positive-Space sensible demand, calls
`correctHumRat`, commits `airHumRatTemp`, computes relative humidity, and only
then returns a temperature delta. ThirdOrder compares against selected
three-step history; Analytical/Euler compare against `T1`. Only unmixed
three-node displacement or UFAD Zone calls use their two RoomAir deltas. An
unknown enum returns zero. Ordered `std::max` calls suppress a NaN outer
candidate and retain positive infinity.

The only local validation is a debug positive-Zone assertion. A malformed
negative Space identity mixes Zone volume/node/demand selection with skipped
exact-Zone RoomAir, thermostat, hybrid, AFN override, and nonmixed-delta
behavior, although production never supplies it. CP207 has no local
upper-bound, topology, enum, denominator, timestep, multiplier, or finite
validation, diagnostic, latch, status, catch, cleanup, transaction, or
rollback. Failure can retain any ordered prefix through histories, capacity,
sums, coefficients, node/control, hybrid, `MAT`, sensible demand, humidity, or
RH; a non-return prevents CP206 from folding the delta. Retry recomputes from
already-mutated shared state and clean replay requires coordinated record,
node, RoomAir, AFN, duct, hybrid, demand, HVAC/environment, diagnostic, and
child reset.

No C++ test calls CP207 directly. The HybridModel fixture reaches its
controlled fully mixed ThirdOrder Zone path five times with history true and
asserts only hybrid child effects. Of 57 active full simulations, one fatal
stops before CP207 and one has zero Zones; the other 55 reach a static
one-pass inventory of 81 Zone records, split 55 controlled and 26
uncontrolled. Forty-nine configurations use ThirdOrder, six use Analytical,
and none uses Euler. One Analytical configuration reaches two uncontrolled
Zones and three uncontrolled Spaces; AFN-distribution and duct configurations
reach their additions without asserting them. No full simulation declares a
RoomAir or HybridModel input, and no test isolates solver output, false
history, node/control writes, sensible demand, RH, returned delta, failure,
retry, or reset.

Rust has no named CP207 wrapper or direct test. Its all-Zone surface-driven
temperature helper has four non-test calls, and its adaptive single-Zone helper
has one; neither has a direct test. The live closure performs all Zones'
temperature work, then all Zones' separate humidity approximation, then
project-specific adaptive/history work. Limited coefficient, ThirdOrder, and
Analytical helpers have focused formula tests, but Rust has no controlled-node
decision, Space state, source sum child, non-air/lagged/AFN/duct inputs, nodes,
RoomAir, thermostat/correction factor, sensible demand, hybrid inverse,
distinct `ZT`/`MAT`/`T1`, relative humidity, Euler path, or returned delta.
Its capacity and solver guards also differ from source raw division and exact
tests. Existing official MAT evidence remains bounded case evidence.

CP207 remains required `source_mapped` and adds no Rust target, state, test,
support, output, numerical, or conformance claim. The inventory becomes 32
algorithms and 215 routines, split 58 `state_mapped` plus 157
`source_mapped`, with 92 required; the heat-balance project list becomes 61.

The following required predictor/corrector definition entry is
`push_zone_timestep_histories`, after
`zone_space_heat_balance_correct_air_temp` and before
`update_final_surface_heat_balance`. Its source boundary is
`PushZoneTimestepHistories(EnergyPlusData &state)`, declared at
`ZoneTempPredictorCorrector.hh` line 293 and implemented at
`ZoneTempPredictorCorrector.cc` lines 4167-4185.

The only built-in direct call is CP195's PushZone selector arm. Its sole
built-in request is the end of `HVACManager::ManageHVAC`, after all system
steps and comfort-average snapshots and before contaminant histories, the
last-system-step-count commit, and demand-manager update. The dispatcher
common input/init prefix runs first; the CP208 arm ignores timestep and history
arguments and preserves `ZoneTempChange`. An external HVAC callback bypasses
this canonical request.

CP208 visits Zones in ascending identity. It calls each Zone record child
first, then, only while the current aggregate `doSpaceHeatBalance` flag is
true, visits that Zone's stored Space memberships in order and calls the same
child with both identities. It does not scan all Spaces independently, sort,
deduplicate, or validate counts, arenas, bounds, membership, or identity
consistency. A nonpositive Zone count is a no-op. Sizing and normal simulation
supply the aggregate flag from their separate Space-HB settings.

The wrapper writes no history itself; the following CP209 child owns all
four-slot record, psychrometric, non-ThirdOrder, RoomAir, and AFN mutation. A
child non-return retains earlier children and its own prefix while suppressing
later traversal. Retry starts again at Zone one and shifts already completed
records a second time. CP208 has no diagnostic, status, latch, catch, cleanup,
transaction, rollback, or completion count.

No C++ test calls CP208, its child, the dispatcher, or `ManageHVAC` directly.
Of 57 active `ManageSimulation` call expressions, one expected EMS fatal
stops before CP208; 56 reach the wrapper, including one zero-Zone case, and
the remaining 55 collectively span a static census of 81 Zone identities.
Eight configurations collectively enable a static census of 24 Space
identities: one simulation-Space case and seven sizing-Space cases. No
assertion isolates history movement, Zone/Space order, failure, retry, or
reset.

Rust has identity source-order and compat wrappers and one live compat call,
but production never selects PushZone through its selector-ignoring
dispatcher. The live closure runs at Predict entry, combines a three-slot
Zone-only temperature/humidity shift with predictor work, and selects average
versus current input using a project-specific adaptive flag. Rust lacks the
source end-of-HVAC placement, Space arena and membership traversal, fourth
slot, and CP209 auxiliary record, RoomAir, and AFN state. One generic wrapper
test checks only its call label and order in a larger vector; a
uniform-temperature integration assertion cannot distinguish the shift.

CP208 remains required `source_mapped` and adds no Rust target, mapped state,
support, output, numerical, or conformance claim. The inventory becomes 32
algorithms and 216 routines, split 58 `state_mapped` plus 158
`source_mapped`, with 93 required; the heat-balance project list becomes 62.

The following required predictor/corrector definition entry is
`zone_space_heat_balance_push_zone_timestep_history`, after
`push_zone_timestep_histories` and before
`update_final_surface_heat_balance`. Its source boundary is
`ZoneSpaceHeatBalanceData::pushZoneTimestepHistory(EnergyPlusData &state,
int zoneNum, int spaceNum = 0)`, declared at
`ZoneTempPredictorCorrector.hh` line 245 and implemented at
`ZoneTempPredictorCorrector.cc` lines 4187-4275.

The only production expressions are CP208's Zone and Space child calls. After
a positive-Zone assertion and unconditional parent AirModel alias, CP209
interleaves descending four-slot temperature and humidity shifts, inserts the
Zone-timestep averages, saves current `ZT` to `XMPT`, commits temporary
humidity to current and previous-system state, and computes percent relative
humidity from current temperature, committed humidity, and barometric
pressure. The psychrometric helper can floor only its local humidity and clamp
an out-of-range result; CP209 does not repair the stored humidity or add a
finite/range guard.

Exact Zone identity alone advances three-node displacement or UFAD Floor,
occupied, and mixed four-slot temperatures, or every RoomAir AFN node's
temperature and humidity histories. Every solution enum except exact
ThirdOrder then advances record `TM2/TMX` and `WM2/WMX` from the averages.
For exact Zones it also advances applicable stratified M2/MX or AFN T2/TX
pairs. Spaces receive the common and non-ThirdOrder record writes but no shared
RoomAir work. Invalid or unmatched solution enums enter the non-ThirdOrder
branch.

CP209 has no upper-bound, Space identity, membership, record-kind, allocation,
enum, topology, pressure, or finite validation and no local diagnostic beyond
the psychrometric dependency, status, catch, cleanup, transaction, or rollback.
A late non-return can preserve common record and RoomAir prefixes. Retry shifts
already shifted histories again; the later revert helper is not a full inverse,
so clean replay requires coordinated record, RoomAir/AFN, psychrometric,
topology, environment, and caller reset.

No C++ test directly calls CP209 or asserts a destination. Fifty-five
completing nonzero-Zone configurations collectively span 105 static record
identities: 95 ThirdOrder and ten Analytical, with no Euler. The common path
reaches all 105, the non-ThirdOrder scalar branch reaches ten, and special
RoomAir/AFN branches reach zero because all 81 Zones are Mixing and the 24
Spaces skip shared RoomAir. Tracked official hourly outputs remain indirect
downstream sensitivity, not record-commit proof.

Rust has no singular record helper or direct test. Its nearest live code shifts
only three Zone temperature and humidity slots at Predict entry, conditionally
selects average versus current input, and has no Space arena, fourth slot,
`XMPT`, temporary/previous-system humidity scalars, stored RH,
non-ThirdOrder scalar state, RoomAir, or AFN histories. A uniform history
assertion cannot identify this shift; separate system-step, psychrometric
formula, and IdealLoads history tests do not compose CP209.

CP209 remains required `source_mapped` and adds no Rust target, mapped state,
support, output, numerical, or conformance claim. The inventory becomes 32
algorithms and 217 routines, split 58 `state_mapped` plus 159
`source_mapped`, with 94 required; the heat-balance project list becomes 63.

The following required predictor/corrector definition entry is
`push_system_timestep_histories`, after
`zone_space_heat_balance_push_zone_timestep_history` and before
`update_final_surface_heat_balance`. Its source boundary is
`PushSystemTimestepHistories(EnergyPlusData &state)`, declared at
`ZoneTempPredictorCorrector.hh` line 295 and implemented at
`ZoneTempPredictorCorrector.cc` lines 4277-4295.

Its only direct source call is the CP195 dispatcher arm, and its only built-in
request is `HVACManager` lines 388-393. The request occurs after each selected
global fine-step Predict/HVAC/Correct sequence under the strict
`TimeStepSys < TimeStepZone` gate and before contaminant history,
`PreviousTimeStep`, and current-step average commits. Equal-step and kickoff
paths skip it. The dispatcher prefix still runs, but CP210 ignores its
shortening, history-selection, and prior-step arguments and preserves
`ZoneTempChange`.

CP210 visits Zones in ascending order, calls the Zone record child first, then
visits stored Spaces in container order only under the current aggregate
`doSpaceHeatBalance` flag. It has no independent all-Space scan, sorting,
deduplication, membership/count/arena validation, or direct state write. CP211
owns all downstepped record and RoomAir/AFN history mutations.

An abnormal child non-return preserves completed and failing prefixes, blocks
later traversal and the parent's following contaminant/time/average work, and
makes same-state retry destructive. Zone-timestep revert does not restore the
downstepped histories, so clean replay requires coordinated record,
RoomAir/AFN, topology, aggregate-flag, HVAC-clock, dispatcher, and dependency
reset.

No C++ test directly calls CP210 or CP211, and no assertion or tracked oracle
proves the numerical adaptive gate. Fifty-five completing nonzero-Zone
configurations provide only potential topology of 81 Zones and 24 eligible
Spaces. The tracked varied-timestep fixture changes the Zone timestep but is
planned-not-claimed and does not prove a system-step push.

Rust exposes the selector and generic wrappers but never dispatches the
PushSystem selector in production. Production has three lexical compat sites.
The feature-disabled site is exclusive with adaptive correction; each enabled
Zone independently takes count one or greater than one, so both adaptive sites
may execute in a multi-Zone timestep, but exactly one site executes per Zone.
Each site either rebuilds three slots at a full Zone step or commits only after
all locally chosen adaptive substeps. Rust has no source global cadence, Space
arena, fourth slot, singular child, or CP211 RoomAir/AFN and auxiliary state.
The source-order wrapper has one direct label/order test; the compat wrapper and
selector dispatch have none. One indirect count-one three-slot synchronization
test does not establish CP210 parity.

CP210 remains required `source_mapped` and adds no Rust target, mapped state,
support, output, numerical, or conformance claim. The inventory becomes 32
algorithms and 218 routines, split 58 `state_mapped` plus 160
`source_mapped`, with 95 required; the heat-balance project list becomes 64.

The following required predictor/corrector definition entry is
`zone_space_heat_balance_push_system_timestep_history`, after
`push_system_timestep_histories` and before
`update_final_surface_heat_balance`. Its source boundary is
`ZoneSpaceHeatBalanceData::pushSystemTimestepHistory(EnergyPlusData &state,
int zoneNum, int spaceNum = 0)`, declared at
`ZoneTempPredictorCorrector.hh` line 247 and implemented at
`ZoneTempPredictorCorrector.cc` lines 4297-4370.

Its only production expressions are CP210's Zone and Space child calls. After
a debug-only positive-Zone assertion, CP211 interleaves descending four-slot
`DSXMAT` and `DSWPrevZoneTS` shifts, then inserts current `MAT` and
`airHumRat`. Exact Zones under the global nonmixing flag additionally advance
applicable Floor/occupied/mixed or AirflowNetwork-node four-slot histories.
Spaces receive only record state.

Every solution value except exact ThirdOrder then advances record `TM2/TMX`
from `MAT` and `WM2/WMX` from distinct `airHumRatTemp`. Exact Zones also
advance applicable stratified M2/MX or AFN temperature/humidity T2/TX pairs.
ThirdOrder skips this final stage but retains the common record and applicable
four-slot RoomAir work. The two AFN stages use stored-container traversal and
`NumOfAirNodes` indexing respectively.

CP211 has no upper-bound, Space-sign, membership, record-kind, allocation,
topology, node-count, enum, or finite validation and no diagnostic, status,
catch, cleanup, transaction, or rollback. A late non-return can preserve common,
RoomAir/AFN, and non-ThirdOrder prefixes, blocks the rest of CP210 and its
parent's following work, and makes same-state retry destructive. The following
Zone-timestep revert does not restore CP211 state.

No C++ test directly calls CP211 or asserts a destination. Fifty-five
nonzero-Zone configurations provide only conditional one-pass potential if the
adaptive gate selects CP210: 105 records split 95 ThirdOrder and ten
Analytical, with no Euler. All 81 Zones are Mixing and 24 Spaces skip shared
RoomAir, so special RoomAir/AFN destinations have zero corpus potential.
Tracked one-Zone outputs are hourly downstream variables and expose neither a
system-history slot nor a push count.

Rust has no singular helper, Space arena, fourth slot, temporary-humidity or
non-ThirdOrder scalar state, RoomAir, or AFN history. Count-one and
feature-disabled paths rebuild three Zone slots from current and Zone histories
even at a full Zone step; adaptive count-greater-than-one shifts local
three-slot arrays after each local correction outside the named wrapper and
commits only once after all local substeps. One source-order label test and one
indirect count-one rebuild test do not establish CP211; the latter positively
shows prior system histories being discarded.

CP211 remains required `source_mapped` and adds no Rust target, mapped state,
support, output, numerical, or conformance claim. The inventory becomes 32
algorithms and 219 routines, split 58 `state_mapped` plus 161
`source_mapped`, with 96 required; the heat-balance project list becomes 65.

The following required predictor/corrector definition entry is
`revert_zone_timestep_histories`, after
`zone_space_heat_balance_push_system_timestep_history` and before
`update_final_surface_heat_balance`. Its source boundary is
`RevertZoneTimestepHistories(EnergyPlusData &state)`, declared at
`ZoneTempPredictorCorrector.hh` line 297 and implemented at
`ZoneTempPredictorCorrector.cc` lines 4372-4389.

Its sole direct thermal call is the CP195 dispatcher arm, but all nine built-in
`ManageZoneAirUpdates` call sites and the test tree select other controls.
EnergyPlus 26.1 therefore has no built-in CP212 request, timing, or runtime
gate. An external dispatcher request would run the common setpoint input/init
prefix, ignore the three timestep arguments, and preserve `ZoneTempChange`.

If requested, CP212 visits Zones ascending, calls each Zone child first, then
under the current aggregate `doSpaceHeatBalance` flag visits stored Spaces in
container order. It does not scan, sort, deduplicate, validate topology, or
write history directly. CP213 owns all Zone-timestep record and RoomAir/AFN
mutation. Neither routine restores CP211 downstepped histories or
non-ThirdOrder scalar state.

A child non-return preserves earlier and failing prefixes and suppresses later
traversal. Same-state retry starts at Zone one and reapplies CP213's
forward-copy revert, so it is destructive. CP212 has no local assertion,
diagnostic, status, completion count, catch, cleanup, transaction, or rollback.

All 57 active `ManageSimulation` expressions execute CP212 zero times. The
usual 81 Zone plus 24 eligible Space records are only 105 counterfactual
once-requested identities, not runtime evidence. Special RoomAir/AFN topology
is absent, no C++ test calls the wrapper or child or asserts a destination, and
tracked outputs expose no history slot or revert count.

Rust defines the selector and plural identity wrappers, but its dispatcher
ignores the selector. Its sole live compat call runs per Zone only for adaptive
count greater than one and conditionally resets current temperature and
humidity when the count changed; it shifts no Zone history and has no global
Zone/Space traversal. The one wrapper label test has no state assertion, and
the count-one adaptive test never reaches this path.

CP212 remains required `source_mapped` and adds no Rust target, mapped state,
support, output, numerical, or conformance claim. The inventory becomes 32
algorithms and 220 routines, split 58 `state_mapped` plus 162
`source_mapped`, with 97 required; the heat-balance project list becomes 66.

The following required record definition entry is
`zone_space_heat_balance_revert_zone_timestep_history`, after
`revert_zone_timestep_histories` and before
`update_final_surface_heat_balance`. Its source boundary is
`ZoneSpaceHeatBalanceData::revertZoneTimestepHistory(EnergyPlusData &state,
int zoneNum, int spaceNum = 0)`, declared at
`ZoneTempPredictorCorrector.hh` line 249 and implemented at
`ZoneTempPredictorCorrector.cc` lines 4391-4431.

Its only callers are the CP212 Zone and Space children. Because the parent
Revert selector has no built-in request, all 57 active `ManageSimulation`
expressions execute CP213 zero times. The 81 Zone plus 24 eligible Space
records bound only 105 maximum counterfactual once-requested common-path identities when CP212 is injected
in each configuration's corresponding active sizing or simulation phase. The
24 Spaces split into three simulation and 21 sizing identities across eight
configurations; special RoomAir/AFN corpus potential is zero.

After a debug-only positive-Zone assertion, CP213 forward-copies four-slot
`XMAT` and `WPrevZoneTS` histories from slots one through three into zero
through two for every reached record. Exact zero Space identity enables
RoomAir handling: the corresponding exact stratified enum copies Floor/occupied histories, while exact `AirflowNetwork`
copies each stored node's temperature/humidity histories. These enum branches
are mutually exclusive in normal state. The literal mixed-level code leaves
slot two unchanged and self-assigns slot three, yielding
`[old1, old2, old2, old3]`; this potential source typo is mapped, not
normalized.

CP213 changes no current value, downstepped history, non-ThirdOrder scalar, or
solution state and is not a full inverse of CP209. It has no upper-bound,
Space-sign, record-kind, membership, topology, allocation, enum, or finite
validation and no diagnostic, status, catch, cleanup, transaction, or rollback.
A late non-return retains ordered prefixes, while same-state retry collapses
histories again.

Rust has no singular helper, Space arena, fourth slot, RoomAir/AFN state, or
production Zone-history forward copy. Its nearest predictor and adaptive paths
push three-slot Zone or local system histories in the opposite direction, and
the plural Revert compat site conditionally resets only current Zone
temperature and humidity. Existing label, count-one, interpolation, and
uniform-history tests do not establish CP213.

CP213 remains required `source_mapped` and adds no Rust target, mapped state,
support, output, numerical, or conformance claim. The inventory becomes 32
algorithms and 221 routines, split 58 `state_mapped` plus 163
`source_mapped`, with 98 required; the heat-balance project list becomes 67.

The following required record definition entry is
`zone_space_heat_balance_correct_hum_rat`, after
`zone_space_heat_balance_revert_zone_timestep_history` and before
`update_final_surface_heat_balance`. Its source boundary is
`ZoneSpaceHeatBalanceData::correctHumRat(EnergyPlusData &state, int zoneNum,
int spaceNum = 0)`, declared at `ZoneTempPredictorCorrector.hh` line 241 and
implemented at `ZoneTempPredictorCorrector.cc` lines 4433-4619.

Its sole production call is CP207 line 4128. The parent has already solved
temperature, written `MAT`, and reported sensible demand. Only a successful
return commits record humidity and RH and produces the correction delta.
Initial HVAC correction and every selected adaptive fine step repeat this
record transaction. Each pass visits every Zone and only active non-sizing
simulation Spaces. The current corpus therefore has an 84-record static
one-pass topology: 81 Zones plus three Spaces, split 74 ThirdOrder and ten
Analytical, with no Euler. This is not a runtime call total. The records split
55 controlled versus 29 without controlled primary flow; plenum and
parallel-PIU paths have zero corpus reach. AFN multizone replacement can reach
five Zone identities, AFN distribution latent addition three, and duct latent
addition three, but no assertion isolates those coefficient effects.

CP214 collects primary moisture/mass flow from exactly one controlled,
return-plenum, supply-plenum, or empty branch, then independently adds
parallel-PIU leakage. The literal PIU loop indexes global PIUs one through the
stored list size rather than reading the list's identities. It builds latent
gain, density, vapor enthalpy, and default `A/B`; an active AFN multizone
condition replaces `A/B`, while AFN distribution and duct latent terms are
added independently. `C` always uses parent-Zone volume and moisture capacity,
including for Space records.

ThirdOrder, Analytical, and Euler equations write `airHumRatTemp`; an
unmatched enum retains its old value. A strict negative-to-zero clamp and
saturation cap precede a parent-Zone RoomAir AFN control-node overwrite that
also applies to Space records and is not reclamped. Exact-Zone hybrid humidity
inference follows, then a positive selected node receives humidity before
enthalpy. Optional latent sizing finally reports raw latent gain with the
already reported sensible load to the selected Zone or Space moisture-demand
owner.

A positive Space uses its record, node, and latent-sizing demand but retains
parent-Zone equipment, multiplier, capacity, AFN/duct, radiant/pool, and
RoomAir context. A malformed negative Space identity uses Zone node and demand
state and skips only the exact-Zone hybrid branch. CP214 has only a debug
positive-Zone assertion and no bounds, membership, topology, allocation,
multiplier, timestep, pressure, denominator, enum, or finite validation. It
has no status, catch, transaction, cleanup, or rollback. Failure can retain
temporary humidity, hybrid, node, or demand prefixes after the parent's prior
`MAT` and sensible writes while blocking final humidity/RH and later record
traversal. Retry can resample hybrid schedules and repeat histories, reports,
and diagnostics.

Ten direct C++ calls and five indirect focused calls cover only Euler node
humidity and bounded hybrid-inference slices. No focused assertion establishes
Space, plenum/ADU/PIU, AFN/duct, alternate solver, clamp, enthalpy,
latent-sizing, failure, retry, or reset behavior.

Rust has no singular Zone/Space helper. Main heat-balance humidity correction
uses only three-slot history, writes current Zone humidity directly, enforces
a `1e-5` floor and `0.008` invalid fallback, and omits the source `A/B/C`
moisture transaction and all Space/lifecycle effects. A separate guarded
IdealLoads helper implements one no-OA ThirdOrder purchased-air subset and
returns corrected humidity plus `A/B`, but omits the other source terms,
solvers, topology, node/RH/hybrid/sizing effects, and partial-failure
semantics. Existing official dynamic and no-OA IdealLoads humidity claims
remain at their declared case boundaries.

CP214 remains required `source_mapped` and adds no Rust target, mapped state,
support, output, numerical, or conformance claim. The inventory becomes 32
algorithms and 222 routines, split 58 `state_mapped` plus 164
`source_mapped`, with 99 required; the heat-balance project list becomes 68.

The following required logical routine entry is
`down_interpolate_4_history_values`, after
`zone_space_heat_balance_correct_hum_rat` and before
`update_final_surface_heat_balance`. CP215 maps only its first, void
scalar-output overload, declared at `ZoneTempPredictorCorrector.hh` lines
299-308 and implemented at `ZoneTempPredictorCorrector.cc` lines 4621-4702.

The helper computes the raw old/new timestep ratio before any output, writes
`newVal0` through `newVal4` in order, selects strict `0.01` ratio-two then
ratio-three bands, and sends every other ratio through a sequential
interpolation recurrence. It validates no positive or finite timestep,
shortening direction, integer ratio, history value, or output aliasing.
Inputs are copied by value; shared output storage is last-write-wins. It has no
status, diagnostic, transaction, rollback, or reset owner.

Its only production expressions interpolate CO2 then generic-contaminant
histories per Zone in `PredictZoneContaminants`. Entry requires contaminant
simulation, `PredictStep`, shortening, a system-step count different from the
previous Zone timestep, and the matching species flag. Normal HVAC timing can
enter only on the first shortened adaptive fine-step prediction. A stable
distinct-output retry is overwrite-idempotent; changed or aliased caller state
is not.

One direct C++ ratio-two call has five destination assertions. Three focused
contaminant predictor fixtures keep shortening false, and all 57 active
full-simulation expressions have contaminant simulation disabled, so both
focused indirect and full-corpus reach are zero.

Rust's nearest helper produces only the first three analogous values for
thermal Zone temperature and humidity histories and returns its old values for
a nonpositive timestep. Its two compatibility-only production call sites
run only when a local adaptive count greater than one changes, and one test
covers ratios two, three, and four. Rust has no source contaminant ownership, final two outputs,
reference-alias transaction, or invalid-input parity.

CP215 remains required `source_mapped` and adds no Rust target, mapped state,
support, output, numerical, or conformance claim. The inventory becomes 32
algorithms and 223 routines, split 58 `state_mapped` plus 165
`source_mapped`, with 100 required; the heat-balance project list becomes 69.

CP216 completes the current logical
`down_interpolate_4_history_values` evidence boundary by mapping its
independent array-return overload, declared at
`ZoneTempPredictorCorrector.hh` lines 310-311 and implemented at
`ZoneTempPredictorCorrector.cc` lines 4704-4736. It computes the raw old/new
timestep ratio, writes four array elements in ordered ratio-two, ratio-three,
or fallback branches, then returns `oldVals[0]`. The last input
element is never read, while the third is used only for the ratio-two final
output.

The helper validates no positive or finite timestep, shortening direction,
integer ratio, history value, or distinct input/output arrays. Its const input
reference can alias its mutable output reference, making later reads observe
earlier writes. Distinct-array replay is deterministic overwrite-idempotent;
same-array replay generally is not. It owns no status, diagnostic,
transaction, rollback, or reset.

Seven production expressions live in
`ZoneSpaceHeatBalanceData::updateTemperatures`: two base temperature/humidity
calls per eligible Zone or active Space, three exact-Zone displacement/UFAD
temperature calls, and two calls per exact-Zone AFN node. Entry requires
shortening plus a system-step count different from the previous Zone
timestep. Normal HVAC timing permits only the first shortened fine-step
prediction, and a matching count reuses existing downstepped state.

One direct C++ ratio-two call has nine post-call assertions for the return,
four outputs, and four unchanged distinct inputs. Focused wrapper tests have
zero Zones. The 55 nonzero-Zone completing corpus configurations leave actual
adaptive entry unobserved; their conditional one-pass topology is 81 Zones
plus 24 eligible Spaces, or 210 base calls, with no stratified or AFN
potential because all Zones are Mixing.

Rust's nearest helper returns only three values by value, rejects nonpositive
timesteps, and has two Zone-only compatibility-path calls plus ratio-two,
ratio-three, and ratio-four tests. It has no fourth array output, separate
scalar return, Space/RoomAir/AFN topology, node rollback, alias transaction, or
invalid-input parity.

CP216 expands the same required `source_mapped` routine and adds no new
routine, project-contract item, Rust target, mapped state, support, output,
numerical, or conformance claim. Counts remain 32 algorithms and 223 routines,
split 58 `state_mapped` plus 165 `source_mapped`, with 100 required; the
heat-balance project list remains 69.

CP217 adds required `inverse_model_temperature` immediately after
`down_interpolate_4_history_values` and before
`update_final_surface_heat_balance`. Its EnergyPlus boundary is
`InverseModelTemperature`, declared at `ZoneTempPredictorCorrector.hh` lines
313-325 and implemented at `ZoneTempPredictorCorrector.cc` lines 4737-4951.

Every call first samples the measured-temperature schedule or zero and resets
the current thermal-mass multiplier to one. An inclusive hybrid date window
then overwrites `ZT` with measured temperature before three independent,
source-ordered infiltration, internal-mass, and people inverse branches. The
branches require global Zone-timestep history. The final three-slot measured
history shift is unconditional on both the date window and history selection,
so an outside-window call inserts ordinary solved `ZT`, while adaptive
fine-step calls repeat the measured override and shift without recalculating
the inverse outputs.

The infiltration branch optionally substitutes measured supply temperature,
mass flow, and humidity, solves mass flow only for strict
`abs(Tmeasured - Tout) > 0.5`, clamps air changes per hour to `[0, 10]`, and
reconstructs mass flow. The internal-mass branch requires exact zero
`SumSysMCpT` and a changed measured temperature, adds AFN sensible exchange and
the literal duct `ZoneLat` term, analytically inverts air capacity, derives a
multiplier only above a strict 0.05 K change, and delegates clamp/warning and
aggregate ownership to the next routine. The people branch re-samples measured
temperature, samples activity twice, stores raw schedule values while applying
local 130/0.6/0.7 defaults, and bounds inferred people by the current summed
convective internal gain before zeroing values below 0.05.

The only production expression is the exact-Zone HybridModel gate inside
`ZoneSpaceHeatBalanceData::correctAirTemp`, after the forward temperature and
load work but before `MAT`, reporting, and humidity correction. Initial normal
HVAC correction can calculate with Zone history. If adaptive shortening is
then selected, every fine correction still re-enters CP217 with system history:
the three calculations skip, but schedule sampling, multiplier reset,
active-window `ZT` replacement, and history shift repeat. Demand resimulation
adds no correction call.

CP217 has no local configuration, bounds, finite, schedule, denominator,
psychrometric, or lifecycle validation and no status, transaction, rollback,
or reset. A same-state retry can consume already shifted histories and
double-add the next helper's statistics/warnings. Begin-environment setup resets
only the three measured histories, so skipped inference outputs can remain
stale.

One C++ fixture reaches CP217 indirectly five times and asserts only an
approximately 15.13 internal-mass multiplier, approximately 0.2444 and 0.49
infiltration rates, and two zero people counts. There is no direct CP217 test.
All 57 active full-simulation expressions configure no HybridModel, so actual
corpus reach and hybrid output-oracle count are zero.

Rust has no typed `HybridModel:Zone`, inverse configuration, measured history,
inferred-result state, exact hybrid output names, runtime path, or focused
test. Its three-slot Zone histories and temperature coefficients feed a
guarded forward solve; typed People data supports design-count consumers, not
temperature inverse inference. `HybridModel:Zone` remains RawOnly and
run-blocking.

CP217 remains required `source_mapped` and adds no Rust target, mapped state,
support, output, numerical, or conformance claim. The inventory becomes 32
algorithms and 224 routines, split 58 `state_mapped` plus 166 `source_mapped`,
with 101 required; the heat-balance project list becomes 70.

CP218 adds required `process_inverse_model_multp_hm` immediately after
`inverse_model_temperature` and before `update_final_surface_heat_balance`.
Its EnergyPlus boundary is `processInverseModelMultpHM`, declared at
`ZoneTempPredictorCorrector.hh` lines 327-333 and implemented at
`ZoneTempPredictorCorrector.cc` lines 4953-4991.

The helper first accesses the selected Zone and heat-balance record. A
multiplier strictly below one is overwritten with one and excluded from
statistics. Exactly one is unchanged and excluded. Every value strictly above
one is added to the `Real64` sum and increments the `Real64` count, after which
any count at least one recomputes the average. The exact value 30 is accepted
without warning. A value above 30 emits first-occurrence and recurring
diagnostics but is neither capped nor excluded; this executable predicate
contradicts the source comment that statistics stop at the maximum.

The sole production call is CP217's internal-mass branch. It passes local
multiplier state plus the Zone-owned sum, count, and average, then CP217 writes
the returned/lower-clamped current multiplier. All CP217 exact-Zone, date,
hybrid, history, non-warmup, non-sizing, and correction-cadence gates therefore
apply. Initial Zone-timestep correction can add one sample; adaptive fine
corrections use system history and skip CP218.

The first value above 30 writes one immediate warning with two continuation
lines when the per-Zone error index is zero, then every such value updates an
at-end recurring record and its index. Diagnostics precede statistics. The
Zone sum/count default to zero, average defaults to one, and the heat-balance
warning index defaults to zero. No ordinary begin-environment path resets any
of them; the average feeds the Hybrid Model internal-thermal-mass tabular
subtable, while CP218 registers no output itself.

CP218 validates no finite value, overflow, count shape, denominator, reference
distinctness, Zone bound, or allocation. NaN is not clamped, warned, or added;
positive infinity is warned and added; negative infinity clamps to one. Its
four mutable references can alias. There is no status, catch, transaction, or
rollback. Repeating a value above one adds it again, and repeating an
above-30 value also updates recurring state again.

One direct C++ fixture makes five calls and has 26 assertions. It proves
`0.5 -> 1` without aggregation, exact one exclusion, 10 aggregation,
uncapped-and-aggregated 50 with warning, and a later low sample preserving
prior statistics. It uses local aggregate references initialized to zero,
does not prove the production average default, and does not test exact 30,
nonfinite/alias/malformed state, repeated-high recurring behavior, failure,
retry, or reset. One indirect internal-mass case asserts only the downstream
current multiplier near 15.13. All 57 active full simulations configure no
HybridModel, so CP218 and its output/report oracles have zero corpus reach.

Rust has no typed `HybridModel:Zone`, inferred multiplier, aggregate fields,
per-Zone recurring index, exact output, report table, runtime path, capability,
or focused test. Its physical air-capacity state serves the forward solver.
The raw HybridModel object remains run-blocking.

CP218 remains required `source_mapped` and adds no Rust target, mapped state,
support, output, numerical, or conformance claim. The inventory becomes 32
algorithms and 225 routines, split 58 `state_mapped` plus 167 `source_mapped`,
with 102 required; the heat-balance project list becomes 71.

CP219 adds required `inverse_model_humidity` immediately after
`process_inverse_model_multp_hm` and before
`update_final_surface_heat_balance`. Its EnergyPlus boundary is
`InverseModelHumidity`, declared at `ZoneTempPredictorCorrector.hh` lines
335-343 and implemented at `ZoneTempPredictorCorrector.cc` lines 4993-5131.

Every call unconditionally dereferences and samples the measured-humidity
schedule before the inclusive hybrid date-window test. An active window writes
the measurement to `airHumRat`, then independently runs infiltration followed
by People inverse branches only when global Zone-timestep history is selected.
Every normal return shifts the three measured-humidity histories even outside
the date window or when system history skips both calculations.

The infiltration branch optionally substitutes unguarded measured supply mass
flow and humidity. Both paths omit `OAMFL` and ignore the caller's Zone and
moisture flow inputs. It solves only for strict measured/outdoor humidity
difference above `1.0e-7`, clamps ACH to `[0, 10]`, reconstructs kg/s mass
flow, and writes mass flow before ACH.

The People branch stores nullable activity, sensible, and radiant schedule
values. Sensible fraction defaults to 0.6 only when nonpositive and radiant
fraction is unused. A source anomaly leaves local activity initialized to zero
and never assigns the sampled value, so calculation always defaults to
130 W/person. The result is bounded by total latent gain, rounded half-up to
two decimals, then zeroed only when strictly below 0.05.

The sole production call is the exact-Zone HybridModel gate inside
`ZoneSpaceHeatBalanceData::correctHumRat`, after forward humidity solving,
clamps, and RoomAir override but before node and latent-sizing work. CP219
changes `airHumRat`, not `airHumRatTemp`; the following node and sizing work
uses the forward temporary, and `correctAirTemp` overwrites the measured record
write with that temporary after return. Adaptive fine corrections still
resample, transiently overwrite, and shift histories while global system
history skips inference. Demand resimulation adds no correction call.

CP219 has no local pointer, date, bounds, finite, denominator, psychrometric,
or lifecycle validation and no direct diagnostic, status, transaction,
rollback, or reset. An abnormal non-return can retain sampled, transient, supply, or inferred
prefixes while blocking all or part of the final history shift. A same-state
replay after a normal return consumes already shifted histories and is
non-idempotent; retry after an abnormal non-return observes only the ordered
prefix that actually committed. Begin-environment setup resets
only the three histories; skipped inference outputs can remain stale.

Four indirect C++ calls assert only two approximately 0.5 ACH results and two
approximately 4 People results. There is no direct CP219 test, and no assertion
isolates the activity-130 anomaly, history, transient overwrite, threshold,
rounding, failure, retry, reset, or output registration. All 57 active full
simulations configure no HybridModel, so actual corpus reach and hybrid output
oracles are zero.

Rust has no typed `HybridModel:Zone`, measured-humidity schedule/history,
humidity inverse state, typed infiltration, inferred People state, exact
HybridModel output, runtime path, capability, or focused test. Its forward
Zone humidity histories and no-OA IdealLoads helper do not implement this
inverse transaction; the raw HybridModel object remains run-blocking.

CP219 remains required `source_mapped` and adds no Rust target, mapped state,
support, output, numerical, or conformance claim. The inventory becomes 32
algorithms and 226 routines, split 58 `state_mapped` plus 168 `source_mapped`,
with 103 required; the heat-balance project list becomes 72.

CP220 adds required `zone_space_heat_balance_calc_zone_or_space_sums`
immediately after `inverse_model_humidity` and before
`update_final_surface_heat_balance`. Its EnergyPlus boundary is
`ZoneSpaceHeatBalanceData::calcZoneOrSpaceSums`, declared at
`ZoneTempPredictorCorrector.hh` lines 226-230 and implemented at
`ZoneTempPredictorCorrector.cc` lines 5133-5281.

Every call first zeros the three surface sums and two system-air sums, assigns
Zone- or Space-owned internal convection gains, and adds the parent Zone's
complete radiant-system and pool convection terms. `NoHeatToReturnAir`
optionally adds Zone- or Space-owned return-air gains. It then overwrites the
ordinary infiltration/ventilation/mixing/earth-tube/cooltower/outdoor-air coefficients or replaces
them with parent-Zone AFN multizone exchange values.

Only `CorrectorFlag` true assembles controlled-inlet, return-plenum, or
supply-plenum system airflow and the independent parallel-PIU leakage tail.
All heat capacities use the receiver's humidity ratio rather than node
humidity. The PIU loop ignores the identities stored in
`leakageParallelPIUNums` and instead reads global ordinals one through its
size. System sums are divided by the unguarded parent Zone multiplier product;
an uncontrolled Space then volume-scales only those two system fields.
Internal, ordinary/AFN non-system, and surface terms are not Space-scaled.

The routine always dispatches virtual `calcSumHAT` after coefficient assembly
and commits its four returned values only after the child returns. The
following Zone override, mapped by CP221, traverses every stored Space without
testing `doSpaceHeatBalance`, so Zone-first work can already visit child
surfaces before an explicit Space pass. Its Space dependency reads parent-Zone
system sums for supply-air reference temperature and can increment external
Window report state. Completed replay is therefore deterministic only when
those child dependencies are side-effect-free.

The two production expressions are the false predictor call inside
`predictSystemLoad` and the true correction call inside `correctAirTemp`.
Initial and adaptive HVAC work can repeat both; demand resimulation adds only
prediction. The false flag gates system/PIU assembly only: Space equipment
lookup, internal/non-system work, AFN replacement, and surface dispatch still
run.

CP220 has two debug positive-Zone assertions but no complete identity,
topology, multiplier, allocation, finite, status, diagnostic, transaction, or
rollback boundary. Failure can leave the entry-zeroed fields alongside stale
not-yet-assigned fields, or retain later gain, airflow, allocation, and child
Surface prefixes. `beginEnvironmentInit` does not reset these sums; coordinated
owner reset is required for clean recovery.

One direct C++ fixture makes five Zone calls and 12 assertions over surface
reference modes, system flow, false-flag zeros, and one PIU leak. It does not
exercise a Space receiver, AFN, plenums, PIU identity, volume allocation,
Window side effects, malformed state, failure, retry, or reset. The 55
nonzero-Zone corpus configurations provide a static one-prediction census of
105 Zone/Space calls and one-correction census of 84, not a runtime-call total;
controlled Space, plenum, and PIU topology are absent and AFN coefficients are
not isolated.

Rust has no matching routine or Space heat-balance record. Its nearest
Zone-only opaque-surface hA/hAT helper, OtherEquipment convection subset, and
zero-initialized airflow fields omit return-air, ordinary airflow/AFN
assembly, controlled/plenum/PIU topology, Space allocation, Window/reference
branches, and source lifecycle. Existing one-Zone evidence has no such
topology and is zero-only for the relevant gain/transfer boundary.

CP220 remains required `source_mapped` and adds no Rust target, mapped state,
support, output, numerical, or conformance claim. The inventory becomes 32
algorithms and 227 routines, split 58 `state_mapped` plus 169 `source_mapped`,
with 104 required; the heat-balance project list becomes 73.

CP221 adds required `zone_heat_balance_calc_sum_hat` immediately after
`zone_space_heat_balance_calc_zone_or_space_sums` and before
`update_final_surface_heat_balance`. Its EnergyPlus boundary is
`ZoneHeatBalanceData::calcSumHAT`, declared at
`ZoneTempPredictorCorrector.hh` line 254 and implemented at
`ZoneTempPredictorCorrector.cc` lines 5283-5298.

The Zone override debug-asserts positive Zone and exact-zero Space identity,
zero-initializes a four-field local return, and visits every stored
`Zone.spaceIndexes` identity in container order. Each child
`SpaceHeatBalanceData::calcSumHAT` result is added as internal gain, HA,
HATsurf, then HATref. An empty list returns four zeros. There is no
`doSpaceHeatBalance` gate, sorting, filtering, deduplication, membership
validation, or record write in CP221.

Its sole production ingress is CP220's virtual surface-sum call. Zone receivers
enter CP221; Space receivers dispatch directly to the CP222 child. Initial and
adaptive prediction/correction and demand-resimulation prediction therefore
invoke CP221 once per Zone CP220 call, and CP221 traverses all stored Spaces
even when the outer wrapper schedules no explicit Space record.

CP221 owns no persistent state, diagnostic, status, transaction, or rollback.
A child non-return discards the local partial aggregate and blocks later
children, while completed child Window/report effects remain. Retry starts
again from zero and repeats children from the first identity, so stateful
Window paths are not generally idempotent. Release builds ignore the asserted
`spaceNum == 0` convention and retain no Zone upper-bound or topology guard.

No test calls CP221 directly. One focused CP220 fixture reaches it five times
with one Space; only its first two calls assert six aggregate surface values.
The 55 nonzero-Zone corpus configurations provide a static one-pass census of
81 CP221 calls and 99 nested Space children for prediction and the same for
correction. The combined 162/198 census is structural, not a runtime total;
outer explicit Space CP220 calls can repeat 24 prediction and three correction
children.

Rust has no matching routine, four-field result, or Space heat-balance record.
Its nearest helper directly folds Zone-owned opaque Surface indexes into
HA/HATsurf with HATref fixed to zero, skips invalid indexes, returns no
internal-gain term, and has no Space order, duplicate, child-side-effect, or
failure contract. Existing one-Zone surface-convection tolerance evidence
covers only a single default partition whose Space identity runtime discards;
it does not establish CP221.

CP221 remains required `source_mapped` and adds no Rust target, mapped state,
support, output, numerical, or conformance claim. The inventory becomes 32
algorithms and 228 routines, split 58 `state_mapped` plus 170 `source_mapped`,
with 105 required; the heat-balance project list becomes 74.

CP222 expands the existing required `zone_heat_balance_calc_sum_hat` source
mapping to its independent Space override. It adds no second routine or
project-contract item because both C++ definitions share the unqualified
source identifier `calcSumHAT`; project order remains
`zone_heat_balance_calc_sum_hat` immediately before
`update_final_surface_heat_balance`. The added EnergyPlus boundary is
`SpaceHeatBalanceData::calcSumHAT`, declared at
`ZoneTempPredictorCorrector.hh` line 259 and implemented at
`ZoneTempPredictorCorrector.cc` lines 5300-5413.

The Space override debug-asserts positive Zone then Space identities,
zero-initializes a four-field local result, and walks the selected Space's
inclusive `HTSurfaceFirst..HTSurfaceLast` integer range. It neither reads the
receiver nor filters by class or membership nor validates bounds or ownership;
an inverted range returns four zeros.

Each Window contributes shade/blind, equivalent-layer, airflow, frame, divider,
and glazing terms in source order. The airflow/no-return-air path also mutates
`SurfWinHeatGain` with `+=`, updates the gain side for a nonnegative comparison
and otherwise the loss side, including for NaN, leaves the opposite gain/loss
pair stale, and overwrites signed transfer energy. All Surface classes then
contribute base HA and HATsurf.

Zone-mean reference air adds HA, adjacent air adds HA times effective bulk-air
temperature, and supply air uses the parent Zone's system-weighted temperature
only for a strictly positive parent `SumSysMCp`. Zero, negative, or NaN flow
falls back to HA. An uncontrolled Zone is fatal; an invalid/default reference
silently falls back to HA. CP222 does not use a Space record's system sums.

CP222 is reached both through every CP221 stored-Space child and directly from
explicit Space CP220 receivers. The corrected static corpus census is 123
prediction calls, 99 nested plus 24 explicit, and 102 correction calls, 99
nested plus three explicit, or 225 combined. These are configuration counts,
not runtime totals; warmup, adaptive steps, and demand resimulation can repeat
them, and nested plus explicit visits can repeat the same child.

No test calls CP222 directly. One CP220 fixture reaches one three-surface,
non-Window child five times; only its first two calls assert six
HA/HATsurf/HATref values. It gives bounded ZoneMean, Adjacent, and ZoneSupply
positive-flow/fallback coverage, but no Window, `sumIntGain`, fatal, default,
failure, retry, or reset coverage.

Rust has no matching four-field result, Space heat-balance arena, Window
runtime, or reference-air switch. Its nearest Zone-only opaque-Surface helper
returns HA, HATsurf, and a fixed-zero HATref while silently skipping invalid
indexes. Existing one-Zone opaque-surface evidence does not establish CP222.

CP222 expands the same required `source_mapped` routine and adds no new routine,
project-contract item, Rust target, mapped state, support, output, numerical,
or conformance claim. Counts remain 32 algorithms and 228 routines, split 58
`state_mapped` plus 170 `source_mapped`, with 105 required; the heat-balance
project list remains 74.

CP223 adds required `calc_zone_component_load_sums` immediately after
`zone_heat_balance_calc_sum_hat` and before
`update_final_surface_heat_balance`. Its EnergyPlus boundary is the nonmember
`CalcZoneComponentLoadSums`, declared at `ZoneTempPredictorCorrector.hh` lines
345-348 and implemented at `ZoneTempPredictorCorrector.cc` lines 5414-5677.

The routine is a correction-only reporting update sequence. It first overwrites,
in order, internal gain, surface convection, interzone, outdoor, system-air,
non-air-system, air-storage, imbalance, melting-enthalpy, and
freezing-enthalpy fields with zero. It then rebuilds the first six component
rates from the current receiver heat-balance record plus parent-Zone state.
Space report calls still use Zone-wide internal gains and return-air gains,
parent-Zone AFN exchange and equipment/plenum/PIU topology, Zone radiant and
pool convection, and parent-Zone volume.

Controlled-Zone inlet work calculates sensible transfer using receiver MAT and
humidity ratio. A mapped ADU independently receives overwritten heating and
cooling rates plus system-timestep energies from its outlet node. A return plenum follows its inlet and stored-ADU leak paths; a supply
plenum follows its single inlet. The independent
parallel-PIU tail uses only the stored list size and reads global PIU ordinals
one through that size rather than the stored identities. No Zone multiplier
is applied.

Every call then walks all stored Spaces of the parent Zone and each raw
inclusive heat-transfer-Surface range. There is no selected-Space argument, so
each Space report repeats the complete parent-Zone Surface topology. Reference
air is recomputed through `Surface::getInsideAirTemperature` from the Surface's
owning Space record: ZoneMean uses that Space MAT, Adjacent uses effective
bulk-air temperature, and ZoneSupply uses owning-Space inlets when aggregate
Space heat balance is active or Zone inlets otherwise. An uncontrolled
ZoneSupply Surface is fatal.

The Window path adds interior shade/blind, equivalent-layer, natural-gap,
airflow, frame, divider, and base convection terms without CP222's Window
report mutations. Exact CondFD Surfaces add raw melting and freezing enthalpy
fields. ThirdOrder storage uses current receiver MAT, humidity, and first
history with parent-Zone volume and sensible capacitance multiplier;
Analytical and Euler use their respective receiver coefficients. An unknown
solution enum retains the entry zero.

Only `DisplayZoneAirHeatBalanceOffBalance` computes imbalance and its
20-percent quadrature threshold. A strict excess outside warmup and sizing
uses the parent Zone name and shared `AirHBimBalanceErrIndex`; Zone and every
Space report can therefore update the same recurring-warning state. The
routine has no local validation, status, catch, transaction, or rollback.
Failure retains the initial report reset, any later component prefix, ADU
overwrites, earlier Surface sums, and dependency diagnostics. Retry resets the
target report again but can repeat shared recurring warnings; repeated Space
calls can leave the last Space calculation on shared ADU report fields.

Its only production expressions are the Zone report call and conditional
simulation-Space report loop at the end of `correctZoneAirTemps`, after all
Zone/Space correction or mirroring for that Zone. Initial correction and each
adaptive fine-step correction can repeat CP223. Demand resimulation adds
prediction only and does not. The Space report gate does not test
`DoingSizing`.

No C++ test calls CP223 directly. Five HybridModel wrapper calls reach one
Zone-only report each through an empty Surface range, but every immediate
assertion targets unrelated hybrid state. The 55 completing nonzero-Zone
full-simulation configurations provide a static one-correction census of 81
Zone plus three active simulation-Space reports, or 84 calls. Those calls
perform 104 stored-Space range walks and split 74 ThirdOrder versus ten
Analytical records, with no Euler. Full-simulation imbalance-warning reach is
zero, and no assertion isolates a CP223 field, ADU effect, failure, retry, or
reset.

Rust has no ten-field `AirReportVars` analog, Space report arena, or singular
CP223 update sequence. Its run-period path separately samples Zone internal gain,
one of several opaque-Surface convection helpers, and a guarded
ThirdOrder-or-Analytical storage helper, then publishes those three series plus
a hard-coded zero outdoor-transfer series. It has no complete interzone,
system-air, non-air, deviation, PCM, Window, AFN, plenum, PIU, ADU, or shared
diagnostic ownership.

The official one-Zone candidate has one uncontrolled Zone, six opaque
ZoneMean-reference Surfaces, and no Space simulation, Window, airflow, or HVAC
topology. Across 8760 rows its internal and outdoor reports are exact zero;
surface convection has maximum absolute difference 0.085845581243 W and RMSE
0.005357748923 W, while air storage has 0.076879349871 W and
0.005076386180 W. Those four existing bounded output claims do not establish
the complete CP223 routine.

CP223 remains required `source_mapped` and adds no algorithm-level `energyplus_source` entry,
Rust target, code, mapped state, test, support, capability, output
implementation, comparator, manifest, numerical, performance, or conformance
promotion. The inventory becomes 32 algorithms and 229 routines, split 58
`state_mapped` plus 171 `source_mapped`, with 106 required; the heat-balance
project list becomes 75.

CP224 adds required `verify_thermostat_in_zone` immediately after
`calc_zone_component_load_sums` and before
`update_final_surface_heat_balance`. Its EnergyPlus boundary is the nonmember
`VerifyThermostatInZone(EnergyPlusData &state, std::string const &ZoneName)`,
declared at `ZoneTempPredictorCorrector.hh` line 350 and implemented at
`ZoneTempPredictorCorrector.cc` lines 5679-5700.

The routine first tests the shared `GetZoneAirStatsInputFlag`. When true, it
calls CP196 `GetZoneAirSetPoints` and clears the flag only after normal return.
It then uses `NumTempControlledZones > 0` solely as a gate and exact-string
searches the full allocated `TempControlledZone` arena through the
`ZoneTempControls::ZoneName` member. A positive one-based first-match index
returns true; no match, a nonpositive count, or an empty arena returns false.
The lookup does not normalize case or whitespace, resolve an actual Zone,
validate count-versus-allocation consistency, or inspect comfort, humidity, or
equipment controls.

The sole production call is `SetUpZoneSizingArrays` line 812. Once any
`ZoneEquipConfig` is controlled, each `ZoneSizingInput` whose cooling or
heating airflow method is exactly `FromDDCalc` calls CP224 once, even when both
methods match and even when that sizing Zone was not found in the controlled
equipment list. A false result makes the caller emit the non-pulse
missing-thermostat warning but does not set its `ErrorsFound` flag. Normal
`SizeZoneEquipmentOneTimeFlag` ownership limits this to first zone-sizing
setup rather than every HVAC iteration.

CP224 owns no diagnostic, output, allocation, cache, or mutation after input
acquisition. A CP196 fatal prevents the latch clear and boolean return, retains
the parser's allocated, output, and diagnostic prefix, and also prevents the
production sizing one-time flag from clearing. Same-state retry can therefore
re-enter the non-idempotent full input loader. After successful acquisition,
stable repeated CP224 lookup is read-only and deterministic. Clean replay
requires the CP196 owners plus both Zone-controls and Zone-equipment-manager
latches to be reset.

No C++ test calls CP224 directly. The
`AirTerminalSingleDuctMixer_GetInputDOASpecs` fixture reaches two false
lookups for two DesignDay sizing Zones with controlled equipment and no
thermostat, but asserts only outdoor-air pointer results. Of 57 active
full-simulation expressions, 34 completing configurations contain 48 direct
`Sizing:Zone` records. Every record uses DesignDay for both airflow methods
and has matching direct thermostat and equipment-connection names, yielding a
static first-setup census of 48 true calls. No assertion isolates CP224's
boolean, exact-name mismatch, lazy-latch timing, missing-thermostat warning,
failure, retry, or reset behavior.

Rust has no `VerifyThermostatInZone`, `verify_thermostat_in_zone`, equivalent
runtime predicate, or executable `Sizing:Zone` setup. Its compiler eagerly
creates only a bounded direct-Zone DualSetpoint `ZoneThermostat` subset,
normalizes names, builds ZoneId/thermostat graph edges, and emits
`EvaluateZoneThermostat` planning metadata. A separate
IdealLoads diagnostic consumes a normalized ZoneId edge and errors when the
thermostat is missing rather than returning CP224's boolean. The adjacent
`get_zone_air_set_points_compat` is an identity closure without the shared
input latch. Those typed records can support a project-specific membership
query but do not implement CP224's full CP196 acquisition, exact-string arena
lookup, caller warning, or failure lifecycle.

CP224 remains required `source_mapped` and adds no algorithm-level
`energyplus_source` entry, Rust target, code, mapped state, test, support,
capability, output implementation, comparator, manifest, numerical,
performance, or conformance promotion. The inventory becomes 32 algorithms
and 230 routines, split 58 `state_mapped` plus 172 `source_mapped`, with 107
required; the heat-balance project list becomes 76.

CP225 adds required `verify_controlled_zone_for_thermostat` immediately after
`verify_thermostat_in_zone` and before
`update_final_surface_heat_balance`. Its EnergyPlus boundary is the nonmember
`VerifyControlledZoneForThermostat(EnergyPlusData &state, std::string const &ZoneName)`,
declared at `ZoneTempPredictorCorrector.hh` line 352 and
implemented at `ZoneTempPredictorCorrector.cc` lines 5702-5713.

The sole body expression exact-string searches the full allocated
`ZoneEquipConfig` arena through the `EquipConfiguration::ZoneName` member and
returns whether the first-match index is positive. It does not load input,
normalize the argument, inspect `IsControlled` or the equipment list, resolve a
Zone identity, consult a controlled-Zone count, or search `spaceEquipConfig`.
It owns no write, diagnostic, output, separate status state, cache, or rollback.

Normal `GetZoneEquipmentData` setup allocates one configuration per Zone and
stores each valid equipment-connection name at its actual Zone index.
Uncontrolled slots retain the mixed-case sentinel `Uncontrolled Zone` with
`IsControlled == false`. Because CP225 ignores that flag, a direct argument
exactly equal to the sentinel or any manually corrupted uncontrolled-slot name
can return true. Standard parsed Zone names are uppercased, so an ordinary
input Zone written with that spelling does not collide with the mixed-case
sentinel.

The only direct production expressions are inside CP199
`InitZoneAirSetPoints`: ordinary temperature controls at line 2684 are checked
first, then comfort controls at line 2746. Each loop calls CP225 once per record
only while `ZoneEquipInputsFilled` is true and `ControlledZonesChecked` is
false. A false result makes the caller emit its family-specific Severe and
Continue diagnostics and set persistent `ErrorsFound`. Both loops finish
before that sticky error causes the line-2810 fatal. Only a normal input-filled
return reaches the state-lifetime `ControlledZonesChecked = true` commit.

CP199 is entered before every `ManageZoneAirUpdates` selector and also by the
external-HVAC initializer. Verification is deferred until the first invocation
that observes completed Zone-equipment input; the standard equipment manager
sets `ZoneEquipInputsFilled` only after its loader returns normally. A
successful pass prevents later rechecking even if the arena changes. An
input-filled state with no ordinary or comfort records performs no CP225 call
but still commits the checked latch.

CP225 itself is read-only, deterministic, and idempotent for stable state.
Caller failure is not a clean retry: the CP199 one-time initialization latch
has already cleared, diagnostics and other initialization effects persist,
`ErrorsFound` remains true, and `ControlledZonesChecked` remains false. A
caught retry repeats the scans and any missing-zone diagnostics, then fatals
again even if every later lookup succeeds unless the owning state is reset.

No C++ test calls CP225 directly. Four direct `InitZoneAirSetPoints` fixture
calls run before Zone-equipment input is marked filled and therefore reach it
zero times. Among 57 active full-simulation expressions, 38 completing
configurations contain 52 active direct-Zone ordinary thermostat records and
no comfort thermostat record. Every ordinary name exactly matches a
`ZoneHVAC:EquipmentConnections` name, yielding a static first-ready-check
census of 52 true calls and zero false calls. No assertion isolates CP225,
the comfort path, the sentinel case, a mismatch diagnostic, sticky failure,
retry, post-success mutation, or reset.

Rust has no `VerifyControlledZoneForThermostat`,
`verify_controlled_zone_for_thermostat`, preserved-name lookup arena, or
ordinary/comfort cross-validation lifecycle. The compiler first marks
`Zone::is_nominal_controlled` from nonblank raw equipment-connection Zone names
using trimmed ASCII-uppercase comparison, before full connection parsing; even
an incomplete connection can set this marker, and production code does not
consume it. It later parses bounded direct-Zone DualSetpoint thermostats and
typed equipment connections independently to `ZoneId`. Connection parsing
requires Zone, equipment-list, and Zone-air-node fields, resolves the first two
to typed IDs, and rejects missing required fields, unresolved Zone/list
references, and duplicate Zone connections, but does not
reject a valid thermostat whose Zone has no connection. Typed connections
discard the original Zone string. An IdealLoads-only dispatch validator checks
typed system-dispatch prerequisites and returns Rust issues, not this
thermostat-to-equipment predicate or caller diagnostics.

CP225 remains required `source_mapped` and adds no algorithm-level
`energyplus_source` entry, Rust target, code, mapped state, test, support,
capability, output implementation, comparator, manifest, numerical,
performance, or conformance promotion. The inventory becomes 32 algorithms
and 231 routines, split 58 `state_mapped` plus 173 `source_mapped`, with 108
required; the heat-balance project list becomes 77.

CP226 adds required `detect_oscillating_zone_temp` immediately after
`verify_controlled_zone_for_thermostat` and before
`update_final_surface_heat_balance`. Its EnergyPlus boundary is the nonmember
`DetectOscillatingZoneTemp(EnergyPlusData &state)`, declared at
`ZoneTempPredictorCorrector.hh` line 354 and implemented at
`ZoneTempPredictorCorrector.cc` lines 5715-5861.

The first reached call allocates and zero-seeds a four-by-Zone temperature
history plus three per-Zone duration arrays. It registers three System/Sum
hour outputs for every Zone and three matching Facility-any-Zone outputs,
then queries the six variable names and permanently latches calculation on
when any is requested. A valid `PerformancePrecisionTradeoffs` object also
sets the same calculation flag before this setup. Registration and allocation
still occur when calculation remains disabled, while a request added after
successful setup is never rescanned.

When enabled, every call shifts each Zone's four samples newest-first from
the current `ZT`. Oscillation requires the strict difference sequence
`> +0.15 C`, `< -0.15 C`, `> +0.15 C`, or the exact opposite signs.
Equality, NaN comparisons, and nonalternating swings fail. There is no valid
sample count, timestep normalization, or history-mode gate, so the
zero-seeded fourth slot can participate on the third enabled call.

An oscillating Zone receives `TimeStepSys` hours; its occupancy duration
requires an allocated ASH55 record whose Zone slot is occupied, and its
deadband duration requires `CurDeadBandOrSetback`. The three Facility values
are each `TimeStepSys` when any Zone qualifies, not a sum across Zones, and
are added once to three state-lifetime annual/perflog scalars. Occupancy and
deadband classifications are independent and may overlap.

The sole production call is `HVACManager::ManageHVAC` line 431, once after
Zone averaging for every accepted system timestep that reaches it and before
system reporting. It has no warmup, sizing, kickoff, output-reporting, or
environment gate; shortened timesteps therefore contribute separately, while
the external-HVAC route bypasses it. A stopped or earlier-failed loop skips
the call.

Setup clears its latch only after normal completion. Failure can retain
allocated arrays or partial registrations for a retry. Failure during the
Zone loop can retain a shifted prefix and stale Facility/annual values.
Normal duplicate enabled calls are non-idempotent because they shift again and
can add duration again. No day, environment, or annual reset exists; only the
owner's placement-new `clear_state()` restores empty arrays, zero scalars,
setup true, and calculation false.

No C++ test directly or indirectly names CP226 state or outputs. Of 57 active
full-simulation expressions, one expected fatal stops before `ManageHVAC`;
the other 56 reach first setup, including one zero-Zone configuration. Their
static topology registers 243 Zone plus 168 Facility variables, 411 total,
and allocates 324 history slots plus three 81-entry result arrays. None of
those enclosing tests requests any of the six variables, the oscillation
monthly report, or a performance-tradeoff object, so all 56 leave calculation
disabled and provide no threshold, history, occupancy, deadband, Facility, or
annual execution evidence.

Rust has adjacent current MAT and separate three-slot Zone/system histories,
a `0.3 C` adaptive step-count path, typed People and IdealLoads-only occupancy
inputs, IdealLoads-local deadband modes, hourly MAT/debug reporting, and broad
predictor/corrector planning metadata. It has no CP226 helper or HVAC-manager
caller, independent zero-seeded four-sample system-timestep history, strict
alternating `0.15 C` predicate, setup/request latch, six System/Sum output-name families,
ASH55/deadband classification, Facility-any/annual/perflog state, or focused
parity test.

CP226 remains required `source_mapped` and adds no algorithm-level
`energyplus_source` entry, Rust target, code, mapped state, test, support,
capability, output implementation, comparator, manifest, numerical,
performance, or conformance promotion. The inventory becomes 32 algorithms
and 232 routines, split 58 `state_mapped` plus 174 `source_mapped`, with 109
required; the heat-balance project list becomes 78.

CP227 adds required `adjust_air_set_points_for_op_temp_cntrl` immediately
after `detect_oscillating_zone_temp` and before
`update_final_surface_heat_balance`. Its EnergyPlus boundary is the nonmember
`AdjustAirSetPointsforOpTempCntrl(EnergyPlusData &state,
int TempControlledZoneID, int ActualZoneNum, Real64 &ZoneAirSetPoint)`,
declared at `ZoneTempPredictorCorrector.hh` line 356 and implemented at
`ZoneTempPredictorCorrector.cc` lines 5863-5897.

A false global `AnyOpTempControl` returns before either identity is accessed.
Otherwise CP227 reads the selected temperature-control record and returns only
for exact `OpTempCtrl::None`. Scheduled mode samples
`opTempRadiativeFractionSched`; every other enum uses
`FixedRadiativeFraction`. It then copies the selected actual Zone's MRT and
overwrites the referenced setpoint with
`(setpoint - fraction * MRT) / (1 - fraction)`.

Normal input sets the global flag when any
`ZoneControl:Thermostat:OperativeTemperature` exists and validates constant or
scheduled fractions to `[0.0, 0.9)`, including schedule existence and range.
CP227 itself repeats none of those checks. The record default is instead
`OpTempCtrl::Invalid` with fixed fraction zero. Parsed None is assigned only
on an invalid-input path that cannot complete normally; when another Zone
activates the global flag, an untargeted record
therefore takes the fixed-zero calculation rather than its per-record return.
That is an identity for finite MRT, but `0 * Inf` or `0 * NaN` still propagates
NaN. Direct or corrupted state can also expose zero denominators and other
nonfinite results.

All five production expressions are inside `CalcZoneAirTempSetPoints`.
SingleHeat, SingleCool, and SingleHeatCool use one call, while DualHeatCool
converts cooling then heating and Uncontrolled uses none. Cooling paths apply
adaptive comfort before CP227. Optimum start can later replace the converted
high/low values with raw occupied schedules; humidity overcooling, thermostat
fault offsets, comfort control, and EMS overrides can also modify or replace
the intermediate result. SingleHeat, SingleCool, and Dual branches separately
preserve their raw targets before CP227; SingleHeatCool does not write those
record fields.

The normal `ManageHVAC` path reaches the parent once per Zone timestep before
the system-substep loop, so shortening alone does not repeat CP227. Demand
resimulation can add same-time `GetZoneSetPoints` passes; a full parent replay
reloads raw schedules before converting and therefore does not compound a
stable result. Directly calling CP227 twice on the already converted reference
is generally non-idempotent. The external-HVAC route bypasses the built-in
caller.

CP227 owns no persistent state, output registration, diagnostic, clamp,
status, catch, or rollback. Invalid record or MRT indexes, or a null scheduled pointer, can fail
before the sole final assignment and leave the referenced setpoint unchanged.
A later-Zone failure in the parent retains earlier converted outputs. Full
parent retry reloads its schedule prefix; direct routine retry after a normal
write transforms the result again. There is no independent reset lifecycle.

No C++ test calls CP227 by name. Four fixtures make 21 direct parent calls and
46 CP227 entries, all returning at the false global gate; their 33 thermostat
assertions do not exercise fixed, scheduled, MRT, or formula behavior. The
only four raw operative-temperature objects in the unit tree are Constant
with zero fraction in an adaptive-comfort fixture that calls neither the
operative parser nor CP227. There is no scheduled fixture.

None of 57 active full-simulation expressions contains a
`ZoneControl:Thermostat:OperativeTemperature` object. Of the 56 configurations
reaching normal HVAC setup, 38
contain 52 ordinary thermostat records: 49 Dual-only plus three
SingleHeat/SingleCool-switching records. One active setpoint sweep therefore
has 101 static CP227 entry opportunities, all global-gate no-ops; the
record-specific modes, MRT read, and transformation have zero corpus reach.
Runtime entry totals remain timestep-, schedule-, warmup-, and
resimulation-dependent.

Rust retains a bounded direct-Zone DualSetpoint graph and raw-schedule
IdealLoads evidence, but no typed operative-temperature object, global or
per-record operative-control state, fixed/scheduled MRT-fraction binding,
Zone MRT, in-place conversion, exact caller lifecycle, or focused test. Its
setpoint compatibility wrapper is an identity around an empty PredictStep
closure, and its thermostat execution step is planning metadata. Raw
`ZoneControl:Thermostat:OperativeTemperature` is outside typed/capability
coverage and run-blocks.

CP227 remains required `source_mapped` and adds no algorithm-level
`energyplus_source` entry, Rust target, code, mapped state, test, support,
capability, output implementation, comparator, manifest, numerical,
performance, or conformance promotion. The inventory becomes 32 algorithms
and 233 routines, split 58 `state_mapped` plus 175 `source_mapped`, with 110
required; the heat-balance project list becomes 79.

CP228 adds required `adjust_operative_set_points_for_adap_comfort`
immediately after `adjust_air_set_points_for_op_temp_cntrl` and before
`update_final_surface_heat_balance`. Its EnergyPlus boundary is the nonmember
`AdjustOperativeSetPointsforAdapComfort(EnergyPlusData &state,
int TempControlledZoneID, Real64 &ZoneAirSetPoint)`, declared at
`ZoneTempPredictorCorrector.hh` line 358 and implemented at
`ZoneTempPredictorCorrector.cc` lines 5899-5964.

CP228 indexes the selected temperature-control record, aliases the shared daily
adaptive schedules, truncates the incoming `Real64` setpoint toward zero into
an `int`, and copies the adaptive model index before checking
`AdaptiveComfortTempControl`. A false flag therefore leaves a finite,
int-representable reference unchanged but does not protect invalid record
state or an undefined nonfinite/out-of-range floating-to-integer conversion.
The routine owns no state write other than the referenced setpoint.

Every environment except exact `DesignDay` and `HVACSizeDesignDay` selects one
of seven one-based daily arrays by model index 2-8 and `DayOfYear`: ASH55
central, 90-percent upper, and 80-percent upper, followed by CEN15251 central
and categories I-III. An unrecognized or corrupted integer index takes the
switch default and makes no candidate write; unknown input model text instead
accumulates a Severe error and reaches CP196's fatal tail before normal
runtime. The two design-day kinds resolve the current `DesignDayNum`; only
`DayType == 9` reads the shared seven-slot summer vector at `model_index - 2`.
CP228 checks neither model bounds nor schedule initialization/allocation,
`DayOfYear`, `Envrn`, or `DesignDayNum` bounds, nor whether the shared vector
was populated from the current design day.

After selection, CP228 first replaces a candidate lower than the truncated
baseline with that integer and then separately assigns the same integer when
the current value is exact `-1`; the result can remain `-1` when that integer
is also `-1`. This is not `max(candidate, original Real64)`. For example,
baseline 26.8 and candidate 26.5 yields 26.5, while candidate 25 or `-1`
yields 26.0. A no-candidate path normally preserves a positive baseline but
can raise a negative fractional baseline to its truncated integer. Candidate
NaN survives both comparisons, positive infinity survives, and negative
infinity falls back; a nonfinite or non-representable original has already
entered undefined conversion territory.

CP196 creates adaptive state only for a Constant or Scheduled
`ZoneControl:Thermostat:OperativeTemperature` record with a nonblank,
recognized, non-None model. The record defaults are false and index zero, and
the parser only sets active state; a later repeated target with None or blank
does not explicitly clear an earlier selection. The first active record calls
CP197 and CP198 under the shared initialized latch. CP198 provides strict
ASH `10 < T < 33.5` and CEN `10 < T < 30` daily candidates or `-1`, plus a
single model-indexed summer-design-day vector shared across all design days.
That vector starts as `[-1,0,0,0,0,0,0]`, retains earlier values when a later
day is invalid, and lets the last qualifying day win independently per
family. CP228 only consumes this committed state.

The three production expressions are all in `CalcZoneAirTempSetPoints` and
are additionally guarded by the same adaptive flag. SingleCool and
SingleHeatCool adjust their cooling/shared target; DualHeatCool adjusts only
the cooling high target, never the heating low target. Each caller saves
`setptAdapComfortCool` immediately after CP228 and before CP227 converts the
operative target to an MRT-weighted air target. SingleCool and Dual preserve
their raw high targets first; SingleHeatCool does not preserve raw low/high
record fields.

`setptAdapComfortCool` backs the Zone-timestep
`Zone Adaptive Comfort Operative Temperature Set Point` output. It is reset
to zero at begin-environment, but a later Uncontrolled or SingleHeat
control-type branch does not refresh it, so a prior adaptive value can remain
visible. A false adaptive flag likewise skips refresh in direct or malformed
state. CP227, optimum start, humidity overcooling, thermostat faults,
comfort control, and EMS do not revise that snapshot.

The built-in parent runs once per Zone timestep before system substeps, with
additional same-time passes possible through demand resimulation. A full
parent replay reloads the raw thermostat schedule before CP228; ordinary
positive finite candidates or `-1` therefore do not accumulate. Direct
malformed replay can still expose truncation changes or undefined conversion.
External HVAC bypasses the caller. CP228 has no warmup, sizing, kickoff,
occupancy, window-opening, or current-zone-condition gate beyond its
environment and per-record branches.

There is no authored diagnostic, clamp, status, catch, cleanup, or rollback.
Invalid record, environment, design-day, daily-array, or day state can fail
before a candidate assignment, as can an invalid model index specifically in
the summer design-day vector branch, and leave the reference unchanged; the
initial integer conversion can already be undefined. A later
Zone failure retains earlier parent outputs. CP228 has no independent reset;
the daily/design state resets only with full predictor/corrector owner
reconstruction, while the reported snapshot has the parent begin-environment
reset.

One C++ fixture calls CP228 directly four times, all in a run-period
environment with manually allocated active records. It proves ASH central
`0 -> 25.55`, CEN central `0 -> 27.05`, and two `-1` candidates restored to 0
and 26 by the earlier lower-than-integer-baseline test. The later exact
`== -1` true branch is therefore not isolated, and the fourth call still does
not test a valid candidate below the baseline. The fixture's four raw
operative objects are not parser or production integration evidence: it calls
neither CP196 nor the parent setpoint routine, then freshly allocates and
manually activates all four control records.

Four other fixtures make 21 direct parent calls and evaluate the outer
adaptive guard 26 times, but every flag is default false and CP228 is entered
zero times. None of 57 active full simulations contains an
operative-temperature object; one expected EMS fatal stops before setpoint
acquisition and the other 56 retain false adaptive flags. Thus the full corpus
has zero
CP228 entries and zero selector-body reach. Five upper-model selections, the
false internal guard, default switch, both design-day kinds, integer
truncation, independent exact-sentinel comparison, caller snapshot/order,
failure, retry, and reset remain
uncovered.

Rust has no exact routine, adaptive flag/model, seven daily arrays, summer
design-day vector, environment dispatch, truncating selector, adaptive output,
or live caller. Its thermostat type and compiler cover only bounded
direct-Zone DualSetpoint controls, the only live setpoint-wrapper call passes
an empty closure, and the narrow IdealLoads diagnostic repeats only a
referenced `Schedule:Constant` value. The operative-temperature object is
RawOnly without a partial rule, becomes an unsupported object, and run-blocks.
A day-of-year field in the Rust time axis does not provide the missing
design-day environments or adaptive state, and no Rust test exercises this
behavior.

CP228 remains required `source_mapped` and adds no algorithm-level
`energyplus_source` entry, Rust target, code, mapped state, test, support,
capability, output implementation, comparator, manifest, numerical,
performance, or conformance promotion. The inventory becomes 32 algorithms
and 234 routines, split 58 `state_mapped` plus 176 `source_mapped`, with 111
required; the heat-balance project list becomes 80.

CP229 adds required `calc_zone_air_comfort_set_points` immediately after
`adjust_operative_set_points_for_adap_comfort` and before
`update_final_surface_heat_balance`. Its EnergyPlus boundary is
`void CalcZoneAirComfortSetPoints(EnergyPlusData &state)`, declared at
`ZoneTempPredictorCorrector.hh` line 360 and implemented at
`ZoneTempPredictorCorrector.cc` lines 5966-6329.

The routine initializes low/high/shared temperature locals and
`ObjectCount`/`PeopleCount` once per invocation, not per comfort Zone. On its
first entry it calls `ManageThermalComfort(state, true)` and clears its own
latch only after normal return. That child independently initializes People
comfort state and outputs and updates its six-AM temperature scratch before
the InitializeOnly return. CP229 then resets the complete
`ComfortControlType` array to Uncontrolled, but not its report or Fanger
arrays, and visits comfort records in stored order.

Each record casts the current control schedule value to `SetptType`, reports
the integer, and samples PMV schedules. Uncontrolled writes both PMVs to
`-999` without clearing `FangerType`; SingleHeat, SingleCool, and
SingleHeatCool write their branch-specific type and PMVs; Dual samples both
and, when low exceeds high, advances persistent first/recurring diagnostics
before forcing low equal to high. Invalid control values emit one Severe,
retain stale PMV state, continue through averaging, and emit another Severe
at final assignment.

NO and SPE are identical, use `SpecificObjectNum`, select HighPMV only for
SingleCool, and make a second child call for Dual. OBJ and PEO scan all People
in the actual Zone, but neither has that SingleCool special case: both pass
the `LowPMV = -999` sentinel and normally drive cooling toward the minimum
dry-bulb bound. OBJ divides Zone-local sums by the function-scope cumulative
`ObjectCount`. PEO converts each design-count-times-schedule product to an
`int`, accumulates that integer in the non-reset `PeopleCount`, and weights
each child result even though zero occupants still trigger a child call.

When cumulative PeopleCount is nonpositive, PEO warns and repeats the Zone as
an object average using the same non-reset ObjectCount. A prior occupied Zone
can instead suppress that fallback and divide the current zero numerator by a
prior denominator. Thus later Zones are order-dependent. Invalid averaging
state can consume stale low/high locals. With ordered or equal endpoint PMVs,
CP230 can leave an output reference untouched when PMV equals a bound, so
shared `Tset` or a setpoint can retain zero or an earlier People/Zone result;
reversed endpoint equality can instead select the opposite temperature bound.

The final branch preserves ordinary control for Uncontrolled while clearing
only the unused opposite bound for ordinary SingleHeat or SingleCool.
Active SingleHeat and SingleCool clamp only one side, write scalar plus their
active bound, and leave the opposite bound stale. SingleHeatCool clamps before
testing its range warning, making that warning unreachable for ordinary
finite ordered bounds, then writes scalar and both bounds. Dual clamps only
low-below-min and high-above-max, never enforces final low less than or equal
to high, writes both bounds, and leaves scalar setpoint stale. Its high
recurring warning mistakenly records SetPointLo twice. Every active branch
overwrites ordinary `TempControlType` and its report.

CP196 normally requires at least one People object, validates dry-bulb bounds,
control schedule extrema `[0,4]`, and PMV schedule extrema `[-3,3]`, and
fatals on accumulated errors. It forces NO when a Zone has one People object;
SPE with multiple People resolves a global name without checking Zone
membership. Equal bounds emit a Severe in the shown input branch without
setting its local error flag, and fractional control schedule samples are not
rejected by the range check.

The sole production call is inside `CalcZoneAirTempSetPoints` after ordinary
setpoint selection, CP228/CP227, optimum-start, humidity, and thermostat-fault
work. Comfort control can overwrite those results; the following unconditional
EMS override has final precedence. CP199 registers Zone comfort control type
and low/high Fanger PMV outputs. Normal built-in execution is once per Zone
timestep before system substeps, with extra same-time demand resimulation
passes possible; external HVAC bypasses the caller.

There is no status, catch, cleanup, transaction, or rollback. First-use child
failure retains the CP229 latch, while later failure occurs after the latch
clear and whole-array control reset. Earlier Zones and transitive
Fanger/People writes survive; unvisited types are Uncontrolled while report
and PMV fields can be stale. Retry recreates local counts but retains warning,
report, setpoint, and child state. A clean reset spans predictor/corrector,
HeatBalFanSys, Zone-controls, ThermalComfort, People/schedule, output, and
environment owners.

No C++ test calls CP229 or CP230 directly. Twenty-one direct parent calls all
have zero comfort Zones. Two raw comfort-input fixtures stop in input or
Surface setup, and a comfort-count EMS fixture calls only the final override.
None of 57 active full simulations contains comfort input. Separate Fanger
tests cover only the forward model. The installed
`FurnaceWithDXSystemComfortControl.idf` is an unadopted oracle candidate; its
comfort-controlled EAST Zone has one People record, making the runtime
averaging method NO.

Rust has no comfort thermostat/setpoint types, PMV/Fanger state, inverse
child, averaging, comfort bounds, outputs, diagnostics, or live caller. Its
People type supports only count/schedule uses for sizing and bounded
IdealLoads ventilation, while its thermostat type covers ordinary
DualSetpoint only. Comfort objects remain RawOnly without a partial rule and
run-block. CP229 remains required `source_mapped` and adds no Rust, state,
support, output, numerical, performance, or conformance promotion. The
inventory becomes 32 algorithms and 235 routines, split 58 `state_mapped`
plus 177 `source_mapped`, with 112 required; the heat-balance project list
becomes 81.

CP230 adds required `get_comfort_set_points` immediately after
`calc_zone_air_comfort_set_points` and before
`update_final_surface_heat_balance`. Its EnergyPlus boundary is
`GetComfortSetPoints(EnergyPlusData &, int PeopleNum, int ComfortControlNum,
Real64 PMVSet, Real64 &Tset)`, declared at
`ZoneTempPredictorCorrector.hh` lines 362-367 and implemented at
`ZoneTempPredictorCorrector.cc` lines 6331-6415.

The routine snapshots the selected comfort record's minimum and maximum
dry-bulb bounds, then unconditionally evaluates Fanger at the minimum followed
by the maximum. Only strict `PMVMin < PMVSet < PMVMax` enters root solving.
A target below or above writes the corresponding temperature bound. With
ordered or equal endpoint PMVs, exact endpoint equality leaves the output
untouched; a NaN target or any case in which all three comparisons fail does
the same. Reversed endpoint equality can instead select the opposite
temperature bound. Bounds and endpoint PMVs are neither validated nor
reordered.
Consequently a lower clamp returns the minimum while transitive report state
still reflects the last maximum-temperature evaluation, and equal/reversed
or nonfinite endpoints follow literal branch order.

The interior branch calls configurable `General::SolveRoot` with a
`PMVSet - PMV(candidate)` callback, absolute PMV tolerance 0.001, and maximum
500. The solver reads global root-algorithm state and repeats both endpoint
evaluations before generating candidates. Its strict post-evaluation limit
permits a 501st candidate to converge; an iterated `-1` retains that last
candidate, while a pre-candidate narrow-width `-1` retains the seeded minimum.
Same-sign `-2` also returns the minimum, although stable finite ascending
endpoint results make that flag normally inconsistent with the
preceding strict bracket. A clamp/no-write path therefore makes two Fanger
calls, a `-2` or zero-candidate width failure makes four, and a normal-width
interior path makes five through 505.

Fanger evaluation is impure. Every endpoint and candidate can resample
People activity, work, clothing, and air velocity; read Zone humidity,
pressure, MRT, radiant-person and room-air state; and overwrite shared
ThermalComfort scratch, People temperature/RH, PMV/PPD, MRT, operative and
clothing reports. Nonmixing room-air modes can replace the candidate
temperature and flatten the residual. Child air-velocity diagnostics can
advance once per trial, including duplicated endpoints and warmup. An
unmatched People identity leaves the freshly zeroed PMV result at both
endpoints rather than indexing the requested record. SurfaceWeighted MRT
first-use also updates its first-use/error latches and initializes Surface
`AE`/`enclAESum`; with
a bad sum, its first trial warns and returns the Space-MAT/surface average
while later trials return zero, so repeated endpoints can invalidate the
initial bracket and make `-2` reachable.

Outside warmup, solver `-1` and `-2` advance separate first/recurring
ZoneTempPredictorCorrector counter/index pairs. They are global across all
comfort Zones and People and have no environment reset; recurring values use
the returned temperature twice. Warmup still solves and applies the result
but suppresses only CP230's two diagnostic families, not transitive Fanger
warnings or state writes. CP230 owns no output registration.

All 12 production expressions are inside CP229. NO/SPE make one call, or two
for Dual; OBJ and positive PEO make `N` or `2N`; nonpositive PEO plus fallback
makes `2N` or `4N`. Uncontrolled and malformed controls can still enter the
child through an averaging branch. With ordered or equal endpoint PMVs,
equality can carry a prior shared temperature into a later People/Zone sum;
reversed equality can instead select the opposite bound. Normal built-in
cadence is the
parent's Zone-timestep call before system substeps, with possible
demand-resimulation repeats and external-HVAC bypass.

There is no status, catch, transaction, rollback, or latch. Failure can retain
partial Fanger, People, report, radiant, diagnostic, and output-reference
state; retry repeats all reached trials and can advance warnings. Clean replay
also spans RootFinding, ThermalComfort, People/HeatBalance, HeatBalFanSys,
Surface, Construction, ViewFactor, HeatBalSurf, Zone-controls, RoomAir,
schedules, environment/psychrometric, output, and diagnostic owners.

No C++ test calls or reaches CP230. Twenty-one direct setpoint-parent calls
skip comfort, two raw comfort fixtures stop in setup, and all 57 active full
simulations have zero comfort input and zero CP230 reach. Separate tests make
six direct forward-Fanger calls, but the sole call supplying optional
`PeopleNum` does not assert its PMV result; ten generic SolveRoot calls cover
successes and `-1` without a Fanger composition or `-2` assertion.

A manual stock-26.1 run of
`FurnaceWithDXSystemComfortControl.idf` completed with no Warning or Severe.
Its comfort-controlled EAST Zone has one People record and therefore forces
NO averaging. Schedule/cadence arithmetic over two DesignDays, each with six
warmup days and one reported day at six
timesteps per hour, gives 2,226 CP230 calls; the 318 reported-day calls include
168 sentinel and 150 active-target calls. This is an uninstrumented manual
oracle diagnostic, not a checked-in repository case, Rust comparison, or
conformance claim.

Rust has no comfort types, PMV/Fanger state, inverse callback, configurable
generic solver, diagnostic state, output effects, or live caller. Its narrow
People fields and ordinary DualSetpoint graph do not cover the boundary, and
an unrelated private IdealLoads psychrometric bisection has different
semantics. Raw comfort objects still run-block. CP230 remains required
`source_mapped` and adds no Rust, state, support, output, numerical,
performance, or conformance promotion. The inventory becomes 32 algorithms
and 236 routines, split 58 `state_mapped` plus 178 `source_mapped`, with 113
required; the heat-balance project list becomes 82.

CP231 adds required `adjust_cooling_set_point_for_temp_and_humidity_control`
immediately after `get_comfort_set_points` and before
`update_final_surface_heat_balance`. Its EnergyPlus boundary is
`AdjustCoolingSetPointforTempAndHumidityControl(EnergyPlusData &,
int TempControlledZoneID, int ActualZoneNum)`, declared at
`ZoneTempPredictorCorrector.hh` lines 369-372 and implemented at
`ZoneTempPredictorCorrector.cc` lines 6417-6458.

The routine aliases the selected `TempControlledZone` and the actual Zone's
thermostat-setpoint record before testing either guard. A false global
`AnyZoneTempAndHumidityControl` therefore does not protect bad indexes. It
then returns on that false flag or on exact per-record `OvercoolCtrl == None`.
The record default is `Invalid`, which follows the constant-range path rather
than returning. There is no assertion that both indexes identify the same
Zone, so mode, range, ratio, and schedules from one record can be combined
with another Zone's setpoints and relative humidity.

After the guards, Scheduled mode samples its range schedule; every other mode
uses the stored constant. The code copies the percent-RH-per-kelvin ratio,
caps the range by `setptHi - setptLo` only when that gap is strictly positive,
then always samples the independent dehumidifying schedule and subtracts it
from the Zone `airRelHum`. Only strict positive humidity excess and ratio
enter the final cap `min(range, excess / ratio)`, followed by the routine's
sole write, `setptHi -= range`. It does not change scalar `setpt`, `setptLo`,
the thermostat record's raw setpoint snapshots, an output registration,
status, or diagnostics.

The positive-gap rule is applied to SingleCool as well as Dual despite the
source comment. SingleCool has just refreshed scalar and high but not low, so
a stale or default low participates in its cap. A zero, reversed, NaN, or
negative-infinite gap skips the cap. The ObjexxFCL double `min` chooses its
second argument on equality or an unordered comparison, which makes NaN
behavior argument-order-dependent. The strict humidity and ratio gates reject
zero, negative, and NaN values but accept positive infinity. Malformed
negative range raises the cooling setpoint, and a zero range still reaches a
zero-subtraction assignment when both gates pass.

CP196 sets the global flag from raw object count before validating any
`ZoneControl:Thermostat:TemperatureAndHumidity` record, and the flag persists
until the DataZoneControls owner is cleared. In the ordinary thermostat-object
branch, Constant input can bind the dehumidifying schedule, range, and ratio.
The Scheduled branch asks for alpha slot 6 although the schema's range
schedule is A5. The documented selective-Zone fallback instead parses A3
`Overcool` against the None/Constant/Scheduled enum, making valid Overcool
invalid. Its Severe incorrectly reports A4's field and value instead of A3,
and a malformed Scheduled value can leave the ratio at its default zero.
Those pinned producer anomalies remain source behavior, not Rust validation
guidance.

A second lifecycle anomaly follows from the global flag. Untargeted
temperature-control records retain default `Invalid` and null schedule
pointers. Once any temperature-and-humidity object raises the flag, a
SingleCool or Dual record without its own modifier can pass the exact-None
guard, choose constant zero, and dereference its null dehumidifying schedule.
Even a globally present object configured as None does not prove all other
records are safe. The Constant producer path validates range within `[0,3]`
inclusive. A Scheduled branch that obtains a schedule pointer attempts the
same all-values check, while ratio has only a zero lower bound. CP231 does not
revalidate live schedule values, RH, finite values, or pointers.

Its only two production expressions are in CP204
`CalcZoneAirTempSetPoints`. SingleCool orders raw cooling schedule, adaptive
selection, operative conversion, copy of scalar to high, then CP231. Dual
orders cooling and heating schedule work plus operative conversion, lets
optimum start replace both bounds, then calls CP231 once. SingleHeatCool never
calls it. The ordinary thermostat-fault offset follows, CP229 comfort control
can overwrite ordinary results, and CP232 EMS override has final setpoint
precedence inside the parent.

The scalar/high asymmetry is observable: ordinary SingleCool load prediction
uses scalar `setpt`, so CP231's high-only reduction does not drive that load,
whereas Dual prediction consumes `setptHi`. The already registered
`Zone Thermostat Cooling Setpoint Temperature` output is backed by high, but
CP231 registers nothing itself. In the later prediction phase, any positive
cutout-difference control rebuilds SingleCool or Dual high from the earlier
raw thermostat snapshot, potentially discarding CP231 before load prediction
and reporting.

Built-in cadence is once per matching SingleCool or Dual record during the
Zone-timestep setpoint sweep before system substeps. Demand-manager HVAC
resimulation can repeat the whole parent at the same simulation time, while
the external-HVAC path bypasses it. CP231 has no local warmup, sizing,
kickoff, environment, occupancy, or control-availability guard.

Every potentially failing lookup or schedule read precedes the sole write, so
a non-return before line 6456 leaves no CP231-owned partial mutation while
retaining all earlier parent work. There is no catch, rollback, latch, or
retry status. A direct duplicate successful call is generally
non-idempotent: it subtracts again from the already reduced high, and if the
first call reaches low exactly, the next zero gap skips the protective cap
and can cross below low. A full-parent retry normally reloads SingleCool
scalar/high or Dual high/low but repeats schedules and all earlier/later
modifiers. Clean replay spans
DataZoneControls records and flag, HeatBalFanSys setpoints,
ZoneTempPredictorCorrector RH state, schedules, optimum-start, fault, comfort,
EMS, and environment owners. Begin-environment initialization zeroes the
live thermostat setpoints and Zone RH, but it does not clear the
DataZoneControls global flag, mode, range, ratio, or schedule pointers.

No C++ unit test calls CP231 or contains the exact temperature-and-humidity
object. Four fixtures make 21 direct parent calls and produce 20 indirect
CP231 expression entries: three optimum-start Dual entries, seven reporting
entries, and five entries in each of two cutout fixtures. Every one returns
at the false global guard, and no assertion isolates range, RH, ratio, or the
high-only mutation.

Of 57 active full-simulation expressions, one expected EMS fatal stops before
setpoint acquisition and 56 reach it. Thirty-eight configurations contain 52
ordinary thermostat records: 49 Dual and three SingleHeat/SingleCool
schedule-switching records. Aggregating one reached sweep per configuration
yields 49 guaranteed CP231 call-expression entries and at most 52, not a fixed
runtime total across
warmup and resimulation. None of the 57 inputs has the temperature-and-humidity
modifier, so all entries return globally and active-body corpus reach is zero.

The installed
`AirflowNetwork_MultiZone_House_OvercoolDehumid.idf` is the sole exact
ExampleFile candidate. It uses one Dual thermostat, a 45 percent RH schedule,
constant 1.7 K range, and 3 percent/K ratio. Upstream
`testfiles/CMakeLists.txt` registers it as a Miami-EPW
`add_simulation_test`, so it is full-model Constant-path regression evidence,
but it does not request the adjusted cooling-setpoint output, directly assert
CP231, or cover Scheduled mode. This repository adopts neither its input nor
a script. The 85 separate Humidistat ExampleFiles do not activate CP231 and
are not evidence for it.

Rust has no typed temperature-and-humidity thermostat modifier, overcool
mode/range/schedule/ratio, percent-RH setpoint input, mutable scalar/low/high
setpoint record, exact helper, or live caller. Its thermostat graph is
bounded to ordinary direct-Zone DualSetpoint state, and the compatibility
setpoint wrapper still receives an empty live closure. The raw modifier has
no partial-support rule and run-blocks.

Typed `ZoneControl:Humidistat`, psychrometric RH primitives, and IdealLoads
moisture-demand logic are separate adjacent paths. They neither lower an
ordinary cooling `setptHi` nor reproduce CP231's guards, caps, producer bugs,
caller order, or lifecycle. CP231 therefore adds no algorithm-level
`energyplus_source` entry, Rust target, code, mapped state, test, support,
capability, output implementation, comparator, case, manifest, numerical,
performance, or conformance promotion. The inventory becomes 32 algorithms
and 237 routines, split 58 `state_mapped` plus 179 `source_mapped`, with 114
required; the heat-balance project list becomes 83.

CP232 adds required `override_air_set_points_for_ems_cntrl` immediately after
`adjust_cooling_set_point_for_temp_and_humidity_control` and before
`update_final_surface_heat_balance`. Its EnergyPlus boundary is
`OverrideAirSetPointsforEMSCntrl(EnergyPlusData &)`, declared at
`ZoneTempPredictorCorrector.hh` line 374 and implemented at
`ZoneTempPredictorCorrector.cc` lines 6460-6555.

The routine traverses all ordinary temperature-control records in ascending
order, then all comfort-control records. Each record is aliased before its two
flags are tested; heating is always processed before the independent cooling
flag. An active flag reads `ActualZoneNum` and aliases the live thermostat
setpoint before switching on `TempControlType` for ordinary records or
`ComfortControlType` for comfort records. Bad counts or indexes can therefore
fail even when an eventual type would make no write. There is no identity,
duplicate-target, range, finite, unit, deadband, or monotonicity validation.

SingleHeat heating and SingleCool cooling copy the value to scalar plus their
matching low/high bound. SingleHeatCool heating writes scalar/low before
cooling writes scalar/high, so both flags leave scalar=cooling, low=heating,
and high=cooling. Dual writes only low and high and preserves scalar. Opposite
single-mode flags and unsupported types silently do nothing. Chained
assignments write scalar before the bound. NaN, infinity, signed zero, reversed
deadband, and arbitrary values are copied literally without diagnostics.

CP229 comfort processing precedes CP232 and can replace live ordinary
`TempControlType`. The ordinary EMS loop therefore dispatches on that final
live type rather than necessarily the record's authored family. Multiple
records targeting one Zone resolve field-by-field in record order, and the
later comfort loop has final precedence for overlapping fields. Unwritten
fields can survive from earlier records, producing a mixed triple.

`EMSManager::SetupThermostatActuators` registers ordinary controls under
`Zone Temperature Control` with `[C]` units and comfort controls under
`Zone Comfort Control` with `[]` units. Each unique actuator key binds directly
to its record's boolean/value fields; uppercase duplicate keys are suppressed
and retain the first binding even though CP232 still visits every record. The
comfort dimensionless registration is copied without PMV inversion or unit
conversion into Celsius-backed live setpoints; that mismatch is pinned source
behavior. CP232 neither runs EMS nor checks the global EMS-present flag.

The sole production call is the unconditional final action of
`CalcZoneAirTempSetPoints`, after ordinary schedules and modifiers, fault
offsets, and optional comfort calculation. The normal HVAC path runs the
`BeginTimestepBeforePredictor` EMS calling point earlier in the same setup,
before the setpoint sweep. Demand resimulation can repeat the sweep without rerunning
that calling point, while external HVAC bypasses it.

The existing heating/cooling thermostat outputs expose low/high. SingleHeat,
SingleCool, and SingleHeatCool load prediction consume scalar; Dual consumes
low/high. Thus cooling wins a both-active SingleHeatCool load even though both
bounds remain visible. A later staged-control update can replace low/high, and
positive cutout control rebuilds SingleHeat, SingleCool, or Dual fields from
pre-modifier ordinary snapshots, potentially erasing CP232 before load
calculation and output sampling. CP232 registers no output or status.

Failure after earlier records leaves prefix writes. There is no catch,
transaction, rollback, cleanup, or latch. Exact unchanged duplicate calls are
overwrite-idempotent, while changed inputs or partial retries can produce new
mixed results. Constructors default both flags false and values zero.
Begin-environment RuntimeLanguage initialization clears used actuator
flags/values, and setpoint initialization separately zeroes the live triple,
but CP232 owns no reset and manually populated/unregistered records are outside
that actuator reset guarantee.

One direct C++ test calls CP232 twice and asserts only ordinary Dual low/high
23/26 followed by comfort Dual low/high 22/25. Twenty-one direct parent calls
visit 35 ordinary records and make 70 false flag checks with zero writes. Of
57 active full-simulation expressions, one expected EMS fatal stops before
setpoint acquisition; a one-sweep census across the other 56 visits 52
ordinary records and makes 104 false flag checks, with zero comfort visits or
active writes. Installed 26.1 testfiles contain no exact actuator-active
ExampleFile for either CP232 component key.

Rust has no actuator registry or EMS engine, override fields, comfort-control
state, mutable setpoint triple, exact helper, or live caller. Its ordinary
typed graph is bounded to direct-Zone DualSetpoint schedules, the compatibility
setpoint wrapper has an empty live closure, and IdealLoads diagnostics sample
raw schedules separately. All `EnergyManagementSystem:*` input run-blocks;
existing EMS stages are execution-plan metadata only. CP232 remains required
`source_mapped` and adds no Rust, state, support, output, numerical,
performance, or conformance promotion. The inventory becomes 32 algorithms
and 238 routines, split 58 `state_mapped` plus 180 `source_mapped`, with 115
required; the heat-balance project list becomes 84.

CP233 adds required `fill_predefined_table_on_thermostat_setpoints`
immediately after `override_air_set_points_for_ems_cntrl` and before
`update_final_surface_heat_balance`. Its EnergyPlus boundary is
`FillPredefinedTableOnThermostatSetpoints(EnergyPlusData &)`, declared at
`ZoneTempPredictorCorrector.hh` line 376 and implemented at
`ZoneTempPredictorCorrector.cc` lines 6558-6672.

The routine reads every authored ordinary thermostat-setpoint definition, not
only definitions referenced by a Zone. It visits SingleHeating,
SingleCooling, SingleHeatingOrCooling, then DualSetpoint in input order. One
invocation-wide vector suppresses every schedule after the first occurrence
of its numeric `Schedule::Num`; the winning occurrence fixes its object name,
heating/cooling role, and winter/summer treatment. Dual processes heating
before cooling, so the heating interpretation wins when both sides share one
schedule. Counts only reserve vector capacity; the allocated arrays determine
iteration.

SingleHeating and Dual heating write winter samples under the base schedule
name; SingleCooling and Dual cooling write summer samples there. Each surviving
normal schedule appends six cells: first setpoint object, assumed month, 11:00
value/count, and 23:00 value/count. SingleHeatingOrCooling instead appends ten
cells: first object and combined months on the base row, with four numeric
cells each on synthetic `<name> (summer)` and `<name> (winter)` rows. An
unused setpoint definition can therefore become `First Object Used`. Normal
input uppercases schedule names while CP233's literal suffixes are lowercase,
so valid parsed names do not collide; only manually constructed or corrupted
mixed-case schedule state can merge with a synthetic row.

`ScheduleDetailed::getValAndCountOnDay` chooses July as summer and January as
winter only for latitude strictly above zero; southern and zero latitude use
the reverse. It derives the first Wednesday from run-period start weekday and
leap-year state, applies that selected date's DST shift once, and reads the
first timestep of hours 11 and 23 without holiday adjustment or averaging.
The count walks all 365/366 Julian dates, tests each date's Wednesday profile
at that same fixed shifted-hour index, short-circuits identical week/day
pointers, and otherwise uses exact floating equality. It counts calendar-day
rules, not actual Wednesdays. Constant schedules ignore weekday/hour and
return their end-of-run `currentVal` with every day; that value can include an
EMS override, while detailed schedules read definition `tsVals` and ignore
EMS actuation.

The sole production call is `OutputReportTabular.cc` line 6998 inside
`FillRemainingPredefinedEntries`, immediately before CP234. Top-level
`WriteTabularReports` reaches it once after the environment loop and before
the later `WriteTabularFiles` guard, so predefined state is populated even
when no table file is emitted. The routine neither loads Zone setpoint input
nor checks report visibility, schedule type, pointer validity, or calendar
shape, and it owns no once latch.

Every real, integer, and string `PreDefTableEntry` call appends rather than
upserting. Failure leaves all earlier cells, and retry creates a fresh local
dedup vector and appends them again. Later duplicate cells win while rendered
tables are assembled, but `RetrievePreDefTableEntry` returns the earliest
match; changed retry state can therefore make the two views disagree.
Multiyear tabular reset does not clear these entries. CP233 owns no
transaction, rollback, cleanup, diagnostic, or reset.

No C++ test calls CP233 or references its six column handles/text. The closest
fixture calls `getValAndCountOnDay` nine times with 21 assertions for
hemisphere, month, value, hour, and matching-day behavior, but it does not
exercise family order, deduplication, row keys, or table mutation. Of 57
active full-simulation expressions, one expected fatal stops before
finalization. A static one-finalization census of the other 56 gives 18 empty
and 38 nonempty calls, 47 setpoint definitions (6 SingleHeating, 6
SingleCooling, 0 SingleHeatingOrCooling, and 35 DualSetpoint), 76 surviving
schedule rows, 152 helper calls, and 456 appended cells. None asserts CP233
output. Installed files provide `5ZoneAirCooled.idf` as a cross-family
deduplication candidate and `TermRhSingleHeatCoolNoDB.idf` as a split-row
candidate, but neither is adopted as focused repository evidence.

Rust retains only adjacent normalized DualSetpoint graph state,
calendar-aware schedule-series evaluation, and constant-schedule IdealLoads
diagnostics. It has no four-family setpoint arena, source numeric schedule-ID
deduplication, seasonal representative query, predefined LEED table store or
column identities, exact helper, tabular caller, or composed test.
`Output:Table:SummaryReports` remains a RawOnly ignored reporting object. The
30 repository thermostat cases are all DualSetpoint and none composes a
summary request with a CP233 comparator. CP233 remains required
`source_mapped` and adds no algorithm-level source, Rust, state, support,
output, numerical, performance, or conformance promotion. The inventory
becomes 32 algorithms and 239 routines, split 58 `state_mapped` plus 181
`source_mapped`, with 116 required; the heat-balance project list becomes 85.

CP234 adds required `fill_predefined_table_on_thermostat_schedules`
immediately after `fill_predefined_table_on_thermostat_setpoints` and before
`update_final_surface_heat_balance`. Its EnergyPlus boundary is
`FillPredefinedTableOnThermostatSchedules(EnergyPlusData &)`, declared at
`ZoneTempPredictorCorrector.hh` line 378 and implemented at
`ZoneTempPredictorCorrector.cc` lines 6674-6766.

The routine visits the materialized ordinary
`TempControlledZone(1..NumTempControlledZones)` arena in stored order and keys
all cells by exact `ZoneName`. It always appends thermostat name and
control-type schedule name, then examines the fixed SingleHeating,
SingleCooling, SingleHeatingOrCooling, and DualSetpoint slots. A slot is
included solely when its control-object name is nonempty; `isUsed` and the
currently selected control type are ignored. Comfort, staged, and humidity
controls are not merged.

For each participating slot CP234 captures the source control-type display
name, control-object name, and applicable schedule names. Dual and
SingleHeatingOrCooling dereference cooling before heating. The local vector
starts with five blank entries, then move-appends each populated indexed
entry. Current libraries leave the moved-from strings empty, but that is only
a valid-unspecified C++ state, so suppression of duplicate remnants is
portability-sensitive.

The complete vector is tuple-sorted by type, control name, heating schedule,
then cooling schedule. Normal multi-type display order is
DualSetPointWithDeadBand, SingleCooling, SingleHeatCool, SingleHeating,
regardless of authored field-set order. Each column independently drops empty
strings and joins with comma-space. Heating therefore omits SingleCooling and
cooling omits SingleHeating; cross-column list positions are not stable
pairings, and repeated names are not deduplicated.

The `Thermostat Schedules` System Summary subtable has six columns:
thermostat, control-type schedule, joined control types, joined control names,
heating schedules, and cooling schedules. The first four cells are always
appended, including empty joined lists; heat and cool cells are appended only
when nonempty. Each Zone therefore contributes four through six cells. The
sole production call is `OutputReportTabular.cc` line 6999 inside
`FillRemainingPredefinedEntries`, immediately after CP233 and before the
later table-file guard.

Every `PreDefTableEntry` is append-only. A null control-type pointer fails
after the thermostat cell, participating schedule nulls fail after the first
two cells, and later allocation/sort/join/table failures retain every earlier
prefix. Retry duplicates that prefix. Rendering resolves duplicate cells
last-wins, retrieval first-wins, and multiyear tabular reset does not clear
entries. CP234 owns no validation, once latch, status, diagnostic,
transaction, rollback, cleanup, or reset.

One active direct C++ test calls CP234 once and makes 24 assertions over one
record from each control family, but it does not cover multi-type sorting,
filtered-list alignment, moved-from behavior, failures, replay, or rendered
output. Of 57 active full simulations, one expected fatal stops before final
reporting; the other 56 comprise 18 empty and 38 nonempty calls. The latter
contain 52 expanded Zone records, visit 208 slots, retain 55 populated slots,
and append 312 cells. Three records exercise a two-type lexical join, but no
full-simulation assertion reads the table.

Rust has only adjacent direct-Zone DualSetpoint graph records, schedule IDs,
and IdealLoads schedule resolution. It lacks the other three control
families, ZoneList-expanded source arena/order, fixed-slot sort/join behavior,
predefined System Summary store and column identities, append lifecycle,
final-report caller, serializer, and comparator.
`Output:Table:SummaryReports` remains ignored. CP234 remains required
`source_mapped` and adds no algorithm-level source, Rust/state/support/output,
numerical, performance, or conformance promotion. The inventory becomes 32
algorithms and 240 routines, split 58 `state_mapped` plus 182
`source_mapped`, with 117 required; the heat-balance project list becomes 86.

CP235 adds required `zone_space_heat_balance_update_temperatures`
immediately after `fill_predefined_table_on_thermostat_schedules` and before
`update_final_surface_heat_balance`. Its EnergyPlus boundary is
`ZoneSpaceHeatBalanceData::updateTemperatures`, declared at
`ZoneTempPredictorCorrector.hh` lines 233-234 and implemented at
`ZoneTempPredictorCorrector.cc` lines 6768-6833.

The sole production expression is the first child of CP203
`predictSystemLoad`. CP202 supplies Zone-first then active stored-Space
traversal. CP203 already owns that parent transaction, and CP216 owns the
down-interpolation formulas; CP235 separately owns rollback, helper
conditions/order, current-value commits, and working-history selection.

On every normal return CP235 copies all four slots of either `XMAT` or
`DSXMAT` into `ZTM`, followed by all four slots of the corresponding humidity
history into `WPrevZoneTSTemp`. This selection is independent of shortening
and does not itself change `MAT` or `airHumRat`. A non-shortened call can
therefore select existing downstepped state, while a shortened count-change
call can populate downstepped arrays and then select Zone histories.

Shortening first selects a Zone system node only for exact `spaceNum == 0`;
every nonzero identity selects a Space node. A positive node receives
temperature, parent-Zone `TempTstatAir`, raw humidity, then enthalpy from the
first Zone-timestep history values. The enthalpy formula floors negative
humidity at `1e-5` without changing node `HumRat`. Space calls overwrite the
shared parent-Zone thermostat value, so the last reached positive-node Space
can win.

Only a current/last system-step count mismatch invokes CP216: record
temperature then humidity, followed for exact Zones under the global
non-Mixing gate by Floor/occupied/mixed histories and independently every
RoomAir AFN node's temperature then humidity. Spaces get only the two base
calls. A matching count still rolls nodes back but leaves downstepped/current
state untouched before the final selector. The AFN branch does not test
`IsUsed`.

`HVACManager` begins a Zone timestep unshortened with Zone-history selection.
Adaptive entry makes only the first fine-step prediction shortened; later
fine predictions keep downstepped selection but skip rollback. A repeated
system-step count reuses prior downstepped arrays.
`SimulationManager::Resimulate` always passes false shortening.

CP235 has only the debug positive-Zone assertion and no identity, membership,
upper-bound, topology, count, timestep, finite-value, or history validation.
It owns no status, diagnostic, latch, transaction, rollback, cleanup, or
reset. Failure preserves ordered node and helper prefixes before suppressing
the final selector and every later CP203 effect. Stable complete replay is
deterministic overwrite behavior because production source/destination arrays
are distinct; changed counts/topology and the surrounding parent transaction
do not inherit that property.

No C++ test calls CP235 or CP203 directly. Sixteen focused CP202 calls with 24
setpoint assertions retain zero Zones, and the nine-assertion CP216 helper
test does not compose the wrapper. Of 57 active full simulations, one expected
fatal stops before prediction and one has zero Zones. Across one initial
prediction sweep of the other 55 configurations, the aggregate is 81 Zones
plus 24 active Spaces, or 105 CP235 calls and 105 pairs of complete working
history selections.

Actual shortened entry remains unobserved and is bounded at zero through 55
configurations. One hypothetical shortened sweep has at most 76 positive-node
records and, on count change, 210 base helper calls. All 81 Zones are Mixing,
so the special RoomAir calls have zero corpus potential. No assertion isolates
CP235 state. Installed RoomAir-AFN, displacement/UFAD, and Space-heat-balance
files are unadopted candidates only.

Rust owns adjacent Zone-only three-slot histories, flags, adaptive count logic,
and a by-value helper that rejects nonpositive timesteps. Its correction path
has no fourth slot or CP235 working-history transaction, Space record,
Zone/Space node or shared thermostat rollback, enthalpy update, RoomAir/AFN
topology, source HVAC cadence, exact wrapper, failure shape, or composed test.
CP235 remains required `source_mapped` and adds no algorithm-level source,
Rust/state/support/output/numerical/performance/conformance promotion. The
inventory becomes 32 algorithms and 241 routines, split 58 `state_mapped`
plus 183 `source_mapped`, with 118 required; the heat-balance project list
becomes 87.

CP236 adds required `zone_space_heat_balance_calc_predicted_system_load`
immediately after `zone_space_heat_balance_update_temperatures` and before
`update_final_surface_heat_balance`. Its EnergyPlus boundary is
`ZoneSpaceHeatBalanceData::calcPredictedSystemLoad`, declared at
`ZoneTempPredictorCorrector.hh` line 224 and implemented at
`ZoneTempPredictorCorrector.cc` lines 6835-7243.

The sole production expression is CP203 `predictSystemLoad` line 3253, after
history, capacitance, sums, coefficients, and RoomAir preparation and before
predicted humidity. CP202 supplies Zone-first then active stored-Space
traversal. Exact positive `spaceNum` selects Space node, stage, record, and
demand state; all control, setpoint, ITE, multiplier, staged gate, and
diagnostic context still comes from the parent Zone.

ThirdOrder uses `D * S - I`. Analytical uses
`C * (S - T1) - I` only for exact `D == 0`, otherwise
`D * (S - T1 * exp(min(700, -D / C))) /
(1 - exp(min(700, -D / C))) - I`; Euler uses
`C * (S - T1) + D * S - I`. CP236 adds no local capacity, denominator,
finite-value, or enum validation.

Uncontrolled, SingleHeat, SingleCool, SingleHeatCool, and DualHeatCool select
the total and heating/cooling-setpoint loads. Only strictly positive RAFN
fractions scale loads. SingleCool contains the source defect that divides the
still-zero heating local instead of cooling. ITE then replaces cooling only
with an unscaled ThirdOrder-shaped expression for SingleCool and both combined
controls. Combined inconsistent or unclassified loads fatal before staged
logic.

The staged override reads Zone or Space `StageNum` before its gates. Zero stage
sets zero load and deadband but, without a node, retains the ordinary branch's
setpoint. Negative and positive stages recompute cooling at high or heating at
low respectively; magnitude is ignored, RAFN/ITE are not reapplied, and the
ordinary deadband flag is never cleared.

Normal completion writes selected node setpoint, shared Zone Setback using the
record's prior `setPointLast`, that record's new last setpoint, shared scalar
thermostat setpoint, both shared deadband flags, then the selected Zone/Space
demand. Reporting applies load correction, Zone and list multipliers, and
optional equipment-sequence overwrites. Space traversal can therefore make
the last Space win shared Zone state while comparing against its own prior
record.

Fatal combined-control paths preserve diagnostics and old final state; later
node/report failures preserve an ordered prefix and block humidity. There is
no transaction or rollback. Stable replay can change Setback after updating
`setPointLast`. Environment initialization does not reset `setPointLast`,
Setback, CurDeadBand, StageNum, or every sensible-demand field.

One C++ fixture makes seven direct calls with 19 related assertions across
Uncontrolled, both SingleHeat signs, SingleCool cooling, SingleHeatCool
cooling, and Dual heating/cooling. It is Zone-only, ThirdOrder, unit-scaled,
and covers no node, Space, ITE, staged, defect, failure, or replay path.
Focused CP202 fixtures have zero Zones. A separate report-helper test is not
composed.

Across one initial sweep of 55 applicable active configurations, 81 Zones plus
24 active Spaces yield 105 calls: 95 ThirdOrder and 10 Analytical, with zero
Euler. The corpus has no staged object, adjusted-return ITE object, or
non-Mixing RoomAir model. No full-simulation assertion isolates CP236.

Rust has adjacent guarded Zone-only coefficient helpers, a bounded
DualSetpoint graph, node setpoint storage, oracle-fed IdealLoads demand, and
Zone multipliers, but no exact CP236 dispatcher, Space binding, five-way
control, Euler/load selection, RAFN/ITE/staged behavior, `setPointLast`,
shared flags, or composed sensible-demand report helper. CP236 remains
required `source_mapped` and adds no algorithm-level source, Rust/state/test,
support, output, numerical, performance, or conformance promotion. The
inventory becomes 32 algorithms and 242 routines, split 58 `state_mapped`
plus 184 `source_mapped`, with 119 required; the heat-balance project list
becomes 88.

CP237 expands the existing required `routine.manage_zone_equipment` mapping for
`ZoneEquipmentManager::ManageZoneEquipment`, declared at
`ZoneEquipmentManager.hh` lines 82-86 and implemented at
`ZoneEquipmentManager.cc` lines 141-167. It adds no routine or project item.
Every entry ignores the incoming `SimZone`, calls `InitZoneEquipment`, selects
`SizeZoneEquipment` only while `ZoneSizingCalc` is true or otherwise calls
`SimZoneEquipment` and then sets `ZoneEquipSimulatedOnce = true`, calls
`UpdateZoneEquipment`, and clears `SimZone` only after that child returns.
`FirstHVACIteration` reaches Init and the non-sizing Sim child; `SimAir` is
never cleared locally and is passed by reference through Sim when selected and
then Update.

The wrapper has no local validation, status, catch, cleanup, transaction, or
rollback. A failing child preserves its completed prefix. In particular, a
non-sizing Update failure occurs after the one-way simulated-once write but
before the caller's `SimZone` is cleared. Re-entry always repeats the children
because incoming `SimZone` and the latch are not gates. Nine direct C++ calls
across eight tests all use the non-sizing branch and assert descendant
equipment, node, or load effects rather than the parent protocol. Of 57 active
full-simulation expressions, one expected EMS fatal stops before HVAC and the
other 56 establish only a lower bound of 56 parent executions; repeated HVAC,
warmup, and sizing calls are not instrumented.

Existing Rust three-stage metadata, the typed IdealLoads graph validator,
execution-plan labels, and the direct PurchasedAir compatibility loop do not
implement the exact Init/Size-or-Sim/Update parent, its reference flags, latch,
failure prefixes, replay, reset, multi-family dispatch, or broad HVAC
behavior. CP237 therefore changes no Rust code, mapped state, support,
capability, output, numerical, performance, or conformance claim. The
inventory remains 32 algorithms and 242 routines, split 58 `state_mapped`
plus 184 `source_mapped`, with 119 required; the heat-balance and HVAC project
lists remain 88 and 8.

CP238 adds canonical required `routine.get_zone_equipment` after
`manage_zone_equipment` and before `sim_zone_equipment`, plus the matching HVAC
project item. `ZoneEquipmentManager::GetZoneEquipment` is declared at
`ZoneEquipmentManager.hh` line 88 and implemented at
`ZoneEquipmentManager.cc` lines 169-197. Its sole one-time guard encloses every
operation. A true entry calls the separate full `GetZoneEquipmentData`
dependency, clears `GetZoneEquipmentInputFlag`, sets
`ZoneEquipInputsFilled = true`, snapshots `NumOfTimeStepInDay` as the raw
integer `TimeStepsInHour * 24`, scans controlled Zone indexes for the maximum
same-index equipment-list count, and allocates but does not populate or sort
`PrioritySimOrder` to that extent. A false entry is a complete no-op.

The wrapper has no local range, allocation, count, arena, or consistency
validation and no status, diagnostic, catch, cleanup, transaction, or rollback.
A child fatal leaves the wrapper guard true and does not modify readiness
(false on a fresh-state entry), but can retain the child's partial input state
and sticky errors. Once the child returns, the guard commits false before
readiness, arithmetic, scanning, and allocation;
a later failure can therefore leave a false guard and true readiness with
unfinished derived state, and retry silently does nothing. There is no
per-environment rearm. The manager and data-owner clear paths reconstruct their
flags separately, so only coordinated full-state reset restores the normal
pair.

The only production expression is `SurfaceGeometry::SetupZoneGeometry` after
successful `GetSurfaceData` and before window-gap and storm-window input; CP237
`ManageZoneEquipment` never calls this routine. Twenty-three direct C++ calls
span 22 tests. The focused two-call test proves the default-true guard, first
snapshot `1 * 24 = 24`, populated Zone configuration, and a second-call no-op
after changing `TimeStepsInHour` to 2, but it does not assert readiness,
priority extent/content, failure, retry, or reset. Source-order tracing shows
all 57 active `ManageSimulation` expressions complete the one-time wrapper
during input setup, including the case that later fatals in EMS; 56 later
complete the simulation. No full-simulation assertion isolates CP238-owned
state.

Rust eagerly compiles immutable, IdealLoads-only typed equipment
lists/connections and separately derives time-axis sizes. It has no lazy
`GetZoneEquipment`, input/readiness latches, equipment-manager day snapshot,
full Zone/Space configuration, controlled-Zone maximum scan,
`SimulationOrder` scratch allocation, or source failure/retry/reset lifecycle.
Its graph sort and execution labels are not `PrioritySimOrder`, which CP238
only allocates and a later source routine fills. CP238 adds no algorithm-level
source, Rust target/code/state, support, capability, output, numerical,
performance, or conformance promotion. The inventory becomes 32 algorithms
and 243 routines, split 58 `state_mapped` plus 185 `source_mapped`, with 120
required; the heat-balance project list remains 88 and the HVAC list becomes 9.

CP239 adds canonical required `routine.init_zone_equipment` after
`get_zone_equipment` and before `sim_zone_equipment`, plus the matching HVAC
project item. `ZoneEquipmentManager::InitZoneEquipment` is declared at
`ZoneEquipmentManager.hh` line 90 and implemented at
`ZoneEquipmentManager.cc` lines 199-316. Its sole direct production call is the
unconditional `ManageZoneEquipment` line-155 child before that parent's
sizing-versus-simulation branch; it does not acquire equipment input.

A true one-time flag clears itself before any allocation, allocates
`ZoneEqSizing` to `NumOfZones`, and then visits ascending controlled Zone
indexes with nonzero equipment-list pointers. It publishes each selected
list's equipment count into sensible and moisture demand state, allocates six
sequenced-demand vectors, and allocates and zeroes the 35-entry sizing-method
array. Space demand vectors receive the parent Zone count only when Space heat
balance simulation or sizing is active. This allocation path uses each Zone's
stored Space membership, while the later Space initialization paths use the
full Space configuration array.

The independent begin-environment gate resets the Zone availability array and
the status/start/stop fields of allocated managers for the 14 valid component
types, then calls the separate `EquipConfiguration::beginEnvirnInit` dependency
for every controlled Zone and, only during Space simulation, controlled Space.
Those children reset selected Zone/inlet/exhaust/return node fields from fixed
20 C and current outdoor conditions. The environment flag clears only after
that whole block returns and rearms only on a reached call with
`BeginEnvrnFlag = false`.

Every invocation then calls `EquipConfiguration::hvacTimeStepInit` for
controlled Zones and optional simulation-time Spaces. It always clears each
configuration's excess exhaust; only `FirstHVACIteration` copies its Zone node
state to exhaust nodes and zeroes their flow availability. Finally CP239 zeros
exactly `SupFlow`, `ZoneRetFlow`, `SysRetFlow`, `RecircFlow`, `LeakFlow`, and
`ExcessZoneExhFlow` for every primary air loop.

There is no local topology, bounds, allocation, node, or finite-value
validation and no diagnostic, status, catch, cleanup, transaction, or rollback.
Failure after the early one-time-flag clear leaves unfinished storage that
retry skips. Environment failure before its late flag clear replays the prefix,
whereas timestep or air-loop failure after that clear retries without the
environment block during the same BeginEnvironment interval. Manager-only
reset restores the two flags but not the separately owned mutated state.

No C++ unit test directly calls CP239 or either delegated configuration method.
Nine non-sizing `ManageZoneEquipment` expressions across eight tests enter it
indirectly, but zero assertions target its latches, storage, availability,
node-reset protocol, excess exhaust, or six air-loop fields. Fifty-six active
full simulations provide only a lower bound of one CP239 entry each: 55 have
Zones and the WeatherManager fixture has zero Zones. The remaining intentional
EMS-fatal expression stops before HVAC. Exact sizing, warmup, environment, and
HVAC-iteration multiplicity is uninstrumented.

Rust has adjacent immutable IdealLoads equipment graphs, a four-scalar
`ZoneSysEnergyDemand`, diagnostic node state, and precomputed
begin-environment time-axis metadata. It has no equipment-count/sequenced
demand arenas, separate Zone moisture-demand arena or Space demand state, `ZoneEqSizing`,
availability-manager lifecycle, complete role-specific node state, persistent
one-time/environment latches, or primary-air-loop aggregate-flow state.
`IdealLoadsInitFlags` belongs to `InitPurchasedAir` and is not CP239. CP239 adds
no algorithm-level source, Rust target/code/state, test, support, capability,
output, numerical, performance, or conformance promotion. The inventory
becomes 32 algorithms and 244 routines, split 58 `state_mapped` plus 186
`source_mapped`, with 121 required; the heat-balance project list remains 88
and the HVAC list becomes 10.

CP240 adds canonical required
`routine.size_zone_space_equipment_part1` after `init_zone_equipment` and before
`sim_zone_equipment`, plus the matching HVAC project item. The exact lowercase
`ZoneEquipmentManager::sizeZoneSpaceEquipmentPart1` is declared at
`ZoneEquipmentManager.hh` lines 92-99 and implemented at
`ZoneEquipmentManager.cc` lines 317-597.

Its two production call expressions are the Zone call and optional Space-loop
call inside `SizeZoneEquipment`. The parent visits controlled Zones ascending,
calls the Zone first, and under current `doSpaceHeatBalance` visits every stored
Space without checking the Space configuration's controlled flag. The Space
call selects Space configuration, sizing, demand, heat-balance state, and node,
but deliberately retains the parent `ZoneData` and `zoneNum` for deadband, ITE,
multipliers, and final-Zone outdoor-air sizing.

Every entry zeros selected non-air and system-dependent responses, then calls
`initOutputRequired` with first-iteration true and simulation-order reset false.
That child rebuilds twelve remaining/unadjusted scalars, restores the shared
parent-Zone current deadband from its original flag on every entry, and on the
production sizing path fills allocated sequence arrays from full demand. CP240
snapshots
pre-DOAS sensible and moisture loads with separate deadband and strict
same-sign humidistat gates.

`AccountForDOAS` requires at least one inlet. It derives 90-percent-RH bounds,
uses final-Zone minimum outdoor air times standard density, delegates supply
conditions, updates remaining demand, writes DOAS state to inlet 1, and records
sensible/latent sizing fields. Two inlets route the residual load to inlet 2;
one inlet routes the residual through the non-air path. The false branch leaves
eight earlier DOAS fields stale.

The main sensible gate requires no original deadband and more than 1 W. It
selects cooling/heating supply temperature or difference, applies cooling-only
post-BeginSim ITE return adjustment, solves nonnegative mass flow above the
1e-5 C delta threshold, and applies only an adjustment factor above one.
Latent sizing independently uses strict same-sign setpoint loads and a 1e-30
absolute humidity-difference threshold, then can recompute the shared supply
state. Its
false branch leaves eight latent/no-DOAS fields stale.

A positive residual node receives only temperature, humidity ratio, enthalpy,
and mass flow. Otherwise CP240 writes non-air response and, when latent sizing
is active, additively updates latent gain; a Zone no-air result may first
distribute to Spaces, but each
following Space call zeros its own response before writing its result. The final
demand update makes two update calls on a DOAS path. CP240 has no local latch,
validation, status, catch, transaction, or rollback; failure retains ordered
Zone/Space, demand, sizing, node, non-air, and additive latent prefixes and
suppresses mass balance, leaving conditions, Part2, and the manager suffix.

No test directly calls CP240. Six `SizeZoneEquipment` calls across three tests
produce seven Zone entries, zero Space entries, and 88 mixed CP240/Part2/
downstream assertion lines. The fixtures bypass sizing setup and make
configuration controlled while `ZoneData` remains uncontrolled, so they do not
prove coherent controlled demand distribution. They omit Space, ITE, one- and
zero-inlet edges, non-air Zone output, adjustment above one, and failure/retry.

Of 56 completing active full simulations, 34 sizing configurations reach CP240
with a static first-sweep topology of 48 controlled Zones. Seven of them add 21
stored Spaces. Those Spaces are uncontrolled, zero-inlet records without DOAS,
yet CP240 still takes their non-air path. Across all 69 static roles, six Zones
enable DOAS, 13 roles enable latent sizing, 43 have a residual supply node, and
26 use non-air output; cooling ITE and an adjustment factor above one have zero
active roles. The other 22 completing simulations and the EMS-fatal context do
not reach CP240; exact repeated sizing cadence is uninstrumented.

Rust's sole raw `Sizing:Zone` epJSON fixture expects `UnsupportedSizing` before
runtime, and active IDFs contain none. It has exact adjacent psychrometrics,
fixed-option four-scalar `ZoneSysEnergyDemand`, IdealLoads supply limits, a
narrow purchased-air node update, and diagnostic node state, but no
typed/executable `Sizing:Zone`,
Zone/Space sizing and moisture arenas, total/unadjusted/sequenced demand
transaction, DOAS sizing/routing, non-air/latent distribution, or CP240
failure/replay lifecycle. CP240 adds no algorithm-level source, Rust
target/code/state, test, support, capability, output, numerical, performance,
or conformance promotion. The inventory becomes 32 algorithms and 245
routines, split 58 `state_mapped` plus 187 `source_mapped`, with 122 required;
the heat-balance project list remains 88 and the HVAC list becomes 11.

CP241 adds canonical required
`routine.size_zone_space_equipment_part2` after Part1 and before
`sim_zone_equipment`, plus the matching HVAC project item. The exact lowercase
`ZoneEquipmentManager::sizeZoneSpaceEquipmentPart2` is declared at
`ZoneEquipmentManager.hh` lines 101-105 and implemented at
`ZoneEquipmentManager.cc` lines 599-625.

Its only two production call expressions are the Zone and Space calls in
`SizeZoneEquipment`'s second pass. That pass starts only after every CP240
Zone/Space call, `CalcZoneMassBalance(state, true)`, and
`CalcZoneLeavingConditions(state, true)` return. It again visits controlled
Zones ascending, calls the Zone first, and under current `doSpaceHeatBalance`
calls every stored Space without a Space-control check.

The Zone call passes its Zone equipment configuration and `CalcZoneSizing`.
The Space call passes `CalcSpaceSizing`, parent `zoneNum`, and `spaceNum`, but
deliberately reuses the parent Zone equipment configuration rather than
`spaceEquipConfig`. Thus both calls use the parent Zone return-node list and
thermostat triplet; only the fallback system node and sizing record become
Space-specific.

CP241 selects the parent's first return node when `NumReturnNodes > 0` and that
first node identity is positive. A nonpositive count or first identity falls
back to the selected Zone/Space `SystemZoneNodeNumber`; later return nodes are
ignored. It reads only that node's temperature after the leaving-condition
dependency and never writes a node.

Strict-positive `HeatLoad` selects the heating branch before strict-positive
`CoolLoad`; all other values take the catch-all branch. Heating writes
`HeatZoneRetTemp`, chooses `HeatTstatTemp` from a strict-positive central
`setpt` or `setptLo`, and writes `CoolTstatTemp = setptHi`. Cooling writes
`CoolZoneRetTemp`, chooses `CoolTstatTemp` from the central setpoint or
`setptHi`, and writes `HeatTstatTemp = setptLo`. The catch-all writes the cool
return snapshot and both low/high thermostat bounds. Every branch overwrites
both thermostat fields and exactly one return field, leaving the opposite
return snapshot stale; heating wins if both loads are positive.
`UpdateZoneSizing` later consumes both return snapshots into sizing sequences,
so the inactive stale value is downstream-observable.

There is no child call or local latch, allocation, validation, diagnostic,
status, catch, cleanup, transaction, or rollback. Indexed return/configuration,
Zone/Space, node, or thermostat failures occur before the current record's
three writes. Parent failure before the second pass suppresses CP241 entirely;
failure during it retains the complete CP240/mass/leaving prefix and earlier
Part2 records. Same-state retry reruns that parent prefix and overwrites the
selected CP241 fields, while the inactive return field and CP240 additive
effects can remain history-dependent.

No test calls CP241 directly. Six direct `SizeZoneEquipment` calls across three
tests produce seven Zone entries and zero Space entries: one heating, one
cooling, and five catch-all, all through system-node fallback. Only four
assertions in two catch-all tests name CP241 thermostat fields; no direct
wrapper assertion names either return snapshot. The separate sizing-array reset
test proves zero reset only and never executes CP241.

Seventeen of 18 direct `ManageSizing` contexts reach 24 Zone entries, all with
one positive return node, but assert none of the four CP241 fields. Among 57
active full `ManageSimulation` contexts, 56 complete and exactly 34 reach a
static 48-Zone plus 21-Space Part2 topology. Fifty-six roles use a first return
and 13 use system-node fallback: Zones split 44/4 and Spaces 12/9. The 12 Space
roles share their parent first return; the other nine use their own Space
system node. No active role has multiple returns. Exact heat/cool/catch-all,
central-setpoint, design-day, warmup, timestep, retry, and repeated-sweep
cadence is uninstrumented.

Rust has typed thermostat schedules and direct thermostat report series,
equipment-connection return identities, diagnostic node temperatures, and a
finite-limit recirculation helper that can resolve a first return. These are
adjacent only. Rust has no Zone/Space sizing snapshot, four CP241 fields,
post-leaving second pass, parent-config Space alias, mutable
`setpt`/`setptLo`/`setptHi` triplet, load/setpoint selection, stale-field
lifecycle, or failure/replay transaction. The sole raw `Sizing:Zone` fixture
still blocks before runtime and active IDFs contain none.

CP241 adds no algorithm-level source, Rust target/code/state, test, support,
capability, output, comparator, case, manifest, numerical, performance, or
conformance promotion. The inventory becomes 32 algorithms and 246 routines,
split 58 `state_mapped` plus 188 `source_mapped`, with 123 required; the
heat-balance project list remains 88 and the HVAC list becomes 12.

CP242 adds canonical required `routine.size_zone_equipment` after Part2 and
before `sim_zone_equipment`, plus the matching HVAC project item. The exact
capitalized `ZoneEquipmentManager::SizeZoneEquipment` is declared at
`ZoneEquipmentManager.hh` line 107 and implemented completely at
`ZoneEquipmentManager.cc` lines 627-694.

Its sole production call expression is `ManageZoneEquipment` line 158, after
CP239 Init and only when the current `ZoneSizingCalc` is true. CP242 itself
accepts only `state`, does not inspect that selector, and can be called
directly. The manager's `FirstHVACIteration` argument is not forwarded.

The manager-data latch `SizeZoneEquipmentOneTimeFlag` defaults true. A true
entry delegates the still-separate `SetUpZoneSizingArrays` dependency and
clears the latch only after normal return. Setup failure therefore retains a
true latch and any child prefix; success followed by later failure leaves the
latch false, so retry skips setup. Begin-environment transitions do not rearm
it. Manager `clear_state()` reconstructs the default-true latch but does not
undo independently owned child state. External `RezeroZoneSizingArrays` is
not a CP242 per-call reset and does not change the latch.

After setup, CP242 completes an ascending controlled-Zone Part1 pass. Each
Zone call precedes its stored-order Spaces when the current
`doSpaceHeatBalance` is true; no Space-controlled check, sort, deduplication,
or cross-pass membership snapshot exists. Space Part1 uses Space
configuration, sizing, and demand state but the parent Zone record. The
parent then unconditionally calls `CalcZoneMassBalance(state, true)` and
`CalcZoneLeavingConditions(state, true)`, even with no controlled Zone.
Only after both return does a second ascending pass call Part2 for each Zone
then its gated Spaces; Space Part2 deliberately reuses the parent Zone
configuration.

Apart from clearing its setup latch, CP242 owns no output assignment,
validation, diagnostic, status, catch, cleanup, transaction, or rollback.
Child or indexed-access failure preserves completed prefixes plus any
partial effects of the failing child: setup, earlier Part1 roles, mass
balance, leaving conditions, and earlier Part2 roles as applicable. Later
roles and the outer manager update are suppressed. Same-state retry restarts
the traversal; delegated additive Part1 and mass-balance effects make the
parent generally non-idempotent.

Six direct C++ calls across three tests produce six complete wrapper
invocations, seven Zone Part1/Part2 role pairs, six mass-balance calls, six
leaving-condition calls, and zero Spaces. All three tests force the setup
latch false, and their 88 assertion lines inspect descendant or downstream
results rather than the latch, either global barrier, exact call trace, or
failure prefix. Within these direct wrappers, setup-true,
uncontrolled/zero-Zone, Space, malformed-topology, child failure, and retry
recovery are absent.

Across one parent invocation in each of the 17 reaching among 18 direct
`ManageSizing` contexts, the static aggregate is 24 Zones and zero Spaces;
the plant-only context does not enter CP242. Across one parent invocation in
each of the 34 reaching among 56 completing active `ManageSimulation`
contexts, the static aggregate is 48 Zones plus 21 Spaces. Each context
contributes only its subset. Fresh successful sizing states necessarily
cross the default-true setup route once, but no assertion isolates its call
count or latch transition. Exact design-day, warmup, timestep, HVAC-iteration,
and repeated-parent invocation counts remain uninstrumented.

Rust contains no CP242 symbol, snake-case counterpart, Zone/Space sizing
arena, mass-balance or leaving-condition parent, or one-time sizing latch.
Its three zone-equipment stage labels are only Manage, Sim, and
SimPurchasedAir; graph validation, a four-scalar demand snapshot,
psychrometrics, node projection, and direct prebound IdealLoads execution are
adjacent rather than this setup-plus-two-pass transaction. `Sizing:*` remains
run-blocked, the sole raw `Sizing:Zone` fixture fails before runtime, and the
active data-model corpus contains no `Sizing:Zone`.

CP242 adds no algorithm-level source, Rust target/code/state, test, support,
capability, output, comparator, case, manifest, numerical, performance, or
conformance promotion. The algorithm remains `scaffold` with claim level
`none`. The inventory becomes 32 algorithms and 247 routines, split 58
`state_mapped` plus 189 `source_mapped`, with 124 required; the heat-balance
project list remains 88 and the HVAC list becomes 13.

## CP243 `CalcDOASSupCondsForSizing` DOAS Supply Selector

CP243 adds canonical required
`routine.calc_doas_sup_conds_for_sizing` after `size_zone_equipment` and
before `sim_zone_equipment`, plus the matching HVAC project item. The exact
routine is declared at `ZoneEquipmentManager.hh` lines 244-254 and implemented
completely at `ZoneEquipmentManager.cc` lines 696-765. Its sole production
call expression is CP240 `sizeZoneSpaceEquipmentPart1` line 387, reached only
for a current Zone or Space sizing role whose `AccountForDOAS` is true.

The helper first writes `DOASSupTemp = 0.0` and then `DOASSupHR = 0.0`.
`NeutralSup` clamps temperature below Low or above High, using outdoor
humidity below Low and `min(OutHR, W90H)` above High; its middle branch passes
through both outdoor values. `NeutralDehumSup` always selects High
temperature, using outdoor humidity below Low and `min(OutHR, W90L)`
otherwise. `CoolSup` selects High temperature plus outdoor humidity below
Low, otherwise Low temperature plus `min(OutHR, W90L)`. Comparisons are raw
strict `<` and `>` with no epsilon, threshold-order check, finite check,
nonnegative-humidity check, or clamp beyond those explicit branches.

The unqualified `min` is ObjexxFCL's `a < b ? a : b`, not `std::fmin`.
Ordinary values select the numeric minimum, but ties select the second
`W90*` operand, including its signed-zero bit. A NaN first operand therefore
selects a finite second operand, while a NaN second operand is selected after
a finite first operand. Raw IEEE comparisons also send `OutDB = NaN` to the
`NeutralSup` pass-through branch and to the other strategies' else branches.
With inverted thresholds, the first `OutDB < Low` test owns the overlapping
`NeutralSup` range.

`Invalid`, `Num`, and cast enum values outside the three valid enumerators
retain the two zero writes and then fatal with
`CalcDOASSupCondsForSizing:illegal DOAS design control strategy`. Valid paths
do not read or mutate `state`; only the fatal path uses it for diagnostics.
There is no local latch, allocation, numeric-input validation beyond enum
dispatch, status, catch, cleanup, checkpoint, transaction, or rollback.
Output-reference aliasing is unchecked: temperature is written first and
humidity second, so the final shared value is the humidity result. All
scalar/control calculation inputs other than `state` are passed by value.

CP240 has already reset current response state, rebuilt demands, snapshotted
pre-DOAS loads, validated inlet count, calculated the two 90%-RH values, and
calculated DOAS mass flow before calling CP243. Only a normal return permits
its heat-capacity, enthalpy, load, demand, inlet-node, and sizing-record
suffix. An invalid-control fatal therefore retains the completed model-state
prefix, writes only stack-local outputs to zero without publishing them to
node or sizing state, and suppresses the current suffix, later Part1 roles,
mass/leaving barriers, all Part2 roles, and the production manager suffix.
A valid direct repeat deterministically overwrites its two outputs; an
invalid repeat can zero again and repeat the fatal diagnostic. A CP242
retry remains generally non-idempotent because it replays the wider Part1
transaction.

The direct helper test makes seven calls and has 14 output assertions:
three `NeutralSup`, two `NeutralDehumSup`, and two `CoolSup` calls cover every
valid branch. Its finite ordered inputs test only cap-selected min branches.
It does not cover equality, inverted thresholds, IEEE specials, signed zero,
invalid enum, output aliasing, failure, or retry. Six direct
`SizeZoneEquipment` wrapper calls across three tests cause only three CP243
executions: two `CoolSup` else/cap executions and one `NeutralSup`
high/non-cap execution. Six stored-output assertions observe those results;
the other four wrapper calls have DOAS disabled.

Across one parent invocation in each of the 17 reaching direct
`ManageSizing` contexts, all 24 Zone roles have `AccountForDOAS` false, so
CP243 is not reached. Across one parent invocation in each of the 34 reaching
among 56 completing active full simulations, the static aggregate is 48
Zones plus 21 Spaces; exactly six Zone roles and no Space role enable DOAS.
Five fixed `CoolSup` Zones use 12.8/15.6 C setpoints, while one defaults to
auto-resolved `NeutralSup`; only downstream results are asserted.
Exact repeated sizing and dynamic call counts remain uninstrumented; each
context contributes only its own subset.

Rust has no exact CP243 symbol, snake-case counterpart, `DOASControl`,
`AccountForDOAS`, or DOAS sizing-output field. Its PurchasedAir outdoor-air
supply path, IdealLoads supply limits, and psychrometric helpers are adjacent
runtime behavior, not the `Sizing:Zone` DOAS selector. `Sizing:*` and
`ZoneSizing*` remain run-blocked, the sole raw `Sizing:Zone` fixture fails
before runtime, and the active data-model corpus contains no `Sizing:Zone`.

CP243 adds no algorithm-level EnergyPlus source, Rust target/code/state,
test, object support, capability, output implementation, comparator, case,
manifest, numerical, performance, or conformance promotion. The algorithm
remains `scaffold` with claim level `none`. The inventory becomes 32
algorithms and 248 routines, split 58 `state_mapped` plus 190
`source_mapped`, with 125 required; the heat-balance project list remains 88
and the HVAC list becomes 14.

## CP244 `SetUpZoneSizingArrays` One-Time Sizing-State Constructor

CP244 adds canonical required `routine.set_up_zone_sizing_arrays` after
`calc_doas_sup_conds_for_sizing` and before `sim_zone_equipment`, plus the
matching HVAC project item. The exact routine is declared at
`ZoneEquipmentManager.hh` line 109 and implemented completely at
`ZoneEquipmentManager.cc` lines 767-1082.

Its sole production call expression is `SizeZoneEquipment` line 644 under
the default-true `SizeZoneEquipmentOneTimeFlag`. The caller clears that latch
at line 645 only after CP244 returns normally. Direct calls do not read or
write the latch. Production therefore runs setup once on the first reached
sizing-parent entry in a fresh state, skips it on later sizing timesteps, and
retains a true latch after any setup abnormal non-return.

CP244 starts with local `ErrorsFound = false`. If `ZoneIntGain` alone is not
allocated, it delegates `AllocateIntGains`; that guard does not independently
check the other arrays the child creates. It then visits every
`ZoneSizingInput` in stored order. An exact HeatBalance Zone-name miss emits
a severe and latches the local error. For each record it recomputes whether
any equipment configuration is controlled. When at least one is, an exact
configuration-name match writes `ZoneNum`; a miss only warns outside pulse
sizing, and the matched configuration itself is not rechecked as controlled.
If either airflow method is `FromDDCalc`, CP224 lazily verifies the exact
thermostat name and owns another pulse-suppressed warning. With no controlled
configuration anywhere, every sizing input emits a severe and latches an
error. An empty sizing-input arena skips this validation loop entirely.

The still-separate `AutoCalcDOASControlStrategy` child then runs
unconditionally, even when the parent's local error is already true. It can
mutate and report DOAS setpoints and can issue its own earlier fatal for an
inverted low/high pair. On normal return, CP244 allocates four Zone sizing
arenas over design days and Zones, four analogous Space arenas only under
`doSpaceHeatBalanceSizing`, terminal-final member sequences, three zeroed
weather sequences per design day, and averaging storage.

Each controlled Zone next selects an exact-name sizing input or, when none
matches, unguardedly uses input 1 and emits the third pulse-suppressed
warning. A missing first input is therefore not locally protected. The
separate `fillZoneSizingFromInput` child fills the Zone and then each stored
Space under the sizing-Space gate from that same selected input. With EMS
present, CP244 registers 17 internal variables and six actuators per
controlled Zone; it registers none for Spaces.

The routine then scans every `DesignSpecification:OutdoorAir:SpaceList`.
Valid Space indexes are appended without first clearing persistent
`dsoaSpaceIndexes`; missing and already-seen members emit severe diagnostics,
set shared `dsoaError` and `ErrorsFound`, and duplicates remain appended.
A single shared `dsoaError` suppresses DSOA dereference and design-OA
calculation for every later Zone and Space child. CP244 calls the separate
`calcSizingOA` for controlled Zones first, then, when Space sizing is active,
for every globally indexed Space whose parent Zone is controlled.
Cross-Zone SpaceList membership can add to `ErrorsFound` while calculation
continues.

Finally CP244 writes the averaging-window EIO rows, global heating factor,
nonunit controlled-Zone heating factors, global cooling factor, and nonunit
controlled-Zone cooling factors. If the accumulated flag is true, only after
those writes it fatals with
`SetUpZoneSizingArrays: Errors found in Sizing:Zone input`. The routine owns
no status, catch, cleanup, checkpoint, transaction, or rollback.

A late failure preserves allocation, fills, OA and equipment mutations, EMS
registrations, EIO, diagnostics, and the true caller latch. Same-state replay
is not idempotent: SpaceList indexes append again and can create new
duplicates; DOAS and main EIO plus diagnostics repeat; EMS registration
attempts repeat; final Zone/Space records are re-zeroed, so peak occupancy is
rebuilt rather than carried across a full replay; selected member/weather
sequences reset, while other same-extent fields can remain. A partial
`AllocateIntGains` failure can also leave `ZoneIntGain` present, causing its
single guard to skip repair. Manager `clear_state()` rearms the caller latch
but does not reset all other modules, and `RezeroZoneSizingArrays` is not a
complete setup reset.

Three tests call CP244 directly. None immediately asserts a CP244-owned
field; five downstream assertions depend on its results. The AirTerminal
fixture covers two matching controlled Zone records and two unasserted
missing-thermostat warnings. The other two direct fixtures have no sizing
input or controlled Zone and append four plus six valid unique DSOA
SpaceList members. All three omit Space sizing, EMS, and enabled DOAS.
Six direct `SizeZoneEquipment` calls force the setup latch false and execute
CP244 zero times.

Seventeen fresh direct `ManageSizing` contexts each complete setup once:
their aggregate is 24 matching Zone inputs, controlled Zones, and successful
thermostat checks, with 24 Zone fills and OA calls but no Space fill, DOAS,
EMS, or DSOA SpaceList. Thirty-four sizing configurations among 56
completing active simulations likewise complete one setup each. Their static
aggregate is 48 matching Zone inputs and fills, plus 21 Space fills and OA
calls across seven Space-sizing configurations. Exactly six Zones enable
DOAS, one configuration has a valid unique two-member DSOA SpaceList, and
none reaches EMS registration. These are per-fresh-state setup counts, not
sizing-timestep counts.

No test isolates allocation extents or initialization, `ZoneNum`, copied
sizing fields, `MinOA`, EMS bindings, sizing-factor EIO, latch transition,
pulse warning suppression, fallback, malformed Zone or SpaceList input,
child failure, partial prefix, retry, or reset. Direct and corpus evidence is
normal-path composition only.

Rust has no exact CP244 symbol, snake-case counterpart, `ZoneSizingInput`,
Final/Calc Zone or Space sizing arenas, terminal sizing arrays, design-day
weather sizing store, DSOA SpaceList population, EMS sizing binding, or
sizing-factor EIO path. Its typed Zone, Space, ordinary `SpaceList`,
thermostat, equipment connection, and individual DSOA objects are adjacent
typed or limited-runtime subsets, not this setup transaction. Authored
`Space` and ordinary `SpaceList` remain run-blocked;
`DesignSpecification:OutdoorAir:SpaceList` is untyped, and `Sizing:*`,
`ZoneSizing*`, and EMS remain run-blocked.

CP244 adds no algorithm-level EnergyPlus source, Rust target/code/state,
test, object support, capability, output implementation, comparator, case,
manifest, numerical, performance, or conformance promotion. The algorithm
remains `scaffold` with claim level `none`. The inventory becomes 32
algorithms and 249 routines, split 58 `state_mapped` plus 191
`source_mapped`, with 126 required; the heat-balance project list remains 88
and the HVAC list becomes 15.

## CP245 `calcSizingOA` Zone/Space Outdoor-Air Sizing Mutator

CP245 adds canonical required `routine.calc_sizing_oa` immediately after
`set_up_zone_sizing_arrays` and before `sim_zone_equipment`, plus the matching
HVAC project item. The exact routine is declared at
`ZoneEquipmentManager.hh` lines 111-117 and implemented completely at
`ZoneEquipmentManager.cc` lines 1084-1206. It returns `void`, mutates separate
final and calculated-final sizing records plus shared module state, reads the
non-const `dsoaError` reference without assigning it, and only raises
`ErrorsFound` on a cross-Zone DSOA SpaceList member. It validates neither
record aliasing, bool aliasing, indexes, allocation, nor a Space's parent Zone.

The only production call expressions are CP244's controlled-Zone call at line
1032 and optional controlled-parent Space call at line 1042. CP244 visits all
controlled Zones first, then globally indexed Spaces in ascending order, and
shares the same two error flags across the whole pass. CP245 snapshots the
final record's `ZoneDesignSpecOAIndex`, reads the Zone's signed integer
`Multiplier * ListMultiplier`, and selects Zone or Space floor area. Only a
positive DSOA pointer with false `dsoaError` is dereferenced. A DSOA SpaceList
then checks every positive member against `zoneNum`; each mismatch emits one
severe plus two continuation messages and sets `ErrorsFound`, but does not
break, remove the member, set `dsoaError`, or stop later calculation. Zero
member indexes are skipped locally. The same guarded block writes only the
final record's per-person and per-area design OA rates; an existing value is
retained when the guard is false.

CP245 scans the complete People arena. Zone roles select `People.ZonePtr`,
while Space roles select `People.spaceIndex`; each matching design count is
multiplied by the Zone multiplier. Peak occupancy is accumulated with `+=`.
A strictly positive schedule maximum scales the contribution, whereas zero,
negative, or NaN maxima fall back to the full design count. Minimum occupancy
always uses the schedule minimum without a clamp. Null schedules, inconsistent
Zone/Space ownership, negative values, and non-finite values have no local
protection, and schedule extrema are lazily cached by the child accessors.

The final record then receives multiplied floor area, total design People,
and per-person and per-area OA totals. Minimum breathing-zone OA for the
predefined report uses minimum scheduled occupancy and
`std::min(ZoneADEffCooling, ZoneADEffHeating)`, replacing either signed zero
with one; negative and NaN values otherwise flow through. Only a Zone role
publishes `ZonePreDefRep.VozMin`; a Space role computes and discards that
report value. The selected Zone or Space equipment configuration always
stores the DSOA and air-distribution indexes, including when `dsoaError` is
already true.

With false `dsoaError`, CP245 delegates
`calcDesignSpecificationOutdoorAir` using four false control flags, the
current role, and the default-enabled IAQ-method path. The child owns DSOA
method arithmetic, SpaceList selection, multiplier application,
contaminant-state dependencies, diagnostics, fatals, and persistent warning
flags. CP245 does not multiply its returned OA again. The local accumulator
stays zero when the child is suppressed; otherwise it receives the child
result, which CP245 writes to both `MinOA` records, then, if either final air-distribution effectiveness
is positive, divides by unqualified ObjexxFCL `min` and copies the final
answer back to calculated-final. This second minimum chooses the heating
operand on a tie, differs from the earlier `std::min` for NaN operands, and
can divide by zero or a negative/NaN operand; no finite or nonnegative clamp
exists and calculated-final effectiveness is ignored.

Per-area cooling and heating limits are freshly derived from role floor area
and Zone multiplier. Four final/calculated-final input-flow fields are then
scaled in place with `*=`, so direct same-state replay compounds any nonunit
multiplier. Every design or run-period design day finally receives exactly
five final and calculated Zone-array fields: `MinOA`,
`DesCoolMinAirFlow2`, `DesCoolMinAirFlow`, `DesHeatMaxAirFlow2`, and
`DesHeatMaxAirFlow`. Space roles do not write the corresponding Space daily
arrays; they overwrite the parent Zone's daily column, so under CP244 order
the highest global Space index belonging to a Zone supplies the last values.

The routine owns no status, catch, checkpoint, cleanup, transaction, or
rollback. Any failure preserves its completed prefix. In particular, an OA
child failure occurs after aggregates, `VozMin`, and equipment indexes but
before the new `MinOA`; an interrupted daily loop preserves earlier days and
earlier fields. Direct retry is non-idempotent because peak occupancy and the
four multiplier-scaled flows accumulate, diagnostics can repeat, and schedule
or OA-child caches and flags can change behavior. Aliasing the two sizing
records applies the four `*=` operations twice; aliasing the two bool
references lets a cross-Zone error suppress the same call's OA child. A full
CP244 replay zero-fills and refills final records, avoiding those two direct
record accumulations, but remains non-idempotent through DSOA indexes,
diagnostics, and child state. `RezeroZoneSizingArrays` and manager
`clear_state()` do not constitute a coordinated reset of all CP245 owners.

No C++ test calls CP245 directly or immediately asserts a CP245-owned field.
Static corpus reachability is 95 calls: 74 Zone and 21 Space roles. All enter with false
`dsoaError` and reach the OA child; 94 have positive DSOA pointers and one PIU
role has zero. The positive set contains 93 individual DSOA roles and one
valid two-member SpaceList role. OA methods are 34 Sum, 54 Flow/Person, and
six Flow/Zone plus the pointer-zero role. Forty-one calls match one People
object with a positive schedule maximum, while 54 match none. All 95 use
1.0/1.0 effectiveness; 67 roles use unit multipliers and 28 use 10. Existing
Beam, OccupantDiversity, OutputReportTabular, and Standard621 assertions are
downstream composition oracles, not isolated checks. No test covers the
owned scalar and daily writes, `VozMin`, equipment indexes, true
`dsoaError`, malformed or cross-Zone topology, schedule fallback, multiple or
null-schedule People, nonunit/IEEE effectiveness, Maximum OA, aliasing,
partial failure, direct retry, reset, or Space overwrite.

Rust has no exact `calcSizingOA` routine or Zone/Space sizing and design-day
arrays, shared `dsoaError`/`ErrorsFound` protocol, DSOA SpaceList cross-Zone
validation, mutable equipment OA/air-distribution indexes, air-distribution
effectiveness, per-People schedule-extrema accumulation,
multiplier-applied occupancy/floor-area state, `ZonePreDefRep.VozMin`,
in-place sizing-flow scaling, or design-day fanout. Typed Zone, Space, People,
schedules, IdealLoads equipment, and individual DSOA plus the PurchasedAir OA
design-flow helper are adjacent subsets only; authored Space/SpaceList, zone
grouping, and sizing remain run-blocked.

CP245 adds no algorithm-level EnergyPlus source, Rust target/code/state, test,
object support, capability, output implementation, comparator, case,
manifest, numerical, performance, or conformance promotion. The algorithm
remains `scaffold` with claim level `none`. The inventory becomes 32
algorithms and 250 routines, split 58 `state_mapped` plus 192
`source_mapped`, with 127 required; the heat-balance project list remains 88
and the HVAC list becomes 16.

## CP246 `fillZoneSizingFromInput` Sizing-Input Projection and Sequence Allocation

CP246 adds canonical required `routine.fill_zone_sizing_from_input` after
`calc_sizing_oa` and before `sim_zone_equipment`, plus the matching HVAC
project item. The exact helper is declared at `ZoneEquipmentManager.hh` lines
119-126 and implemented completely at `ZoneEquipmentManager.cc` lines
1208-1400. It returns `void`, takes one const `ZoneSizingInputData`, two
mutable daily arrays, two mutable final records, and caller-provided identity.
It reads only the total design/run-period design-day count and manager
`NumOfTimeStepInDay`; it owns no Zone/Space lookup or input interpretation.

The only production call expressions are CP244 lines 876 and 886. For each
controlled Zone in ascending order, CP244 selects one exact-name or fallback
Zone sizing input, calls CP246 for that Zone, and, when
`doSpaceHeatBalanceSizing` is true, calls it for the Zone's stored
`spaceIndexes` in order using the same parent input. CP246 is
role-agnostic. A Space call targets the Space arrays and final records, writes
the Space name, and stores the global Space index even in the destination
field named `ZoneNum`; it neither reads the input's `ZoneName`/`ZoneNum` nor
checks the Space-parent relationship.

All daily records are processed before either final record. For each design
or run-period design day, CP246 obtains both normal and calculated record
references, writes both identities, projects the normal subset, projects the
calculated subset, then allocates/zeros normal and calculated sequences in
that order. It next writes both final identities, the complete final subset,
the calculated-final subset, and finally allocates/zeros those two sequence
sets. With a nonpositive summed day count, the daily loop is skipped but both
final records are still filled and dimensioned.

Every destination receives caller identity plus the same 35 input fields:
sensible supply-air methods, temperatures, differences, and humidity ratios;
cooling/heating airflow methods and input/per-area/absolute/fraction values;
sizing factors; DOAS enable/strategy/setpoints; Space concurrence and Zone
sizing method; latent enable, RH constants and shallow schedule pointers,
and latent method integers; and heat-coil sizing method/ratio.
`InpDesCoolAirFlow` and `InpDesHeatAirFlow` receive input
`DesCoolAirFlow` and `DesHeatAirFlow`. Enum, integer, pointer, and floating
values are copied without validation or arithmetic.

The four destination write sets are intentionally asymmetric:

| Destination kind | Member assignments | Additional fields beyond identity plus common 35 |
|---|---:|---|
| normal daily | 37 | none |
| calculated daily | 41 | latent cooling/heating design humidity ratios and differences |
| final | 47 | latent four; DSOA and air-distribution indexes; cooling/heating air-distribution effectiveness; secondary recirculation; ventilation efficiency |
| calculated-final | 45 | latent four; both indexes; cooling/heating air-distribution effectiveness |

Thus normal daily records retain any prior values in the four omitted latent
fields, daily records receive no OA/air-distribution indexes or
effectiveness, and calculated-final retains prior secondary-recirculation and
ventilation-efficiency values. The two input object-name strings are not
copied; only resolved indexes reach the final pair. Production copies the
current DOAS values after `AutoCalcDOASControlStrategy` has already run.

After each member projection, `ZoneSizingData::allocateMemberArrays`
dimensions exactly 36 sequences from `HeatFlowSeq` through
`LatentHeatFlowSeq` to `NumOfTimeStepInDay` with `0.0`. ObjexxFCL
`dimension(range, value)` assigns the initializer even at an unchanged
extent, so every completed retry zeros all 36 sequences again. A completed
CP246 call therefore performs `2 * max(day_count, 0) + 2` member-array
helper calls. It does not initialize any member outside the listed projection.

There is no local validation of allocation, bounds, identity, topology,
record distinctness, enum values, finite/nonnegative values, timestep extent,
or old contents. Invalid enums, out-of-range method integers, negative or
non-finite values, and schedule pointers are copied raw. There is no
diagnostic, error flag, status, catch, checkpoint, transaction, cleanup, or
rollback. CP246 still runs when CP244 has already accumulated another input
error.

Failure preserves exact source-order prefixes. Both daily references are
obtained before the current day's first write, so failure obtaining the
calculated reference leaves that day untouched while preserving earlier
days. Later failure can leave normal or calculated member-assignment prefixes and a
partially dimensioned 36-sequence prefix. After every daily record completes,
both final identities are written before either final projection; both final
member-assignment subsets complete before final and calculated-final sequence
allocation. A CP246 abnormal exit prevents remaining role fills, later EMS
registration, DSOA population, CP245 OA work, sizing-factor EIO, and the
parent latch transition.

Mutable destinations need not be distinct. Aliased daily arrays finish with
the calculated-only latent four added to the common union and zero all
sequences twice. Aliased final records retain the final-only secondary and
ventilation fields because the later calculated-final block does not clear
them, and also zero sequences twice. If a final reference aliases a daily
element, the final suffix overwrites it after all daily work. Production
passes distinct stores.

With stable input, extents, and nonaliased destinations, a completed direct
retry deterministically overwrites the copied subset and rezeros sequences;
there is no CP245-style `+=` or `*=` accumulation. CP246 is not a full-record
reset, however. Omitted fields and all unrelated computed/EMS/OA/peak sizing
scalars survive. Same-extent parent replay can therefore preserve stale
normal-daily latent values and other untouched state even as CP246 resets its
sequences. The separate `RezeroZoneSizingArrays` wrapper delegates
`zeroMemberData`, which returns without changing that record unless
`DOASSupMassFlowSeq` is allocated. When that guard passes, the helper
zero-fills the current extents of 36 sequences and resets only 104 selected
members while preserving CP246 identity/static input fields. `ZoneEquipmentManager` state
reset does not own the DataSizing stores. Clean replay still requires
coordinated owner reset.

No C++ test calls CP246 directly or immediately asserts a CP246-owned write.
Static fresh-state reachability is 95 calls: two direct-CP244 Zone roles, 24
Zone roles across 17 `ManageSizing` contexts, and 48 Zone plus 21 Space roles
across 34 sizing-active simulations. The other two direct CP244 fixtures and
all six direct `SizeZoneEquipment` wrappers execute CP246 zero times. Of the
95 roles, 89 have DOAS disabled; five Zones use fixed cold-supply DOAS and one
uses auto-resolved neutral-supply DOAS. Zone sizing methods are 82
sensible-only/no-latent, nine sensible, and four sensible-and-latent, making
latent sizing active in 13 roles. Both RH schedule pointers are null and both
latent methods are humidity-ratio difference in all 95 roles. Existing
descendant sizing assertions prove only normal-path composition.

There is no isolated evidence for the four write sets, exact 36-array order
or zero contents, Space identity and parent-input reuse, calculated-final
omissions, zero design days, invalid/raw values, schedule pointers, malformed
array shapes, aliasing, allocation failure, partial prefix, retry,
same-extent stale fields, or reset behavior.

Rust has no exact `fillZoneSizingFromInput` routine, typed `Sizing:Zone`
input, Zone/Space design-day/final/calculated-final sizing arenas, source
field-copy asymmetries, or per-record timestep-sequence allocation. Typed
Zone/Space identities, schedules, Humidistat controls, individual DSOA,
IdealLoads operational supply limits, equipment graph, time-axis metadata,
and sizing-checked flags are adjacent subsets only; authored
Space/SpaceList, grouping, and sizing/autosizing remain blocked.

CP246 adds no algorithm-level EnergyPlus source, Rust target/code/state,
test, object support, capability, output implementation, comparator, case,
manifest, numerical, performance, or conformance promotion. The parent
algorithm remains `scaffold` with claim level `none`. Inventory becomes 32
algorithms and 251 routines, split 58 `state_mapped` plus 193
`source_mapped`, with 128 required; the heat-balance project list remains 88
and the HVAC list becomes 17.

## CP247 `RezeroZoneSizingArrays` Pulse-to-Normal Selective Sizing Reset

CP247 adds canonical required `routine.rezero_zone_sizing_arrays` after
`fill_zone_sizing_from_input` and before `sim_zone_equipment`, plus the
matching HVAC project item. The exact wrapper is declared at
`ZoneEquipmentManager.hh` line 128 and implemented completely at
`ZoneEquipmentManager.cc` lines 1401-1430. Its reset dependency is
`ZoneSizingData::zeroMemberData`, declared at `DataSizing.hh` line 646 and
implemented at `DataSizing.cc` lines 131-278.

The sole production expression is `SizingManager.cc` lines 400-402 after a
Zone sizing iteration's end-of-calculation updates. When an accepted
component-load report is requested and `DoZoneSizing`, at least one Zone
sizing input, and sizing periods are present, the caller selects two Zone
sizing iterations and makes the first a pulse pass. When
`isPulseZoneSizing && runZeroingOnce`, the caller invokes CP247 and clears
`runZeroingOnce` only after normal return. The latch defaults true and
`SizingManagerData::clear_state()` rearms it. There is no `ErrorsFound` gate,
so the condition can also be evaluated after the no-sizing-period severe
path. CP247 itself changes neither pulse/report flags, the latch,
`ZoneSizingRunDone`, nor component-load pulse/decay storage.

The wrapper first unconditionally emits
`Re-zeroing zone sizing arrays`. It then traverses global Zone indexes in
ascending order, reads same-index `ZoneEquipConfig`, and skips uncontrolled
Zones. For every selected Zone and each
`D = TotDesDays + TotRunDesPersDays`, it resets normal daily then calculated
daily records. After all days it resets calculated-final before final. Only
after all Zones, `doSpaceHeatBalanceSizing` gates an ascending traversal of
all global Space indexes. A Space is selected solely when its stored parent
Zone's equipment configuration is controlled; no Space-local control flag or
parent `spaceIndexes` membership is checked. Space record order is likewise
normal daily, calculated daily, calculated-final, then final.

A nonpositive `D` skips daily records but not either final record. With
`Cz` controlled Zones and `Cs` global Spaces whose parent is controlled, a
completed valid-state wrapper dispatches

```text
(Cz + (doSpaceHeatBalanceSizing ? Cs : 0))
    * (2 * max(D, 0) + 2)
```

`zeroMemberData` calls. Each record independently applies one sentinel guard:
if `DOASSupMassFlowSeq` is not allocated, the whole helper returns silently
without changing that record. This sequence is allocation step 25 of 36 in
CP246, so a partial allocation before it can leave earlier arrays and every
member untouched. A passing guard zero-fills the current, independently
retained extents of exactly 36 sequence fields; it neither allocates,
redimensions, nor normalizes heterogeneous extents.

All 36 sequence fills precede exactly 104 selected member assignments:
12 strings become empty, 80 `Real64` values become `0.0`, and 12 integers
become zero. No bool, enum, pointer, or allocation state is assigned. The
strings cover eight sensible/latent with/without-DOAS design-day names and
four sensible/latent peak dates. The integers cover sensible/latent peak
timestep and design-day indexes. Reals cover selected design flows, loads,
densities, coil-inlet states, current sensible/latent/DOAS state, and
sensible/latent peak conditions.

This is not a blanket `ZoneSizingData` clear. It preserves CP246 identity,
input methods, temperatures/humidity targets, flows and factors, DOAS
configuration, concurrence, indexes/effectiveness, latent RH pointers and
methods, and heat-coil sizing fields. It also preserves EMS flags/values,
OA/People/area aggregates, non-air and several no-OA results,
`ZonePeakOccupancy`, scalar `DOASHeatAdd`/`DOASLatAdd`, selected no-DOAS and
latent peak metadata, and all other unlisted state. Component-load arrays
outside these records are untouched so the later decay/report pipeline keeps
its pulse evidence.

There is no local allocation, bounds, topology, extent, day-count, or old
state validation. Apart from the progress output, it emits no
warning/severe/fatal and mutates no error state; it owns no status, catch,
checkpoint, cleanup, transaction, or rollback. Output is committed before the first indexed read. Failure retains
source-order prefixes: earlier Zones; normal before calculated within a day;
calculated-final before final; every Zone before any Space; and earlier
Spaces. A malformed Space parent fails before the current Space record.
Within a guard-passing helper, all sequence fills precede the 104-member
assignment prefix. Ordinary owning records are distinct; every reset write is
zero or an empty string, and completed direct replay is idempotent over the
touched subset, but
it repeats the progress line and never repairs guard-skipped or unlisted
state. Production abnormal return leaves `runZeroingOnce` true and a partial
reset for a retry; successful return makes later same-state caller entries
skip CP247 until the sizing-manager state is cleared.

The focused C++ unit calls CP247 once with five controlled Zones, 12 design
days, three run-period design days, four timesteps, and no Spaces. It
dispatches 150 daily records whose sentinel is allocated and ten unseeded
final records whose guard returns. For both daily kinds, active checks cover
only 58 of 104 reset members and 28 of 36 sequences; 172 assertion source
lines execute 25,500 checks. The eight missing sequence oracles are the two
no-DOAS sensible, four latent/no-DOAS load, and two latent-flow sequences.
Another 46 reset members are not seeded or asserted. Seventy-five seeded
preserved members have no active preservation assertion; 154 expectation
lines are commented out. Final mutation, guard no-op, Space and uncontrolled
selection, display, latch, failure, and replay are not proved.

Exactly six fresh production contexts reach CP247: two direct
`ManageSizing` tests and four full simulations, all through
`AllSummaryAndSizingPeriod`. Their aggregate is nine controlled Zones, no
Spaces, two design days per role, and no run-period design day: 36 daily plus
18 final guard-passing records. Six records have extent 24 and 48 have extent
96, for 171,072 statically zero-filled sequence slots. Downstream checks
cover selected component-load reports, final sizing, and OA results only
after the following normal pass; none isolates the intermediate reset,
message, call count, flags, or latch transition.

Rust contains no exact `RezeroZoneSizingArrays`, `zeroMemberData`,
`runZeroingOnce`, `isPulseZoneSizing`, `ZoneSizingData`, Zone/Space sizing
record arena, or component-load pulse/reset/decay orchestration. Typed
Zone/Space identities, equipment graphs, IdealLoads scalar demand and limits,
OA helpers, time metadata, and a `sizing_checked` flag are adjacent only.
The raw `Sizing:Zone` fixture expects `UnsupportedSizing`, active cases have
neither executable sizing input nor component-load-summary requests, and
sizing remains run-blocked.

CP247 adds no algorithm-level EnergyPlus source, Rust target/code/state,
test, object support, capability, output implementation, comparator, case,
manifest, numerical, performance, or conformance promotion. The parent
algorithm remains `scaffold` with claim level `none`. Inventory becomes 32
algorithms and 252 routines, split 58 `state_mapped` plus 194
`source_mapped`, with 129 required; the heat-balance project list remains 88
and the HVAC list becomes 18.

## CP248 `updateZoneSizingBeginDay` Calculated Daily Metadata Seed

CP248 adds canonical required `routine.update_zone_sizing_begin_day` after
`rezero_zone_sizing_arrays` and before `sim_zone_equipment`, plus the matching
HVAC project item. The exact role-agnostic helper is declared at
`ZoneEquipmentManager.hh` line 132 and implemented completely at
`ZoneEquipmentManager.cc` lines 1431-1453.

The only helper call expressions are in the `UpdateZoneSizing` `BeginDay` arm.
The sole production parent expression is `SizingManager.cc` line 307, once per
non-warmup day in each retained, non-`RunPeriodWeather` Zone-sizing
environment and iteration, before Facility begin-day and the hourly
simulation. Component-load reporting executes pulse then normal iterations,
resets `CurOverallSimDay` for each, and therefore rewrites the same daily
records after CP247 runs between the passes.

The parent scans Zone indexes ascending, skips an uncontrolled Zone, writes
its `CalcZoneSizing(CurOverallSimDay, zone)`, then, only under
`doSpaceHeatBalanceSizing`, writes every
`CalcSpaceSizing(CurOverallSimDay, space)` in that Zone's stored
`spaceIndexes` order. There is no Space-local control check, global Space
scan, sort, deduplication, or membership/parent validation. For `C` controlled
Zones and `M` stored membership occurrences under them, one completed
begin-day parent call dispatches
`C + (doSpaceHeatBalanceSizing ? M : 0)` helpers. Only calculated daily
records are selected; normal daily and both final record families are
untouched. This Zone-then-its-Spaces order differs from CP247's all-Zones then
global-Spaces traversal.

The branchless helper performs exactly 20 ordered assignments:

1. `CoolDesDay` and `HeatDesDay` copy `EnvironmentName`;
2. `DesHeatDens` and `DesCoolDens` copy raw `StdRhoAir`;
3. `HeatDDNum` and `CoolDDNum` copy raw `CurOverallSimDay`;
4. six sensible/latent with/without-DOAS design-day names copy
   `EnvironmentName`;
5. six sensible no-DOAS and latent day indexes copy `CurOverallSimDay`;
6. `CoolSizingType` becomes `Cooling`, then `HeatSizingType` becomes
   `Heating`.

That is ten string, two `Real64`, and eight integer writes, with every source
read repeated rather than locally snapshotted. Empty names, non-finite or
negative density, and arbitrary direct-call day indexes are copied unchanged.
No sequence or result array is zeroed despite the parent's legacy BeginDay
comment. Outside the 20 named metadata members, identity/input state,
load/flow/condition peak values, peak timestep/date-string fields, OA/DOAS
load state, latent calculation state, EMS, pointers, extents, and allocation
persist; normal daily and both final record families are untouched. CP247 clears 16 of the 20
fields on a guard-passing pulse reset but preserves the two sensible no-DOAS
day indexes and both sizing-type strings; normal CP248 overwrites all 20.

There is no explicit allocation, local bounds/topology/finite/day validation,
diagnostic, status, latch, catch, checkpoint, cleanup, transaction, or
rollback. Parent lookup failure occurs before the current record is passed.
A later Space or helper failure retains prior Zone/Space records, and a string
assignment failure retains its statement prefix. Stable completed replay is
idempotent over only the 20-field subset; changed source values replace it,
while omitted state remains stale.

No C++ test calls CP248 directly. Two direct parent tests each dispatch one
controlled Zone, no Space, and day one with an empty environment name and
standard density zero or `1.20`; neither asserts a CP248 field. Across
production-style active tests, 105 parent begin-day calls dispatch 195
helpers: 153 Zone and 42 Space, split into 135 normal Zone, 42 normal Space,
and 18 pulse Zone writes. The sole direct member-name descendant is one
`CalcFinalZoneSizing.HeatDesDay` assertion after later peak/final processing.
Another 60 predefined-table design-day assertions are composite report
evidence. No immediate oracle covers the 20-write transaction, density/day
copies, labels, invalid state, topology, failure, replay, warmup, or
run-period-design-day behavior.

Rust has no exact helper, field family, overall sizing-day index,
Zone/Space calculated sizing arena, begin-day dispatcher, stored-Space
traversal, or downstream sizing peak/report transaction. Run-period timing,
design-day schedule labels, EIO parsing, standard-density-derived IdealLoads
limits, identities, and equipment graphs are adjacent only. Four active-case
IDFs contain raw design-day declarations but disable Zone sizing and the
runtime ignores them; the raw `Sizing:Zone` fixture remains run-blocked.

CP248 adds no algorithm-level source, Rust target/code/state, test, object
support, capability, output implementation, comparator, case, manifest,
numerical, performance, or conformance promotion. The parent algorithm
remains `scaffold` with claim level `none`. Inventory becomes 32 algorithms
and 253 routines, split 58 `state_mapped` plus 195 `source_mapped`, with 130
required; the heat-balance project list remains 88 and the HVAC list becomes
19.

## CP249 `updateZoneSizingDuringDay` System-Substep Sizing Accumulation

CP249 adds canonical required `routine.update_zone_sizing_during_day` after
`update_zone_sizing_begin_day` and before `sim_zone_equipment`, plus the
matching HVAC project item. The exact helper is declared at
`ZoneEquipmentManager.hh` lines 134-141 and implemented completely at
`ZoneEquipmentManager.cc` lines 1455-1506.

The only helper expressions are in the `UpdateZoneSizing` `DuringDay` arm.
The sole production parent expression is `HVACManager.cc` line 475, inside
the accepted `SysTimestepLoop` and under both `!WarmupFlag` and
`ZoneSizingCalc`. The full-zone trial and optional optimized-condenser HVAC
repeats have no separate CP249 expression: a no-downstep result is accumulated
once by the one-iteration loop, while adaptive downstepping recalculates and
accumulates once per smaller accepted system substep. Each call uses
`FracTimeStepZone = TimeStepSys / TimeStepZone` and one zone-timestep slot
computed from hour, timesteps per hour, and current timestep. All substeps of
that zone timestep share the slot.

The parent scans controlled Zones ascending. For each it passes current-day
normal and calculated Zone records, the Zone thermostat pair, and that Zone's
final high/low extrema, then conditionally visits stored Spaces in container
order. Space calls use Space normal/calculated records but reuse their parent
Zone's thermostat values and the same parent final extrema; there is no
`FinalSpaceSizing` target, Space-local control check, global scan, sort,
deduplication, or membership validation. One parent call dispatches
`C + (doSpaceHeatBalanceSizing ? M : 0)` helpers for controlled Zone count
`C` and stored membership occurrence count `M`.

The helper first applies two raw strict conditions. Positive `tstatHi` replaces
`sizTstatHi` only when greater. Positive `tstatLo` then replaces
`sizTstatLo` only when less than the possibly updated **high**. It never reads
the old low, whose declaration default is `1000.0`; low is the last eligible
positive value below current high, not a running minimum. NaN comparisons are
false, and equal, zero, or negative inputs do not update. Both possible
extrema writes are unweighted and occur before sequence access. Valid Space
calls cannot raise high after their preceding Zone call with identical input.

Next, CP249 unconditionally overwrites four normal-daily slots in exact order:
heating design setpoint, heating calculated thermostat temperature, cooling
design setpoint, and cooling calculated thermostat temperature. These copy
raw values without fraction weighting, so the last completed system substep
wins.

It then applies 22 unconditional calculated-daily
`destination += source * fracTimeStepZone` statements: seven heating
flow/load/Zone/outdoor/return/humidity fields, the analogous seven cooling
fields, and eight DOAS load/addition/supply fields. There is no
`AccountForDOAS` gate. When `zoneLatentSizing` is true, eight more weighted
additions follow: latent heating/cooling load and flow, four no-DOAS load
fields, including sensible `CoolLoadNoDOASSeq` and `HeatLoadNoDOASSeq` inside
the latent gate. A false gate preserves those eight elements.
`HeatFlowSeqNoOA` and `CoolFlowSeqNoOA` are never CP249 targets.

Thus one latent-false helper mutates 26 sequence elements and a latent-true
helper 34, plus zero to two extrema scalars. The four normal fields overwrite;
the 22 or 30 calculated fields accumulate. The fraction and the 22/30
additive source scalars are neither checked nor normalized; negative,
greater-than-one, zero, NaN, and infinite values follow raw IEEE arithmetic.
A zero fraction can still create NaN from infinity, and accumulation order can
change rounding.

CP246 provides initial array allocation/zeros. Under consistent production
topology, a completed guard-passing CP247 clears every sequence CP249 touches
between pulse and normal passes, but does not clear the two final extrema;
pulse extrema carry into normal. The different CP247 global-Space and CP249
stored-membership traversals do not guarantee that reset for malformed
topology. CP248 updates disjoint calculated-record metadata and is not a
prerequisite for a direct CP249 call. CP249 has no local reset, latch, allocation, bounds, extent,
timestep, fraction, topology, finite-value, role, diagnostic, status, catch,
checkpoint, cleanup, transaction, or rollback.

Possible extrema changes precede the first of 34 independent sequence
accesses. Failure retains that scalar and array-statement prefix. Retry
overwrites the four normal slots but repeats already committed `+=`
contributions, so stable replay is generally non-idempotent. Duplicate Space
membership also double-adds. Parent argument lookup failure occurs before
helper entry, with argument evaluation order unspecified. Production storage
is distinct, but direct callers can alias the two records, high/low refs, or a
scalar ref with record state and thereby alter later reads; no alias guard
exists.

No C++ test calls the helper directly. Two direct parent tests each use one
Zone, no Space, one slot, unit fraction, latent false, and positive thermostat
pairs. Both extrema conditions succeed, but neither extrema nor any sequence
element is asserted. The only test expectations naming the 26 unconditional
sequence fields belong to CP247's manually seeded reset test, which never
calls DuringDay; the eight latent-gated sequences and both extrema have no
direct oracle.

Adaptive traces are absent, so production-style active-test calls are not
exactly measured. Their one-system-substep nominal floor is 12,288 parent
calls and 23,424 helpers: 17,376 Zone plus 6,048 Space, with 21,840 normal and
1,584 pulse helpers. The latent gate is true for a nominal 3,744 and false for
19,680. Adaptive downsteps can increase those counts. Later final
thermostat/peak and sizing-table assertions are composite evidence after
moving average, peak selection, final propagation, and reporting, not
isolating CP249.

Rust has no exact helper, fraction, extrema, sequence family,
normal/calculated Zone/Space sizing records, dispatcher, or accumulation
transaction. Thermostat links, diagnostic setpoint series, demand snapshots,
IdealLoads timing, and adaptive run-period heat-balance averages are adjacent
only. No active case has `Sizing:Zone`; raw design days disable sizing and are
ignored, while the raw sizing fixture remains run-blocked.

CP249 adds no algorithm-level source, Rust target/code/state, test, object
support, capability, output implementation, comparator, case, manifest,
numerical, performance, or conformance promotion. The parent algorithm
remains `scaffold` with claim level `none`. Inventory becomes 32 algorithms
and 254 routines, split 58 `state_mapped` plus 196 `source_mapped`, with 131
required; the heat-balance project list remains 88 and the HVAC list becomes
20.

## CP250 `updateZoneSizingEndDayMovingAvg` Circular End-Day Smoothing

CP250 adds canonical required
`routine.update_zone_sizing_end_day_moving_avg` after
`update_zone_sizing_during_day` and before `sim_zone_equipment`, plus the
matching HVAC project item. This is the physical source-definition order. The
public helper is declared at `ZoneEquipmentManager.hh` line 143 and its
complete wrapper is `ZoneEquipmentManager.cc` lines 1508-1529:

```cpp
void updateZoneSizingEndDayMovingAvg(
    DataSizing::ZoneSizingData &zsCalcSizing,
    int const numTimeStepsInAvg);
```

The body has one `if`, no direct assignment, and at most 16 ordered
`General::MovingAvg` child calls. Twelve calculated-daily sequences are
unconditional:

```text
CoolFlowSeq
CoolLoadSeq
HeatFlowSeq
HeatLoadSeq
CoolZoneRetTempSeq
HeatZoneRetTempSeq
DOASHeatAddSeq
DOASLatAddSeq
CoolLatentLoadNoDOASSeq
HeatLatentLoadNoDOASSeq
CoolLoadNoDOASSeq
HeatLoadNoDOASSeq
```

There is no `AccountForDOAS` gate, and all four no-DOAS fields remain in this
unconditional set. Only when `zoneLatentSizing` is true does the wrapper then
smooth `LatentHeatLoadSeq`, `LatentHeatFlowSeq`, `LatentCoolLoadSeq`, and
`LatentCoolFlowSeq`, in that order. It targets only the current calculated
Zone/Space daily record. It does not touch normal-daily thermostat sequences,
final records, any scalar, either no-OA flow sequence, or CP249's remaining
14 calculated temperature, humidity, DOAS-load, and DOAS-supply sequences.

`General::MovingAvg` is declared at `General.hh` line 107 and implemented at
`General.cc` lines 374-393. For `N <= 1` it returns before inspecting or
allocating the array. For `N > 1` and extent `L`, it allocates `2L` scratch
elements, duplicates the original array into both halves while zeroing the
target, then evaluates:

```text
out(i) = sum(j = 1..N, scratch(L - N + i + j)) / N
```

For `2 <= N <= L`, this is a circular trailing mean of the current element
and the preceding `N - 1`, so early-day outputs wrap through end-of-day
samples. `N = L` is a whole-day mean. `N = L + 1` is still in bounds but
weights the current element twice; an empty array skips both loops. For a
positive extent and `N > L + 1`, unsigned index arithmetic reaches an invalid
element. ObjexxFCL asserts membership before raw storage access, so that
invalid index terminates with assertions enabled and has undefined behavior
otherwise; it is not a recoverable throw. Non-one-based arrays are likewise
unsupported by the hard-coded `1..size` traversal. No local guard normalizes
the window to the extent.

Production sequence extent is `24 * TimeStepsInHour`. The
`Sizing:Parameters` averaging-window field is an integer with minimum one and
no upper maximum. Blank, absent, nonpositive source fallback, and fast-mode
override paths select `TimeStepsInHour`; the only range warning is for a
window shorter than one hour. There is no upper clamp. Raw ordered additions
and division have no finite-value guard, so NaN, infinity, overflow, and
rounding behavior propagate. Each child snapshots its own entire target
before output, but a second completed call generally smooths the already
smoothed result and is not idempotent.

The `UpdateZoneSizing(EndDay)` parent first completes one entire smoothing
sweep: controlled Zones in ascending index order, each Zone first and then
its stored `spaceIndexes` when Space sizing is enabled. It passes only
`CalcZoneSizing(CurOverallSimDay, zone)` or
`CalcSpaceSizing(CurOverallSimDay, space)` and the one global window. There is
no Space-local control check, global Space scan, sort, deduplication,
membership validation, or parent validation. With `C` controlled Zones, `M`
stored membership occurrences, and `R` latent-true role occurrences, a
completed valid-state parent dispatches:

```text
H = C + (doSpaceHeatBalanceSizing ? M : 0)
helper calls = H
MovingAvg calls = 12 * H + 4 * R
```

Duplicate or cross-listed Space indexes therefore smooth the same calculated
record repeatedly. Only after the full CP250 sweep completes does the parent
start its analogous CP251 `updateZoneSizingEndDay` peak-selection sweep.
CP251 sees every role's fully smoothed arrays, including any compounded
duplicate. It selects peaks from smoothed load fields and reads paired
smoothed flow/return-temperature fields, but samples unsmoothed Zone/outdoor
temperature and humidity companions at those selected timesteps. CP250 writes
no peak or final scalar.

The sole production parent expression is `SizingManager.cc` line 374, after
all hourly/timestep work for a completed non-warmup sizing day and before
facility end-day processing or the current-overall-day increment. The parent
has no equivalent local guard, so direct calls bypass that cadence. A
load-component pulse pass also reaches CP250. After a successful pulse sizing
iteration, a guard-passing CP247 clears the selected arrays before the normal
pass under consistent topology. Because CP247 globally scans Spaces while
CP250 follows stored membership, malformed cross-listing can evade that reset.

CP250 has no status, diagnostic, catch, checkpoint, transaction, cleanup, or
rollback. Scratch construction `std::bad_alloc` leaves the current target
untouched but retains earlier child and role results. Once scratch exists, the
loops have no source-defined recoverable exception path. Invalid indexing
assert-terminates or has undefined behavior, so no post-failure state or retry
is guaranteed. Only as a hypothetical statement-order interruption model, the
copy loop could expose a zeroed prefix and the averaging loop could expose
completed outputs, a partial current element, and later zeros; this is not a
recoverable C++ guarantee. Defined re-entry after a completed call or caught
allocation failure starts at the first role and smooths prior completed arrays
again. Scratch-allocation non-return suppresses every CP251 call; a later
CP251 non-return occurs after all CP250 mutations are committed. Production
array members are distinct, so duplicate/cross-listed record identity is the
material same-record replay route.

No C++ test calls CP250 directly. Two unit tests call the EndDay parent
directly with one Zone, no Space, latent sizing false, extent one, and
`N = 1`; all 12 child calls return immediately and no assertion reads a CP250
target. The independent `General_MovingAvg` test uses a 12-element quadratic
array and checks all 12 outputs for `N = 1`, `N = 2`, and `N = 4`. It proves
the child algorithm, not CP250's field set, order, gate, or parent routing.

A fresh completing production-style census finds 105 parent calls and 195
helpers: 153 Zone plus 42 Space, split 177 normal and 18 pulse. Helper windows
are exactly `N = 1/4/6` in counts `4/87/104`; the 191 `N > 1` helpers perform
real smoothing. The latent gate is true for 26 helpers, all at `N = 6`, and
false for 169. Thus the corpus dispatches exactly 2,444 child calls: 48 no-op
calls at `N = 1`, 1,044 transformations at `N = 4`, and 1,352 at `N = 6`.
Unlike CP249, there is no adaptive-system-substep multiplier.

Eight `SizingManager` production runs assert exact downstream Zone/Space
design load, flow, design-day, and peak-time report values at `N = 6`,
including one final Space latent-cooling peak timestep. Those are composite
results after CP249 accumulation, CP250 smoothing, CP251 selection, final
propagation, and reporting. The reset test asserts eight overlapping
sequences but never calls CP250. No focused oracle covers all 16 targets,
parent `N > 1` before/after values, either latent-gate branch with sentinels,
duplicate topology, invalid windows/extents, IEEE-special values, scratch
allocation failure, invalid-access termination or undefined behavior,
hypothetical statement-order interruption, defined re-entry, or replay.

Exact `crates` and `data` searches find no CP250 helper or canonical key,
`MovingAvg`/`moving_avg`, `NumTimeStepsInAvg`, `Sizing:Parameters`,
`ZoneSizingData`, `zoneLatentSizing`, or any of the 16 target sequences. Rust
has no Zone/Space sizing-day arena, circular trailing-window transaction,
EndDay dispatcher, stored-Space sizing traversal, or peak-selection handoff.
Adaptive heat-balance weighted averages, schedule averages, report-frequency
`Average` classification, and run-period time state are adjacent only and do
not implement this design-day mutation.

No active case contains `Sizing:Zone` or `Sizing:Parameters`. Four active-case
files contain five raw `SizingPeriod:DesignDay` objects, but all disable Zone,
System, and Plant sizing and are ignored by the compatibility runtime. The
raw `Sizing:Zone` fixture expects `UnsupportedSizing`; sizing and authored
Space/SpaceList workflows remain run-blocked.

CP250 adds no algorithm-level EnergyPlus source, Rust target/code/state, test,
object support, capability, output implementation, comparator, case,
manifest, numerical, performance, or conformance promotion. The parent
algorithm remains `scaffold` with claim level `none`. Inventory becomes 32
algorithms and 255 routines, split 58 `state_mapped` plus 197
`source_mapped`, with 132 required; the heat-balance project list remains 88
and the HVAC list becomes 21.

## CP251 `updateZoneSizingEndDay` Daily Peak and Final-Period Reduction

CP251 adds canonical required `routine.update_zone_sizing_end_day` after
`update_zone_sizing_end_day_moving_avg` and before `sim_zone_equipment`. Its
complete source is `ZoneEquipmentManager.hh` lines 145-149 and
`ZoneEquipmentManager.cc` lines 1531-1944:

```cpp
void updateZoneSizingEndDay(
    DataSizing::ZoneSizingData &zsCalcSizing,
    DataSizing::ZoneSizingData &zsCalcFinalSizing,
    int const numTimeStepInDay,
    DataSizing::DesDayWeathData const &desDayWeath,
    Real64 const stdRhoAir);
```

The leaf has no EnergyPlus child, state argument, diagnostic, status, or
return value. It first overwrites final `CoolSizingType` then
`HeatSizingType` from the current day, so those strings follow the latest
call rather than necessarily the winning period.

Its ordered daily reducers use strict `>` throughout:

- sensible heat writes load, smoothed flow, Zone/outdoor/return conditions,
  humidity companions, and timestep;
- latent heat does the analogous nine writes only under current-record latent
  sizing, assigning both latent heat mass fields from one flow;
- one unconditional loop reduces sensible/latent heat and cool no-DOAS loads,
  with no latent or `AccountForDOAS` gate;
- sensible cool then optional latent cool follow; latent cool does not assign
  `ZoneCoolLatentMassFlow`.

With zero incumbents, only positive values win, ascending ties retain the
first timestep, NaN candidates lose, and a NaN incumbent blocks later
candidates. CP251 selects every ordinary load peak above from a
CP250-smoothed load array. Where the sensible and enabled latent reducers
have flow and return-temperature companions, they sample CP250-smoothed
values; their Zone/outdoor temperature and humidity companions remain
unsmoothed. CP251 reads no DOAS sequence, including the two DOAS-addition
arrays CP250 smooths.

Positive sensible mass becomes volume by its stored sensible density. OA
fraction is `clamp(MinOA / max(volume, 0.001), 0, 1)`, and current-day weather
is mixed with the sensible Zone peak. Latent mass divides by `stdRhoAir`, but
its OA denominator is still the corresponding sensible volume and its Zone
side still uses the sensible peak; only the weather index is latent. The four
mass-flow derivations are strictly positive-gated, but there is no finite
validation, no zero/sign validation of any density divisor, and no peak-index
bounds check. The ordered source clamp maps a NaN raw fraction to zero, but
multiplication does not short-circuit, so zero-weight NaN/infinity can still
propagate.

Final selection is flow-first and can form hybrid records:

| family | larger-volume branch | volume `else` / larger-load branch |
|---|---|---|
| sensible heat/cool | copies 22 fields, including volume/mass, seven sequences, five peaks, identifiers, density, and coil inputs; thermostat is omitted | unconditionally overwrites final density with `stdRhoAir`, then a larger load copies 19 fields including thermostat but retains prior volume, mass, and flow sequence |
| latent heat | copies 14 fields, mass from `ZoneHeatLatentMassFlow`, no outdoor latent peak | copies only load, date/DD/time, and load+flow sequences; day name and peak/coil/flow scalars remain stale |
| latent cool | analogous 14 fields, mass from `DesLatentCoolMassFlow`, no outdoor latent peak | copies load, date/DD/day/time and only the load sequence |
| four no-DOAS loads | each strict winner copies scalar, sequence, DD, day, and time | no alternate branch and no date-string copy |

Strict cross-day ties retain the prior winner. A larger-volume day may lower
the associated final load; a lower/equal-volume but higher-load day may
replace load companions while retaining prior flow state. Any sensible
volume loser overwrites the selected density even when its load also loses.

Four zero-load fallbacks follow. Sensible heat chooses the within-day minimum
Zone temperature, then inclusively prefers a lower/equal paired outdoor
temperature across days and copies 17 companion fields. Sensible cool chooses
the within-day maximum and strictly prefers a higher paired outdoor
temperature. Latent heat selects the current-day minimum Zone temperature
while mutating only the current-day record's latent Zone temperature and
paired outdoor temperature/humidity. An independent scan selects the final
minimum outdoor temperature and its humidity companion plus metadata, but
copies the current-day record's existing latent timestep rather than the loop
index; it copies no final latent Zone peak or sequence. Latent cool mutates
a running current-day maximum, compares its paired outdoor value against a
final threshold that CP251 never updates, and writes only day/DD/date/stale
time. It is not a maximum reducer.

The EndDay parent completes the entire CP250 Zone/Space smoothing sweep before
starting this CP251 sweep. It scans controlled Zones ascending, then each
stored Space in order when Space sizing is enabled, with no local Space
control, global scan, sort, deduplication, membership, or parent validation.
Duplicate/cross-listed Space identity repeats CP251 against the same daily and
final pair after CP250 has already multiply-smoothed it. The sole production
call is once per completed non-warmup sizing day before facility EndDay and
the day-index increment; direct callers bypass those guards.

A pulse sizing iteration also reaches CP251. CP247 later resets most fields
before normal sizing, but preserves the sensible no-DOAS heat/cool peak
timesteps in daily and final records, their DD numbers in final records, and
the latent heat/cool Zone peak temperature/humidity in both; CP248/CP251
overwrite the also-preserved sizing labels.
No-winner normal paths can retain that pulse state; no current test has a
latent pulse role. Malformed stored membership can also evade CP247's
actual-parent Space scan.

`T <= 0` skips all nine possible loops but not stale-scalar finalization.
Out-of-range sequence/weather access assertion-terminates or has undefined
behavior and supplies no defined continuation. String and whole-array copies
can allocate after a strict winning scalar has committed; a caught allocation
failure can leave companions incomplete, and equality makes retry skip that
branch. Successful replay can also be non-idempotent: an equal sensible
volume enters `else` and can replace the copied density with `stdRhoAir`.
Parent replay first reruns CP250. The two references may alias, collapsing
ordinary final strict winner comparisons into self-comparisons and producing
a separate unvalidated in-place hybrid.

No C++ test calls CP251 directly. Two direct EndDay parent tests each use one
Zone, no Space, extent one, and latent false, but their four peak assertions
occur after helper 7 rewrites those peaks; they prove only integrated reach.
A BaseSizer full simulation pins the positive-heating-load/zero-flow fallback.
One latent Space simulation pins calculated-final latent cooling timestep 72.
Seven Space-sizing simulations assert downstream Space load, flow, day, and
peak-time reports.

The completing production-style corpus has exactly 105 parents and 195
helpers: 153 Zone plus 42 Space, split 177 normal plus 18 pulse. Helper
extents 24/96/144 occur 4/87/104 times; 26 latent-true helpers all have extent
144. The fixed daily peak scans execute 77,760 loop bodies and 148,032
comparisons. Six DOAS-enabled Zones contribute 14 calls, all latent false;
CP251 has no DOAS branch, and no test asserts a final no-DOAS field. Ties,
latent heat/zero fallbacks, hybrid/density behavior, latent coil asymmetry,
invalid state, aliasing, failure, retry, and pulse omissions remain
unisolated.

Exact Rust/data searches find no helper/key, calculated Zone/Space sizing-day
or final arena, peak reducer, or any of the 103 accessed members in token or
snake-case form. Current-timestep demand, IdealLoads limits/OA mixing, warmup
extrema, and sizing-object-name detection are adjacent only. Active cases
contain no `Sizing:Zone`, `Sizing:Parameters`, authored Space, authored
SpaceList, or Zone-sizing-enabled `SimulationControl`; sizing remains
run-blocked.

CP251 adds no EnergyPlus algorithm source, Rust target/state, support, output,
case, numerical, performance, or conformance promotion. Counts become 32
algorithms and 256 routines, split 58 `state_mapped` plus 198
`source_mapped`, with 133 required; heat-balance/HVAC lists become 88/22 and
HVAC readiness remains `0/22`. The parent stays `scaffold` with claim level
`none`.

## CP252 `updateZoneSizingEndZoneSizingCalc1` Noncoincident Space Aggregation

CP252 adds canonical required
`routine.update_zone_sizing_end_zone_sizing_calc1` after
`update_zone_sizing_end_day` and before `sim_zone_equipment`. Its complete
source is `ZoneEquipmentManager.hh` line 151 and
`ZoneEquipmentManager.cc` lines 1946-2278:

```cpp
void updateZoneSizingEndZoneSizingCalc1(EnergyPlusData &state,
                                        int const zoneNum);
```

The leaf has no EnergyPlus child, diagnostic, output, status, catch, or return
value. It writes 92 calculated-final Zone members and accesses 95 unique
sizing-record member names across the Zone target and Space sources. It has
six explicit loops plus four ordinary and four latent-gated
`std::max_element` scans.

The sole production parent reaches EndZone sizing from `SizingManager` only
after at least one sizing period. It first runs Zone-sizing EMS, then
independently applies each of six Zone volume/mass/load overrides only when
EMS is present, that actuator's flag is on, and its preoverride target is
strictly positive. Only inside the non-pulse block, Space
sizing then visits controlled Zones ascending, skips exactly
`Zone.numSpaces == 1`, and calls CP252. The leaf binds the calculated-final
Zone record and returns unchanged only for exact `Coincident`; NonCoincident
and Invalid values rebuild. A normally completing non-Coincident call
therefore resets and rebuilds all
six EMS-adjustable fields from Space aggregates, including any applied
override.

The leaf does not recheck pulse, Space sizing, control, Zone bounds,
`numSpaces`, list length, membership parent, duplicates, cross-listing, Space
latent flags, or extents. It indexes `spaceIndexes[0]` after its numeric reset,
so malformed empty topology fails after that prefix. Stored order and
multiplicity are authoritative. A local Space counter increments but is
unused.

For target `F`, raw timestep count `T`, and latent gate `L`, reset and fold are:

| phase | sensible/unconditional | latent-gated |
|---|---|---|
| reset scalars | eight volume/load/mass/no-DOAS sums and 16 density/peak/coil numerators | eight latent sums and ten latent peak/coil numerators |
| reset arrays over `1..T` | 16 flow/load/no-DOAS/condition arrays | six latent load/flow/no-DOAS arrays |
| first-Space seed | 11 heat, heat-no-DOAS, and cool day/DD/date/timestep fields | 14 latent fields plus the three *ordinary* cool-no-DOAS fields |
| each Space | eight scalar sums; 16 peak products weighted by sensible design mass; six sequence sums; ten condition products weighted by timestep flow | eight latent sums; ten peak products; four DD checks; six sequence sums |

The ordinary cool-no-DOAS first-Space seed being inside `L` means a nonlatent
call begins that DD/name consensus from incoming Zone state. The first-Space
timestep copies are later replaced by maximum scans on normal completion.

Every ordinary or latent consensus compares only DD numbers. While the
current DD is nonzero, the first mismatch changes a primary day/DD/date to
`"N/A"/0/""` or a no-DOAS DD/day to `0/"N/A"`. Zero then latches off all
later comparisons. A first DD of zero suppresses mismatch detection from the
start; names, dates, and timesteps are never compared independently.

Sensible peak companions are divided by summed design mass only when it is
strictly positive. Timestep condition numerators are divided by summed
timestep flow only when positive. The four ordinary maximum scans then run.
Only afterward, under `L`, latent heat uses summed Space
`ZoneHeatLatentMassFlow` for both numerator weight and denominator, while
latent cool weights five peak/coil numerators by Space
`DesLatentCoolVolFlow` but divides by summed `DesLatentCoolMassFlow`; four
latent maximum scans follow. Nonpositive or NaN denominators leave raw
weighted sums; positive infinity enters division.

Each maximum scan recomputes a one-based timestep from its full allocated
array extent, not `1..T`:

- sensible heat/cool and their no-DOAS fields use their load arrays;
- latent heat uses `LatentHeatFlowSeq`, while latent cool uses its load array;
- latent no-DOAS fields use their corresponding no-DOAS load arrays.

Finite ties retain the first maximum; a portable NaN selection rule is not
claimed. Scalar loads/flows remain sums of independent Space peaks, so the
aggregate-sequence timestep can describe a different coincident peak.
Untouched tails can win when `T` is smaller than an extent.

CP252 is a subset rebuild. Thermostats, sizing configuration/labels, latent
outdoor peaks, Zone latent mass fields, DOAS state, EMS flags/values, and many
identity/input fields remain from the pre-call Zone record. The result can
therefore mix Space sums, weighted Space conditions, sequence maxima,
consensus/stale metadata, and untouched Zone state.

`T <= 0` skips timestep reset/fold/normalization but not scalar work,
metadata, or full-array scans. Excess `T`, invalid indexes, or malformed
extents assertion-terminate or have undefined unchecked behavior after an
ordered prefix. Floating sums/products preserve raw source-order IEEE
effects. String copies can allocate. Every no-DOAS mismatch arm sets DD
zero before
`"N/A"`, so a failure can leave a torn label in that invocation. Heat and
latent no-DOAS fields are reseeded from the first Space on retry; only
ordinary cool-no-DOAS under latent false lacks that reseed and can retain the
zero latch while skipping label repair. Stable valid replay normally
reconstructs the touched numerical subset, but it does not repair untouched
fields, tails, or that nonlatent cool-no-DOAS state.

Pulse EndZone skips CP252 entirely; the normal pass can later aggregate
pulse-preserved Space omissions. CP253 runs for all controlled Zones and
stored Spaces only after the complete CP252 Zone sweep, then owns diagnostics
and peak timestamp strings. Reporting and calculated-to-user copies remain
downstream.

No C++ test calls CP252 directly; two direct parent tests are pulse-gated and
dispatch none. Across 57 completing production-style EndZone parent entries,
only seven normal full simulations call CP252: five Coincident returns and
two NonCoincident bodies. Each call has one Zone, three Spaces, and `T = 144`;
the bodies are latent false and total six Space visits, 1,440 explicit
timestep-loop iterations, and eight maximum scans over 1,152 elements.

The two `SizingManager_ZoneSizing_NonCoincident*` tests strongly assert
downstream cooling load/volume Space sums. The common-day case retains the
day and reports `7/21 16:00:00`; the different-day case reports day `"N/A"`
and time-only `16:00:00`. Five Coincident tests retain Zone values distinct
from Space sums. There is no executed latent, positive-heating, no-DOAS,
DOAS, EMS, pulse, weighted-field, malformed-topology, IEEE, failure, replay,
or retry oracle.

Exact Rust/data searches find no helper/key, concurrence type/value,
calculated-final Zone/Space arena, or any of the 95 sizing members in token or
snake-case form. Typed Zone/Space topology, demand, equipment sequences,
autosize wrappers, counters, and sizing-object names are adjacent only.
Active data contain no `Sizing:Zone`, `Sizing:Parameters`, authored
`Space`/`SpaceList`, `NonCoincident`, Space-sizing enablement, or
Zone-sizing-enabled `SimulationControl`; sizing and Space partitioning remain
run-blocked.

CP252 adds no EnergyPlus algorithm source, Rust target/state, support, output,
case, numerical, performance, or conformance promotion. Counts become 32
algorithms and 257 routines, split 58 `state_mapped` plus 199
`source_mapped`, with 134 required; heat-balance/HVAC lists become 88/23 and
HVAC readiness remains `0/23`. The parent stays `scaffold` with claim level
`none`.

CP253 next maps
`ZoneEquipmentManager::updateZoneSizingEndZoneSizingCalc2`, declared at
`ZoneEquipmentManager.hh` line 153 and implemented completely at
`ZoneEquipmentManager.cc` lines 2280-2387, together with its
`sizingPeakTimeStamp` dependency declared at header line 162 and defined at
source lines 2389-2399.

The inventory now also includes `update_final_surface_heat_balance` after
`zone_space_heat_balance_calc_predicted_system_load`,
preserving the completed predictor/corrector definition slice before
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
`zone_space_heat_balance_begin_environment_init` /
`zone_space_heat_balance_set_up_output_vars` / `predict_system_loads` /
`zone_space_heat_balance_predict_system_load` /
`calc_zone_air_temp_set_points` /
`zone_space_heat_balance_calc_predicted_humidity_ratio` /
`correct_zone_air_temps` /
`zone_space_heat_balance_correct_air_temp` /
`push_zone_timestep_histories` /
`zone_space_heat_balance_push_zone_timestep_history` /
`push_system_timestep_histories` /
`zone_space_heat_balance_push_system_timestep_history` /
`revert_zone_timestep_histories` /
`zone_space_heat_balance_revert_zone_timestep_history` /
`zone_space_heat_balance_correct_hum_rat` /
`down_interpolate_4_history_values` entries, this
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
