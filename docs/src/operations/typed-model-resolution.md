# TypedModel Contract

v0.1.0 introduced a TypedModel preview for the first RawModel object families.
v0.2.0 hardens that path into a supported coverage and diagnostics contract for
the first typed subset.

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

## Compile Coverage

`eplus-rs model compile <input.epJSON>` reports a deterministic coverage table
for every object type seen in RawModel:

```text
coverage:
  Building: 1 [typed]
  Output:Variable: 48 [raw-only]
```

`typed` means the object type is part of the current TypedModel contract.
`raw-only` means the object type is preserved by RawModel but not converted by
the typed compiler stage.

## Diagnostics

The compiler reports structured diagnostics with severity, code, object type,
object name, field, and message. The typed smoke verifies:

- missing references
- missing required fields in the typed subset
- invalid enum values
- invalid numeric field types
- typed/raw-only coverage output

## Boundary

This is not full schema validation. Graph validation, topology validation,
execution plan generation, and simulation begin in later milestones.
