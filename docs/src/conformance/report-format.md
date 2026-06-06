---
status: active
claim_level: none
owner: conformance
last_reviewed: 2026-06-07
---

# Report Format

ExampleFiles-based evidence must produce both a machine-readable summary and a
human-readable report. A command that only prints console output is smoke or
diagnostic evidence, not release conformance evidence.

## Artifact Layout

Canonical release evidence should converge on this shape:

```text
.runtime/evidence/<version>/<case_id>/
  oracle/
    input.idf
    input.epJSON
    eplusout.err
    eplusout.eso
    eplusout.mtr
    eplusout.sql
    eplusout.rdd
    eplusout.mdd
    selected_outputs.csv
    selected_meters.csv
  rust/
    result_store.json
    diagnostics.json
    trace.json
    selected_outputs.csv
    selected_meters.csv
  compare/
    compare-summary.json
    compare-report.md
    variable-deltas.csv
    meter-deltas.csv
    divergence-points.csv
    tolerance-failures.csv
```

Current scripts still write milestone-specific `.runtime/compare-*` and
`.runtime/conformance-*` directories. Those are acceptable smoke artifacts
until the canonical evidence layout is implemented.

## Summary JSON

`compare-summary.json` should include:

- case id
- oracle version
- rusted-energyplus git commit
- comparison class
- conformance claim
- status
- input and weather paths
- requested variable and meter coverage
- per-variable samples, tolerance, max absolute delta, RMSE, first divergence,
  and status
- unsupported or skipped outputs

## Markdown Report

`compare-report.md` must include:

1. case metadata
2. stage summary
3. requested output coverage
4. meter summary
5. variable summary
6. surface-level details, when present
7. node-level details, when present
8. component-level details, when present
9. known gaps
10. gate decision

The gate decision must state whether the report is `baseline-only`,
`diagnostic-only`, or `conformance`, and whether it blocks the release.

## Release Index

Each release should eventually publish:

```text
reports/<version>/
  conformance-index.md
  conformance-summary.json
  cases/<case_id>/compare-report.md
  cases/<case_id>/compare-summary.json
```

The index should summarize case tier counts, pass/fail/diagnostic/baseline-only
status, variable coverage, object coverage, and known gaps.
