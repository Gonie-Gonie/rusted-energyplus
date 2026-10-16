//! Genuine pre-primary connection reports over the existing retained owners.
use super::{Error, SourceBranchNodeConnections};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::ordinary_equipment::OrdinaryEquipment;
use crate::heat_balance::ordinary_hvac_manager::SourceOutdoorAirNodes;
use crate::heat_balance::ordinary_hvac_manager::simulation::state::SourcePlantManager;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_simulation_startup::SelectedHvacGlobals;
use std::convert::Infallible;
use std::io::Write;

const SEPARATOR: &str = "! ===============================================================";
const LOOP_FORMAT_700: &str = "! <#Component Sets>,<Number of Component Sets>";
const LOOP_FORMAT_702: &str = "! <Component Set>,<Component Set Count>,<Parent Object Type>,<Parent Object Name>,<Component Type>,<Component Name>,<Inlet Node ID>,<Outlet Node ID>,<Description>";
const LOOP_FORMAT_720: &str = "! <#Zone Equipment Lists>,<Number of Zone Equipment Lists>";
const LOOP_FORMAT_722: &str = "! <Zone Equipment List>,<Zone Equipment List Count>,<Zone Equipment List Name>,<Zone Name>,<Number of Components>";
const LOOP_FORMAT_723: &str = "! <Zone Equipment Component>,<Component Count>,<Component Type>,<Component Name>,<Zone Name>,<Heating Priority>,<Cooling Priority>";
const AIR_FORMAT_706: &str = "! <#AirLoopHVACs>,<Number of AirLoopHVACs>";
const AIR_FORMAT_708: &str = "! <AirLoopHVAC>,<Air Loop Name>,<# Return Nodes>,<# Supply Nodes>,<# Zones Cooled>,<# Zones Heated>,<Outdoor Air Used>";
const AIR_FORMAT_709: &str = "! <AirLoop Return Connections>,<Connection Count>,<AirLoopHVAC Name>,<Zn Eqp Return Node #>,<Zn Eqp Return Node Name>,<AirLoop Return Node #>,<Air Loop Return Node Name>";
const AIR_FORMAT_710: &str = "! <AirLoop Supply Connections>,<Connection Count>,<AirLoopHVAC Name>,<Zn Eqp Supply Node #>,<Zn Eqp Supply Node Name>,<AirLoop Supply Node #>,<Air Loop Supply Node Name>";
const AIR_FORMAT_711: &str = "! <Cooled Zone Info>,<Cooled Zone Count>,<Cooled Zone Name>,<Cooled Zone Inlet Node #>,<Cooled Zone Inlet Node Name>,<AirLoopHVAC Name>";
const AIR_FORMAT_712: &str = "! <Heated Zone Info>,<Heated Zone Count>,<Heated Zone Name>,<Heated Zone Inlet Node #>,<Heated Zone Inlet Node Name>,<AirLoopHVAC Name>";
const AIR_FORMAT_714: &str = "! <Outdoor Air Connections>,<OA Inlet Node #>,<OA Return Air Inlet Node Name>,<OA Outlet Node #>,<OA Mixed Air Outlet Node Name>,<AirLoopHVAC Name>";

fn line(bnd: &mut dyn Write, text: &str) -> Result<(), Error> {
    writeln!(bnd, "{text}")?;
    Ok(())
}

