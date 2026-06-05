---
status: active
claim_level: none
owner: docs
last_reviewed: 2026-06-05
---

# Script Index

| Script | Class | Purpose | Blocking release? | Main artifacts |
|---|---|---|---:|---|
| `setup.cmd` | setup | prepare toolchain, oracle, and docs tools | yes | `.runtime`, `.reference` |
| `check.cmd` | smoke | run fmt, clippy, tests, docs, and guards | yes | console output |
| `v0.1-verify.cmd` | release gate | verify v0.1 foundation/model-intake release | yes | package inputs |
| `oracle-smoke.cmd` | smoke | run EnergyPlus oracle example and conversion | no | `.runtime/oracle-smoke` |
| `source-smoke.cmd` | setup | verify reference source checkout | yes | console output |
| `compare-schedule-smoke.cmd` | smoke | compare constant schedule ESO values | no | `.runtime/compare-schedule` |
| `compare-weather-smoke.cmd` | smoke | compare EPW dry-bulb against ESO | no | `.runtime/compare-weather` |
| `compare-geometry-smoke.cmd` | smoke | compare Rust geometry summary with EIO | no | console output |
| `compare-internal-gains-smoke.cmd` | smoke | compare nominal OtherEquipment EIO rows | no | console output |
| `compare-internal-convective-gain-smoke.cmd` | smoke | compare internal convective gain ESO trace | no | `.runtime/compare-internal-convective-gain` |
| `first-zone-smoke.cmd` | diagnostic | exercise first-zone runtime plumbing | no | diagnostic output |
| `compare-zone-smoke.cmd` | diagnostic | extract zone-temperature deltas only | no | `.runtime/compare-zone` |
| `compare-regression.cmd` | regression | run current compare smoke/diagnostic suite and write reports | no | `.runtime/compare-regression` |
| `conformance-schema-smoke.cmd` | smoke | validate conformance case/suite schema | yes | console output |
| `conformance-baseline-smoke.cmd` | baseline-only | generate EnergyPlus baseline artifacts | no | `.runtime/conformance-baseline` |
| `conformance-report-smoke.cmd` | baseline-only | write baseline-only report skeleton | no | `.runtime/conformance-report` |
| `strict-no-false-conformance.cmd` | release guard | scan for forbidden compatibility wording | yes | failure on wording |
| `docs-check.cmd` | docs | build mdBook | yes | `docs/book` |
| `package.cmd` | release utility | build local package artifact | yes for package release | package zip |

No diagnostic script should be listed as conformance evidence.

