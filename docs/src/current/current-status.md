---
status: active
claim_level: none
owner: core
last_reviewed: 2026-06-15
---

# Current Status

The current public release line is v0.1.0. It packages the accumulated limited
conformance evidence, user coverage handbook, release evidence manifest,
arbitrary IDF/epJSON support-assessment pipeline, no-console Windows launcher,
and locked EnergyPlus 26.1.0 oracle runtime.

Current numerical conformance is limited to promoted cases and their declared
variables:

- `heat_balance_nomass_001`
- `surface_temperature_nomass_001`, including no-mass adiabatic surface
  conduction rate/per-area series
- `schedule_constant_001`
- `weather_fields_001` dry-bulb only
- `internal_gains_001` `Zone Total Internal Convective Heating Rate` only
- `official_1zone_uncontrolled_dynamic_conformance_candidate_001` declared
  weather, zone-air, surface temperature, and surface conduction variables only
- `ideal_loads_no_oa_sensible_conformance_001` declared no-OA/no-limit
  IdealLoads sensible thermostat, rate, and supply-node variables only
- `ideal_loads_no_oa_report_energy_conformance_candidate_001` declared
  no-OA ReportPurchasedAir supply-air and zone total heating/cooling
  non-fuel energy variables only
- `ideal_loads_blank_fuel_efficiency_conformance_candidate_001` declared
  no-OA blank fuel-efficiency fuel energy-rate and fuel energy variables only
- `ideal_loads_constant_fuel_efficiency_conformance_candidate_001` declared
  no-OA constant Schedule:Constant fuel-efficiency fuel energy-rate and fuel
  energy variables only
- `ideal_loads_non_constant_fuel_efficiency_conformance_candidate_001`
  declared no-OA all-days Schedule:Compact fuel-efficiency fuel energy-rate
  and fuel energy variables only
- `ideal_loads_capacity_limit_conformance_001` declared no-OA numeric
  capacity-limit IdealLoads thermostat, rate, and supply-node variables only
- `ideal_loads_flow_limit_conformance_001` declared no-OA numeric flow-limit
  IdealLoads thermostat, rate, and supply-node variables only
- `ideal_loads_flow_capacity_limit_conformance_001` declared no-OA numeric
  flow-and-capacity-limit IdealLoads thermostat, rate, and supply-node
  variables only
- `ideal_loads_constant_shr_conformance_001` declared no-OA
  `ConstantSensibleHeatRatio` cooling thermostat, rate, and supply-node
  variables only
- `ideal_loads_constant_supply_humidity_cooling_conformance_candidate_001`
  declared no-OA `ConstantSupplyHumidityRatio` cooling thermostat, rate, and
  supply-node variables, ReportPurchasedAir energy/fuel rows, and
  hourly/monthly/run-period
  `DistrictHeatingWater:Facility`/`DistrictCooling:Facility` meters only
- `ideal_loads_constant_supply_humidity_cooling_annual_meter_conformance_candidate_001`
  declared no-OA full-year `ConstantSupplyHumidityRatio` cooling annual
  `DistrictHeatingWater:Facility`/`DistrictCooling:Facility` meters only
- `ideal_loads_constant_supply_humidity_heating_conformance_candidate_001`
  declared no-OA `ConstantSupplyHumidityRatio` heating thermostat, rate, and
  supply-node variables, ReportPurchasedAir energy/fuel rows, and
  hourly/monthly/run-period
  `DistrictHeatingWater:Facility`/`DistrictCooling:Facility` meters only
- `ideal_loads_constant_supply_humidity_heating_annual_meter_conformance_candidate_001`
  declared no-OA full-year `ConstantSupplyHumidityRatio` heating annual
  `DistrictHeatingWater:Facility`/`DistrictCooling:Facility` meters only
- `ideal_loads_humidistat_dehumidification_conformance_candidate_001`
  declared no-OA Humidistat dehumidification thermostat, rate, and supply-node
  variables, ReportPurchasedAir energy/fuel rows, and hourly/monthly/run-period
  `DistrictHeatingWater:Facility`/`DistrictCooling:Facility` meters only
- `ideal_loads_humidistat_dehumidification_annual_meter_conformance_candidate_001`
  declared no-OA full-year Humidistat dehumidification annual
  `DistrictHeatingWater:Facility`/`DistrictCooling:Facility` meters only
