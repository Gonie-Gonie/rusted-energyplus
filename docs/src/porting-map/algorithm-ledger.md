---
status: active
claim_level: conformance-boundary
owner: core
last_reviewed: 2026-07-14
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

Before a source-order compatibility algorithm is implemented or promoted, the
PR must include a completed Algorithm Port Ticket field snapshot; it may also
link to a checked ticket. The ticket records the
EnergyPlus source file, routine, Rust target module/function, read/write state,
history ownership, proof variables, first target case, not-claimed branches,
tolerance/report/gate, and whether any diagnostic probe is involved. Use
`specs/algorithm_port_ticket_template.toml` as the field contract.

Diagnostic probes must be identified as diagnostic in the ticket and cannot
support conformance promotion. A refactor that touches a source-order-sensitive
production path uses a completed `refactor_only` ticket; non-sensitive diffs
pass the changed-file classifier without a ticket.

v0.21 makes this rule executable through `algorithm-ledger-check`. The gate
loads `specs/algorithm_ledger.toml`, checks each `source_map`, verifies
EnergyPlus source files against `.reference/energyplus-src/26.1.0`, verifies
Rust target files, and checks first-case manifests, proof variables, and
blocking gates for conformance-status entries.

The generated ledger at `docs/src/generated/algorithm-ledger.md` is the
machine-readable spec rendered for review. Keep this narrative page for policy
and maintenance notes; keep row-level algorithm evidence and routine completion
state in the spec. Algorithm evidence status and routine completion status are
independent: a limited conformance algorithm does not promote any required
EnergyPlus routine automatically.

## Current Ledger

| Domain | EnergyPlus source anchor | Rust target | Evidence state | Claim boundary |
|---|---|---|---|---|
| Schedule constant and compact subset | Schedule manager routines, source-map scope outside current claim | typed schedules and schedule traces | smoke and input-evidence gates | schedule parsing/value evidence only |
| Weather dry-bulb input | weather data manager routines, source-map scope outside current claim | EPW records and weather traces | weather-field smoke gate | selected weather field evidence only |
| Geometry and constructions | heat-balance input managers | typed geometry/material/construction summaries | EIO smoke gates | input interpretation evidence only |
| Internal convective gains | `HeatBalanceInternalHeatGains.cc`, `InternalHeatGains.cc` | runtime internal-gain trace | ESO smoke comparison | not zone air compatibility by itself |
| No-mass zone mean air temperature | `ManageHeatBalance`, `ManageZoneAirUpdates`, `correctZoneAirTemps` | heat-balance state and zone MAT trace | v0.8 promoted conformance case | only `heat_balance_nomass_001` MAT |
| No-mass surface temperatures | `CalcHeatBalanceOutsideSurf`, `CalcHeatBalanceInsideSurf` | surface state trace | v0.9 promoted conformance case | only `surface_temperature_nomass_001` declared variables |
| Thermostat and IdealLoads no-OA sensible branch | `PurchasedAirManager.cc`, `ZoneEquipmentManager.cc`, `ZoneTempPredictorCorrector.cc`, and node/source maps | `ep_runtime::ideal_loads` helper, zone equipment demand state, and supply-node update | `ideal_loads_no_oa_sensible_conformance_001`, `ideal_loads_capacity_limit_conformance_001`, `ideal_loads_flow_limit_conformance_001`, and `ideal_loads_flow_capacity_limit_conformance_001` promoted conformance gates | declared no-OA/no-limit and numeric finite-limit thermostat, IdealLoads rate, supply-node temperature, and supply-node flow variables only |
| IdealLoads no-OA ConstantSensibleHeatRatio cooling branch | `PurchasedAirManager.cc`, `Psychrometrics.hh`, `OutputProcessor.cc`, and node/source maps | `ep_runtime::ideal_loads` latent/sensible split helpers and supply-node humidity update | `ideal_loads_constant_shr_conformance_001` promoted conformance gate | declared no-OA Constant SHR cooling total/sensible/latent and supply-node humidity rows only |
| Air-side node state | node and HVAC manager source map | `NodeStateStore` projection plumbing | v0.11 diagnostic-only baseline/projection | not node or HVAC numerical conformance |
| Node source mapping policy | node state source map | planning guard | v0.12 policy/readiness | no new numerical claim |
| PlantLoop typed graph | plant manager source-map scope outside current claim | typed PlantLoop graph edges | v0.13 smoke gate | no plant loop simulation |
| Plant manager and flow source map | `ManagePlantLoops`, `SetComponentFlowRate` | plant source-map planning guard | v0.14 planning-ready evidence | no plant numerical claim |
| PlantLoadProfile baseline | plant loop and component reporting anchors | plant diagnostic output classes | v0.15 baseline-only diagnostic | not plant, equipment, meter, or flow conformance |
| PlantLoadProfile projection addendum | same source-map anchors, algorithms not ported | `simulate_plant_state_projection` and `run plant-state-projection` | post-v0.15 projected diagnostic artifact | `algorithm_parity: false`; not plant numerical conformance |

## Ledger Maintenance

