# Changelog

## Unreleased

No unreleased changes.

## v0.29.0 - 2026-06-08

Output variable coverage metadata.

### Added

- v0.29 output coverage metadata milestone.
- `support_boundary` metadata for tracked variable coverage rows.
- generated mdBook variable coverage columns for first evidence and support
  boundary.
- support coverage report output tables with observed levels, best evidence
  cases, and support boundaries.
- support coverage JSON aggregates for output first-evidence and boundary
  completeness.
- v0.29 release verification script.

### Changed

- Output variable first evidence now resolves from the strongest available
  evidence level instead of the earliest manifest appearance.
- Release evidence report defaults now target v0.29.0.

### Scope

- v0.29.0 is reporting metadata work and does not add new numerical
  conformance, full EnergyPlus compatibility, HVAC numerical conformance,
  plant numerical conformance, or meter conformance.

## v0.28.0 - 2026-06-08

Input object coverage metadata.

### Added

- v0.28 input coverage metadata milestone.
- `first_evidence` and `support_boundary` fields for all tracked input
  objects.
- generated mdBook object coverage columns for first evidence and support
  boundary.
- support coverage report input tables that expose first evidence and
  boundaries in PDF/HTML/JSON/Markdown.
- v0.28 release verification script.

### Changed

- Release evidence report defaults now target v0.28.0.
- Support coverage JSON now reports how many input objects have first evidence.

### Scope

- v0.28.0 is reporting metadata work and does not add new numerical
  conformance, full EnergyPlus compatibility, HVAC numerical conformance,
  plant numerical conformance, or meter conformance.

## v0.27.0 - 2026-06-08

User support coverage reporting.

### Added

- v0.27 user support coverage report milestone.
- `support-coverage-report` release command.
- `tools/reporting/support_coverage_report.py` oodocs/matplotlib generator.
- PDF/HTML/JSON/Markdown support coverage artifacts for tracked inputs,
  outputs, algorithms, cases, and explicit gaps.
- v0.27 release verification script.

### Changed

- Release evidence report defaults now target v0.27.0.
- `check` now generates the user support coverage report.
- mdBook navigation exposes the Support Coverage Report page.

### Scope

- v0.27.0 is reporting infrastructure and does not add new numerical
  conformance, full EnergyPlus compatibility, HVAC numerical conformance,
  plant numerical conformance, or meter conformance.

## v0.26.0 - 2026-06-07

Internal convective gains conformance.

### Added

- v0.26 internal convective gains conformance milestone.
- `compare-internal-convective-gain-conformance` gate and
  `conformance internal-gains-report` CLI path.
- Timestamp-aligned `compare-report.md` and `compare-summary.json` artifacts
  for `internal_gains_001`.
- v0.26 release verification script.

### Changed

- Promoted only `Zone Total Internal Convective Heating Rate` in
  `internal_gains_001` to conformance level.
- Split the internal convective gain CLI comparison into
  `crates/ep_cli/src/internal_gains.rs` to reduce `main.rs` growth.
- Release evidence report defaults now target v0.26.0 and include 5 promoted
  numerical cases / 7 promoted series.

### Scope

- v0.26.0 does not claim zone air temperature response to internal gains,
  radiant/latent internal-gain coupling, HVAC compatibility, plant
  compatibility, meter conformance, or general heat-balance compatibility.

## v0.25.0 - 2026-06-07

Opaque no-mass heat-balance generalization.

### Added

- Adiabatic opaque surface boundary handling that keeps no-mass heat gain at
  zero when the boundary follows the owning zone air temperature.
- Interzone opaque surface boundary target resolution for `Surface`, `Zone`,
  and `Space` boundary objects.
- Explicit missing interzone surface/zone boundary target errors.
- Runtime tests for adiabatic, interzone, and missing boundary target behavior.
- `heat-balance-generalization-smoke` gate.
- v0.25 release verification script.

### Changed

- Heat-balance surface state now stores outside boundary object names and
  resolved adjacent surface/zone IDs.
- Existing v0.8/v0.9 heat-balance conformance gates remain blocking and are
  included in the v0.25 smoke path.
- Default release evidence report version is v0.25.0.

### Boundaries

- v0.25.0 remains limited-conformance for declared no-mass heat-balance cases
  and variables only.
- No general heat-balance, HVAC, plant, meter, fenestration, solar, sizing,
  warmup, or broad ExampleFiles compatibility is claimed.

## v0.24.0 - 2026-06-07

Runtime state and output registry hardening.

### Added