- `ideal_loads_humidistat_humidification_conformance_candidate_001` declared
  no-OA Humidistat humidification thermostat, rate, and supply-node variables,
  ReportPurchasedAir energy/fuel rows, and hourly/monthly/run-period
  `DistrictHeatingWater:Facility`/`DistrictCooling:Facility` meters only
- `ideal_loads_humidistat_humidification_annual_meter_conformance_candidate_001`
  declared no-OA full-year Humidistat humidification annual
  `DistrictHeatingWater:Facility`/`DistrictCooling:Facility` meters only
- `ideal_loads_no_oa_facility_meter_conformance_candidate_001` declared
  no-OA hourly `DistrictHeatingWater:Facility` and `DistrictCooling:Facility`
  meters only
- `ideal_loads_no_oa_facility_meter_monthly_run_period_conformance_candidate_001`
  declared no-OA monthly, annual, and run-period
  `DistrictHeatingWater:Facility` and `DistrictCooling:Facility` meters only
- `ideal_loads_outdoor_air_flow_zone_conformance_candidate_001` declared
  outdoor-air Flow/Zone mass/volume, no-humidity report-rate, supply-air
  state, and mixed-air state variables only
- `ideal_loads_outdoor_air_flow_person_conformance_candidate_001` declared
  outdoor-air Flow/Person mass/volume, no-humidity report-rate, supply-air
  state, and mixed-air state variables only
- `ideal_loads_outdoor_air_occupancy_dcv_conformance_candidate_001` declared
  outdoor-air Flow/Person OccupancySchedule DCV mass/volume, no-humidity
  report-rate, supply-air state, and mixed-air state variables only
- `ideal_loads_outdoor_air_co2_dcv_conformance_candidate_001` declared
  outdoor-air Flow/Person CO2Setpoint DCV mass/volume, no-humidity
  report-rate, supply-air state, and mixed-air state variables only; CO2
  contaminant-balance/concentration conformance and broader DCV combinations
  remain outside the claim
- `ideal_loads_outdoor_air_flow_area_conformance_candidate_001` declared
  outdoor-air Flow/Area mass/volume, no-humidity report-rate, supply-air
  state, and mixed-air state variables only
- `ideal_loads_outdoor_air_air_changes_conformance_candidate_001` declared
  outdoor-air AirChanges/Hour mass/volume, no-humidity report-rate,
  supply-air state, and mixed-air state variables only
- `ideal_loads_outdoor_air_sum_conformance_candidate_001` declared
  outdoor-air Sum mass/volume, no-humidity report-rate, supply-air state, and
  mixed-air state variables only
- `ideal_loads_outdoor_air_maximum_conformance_candidate_001` declared
  outdoor-air Maximum mass/volume, no-humidity report-rate, supply-air state,
  and mixed-air state variables only
- `ideal_loads_outdoor_air_differential_dry_bulb_economizer_conformance_candidate_001`
  declared outdoor-air Flow/Zone DifferentialDryBulb economizer mass/volume,
  no-humidity report-rate, supply-air state, mixed-air state, and economizer
  active-time variables only
- `ideal_loads_outdoor_air_differential_enthalpy_economizer_conformance_candidate_001`
  declared outdoor-air Flow/Zone DifferentialEnthalpy economizer mass/volume,
  no-humidity report-rate, supply-air state, mixed-air state, and economizer
  active-time variables only
- `ideal_loads_outdoor_air_sensible_heat_recovery_conformance_candidate_001`
  declared outdoor-air Flow/Zone Sensible heat-recovery mass/volume,
  no-humidity report-rate, supply-air state, mixed-air state, heat-recovery
  rate, and heat-recovery active-time variables only
- `ideal_loads_outdoor_air_enthalpy_heat_recovery_conformance_candidate_001`
  declared outdoor-air Flow/Zone Enthalpy heat-recovery mass/volume,
  no-humidity report-rate, supply-air state, mixed-air state, heat-recovery
  rate, and heat-recovery active-time variables only; general heat-recovery
  saturation-limit branch parity remains outside the claim

## Current Evidence Boundary

