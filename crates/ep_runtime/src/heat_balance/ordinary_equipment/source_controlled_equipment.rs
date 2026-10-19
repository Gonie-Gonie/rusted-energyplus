//! Native processZoneEquipmentInput814-1260, finite one-zone IdealLoads input.
use super::source_equipment_getter::{SourceEquipmentGetterLocals, alpha, integer, sole_object};
use super::source_nodes::SourceNodeFluidType;
use super::*;
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputStatisticsValue as V,
};
use crate::heat_balance::selected_schedule_manager::SelectedScheduleManager;
use crate::heat_balance::startup_integrity::{
    SourceBranchNodeConnections, SourceCompFluidStream as Stream,
    SourceNodeConnectionType as Connection, SourceNodeObjectType as ObjectType,
};
fn unavailable(text: &'static str) -> Unavailable {
    Unavailable::Input(text.into())
}
fn schedule_error(error: impl std::fmt::Display) -> Unavailable {
    Unavailable::Input(error.to_string())
}
impl OrdinaryEquipment {
    fn source_config_mut(&mut self, index: usize) -> Result<&mut EquipmentConfig, Unavailable> {
        self.zone_config
            .as_mut()
            .and_then(|rows| rows.get_mut(index))
            .ok_or(Unavailable::InconsistentOwner)
    }
    pub(super) fn process_source_zone_equipment_input(
        &mut self,
        index: usize,
        input: &mut SelectedInputProcessor,
        branch: &mut SourceBranchNodeConnections,
        schedules: &mut SelectedScheduleManager,
        locals: &SourceEquipmentGetterLocals,
        node_numbers: &mut [usize],
        terminal_counter: &mut usize,
        overall_equipment_count: &mut usize,
    ) -> Result<(), Unavailable> {
        self.source_config_mut(index)?.is_controlled = true;
        self.source_config_mut(index)?.zone_name = locals.alphas[0].clone();
        // Genuine IntraObjUniquenessCheck success branch on the same set.
        if locals.alphas[1].is_empty() {
            return Err(unavailable(
                "actual blank equipment-list-name severe diagnostic",
            ));
        }
        if !self
            .unique_equipment_list_names
            .insert(locals.alphas[1].clone())
        {
            return Err(unavailable(
                "actual duplicate controlled equipment-list severe diagnostic",
            ));
        }
        self.source_config_mut(index)?.equipment_list_name = locals.alphas[1].clone();
        let inlet_name = locals.alphas[2].clone();
        let exhaust_name = locals.alphas[3].clone();
        let zone_node = self.nodes.get_source_only_single_node(
            input,
            branch,
            &locals.alphas[4],
            &mut self.equipment_data_errors_found,
            ObjectType::ZoneHvacEquipmentConnections,
            &locals.alphas[0],
            SourceNodeFluidType::Air,
            Connection::ZoneNode,
            Stream::Primary,
            false,
            "",
        )?;
        self.source_config_mut(index)?.zone_node = zone_node;
        if zone_node == 0 {
            return Err(unavailable(
                "actual missing Zone air node severe diagnostic",
            ));
        }
        let mut unique_error = false;
        self.nodes.check_source_unique_node_names(
            &locals.alpha_names[4],
            &mut unique_error,
            &locals.alphas[4],
            &locals.alphas[0],
        )?;
        if unique_error {
            self.equipment_data_errors_found = true;
        }
        let return_name = locals.alphas[5].clone();
        self.source_config_mut(index)?.return_flow_fraction_schedule = if locals.alpha_blanks[6] {
            Some(schedules.always_on().map_err(schedule_error)?)
        } else {
            let schedule = schedules
                .get_source_schedule(&locals.alphas[6])
                .map_err(schedule_error)?;
            if schedule.is_none() {
                return Err(unavailable(
                    "actual return-flow-fraction schedule not-found severe diagnostic",
                ));
            }
            schedule
        };
        let return_basis_name = locals.alphas[7].clone();
        self.read_source_zone_equipment_list(index, input, schedules, overall_equipment_count)?;

        let mut node_list_error = false;
        let mut number_nodes = 0usize;
        self.nodes.get_source_node_nums(
            input,
            branch,
            &inlet_name,
            &mut number_nodes,
            node_numbers,
            &mut node_list_error,
            SourceNodeFluidType::Air,
            ObjectType::ZoneHvacEquipmentConnections,
            &locals.alphas[0],
            Connection::ZoneInlet,
            Stream::Primary,
            false,
            false,
            "",
        )?;
        if node_list_error {
            return Err(unavailable(
                "actual invalid inlet NodeList continuation diagnostic",
            ));
        }
        let row = self.source_config_mut(index)?;
        row.inlet_nodes = number_nodes;
        row.inlet_node_numbers = Some(vec![0; number_nodes]);
        row.inlet_node_air_loop_numbers = Some(vec![0; number_nodes]);
        row.inlet_node_air_distribution_unit_numbers = Some(vec![0; number_nodes]);
        row.air_distribution_units_cool = Some(vec![SourceAirIn::default(); number_nodes]);
        row.air_distribution_units_heat = Some(vec![SourceAirIn::default(); number_nodes]);
        for number in 0..number_nodes {
            let node = *node_numbers
                .get(number)
                .ok_or(Unavailable::InconsistentOwner)?;
            self.source_config_mut(index)?
                .inlet_node_numbers
                .as_mut()
                .ok_or(Unavailable::InconsistentOwner)?[number] = node;
            let mut unique_error = false;
            self.nodes.check_source_unique_node_numbers(
                "Zone Air Inlet Nodes",
                &mut unique_error,
                node,
                &locals.alphas[0],
            )?;
            if unique_error {
                self.equipment_data_errors_found = true;
            }
            let row = self.source_config_mut(index)?;
            row.inlet_node_air_loop_numbers
                .as_mut()
                .ok_or(Unavailable::InconsistentOwner)?[number] = 0;
            row.inlet_node_air_distribution_unit_numbers
                .as_mut()
                .ok_or(Unavailable::InconsistentOwner)?[number] = 0;
            row.air_distribution_units_cool
                .as_mut()
                .ok_or(Unavailable::InconsistentOwner)?[number]
                .inlet_node = 0;
            row.air_distribution_units_heat
                .as_mut()
                .ok_or(Unavailable::InconsistentOwner)?[number]
                .inlet_node = 0;
            row.air_distribution_units_cool
                .as_mut()
                .ok_or(Unavailable::InconsistentOwner)?[number]
                .outlet_node = 0;
            row.air_distribution_units_heat
                .as_mut()
                .ok_or(Unavailable::InconsistentOwner)?[number]
                .outlet_node = 0;
            *terminal_counter = terminal_counter
                .checked_add(1)
                .filter(|&x| i32::try_from(x).is_ok())
                .ok_or(Unavailable::InconsistentOwner)?;
            row.air_distribution_units_cool
                .as_mut()
                .ok_or(Unavailable::InconsistentOwner)?[number]
                .terminal_unit_sizing_index = *terminal_counter;
            row.air_distribution_units_heat
                .as_mut()
                .ok_or(Unavailable::InconsistentOwner)?[number]
                .terminal_unit_sizing_index = *terminal_counter;
        }
        node_list_error = false;
        self.nodes.get_source_node_nums(
            input,
            branch,
            &exhaust_name,
            &mut number_nodes,
            node_numbers,
            &mut node_list_error,
            SourceNodeFluidType::Air,
            ObjectType::ZoneHvacEquipmentConnections,
            &locals.alphas[0],
            Connection::ZoneExhaust,
            Stream::Primary,
            false,
            false,
            "",
        )?;
        if node_list_error {
            return Err(unavailable(
                "actual invalid exhaust NodeList continuation diagnostic",
            ));
        }
        self.source_config_mut(index)?.exhaust_nodes = number_nodes;
        self.source_config_mut(index)?.exhaust_node_numbers = Some(vec![0; number_nodes]);
        for number in 0..number_nodes {
            let node = *node_numbers
                .get(number)
                .ok_or(Unavailable::InconsistentOwner)?;
            self.source_config_mut(index)?
                .exhaust_node_numbers
                .as_mut()
                .ok_or(Unavailable::InconsistentOwner)?[number] = node;
            let mut unique_error = false;
            self.nodes.check_source_unique_node_numbers(
                "Zone Air Exhaust Nodes",
                &mut unique_error,
                node,
                &locals.alphas[0],
            )?;
            if unique_error {
                self.equipment_data_errors_found = true;
            }
        }
        node_list_error = false;
        self.nodes.get_source_node_nums(
            input,
            branch,
            &return_name,
            &mut number_nodes,
            node_numbers,
            &mut node_list_error,
            SourceNodeFluidType::Air,
            ObjectType::ZoneHvacEquipmentConnections,
            &locals.alphas[0],
            Connection::ZoneReturn,
            Stream::Primary,
            false,
            false,
            "",
        )?;
        if node_list_error {
            return Err(unavailable(
                "actual invalid return NodeList continuation diagnostic",
            ));
        }
        let row = self.source_config_mut(index)?;
        row.return_nodes = number_nodes;
        row.return_node_numbers = Some(vec![0; number_nodes]);
        row.return_node_space_mixer_indices = Some(vec![0; number_nodes]);
        row.return_node_space_mixer_indices
            .as_mut()
            .ok_or(Unavailable::InconsistentOwner)?
            .fill(-1);
        row.return_node_air_loop_numbers = Some(vec![0; number_nodes]);
        row.return_node_return_path_numbers = Some(vec![0; number_nodes]);
        row.return_node_return_path_component_numbers = Some(vec![0; number_nodes]);
        row.return_node_inlet_numbers = Some(vec![0; number_nodes]);
        row.fixed_return_flow = Some(vec![false; number_nodes]);
        row.return_node_plenum_numbers = Some(vec![0; number_nodes]);
        row.return_node_exhaust_node_numbers = Some(vec![0; number_nodes]);
        row.shared_exhaust_nodes = Some(vec![0; number_nodes]);
        row.return_node_numbers
            .as_mut()
            .ok_or(Unavailable::InconsistentOwner)?
            .fill(0);
        row.return_node_air_loop_numbers
            .as_mut()
            .ok_or(Unavailable::InconsistentOwner)?
            .fill(0);
        row.return_node_inlet_numbers
            .as_mut()
            .ok_or(Unavailable::InconsistentOwner)?
            .fill(0);
        row.return_node_return_path_numbers
            .as_mut()
            .ok_or(Unavailable::InconsistentOwner)?
            .fill(0);
        row.return_node_return_path_component_numbers
            .as_mut()
            .ok_or(Unavailable::InconsistentOwner)?
            .fill(0);
        row.fixed_return_flow
            .as_mut()
            .ok_or(Unavailable::InconsistentOwner)?
            .fill(false);
        row.return_node_plenum_numbers
            .as_mut()
            .ok_or(Unavailable::InconsistentOwner)?
            .fill(0);
        row.return_node_exhaust_node_numbers
            .as_mut()
            .ok_or(Unavailable::InconsistentOwner)?
            .fill(0);
        row.shared_exhaust_nodes
            .as_mut()
            .ok_or(Unavailable::InconsistentOwner)?
            .fill(0);
        for number in 0..number_nodes {
            let node = *node_numbers
                .get(number)
                .ok_or(Unavailable::InconsistentOwner)?;
            self.source_config_mut(index)?
                .return_node_numbers
                .as_mut()
                .ok_or(Unavailable::InconsistentOwner)?[number] = node;
            let mut unique_error = false;
            self.nodes.check_source_unique_node_numbers(
                "Zone Return Air Nodes",
                &mut unique_error,
                node,
                &locals.alphas[0],
            )?;
            if unique_error {
                self.equipment_data_errors_found = true;
            }
        }
        node_list_error = false;
        self.nodes.get_source_node_nums(
            input,
            branch,
            &return_basis_name,
            &mut number_nodes,
            node_numbers,
            &mut node_list_error,
            SourceNodeFluidType::Air,
            ObjectType::ZoneHvacEquipmentConnections,
            &locals.alphas[0],
            Connection::Sensor,
            Stream::Primary,
            false,
            false,
            "",
        )?;
        if node_list_error {
            return Err(unavailable(
                "actual invalid return-flow-basis NodeList continuation diagnostic",
            ));
        }
        self.source_config_mut(index)?
            .number_return_flow_basis_nodes = number_nodes;
        self.source_config_mut(index)?
            .return_flow_basis_node_numbers = Some(vec![0; number_nodes]);
        for number in 0..number_nodes {
            self.source_config_mut(index)?
                .return_flow_basis_node_numbers
                .as_mut()
                .ok_or(Unavailable::InconsistentOwner)?[number] = *node_numbers
                .get(number)
                .ok_or(Unavailable::InconsistentOwner)?;
        }
        Ok(())
    }

