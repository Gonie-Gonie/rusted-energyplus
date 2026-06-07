---
status: active
claim_level: none
owner: core
last_reviewed: 2026-06-07
---

# Current Status

The current public release line is v0.17.0. It gates Case Manifest and Output
Request Schema v2 for tracked conformance cases.

Current numerical conformance is limited to the promoted v0.8 and v0.9 no-mass
cases and their declared variables:

- `heat_balance_nomass_001`
- `surface_temperature_nomass_001`

The repository also contains smoke, baseline-only, and diagnostic evidence for
model intake, schedules, geometry, internal gains, node projection, IdealLoads
typed graph work, and plant-loop diagnostic plumbing. Those artifacts are
useful development evidence, but they are not general compatibility claims.

The current public scope includes:

- Rust workspace and pinned toolchain
- repo-local EnergyPlus 26.1.0 oracle and reference source setup
- repo-local portable Python for reporting
- RawModel and TypedModel intake for declared seed objects
- conformance manifests, output requests, tolerance rules, gates, and reports
- oodocs/matplotlib release evidence documents
- schema v2 validation for all tracked case manifests

Not claimed:

- general EnergyPlus heat-balance compatibility
- HVAC compatibility
- plant compatibility
- node, IdealLoads, meter, or full runtime conformance
- broad ExampleFiles compatibility
