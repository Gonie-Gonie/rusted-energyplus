# TypedModel Preview

v0.1.0 includes a TypedModel preview for the first RawModel object families and
resolves a small set of name references into typed IDs. v0.2.0 will harden this
preview into a supported coverage and diagnostics contract.

## Supported Inputs

The first typed subset is intentionally small:

- `Version`
- `Building`
- `Timestep`
- `Site:Location`
- `Material`
- `Material:NoMass`
- `Construction`
- `ScheduleTypeLimits`
- `Schedule:Constant`
- `Zone`
- `BuildingSurface:Detailed`

Objects outside this subset remain available through RawModel inspection and do
not block typed compilation unless a supported typed object references them.

## Reference Rules

Names are normalized with trim plus ASCII case folding while compiling. Runtime
structures use typed IDs rather than string lookup.

Current resolved references:

- `Construction.outside_layer` -> `MaterialId`
- `Schedule:Constant.schedule_type_limits_name` -> `ScheduleTypeLimitId`
- `BuildingSurface:Detailed.construction_name` -> `ConstructionId`
- `BuildingSurface:Detailed.zone_name` -> `ZoneId`

## Preview Diagnostics

The compiler reports structured diagnostics with severity, code, object type,
object name, field, and message. The v0.1.0 preview smoke verifies:

- missing references
- missing required fields in the typed subset
- invalid enum values
- invalid numeric field types

## Boundary

This is not full schema validation. Graph validation, topology validation,
execution plan generation, and simulation begin in later milestones.