When a milestone adds a new runtime algorithm or advances a routine, update
this ledger in the same
change as the source map, manifest, gate, and readiness note. The entry should
show its algorithm evidence class:

| Algorithm evidence state | Meaning |
|---|---|
| source_mapped | EnergyPlus routine and Rust target are identified. |
| scaffold | Rust structures exist, but algorithm parity is not claimed. |
| diagnostic-only | Baseline or projection exists with `conformance_claim = false`. |
| conformance | Manifest, artifacts, tolerance, report, and gate prove the claim. |
| superseded | A broader conformance case replaces lower-level evidence. |

Each tracked EnergyPlus routine has a separate ordered completion status:
Routine records use dotted keys under their parent algorithm, for example
`routine.manage_heat_balance.completion_status`, so routine promotion remains
inside the same Algorithm Port Ticket review boundary.

| Routine completion status | Minimum durable evidence |
|---|---|
| `not_started` | A stable canonical routine ID exists. |
| `source_mapped` | The EnergyPlus 26.1.0 source file and routine are present in a checked-in source map. |
| `state_mapped` | Read/write state, history ownership, unsupported state, and inactive/active branch boundaries are checked in. |
| `implemented` | A Rust file and symbol implementing the mapped routine are checked in. |
| `family_gated` | A declared conformance case exercises the routine through a registered blocking gate. |
| `complete` | Closure evidence exists and no unsupported active or not-claimed branch remains. |

At `state_mapped` and above, `state_mapping_ref` must contain one delimited
contract block for the routine. The opening and closing markers are
`<!-- routine-state-contract:v1 begin ROUTINE_ID -->` and
`<!-- routine-state-contract:v1 end ROUTINE_ID -->`. Inside that block,
declare `read_state:`, `write_state:`, `history_state_ownership:`,
`unsupported_state:`, `inactive_branches:`, `unsupported_active_branches:`,
and `not_claimed_branches:`. Every ledger value must occur inside its routine's
block, and read/write/history values must name state rather than repeat the C++
`source_routine` as a placeholder. One state-map file may contain separate
blocks for multiple routines.

At `implemented` and above, every routine `rust_target` must exactly match one
of its parent algorithm's `rust_target` entries. The parent must also contain a
four-part `port_ticket_mappings` entry whose source file and routine are the
routine record's exact `source_file` and `source_routine`. This keeps routine
promotion representable by the same single-algorithm PR ticket used for the
implementation.

Later conformance cases are declared once on the parent with `family_cases`.
A routine may select the parent's `first_case` or one of those family cases in
`family_gate_ids`; the selected case must name both the parent and the routine
in its manifest. The case `scope.domains` must include the routine's exact
domain; related output-domain aliases may filter proof rows but cannot stand in
for that exact scope. Routine `proof_variables` must also be a subset of the
parent algorithm's reviewed `proof_variables`.

The following excerpt shows the cross-file family-gate linkage only; all
`source_mapped` and structured `state_mapped` fields described above remain
mandatory at `family_gated`:

```toml
# specs/algorithm_ledger.toml, inside one [[algorithm]] entry
domain = "example"
family_cases = ["example_family_conformance_001"]
proof_variables = ["Example Routine Output"]
port_ticket_mappings = [
  "src/EnergyPlus/ExampleManager.cc|ManageExample|ManageExample|ManageExample",
]
rust_target = ["crates/ep_runtime/src/example.rs::manage_example"]

routine.manage_example.source_file = "src/EnergyPlus/ExampleManager.cc"
routine.manage_example.source_routine = "ManageExample"
routine.manage_example.completion_status = "family_gated"
routine.manage_example.rust_target = ["crates/ep_runtime/src/example.rs::manage_example"]
routine.manage_example.family_gate_ids = ["example_family_conformance_001"]
routine.manage_example.proof_variables = ["Example Routine Output"]
routine.manage_example.required_for_full_domain = true

# data/conformance_cases/example_family_conformance_001/case.toml
[scope]
domains = ["example"]

[routine_coverage]
algorithm_ids = ["example_manager_source_order"]
routine_ids = ["manage_example"]
```

Set `required_for_full_domain = true` only for routines that belong to the
domain's canonical complete inventory. The corresponding
`domain_claim.required_routines` list must match all and only those flagged
routines; the validator rejects list-only omissions or additions. The first 13
source-order routines form an immutable minimum seed across heat balance,
HVAC, and plant. A change cannot remove a seed routine by clearing both its
ledger flag and its project-contract list entry; future inventories may only
add required routines to that minimum.

`specs/project_contract.toml` owns the canonical required-routine list for each
full domain. A heat-balance, HVAC, or plant full-domain claim is valid only
when its routine inventory is explicitly complete and every required routine
is `family_gated` or `complete`. Missing, empty, duplicate, unknown, or
cross-domain required-routine entries fail the contract check. Full runtime
compatibility remains locked until every EnergyPlus domain, including domains
outside these first three inventories, has the same machine-readable closure.

Low-level development checks should be retired from release evidence when a
higher-level conformance case covers the same behavior more directly.
