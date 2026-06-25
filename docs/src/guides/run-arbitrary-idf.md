# Run Arbitrary IDF

`eplus-rs run <input>` is the ad-hoc run pipeline for IDF or epJSON files.
It stages the input, converts IDF through the locked EnergyPlus 26.1.0
`ConvertInputFormat.exe`, compiles the RawModel/TypedModel subset, writes a
support assessment, maps the result to `run_blocked`,
`partial_supported_run`, or `supported_compatibility_run`, and only then runs
Rust when the selected mode and partial policy permit it.
Runtime execution also passes the `ExecutionPlan` source-order gate: expected
and actual source-order stages are written to `model/execution-plan.json` and
`run-summary.json`, and a mismatch exits with code `5` before the runtime runs.

```powershell
eplus-rs run .\model.idf -w .\weather.epw -d .\out --oracle-baseline --compare-oracle
```

On Windows release packages, double-click `eplus-rs-launch.exe` to open the
small button-based launcher for the same pipeline. From a source checkout, use
`.\scripts\dev.cmd launch-ui` or `.\scripts\dev.cmd build-launcher-exe`.

Important boundaries:

- unsupported inputs do not fall back to EnergyPlus as the Rust result
- heat-balance compatibility runs require a weather EPW; a missing weather path
  exits with `args` before Rust runtime execution
- `partial_supported_run` requires `--mode diagnostic --partial allow`; it is
  diagnostic/ad-hoc output and never release conformance evidence
- `--oracle-baseline` writes an EnergyPlus run under `out/oracle`
- `--oracle-baseline` can still generate `out/oracle` for a `run_blocked`
  input; `rust_runtime` remains null and no EnergyPlus output is reported as a
  Rust result
- `--compare-oracle` writes `out/compare/compare-summary.json` and
  `out/compare/compare-report.md`
- `--dry-run` stops after support assessment and plan artifacts; runtime,
  oracle, and compare are reported as skipped
- every run writes `eplusrs.err`, `diagnostics.json`,
  `support-assessment.json`, `support-report.md`, `run-summary.json`, and
  `reports/compatibility-boundary.md`
- ad-hoc runs keep `conformance_claim=false`; release conformance evidence
  still requires reviewed manifests and release gates

Exit codes follow the arbitrary-run contract: `0` success, `1` arguments,
`2` import/parse, `3` compile/reference, `4` unsupported, `5` plan,
`6` runtime, `7` output export, and `8` oracle compare.

The initial Rust runtime boundary is intentionally narrow. One-zone
heat-balance runs and declared IdealLoads no-OA sensible, numeric finite-limit,
ConstantSensibleHeatRatio, selected no-OA humidity-control, and mixed declared
no-OA PurchasedAir branches are the current `supported_compatibility_run`
arbitrary-run paths. The IdealLoads path executes through the typed
ZoneEquipmentManager -> PurchasedAirManager source-order wrapper; its plan
records `SimPurchasedAir`, `GetPurchasedAir`, `InitPurchasedAir`,
`CalcPurchAirLoads`, `UpdatePurchasedAir`, and `ReportPurchasedAir` barriers
before runtime output export. It writes IdealLoads output variables and
supply-node output variables, and still keeps `conformance_claim=false` for
ad-hoc runs. Broad HVAC, plant, EMS, PythonPlugin, AirflowNetwork, sizing
workflows, fenestration/daylighting/shading, broad IdealLoads humidity,
outdoor-air/economizer/heat-recovery branches, and broad surface-boundary
families remain outside the arbitrary runtime.

For a local end-to-end gate that exercises support assessment, Rust execution,
EnergyPlus oracle generation, compare reports, and the oracle-compare exit-code
contract, run:

```powershell
.\scripts\dev.cmd arbitrary-run-smoke
```