/// Same independent DualDuctData fields, constructorzero and array unallocated.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceDualDuctData {
    number_air_terminals: i32,
    air_terminals: Option<Vec<Infallible>>,
}
impl Default for SourceDualDuctData {
    fn default() -> Self {
        Self {
            number_air_terminals: 0,
            air_terminals: None,
        }
    }
}
impl SourceDualDuctData {
    /// Actual DualDuct1859-2003 allocated-array guard; this does not get input.
    fn report_source_connections(
        &self,
        hvac: &SelectedHvacGlobals,
        equipment: &OrdinaryEquipment,
        bnd: &mut dyn Write,
    ) -> Result<(), Error> {
        let number_primary_air_systems = hvac.number_of_primary_air_systems;
        if self.air_terminals.is_none() {
            return Ok(());
        }
        line(bnd, SEPARATOR)?;
        line(
            bnd,
            "! <#Dual Duct Damper Connections>,<Number of Dual Duct Damper Connections>",
        )?;
        let connections = self
            .number_air_terminals
            .checked_mul(2)
            .ok_or(Error::Unavailable(
                "actual DualDuct1888 signed NumDDAirTerminal*2 domain",
            ))?;
        writeln!(bnd, " #Dual Duct Damper Connections,{connections}")?;
        line(
            bnd,
            "! <Dual Duct Damper>,<Dual Duct Damper Count>,<Dual Duct Damper Name>,<Inlet Node>,<Outlet Node>,<Inlet Node Type>,<AirLoopHVAC Name>",
        )?;
        for _terminal in 1..=self.number_air_terminals {
            let mut found = 0;
            let mut supply_air_path_number = 0;
            for path in 1..=*equipment.source_number_supply_paths() {
                supply_air_path_number = path;
                found = 0;
                let _ = (supply_air_path_number, found);
                return Err(Error::Unavailable(
                    "actual DualDuct1899 SupplyAirPath.NumOutletNodes",
                ));
            }
            if found == 0 {
                supply_air_path_number = 0;
            }
            found = 0;
            for _system in 1..=number_primary_air_systems {
                return Err(Error::Unavailable(
                    "actual DualDuct1928 AirToZoneNodeInfo.AirLoopName",
                ));
            }
            let mut air_loop_name = None;
            if found == 0 {
                air_loop_name = Some("**Unknown**");
            }
            let _ = air_loop_name;
            let _ = supply_air_path_number;
            return Err(Error::Unavailable(
                "actual DualDuct1963 dd_airterminal.DamperType",
            ));
        }
        Ok(())
    }
}

/// Short views of the same module/zone/node counters; none is stored or copied.
pub(crate) struct SourceLoopConnectionReportOwners<'a> {
    pub(crate) connections: &'a SourceBranchNodeConnections,
    pub(crate) outdoor_air_nodes: &'a SourceOutdoorAirNodes,
    pub(crate) hvac: &'a SelectedHvacGlobals,
    pub(crate) plant: &'a SourcePlantManager,
    pub(crate) equipment: &'a OrdinaryEquipment,
    pub(crate) globals: &'a SelectedHeatBalanceGlobals,
    pub(crate) dual_duct: &'a SourceDualDuctData,
}

