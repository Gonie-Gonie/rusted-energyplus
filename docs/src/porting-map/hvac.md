---
status: active
claim_level: limited-ideal-loads-no-oa-sensible-conformance
owner: runtime
last_reviewed: 2026-06-15
---

# HVAC

Broad HVAC numerical compatibility is not in the current public compatibility
scope. The current exceptions are the narrow no-OA/no-limit, numeric
finite-limit, and ConstantSensibleHeatRatio cooling IdealLoads claims for
declared thermostat, IdealLoads rate, and supply-node outputs.

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
and mass flow, plus hourly `DistrictHeatingWater:Facility` and
`DistrictCooling:Facility` oracle-MTR requests. The conformance compare
command writes matching Rust `ResultStore` series for 28 Detailed output rows
over 110 samples. The 10 declared conformance rows are tolerance-gated; the
remaining rows are diagnostic proof only. Energy rows use the EnergyPlus
`ReportPurchasedAir` raw `rate * TimeStepSysSec` branch with a fixed
8-substep, 112.5 s system timestep in this fixture, then the `OutputProcessor`
`Sum` report interval emits the 900 s zone-timestep total. Fuel energy rows in
this conformance fixture use the blank fuel-efficiency schedule branch. The
facility meters are hourly oracle-MTR vs Rust aggregated fuel-energy
diagnostics only; adaptive system timestep, fuel-efficiency conformance, and
broad meter conformance remain outside the claim.

## IdealLoads Fuel-Efficiency Diagnostic

`ideal_loads_fuel_efficiency_diagnostic_001` is a diagnostic-only proof lane
for the same no-OA/no-limit sensible branch with non-unity constant fuel
efficiency schedules:

```text
comparison_class: diagnostic-only
conformance_claim: false
tolerance_policy: diagnostic-draft
status: diagnostic
```

The fixture sets the heating fuel-efficiency `Schedule:Constant` to 0.8 and
the cooling fuel-efficiency `Schedule:Constant` to 0.75. The compare lane
divides the no-OA zone and supply-air heating/cooling rates by those schedule
values, then uses the same detailed `TimeStepSysSec` energy accumulation for
fuel energy rows. It compares 12 Detailed series over 110 samples with zero
tolerance failures. This proves the blank and constant `Schedule:Constant`
ReportPurchasedAir fuel-efficiency branches diagnostically; non-constant
efficiency schedules and broad fuel or meter conformance remain outside the
claim. The same lane compares hourly DistrictHeatingWater/DistrictCooling
oracle-MTR values against Rust fuel-energy aggregates diagnostically.

## IdealLoads Finite Flow/Capacity Evidence

Three no-OA finite-limit lanes are now promoted to conformance for the declared
thermostat, IdealLoads rate, and supply-node temperature/mass-flow outputs:

- `ideal_loads_capacity_limit_conformance_001`
- `ideal_loads_flow_limit_conformance_001`
- `ideal_loads_flow_capacity_limit_conformance_001`

The original no-OA diagnostic-only lanes remain as regression/proof evidence
for finite heating/cooling flow and capacity limits:

- `ideal_loads_capacity_limit_diagnostic_001`
- `ideal_loads_flow_limit_diagnostic_001`
- `ideal_loads_flow_capacity_limit_diagnostic_001`

Each fixture now requests 18 Detailed series, including `ZONE ONE RETURN`
`System Node Temperature` and `System Node Humidity Ratio` proof rows. The
Rust compare lane records the resolved return node as the no-OA recirculation
node and uses that same-call recirculation state for the finite-limit no-OA
mixed-air and report calculations.

The promoted capacity-limit, flow-limit, and flow-and-capacity-limit lanes
have `comparison_class = "conformance"`, `conformance_claim = true`,
`tolerance_policy: conformance-gate`, and `status: pass`, with 10 conformance
rows and 8 diagnostic proof rows. Capacity-limit covers 188 Detailed samples,
flow-limit covers 128 Detailed samples, and flow-and-capacity-limit covers 189
Detailed samples. The original diagnostic finite-limit lanes remain available
as non-claim regression evidence with zero tolerance failures.

## IdealLoads Constant SHR Conformance

`ideal_loads_constant_shr_conformance_001` promotes the no-OA
`ConstantSensibleHeatRatio` cooling lane for declared thermostat setpoints,
cooling total/sensible/latent rate rows, and supply-node
temperature/mass-flow/humidity rows. It reuses the diagnostic fixture IDF and
keeps return-node and zone-air-node humidity rows as proof rows only.

