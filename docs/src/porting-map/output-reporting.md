---
status: active
claim_level: none
owner: conformance
last_reviewed: 2026-06-05
---

# Output and Reporting

Output/reporting work should preserve EnergyPlus variable identity and
frequency semantics.

Current foundations:

- `OutputRegistry`
- `ResultStore`
- `compare-summary.json`
- `compare-report.md`
- first-divergence reporting
- diagnostic-only MAT delta artifacts for `compare zone-temperature --report-dir`
- manifest-driven diagnostic MAT report generation
- manifest metadata in diagnostic MAT `compare-summary.json` and report

Next evidence target:

- manifest-driven multi-series comparison
- artifact directories split into `oracle`, `rust`, and `compare`
- variable-level tolerance summaries
