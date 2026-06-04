# rusted-energyplus

Rust-only EnergyPlus-compatible engine prototype.

The project is compatibility-first: the initial oracle is EnergyPlus 26.1.0,
while the Rust implementation is organized around epJSON/schema-native input,
a model compiler, typed IDs, explicit simulation state, graph validation,
execution plans, structured diagnostics, and reproducible releases.

## Current Scope

This repository is at the v0.2.0 RawModel inspection stage:

- Rust toolchain pinned in `rust-toolchain.toml`
- Cargo workspace skeleton
- portable EnergyPlus oracle setup
- reference EnergyPlus source setup
- docs skeleton and copied development plan
- epJSON RawModel inspection CLI
- smoke/check scripts

## Quick Start

```powershell
.\scripts\setup.cmd -InstallRust -InstallDocsTools
.\scripts\check.cmd
.\scripts\oracle-smoke.cmd
.\scripts\v0.1-verify.cmd
.\scripts\raw-model-smoke.cmd
.\scripts\v0.2-verify.cmd
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

The CLI can inspect epJSON into a RawModel summary:

```powershell
cargo run -p ep_cli -- --version
cargo run -p ep_cli -- oracle-info
cargo run -p ep_cli -- model inspect .runtime\oracle-smoke\26.1.0\convert\smoke.epJSON
```

Unsupported runtime commands should fail explicitly until their milestone is
implemented.

## Documentation

- Development plan: `docs/src/development-plan-v2.md`
- Rust-only policy: `docs/src/architecture/rust-only-policy.md`
- Data architecture: `docs/src/architecture/data-architecture.md`
- Oracle setup: `docs/src/operations/oracle-setup.md`
- External review log: `docs/src/operations/external-checkpoints.md`
- v0.1 readiness: `docs/src/operations/v0.1.0-readiness.md`
- v0.2 readiness: `docs/src/operations/v0.2.0-readiness.md`