| Area | Current conformance | Diagnostic or baseline evidence | Not claimed |
|---|---|---|---|
| Numerical time series | 38 promoted conformance manifests, 657 declared hourly/detailed ESO series, the declared no-OA hourly facility meter pairs, the declared humidity-control hourly/monthly/run-period facility meter pairs, the declared humidity-control full-year annual facility meter pairs for four branches, plus the declared no-OA monthly/annual/run-period facility meter pairs, and 22 passed release-evidence series | `official_1zone_uncontrolled_baseline_001` keeps oracle series; `official_1zone_uncontrolled_dynamic_diagnostic_001` keeps broad run-period-filtered probe deltas; `official_1zone_uncontrolled_dynamic_conformance_candidate_001` gates the declared compatibility-candidate variable set; `ideal_loads_no_oa_sensible_conformance_001` gates the declared no-OA/no-limit IdealLoads sensible variable set; `ideal_loads_no_oa_report_energy_conformance_candidate_001` gates only the declared no-OA ReportPurchasedAir non-fuel energy rows; `ideal_loads_blank_fuel_efficiency_conformance_candidate_001` gates only the declared no-OA blank fuel-efficiency fuel energy-rate and fuel energy rows; `ideal_loads_constant_fuel_efficiency_conformance_candidate_001` gates only the declared no-OA constant Schedule:Constant fuel-efficiency fuel energy-rate and fuel energy rows; `ideal_loads_non_constant_fuel_efficiency_conformance_candidate_001` gates only the declared no-OA all-days Schedule:Compact fuel-efficiency fuel energy-rate and fuel energy rows; `ideal_loads_capacity_limit_conformance_001`, `ideal_loads_flow_limit_conformance_001`, and `ideal_loads_flow_capacity_limit_conformance_001` gate the declared no-OA finite-limit IdealLoads sensible variable set; `ideal_loads_constant_shr_conformance_001` gates the declared no-OA ConstantSensibleHeatRatio cooling variable set; `ideal_loads_constant_supply_humidity_cooling_conformance_candidate_001` and `ideal_loads_constant_supply_humidity_heating_conformance_candidate_001` gate the declared no-OA ConstantSupplyHumidityRatio cooling/heating variable sets and hourly/monthly/run-period DistrictHeatingWater/DistrictCooling facility meters; `ideal_loads_constant_supply_humidity_cooling_annual_meter_conformance_candidate_001` and `ideal_loads_constant_supply_humidity_heating_annual_meter_conformance_candidate_001` gate only the declared full-year annual DistrictHeatingWater/DistrictCooling facility meters for ConstantSupplyHumidityRatio cooling/heating; `ideal_loads_humidistat_dehumidification_conformance_candidate_001` and `ideal_loads_humidistat_humidification_conformance_candidate_001` gate the declared no-OA Humidistat humidity-control variable sets, closed-loop moisture-demand rows, and hourly/monthly/run-period DistrictHeatingWater/DistrictCooling facility meters; `ideal_loads_humidistat_dehumidification_annual_meter_conformance_candidate_001` and `ideal_loads_humidistat_humidification_annual_meter_conformance_candidate_001` gate only the declared full-year annual DistrictHeatingWater/DistrictCooling facility meters for Humidistat dehumidification/humidification; `ideal_loads_no_oa_facility_meter_conformance_candidate_001` gates only the declared no-OA hourly DistrictHeatingWater/DistrictCooling facility meters; `ideal_loads_no_oa_facility_meter_monthly_run_period_conformance_candidate_001` gates only the declared no-OA monthly/annual/run-period DistrictHeatingWater/DistrictCooling facility meters; `ideal_loads_outdoor_air_flow_zone_conformance_candidate_001`, `ideal_loads_outdoor_air_flow_person_conformance_candidate_001`, `ideal_loads_outdoor_air_occupancy_dcv_conformance_candidate_001`, `ideal_loads_outdoor_air_co2_dcv_conformance_candidate_001`, `ideal_loads_outdoor_air_flow_area_conformance_candidate_001`, `ideal_loads_outdoor_air_air_changes_conformance_candidate_001`, `ideal_loads_outdoor_air_sum_conformance_candidate_001`, `ideal_loads_outdoor_air_maximum_conformance_candidate_001`, `ideal_loads_outdoor_air_differential_dry_bulb_economizer_conformance_candidate_001`, `ideal_loads_outdoor_air_differential_enthalpy_economizer_conformance_candidate_001`, `ideal_loads_outdoor_air_sensible_heat_recovery_conformance_candidate_001`, and `ideal_loads_outdoor_air_enthalpy_heat_recovery_conformance_candidate_001` gate the declared outdoor-air Flow/Zone, Flow/Person, Flow/Person OccupancySchedule DCV, Flow/Person CO2Setpoint DCV, Flow/Area, AirChanges/Hour, Sum, Maximum, Flow/Zone DifferentialDryBulb/DifferentialEnthalpy economizer, and Flow/Zone Sensible/Enthalpy heat-recovery variable sets | broad ExampleFiles dynamic conformance |
| Static model | official `1ZoneUncontrolled` EIO surface geometry, Construction CTF, Material CTF Summary, and OtherEquipment nominal fields | generated support/index/release evidence PDFs | dynamic behavior from the static EIO case |
| Heat balance | no-mass zone MAT, no-mass surface inside/outside temperature, no-mass adiabatic conduction series, selected official `1ZoneUncontrolled` dynamic weather/zone-air/surface-temperature/surface-conduction and conduction-per-area variables, and the declared floor `Surface Heat Storage Rate` / `Surface Heat Storage Rate per Area` rows under dedicated flux/storage tolerances | official `1ZoneUncontrolled` broad diagnostic decomposition, radiation/solar/convection diagnostics, and non-promoted probe lanes | broad CTF storage parity, EnergyPlus warmup convergence parity outside the official candidate, solar, radiation exchange, fenestration, infiltration, zone air predictor/corrector parity, or general heat-balance compatibility |
| Time, weather, schedule | `Schedule Value` and `Site Outdoor Air Drybulb Temperature` hourly series | dewpoint, relative humidity, pressure, wind speed, and wind direction diagnostics | broad weather processor compatibility |
| Internal gains | `Zone Total Internal Convective Heating Rate` for `internal_gains_001` | static OtherEquipment nominal fields | zone air temperature response to gains, radiant/latent coupling, or broad internal-gain compatibility |
| HVAC, node, plant | no-OA/no-limit and numeric finite-limit IdealLoads sensible conformance plus no-OA ReportPurchasedAir non-fuel energy conformance, no-OA blank fuel-efficiency conformance, no-OA constant Schedule:Constant fuel-efficiency conformance, no-OA all-days Schedule:Compact fuel-efficiency conformance, no-OA ConstantSensibleHeatRatio cooling, ConstantSupplyHumidityRatio cooling/heating, Humidistat dehumidification/humidification, no-OA hourly DistrictHeatingWater/DistrictCooling facility meter conformance in the dedicated meter candidate, no-OA hourly/monthly/run-period DistrictHeatingWater/DistrictCooling facility meter conformance in the humidity-control candidates, full-year annual DistrictHeatingWater/DistrictCooling facility meter conformance for ConstantSupplyHumidityRatio cooling/heating and Humidistat dehumidification/humidification, plus monthly/annual/run-period DistrictHeatingWater/DistrictCooling facility meter conformance in the meter-only candidate, and outdoor-air Flow/Zone/Flow/Person/Flow/Person OccupancySchedule DCV/Flow/Person CO2Setpoint DCV/Flow/Area/AirChanges/Hour/Sum/Maximum/DifferentialDryBulb/DifferentialEnthalpy economizer and Sensible/Enthalpy heat-recovery conformance for declared thermostat, IdealLoads rate, ReportPurchasedAir non-fuel energy, declared fuel energy-rate/fuel energy, supply-node, outdoor-air, supply-air-state, mixed-air-state, economizer-active-time, Sensible/Enthalpy heat-recovery rate, Sensible/Enthalpy heat-recovery active-time variables, and declared no-OA facility meters only | node proof rows, blank/constant/all-days Schedule:Compact fuel-efficiency raw-rate and facility-meter proof rows, remaining IdealLoads facility meter diagnostics outside the declared meter rows, finite-limit return-node proof rows, ConstantSensibleHeatRatio return/zone-node humidity proof rows, ConstantSupplyHumidityRatio cooling/heating return-node/zone-air humidity proof rows, no-OA Humidistat dehumidification/humidification return-node/zone-air humidity proof rows, seeded closed-loop Humidistat humidity diagnostics, remaining IdealLoads outdoor-air predecessor diagnostics, and plant-loop baseline/diagnostic reports | broad HVAC, broad node, full IdealLoads, broader DCV combinations, CO2 contaminant-balance/concentration conformance, broad or remaining humidity-control branches, outdoor-air methods beyond Flow/Zone/Flow/Person/Flow/Person OccupancySchedule DCV/Flow/Person CO2Setpoint DCV/Flow/Area/AirChanges/Hour/Sum/Maximum/DifferentialDryBulb/DifferentialEnthalpy economizer and declared Sensible/Enthalpy heat recovery, broad humidistat schedule/history generalization beyond the declared seeded no-OA candidates, heat-recovery saturation-limit branch generality, fuel-efficiency schedules beyond the declared blank/constant/all-days Schedule:Compact candidates, broad meter conformance beyond the declared no-OA hourly, humidity-control hourly/monthly/run-period, four full-year humidity-control annual, and meter-only monthly/annual/run-period facility meter rows, multi-year annual grouping, annual meter rows in the short-run humidity-control candidates, and plant numerical conformance |

