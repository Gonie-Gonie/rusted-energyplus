//! Source GetZoneEquipment over the same retained InputProcessor.
//! Local getter arrays do not overwrite the unrelated shared IP shortcut.
use super::*;
use crate::heat_balance::ctf_initialization_context::internal_heat_gains_input::SelectedInternalHeatGains;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_input_processor::{SelectedInputProcessor, SourceArgumentCounts};

use super::source_equipment_getter::SourceEquipmentGetterLocals;
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneData;
use crate::heat_balance::selected_schedule_manager::SelectedScheduleManager;
use crate::heat_balance::selected_sizing::{SelectedSizingManager, SourceTerminalUnitSizingData};
use crate::heat_balance::startup_integrity::SourceBranchNodeConnections;

pub(crate) struct SourceEquipmentInputOwners<'a> {
    pub(crate) zones: &'a mut [SelectedZoneData],
    pub(crate) schedules: &'a mut SelectedScheduleManager,
    pub(crate) sizing: &'a mut SelectedSizingManager,
    pub(crate) branch: Option<&'a mut SourceBranchNodeConnections>,
    pub(crate) source_is_epjson: bool,
}

fn definition(
    input: &mut SelectedInputProcessor,
    family: &str,
) -> Result<SourceArgumentCounts, Unavailable> {
    input
        .get_object_def_max_args(family)
        .map_err(|_| Unavailable::Input(family.into()))
}

impl OrdinaryEquipment {
    pub(crate) fn new_for_source(
        model: &ep_model::TypedModel,
        clock: OrdinaryEquipmentInput,
    ) -> Result<Self, Unavailable> {
        if model.zones.len() != 1
            || model.spaces.len() != 1
            || model.spaces[0].origin != ep_model::SpaceOrigin::AutoZoneDefault
            || model.spaces[0].zone != model.zones[0].id
            || model.zones[0].spaces.as_slice() != [model.spaces[0].id]
            || model.timestep.number_of_timesteps_per_hour as usize != clock.timesteps_per_hour
        {
            return Err(Unavailable::GeometryOrTimestep);
        }
        // No second family projection is constructed on the Source route.
        Self::new_owners(None, &[model.zones[0].id], &[model.spaces[0].id])
    }

    pub(crate) fn get_zone_equipment_source(
        &mut self,
        input: &mut SelectedInputProcessor,
        clock: OrdinaryEquipmentInput,
        space_sizing: bool,
        space_simulation: bool,
        globals: &SelectedHeatBalanceGlobals,
        gains: &mut SelectedInternalHeatGains,
        mut owners: SourceEquipmentInputOwners<'_>,
    ) -> Result<(), Unavailable> {
        if self.input.is_some()
            || clock.timesteps_per_hour == 0
            || clock.timesteps_per_hour > 60
            || 60 % clock.timesteps_per_hour != 0
        {
            return Err(Unavailable::InconsistentOwner);
        }
        if !self.get_equipment_input {
            return Ok(());
        }
        self.get_zone_equipment_data_source(input, space_sizing, space_simulation, globals, gains, &mut owners)?;
        self.finish_get_zone_equipment(clock.timesteps_per_hour)?;
        self.validate_initialized_storage(
            &self.zone_ids.clone(),
            &self.space_ids.clone(),
            Some(gains),
        )
    }

