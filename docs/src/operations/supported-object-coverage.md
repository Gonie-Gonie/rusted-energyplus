# Supported Object Coverage

Each object should move through these stages:

```text
NotStarted
Parsed
Validated
Typed
ReferenceResolved
GraphResolved
Planned
Initialized
Simulated
OutputCompared
TraceCompared
Documented
```

Current table:

| Object | Parse | Validate | Typed | Ref | Graph | Plan | Simulate | Compare | Notes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| Version | yes | partial | yes | n/a | n/a | n/a | n/a | planned | v0.2 typed contract |
| Building | yes | partial | yes | n/a | partial | partial | partial | partial | v0.2 typed contract |
| Timestep | yes | partial | yes | n/a | n/a | planned | planned | planned | v0.2 typed contract |
| RunPeriod | yes | planned | planned | n/a | n/a | planned | planned | planned | generic raw parse only |
| Site:Location | yes | partial | yes | n/a | n/a | planned | planned | planned | v0.2 typed contract |
| Material | yes | partial | yes | n/a | planned | planned | planned | planned | v0.2 typed contract |
| Material:NoMass | yes | partial | yes | n/a | planned | planned | planned | planned | v0.2 typed contract |
| Construction | yes | partial | yes | yes | planned | planned | planned | planned | v0.2 typed contract |
| ScheduleTypeLimits | yes | partial | yes | n/a | n/a | planned | planned | planned | v0.2 typed contract |
| Zone | yes | partial | yes | n/a | planned | planned | planned | planned | v0.2 typed contract |
| BuildingSurface:Detailed | yes | partial | yes | yes | planned | planned | partial | partial | v0.2 typed contract |
| Schedule:Constant | yes | partial | yes | yes | n/a | planned | planned | planned | v0.2 typed contract |
| Schedule:Compact | yes | planned | planned | planned | n/a | partial | partial | partial | generic raw parse only |
| Output:Variable | yes | planned | no | planned | n/a | planned | planned | planned | raw-only in compile coverage |
| ZoneHVAC:IdealLoadsAirSystem | yes | planned | planned | planned | planned | planned | planned | planned | generic raw parse only |
| PlantLoop | yes | planned | planned | planned | planned | no | no | no | generic raw parse only |

v0.1.0 RawModel parse support is intentionally generic: unknown object types are
preserved in RawModel and reported as untracked by the CLI. Typed support is a
contract for the first seed object families. `model compile` reports every
object type it sees as either `typed` or `raw-only`.
