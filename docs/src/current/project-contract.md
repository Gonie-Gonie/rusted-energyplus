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

The canonical heat-balance inventory includes `init_heat_balance` after the
required input/view-factor/internal-gain routines and before
`manage_surface_heat_balance`. Its EnergyPlus boundary is the unconditional
`HeatBalanceManager::InitHeatBalance` call at line 198 and the flag-driven
implementation at lines 2594-2821. It remains `source_mapped` and required:
the current Rust execution-plan stage, identity wrapper, and separately
bounded initialization state do not complete or promote this routine.

The inventory now also includes `init_surface_heat_balance` immediately after
`manage_surface_heat_balance`. Its EnergyPlus boundary is the unconditional
`HeatBalanceSurfaceManager::InitSurfaceHeatBalance` call at parent line 161
and the flag-driven implementation at lines 272-621. It remains
`source_mapped` and required. The existing Rust
`init_surface_heat_balance_stage` metadata and identity wrapper surround only
a limited outside-balance closure, remain intentionally absent from the
surface algorithm's target list, and do not implement or promote the complete
source routine, state, lifecycle, output, or numerical behavior.

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