The repository also contains smoke, baseline-only, and diagnostic evidence for
model intake, additional weather variables, local fixture geometry/internal
gain checks, node projection, IdealLoads typed graph work, and plant-loop
diagnostic plumbing. Those artifacts are useful development evidence, but
they are not general compatibility claims.

The active dynamic expansion target is tracked by
`scripts\dev.cmd v0.26-dynamic-idf-inventory`. As of the current inventory,
12 IDF-backed case manifests through v0.26 are in scope, 5 have dynamic
conformance-gated evidence, and 4 are active dynamic gaps after static fixtures
and static-only evidence are separated out. There is 1 EnergyPlus ExampleFile
dynamic candidate through v0.26, `1ZoneUncontrolled.idf`, and it is now split
into a diagnostic probe tracker plus a blocking compatibility-candidate
conformance gate for declared variables.

Current static model conformance is limited to:

- `official_1zone_static_model_001`
- declared static EIO surface geometry fields
- declared Construction CTF and Material CTF Summary fields
- declared OtherEquipment Internal Gains Nominal fields
The historical v0.1 through v0.15 boundaries are summarized in
`specs/milestones.toml`; their old planning pages are intentionally not
retained in the docs tree.

The current public scope includes:

- Rust workspace and pinned toolchain
- repo-local EnergyPlus 26.1.0 oracle and reference source setup
- repo-local portable Python for reporting
- RawModel and TypedModel intake for declared seed objects
- conformance manifests, output requests, tolerance rules, gates, and reports
- output request injection for staged oracle baselines
- selected-series timestamp alignment, RMSE, relative-delta, and first
  divergence reporting
