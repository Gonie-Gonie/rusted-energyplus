---
status: active
claim_level: planning-guard
owner: runtime
last_reviewed: 2026-06-12
---

# Warmup Semantics Map

Reference version: EnergyPlus 26.1.0

Purpose: make warmup handling explicit before any broader official ExampleFile
dynamic conformance claim is made.

## Current Rule

Current promoted numerical reports compare only reported run-period ESO series.
Warmup iterations are not represented as Rust output samples. This is acceptable
for the declared local no-mass, schedule, weather, internal-gain cases and the
official 1Zone dynamic compatibility candidate, whose report records warmup
metadata and compares only run-period samples.

`official_1zone_uncontrolled_dynamic_conformance_candidate_001` runs a Rust
warmup loop before recording run-period samples, using the EnergyPlus EIO
run-period warmup day count as the minimum. The loop repeats the first
run-period weather day, records Rust warmup days/timesteps/convergence, passes
available EPW records through the same exterior surface forcing context used by
reported run-period timesteps, and stores the EnergyPlus EIO run-period
`Environment:WarmupDays` value in the compare summary. This is sufficient for
the declared candidate variables; it is still not a broad EnergyPlus warmup
convergence parity claim.

## Required Official ExampleFile Work

| Topic | Requirement |
|---|---|
| warmup convergence | record EnergyPlus warmup days/iterations and the Rust equivalent |
| initial histories | initialize zone air and surface histories from the same semantic state |
| reporting filter | prove that compared hourly samples exclude warmup exactly as EnergyPlus ESO does |
| failure diagnosis | if first-hour deltas dominate, report whether the cause is warmup, initial history, or algorithm mismatch |
| gate policy | allow `conformance_claim=true` only for the official dynamic compatibility candidate where warmup metadata, run-period filtering, EIO CTF seeding, and the blocking gate all pass |

## Current Boundary

Official `1ZoneUncontrolled` dynamic outputs are split between a broad
diagnostic tracker and the promoted compatibility candidate.
`official_1zone_uncontrolled_dynamic_conformance_candidate_001` now uses
`conformance_claim=true` only for the declared weather, zone-air,
face-temperature, and conduction variables. The broad diagnostic case remains
`conformance_claim=false`, and storage/radiation/solar/convection diagnostics
must not inherit the candidate claim.

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

EnergyPlus 26.1.0 `HeatBalanceManager.cc::ManageHeatBalance` checks warmup
convergence only after `ManageSurfaceHeatBalance`, end-zone-timestep EMS,
`RecKeepHeatBalance`, and `ReportHeatBalance`, and only when `WarmupFlag` and
`EndDayFlag` are both true. When convergence clears `WarmupFlag`, EnergyPlus
resets `DayOfSim` and `DayOfSimChr` to `0` before the run period proceeds. The
official 1Zone diagnostic should keep this as a stage boundary: a fixed Rust
warmup-day count is insufficient unless the surface CTF histories, zone air
histories, and reporting filter match the post-report end-of-day handoff.
