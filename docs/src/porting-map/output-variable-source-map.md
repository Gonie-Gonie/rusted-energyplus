---
status: active
claim_level: planning-guard
owner: conformance
last_reviewed: 2026-06-08
---

# Output Variable Source Map

Reference version: EnergyPlus 26.1.0

Purpose: map the first heat-balance candidate output variables to EnergyPlus
source files and Rust result locations before promoting conformance claims.
`heat_balance_nomass_001` promotes the first zone variable in v0.8, and
`surface_temperature_nomass_001` promotes the first surface variables in v0.9.

## Candidate Variables

| Variable | Frequency | EnergyPlus source | Rust source or target | Current level |
|---|---|---|---|---|
| `Zone Mean Air Temperature` | hourly | `src/EnergyPlus/ZoneTempPredictorCorrector.cc` | `ResultStore` series from heat-balance trace | conformance for `heat_balance_nomass_001` and the official dynamic compatibility candidate |
| `Zone Total Internal Convective Heating Rate` | hourly | `src/EnergyPlus/InternalHeatGains.cc` | `simulate_zone_internal_convective_gains` | conformance for `internal_gains_001` only |
| `Zone Air Heat Balance Internal Convective Heat Gain Rate` | hourly | `src/EnergyPlus/DataHeatBalance.cc`; `src/EnergyPlus/ZoneTempPredictorCorrector.cc`; `src/EnergyPlus/InternalHeatGains.cc` | `ResultStore` series from `ZoneHeatBalanceState::convective_internal_gain_w` | conformance for the official dynamic compatibility candidate |
| `Zone Air Heat Balance Surface Convection Rate` | hourly | `src/EnergyPlus/DataHeatBalance.cc`; `src/EnergyPlus/ZoneTempPredictorCorrector.cc`; `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `ResultStore` series from `SumHA`, `SumHATsurf`, and `SumHATref` shell state | conformance for the official dynamic compatibility candidate |
| `Zone Air Heat Balance Air Energy Storage Rate` | hourly | `src/EnergyPlus/DataHeatBalance.cc`; `src/EnergyPlus/ZoneTempPredictorCorrector.cc` | `ResultStore` series from air heat capacity, MAT, and timestep delta | conformance for the official dynamic compatibility candidate |
| `Surface Inside Face Temperature` | hourly | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `ResultStore` series from heat-balance trace | conformance for `surface_temperature_nomass_001` and named official dynamic candidate surfaces |
| `Surface Outside Face Temperature` | hourly | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `ResultStore` series from heat-balance trace | conformance for `surface_temperature_nomass_001` and named official dynamic candidate surfaces |
| `Surface Inside Face Convection Heat Transfer Coefficient` | hourly | `src/EnergyPlus/HeatBalanceSurfaceManager.cc`; `src/EnergyPlus/ConvectionCoefficients.cc` | diagnostic `ResultStore` series from the EnergyPlus TARP inside convection helper stored on each `SurfaceHeatBalanceState` | diagnostic-only for official dynamic heat-balance case |
| `Surface Inside Face Convection Heat Gain Rate` | hourly | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` `SurfQdotConvInRep` | diagnostic `ResultStore` series from `HConvInt * area * (RefAirTemp - SurfTempIn)`, matching EnergyPlus positive-into-inside-face sign convention | diagnostic-only for official dynamic heat-balance case |
| `Surface Inside Face Net Surface Thermal Radiation Heat Gain Rate` | hourly | `src/EnergyPlus/HeatBalanceSurfaceManager.cc`; `src/EnergyPlus/HeatBalanceIntRadExchange.cc` | diagnostic `ResultStore` series from inside net longwave exchange multiplied by surface area | diagnostic-only for official dynamic heat-balance case |
| `Surface Inside Face Conduction Heat Transfer Rate` | hourly | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | steady `SurfaceHeatBalanceState` CTF inside flux shell | conformance for no-mass adiabatic `surface_temperature_nomass_001` and named official dynamic candidate surfaces |
| `Surface Inside Face Conduction Heat Transfer Rate per Area` | hourly | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | surface conduction rate divided by area | conformance for no-mass adiabatic `surface_temperature_nomass_001`; official ExampleFile diagnostic candidate |
| `Surface Outside Face Conduction Heat Transfer Rate` | hourly | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | opposite sign of scalar inside conduction shell | conformance for no-mass adiabatic `surface_temperature_nomass_001` and named official dynamic candidate surfaces |
| `Surface Outside Face Conduction Heat Transfer Rate per Area` | hourly | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | outside conduction rate divided by area | conformance for no-mass adiabatic `surface_temperature_nomass_001`; official ExampleFile diagnostic candidate |
| `Surface Outside Face Convection Heat Gain Rate` | hourly | `src/EnergyPlus/HeatBalanceSurfaceManager.cc::GetQdotConvOutPerArea` | diagnostic hourly-average `ResultStore` series from timestep outside face temperature and EnergyPlus-shaped dry/wet exterior convection terms; dry terms use timestep terrain-adjusted `SurfOutWindSpeed` and exposed rain timesteps use wet-bulb reference temperature | diagnostic-only for official dynamic heat-balance case |
| `Surface Outside Face Convection Heat Transfer Coefficient` | hourly | `src/EnergyPlus/HeatBalanceSurfaceManager.cc`; `src/EnergyPlus/ConvectionCoefficients.cc`; `src/EnergyPlus/WeatherManager.cc`; `src/EnergyPlus/DataSurfaces.cc::SetSurfaceWindSpeedAt` | diagnostic exterior coefficient helper; explicit `SurfaceConvectionAlgorithm:Outside,DOE-2` selects the DOE-2 dry coefficient path using timestep surface-local wind speed/direction, and exposed wet timesteps mix in EnergyPlus `SurfHConvExt = 1000.0` using hourly precipitation interpolation | diagnostic-only for official dynamic heat-balance case |
| `Surface Outside Face Net Thermal Radiation Heat Gain Rate` | hourly | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` outside radiation report variables; `src/EnergyPlus/ConvectionCoefficients.cc::InitExtConvCoeff` | diagnostic hourly-average exterior longwave source report from timestep outside face temperature and EnergyPlus-shaped `HSky`/`HAir`/`HGrd` linearized radiation components | diagnostic-only for official dynamic heat-balance case |
| `Surface Outside Face Incident Solar Radiation Rate per Area` | hourly | `src/EnergyPlus/SolarShading.cc`; `src/EnergyPlus/HeatBalanceSurfaceManager.cc`; `src/EnergyPlus/WeatherManager.cc` | diagnostic exterior incident solar total from beam, Perez anisotropic sky diffuse, and ground-reflected beam/diffuse components | diagnostic-only for official dynamic heat-balance case |
| `Surface Outside Face Incident Beam Solar Radiation Rate per Area` | hourly | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` `SurfQRadSWOutIncidentBeam`; `src/EnergyPlus/SolarShading.cc::FigureSolarBeamAtTimestep` | diagnostic exterior incident beam component from timestep direct normal radiation, shadowing-period cosine of incidence, and sunlit fraction assumption for the MinimalShadowing case | diagnostic-only for official dynamic heat-balance case |
| `Surface Outside Face Incident Sky Diffuse Solar Radiation Rate per Area` | hourly | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` `SurfQRadSWOutIncidentSkyDiffuse`; `src/EnergyPlus/SolarShading.cc::AnisoSkyViewFactors` | diagnostic exterior incident sky diffuse component from timestep diffuse horizontal radiation and Perez anisotropic sky multiplier | diagnostic-only for official dynamic heat-balance case |
| `Surface Outside Face Incident Ground Diffuse Solar Radiation Rate per Area` | hourly | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` ground diffuse incident report variables; `src/EnergyPlus/WeatherManager.cc` ground solar radiation | diagnostic exterior incident ground diffuse component from beam/diffuse ground-reflected terms and surface ground view factor | diagnostic-only for official dynamic heat-balance case |
| `Surface Outside Face Solar Radiation Heat Gain Rate` | hourly | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` outside absorbed shortwave report variables; `src/EnergyPlus/SolarShading.cc::AnisoSkyViewFactors`; `src/EnergyPlus/WeatherManager.cc` weather day indexing | diagnostic exterior solar source report from incident solar, Perez anisotropic sky diffuse, default ground reflection, non-leap weather ordinals for TMY records, and outside-layer solar absorptance | diagnostic-only for official dynamic heat-balance case |
| `Surface Heat Storage Rate` | hourly | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | diagnostic `ResultStore` series derived as `-(inside + outside)` from surface conduction rates | diagnostic-only for official dynamic heat-balance case |
| `Surface Heat Storage Rate per Area` | hourly | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | diagnostic `ResultStore` series derived from surface heat storage divided by surface area | diagnostic-only for official dynamic heat-balance case |
| `Zone Opaque Surface Inside Faces Conduction Rate` | hourly | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` advanced report variables | sum of Rust opaque surface heat gain to zone | conformance for no-mass adiabatic `surface_temperature_nomass_001`; official ExampleFile diagnostic candidate |
| `Zone Opaque Surface Outside Faces Conduction Rate` | hourly | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` advanced report variables | sum of Rust outside-face opaque surface conduction rates | diagnostic-only for official dynamic heat-balance case |
| `Zone Thermostat Heating Setpoint Temperature` | hourly | thermostat setup/output registration in EnergyPlus HVAC/zone predictor code | future thermostat result state | baseline-only for `ideal_loads_thermostat_001` |
| `Zone Thermostat Cooling Setpoint Temperature` | hourly | thermostat setup/output registration in EnergyPlus HVAC/zone predictor code | future thermostat result state | baseline-only for `ideal_loads_thermostat_001` |
| `Zone Ideal Loads Zone Total Heating Rate` | hourly | IdealLoads HVAC component implementation and zone equipment managers | future IdealLoads result state | baseline-only for `ideal_loads_thermostat_001`; mapped-not-ported |
| `Zone Ideal Loads Zone Total Cooling Rate` | hourly | IdealLoads HVAC component implementation and zone equipment managers | future IdealLoads result state | baseline-only for `ideal_loads_thermostat_001`; mapped-not-ported |
| `System Node Temperature` | hourly | `node-state-source-map.md`; `NodeInputManager.cc`; `ZoneTempPredictorCorrector.cc`; `PurchasedAirManager.cc`; `ZoneEquipmentManager.cc` | diagnostic `NodeStateStore` sampled by `simulate_ideal_loads_node_state_projection` | diagnostic-only for `air_side_node_diagnostic_001`; mapped-not-conformance |
| `System Node Humidity Ratio` | hourly | `node-state-source-map.md`; `NodeInputManager.cc`; `ZoneTempPredictorCorrector.cc`; `PurchasedAirManager.cc`; `ZoneEquipmentManager.cc` | diagnostic `NodeStateStore` sampled by `simulate_ideal_loads_node_state_projection` | diagnostic-only for `air_side_node_diagnostic_001`; mapped-not-conformance |
| `System Node Mass Flow Rate` | hourly | `node-state-source-map.md`; `NodeInputManager.cc`; `DataZoneEquipment.cc`; `PurchasedAirManager.cc` | diagnostic `NodeStateStore` sampled by `simulate_ideal_loads_node_state_projection` | diagnostic-only for `air_side_node_diagnostic_001`; mapped-not-conformance |
| `System Node Setpoint Temperature` | hourly | `node-state-source-map.md`; `NodeInputManager.cc`; `ZoneTempPredictorCorrector.cc` | future node-state result store | future-gated; sentinel handling required |
| `Site Outdoor Air Drybulb Temperature` | hourly | `src/EnergyPlus/WeatherManager.cc` | EPW weather trace | smoke |
| `Schedule Value` | hourly | output processor plus schedule managers | schedule trace | smoke |

## Registration Boundary

- EnergyPlus output variables are registered through `SetupOutputVariable`.
- Rust output variables must be declared in case manifests before comparison.
- Rust values must be written to `ResultStore` or a successor output store with
  key, variable, frequency, class, source, and timestamp semantics.
- A console-only value is not a v0.8 conformance variable.

## Promotion Requirements

A variable can move from `diagnostic-only` or `smoke` to `conformance` only
when all of these exist:

- case manifest with the requested output
- EnergyPlus baseline artifact containing the requested variable
- Rust result artifact for the same key, variable, and frequency
- timestamp alignment rule
- tolerance policy
- compare-summary row with first divergence information
- blocking release gate

## Explicit Non-Claims

The current `Zone Mean Air Temperature` diagnostic report has
`tolerance_policy: none` and `status: extracted`. It is useful for locating
deltas, but it is not a zone heat-balance conformance result.

The v0.8 `heat_balance_nomass_001` report is a separate conformance result for
hourly `Zone Mean Air Temperature` only. It requires a case manifest,
zone-state tolerance, markdown/JSON report artifacts, and a blocking gate.

The v0.9 `surface_temperature_nomass_001` report is a separate conformance
result for hourly `Zone Mean Air Temperature`, `Surface Inside Face
Temperature`, `Surface Outside Face Temperature`, and no-mass adiabatic
surface conduction series only. It requires zone-state and surface-state
tolerances, markdown/JSON report artifacts, mean/max/RMSE delta rows, and a
blocking gate.

The v0.33 `official_1zone_uncontrolled_dynamic_conformance_candidate_001`
report is a separate official ExampleFile dynamic conformance result for its
declared conformance-level weather, zone-air, surface temperature, and
inside/outside conduction variables. Diagnostic storage, radiation, solar,
convection coefficient, and broader probe rows remain outside that claim.

The official `1ZoneUncontrolled` baseline case now requests zone temperature,
zone air heat-balance, weather, internal gain, and surface conduction hourly
oracle series. The dynamic diagnostic case compares run-period-filtered zone,
roof/wall/floor face-temperature decomposition, surface/zone conduction, roof
exterior source rows, and zone air heat-balance deltas and records Rust/oracle
warmup day metadata. These are conformance candidates, but they remain
non-claiming until Rust produces matching hourly series under a blocking gate.

The v0.10 `ideal_loads_thermostat_001` report is baseline-only smoke evidence
for thermostat and IdealLoads output availability plus typed graph coverage.
It is not an IdealLoads load-conformance claim and keeps
`tolerance_policy: none`.

The v0.11 `air_side_node_diagnostic_001` report is diagnostic-only node output
evidence. The v0.12 source map identifies where EnergyPlus registers and
updates those node fields, and the Rust `NodeStateStore`-backed projection
adds diagnostic samples with `algorithm_parity: false`; it does not add
tolerances or node numerical conformance.
