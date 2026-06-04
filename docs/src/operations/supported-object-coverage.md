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
| Version | planned | planned | planned | n/a | n/a | n/a | n/a | planned | v0.2 |
| Building | planned | planned | planned | n/a | partial | partial | partial | partial | v0.3+ |
| Timestep | planned | planned | planned | n/a | n/a | planned | planned | planned | v0.3+ |
| RunPeriod | planned | planned | planned | n/a | n/a | planned | planned | planned | v0.5 |
| Site:Location | planned | planned | planned | n/a | n/a | planned | planned | planned | v0.5 |
| Zone | planned | planned | planned | planned | planned | planned | planned | planned | v0.6+ |
| BuildingSurface:Detailed | planned | planned | planned | planned | planned | planned | partial | partial | v0.7+ |
| Schedule:Constant | planned | planned | planned | planned | n/a | planned | planned | planned | v0.5 |
| Schedule:Compact | planned | planned | planned | planned | n/a | partial | partial | partial | v0.5 |
| ZoneHVAC:IdealLoadsAirSystem | planned | planned | planned | planned | planned | planned | planned | planned | v0.9 |
| PlantLoop | planned | planned | planned | planned | planned | no | no | no | v0.11 preview |

