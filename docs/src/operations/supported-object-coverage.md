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

Initial table:

| Object | Parse | Validate | Typed | Ref | Graph | Plan | Simulate | Compare | Notes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| Version | yes | partial | preview | n/a | n/a | n/a | n/a | planned | v0.1 typed preview |
| Building | yes | partial | preview | n/a | partial | partial | partial | partial | v0.1 typed preview |
| Timestep | yes | partial | preview | n/a | n/a | planned | planned | planned | v0.1 typed preview |
| RunPeriod | yes | planned | planned | n/a | n/a | planned | planned | planned | generic raw parse only |
| Site:Location | yes | partial | preview | n/a | n/a | planned | planned | planned | v0.1 typed preview |
| Material | yes | partial | preview | n/a | planned | planned | planned | planned | v0.1 typed preview |
| Material:NoMass | yes | partial | preview | n/a | planned | planned | planned | planned | v0.1 typed preview |
| Construction | yes | partial | preview | preview | planned | planned | planned | planned | v0.1 typed preview |
| ScheduleTypeLimits | yes | partial | preview | n/a | n/a | planned | planned | planned | v0.1 typed preview |
| Zone | yes | partial | preview | n/a | planned | planned | planned | planned | v0.1 typed preview |
| BuildingSurface:Detailed | yes | partial | preview | preview | planned | planned | partial | partial | v0.1 typed preview |
| Schedule:Constant | yes | partial | preview | preview | n/a | planned | planned | planned | v0.1 typed preview |
| Schedule:Compact | yes | planned | planned | planned | n/a | partial | partial | partial | generic raw parse only |
| ZoneHVAC:IdealLoadsAirSystem | yes | planned | planned | planned | planned | planned | planned | planned | generic raw parse only |
| PlantLoop | yes | planned | planned | planned | planned | no | no | no | generic raw parse only |

v0.1.0 RawModel parse support is intentionally generic: unknown object types are
preserved in RawModel and reported as untracked by the CLI. Typed support is a
preview for the first seed object families. v0.2.0 hardens that preview into an
explicit coverage and diagnostics contract.
