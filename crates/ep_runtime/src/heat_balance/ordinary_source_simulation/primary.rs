//! Literal SimulationManager368–564 primary environment/day/hour/timestep loop.
//! Actual per-call borrows and required children; no legacy phase/configuration loop.
use crate::RuntimeError;
use crate::heat_balance::radiation::ordinary::source_children::SourceSolarReportFlags;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError;
use crate::heat_balance::selected_project_data::ProjectSystemVariables;
use crate::heat_balance::selected_schedule_manager::SourceScheduleIndex;
use crate::heat_balance::source_output_registration::SourceOutputRegistration;
use crate::weather::day::WeatherDayState;

/// Every accessor is a short borrow of the actual retained owner named here.
/// Native180 underwater locals are passed separately to the parent function;
/// the same caller initializes them before output options and never at368.
pub(crate) trait SourcePrimarySimulationChildren {
    fn state(&self) -> &WeatherDayState;
    fn state_mut(&mut self) -> &mut WeatherDayState;
    fn globals(&self) -> Result<&SelectedHeatBalanceGlobals, RuntimeError>;
    fn system(&self) -> Result<&ProjectSystemVariables, RuntimeError>;
    fn report_flags(&self) -> &SourceSolarReportFlags;
    fn report_flags_mut(&mut self) -> &mut SourceSolarReportFlags;
    fn output_registration_mut(&mut self) -> &mut SourceOutputRegistration;
    /// Actual CtfReports.source_run_available Some cell written at249.
    fn primary_available(&self) -> Result<&bool, RuntimeError>;
    /// Reborrow that SAME249 cell internally to genuine GetNext; do not touch
    /// Setup1979's separate local and do not copy the result through a shadow.
    fn get_next_primary_environment(&mut self) -> Result<(), RuntimeError>;
    fn errors_found(&self) -> bool;
    /// Actual source_run_sims_done Some cell assigned false at236.
    fn sims_done_mut(&mut self) -> Result<&mut bool, RuntimeError>;
    /// Actual ErrorTracking.ExitDuringSimulations, not a new simulation flag.
    fn exit_during_simulations_mut(&mut self) -> Result<&mut bool, RuntimeError>;
    fn sqlite_present(&self) -> bool;
    fn sqlite_begin(&mut self) -> Result<(), RuntimeError>;
    fn sqlite_environment_period_record(&mut self) -> Result<(), RuntimeError>;
    fn sqlite_commit(&mut self) -> Result<(), RuntimeError>;
    fn display_string(&mut self, message: &str) -> Result<(), RuntimeError>;
    fn reset_node_data(&mut self) -> Result<(), RuntimeError>;
    /// Literal EMS calling point2. Native caller local is initially unwritten;
    /// the child must write it at the actual ManageEMS operation position.
    fn manage_begin_new_environment_ems(
        &mut self,
        any_ran: &mut Option<bool>,
    ) -> Result<(), RuntimeError>;
    fn display_simulation_days_progress(&mut self) -> Result<(), RuntimeError>;
    fn reset_accumulation_when_warmup_complete(&mut self) -> Result<(), RuntimeError>;
    fn write_eio(&mut self, bytes: &[u8]) -> Result<(), RuntimeError>;
    fn reset_tabular_reports(&mut self) -> Result<(), RuntimeError>;
    fn simulate_ground_domains(&mut self, initialization: bool) -> Result<(), RuntimeError>;
    fn update_underwater_boundaries(&mut self) -> Result<(), RuntimeError>;
    /// Actual DataEnvironment.varyingOrientationSched pointer. The two actual
    /// latitude/longitude pointers are read directly from state at Native507.
    fn varying_orientation_schedule(&self) -> Result<&Option<SourceScheduleIndex>, RuntimeError>;
    fn update_location_and_orientation(&mut self) -> Result<(), RuntimeError>;
    fn external_interface_exchange_variables(&mut self) -> Result<(), RuntimeError>;
    /// Exactly Weather -> ExteriorEnergyUse -> HeatBalance on these same owners.
    fn manage_weather_exterior_heat_balance(&mut self) -> Result<(), RuntimeError>;
    fn check_if_any_underwater_boundaries(&mut self) -> Result<bool, RuntimeError>;
}
fn input_error(error: HeatBalanceInputError) -> RuntimeError {
    RuntimeError::HeatBalanceInput(Box::new(error))
}
fn increment(value: &mut i32) -> Result<(), RuntimeError> {
    *value = value.checked_add(1).ok_or_else(|| {
        input_error(HeatBalanceInputError::Unavailable(
            "actual primary simulation signed-int increment outside defined Native domain",
        ))
    })?;
    Ok(())
}

