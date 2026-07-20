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
| Material:AirGap | yes | partial | yes | yes | partial | partial | no | diagnostic-only | positive R-only opaque variant and middle-layer validation; `material_opaque_variants_001` compares exact grouped-EIO layer name, order, and resistance without a dynamic or conformance claim |
| Material:InfraredTransparent | yes | partial | yes | yes | partial | partial | no | diagnostic-only | fixed-value name-only opaque variant and intended sole-layer validation; `material_opaque_variants_001` compares the exact sole-layer grouped-EIO row without treating its generic row shape as an independent type discriminator |
| Construction | yes | partial | yes | yes | partial | partial | partial | partial | ordered opaque layer stack used for first-zone UA and compared against EIO `Construction CTF`; official `1ZoneUncontrolled` static EIO fields are v0.23 conformance, while `material_opaque_variants_001` adds nonblocking exact ordered-layer diagnostic evidence only |
| ScheduleTypeLimits | yes | partial | yes | n/a | n/a | planned | planned | planned | v0.2 typed contract |
| Zone | yes | partial | yes | yes | partial | partial | partial | partial | complete public declaration fields plus bounded nominal-control, ZoneGroup, last-nonblank local-environment node-link, and ordered authored/default Space side effects; local convection overrides, zone grouping, local environments, and authored space partitioning fail closed before runtime; existing first-zone numerical boundaries are unchanged |
| ZoneList | yes | partial | yes | yes | no | no | no | no | ordered Zone references and validation are typed; all definitions run-block until Zone-or-ZoneList target expansion is implemented |
| ZoneGroup | yes | partial | yes | yes | no | no | no | no | ZoneList reference, multiplier, overlap checks, and Zone side effects are typed; all definitions run-block until list multipliers are comprehensively consumed |
| ZoneProperty:LocalEnvironment | yes | partial | yes | yes | no | no | no | no | ordered Zone and optional generic Node references plus the Zone link are typed; all definitions run-block until local outdoor-air conditions and downstream weather consumers are implemented |
| Space | yes | partial | yes | yes | no | no | no | no | lexical authored declarations, type/tag/default selectors, Zone links, and one generated default per otherwise empty Zone are typed; authored definitions and active raw surface space assignments run-block while generated defaults alone do not |
| SpaceList | yes | partial | yes | yes | no | no | no | no | lexical lists retain ordered authored Space references and allow source-valid empty lists; all definitions run-block until Space-or-SpaceList target expansion is implemented |
| BuildingSurface:Detailed | yes | partial | yes | yes | partial | partial | partial | partial | exterior area, area/tilt/azimuth, thermal inputs, adiabatic v0.8 equilibrium surfaces, v0.9 surface inside/outside face temperature conformance for `surface_temperature_nomass_001`, v0.23 official static EIO surface evidence, and heat gains used for first-zone UA, heat-balance MAT trace, geometry summary, and EIO comparison; nonempty or non-string `space_name` fails closed until surface-to-space resolution is ported |
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
