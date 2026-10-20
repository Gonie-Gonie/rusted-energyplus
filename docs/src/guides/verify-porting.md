---
status: active
claim_level: none
owner: qa
last_reviewed: 2026-10-20
---

# Verify The Port

The fixed A/B cases can run independently through EnergyPlus and Rust and be
compared from files. This developer procedure uses existing commands; the
single-command Source `--compare-oracle` integration is still outstanding.

The [accepted fifteen-case validation](../../../energyplus_porting_plan/evidence/SRC-06-final-current-source-validation-60/summary.json)
records the current numerical result. Comparison means the declared reported
values meet their fixed per-variable absolute/RMSE policy. Whole-file byte
identity, all ancillary files and arbitrary new IDFs are separate claims.

## Prerequisites

Use a normal checkout with Git history, the pinned Rust toolchain, and
EnergyPlus 26.1.0 installed by [Setup](setup.md). The checker verifies the
historical tolerance policy from Git. The setup's local Python is used below.
No special Cargo feature or fixture-demand mode is required.

The example is A-24H. Start with a fresh run folder; do not overwrite accepted
evidence. Other fixed IDs and their scope/input/weather hashes are listed in
[scope.json](../../../energyplus_porting_plan/contracts/scope.json).

```powershell
$py = (Resolve-Path .runtime/python/3.11.9/python.exe).Path
$oracle = (Resolve-Path .runtime/energyplus/26.1.0).Path
$idf = (Resolve-Path energyplus_porting_plan/cases/A-24H/input.idf).Path
$weather = (Resolve-Path "$oracle/WeatherData/USA_CO_Golden-NREL.724666_TMY3.epw").Path
$caseId = "A-24H"
$scope = "A"
$runRoot = ".runtime/porting/SRC-06/repro-A24"
$outBase = Join-Path (Get-Location).Path $runRoot
```

## Run Each Engine With The Same Inputs

`record_command.py` executes the command and records its exit code, input
command, Git state, source snapshot and stdout/stderr. Receipts are provenance;
they do not supply reference values to Rust.

```powershell
& $py tools/porting/record_command.py --output-dir "$runRoot/native-command" -- "$oracle/energyplus.exe" --weather $weather --output-directory "$outBase/native" --output-prefix energyplus $idf
if ($LASTEXITCODE -ne 0) { throw "Oracle run failed" }

& $py tools/porting/record_command.py --output-dir "$runRoot/source-command" -- cargo run --locked -p ep_cli --bin eplus-rs -- run $idf --weather $weather --output-dir "$outBase/rust" --oracle-root $oracle --porting-scope $scope --mode compatibility --partial deny
if ($LASTEXITCODE -ne 0) { throw "Rust run failed" }
```

Do not request Oracle comparison on the Rust command. The separate Oracle run
above preserves the same IDF's output requests and weather. Default tracing is
sufficient for reported values; request detailed traces only for a state issue.

## Compare And Inspect Files

```powershell
$sourceReceipt = "$runRoot/source-command/receipt.json"
$nativeReceipt = "$runRoot/native-command/receipt.json"
$sourceSha = (Get-FileHash -LiteralPath $sourceReceipt -Algorithm SHA256).Hash.ToLowerInvariant()
$nativeSha = (Get-FileHash -LiteralPath $nativeReceipt -Algorithm SHA256).Hash.ToLowerInvariant()

& $py tools/porting/check_completed_reported_streams.py --case $caseId --source-receipt $sourceReceipt --source-receipt-sha256 $sourceSha --native-receipt $nativeReceipt --native-receipt-sha256 $nativeSha --output "$runRoot/comparison"
if ($LASTEXITCODE -ne 0) { throw "Reported-value comparison failed" }
```

The checker validates the fixed case/input hashes, successful receipts, full
reported dictionaries, IDs, timestamps/intervals, counts and values under the
unchanged [tolerance policy](../../../energyplus_porting_plan/contracts/output-tolerances.json).
A different IDF cannot be relabeled as an accepted fixed case.

| File under the run folder | Inspect |
|---|---|
| `native/energyplusout.eso`, `energyplusout.mtr` | Oracle physical streams |
| `rust/eplusrs.eso`, `eplusrs.mtr` | Rust physical streams |
| `rust/results/source-reported-results.json` | Rust reported records |
| `rust/results/source-eso.csv`, `source-mtr.csv` | Rust CSV records |
| `comparison/comparison.json` | Acceptance, series/sample counts and per-series error metrics |
| `comparison/all-tolerance-failures.jsonl` | Every tolerance failure; empty is expected on a passing run |

Require the checker's successful exit and `accepted: true`, with zero tolerance
and RMSE failures. Do not infer success from matching final temperatures,
matching file sizes, or a successful simulation exit alone. Headers and logs
may differ even when all declared reported values pass.
