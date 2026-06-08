---
status: active
claim_level: none
owner: core
last_reviewed: 2026-06-08
---

# Current Status

The current public release line is v0.32.0. It adds a user coverage handbook on
top of the v0.31 release evidence asset manifest, v0.30 algorithm coverage
metadata, v0.29 output variable coverage metadata, v0.28 input object coverage
metadata, the v0.27 user-facing support coverage report, v0.26 internal convective gain
conformance, v0.25 opaque no-mass heat-balance boundary handling, v0.24
runtime registry infrastructure, v0.23 official ExampleFile static model
evidence, and v0.22 declared time, weather, and schedule numerical
conformance.

Current numerical conformance is limited to promoted cases and their declared
variables:

- `heat_balance_nomass_001`
- `surface_temperature_nomass_001`, including no-mass adiabatic surface
  conduction rate/per-area series
- `schedule_constant_001`
- `weather_fields_001` dry-bulb only
- `internal_gains_001` `Zone Total Internal Convective Heating Rate` only

## Current Evidence Boundary

| Area | Current conformance | Diagnostic or baseline evidence | Not claimed |
|---|---|---|---|
| Numerical time series | 5 promoted cases, 12 passed hourly series, all tolerance-gated with blocking gates | `official_1zone_uncontrolled_baseline_001` keeps oracle series and `official_1zone_uncontrolled_dynamic_diagnostic_001` reports run-period-filtered Rust deltas with Rust/oracle warmup day metadata | broad ExampleFiles dynamic conformance |
| Static model | official `1ZoneUncontrolled` EIO surface geometry, Construction CTF, Material CTF Summary, and OtherEquipment nominal fields | generated support/index/release evidence PDFs | dynamic behavior from the static EIO case |
| Heat balance | no-mass zone MAT, no-mass surface inside/outside temperature, and no-mass adiabatic conduction series | official `1ZoneUncontrolled` zone, roof/wall/floor surface decomposition, surface/zone conduction, and zone air heat-balance hourly oracle baselines plus failing warmup-aware diagnostic/probe deltas | CTF transient conduction, EnergyPlus warmup convergence parity, solar, radiation exchange, fenestration, infiltration, zone air predictor/corrector parity, or general heat-balance compatibility |
| Time, weather, schedule | `Schedule Value` and `Site Outdoor Air Drybulb Temperature` hourly series | dewpoint, relative humidity, pressure, wind speed, and wind direction diagnostics | broad weather processor compatibility |
| Internal gains | `Zone Total Internal Convective Heating Rate` for `internal_gains_001` | static OtherEquipment nominal fields | zone air temperature response to gains, radiant/latent coupling, or broad internal-gain compatibility |
| HVAC, node, plant | none | node, IdealLoads, and plant-loop baseline/diagnostic reports | HVAC, node, IdealLoads, meter, and plant numerical conformance |

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
- user coverage handbooks that reorganize supported inputs, outputs,
  algorithms, promoted cases, and known gaps around user decision rules
- release evidence asset manifests with package/report paths, SHA-256 hashes,
  content types, user-facing purposes, and JSON evidence summaries
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
- official dynamic heat-balance diagnostic reports that run a Rust
  first-run-period-day warmup loop, filter oracle ESO values to run-period
  samples, compare 30 roof/wall/floor face-temperature, conduction
  decomposition, and zone air heat-balance series, rank bottleneck series by
  RMSE, and record EnergyPlus EIO run-period warmup day counts, the current
  CTF seed policy, surface iteration count, and explicit all-CTF, warmup,
  surface-iteration, analytical zone-air, analytical surface-first, analytical
  coupled surface rebalance, and third-order probe metadata without claiming
  parity
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
- official `1ZoneUncontrolled` dynamic heat-balance parity from the current
  diagnostic or probe lanes
- new numerical conformance from the v0.27 support coverage report
- new numerical conformance from the v0.31 release evidence asset manifest
- new numerical conformance from the v0.32 user coverage handbook
- broad ExampleFiles compatibility