- `ep_runtime::RuntimeOutputRegistry` for model-derived output handles.
- `ep_runtime::RuntimeMeterRegistry` with explicit unavailable-meter diagnostics.
- Runtime diagnostic codes for unavailable outputs/meters and duplicate output
  handles or series.
- `ResultStoreProfile` and duplicate-handle/duplicate-series diagnostics.
- `runtime-registry-smoke` gate.
- v0.24 release verification script.

### Changed

- `build_execution_plan` now writes registry-derived `WriteOutput` handles
  instead of a placeholder `OutputHandle(0)`.
- `ep_runtime::ResultStore` and output primitives are split into
  `crates/ep_runtime/src/output.rs`.
- `model-plan-smoke` now checks registry-backed output steps.
- Default release evidence report version is v0.24.0.

### Boundaries

- v0.24.0 is runtime-infrastructure only.
- No new numerical conformance, meter conformance, or general runtime
  compatibility is claimed.

## v0.23.0 - 2026-06-07

Static model evidence expansion.

### Added

- `conformance static-model-report` command.
- Blocking `compare-static-model-conformance` gate.
- Official ExampleFile case `official_1zone_static_model_001`.
- Static EIO report artifacts for declared surface, construction/material, and
  OtherEquipment nominal fields.
- Surface detail EIO staging via `Output:Surfaces:List,Details` injection.

### Changed

- The conformance index now includes the v0.23 static model evidence case.
- Default release evidence report version is v0.23.0.

### Boundaries

- v0.23.0 is static-evidence only.
- The numeric conformance evidence PDF remains limited to promoted numerical
  v0.8/v0.9/v0.22 cases.
- No dynamic heat-balance, HVAC, plant, meter, solar, fenestration, sizing,
  warmup, or broad ExampleFiles compatibility is claimed.

## v0.22.0 - 2026-06-07

Time, weather, and schedule conformance expansion.

### Added

- Timestamp-aligned `conformance time-weather-schedule-report` command.
- Blocking `compare-schedule-conformance` and `compare-weather-conformance`
  gates.
- v0.22 release evidence coverage for `Schedule Value` and
  `Site Outdoor Air Drybulb Temperature`.

### Changed

- `schedule_constant_001` and `weather_fields_001` are promoted to
  conformance cases for declared variables only.
- Numeric conformance evidence PDF/HTML/JSON now includes the v0.22 promoted
  schedule and dry-bulb series.
- `ep_cli` time/weather/schedule report logic is split into
  `time_weather_schedule.rs`.

### Boundaries

- v0.22.0 does not claim general runtime compatibility.
- Weather variables other than dry-bulb remain diagnostic rows in the weather
  report.

## v0.21.0 - 2026-06-07

Source map and algorithm ledger release.

### Added

- `source_map` links in `specs/algorithm_ledger.toml`.
- generated algorithm ledger source-map column.
- `algorithm-ledger-check` gate for EnergyPlus source files, source-map docs,
  Rust target anchors, first-case manifests, proof variables, and
  claim-appropriate blocking gates.
- `v0.21-verify` release gate.
- `specs/` packaging in the release zip.

### Notes

- v0.21.0 is a planning-guard release, not a new numerical conformance
  release.
- Algorithm entries with non-`none` claim levels must be backed by conformance
  cases and blocking gates.
- Tolerance-gated conformance remains limited to the declared v0.8 and v0.9
  no-mass cases.

## v0.20.0 - 2026-06-07

Conformance report generator release.

### Added

- Python `oodocs` conformance index report generator.
- release-level case, output, meter, domain, report, and gate coverage
  matrices.
- `conformance-index.md`, `conformance-index-report.html`,
  `conformance-index-report.pdf`, and `conformance-index-report.json`
  artifacts under `.runtime/release-evidence/vX.Y.Z`.
- `conformance-index-report` and `v0.20-verify` gates.

### Notes

- v0.20.0 is reporting infrastructure work, not a new numerical conformance
  release.
- The conformance index maps coverage and report/gate contracts; it does not
  promote cases by itself.
- Tolerance-gated conformance remains limited to the declared v0.8 and v0.9
  no-mass cases.

## v0.19.0 - 2026-06-07

Series reader and compare engine v2 release.

### Added

- `SeriesComparisonV2` with index or timestamp alignment, compared sample
  counts, max absolute delta, RMSE, max relative delta, status, and first
  divergence reason.
- `SeriesSample` and timestamp-aware comparison helpers for selected output
  series.
- timestamp-aware ESO selected-series parsing with dictionary metadata,
  hourly timestamp labels, units, and frequency extraction.
- `compare-series-v2-smoke` and `v0.19-verify` gates.

### Notes

- v0.19.0 is comparison infrastructure work, not a new numerical conformance
  release.
