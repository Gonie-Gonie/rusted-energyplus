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
  output-variable scope: 82 tracked variables, 43 conformance variables, 33
  diagnostic variables, and 6 baseline variables
- source-map and algorithm ledger validation gate
- timestamp-aligned time/weather/schedule conformance report gate
- official ExampleFile static model EIO conformance report gate
- runtime output registry, IdealLoads facility meter request handles, meter
  registry diagnostics, ResultStore duplicate checks, and profile scaffolding
- opaque no-mass heat-balance adiabatic/interzone boundary handling
- internal convective gain conformance report gate for the declared
  `Zone Total Internal Convective Heating Rate` hourly series
- 51 declared numerical hourly/detailed series and 22 passed release-evidence
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
  energy and blank-efficiency fuel energy rows kept diagnostic-only and
  DistrictHeatingWater/DistrictCooling facility meters tracked only as
  oracle-MTR diagnostics with RuntimeMeterRegistry request resolution
- limited IdealLoads no-OA numeric capacity-limit conformance gate for the
  same declared thermostat, IdealLoads rate, and supply-node temperature/flow
  Detailed series in `ideal_loads_capacity_limit_conformance_001`, with
  return-node and humidity proof rows kept diagnostic-only
- diagnostic-only IdealLoads no-OA `ConstantSensibleHeatRatio`,
  `ConstantSupplyHumidityRatio` cooling/heating, and Humidistat
  dehumidification/humidification
  humidity-control evidence for zone/supply latent and sensible rate rows,
  moisture-demand proof inputs, and supply humidity handling in
  `ideal_loads_constant_shr_diagnostic_001`,
  `ideal_loads_constant_supply_humidity_diagnostic_001`,
  `ideal_loads_constant_supply_humidity_heating_diagnostic_001`,
  `ideal_loads_humidistat_dehumidification_diagnostic_001`, and
  `ideal_loads_humidistat_humidification_diagnostic_001`; broad
  humidity-control conformance remains outside the claim
- diagnostic-only IdealLoads Flow/Person, Flow/Zone, Flow/Area,
  AirChanges/Hour, Sum, and Maximum outdoor-air design-flow evidence for
  outdoor-air mass flow, standard-density volume flow, no-humidity
  outdoor-air report rates, supply-air state, mixed-air state, inactive
  economizer/heat-recovery reports, DifferentialDryBulb and
  DifferentialEnthalpy economizer active-time/flow parity, and Sensible and
  Enthalpy heat-recovery active-time/rate parity in
  `ideal_loads_outdoor_air_flow_person_diagnostic_001`,
  `ideal_loads_outdoor_air_design_flow_diagnostic_001`,
  `ideal_loads_outdoor_air_flow_area_diagnostic_001` and
  `ideal_loads_outdoor_air_air_changes_diagnostic_001`,
  `ideal_loads_outdoor_air_sum_diagnostic_001`, and
  `ideal_loads_outdoor_air_maximum_diagnostic_001`, plus
  `ideal_loads_outdoor_air_differential_dry_bulb_economizer_diagnostic_001`
  and
  `ideal_loads_outdoor_air_differential_enthalpy_economizer_diagnostic_001`,
  and
  `ideal_loads_outdoor_air_sensible_heat_recovery_diagnostic_001` and
  `ideal_loads_outdoor_air_enthalpy_heat_recovery_diagnostic_001`;
  active DCV, active humidity control, heat-recovery saturation-limit
  branches, and broad OA conformance remain outside the claim
- oodocs/matplotlib release evidence generation
- Case Manifest and Output Request Schema v2 validation
- tolerance-gated conformance only for declared v0.8/v0.9 no-mass cases,
  declared v0.22 `Schedule Value` / dry-bulb hourly variables, the v0.26
  internal convective gain hourly variable, and the official
  `1ZoneUncontrolled` dynamic compatibility-candidate variables, plus the
  declared no-OA/no-limit and numeric capacity-limit IdealLoads sensible
  variables
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
- broad node, full IdealLoads, meter, or full runtime conformance
- IdealLoads economizer, heat-recovery, DCV, humidity, saturation-limit, or
  meter conformance beyond oracle-MTR diagnostic tracking
- broad ExampleFiles compatibility

## Quick Start

```powershell
.\scripts\dev.cmd setup -InstallRust -InstallDocsTools
.\scripts\dev.cmd check
```

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
.\scripts\dev.cmd heat-balance-generalization-smoke
.\scripts\dev.cmd official-dynamic-heat-balance-diagnostic
.\scripts\dev.cmd conformance-index-report -Version 0.32.0
.\scripts\dev.cmd conformance-evidence-report -Version 0.32.0
.\scripts\dev.cmd support-coverage-report -Version 0.32.0
.\scripts\dev.cmd user-coverage-handbook -Version 0.32.0
.\scripts\dev.cmd release-evidence-manifest -Version 0.32.0
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
