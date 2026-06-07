---
status: active
claim_level: none
owner: conformance
last_reviewed: 2026-06-07
---

# Output and Reporting

Output/reporting work should preserve EnergyPlus variable identity and
frequency semantics.

Current foundations:

- `OutputRegistry`
- `ResultStore`
- `trace.json`
- `compare-summary.json`
- `compare-report.md`
- `profile-summary.json`
- first-divergence reporting
- diagnostic-only MAT delta artifacts for `compare zone-temperature --report-dir`
- manifest-driven diagnostic MAT report generation
- manifest metadata in diagnostic MAT `compare-summary.json` and report

Current v0.6 artifact contract:

- `first-zone-smoke` verifies the runtime-native `ResultStore` diagnostic path.
- `compare-zone-smoke` writes diagnostic-only MAT
  `compare-summary.json` and `compare-report.md`.
- `conformance-diagnostic-report-smoke` stages oracle artifacts and writes a
  manifest-driven diagnostic MAT report for `zone_temperature_diagnostic_001`.
- `compare-regression` writes suite-level `trace.json`,
  `compare-summary.json`, `compare-report.md`, and `profile-summary.json`.
- Zone-temperature artifacts must remain `diagnostic-only`,
  `conformance_claim: false`, and `tolerance_policy: none`.

Next evidence target:

- manifest-driven multi-series comparison
- artifact directories split into `oracle`, `rust`, and `compare`
- variable-level tolerance summaries
