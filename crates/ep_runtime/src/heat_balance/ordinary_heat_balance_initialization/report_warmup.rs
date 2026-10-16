//! Actual ReportHeatBalance3356--3411 else branches after the real normal guard.
//! ReportSchedule3340 and the normal branch remain the genuine parent's responsibility.
use crate::heat_balance::radiation::ordinary::source_children::SourceSolarReportFlags;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_project_data::SelectedProjectData;
use crate::heat_balance::selected_simulation_startup::SourceHvacSizingSimulationData;
use crate::heat_balance::source_output_registration::SourceOutputReportStreams;
use crate::weather::day::{WeatherEnvironmentState, WeatherGlobalState};

/// All accessors borrow the actual retained caller owners. No successful defaults.
/// A real host must call the concrete stamp helper and same Zone UpdateData child;
/// no clock, output registration, stream or report flag may be copied into this host.
pub(crate) trait SourceHeatBalanceWarmupChildren {
    fn clock(&self) -> &WeatherGlobalState;
    fn globals(&self) -> &SelectedHeatBalanceGlobals;
    fn project(&self) -> &SelectedProjectData;
    fn hvac_sizing_simulation(&self) -> &SourceHvacSizingSimulationData;
    fn write_warmup_environment_stamp(&mut self) -> Result<(), Error>;
    fn calculate_more_node_info(&mut self) -> Result<(), Error>;
    fn update_zone_data_and_report(&mut self) -> Result<(), Error>;
}

/// Enter only after Native3342 normal predicate evaluated false on this SAME clock.
/// No additional WarmupFlag/phase inference: external-interface3404 is a separate guard.
pub(crate) fn report_source_heat_balance_warmup_branch(
    children: &mut impl SourceHeatBalanceWarmupChildren,
) -> Result<(), Error> {
    if !children.globals().kickoff_simulation
        && children.clock().do_output_reporting
        && children.project().system.report_during_warmup
    {
        children.write_warmup_environment_stamp()?;
        if !children.globals().doing_sizing {
            children.calculate_more_node_info()?;
        }
        children.update_zone_data_and_report()?;
        if children.clock().kind_of_sim == 4 || children.clock().kind_of_sim == 5 {
            children
                .hvac_sizing_simulation()
                .update_source_zone_step_logs()?;
        }
    } else if children
        .project()
        .system
        .update_data_during_warmup_external_interface
    {
        children.update_zone_data_and_report()?;
        if children.clock().kind_of_sim == 4 || children.clock().kind_of_sim == 5 {
            children
                .hvac_sizing_simulation()
                .update_source_zone_step_logs()?;
        }
    }
    Ok(())
}

/// Genuine fixed7.2F text: uppercase nonfinite spelling/sign/default-LEFT padding follows fmt8.
/// Finite formatting uses fixed two fractional digits, space padding and retained -0.
pub(crate) fn source_fixed_7_2_upper(value: f64) -> String {
    if value.is_finite() {
        format!("{value:7.2}")
    } else {
        let name = if value.is_infinite() { "INF" } else { "NAN" };
        let signed = if value.is_sign_negative() {
            format!("-{name}")
        } else {
            name.to_owned()
        };
        format!("{signed:<7}")
    }
}
fn warmup_environment_stamp(
    environment: &WeatherEnvironmentState,
    report_flags: &SourceSolarReportFlags,
) -> String {
    format!(
        "1,Warmup {{{}}} {},{},{},{},{}\n",
        report_flags.warmup_day,
        environment.environment_name,
        source_fixed_7_2_upper(environment.latitude),
        source_fixed_7_2_upper(environment.longitude),
        source_fixed_7_2_upper(environment.time_zone_number),
        source_fixed_7_2_upper(environment.elevation),
    )
}

/// Native3357--3391 prefix; actual ESO then MTR writes have NO extra good/SQLite guards.
/// Environment/header lines do not increment the OutputProcessor record counters.
pub(crate) fn write_source_heat_balance_warmup_environment_stamp(
    clock: &WeatherGlobalState,
    environment: &mut WeatherEnvironmentState,
    report_flags: &mut SourceSolarReportFlags,
    streams: &mut impl SourceOutputReportStreams,
) -> Result<(), Error> {
    if clock.begin_day_flag && !environment.print_environment_stamp_warmup_printed {
        environment.print_environment_stamp_warmup = true;
        environment.print_environment_stamp_warmup_printed = true;
    }
    if !clock.begin_day_flag {
        environment.print_environment_stamp_warmup_printed = false;
    }
    if environment.print_environment_stamp_warmup {
        if report_flags.print_end_data_dictionary && clock.do_output_reporting {
            streams.write_eso(b"End of Data Dictionary\n")?;
            streams.write_mtr(b"End of Data Dictionary\n")?;
            report_flags.print_end_data_dictionary = false;
        }
        if clock.do_output_reporting {
            streams.write_eso(warmup_environment_stamp(environment, report_flags).as_bytes())?;
            streams.write_mtr(warmup_environment_stamp(environment, report_flags).as_bytes())?;
            environment.print_environment_stamp_warmup = false;
        }
    }
    Ok(())
}
