//! Actual SimulationManager environment/report checks, borrowing the same owners.
use super::StartupError;
use super::fatal;
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_input_processor::SelectedInputProcessor;
use crate::weather::day::SetupSimulationControls;
use std::path::Path;

// Actual FileSystem::fileExists: exists and not a directory, following symlinks.
// Unexpected filesystem errors propagate instead of becoming "not attached".
fn weather_file_exists(path: &Path) -> Result<bool, StartupError> {
    match std::fs::metadata(path) {
        Ok(metadata) => Ok(!metadata.is_dir()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(StartupError::Filesystem(error)),
    }
}

pub(crate) fn check_environment_specifications(
    input: &SelectedInputProcessor,
    controls: SetupSimulationControls,
    run_control_in_input: bool,
    run_periods_in_input: bool,
    globals: &SelectedHeatBalanceGlobals,
    input_weather_file_path: &Path,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), StartupError> {
    let mut errors_found = false;
    let zone = input.get_num_objects_found("Sizing:Zone");
    let system = input.get_num_objects_found("Sizing:System");
    let plant = input.get_num_objects_found("Sizing:Plant");
    let design_days = input.get_num_objects_found("SizingPeriod:DesignDay");
    let weather_days = input.get_num_objects_found("SizingPeriod:WeatherFileDays");
    let condition_types = input.get_num_objects_found("SizingPeriod:WeatherFileConditionType");
    let sizing_days = design_days > 0 || weather_days > 0 || condition_types > 0;
    let weather_design = weather_days > 0 || condition_types > 0;
    let weather_attached = weather_file_exists(input_weather_file_path)?;
    if run_control_in_input {
        if controls.zone_sizing() {
            if zone > 0 && !sizing_days {
                errors_found = true;
                backend.show_severe_error("CheckEnvironmentSpecifications: Sizing for Zones has been requested but there are no design environments specified.")?;
                backend.show_continue_error(
                    "...Add appropriate SizingPeriod:* objects for your simulation.",
                )?;
            }
            if zone > 0 && weather_design && !weather_attached {
                errors_found = true;
                backend.show_severe_error("CheckEnvironmentSpecifications: Sizing for Zones has been requested; Design period from the weather file requested; but no weather file specified.")?;
            }
        }
        if controls.system_sizing() {
            if system > 0 && !sizing_days {
                errors_found = true;
                backend.show_severe_error("CheckEnvironmentSpecifications: Sizing for Systems has been requested but there are no design environments specified.")?;
                backend.show_continue_error(
                    "...Add appropriate SizingPeriod:* objects for your simulation.",
                )?;
            }
            if system > 0 && weather_design && !weather_attached {
                errors_found = true;
                backend.show_severe_error("CheckEnvironmentSpecifications: Sizing for Systems has been requested; Design period from the weather file requested; but no weather file specified.")?;
            }
        }
        if controls.plant_sizing() {
            if plant > 0 && !sizing_days {
                errors_found = true;
                backend.show_severe_error("CheckEnvironmentSpecifications: Sizing for Equipment/Plants has been requested but there are no design environments specified.")?;
                backend.show_continue_error(
                    "...Add appropriate SizingPeriod:* objects for your simulation.",
                )?;
            }
            if plant > 0 && weather_design && !weather_attached {
                errors_found = true;
                backend.show_severe_error("CheckEnvironmentSpecifications: Sizing for Equipment/Plants has been requested; Design period from the weather file requested; but no weather file specified.")?;
            }
        }
        if controls.run_sizing_periods() && !sizing_days {
            backend.show_warning_error("CheckEnvironmentSpecifications: SimulationControl specified doing design day simulations, but no design environments specified.")?;
            backend.show_continue_error("...No design environment results produced. For these results, add appropriate SizingPeriod:* objects for your simulation.")?;
        }
        if controls.run_sizing_periods() && weather_design && !weather_attached {
            errors_found = true;
            backend.show_severe_error("CheckEnvironmentSpecifications: SimulationControl specified doing design day simulations; weather file design environments specified; but no weather file specified.")?;
        }
        if controls.run_weather_periods() && !run_periods_in_input {
            backend.show_warning_error("CheckEnvironmentSpecifications: SimulationControl specified doing weather simulations, but no run periods for weather file specified.  No annual results produced.")?;
        }
        if controls.run_weather_periods() && run_periods_in_input && !weather_attached {
            backend.show_warning_error("CheckEnvironmentSpecifications: SimulationControl specified doing weather simulations; run periods for weather file specified; but no weather file specified.")?;
        }
    }
    if !controls.run_sizing_periods() && !controls.run_weather_periods() {
        if !controls.hvac_sizing_simulation() && !globals.do_pure_load_calc {
            backend.show_severe_error("All elements of SimulationControl are set to \"No\". No simulations can be done. Program terminates.")?;
            errors_found = true;
        } else {
            backend.show_warning_error("\"Run Simulation for Sizing Periods\" and \"Run Simulation for Weather File Run Periods\" are both set to \"No\". No simulations will be performed, and most input will not be read.")?;
        }
    }
    if errors_found {
        return fatal(backend, "Program terminates due to preceding conditions.");
    }
    Ok(())
}

pub(crate) fn check_for_requested_reporting(
    input: &SelectedInputProcessor,
    controls: SetupSimulationControls,
    globals: &SelectedHeatBalanceGlobals,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), StartupError> {
    let simulation_periods = input.get_num_objects_found("SizingPeriod:DesignDay") > 0
        || input.get_num_objects_found("SizingPeriod:WeatherFileDays") > 0
        || input.get_num_objects_found("SizingPeriod:WeatherFileConditionType") > 0
        || input.get_num_objects_found("RunPeriod") > 0;
    if (controls.run_sizing_periods()
        || controls.run_weather_periods()
        || globals.do_pure_load_calc)
        && simulation_periods
    {
        let requested = [
            "Output:Table:SummaryReports",
            "Output:Table:TimeBins",
            "Output:Table:Monthly",
            "Output:Table:Annual",
            "Output:Variable",
            "Output:Meter",
            "Output:Meter:MeterFileOnly",
            "Output:Meter:Cumulative",
            "Output:Meter:Cumulative:MeterFileOnly",
        ]
        .iter()
        .any(|name| input.get_num_objects_found(name) > 0);
        if !requested {
            backend.show_warning_error(
                "No reporting elements have been requested. No simulation results produced.",
            )?;
            backend.show_continue_error("...Review requirements such as \"Output:Table:SummaryReports\", \"Output:Table:Monthly\", \"Output:Variable\", \"Output:Meter\" and others.")?;
        }
    }
    Ok(())
}
