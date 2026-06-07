---
status: active
claim_level: conformance-boundary
owner: conformance
last_reviewed: 2026-06-07
---

# Numeric Release Evidence

Release-stage numerical conformance evidence is distributed as a PDF evidence
pack, not only as markdown reports. The release PDF/HTML/JSON evidence pack is
generated from the promoted conformance summaries and is packaged with the
release artifact.

Current command:

```powershell
.\scripts\dev.cmd conformance-evidence-report -Version 0.13.0
```

Current generated files:

```text
.runtime/release-evidence/v0.13.0/numeric-conformance-evidence.html
.runtime/release-evidence/v0.13.0/numeric-conformance-evidence.pdf
.runtime/release-evidence/v0.13.0/numeric-conformance-evidence.json
```

Current packaged release paths:

```text
evidence/v0.13.0/numeric-conformance-evidence.html
evidence/v0.13.0/numeric-conformance-evidence.pdf
evidence/v0.13.0/numeric-conformance-evidence.json
```

## Included Cases

Only promoted, tolerance-gated numerical conformance cases enter the release
PDF. For v0.13.0, that still means the earlier v0.8/v0.9 cases only:

| Milestone | Case | Variables |
|---|---|---|
| v0.8 | `heat_balance_nomass_001` | `Zone Mean Air Temperature` |
| v0.9 | `surface_temperature_nomass_001` | zone MAT plus surface inside/outside face temperature |

The PDF includes:

- claim boundary and explicit non-claims
- case matrix
- accuracy graph against declared tolerances
- execution-time graph using release gate wall-clock and EnergyPlus oracle
  elapsed time
- per-series max absolute delta, RMSE, tolerance, sample count, and status
- artifact paths for the HTML, PDF, and JSON evidence

## Excluded Evidence

The PDF must not accumulate every development check. These remain outside the
release evidence pack unless promoted by manifest, tolerance, result artifact,
report, and blocking gate:

- parser, schema, and intake-only checks
- diagnostic-only comparisons
- baseline-only EnergyPlus extracts
- smoke tests that prove plumbing or typed graph presence
- exploratory traces without a declared tolerance policy

This keeps the PDF focused on the public numerical claim rather than on normal
development hygiene.

## Promotion Rule

A future numerical case may enter this PDF only after it has:

- a tracked conformance case manifest
- requested output variables and output classes
- a declared tolerance policy
- EnergyPlus oracle artifacts
- Rust result artifacts
- `compare-summary.json` and `compare-report.md`
- a blocking gate script
- a release note claim boundary

Once a higher-level case supersedes low-level experiments, retire duplicate
development rows from release evidence and summarize them in the promoted case
notes instead.
