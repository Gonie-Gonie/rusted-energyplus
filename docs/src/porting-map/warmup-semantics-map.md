---
status: active
claim_level: planning-guard
owner: runtime
last_reviewed: 2026-06-08
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
keeps the case `conformance_claim=false` while reporting first-hour and
run-period-filtered deltas. Large first-hour or warmup-sensitive deltas must
keep the case diagnostic until this map is implemented.
