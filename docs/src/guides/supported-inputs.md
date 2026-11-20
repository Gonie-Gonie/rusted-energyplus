---
status: active
claim_level: none
owner: docs
last_reviewed: 2026-11-20
---

# Supported inputs

Version 0.2.0 is a bounded-scope prerelease. A recognized object may have only
some fields and consumers implemented. A valid EnergyPlus model can therefore
still be rejected. Check the complete input through its selected route.

## Choose a route

| Route | Admitted model range |
| --- | --- |
| Source A: `--porting-scope A` | Opaque unconditioned heat balance. The fixed case is one Zone; reviewed extensions include selected closed opaque multizone, Space/list and gain combinations. |
| Source B: `--porting-scope B` | Direct-Zone IdealLoads. Selected sensible, hard-limit, humidity, outdoor-air, economizer and heat-recovery combinations have individual boundaries. |
| Legacy: no scope flag | Older generic/diagnostic execution, with separate capability checks and comparison options. |

The Windows launcher uses the legacy route. Select Source A/B through the CLI.

## Object and value ranges

The [complete example index](../generated/example-object-union.md) lists every
one of the **110 types in 188 input files**, plus 13 additional reviewed types.
The [147-entry registry](../generated/object-coverage.md) separates compiler
representation from ordinary Source admission.

| Family | Bounded release behavior |
| --- | --- |
| Project, time and weather | Selected Building/Zone, geometry rules, simulation/heat-balance/convection controls, Timestep, RunPeriod, DesignDay, site, calendar and ground-temperature fields. Sizing is excluded. |
| Opaque materials and geometry | Selected Wall/Floor/Roof/Ceiling, Construction, Material, NoMass, AirGap and InfraredTransparent combinations. Layers, topology, references, transforms and boundaries remain constrained. |
| Schedules and gains | Selected Constant/Compact/File/Day/Week/Year schedules, TypeLimits, People, OtherEquipment and Exterior:Lights. Source A has separately bounded ElectricEquipment and Space/list/group extensions. |
| Source B equipment | Selected IdealLoads, thermostat/humidistat, equipment-list/connections, outdoor-air specification/node and scheduled-setpoint combinations. General HVAC is excluded. |
| Source B material-only definitions | Registered unused WindowMaterial, spectral, helper and analytic Curve definitions reach their getters/reports. This does not admit active Windows or arbitrary window constructions. EcoRoof has a separate finite active boundary. |
| Outputs | Selected variable/meter, dictionary, schedule, construction/surface and table outputs. A request needs an implemented producer on its actual model. |

Within the 110-type corpus, **47** rows have bounded A/B admission, **27**
bounded B-only admission and **36** are blocked. No count means whole-object
or arbitrary-value support. Every applicable field, reference, topology,
algorithm and consumer condition in the detailed registry still applies.

Active `FenestrationSurface:Detailed` Windows, shading/CFS/EQL consumers,
general Plant/AirLoop systems, sizing, EMS, PythonPlugin and AirflowNetwork
remain outside ordinary Source release support. Typed graphs and unused
declarations do not override these exclusions.

## Check an input

From the extracted Windows package:

```powershell
.\bin\eplus-rs.exe run .\model.idf --porting-scope A --weather .\weather.epw --dry-run --output-dir .\out\check-model
```

Use B for bounded IdealLoads. Direct epJSON skips the locked EnergyPlus 26.1.0
converter, but does not broaden the simulation domain. IDF and epJSON retain
their format-specific rules.

Read `support-report.md`, `support-assessment.json`, `diagnostics.json` and
`run-summary.json`. A passing dry run is admission/preflight only. Remove
`--dry-run` and choose a fresh output directory to simulate. Ad-hoc runs keep
`conformance_claim=false`; unsupported inputs do not substitute oracle results.

## Evidence and limitations

The fixed CON-01 archive retains 15 cases and 16,427,880 reported values.
The separate published domain archive retains 284 paired cases and 13,276,216
values through Source212. Both use declared policies with zero tolerance/RMSE
failures. Their different totals are not added into a unique count.

Agreement for those inputs does not certify every new model, valid value
combination, private state or whole-file byte equality. Local `.runtime/`
evidence is absent from a fresh checkout unless separately supplied. See
[current status](../current/current-status.md) and [verification](verify-porting.md).
