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
| Version | yes | planned | planned | n/a | n/a | n/a | n/a | planned | RawModel generic parse in v0.2 |
| Building | yes | planned | planned | n/a | partial | partial | partial | partial | RawModel generic parse in v0.2 |
| Timestep | yes | planned | planned | n/a | n/a | planned | planned | planned | RawModel generic parse in v0.2 |
| RunPeriod | yes | planned | planned | n/a | n/a | planned | planned | planned | RawModel generic parse in v0.2 |
| Site:Location | yes | planned | planned | n/a | n/a | planned | planned | planned | RawModel generic parse in v0.2 |
| Zone | yes | planned | planned | planned | planned | planned | planned | planned | RawModel generic parse in v0.2 |
| BuildingSurface:Detailed | yes | planned | planned | planned | planned | planned | partial | partial | RawModel generic parse in v0.2 |
| Schedule:Constant | yes | planned | planned | planned | n/a | planned | planned | planned | RawModel generic parse in v0.2 |
| Schedule:Compact | yes | planned | planned | planned | n/a | partial | partial | partial | RawModel generic parse in v0.2 |
| ZoneHVAC:IdealLoadsAirSystem | yes | planned | planned | planned | planned | planned | planned | planned | generic raw parse only |
| PlantLoop | yes | planned | planned | planned | planned | no | no | no | generic raw parse only |

v0.2.0 parse support is intentionally generic: unknown object types are
preserved in RawModel and reported as untracked by the CLI until an object
family receives typed/model support.
