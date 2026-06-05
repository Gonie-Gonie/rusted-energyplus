---
status: active
claim_level: none
owner: runtime
last_reviewed: 2026-06-05
---

# Simulation State

Runtime state is being built in layers:

- `SimulationState` for current diagnostic execution
- `HeatBalanceState` shell for future EnergyPlus-aligned heat-balance work
- future zone/surface/HVAC/plant state structs as porting maps mature

The current heat-balance state shell initializes zone and surface state without
advancing a solver.

