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

Scripted release and evidence documents follow the
[Documentation Framework](documentation-framework.md): PowerShell entry points
orchestrate repo-local Python, and Python generators use `oodocs` plus
matplotlib for document layout and charts.

| Command | Area | Purpose | Blocking release? | Main artifacts |
|---|---|---|---:|---|
| `setup` | setup | prepare toolchain, oracle, reference source, docs tools, portable Python, and report venv | yes | `.runtime`, `.reference` |
| `oracle-smoke` | setup | run EnergyPlus oracle example and conversion | no | `.runtime/oracle-smoke` |
| `source-smoke` | setup | verify reference source checkout | yes | console output |
| `python-smoke` | setup | verify portable Python and the pinned report-generation venv | yes | console output |
| `check` | quality | run fmt, clippy, tests, smoke gates, docs, and guards | yes | console output |
| `test` | quality | run Rust workspace tests | yes | console output |
| `docs-generate` | quality | regenerate mdBook generated references from `specs/` and tracked case manifests | yes for docs/spec changes | `docs/src/generated`, `tools/docs/generated-docs.manifest.json` |
| `docs-check` | quality | build mdBook | yes | `docs/book` |
| `file-size-check` | quality | warn/fail on oversized source files with explicit legacy waivers | yes | console output |
| `perf` | quality | run local performance checks | no | console output |
| `strict-no-false-conformance` | quality | scan for forbidden compatibility wording | yes | failure on wording |
| `algorithm-ledger-check` | quality | validate source-map, EnergyPlus source, Rust target, first-case, and gate links | yes | console output |
| `raw-model-smoke` | smoke | inspect RawModel fixtures | no | console output |
| `typed-model-smoke` | smoke | compile TypedModel fixtures | no | console output |
| `model-plan-smoke` | smoke | verify graph and execution-plan summaries | no | console output |
| `schedule-compact-smoke` | smoke | verify `Schedule:Compact` intake | no | console output |
| `geometry-smoke` | smoke | summarize Rust geometry interpretation | no | console output |
| `first-zone-smoke` | diagnostic | exercise first-zone runtime plumbing | no | diagnostic output |
| `runtime-registry-smoke` | smoke | gate runtime output/meter registry and ResultStore diagnostics | yes | cargo test filters |
| `heat-balance-generalization-smoke` | smoke | gate opaque no-mass boundary handling and existing heat-balance conformance gates | yes | cargo test filters, `.runtime/heat-balance-conformance`, `.runtime/surface-temperature-conformance` |
| `ideal-loads-thermostat-smoke` | smoke | gate the v0.10 thermostat, equipment, IdealLoads typed graph, warning policy, and nonzero baseline signal | yes | `.runtime/ideal-loads-thermostat` |
| `air-side-node-diagnostic-smoke` | smoke | gate the v0.11 air-side node baseline evidence and diagnostic Rust projection | yes | `.runtime/air-side-node-diagnostic` |
| `plant-loop-skeleton-smoke` | smoke | gate the v0.13 PlantLoop typed graph skeleton fixture | yes | console output |
| `plant-loop-diagnostic-smoke` | smoke | gate the v0.15 PlantLoadProfile baseline-only plant diagnostic | yes | `.runtime/plant-loop-diagnostic` |
| `plant-loop-projection-smoke` | smoke | gate the diagnostic Rust plant-state projection addendum | yes | `.runtime/plant-loop-diagnostic/plant-state-projection` |
| `compare-schedule-smoke` | compare | compare constant schedule ESO values | no | `.runtime/compare-schedule` |
| `compare-weather-smoke` | compare | compare selected EPW weather fields against ESO | no | `.runtime/compare-weather` |
| `compare-schedule-conformance` | compare | run the v0.22 tolerance-gated `Schedule Value` conformance case | yes | `.runtime/time-weather-schedule-conformance` |
| `compare-weather-conformance` | compare | run the v0.22 tolerance-gated dry-bulb conformance case | yes | `.runtime/time-weather-schedule-conformance` |
| `compare-static-model-conformance` | compare | run the v0.23 official ExampleFile static model conformance case | yes | `.runtime/static-model-conformance` |
| `compare-geometry-smoke` | compare | compare Rust geometry summary with EIO | no | console output |
| `compare-surface-geometry-smoke` | compare | compare Rust surface area, azimuth, and tilt with EIO | no | `.runtime/compare-surface-geometry` |
| `compare-construction-materials-smoke` | compare | compare construction/material thermal inputs with EIO | no | console output |
| `compare-internal-gains-smoke` | compare | compare nominal OtherEquipment EIO rows | no | console output |
| `compare-internal-convective-gain-smoke` | compare | compare internal convective gain ESO trace | no | `.runtime/compare-internal-gains` |
| `compare-internal-convective-gain-conformance` | compare | run the v0.26 tolerance-gated internal convective gain conformance case | yes | `.runtime/internal-gains-conformance` |
| `compare-zone-smoke` | diagnostic | extract heat-balance zone-temperature deltas and report artifacts only | no | `.runtime/compare-zone/compare` |
| `compare-heat-balance-conformance` | compare | run the v0.8 tolerance-gated heat-balance conformance case | yes | `.runtime/heat-balance-conformance` |
| `compare-surface-temperature-conformance` | compare | run the v0.9 tolerance-gated surface-temperature conformance case | yes | `.runtime/surface-temperature-conformance` |
| `compare-regression` | compare | run current compare suite and write reports | no | `.runtime/compare-regression` |
| `compare-series-v2-smoke` | compare | gate timestamp-aware selected series reader and comparison metrics v2 | yes | console output |
| `conformance-schema-smoke` | conformance | validate case/suite schema fixtures | yes | console output |
| `manifest-validate-all` | conformance | validate all tracked case manifests against Case Manifest / Output Request Schema v2 | yes | console output |
| `conformance-baseline-smoke` | conformance | generate EnergyPlus baseline artifacts | no | `.runtime/conformance-baseline` |
| `conformance-report-smoke` | conformance | write baseline-only report skeleton | no | `.runtime/conformance-report` |
| `conformance-diagnostic-report-smoke` | conformance | generate diagnostic-only compare artifacts from a case manifest | no | `.runtime/conformance-diagnostic` |
| `package` | release | build local package artifact | yes for package release | package zip |
| `conformance-evidence-report` | release | generate oodocs/matplotlib PDF/HTML/JSON release evidence for promoted numerical conformance cases | yes for conformance release | `.runtime/release-evidence` |
| `conformance-index-report` | release | generate oodocs/matplotlib PDF/HTML/JSON/Markdown conformance index coverage matrices | yes for release coverage | `.runtime/release-evidence` |
| `support-coverage-report` | release | generate oodocs/matplotlib PDF/HTML/JSON/Markdown user-facing support coverage for inputs, outputs, and algorithms | yes for release coverage | `.runtime/release-evidence` |
| `user-coverage-handbook` | release | generate oodocs/matplotlib PDF/HTML/JSON/Markdown user decision guide for supported inputs, outputs, and algorithms | yes for release coverage | `.runtime/release-evidence` |
| `release-evidence-manifest` | release | generate oodocs PDF/HTML/JSON/Markdown release package and evidence asset manifest | yes for release coverage | `.runtime/release-evidence` |
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
| `v0.12-verify` | release | verify v0.12 node source mapping evidence and release evidence assets | yes | source-map docs, `.runtime/release-evidence` |
| `v0.13-verify` | release | verify v0.13 PlantLoop typed graph skeleton and release evidence assets | yes | plant fixture, source-map docs, `.runtime/release-evidence` |
| `v0.14-verify` | release | verify v0.14 plant source mapping and release evidence assets | yes | plant source-map docs, `.runtime/release-evidence` |
| `v0.15-verify` | release | verify v0.15 plant-loop diagnostic baseline and release evidence assets | yes | plant diagnostic case, report skeleton, `.runtime/release-evidence` |
| `v0.16-verify` | release | verify v0.16 versioning/evidence cleanup, diagnostic plant projection addendum, and release evidence assets | yes | roadmap docs, plant projection artifacts, `.runtime/release-evidence` |
| `v0.17-verify` | release | verify v0.17 Case Manifest and Output Request Schema v2 gate | yes | v2 manifests, `.runtime/release-evidence` |
| `v0.18-verify` | release | verify v0.18 output request injection and official baseline gate | yes | official baseline, `.runtime/release-evidence` |
| `v0.19-verify` | release | verify v0.19 series reader and compare engine v2 gate | yes | compare-series-v2 smoke, `.runtime/release-evidence` |
| `v0.20-verify` | release | verify v0.20 conformance report generator and coverage matrix gate | yes | conformance index report, `.runtime/release-evidence` |
| `v0.21-verify` | release | verify v0.21 source-map and algorithm ledger validation gate | yes | algorithm ledger, generated docs, `.runtime/release-evidence` |
| `v0.22-verify` | release | verify v0.22 declared time/weather/schedule conformance gates | yes | schedule/weather reports, `.runtime/release-evidence` |
| `v0.23-verify` | release | verify v0.23 official ExampleFile static model evidence gate | yes | static model reports, `.runtime/release-evidence` |
| `v0.24-verify` | release | verify v0.24 runtime state and output registry hardening gate | yes | runtime registry smoke, `.runtime/release-evidence` |
| `v0.25-verify` | release | verify v0.25 opaque no-mass heat-balance generalization gate | yes | heat-balance generalization smoke, `.runtime/release-evidence` |
| `v0.26-verify` | release | verify v0.26 internal convective gains conformance gate | yes | internal-gains conformance report, `.runtime/release-evidence` |
| `v0.27-verify` | release | verify v0.27 user support coverage report gate | yes | support coverage PDF/HTML/JSON/Markdown, `.runtime/release-evidence` |
| `v0.28-verify` | release | verify v0.28 input object coverage metadata gate | yes | support coverage PDF/HTML/JSON/Markdown, generated object coverage |
| `v0.29-verify` | release | verify v0.29 output variable coverage metadata gate | yes | support coverage PDF/HTML/JSON/Markdown, generated variable coverage |
| `v0.30-verify` | release | verify v0.30 algorithm coverage metadata gate | yes | support coverage PDF/HTML/JSON/Markdown, generated algorithm ledger |
| `v0.31-verify` | release | verify v0.31 release evidence asset manifest gate | yes | release package, evidence manifest PDF/HTML/JSON/Markdown, `.runtime/release-evidence` |
| `v0.32-verify` | release | verify v0.32 user coverage handbook gate | yes | user coverage handbook PDF/HTML/JSON/Markdown, release manifest |

No diagnostic command should be listed as conformance evidence.
