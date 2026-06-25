---
status: active
claim_level: none
owner: core
last_reviewed: 2026-06-25
---

# P1-P4 Checklist Closure

This document closes the execution checklist scope summarized as P1-P4 from the
source-order reset and current-repo addendum notes. It is an implementation and
verification map, not a broad EnergyPlus compatibility claim. Compatibility
claims still require declared cases, variables or meters, tolerances, oracle
artifacts, Rust artifacts, generated reports, and blocking gates.

## Closure Rules

- No checklist row is closed by prose alone; each row points to code, specs,
  generated artifacts, or a blocking script.
- Diagnostic probes and ad-hoc arbitrary runs do not create conformance claims.
- Partial runs are explicit, diagnostic/ad-hoc only, and keep
  `conformance_claim=false`.
- Unsupported active semantics produce `run_blocked`; Rust runtime is not
  executed for blocked inputs.

## P1 Source-Order Runtime

| Checklist row | Closure evidence | Gate |
|---|---|---|
| runtime probe/compatibility separation | `crates/ep_runtime/src/heat_balance/algorithm.rs` defines compatibility selectors; `crates/ep_runtime/src/diagnostic_probes/heat_balance.rs` owns diagnostic selectors; compatibility modules do not call diagnostic probes. | `scripts/quality/heat-balance-structure-audit.ps1`, `scripts/quality/strict-no-false-conformance.ps1` |
| actual ExecutionPlan source-order stages | `crates/ep_runtime/src/execution_plan.rs` defines EnergyPlus heat-balance and IdealLoads source-order stage kinds, expected/actual source-order ID comparison, and mismatch detection; `crates/ep_run/src/pipeline.rs` fails before runtime when the plan does not match. | `cargo test -p ep_run`, `scripts/smoke/arbitrary-run-smoke.ps1` |
| heat_balance module split | `crates/ep_runtime/src/heat_balance/*` owns manager, surface manager, air manager, zone predictor/corrector, CTF, convection, radiation, reports, state, trace, warmup, and timestep responsibilities outside the runtime root. | `scripts/quality/heat-balance-structure-audit.ps1`, `scripts/quality/file-size-check.ps1` |
| Algorithm Port Ticket introduced and enforced | `specs/algorithm_port_ticket_template.toml`, `docs/src/porting-map/algorithm-port-ticket.md`, PR templates, and `.github/workflows/pull-request.yml` require compatibility algorithm-port metadata or explicit non-algorithm classification. | `scripts/quality/pr-port-ticket-check.ps1 -SelfTest`, `scripts/quality/algorithm-ledger-check.ps1` |

The P1 scope is closed at the runtime-contract level: compatibility lanes,
diagnostic lanes, source-order stage metadata, plan mismatch failure, and PR gate
rules are in code and scripts. This does not claim broad heat-balance parity.

## P2 Run Framework

| Checklist row | Closure evidence | Gate |
|---|---|---|
| support assessment capability registry connection | `specs/capabilities.toml` is embedded and loaded by `crates/ep_run/src/support_registry.rs`; `crates/ep_run/src/support.rs` reports matched capability IDs, unsupported rule IDs, partial rule IDs, and claim boundaries. | `cargo test -p ep_run`, `scripts/smoke/arbitrary-run-smoke.ps1` |
| runtime class differentiation | `RuntimeClass` separates one-zone heat balance compatibility/diagnostic, IdealLoads no-OA/finite/ConstantSHR/selected humidity/mixed compatibility, node projection diagnostic, and unsupported classes. Compatibility mode blocks diagnostic-only support unless diagnostic mode and explicit partial policy are selected. | `crates/ep_run/src/support/tests.rs`, `cargo test -p ep_run` |
| three run states | `RunResultState` maps support output to `run_blocked`, `partial_supported_run`, and `supported_compatibility_run`; blocked runs skip Rust runtime and partial runs force `conformance_claim=false`. | `crates/ep_run/tests/arbitrary_run.rs`, `crates/ep_run/tests/arbitrary_run_exit_codes.rs` |
| arbitrary-run integration tests | Supported one-zone, supported IdealLoads, unsupported PlantLoop/EMS, missing weather, invalid import, compile-reference failures, oracle baseline for blocked Rust runs, oracle compare for supported Rust runs, and partial supported output-request cases are covered. | `cargo test -p ep_run`, `scripts/smoke/arbitrary-run-smoke.ps1` |
| output directory golden tests | `crates/ep_run/tests/arbitrary_run/output_manifest.rs` asserts the supported and blocked artifact layouts, including `run-summary.json`, diagnostics, support assessment/report, execution plan, result store, selected outputs, oracle, and compare artifacts. | `cargo test -p ep_run`, `scripts/smoke/arbitrary-run-smoke.ps1` |

