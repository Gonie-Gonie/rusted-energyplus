---
status: active
claim_level: none
owner: core
last_reviewed: 2026-06-23
---

# Current Status

## Summary

The current public release line is v0.1.0. It packages the Rust CLI, locked
EnergyPlus 26.1.0 oracle setup, conformance/reporting infrastructure,
arbitrary IDF/epJSON support assessment, and a small Windows launcher.

The addendum rule for current-state judgment is: do not treat existing files as
done merely because they exist. `ep_run`, support assessment, arbitrary-run
artifacts, launcher scripts, and IdealLoads candidate cases are existing
implementation surfaces that now need source-order, capability, launcher, and
evidence hardening.

## Current Conformance Claims

Current conformance evidence is limited to promoted manifests and declared
variables/meters. The active promoted groups are:

- static intake, schedule, dry-bulb weather, no-mass heat balance, no-mass
  surface temperature, and internal convective gain seed cases
- official `1ZoneUncontrolled` dynamic source-order compatibility rows for the
  declared weather, zone-air, surface temperature, surface conduction,
  convection/radiation source, convection coefficient, solar, aggregate
  conduction, floor storage, humidity-ratio, adjacent-air, longwave coefficient,
  and iteration-count variables only
- IdealLoads no-OA/no-limit sensible, numeric capacity limit, numeric flow
  limit, flow-and-capacity limit, ConstantSensibleHeatRatio, selected
  ConstantSupplyHumidityRatio, selected Humidistat, selected outdoor-air,
  economizer, heat-recovery, ReportPurchasedAir energy/fuel, and declared
  facility-meter candidate rows only

The exact case list is generated in
`docs/src/generated/conformance-case-index.md`. The exact algorithm and
coverage boundaries are generated from `specs/algorithm_ledger.toml`,
`specs/object_coverage.toml`, and `specs/variable_coverage.toml`.

Passed release-evidence series and tracked output variables are intentionally
separate counts. Current public documentation tracks 22 passed release-evidence
series and 117 output-variable coverage rows, including conformance,
diagnostic, and baseline variables.

The official `1ZoneUncontrolled` target is currently declared-variable
compatibility only. Broad CTF storage parity, broad solar/radiation parity,
fenestration, infiltration, and general heat-balance compatibility remain
outside the claim.

IdealLoadsAirSystem evidence is branch-scoped and variable-scoped. No-OA
sensible, finite-limit, ConstantSensibleHeatRatio, selected humidity,
selected outdoor-air/economizer/heat-recovery, ReportPurchasedAir, and meter
rows remain limited to explicitly declared candidate cases and do not imply
full IdealLoads compatibility.

## Diagnostic-Only Evidence

Diagnostic-only evidence remains useful for source-order porting but does not
create compatibility claims. Current diagnostic-only groups include broad
official `1ZoneUncontrolled` heat-balance probes, remaining IdealLoads
predecessor/branch probes, air-side node projection diagnostics, plant-loop
state projection diagnostics, and smoke-level model intake checks.

## Not Claimed

The project does not currently claim general EnergyPlus compatibility, broad
heat-balance compatibility, broad HVAC/node/meter compatibility, plant
compatibility, sizing, EMS, PythonPlugin, AirflowNetwork, fenestration,
infiltration, broad weather-processor compatibility, broad ExampleFiles
compatibility, or broad IdealLoads branch coverage beyond the declared
candidate rows.

## Current Arbitrary-Run State

The arbitrary run framework writes support assessment artifacts and classifies
each input as `run_blocked`, `partial_supported_run`, or
`supported_compatibility_run`. Ad-hoc runs and launcher runs always keep their
claim boundary visible and do not become release conformance evidence unless
promoted later through manifests, reports, and blocking gates.
Unsupported AirLoop, PlantLoop, EMS, invalid epJSON, and missing heat-balance
weather inputs are covered by integration tests as pre-runtime failures.
When `--oracle-baseline` is requested for a `run_blocked` input, the
EnergyPlus baseline can still be generated under `out/oracle`, but
`rust_runtime` stays null, oracle comparison is skipped, and no oracle artifact
is counted as a Rust result.
Dry-run execution is also covered as a support-assessment-only path: even if
oracle baseline and compare are requested, runtime, oracle, and compare remain
skipped.

The current compatibility-mode arbitrary runtime covers the official
`1ZoneUncontrolled` heat-balance path and the declared IdealLoads
ZoneEquipmentManager -> PurchasedAirManager no-OA sensible, numeric
finite-limit, and ConstantSensibleHeatRatio branches. Other IdealLoads
humidity, outdoor-air, economizer, heat-recovery, and broad HVAC branches
remain outside arbitrary-run compatibility unless separately promoted through
the release conformance manifests.

## Current Launcher State

The current Windows launcher script invokes `eplus-rs run` as a CLI process. It
can choose and remember input, weather, output folder, oracle folder, and CLI
binary; map mode, partial policy, output format, trace level, strict warning
failure, oracle baseline, oracle compare, and overwrite controls to CLI
arguments; show the three run-result states; display exit-code-aware status
details including oracle and compare status, stage timing, top diagnostics, and
the `conformance_claim=false` boundary; open output, diagnostics, run report,
support report, and compare report artifacts; and show that Rusted EnergyPlus
is not a drop-in replacement.

The launcher self-test covers command construction for diagnostic, oracle
baseline, and oracle compare runs; the three run-state presentations including
a `run_blocked` case where oracle generation is shown separately from Rust
runtime success; phase timing formatting; and wrapper invocation through
`scripts/dev.cmd launch-ui`.

## Current Structure Gates

- heat-balance compatibility and diagnostic selections have separate typed
  APIs, the diagnostic probe enum lives under `diagnostic_probes`, and the
  official 1Zone compatibility selector resolves to an explicit compatibility
  execution variant.
- heat-balance source-order stage definitions live under
  `heat_balance::{manager,surface_manager,air_manager,zone_predictor_corrector,
  ctf,convection,radiation,reports}` and are guarded by
  `scripts/quality/heat-balance-structure-audit.ps1`.
- `ExecutionPlan` records EnergyPlus heat-balance and IdealLoads source-order
  barriers, including `ManageZoneAirUpdates` for heat balance and
  `SimPurchasedAir`, `GetPurchasedAir`, `InitPurchasedAir`,
  `CalcPurchAirLoads`, `UpdatePurchasedAir`, and `ReportPurchasedAir` for the
  arbitrary-run IdealLoads lane. It writes expected/actual source-order stage
  IDs to `execution-plan.json` and `run-summary.json`, and blocks runtime
  execution with a Plan exit if the lists diverge.
