# rusted-energyplus

Rust-only EnergyPlus-compatible porting project.

## Compatibility Contract

rusted-energyplus targets compatibility with the locked EnergyPlus 26.1.0
oracle. The Rust project does not replace EnergyPlus engineering algorithms;
optimization is limited to Rust data structures, execution planning, caching,
diagnostics, result storage, numerical implementation within declared
tolerance, and code organization.

## Current Public Scope

- pinned Rust toolchain
- repo-local EnergyPlus 26.1.0 oracle and reference source setup
- repo-local portable Python report environment
- epJSON RawModel inspection and TypedModel compile preview
- conformance manifests, output requests, tolerance policy, and report gates
- manifest-owned output request injection for staged oracle baselines
- timestamp-aware selected series reader and compare metrics v2
- release conformance index and coverage matrix report generation
- user-facing support coverage report generation for inputs, outputs, and
  algorithm scope
- support coverage metadata and manifests as the canonical current
  output-variable scope: 103 tracked variables, 84 conformance variables, 13
  diagnostic variables, and 6 baseline variables
- source-map and algorithm ledger validation gate
- timestamp-aligned time/weather/schedule conformance report gate
- official ExampleFile static model EIO conformance report gate
- runtime output registry, IdealLoads facility meter request handles, meter
  registry diagnostics, ResultStore duplicate checks, and profile scaffolding
- arbitrary IDF/epJSON run pipeline with support assessment artifacts, optional
  bundled EnergyPlus oracle baseline, and diagnostic oracle comparison reports;
  ad-hoc runs keep `conformance_claim=false`
- EPLaunch-style Windows launcher for choosing input, weather, output, and
  oracle compare without typing CLI commands
- opaque no-mass heat-balance adiabatic/interzone boundary handling
- internal convective gain conformance report gate for the declared
  `Zone Total Internal Convective Heating Rate` hourly series
- 52 declared numerical hourly/detailed series and 22 passed release-evidence
  series separated from broader declared conformance output requests in the
  user coverage handbook
- no-mass adiabatic surface conduction rate/per-area conformance for the
  declared `surface_temperature_nomass_001` hourly series
- official `1ZoneUncontrolled` dynamic heat-balance conformance gate for
  declared weather, zone-air, surface temperature, and surface conduction
  hourly series in the compatibility-candidate lane, with floor storage kept as
  diagnostic-only evidence
- official `1ZoneUncontrolled` dynamic heat-balance diagnostic report for
  broader run-period-filtered zone temperature, surface conduction, and
  diagnostic decomposition deltas; this broad probe remains explicitly
  `conformance_claim=false`
- limited IdealLoads no-OA/no-limit sensible conformance gate for declared
  thermostat setpoints, IdealLoads total/sensible/supply-air rates, and
  supply-node temperature/mass-flow Detailed series in
  `ideal_loads_no_oa_sensible_conformance_001`, with ReportPurchasedAir
  non-fuel energy rows promoted only in
  `ideal_loads_no_oa_report_energy_conformance_candidate_001`, blank
  fuel-efficiency rows promoted only in
  `ideal_loads_blank_fuel_efficiency_conformance_candidate_001`, constant
  Schedule:Constant fuel-efficiency rows promoted only in
  `ideal_loads_constant_fuel_efficiency_conformance_candidate_001`,
  all-days Schedule:Compact fuel-efficiency rows promoted only in
  `ideal_loads_non_constant_fuel_efficiency_conformance_candidate_001`,
  DistrictHeatingWater/DistrictCooling facility meters promoted only by the
  separate hourly meter candidate, the four humidity-control
  hourly/monthly/run-period meter candidates, and the
  monthly/annual/run-period meter-only candidate, plus the four full-year
  humidity-control annual meter candidates, while tracked as oracle-MTR
  diagnostics in this sensible case with RuntimeMeterRegistry request
  resolution
