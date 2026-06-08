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
| Material | yes | partial | yes | n/a | partial | partial | partial | partial | thermal properties used for first-zone UA and compared against EIO `Material CTF Summary`; official `1ZoneUncontrolled` static EIO fields are v0.23 conformance |
| Material:NoMass | yes | partial | yes | n/a | partial | partial | partial | partial | thermal resistance used for first-zone UA and compared against EIO `Material CTF Summary`; official `1ZoneUncontrolled` static EIO fields are v0.23 conformance |
| Construction | yes | partial | yes | yes | partial | partial | partial | partial | ordered opaque layer stack used for first-zone UA and compared against EIO `Construction CTF`; official `1ZoneUncontrolled` static EIO fields are v0.23 conformance |
| ScheduleTypeLimits | yes | partial | yes | n/a | n/a | planned | planned | planned | v0.2 typed contract |
| Zone | yes | partial | yes | n/a | partial | partial | partial | partial | first zone simulated, heat-balance MAT trace compared diagnostically, `heat_balance_nomass_001` MAT compared as v0.8 conformance, `surface_temperature_nomass_001` MAT included in v0.9 conformance, regression-traced, geometry-summarized, and EIO-compared |
| BuildingSurface:Detailed | yes | partial | yes | yes | partial | partial | partial | partial | exterior area, area/tilt/azimuth, thermal inputs, adiabatic v0.8 equilibrium surfaces, v0.9 surface inside/outside face temperature conformance for `surface_temperature_nomass_001`, v0.23 official static EIO surface evidence, and heat gains used for first-zone UA, heat-balance MAT trace, geometry summary, and EIO comparison |
| Schedule:Constant | yes | partial | yes | yes | n/a | partial | partial | partial | exact comparison in regression trace suite |
| OtherEquipment | yes | partial | yes | yes | partial | partial | partial | partial | internal gains used for first-zone subset, EIO nominal-gains comparison, v0.23 official static EIO nominal-gains evidence, and v0.26 ESO convective-gain conformance trace |
| Schedule:Compact | yes | partial | yes | yes | n/a | partial | partial | partial | all-days Until segment subset |
| Output:Variable | yes | planned | no | planned | n/a | planned | planned | planned | raw-only in compile coverage |
| ThermostatSetpoint:DualSetpoint | yes | partial | yes | yes | partial | partial | planned | baseline-only | v0.10 typed graph coverage in `ideal_loads_thermostat_001`; no thermostat numerical parity claim |
| ZoneControl:Thermostat | yes | partial | yes | yes | yes | partial | planned | baseline-only | v0.10 zone thermostat graph edge to dual setpoint and zone |
| NodeList | yes | partial | yes | yes | yes | partial | planned | baseline-only | v0.10 NodeList members resolve to typed nodes and IdealLoads supply-node graph edges; v0.11 records baseline-only node-state output evidence plus diagnostic NodeStateStore projection, with no node output parity claim |
| ZoneHVAC:EquipmentList | yes | partial | yes | yes | yes | partial | planned | baseline-only | v0.10 equipment list resolves IdealLoads equipment entries and validates sequence integrity before v0.11 |
| ZoneHVAC:EquipmentConnections | yes | partial | yes | yes | yes | partial | planned | baseline-only | v0.10 zone equipment connection resolves zone and equipment list; duplicate zone connections are rejected; v0.11 records zone air-node diagnostic outputs and NodeStateStore projection |
| ZoneHVAC:IdealLoadsAirSystem | yes | partial | yes | yes | yes | partial | planned | baseline-only | v0.10 typed graph coverage in `ideal_loads_thermostat_001`, with nonzero baseline signal and range diagnostics; not an IdealLoads load-conformance claim |
| PlantLoop | yes | partial | yes | yes | yes | partial | projection-only | diagnostic-only | v0.13 typed graph smoke, v0.15 baseline-only plant output rows, and post-v0.15 Rust projection addendum artifacts; no plant loop algorithm parity |
| Branch | yes | partial | yes | yes | yes | partial | projection-only | diagnostic-only | v0.13 typed graph smoke; component nodes register, and the projection addendum uses branch component order for artifact shape only |
| BranchList | yes | partial | yes | yes | yes | partial | projection-only | diagnostic-only | v0.13 typed graph smoke; branch members resolve and feed the projection addendum order |
| Connector:Splitter | yes | partial | yes | yes | yes | partial | no | no | v0.13 typed graph smoke only; inlet/outlet branch references resolve |
| Connector:Mixer | yes | partial | yes | yes | yes | partial | no | no | v0.13 typed graph smoke only; inlet/outlet branch references resolve |
| ConnectorList | yes | partial | yes | yes | yes | partial | no | no | v0.13 typed graph smoke only; connector entries resolve |
| Pump:ConstantSpeed | yes | partial | yes | yes | partial | partial | no | no | v0.13 typed identity only; no pump head, power, or flow parity |
| Boiler:HotWater | yes | partial | yes | yes | partial | partial | no | no | v0.13 typed identity only; no boiler load or fuel parity |
| Chiller:Electric:EIR | yes | partial | yes | yes | partial | partial | no | no | v0.13 typed identity only; no chiller load, COP, or condenser-loop parity |

v0.1.0 RawModel parse support is intentionally generic: unknown object types are
preserved in RawModel and reported as untracked by the CLI. Typed support is a
contract for the first seed object families. `model compile` reports every
object type it sees as either `typed` or `raw-only`.
