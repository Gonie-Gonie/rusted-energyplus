//! Selected processEnvironmentVariables operands shared with GetProjectData.
//! Other Source environment consumers keep their own genuine caller operations.
use super::{ProjectDataError, SelectedHeatBalanceGlobals, SelectedProjectData};
use crate::weather::day::WeatherEnvironmentState;

fn get(name: &str) -> Result<String, ProjectDataError> {
    match std::env::var(name) {
        Ok(value) => Ok(value),
        Err(std::env::VarError::NotPresent) => Ok(String::new()),
        Err(std::env::VarError::NotUnicode(_)) => Err(ProjectDataError::Unavailable(
            "non-Unicode actual environment value",
        )),
    }
}
fn on(value: &str) -> bool {
    value
        .as_bytes()
        .first()
        .is_some_and(|byte| matches!(byte, b'Y' | b'y' | b'T' | b't'))
}
fn update(name: &str, field: &mut bool) -> Result<(), ProjectDataError> {
    let value = get(name)?;
    if !value.is_empty() {
        *field = on(&value);
    }
    Ok(())
}
impl SelectedProjectData {
    pub(crate) fn resolve_environment_operands(
        &mut self,
        globals: &mut SelectedHeatBalanceGlobals,
        environment: &mut WeatherEnvironmentState,
        cli_dd_only: bool,
        cli_annual: bool,
        reverse_design_days: bool,
    ) -> Result<(), ProjectDataError> {
        self.system.dd_only = on(&get("DDONLY")?);
        if cli_dd_only {
            self.system.dd_only = true;
        }
        // REVERSEDD remains Header's sole retained actual DD configuration.
        self.system.full_annual_run = on(&get("FULLANNUALRUN")?);
        if cli_annual {
            self.system.full_annual_run = true;
        }
        globals.display_all_warnings = on(&get("DisplayAllWarnings")?);
        if globals.display_all_warnings {
            globals.display_all_warnings = true;
            globals.display_extra_warnings = true;
            globals.display_unused_schedules = true;
            globals.display_unused_objects = true;
        }
        update("DisplayExtraWarnings", &mut globals.display_extra_warnings)?;
        update("DisplayUnusedObjects", &mut globals.display_unused_objects)?;
        update(
            "DisplayUnusedSchedules",
            &mut globals.display_unused_schedules,
        )?;
        update(
            "DisplayZoneAirHeatBalanceOffBalance",
            &mut globals.display_zone_air_heat_balance_off_balance,
        )?;
        update(
            "DisplayAdvancedReportVariables",
            &mut globals.display_advanced_report_variables,
        )?;
        update("ReportDuringWarmup", &mut self.system.report_during_warmup)?;
        if reverse_design_days {
            self.system.report_during_warmup = false;
        }
        // The Original reads this variable a second time after ReverseDD.
        update("ReportDuringWarmup", &mut self.system.report_during_warmup)?;
        update(
            "REPORTDURINGHVACSIZINGSIMULATION",
            &mut self.system.report_during_hvac_sizing_simulation,
        )?;
        update(
            "IgnoreSolarRadiation",
            &mut environment.ignore_solar_radiation,
        )?;
        let minimum_report_frequency = get("MINREPORTFREQUENCY")?;
        if !minimum_report_frequency.is_empty() {
            self.system.minimum_report_frequency = minimum_report_frequency;
        }
        update("DeveloperFlag", &mut self.system.developer_flag)?;
        update(
            "IgnoreBeamRadiation",
            &mut environment.ignore_beam_radiation,
        )?;
        update(
            "IgnoreDiffuseRadiation",
            &mut environment.ignore_diffuse_radiation,
        )?;
        update(
            "SutherlandHodgman",
            &mut self.system.shadowing_clipping.sutherland_hodgman,
        )?;
        update(
            "SlaterBarsky",
            &mut self.system.shadowing_clipping.slater_barsky,
        )?;
        update("MinimalShadowing", &mut self.system.minimal_shadowing)?;
        update("TimingFlag", &mut self.system.timing_flag)?;
        update("TRACK_AIRLOOP", &mut self.system.track_air_loop_env_flag)?;
        update("CI_FORCE_TIME_STEP", &mut self.system.ci_force_time_step)?;
        Ok(())
    }
}
