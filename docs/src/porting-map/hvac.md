---
status: active
claim_level: limited-ideal-loads-no-oa-sensible-conformance
owner: runtime
last_reviewed: 2026-06-15
---

# HVAC

Broad HVAC numerical compatibility is not in the current public compatibility
scope. The current exceptions are the narrow no-OA/no-limit, numeric
finite-limit, ConstantSensibleHeatRatio cooling, ConstantSupplyHumidityRatio,
Humidistat, outdoor-air, Sensible heat-recovery, blank/constant/all-days
Schedule:Compact fuel-efficiency, and no-OA hourly plus monthly/annual/run-period
facility meter IdealLoads claims for declared outputs and meters only.

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
`ReportPurchasedAir` raw `rate * TimeStepSysSec` branch and the
`OutputProcessor` `Sum` report interval. Rust obtains the 900 s nominal
zone/system timestep and nominal count of one from `ep_runtime::TimeAxis`, and
uses each ESO sample's start/end duration. Because ESO minutes are printed to
two decimals, a duration within that display precision of an integer TimeAxis
subdivision is restored to the exact subdivision; other valid durations remain
unchanged, with the nominal zone timestep as the missing/invalid fallback. Fuel
energy rows in this conformance fixture use the blank fuel-efficiency schedule
branch. The facility meters are
hourly oracle-MTR vs Rust aggregated fuel-energy diagnostics only in this
sensible case; adaptive system timestep,
monthly/annual/run-period facility meter aggregation, and broad meter
conformance remain outside the claim.
Fuel-efficiency conformance is claimed only by the separate
blank, constant Schedule:Constant, and all-days Schedule:Compact
fuel-efficiency candidates.

## IdealLoads No-OA Facility Meter Conformance

`ideal_loads_no_oa_facility_meter_conformance_candidate_001` is a meter-only
conformance gate for the same no-OA/no-limit sensible fixture:

```text
comparison_class: conformance
conformance_claim: true
tolerance_policy: conformance-gate
status: pass
```

Only the hourly `DistrictHeatingWater:Facility` and
`DistrictCooling:Facility` MTR rows are conformance-level. The 28 Detailed ESO
rows in the case remain diagnostic proof evidence for ReportPurchasedAir rate,
energy, fuel-energy, thermostat, demand, humidity, and node behavior. The Rust
side aggregates the detailed fuel-energy series through
`ep_runtime::RuntimeMeterRegistry` and
`ep_runtime::ideal_loads_facility_meter_binding`, compares them against
EnergyPlus hourly MTR values, and keeps broad meter conformance,
fuel-efficiency schedules beyond the declared blank/constant/all-days
Schedule:Compact candidates, daily meter aggregation and multi-year annual grouping, outdoor air,
humidity controls, finite limits, air loops, plant loops, EMS, and
PythonPlugin behavior outside the claim.

`ideal_loads_no_oa_facility_meter_monthly_run_period_conformance_candidate_001`
extends that meter-only branch to monthly, annual, and run-period MTR rows for
the same two facility meters. Its 28 Detailed ESO rows remain diagnostic proof
evidence, the Rust side groups detailed fuel-energy by month, sums it over the
annual report period, or sums it over the run period, and multi-year annual
grouping remains outside the claim.

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
tolerance failures.

`ideal_loads_blank_fuel_efficiency_conformance_candidate_001` promotes only
the declared no-OA blank fuel energy-rate and fuel energy rows. The matching
`ideal_loads_constant_fuel_efficiency_conformance_candidate_001` promotes only
the declared no-OA constant `Schedule:Constant` fuel energy-rate and fuel
energy rows. `ideal_loads_non_constant_fuel_efficiency_conformance_candidate_001`
promotes only the declared no-OA all-days `Schedule:Compact` fuel energy-rate
and fuel energy rows. Raw IdealLoads rates and facility meters remain proof
evidence in those cases; fuel-efficiency schedules beyond the declared
blank/constant/all-days Schedule:Compact candidates and broad fuel or meter
conformance remain outside the claim. The diagnostic lane still compares
hourly DistrictHeatingWater/DistrictCooling oracle-MTR values against Rust
fuel-energy aggregates diagnostically.

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
proof rows. `ConstantSupplyHumidityRatio` is covered by separate cooling/heating
candidates; Humidistat, outdoor-air humidity, finite-limit humidity-control
behavior, and broad humidity-control conformance remain outside this claim.

## IdealLoads Constant Supply Humidity Cooling Conformance

