# Run Arbitrary IDF

`eplus-rs run <input>` is the ad-hoc run pipeline for IDF or epJSON files.
It stages the input, converts IDF through the locked EnergyPlus 26.1.0
`ConvertInputFormat.exe`, compiles the RawModel/TypedModel subset, writes a
support assessment, and only then runs a Rust diagnostic runtime when the input
is inside the declared boundary.

```powershell
eplus-rs run .\model.idf -w .\weather.epw -d .\out --oracle-baseline --compare-oracle
```

On Windows, double-click `eplus-rs-launch.cmd` to open the small button-based
launcher for the same pipeline.

Important boundaries:

- unsupported inputs do not fall back to EnergyPlus as the Rust result
- `--oracle-baseline` writes an EnergyPlus run under `out/oracle`
- `--compare-oracle` writes `out/compare/compare-summary.json` and
  `out/compare/compare-report.md`
- every run writes `eplusrs.err`, `diagnostics.json`,
  `support-assessment.json`, `support-report.md`, `run-summary.json`, and
  `reports/compatibility-boundary.md`
- ad-hoc runs keep `conformance_claim=false`; release conformance evidence
  still requires reviewed manifests and release gates

Exit codes follow the arbitrary-run contract: `0` success, `1` arguments,
`2` import/parse, `3` compile/reference, `4` unsupported, `5` plan,
`6` runtime, `7` output export, and `8` oracle compare.

The initial Rust runtime boundary is intentionally narrow. One-zone
heat-balance diagnostic runs are supported in compatibility mode. IdealLoads
no-OA sensible systems can run through the node-state projection path as
diagnostic-only evidence. Broad HVAC, plant, EMS, PythonPlugin,
AirflowNetwork, sizing workflows, fenestration/daylighting/shading, and broad
surface-boundary families remain outside the arbitrary runtime.

For a local end-to-end gate that exercises support assessment, Rust execution,
EnergyPlus oracle generation, compare reports, and the oracle-compare exit-code
contract, run:

```powershell
.\scripts\dev.cmd arbitrary-run-smoke
```
