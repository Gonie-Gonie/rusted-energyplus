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

The [accepted fifteen-case validation](../../validation/evidence/SRC-06-final-current-source-validation-60/summary.json)
records the accepted numerical baseline. Comparison means the declared reported
values meet their fixed per-variable absolute/RMSE policy. Whole-file byte
identity, all ancillary files and arbitrary new IDFs are separate claims.

## Prerequisites

Use a normal checkout with Git history, the pinned Rust toolchain, and
EnergyPlus 26.1.0 installed by [Setup](setup.md). The checker verifies the
historical tolerance policy from Git. The setup's local Python is used below.
No special Cargo feature or fixture-demand mode is required.

The example is A-24H. Start with a fresh run folder; do not overwrite accepted
evidence. Other fixed IDs and their scope/input/weather hashes are listed in
[scope.json](../../validation/contracts/scope.json).
Retained JSON/ZIP provenance keeps historical paths and hashes; current asset
locations are described in the [validation README](../../validation/README.md).

```powershell
$py = (Resolve-Path .runtime/python/3.11.9/python.exe).Path
$oracle = (Resolve-Path .runtime/energyplus/26.1.0).Path
$idf = (Resolve-Path docs/validation/cases/A-24H/input.idf).Path
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
unchanged [tolerance policy](../../validation/contracts/output-tolerances.json).
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


## Reproduce The Composed Input Example

[`COMPOSED-B102-LEAP12`](../../../data/examples/source-input-domains/composed-b102.json)
is one tested integration example with its own identity, separate from the 15
fixed CON-01 cases. It combines a 2016 leap day, 12 steps/hour, global Simple/DOE2
and local CeilingDiffuser/MoWiTT, a Ground floor, an interior AirGap, Relative
geometry and opaque FullInteriorAndExterior. Its original B102 request bytes and
per-variable tolerances are retained: 102 series * 288 intervals = 29,376 values.
All surfaces retain NoSun/NoWind and there are no windows. Exact earlier
convection-call ACH/branch/cap counts are unmeasured; this example does not
establish arbitrary valid combinations or full-object support.

The stdlib preparer derives controlled synthetic weather from the pinned Golden
TMY3 file (SHA256 `c184b947cd34d41c6d6474d63d66dbb82bc0e6cae4c888edcf837348282bce7f`).
It changes the years to 2016, copies February 28's 24 physical records into
February 29, enables leap years and qualifies the DATA PERIODS calendar. The
result must hash to `134042fbd42aa79d0886d45d852d9a5a60d53b70ffa34b2485ac8f350bc92059`.
It is not measured actual-year weather. No derived EPW or prior run output is
required from another developer's checkout.

Use the Python and EnergyPlus 26.1 paths from the setup above. Choose a fresh run
folder, then keep the source, executable, input, weather and all run files
immutable until comparison finishes. The recorder captures commands, exits and
source snapshots; it does not hash simulation inputs/executables at the instant
of launch. The checker authenticates the supplied receipts and current bytes,
and links the Source command to the emitted Cargo executable and source snapshot.
It does not launch or claim fresh simulations itself.

```powershell
$exampleRun = ".runtime/porting/examples/composed-b102-01"
$exampleOut = Join-Path (Get-Location).Path $exampleRun
$exampleIdf = (Resolve-Path "data/examples/source-input-domains/composed-b102.idf").Path
$exampleWeather = "$exampleOut/controlled-synthetic-2016.epw"
& $py tools/porting/source_input_example.py prepare-weather --golden "$oracle/WeatherData/USA_CO_Golden-NREL.724666_TMY3.epw" --output $exampleWeather
if ($LASTEXITCODE -ne 0) { throw "Weather preparation failed" }

# Remove optional developer clock/surface overrides before either engine runs.
Get-ChildItem Env: | Where-Object { $_.Name -match 'SURFACE_CADENCE|^CI_FORCE_TIME_STEP$' } | ForEach-Object { Remove-Item -LiteralPath ("Env:" + $_.Name) }
& $py tools/porting/record_command.py --output-dir "$exampleRun/build-command" -- cargo build --locked -p ep_cli --bin eplus-rs --message-format=json-render-diagnostics
if ($LASTEXITCODE -ne 0) { throw "Build failed" }
$exampleExe = (Resolve-Path "target/debug/eplus-rs.exe").Path
& $py tools/porting/record_command.py --output-dir "$exampleRun/native-command" -- "$oracle/energyplus.exe" --weather $exampleWeather --output-directory "$exampleOut/native" --output-prefix composed $exampleIdf
if ($LASTEXITCODE -ne 0) { throw "Oracle example failed" }
& $py tools/porting/record_command.py --output-dir "$exampleRun/source-command" -- $exampleExe run $exampleIdf --weather $exampleWeather --output-dir "$exampleOut/source" --mode compatibility --partial deny --porting-scope B
if ($LASTEXITCODE -ne 0) { throw "Source example failed" }

$exampleNative = "$exampleRun/native-command/receipt.json"
$exampleSource = "$exampleRun/source-command/receipt.json"
$exampleBuild = "$exampleRun/build-command/receipt.json"
$exampleNativeSha = (Get-FileHash -LiteralPath $exampleNative -Algorithm SHA256).Hash.ToLowerInvariant()
$exampleSourceSha = (Get-FileHash -LiteralPath $exampleSource -Algorithm SHA256).Hash.ToLowerInvariant()
$exampleBuildSha = (Get-FileHash -LiteralPath $exampleBuild -Algorithm SHA256).Hash.ToLowerInvariant()
& $py tools/porting/source_input_example.py check --native-receipt $exampleNative --native-sha256 $exampleNativeSha --source-receipt $exampleSource --source-sha256 $exampleSourceSha --build-receipt $exampleBuild --build-sha256 $exampleBuildSha --output-dir "$exampleRun/comparison"
if ($LASTEXITCODE -ne 0) { throw "Composed physical-stream comparison failed" }
```

Inspect `comparison/comparison.json`: require `accepted: true`, `series: 102`,
`numeric_records: 29376`, and zero `totals.tolerance_failures` and
`totals.rmse_failures`. All physical ESO dictionaries, report IDs/types, intervals,
values, footers and EOF pass through the unchanged fixed checker kernel and B
policy. Native filenames come from its recorded prefix/suffix; Source filenames
come from its actual retained stream owners. The unrequested MTR must be absent
on both sides with Source's genuine deleted-empty owner. The checker also requires
the leap-day clocks, effective EIO controls, matching opaque SHD, clean ERR/END and
288 exact 18 C Ground-floor endpoint values. Unrequested DXF and detailed CTF
reports remain absent. A nonzero raw-operand difference count can pass only
within the original tolerance and RMSE limits; it is not bit equality.

For archived results, `--source-executable <immutable-copy>` accepts only bytes
matching the executable SHA emitted by that recorded build. Retain the original
receipts and their referenced source objects/stdout/stderr alongside the outputs.
