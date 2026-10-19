//! Genuine zero-primary-air-system ManageAirLoops and its reached child bodies.
mod get_input;
mod initialize;
pub(crate) mod output;
use super::state::SourceAirLoopData;
use crate::heat_balance::ordinary_air_heat_balance::predictor_corrector::simple_air::SourceAirReportRegistrationContext;
use crate::heat_balance::ordinary_equipment::OrdinaryEquipment;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_input_processor::SelectedInputProcessor;
use crate::heat_balance::selected_project_data::SelectedProjectData;
use crate::heat_balance::selected_simulation_startup::SelectedHvacGlobals;
use crate::heat_balance::selected_sizing::SelectedSizingManager;
use crate::heat_balance::source_output_registration::{
    SourceOutputDictionaryHost, SourceOutputFileHost,
};
use crate::weather::day::WeatherGlobalState;
use std::convert::Infallible;

/// Actual temporary borrows, including genuine existing equipment/sizing owners.
pub(crate) struct SourceAirLoopOwners<'a> {
    pub(crate) input: &'a mut SelectedInputProcessor,
    pub(crate) globals: &'a SelectedHeatBalanceGlobals,
    pub(crate) clock: &'a WeatherGlobalState,
    pub(crate) hvac: &'a mut SelectedHvacGlobals,
    pub(crate) project: &'a mut SelectedProjectData,
    pub(crate) equipment: &'a OrdinaryEquipment,
    pub(crate) sizing: &'a mut SelectedSizingManager,
    pub(crate) distribution_simulated: &'a bool,
}
fn index(number: i32) -> Result<usize, Error> {
    number
        .checked_sub(1)
        .and_then(|v| usize::try_from(v).ok())
        .ok_or(Error::Unavailable("actual positive Source AirLoop ordinal"))
}
fn allocate_empty(target: &mut Option<Vec<Infallible>>) {
    *target = Some(Vec::new());
}
impl SourceAirLoopData {
    /// Actual SimSelectedEquipment caller supplies these same HVAC flags.
    /// inputs_filled is written by that caller only after this full method succeeds.
    pub(crate) fn manage_source_air_loops<
        H: SourceOutputFileHost,
        D: SourceOutputDictionaryHost,
    >(
        &mut self,
        first_hvac_iteration: bool,
        o: &mut SourceAirLoopOwners<'_>,
        output: &mut SourceAirReportRegistrationContext<'_, H, D>,
    ) -> Result<(), Error> {
        if self.get_air_loop_input_flag {
            self.get_source_air_path_data(o)?;
            self.get_air_loop_input_flag = false;
        }
        self.initialize_source_air_loops(first_hvac_iteration, o)?;
        if o.globals.system_sizing_calculation {
            return Err(Error::Unavailable(
                "actual SizeAirLoops171 SetupSysSizingArrays first child4058",
            ));
        }
        self.simulate_source_air_loops(first_hvac_iteration, o, output)?;
        // Actual any_of sees the authentic empty/unallocated EPVector. Positive
        // AirLoopControlData constructors remain unavailable at GetAirPath393.
        o.hvac.sim_air_loops = match self.control_info.as_deref().and_then(|rows| rows.first()) {
            Some(row) => match *row {},
            None => false,
        };
        Ok(())
    }
}
