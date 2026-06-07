# ep_runtime

## Responsibility

Runs compiled simulation plans and manages simulation state, result storage,
trace output, weather/schedule helpers, and diagnostic projections.

## Not responsible for

- raw epJSON parsing
- TypedModel construction
- conformance claim policy
- EnergyPlus algorithm changes

## Current claim level

Diagnostic/runtime shell except for specific declared case variables promoted
through conformance specs and reports.

## Main modules

- `runtime`
- planned: `mode`, `time_axis`, `execution_plan`, `simulation_state`
- planned domain modules: `heat_balance`, `hvac`, `plant`
