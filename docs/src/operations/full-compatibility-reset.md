# Full Compatibility Reset

Status: active feedback incorporated.

This document records the June 2026 compatibility reset feedback and maps it
onto the current repository state. The goal is to keep the project
compatibility-first without accidentally presenting diagnostic plumbing as
EnergyPlus numerical conformance.

## Current State

Good foundation already exists:

- repo-local Rust toolchain and EnergyPlus 26.1.0 oracle
- reference EnergyPlus source checkout
- RawModel inspection and typed compile preview
- typed IDs, `NameMap`, `SimulationModel`, `ModelGraph`, and `ExecutionPlan`
- schedule and weather ESO comparisons
- `ResultStore` and first-zone runtime diagnostic
- compare regression driver with `trace.json`, `compare-summary.json`,
  `compare-report.md`, and `profile-summary.json`
- numeric first-divergence reporting in `ep_compare`
- typed conformance case/suite manifests in `ep_conformance`

Important boundary:

```text
No conformance claim without case + variable list + tolerance + report + gate.
```

The current `run first-zone` and `compare zone-temperature` commands are
diagnostic plumbing. They do not implement EnergyPlus heat balance and do not
claim numerical compatibility.

## Feedback Applied

| Feedback | Applied action |
|---|---|
| Zone-temperature smoke must not print a pass that looks like conformance | CLI now prints `comparison_class: diagnostic-only`, `conformance_claim: false`, `tolerance_policy: none`, and `status: extracted` |
| First-zone runtime is a toy diagnostic, not EnergyPlus heat balance | CLI now prints `runtime_class: diagnostic-toy`, `algorithm_parity: false`, and `status: extracted` |
| README should separate conformance scope and diagnostics | README now has a development-only diagnostics section |
| v0.6 should be infrastructure/diagnostic, not a conformance simulation release | v0.6 readiness now calls the path runtime plumbing diagnostics |
| v0.7 should keep report/trace infrastructure while avoiding false claims | v0.7 readiness clarifies extraction-only semantics for zone temperature |
| Release gates need a false-conformance guard | `strict-no-false-conformance` script tracks forbidden wording patterns |

## Test Taxonomy

Every comparison or report must belong to one of these classes:

| Class | Meaning | Exit-code semantics |
|---|---|---|
| smoke | Execution or extraction succeeded | `0` on successful execution |
| diagnostic | Values were extracted and deltas may be reported, but no tolerance is enforced | `0` on successful extraction |
| conformance | EnergyPlus oracle values are compared against declared tolerances | `1` when tolerance fails |
| regression | Current eplus-rs behavior is compared against an eplus-rs baseline | `1` when regression policy fails |
| performance | Runtime, memory, or profile counters are compared | `1` when performance gate fails |

Required user-facing fields for diagnostic or conformance comparisons:

```text
comparison_class: smoke | diagnostic-only | conformance | regression | performance
conformance_claim: true | false
oracle_version: 26.1.0
tolerance_policy: none | case.toml | default-<milestone>
status: extracted | pass | fail | skipped | unsupported
```

## P1 Progress

The reset plan is now being rebuilt from evidence contracts upward.

| Area | Current status | Evidence |
|---|---|---|
| case metadata schema | implemented | `crates/ep_conformance` parses and validates `ConformanceCase` |
| output request schema | implemented | `OutputRequest` requires key, variable, frequency, and variable class |
| false conformance guard | implemented in schema | `conformance_claim=true` is rejected unless class, outputs, tolerances, report, and blocking gate exist |
| suite shape | implemented | `ConformanceSuite` validates ordered case paths |
| first fixture | schema-only smoke | `data/conformance_cases/schedule_constant_001/case.toml` |
| release check hook | implemented | `scripts/conformance-schema-smoke.ps1` runs `cargo test -p ep_conformance` |

The first fixture remains `comparison_class = "smoke"` and
`conformance_claim = false`. It defines the variable surface for a future
baseline-generation step without pretending that a tolerance-gated comparison
already exists.

## Revised Milestone Intent

The historical milestone docs remain useful as implementation history. Going
forward, release language should follow this stricter interpretation.

### F0 Foundation

Repository setup only:

- pinned Rust toolchain
- Cargo workspace
- EnergyPlus oracle and reference source setup
- source/oracle smoke
- docs and package skeleton

No public conformance claim.

### v0.1 Model Intake

Supported:

