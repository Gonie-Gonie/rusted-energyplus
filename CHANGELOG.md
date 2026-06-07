# Changelog

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
