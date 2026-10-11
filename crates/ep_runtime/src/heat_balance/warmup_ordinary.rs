//! Explicit ordinary A report/diagnostic caller; generic/controlled inputs stay absent.
use super::report::{WarmupRecordClock, WarmupReportConfiguration, WarmupReportState};
use super::*;
use crate::heat_balance::ctf_initialization_context::CtfInitializationContext;
use crate::weather::day::{ProductionWarmupReportContext, WeatherDayPhase};

fn view(
    driver: Option<HeatBalanceWeatherDriver<'_>>,
) -> Result<ProductionWarmupReportContext, RuntimeError> {
    match driver {
        Some(HeatBalanceWeatherDriver::Production(series)) => Ok(series.warmup_report_context()?),
        _ => Err(unavailable(
            "ordinary warmup reporting requires actual production weather owners",
        )),
    }
}

fn context<'a, 'sink>(
    value: Option<&'a mut CtfInitializationContext<'sink>>,
) -> Result<&'a mut CtfInitializationContext<'sink>, RuntimeError> {
    value.ok_or_else(|| {
        unavailable("ordinary warmup reporting requires the actual retained EIO/backend")
    })
}

fn clock(current: &ProductionWarmupReportContext) -> Result<WarmupRecordClock, RuntimeError> {
    let day = u32::try_from(current.day).map_err(|_| unavailable("actual report DayOfSim"))?;
    let hour = u32::try_from(current.hour).map_err(|_| unavailable("actual report HourOfDay"))?;
    let time_step =
        u32::try_from(current.time_step).map_err(|_| unavailable("actual report TimeStep"))?;
    if day == 0
        || !(1..=24).contains(&hour)
        || time_step == 0
        || i32::try_from(time_step)
            .ok()
            .is_none_or(|step| step > current.time_steps_in_hour)
    {
        return Err(unavailable("actual report clock outside owner bounds"));
    }
    Ok(WarmupRecordClock {
        day,
        hour,
        time_step,
    })
}

pub(crate) fn prepare(
    model: &SimulationModel,
    state: &mut HeatBalanceState,
    driver: Option<HeatBalanceWeatherDriver<'_>>,
    input: super::super::state::HeatBalanceOrdinaryWarmupReportInputs,
    output: &mut CtfInitializationContext<'_>,
) -> Result<(), RuntimeError> {
    admit_uncontrolled_warmup(model, state)?;
    let current = view(driver)?;
    let _ = clock(&current)?;
    if !current.begin_environment
        || !current.run_period_environment
        || output.backend.state.doing_sizing
        || input.do_pure_load_calc
    {
        return Err(unavailable(
            "ordinary A report caller requires actual weather BeginEnvrn/non-sizing controls",
        ));
    }
    let mut report = WarmupReportState::new(
        state
            .zones
            .iter()
            .map(|zone| zone.zone_name.clone())
            .collect(),
        u32::try_from(current.time_steps_in_hour)
            .map_err(|_| unavailable("actual report allocation extent"))?,
        WarmupReportConfiguration {
            doing_sizing: output.backend.state.doing_sizing,
            do_pure_load_calc: input.do_pure_load_calc,
            run_period_environment: current.run_period_environment,
            environment_name: current.environment_name,
            detailed: input.detailed,
        },
    )?;
    let mut zones = state
        .zones
        .iter()
        .map(|zone| HeatBalanceWarmupZoneConvergence::new(zone.zone_id))
        .collect::<Vec<_>>();
    // InitHeatBalance's actual BeginEnvrn assignments are distinct from Allocate.
    // LoadZoneSecPrevDay is deliberately untouched (+0 from this fresh allocation).
    for zone in &mut zones {
        zone.previous.fill(0.0);
        zone.current = [-9999.0, -9999.0, -9999.0, 1000.0];
        zone.rolling[0] = -9999.0;
        zone.rolling[1] = -9999.0;
        zone.rolling[2] = 1000.0;
        zone.rolling[3] = -9999.0;
        zone.rolling[4] = -9999.0;
    }
    report.begin_environment()?;
    state.warmup_convergence = Some(HeatBalanceWarmupConvergenceState {
        zones,
        checked_day: 0,
        convergence_checks_failed: false,
        warmup_flag: current.warmup_flag,
        maximum_day_forced_exit: false,
        unavailable_reason: None,
        report: Some(report),
    });
    output.backend.state.warmup_flag = current.warmup_flag;
    Ok(())
}