- limited IdealLoads no-OA ReportPurchasedAir energy conformance candidate for
  declared supply-air total heating/cooling energy and zone total
  heating/cooling energy rows in
  `ideal_loads_no_oa_report_energy_conformance_candidate_001`, with constant
  Schedule:Constant fuel-efficiency rows promoted only in
  `ideal_loads_constant_fuel_efficiency_conformance_candidate_001`, and
  blank fuel-efficiency rows promoted only in
  `ideal_loads_blank_fuel_efficiency_conformance_candidate_001`, and all-days
  Schedule:Compact fuel-efficiency rows promoted only in
  `ideal_loads_non_constant_fuel_efficiency_conformance_candidate_001`; raw rates,
  node, demand, humidity, and facility meter rows are kept diagnostic proof
  only
- limited IdealLoads no-OA blank fuel-efficiency conformance candidate for
  declared supply-air/zone heating/cooling fuel energy-rate and fuel energy
  rows in
  `ideal_loads_blank_fuel_efficiency_conformance_candidate_001`, with raw
  IdealLoads rate and facility meter rows kept diagnostic proof only;
  constant Schedule:Constant and all-days Schedule:Compact efficiency
  schedules remain outside this claim
- limited IdealLoads no-OA constant Schedule:Constant fuel-efficiency
  conformance candidate for declared supply-air/zone heating/cooling fuel
  energy-rate and fuel energy rows in
  `ideal_loads_constant_fuel_efficiency_conformance_candidate_001`, with raw
  IdealLoads rate and facility meter rows kept diagnostic proof only;
  blank and all-days Schedule:Compact efficiency schedules remain outside this
  claim
- limited IdealLoads no-OA all-days Schedule:Compact fuel-efficiency
  conformance candidate for declared supply-air/zone heating/cooling fuel
  energy-rate and fuel energy rows in
  `ideal_loads_non_constant_fuel_efficiency_conformance_candidate_001`, with
  raw IdealLoads rate and facility meter rows kept diagnostic proof only;
  blank and constant Schedule:Constant efficiency schedules remain outside
  this claim
- limited IdealLoads no-OA hourly facility meter conformance candidate for
  `DistrictHeatingWater:Facility` and `DistrictCooling:Facility` in
  `ideal_loads_no_oa_facility_meter_conformance_candidate_001`, with
  ReportPurchasedAir rate, energy, fuel-energy, thermostat, demand, humidity,
  and node rows kept diagnostic proof only
- limited IdealLoads no-OA monthly/annual/run-period facility meter conformance
  candidate for `DistrictHeatingWater:Facility` and
  `DistrictCooling:Facility` in
  `ideal_loads_no_oa_facility_meter_monthly_run_period_conformance_candidate_001`,
  with ReportPurchasedAir rate, energy, fuel-energy, thermostat, demand,
  humidity, and node rows kept diagnostic proof only
- limited IdealLoads no-OA numeric capacity-limit conformance gate for the
  same declared thermostat, IdealLoads rate, and supply-node temperature/flow
  Detailed series in `ideal_loads_capacity_limit_conformance_001`, with
  return-node and humidity proof rows kept diagnostic-only
- limited IdealLoads no-OA numeric flow-limit conformance gate for the same
  declared thermostat, IdealLoads rate, and supply-node temperature/flow
  Detailed series in `ideal_loads_flow_limit_conformance_001`, with
  return-node and humidity proof rows kept diagnostic-only
- limited IdealLoads no-OA numeric flow-and-capacity-limit conformance gate
  for the same declared thermostat, IdealLoads rate, and supply-node
  temperature/flow Detailed series in
  `ideal_loads_flow_capacity_limit_conformance_001`, with return-node and
  humidity proof rows kept diagnostic-only
- limited IdealLoads no-OA `ConstantSensibleHeatRatio` cooling conformance
  gate for declared thermostat, cooling total/sensible/latent rate, and
  supply-node temperature/flow/humidity Detailed series in
  `ideal_loads_constant_shr_conformance_001`, with return-node and zone-air
  humidity proof rows kept diagnostic-only
