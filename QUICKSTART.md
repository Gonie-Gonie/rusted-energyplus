# Windows quick start

Extract the entire ZIP to a writable folder. Keep `bin/`, `oracle/`, `examples/`
and `licenses/` together. Rust and a separate EnergyPlus installation are not
required for the packaged CLI. Use Windows PowerShell or PowerShell.

## Run the included example

Open a terminal in the extracted folder:

```powershell
.\bin\eplus-rs.exe --version
.\bin\eplus-rs.exe run .\examples\source-a.idf --porting-scope A --weather .\examples\weather.epw --output-dir .\out\source-a
```

The example is the retained opaque A-24H case. The package includes its weather
file and the locked EnergyPlus 26.1.0 IDF converter. Heat-balance calculations
execute in Rust. Package smoke verification is separate from retained numerical
comparison evidence.

## Check your input before running

```powershell
.\bin\eplus-rs.exe run .\model.idf --porting-scope A --weather .\weather.epw --dry-run --output-dir .\out\check-model
```

Select B for a bounded direct-Zone IdealLoads model. Direct `.epJSON` input
skips IDF conversion. It retains the same bounded simulation scope, with
format-specific input rules.

| Output file | Purpose |
| --- | --- |
| `support-report.md`, `support-assessment.json` | Admitted and blocked semantics. |
| `diagnostics.json` | Blocking reasons and warnings. |
| `run-summary.json` | Actual status, runtime kind and artifact locations. |

Resolve every blocker, remove `--dry-run` and choose a fresh output directory.
Supply the appropriate EPW for weather-period runs. Keep referenced schedule
and other files beside the input as required.

Source writes `eplusrs.eso`, `eplusrs.mtr` and inspectable files under `results/`,
as listed in the summary. A dry run is only preflight. Ad-hoc simulations retain
`conformance_claim=false`.

Do not add `--oracle-baseline` or `--compare-oracle` to Source A/B: that
integration is not connected. Use the
[separate-run verification guide](docs/src/guides/verify-porting.md).

## Scope and integrity

Read [SUPPORT.md](SUPPORT.md) before using a new model. Unsupported features
do not silently fall back to reference-engine results. The optional launcher
uses the older generic route; the CLI above selects Source explicitly.

`SHA256SUMS` lists package-file hashes. The release also supplies the ZIP's
SHA256 checksum and a package manifest. Licenses and notices accompany the files.
