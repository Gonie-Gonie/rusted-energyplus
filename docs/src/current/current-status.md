---
status: active
claim_level: none
owner: core
last_reviewed: 2026-06-15
---

# Current Status

The current public release line is v0.32.0. It adds a user coverage handbook on
top of the v0.31 release evidence asset manifest, v0.30 algorithm coverage
metadata, v0.29 output variable coverage metadata, v0.28 input object coverage
metadata, the v0.27 user-facing support coverage report, v0.26 internal convective gain
conformance, v0.25 opaque no-mass heat-balance boundary handling, v0.24
runtime registry infrastructure, v0.23 official ExampleFile static model
evidence, and v0.22 declared time, weather, and schedule numerical
conformance.

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
  supply-node variables only
- `ideal_loads_constant_supply_humidity_heating_conformance_candidate_001`
  declared no-OA `ConstantSupplyHumidityRatio` heating thermostat, rate, and
  supply-node variables only
- `ideal_loads_humidistat_dehumidification_conformance_candidate_001`
  declared no-OA Humidistat dehumidification thermostat, rate, and supply-node
  variables only
- `ideal_loads_humidistat_humidification_conformance_candidate_001` declared
  no-OA Humidistat humidification thermostat, rate, and supply-node variables
  only
- `ideal_loads_outdoor_air_flow_zone_conformance_candidate_001` declared
  outdoor-air Flow/Zone mass/volume, no-humidity report-rate, supply-air
  state, and mixed-air state variables only
- `ideal_loads_outdoor_air_flow_person_conformance_candidate_001` declared
  outdoor-air Flow/Person mass/volume, no-humidity report-rate, supply-air
  state, and mixed-air state variables only
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

## Current Evidence Boundary

| Area | Current conformance | Diagnostic or baseline evidence | Not claimed |
|---|---|---|---|
| Numerical time series | 25 promoted conformance manifests, 469 declared hourly/detailed ESO series, and 22 passed release-evidence series | `official_1zone_uncontrolled_baseline_001` keeps oracle series; `official_1zone_uncontrolled_dynamic_diagnostic_001` keeps broad run-period-filtered probe deltas; `official_1zone_uncontrolled_dynamic_conformance_candidate_001` gates the declared compatibility-candidate variable set; `ideal_loads_no_oa_sensible_conformance_001` gates the declared no-OA/no-limit IdealLoads sensible variable set; `ideal_loads_capacity_limit_conformance_001`, `ideal_loads_flow_limit_conformance_001`, and `ideal_loads_flow_capacity_limit_conformance_001` gate the declared no-OA finite-limit IdealLoads sensible variable set; `ideal_loads_constant_shr_conformance_001` gates the declared no-OA ConstantSensibleHeatRatio cooling variable set; `ideal_loads_constant_supply_humidity_cooling_conformance_candidate_001` and `ideal_loads_constant_supply_humidity_heating_conformance_candidate_001` gate the declared no-OA ConstantSupplyHumidityRatio cooling/heating variable sets; `ideal_loads_humidistat_dehumidification_conformance_candidate_001` and `ideal_loads_humidistat_humidification_conformance_candidate_001` gate the declared no-OA Humidistat humidity-control variable sets; `ideal_loads_outdoor_air_flow_zone_conformance_candidate_001`, `ideal_loads_outdoor_air_flow_person_conformance_candidate_001`, `ideal_loads_outdoor_air_flow_area_conformance_candidate_001`, `ideal_loads_outdoor_air_air_changes_conformance_candidate_001`, `ideal_loads_outdoor_air_sum_conformance_candidate_001`, `ideal_loads_outdoor_air_maximum_conformance_candidate_001`, `ideal_loads_outdoor_air_differential_dry_bulb_economizer_conformance_candidate_001`, `ideal_loads_outdoor_air_differential_enthalpy_economizer_conformance_candidate_001`, and `ideal_loads_outdoor_air_sensible_heat_recovery_conformance_candidate_001` gate the declared outdoor-air Flow/Zone, Flow/Person, Flow/Area, AirChanges/Hour, Sum, Maximum, Flow/Zone DifferentialDryBulb/DifferentialEnthalpy economizer, and Flow/Zone Sensible heat-recovery variable sets | broad ExampleFiles dynamic conformance |
| Static model | official `1ZoneUncontrolled` EIO surface geometry, Construction CTF, Material CTF Summary, and OtherEquipment nominal fields | generated support/index/release evidence PDFs | dynamic behavior from the static EIO case |
| Heat balance | no-mass zone MAT, no-mass surface inside/outside temperature, no-mass adiabatic conduction series, and selected official `1ZoneUncontrolled` dynamic weather/zone-air/surface-temperature/surface-conduction variables | official `1ZoneUncontrolled` broad diagnostic decomposition, floor storage blocker traces, radiation/solar/convection diagnostics, and non-promoted probe lanes | broad CTF storage parity, EnergyPlus warmup convergence parity outside the official candidate, solar, radiation exchange, fenestration, infiltration, zone air predictor/corrector parity, or general heat-balance compatibility |
| Time, weather, schedule | `Schedule Value` and `Site Outdoor Air Drybulb Temperature` hourly series | dewpoint, relative humidity, pressure, wind speed, and wind direction diagnostics | broad weather processor compatibility |
| Internal gains | `Zone Total Internal Convective Heating Rate` for `internal_gains_001` | static OtherEquipment nominal fields | zone air temperature response to gains, radiant/latent coupling, or broad internal-gain compatibility |
| HVAC, node, plant | no-OA/no-limit and numeric finite-limit IdealLoads sensible conformance plus no-OA ConstantSensibleHeatRatio cooling, ConstantSupplyHumidityRatio cooling/heating, Humidistat dehumidification/humidification, and outdoor-air Flow/Zone/Flow/Person/Flow/Area/AirChanges/Hour/Sum/Maximum/DifferentialDryBulb/DifferentialEnthalpy economizer and Sensible heat-recovery conformance for declared thermostat, IdealLoads rate, supply-node, outdoor-air, supply-air-state, mixed-air-state, economizer-active-time, Sensible heat-recovery rate, and Sensible heat-recovery active-time variables only | node proof rows, ReportPurchasedAir IdealLoads energy and blank/constant Schedule:Constant fuel energy proof rows, hourly IdealLoads facility meter oracle-MTR vs Rust aggregated fuel-energy diagnostics with RuntimeMeterRegistry request resolution, finite-limit return-node proof rows, ConstantSensibleHeatRatio return/zone-node humidity proof rows, ConstantSupplyHumidityRatio cooling/heating return/zone-node humidity and energy/fuel/meter proof rows, no-OA Humidistat dehumidification/humidification moisture-demand proof inputs and energy/fuel/meter proof rows, remaining IdealLoads Enthalpy heat-recovery outdoor-air mass/standard-density volume-flow, no-humidity report-rate, supply-air-state, mixed-air-state, inactive heat-recovery, and heat-recovery active-time/rate diagnostic evidence, and plant-loop baseline/diagnostic reports | broad HVAC, broad node, full IdealLoads, active DCV, broad or remaining humidity-control branches, outdoor-air methods beyond Flow/Zone/Flow/Person/Flow/Area/AirChanges/Hour/Sum/Maximum/DifferentialDryBulb/DifferentialEnthalpy economizer and declared Sensible heat recovery, Enthalpy heat recovery, humidistat moisture-demand conformance, saturation-limit branches, broad meter conformance, non-constant efficiency schedules, and plant numerical conformance |

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
  roof/wall/floor face-temperature, and conduction series in the
  compatibility-candidate lane, and keep floor storage as diagnostic-only
  blocker evidence
