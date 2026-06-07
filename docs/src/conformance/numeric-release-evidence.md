---
status: active
claim_level: conformance-boundary
owner: conformance
last_reviewed: 2026-06-07
---

# Numeric Release Evidence

Release-stage numerical conformance evidence is distributed as a PDF evidence
pack, not only as markdown reports. The release PDF/HTML/JSON evidence pack is
generated from the promoted conformance summaries and uploaded as GitHub
Release assets beside the binary zip.

Current command:

```powershell
.\scripts\dev.cmd conformance-evidence-report -Version 0.23.0
```

Current generated files:

```text
.runtime/release-evidence/v0.23.0/numeric-conformance-evidence.html
.runtime/release-evidence/v0.23.0/numeric-conformance-evidence.pdf
.runtime/release-evidence/v0.23.0/numeric-conformance-evidence.json
```

Current GitHub Release asset names:

```text
numeric-conformance-evidence.html
numeric-conformance-evidence.pdf
numeric-conformance-evidence.json
```

## Included Cases

Only promoted, tolerance-gated numerical conformance cases enter the release
PDF. v0.22.0 added declared time/weather/schedule variables to the earlier
v0.8/v0.9 no-mass cases. v0.23.0 adds static EIO evidence, but that evidence
is intentionally excluded from this numerical PDF and represented instead by
`compare-static-model-conformance` plus the conformance index:

Historical note: v0.12 through v0.21 release evidence kept the earlier
v0.8/v0.9 cases only.

| Milestone | Case | Variables |
|---|---|---|
| v0.8 | `heat_balance_nomass_001` | `Zone Mean Air Temperature` |
| v0.9 | `surface_temperature_nomass_001` | zone MAT plus surface inside/outside face temperature |
| v0.22 | `schedule_constant_001` | `Schedule Value` |
| v0.22 | `weather_fields_001` | `Site Outdoor Air Drybulb Temperature` only |

The PDF includes:

- claim boundary and explicit non-claims
- table of contents and release summary sections
- case matrix
- matplotlib accuracy graph against declared tolerances
- matplotlib execution-time graph using release gate wall-clock and EnergyPlus oracle
  elapsed time
- per-series max absolute delta, RMSE, tolerance, sample count, and status
- artifact paths for the HTML, PDF, and JSON evidence

## Generator Architecture

The release command is intentionally split into a thin PowerShell wrapper and a
Python document generator:

| Layer | Path | Responsibility |
|---|---|---|
| wrapper | `scripts/release/conformance-evidence-report.ps1` | locate the repo-local report Python and invoke the generator |
| runtime setup | `scripts/lib/python.ps1`, `scripts/setup/setup.ps1` | provision portable Python `3.11.9` and the report virtual environment |
| dependencies | `tools/python/requirements-report.txt` | pin `oodocs` and `matplotlib` for stable PDF generation |
| generator | `tools/reporting/conformance_evidence_report.py` | collect summaries, build JSON evidence, render HTML/PDF |

The generator uses `oodocs` for document structure and PDF/HTML serialization.
Charts are built as matplotlib figure objects and inserted directly into
`oodocs.Figure`, so the report does not depend on a one-off HTML chart renderer
or a LaTeX toolchain. Numeric labels stay in tables below the figures instead
of being drawn densely on top of the charts.

This is now the standard framework for scripted documentation. PowerShell entry
points remain stable user-facing commands, while Python generators under
`tools/reporting` own document layout, charting, data aggregation, and
serialization. See
[Documentation Framework](../operations/documentation-framework.md).

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
