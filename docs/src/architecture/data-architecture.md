# Data Architecture

The main pipeline is:

```text
epJSON
  -> RawModel
  -> TypedModel
  -> SimulationModel
  -> ModelGraph
  -> ExecutionPlan
  -> SimulationState
  -> ResultStore / DiagnosticStore / TraceStore
```

## Principles

- Parse and validation are separate stages.
- Names are resolved to typed IDs before runtime.
- Runtime state is explicit and resettable.
- Derived data belongs in explicit cache structures.
- Legacy files are export targets, not the native result model.

## Hot Path Rule

Runtime code should consume typed IDs, typed handles, and precomputed caches.
It should not repeatedly normalize names, inspect object type strings, or
resolve references after initialization. `RawModel` and `TypedModel` may retain
source-level shape for diagnostics, but `SimulationModel`, `ModelGraph`,
`ExecutionPlan`, and `SimulationState` are the runtime boundary.

When a future optimization changes layout or cache strategy, it must preserve
EnergyPlus-compatible algorithm order in `SimulationMode::Compatibility`.

## Stage Contracts

| Stage | Input | Output | First tests |
|---|---|---|---|
| Parse | epJSON | RawModel | object count, raw field preservation |
| Schema validation | RawModel | ValidatedRawModel | enum, field, required object |
| Normalize | ValidatedRawModel | NormalizedRawModel | defaults, canonical ordering |
| Typed conversion | NormalizedRawModel | TypedModel | units, enums, typed structs |
| Reference resolution | TypedModel | SimulationModel | name to ID mapping |
| Graph build | SimulationModel | ModelGraph | zone/surface/node graph |
| Execution plan | ModelGraph | ExecutionPlan | deterministic order |
| Runtime init | ExecutionPlan | SimulationState | initial states, output handles |
