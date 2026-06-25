---
status: active
claim_level: none
owner: qa
last_reviewed: 2026-06-23
---

# Verification

The standard local gate is:

```powershell
.\scripts\dev.cmd check
```

Documentation and generated specs:

```powershell
.\scripts\dev.cmd docs-generate
.\scripts\dev.cmd docs-check
.\scripts\dev.cmd algorithm-ledger-check
```

Claim and manifest guards:

```powershell
.\scripts\dev.cmd manifest-validate-all
.\scripts\dev.cmd strict-no-false-conformance
```

Conformance gates require declared cases, variables/meters, tolerances,
oracle baselines, Rust artifacts, generated compare reports, and blocking
scripts. Diagnostic reports can explain source-order mismatches, but they do
not support compatibility claims.

Support assessment is the internal gate for arbitrary IDF/epJSON runs. It
loads capability rules, checks active objects and algorithms, classifies the
run state, and writes `support-assessment.json` plus `support-report.md`.

Launcher smoke verifies that the Windows launcher builds `eplus-rs run`
commands, maps `run-summary.json` states and exit codes, preserves the
claim-boundary presentation, and can build the no-console executable wrapper:

```powershell
.\scripts\dev.cmd launcher-smoke
```

Oracle baseline and compare are optional for arbitrary runs. When enabled, the
pipeline writes `oracle/`, `compare/compare-summary.json`, and
`compare/compare-report.md`; these artifacts remain ad-hoc unless promoted by
a reviewed release manifest.

Release evidence is generated with:

```powershell
.\scripts\dev.cmd conformance-evidence-report -Version 0.1.0
.\scripts\dev.cmd conformance-index-report -Version 0.1.0
.\scripts\dev.cmd support-coverage-report -Version 0.1.0
.\scripts\dev.cmd user-coverage-handbook -Version 0.1.0
.\scripts\dev.cmd release-evidence-manifest -Version 0.1.0
```

PDF and report artifacts must clearly label release conformance, diagnostics,
and arbitrary ad-hoc runs as separate evidence classes.
