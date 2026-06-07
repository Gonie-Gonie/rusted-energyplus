---
status: active
claim_level: none
owner: core
last_reviewed: 2026-06-07
---

# Architecture Overview

The intended flow is:

```text
RawModel -> TypedModel -> SimulationModel -> ModelGraph -> RuntimeOutputRegistry
         -> ExecutionPlan -> SimulationState -> ResultStore -> Compare/Report
```

Source files should stay small enough for review and LLM-assisted development.
Recommended source files are 400 LOC or less, files over 800 LOC require
attention, and files over 1200 LOC require an explicit waiver until they are
split.

Module responsibilities:

- `ep_raw_model`: raw epJSON parsing and object storage
- `ep_model`: typed IDs, typed objects, normalized names, model graph data
- `ep_compiler`: input interpretation, defaults, references, graph assembly
- `ep_runtime`: simulation modes, state, output/meter registries, execution
  plans, traces, diagnostics
- `ep_compare`: series readers, tolerances, comparison summaries
- `ep_conformance`: case manifests, output requests, report/gate contracts
- `ep_cli`: command dispatch and user-facing text/json output

Algorithm ports require source maps before implementation. Diagnostic paths
must remain visibly separate from compatibility paths.
