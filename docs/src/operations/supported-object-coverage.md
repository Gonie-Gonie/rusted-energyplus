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
| Building | yes | partial | yes | n/a | partial | partial | partial | partial | first-zone run uses building-level typed context |
| Timestep | yes | partial | yes | n/a | n/a | partial | partial | planned | first-zone run uses zone timesteps per hour |
| RunPeriod | yes | partial | yes | n/a | n/a | partial | partial | planned | typed date range and hourly time-axis foundation |
| Site:Location | yes | partial | yes | n/a | n/a | planned | planned | planned | v0.2 typed contract |
| Material | yes | partial | yes | n/a | partial | partial | partial | partial | thermal properties used for first-zone UA and compared against EIO `Material CTF Summary` |
| Material:NoMass | yes | partial | yes | n/a | partial | partial | partial | partial | thermal resistance used for first-zone UA and compared against EIO `Material CTF Summary` |
| Construction | yes | partial | yes | yes | partial | partial | partial | partial | outside layer used for first-zone UA and compared against EIO `Construction CTF` |
| ScheduleTypeLimits | yes | partial | yes | n/a | n/a | planned | planned | planned | v0.2 typed contract |
| Zone | yes | partial | yes | n/a | partial | partial | partial | partial | first zone simulated, heat-balance MAT trace compared diagnostically, `heat_balance_nomass_001` MAT compared as v0.8 conformance, `surface_temperature_nomass_001` MAT included in v0.9 conformance, regression-traced, geometry-summarized, and EIO-compared |
| BuildingSurface:Detailed | yes | partial | yes | yes | partial | partial | partial | partial | exterior area, area/tilt/azimuth, thermal inputs, adiabatic v0.8 equilibrium surfaces, v0.9 surface inside/outside face temperature conformance for `surface_temperature_nomass_001`, and heat gains used for first-zone UA, heat-balance MAT trace, geometry summary, and EIO comparison |
| Schedule:Constant | yes | partial | yes | yes | n/a | partial | partial | partial | exact comparison in regression trace suite |
| OtherEquipment | yes | partial | yes | yes | partial | partial | partial | partial | internal gains used for first-zone subset, EIO nominal-gains comparison, and ESO convective-gain trace |
| Schedule:Compact | yes | partial | yes | yes | n/a | partial | partial | partial | all-days Until segment subset |
| Output:Variable | yes | planned | no | planned | n/a | planned | planned | planned | raw-only in compile coverage |
| ZoneHVAC:IdealLoadsAirSystem | yes | planned | planned | planned | planned | planned | planned | planned | generic raw parse only |
| PlantLoop | yes | planned | planned | planned | planned | no | no | no | generic raw parse only |

v0.1.0 RawModel parse support is intentionally generic: unknown object types are
preserved in RawModel and reported as untracked by the CLI. Typed support is a
contract for the first seed object families. `model compile` reports every
object type it sees as either `typed` or `raw-only`.
