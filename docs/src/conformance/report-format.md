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

For the v0.6 diagnostic infrastructure release, `compare-zone-smoke` and
`conformance-diagnostic-report-smoke` write diagnostic-only MAT reports, while
`compare-regression` writes suite-level trace, summary, markdown report, and
profile skeleton artifacts. These artifacts are release infrastructure
evidence; they do not become heat-balance conformance without tolerances, Rust
result artifacts for the declared outputs, and blocking gates.

For the v0.2 baseline-only harness, `conformance baseline` writes
`case-expanded.toml` beside the EnergyPlus artifacts, and `conformance
report-skeleton` writes both `compare-report.md` and `compare-summary.json`.
Those files are report-contract evidence only; they are not numerical
conformance evidence until Rust artifacts, tolerances, and a blocking gate are
present.

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
- per-variable samples, baseline min, baseline max, nonzero sample count,
  tolerance, max absolute delta, RMSE, first divergence, and status
- EnergyPlus ERR warning counts and warning excerpts for baseline-only reports
- unsupported or skipped outputs

The v0.19 compare engine v2 supplies the timestamp-aware alignment mode,
compared-sample count, maximum absolute delta, RMSE, maximum relative delta,
status, and first divergence reason that future selected output and meter
reports should serialize.

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
  conformance-index-report.html
  conformance-index-report.pdf
  conformance-index-report.json
  support-coverage.md
  support-coverage-report.html
  support-coverage-report.pdf
  support-coverage-report.json
  cases/<case_id>/compare-report.md
  cases/<case_id>/compare-summary.json
```

The index should summarize case tier counts, pass/fail/diagnostic/baseline-only
status, variable coverage, object coverage, report contracts, gate contracts,
and known gaps. v0.20 implements the release-level `conformance-index-report.pdf`
and companion HTML/JSON/Markdown artifacts under `.runtime/release-evidence`.

v0.27 adds the user-facing support coverage report under the same release
evidence root. It is generated with `oodocs` from object coverage, variable
coverage, the algorithm ledger, and case manifests, and it answers which input
objects, output variables, and algorithm families are currently supported.
