# rusted-energyplus

Rust-only EnergyPlus-compatible porting project.

## Compatibility Contract

rusted-energyplus targets compatibility with the locked EnergyPlus 26.1.0
oracle. The Rust project does not replace EnergyPlus engineering algorithms;
optimization is limited to Rust data structures, execution planning, caching,
diagnostics, result storage, numerical implementation within declared
tolerance, and code organization.

## Current Public Scope

- pinned Rust toolchain
- repo-local EnergyPlus 26.1.0 oracle and reference source setup
- repo-local portable Python report environment
- epJSON RawModel inspection and TypedModel compile preview
- conformance manifests, output requests, tolerance policy, and report gates
- manifest-owned output request injection for staged oracle baselines
- timestamp-aware selected series reader and compare metrics v2
- release conformance index and coverage matrix report generation
- source-map and algorithm ledger validation gate
- timestamp-aligned time/weather/schedule conformance report gate
- official ExampleFile static model EIO conformance report gate
- runtime output registry, meter registry diagnostics, ResultStore duplicate
  checks, and profile scaffolding
- oodocs/matplotlib release evidence generation
- Case Manifest and Output Request Schema v2 validation
- tolerance-gated conformance only for declared v0.8/v0.9 no-mass cases and
  declared v0.22 `Schedule Value` / dry-bulb hourly variables
- static EIO model conformance only for the declared v0.23 official
  `1ZoneUncontrolled` surface, construction/material, and OtherEquipment
  nominal fields
- v0.24 runtime registry hardening only as infrastructure; no new numerical
  conformance

Not claimed:

- general EnergyPlus heat-balance compatibility
- HVAC or plant simulation compatibility
- node, IdealLoads, meter, or full runtime conformance
- broad ExampleFiles compatibility

## Quick Start

```powershell
.\scripts\dev.cmd setup -InstallRust -InstallDocsTools
.\scripts\dev.cmd check
```

Useful focused checks:

```powershell
.\scripts\dev.cmd docs-generate
.\scripts\dev.cmd docs-check
.\scripts\dev.cmd manifest-validate-all
.\scripts\dev.cmd strict-no-false-conformance
.\scripts\dev.cmd official-baseline-smoke
.\scripts\dev.cmd compare-series-v2-smoke
.\scripts\dev.cmd algorithm-ledger-check
.\scripts\dev.cmd compare-schedule-conformance
.\scripts\dev.cmd compare-weather-conformance
.\scripts\dev.cmd compare-static-model-conformance
.\scripts\dev.cmd runtime-registry-smoke
.\scripts\dev.cmd conformance-index-report -Version 0.24.0
.\scripts\dev.cmd conformance-evidence-report -Version 0.24.0
```

## Documentation

Start with the current docs:

- `docs/src/current/project-contract.md`
- `docs/src/current/current-status.md`
- `docs/src/current/roadmap.md`
- `docs/src/current/verification.md`
- `docs/src/current/architecture-overview.md`

Old planning docs are not retained in the mdBook tree. Use Git history,
release notes, and GitHub Release assets for historical planning and frozen
evidence.

Build the book with:

```powershell
.\scripts\dev.cmd docs-check
```
