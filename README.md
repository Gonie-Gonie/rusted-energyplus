# rusted-energyplus

Rust-only EnergyPlus-compatible porting project.

## Project Identity

`rusted-energyplus` is a Rust-only porting project that keeps EnergyPlus
26.1.0 as the locked oracle and treats generated evidence, not demos, as the
basis for compatibility claims.

## Compatibility Contract

`rusted-energyplus` targets the locked EnergyPlus 26.1.0 oracle. Compatibility
mode means source-order EnergyPlus algorithm behavior with Rust data structures,
execution planning, caching, output handling, diagnostics, and result storage.
Engineering algorithm changes do not belong in compatibility mode.

Ad-hoc user runs are not release conformance evidence. A conformance claim
requires a case manifest, declared variables or meters, tolerances, oracle and
Rust artifacts, generated compare reports, and a blocking gate.

## Current Public Scope Summary

- Rust workspace with pinned toolchain and repo-local EnergyPlus 26.1.0 oracle
- RawModel, TypedModel, SimulationModel, and the arbitrary IDF/epJSON `ep_run`
  pipeline with support assessment, run states, and stable artifacts
- generated support assessment, diagnostics, run summary, compatibility
  boundary, and optional oracle compare artifacts
- declared conformance infrastructure for output requests, tolerances,
  reports, and blocking gates
- limited official `1ZoneUncontrolled` source-order heat-balance evidence for
  declared variables only
- limited IdealLoadsAirSystem evidence for declared no-OA, finite-limit,
  ConstantSHR, selected humidity, outdoor-air, economizer, heat-recovery, and
  meter candidate branches only
- small Windows launcher that invokes the CLI run pipeline and shows the
  support/claim boundary instead of hiding unsupported features

Current counts are tracked outside the README. Generated coverage specs track
the output-variable inventory, and release evidence separates passed
release-evidence series from broader declared conformance/output requests.

## How To Run And Verify

```powershell
.\scripts\dev.cmd setup -InstallRust -InstallDocsTools
.\scripts\dev.cmd check
eplus-rs run .\model.idf -w .\weather.epw -d .\out --oracle-baseline --compare-oracle
.\scripts\dev.cmd conformance-evidence-report -Version 0.1.0
.\scripts\dev.cmd support-coverage-report -Version 0.1.0
.\scripts\dev.cmd user-coverage-handbook -Version 0.1.0
.\scripts\dev.cmd release-evidence-manifest -Version 0.1.0
```

Focused checks:

```powershell
.\scripts\dev.cmd docs-generate
.\scripts\dev.cmd docs-check
.\scripts\dev.cmd manifest-validate-all
.\scripts\dev.cmd strict-no-false-conformance
.\scripts\dev.cmd algorithm-ledger-check
.\scripts\dev.cmd arbitrary-run-smoke
.\scripts\dev.cmd launcher-smoke
```

Current docs:

- `docs/src/current/project-contract.md`
- `docs/src/current/current-status.md`
- `docs/src/current/roadmap.md`
- `docs/src/current/verification.md`
- `docs/src/current/architecture-overview.md`
- `docs/src/current/launcher-and-run-framework.md`

Generated references live under `docs/src/generated` and are produced from
`specs/*.toml` plus case manifests. Release evidence is generated under
`.runtime/release-evidence` and curated through GitHub Release assets.

Old planning docs are not retained as current navigation. Use Git history,
release notes, generated specs, and GitHub Release assets for historical
planning and frozen evidence.

## What Is Not Claimed

- broad or complete EnergyPlus replacement behavior
- general heat-balance, weather processor, fenestration, infiltration, HVAC,
  plant, sizing, EMS, PythonPlugin, AirflowNetwork, or broad meter
  compatibility
- broad ExampleFiles compatibility
- compatibility from diagnostic probes, fast mode, experimental mode, smoke
  tests, or arbitrary user runs
