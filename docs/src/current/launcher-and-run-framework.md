---
status: active
claim_level: none
owner: core
last_reviewed: 2026-06-23
---

# Launcher And Run Framework

The arbitrary run framework lets a user provide an IDF or epJSON plus optional
EPW and output directory:

```powershell
eplus-rs run input.idf -w weather.epw -d output
```

The current CLI has `RunConfig`, run modes, partial-run policy, trace levels,
dry-run, oracle baseline, oracle compare, JSON stdout, exit codes,
capability-driven run states, and golden artifact tests for the supported and
blocked arbitrary-run layouts.

## Workflow

```text
InputResolver
  -> IDF to epJSON conversion when needed
  -> RawModel
  -> TypedModel
  -> SimulationModel
  -> ModelGraph
  -> SupportAssessment
  -> ExecutionPlan
  -> Runtime
  -> ResultStore
  -> OutputExport
  -> OracleCompare optional
  -> RunSummary
```

Support assessment is internal. Users do not run a separate command, but every
run writes `support-assessment.json` and `support-report.md`.

## Run States

- `run_blocked`: unsupported active objects or calculations affect required
  semantics and no safe partial rule exists. Rust runtime is not executed.
- `partial_supported_run`: unsupported objects are inactive or explicitly
  ignorable, the user allowed partial execution, and the result is diagnostic
  ad-hoc output only.
- `supported_compatibility_run`: all active objects and algorithms match
  declared capabilities, execution plan builds, runtime completes, and outputs
  are exported.

Partial runs never set `conformance_claim=true`. Ignored objects and claim
boundaries must be listed in the artifacts.

## Artifacts

Every run writes:

```text
run-summary.json
eplusrs.err
diagnostics.json
support-assessment.json
support-report.md
reports/compatibility-boundary.md
```

Completed Rust runs also write result-store and selected-output artifacts.
Oracle and compare artifacts are written only when their options are enabled.

## Launcher

The Windows launcher is a thin UI over the CLI. The script invokes
`eplus-rs run` as a child process and keeps simulation behavior in `ep_run`.
It lets the user select input, weather, output, oracle, and CLI paths, then maps
mode, partial policy, output format, trace level, strict warning failure,
oracle baseline, oracle compare, and overwrite controls directly to CLI
arguments.

The launcher displays the final run state, claim boundary, oracle status,
compare status, stage timing summary from `run-summary.json`, top diagnostics
from `diagnostics.json`, and result tabs for summary, diagnostics, support
report, selected results, oracle compare, plot artifacts, and logs. It also provides artifact
links for the output folder, diagnostics, run report, support report, and
compare report. It saves the last input,
weather, output, oracle, CLI, mode, partial, format, trace, warning, oracle,
compare, and overwrite selections in the user's application-data settings.
Oracle compare enables oracle baseline explicitly; the GUI does not silently
fall back to oracle or contain independent simulation logic, and it shows that
Rusted EnergyPlus is not a drop-in replacement for EnergyPlus.
