# ep_compiler

## Responsibility

Interprets RawModel input into typed model records, applies EnergyPlus defaults
for the supported subset, resolves references, and builds model graph data.

## Not responsible for

- raw epJSON parsing
- runtime numerical simulation
- conformance claim policy
- replacing EnergyPlus engineering algorithms

## Current claim level

Input interpretation and typed-graph evidence only unless a case, variable,
tolerance, report, and blocking gate explicitly promote a result.

## Main modules

- `compiler`
- planned: `context`, `diagnostics`, `report`, `passes`, `objects`