- limited IdealLoads no-OA `ConstantSupplyHumidityRatio` cooling conformance
  candidate gate for declared thermostat, heating/cooling total/sensible/latent
  rate, supply-air report-rate, ReportPurchasedAir energy/fuel,
  supply-node temperature/flow/humidity Detailed series, and
  hourly/monthly/run-period `DistrictHeatingWater:Facility`/
  `DistrictCooling:Facility` meters in
  `ideal_loads_constant_supply_humidity_cooling_conformance_candidate_001`,
  with return-node and zone-air humidity proof rows plus annual meter rows in
  this short-run candidate kept outside the claim
- limited IdealLoads no-OA full-year `ConstantSupplyHumidityRatio` cooling
  annual facility meter conformance candidate for
  `DistrictHeatingWater:Facility` and `DistrictCooling:Facility` in
  `ideal_loads_constant_supply_humidity_cooling_annual_meter_conformance_candidate_001`,
  with Detailed humidity-control branch rows kept diagnostic proof only
- limited IdealLoads no-OA `ConstantSupplyHumidityRatio` heating conformance
  candidate gate for declared thermostat, heating/cooling total/sensible/latent
  rate, supply-air report-rate, ReportPurchasedAir energy/fuel,
  supply-node temperature/flow/humidity Detailed series, and
  hourly/monthly/run-period `DistrictHeatingWater:Facility`/
  `DistrictCooling:Facility` meters in
  `ideal_loads_constant_supply_humidity_heating_conformance_candidate_001`,
  with return-node and zone-air humidity proof rows plus annual meter rows in
  this short-run candidate kept outside the claim
- limited IdealLoads no-OA full-year `ConstantSupplyHumidityRatio` heating
  annual facility meter conformance candidate for
  `DistrictHeatingWater:Facility` and `DistrictCooling:Facility` in
  `ideal_loads_constant_supply_humidity_heating_annual_meter_conformance_candidate_001`,
  with Detailed humidity-control branch rows kept diagnostic proof only
- limited IdealLoads no-OA Humidistat dehumidification conformance candidate
  gate for declared thermostat, heating/cooling total/sensible/latent rate,
  supply-air report-rate, ReportPurchasedAir energy/fuel, paired trace-driven
  moisture-demand rows, supply-node temperature/flow/humidity Detailed series,
  and hourly/monthly/run-period
  `DistrictHeatingWater:Facility`/`DistrictCooling:Facility` meters in
  `ideal_loads_humidistat_dehumidification_conformance_candidate_001`; fully
  owned moisture-history closure, annual meter rows in this short-run
  candidate, and broad humidity-control conformance remain outside the claim
- limited IdealLoads no-OA Humidistat humidification conformance candidate
  gate for declared thermostat, heating/cooling total/sensible/latent rate,
  supply-air report-rate, ReportPurchasedAir energy/fuel, paired trace-driven
  moisture-demand rows, supply-node temperature/flow/humidity Detailed series,
  and hourly/monthly/run-period
  `DistrictHeatingWater:Facility`/`DistrictCooling:Facility` meters in
  `ideal_loads_humidistat_humidification_conformance_candidate_001`; fully
  owned moisture-history closure and annual meter rows in this short-run
  candidate remain outside the claim
- limited IdealLoads no-OA full-year Humidistat dehumidification annual facility
  meter conformance candidate for `DistrictHeatingWater:Facility` and
  `DistrictCooling:Facility` in
  `ideal_loads_humidistat_dehumidification_annual_meter_conformance_candidate_001`,
  with Detailed humidity-control branch rows and humidistat moisture-demand
  calculation kept diagnostic proof only
- limited IdealLoads no-OA full-year Humidistat humidification annual facility
  meter conformance candidate for `DistrictHeatingWater:Facility` and
  `DistrictCooling:Facility` in
  `ideal_loads_humidistat_humidification_annual_meter_conformance_candidate_001`,
  with Detailed humidity-control branch rows and humidistat moisture-demand
  calculation kept diagnostic proof only
