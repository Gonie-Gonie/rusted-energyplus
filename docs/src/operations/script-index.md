---
status: active
claim_level: none
owner: docs
last_reviewed: 2026-06-07
---

# Script Index

All repository tasks are launched through one Windows wrapper:

```powershell
.\scripts\dev.cmd <command> [args...]
```

PowerShell users can also call `.\scripts\dev.ps1 <command> [args...]`
directly. The implementation scripts live in field-specific folders:

- `scripts/setup`
- `scripts/quality`
- `scripts/smoke`
- `scripts/compare`
- `scripts/conformance`
- `scripts/release`
- `scripts/lib`

Run `.\scripts\dev.cmd list` for the command catalog.

| Command | Area | Purpose | Blocking release? | Main artifacts |
|---|---|---|---:|---|
| `setup` | setup | prepare toolchain, oracle, reference source, docs tools, portable Python, and report venv | yes | `.runtime`, `.reference` |
| `oracle-smoke` | setup | run EnergyPlus oracle example and conversion | no | `.runtime/oracle-smoke` |
| `source-smoke` | setup | verify reference source checkout | yes | console output |
| `python-smoke` | setup | verify portable Python and the pinned report-generation venv | yes | console output |
| `check` | quality | run fmt, clippy, tests, smoke gates, docs, and guards | yes | console output |
| `test` | quality | run Rust workspace tests | yes | console output |
| `docs-check` | quality | build mdBook | yes | `docs/book` |
| `perf` | quality | run local performance checks | no | console output |
| `strict-no-false-conformance` | quality | scan for forbidden compatibility wording | yes | failure on wording |
| `raw-model-smoke` | smoke | inspect RawModel fixtures | no | console output |
| `typed-model-smoke` | smoke | compile TypedModel fixtures | no | console output |
| `model-plan-smoke` | smoke | verify graph and execution-plan summaries | no | console output |
| `schedule-compact-smoke` | smoke | verify `Schedule:Compact` intake | no | console output |
| `geometry-smoke` | smoke | summarize Rust geometry interpretation | no | console output |
| `first-zone-smoke` | diagnostic | exercise first-zone runtime plumbing | no | diagnostic output |
| `ideal-loads-thermostat-smoke` | smoke | gate the v0.10 thermostat, equipment, IdealLoads typed graph, warning policy, and nonzero baseline signal | yes | `.runtime/ideal-loads-thermostat` |
| `air-side-node-diagnostic-smoke` | smoke | gate the v0.11 air-side node baseline evidence and diagnostic Rust projection | yes | `.runtime/air-side-node-diagnostic` |
| `plant-loop-skeleton-smoke` | smoke | gate the v0.13 PlantLoop typed graph skeleton fixture | yes | console output |
| `compare-schedule-smoke` | compare | compare constant schedule ESO values | no | `.runtime/compare-schedule` |
| `compare-weather-smoke` | compare | compare selected EPW weather fields against ESO | no | `.runtime/compare-weather` |
| `compare-geometry-smoke` | compare | compare Rust geometry summary with EIO | no | console output |
| `compare-surface-geometry-smoke` | compare | compare Rust surface area, azimuth, and tilt with EIO | no | `.runtime/compare-surface-geometry` |
| `compare-construction-materials-smoke` | compare | compare construction/material thermal inputs with EIO | no | console output |
| `compare-internal-gains-smoke` | compare | compare nominal OtherEquipment EIO rows | no | console output |
| `compare-internal-convective-gain-smoke` | compare | compare internal convective gain ESO trace | no | `.runtime/compare-internal-gains` |
| `compare-zone-smoke` | diagnostic | extract heat-balance zone-temperature deltas and report artifacts only | no | `.runtime/compare-zone/compare` |
| `compare-heat-balance-conformance` | compare | run the v0.8 tolerance-gated heat-balance conformance case | yes | `.runtime/heat-balance-conformance` |
| `compare-surface-temperature-conformance` | compare | run the v0.9 tolerance-gated surface-temperature conformance case | yes | `.runtime/surface-temperature-conformance` |
| `compare-regression` | compare | run current compare suite and write reports | no | `.runtime/compare-regression` |
| `conformance-schema-smoke` | conformance | validate case/suite schema fixtures | yes | console output |
| `conformance-baseline-smoke` | conformance | generate EnergyPlus baseline artifacts | no | `.runtime/conformance-baseline` |
| `conformance-report-smoke` | conformance | write baseline-only report skeleton | no | `.runtime/conformance-report` |
| `conformance-diagnostic-report-smoke` | conformance | generate diagnostic-only compare artifacts from a case manifest | no | `.runtime/conformance-diagnostic` |
| `package` | release | build local package artifact | yes for package release | package zip |
| `conformance-evidence-report` | release | generate oodocs/matplotlib PDF/HTML/JSON release evidence for promoted numerical conformance cases | yes for conformance release | `.runtime/release-evidence` |
| `github-release` | release | publish a release with GitHub CLI | manual fallback | GitHub Release |
| `v0.1-verify` | release | verify v0.1 foundation/model-intake release | yes | package inputs |
| `v0.2-verify` | release | verify v0.2 conformance harness evidence | yes | `.runtime/conformance-*` |
| `v0.3-verify` | release | verify v0.3 input interpretation evidence | yes | console output |
| `v0.4-verify` | release | verify v0.4 time/weather/schedule evidence | yes | `.runtime/compare-*`, `.runtime/conformance-report` |
| `v0.5-verify` | release | verify v0.5 geometry/internal-variable evidence | yes | `.runtime/compare-*` |
| `v0.6-verify` | release | verify v0.6 output/trace/report diagnostic infrastructure | yes | `.runtime/compare-zone`, `.runtime/conformance-diagnostic`, `.runtime/compare-regression` |
| `v0.7-verify` | release | verify v0.7 source mapping and algorithm readiness gate | yes | source-map docs |
| `v0.8-verify` | release | verify v0.8 heat-balance conformance evidence | yes | `.runtime/heat-balance-conformance` |
| `v0.9-verify` | release | verify v0.9 surface-temperature conformance evidence | yes | `.runtime/surface-temperature-conformance` |
| `v0.10-verify` | release | verify v0.10 IdealLoads thermostat typed-graph evidence | yes | `.runtime/ideal-loads-thermostat` |
| `v0.11-verify` | release | verify v0.11 air-side node diagnostic and projection evidence | yes | `.runtime/air-side-node-diagnostic` |
| `v0.12-verify` | release | verify v0.12 node source mapping evidence and numeric evidence packaging | yes | source-map docs, `.runtime/release-evidence` |
| `v0.13-verify` | release | verify v0.13 PlantLoop typed graph skeleton and numeric evidence packaging | yes | plant fixture, source-map docs, `.runtime/release-evidence` |

No diagnostic command should be listed as conformance evidence.
