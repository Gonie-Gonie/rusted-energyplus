//! Source GetZoneEquipment over the same retained InputProcessor.
//! Local getter arrays do not overwrite the unrelated shared IP shortcut.
use super::*;
use crate::heat_balance::selected_input_processor::{SelectedInputProcessor, SourceArgumentCounts};

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
        self.get_zone_equipment_data_source(input, space_sizing, space_simulation)?;
        self.finish_get_zone_equipment(clock.timesteps_per_hour)?;
        self.validate_initialized(&self.zone_ids.clone(), &self.space_ids.clone())
    }

    fn get_zone_equipment_data_source(
        &mut self,
        input: &mut SelectedInputProcessor,
        space_sizing: bool,
        space_simulation: bool,
    ) -> Result<(), Unavailable> {
        // Original224-227: the real three counts precede all definitions.
        let controlled_zones = input.get_num_objects_found("ZoneHVAC:EquipmentConnections");
        let controlled_spaces = input.get_num_objects_found("SpaceHVAC:EquipmentConnections");
        self.number_of_equipment_lists = input.get_num_objects_found("ZoneHVAC:EquipmentList");
        if controlled_zones != 0 || controlled_spaces != 0 || self.number_of_equipment_lists != 0 {
            return Err(Unavailable::Input(
                "positive controlled equipment input".into(),
            ));
        }
        let node = definition(input, "NodeList")?;
        let _node_numbers = vec![0_i32; node.total().map_err(|_| Unavailable::InconsistentOwner)?];
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
        let alpha_args = vec![String::new(); max_alphas];
        let numeric_args = vec![0.0_f64; max_numbers];
        let alpha_names = vec![String::new(); max_alphas];
        let numeric_names = vec![String::new(); max_numbers];
        let alpha_blanks = vec![true; max_alphas];
        let numeric_blanks = vec![true; max_numbers];
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
        self.internal_gains
            .allocate_if_zone_unallocated(self.zone_ids.len(), self.space_ids.len());
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
        self.nodes
            .init_unique_check_source("ZoneHVAC:EquipmentConnections", input)?;
        // All controlled/space/term-unit/duplicate loops are genuinely empty.
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
        drop(alpha_args);
        drop(numeric_args);
        drop(alpha_names);
        drop(numeric_names);
        drop(alpha_blanks);
        drop(numeric_blanks);
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
            let counts = definition(input, "NodeList")?;
            let alpha_args = vec![String::new(); counts.alphas];
            let numeric_args = vec![0.0_f64; counts.numerics];
            self.number_of_node_lists = input.get_num_objects_found("NodeList");
            if self.number_of_node_lists != 0 {
                return Err(Unavailable::Input("positive NodeList input".into()));
            }
            self.node_lists = Some(Vec::new());
            // Both nested validation loops are empty at zero count.
            drop(alpha_args);
            drop(numeric_args);
            self.get_input = false;
        }
        self.begin_unique_check_context(context)
    }
}