    fn get_zone_equipment_data_source(
        &mut self,
        input: &mut SelectedInputProcessor,
        space_sizing: bool,
        space_simulation: bool,
        globals: &SelectedHeatBalanceGlobals,
        gains: &mut SelectedInternalHeatGains,
        owners: &mut SourceEquipmentInputOwners<'_>,
    ) -> Result<(), Unavailable> {
        // Original224-227: the real three counts precede all definitions.
        let controlled_zones = input.get_num_objects_found("ZoneHVAC:EquipmentConnections");
        let controlled_spaces = input.get_num_objects_found("SpaceHVAC:EquipmentConnections");
        self.number_of_equipment_lists = input.get_num_objects_found("ZoneHVAC:EquipmentList");
        let node = definition(input, "NodeList")?;
        let mut node_numbers = vec![0_usize; node.total().map_err(|_| Unavailable::InconsistentOwner)?];
        let first = definition(input, "ZoneHVAC:EquipmentList")?;
        let mut max_alphas = first.alphas;
        let mut max_numbers = first.numerics;
        for family in [
            "ZoneHVAC:EquipmentConnections",
            "SpaceHVAC:EquipmentConnections",
            "AirLoopHVAC:SupplyPath",
            "AirLoopHVAC:ReturnPath",
        ] {
            let counts = definition(input, family)?;
            max_alphas = max_alphas.max(counts.alphas);
            max_numbers = max_numbers.max(counts.numerics);
        }
        // Genuine getter-local arrays, including dimension(0); no identity/OOM claim.
        let mut locals = SourceEquipmentGetterLocals {
            alphas: vec![String::new(); max_alphas],
            numbers: vec![0.0_f64; max_numbers],
            alpha_names: vec![String::new(); max_alphas],
            numeric_names: vec![String::new(); max_numbers],
            alpha_blanks: vec![true; max_alphas],
            numeric_blanks: vec![true; max_numbers],
        };
        if self.supply_paths.is_none() {
            self.number_of_supply_paths = input.get_num_objects_found("AirLoopHVAC:SupplyPath");
            if self.number_of_supply_paths != 0 {
                return Err(Unavailable::Input("positive SupplyPath allocation".into()));
            }
            self.supply_paths = Some(Vec::new());
        }
        if self.return_paths.is_none() {
            self.number_of_return_paths = input.get_num_objects_found("AirLoopHVAC:ReturnPath");
            if self.number_of_return_paths != 0 {
                return Err(Unavailable::Input("positive ReturnPath allocation".into()));
            }
            self.return_paths = Some(Vec::new());
        }
        let num_zones =
            usize::try_from(globals.num_of_zones).map_err(|_| Unavailable::InconsistentOwner)?;
        let num_spaces =
            usize::try_from(globals.num_spaces).map_err(|_| Unavailable::InconsistentOwner)?;
        gains.allocate_source_internal_gains_if_unallocated(num_zones, num_spaces);
        self.zone_config = Some(vec![EquipmentConfig::default(); self.zone_ids.len()]);
        if space_sizing || space_simulation {
            return Err(Unavailable::Input(
                "positive Space equipment configuration allocation".into(),
            ));
        }
        self.equipment_lists = Some(vec![EquipmentList::default(); self.zone_ids.len()]);
        self.equipment_availability = Some(vec![0; self.zone_ids.len()]);
        self.unique_equipment_list_names
            .reserve(self.zone_ids.len());
        // Native281-297 diagnostics precede InitUniqueNodeCheck and node input.
        if self.number_of_equipment_lists != controlled_zones {
            return Err(Unavailable::Input("actual GetZoneEquipmentData282 equipment-list/count severe diagnostic".into()));
        }
        if controlled_zones > num_zones {
            return Err(Unavailable::Input("actual GetZoneEquipmentData292 controlled-zone/count severe diagnostic".into()));
        }
        self.nodes
            .init_unique_check_source("ZoneHVAC:EquipmentConnections", input)?;
        let mut overall_equipment_count = 0_usize;
        let mut local_terminal_unit_sizing_counter = 0_usize;
        for _controlled_zone in 0..controlled_zones {
            locals.get_connections(input, owners.source_is_epjson)?;
            let zone = owners.zones.iter().position(|zone| zone.name == locals.alphas[0])
                .ok_or_else(|| Unavailable::Input("actual controlled Zone name-not-found severe diagnostic".into()))?;
            if owners.zones[zone].is_controlled {
                return Err(Unavailable::Input("actual duplicate controlled Zone severe diagnostic".into()));
            }
            owners.zones[zone].is_controlled = true;
            self.process_source_zone_equipment_input(zone, input,
                owners.branch.as_deref_mut().ok_or_else(|| Unavailable::Input("actual existing BranchNodeConnections owner required".into()))?,
                owners.schedules, &locals, &mut node_numbers,
                &mut local_terminal_unit_sizing_counter, &mut overall_equipment_count)?;
            owners.zones[zone].system_zone_node_number = i32::try_from(
                self.zone_config.as_ref().and_then(|rows| rows.get(zone)).ok_or(Unavailable::InconsistentOwner)?.zone_node
            ).map_err(|_| Unavailable::InconsistentOwner)?;
        }
        if controlled_spaces > 0 {
            return Err(Unavailable::Input("actual controlled-space ignored-input warning or positive getter".into()));
        }
        // Space sizing/simulation was bounded at its earlier actual allocation.
        if local_terminal_unit_sizing_counter > 0 {
            owners.sizing.number_of_air_terminal_units = local_terminal_unit_sizing_counter;
            owners.sizing.terminal_unit_sizing = Some(vec![SourceTerminalUnitSizingData::default(); local_terminal_unit_sizing_counter]);
            for (zone, config) in self.zone_config.as_ref().ok_or(Unavailable::InconsistentOwner)?.iter().enumerate() {
                for number in 0..config.inlet_nodes {
                    let terminal = config.air_distribution_units_cool.as_ref().and_then(|rows| rows.get(number))
                        .map(|row| row.terminal_unit_sizing_index).and_then(|n| n.checked_sub(1)).ok_or(Unavailable::InconsistentOwner)?;
                    owners.sizing.terminal_unit_sizing.as_mut().and_then(|rows| rows.get_mut(terminal))
                        .ok_or(Unavailable::InconsistentOwner)?.controlled_zone_number = i32::try_from(zone + 1).map_err(|_| Unavailable::InconsistentOwner)?;
                }
            }
        }
        if self.equipment_data_errors_found {
            return Err(Unavailable::Input("actual duplicate-items-not-checked warning diagnostic".into()));
        }
        if overall_equipment_count > 0 {
            // Native ObjectType is std::string (initially empty), assigned
            // the unscoped ZoneEquipType through operator=(char), not a type name.
            let mut audit = vec![(String::new(), String::new(), 0usize); overall_equipment_count];
            overall_equipment_count = 0;
            for zone in 0..controlled_zones {
                let list = self.equipment_lists.as_ref().and_then(|rows| rows.get(zone)).ok_or(Unavailable::InconsistentOwner)?;
                for number in 0..list.number_of_equipment_types {
                    let row = audit.get_mut(overall_equipment_count).ok_or(Unavailable::InconsistentOwner)?;
                    overall_equipment_count += 1;
                    let kind = *list.equipment_types.as_ref().and_then(|rows| rows.get(number)).ok_or(Unavailable::InconsistentOwner)?;
                    let character = u8::try_from(kind).ok().filter(u8::is_ascii).ok_or(Unavailable::InconsistentOwner)?;
                    row.0 = char::from(character).to_string();
                    row.1 = list.equipment_names.as_ref().and_then(|rows| rows.get(number)).ok_or(Unavailable::InconsistentOwner)?.clone();
                    row.2 = zone + 1;
                }
            }
            for first in 0..overall_equipment_count {
                for other in first + 1..overall_equipment_count {
                    if audit[first].0 != audit[other].0 || audit[first].1 != audit[other].1 { continue; }
                    return Err(Unavailable::Input("actual duplicate equipment-list item severe diagnostic".into()));
                }
            }
            drop(audit);
        }
        let configs = self
            .zone_config
            .as_mut()
            .ok_or(Unavailable::InconsistentOwner)?;
        for (index, list) in self
            .equipment_lists
            .as_ref()
            .ok_or(Unavailable::InconsistentOwner)?
            .iter()
            .enumerate()
        {
            self.equipment_data_found = configs
                .iter()
                .position(|row| row.equipment_list_name == list.name)
                .map_or(0, |n| n + 1);
            if self.equipment_data_found > 0 {
                configs[self.equipment_data_found - 1].equipment_list_index = index + 1;
            }
        }
        self.nodes
            .end_unique_check("ZoneHVAC:EquipmentConnections")?;
        // Actual exact-case finds; a present Null still enters Native's warning branch.
        for family in [
            "SpaceHVAC:ZoneEquipmentSplitter",
            "SpaceHVAC:ZoneEquipmentMixer",
            "SpaceHVAC:ZoneReturnMixer",
        ] {
            if input.find_exact_family(family).is_some() {
                return Err(Unavailable::Input(format!(
                    "present Space equipment find branch: {family}"
                )));
            }
        }
        // Actual zero supply/return item loops, then local deallocation794-799.
        drop(locals.alphas);
        drop(locals.numbers);
        drop(locals.alpha_names);
        drop(locals.numeric_names);
        drop(locals.alpha_blanks);
        drop(locals.numeric_blanks);
        if self.equipment_data_errors_found {
            return Err(Unavailable::Fatal(
                "GetZoneEquipmentData: Errors found in getting Zone Equipment input.".into(),
            ));
        }
        Ok(())
    }
}

impl NodeOwner {
    fn init_unique_check_source(
        &mut self,
        context: &str,
        input: &mut SelectedInputProcessor,
    ) -> Result<(), Unavailable> {
        if self.get_input {
            let mut errors_found = false;
            self.get_source_node_lists_input(input, &mut errors_found)?;
            self.get_input = false;
        }
        self.begin_unique_check_context(context)
    }
}
