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

Current hardening targets are capability-registry matching, branch-specific
runtime classes, compatibility-mode rejection of diagnostic-only runtime
classes, explicit partial-run policy, and golden artifact/exit-code tests.

## Current Launcher State

The current Windows launcher script exists and invokes `eplus-rs run` as a CLI
process. It can choose input, weather, output folder, oracle folder, compare
toggle, overwrite behavior, and report links.

It is not yet complete against the target launcher contract. Remaining
hardening includes mode selection, partial-run policy controls, three-state
status display, exit-code-specific messages, diagnostics/support report tabs,
and no-silent-oracle-fallback tests.

## Current Known Blockers

- `runtime.rs` still owns too much source-order, compatibility, and diagnostic
  behavior.
- heat-balance compatibility and diagnostic selections now have separate typed
  APIs and the diagnostic probe enum lives under `diagnostic_probes`, but
  `HeatBalanceSimulationOptions` and the CLI still use the legacy
  `HeatBalanceZoneAirAlgorithm` selector during the module split.
- `ExecutionPlan` now records EnergyPlus heat-balance and IdealLoads
  source-order barriers, but those barriers still need deeper stage-level
  dispatch, snapshots, and mismatch gates.
- `SupportAssessment` exists but is not yet driven centrally by
  `specs/capabilities.toml`.
- arbitrary-run IdealLoads support needs to distinguish declared compatibility
  branches from diagnostic node-state projection.
- old plan/readiness content is still present outside current navigation and
  must be shrunk, moved to specs/ADR, or removed in later cleanup commits.
