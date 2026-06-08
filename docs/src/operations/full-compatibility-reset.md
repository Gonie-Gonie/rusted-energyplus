# Full Compatibility Reset

Status: active feedback incorporated.

This document records the June 2026 compatibility reset feedback and maps it
onto the current repository state. The canonical milestone boundaries live in
`specs/milestones.toml` and the generated milestone map. The goal is to keep
the project compatibility-first without accidentally presenting diagnostic
plumbing as EnergyPlus numerical conformance.

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
| v0.6 should be infrastructure/diagnostic, not a conformance simulation release | `specs/milestones.toml` labels v0.6 as diagnostic-only and records heat-balance conformance as not claimed |
| v0.7 should keep report/trace infrastructure while avoiding false claims | `specs/milestones.toml` labels v0.7 as a planning guard and records zone-temperature pass wording as not claimed |
| Release gates need a false-conformance guard | `strict-no-false-conformance` script tracks forbidden wording patterns |

## Test Taxonomy

Every comparison or report must belong to one of these classes:

| Class | Meaning | Exit-code semantics |
|---|---|---|
| smoke | Execution or extraction succeeded | `0` on successful execution |
| diagnostic-only | Values were extracted and deltas may be reported, but no tolerance is enforced | `0` on successful extraction |
| baseline-only | EnergyPlus oracle artifacts were generated from a manifest | `0` on successful generation |
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
| baseline generation | implemented for first fixture | `eplus-rs conformance baseline` writes oracle artifacts from a case manifest |
| report skeleton | implemented for baseline-only evidence | `eplus-rs conformance report-skeleton` writes output-request rows from ESO |
| `Output:Variable` registry | implemented foundation | `OutputRegistry` normalizes case output requests and rejects duplicates |
| release check hook | implemented | `scripts/dev.cmd conformance-schema-smoke` runs `cargo test -p ep_conformance` |

The first fixture remains `comparison_class = "smoke"` and
`conformance_claim = false`. It defines the variable surface for a future
multi-series report step without pretending that a tolerance-gated comparison
already exists. Its EnergyPlus baseline and report skeleton can be generated,
but that report is still baseline-only evidence.

## P2 Progress

| Area | Current status | Evidence |
|---|---|---|
| `RunPeriod` intake | implemented foundation | `ep_compiler` parses date ranges into `TypedModel.run_periods` |
| aligned hourly time axis | implemented foundation | `ep_runtime::build_hourly_time_axis` expands the first run period into EnergyPlus-style hour-ending samples |
| EPW weather fields | implemented foundation | `ep_runtime::parse_epw_records` reads dry-bulb, dew point, RH, pressure, radiation, and wind fields |
| EPW weather field comparison | implemented smoke gate | `eplus-rs compare weather-fields` compares dry-bulb, dew point, RH, pressure, wind speed, and wind direction with EnergyPlus ESO |
| `Schedule:Compact` subset | implemented foundation | typed all-days `Until` segments and runtime hourly evaluation |
| zone geometry summary | implemented foundation | `eplus-rs model geometry` prints zone surface count, floor area, volume, and exterior wall area |
| EIO geometry comparison | implemented smoke gate | `eplus-rs compare geometry` compares Rust geometry summary with EnergyPlus `Zone Information` |
| EIO construction/material comparison | implemented smoke gate | `eplus-rs compare construction-materials` compares construction layer-stack conductance and outside-layer material inputs with EnergyPlus `Construction CTF` and `Material CTF Summary` |
| EIO internal gains comparison | implemented smoke gate | `eplus-rs compare internal-gains` compares typed `OtherEquipment` nominal gains with EnergyPlus EIO |
| heat-balance state shell | implemented foundation | `ep_runtime::initialize_heat_balance_state` initializes zone and surface heat-balance state without advancing a solver |
| internal convective gain trace | implemented declared conformance gate | `eplus-rs conformance internal-gains-report` writes the v0.26 Rust hourly trace comparison against EnergyPlus ESO |

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

### v0.4 Time, Weather, Schedule Evidence

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

Current implemented subset:

- native `ResultStore` diagnostic path
- diagnostic-only zone temperature extraction
- manifest-driven diagnostic MAT reports
- compare regression `trace.json`, `compare-summary.json`,
  `compare-report.md`, and `profile-summary.json`
- schedule comparison
- weather field comparison
- first numeric divergence for series comparisons

### v0.7 Source Mapping Release

Goal:

- EnergyPlus source/function maps for heat-balance work
- output-variable to Rust state/source maps
- no algorithm port without a source-map entry
- blocking source-map gate before heat-balance conformance work

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

### v0.10 IdealLoads Typed Graph Foundation

Goal:

- thermostat schedules
- thermostat, equipment-list, equipment-connection, and IdealLoads typed intake
- graph edges from zone to thermostat, thermostat to setpoint, and zone to
  IdealLoads equipment
- baseline-only thermostat and IdealLoads output availability

Not yet claimed:

- IdealLoads load parity
- hourly/monthly/annual heating and cooling conformance
- HVAC node, sizing, availability, ventilation, humidity, economizer, heat
  recovery, or fuel-use parity

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
- [x] add baseline generation script
- [x] add multi-series compare report skeleton
- [x] add `Output:Variable` registry plan
- [x] add schedule/weather conformance suite shape

P2 compatibility work:

- [x] implement `RunPeriod` and aligned time axis foundation
- [x] expand EPW weather fields foundation
- [x] expand weather comparison smoke beyond dry-bulb
- [x] add `Schedule:Compact` subset foundation
- [x] add geometry summary foundation
- [x] add EnergyPlus EIO geometry comparison gate
- [x] add EnergyPlus EIO surface area/tilt/azimuth comparison gate
- [x] add EnergyPlus EIO construction/material thermal input gate
- [x] add EnergyPlus EIO `OtherEquipment` nominal internal-gains gate
- [x] write heat-balance porting map before heat-balance code
- [x] add heat-balance state shell without solver changes
- [x] port internal convective gains as a separate runtime trace
- [x] add opaque surface thermal inputs to heat-balance state
- [x] add first heat-balance timestep state advance without conformance claim
- [x] connect heat-balance zone-air trace to diagnostic-only MAT comparison
- [x] write diagnostic-only MAT compare summary/report artifacts
- [x] register diagnostic-only MAT case manifest without tolerance claim
- [x] generate manifest-driven MAT diagnostic report artifacts
- [x] embed MAT manifest metadata in diagnostic summary/report artifacts

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
