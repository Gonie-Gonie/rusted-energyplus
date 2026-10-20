# Contributing

## Working Rule

Work in meaningful, reviewable units.

For this repository, that means:

- keep each change aligned to one setup, architecture, feature, test, or docs goal
- run the relevant local script before committing
- commit after each meaningful unit is complete
- push after committing when the remote accepts the update
- do not mix unrelated refactors into compatibility or oracle changes
- update docs, porting maps, and supported object coverage when behavior changes

## Local Gates

Use the existing command wrapper to select the checks needed by the change:

```powershell
.\scripts\dev.cmd check -Package ep_runtime -Filter surface_environment -Lib
.\scripts\dev.cmd test -Package ep_run
.\scripts\dev.cmd test
.\scripts\dev.cmd check -Full
```

The first two commands are examples for iteration; choose the affected area.
Run workspace tests once when integrating a reviewable unit. Run `check -Full`
at a broader integration milestone, when its oracle and tooling prerequisites
are available. It includes the repository's comparison and documentation
checks. Packaging and historical release verification are separate operations.
`-Plan` shows the exact scope without starting Cargo or a comparison.

## Test Selection

| Change | During implementation | Before integration |
| --- | --- | --- |
| Runtime equation, branch or state update | `check -Package ep_runtime -Filter <module_or_test> -Lib` | Workspace tests once; the affected card's same-input Native comparison and connected execution |
| Controlled zone air prediction or correction | Dedicated Native cohort command below | Same cohort once and the affected production case |
| IDF/epJSON parsing or compilation | `check -Package ep_raw_model,ep_compiler -Filter <area> -Lib` | Workspace tests, including affected model/run consumers |
| Run pipeline or CLI | `check -Package ep_run,ep_cli -Filter <area>` | Workspace tests; the affected end-to-end scenario |
| Shared model, public API or Cargo configuration | Relevant package checks | Workspace Clippy and tests once |
| Validation inputs, contracts or evidence | Preserve authenticated bytes; run the affected comparer or input checker | Affected documentation/spec checks; see `docs/validation/README.md` |
| Test runner or PowerShell script | Parse scripts; inspect `-Plan` and invalid selector combinations | Run the affected real command and relevant script/spec checks |

Package lists use commas through `dev.cmd`. A filter that selects no tests is
an error. Listing tests with `-List` does not count as executing them. Inspect
the printed scope before relying on a quick check; a package test does not
exercise every consumer of its public API.

The Native comparison cohort uses one dedicated process-main target:

```powershell
cargo test --locked -p ep_runtime --features native-controlled-air-cohort --test native_controlled_air_cohort
```

The default runs all 1,168 predictor entries, 1,168 whole-corrector entries
(810 positive inlet-flow cases), and 471 system-report cases. For changes to
energy or ventilation reporting and its consumed state, select reports only:

```powershell
cargo test --locked -p ep_runtime --features native-controlled-air-cohort --test native_controlled_air_cohort -- --report-only
```

This runs 306 engine-origin report calls and 165 separate full-Native-function
unit input variants, including nonzero loads and adjacent binary64 boundaries.
It skips the unchanged predictor/corrector cases. Run the full target when those
calculations or their consumed state change, and before closing integration gates.
On Windows GNU, process main preserves the Native comparison context without
changing floating-point controls or assertions. Without its explicit required
feature, ordinary library tests exclude these large cohort fixtures.

Keep one behavioral regression for each distinct failure or contract. Cover
parallel input cases in a named table where their setup and assertions match.
Retain separate tests for different state transitions, error prefixes, branch
boundaries, signed zero/nonfinite behavior and caller integration. Do not add
tests solely to copy an ID, count source files, enforce a historical line cap,
or mirror a private implementation. Repository structure belongs to the
existing structure and file-size checks.

Focused checks provide feedback during editing. A successful workspace suite
already includes those unit tests; do not run them again for the same source.
Repeat validation only after a relevant change, a failure, or a new unresolved
concern. Record the actual command and scope with each porting result. Card
completion still requires its scope, unit, integration and production gates;
test counts or a successful build do not replace that evidence.

Development and test builds share the workspace's debug-free, non-incremental
profile, avoiding a second cache created by command-specific profile overrides.
An explicit debugging session can override it with Cargo's `--config` options.

## Commit Message Shape

Prefer concise imperative messages:

```text
Initialize reproducible setup skeleton
Add oracle smoke script
Document Rust-only architecture policy
```
