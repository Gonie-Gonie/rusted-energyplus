//! Actual zero-object OutAirNode/HybridVentilation/IndoorGreen manager children.
use super::*;
use crate::heat_balance::selected_input_processor::SelectedInputProcessor;
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceOutdoorAirNodes {
    pub(crate) outside_air_node_list: Option<Vec<i32>>,
    pub(crate) number_outside_air_nodes: usize,
    pub(crate) get_input_flag: bool,
}
impl Default for SourceOutdoorAirNodes {
    fn default() -> Self {
        Self {
            outside_air_node_list: None,
            number_outside_air_nodes: 0,
            get_input_flag: true,
        }
    }
}
impl SourceOutdoorAirNodes {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn set_source_outdoor_air_nodes(
        &mut self,
        input: &mut SelectedInputProcessor,
        equipment: &mut super::super::ordinary_equipment::OrdinaryEquipment,
        environment: &crate::weather::day::WeatherEnvironmentState,
        control: &super::super::selected_heat_balance_input::SelectedHeatBalanceInputState,
        branch: &mut super::super::startup_integrity::SourceBranchNodeConnections,
        source_is_epjson: bool,
        backend: &mut super::super::error_backend::SourceErrorBackend<'_>,
        flags: Option<super::super::error_backend::SourceDiagnosticFlags<'_>>,
    ) -> Result<(), Error> {
        if self.get_input_flag {
            self.get_source_input(input, equipment, branch, source_is_epjson, backend, flags)?;
            self.get_input_flag = false;
        }
        for ordinal in 1..=self.number_outside_air_nodes {
            let node = *self
                .outside_air_node_list
                .as_ref()
                .and_then(|v| v.get(ordinal - 1))
                .ok_or(Error::Unavailable(
                    "actual outside-air node list ordinal381",
                ))?;
            equipment.set_source_outdoor_air_node_values(
                node,
                true,
                environment,
                control,
                backend,
                flags,
            )?;
        }
        Ok(())
    }

