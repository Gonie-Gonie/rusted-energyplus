---
status: active
claim_level: none
owner: conformance
last_reviewed: 2026-06-07
---

# ExampleFiles Coverage Plan

EnergyPlus ExampleFiles and testfiles are the long-term source of real
compatibility evidence. They must be added gradually with explicit output
requests and report artifacts. Running an example successfully is not the same
as matching EnergyPlus.

Each selected case must answer:

- which EnergyPlus IDF was used
- which object families were parsed, typed, graphed, initialized, simulated,
  compared, or left raw-only
- which variables and meters were requested
- which frequency was compared
- which outputs are baseline-only, diagnostic-only, or conformance-level
- where the first divergence occurs
- which report and summary artifacts were produced

## Version Coverage Plan

| Milestone | Tier focus | Intended case families | Evidence focus |
|---|---|---|---|
| v0.1 | Tier A0 intake | `1ZoneUncontrolled.idf`; simple window variant | RawModel and TypedModel preview only |
| v0.2 | Tier A0 harness | one-zone uncontrolled and simple variants | baseline-only layout and report skeletons |
| v0.3 | Tier A0/A1 input interpretation | one-zone, three-surface, simple window, simple shading | object/default/reference coverage |
| v0.4 | Tier A0/A1 time series | one-zone and simple weather/schedule cases | weather, schedule, timestamp evidence |
| v0.5 | Tier A0/A1 static input | one-zone, window variants, simple shading; `5ZoneAirCooled` baseline-only | geometry, construction/material, nominal gains |
| v0.6 | Tier A/B report infrastructure | one-zone, window variants, simple shading, `5ZoneAirCooled` | automatic compare reports and summary JSON |
| v0.7 | Tier A source mapping | uncontrolled and no-HVAC heat-balance candidates | source/function and output-variable maps |
| v0.8 | Tier A heat balance | one-zone no-mass adiabatic candidate | first tolerance-gated `Zone Mean Air Temperature` subset |
| v0.9 | Tier A surface state | one-zone no-mass adiabatic surface candidate | first tolerance-gated surface inside/outside face temperature subset |
| v0.10 | Tier A/B IdealLoads | selected IdealLoads and thermostat cases | thermostat, equipment, and IdealLoads typed graph; baseline-only outputs |
| v0.11 | Tier B air-side HVAC | `air_side_node_diagnostic_001` first; `5ZoneAirCooled`, PTAC, selected CAV cases later | node and component diagnostics |
| v0.12 | Tier B air-side HVAC source mapping | v0.11 node fixture and EnergyPlus node source paths | node registration, update, and output source-function map |
| v0.13 | Tier B plant foundation | `plant-loop-skeleton.epJSON` typed graph smoke first; selected boiler, pump, chiller, and plant-loop ExampleFiles later | plant graph intake only; no plant numerical conformance |
| v0.14 | Tier B plant source mapping | v0.13 plant fixture and EnergyPlus plant source paths | plant source-function map only; no plant numerical conformance |
| v0.15 | Tier B plant diagnostic | `plant_loop_diagnostic_001` PlantLoadProfile baseline first; selected boiler, pump, chiller, and plant-loop ExampleFiles later | plant oracle baseline and report skeleton only; no plant numerical conformance |
| v0.16 | roadmap cleanup | legacy evidence reclassification and Road to v1.0 plan reset; plant projection remains an addendum | planning/documentation gate; no new numerical conformance |
| v0.17 | manifest schema | Case Manifest and Output Request Schema v2 | infrastructure only |
| v0.18 | output injection and oracle baseline | official IDF output request patching and selected oracle artifacts | complete; baseline-only |
| v0.19 | series reader and compare engine v2 | selected ESO series and future meter series | comparison infrastructure only |
| v1.0 | locked declared subset | promoted Tier A cases only | release conformance index |

## Required Case Structure

The repository currently uses `data/conformance_cases/<case_id>/case.toml`.
Future ExampleFiles cases should keep that convention and may add separate
request files when the schema supports them.

Current v0.5 static-input evidence cases:

- `surface_geometry_001`
- `construction_materials_001`
- `internal_gains_001`

Current v0.8 heat-balance conformance case:

- `heat_balance_nomass_001`

Current v0.9 surface-temperature conformance case:

- `surface_temperature_nomass_001`

Current v0.10 IdealLoads thermostat smoke case:

- `ideal_loads_thermostat_001`

Current v0.11 air-side node diagnostic case:

- `air_side_node_diagnostic_001`

Current v0.12 source-mapping evidence:

- `node-state-source-map.md`

Current v0.13 plant foundation evidence:

- `data/testcases/minimal/plant-loop-skeleton.epJSON`
- `plant-loop-skeleton-smoke`

Current v0.14 plant source-mapping evidence:

- `docs/src/porting-map/plant-source-map.md`
- `v0.14-verify`

Current v0.15 plant diagnostic evidence:

- `data/conformance_cases/plant_loop_diagnostic_001/case.toml`
- `data/conformance_cases/plant_loop_diagnostic_001/plant_loop_diagnostic.idf`
- `plant-loop-diagnostic-smoke`
- `v0.15-verify`

Current v0.18 official baseline evidence:

- `data/conformance_cases/official_1zone_uncontrolled_baseline_001/case.toml`
- manifest-owned staged IDF output request injection
- `official-baseline-smoke`
- `v0.18-verify`

Current v0.19 comparison infrastructure evidence:

- timestamp-aware selected ESO series parser
- index and timestamp alignment for selected numeric series
- max absolute delta, RMSE, max relative delta, status, and first divergence
  reason in `SeriesComparisonV2`
- `compare-series-v2-smoke`
- `v0.19-verify`

Post-v0.15 plant diagnostic addendum evidence:

- `run plant-state-projection`
- `plant-state-summary.md` and `plant-state-summary.json`
- `plant-loop-projection-smoke`
- `v0.16-verify` packaging and guard checks

Planned fields:

- source family and source file
- oracle version
- weather file
- patched IDF path
- feature flags such as surfaces, fenestration, HVAC, plant, EMS, plugins, and
  daylighting
- requested output frequencies by domain
- release gate and CI gate policy

## Pipeline

The intended pipeline is:

1. patch output requests into a copied IDF
2. run EnergyPlus oracle baseline
3. verify requested output availability from RDD/MDD/ESO/MTR/SQL artifacts
4. run Rust stages and write ResultStore, diagnostics, and trace artifacts
5. compare selected outputs and meters
6. write summary JSON and markdown report
7. update the release conformance index

Until steps 4 through 7 exist for a case, it must remain smoke,
baseline-only, or diagnostic-only.

## Immediate Backlog

- generate selected output CSV files from oracle artifacts
- generate a release-level conformance index
- add Tier A candidates for one-zone uncontrolled and simple window cases
- keep fenestration and solar outputs diagnostic-only until a separate declared case exists
- keep IdealLoads load outputs baseline-only until a solver declares tolerances
- keep `compare zone-temperature` diagnostic-only until v0.8
