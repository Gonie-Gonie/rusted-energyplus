//! Literal SimulationManager283-367 pre-primary reporting and integrity parent.
//! Actual same-owner borrows and mandatory real children; no success defaults.
use super::super::ctf_initialization_context::hb_input_error;
use super::super::ordinary_ems_plugin::{
    SourceHeatBalanceEmsCallingPoint, SourcePluginData, manage_source_heat_balance_ems,
};
use super::super::run_input_prefix::SelectedPluginManager;
use super::super::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::RuntimeError;
use crate::weather::day::WeatherDayState;
use std::convert::Infallible;

/// Every accessor returns the original owner; children retain reached prefixes.
/// No method has a default implementation or substitutes successful completion.
pub(crate) trait SourcePrimaryPreludeChildren {
    fn state(&self) -> &WeatherDayState;
    fn globals(&self) -> Result<&SelectedHeatBalanceGlobals, RuntimeError>;
    fn globals_mut(&mut self) -> Result<&mut SelectedHeatBalanceGlobals, RuntimeError>;
    fn plugin_manager(&self) -> &Option<SelectedPluginManager>;
    fn plugin_data_mut(&mut self) -> &mut SourcePluginData;
    fn display_string(&mut self, message: &str) -> Result<(), RuntimeError>;
    fn show_message(&mut self, message: &str) -> Result<(), RuntimeError>;
    fn report_surfaces(&mut self) -> Result<(), RuntimeError>;
    fn setup_node_variables_for_reporting(&mut self) -> Result<(), RuntimeError>;
    fn setup_pollution_meter_reporting(&mut self) -> Result<(), RuntimeError>;
    fn allocate_and_setup_ventilation_reports(&mut self) -> Result<(), RuntimeError>;
    fn setup_plugin_output_variables(&mut self) -> Result<(), RuntimeError>;
    fn update_meter_reporting(&mut self) -> Result<(), RuntimeError>;
    fn check_pollution_meter_reporting(&mut self) -> Result<(), RuntimeError>;
    fn verify_custom_electric_power_meters(&mut self) -> Result<(), RuntimeError>;
    fn setup_pollution_calculations(&mut self) -> Result<(), RuntimeError>;
    fn initialize_demand_managers(&mut self) -> Result<(), RuntimeError>;
    /// All six integrity children borrow the SAME original ErrFound local.
    fn test_branch_integrity(&mut self, errors_found: &mut bool) -> Result<(), RuntimeError>;
    fn test_air_path_integrity(&mut self, errors_found: &mut bool) -> Result<(), RuntimeError>;
    fn check_marked_nodes(&mut self, errors_found: &mut bool) -> Result<(), RuntimeError>;
    fn check_node_connections(&mut self, errors_found: &mut bool) -> Result<(), RuntimeError>;
    fn test_component_set_inlet_outlet_nodes(
        &mut self,
        errors_found: &mut bool,
    ) -> Result<(), RuntimeError>;
    fn check_controller_lists(&mut self, errors_found: &mut bool) -> Result<(), RuntimeError>;
    fn report_loop_connections(&mut self) -> Result<(), RuntimeError>;
    fn report_air_loop_connections(&mut self) -> Result<(), RuntimeError>;
    fn report_node_connections(&mut self) -> Result<(), RuntimeError>;
    fn create_energy_report_structure(&mut self) -> Result<(), RuntimeError>;
    fn produce_rdd_mdd(&mut self) -> Result<(), RuntimeError>;
    /// The actual Fatal child cannot report successful function completion.
    fn show_fatal_error(&mut self, message: &str) -> Result<Infallible, RuntimeError>;
    /// Reads the actual retained SQLite pointer, never an inferred input count.
    fn sqlite_present(&self) -> bool;
    fn sqlite_begin(&mut self) -> Result<(), RuntimeError>;
    /// Arguments are Native literal1 and the actual just-read TimeStepsInHour.
    fn sqlite_update_simulation_record(
        &mut self,
        simulation_index: i32,
        time_steps_in_hour: i32,
    ) -> Result<(), RuntimeError>;
    fn sqlite_commit(&mut self) -> Result<(), RuntimeError>;
    fn get_life_cycle_cost_input(&mut self) -> Result<(), RuntimeError>;
    fn read_variable_location_orientation(&mut self) -> Result<(), RuntimeError>;
    /// Internally reborrows the same WeatherSession.errors_found caller179 cell.
    /// This does not create or reuse the separate reporting ErrFound local.
    fn manage_hvac_sizing_simulation(&mut self) -> Result<(), RuntimeError>;
}

