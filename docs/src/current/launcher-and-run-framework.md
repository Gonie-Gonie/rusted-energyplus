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

The current CLI already has `RunConfig`, run modes, trace levels, dry-run,
oracle baseline, oracle compare, JSON stdout, and exit codes. The target
contract still needs explicit partial-run policy, strict-output policy,
capability-driven run states, and golden artifact tests.

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

The launcher is a thin UI over the CLI. The current script already invokes
`eplus-rs run`, lets the user select input/weather/output/oracle paths, toggles
oracle compare, toggles overwrite, and opens output/report artifacts.

The target launcher should also collect mode, partial policy, oracle baseline,
trace level, and strict-output policy, then map those controls directly to
`RunConfig`.

The launcher must display stage progress, final run state, top diagnostics,
claim boundary, timing summary, and artifact links. It must not place
simulation logic in the GUI, silently fall back to oracle, or hide that the
project is not a drop-in EnergyPlus replacement.
