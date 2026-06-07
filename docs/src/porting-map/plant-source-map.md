---
status: active
claim_level: planning-guard
owner: runtime
last_reviewed: 2026-06-07
---

# Plant Source Map

Reference version: EnergyPlus 26.1.0

Reference source root:

```text
.reference/energyplus-src/26.1.0/
```

Purpose: lock the EnergyPlus plant-loop source files, routines, data fields,
and first Rust targets before any plant-loop, equipment, node-state, meter, or
ExampleFiles numerical claim is promoted. This map is a planning guard.

## Primary Source Files

| Area | EnergyPlus source | Rust target |
|---|---|---|
| plant manager and plant input | `src/EnergyPlus/Plant/PlantManager.cc`, `PlantManager.hh` | future `ep_runtime` plant scheduler; current `ep_compiler` PlantLoop graph intake |
| loop and loop-side state | `src/EnergyPlus/Plant/Loop.hh`, `LoopSide.hh`, `LoopSide.cc` | future plant loop runtime state; current graph edge summaries |
| branch and component records | `src/EnergyPlus/Plant/Branch.hh`, `Component.hh`, `Component.cc` | `ep_model::PlantBranch`, branch component edges, future plant component dispatch |
| plant global data and locations | `src/EnergyPlus/Plant/DataPlant.hh`, `PlantLocation.hh` | future typed plant state keyed by loop, side, branch, and component |
| connector topology | `src/EnergyPlus/Plant/MixerData.hh`, `SplitterData.hh` | current `Connector:Mixer` and `Connector:Splitter` typed records; future flow mixing/splitting |
| plant utility bridge | `src/EnergyPlus/PlantUtilities.cc`, `PlantUtilities.hh` | future node flow requests, component location scan, and inter-loop connection handling |
| pump identity and outputs | `src/EnergyPlus/Pumps.cc`, `Pumps.hh` | current `Pump:ConstantSpeed` identity; future pump flow and electricity reporting |
| boiler identity and outputs | `src/EnergyPlus/Boilers.cc`, `Boilers.hh` | current `Boiler:HotWater` identity; future boiler load and fuel reporting |
| chiller identity and outputs | `src/EnergyPlus/ChillerElectricEIR.cc`, `ChillerElectricEIR.hh` | current `Chiller:Electric:EIR` identity; future chiller load and electricity reporting |

## Required Routine And Field Map

| Behavior | EnergyPlus routine or field | Current Rust status |
|---|---|---|
| plant loop input and topology | `GetPlantLoopData` | v0.13 parses loop-side branch-list and connector-list names, but does not reproduce full input validation |
| plant simulation entry | `ManagePlantLoops` | not ported |
| loop-side scheduling order | `SetupInitialPlantCallingOrder`, `FindLoopSideInCallingOrder` | not ported |
| first-HVAC-iteration reset | `ReInitPlantLoopsAtFirstHVACIteration` | not ported |
| loop sizing and design flow | `SizePlantLoop`, `InitOneTimePlantSizingInfo` | not ported |
| loop side state flags | `HalfLoopData::SimLoopSideNeeded`, `FlowLock` | not ported |
| loop side demand and component pass | `HalfLoopData::simulate`, `EvaluateLoopSetPointLoad`, `UpdateAnyLoopDemandAlterations` | not ported |
| mixer and splitter updates | `UpdatePlantMixer`, `UpdatePlantSplitter` | typed topology exists; flow/temperature updates are not ported |
| component dispatch | `CompData::simulate`, `PlantEquipmentCtrlType` | branch-to-component graph edges exist; runtime dispatch is not ported |
| component location scan | `PlantUtilities::ScanPlantLoopsForObject` | current compiler resolves identities by explicit graph references, not EnergyPlus scan semantics |
| component flow request | `PlantUtilities::SetComponentFlowRate`, `InitComponentNodes` | not ported |
| inter-loop connection | `PlantUtilities::InterConnectTwoPlantLoopSides` | not ported |
| node copying | `PlantUtilities::SafeCopyPlantNode` | not ported |
| pump execution | `SimPumps`, `SizePump` | `Pump:ConstantSpeed` identity only |
| boiler execution | `BoilerSpecs::simulate`, `BoilerSpecs::onInitLoopEquip` | `Boiler:HotWater` identity only |
| chiller execution | `ElectricEIRChillerSpecs::simulate`, `ElectricEIRChillerSpecs::onInitLoopEquip` | `Chiller:Electric:EIR` identity only |

## Output And Meter Boundary

