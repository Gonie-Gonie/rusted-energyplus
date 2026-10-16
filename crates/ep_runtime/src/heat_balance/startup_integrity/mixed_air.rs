//! Real zero-controller/OA-system GetOutsideAirSysInputs and CheckControllerLists.
use super::Error;
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::ordinary_hvac_manager::simulation::state::SourceAirLoopData;
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceArgumentCounts, SourceInputProcessorShortCut,
};
use crate::heat_balance::selected_sizing::SelectedSizingManager;
use std::collections::HashSet;
use std::convert::Infallible;

/// Actual seven MixedAirData members consumed by this genuine getter prefix.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceMixedAir {
    number_controller_lists: i32,
    get_oa_system_input: bool,
    controller_lists: Option<Vec<Infallible>>,
    controller_list_unique_names: HashSet<String>,
    my_one_time_error: Option<Vec<bool>>,
    my_one_time_check_unitary_system: Option<Vec<bool>>,
    initialize_oa_system: Option<Vec<bool>>,
}
impl Default for SourceMixedAir {
    fn default() -> Self {
        Self {
            number_controller_lists: 0,
            get_oa_system_input: true,
            controller_lists: None,
            controller_list_unique_names: HashSet::new(),
            my_one_time_error: None,
            my_one_time_check_unitary_system: None,
            initialize_oa_system: None,
        }
    }
}
fn definition(
    input: &mut SelectedInputProcessor,
    family: &str,
) -> Result<SourceArgumentCounts, Error> {
    let count = input
        .get_object_def_max_args(family)
        .map_err(|_| Error::Unavailable("actual MixedAir getObjectDefMaxArgs definition/size"))?;
    i32::try_from(
        count
            .total()
            .map_err(|_| Error::Unavailable("actual MixedAir TotalArgs extent"))?,
    )
    .map_err(|_| Error::Unavailable("actual MixedAir TotalArgs signed i32 domain"))?;
    Ok(count)
}
fn count(input: &SelectedInputProcessor, family: &str) -> Result<i32, Error> {
    i32::try_from(input.get_num_objects_found(family))
        .map_err(|_| Error::Unavailable("actual MixedAir getNumObjectsFound signed i32 domain"))
}
impl SourceMixedAir {
    /// Native5381-5457: shared caller ErrFound is changed only by positive diagnostics.
    pub(crate) fn check_source_controller_lists(
        &mut self,
        air_loop: &mut SourceAirLoopData,
        sizing: &mut SelectedSizingManager,
        input: &mut SelectedInputProcessor,
        _shortcut: &mut SourceInputProcessorShortCut,
        _err_found: &mut bool,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        if self.get_oa_system_input {
            self.get_source_outside_air_system_inputs(air_loop, sizing, input, backend)?;
            self.get_oa_system_input = false;
        }
        let number_controllers = count(input, "AirLoopHVAC:ControllerList")?;
        let _number_air_loops = count(input, "AirLoopHVAC")?;
        if number_controllers > 0 {
            return Err(Error::Unavailable(
                "actual CheckControllerLists5412 first getObjectItem on same shared alpha/numeric shortcut",
            ));
        }
        Ok(())
    }
    fn get_source_outside_air_system_inputs(
        &mut self,
        air_loop: &mut SourceAirLoopData,
        sizing: &mut SelectedSizingManager,
        input: &mut SelectedInputProcessor,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        let errors_found = false;
        let _initial_total_arguments = 0;
        if !self.get_oa_system_input {
            return Ok(());
        }
        let first = definition(input, "AirLoopHVAC:OutdoorAirSystem")?;
        let mut maximum_numbers = first.numerics;
        let mut maximum_alphas = first.alphas;
        let second = definition(input, "AirLoopHVAC:OutdoorAirSystem:EquipmentList")?;
        maximum_numbers = maximum_numbers.max(second.numerics);
        maximum_alphas = maximum_alphas.max(second.alphas);
        let third = definition(input, "AirLoopHVAC:ControllerList")?;
        maximum_numbers = maximum_numbers.max(third.numerics);
        maximum_alphas = maximum_alphas.max(third.alphas);
        let alphas = vec![String::new(); maximum_alphas];
        let alpha_fields = vec![String::new(); maximum_alphas];
        let numbers = vec![0.0_f64; maximum_numbers];
        let numeric_fields = vec![String::new(); maximum_numbers];
        let alpha_blanks = vec![true; maximum_alphas];
        let numeric_blanks = vec![true; maximum_numbers];
        let mut current_module_object = "AirLoopHVAC:ControllerList";
        self.number_controller_lists = count(input, current_module_object)?;
        if self.number_controller_lists > 0 {
            return Err(Error::Unavailable(
                "actual GetOutsideAirSysInputs917 positive ControllerListProps row allocation before923getter",
            ));
        }
        self.controller_lists = Some(Vec::new());
        current_module_object = "AirLoopHVAC:OutdoorAirSystem";
        air_loop.number_outside_air_systems = count(input, current_module_object)?;
        if air_loop.number_outside_air_systems > 0 {
            return Err(Error::Unavailable(
                "actual GetOutsideAirSysInputs967 positive OutsideAirSysProps row allocation before976getter",
            ));
        }
        air_loop.outside_air_systems = Some(Vec::new());
        sizing.outside_air_system_equipment_sizing = Some(Vec::new());
        let systems = usize::try_from(air_loop.number_outside_air_systems)
            .map_err(|_| Error::Unavailable("actual MixedAir OA system allocation extent"))?;
        self.controller_list_unique_names.reserve(systems);
        self.my_one_time_error = Some(vec![true; systems]);
        self.my_one_time_check_unitary_system = Some(vec![true; systems]);
        self.initialize_oa_system = Some(vec![true; systems]);
        // Actual OA-record and component/controller passes are all empty at the
        // retained count0. No component model or controller absence is inferred.
        if errors_found {
            let message =
                format!("GetOutsideAirSysInputs: Errors found in getting {current_module_object}.");
            return match backend.show_fatal_error(&message) {
                Ok(never) => match never {},
                Err(stop) => Err(Error::Fatal(stop)),
            };
        }
        drop(alphas);
        drop(alpha_fields);
        drop(numbers);
        drop(numeric_fields);
        drop(alpha_blanks);
        drop(numeric_blanks);
        self.get_oa_system_input = false;
        // Native1131 post-flag factory loop is empty at actualNumOASystems0.
        Ok(())
    }
}
