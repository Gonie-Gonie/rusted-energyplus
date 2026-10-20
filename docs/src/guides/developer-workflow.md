---
status: active
claim_level: none
owner: docs
last_reviewed: 2026-10-20
---

# Developer Workflow

Use `scripts/dev.cmd` as the command entry point. Run from the repository root:

```powershell
.\scripts\dev.cmd list
.\scripts\dev.cmd check -Package ep_runtime -Filter surface_environment -Lib
.\scripts\dev.cmd check -Full -Plan
```

## Choose The Smallest Relevant Check

| Change | Check during development |
|---|---|
| Runtime calculation or state | Affected package/test filter and the card's same-input Native comparison |
| Parser, compiler, CLI or run pipeline | Affected package/test filter and its real input scenario |
| Documentation | `docs-generate`, then `docs-check` |
| Plan/evidence metadata | `python tools/porting/check_plan.py --check` and `python tools/porting/sync_plan_views.py --check` |
| Claims/specs | Matching spec validator, `manifest-validate-all`, `strict-no-false-conformance` |

`check` defaults to formatting, `ep_runtime` lint and one package test run.
`-Package ep_runtime,ep_model` selects packages; `-Lib` limits to library tests;
`-Filter` selects a test substring. `-Plan` prints commands, and `-List` lists
matching tests. Zero matches and ignored-only selections fail.

At an integration milestone, `check -Full` runs workspace lint/tests and
broader guards once. It rejects partial package/test selectors. `test` without
selectors runs the workspace once. Reuse a successful unchanged run rather
than repeating focused and full suites. The detailed change-to-test mapping
and optional Native cohort are in [Contributing](../../../CONTRIBUTING.md#test-selection).

## Inspect Inputs And State

```powershell
cargo run --locked -p ep_cli -- model inspect <input.epJSON>
cargo run --locked -p ep_cli -- model compile <input.epJSON>
cargo run --locked -p ep_cli -- model plan <input.epJSON>
```

Inspection and tracing do not establish numerical compatibility. Add
`--trace-level surface` to inspect geometry, or `full` only when detailed
schedule/weather/psychrometric events are needed. Compare the state fields
required by the affected card. Use [Verify the port](verify-porting.md) for
same-IDF Oracle/Rust reported-value comparisons.

The optional SUR-02 isolated-history checker remains outside the default suite:

```powershell
python tools/porting/check_sur02_units.py --output .runtime/porting/SUR-02/isolated-history-local
```

It requires the pinned EnergyPlus source, C++/Rust compilers and the retained
Source59 compiler-input plan named in the checker; use a fresh output folder.

## Update Documentation And Commit

Update the current contract or guide where a reader needs the change.
Machine-readable claim/coverage decisions live under `specs/`; regenerate their
reference pages rather than copying tables into prose:

```powershell
.\scripts\dev.cmd docs-generate
.\scripts\dev.cmd docs-check
```

Keep one current result/limitation and the command needed to verify it. Put
execution data under `.runtime`; publish only the curated evidence needed for
a claim. Git retains replaced prose. See the [retention policy](../adr/0001-docs-specs-and-evidence-retention.md).

Commit and push each coherent change after its relevant checks. Release
packaging and evidence generation follow [Release Process](release-process.md).