/// Literal SimulationManager2206-2775 reached report sections and final flag.
pub(crate) fn report_source_loop_connections(
    o: &SourceLoopConnectionReportOwners<'_>,
    bnd: &mut dyn Write,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), Error> {
    let node_connection_error_flag = false;
    line(bnd, SEPARATOR)?;
    line(bnd, "! #Outdoor Air Nodes,<Number of Outdoor Air Nodes>")?;
    i32::try_from(o.outdoor_air_nodes.number_outside_air_nodes).map_err(|_| {
        Error::Unavailable("actual ReportLoopConnections2237 NumOutsideAirNodes signed domain")
    })?;
    writeln!(
        bnd,
        " #Outdoor Air Nodes,{}",
        o.outdoor_air_nodes.number_outside_air_nodes
    )?;
    if o.outdoor_air_nodes.number_outside_air_nodes > 0 {
        line(bnd, "! <Outdoor Air Node>,<NodeNumber>,<Node Name>")?;
    }
    for ordinal in 1..=o.outdoor_air_nodes.number_outside_air_nodes {
        let _node = o
            .outdoor_air_nodes
            .outside_air_node_list
            .as_ref()
            .and_then(|rows| rows.get(ordinal - 1))
            .ok_or(Error::Unavailable(
                "actual ReportLoopConnections2244 OutsideAirNodeList",
            ))?;
        return Err(Error::Unavailable(
            "actual ReportLoopConnections2245 NodeID",
        ));
    }
    line(bnd, SEPARATOR)?;
    line(bnd, LOOP_FORMAT_700)?;
    writeln!(
        bnd,
        " #Component Sets,{}",
        o.connections.source_number_component_sets()
    )?;
    line(bnd, LOOP_FORMAT_702)?;
    for _set in 1..=*o.connections.source_number_component_sets() {
        return Err(Error::Unavailable(
            "actual ReportLoopConnections2257 CompSets.ParentObjectType",
        ));
    }
    // Both component passes are governed by the same actual count. The first
    // positive read above precedes every warning/counter/error flag operation.
    line(bnd, SEPARATOR)?;
    line(bnd, "! <# Plant Loops>,<Number of Plant Loops>")?;
    writeln!(bnd, " #Plant Loops,{}", o.hvac.number_plant_loops)?;
    line(
        bnd,
        "! <Plant Loop>,<Plant Loop Name>,<Loop Type>,<Inlet Node Name>,<Outlet Node Name>,<Branch List>,<Connector List>",
    )?;
    line(
        bnd,
        "! <Plant Loop Connector>,<Connector Type>,<Connector Name>,<Loop Name>,<Loop Type>,<Number of Inlets/Outlets>",
    )?;
    line(
        bnd,
        "! <Plant Loop Connector Branches>,<Connector Node Count>,<Connector Type>,<Connector Name>,<Inlet Branch>,<Outlet Branch>,<Loop Name>,<Loop Type>",
    )?;
    line(
        bnd,
        "! <Plant Loop Connector Nodes>,<Connector Node Count>,<Connector Type>,<Connector Name>,<Inlet Node>,<Outlet Node>,<Loop Name>,<Loop Type>",
    )?;
    line(
        bnd,
        "! <Plant Loop Supply Connection>,<Plant Loop Name>,<Supply Side Outlet Node Name>,<Demand Side Inlet Node Name>",
    )?;
    line(
        bnd,
        "! <Plant Loop Return Connection>,<Plant Loop Name>,<Demand Side Outlet Node Name>,<Supply Side Inlet Node Name>",
    )?;
    for _loop_number in 1..=o.hvac.number_plant_loops {
        return Err(Error::Unavailable(
            "actual ReportLoopConnections2380 PlantLoop.Name",
        ));
    }
    line(bnd, SEPARATOR)?;
    line(bnd, "! <# Condenser Loops>,<Number of Condenser Loops>")?;
    writeln!(bnd, " #Condenser Loops,{}", o.hvac.number_condenser_loops)?;
    line(
        bnd,
        "! <Condenser Loop>,<Condenser Loop Name>,<Loop Type>,<Inlet Node Name>,<Outlet Node Name>,<Branch List>,<Connector List>",
    )?;
    line(
        bnd,
        "! <Condenser Loop Connector>,<Connector Type>,<Connector Name>,<Loop Name>,<Loop Type>,<Number of Inlets/Outlets>",
    )?;
    line(
        bnd,
        "! <Condenser Loop Connector Branches>,<Connector Node Count>,<Connector Type>,<Connector Name>,<Inlet Branch>,<Outlet Branch>,<Loop Name>,<Loop Type>",
    )?;
    line(
        bnd,
        "! <Condenser Loop Connector Nodes>,<Connector Node Count>,<Connector Type>,<Connector Name>,<Inlet Node>,<Outlet Node>,<Loop Name>,<Loop Type>",
    )?;
    line(
        bnd,
        "! <Condenser Loop Supply Connection>,<Condenser Loop Name>,<Supply Side Outlet Node Name>,<Demand Side Inlet Node Name>",
    )?;
    line(
        bnd,
        "! <Condenser Loop Return Connection>,<Condenser Loop Name>,<Demand Side Outlet Node Name>,<Supply Side Inlet Node Name>",
    )?;
    let condenser_first = o
        .hvac
        .number_plant_loops
        .checked_add(1)
        .ok_or(Error::Unavailable(
            "actual ReportLoopConnections2520 NumPlantLoops+1 signed domain",
        ))?;
    for _loop_number in condenser_first..=o.plant.total_loops {
        return Err(Error::Unavailable(
            "actual ReportLoopConnections2536 PlantLoop.Name",
        ));
    }
    line(bnd, SEPARATOR)?;
    let mut number_controlled_zones: i32 = 0;
    for number in 1..=o.globals.num_of_zones {
        if !o.equipment.source_zone_equipment_config_allocated() {
            continue;
        }
        if *o
            .equipment
            .source_zone_equipment_config(number)?
            .is_controlled()
        {
            number_controlled_zones =
                number_controlled_zones
                    .checked_add(1)
                    .ok_or(Error::Unavailable(
                        "actual ReportLoopConnections2660 controlled-zone increment domain",
                    ))?;
        }
    }
    if number_controlled_zones > 0 {
        line(bnd, "! <# Controlled Zones>,<Number of Controlled Zones>")?;
        writeln!(bnd, " #Controlled Zones,{number_controlled_zones}")?;
        line(
            bnd,
            "! <Controlled Zone>,<Controlled Zone Name>,<Equip List Name>,<Control List Name>,<Zone Node Name>,<# Inlet Nodes>,<# Exhaust Nodes>,<# Return Nodes>",
        )?;
        line(
            bnd,
            "! <Controlled Zone Inlet>,<Inlet Node Count>,<Controlled Zone Name>,<Supply Air Inlet Node Name>,<SD Sys:Cooling/Heating [DD:Cooling] Inlet Node Name>,<DD Sys:Heating Inlet Node Name>",
        )?;
        line(
            bnd,
            "! <Controlled Zone Exhaust>,<Exhaust Node Count>,<Controlled Zone Name>,<Exhaust Air Node Name>",
        )?;
        for number in 1..=o.globals.num_of_zones {
            if !*o
                .equipment
                .source_zone_equipment_config(number)?
                .is_controlled()
            {
                continue;
            }
            return Err(Error::Unavailable(
                "actual ReportLoopConnections2684 controlled ZoneEquipConfig.ZoneName",
            ));
        }
        line(bnd, SEPARATOR)?;
        line(bnd, LOOP_FORMAT_720)?;
        writeln!(bnd, " #Zone Equipment Lists,{number_controlled_zones}")?;
        line(bnd, LOOP_FORMAT_722)?;
        line(bnd, LOOP_FORMAT_723)?;
        for number in 1..=o.globals.num_of_zones {
            if !*o
                .equipment
                .source_zone_equipment_config(number)?
                .is_controlled()
            {
                continue;
            }
            return Err(Error::Unavailable(
                "actual ReportLoopConnections2737 ZoneEquipList.Name",
            ));
        }
    }
    o.dual_duct
        .report_source_connections(o.hvac, o.equipment, bnd)?;
    if *o.connections.source_number_connection_errors() == 0 {
        backend.show_message("No node connection errors were found.")?;
    } else if *o.connections.source_number_connection_errors() > 1 {
        backend.show_message(&format!(
            "There were {} node connection errors noted.",
            o.connections.source_number_connection_errors()
        ))?;
    } else {
        backend.show_message(&format!(
            "There was {} node connection error noted.",
            o.connections.source_number_connection_errors()
        ))?;
    }
    backend.state.tracking.ask_for_connections_report = false;
    if node_connection_error_flag {
        return match backend.show_fatal_error("Please see severe error(s) and correct either the branch nodes or the component nodes so that they match.") {
            Ok(never) => match never {}, Err(stop) => Err(Error::Fatal(stop)),
        };
    }
    Ok(())
}

