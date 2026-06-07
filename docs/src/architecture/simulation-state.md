---
status: active
claim_level: none
owner: runtime
last_reviewed: 2026-06-07
---

# Simulation State

Runtime state is being built in layers:

- `SimulationState` for current diagnostic execution
- `HeatBalanceState` shell for future EnergyPlus-aligned heat-balance work
- future zone/surface/HVAC/plant state structs as porting maps mature

The current heat-balance state shell initializes zone and surface state without
advancing a solver.

## State Ownership

`SimulationModel` is immutable during a run. `SimulationState` owns mutable
runtime values and must be resettable between runs.

Core runtime code must not depend on process-wide current model state, hidden
singletons, `static mut`, or ambient global simulation state. Shared data should
be passed through typed model, plan, cache, state, result, diagnostic, or trace
structures.
