---
status: active
claim_level: baseline-only
owner: runtime
last_reviewed: 2026-06-07
---

# HVAC

HVAC numerical compatibility is not in the current public compatibility scope.
v0.10 adds the IdealLoads typed graph foundation so later HVAC work has named
model nodes, graph edges, and output requests to build on.

## v0.10 Foundation

`ideal_loads_thermostat_001` is the first HVAC-owned smoke case. It is a
blocking release gate, but it is not an IdealLoads load-conformance claim.

Typed objects:

- `ThermostatSetpoint:DualSetpoint`
- `ZoneControl:Thermostat`
- `ZoneHVAC:IdealLoadsAirSystem`
- `ZoneHVAC:EquipmentList`
- `ZoneHVAC:EquipmentConnections`
- `NodeList`

Graph edges:

- zone to thermostat
- thermostat to dual setpoint
- zone to IdealLoads equipment through equipment connections and equipment list
- NodeList to member node
- IdealLoads to resolved supply node
- zone to zone air node

Execution-plan placeholders:

- `EvaluateZoneThermostat`
- `EvaluateIdealLoadsAirSystem`

These placeholders make ordering visible. They do not mean that EnergyPlus HVAC
control, load, sizing, availability, humidity, ventilation, economizer, fuel,
or heat-recovery algorithms have been ported.

## Baseline Outputs

The v0.10 case requests these ESO variables:

| Variable | Class | Level |
|---|---|---|
| `Zone Thermostat Heating Setpoint Temperature` | `zone-state` | baseline-only |
| `Zone Thermostat Cooling Setpoint Temperature` | `zone-state` | baseline-only |
| `Zone Ideal Loads Zone Total Heating Rate` | `hvac-state` | baseline-only |
| `Zone Ideal Loads Zone Total Cooling Rate` | `hvac-state` | baseline-only |

The report must keep:

```text
comparison_class: smoke
conformance_claim: false
tolerance_policy: none
status: baseline-only
```

## v0.11 Air-Side Node Diagnostic

`air_side_node_diagnostic_001` is the first node-owned HVAC diagnostic case. It
keeps `comparison_class = "diagnostic-only"`, `conformance_claim = false`,
and `tolerance_policy: none`. The EnergyPlus report skeleton remains
`status: baseline-only`; the Rust node-state projection is backed by a
diagnostic `NodeStateStore` and remains `status: projected` with
`algorithm_parity: false`.

The case records EnergyPlus baseline-only ESO evidence for:

- `ZONE ONE INLET`
- `ZONE ONE AIR NODE`
- `ZONE ONE RETURN`

Requested node-state variables:

- `System Node Temperature`
- `System Node Humidity Ratio`
- `System Node Mass Flow Rate`

`System Node Setpoint Temperature` remains future-gated because the current
inlet and return nodes emit the EnergyPlus `-999` sentinel. The v0.11 gate does
not claim node, IdealLoads, fan, coil, air-loop, sizing, availability, or meter
numerical compatibility.

## v0.12 Node Source Map

`node-state-source-map.md` records the first EnergyPlus 26.1.0 source-function
map for system-node registration, storage, update, and output registration. It
maps:

- `NodeInputManager.cc` for `SetupOutputVariable`, `AssignNodeNumber`, and
  `CalcMoreNodeInfo`
- `DataLoopNode.hh` for `Node::NodeData::Temp`, `MassFlowRate`, `HumRat`, and
  `TempSetPoint`
- `PurchasedAirManager.cc` for IdealLoads supply and return node writes
- `DataZoneEquipment.cc` and `ZoneEquipmentManager.cc` for zone node flow and
  return node updates
- `ZoneTempPredictorCorrector.cc` for zone node temperature, humidity, and
  setpoint writes

This is a planning guard only. The Rust projection writes diagnostic samples
from `NodeStateStore`, but it does not port the node update algorithms and
does not promote `air_side_node_diagnostic_001` beyond diagnostic-only
evidence.

## EnergyPlus Source Areas To Map Next

Before a load-conformance claim, the remaining source maps must identify the
specific EnergyPlus 26.1.0 functions and state transitions for:

- zone thermostat control type and setpoint selection
- IdealLoads sensible and latent load calculation
- zone equipment availability and sequencing
- sizing interactions with autosized flow and capacity fields
- outdoor air, demand controlled ventilation, economizer, and heat recovery
- humidification and dehumidification control
- output variable registration and meter accounting

Likely source areas include `ZoneTempPredictorCorrector`,
`ZoneEquipmentManager`, `ZoneAirLoopEquipmentManager`, `HVACManager`, and the
IdealLoads component implementation in the EnergyPlus HVAC source tree. The
exact source-function map must be recorded before any numerical claim is
promoted.

## Promotion Requirements

An IdealLoads output can move from baseline-only to conformance only when all
of these exist:

- a declared case manifest with `comparison_class = "conformance"`
- `conformance_claim = true`
- requested thermostat, zone, IdealLoads, and node variables
- Rust result artifacts for the same keys, variables, and frequencies
- timestamp and warmup handling notes
- absolute and relative tolerances
- compare-summary rows with first divergence information
- markdown report artifact
- blocking release gate

Until then, IdealLoads rates and thermostat behavior remain baseline-only or
diagnostic-only evidence.
