# rusted-energyplus

A Rust simulation engine for a **bounded subset of EnergyPlus 26.1.0** inputs.
Version 0.2.0 is a limited-scope prerelease. It does not run every EnergyPlus
model or every valid combination of a recognized object.

## Start here

- [Download a Windows package](https://github.com/Gonie-Gonie/rusted-energyplus/releases)
- [Quick start](QUICKSTART.md): run the example and check your own input
- [Supported inputs](docs/src/guides/supported-inputs.md): routes, limits and exclusions
- [Complete object inventory](docs/src/generated/example-object-union.md): 110 types found in examples and 13 additional reviewed types
- [Verify results](docs/src/guides/verify-porting.md): independent runs and comparison files

After extracting the Windows package, run from its folder:

```powershell
.\bin\eplus-rs.exe run .\examples\source-a.idf --porting-scope A --weather .\examples\weather.epw --output-dir .\out\source-a
```

For your model, first use `--dry-run` with a fresh output directory and read
`support-report.md`, `support-assessment.json` and `diagnostics.json`.
Passing input checks does not establish numerical agreement for a new model.

## Execution routes

| Route | Scope |
| --- | --- |
| `--porting-scope A` | Bounded opaque, unconditioned heat balance, including reviewed geometry and gains extensions. |
| `--porting-scope B` | Bounded direct-Zone IdealLoads, with selected hard limits, humidity and outdoor-air combinations. |
| No `--porting-scope` | Older generic/diagnostic route; its results and comparison options have separate boundaries. |

The Windows launcher uses the older generic route. Use the CLI to select A/B.
Active windows, general HVAC/Plant, sizing, EMS, PythonPlugin and AirflowNetwork
are outside ordinary Source release scope. Field and composition limits are
in the [object registry](docs/src/generated/object-coverage.md).

## Validation boundary

The retained fixed CON-01 suite has 15 cases and 16,427,880 reported values.
The separate published input-domain archive has 284 paired cases and 13,276,216
values through Source212. Both pass declared tolerances; these totals are
distinct and are not added together. They do not certify all new models,
whole-file byte identity or complete EnergyPlus compatibility.

Compatibility mode preserves source-order algorithms against the locked
EnergyPlus 26.1.0 reference. Numerical claims require case manifests, declared
outputs, tolerances, both engines' artifacts, comparison reports and a blocking
gate. Algorithm changes do not belong in compatibility mode.

Ad-hoc runs retain `conformance_claim=false`. Unsupported inputs are reported;
Rust does not silently substitute oracle results. Source A/B automatic oracle
comparison is not connected; use the [separate-run guide](docs/src/guides/verify-porting.md).

## Develop

```powershell
.\scripts\dev.cmd setup -InstallRust
.\scripts\dev.cmd check -Package ep_runtime -Filter <test_filter> -Lib
```

Use focused tests while editing and one workspace run before a release. See
[Contributing](CONTRIBUTING.md), [developer workflow](docs/src/guides/developer-workflow.md)
and [release process](docs/src/guides/release-process.md).
Generated references come from `specs/`; local `.runtime/` and `.reference/`
assets are excluded from the portable package.

See [LICENSE](LICENSE) and [third-party notices](THIRD_PARTY_NOTICES.md).