- release conformance index reports with case, output, meter, domain, report,
  and gate coverage matrices
- user-facing support coverage reports with input object, output variable, and
  algorithm support matrices
- user coverage handbooks that reorganize supported inputs, outputs,
  algorithms, promoted cases, and known gaps around user decision rules
- release evidence asset manifests with package/report paths, SHA-256 hashes,
  content types, user-facing purposes, and JSON evidence summaries
- source-map and algorithm ledger checks that validate EnergyPlus source
  anchors, Rust target anchors, first cases, proof variables, and blocking
  gates
- timestamp-aligned conformance reports for declared schedule and dry-bulb
  hourly series
- static EIO model conformance reports for the official `1ZoneUncontrolled`
  ExampleFile
- runtime output registry handles for currently implemented output variables
  and IdealLoads facility meter request handles
- explicit unavailable-output and unavailable-meter runtime diagnostics
- ResultStore duplicate-handle/duplicate-series diagnostics and profile
  scaffolding
- opaque no-mass adiabatic and interzone surface boundary target handling in
  heat-balance state
- timestamp-aligned internal convective gain conformance for the declared
  `internal_gains_001` hourly ESO series
- official dynamic heat-balance conformance reports that run a Rust warmup
  loop aligned to the EnergyPlus EIO run-period warmup day count, filter oracle
  ESO values to run-period samples, gate declared weather, zone-air,
  roof/wall/floor face-temperature, conduction, conduction per-area, and floor
  Surface Heat Storage Rate plus Surface Heat Storage Rate per Area series in the
  compatibility-candidate lane while keeping broad
  storage/radiation/solar/convection decomposition evidence diagnostic-only
