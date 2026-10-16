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
    pub(super) fn set_source_outdoor_air_nodes(
        &mut self,
        input: &mut SelectedInputProcessor,
    ) -> Result<(), Error> {
        if self.get_input_flag {
            self.get_source_input(input)?;
            self.get_input_flag = false;
        }
        if let Some(ordinal) = (1..=self.number_outside_air_nodes).next() {
            let _node = self
                .outside_air_node_list
                .as_ref()
                .and_then(|v| v.get(ordinal - 1))
                .ok_or(Error::Unavailable(
                    "actual outside-air node list ordinal381",
                ))?;
            return Err(Error::Unavailable("actual positive SetOANodeValues382"));
        }
        Ok(())
    }
    fn get_source_input(&mut self, input: &mut SelectedInputProcessor) -> Result<(), Error> {
        let number_lists = input.get_num_objects_found("OutdoorAir:NodeList");
        let number_singles = input.get_num_objects_found("OutdoorAir:Node");
        self.number_outside_air_nodes = 0;
        let _errors_found = false;
        let _next_fluid_stream_number = 1;
        let list_size = 0;
        let current_size = 100;
        let _temporary_numbers = vec![0; current_size];
        let nodes = input
            .get_object_def_max_args("NodeList")
            .map_err(|_| Error::Unavailable("actual NodeList getObjectDefMaxArgs176"))?;
        let _node_numbers = vec![0; nodes.alphas + nodes.numerics];
        let mut maximum_numbers = 0;
        let mut maximum_alphas = 0;
        let definition = input
            .get_object_def_max_args("OutdoorAir:NodeList")
            .map_err(|_| Error::Unavailable("actual OutdoorAir:NodeList definition179"))?;
        maximum_numbers = maximum_numbers.max(definition.numerics);
        maximum_alphas = maximum_alphas.max(definition.alphas);
        let definition = input
            .get_object_def_max_args("OutdoorAir:Node")
            .map_err(|_| Error::Unavailable("actual OutdoorAir:Node definition182"))?;
        maximum_numbers = maximum_numbers.max(definition.numerics);
        maximum_alphas = maximum_alphas.max(definition.alphas);
        let _alphas = vec![String::new(); maximum_alphas];
        let _alpha_fields = vec![String::new(); maximum_alphas];
        let _numeric_fields = vec![String::new(); maximum_numbers];
        let _numbers = vec![0.0; maximum_numbers];
        let _alpha_blanks = vec![true; maximum_alphas];
        let _numeric_blanks = vec![true; maximum_numbers];
        if number_lists > 0 {
            let _current_module_object = "OutdoorAir:NodeList";
            return Err(Error::Unavailable(
                "actual positive outdoor-air list getObjectItem197",
            ));
        }
        if number_singles > 0 {
            let _current_module_object = "OutdoorAir:Node";
            return Err(Error::Unavailable(
                "actual positive outdoor-air single getObjectItem261",
            ));
        }
        if list_size > 0 {
            return Err(Error::Unavailable(
                "actual positive outside-air list assignment360",
            ));
        }
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
