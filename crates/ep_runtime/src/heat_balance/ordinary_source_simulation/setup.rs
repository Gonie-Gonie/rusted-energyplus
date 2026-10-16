//! Literal SimulationManager::SetupSimulation1957--2064 orchestration.
//! Every child acts on the same retained run; there are no successful defaults.
use crate::RuntimeError;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_project_data::ProjectSystemVariables;
use crate::weather::day::WeatherDayState;
use std::convert::Infallible;

pub(crate) trait SourceSetupSimulationChildren {
    fn state(&self) -> &WeatherDayState;
    fn state_mut(&mut self) -> &mut WeatherDayState;
    fn globals(&self) -> Result<&SelectedHeatBalanceGlobals, RuntimeError>;
    fn system(&self) -> Result<&ProjectSystemVariables, RuntimeError>;
    fn errors_found(&self) -> bool;
    fn get_next_environment(&mut self, available: &mut bool) -> Result<(), RuntimeError>;
    fn display_string(&mut self, message: &str) -> Result<(), RuntimeError>;
    /// Exact Weather -> Exterior -> HeatBalance, without intervening flag writes.
    fn manage_weather_exterior_heat_balance(&mut self) -> Result<(), RuntimeError>;
    fn simulate_ground_domains(&mut self, initialization: bool) -> Result<(), RuntimeError>;
    fn simulate_cost_estimate(&mut self) -> Result<(), RuntimeError>;
    fn show_fatal_error(&mut self, message: &str) -> Result<Infallible, RuntimeError>;
}

pub(crate) fn setup_source_simulation(
    children: &mut impl SourceSetupSimulationChildren,
) -> Result<(), RuntimeError> {
    let mut available = true; // Actual distinct Setup local1979, not primary249.
    while available {
        children.get_next_environment(&mut available)?; //1983
        if !available {
            break;
        }
        if children.errors_found() {
            break;
        }
        {
            let state = children.state_mut();
            state.global.begin_envrn_flag = true; //1992
            state.global.end_envrn_flag = false;
            state.environment.end_month_flag = false;
            state.global.warmup_flag = true;
            state.global.day_of_sim = 0;
            state.global.day_of_sim += 1; // Defined Native increment after literal0.
            state.global.begin_day_flag = true;
            state.global.end_day_flag = false;
            state.global.hour_of_day = 1;
            state.global.begin_hour_flag = true;
            state.global.end_hour_flag = false;
            state.global.time_step = 1; //2007
        }
        if children.system()?.developer_flag {
            let message = "Initializing Simulation - timestep 1:".to_owned()
                + &children.state().environment.environment_name;
            children.display_string(&message)?;
        }
        children.state_mut().global.begin_time_step_flag = true; //2013
        children.manage_weather_exterior_heat_balance()?; //2015/2017/2019
        {
            let clock = &mut children.state_mut().global;
            clock.begin_hour_flag = false; //2021
            clock.begin_day_flag = false;
            clock.begin_envrn_flag = false;
            clock.begin_sim_flag = false;
        }
        if children.system()?.developer_flag {
            let message = "Initializing Simulation - 2nd timestep 1:".to_owned()
                + &children.state().environment.environment_name;
            children.display_string(&message)?;
        }
        // Native repeats this SAME clock and does not rewrite BeginTimeStepFlag.
        children.manage_weather_exterior_heat_balance()?; //2031/2033/2035
        {
            let clock = &mut children.state_mut().global;
            clock.hour_of_day = 24; //2039
            clock.time_step = clock.time_steps_in_hour;
            clock.end_envrn_flag = true;
            // Native intentionally does not set EndHourFlag or EndDayFlag here.
        }
        if children.system()?.developer_flag {
            let message = "Initializing Simulation - hour 24 timestep 1:".to_owned()
                + &children.state().environment.environment_name;
            children.display_string(&message)?;
        }
        children.manage_weather_exterior_heat_balance()?; //2046/2048/2050
    }
    if children.globals()?.any_slabs_in_model || children.globals()?.any_basements_in_model {
        children.simulate_ground_domains(true)?; //2055
    }
    if !children.errors_found() {
        children.simulate_cost_estimate()?; //2059, genuine input child required.
    }
    if children.errors_found() {
        return match children.show_fatal_error("Previous conditions cause program termination.") {
            Ok(never) => match never {},
            Err(stop) => Err(stop),
        };
    }
    Ok(())
}
