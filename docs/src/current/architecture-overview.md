---
status: active
claim_level: none
owner: core
last_reviewed: 2026-10-20
---

# Architecture Overview

The compiler and runtime pipeline is:

```text
RawModel -> TypedModel -> SimulationModel -> ModelGraph -> SupportAssessment
         -> ExecutionPlan -> Runtime -> Engine-owned results -> OutputExport
         -> OracleCompare/Report
```

Oracle comparison is an optional validation step. The Source port's normal
runtime does not currently support `--compare-oracle`; see the
[run guide](../guides/run-arbitrary-idf.md) for the supported command path.

## Data and State Ownership

Model data should be immutable after compilation. Runtime mutation belongs in
`SimulationState`, node state, output registries, result stores, trace stores,
and subsystem-owned history structures.

Parsing, validation, normalization, typed conversion, reference resolution,
graph assembly, and execution planning are separate stages. Normalize names and
resolve references to typed IDs before runtime. Timestep code consumes typed
IDs, handles, and explicit caches rather than repeated object-name or type-string
lookup. Keep source-level input shape available for diagnostics.

Mutable state must reset between runs. Pass model, plan, cache, state, result,
diagnostic, and trace owners explicitly; core runtime must not depend on hidden
singletons, `static mut`, or ambient process-wide simulation state. Initialization,
warmup, history updates, and producer/consumer timing must follow the applicable
[state contracts](../../../energyplus_porting_plan/STATE_CONTRACTS.md).

## Crates and Runtime Boundaries

Core crates:

- `ep_raw_model`: raw epJSON parsing and object storage
- `ep_model`: typed IDs, typed objects, normalized names, model graph data
- `ep_compiler`: input interpretation, defaults, references, graph assembly
- `ep_runtime`: compatibility algorithms, execution plans, simulation state,
  output/meter registries, traces, diagnostics, and result storage
- `ep_run`: CLI-independent arbitrary run pipeline and artifact layout
- `ep_compare`: ESO/MTR readers, tolerances, comparison summaries
- `ep_conformance`: case manifests, output requests, report/gate contracts
- `ep_cli`: command dispatch and user-facing text/json output

Compiler output distinguishes typed objects, raw-only objects, missing
references, unsupported objects, and coverage status. Typed input support is
an interpretation claim; runtime and numerical evidence must be established
separately.

Compatibility modules must map to EnergyPlus source routines. Diagnostic probe
modules may call compatibility functions and add instrumentation, but
compatibility modules must not call diagnostic probes. Heat-balance diagnostic
selection is converted once, inside `ep_runtime::diagnostic_probes`, into the
probe-agnostic `HeatBalanceRuntimeConfig`; production runtime consumers accept
that config and must not import or match `DiagnosticHeatBalanceProbe` or its
legacy long-form variants. The CLI delegates diagnostic selector parsing and
display names to the same diagnostic module instead of duplicating those
variant matches.

Production simulation is implemented in Rust. EnergyPlus C++ is a reference and
test oracle, not a linked simulation kernel or fallback for Rust results. Safe
Rust is the default; any future unsafe fast path needs a documented boundary,
benchmark evidence, and scalar compatibility tests.

## Execution Order

`ExecutionPlan` is the source-order barrier between support assessment and
runtime execution. Arbitrary runs write expected and actual source-order stage
IDs to `model/execution-plan.json` and `run-summary.json`; a mismatch exits at
the plan stage before Rust runtime execution. IdealLoads plans include the
ZoneEquipmentManager dispatch and PurchasedAirManager `SimPurchasedAir`,
`GetPurchasedAir`, `InitPurchasedAir`, `CalcPurchAirLoads`,
`UpdatePurchasedAir`, and `ReportPurchasedAir` barriers. A conformance case
that selects a diagnostic algorithm must hard fail.

For the completed Source route, this gate is execution-plan preflight metadata,
not an actual Source caller trace. Runtime call-order evidence is recorded and
reviewed separately.

Heat-balance source-order stage definitions live under `ep_runtime::heat_balance`
instead of in the legacy runtime root. The current plan includes
`ManageAirHeatBalance` and the separate
`ZoneTempPredictorCorrector::ManageZoneAirUpdates` barrier before
`UpdateFinalSurfaceHeatBalance`, so zone-air solve steps are attached to the
zone predictor/corrector stage rather than a broad air-manager bucket.

Order can change within a barrier only when EnergyPlus semantics allow it.
Future EMS, PythonPlugin, and API actuator callbacks need explicit barriers and
cache invalidation before dependent values are reused. Recording a stage does
not prove that every EnergyPlus branch behind it is implemented.

## Outputs, Diagnostics, and Performance

`ep_conformance::OutputRegistry` validates declared comparison requests.
`RuntimeOutputRegistry` and `RuntimeMeterRegistry` resolve model-owned output
handles before timestep execution. Missing outputs must produce explicit
unavailable diagnostics rather than empty successful-looking series.

Runtime-native result stores own the computed series; legacy formats are export
targets. Exports must preserve timestamp, key, variable, frequency, and the
request/tolerance mapping used by comparison reports. Handle-based or columnar
storage may improve layout without changing numerical behavior.

Diagnostics must distinguish unsupported input, missing references, invalid
ranges, unavailable outputs, timestamp/sample-count mismatches, non-finite
values, tolerance failures, and nonconvergence. A trace or structural check
does not establish numerical compatibility; a claim needs the declared case,
oracle, Rust artifacts, tolerances, report, and blocking gate.

Optimization may change representation, cache locality, compilation, and
algorithm-preserving execution details. It must preserve equations, branch and
state-update order, and the compatibility contract. Fixed-input timings support
performance claims, not numerical compatibility. The durable mode and review
rules are in
[Performance, Stability, and Core Porting Philosophy](../architecture/performance-stability-core-porting-philosophy.md).

Source files should stay small enough for review and LLM-assisted development:
400 LOC is preferred, 800 LOC needs attention, and 1200 LOC needs an explicit
temporary waiver.

## Source And State References

Use these maps for durable input, state-owner and branch contracts:

- Input: [geometry](../porting-map/geometry-source-map.md),
  [materials](../porting-map/material-source-map.md),
  [time/weather/schedules](../porting-map/time-weather-schedule.md)
- Heat balance: [manager and input](../porting-map/heat-balance-source-map.md),
  [conduction](../porting-map/conduction-map.md),
  [surface](../porting-map/surface-balance-map.md),
  [zone air](../porting-map/zone-air-update-map.md),
  [warmup](../porting-map/warmup-semantics-map.md)
- Equipment: [IdealLoads](../porting-map/ideal-loads-source-map.md),
  [nodes](../porting-map/node-state-source-map.md),
  [Plant](../porting-map/plant-source-map.md)
- Shared calculations and reporting:
  [psychrometrics](../porting-map/psychrometrics-source-map.md),
  [output variables](../porting-map/output-variable-source-map.md)

Current completion and scope decisions remain in the
[algorithm ledger](../generated/algorithm-ledger.md) and
[conformance contract](../conformance/overview.md), rather than progress logs.
