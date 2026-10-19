---
status: active
claim_level: none
owner: qa
last_reviewed: 2026-10-12
---

# Verification

During development, select the affected package and test-name substring:

```powershell
.\scripts\dev.cmd check -Package ep_runtime -Filter surface_environment -Lib
.\scripts\dev.cmd check -Full -Plan
```

`check` defaults to workspace formatting, `ep_runtime` lint, and one package test
invocation. `-Plan` prints commands; `-List` lists matching tests. A filtered run must execute at least one test; zero matches and ignored-only
matches fail. `test` without selectors runs workspace tests once.

The integration gate is `.\scripts\dev.cmd check -Full`. It runs workspace
lint/tests once, then the broader conformance, documentation, and claim guards.
It rejects partial package/test selectors. Run it at integration milestones
and before release validation; use focused checks while implementing a card.

The [SUR-02 focused unit comparison](../../../energyplus_porting_plan/evidence/SUR-02-isolated-history-impulse-61/summary.json) is an explicit porting check outside the default suite. It needs the pinned EnergyPlus source, C++/Rust compilers and the retained Source59 compiler-input plan named in the checker. Use a fresh output directory:

```powershell
python tools/porting/check_sur02_units.py --output .runtime/porting/SUR-02/isolated-history-local
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

Object support is recorded in `specs/object_coverage.toml` and the generated
object-coverage page. Parsing or typing an object does not establish that its
runtime behavior was simulated or compared with EnergyPlus.

Support assessment is the internal gate for arbitrary IDF/epJSON runs. It
loads capability rules, checks active objects and algorithms, classifies the
run state, and writes `support-assessment.json` plus `support-report.md`.
The run-framework smoke is part of the full integration gate because it verifies
supported, blocked, oracle-baseline, compare, exit-code, and output-layout
contracts end to end:

```powershell
.\scripts\dev.cmd arbitrary-run-smoke
```

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
