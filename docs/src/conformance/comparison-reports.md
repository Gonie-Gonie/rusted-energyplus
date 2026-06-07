---
status: active
claim_level: none
owner: conformance
last_reviewed: 2026-06-05
---

# Comparison Reports

A human-reviewable report should include:

- case id
- oracle version
- rusted-energyplus git commit
- input file
- weather file, when present
- comparison class
- conformance claim
- blocking gate
- output variables
- tolerance policy
- sample count
- max absolute delta
- max relative delta, when applicable
- first divergence timestamp or sample index
- status per variable
- unsupported diagnostics

`compare-report.md` and `compare-summary.json` are the per-case release-facing
evidence artifacts once a case becomes conformance-level. Promoted numerical
cases are also summarized in the generated release PDF/HTML/JSON evidence pack
described in `numeric-release-evidence.md`.

v0.20 adds a release conformance index report for all tracked manifests. That
index summarizes case, output, meter, domain, report-contract, and gate-contract
coverage, but it does not promote numerical conformance by itself.

v0.19 adds the compare engine v2 metric layer for future reports. It can align
selected series by timestamp when both sides provide timestamp labels, falls
back to index alignment otherwise, and records RMSE, maximum relative delta,
status, and first divergence reason.

ExampleFiles-based reports should also summarize stage status, requested output
coverage, meter coverage, surface-level details, node-level details,
component-level details, known gaps, and the final gate decision. The canonical
format is defined in `report-format.md`.
