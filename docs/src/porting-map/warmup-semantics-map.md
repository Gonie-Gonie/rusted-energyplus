---
status: active
claim_level: planning-guard
owner: runtime
last_reviewed: 2026-06-09
---

# Warmup Semantics Map

Reference version: EnergyPlus 26.1.0

Purpose: make warmup handling explicit before any broader official ExampleFile
dynamic conformance claim is made.

## Current Rule

Current promoted numerical reports compare only reported run-period ESO series.
Warmup iterations are not represented as Rust output samples. This is acceptable
only for the currently declared local no-mass, schedule, weather, and internal
gain cases.

`official_1zone_uncontrolled_dynamic_diagnostic_001` now runs a diagnostic
Rust warmup loop before recording run-period samples. The loop repeats the
first run-period weather day, records Rust warmup days/timesteps/convergence,
passes available EPW records through the same exterior surface forcing context
used by reported run-period timesteps, and stores the EnergyPlus EIO run-period
`Environment:WarmupDays` value in the compare summary. This is instrumentation
for diagnosis only; it is not EnergyPlus warmup parity.

## Required Official ExampleFile Work

| Topic | Requirement |
|---|---|
| warmup convergence | record EnergyPlus warmup days/iterations and the Rust equivalent |
| initial histories | initialize zone air and surface histories from the same semantic state |
| reporting filter | prove that compared hourly samples exclude warmup exactly as EnergyPlus ESO does |
| failure diagnosis | if first-hour deltas dominate, report whether the cause is warmup, initial history, or algorithm mismatch |
| gate policy | do not set `conformance_claim=true` for official dynamic cases until warmup handling is part of the report |

## Current Boundary

Official `1ZoneUncontrolled` dynamic outputs are baseline and diagnostic
candidates. `official_1zone_uncontrolled_dynamic_diagnostic_001` currently
keeps the case `conformance_claim=false` while reporting first-hour,
run-period-filtered deltas, Rust warmup metadata, and oracle run-period warmup
days. If Rust warmup day count, initial histories, surface CTF history, or
post-warmup hourly values differ from EnergyPlus, the case must remain
diagnostic.

Developers can run `scripts\dev.cmd official-dynamic-heat-balance-warmup-20-probe`
to raise the Rust diagnostic warmup minimum to the EnergyPlus run-period warmup
day count for the official 1Zone case. That lane is diagnostic-only and exists
to isolate whether current first-hour and floor-history deltas are driven by
early Rust warmup convergence or by deeper CTF/iteration differences.
`scripts\dev.cmd official-dynamic-heat-balance-all-ctf-warmup-20-probe` applies
the same warmup minimum while enabling all EIO CTF rows. Current probe evidence
shows only negligible movement from the all-CTF lane, so warmup day count alone
is not the mass-floor fix. EPW exterior forcing is now used during Rust warmup
when full weather records are available; this source-match refinement only
nudges current official dynamic metrics and does not change the mass-floor
CTF/history promotion blocker. Re-running the current best converged
frozen-reference-air/current-longwave lane with the EnergyPlus
`SurfInitialTemp`-shaped initial CTF history policy is bit-identical after
warmup, so the next warmup-facing work is the repeated-day history evolution
and source-order handoff into the run period, not the pre-warmup seed values
alone.
