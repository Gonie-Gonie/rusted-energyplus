---
status: active
claim_level: none
owner: conformance
last_reviewed: 2026-10-20
---

# Conformance Contract

Conformance means Rust output is compared against EnergyPlus oracle output with
declared variables, declared tolerances, human-reviewable reports, and a
blocking gate.

Claims apply to declared cases, outputs, frequencies, and boundaries. Successful
execution, matching sample counts, input parsing, a trace, or a performance
measurement does not establish numerical compatibility. Diagnostic-only cases
keep `comparison_class = "diagnostic-only"` and `conformance_claim = false`.

Use the [current status](../current/current-status.md) for accepted claims and
the [case index](../generated/conformance-case-index.md) for tracked cases.
The completed CON-01 A/B port has its own
[fixed scope](../current/current-status.md#fixed-porting-scope) and
[validation assets](../../validation/README.md);
it does not establish general EnergyPlus compatibility or make every older
diagnostic case a standalone Rust simulation.

## Case and Output Contracts

Cases live in `data/conformance_cases/<case_id>/case.toml`. Start from an
existing reviewed manifest, such as
[schedule_constant_001](../../../data/conformance_cases/schedule_constant_001/case.toml).
The accepted fields are defined by
[ConformanceCase](../../../crates/ep_conformance/src/conformance.rs) and its
[output contracts](../../../crates/ep_conformance/src/conformance/output_contract.rs).

| Section | Required information |
|---|---|
| Case identity | ID, title, purpose, oracle version, comparison class, claim flag |
| `manifest_v2` | Schema marker, source kind/file, case tier |
| `scope` and optional `boundary` | Domains, active feature flags, exact subset and exclusions |
| `input` | IDF, weather where required, optional epJSON and auxiliary files |
| `outputs` / `meters` | Exact key/name, frequency, source artifact, class/domain, evidence level |
| `tolerances` | Declared numerical limits for the compared output classes |
| `report` / `gate` | Report format/path and executable blocking gate |
| `waivers` | Explicit known gaps or temporary exceptions, when present |

Output sources are `eso`, `eio`, `mtr`, `sql`, or `csv`; static EIO fields use
`frequency = "static"`. Evidence levels are `required`, `optional`, `baseline`,
`diagnostic`, and `conformance`. Only declared conformance outputs can support
a claim, and the enclosing case must have the corresponding evidence and
blocking gate. A report must not silently compare whichever variables happen
to be available.

Each conformance output needs an explicit numerical policy: units, frequency,
aggregation, absolute error and RMSE limits, relative error when applicable,
and the reason for those limits. Preserve timestamp order and uniqueness where
the case requires them; report alignment, sample counts, and first divergence.
Smoke defaults are not a release tolerance policy, and tolerances must not be
relaxed to hide a mismatch.

Case tiers control execution cost and promotion, independently of output level:

| Tier | Use | Default gate policy |
|---|---|---|
| A | Small deterministic cases suitable for local release checks | May block after evidence review |
| B | Nightly or scheduled coverage and diagnostics | Nonblocking until explicitly promoted |
| C | Broad or complex system exploration | Baseline-only or diagnostic-only |

A larger IDF does not create a stronger claim. Promotion needs stable inputs,
declared outputs, oracle and Rust artifacts, explicit unsupported features,
tolerances, reports, and a reviewed release/CI gate.

## Oracle and Comparison Workflow

The pinned oracle is EnergyPlus 26.1.0. Repository-local locations are
`.runtime/energyplus/26.1.0` for executables and
`.reference/energyplus-src/26.1.0` for source interpretation. Record the oracle
version in each case; do not silently switch to a globally installed executable.
Reference source and oracle results are validation inputs, not production Rust
simulation results.

For a case, copy the input before injecting required `Output:Variable`,
`Output:Meter`, `Output:SQLite`, and file-output controls. Keep a patch report
listing added and already-present requests. Run both engines with the declared
input, weather, initial-state and reporting contracts; verify requested output
availability before comparison.

Oracle-only baseline generation remains available through:

```powershell
cargo run -p ep_cli -- conformance baseline data\conformance_cases\schedule_constant_001\case.toml .runtime\energyplus\26.1.0 .runtime\conformance-baseline\26.1.0
```

The baseline includes staged input, converted epJSON, ERR, requested oracle
artifacts, and `case-expanded.toml`. An oracle baseline alone makes no Rust
conformance claim. The Source port's normal run does not currently support
`--compare-oracle`; see the [run guide](../guides/run-arbitrary-idf.md) for its
execution boundary.

The minimum comparison bundle is:

```text
case.toml
output requests
tolerance policy
EnergyPlus baseline artifacts
Rust result artifacts
compare-summary.json
compare-report.md
blocking script or CI gate
```

The machine summary and human report must identify the case, input/weather,
oracle version, Rust source revision, comparison class, claim flag, requested
outputs, tolerances, stage status, unsupported/skipped outputs, and gate
decision. Per-series results include sample count, maximum absolute error,
RMSE, relative error where applicable, and first divergence. Reports must
retain failed or unavailable results explicitly. Matching selected numeric
series does not imply byte-identical files, warning parity, or parity for
unrequested values.

The detailed report layout is in [Report Format](report-format.md).
[Numeric Release Evidence](numeric-release-evidence.md),
[Support Coverage Report](support-coverage-report.md),
[User Coverage Handbook](user-coverage-handbook.md), and the
[Release Evidence Manifest](release-evidence-manifest.md) describe the retained
release artifacts.

## Validation and Release Promotion

Validate the manifests and claim boundaries with:

```powershell
.\scripts\dev.cmd manifest-validate-all
.\scripts\dev.cmd strict-no-false-conformance
```

Before a compatibility claim appears in release notes, verify the declared
oracle and evidence level, reviewed manifests, requested variables/meters,
tolerances, actual oracle and Rust artifacts, summary/report, release evidence,
unsupported scope, and blocking script or CI gate. Missing evidence permits an
engineering-progress report, not a numerical compatibility claim.

Authoritative registries are
[claim rules](../../../specs/claim_rules.toml),
[evidence levels](../../../specs/evidence_levels.toml),
[object coverage](../../../specs/object_coverage.toml),
[variable coverage](../../../specs/variable_coverage.toml), and the
[algorithm ledger](../../../specs/algorithm_ledger.toml). Their generated
[object](../generated/object-coverage.md),
[variable](../generated/variable-coverage.md), and
[algorithm](../generated/algorithm-ledger.md) pages replace hand-maintained
coverage tables. Parsing, typing, simulation, output comparison, and trace
comparison remain separate evidence stages.