`ideal_loads_constant_supply_humidity_cooling_conformance_candidate_001`
promotes the no-OA `ConstantSupplyHumidityRatio` cooling lane for declared
thermostat setpoints, heating/cooling total/sensible/latent rate rows,
supply-air heating/cooling report rows, ReportPurchasedAir energy/fuel rows,
and supply-node temperature/mass-flow/humidity rows. It reuses the diagnostic
fixture IDF and promotes the hourly, monthly, and run-period
`DistrictHeatingWater:Facility` and `DistrictCooling:Facility` meters from the
meter side. It keeps return-node and zone-air-node humidity rows, annual meter
rows, and broader meter behavior as diagnostic proof only.

The compare run has `comparison_class = "conformance"`, `conformance_claim =
true`, `tolerance_policy: conformance-gate`, and `status: pass`. It compares
36 Detailed ESO series with 29 conformance rows and 7 diagnostic proof rows,
plus six conformance meter rows: hourly, monthly, and run-period for both facility meters.
Humidistat, outdoor-air humidity, finite-limit humidity-control behavior, and
broad humidity-control conformance remain outside the claim.

## IdealLoads Constant Supply Humidity Heating Conformance

`ideal_loads_constant_supply_humidity_heating_conformance_candidate_001`
promotes the no-OA `ConstantSupplyHumidityRatio` heating lane for declared
thermostat setpoints, heating/cooling total/sensible/latent rate rows,
supply-air heating/cooling report rows, ReportPurchasedAir energy/fuel rows,
and supply-node temperature/mass-flow/humidity rows. It reuses the diagnostic
fixture IDF and promotes the hourly, monthly, and run-period
`DistrictHeatingWater:Facility` and `DistrictCooling:Facility` meters from the
meter side. It keeps return-node and zone-air-node humidity rows, annual meter
rows, and broader meter behavior as diagnostic proof only.

The compare run has `comparison_class = "conformance"`, `conformance_claim =
true`, `tolerance_policy: conformance-gate`, and `status: pass`. It compares
36 Detailed ESO series with 29 conformance rows and 7 diagnostic proof rows,
plus six conformance meter rows: hourly, monthly, and run-period for both facility meters.
Humidistat, outdoor-air humidity, finite-limit humidity-control behavior, and
broad humidity-control conformance remain outside the claim.

## IdealLoads Humidistat Dehumidification Conformance

`ideal_loads_humidistat_dehumidification_conformance_candidate_001` promotes
the no-OA `Humidistat` dehumidification lane for declared thermostat setpoints,
heating/cooling total/sensible/latent rate rows, supply-air heating/cooling
report rows, paired trace-driven moisture-demand rows, and supply-node
temperature/mass-flow/humidity rows, plus ReportPurchasedAir energy/fuel rows.
It reuses the diagnostic fixture IDF and keeps return-node and zone-air-node
humidity rows, annual meter rows, fully owned moisture-history closure, and
broad meter behavior as diagnostic proof only; the
hourly, monthly, and run-period `DistrictHeatingWater:Facility` and
`DistrictCooling:Facility` meters are conformance rows.
The seeded closed-loop ThirdOrder predictor and `SimPurchasedAir` results are
the promoted calculation path and moisture-demand rows for this declared
candidate. The corrected zone-humidity and history-residual comparisons remain
diagnostic evidence for isolating `WPrevZoneTSTemp` warmup/system-history
differences; the path is still seeded and forced by EnergyPlus trace inputs and
is not a standalone Humidistat simulation.
The seeded fixed zone-timestep predictor-to-corrector transition and both
humidity-history pushes are owned atomically by
`ep_runtime::advance_no_oa_humidistat_zone_timestep_compat`. Adaptive or
multiple system substeps remain outside this boundary. EnergyPlus trace data
still supplies the initial histories and timestep forcing, so this ownership
change does not promote fully standalone history closure.

The compare run has `comparison_class = "conformance"`, `conformance_claim =
true`, `tolerance_policy: conformance-gate`, and `status: pass`. It compares
38 Detailed ESO series with 31 conformance rows and 7 diagnostic proof rows,
plus six conformance meter rows: hourly, monthly, and run-period for both
facility meters. Fully owned `WPrevZoneTSTemp` warmup/system-history closure,
outdoor-air humidity, finite-limit humidity-control behavior, and broad
humidity-control conformance remain outside the claim.

## IdealLoads Humidistat Humidification Conformance