- limited IdealLoads no-OA/no-limit sensible conformance for declared
  thermostat setpoints, IdealLoads total/sensible/supply-air rates, and
  supply-node temperature/mass-flow Detailed series, with ReportPurchasedAir
  non-fuel energy rows promoted only in the separate
  `ideal_loads_no_oa_report_energy_conformance_candidate_001` case,
  blank fuel-efficiency rows promoted only in the separate
  `ideal_loads_blank_fuel_efficiency_conformance_candidate_001` case,
  constant Schedule:Constant fuel-efficiency rows promoted only in the separate
  `ideal_loads_constant_fuel_efficiency_conformance_candidate_001` case,
  all-days Schedule:Compact fuel-efficiency rows promoted only in the separate
  `ideal_loads_non_constant_fuel_efficiency_conformance_candidate_001` case,
  hourly DistrictHeatingWater/DistrictCooling facility meters in that case
  compared as oracle-MTR vs Rust aggregated fuel-energy diagnostics with
  RuntimeMeterRegistry request resolution, while broad meter conformance beyond
  the declared meter candidates, multi-year annual grouping, humidity,
  predictor/corrector proof rows, outdoor-air, adaptive system timestep,
  sizing, fuel-efficiency schedules beyond the declared blank/constant/all-days
  Schedule:Compact candidates, and broad HVAC compatibility kept outside the
  claim
- limited IdealLoads no-OA ReportPurchasedAir energy conformance for declared
  supply-air total heating/cooling energy and zone total heating/cooling
  energy Detailed rows, with constant Schedule:Constant fuel-efficiency rows
  promoted only in the separate
  `ideal_loads_constant_fuel_efficiency_conformance_candidate_001` case and
  blank fuel-efficiency rows promoted only in the separate
  `ideal_loads_blank_fuel_efficiency_conformance_candidate_001` case and
  all-days Schedule:Compact fuel-efficiency rows promoted only in the separate
  `ideal_loads_non_constant_fuel_efficiency_conformance_candidate_001` case;
  raw rate, thermostat, demand, humidity, node, and facility meter rows are
  kept diagnostic proof only
- limited IdealLoads no-OA blank fuel-efficiency conformance for declared
  supply-air/zone heating/cooling fuel energy-rate and fuel energy Detailed
  rows, with raw IdealLoads rates and facility meters kept diagnostic proof
  only; constant Schedule:Constant and all-days Schedule:Compact efficiency
  schedules remain outside this claim
- limited IdealLoads no-OA constant Schedule:Constant fuel-efficiency
  conformance for declared supply-air/zone heating/cooling fuel energy-rate
  and fuel energy Detailed rows, with raw IdealLoads rates and facility meters
  kept diagnostic proof only; blank and all-days Schedule:Compact efficiency
  schedules remain outside this claim
- limited IdealLoads no-OA all-days Schedule:Compact fuel-efficiency
  conformance for declared supply-air/zone heating/cooling fuel energy-rate
  and fuel energy Detailed rows, with raw IdealLoads rates and facility meters
  kept diagnostic proof only; blank and constant Schedule:Constant efficiency
  schedules remain outside this claim
- limited IdealLoads no-OA hourly facility meter conformance for
  `DistrictHeatingWater:Facility` and `DistrictCooling:Facility`, with
  ReportPurchasedAir rate, energy, fuel-energy, thermostat, demand, humidity,
  and node rows kept diagnostic proof only
- limited IdealLoads no-OA monthly/annual/run-period facility meter conformance for
  `DistrictHeatingWater:Facility` and `DistrictCooling:Facility`, with
  ReportPurchasedAir rate, energy, fuel-energy, thermostat, demand, humidity,
  and node rows kept diagnostic proof only
- limited IdealLoads no-OA numeric capacity-limit conformance for declared
  thermostat setpoints, IdealLoads total/sensible/supply-air rates, and
  supply-node temperature/mass-flow Detailed series, with return-node and
  humidity rows kept diagnostic-only
- limited IdealLoads no-OA numeric flow-limit conformance for declared
  thermostat setpoints, IdealLoads total/sensible/supply-air rates, and
  supply-node temperature/mass-flow Detailed series, with return-node and
  humidity rows kept diagnostic-only
