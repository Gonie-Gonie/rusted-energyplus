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

The inventory now also includes `calc_heat_balance_inside_surf` immediately
after `calc_heat_balance_outside_surf` and before the distinct optimized
`calc_heat_balance_inside_surf_2_ctf_only` child. Its EnergyPlus boundary is
the unconditional parent line-172 `CalcHeatBalanceInsideSurf(state)` call,
which omits the optional Zone-resimulation argument, and the canonical wrapper
at lines 7738-7813. It remains `source_mapped` and required. That wrapper owns
first-call and BeginEnvrn lifecycle, radiant-HVAC aggregation, complete versus
partial and AllCTF versus general dispatch, MRT calculation, and intermediate
result updates; its dependencies own the complete general iteration,
surface/window/moisture/Kiva topology, non-local partial-resimulation side
effects, errors, and the pass-by-value warmup-counter reachability boundary.
Existing Rust inside-balance stage metadata, its identity wrapper, bounded
surface passes, and the separate CTF-only routine mapping do not implement or
promote this complete canonical routine, state, lifecycle, dispatch, error
behavior, or numerics.

The required inventory now places
`update_intermediate_surface_heat_balance_results` after the optimized
`calc_heat_balance_inside_surf_2_ctf_only` child and before
`manage_air_heat_balance`, preserving the canonical inside-balance
parent/optimized-child/tail grouping before the Air subtree. Its EnergyPlus
boundary is the sole production call after `CalculateZoneMRT` at
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

The inventory now also includes `update_final_surface_heat_balance` after
`manage_zone_air_updates`, preserving the completion of the Air subtree before
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
`manage_air_heat_balance` and nested `manage_zone_air_updates` entries, this
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
