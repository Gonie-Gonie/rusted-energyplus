# rusted-energyplus

Rust-only EnergyPlus-compatible engine prototype.

The project is compatibility-first: the initial oracle is EnergyPlus 26.1.0,
while the Rust implementation is organized around epJSON/schema-native input,
a model compiler, typed IDs, explicit simulation state, graph validation,
execution plans, structured diagnostics, and reproducible releases.

## Current Scope

This repository is now past the RawModel/TypedModel seed and has the first
traceable comparison diagnostics. It does not yet claim EnergyPlus numerical
compatibility for zone heat balance or HVAC simulation.

- Rust toolchain pinned in `rust-toolchain.toml`
- Cargo workspace skeleton
- portable EnergyPlus oracle setup
- reference EnergyPlus source setup
- docs skeleton and copied development plan
- epJSON RawModel inspection CLI
- typed compile preview CLI for the first seed object families
- preview missing reference diagnostics
- SimulationModel, ModelGraph, and ExecutionPlan summaries
- typed `RunPeriod` intake and hourly time-axis foundation
- EPW hourly weather record parsing beyond dry-bulb
- `Schedule:Compact` all-days `Until` segment subset
- zone geometry summary foundation for future EnergyPlus EIO comparison
- EnergyPlus oracle comparisons for constant schedules, EIO zone geometry, and
  EPW dry-bulb weather
- EnergyPlus EIO nominal internal-gains comparison for `OtherEquipment`
- typed conformance case/suite manifests for future tolerance-gated evidence
- `OutputRegistry` foundation for output-request driven reports
- compare regression artifacts: `trace.json`, `compare-summary.json`,
  `compare-report.md`, and `profile-summary.json`
- smoke/check scripts

Development-only diagnostics:

- `run first-zone` exercises runtime plumbing with a deterministic RC-style toy
  model and writes a `ResultStore`
- `compare zone-temperature` extracts EnergyPlus and Rust zone-temperature
  series and reports deltas, but it is diagnostic-only and does not enforce a
  tolerance or claim EnergyPlus heat-balance compatibility

## Quick Start

```powershell
.\scripts\setup.cmd -InstallRust -InstallDocsTools
.\scripts\check.cmd
.\scripts\oracle-smoke.cmd
.\scripts\v0.1-verify.cmd
.\scripts\raw-model-smoke.cmd
.\scripts\typed-model-smoke.cmd
.\scripts\model-plan-smoke.cmd
.\scripts\schedule-compact-smoke.cmd
.\scripts\geometry-smoke.cmd
.\scripts\compare-geometry-smoke.cmd
.\scripts\compare-internal-gains-smoke.cmd
.\scripts\compare-schedule-smoke.cmd
.\scripts\compare-weather-smoke.cmd
.\scripts\first-zone-smoke.cmd
.\scripts\compare-zone-smoke.cmd
.\scripts\compare-regression.cmd
.\scripts\conformance-schema-smoke.cmd
.\scripts\conformance-baseline-smoke.cmd
.\scripts\conformance-report-smoke.cmd
.\scripts\strict-no-false-conformance.cmd
.\scripts\package.cmd
```

On Windows, the pinned Rust toolchain uses the GNU target so the early
workspace can build without requiring Visual Studio Build Tools.

The `.cmd` wrappers call the PowerShell scripts with `-ExecutionPolicy Bypass`
for the current process only. You can also call the `.ps1` files directly when
your shell policy allows it.

The setup script keeps external oracle assets inside repo-local directories:

- `.runtime/energyplus/26.1.0`
- `.reference/energyplus-src/26.1.0`

It does not use a globally installed EnergyPlus as the oracle.

## Core Commands

The CLI can inspect epJSON into a RawModel summary and compile the first typed subset:

```powershell
cargo run -p ep_cli -- --version
cargo run -p ep_cli -- oracle-info
cargo run -p ep_cli -- model inspect .runtime\oracle-smoke\26.1.0\convert\smoke.epJSON
cargo run -p ep_cli -- model compile .runtime\oracle-smoke\26.1.0\convert\smoke.epJSON
cargo run -p ep_cli -- model plan .runtime\oracle-smoke\26.1.0\convert\smoke.epJSON
cargo run -p ep_cli -- model geometry .runtime\oracle-smoke\26.1.0\convert\smoke.epJSON
cargo run -p ep_cli -- compare geometry .runtime\oracle-smoke\26.1.0\convert\smoke.epJSON .runtime\oracle-smoke\26.1.0\eplusout.eio
cargo run -p ep_cli -- compare internal-gains .runtime\oracle-smoke\26.1.0\convert\smoke.epJSON .runtime\oracle-smoke\26.1.0\eplusout.eio
cargo run -p ep_cli -- run first-zone .runtime\oracle-smoke\26.1.0\convert\smoke.epJSON .runtime\energyplus\26.1.0\WeatherData\USA_CO_Golden-NREL.724666_TMY3.epw --hours 24
cargo run -p ep_cli -- compare zone-temperature .runtime\compare-zone\26.1.0\zone-temperature.epJSON .runtime\energyplus\26.1.0\WeatherData\USA_CO_Golden-NREL.724666_TMY3.epw .runtime\compare-zone\26.1.0\eplusout.eso
cargo run -p ep_cli -- conformance validate-case data\conformance_cases\schedule_constant_001\case.toml
cargo run -p ep_cli -- conformance baseline data\conformance_cases\schedule_constant_001\case.toml .runtime\energyplus\26.1.0 .runtime\conformance-baseline\26.1.0
cargo run -p ep_cli -- conformance report-skeleton data\conformance_cases\schedule_constant_001\case.toml .runtime\conformance-baseline\26.1.0\schedule_constant_001 .runtime\conformance-report\26.1.0
```

`.\scripts\compare-regression.cmd` runs the current schedule, weather, and zone
comparison smoke suite and writes `trace.json`, `compare-summary.json`,
`compare-report.md`, and `profile-summary.json` under
`.runtime\compare-regression\26.1.0`.

The `first-zone` and `zone-temperature` commands are intentionally labeled as
diagnostics. They are useful for plumbing and report generation work, but they
are not release evidence for EnergyPlus heat-balance conformance.

`.\scripts\geometry-smoke.cmd` summarizes zone surface count, floor area,
derived volume, and exterior wall area from the EnergyPlus-converted oracle
fixture. This is comparison preparation; it is not yet a tolerance-gated EIO
comparison.

`.\scripts\compare-geometry-smoke.cmd` compares that Rust geometry summary with
EnergyPlus `eplusout.eio` Zone Information using a small absolute tolerance.
This is an input-interpretation smoke gate, not heat-balance simulation
evidence.

`.\scripts\compare-internal-gains-smoke.cmd` compares typed `OtherEquipment`
nominal gains with EnergyPlus `eplusout.eio`. It verifies schedule/zone binding,
design level, W/m2, and gain fractions as input-interpretation evidence.

`.\scripts\conformance-schema-smoke.cmd` validates the current P1
case/suite manifest schema. The first fixture is intentionally a smoke manifest
with `conformance_claim = false`; tolerance-gated cases must add output
requests, tolerances, report contract, and a blocking gate before claiming
EnergyPlus numerical conformance.

`.\scripts\conformance-baseline-smoke.cmd` generates the first manifest-driven
EnergyPlus baseline under `.runtime\conformance-baseline\26.1.0`. This proves
case-to-oracle artifact generation, not Rust simulation conformance.

`.\scripts\conformance-report-smoke.cmd` writes a baseline-only markdown report
from the case output requests and EnergyPlus ESO. It still reports
`conformance_claim: false` and `tolerance_policy: none`.

Unsupported runtime commands should fail explicitly until their milestone is
implemented.

## Release Publishing

Public releases are published by `.github/workflows/release.yml` when a
`vX.Y.Z` tag is pushed. The workflow prepares the toolchain and oracle, runs the
matching `scripts\vX.Y-verify.ps1`, builds the zip artifact, and creates or
updates the GitHub Release.

`scripts\github-release.cmd` is kept as a local manual fallback for token-based
publishing.

## Documentation

- Development plan: `docs/src/development-plan-v2.md`
- Rust-only policy: `docs/src/architecture/rust-only-policy.md`
- Data architecture: `docs/src/architecture/data-architecture.md`
- Oracle setup: `docs/src/operations/oracle-setup.md`
- External review log: `docs/src/operations/external-checkpoints.md`
- Foundation checkpoints: `docs/src/operations/foundation-checkpoints.md`
- v0.1 readiness: `docs/src/operations/v0.1.0-readiness.md`
- v0.6 readiness: `docs/src/operations/v0.6.0-readiness.md`
- v0.7 readiness: `docs/src/operations/v0.7.0-readiness.md`
- Conformance schema: `docs/src/operations/conformance-schema.md`
- full compatibility reset: `docs/src/operations/full-compatibility-reset.md`
