---
status: active
claim_level: smoke
owner: runtime
last_reviewed: 2026-06-07
---

# Plant

Plant loops are entering the codebase as a typed graph foundation, not as a
solver. The v0.13 Plant Loop Skeleton is a typed graph smoke gate for the first
loop, branch, connector, and equipment identity objects.

This map is a planning guard for future plant numerical work. It is not a plant
numerical conformance claim.

## v0.13 Plant Loop Skeleton

The v0.13 typed graph smoke covers:

- `PlantLoop`
- `Branch`
- `BranchList`
- `Connector:Splitter`
- `Connector:Mixer`
- `ConnectorList`
- `Pump:ConstantSpeed`
- `Boiler:HotWater`
- `Chiller:Electric:EIR`

The smoke fixture is:

```text
data/testcases/minimal/plant-loop-skeleton.epJSON
```

The gate is:

```powershell
.\scripts\dev.cmd plant-loop-skeleton-smoke
```

The fixture proves typed object intake, reference resolution, node
registration, and graph edge summaries. It does not execute plant hydraulics,
operation schemes, equipment loads, meters, or node-state output comparisons.

## Reference Source Targets

Future plant work must be ported against the locked EnergyPlus 26.1.0 source
tree. The first source targets are:

| Area | EnergyPlus source files | v0.13 use |
|---|---|---|
| plant manager | `src/EnergyPlus/Plant/PlantManager.cc`, `PlantManager.hh` | source-map target only |
| loop and loop side | `src/EnergyPlus/Plant/Loop.cc`, `Loop.hh`, `LoopSide.cc`, `LoopSide.hh` | source-map target only |
| branches and components | `src/EnergyPlus/Plant/Branch.cc`, `Branch.hh`, `Component.cc`, `Component.hh` | typed graph target |
| plant data | `src/EnergyPlus/Plant/DataPlant.hh`, `Enums.hh`, `PlantLocation.hh` | source-map target only |
| connectors | `src/EnergyPlus/Plant/MixerData.hh`, `SplitterData.hh`, `Connection.hh` | typed graph target |
| pump identity | `src/EnergyPlus/Pumps.cc`, `Pumps.hh` | typed identity only |
| boiler identity | `src/EnergyPlus/Boilers.cc`, `Boilers.hh` | typed identity only |
| chiller identity | `src/EnergyPlus/ChillerElectricEIR.cc`, `ChillerElectricEIR.hh` | typed identity only |

## Rust Boundary

Current Rust coverage:

- `ep_model` stores plant loop, branch, connector, and first equipment identity
  records.
- `ep_compiler` resolves plant loop sides to branch lists and connector lists,
  branch lists to branches, connector lists to connectors, and branch
  components to plant nodes.
- `ep_cli` reports typed plant counts and graph edge counts.

Current non-claims:

- no plant loop simulation
- no plant flow balancing
- no pump, boiler, chiller, or condenser-loop numerical parity
- no plant node output parity
- no plant meter parity
- no plant ExampleFiles compatibility

## Promotion Requirements

Before any plant implementation claim, the repository must add:

- plant loop data structures
- loop side and branch semantics
- pump/boiler/chiller subset
- node temperature and mass-flow variables
- meter variables
- tolerance policy
- EnergyPlus baseline case and output requests
- Rust result artifacts
- compare report and summary JSON
- blocking gate
