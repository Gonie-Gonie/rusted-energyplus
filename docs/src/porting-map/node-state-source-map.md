---
status: active
claim_level: planning-guard
owner: runtime
last_reviewed: 2026-06-07
---

# Node State Source Map

Reference version: EnergyPlus 26.1.0

Reference source root:

```text
.reference/energyplus-src/26.1.0/
```

Purpose: lock the EnergyPlus source files, routines, data fields, and first
fixture mapping for system-node outputs before any node, IdealLoads, or HVAC
numerical conformance claim is promoted. This map is a planning guard.

## Primary Source Files

| Area | EnergyPlus source | Rust target |
|---|---|---|
| node registration and output registration | `src/EnergyPlus/NodeInputManager.cc` | `ep_compiler` node registry; future `ep_runtime::node_state` output writer |
| node registration declarations | `src/EnergyPlus/NodeInputManager.hh` | `ep_model::Node`, `ep_model::NodeList` |
| node storage | `src/EnergyPlus/DataLoopNode.hh` | future `ep_runtime::NodeState` |
| zone equipment node links | `src/EnergyPlus/DataZoneEquipment.hh`; `src/EnergyPlus/DataZoneEquipment.cc` | `ep_model::ModelGraph`; future zone node flow aggregation |
| zone air node temperature and humidity | `src/EnergyPlus/ZoneTempPredictorCorrector.cc` | future node-state projection from zone air balance |
| zone equipment orchestration and return node update | `src/EnergyPlus/ZoneEquipmentManager.cc` | future zone equipment runtime |
| IdealLoads supply and return node update | `src/EnergyPlus/PurchasedAirManager.cc` | future IdealLoads runtime |
| output processor lookup and reporting | `src/EnergyPlus/OutputProcessor.cc` | `ep_conformance`; `ep_runtime::ResultStore` |

## Required Routine And Field Map

| Behavior | EnergyPlus routine or field | Current Rust status |
|---|---|---|
| node list/name lookup | `GetNodeNums`; `GetOnlySingleNode` | typed NodeList and node registry exist |
| node allocation | `AssignNodeNumber` | compiler-side node names exist; runtime state not ported |
| report variable registration | `SetupOutputVariable` in `NodeInputManager.cc` | manifest output class exists; Rust node samples not produced |
| node scalar storage | `Node::NodeData::Temp`, `MassFlowRate`, `HumRat`, `TempSetPoint` | no node-state result projection yet |
| derived node reporting | `CalcMoreNodeInfo` | not ported |
| zone equipment connections | `EquipConfiguration::ZoneNode`, `InletNode`, `ReturnNode` | graph edges exist for v0.10/v0.11 fixture |
| zone node number | `ZoneData::SystemZoneNodeNumber` | represented through zone air node graph edge only |
| zone node flow aggregation | `EquipConfiguration::setTotalInletFlows` | not ported |
| zone node temperature | `ZoneHeatBalanceData::correctZoneAirTemp` updates `thisSystemNode.Temp` | not ported for HVAC node output |
| zone node humidity | `ZoneHeatBalanceData::correctHumRat` updates `Node(ZoneNodeNum).HumRat` | not ported |
| zone node setpoint | thermostat load path writes `Node(zoneNodeNum).TempSetPoint` | not ported |
| IdealLoads supply node state | `CalcPurchAirLoads` writes `Node(InNodeNum).Temp`, `HumRat`, and `MassFlowRate` | not ported |
| IdealLoads return node flow | `CalcPurchAirLoads` writes `Node(RecircNodeNum).MassFlowRate` | not ported |
| return node temperature and humidity | `CalcZoneLeavingConditions` writes return node `Temp`, `HumRat`, and `Enthalpy` | not ported |

## Output Registration Boundary

`NodeInputManager.cc` registers `System Node Temperature`, `System Node Mass
Flow Rate`, `System Node Humidity Ratio`, and `System Node Setpoint
Temperature` against each discovered `NodeID` through `SetupOutputVariable`.
The registered report fields bind directly to `Node::NodeData` members.

The v0.11 fixture requests only:

- `System Node Temperature`
- `System Node Humidity Ratio`
- `System Node Mass Flow Rate`

`System Node Setpoint Temperature` remains future-gated because not every node
in `air_side_node_diagnostic_001` has a meaningful setpoint value. A future
setpoint claim must identify which node owns the setpoint, which thermostat or
manager writes it, and how `SensedNodeFlagValue` sentinel values are filtered.

## v0.11 Fixture Node Map

| Fixture key | EnergyPlus source path | Current Rust evidence |
|---|---|---|
| `ZONE ONE INLET` | `ZoneHVAC:IdealLoadsAirSystem` input through `GetOnlySingleNode`; `CalcPurchAirLoads` writes supply node temperature, humidity ratio, and mass flow | typed node and baseline-only ESO rows |
| `ZONE ONE AIR NODE` | `ZoneEquipConfig.ZoneNode` and `ZoneData::SystemZoneNodeNumber`; zone predictor/corrector writes temperature and humidity; `setTotalInletFlows` aggregates inlet flow to the zone node | typed zone air node graph edge and baseline-only ESO rows |
| `ZONE ONE RETURN` | `ZoneEquipConfig.ReturnNode`; `CalcPurchAirLoads` writes recirculation flow; `CalcZoneLeavingConditions` writes return temperature and humidity | typed return node plus baseline-only ESO rows |

## Porting Order

Node numerical work must preserve this source-derived order unless a
case-specific waiver documents why it differs:

1. register node names from equipment and NodeList input
2. attach nodes to zone equipment connections and zone heat-balance data
3. initialize node state and per-timestep demand fields
4. calculate zone demand and IdealLoads supply state
5. aggregate inlet mass flow to the zone node
6. correct zone node temperature and humidity
7. calculate return node flow, temperature, humidity, and enthalpy
8. sample requested node output variables into the Rust result store
9. compare only variables that have declared timestamp rules and tolerances

## Stop Rule

No node output may move beyond diagnostic-only or baseline-only evidence until
the implementation names:

- the EnergyPlus source file and routine being ported
- the Rust state field being written
- the node key and variable being sampled
- the case manifest and report artifact
- the timestamp, warmup, and sentinel handling rules
- the tolerance policy and blocking gate

Until then, node temperature, humidity ratio, mass flow, and setpoint outputs
remain baseline-only or diagnostic-only evidence.
