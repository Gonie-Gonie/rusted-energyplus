# rusted-energyplus

Rust-only EnergyPlus-compatible porting project.

## Project Identity

`rusted-energyplus` is a Rust-only porting project that keeps EnergyPlus
26.1.0 as the locked oracle and treats generated evidence, not demos, as the
basis for compatibility claims.

## Porting Work

The [fixed porting plan](energyplus_porting_plan/README.md) records the completed
64-card A/B scope: an unconditioned opaque one-zone model and a single-zone,
direct, no-outdoor-air sensible IdealLoads model with explicit limits.
New scope starts with the `CON-01` contract and applicable predecessor gates.
The packet's Rust commit is the reviewed baseline; each new result records its
actual implementation and execution commit.

For each card, fix the original EnergyPlus source range and state ownership,
compare equal inputs and initial states with EnergyPlus, verify the connected
execution, and confirm that Rust performs the production calculation.
Record commands, results, unresolved issues, and the four gates in the card,
`CHECKLIST.md`, and `plan.json`. Existing code and skipped branches do not
establish numerical equivalence.

Use focused checks while implementing a card:

```powershell
python tools/porting/check_plan.py --check
python tools/porting/sync_plan_views.py --check
.\scripts\dev.cmd check -Package ep_runtime -Filter <card_test_filter> -Lib
```

Use `dev test -Package <crate> -Filter <test_filter>` for a single test area,
`dev test` for the workspace once, and `dev check -Full` for the broader
integration checks at a milestone. `-Plan` prints the selected commands without
running them. The default `dev check` covers `ep_runtime`; select every affected
package explicitly when changing another layer or a public interface. See
[Contributing](CONTRIBUTING.md#test-selection) for the change-to-test mapping.

Run the card's EnergyPlus comparison and execution test before closing its
gates. Commit and push each reviewable unit after its relevant validation.
Do not repeat a focused suite after the same unchanged code has passed the
workspace suite.
After reviewing a card and updating its evidence and gate metadata, run
`python tools/porting/sync_plan_views.py --write` to refresh the checklist and
offline HTML plan. Browser checks are local work notes; recorded repository
evidence governs completion.

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

The [accepted finite validation](energyplus_porting_plan/evidence/SRC-06-final-current-source-validation-60/summary.json)
and [SUR-02 unit supplement](energyplus_porting_plan/evidence/SUR-02-isolated-history-impulse-61/summary.json)
retain the actual commands, policies, results and limitations. Declared reported
values pass their fixed tolerances; whole-file byte equality and general
EnergyPlus compatibility are separate claims. See [Current Status](docs/src/current/current-status.md)
for the current result and remaining user-facing work.

## How To Run And Verify

```powershell
.\scripts\dev.cmd setup -InstallRust -InstallDocsTools
.\scripts\dev.cmd check
cargo run --locked -p ep_cli --bin eplus-rs -- run .\model.idf --weather .\weather.epw --output-dir .\out
```

The completed Source route uses `--porting-scope A|B`. Its automatic
`--oracle-baseline`/`--compare-oracle` integration is not connected yet; use
[Verify the port](docs/src/guides/verify-porting.md) to run both engines separately
and inspect the fixed-case comparison files. [Run an IDF](docs/src/guides/run-arbitrary-idf.md)
explains supported inputs, artifacts and launcher controls.

Focused checks:

```powershell
.\scripts\dev.cmd docs-generate
.\scripts\dev.cmd docs-check
.\scripts\dev.cmd check -Full -Plan
```

Use [Developer Workflow](docs/src/guides/developer-workflow.md) for test selection
and [Release Process](docs/src/guides/release-process.md) for packaging/evidence commands.

Current docs:

- `docs/src/current/project-contract.md`
- `docs/src/current/current-status.md`
- `docs/src/current/roadmap.md`
- `docs/src/current/architecture-overview.md`

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