    /// Native CheckAndAdd440-519; the positive PurchasedAir getter remains held.
    #[allow(
        dead_code,
        reason = "Owner prerequisite only; the genuine positive-OA getter is a separate port."
    )]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn check_and_add_source_air_node_number(
        &mut self,
        input: &mut SelectedInputProcessor,
        equipment: &mut super::super::ordinary_equipment::OrdinaryEquipment,
        branch: &mut super::super::startup_integrity::SourceBranchNodeConnections,
        node_number: i32,
        okay: &mut bool,
        environment: &crate::weather::day::WeatherEnvironmentState,
        control: &super::super::selected_heat_balance_input::SelectedHeatBalanceInputState,
        backend: &mut super::super::error_backend::SourceErrorBackend<'_>,
        flags: Option<super::super::error_backend::SourceDiagnosticFlags<'_>>,
        source_is_epjson: bool,
    ) -> Result<(), Error> {
        if self.get_input_flag {
            self.get_source_input(input, equipment, branch, source_is_epjson, backend, flags)?;
            self.get_input_flag = false;
            self.set_source_outdoor_air_nodes(
                input,
                equipment,
                environment,
                control,
                branch,
                source_is_epjson,
                backend,
                flags,
            )?;
        }
        *okay = false;
        if self.number_outside_air_nodes > 0 {
            let nodes = self
                .outside_air_node_list
                .as_ref()
                .and_then(|v| v.get(..self.number_outside_air_nodes))
                .ok_or(Error::Unavailable(
                    "actual CheckAndAdd outside-air registry",
                ))?;
            *okay = nodes.contains(&node_number);
        }
        if node_number > 0 && !*okay {
            self.number_outside_air_nodes = self
                .number_outside_air_nodes
                .checked_add(1)
                .filter(|&n| i32::try_from(n).is_ok())
                .ok_or(Error::Unavailable(
                    "defined Native NumOutsideAirNodes increment",
                ))?;
            let nodes = self.outside_air_node_list.get_or_insert_with(Vec::new);
            nodes.resize(self.number_outside_air_nodes, 0);
            nodes[self.number_outside_air_nodes - 1] = node_number;
            // Real Native TmpNums assignment is local ordinal scratch, not Node state.
            let mut temporary = nodes
                .iter()
                .map(|&number| usize::try_from(number))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| Error::Unavailable("actual CheckAndAdd temporary Node ordinals"))?;
            equipment.register_source_outdoor_air_node(
                input,
                branch,
                node_number,
                self.number_outside_air_nodes,
                &mut temporary,
                backend,
                flags,
            )?;
            equipment.set_source_outdoor_air_node_values(
                node_number,
                false,
                environment,
                control,
                backend,
                flags,
            )?;
        }
        Ok(())
    }
    #[allow(clippy::too_many_arguments)]
    fn get_source_input(
        &mut self,
        input: &mut SelectedInputProcessor,
        equipment: &mut super::super::ordinary_equipment::OrdinaryEquipment,
        branch: &mut super::super::startup_integrity::SourceBranchNodeConnections,
        source_is_epjson: bool,
        backend: &mut super::super::error_backend::SourceErrorBackend<'_>,
        flags: Option<super::super::error_backend::SourceDiagnosticFlags<'_>>,
    ) -> Result<(), Error> {
        use crate::heat_balance::selected_input_processor::SourceInputStatisticsValue as Value;
        const FAMILY: &str = "OutdoorAir:Node";
        const FIELDS: [&str; 9] = [
            "name",
            "height_above_ground",
            "drybulb_temperature_schedule_name",
            "wetbulb_temperature_schedule_name",
            "wind_speed_schedule_name",
            "wind_direction_schedule_name",
            "wind_pressure_coefficient_curve_name",
            "symmetric_wind_pressure_coefficient_curve",
            "wind_angle_type",
        ];
        let number_lists = input.get_num_objects_found("OutdoorAir:NodeList");
        let number_singles = input.get_num_objects_found(FAMILY);
        self.number_outside_air_nodes = 0;
        let mut next_fluid_stream_number = 1_i32;
        let mut list_size = 0_usize;
        let mut current_size = 100_usize;
        let mut temporary_numbers = vec![0_usize; current_size];
        let nodes = input
            .get_object_def_max_args("NodeList")
            .map_err(|_| Error::Unavailable("actual NodeList getObjectDefMaxArgs176"))?;
        let mut node_numbers = vec![0_usize; nodes.alphas + nodes.numerics];
        let mut maximum_numbers = 0;
        let mut maximum_alphas = 0;
        let definition = input
            .get_object_def_max_args("OutdoorAir:NodeList")
            .map_err(|_| Error::Unavailable("actual OutdoorAir:NodeList definition179"))?;
        maximum_numbers = maximum_numbers.max(definition.numerics);
        maximum_alphas = maximum_alphas.max(definition.alphas);
        let definition = input
            .get_object_def_max_args(FAMILY)
            .map_err(|_| Error::Unavailable("actual OutdoorAir:Node definition182"))?;
        maximum_numbers = maximum_numbers.max(definition.numerics);
        maximum_alphas = maximum_alphas.max(definition.alphas);
        let mut alphas = vec![String::new(); maximum_alphas];
        let mut alpha_fields = vec![String::new(); maximum_alphas];
        let mut numeric_fields = vec![String::new(); maximum_numbers];
        let mut numbers = vec![0.0; maximum_numbers];
        let mut alpha_blanks = vec![true; maximum_alphas];
        let mut numeric_blanks = vec![true; maximum_numbers];
        if number_lists > 0 {
            return Err(Error::Unavailable(
                "actual positive outdoor-air list getObjectItem197",
            ));
        }
        if number_singles > 0 {
            if !source_is_epjson || number_singles != 1 {
                return Err(Error::Unavailable(
                    "authored outdoor-air single original-IDF/multiple-object getter",
                ));
            }
            if alphas.len() < 8 || numbers.is_empty() {
                return Err(Error::Unavailable(
                    "actual outdoor-air local getter allocation",
                ));
            }
            let object = input
                .source_raw_object_names(FAMILY)
                .map_err(|_| Error::Unavailable("actual outdoor-air retained instance order"))?
                .first()
                .cloned()
                .ok_or(Error::Unavailable(
                    "actual outdoor-air single instance ordinal",
                ))?;
            alphas.fill(String::new());
            numbers.fill(0.0);
            alpha_blanks.fill(true);
            numeric_blanks.fill(true);
            alpha_fields.fill(String::new());
            numeric_fields.fill(String::new());
            input.mark_object_as_used(FAMILY, &object);
            let mut maximum = 0_usize;
            let mut num_alphas = 1_usize;
            let num_numbers;
            {
                let fields = input
                    .source_raw_object_fields(FAMILY, &object)
                    .map_err(|_| {
                        Error::Unavailable("actual outdoor-air getObjectItem raw fields")
                    })?;
                for (index, field) in FIELDS.iter().enumerate() {
                    if fields.contains_key(*field) {
                        maximum = maximum.max(index + 1);
                    }
                }
                alphas[0] = object.to_ascii_uppercase();
                alpha_blanks[0] = object.is_empty();
                alpha_fields[0] = "name".into();
                numeric_fields[0] = "height_above_ground".into();
                numbers[0] = match fields.get("height_above_ground") {
                    None => -1.0,
                    Some(Value::Number(token)) => ep_model::classify_source_json_number(token)
                        .map(ep_model::SourceJsonNumber::as_f64)
                        .map_err(|_| {
                            Error::Unavailable("actual outdoor-air JSON numeric array scalar")
                        })?,
                    _ => return Err(Error::Unavailable("actual outdoor-air numeric field type")),
                };
                numeric_blanks[0] = !fields.contains_key("height_above_ground");
                num_numbers = usize::from(maximum >= 2);
                for index in 1..8 {
                    let field = FIELDS[index + 1];
                    let default = match index {
                        6 => "No",
                        7 => "Absolute",
                        _ => "",
                    };
                    let (value, blank) = match fields.get(field) {
                        None => (default, true),
                        Some(Value::String(value)) if value.is_empty() => (default, true),
                        Some(Value::String(value)) => (value.as_str(), false),
                        _ => return Err(Error::Unavailable("actual outdoor-air alpha field type")),
                    };
                    alphas[index] = value.to_ascii_uppercase();
                    alpha_blanks[index] = blank;
                    alpha_fields[index] = field.into();
                    if index + 1 < maximum {
                        num_alphas = index + 1;
                    }
                }
            }
            let mut num_nodes = 0;
            let mut err_in_list = false;
            equipment.get_source_authored_outdoor_air_node_nums(
                input,
                branch,
                &alphas[0],
                &alpha_fields[0],
                next_fluid_stream_number,
                &mut num_nodes,
                &mut node_numbers,
                &mut err_in_list,
                backend,
                flags,
            )?;
            next_fluid_stream_number = next_fluid_stream_number
                .checked_add(i32::try_from(num_nodes).map_err(|_| {
                    Error::Unavailable("actual outdoor-air fluid stream NumNodes integer")
                })?)
                .ok_or(Error::Unavailable(
                    "actual outdoor-air next fluid stream integer",
                ))?;
            if err_in_list {
                return Err(Error::Unavailable(
                    "actual outdoor-air GetNodeNums continuation284",
                ));
            }
            if num_nodes > 1 {
                return Err(Error::Unavailable(
                    "actual outdoor-air single NodeList severe290",
                ));
            }
            let node_number = *node_numbers.first().ok_or(Error::Unavailable(
                "actual outdoor-air NodeNums first ordinal",
            ))?;
            if temporary_numbers.contains(&node_number) {
                return Err(Error::Unavailable(
                    "actual duplicate authored outdoor-air node305",
                ));
            }
            list_size += 1;
            if list_size > current_size {
                current_size += 100;
                temporary_numbers.resize(current_size, 0);
            }
            temporary_numbers[list_size - 1] = node_number;
            if num_numbers > 0 {
                equipment.set_source_authored_outdoor_air_node_height(node_number, numbers[0])?;
            }
            if num_alphas > 1 {
                return Err(Error::Unavailable(
                    "actual outdoor-air AnyLocalEnvironmentsInModel write316",
                ));
            }
            // The admitted single has NumAlphas1: all schedule, WPC and local
            // alpha children are absent, and its physical IsLocalNode stays false.
        }
        if list_size > 0 {
            self.number_outside_air_nodes = list_size;
            self.outside_air_node_list = Some(
                temporary_numbers[..list_size]
                    .iter()
                    .map(|&node| i32::try_from(node))
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|_| {
                        Error::Unavailable("actual outside-air node list int assignment360")
                    })?,
            );
        }
        let _ = next_fluid_stream_number; // Native local increment precedes all validation.
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceHybridVentilation {
    pub(crate) number_hybrid_ventilation_managers: usize,
    pub(crate) get_input_flag: bool,
}
impl Default for SourceHybridVentilation {
    fn default() -> Self {
        Self {
            number_hybrid_ventilation_managers: 0,
            get_input_flag: true,
        }
    }
}
impl SourceHybridVentilation {
    pub(super) fn manage_source_hybrid_ventilation(
        &mut self,
        input: &SelectedInputProcessor,
    ) -> Result<(), Error> {
        if self.get_input_flag {
            // GetHybridVentilationInputs3796-3801, local module name, no shared shortcut write.
            self.number_hybrid_ventilation_managers =
                input.get_num_objects_found("AvailabilityManager:HybridVentilation");
            if self.number_hybrid_ventilation_managers != 0 {
                return Err(Error::Unavailable(
                    "actual positive HybridVentData allocation3805",
                ));
            }
            self.get_input_flag = false;
        }
        if self.number_hybrid_ventilation_managers == 0 {
            return Ok(());
        }
        Err(Error::Unavailable(
            "actual positive InitHybridVentSysAvailMgr3736",
        ))
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceIndoorGreen {
    pub(crate) number_indoor_green: usize,
    pub(crate) get_input_flag: bool,
    pub(crate) indoor_greens: Option<Vec<std::convert::Infallible>>,
}
impl Default for SourceIndoorGreen {
    fn default() -> Self {
        Self {
            number_indoor_green: 0,
            get_input_flag: true,
            indoor_greens: None,
        }
    }
}
impl SourceIndoorGreen {
    pub(super) fn simulate_source_indoor_green(
        &mut self,
        input: &SelectedInputProcessor,
    ) -> Result<(), Error> {
        if self.get_input_flag {
            let _errors_found = false;
            self.number_indoor_green = input.get_num_objects_found("IndoorLivingWall");
            if self.number_indoor_green > 0 {
                return Err(Error::Unavailable(
                    "actual positive indoorGreens allocation128",
                ));
            }
            // Actual SetIndoorGreenOutput287 zero-count loop, after getter return.
            if let Some(_number) = (1..=self.number_indoor_green).next() {
                return Err(Error::Unavailable(
                    "actual positive SetIndoorGreenOutput same row",
                ));
            }
            self.get_input_flag = false;
        }
        if self.number_indoor_green > 0 {
            return Err(Error::Unavailable("actual positive InitIndoorGreen98"));
        }
        Ok(())
    }
}
/// Separate real module owners, retained once beside the actual HVACManager fields.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceHvacAncillaryModules {
    pub(crate) dual_duct: crate::heat_balance::startup_integrity::SourceDualDuctData,
    pub(crate) branch_node_connections:
        crate::heat_balance::startup_integrity::SourceBranchNodeConnections,
    pub(crate) mixed_air: crate::heat_balance::startup_integrity::SourceMixedAir,
    pub(crate) splitter_component:
        crate::heat_balance::startup_integrity::SourceSplitterComponentCounts,
    pub(crate) mixer_component: crate::heat_balance::startup_integrity::SourceMixerComponentCounts,
    pub(crate) zone_plenum: crate::heat_balance::startup_integrity::SourceZonePlenumCounts,
    pub(crate) system_reports: crate::heat_balance::ordinary_system_reports::SourceSystemReports,
    pub(crate) pollution: crate::heat_balance::ordinary_pollution::SourcePollutionModule,
    pub(crate) heat_emission: super::heat_emission::state::SourceHeatEmissionDeviceModules,
    pub(crate) postlude_modules: super::postlude::SourceHvacPostludeModules,
    pub(crate) availability:
        crate::heat_balance::ordinary_equipment::management::availability::SourceAvailability,
    pub(crate) water: super::water_manager::SourceWaterManager,
    pub(crate) flow_systems: super::simple_airflow::SourceSimpleAirflowSystems,
    pub(crate) outdoor_air_nodes: SourceOutdoorAirNodes,
    pub(crate) hybrid_ventilation: SourceHybridVentilation,
    pub(crate) indoor_green: SourceIndoorGreen,
}