- Meter conformance remains explicitly out of scope for this milestone.
- Tolerance-gated conformance remains limited to the declared v0.8 and v0.9
  no-mass cases.

## v0.18.0 - 2026-06-07

Output request injection and official oracle baseline pipeline release.

### Added

- manifest-owned `Output:Variable` and `Output:Meter` injection when staging
  IDFs for oracle baselines.
- idempotent output-request staging that skips requests already present in the
  source IDF.
- expanded baseline manifests with `rusted-energyplus.output-injection.v1`
  metadata and injected output/meter counts.
- official `1ZoneUncontrolled.idf` baseline-only case seeded from the
  repo-local EnergyPlus 26.1.0 ExampleFiles tree.
- `official-baseline-smoke` and `v0.18-verify` gates.

### Notes

- v0.18.0 is baseline infrastructure work, not a new numerical conformance
  release.
- Official ExampleFiles execution is baseline-only until Rust artifacts,
  tolerances, compare reports, and blocking gates promote a specific case.
- Tolerance-gated conformance remains limited to the declared v0.8 and v0.9
  no-mass cases.

## v0.17.0 - 2026-06-07

Case Manifest and Output Request Schema v2 release.

### Added

- `manifest_v2` metadata and `[scope]` feature/domain metadata for tracked
  case manifests.
- output request v2 fields: `domain`, `level`, and optional per-output
  tolerances.
- meter request and waiver schema support in `ep_conformance`.
- `eplus-rs conformance validate-case-v2 <case.toml>`.
- `manifest-validate-all` dev gate for every tracked case manifest.
- v0.17 plan, readiness, release notes, verification gate, and package checks.

### Notes

- v0.17.0 is schema/infrastructure work, not a new numerical conformance
  release.
- `level = "conformance"` remains limited to the declared v0.8 and v0.9
  no-mass cases.
- Baseline-only and diagnostic-only cases continue to force
  `conformance_claim = false`.

## v0.16.0 - 2026-06-07

Versioning and evidence cleanup release.

### Added

- `versioning-reset-v2.md` canonical roadmap reset.
- `legacy-milestones.md` classification of v0.1 through v0.15 as the
  Historical Pre-Alpha Evidence Series.
- rewritten milestone map around Road to v1.0 and v1/v2/v3 target boundaries.
- v1, v2, and v3 scope documents.
- v0.17 and v0.18 plan seeds for manifest v2, output request schema, output
  injection, and oracle baseline pipeline.
- `run plant-state-projection` diagnostic command for the v0.15
  PlantLoadProfile fixture as an additional addendum.
- `plant-loop-projection-smoke` gate for the diagnostic plant projection
  artifact.

### Notes

- v0.16.0 is a versioning/evidence cleanup release, not a new numerical
  conformance release.
- The plant projection addendum is not the defining purpose of v0.16.
- Projected plant rows keep `algorithm_parity: false`,
  `conformance_claim: false`, and `tolerance_policy: none`.
- Tolerance-gated conformance remains limited to the declared v0.8 and v0.9
  no-mass cases.

## v0.15.0 - 2026-06-07

Plant-loop diagnostic baseline release.

### Added

- `plant_loop_diagnostic_001` diagnostic-only PlantLoadProfile baseline case.
- `plant-state` and `plant-equipment` output request classes.
- `plant-loop-diagnostic-smoke` gate for manifest validation, zero-warning
  EnergyPlus baseline generation, staged epJSON compile/plan checks, and
  baseline-only report summary validation.
- v0.15 plan, readiness, release notes, verification gate, and package checks.

### Notes

- v0.15.0 is a diagnostic baseline release, not a plant numerical conformance
  claim.
- No plant flow balancing, operation scheme, pump electricity, district
  heating, boiler load, chiller load, plant node-state, meter, sizing, or
  ExampleFiles compatibility claim is made.
- Tolerance-gated conformance remains limited to the declared v0.8 and v0.9
  no-mass cases.

## v0.14.0 - 2026-06-07

Plant source mapping release.

### Added

- `plant-source-map.md` source-function map for EnergyPlus plant manager,
  loop-side simulation, component dispatch, plant utilities, and first
  pump/boiler/chiller output paths.
- v0.14 plan, readiness, release notes, verification gate, and package checks.
- plant output-variable matrix entries for future diagnostic-only plant loop
  and first equipment outputs.

### Notes

- v0.14.0 is a planning-guard release, not a plant numerical conformance claim.
- No plant flow balancing, operation scheme, pump electricity, boiler load,
  chiller load, plant node-state, meter, sizing, or ExampleFiles compatibility
  claim is made.