    fn read_source_zone_equipment_list(
        &mut self,
        index: usize,
        input: &mut SelectedInputProcessor,
        schedules: &mut SelectedScheduleManager,
        overall_count: &mut usize,
    ) -> Result<(), Unavailable> {
        let list_name = self.source_config_mut(index)?.equipment_list_name.clone();
        let object = sole_object(input, "ZoneHVAC:EquipmentList")?;
        if !object.eq_ignore_ascii_case(&list_name) {
            return Err(unavailable(
                "actual EquipmentList not-found severe diagnostic",
            ));
        }
        input.mark_object_as_used("ZoneHVAC:EquipmentList", &object);
        let fields = input
            .source_raw_object_fields("ZoneHVAC:EquipmentList", &object)
            .map_err(|_| unavailable("actual getJSONObjectItem EquipmentList identity"))?;
        let list = self
            .equipment_lists
            .as_mut()
            .and_then(|rows| rows.get_mut(index))
            .ok_or(Unavailable::InconsistentOwner)?;
        list.name = list_name;
        let distribution = alpha(fields, "load_distribution_scheme", "SequentialLoad")?;
        list.load_distribution_scheme = match distribution.as_str() {
            "SEQUENTIALLOAD" => 0,
            "UNIFORMLOAD" => 1,
            "UNIFORMPLR" => 2,
            "SEQUENTIALUNIFORMPLR" => 3,
            _ => {
                return Err(unavailable(
                    "actual invalid load-distribution severe diagnostic",
                ));
            }
        };
        if let Some(value) = fields.get("equipment") {
            let V::Array(equipment) = value else {
                return Err(unavailable("actual EquipmentList extensible array type"));
            };
            list.number_of_equipment_types = equipment.len();
            *overall_count = overall_count
                .checked_add(list.number_of_equipment_types)
                .filter(|&x| i32::try_from(x).is_ok())
                .ok_or(Unavailable::InconsistentOwner)?;
            let n = list.number_of_equipment_types;
            list.equipment_type_names = Some(vec![String::new(); n]);
            list.equipment_types = Some(vec![-1; n]);
            list.component_pointers.resize(n + 1, None);
            list.equipment_names = Some(vec![String::new(); n]);
            list.equipment_indices = Some(vec![0; n]);
            list.equipment_splitter_indices = Some(vec![0; n]);
            list.equipment_data = Some(vec![SourceEquipmentData::default(); n]);
            list.cooling_priorities = Some(vec![0; n]);
            list.heating_priorities = Some(vec![0; n]);
            list.cooling_capacities = Some(vec![0; n]);
            list.heating_capacities = Some(vec![0; n]);
            list.sequential_cooling_fraction_schedules = Some(vec![None; n]);
            list.sequential_heating_fraction_schedules = Some(vec![None; n]);
            for number in 0..n {
                list.equipment_type_names
                    .as_mut()
                    .ok_or(Unavailable::InconsistentOwner)?[number]
                    .clear();
                list.equipment_types
                    .as_mut()
                    .ok_or(Unavailable::InconsistentOwner)?[number] = -1;
                list.equipment_names
                    .as_mut()
                    .ok_or(Unavailable::InconsistentOwner)?[number]
                    .clear();
                list.equipment_indices
                    .as_mut()
                    .ok_or(Unavailable::InconsistentOwner)?[number] = 0;
                list.equipment_splitter_indices
                    .as_mut()
                    .ok_or(Unavailable::InconsistentOwner)?[number] = -1;
                list.component_pointers[number + 1] = None;
                list.cooling_priorities
                    .as_mut()
                    .ok_or(Unavailable::InconsistentOwner)?[number] = 0;
                list.heating_priorities
                    .as_mut()
                    .ok_or(Unavailable::InconsistentOwner)?[number] = 0;
                list.cooling_capacities
                    .as_mut()
                    .ok_or(Unavailable::InconsistentOwner)?[number] = 0;
                list.heating_capacities
                    .as_mut()
                    .ok_or(Unavailable::InconsistentOwner)?[number] = 0;
                list.sequential_cooling_fraction_schedules
                    .as_mut()
                    .ok_or(Unavailable::InconsistentOwner)?[number] = None;
                list.sequential_heating_fraction_schedules
                    .as_mut()
                    .ok_or(Unavailable::InconsistentOwner)?[number] = None;
            }
            for (number, value) in equipment.iter().enumerate() {
                let V::Object(fields) = value else {
                    return Err(unavailable("actual EquipmentList extensible object type"));
                };
                let kind = alpha(fields, "zone_equipment_object_type", "")?;
                list.equipment_type_names
                    .as_mut()
                    .ok_or(Unavailable::InconsistentOwner)?[number] = kind.clone();
                let name = alpha(fields, "zone_equipment_name", "")?;
                list.equipment_names
                    .as_mut()
                    .ok_or(Unavailable::InconsistentOwner)?[number] = name.clone();
                if kind != "ZONEHVAC:IDEALLOADSAIRSYSTEM" {
                    return Err(unavailable(
                        "actual ValidateComponent finite IdealLoads type",
                    ));
                }
                let objects = input
                    .source_raw_object_names("ZoneHVAC:IdealLoadsAirSystem")
                    .map_err(|_| {
                        unavailable("actual ValidateComponent IdealLoads object family")
                    })?;
                if !objects
                    .iter()
                    .any(|object| object.eq_ignore_ascii_case(&name))
                {
                    return Err(unavailable(
                        "actual ValidateComponent IdealLoads name severe diagnostic",
                    ));
                }
                let cooling = integer(fields, "zone_equipment_cooling_sequence")?;
                list.cooling_priorities
                    .as_mut()
                    .ok_or(Unavailable::InconsistentOwner)?[number] = cooling;
                if cooling < 0 || usize::try_from(cooling).map_or(true, |x| x > n) {
                    return Err(unavailable(
                        "actual cooling-sequence range severe diagnostic",
                    ));
                }
                let heating = integer(fields, "zone_equipment_heating_or_no_load_sequence")?;
                list.heating_priorities
                    .as_mut()
                    .ok_or(Unavailable::InconsistentOwner)?[number] = heating;
                if heating < 0 || usize::try_from(heating).map_or(true, |x| x > n) {
                    return Err(unavailable(
                        "actual heating-sequence range severe diagnostic",
                    ));
                }
                let cooling_schedule = alpha(
                    fields,
                    "zone_equipment_sequential_cooling_fraction_schedule_name",
                    "",
                )?;
                list.sequential_cooling_fraction_schedules
                    .as_mut()
                    .ok_or(Unavailable::InconsistentOwner)?[number] = if cooling_schedule.is_empty()
                {
                    Some(schedules.always_on().map_err(schedule_error)?)
                } else {
                    let schedule = schedules
                        .get_source_schedule(&cooling_schedule)
                        .map_err(schedule_error)?;
                    if schedule.is_none() {
                        return Err(unavailable(
                            "actual sequential-cooling schedule not-found diagnostic",
                        ));
                    }
                    schedule
                };
                let heating_schedule = alpha(
                    fields,
                    "zone_equipment_sequential_heating_fraction_schedule_name",
                    "",
                )?;
                list.sequential_heating_fraction_schedules
                    .as_mut()
                    .ok_or(Unavailable::InconsistentOwner)?[number] = if heating_schedule.is_empty()
                {
                    Some(schedules.always_on().map_err(schedule_error)?)
                } else {
                    let schedule = schedules
                        .get_source_schedule(&heating_schedule)
                        .map_err(schedule_error)?;
                    if schedule.is_none() {
                        return Err(unavailable(
                            "actual sequential-heating schedule not-found diagnostic",
                        ));
                    }
                    schedule
                };
                if heating > 0 {
                    list.number_available_heating_equipment += 1;
                }
                if cooling > 0 {
                    list.number_available_cooling_equipment += 1;
                }
                // Native ZoneEquipType::PurchasedAir =12; no unitary factory.
                list.equipment_types
                    .as_mut()
                    .ok_or(Unavailable::InconsistentOwner)?[number] = 12;
            }
        }
        for number in 1..=list.number_of_equipment_types {
            let key = i32::try_from(number).map_err(|_| Unavailable::InconsistentOwner)?;
            let cooling = list
                .cooling_priorities
                .as_ref()
                .ok_or(Unavailable::InconsistentOwner)?
                .iter()
                .filter(|&&x| x == key)
                .count();
            if cooling > 1 {
                return Err(unavailable(
                    "actual duplicate cooling-sequence severe diagnostic",
                ));
            }
            if cooling == 0 {
                return Err(unavailable(
                    "actual missing cooling-sequence warning diagnostic",
                ));
            }
            let heating = list
                .heating_priorities
                .as_ref()
                .ok_or(Unavailable::InconsistentOwner)?
                .iter()
                .filter(|&&x| x == key)
                .count();
            if heating > 1 {
                return Err(unavailable(
                    "actual duplicate heating-sequence severe diagnostic",
                ));
            }
            if heating == 0 {
                return Err(unavailable(
                    "actual missing heating-sequence warning diagnostic",
                ));
            }
        }
        Ok(())
    }

