---
status: active
claim_level: none
owner: docs
last_reviewed: 2026-10-20
---

# Run An IDF

After [setup](setup.md), run from the repository root:

```powershell
cargo run --locked -p ep_cli --bin eplus-rs -- run .\model.idf --weather .\weather.epw --output-dir .\out
```

An installed release can use `eplus-rs` directly. IDF import uses the locked
EnergyPlus 26.1.0 converter. The pipeline compiles the supported model,
assesses active capabilities, checks source order, then runs Rust. Unsupported
semantics do not fall back to Oracle results.

## Select The Completed Porting Route

The fixed CON-01 cases use `--porting-scope A` (opaque unconditioned one-zone)
or `B` (direct no-OA sensible IdealLoads, no limit or hard-sized limits).
Use the checked-in case inputs and the [separate-run comparison procedure](verify-porting.md).
These fixed cases do not establish arbitrary-model or general HVAC support.

**Do not add `--oracle-baseline` or `--compare-oracle` to this Source route.**
Its reported-result identity/timestamp comparison is not connected to those
options. Rust files remain available, but the command returns
`SourceReportedOracleComparisonUnsupported` (exit 4), before Oracle generation.

The older ResultStore route supports those options and writes `oracle/` plus
`compare/compare-summary.json` and `compare/compare-report.md`. That comparison
is a separate path; it does not verify the completed Source port. For blocked
legacy inputs, an explicitly requested Oracle baseline can exist while
`rust_runtime` remains null.

## Read The Result

| Artifact | Purpose |
|---|---|
| `run-summary.json` | Status, exit code, runtime kind, timing and actual artifact paths |
| `support-assessment.json`, `support-report.md` | Supported and blocked input semantics |
| `diagnostics.json`, `eplusrs.err` | Diagnostics |
| `model/execution-plan.json` | Plan/preflight stage metadata; Source caller tracing is separate |
| `reports/compatibility-boundary.md` | Scope and claim limits |
| Source: `eplusrs.eso`, `eplusrs.mtr` | Physical reported streams |
| Source: `results/source-reported-results.json`, `source-eso.csv`, `source-mtr.csv` | Inspectable own JSON/CSV results; CSVs are under `results/` |
| Legacy: ResultStore and selected-output files | Older runtime exports, as listed in the summary |

Run states are `run_blocked`, `partial_supported_run`, and
`supported_compatibility_run`. A partial run needs `--mode diagnostic
--partial allow`; it is diagnostic only. All ad-hoc runs keep
`conformance_claim=false`. `--dry-run` stops before runtime/Oracle/comparison.
Heat-balance execution requires weather; a source-order mismatch stops before
runtime. Full IdealLoads humidity/OA, sizing, Plant, EMS, AFN and fenestration
remain outside the fixed port's scope.

Exit codes: 0 success; 1 arguments; 2 import/parse; 3 compile/reference;
4 unsupported; 5 execution plan; 6 runtime; 7 export; 8 Oracle comparison.

## Windows Launcher

Use `.\scripts\dev.cmd launch-ui`, or double-click `eplus-rs-launch.exe` in a
release package. The launcher selects input/weather/output/Oracle paths,
mode, partial policy, format, trace level and overwrite settings, then starts
the same CLI. It displays run state, diagnostics, timing, claim boundary and
artifact links, and remembers settings in the user's application data.
It has no separate simulation or silent Oracle fallback. Oracle checkboxes
have the same Source-route limitation described above.

`arbitrary-run-smoke` and `launcher-smoke` check workflow and UI contracts;
they do not replace the fixed-case numerical comparison.