- limited IdealLoads no-OA numeric flow-and-capacity-limit conformance for
  declared thermostat setpoints, IdealLoads total/sensible/supply-air rates,
  and supply-node temperature/mass-flow Detailed series, with return-node and
  humidity rows kept diagnostic-only
- limited IdealLoads no-OA `ConstantSensibleHeatRatio` cooling conformance for
  declared thermostat setpoints, cooling total/sensible/latent rate rows, and
  supply-node temperature/mass-flow/humidity Detailed series, with return-node
  and zone-air humidity rows kept diagnostic-only
- limited IdealLoads no-OA `ConstantSupplyHumidityRatio` cooling/heating
  conformance for declared thermostat setpoints, heating/cooling
  total/sensible/latent rate rows, supply-air heating/cooling report rows,
  ReportPurchasedAir energy/fuel rows,
  supply-node temperature/mass-flow/humidity Detailed series, and
  hourly/monthly/run-period `DistrictHeatingWater:Facility`/
  `DistrictCooling:Facility` meters, with return-node/zone-air humidity rows,
  annual meter rows in the short-run candidates and other broader meter
  frequencies kept outside the claim
- limited IdealLoads no-OA full-year `ConstantSupplyHumidityRatio`
  cooling/heating annual facility meter conformance for
  `DistrictHeatingWater:Facility` and `DistrictCooling:Facility` in
  `ideal_loads_constant_supply_humidity_cooling_annual_meter_conformance_candidate_001`
  and
  `ideal_loads_constant_supply_humidity_heating_annual_meter_conformance_candidate_001`,
  with Detailed humidity-control branch rows kept diagnostic proof only
- limited IdealLoads no-OA Humidistat dehumidification/humidification
  conformance for declared thermostat setpoints, heating/cooling
  total/sensible/latent rate rows, supply-air heating/cooling report rows,
  ReportPurchasedAir energy/fuel rows, paired closed-loop moisture-demand
  rows, and supply-node
  temperature/mass-flow/humidity Detailed series, plus hourly/monthly/run-period
  `DistrictHeatingWater:Facility`/`DistrictCooling:Facility` meters, with
  return-node/zone-air humidity rows and seeded closed-loop humidity rows kept
  as diagnostic proof, annual meter rows in the short-run candidates, and
  broader humidity-control/meter frequencies kept outside the claim
- limited IdealLoads no-OA full-year Humidistat dehumidification annual
  facility meter conformance for `DistrictHeatingWater:Facility` and
  `DistrictCooling:Facility` in
  `ideal_loads_humidistat_dehumidification_annual_meter_conformance_candidate_001`,
  with Detailed humidity-control branch rows and humidistat moisture-demand
  calculation kept diagnostic proof only
- limited IdealLoads no-OA full-year Humidistat humidification annual facility
  meter conformance for `DistrictHeatingWater:Facility` and
  `DistrictCooling:Facility` in
  `ideal_loads_humidistat_humidification_annual_meter_conformance_candidate_001`,
  with Detailed humidity-control branch rows and humidistat moisture-demand
  calculation kept diagnostic proof only
- limited IdealLoads outdoor-air `Flow/Zone` conformance for declared
  outdoor-air mass/standard-density volume flow, no-humidity
  sensible/latent/total report rates, supply-air state, and mixed-air state
  Detailed series, with inactive economizer and inactive heat-recovery rows
  kept diagnostic-only
- limited IdealLoads outdoor-air `Flow/Person` conformance for declared
  outdoor-air mass/standard-density volume flow, no-humidity
  sensible/latent/total report rates, supply-air state, and mixed-air state
  Detailed series, using typed `People` design occupant count as a proof input
  while People heat-gain behavior remains outside the claim
- limited IdealLoads outdoor-air `Flow/Person` `OccupancySchedule` DCV
  conformance for declared outdoor-air mass/standard-density volume flow,
  no-humidity sensible/latent/total report rates, supply-air state, and
  mixed-air state Detailed series, using traced non-constant People occupancy
  schedule values to vary the minimum outdoor-air flow while People heat-gain
  behavior and broader DCV combinations remain outside the claim
