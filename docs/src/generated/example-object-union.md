<!-- DO NOT EDIT. Generated from repository samples, the pinned object catalog and specs/object_coverage.toml by tools/docs/generate_docs.py. -->

# Example Object Union

The selected corpus has **187 files** (175 IDF, 12 epJSON), **110 canonical object types**, and **8978 instances** of those types.

Selection includes tracked and ordinary untracked input files under `data/` and `docs/validation/`, respecting Git ignores. It excludes runtime/reference trees and immutable `objects` stores. Invalid-field and negative fixtures remain included; embedded code literals and compressed artifacts are outside this file corpus.

This is an occurrence inventory, not schema validation or evidence that arbitrary fields, values, object combinations or runtime consumers are supported. Counts include inactive and unused declarations. A typing status is copied from the existing [object registry](../../../specs/object_coverage.toml); unregistered means that registry has no reviewed entry, even when a separate runtime getter exists. [Capabilities](../../../specs/capabilities.toml) govern admitted runtime combinations.

Review groups provide a development queue, not IDF declaration order or runtime execution order. Cross-group references and cycles require explicit register, resolve and consume phases. The [roadmap](../current/roadmap.md) defines the expansion work and [project contract](../current/project-contract.md#reviewing-a-porting-change) defines its evidence requirements.

Canonical names and official groups come from the compact [EnergyPlus 26.1 catalog](../../../data/schema/energyplus-26.1-object-index.json). It records the original schema SHA256 and pinned source commit; installed EnergyPlus is not needed to regenerate this page.

| Object type | Input files | Instances | Dependency review group | Existing typing status |
|---|---|---|---|---|
| `Building` | 174 | 174 | 01 Project, time and site | typed |
| `GlobalGeometryRules` | 164 | 164 | 01 Project, time and site | unregistered / no reviewed entry |
| `HeatBalanceAlgorithm` | 69 | 69 | 01 Project, time and site | unregistered / no reviewed entry |
| `RunPeriod` | 168 | 168 | 01 Project, time and site | typed |
| `RunPeriodControl:DaylightSavingTime` | 1 | 1 | 01 Project, time and site | typed |
| `RunPeriodControl:SpecialDays` | 18 | 24 | 01 Project, time and site | typed |
| `SimulationControl` | 114 | 114 | 01 Project, time and site | unregistered / no reviewed entry |
| `Site:GroundTemperature:BuildingSurface` | 15 | 15 | 01 Project, time and site | unregistered / no reviewed entry |
| `Site:Location` | 116 | 116 | 01 Project, time and site | typed |
| `SizingPeriod:DesignDay` | 42 | 81 | 01 Project, time and site | unregistered / no reviewed entry |
| `SurfaceConvectionAlgorithm:Inside` | 69 | 69 | 01 Project, time and site | unregistered / no reviewed entry |
| `SurfaceConvectionAlgorithm:Outside` | 69 | 69 | 01 Project, time and site | unregistered / no reviewed entry |
| `Timestep` | 174 | 174 | 01 Project, time and site | typed |
| `Version` | 185 | 185 | 01 Project, time and site | typed |
| `ZoneAirHeatBalanceAlgorithm` | 53 | 53 | 01 Project, time and site | unregistered / no reviewed entry |
| `ExternalInterface:FunctionalMockupUnitExport:To:Schedule` | 3 | 3 | 02 Schedules | typed |
| `ExternalInterface:FunctionalMockupUnitImport:To:Schedule` | 1 | 1 | 02 Schedules | typed |
| `ExternalInterface:Schedule` | 1 | 1 | 02 Schedules | typed |
| `Schedule:Compact` | 53 | 77 | 02 Schedules | typed |
| `Schedule:Constant` | 93 | 200 | 02 Schedules | typed |
| `Schedule:Day:Hourly` | 2 | 6 | 02 Schedules | typed |
| `Schedule:Day:Interval` | 2 | 5 | 02 Schedules | typed |
| `Schedule:Day:List` | 2 | 5 | 02 Schedules | typed |
| `Schedule:File` | 1 | 1 | 02 Schedules | typed |
| `Schedule:File:Shading` | 1 | 1 | 02 Schedules | typed |
| `Schedule:Week:Compact` | 1 | 1 | 02 Schedules | typed |
| `Schedule:Week:Daily` | 3 | 4 | 02 Schedules | typed |
| `Schedule:Year` | 4 | 4 | 02 Schedules | typed |
| `ScheduleTypeLimits` | 110 | 243 | 02 Schedules | typed |
| `Construction` | 112 | 215 | 03 Materials and constructions | typed |
| `Construction:WindowEquivalentLayer` | 6 | 22 | 03 Materials and constructions | typed |
| `Material` | 100 | 102 | 03 Materials and constructions | typed |
| `Material:AirGap` | 1 | 2 | 03 Materials and constructions | typed |
| `Material:InfraredTransparent` | 1 | 1 | 03 Materials and constructions | typed |
| `Material:NoMass` | 51 | 90 | 03 Materials and constructions | typed |
| `Material:RoofVegetation` | 1 | 3 | 03 Materials and constructions | typed |
| `WindowGap:DeflectionState` | 1 | 1 | 03 Materials and constructions | unregistered / no reviewed entry |
| `WindowGap:SupportPillar` | 1 | 1 | 03 Materials and constructions | unregistered / no reviewed entry |
| `WindowMaterial:Blind` | 1 | 3 | 03 Materials and constructions | typed |
| `WindowMaterial:Blind:EquivalentLayer` | 1 | 3 | 03 Materials and constructions | typed |
| `WindowMaterial:ComplexShade` | 1 | 7 | 03 Materials and constructions | typed |
| `WindowMaterial:Drape:EquivalentLayer` | 1 | 5 | 03 Materials and constructions | typed |
| `WindowMaterial:Gap` | 1 | 3 | 03 Materials and constructions | typed |
| `WindowMaterial:Gap:EquivalentLayer` | 5 | 11 | 03 Materials and constructions | typed |
| `WindowMaterial:Gas` | 2 | 7 | 03 Materials and constructions | typed |
| `WindowMaterial:GasMixture` | 2 | 7 | 03 Materials and constructions | typed |
| `WindowMaterial:Glazing` | 6 | 6 | 03 Materials and constructions | unregistered / no reviewed entry |
| `WindowMaterial:Glazing:EquivalentLayer` | 6 | 6 | 03 Materials and constructions | typed |
| `WindowMaterial:Glazing:RefractionExtinctionMethod` | 1 | 1 | 03 Materials and constructions | typed |
| `WindowMaterial:Screen` | 1 | 3 | 03 Materials and constructions | typed |
| `WindowMaterial:Screen:EquivalentLayer` | 1 | 3 | 03 Materials and constructions | typed |
| `WindowMaterial:Shade` | 1 | 2 | 03 Materials and constructions | typed |
| `WindowMaterial:Shade:EquivalentLayer` | 1 | 3 | 03 Materials and constructions | typed |
| `WindowMaterial:SimpleGlazingSystem` | 1 | 3 | 03 Materials and constructions | typed |
| `BuildingSurface:Detailed` | 112 | 670 | 04 Zones, surfaces and gains | unregistered / no reviewed entry |
| `FenestrationSurface:Detailed` | 13 | 35 | 04 Zones, surfaces and gains | unregistered / no reviewed entry |
| `OtherEquipment` | 85 | 123 | 04 Zones, surfaces and gains | typed |
| `People` | 3 | 3 | 04 Zones, surfaces and gains | typed |
| `Zone` | 114 | 120 | 04 Zones, surfaces and gains | typed |
| `AirLoopHVAC` | 3 | 3 | 05 Controls, nodes and equipment | unregistered / no reviewed entry |
| `AvailabilityManager:Scheduled` | 3 | 3 | 05 Controls, nodes and equipment | unregistered / no reviewed entry |
| `Coil:Cooling:DX:SingleSpeed` | 1 | 1 | 05 Controls, nodes and equipment | unregistered / no reviewed entry |
| `Coil:Heating:Electric` | 2 | 2 | 05 Controls, nodes and equipment | unregistered / no reviewed entry |
| `DesignSpecification:OutdoorAir` | 13 | 13 | 05 Controls, nodes and equipment | diagnostic |
| `EnergyManagementSystem:ConstructionIndexVariable` | 6 | 11 | 05 Controls, nodes and equipment | unregistered / no reviewed entry |
| `Fan:ConstantVolume` | 1 | 1 | 05 Controls, nodes and equipment | unregistered / no reviewed entry |
| `Fan:OnOff` | 1 | 1 | 05 Controls, nodes and equipment | unregistered / no reviewed entry |
| `Fan:VariableVolume` | 1 | 1 | 05 Controls, nodes and equipment | unregistered / no reviewed entry |
| `NodeList` | 46 | 46 | 05 Controls, nodes and equipment | typed_graph_only |
| `OutdoorAir:Node` | 12 | 12 | 05 Controls, nodes and equipment | unregistered / no reviewed entry |
| `SetpointManager:Scheduled` | 4 | 4 | 05 Controls, nodes and equipment | unregistered / no reviewed entry |
| `ThermostatSetpoint:DualSetpoint` | 45 | 45 | 05 Controls, nodes and equipment | typed |
| `ZoneControl:ContaminantController` | 1 | 1 | 05 Controls, nodes and equipment | unregistered / no reviewed entry |
| `ZoneControl:Humidistat` | 4 | 4 | 05 Controls, nodes and equipment | typed |
| `ZoneControl:Thermostat` | 45 | 45 | 05 Controls, nodes and equipment | typed |
| `ZoneHVAC:EquipmentConnections` | 45 | 45 | 05 Controls, nodes and equipment | unregistered / no reviewed entry |
| `ZoneHVAC:EquipmentList` | 45 | 45 | 05 Controls, nodes and equipment | unregistered / no reviewed entry |
| `ZoneHVAC:IdealLoadsAirSystem` | 45 | 45 | 05 Controls, nodes and equipment | typed |
| `ZoneHVAC:PackagedTerminalAirConditioner` | 1 | 1 | 05 Controls, nodes and equipment | unregistered / no reviewed entry |
| `Boiler:HotWater` | 1 | 1 | 06 Plant and loops | typed_graph_only |
| `Branch` | 5 | 13 | 06 Plant and loops | unregistered / no reviewed entry |
| `BranchList` | 5 | 8 | 06 Plant and loops | unregistered / no reviewed entry |
| `Chiller:Electric:EIR` | 1 | 1 | 06 Plant and loops | typed_graph_only |
| `Connector:Mixer` | 2 | 3 | 06 Plant and loops | unregistered / no reviewed entry |
| `Connector:Splitter` | 2 | 3 | 06 Plant and loops | unregistered / no reviewed entry |
| `ConnectorList` | 2 | 3 | 06 Plant and loops | unregistered / no reviewed entry |
| `DistrictHeating:Water` | 1 | 1 | 06 Plant and loops | unregistered / no reviewed entry |
| `LoadProfile:Plant` | 1 | 1 | 06 Plant and loops | unregistered / no reviewed entry |
| `Pipe:Adiabatic` | 1 | 3 | 06 Plant and loops | unregistered / no reviewed entry |
| `PlantEquipmentList` | 1 | 1 | 06 Plant and loops | unregistered / no reviewed entry |
| `PlantEquipmentOperation:HeatingLoad` | 1 | 1 | 06 Plant and loops | unregistered / no reviewed entry |
| `PlantEquipmentOperationSchemes` | 1 | 1 | 06 Plant and loops | unregistered / no reviewed entry |
| `PlantLoop` | 2 | 2 | 06 Plant and loops | typed_graph_only |
| `Pump:ConstantSpeed` | 1 | 1 | 06 Plant and loops | typed_graph_only |
| `Pump:VariableSpeed` | 1 | 1 | 06 Plant and loops | unregistered / no reviewed entry |
| `Output:Constructions` | 57 | 57 | 07 Reporting | unregistered / no reviewed entry |
| `Output:Diagnostics` | 43 | 43 | 07 Reporting | unregistered / no reviewed entry |
| `Output:Meter` | 13 | 26 | 07 Reporting | unregistered / no reviewed entry |
| `Output:Meter:MeterFileOnly` | 38 | 114 | 07 Reporting | unregistered / no reviewed entry |
| `Output:SQLite` | 1 | 1 | 07 Reporting | unregistered / no reviewed entry |
| `Output:Schedules` | 1 | 1 | 07 Reporting | unregistered / no reviewed entry |
| `Output:Surfaces:Drawing` | 38 | 38 | 07 Reporting | unregistered / no reviewed entry |
| `Output:Surfaces:List` | 17 | 17 | 07 Reporting | unregistered / no reviewed entry |
| `Output:Table:SummaryReports` | 39 | 39 | 07 Reporting | unregistered / no reviewed entry |
| `Output:Variable` | 157 | 4612 | 07 Reporting | typed |
| `Output:VariableDictionary` | 84 | 84 | 07 Reporting | unregistered / no reviewed entry |
| `OutputControl:Table:Style` | 39 | 39 | 07 Reporting | unregistered / no reviewed entry |
| `Exterior:Lights` | 38 | 38 | 08 Other dependencies to review | unregistered / no reviewed entry |
| `WindowShadingControl` | 3 | 6 | 08 Other dependencies to review | unregistered / no reviewed entry |
| `ZoneAirContaminantBalance` | 1 | 1 | 08 Other dependencies to review | unregistered / no reviewed entry |

## Unknown Input Types

These names are absent from the pinned schema. `Unknown:Diagnostic` is an intentional local diagnostic fixture, not an EnergyPlus object type.

| Input type | Input files | Instances |
|---|---|---|
| `Unknown:Diagnostic` | 1 | 1 |