/// Real post-thermal RecKeep call, before caller commit. Every Run step records.
pub(crate) fn record_completed_step(
    state: &mut HeatBalanceState,
    driver: Option<HeatBalanceWeatherDriver<'_>>,
    output: Option<&mut CtfInitializationContext<'_>>,
) -> Result<(), RuntimeError> {
    let current = view(driver)?;
    let stamp = clock(&current)?;
    let output = context(output)?;
    output.backend.state.warmup_flag = current.warmup_flag;
    let owner = state
        .warmup_convergence
        .as_mut()
        .ok_or_else(|| unavailable("missing actual ordinary convergence owner"))?;
    if let Some(reason) = owner.unavailable_reason {
        return Err(unavailable(reason));
    }
    let report = owner
        .report
        .as_mut()
        .ok_or_else(|| unavailable("missing actual ordinary report allocation"))?;
    if owner.warmup_flag != current.warmup_flag
        || report.configuration.doing_sizing != output.backend.state.doing_sizing
        || report.configuration.environment_name != current.environment_name
        || report.configuration.run_period_environment != current.run_period_environment
        || owner.zones.len() != state.zones.len()
    {
        return Err(unavailable(
            "ordinary report phase/environment/Zone owner mismatch",
        ));
    }
    for (index, (zone, actual)) in owner.zones.iter_mut().zip(&state.zones).enumerate() {
        if zone.zone_id != actual.zone_id
            || zone
                .sensible_report_timestep
                .and_then(|step| step.checked_add(1))
                != Some(state.timestep_index)
        {
            return Err(unavailable(
                "ordinary RecKeep requires this actual timestep's accepted sensible report",
            ));
        }
        let (heat, cool) = zone
            .air_system_sensible_rates_w
            .ok_or_else(|| unavailable("unobserved ordinary sensible rates"))?;
        let ztav = actual.zone_timestep_average_air_temperature_c;
        if !ztav.is_finite() || !heat.is_finite() || !cool.is_finite() || heat < 0.0 || cool < 0.0 {
            return Err(unavailable(
                "nonfinite or negative ordinary Record operands",
            ));
        }
        zone.record(ztav, heat, cool);
        report.record_zone(index, zone, current.warmup_flag, stamp, output.eio)?;
    }
    // Exact ManageHeatBalance suffix. Partial first-day samples never reach this.
    if !current.warmup_flag
        && current.end_day
        && stamp.day == 1
        && !output.backend.state.doing_sizing
    {
        report.report(false, &owner.zones, output.eio)?;
    }
    Ok(())
}

fn round(value: f64, precision: usize) -> Result<String, RuntimeError> {
    super::super::source_round_format::round(value, precision).map_err(|error| {
        RuntimeError::WarmupOutput(Box::new(crate::error::WarmupOutputError::Format(format!(
            "{error:?}"
        ))))
    })
}

/// Fallible selected diagnostics precede each transfer/reset, then the flag decision.
pub(crate) fn check_day(
    owner: &mut HeatBalanceWarmupConvergenceState,
    day: u32,
    options: HeatBalanceWarmupOptions,
    driver: Option<HeatBalanceWeatherDriver<'_>>,
    output: Option<&mut CtfInitializationContext<'_>>,
) -> Result<(), RuntimeError> {
    let current = view(driver)?;
    let output = context(output)?;
    if !current.warmup_flag
        || !current.end_day
        || current.day != i32::try_from(day).unwrap_or(-1)
        || current.phase != (WeatherDayPhase::Warmup { day })
    {
        return Err(unavailable(
            "ordinary convergence check requires actual warmup EndDay",
        ));
    }
    check_day_with_context(owner, day, options, &current, output)
}

