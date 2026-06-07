---
status: active
claim_level: none
owner: core
last_reviewed: 2026-06-08
---

# Current Status

The current public release line is v0.29.0. It enriches output variable
coverage with strongest-evidence first references and support-boundary
metadata on top of v0.28 input object coverage metadata, the v0.27
user-facing support coverage report, v0.26 internal convective gain
conformance, v0.25 opaque no-mass heat-balance boundary handling, v0.24
runtime registry infrastructure, v0.23 official ExampleFile static model
evidence, and v0.22 declared time, weather, and schedule numerical
conformance.

Current numerical conformance is limited to promoted cases and their declared
variables:

- `heat_balance_nomass_001`
- `surface_temperature_nomass_001`
- `schedule_constant_001`
- `weather_fields_001` dry-bulb only
- `internal_gains_001` `Zone Total Internal Convective Heating Rate` only

The repository also contains smoke, baseline-only, and diagnostic evidence for
model intake, additional weather variables, local fixture geometry/internal
gain checks, node projection, IdealLoads typed graph work, and plant-loop
diagnostic plumbing. Those artifacts are useful development evidence, but
they are not general compatibility claims.

Current static model conformance is limited to:

- `official_1zone_static_model_001`
- declared static EIO surface geometry fields
- declared Construction CTF and Material CTF Summary fields
- declared OtherEquipment Internal Gains Nominal fields
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
- user-facing support coverage reports with input object, output variable, and
  algorithm support matrices
- source-map and algorithm ledger checks that validate EnergyPlus source
  anchors, Rust target anchors, first cases, proof variables, and blocking
  gates
- timestamp-aligned conformance reports for declared schedule and dry-bulb
  hourly series
- static EIO model conformance reports for the official `1ZoneUncontrolled`
  ExampleFile
- runtime output registry handles for currently implemented output variables
- explicit unavailable-output and unavailable-meter runtime diagnostics
- ResultStore duplicate-handle/duplicate-series diagnostics and profile
  scaffolding
- opaque no-mass adiabatic and interzone surface boundary target handling in
  heat-balance state
- timestamp-aligned internal convective gain conformance for the declared
  `internal_gains_001` hourly ESO series
- oodocs/matplotlib release evidence documents
- schema v2 validation for all tracked case manifests

Not claimed:

- general EnergyPlus heat-balance compatibility
- general runtime compatibility
- HVAC compatibility
- plant compatibility
- node, IdealLoads, meter, or broad weather conformance
- dynamic compatibility for the v0.23 static model case
- new numerical conformance from the v0.24 runtime-infrastructure milestone
- zone air temperature response to internal gains, radiant/latent internal
  gain coupling, or broader heat-balance compatibility from the v0.26
  internal-gain milestone
- new numerical conformance from the v0.27 support coverage report
- broad ExampleFiles compatibility