    /// Rust owner-shape validation after the genuine Native wrapper return.
    /// It does not write readiness flags, derive Native counts, or initialize
    /// another gain/node collection. The exhaustive old empty validator stays
    /// on the existing legacy/A branch.
    pub(super) fn validate_source_controlled_equipment_storage(
        &self,
        zones: &[ZoneId],
        spaces: &[SpaceId],
        gains: &crate::heat_balance::ctf_initialization_context::internal_heat_gains_input::SelectedInternalHeatGains,
    ) -> Result<(), Unavailable> {
        if self.zone_ids != zones
            || self.space_ids != spaces
            || self.get_equipment_input
            || !self.equipment_inputs_filled
            || self.equipment_data_errors_found
            || self.number_of_timesteps_in_day == 0
            || self.number_of_supply_paths != 0
            || self.number_of_return_paths != 0
            || self.nodes.get_input
            || self.nodes.unique_names.is_some()
            || !self.nodes.context_name.is_empty()
            || self.nodes.max_check_nodes != 100
        {
            return Err(Unavailable::InconsistentOwner);
        }
        gains
            .validate_source_initial_gain_allocations(zones.len(), spaces.len())
            .map_err(|_| Unavailable::InconsistentOwner)?;
        if self.internal_gains.zone.is_some()
            || self.internal_gains.space.is_some()
            || self.internal_gains.devices.is_some()
            || self
                .internal_gains
                .daylight_power_reduction_factor
                .is_some()
        {
            return Err(Unavailable::InconsistentOwner);
        }
        let configs = self
            .zone_config
            .as_ref()
            .ok_or(Unavailable::InconsistentOwner)?;
        let lists = self
            .equipment_lists
            .as_ref()
            .ok_or(Unavailable::InconsistentOwner)?;
        if configs.len() != zones.len()
            || lists.len() != zones.len()
            || self
                .equipment_availability
                .as_ref()
                .is_none_or(|rows| rows.len() != zones.len())
            || self
                .supply_paths
                .as_ref()
                .is_none_or(|rows| !rows.is_empty())
            || self
                .return_paths
                .as_ref()
                .is_none_or(|rows| !rows.is_empty())
        {
            return Err(Unavailable::InconsistentOwner);
        }
        let mut maximum = 0;
        for (config, list) in configs.iter().zip(lists) {
            if config.is_controlled {
                maximum = maximum.max(list.number_of_equipment_types);
                let actual_list = config
                    .equipment_list_index
                    .checked_sub(1)
                    .and_then(|n| lists.get(n))
                    .ok_or(Unavailable::InconsistentOwner)?;
                if actual_list.name != config.equipment_list_name || config.zone_node == 0 {
                    return Err(Unavailable::InconsistentOwner);
                }
                for (count, rows) in [
                    (config.inlet_nodes, config.inlet_node_numbers.as_ref()),
                    (config.exhaust_nodes, config.exhaust_node_numbers.as_ref()),
                    (config.return_nodes, config.return_node_numbers.as_ref()),
                    (
                        config.number_return_flow_basis_nodes,
                        config.return_flow_basis_node_numbers.as_ref(),
                    ),
                ] {
                    if rows.is_none_or(|rows| rows.len() != count) {
                        return Err(Unavailable::InconsistentOwner);
                    }
                }
            }
        }
        if self
            .priority_simulation_order
            .as_ref()
            .is_none_or(|rows| rows.len() != maximum)
            || self.nodes.number_of_nodes != self.nodes.number_of_unique_node_names
            || self
                .nodes
                .physical
                .as_ref()
                .is_none_or(|rows| rows.len() != self.nodes.number_of_nodes)
            || self
                .nodes
                .node_ids
                .as_ref()
                .is_none_or(|rows| rows.len() != self.nodes.number_of_nodes + 1)
            || self
                .nodes
                .node_references
                .as_ref()
                .is_none_or(|rows| rows.len() != self.nodes.number_of_nodes)
            || self
                .nodes
                .marked_nodes
                .as_ref()
                .is_none_or(|rows| rows.len() != self.nodes.number_of_nodes)
            || self
                .nodes
                .node_setpoint_checks
                .as_ref()
                .is_none_or(|rows| rows.len() != self.nodes.number_of_nodes)
            || self
                .nodes
                .node_lists
                .as_ref()
                .is_none_or(|rows| rows.len() != self.nodes.number_of_node_lists)
        {
            return Err(Unavailable::InconsistentOwner);
        }
        Ok(())
    }
}