- limited IdealLoads outdoor-air `Flow/Zone` conformance candidate gate for
  declared outdoor-air mass/standard-density volume flow, no-humidity
  outdoor-air report rates, supply-air state, and mixed-air state Detailed
  series in `ideal_loads_outdoor_air_flow_zone_conformance_candidate_001`,
  with inactive economizer and inactive heat-recovery rows kept diagnostic-only
- limited IdealLoads outdoor-air `Flow/Person` conformance candidate gate for
  declared outdoor-air mass/standard-density volume flow, no-humidity
  outdoor-air report rates, supply-air state, and mixed-air state Detailed
  series in `ideal_loads_outdoor_air_flow_person_conformance_candidate_001`,
  using traced five-person design-occupant proof input; People heat-gain
  conformance is not claimed
- limited IdealLoads outdoor-air `OccupancySchedule` DCV conformance candidate
  gate for declared Flow/Person outdoor-air mass/standard-density volume flow,
  no-humidity outdoor-air report rates, supply-air state, and mixed-air state
  Detailed series in
  `ideal_loads_outdoor_air_occupancy_dcv_conformance_candidate_001`, using
  traced non-constant People occupancy schedule values; broader DCV
  combinations remain outside the claim
- limited IdealLoads outdoor-air `CO2Setpoint` DCV conformance candidate gate
  for declared Flow/Person outdoor-air mass/standard-density volume flow,
  no-humidity outdoor-air report rates, supply-air state, and mixed-air state
  Detailed series in
  `ideal_loads_outdoor_air_co2_dcv_conformance_candidate_001`, using the
  EnergyPlus `Zone Air CO2 Predicted Load to Setpoint Mass Flow Rate` proof
  input to apply the source-order `max(minimum OA, CO2 demand)` adjustment;
  CO2 contaminant-balance/concentration conformance, People heat-gain
  conformance, and broader DCV combinations remain outside the claim
- limited IdealLoads outdoor-air `Flow/Area` conformance candidate gate for
  declared outdoor-air mass/standard-density volume flow, no-humidity
  outdoor-air report rates, supply-air state, and mixed-air state Detailed
  series in `ideal_loads_outdoor_air_flow_area_conformance_candidate_001`,
  using traced 1 m2 zone-floor-area proof input
- limited IdealLoads outdoor-air `AirChanges/Hour` conformance candidate gate
  for declared outdoor-air mass/standard-density volume flow, no-humidity
  outdoor-air report rates, supply-air state, and mixed-air state Detailed
  series in `ideal_loads_outdoor_air_air_changes_conformance_candidate_001`,
  using traced 1 m3 zone-volume proof input
- limited IdealLoads outdoor-air `Sum` conformance candidate gate for declared
  outdoor-air mass/standard-density volume flow, no-humidity outdoor-air
  report rates, supply-air state, and mixed-air state Detailed series in
  `ideal_loads_outdoor_air_sum_conformance_candidate_001`, with traced
  Flow/Area, Flow/Zone, and AirChanges/Hour component-flow proof inputs
- limited IdealLoads outdoor-air `Maximum` conformance candidate gate for
  declared outdoor-air mass/standard-density volume flow, no-humidity
  outdoor-air report rates, supply-air state, and mixed-air state Detailed
  series in `ideal_loads_outdoor_air_maximum_conformance_candidate_001`, with
  traced component-flow proof inputs and AirChanges/Hour selected as the
  governing maximum
- limited IdealLoads outdoor-air `DifferentialDryBulb` economizer conformance
  candidate gate for declared Flow/Zone outdoor-air mass/standard-density
  volume flow, no-humidity outdoor-air report rates, supply-air state,
  mixed-air state, and economizer active-time Detailed series in
  `ideal_loads_outdoor_air_differential_dry_bulb_economizer_conformance_candidate_001`,
  using a low minimum outdoor-air flow so the source dry-bulb comparison resets
  the cooling outdoor-air flow above the design minimum
