---
status: active
claim_level: none
owner: core
last_reviewed: 2026-06-23
---

# Architecture Overview

The intended pipeline is:

```text
RawModel -> TypedModel -> SimulationModel -> ModelGraph -> SupportAssessment
         -> ExecutionPlan -> Runtime -> ResultStore -> OutputExport
         -> OracleCompare/Report
```

Model data should be immutable after compilation. Runtime mutation belongs in
`SimulationState`, node state, output registries, result stores, trace stores,
and subsystem-owned history structures.

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

Compatibility modules must map to EnergyPlus source routines. Diagnostic probe
modules may call compatibility functions and add instrumentation, but
compatibility modules must not call diagnostic probes.

`ExecutionPlan` is the source-order barrier between support assessment and
runtime execution. Arbitrary runs write expected and actual source-order stage
IDs to `model/execution-plan.json` and `run-summary.json`; a mismatch exits at
the plan stage before Rust runtime execution. IdealLoads plans include the
ZoneEquipmentManager dispatch and PurchasedAirManager `SimPurchasedAir`,
`GetPurchasedAir`, `InitPurchasedAir`, `CalcPurchAirLoads`,
`UpdatePurchasedAir`, and `ReportPurchasedAir` barriers. A conformance case
that selects a diagnostic algorithm must hard fail.

Source files should stay small enough for review and LLM-assisted development:
400 LOC is preferred, 800 LOC needs attention, and 1200 LOC needs an explicit
temporary waiver.
