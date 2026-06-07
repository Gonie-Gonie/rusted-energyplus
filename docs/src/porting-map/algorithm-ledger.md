---
status: active
claim_level: conformance-boundary
owner: core
last_reviewed: 2026-06-07
---

# Algorithm Ledger

This ledger keeps source mapping, Rust implementation, and evidence state in
one place. It prevents a diagnostic or scaffold from being mistaken for an
EnergyPlus algorithm port.

## Promotion Rule

An algorithm entry can support a compatibility claim only when it has:

- EnergyPlus 26.1.0 source routine and file
- Rust target module or function
- explicit state mapping
- output variable mapping
- conformance case manifest
- EnergyPlus oracle artifact
- Rust result artifact
- tolerance policy
- compare summary/report
- blocking gate

No source map, no algorithm port.

v0.21 makes this rule executable through `algorithm-ledger-check`. The gate
loads `specs/algorithm_ledger.toml`, checks each `source_map`, verifies
EnergyPlus source files against `.reference/energyplus-src/26.1.0`, verifies
Rust target files, and checks first-case manifests, proof variables, and
blocking gates for conformance-status entries.

The generated ledger at `docs/src/generated/algorithm-ledger.md` is the
machine-readable spec rendered for review. Keep this narrative page for policy
and maintenance notes; keep row-level algorithm state in the spec.

## Current Ledger

| Domain | EnergyPlus source anchor | Rust target | Evidence state | Claim boundary |
|---|---|---|---|---|
| Schedule constant and compact subset | Schedule manager routines, source map pending expansion | typed schedules and schedule traces | smoke and input-evidence gates | schedule parsing/value evidence only |
| Weather dry-bulb input | weather data manager routines, source map pending expansion | EPW records and weather traces | weather-field smoke gate | selected weather field evidence only |
| Geometry and constructions | heat-balance input managers | typed geometry/material/construction summaries | EIO smoke gates | input interpretation evidence only |
| Internal convective gains | `HeatBalanceInternalHeatGains.cc`, `InternalHeatGains.cc` | runtime internal-gain trace | ESO smoke comparison | not zone air compatibility by itself |
| No-mass zone mean air temperature | `ManageHeatBalance`, `ManageZoneAirUpdates`, `correctZoneAirTemps` | heat-balance state and zone MAT trace | v0.8 promoted conformance case | only `heat_balance_nomass_001` MAT |
| No-mass surface temperatures | `CalcHeatBalanceOutsideSurf`, `CalcHeatBalanceInsideSurf` | surface state trace | v0.9 promoted conformance case | only `surface_temperature_nomass_001` declared variables |
| Thermostat and IdealLoads intake | thermostat and air-system source mapping pending | execution-plan placeholders and typed graph | v0.10 baseline-only smoke | not HVAC or load conformance |
| Air-side node state | node and HVAC manager source map | `NodeStateStore` projection plumbing | v0.11 diagnostic-only baseline/projection | not node or HVAC numerical conformance |
| Node source mapping policy | node state source map | planning guard | v0.12 policy/readiness | no new numerical claim |
| PlantLoop typed graph | plant manager source map pending at v0.13 | typed PlantLoop graph edges | v0.13 smoke gate | no plant loop simulation |
| Plant manager and flow source map | `ManagePlantLoops`, `SetComponentFlowRate` | plant source-map planning guard | v0.14 planning-ready evidence | no plant numerical claim |
| PlantLoadProfile baseline | plant loop and component reporting anchors | plant diagnostic output classes | v0.15 baseline-only diagnostic | not plant, equipment, meter, or flow conformance |
| PlantLoadProfile projection addendum | same source-map anchors, algorithms not ported | `simulate_plant_state_projection` and `run plant-state-projection` | post-v0.15 projected diagnostic artifact | `algorithm_parity: false`; not plant numerical conformance |

## Ledger Maintenance

When a milestone adds a new runtime algorithm, update this ledger in the same
change as the source map, manifest, gate, and readiness note. The entry should
show whether the work is:

| State | Meaning |
|---|---|
| source-map | EnergyPlus routine and Rust target are identified. |
| scaffold | Rust structures exist, but algorithm parity is not claimed. |
| diagnostic-only | Baseline or projection exists with `conformance_claim = false`. |
| conformance | Manifest, artifacts, tolerance, report, and gate prove the claim. |
| superseded | A broader conformance case replaces lower-level evidence. |

Low-level development checks should be retired from release evidence when a
higher-level conformance case covers the same behavior more directly.