/// Literal SystemReports4721-4927 zero-primary route, with all BND headers.
pub(crate) fn report_source_air_loop_connections(
    hvac: &SelectedHvacGlobals,
    bnd: &mut dyn Write,
) -> Result<(), Error> {
    line(bnd, SEPARATOR)?;
    line(bnd, AIR_FORMAT_706)?;
    writeln!(bnd, " #AirLoopHVACs,{}", hvac.number_of_primary_air_systems)?;
    line(bnd, AIR_FORMAT_708)?;
    line(bnd, AIR_FORMAT_709)?;
    line(bnd, AIR_FORMAT_710)?;
    line(bnd, AIR_FORMAT_711)?;
    line(bnd, AIR_FORMAT_712)?;
    line(bnd, AIR_FORMAT_714)?;
    line(
        bnd,
        "! <AirLoopHVAC Connector>,<Connector Type>,<Connector Name>,<Loop Name>,<Loop Type>,<Number of Inlets/Outlets>",
    )?;
    line(
        bnd,
        "! <AirLoopHVAC Connector Branches>,<Connector Node Count>,<Connector Type>,<Connector Name>,<Inlet Branch>,<Outlet Branch>,<Loop Name>,<Loop Type>",
    )?;
    line(
        bnd,
        "! <AirLoopHVAC Connector Nodes>,<Connector Node Count>,<Connector Type>,<Connector Name>,<Inlet Node>,<Outlet Node>,<Loop Name>,<Loop Type>",
    )?;
    for _number in 1..=hvac.number_of_primary_air_systems {
        return Err(Error::Unavailable(
            "actual ReportAirLoopConnections4774 AirToOANodeInfo.OASysExists",
        ));
    }
    Ok(())
}