- limited IdealLoads outdoor-air `DifferentialEnthalpy` economizer conformance
  candidate gate for declared Flow/Zone outdoor-air mass/standard-density
  volume flow, no-humidity outdoor-air report rates, supply-air state,
  mixed-air state, and economizer active-time Detailed series in
  `ideal_loads_outdoor_air_differential_enthalpy_economizer_conformance_candidate_001`,
  using a low minimum outdoor-air flow so the source enthalpy comparison resets
  the cooling outdoor-air flow above the design minimum
- limited IdealLoads outdoor-air `Sensible` heat-recovery conformance
  candidate gate for declared Flow/Zone outdoor-air mass/standard-density
  volume flow, no-humidity outdoor-air report rates, supply-air state,
  mixed-air state, Sensible heat-recovery rate, and heat-recovery active-time
  Detailed series in
  `ideal_loads_outdoor_air_sensible_heat_recovery_conformance_candidate_001`,
  with inactive economizer active-time kept diagnostic-only
- limited IdealLoads outdoor-air `Enthalpy` heat-recovery conformance
  candidate gate for declared Flow/Zone outdoor-air mass/standard-density
  volume flow, no-humidity outdoor-air report rates, supply-air state,
  mixed-air state, Enthalpy heat-recovery rate, and heat-recovery active-time
  Detailed series in
  `ideal_loads_outdoor_air_enthalpy_heat_recovery_conformance_candidate_001`,
  with inactive economizer active-time kept diagnostic-only and the fixture's
  single saturation-limit timestep covered only by declared tolerances
- diagnostic-only IdealLoads remaining outdoor-air predecessor evidence in
  `ideal_loads_outdoor_air_flow_person_diagnostic_001`,
  `ideal_loads_outdoor_air_design_flow_diagnostic_001`,
  `ideal_loads_outdoor_air_flow_area_diagnostic_001`,
  `ideal_loads_outdoor_air_air_changes_diagnostic_001`,
  `ideal_loads_outdoor_air_sum_diagnostic_001`, and
  `ideal_loads_outdoor_air_maximum_diagnostic_001`, plus
  `ideal_loads_outdoor_air_differential_dry_bulb_economizer_diagnostic_001`
  and
  `ideal_loads_outdoor_air_differential_enthalpy_economizer_diagnostic_001`,
  and
  `ideal_loads_outdoor_air_sensible_heat_recovery_diagnostic_001` and
  `ideal_loads_outdoor_air_enthalpy_heat_recovery_diagnostic_001`;
  broader DCV combinations, CO2 contaminant-balance/concentration
  conformance, active humidity control, heat-recovery saturation-limit branch
  generality, and broad OA conformance remain outside the claim
- oodocs/matplotlib release evidence generation
- Case Manifest and Output Request Schema v2 validation
- tolerance-gated conformance only for declared v0.8/v0.9 no-mass cases,
  declared v0.22 `Schedule Value` / dry-bulb hourly variables, the v0.26
  internal convective gain hourly variable, and the official
  `1ZoneUncontrolled` dynamic compatibility-candidate variables, plus the
  declared no-OA/no-limit, numeric finite-limit, and
  ReportPurchasedAir non-fuel energy, blank fuel-efficiency, constant
  Schedule:Constant fuel-efficiency, all-days Schedule:Compact
  fuel-efficiency, no-OA hourly and monthly/annual/run-period facility meter,
  `ConstantSensibleHeatRatio`, `ConstantSupplyHumidityRatio`,
  Humidistat, and outdoor-air `Flow/Zone`, `Flow/Person`,
  `OccupancySchedule` DCV, `Flow/Area`, `AirChanges/Hour`, `Sum`, `Maximum`,
  `DifferentialDryBulb` economizer, `DifferentialEnthalpy` economizer, and
  `Sensible`/`Enthalpy` heat-recovery IdealLoads variables
- static EIO model conformance only for the declared v0.23 official
  `1ZoneUncontrolled` surface, construction/material, and OtherEquipment
  nominal fields
- v0.24 runtime registry hardening only as infrastructure; no new numerical
  conformance
- v0.25 opaque no-mass heat-balance generalization only for declared existing
  cases and variables
- v0.26 internal convective gain conformance only for `internal_gains_001` /
  `Zone Total Internal Convective Heating Rate`
