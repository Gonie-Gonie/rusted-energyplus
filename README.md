# rusted-energyplus

Rust-only EnergyPlus-compatible porting project.

## Project Identity

`rusted-energyplus` is a Rust-only porting project that keeps EnergyPlus
26.1.0 as the locked oracle and treats generated evidence, not demos, as the
basis for compatibility claims.

## Porting Work

The active work plan is [energyplus_porting_plan/README.md](energyplus_porting_plan/README.md).
Its 64 cards cover an unconditioned opaque one-zone model and a single-zone,
direct, no-outdoor-air sensible IdealLoads model with explicit limits.
Start with `CON-01`, then take a card whose applicable predecessors are verified.
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

Recent [scoped original-source comparisons](energyplus_porting_plan/evidence/SRC-06-canonical-warmup-multiplier-exterior-55/summary.json)
cover 69 persistent warmup calls, 21 multiplier-two predictor/corrector/history
calls, and 170 admitted weather, solar and gain records. Warmup and multiplier
checks compare 45,072 and 17,893 raw state/return fields. Exterior coverage joins
a verified ordered prefix of 137 records with 33 successful later category
records; all 179 Native records remain available, including nine exploratory
shading records outside the fixed input scope. These results retain their
recorded execution stages and unchanged-code bridges. The final fifteen-case
run on one current Source, full Surface cadence and warmup handoff comparison
remain separate work; card gates remain A 14/54 and B 14/64.

The [surface unit checkpoint](energyplus_porting_plan/evidence/SRC-06-canonical-surface-unit-checkpoint-56/summary.json) verifies 24 selected outside-surface calls and 30 convergence-suffix evaluations. The subsequent [quality checkpoint](energyplus_porting_plan/evidence/SRC-06-source-quality-hygiene-57/summary.json) passes formatting, strict workspace Clippy under default features, 4,975 workspace tests (two existing exclusions), and 18 admitted beam records on one Source snapshot. Earlier failed checks and the featured cohort target's unused-helper warnings remain in the evidence. The [HVAC admission comparison](energyplus_porting_plan/evidence/SRC-06-canonical-purchased-air-admission-58/summary.json) then passes seven error-path calls (35 states, 7,096 raw state observations), 60 positive calls and current quality checks. The [live kickoff observer correction](energyplus_porting_plan/evidence/SRC-06-source-live-kickoff-observer-59/summary.json) preserves the first A24 comparison failure and reads the actual scientific flag owner. Current quality checks pass. Next are fresh current-Source Surface cadence, handoff and final fifteen-case verification; card gates remain unchanged.

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
.\scripts\dev.cmd project-contract-check
.\scripts\dev.cmd spec-cross-check
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