- Tolerance-gated conformance remains limited to the declared v0.8 and v0.9
  no-mass cases.

## v0.13.0 - 2026-06-07

PlantLoop typed graph skeleton release.

### Added

- typed `PlantLoop`, `Branch`, `BranchList`, `Connector:Splitter`, `Connector:Mixer`, and `ConnectorList` records.
- typed identity records for `Pump:ConstantSpeed`, `Boiler:HotWater`, and `Chiller:Electric:EIR`.
- plant graph edge summaries for loop-to-branch-list, branch-list-to-branch, connector-list-to-connector, and branch-to-component links.
- `plant-loop-skeleton.epJSON` fixture and `plant-loop-skeleton-smoke` gate.
- v0.13 release verification gate and release notes.
- v0.13 packaging of the promoted v0.8/v0.9 numerical conformance PDF/HTML/JSON evidence pack.
- portable Python `3.11.9` setup plus pinned `oodocs`/`matplotlib` report venv for reproducible evidence generation.
- Python `oodocs` numerical evidence generator with table of contents, direct matplotlib figure insertion, and separate values tables for chart backing data.

### Notes

- v0.13.0 is a typed graph smoke release, not a plant numerical conformance claim.
- No plant flow balancing, equipment load, operation scheme, meter, node-state, or ExampleFiles compatibility claim is made.
- Tolerance-gated conformance remains limited to the declared v0.8 and v0.9 no-mass cases.

## v0.12.0 - 2026-06-07

Node source mapping and diagnostic projection evidence-policy release.

### Added

- v0.12 node-state source-function map for EnergyPlus 26.1.0 node registration, storage, update, and output paths.
- `NodeStateProjectionEvidencePolicy` for the Rust node-state projection artifact.
- source-map, timestamp, warmup, sentinel, and excluded-variable fields in node-state projection markdown/JSON artifacts.
- EnergyPlus `SensedNodeFlagValue` handling boundary for future `System Node Setpoint Temperature` sampling.
- strengthened v0.12 verification gate that runs the air-side node diagnostic smoke and checks projection policy markers.
- release PDF/HTML/JSON numerical conformance evidence pack with accuracy and execution-time graphs for promoted v0.8/v0.9 cases.
- v0.12 release notes and package metadata.

### Notes

- v0.12.0 is a planning-guard release, not a node, IdealLoads, HVAC, plant, meter, or general ExampleFiles numerical conformance claim.
- `System Node Setpoint Temperature` remains excluded until setpoint ownership and sentinel filtering are ported.
- Tolerance-gated conformance remains limited to the declared v0.8 and v0.9 no-mass cases.

## v0.11.0 - 2026-06-07

Air-side node diagnostic release for the typed IdealLoads node graph.

### Added

- `node-state` output request class.
- `air_side_node_diagnostic_001` conformance case and IDF.
- v0.11 air-side node diagnostic smoke gate.
- v0.11 release verification gate.
- baseline-only node report evidence for system node temperature, humidity ratio, and mass flow rate.
- v0.11 release notes and release package coverage for conformance cases and suites.

### Notes

- v0.11.0 does not claim node, IdealLoads, HVAC, plant, meter, or general ExampleFiles numerical conformance.
- Tolerance-gated conformance remains limited to the declared v0.8 and v0.9 no-mass cases.

## v0.1.0 - 2026-06-04

First runnable RawModel / epJSON inspection release with a typed compile preview.

### Added

- epJSON loader in `ep_raw_model`.
- Raw value preservation for strings, booleans, nulls, numbers, arrays, and nested objects.
- RawModel summary with version, object type count, object count, and per-type counts.
- `eplus-rs model inspect <input.epJSON>` CLI command.
- `eplus-rs model compile <input.epJSON>` preview command.
- Seed tracked/untracked object reporting in model inspection.
- TypedModel preview for the first seed object families.
- NameMap-based case-insensitive reference resolution to typed IDs.
- Default application tracking during typed conversion.
- Missing reference diagnostics for the preview typed subset.
- TypedModel valid and negative preview fixtures.
- release package creation.
- v0.1 runnable release verification gate.
- tag-push GitHub Actions release workflow.
- v0.1 readiness and release documentation.

### Notes

- v0.1.0 does not perform full schema validation, graph validation, or simulation.
- v0.2.0 hardens TypedModel / Reference Resolution beyond the v0.1 preview.

## Foundation Checkpoints

These were completed before the first public semver tag:

- reproducible setup
- Cargo workspace skeleton
- Rust-only policy
- portable EnergyPlus 26.1.0 oracle
- reference source download
- oracle smoke and IDF to epJSON conversion smoke
- docs skeleton