- RawModel inspection
- typed compile preview for declared seed objects
- object coverage report
- missing-reference diagnostics

Not supported:

- runtime simulation conformance
- zone-temperature conformance
- HVAC or plant simulation

### v0.2 Baseline And Harness

Goal:

- stable EnergyPlus baseline generation
- case metadata
- output request specification
- ESO/SQL/RDD/MDD capture plan
- compare-summary/report schema

This is harness conformance, not simulation conformance.

### v0.3 Input Interpretation Parity

Goal:

- epJSON object/field handling
- defaults
- enum and string interpretation
- name normalization
- reference diagnostics
- unsupported object classification

### v0.4 Time, Weather, Schedule Parity

Goal:

- `RunPeriod`
- time axis
- EPW weather fields beyond dry-bulb
- `Schedule:Constant`
- `Schedule:Compact` subset
- schedule/weather conformance report

Required conformance style:

- exact sample count
- aligned timestamp/environment metadata
- strict weather/schedule tolerance

### v0.5 Geometry And Internal Variables

Goal:

- zone volume and floor area
- surface area/tilt/azimuth where available
- construction/material layer interpretation
- EnergyPlus internal variable comparison strategy

### v0.6 Output And Trace Infrastructure

Goal:

- `OutputRegistry`
- `ResultStore`
- `DiagnosticStore`
- `TraceStore`
- aligned time axis in native output
- per-variable tolerance policy skeleton

No heat-balance conformance claim.

### v0.7 Compare Release

Goal:

- `trace.json`
- `compare-summary.json`
- `compare-report.md`
- profile summary skeleton
- first divergence detection
- variable-level comparison schema

Current implemented subset:

- schedule comparison
- weather dry-bulb comparison
- diagnostic-only zone temperature extraction
- first numeric divergence for series comparisons

### v0.8 Heat Balance Mapping

Goal:

- EnergyPlus source/function mapping for zone and surface heat balance
- warmup and convergence semantics
- selected uncontrolled reference cases
- output variable list for heat-balance conformance

No implementation claim until mapping and oracle variables are fixed.

### v0.9 Uncontrolled Zone Heat Balance Parity

Goal:

- EnergyPlus-compatible zone air heat balance subset
- surface heat balance subset
- conduction algorithm subset
- internal gains coupling
- warmup handling

Required:

- tolerance-based `Zone Mean Air Temperature`
- surface temperature/rate comparisons
- first divergence report

### v0.10 IdealLoads Parity

Goal:

- thermostat schedules
- IdealLoads-like load calculation following EnergyPlus semantics
- hourly/monthly/annual heating and cooling comparison

### v1.0 Stable Compatibility Subset

Goal:

- locked supported object matrix
- locked tolerance policy
- public regression suite
- CI-enforced conformance reports
- clear unsupported diagnostics

v1.0 is still a declared subset, not a full EnergyPlus replacement.

## Immediate Action Backlog

P0 false-conformance cleanup:

- keep README wording diagnostic-only for toy runtime paths
- keep `compare zone-temperature` extraction-only until heat balance parity
- keep `compare-zone-smoke` assertions on diagnostic fields, not pass fields
- run `strict-no-false-conformance` in local release checks

P1 conformance infrastructure:

- [x] add case metadata schema
- [x] add output request schema
- [ ] add baseline generation script
- [ ] add multi-series compare report skeleton
- [ ] add `Output:Variable` registry plan
- [x] add schedule/weather conformance suite shape

P2 compatibility work:

- implement `RunPeriod` and aligned time axis
- expand EPW weather fields
- add `Schedule:Compact` subset
- add geometry/internal-variable comparison
- write heat-balance porting map before heat-balance code

## Forbidden Wording

Avoid these phrases until the required conformance evidence exists:

- EnergyPlus simulation works
- zone temperature comparison passes
- first EnergyPlus-compatible runtime
- fully compatible subset
- heat-balance compatible

Acceptable replacements:

- runtime plumbing diagnostic
- diagnostic-only zone-temperature extraction
- schedule/weather conformance smoke
- typed model preview
- comparison report infrastructure

## PR Rule

Any PR that changes compatibility behavior must answer:

```text
Does this claim EnergyPlus numerical compatibility?
If yes, which case, variables, tolerances, and report prove it?
Does this change diagnostic-only tooling?
Does this change tolerance policy?
Does this update the relevant porting map?
```