/// Enter after the genuine kickoff/HVAC-sizing/pollution startup continuation.
/// This does not initialize SimsDone236 or either Native180 underwater local.
pub(crate) fn simulate_source_primary(
    children: &mut impl SourcePrimarySimulationChildren,
    one_time_underwater_boundary_check: &mut bool,
    any_underwater_boundaries: &mut bool,
) -> Result<(), RuntimeError> {
    // Exact Weather::ResetEnvironmentCounter159–162 child at Simulation368.
    children.state_mut().weather.envrn = 0;
    let mut environment_count = 0_i32;
    children.state_mut().global.warmup_flag = true;
    while *children.primary_available()? {
        if children.globals()?.stop_simulation {
            break;
        }
        children.get_next_primary_environment()?;
        if !*children.primary_available()? {
            break;
        }
        if children.errors_found() {
            break;
        }
        if !children.state().global.do_des_day_sim && children.state().global.kind_of_sim != 3 {
            continue;
        }
        if !children.state().global.do_weath_sim && children.state().global.kind_of_sim == 3 {
            continue;
        }
        if children.state().global.kind_of_sim == 4 {
            continue;
        }
        if children.state().global.kind_of_sim == 5 {
            continue;
        }
        increment(&mut environment_count)?;
        if children.sqlite_present() {
            children.sqlite_begin()?;
            children.sqlite_environment_period_record()?;
            children.sqlite_commit()?;
        }
        *children.exit_during_simulations_mut()? = true;
        *children.sims_done_mut()? = true;
        children.display_string("Initializing New Environment Parameters")?;
        {
            let state = children.state_mut();
            state.global.begin_envrn_flag = true;
            state.global.end_envrn_flag = false;
            state.environment.end_month_flag = false;
            state.global.warmup_flag = true;
            state.global.day_of_sim = 0;
            state.global.day_of_sim_chr = "0".to_owned();
        }
        children.report_flags_mut().reset_source_warmup_day_count();
        if children.state().environment.current_year_is_leap_year {
            if children.state().global.num_of_day_in_envrn <= 366 {
                children.output_registration_mut().is_final_year = true;
            }
        } else if children.state().global.num_of_day_in_envrn <= 365 {
            children.output_registration_mut().is_final_year = true;
        }
        children.reset_node_data()?;
        let mut any_ems_ran = None; // Actual Native caller bool has no initializer.
        children.manage_begin_new_environment_ems(&mut any_ems_ran)?;
        while children.state().global.day_of_sim < children.state().global.num_of_day_in_envrn
            || children.state().global.warmup_flag
        {
            if children.globals()?.stop_simulation {
                break;
            }
            if children.sqlite_present() {
                children.sqlite_begin()?;
            }
            {
                let clock = &mut children.state_mut().global;
                increment(&mut clock.day_of_sim)?;
                clock.day_of_sim_chr = clock.day_of_sim.to_string();
            }
            if !children.state().global.warmup_flag {
                increment(&mut children.state_mut().environment.current_overall_sim_day)?;
                children.display_simulation_days_progress()?;
            } else {
                children.state_mut().global.day_of_sim_chr = "0".to_owned();
            }
            children.state_mut().global.begin_day_flag = true;
            children.state_mut().global.end_day_flag = false;
            if children.state().global.warmup_flag {
                children
                    .report_flags_mut()
                    .advance_source_warmup_day()
                    .map_err(input_error)?;
                let message = "Warming up {".to_owned() + &children.report_flags().warmup_day + "}";
                children.display_string(&message)?;
            } else if children.state().global.day_of_sim == 1 {
                if children.system()?.report_during_warmup {
                    children.reset_accumulation_when_warmup_complete()?;
                }
                if children.state().global.kind_of_sim == 3 {
                    let message = "Starting Simulation at ".to_owned()
                        + &children.state().environment.current_month_day_year
                        + " for "
                        + &children.state().environment.environment_name;
                    children.display_string(&message)?;
                } else {
                    let message = "Starting Simulation at ".to_owned()
                        + &children.state().environment.current_month_day
                        + " for "
                        + &children.state().environment.environment_name;
                    children.display_string(&message)?;
                }
                let text = format!(
                    "Environment:WarmupDays,{:3}\n",
                    children.report_flags().number_of_warmup_days
                );
                children.write_eio(text.as_bytes())?;
            } else if children.report_flags().display_performance_simulation_flag {
                if children.state().global.kind_of_sim == 3 {
                    let message = "Continuing Simulation at ".to_owned()
                        + &children.state().environment.current_month_day_year
                        + " for "
                        + &children.state().environment.environment_name;
                    children.display_string(&message)?;
                } else {
                    let message = "Continuing Simulation at ".to_owned()
                        + &children.state().environment.current_month_day
                        + " for "
                        + &children.state().environment.environment_name;
                    children.display_string(&message)?;
                }
                children
                    .report_flags_mut()
                    .display_performance_simulation_flag = false;
            }
            if children.state().global.day_of_sim > 365
                && children
                    .state()
                    .global
                    .num_of_day_in_envrn
                    .checked_sub(children.state().global.day_of_sim)
                    .ok_or_else(|| {
                        input_error(HeatBalanceInputError::Unavailable(
                            "actual last-year signed subtraction480",
                        ))
                    })?
                    == 364
                && !children.state().global.warmup_flag
            {
                let message = "Starting last  year of environment at:  ".to_owned()
                    + &children.state().global.day_of_sim_chr;
                children.display_string(&message)?;
                children.reset_tabular_reports()?;
            }
            children.state_mut().global.hour_of_day = 1;
            while children.state().global.hour_of_day <= 24 {
                if children.globals()?.stop_simulation {
                    break;
                }
                children.state_mut().global.begin_hour_flag = true;
                children.state_mut().global.end_hour_flag = false;
                children.state_mut().global.time_step = 1;
                while children.state().global.time_step
                    <= children.state().global.time_steps_in_hour
                {
                    if children.globals()?.stop_simulation {
                        break;
                    }
                    if children.globals()?.any_slabs_in_model
                        || children.globals()?.any_basements_in_model
                    {
                        children.simulate_ground_domains(false)?;
                    }
                    if *any_underwater_boundaries {
                        children.update_underwater_boundaries()?;
                    }
                    if children
                        .state()
                        .environment
                        .varying_location_latitude_schedule
                        .is_some()
                        || children
                            .state()
                            .environment
                            .varying_location_longitude_schedule
                            .is_some()
                        || children.varying_orientation_schedule()?.is_some()
                    {
                        children.update_location_and_orientation()?;
                    }
                    children.state_mut().global.begin_time_step_flag = true;
                    children.external_interface_exchange_variables()?;
                    if children.state().global.time_step
                        == children.state().global.time_steps_in_hour
                    {
                        children.state_mut().global.end_hour_flag = true;
                        if children.state().global.hour_of_day == 24 {
                            children.state_mut().global.end_day_flag = true;
                            if !children.state().global.warmup_flag
                                && children.state().global.day_of_sim
                                    == children.state().global.num_of_day_in_envrn
                            {
                                children.state_mut().global.end_envrn_flag = true;
                            }
                        }
                    }
                    children.manage_weather_exterior_heat_balance()?;
                    if *one_time_underwater_boundary_check {
                        *any_underwater_boundaries =
                            children.check_if_any_underwater_boundaries()?;
                        *one_time_underwater_boundary_check = false;
                    }
                    children.state_mut().global.begin_hour_flag = false;
                    children.state_mut().global.begin_day_flag = false;
                    children.state_mut().global.begin_envrn_flag = false;
                    children.state_mut().global.begin_sim_flag = false;
                    // C++ for increment executes after the body. It may leave
                    // TimeStep=TimeStepsInHour+1 before the enclosing hour suffix.
                    increment(&mut children.state_mut().global.time_step)?;
                }
                {
                    let clock = &mut children.state_mut().global;
                    clock.previous_hour = clock.hour_of_day;
                }
                // C++ for increment leaves HourOfDay25 on normal day completion.
                increment(&mut children.state_mut().global.hour_of_day)?;
            }
            if children.sqlite_present() {
                children.sqlite_commit()?;
            }
        }
        children.external_interface_exchange_variables()?;
    }
    children.state_mut().global.warmup_flag = false;
    Ok(())
}