The compare run has `comparison_class = "conformance"`, `conformance_claim =
true`, `tolerance_policy: conformance-gate`, and `status: pass`. It compares
18 Detailed series over 96 samples, with 11 conformance rows and 7 diagnostic
proof rows. `ConstantSupplyHumidityRatio`, Humidistat, outdoor-air humidity,
finite-limit humidity-control behavior, and broad humidity-control conformance
remain outside the claim.

## IdealLoads Constant Supply Humidity Cooling Conformance

`ideal_loads_constant_supply_humidity_cooling_conformance_candidate_001`
promotes the no-OA `ConstantSupplyHumidityRatio` cooling lane for declared
thermostat setpoints, cooling total/sensible/latent rate rows, supply-air
cooling report rows, and supply-node temperature/mass-flow/humidity rows. It
reuses the diagnostic fixture IDF and keeps heating rows, return-node and
zone-air-node humidity rows, ReportPurchasedAir energy/fuel rows, and facility
meters as proof rows only.

The compare run has `comparison_class = "conformance"`, `conformance_claim =
true`, `tolerance_policy: conformance-gate`, and `status: pass`. It compares
36 Detailed/hourly series with 11 conformance rows and 25 diagnostic proof rows.
Heating-side `ConstantSupplyHumidityRatio`, Humidistat, outdoor-air humidity,
finite-limit humidity-control behavior, and broad humidity-control conformance
remain outside the claim.

## IdealLoads Outdoor-Air Design-Flow Diagnostic

`ideal_loads_outdoor_air_flow_person_diagnostic_001`,
`ideal_loads_outdoor_air_design_flow_diagnostic_001`,
`ideal_loads_outdoor_air_flow_area_diagnostic_001`,
`ideal_loads_outdoor_air_air_changes_diagnostic_001`,
`ideal_loads_outdoor_air_sum_diagnostic_001`,
`ideal_loads_outdoor_air_maximum_diagnostic_001`,
`ideal_loads_outdoor_air_differential_dry_bulb_economizer_diagnostic_001`,
`ideal_loads_outdoor_air_differential_enthalpy_economizer_diagnostic_001`, and
`ideal_loads_outdoor_air_sensible_heat_recovery_diagnostic_001`, and
`ideal_loads_outdoor_air_enthalpy_heat_recovery_diagnostic_001` are
diagnostic-only Flow/Person, Flow/Zone, Flow/Area, AirChanges/Hour, Sum,
Maximum, DifferentialDryBulb, DifferentialEnthalpy, Sensible heat-recovery,
and Enthalpy heat-recovery outdoor-air proof lanes:

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
`StdRhoAir` from `Site:Location`, derives Flow/Person occupant count from typed
`People` objects, derives Flow/Area floor area from typed floor surfaces,
derives AirChanges/Hour volume from the typed zone volume, evaluates
Sum/Maximum aggregate methods over those supported terms, and writes matching
Detailed Rust `ResultStore` series for the 96 no-economizer oracle samples. The
DifferentialDryBulb and DifferentialEnthalpy fixtures use 110 source-order
Detailed samples because EnergyPlus reports system substeps while the
economizer is active. The Sensible heat-recovery fixture keeps 96 samples with
`NoEconomizer` and validates active heat-recovery time plus sensible/total
rate rows. The Enthalpy heat-recovery fixture also keeps 96 samples with
`NoEconomizer` and validates active heat-recovery time plus sensible, latent,
and total rate rows. The no-economizer flow rows are exact; the active
economizer flow rows use narrow diagnostic tolerances. The sensible and total
report rows are diagnostic with a 1 W source-order tolerance, except the
Enthalpy cooling heat-recovery total/latent rows allow a single 6 W
saturation-limit diagnostic timestep. The no-humidity latent report,
supply-air state, and mixed-air state rows are diagnostic; supply-air
temperature stays within 0.02 C, and the Enthalpy humidity-ratio rows allow
5e-5 kg/kg at that saturation-limit timestep. The inactive heat-recovery rows
are exact zeros for the non-heat-recovery fixtures, and economizer active time
is exact for the inactive, DifferentialDryBulb, and DifferentialEnthalpy
branches.

This evidence does not promote active DCV, active humidity controls,
saturation-limit heat-recovery branches, or broad IdealLoads outdoor-air
conformance.

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

Variables outside the declared no-OA/no-limit, numeric finite-limit, and
ConstantSensibleHeatRatio cooling boundaries remain baseline-only or
diagnostic-only evidence until they receive their own source map, Rust state,
oracle evidence, and blocking gate.