The first plant outputs to map before comparison are:

| Output or meter | EnergyPlus source | Current evidence level |
|---|---|---|
| `Plant Supply Side Cooling Demand Rate` | `PlantManager.cc` `SetupOutputVariable` block | future diagnostic |
| `Plant Supply Side Heating Demand Rate` | `PlantManager.cc` `SetupOutputVariable` block | future diagnostic |
| `Plant Supply Side Inlet Mass Flow Rate` | `PlantManager.cc` `SetupOutputVariable` block | future diagnostic |
| `Plant Supply Side Inlet Temperature` | `PlantManager.cc` `SetupOutputVariable` block | future diagnostic |
| `Plant Supply Side Outlet Temperature` | `PlantManager.cc` `SetupOutputVariable` block | future diagnostic |
| `Pump Electricity Rate` | `Pumps.cc` `SetupOutputVariable` block | future diagnostic |
| `Boiler Heating Rate` | `Boilers.cc` `SetupOutputVariable` block | future diagnostic |
| `Chiller Electricity Rate` | `ChillerElectricEIR.cc` `SetupOutputVariable` block | future diagnostic |
| `Chiller Evaporator Cooling Rate` | `ChillerElectricEIR.cc` `SetupOutputVariable` block | future diagnostic |
| `Heating:EnergyTransfer`, `Cooling:EnergyTransfer`, `Electricity:Facility`, `Gas:Facility` | meter output path through EnergyPlus output processor and equipment reporting | future diagnostic |

Output availability does not create a conformance claim. A promoted plant
output must declare key, variable, frequency, timestamp rule, warmup handling,
source artifact, tolerance policy, Rust result artifact, report, and blocking
gate.

## v0.13 Fixture Mapping

The current plant fixture is:

```text
data/testcases/minimal/plant-loop-skeleton.epJSON
```

It maps to EnergyPlus source only at the input/topology level:

| Fixture object | EnergyPlus source path | Current Rust evidence |
|---|---|---|
| `PlantLoop` | `PlantManager.cc` `GetPlantLoopData`; `Loop.hh`; `LoopSide.hh` | typed record, loop-side branch-list and connector-list references, graph counts |
| `Branch` and `BranchList` | `GetPlantLoopData`; `Branch.hh`; branch input manager calls from `PlantManager.cc` | typed branch records and branch-list member edges |
| `Connector:Splitter` | `GetPlantLoopData`; `SplitterData.hh`; `UpdatePlantSplitter` | typed connector record and connector-list member edge only |
| `Connector:Mixer` | `GetPlantLoopData`; `MixerData.hh`; `UpdatePlantMixer` | typed connector record and connector-list member edge only |
| `Pump:ConstantSpeed` | `Pumps.cc` `SimPumps`; plant scan through `ScanPlantLoopsForObject` | typed equipment identity and branch component edge only |
| `Boiler:HotWater` | `Boilers.cc` `BoilerSpecs::simulate` | typed equipment identity and branch component edge only |
| `Chiller:Electric:EIR` | `ChillerElectricEIR.cc` `ElectricEIRChillerSpecs::simulate` | typed equipment identity and branch component edge only |

## Porting Order

Plant numerical work must preserve this source-derived order unless a
case-specific waiver documents why it differs:

1. parse plant loops, loop sides, branch lists, connector lists, branches, and
   first component identities
2. resolve `PlantLocation`-equivalent loop, side, branch, and component ids
3. record loop-side input fields needed for demand calculation, sizing, and
   operation schemes
4. initialize plant node state and component min/max flow limits
5. reproduce the EnergyPlus loop-side calling order and first-iteration reset
6. implement flow request and `FlowLock` transitions for a reduced fixture
7. implement pump flow and electricity state for the first pump subset
8. implement boiler/chiller load calculation only after source-mapped demand
   and component dispatch are available
9. sample requested plant outputs and meters into `ResultStore`
10. compare only variables with declared timestamp, warmup, tolerance, and
    meter/source rules

## Stop Rule

No plant output may move beyond smoke, baseline-only, or diagnostic-only
evidence until the implementation names:

- the EnergyPlus source file and routine being ported
- the Rust state field being written
- the plant loop, side, branch, component, node, output, or meter being sampled
- the case manifest and report artifact
- timestamp, warmup, environment-period, and sizing-period handling rules
- the tolerance policy and blocking gate

Until then, plant loop loads, plant node temperatures, mass flow, pump power,
boiler load, chiller load, and plant meters remain baseline-only or
diagnostic-only evidence.
