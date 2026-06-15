---
status: active
claim_level: limited-ideal-loads-no-oa-sensible-conformance
owner: runtime
last_reviewed: 2026-06-15
---

# HVAC

Broad HVAC numerical compatibility is not in the current public compatibility
scope. The current exception is the narrow
`ideal_loads_no_oa_sensible_conformance_001` claim for declared no-OA/no-limit
IdealLoads sensible outputs.

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

`ideal-loads-source-map.md` now records the first `PurchasedAirManager` and
zone-equipment function map for the no-OA/no-limit sensible diagnostic
candidate. Before a load-conformance claim, the remaining source maps and Rust
state must still identify or implement the specific EnergyPlus 26.1.0
functions and state transitions for:

- zone thermostat control type and setpoint selection
- IdealLoads sensible and latent load calculation
- zone equipment availability and sequencing
- sizing interactions with autosized flow and capacity fields
- outdoor air, demand controlled ventilation, economizer, and heat recovery
- humidification and dehumidification control
- output variable registration and meter accounting

The exact source-function map, Rust result state, and tolerance gate must be
recorded before any numerical claim is promoted.

## IdealLoads No-OA Sensible Conformance

`ideal_loads_no_oa_sensible_conformance_001` is the first narrow IdealLoads
conformance gate:

```text
comparison_class: conformance
conformance_claim: true
tolerance_policy: conformance-gate
status: pass
```

The case requests thermostat setpoints, IdealLoads total and sensible rates,
supply-air total rates, ReportPurchasedAir total energy rows, blank-efficiency
fuel energy/rate proof rows, signed zone predicted load, setpoint-distance
proof loads, zone-air-node proof rows, and supply-node temperature, humidity,
and mass flow. The conformance compare command writes matching Rust
`ResultStore` series for 28 Detailed output rows over 110 samples. The 10
declared conformance rows are tolerance-gated; the remaining rows are
diagnostic proof only. Energy rows use the EnergyPlus `ReportPurchasedAir`
raw `rate * TimeStepSysSec` branch with a fixed 8-substep, 112.5 s system
timestep in this fixture, then the `OutputProcessor` `Sum` report interval
emits the 900 s zone-timestep total. Fuel energy rows also use the blank
fuel-efficiency schedule branch and do not claim adaptive system timestep,
fuel meter, or non-unity efficiency schedule conformance.

## IdealLoads Outdoor-Air Design-Flow Diagnostic

`ideal_loads_outdoor_air_design_flow_diagnostic_001` is a diagnostic-only
Flow/Zone outdoor-air proof lane:

```text
comparison_class: diagnostic-only
conformance_claim: false
tolerance_policy: diagnostic-draft
status: diagnostic
```

The case requests `Zone Ideal Loads Outdoor Air Mass Flow Rate`,
`Zone Ideal Loads Outdoor Air Standard Density Volume Flow Rate`,
`Zone Ideal Loads Outdoor Air Sensible Heating Rate`,
`Zone Ideal Loads Outdoor Air Sensible Cooling Rate`,
`Zone Ideal Loads Outdoor Air Latent Heating Rate`,
`Zone Ideal Loads Outdoor Air Latent Cooling Rate`,
`Zone Ideal Loads Outdoor Air Total Heating Rate`,
`Zone Ideal Loads Outdoor Air Total Cooling Rate`,
`Zone Ideal Loads Supply Air Mass Flow Rate`,
`Zone Ideal Loads Supply Air Standard Density Volume Flow Rate`,
`Zone Ideal Loads Supply Air Temperature`,
`Zone Ideal Loads Supply Air Humidity Ratio`,
`Zone Ideal Loads Mixed Air Temperature`, and
`Zone Ideal Loads Mixed Air Humidity Ratio`,
`Zone Ideal Loads Heat Recovery Sensible Heating Rate`,
`Zone Ideal Loads Heat Recovery Latent Heating Rate`,
`Zone Ideal Loads Heat Recovery Total Heating Rate`,
`Zone Ideal Loads Heat Recovery Sensible Cooling Rate`,
`Zone Ideal Loads Heat Recovery Latent Cooling Rate`,
`Zone Ideal Loads Heat Recovery Total Cooling Rate`,
`Zone Ideal Loads Economizer Active Time`, and
`Zone Ideal Loads Heat Recovery Active Time` for the IdealLoads object.
The Rust compare lane resolves the referenced `DesignSpecification:OutdoorAir`,
applies the blank outdoor-air schedule as always 1.0, derives EnergyPlus
`StdRhoAir` from `Site:Location`, and writes matching Detailed Rust
`ResultStore` series for the 96 oracle samples. The flow rows are exact; the
sensible and total report rows are diagnostic with a 1 W source-order
tolerance. The no-humidity latent report, supply-air mass/volume/humidity, and
mixed-air state rows are exact; supply-air temperature stays within 0.02 C in
this no-economizer/no-heat-recovery fixture. The inactive economizer and
heat-recovery rows are exact zeros.

This evidence does not promote active DCV, economizer, heat recovery, active
humidity controls, saturation-limit branches, finite limits, or broad
IdealLoads outdoor-air conformance.

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

Variables outside the declared no-OA/no-limit sensible boundary remain
baseline-only or diagnostic-only evidence until they receive their own source
map, Rust state, oracle evidence, and blocking gate.
