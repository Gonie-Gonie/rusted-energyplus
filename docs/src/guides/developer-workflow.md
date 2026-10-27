---
status: active
claim_level: none
owner: docs
last_reviewed: 2026-10-28
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
| Runtime calculation or state | Affected package/test filter and the same-input Native comparison for the changed behavior |
| Parser, compiler, CLI or run pipeline | Affected package/test filter and its real input scenario |
| Documentation | `docs-generate`, then `docs-check` |
| Validation metadata | Affected input/source hashes and the relevant comparison or evidence reader |
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
required by the changed behavior. Use [Verify the port](verify-porting.md) for
same-IDF Oracle/Rust reported-value comparisons.

For Source A/B, `compiled-geometry.json` preserves raw and typed input;
`geometry-consumers.json` records actual retained Source owners after completion.
Link surfaces by typed ID. The Source observation contains its own owner fields.

The optional SUR-02 isolated-history checker remains outside the default suite:

```powershell
python tools/porting/check_sur02_units.py --output .runtime/porting/SUR-02/isolated-history-local
```

This is a historical Source59 check. It requires the matching tested Source
checkout, pinned EnergyPlus source, C++/Rust compilers and retained compiler-input
plan named in the checker. Fixture paths changed during repository cleanup, so
the checker does not certify this reorganized checkout. Use a fresh output
folder. The [accepted supplement](../../validation/evidence/SUR-02-isolated-history-impulse-61/summary.json)
retains the actual executed result.

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
