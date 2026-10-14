---
status: active
claim_level: none
owner: docs
last_reviewed: 2026-10-14
---

# Developer Workflow

Use `scripts/dev.cmd` as the single entry point:

```powershell
.\scripts\dev.cmd list
.\scripts\dev.cmd check -Package ep_runtime -Filter surface_environment -Lib
.\scripts\dev.cmd test -Package ep_compare -Filter series_v2 -Lib
.\scripts\dev.cmd check -Full -Plan
```

`check` defaults to the `ep_runtime` package: formatting, package lint, and one
package test invocation. Select the package and test-name substring for the
area being changed. `-Package ep_runtime,ep_model` selects multiple packages;
`-Lib` selects library tests. `-Plan` prints commands without running them,
and `-List` lists matching test names. A filtered run must execute at least one test; zero matches and ignored-only matches fail.

At integration milestones, run `.\scripts\dev.cmd check -Full` for workspace
lint/tests and the broader conformance, documentation, and claim guards.
`-Full` cannot be combined with partial package or test selectors.
`.\scripts\dev.cmd test` runs workspace tests once. For docs-only work, use
`docs-generate`, `docs-check`, and the relevant spec validator.

For input and execution-plan inspection, the CLI also provides:

```powershell
cargo run -p ep_cli -- model inspect <input.epJSON>
cargo run -p ep_cli -- model compile <input.epJSON>
cargo run -p ep_cli -- model plan <input.epJSON>
```

These commands inspect input and planning state; they do not establish
numerical compatibility.

For geometry changes, add `--trace-level surface` to the usual `run` command.
It writes `compiled-geometry.json` and `geometry-consumers.json`, including
retained Source owners when that route is reached. Use `full` when schedule,
weather, psychrometric, and other detailed event traces are required. Compare
the fields declared by the relevant porting card; tracing alone does not
certify numerical compatibility.

When changing plans, claim boundaries, object coverage, variable coverage, or
algorithm status, update the matching file under `specs/` and regenerate the
generated docs.

When adding a conformance case, update `data/conformance_cases/<case>/case.toml`
and run:

```powershell
.\scripts\dev.cmd manifest-validate-all
.\scripts\dev.cmd strict-no-false-conformance
```

Do not add broad compatibility wording unless a generated report and blocking
gate support the exact case, variable or meter, and tolerance.