/// Enter only after the genuine Setup/Fault/Curve and278-281 postlude succeeds.
/// The actual368 ResetEnvironmentCounter remains in the separate primary parent.
pub(crate) fn prepare_source_primary_simulation(
    children: &mut impl SourcePrimaryPreludeChildren,
) -> Result<(), RuntimeError> {
    if children.state().global.do_output_reporting {
        children.display_string("Reporting Surfaces")?;
        let mut error_found = false;
        let mut terminal_error = false;

        children.report_surfaces()?;
        children.setup_node_variables_for_reporting()?;
        children.globals_mut()?.meters_have_been_initialized = true;
        children.setup_pollution_meter_reporting()?;
        children.allocate_and_setup_ventilation_reports()?;
        if children.plugin_manager().is_some() {
            children.setup_plugin_output_variables()?;
        }
        children.update_meter_reporting()?;
        children.check_pollution_meter_reporting()?;
        children.verify_custom_electric_power_meters()?;
        children.setup_pollution_calculations()?;
        children.initialize_demand_managers()?;

        children.test_branch_integrity(&mut error_found)?;
        if error_found {
            terminal_error = true;
        }
        children.test_air_path_integrity(&mut error_found)?;
        if error_found {
            terminal_error = true;
        }
        children.check_marked_nodes(&mut error_found)?;
        if error_found {
            terminal_error = true;
        }
        children.check_node_connections(&mut error_found)?;
        if error_found {
            terminal_error = true;
        }
        children.test_component_set_inlet_outlet_nodes(&mut error_found)?;
        if error_found {
            terminal_error = true;
        }
        children.check_controller_lists(&mut error_found)?;
        if error_found {
            terminal_error = true;
        }

        if children.state().global.do_des_day_sim
            || children.state().global.do_weath_sim
            || children.globals()?.do_pure_load_calc
        {
            children.report_loop_connections()?;
            children.report_air_loop_connections()?;
            children.report_node_connections()?;
        }
        children.create_energy_report_structure()?;

        // The Native local is unwritten before ManageEMS's actual false writer.
        let mut any_ems_ran = None;
        manage_source_heat_balance_ems(
            children.globals()?,
            SourceHeatBalanceEmsCallingPoint::SetupSimulation,
            &mut any_ems_ran,
        )
        .map_err(hb_input_error)?;
        children.produce_rdd_mdd()?;

        if terminal_error {
            return match children.show_fatal_error("Previous Conditions cause program termination.")
            {
                Ok(never) => match never {},
                Err(error) => Err(error),
            };
        }
    }

    children.plugin_data_mut().fully_ready = true;
    if children.sqlite_present() {
        children.sqlite_begin()?;
        // A transient actual Native by-value argument, read after sqliteBegin.
        // No timestep/clock owner or retained scalar snapshot is introduced.
        let time_steps_in_hour = children.state().global.time_steps_in_hour;
        children.sqlite_update_simulation_record(1, time_steps_in_hour)?;
        children.sqlite_commit()?;
    }

    children.get_life_cycle_cost_input()?;
    children.read_variable_location_orientation()?;
    if children.globals()?.do_hvac_sizing_simulation {
        children.manage_hvac_sizing_simulation()?;
    }
    if !children.globals()?.do_pure_load_calc {
        children.show_message("Beginning Simulation")?;
        children.display_string("Beginning Primary Simulation")?;
    }
    Ok(())
}