The P2 scope is closed for the current arbitrary-run contract: support
classification, runtime class selection, output layout, exit codes, oracle
handling, and partial-run behavior are all exercised by tests and smoke scripts.

## P3 Launcher

| Checklist row | Closure evidence | Gate |
|---|---|---|
| launcher entrypoint exists | `scripts/gui/eplus-rs-launch.ps1` is the Windows launcher script and `scripts/gui/build-launcher-exe.ps1` builds the wrapper. | `scripts/smoke/launcher-smoke.ps1` |
| launcher invokes `eplus-rs run` | `scripts/gui/eplus-rs-launch/core.ps1` builds the CLI command, maps mode/partial/output/trace/oracle/compare/overwrite controls to CLI arguments, and runs the CLI as a child process. | launcher self-test and smoke |
| three-state UI implementation | The launcher presentation maps `run-summary.json` to `run_blocked`, `partial_supported_run`, and `supported_compatibility_run`, including claim boundary and oracle/compare status. | `scripts/gui/eplus-rs-launch/self_test.ps1`, `scripts/smoke/launcher-smoke.ps1` |
| report and artifact opening | `scripts/gui/eplus-rs-launch/artifacts.ps1` resolves output folder, diagnostics, run report, support report, compare report, plot artifacts, logs, evidence summary, and evidence PDF paths. | `scripts/gui/eplus-rs-launch.ps1 -SelfTest`, `scripts/smoke/launcher-smoke.ps1` |
| no silent oracle fallback | The launcher exposes oracle baseline and compare status separately and does not treat oracle execution as Rust success for blocked runs. | `scripts/smoke/launcher-smoke.ps1` |

The P3 scope is closed for the first Windows launcher: it is a thin UI over the
CLI pipeline, preserves the run-state and claim-boundary language, and opens the
artifacts created by `ep_run` and release evidence tooling.

## P4 Evidence And PDF

| Checklist row | Closure evidence | Gate |
|---|---|---|
| external judgment PDF/evidence pack | `scripts/release/pdf-evidence-pack.ps1` builds the numeric conformance report, release evidence manifest, user coverage handbook, performance summary, stability summary, plot evidence, and evidence summary artifacts. | `scripts/dev.ps1 pdf-evidence-pack -Version 0.1.0 -Target windows-x64 -SkipPackage -TimingRepeats 1 -DynamicTimingRepeats 1` |
| arbitrary-run summary included | `tools/reporting/conformance_evidence_report.py` includes arbitrary-run summaries and labels ad-hoc run artifacts separately from release conformance evidence. | PDF evidence pack command, `strict-no-false-conformance` |
| plots from actual artifacts | `tools/reporting/plot_evidence.py` and release scripts generate plot PNG/JSON assets from collected result artifacts, not hand-written screenshots. | `scripts/release/plot-evidence.ps1`, PDF evidence pack command |
| performance/stability summary | `tools/reporting/performance_summary.py`, `tools/reporting/stability_summary.py`, and `scripts/quality/perf.ps1` generate/check current performance and stability JSON artifacts. | `scripts/dev.ps1 perf -Version 0.1.0`, PDF evidence pack command |
| manifest hashes | `tools/reporting/release_evidence_manifest.py` records required asset roles, paths, hashes, sizes, and missing-asset count. Release verify scripts assert required PDF/plot/perf/stability/manifest roles. | `scripts/release/v0.1-verify.ps1`, `scripts/release/v0.32-verify.ps1` |

The P4 scope is closed for release evidence packaging: the evidence pack is
built from actual artifacts, separates evidence classes, and fails if required
assets are missing.

## Verification Set

The stable verification set for this closure is:

```powershell
cargo fmt --all -- --check
cargo test -p ep_run
cargo test -p ep_runtime
.\scripts\dev.ps1 docs-check
.\scripts\dev.ps1 strict-no-false-conformance
.\scripts\dev.ps1 algorithm-ledger-check
.\scripts\dev.ps1 arbitrary-run-smoke
.\scripts\dev.ps1 launcher-smoke
.\scripts\dev.ps1 file-size-check
.\scripts\dev.ps1 pdf-evidence-pack -Version 0.1.0 -Target windows-x64 -SkipPackage -TimingRepeats 1 -DynamicTimingRepeats 1
```

Each command above is tied to a code path or artifact class used by P1-P4. The
current release claim boundary remains limited to declared cases and generated
evidence; domains outside those declared cases remain outside the claim rather
than partially promoted.