- limited IdealLoads no-OA/no-limit sensible conformance for declared
  thermostat setpoints, IdealLoads total/sensible/supply-air rates, and
  supply-node temperature/mass-flow Detailed series, with ReportPurchasedAir
  energy and blank/constant Schedule:Constant fuel energy rows kept
  diagnostic-only and hourly DistrictHeatingWater/DistrictCooling facility
  meters compared as oracle-MTR vs Rust aggregated fuel-energy diagnostics
  with RuntimeMeterRegistry request resolution, while broad meter conformance,
  humidity, predictor/corrector proof rows, outdoor-air, adaptive system
  timestep, sizing, non-constant efficiency schedules, and broad HVAC
  compatibility kept outside the claim
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
  conformance for declared thermostat setpoints, total/sensible/latent rate
  rows, supply-air report rows, and supply-node temperature/mass-flow/humidity
  Detailed series, with opposite-side rows, return-node/zone-air humidity
  rows, ReportPurchasedAir energy/fuel rows, and meters kept diagnostic-only
- limited IdealLoads no-OA Humidistat dehumidification/humidification
  conformance for declared thermostat setpoints, total/sensible/latent rate
  rows, supply-air report rows, and supply-node temperature/mass-flow/humidity
  Detailed series, with EnergyPlus moisture-demand rows, opposite-side rows,
  return-node/zone-air humidity rows, ReportPurchasedAir energy/fuel rows, and
  meters kept diagnostic-only
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
- diagnostic-only IdealLoads remaining Enthalpy heat-recovery outdoor-air
  evidence for outdoor-air mass flow, standard-density volume flow,
  no-humidity sensible/latent/total report rates, supply-air state,
  mixed-air state, inactive heat-recovery reports, and heat-recovery
  active-time/rate rows, with active DCV, active humidity controls,
  heat-recovery saturation-limit branches, meters, and broad OA compatibility
  outside the claim
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