fn check_day_with_context(
    owner: &mut HeatBalanceWarmupConvergenceState,
    day: u32,
    options: HeatBalanceWarmupOptions,
    current: &ProductionWarmupReportContext,
    output: &mut CtfInitializationContext<'_>,
) -> Result<(), RuntimeError> {
    let report = owner
        .report
        .as_mut()
        .ok_or_else(|| unavailable("missing actual ordinary report owner"))?;
    if report.zone_names.len() != owner.zones.len()
        || report.configuration.doing_sizing != output.backend.state.doing_sizing
    {
        return Err(unavailable(
            "ordinary diagnostic Zone/configuration owner mismatch",
        ));
    }
    output.backend.state.warmup_flag = current.warmup_flag;
    owner.convergence_checks_failed = false;
    for (index, zone) in owner.zones.iter_mut().enumerate() {
        owner.convergence_checks_failed |= zone.check(
            options.temperature_convergence_tolerance_delta_c,
            options.loads_convergence_tolerance_w,
        );
        if day >= options.maximum_days
            && owner.warmup_flag
            && zone
                .pass_flags
                .iter()
                .map(|flag| u32::from(*flag))
                .sum::<u32>()
                != 8
        {
            output.backend.show_severe_error(&format!("CheckWarmupConvergence: Loads Initialization, Zone=\"{}\" did not converge after {} warmup days.", report.zone_names[index], options.maximum_days))?;
            if !report.warmup_convergence_warning && !output.backend.state.doing_sizing {
                output.backend.show_continue_error(
                    "See Warmup Convergence Information in .eio file for details.",
                )?;
                report.warmup_convergence_warning = true;
            } else if !report.sizing_warmup_convergence_warning && output.backend.state.doing_sizing
            {
                output
                    .backend
                    .show_continue_error("Warmup Convergence failing during sizing.")?;
                report.sizing_warmup_convergence_warning = true;
            }
            let environment_type = if current.run_period_environment {
                "RunPeriod"
            } else {
                "SizingPeriod"
            };
            output.backend.show_continue_error(&format!(
                "...Environment({environment_type})=\"{}\"",
                current.environment_name
            ))?;
            let pass_fail = |flag| if flag == 2 { "Pass" } else { "Fail" };
            output.backend.show_continue_error(&format!("..Max Temp Comparison = {} vs Temperature Convergence Tolerance={} - {} Convergence", round(zone.test_values[0],2)?, round(options.temperature_convergence_tolerance_delta_c,2)?, pass_fail(zone.pass_flags[0])))?;
            output.backend.show_continue_error(&format!("..Min Temp Comparison = {} vs Temperature Convergence Tolerance={} - {} Convergence", round(zone.test_values[1],2)?, round(options.temperature_convergence_tolerance_delta_c,2)?, pass_fail(zone.pass_flags[1])))?;
            output.backend.show_continue_error(&format!("..Max Heat Load Comparison = {} vs Loads Convergence Tolerance={} - {} Convergence", round(zone.test_values[2],4)?, round(options.loads_convergence_tolerance_w,2)?, pass_fail(zone.pass_flags[2])))?;
            output.backend.show_continue_error(&format!("..Max Cool Load Comparison = {} vs Loads Convergence Tolerance={} - {} Convergence", round(zone.test_values[3],4)?, round(options.loads_convergence_tolerance_w,2)?, pass_fail(zone.pass_flags[3])))?;
        }
        zone.retain_and_reset();
    }
    if day >= options.maximum_days
        && owner.warmup_flag
        && owner.convergence_checks_failed
        && options.maximum_days < 25
    {
        output.backend.show_severe_error(&format!(
            "CheckWarmupConvergence: User supplied maximum warmup days={} is insufficient.",
            options.maximum_days
        ))?;
        output
            .backend
            .show_continue_error("Suggest setting maximum number of warmup days to at least 25.")?;
    }
    owner.checked_day = day;
    if !owner.convergence_checks_failed {
        owner.warmup_flag = day < options.minimum_days;
    }
    owner.maximum_day_forced_exit = day >= options.maximum_days && owner.warmup_flag;
    if day >= options.maximum_days && owner.warmup_flag {
        owner.warmup_flag = false;
    }
    Ok(())
}

#[cfg(test)]
#[path = "warmup_ordinary_tests.rs"]
mod tests;