- current surface conduction conformance covers the no-mass adiabatic
  `surface_temperature_nomass_001` series and named official
  `1ZoneUncontrolled` dynamic candidate surfaces; storage/radiation/solar
  diagnostic variables are not promoted
- v0.27 support coverage report only as release documentation infrastructure;
  it does not promote new numerical conformance
- v0.28 input object coverage metadata only as user documentation
  infrastructure; it does not promote new numerical conformance
- v0.29 output variable coverage metadata only as user documentation
  infrastructure; it does not promote new numerical conformance
- v0.30 algorithm coverage metadata only as user documentation
  infrastructure; it does not promote new numerical conformance
- v0.31 release evidence asset manifest only as release documentation
  infrastructure; it does not promote new numerical conformance
- v0.32 user coverage handbook only as user documentation infrastructure; it
  does not promote new numerical conformance

Not claimed:

- general EnergyPlus heat-balance compatibility
- broad HVAC or plant simulation compatibility
- broad node, full IdealLoads, broad meter conformance, or full runtime
  conformance
- IdealLoads autosizing, AirLoop integration, PlantLoop integration, multiple
  equipment interaction, or fuel-efficiency schedules beyond the declared
  blank/constant/all-days Schedule:Compact candidates
- IdealLoads economizer conformance beyond the declared DifferentialDryBulb
  and DifferentialEnthalpy candidates, heat-recovery conformance beyond the
  declared Sensible and Enthalpy candidates, broader DCV combinations beyond
  the declared OccupancySchedule and CO2Setpoint Flow/Person candidates,
  humidity, CO2 contaminant-balance/concentration conformance,
  saturation-limit generality, or broad meter conformance beyond the declared
  no-OA hourly and monthly/annual/run-period facility meter candidates,
  including multi-year annual grouping
- broad ExampleFiles compatibility

## Quick Start

```powershell
.\scripts\dev.cmd setup -InstallRust -InstallDocsTools
.\scripts\dev.cmd check
```

Release packages include `eplus-rs-launch.exe` for the small EPLaunch-style UI.
From a checkout, use `.\scripts\dev.cmd launch-ui` or build a local launcher
with `.\scripts\dev.cmd build-launcher-exe`.

Useful focused checks:

```powershell
.\scripts\dev.cmd docs-generate
.\scripts\dev.cmd docs-check
.\scripts\dev.cmd manifest-validate-all
.\scripts\dev.cmd strict-no-false-conformance
.\scripts\dev.cmd official-baseline-smoke
.\scripts\dev.cmd compare-series-v2-smoke
.\scripts\dev.cmd algorithm-ledger-check
.\scripts\dev.cmd compare-schedule-conformance
.\scripts\dev.cmd compare-weather-conformance
.\scripts\dev.cmd compare-static-model-conformance
.\scripts\dev.cmd compare-internal-convective-gain-conformance
.\scripts\dev.cmd runtime-registry-smoke
.\scripts\dev.cmd arbitrary-run-smoke
.\scripts\dev.cmd heat-balance-generalization-smoke
.\scripts\dev.cmd official-dynamic-heat-balance-diagnostic
.\scripts\dev.cmd conformance-index-report -Version 0.1.0
.\scripts\dev.cmd conformance-evidence-report -Version 0.1.0
.\scripts\dev.cmd support-coverage-report -Version 0.1.0
.\scripts\dev.cmd user-coverage-handbook -Version 0.1.0
.\scripts\dev.cmd release-evidence-manifest -Version 0.1.0
```

## Documentation

Start with the current docs:

- `docs/src/current/project-contract.md`
- `docs/src/current/current-status.md`
- `docs/src/current/roadmap.md`
- `docs/src/current/verification.md`
- `docs/src/current/architecture-overview.md`

Old planning docs are not retained in the mdBook tree. Use Git history,
release notes, and GitHub Release assets for historical planning and frozen
evidence.

Build the book with:

```powershell
.\scripts\dev.cmd docs-check
```
