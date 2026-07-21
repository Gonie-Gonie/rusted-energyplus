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
| Space | yes | partial | yes | yes | no | no | no | no | lexical authored declarations, type/tag/default selectors, Zone links, one whole-zone default per otherwise empty Zone, and mixed-surface remainder Spaces are typed; authored definitions and generated remainders run-block while sole whole-zone defaults do not |
| SpaceList | yes | partial | yes | yes | no | no | no | no | lexical lists retain ordered authored Space references and allow source-valid empty lists; all definitions run-block until Space-or-SpaceList target expansion is implemented |
| BuildingSurface:Detailed | yes | partial | yes | yes | partial | partial | partial | partial | exterior area, area/tilt/azimuth, thermal inputs, adiabatic v0.8 equilibrium surfaces, v0.9 surface inside/outside face temperature conformance for `surface_temperature_nomass_001`, v0.23 official static EIO surface evidence, and heat gains used for first-zone UA, heat-balance MAT trace, geometry summary, and EIO comparison; optional Space references resolve through the full typed arena with same-Zone validation, and mixed explicit/implicit assignments create a typed remainder. A bounded post-surface pass retains immutable variable-absorptance bindings for Outdoors surfaces whose construction outside layer owns a typed overlay and warns on non-Outdoors outside-layer or inside-layer occurrences; every overlay still run-blocks, while full source surface-list/reorder parity, other surface families, runtime absorptance updates, Space surface lists, and space-level geometry/runtime remain deferred |
| SurfaceProperty:IncidentSolarMultiplier | yes | partial | yes | partial | no | no | no | no | request-only CP99 state retains a dense typed ID, nonsemantic normalized declaration key, unresolved normalized window target, defaulted/bounded multiplier, and optional resolved ScheduleId. Duplicate targets and missing schedules fail closed; no source order, FenestrationSurface binding, Surface mutation, schedule evaluation, window behavior, or runtime claim is made, and every definition run-blocks |
| SurfaceProperty:SolarIncidentInside | yes | partial | yes | yes | no | no | no | no | CP100 first-phase state retains a dense typed ID, semantic normalized name without a name map, typed BuildingSurface:Detailed SurfaceId, any typed ConstructionId, and required ScheduleId. Duplicate names and a construction different from the surface's own construction are valid, repeated surface/construction pairs fail closed, no source order is claimed, and every definition run-blocks. Representative-surface mutation, full Zone/Space heat-transfer-surface completeness, pair lookup, schedule sampling, runtime solar replacement, and conformance remain deferred; CP101 separately types the following complex-fenestration request family, while CP102 emits only a nonblocking monotonic warning when retained typed opaque surfaces already contain both an exact current-construction pair match and a miss |
| ComplexFenestrationProperty:SolarAbsorbedLayers | yes | partial | yes | partial | no | no | no | no | CP101 second-phase request state retains a dense typed ID, semantic normalized name without a name map, unresolved normalized fenestration target, complex-fenestration ConstructionId, and outside-to-inside ScheduleIds matching its solid optical layer count. Duplicate target/construction pairs, missing required schedules, any layer field present beyond the count, and non-complex constructions fail closed; schedule values and type limits are not inspected, no source order or IDF trailing-blank positional parity is claimed, and every definition run-blocks before fenestration binding, full completeness checks, pair lookup, BSDF absorption, or runtime. CP102's bounded warning observes only current-construction pairs on retained typed opaque surfaces and cannot consume these unresolved requests |
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