`ideal_loads_humidistat_humidification_conformance_candidate_001` promotes the
no-OA `Humidistat` humidification lane for declared thermostat setpoints,
heating/cooling total/sensible/latent rate rows, supply-air heating/cooling
report rows, paired trace-driven moisture-demand rows, and supply-node
temperature/mass-flow/humidity rows, plus ReportPurchasedAir energy/fuel rows.
It reuses the diagnostic fixture IDF and keeps return-node and zone-air-node
humidity rows, annual meter rows, fully owned moisture-history closure, and
broad meter behavior as diagnostic proof only; the
hourly, monthly, and run-period `DistrictHeatingWater:Facility` and
`DistrictCooling:Facility` meters are conformance rows.
The report uses the same seeded closed-loop predictor and `SimPurchasedAir`
results as the promoted humidification calculation path. Its corrected
zone-humidity and history-residual comparisons remain diagnostic, and broader
promotion still waits on non-oracle warmup/system-history closure for
`WPrevZoneTSTemp`.

The compare run has `comparison_class = "conformance"`, `conformance_claim =
true`, `tolerance_policy: conformance-gate`, and `status: pass`. It compares
38 Detailed ESO series with 31 conformance rows and 7 diagnostic proof rows,
plus six conformance meter rows: hourly, monthly, and run-period for both
facility meters. Fully owned `WPrevZoneTSTemp` warmup/system-history closure,
outdoor-air humidity, finite-limit humidity-control behavior, and broad
humidity-control conformance remain outside the claim.

## IdealLoads Outdoor-Air Design Flow And Economizer Conformance

`ideal_loads_outdoor_air_flow_zone_conformance_candidate_001` promotes the
blank-schedule `DesignSpecification:OutdoorAir` `Flow/Zone` lane for declared
outdoor-air mass flow, standard-density volume flow, no-humidity outdoor-air
sensible/latent/total report rates, supply-air mass/volume/temperature/humidity
state, and mixed-air temperature/humidity state rows. It uses
`comparison_class = "conformance"`, `conformance_claim = true`,
`tolerance_policy: conformance-gate`, and `status: pass`.

`ideal_loads_outdoor_air_flow_person_conformance_candidate_001` promotes the
matching blank-schedule `DesignSpecification:OutdoorAir` `Flow/Person` lane
for the same declared output surface. It uses a typed five-person design
occupant count and 0.01 m3/s-person outdoor-air rate to derive the 0.05 m3/s
design volume. People heat-gain behavior is not part of this claim.

`ideal_loads_outdoor_air_flow_area_conformance_candidate_001` promotes the
matching blank-schedule `DesignSpecification:OutdoorAir` `Flow/Area` lane for
the same declared output surface. It derives a 1 m2 floor area from typed floor
surfaces and applies 0.05 m3/s-m2 outdoor air to derive the 0.05 m3/s design
volume.

`ideal_loads_outdoor_air_air_changes_conformance_candidate_001` promotes the
matching blank-schedule `DesignSpecification:OutdoorAir` `AirChanges/Hour`
lane for the same declared output surface. It derives a 1 m3 typed zone volume
and applies 180 ACH to derive the 0.05 m3/s design volume.

`ideal_loads_outdoor_air_sum_conformance_candidate_001` promotes the matching
blank-schedule `DesignSpecification:OutdoorAir` `Sum` lane for the same
declared output surface. It reports the component proof terms separately:
0.000 m3/s Flow/Person, 0.015 m3/s Flow/Area, 0.025 m3/s Flow/Zone, and
0.010 m3/s AirChanges/Hour, which sum to the 0.05 m3/s design volume.

`ideal_loads_outdoor_air_maximum_conformance_candidate_001` promotes the
matching blank-schedule `DesignSpecification:OutdoorAir` `Maximum` lane for
the same declared output surface. It reports the same component proof terms as
the Sum lane, but selects the 0.050 m3/s AirChanges/Hour term as the governing
design volume.

The six no-economizer design-flow candidates each compare 22 Detailed series
with 14 conformance rows and 8 diagnostic proof rows. The diagnostic rows are
inactive economizer and inactive heat-recovery report variables only. The
guard requires the declared outdoor-air method, `NoEconomizer`, no heat
recovery, no OA schedule, no finite flow/capacity limits, no DCV, default
`ConstantSensibleHeatRatio` dehumidification, and no humidification control.

`ideal_loads_outdoor_air_differential_dry_bulb_economizer_conformance_candidate_001`
promotes the matching Flow/Zone low-minimum outdoor-air lane with
`DifferentialDryBulb` economizer enabled. It compares 110 source-order Detailed
samples, promotes the 14 outdoor-air/supply/mixed rows plus
`Zone Ideal Loads Economizer Active Time`, checks that active time is nonzero,
and verifies outdoor-air mass flow rises above the 0.001 m3/s design minimum.
Inactive heat-recovery rows remain diagnostic-only proof rows in this case.

