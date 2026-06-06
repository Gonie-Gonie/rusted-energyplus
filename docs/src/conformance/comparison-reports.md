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

`compare-report.md` and `compare-summary.json` are the release-facing evidence
artifacts once a case becomes conformance-level.

ExampleFiles-based reports should also summarize stage status, requested output
coverage, meter coverage, surface-level details, node-level details,
component-level details, known gaps, and the final gate decision. The canonical
format is defined in `report-format.md`.
