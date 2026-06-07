---
status: active
claim_level: none
owner: core
last_reviewed: 2026-06-07
---

# Current Status

The current public release line is v0.22.0. It gates declared time, weather,
and schedule numerical conformance on top of the source-map and algorithm
ledger validation.

Current numerical conformance is limited to promoted cases and their declared
variables:

- `heat_balance_nomass_001`
- `surface_temperature_nomass_001`
- `schedule_constant_001`
- `weather_fields_001` dry-bulb only

The repository also contains smoke, baseline-only, and diagnostic evidence for
model intake, additional weather variables, geometry, internal gains, node projection, IdealLoads
typed graph work, and plant-loop diagnostic plumbing. Those artifacts are
useful development evidence, but they are not general compatibility claims.
The historical v0.1 through v0.15 boundaries are summarized in
`specs/milestones.toml`; their old planning pages are intentionally not
retained in the docs tree.

The current public scope includes:

- Rust workspace and pinned toolchain
- repo-local EnergyPlus 26.1.0 oracle and reference source setup
- repo-local portable Python for reporting
- RawModel and TypedModel intake for declared seed objects
- conformance manifests, output requests, tolerance rules, gates, and reports
- output request injection for staged oracle baselines
- selected-series timestamp alignment, RMSE, relative-delta, and first
  divergence reporting
- release conformance index reports with case, output, meter, domain, report,
  and gate coverage matrices
- source-map and algorithm ledger checks that validate EnergyPlus source
  anchors, Rust target anchors, first cases, proof variables, and blocking
  gates
- timestamp-aligned conformance reports for declared schedule and dry-bulb
  hourly series
- oodocs/matplotlib release evidence documents
- schema v2 validation for all tracked case manifests

Not claimed:

- general EnergyPlus heat-balance compatibility
- general runtime compatibility
- HVAC compatibility
- plant compatibility
- node, IdealLoads, meter, or broad weather conformance
- broad ExampleFiles compatibility
