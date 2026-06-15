<!-- DO NOT EDIT.
     Generated from specs/ and data/conformance_cases by tools/docs/generate_docs.py. -->

# Milestone Map

Milestones are maintained in `specs/milestones.toml`.

| Version | Title | Status | Claim level | Required cases | Not claimed |
|---|---|---|---|---|---|
| 0.1 | Model Intake | historical | model-intake-only |  | runtime conformance, heat-balance compatibility |
| 0.2 | Conformance Harness | historical | harness-only |  | numerical conformance |
| 0.3 | Input Interpretation Parity | historical | input-interpretation-only |  | runtime conformance |
| 0.4 | Time, Weather, and Schedule Evidence | historical | smoke-or-baseline | schedule_constant_001, weather_fields_001 | general runtime compatibility |
| 0.5 | Static Geometry, Construction, and Internal Gains | historical | static-smoke | surface_geometry_001, construction_materials_001, internal_gains_001 | dynamic heat-balance compatibility |
| 0.6 | Output, Trace, and Compare Infrastructure | historical | diagnostic-only | zone_temperature_diagnostic_001 | first executable building simulation subset, heat-balance conformance |
| 0.7 | EnergyPlus Source Mapping | historical | planning-guard |  | zone-temperature pass wording, numerical conformance |
| 0.8 | Uncontrolled Heat Balance Port | historical | limited-conformance | heat_balance_nomass_001 | dynamic exterior heat-balance claim, general heat-balance compatibility |
| 0.9 | Surface State Expansion | historical | limited-conformance | surface_temperature_nomass_001 | fenestration compatibility, solar-radiation compatibility |
| 0.10 | IdealLoads and Thermostat Typed Graph | historical | typed-graph-only | ideal_loads_thermostat_001 | IdealLoads load conformance, HVAC numerical conformance |
| 0.11 | Air-side Node Diagnostic | historical | diagnostic-only | air_side_node_diagnostic_001 | node numerical conformance, HVAC numerical conformance |
| 0.12 | Node Source Mapping | historical | planning-guard |  | node numerical conformance, HVAC numerical conformance |
| 0.13 | Plant Loop Skeleton | historical | typed-graph-smoke |  | plant numerical conformance, plant loop simulation |
| 0.14 | Plant Source Mapping | historical | planning-guard |  | plant numerical conformance |
| 0.15 | Plant Loop Diagnostic Baseline | historical | diagnostic-only | plant_loop_diagnostic_001 | plant numerical conformance, HVAC numerical conformance, node numerical conformance, meter conformance, sizing conformance, ExampleFiles numerical conformance |
| 0.16 | Versioning and Evidence Cleanup | complete | planning-documentation |  | new numerical conformance, plant compatibility, HVAC compatibility |
| 0.17 | Case Manifest and Output Request Schema v2 | complete | infrastructure-only | heat_balance_nomass_001, surface_temperature_nomass_001 | new numerical conformance, ExampleFiles compatibility, meter conformance |
| 0.18 | Output Request Injection and Oracle Baseline Pipeline | complete | baseline-only | official_1zone_uncontrolled_baseline_001 | new numerical conformance unless promoted by report and gate, ExampleFiles numerical conformance, general heat-balance compatibility, HVAC compatibility, plant compatibility |
| 0.19 | Series Reader and Compare Engine v2 | complete | comparison-infrastructure |  | new numerical conformance unless a case is explicitly promoted, meter conformance |
| 0.20 | Conformance Report Generator | complete | reporting-infrastructure |  | new numerical conformance unless backed by generated evidence |
| 0.21 | Source Map and Algorithm Ledger v1 | complete | planning-guard |  | algorithm completion without source map |
| 0.22 | Time, Weather, and Schedule Conformance Expansion | complete | declared-variables-only | schedule_constant_001, weather_fields_001 | general runtime compatibility |
| 0.23 | Static Model Evidence Expansion | complete | static-evidence | official_1zone_static_model_001 | dynamic heat-balance compatibility, HVAC compatibility, plant compatibility, meter conformance |
| 0.24 | Runtime State and Output Registry Hardening | complete | runtime-infrastructure |  | new numerical conformance, meter conformance, general runtime compatibility |
| 0.25 | Opaque No-Mass Heat Balance Generalization | complete | limited-conformance | heat_balance_nomass_001, surface_temperature_nomass_001 | general heat-balance compatibility, HVAC compatibility, plant compatibility |
| 0.26 | Internal Convective Gains Conformance | complete | declared-variables-only | internal_gains_001 | zone air temperature response to internal gains, radiant internal-gain coupling, latent or moisture coupling, HVAC compatibility, plant compatibility, meter conformance, general heat-balance compatibility |
| 0.27 | User Support Coverage Report | complete | reporting-infrastructure |  | new numerical conformance, full EnergyPlus compatibility, HVAC numerical conformance, plant numerical conformance, meter conformance |
| 0.28 | Input Object Coverage Metadata | complete | reporting-infrastructure |  | new numerical conformance, full EnergyPlus compatibility, HVAC numerical conformance, plant numerical conformance, meter conformance |
| 0.29 | Output Variable Coverage Metadata | complete | reporting-infrastructure |  | new numerical conformance, full EnergyPlus compatibility, HVAC numerical conformance, plant numerical conformance, meter conformance |
| 0.30 | Algorithm Coverage Metadata | complete | reporting-infrastructure |  | new numerical conformance, full EnergyPlus compatibility, HVAC numerical conformance, plant numerical conformance, meter conformance |
| 0.31 | Release Evidence Asset Manifest | complete | reporting-infrastructure |  | new numerical conformance, full EnergyPlus compatibility, HVAC numerical conformance, plant numerical conformance, meter conformance |
| 0.32 | User Coverage Handbook | complete | reporting-infrastructure |  | new numerical conformance, full EnergyPlus compatibility, HVAC numerical conformance, plant numerical conformance, meter conformance |
| 0.33 | Official Dynamic Heat-Balance Conformance | complete | limited-official-dynamic-conformance | official_1zone_uncontrolled_dynamic_diagnostic_001, official_1zone_uncontrolled_dynamic_conformance_candidate_001 | broad ExampleFiles dynamic numerical conformance, general warmup convergence parity outside the official 1ZoneUncontrolled dynamic candidate, diagnostic storage, radiation, solar, and convection coefficient variables, solar and exterior convection parity, full EnergyPlus compatibility |

## Long-Term Targets

| Version | Title | Claim level |
|---|---|---|
| 1.0 | Substantial Compatibility Draft | declared-subset |
| 2.0 | EnergyPlus 26.1 Full Compatibility | full-compatibility-mode-with-evidence |
| 3.0 | Fast Modernized Successor | mode-specific |