- limited IdealLoads outdoor-air `Flow/Person` `CO2Setpoint` DCV conformance
  for declared outdoor-air mass/standard-density volume flow, no-humidity
  sensible/latent/total report rates, supply-air state, and mixed-air state
  Detailed series, using the EnergyPlus `Zone Air CO2 Predicted Load to
  Setpoint Mass Flow Rate` proof input to apply the source-order `max(minimum
  OA, CO2 demand)` adjustment while People heat-gain behavior, CO2
  contaminant-balance/concentration conformance, and broader DCV combinations
  remain outside the claim
- limited IdealLoads outdoor-air `Flow/Area` conformance for declared
  outdoor-air mass/standard-density volume flow, no-humidity
  sensible/latent/total report rates, supply-air state, and mixed-air state
  Detailed series, using typed zone floor area as a proof input
- limited IdealLoads outdoor-air `AirChanges/Hour` conformance for declared
  outdoor-air mass/standard-density volume flow, no-humidity
  sensible/latent/total report rates, supply-air state, and mixed-air state
  Detailed series, using typed zone volume as a proof input
- limited IdealLoads outdoor-air `Sum` conformance for declared outdoor-air
  mass/standard-density volume flow, no-humidity sensible/latent/total report
  rates, supply-air state, and mixed-air state Detailed series, using typed
  Flow/Area, Flow/Zone, and AirChanges/Hour component flows as proof inputs
- limited IdealLoads outdoor-air `Maximum` conformance for declared outdoor-air
  mass/standard-density volume flow, no-humidity sensible/latent/total report
  rates, supply-air state, and mixed-air state Detailed series, using typed
  component flows with AirChanges/Hour selected as the governing maximum
- limited IdealLoads outdoor-air `DifferentialDryBulb` economizer conformance
  for declared Flow/Zone outdoor-air mass/standard-density volume flow,
  no-humidity sensible/latent/total report rates, supply-air state,
  mixed-air state, and economizer active-time Detailed series, using the
  source dry-bulb comparison to reset outdoor-air flow above the low design
  minimum
- limited IdealLoads outdoor-air `DifferentialEnthalpy` economizer conformance
  for declared Flow/Zone outdoor-air mass/standard-density volume flow,
  no-humidity sensible/latent/total report rates, supply-air state,
  mixed-air state, and economizer active-time Detailed series, using the
  source enthalpy comparison to reset outdoor-air flow above the low design
  minimum
- limited IdealLoads outdoor-air `Sensible` heat-recovery conformance for
  declared Flow/Zone outdoor-air mass/standard-density volume flow,
  no-humidity sensible/latent/total report rates, supply-air state,
  mixed-air state, Sensible heat-recovery rate, and heat-recovery active-time
  Detailed series, with inactive economizer active-time kept diagnostic-only
- limited IdealLoads outdoor-air `Enthalpy` heat-recovery conformance for
  declared Flow/Zone outdoor-air mass/standard-density volume flow,
  no-humidity sensible/latent/total report rates, supply-air state,
  mixed-air state, Enthalpy heat-recovery rate, and heat-recovery active-time
  Detailed series, with inactive economizer active-time kept diagnostic-only
  and general heat-recovery saturation-limit branch parity outside the claim
- official dynamic heat-balance diagnostic reports that retain the broader
  surface/radiation/solar/convection decomposition and probe lanes without
  promoting those diagnostic variables
- oodocs/matplotlib release evidence documents
- schema v2 validation for all tracked case manifests

Not claimed:

- general EnergyPlus heat-balance compatibility
- general runtime compatibility
- broad HVAC compatibility
- plant compatibility
- broad node, full IdealLoads, meter, or broad weather conformance
- IdealLoads autosizing, AirLoop integration, PlantLoop integration, multiple
  equipment interaction, or fuel-efficiency schedules beyond the declared
  blank/constant/all-days Schedule:Compact candidates
- dynamic compatibility for the v0.23 static model case
- new numerical conformance from the v0.24 runtime-infrastructure milestone
- zone air temperature response to internal gains, radiant/latent internal
  gain coupling, or broader heat-balance compatibility from the v0.26
  internal-gain milestone
- broad official `1ZoneUncontrolled` dynamic heat-balance parity beyond the
  declared compatibility-candidate variables
- new numerical conformance from the v0.27 support coverage report
- new numerical conformance from the v0.31 release evidence asset manifest
- new numerical conformance from the v0.32 user coverage handbook
- broad ExampleFiles compatibility