`ideal_loads_outdoor_air_differential_enthalpy_economizer_conformance_candidate_001`
promotes the matching Flow/Zone low-minimum outdoor-air lane with
`DifferentialEnthalpy` economizer enabled. It compares 110 source-order
Detailed samples, promotes the same 14 outdoor-air/supply/mixed rows plus
`Zone Ideal Loads Economizer Active Time`, checks that active time is nonzero,
and verifies outdoor-air mass flow rises above the 0.001 m3/s design minimum
when outdoor enthalpy is below recirculation enthalpy. Inactive heat-recovery
rows remain diagnostic-only proof rows in this case.

`ideal_loads_outdoor_air_sensible_heat_recovery_conformance_candidate_001`
promotes the matching Flow/Zone blank-schedule outdoor-air lane with
`NoEconomizer` and `HeatRecoveryType = Sensible`. It compares 96 Detailed
samples, promotes the same 14 outdoor-air/supply/mixed rows plus the six
Sensible heat-recovery rate rows and `Zone Ideal Loads Heat Recovery Active
Time`, keeps inactive economizer active time diagnostic-only, and requires
heat-recovery active time to be nonzero. Sensible heat recovery leaves humidity
unchanged, so the latent heat-recovery rows remain zero-valued conformance rows
inside this narrow fixture.

Enthalpy heat recovery, active DCV, active humidity controls,
heat-recovery saturation-limit branches, and broad IdealLoads outdoor-air
conformance remain outside this claim.

## IdealLoads Outdoor-Air Design-Flow Diagnostic

`ideal_loads_outdoor_air_flow_person_diagnostic_001`,
`ideal_loads_outdoor_air_design_flow_diagnostic_001`,
`ideal_loads_outdoor_air_flow_area_diagnostic_001`,
`ideal_loads_outdoor_air_air_changes_diagnostic_001`,
`ideal_loads_outdoor_air_sum_diagnostic_001`,
`ideal_loads_outdoor_air_maximum_diagnostic_001`,
`ideal_loads_outdoor_air_differential_dry_bulb_economizer_diagnostic_001`,
`ideal_loads_outdoor_air_differential_enthalpy_economizer_diagnostic_001`,
`ideal_loads_outdoor_air_sensible_heat_recovery_diagnostic_001`, and
`ideal_loads_outdoor_air_enthalpy_heat_recovery_diagnostic_001` are diagnostic
predecessor and remaining proof lanes. The DifferentialDryBulb and
DifferentialEnthalpy diagnostics are predecessors for the promoted economizer
conformance candidates; the Sensible heat-recovery diagnostic is the
predecessor for the promoted Sensible heat-recovery conformance candidate; and
Enthalpy heat-recovery remains a diagnostic-only active outdoor-air proof lane.

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
economizer is active. The Sensible heat-recovery diagnostic and conformance
fixtures keep 96 samples with `NoEconomizer` and validate active heat-recovery
time plus sensible/total rate rows; the promoted candidate also gates the
unchanged-humidity latent zero rows. The Enthalpy heat-recovery fixture keeps
96 samples with `NoEconomizer`, records `ZONE ONE RETURN` as the EnergyPlus
return/recirculation node, and validates active heat-recovery time plus
sensible, latent, and total rate rows. The no-economizer flow rows are exact;
the active economizer flow rows use narrow diagnostic tolerances. The sensible
and total outdoor-air report rows are diagnostic or conformance according to
each manifest's output levels with a 1 W source-order tolerance. The no-humidity
latent report, supply-air state, and mixed-air state rows are diagnostic or
conformance according to each manifest's output levels; supply-air temperature
stays within 0.02 C, while the Enthalpy heat-recovery humidity-ratio and
heat-recovery rate rows now use strict fixture tolerances after the compare
lane was aligned to EnergyPlus `ZoneRecircAirNodeNum` same-call state. The
inactive heat-recovery rows are exact zeros for the non-heat-recovery fixtures,
and economizer active time is exact for the inactive, DifferentialDryBulb,
DifferentialEnthalpy, and Sensible heat-recovery branches.

This evidence does not promote outdoor-air methods beyond the separate
Flow/Zone, Flow/Person, Flow/Area, AirChanges/Hour, Sum, Maximum, and
Flow/Zone DifferentialDryBulb/DifferentialEnthalpy economizer and Sensible
heat-recovery conformance candidates, Enthalpy heat recovery, active DCV,
active humidity controls, saturation-limit heat-recovery branches, or broad
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

Variables outside the declared no-OA/no-limit, numeric finite-limit, and
ConstantSensibleHeatRatio cooling boundaries remain baseline-only or
diagnostic-only evidence until they receive their own source map, Rust state,
oracle evidence, and blocking gate.
