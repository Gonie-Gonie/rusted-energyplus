# ep_model

## Responsibility

Owns typed IDs, normalized names, unit-bearing typed fields, EnergyPlus object
records for the supported subset, and aggregate model graph structures.

## Not responsible for

- raw epJSON parsing
- default application from RawModel fields
- runtime numerical simulation
- conformance claim policy

## Current claim level

Typed data model and graph structure only. Numerical compatibility is not
claimed by model structs alone.

## Main modules

- `ids`
- `names`
- `units`
- `objects`
- `model`
