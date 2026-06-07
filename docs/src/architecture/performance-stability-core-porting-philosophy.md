---
status: active
claim_level: none
owner: core
last_reviewed: 2026-06-07
---

# Performance, Stability, and Core Porting Philosophy

rusted-energyplus is an EnergyPlus-compatible Rust porting project. The
reference oracle is EnergyPlus 26.1.0. Compatibility means preserving the
physical and engineering algorithms until a documented conformance case proves
otherwise.

The project may optimize Rust representation, execution planning, cache layout,
diagnostics, tracing, and numerical implementation details. It must not replace
EnergyPlus algorithms with simpler approximations and then describe the result
as compatibility.

## Non-Negotiable Premise

Compatibility work needs three independent tracks:

| Track | Required question |
|---|---|
| Evidence | Which case, variable, tolerance, report, artifact, and gate prove the claim? |
| Algorithm | Which EnergyPlus source routine and state mapping is being ported? |
| Operations | Which release and documentation workflow keeps the claim reproducible? |

All three tracks are required before a public compatibility claim. A case can
be useful development evidence without being conformance evidence.

## Optimization Boundary

Allowed optimization areas:

- typed IDs instead of repeated runtime string lookup
- columnar or handle-based result storage
- precomputed schedules, weather series, graph order, and output handles
- cache locality and deterministic execution planning
- explicit diagnostic and trace stores
- algorithm-preserving numerical implementation in Rust

Forbidden optimization areas:

- replacing EnergyPlus physical algorithms with toy formulas
- using first/last sample checks as numerical compatibility
- using performance results as conformance evidence
- promoting a diagnostic or smoke test without tolerance-gated artifacts
- hiding missing features behind default values that look like successful output

## Runtime Data Rules

The hot path should run on typed and precomputed structures:

```text
RawModel -> TypedModel -> SimulationModel -> ModelGraph -> ExecutionPlan
```

The runtime should not repeatedly inspect object type strings, normalize names,
or resolve references after initialization. Normalization happens once. Runtime
state uses typed IDs, typed handles, and explicit cache structures.

`SimulationModel` is immutable for a run. `SimulationState` is mutable and
resettable. No core runtime path should depend on process-wide current model
state, `static mut`, hidden singletons, or ambient global state.

## Simulation Modes

The default mode for compatibility work is:

```text
default = SimulationMode::Compatibility
```

Mode meanings:

| Mode | Meaning |
|---|---|
| Compatibility | Deterministic, EnergyPlus-aligned ordering and scalar behavior used for claims. |
| Diagnostic | Extra tracing and projection paths for development evidence. |
| Fast | Performance-oriented execution that must not support a compatibility claim by itself. |
| Experimental | Algorithm experiments that are explicitly outside the compatibility claim boundary. |

Fast and experimental results can guide engineering work, but they cannot enter
release conformance evidence unless they are re-run and proven under the
compatibility rules.

## Execution Plan Rules

`ExecutionPlan` is a compiled runtime artifact. It should encode environment,
warmup, zone, system, plant, reporting, callback, and output barriers once those
domains are implemented.

Work may be reordered only inside a barrier when EnergyPlus semantics allow it.
Callback barriers for EMS, PythonPlugin, API actuators, and future external
hooks must invalidate dependent caches explicitly.

## Diagnostics and Stability Rules

Unsupported or invalid input should become typed diagnostics, not panics or
silent fabricated output. Required diagnostic classes include:

| Class | Meaning |
|---|---|
| `UnsupportedObject` | Object family is outside the implemented subset. |
| `UnsupportedTopology` | Graph shape is not implemented. |
| `MissingReference` | A referenced object cannot be resolved. |
| `DuplicateNormalizedName` | Normalized names collide. |
| `InvalidNumericRange` | Parsed value is outside an allowed range. |
| `OutputVariableUnavailable` | Requested output is not produced by the current runtime. |
| `OracleArtifactMissing` | Required EnergyPlus artifact is absent. |
| `TimestampMismatch` | Rust and oracle samples do not align. |
| `SampleCountMismatch` | Rust and oracle series lengths differ. |
| `ToleranceFailure` | Declared tolerance is exceeded. |
| `NonFiniteValue` | NaN or infinity appears in evidence data. |
| `SolverDidNotConverge` | A numerical solver failed to converge. |

Graph validation should run before execution for zone/surface,
construction/material, node, air-loop, plant-loop, control, and output
dependency graphs as those domains are implemented.

## Algorithm Porting Rule

No source map, no algorithm port.

Every algorithm port must name:

- EnergyPlus source routine and file
- Rust target module or function
- state mapping
- output variable mapping
- baseline artifact
- Rust artifact
- tolerance and comparison report
- blocking gate

The algorithm ledger records this state across domains. A toy runtime path may
remain as development infrastructure, but it must stay diagnostic-only until it
is replaced by mapped EnergyPlus logic and tolerance-gated evidence.

## Performance Evidence

Performance is a project goal, but it is not conformance evidence. Performance
claims require fixed inputs, stage timing, memory notes where practical, and a
clear statement that the result is not an EnergyPlus compatibility claim.

Useful timings include parse, compile, graph build, execution, reporting,
oracle execution, and total gate wall-clock. Release numerical evidence may
include timing charts, but those charts support reproducibility and engineering
tracking rather than numerical compatibility.
