---
status: active
claim_level: reporting-infrastructure
owner: conformance
last_reviewed: 2026-06-08
---

# User Coverage Handbook

The user coverage handbook is the release-facing guide to the current supported
subset. It is generated from the support coverage JSON and conformance index
JSON, then rendered with `oodocs` as PDF/HTML/JSON/Markdown.

Use it to answer:

- which input objects are typed versus structural only
- which output variables are promoted conformance, diagnostic, or baseline only
- which algorithm families have limited conformance evidence versus diagnostic projection evidence
- which conformance cases define the current public numerical claim
- which conformance output requests are declared versus which numerical
  time-series actually passed release evidence
- which gaps must not be inferred from neighboring support rows

Generate it after the support and index reports:

```powershell
.\scripts\dev.cmd support-coverage-report -Version 0.32.0
.\scripts\dev.cmd conformance-index-report -Version 0.32.0
.\scripts\dev.cmd conformance-evidence-report -Version 0.32.0
.\scripts\dev.cmd user-coverage-handbook -Version 0.32.0
```

Artifacts are written to:

```text
.runtime/release-evidence/v0.32.0/user-coverage-handbook.md
.runtime/release-evidence/v0.32.0/user-coverage-handbook.html
.runtime/release-evidence/v0.32.0/user-coverage-handbook.pdf
.runtime/release-evidence/v0.32.0/user-coverage-handbook.json
```

## Difference From Support Coverage

The support coverage report is the detailed matrix. The handbook is the user decision guide over that matrix.
It keeps detailed rows, but organizes them
around decision rules, promoted outputs, diagnostic/baseline outputs, declared
versus passed numerical series, algorithm scope, and known gaps.

## Boundary

The handbook is reporting infrastructure. It does not add new numerical
conformance, full EnergyPlus compatibility, HVAC numerical conformance, plant
numerical conformance, or meter conformance.

Use the release evidence manifest to confirm that the handbook and its source
coverage reports were uploaded as GitHub Release assets.
