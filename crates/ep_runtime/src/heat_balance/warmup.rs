//! Run-period warmup loop and CheckWarmupConvergence helpers.

use crate::error::RuntimeError;
use crate::heat_balance::algorithm::HeatBalanceRuntimeConfig;
use crate::heat_balance::state::{
    HeatBalanceState, HeatBalanceStepInput, HeatBalanceSurfaceLoopZoneAirCorrection,
    HeatBalanceWarmupConvergenceState, HeatBalanceWarmupDayEndZoneAirStateSample,
    HeatBalanceWarmupOptions, HeatBalanceWarmupZoneConvergence,
};
use crate::heat_balance::summary::HeatBalanceWarmupSummary;
use crate::heat_balance::trace::heat_balance_zone_air_state_sample;
use crate::heat_balance::weather_driver::HeatBalanceWeatherDriver;
use crate::weather::day::WeatherDayPhase;
use crate::weather::{
    EpwRecord, HeatBalanceWeatherContext,
    energyplus_weather_dry_bulb_at_timestep_with_starting_values,
};
use ep_model::{FirstHourInterpolationStartingValues, SimulationModel, TypedModel};

#[cfg(test)]
pub(crate) fn run_heat_balance_run_period_warmup<F>(
    simulation_model: &SimulationModel,
    state: &mut HeatBalanceState,
    weather_dry_bulb_c: &[f64],
    weather_records: Option<&[EpwRecord]>,
    weather_driver: Option<HeatBalanceWeatherDriver<'_>>,
    zone_steps_per_hour: u32,
    seconds_per_timestep: f64,
    options: HeatBalanceWarmupOptions,
    runtime_config: HeatBalanceRuntimeConfig,
    surface_iteration_count: u32,
    inside_hconv_reevaluation_interval: Option<u32>,
    surface_loop_zone_air_correction: HeatBalanceSurfaceLoopZoneAirCorrection,
    first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
    day_end_zone_air_states: &mut Vec<HeatBalanceWarmupDayEndZoneAirStateSample>,
    advance_timestep: F,
) -> Result<HeatBalanceWarmupSummary, RuntimeError>
where
    F: for<'weather> FnMut(
        &TypedModel,
        &mut HeatBalanceState,
        HeatBalanceStepInput,
        Option<HeatBalanceWeatherContext<'weather>>,
        HeatBalanceRuntimeConfig,
        u32,
        Option<u32>,
        HeatBalanceSurfaceLoopZoneAirCorrection,
    ) -> Result<(), RuntimeError>,
{
    run_heat_balance_run_period_warmup_with_reporting(
        simulation_model,
        state,
        weather_dry_bulb_c,
        weather_records,
        weather_driver,
        zone_steps_per_hour,
        seconds_per_timestep,
        options,
        runtime_config,
        surface_iteration_count,
        inside_hconv_reevaluation_interval,
        surface_loop_zone_air_correction,
        first_hour_interpolation_starting_values,
        day_end_zone_air_states,
        None,
        advance_timestep,
    )
}

pub(crate) fn run_heat_balance_run_period_warmup_with_reporting<F>(
    simulation_model: &SimulationModel,
    state: &mut HeatBalanceState,
    weather_dry_bulb_c: &[f64],
    weather_records: Option<&[EpwRecord]>,
    weather_driver: Option<HeatBalanceWeatherDriver<'_>>,
    zone_steps_per_hour: u32,
    seconds_per_timestep: f64,
    options: HeatBalanceWarmupOptions,
    runtime_config: HeatBalanceRuntimeConfig,
    surface_iteration_count: u32,
    inside_hconv_reevaluation_interval: Option<u32>,
    surface_loop_zone_air_correction: HeatBalanceSurfaceLoopZoneAirCorrection,
    first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
    day_end_zone_air_states: &mut Vec<HeatBalanceWarmupDayEndZoneAirStateSample>,
    mut output: Option<&mut super::ctf_initialization_context::CtfInitializationContext<'_>>,
    mut advance_timestep: F,
) -> Result<HeatBalanceWarmupSummary, RuntimeError>
where
    F: for<'weather> FnMut(
        &TypedModel,
        &mut HeatBalanceState,
        HeatBalanceStepInput,
        Option<HeatBalanceWeatherContext<'weather>>,
        HeatBalanceRuntimeConfig,
        u32,
        Option<u32>,
        HeatBalanceSurfaceLoopZoneAirCorrection,
    ) -> Result<(), RuntimeError>,
{
    let hourly_count =
        weather_driver.map_or(weather_dry_bulb_c.len(), |driver| driver.hourly_count());
    if !options.enabled || options.maximum_days == 0 || hourly_count == 0 {
        return Ok(HeatBalanceWarmupSummary::disabled());
    }

    admit_uncontrolled_warmup(simulation_model, state)?;
    if !options
        .temperature_convergence_tolerance_delta_c
        .is_finite()
        || options.temperature_convergence_tolerance_delta_c < 0.0
        || !options.loads_convergence_tolerance_w.is_finite()
        || options.loads_convergence_tolerance_w < 0.0
    {
        return Err(unavailable("nonfinite or negative convergence tolerance"));
    }
    let model = &simulation_model.typed;
    if state
        .warmup_convergence
        .as_ref()
        .is_none_or(|owner| owner.report.is_none())
    {
        state.warmup_convergence = Some(HeatBalanceWarmupConvergenceState {
            zones: state
                .zones
                .iter()
                .map(|zone| HeatBalanceWarmupZoneConvergence::new(zone.zone_id))
                .collect(),
            checked_day: 0,
            convergence_checks_failed: false,
            warmup_flag: true,
            maximum_day_forced_exit: false,
            unavailable_reason: None,
            report: None,
        });
    }
    let hours_per_day = hourly_count.min(24);
    let maximum_days = options.maximum_days.max(options.minimum_days).max(1);
    let tolerance = options.temperature_convergence_tolerance_delta_c;
    let timestep_start = state.timestep_index;
    let mut final_delta = f64::INFINITY;

    for day in 1..=maximum_days {
        if let Some(driver) = weather_driver {
            driver.begin_day(WeatherDayPhase::Warmup { day })?;
        }
        for hour_index in 0..hours_per_day {
            let hour_ending = u32::try_from(hour_index % 24 + 1).unwrap_or(24);
            let steps = zone_steps_per_hour.max(1);
            for substep in 1..=steps {
                let weather_context = weather_driver
                    .map(|driver| {
                        driver.current_context(
                            hour_index,
                            steps,
                            substep,
                            first_hour_interpolation_starting_values,
                        )
                    })
                    .transpose()?;
                let outdoor_dry_bulb_c = weather_context
                    .and_then(|context| context.current_record().map(|record| record.dry_bulb_c))
                    .or_else(|| weather_dry_bulb_c.get(hour_index).copied())
                    .ok_or(RuntimeError::NoWeatherData)?;
                let timestep_outdoor_dry_bulb_c = weather_context
                    .and_then(|context| context.sample_value().map(|sample| sample.dry_bulb_c))
                    .unwrap_or_else(|| {
                        energyplus_weather_dry_bulb_at_timestep_with_starting_values(
                            weather_records,
                            hour_index,
                            outdoor_dry_bulb_c,
                            steps,
                            substep,
                            first_hour_interpolation_starting_values,
                        )
                    });
                let prepared_legacy_clock = super::ctf_caller_clock::prepare_legacy_step(
                    state,
                    weather_driver,
                    WeatherDayPhase::Warmup { day },
                    hour_index,
                    steps,
                    substep,
                    seconds_per_timestep,
                )?;
                super::convection::surface_environment::ordinary::invoke_if_active(
                    simulation_model,
                    state,
                    weather_context,
                    weather_driver,
                    output.as_deref_mut(),
                )?;
                advance_timestep(
                    model,
                    state,
                    HeatBalanceStepInput {
                        outdoor_dry_bulb_c: timestep_outdoor_dry_bulb_c,
                        hour_ending,
                        timestep_seconds: seconds_per_timestep,
                    },
                    weather_context,
                    runtime_config,
                    surface_iteration_count,
                    inside_hconv_reevaluation_interval,
                    surface_loop_zone_air_correction,
                )?;
                super::ctf_surface_lifecycle::ensure_available(state)?;
                let reporting = state
                    .warmup_convergence
                    .as_ref()
                    .is_some_and(|owner| owner.report.is_some());
                if reporting {
                    ordinary::record_completed_step(state, weather_driver, output.as_deref_mut())?;
                }
                super::ctf_caller_clock::commit_legacy_step(state, prepared_legacy_clock)?;
                if !reporting {
                    state
                        .warmup_convergence
                        .as_mut()
                        .ok_or_else(|| unavailable("missing actual convergence owner"))?
                        .record_state(&state.zones, state.timestep_index)?;
                }
            }
        }

        day_end_zone_air_states.extend(state.zones.iter().map(|zone| {
            HeatBalanceWarmupDayEndZoneAirStateSample {
                day_index: day,
                state: heat_balance_zone_air_state_sample(zone),
            }
        }));
        let owner = state
            .warmup_convergence
            .as_mut()
            .ok_or_else(|| unavailable("missing actual convergence owner"))?;
        if owner.report.is_some() {
            ordinary::check_day(owner, day, options, weather_driver, output.as_deref_mut())?;
        } else {
            owner.check_day(day, options);
        }
        final_delta = owner
            .zones
            .iter()
            .map(|zone| zone.test_values[0].max(zone.test_values[1]))
            .fold(0.0, f64::max);
        if !owner.warmup_flag {
            let converged = !owner.convergence_checks_failed;
            if let Some(driver) = weather_driver {
                driver.finish_warmup()?;
            }
            if let Some(context) = output.as_deref_mut() {
                context.backend.state.warmup_flag = false;
            }
            return Ok(HeatBalanceWarmupSummary {
                enabled: true,
                minimum_days: options.minimum_days,
                maximum_days,
                temperature_convergence_tolerance_delta_c: tolerance,
                loads_convergence_tolerance_w: options.loads_convergence_tolerance_w,
                day_count: day,
                timestep_count: state.timestep_index - timestep_start,
                hours_per_day,
                converged,
                final_max_zone_temperature_delta_c: final_delta,
            });
        }
    }

    Ok(HeatBalanceWarmupSummary {
        enabled: true,
        minimum_days: options.minimum_days,
        maximum_days,
        temperature_convergence_tolerance_delta_c: tolerance,
        loads_convergence_tolerance_w: options.loads_convergence_tolerance_w,
        day_count: maximum_days,
        timestep_count: state.timestep_index - timestep_start,
        hours_per_day,
        converged: false,
        final_max_zone_temperature_delta_c: final_delta,
    })
}

fn unavailable(reason: impl Into<String>) -> RuntimeError {
    RuntimeError::WarmupUnavailable {
        reason: reason.into(),
    }
}

// Native correctAirTemp selects its uncontrolled branch by the actual absent
// system-zone node. Empty IdealLoads alone does not establish that prerequisite.
fn admit_uncontrolled_warmup(
    model: &SimulationModel,
    state: &HeatBalanceState,
) -> Result<(), RuntimeError> {
    if !model.graph.zone_air_nodes.is_empty()
        || !model.graph.zone_ideal_loads.is_empty()
        || !model.typed.zone_equipment_connections.is_empty()
        || !model.typed.zone_equipment_lists.is_empty()
        || !model.typed.ideal_loads_air_systems.is_empty()
    {
        return Err(unavailable(
            "controlled or unresolved system-zone-node demand producer",
        ));
    }
    if state.zones.is_empty()
        || state.zones.len() != model.typed.zones.len()
        || state
            .zones
            .iter()
            .zip(&model.typed.zones)
            .any(|(actual, declared)| actual.zone_id != declared.id)
    {
        return Err(unavailable(
            "actual Zone owner order differs from declared topology",
        ));
    }
    Ok(())
}

/// Actual uncontrolled corrector report seam, after MAT and before humidity.
/// Only the caller-admitted warmup owner can reach this literal SNLoad=0 branch.
pub(crate) fn report_uncontrolled_sensible_loads(state: &mut HeatBalanceState) {
    let Some(owner) = state.warmup_convergence.as_mut() else {
        return;
    };
    if !owner.warmup_flag && owner.report.is_none() {
        return;
    }
    if owner.zones.len() != state.zones.len()
        || owner
            .zones
            .iter()
            .zip(&state.zones)
            .any(|(owner, actual)| owner.zone_id != actual.zone_id)
    {
        owner.unavailable_reason = Some("Zone ownership changed before sensible reporting");
        return;
    }
    for zone in &mut owner.zones {
        let sn_load = 0.0_f64; // Native uncontrolled correctAirTemp branch, not predicted demand.
        zone.air_system_sensible_rates_w = Some((sn_load.max(0.0), sn_load.min(0.0).abs()));
        zone.sensible_report_timestep = Some(state.timestep_index);
    }
}

impl HeatBalanceWarmupZoneConvergence {
    fn new(zone_id: ep_model::ZoneId) -> Self {
        Self {
            zone_id,
            air_system_sensible_rates_w: None,
            sensible_report_timestep: None,
            current: [-9999.0, -9999.0, -9999.0, 1000.0],
            previous: [0.0; 4],
            rolling: [0.0; 6],
            pass_flags: [2; 4],
            test_values: [0.0; 4],
        }
    }

    fn record(&mut self, ztav: f64, heat_rate: f64, cool_rate: f64) {
        if ztav > self.current[2] {
            self.current[2] = ztav;
        }
        if ztav < self.current[3] {
            self.current[3] = ztav;
        }
        if heat_rate > self.current[0] {
            self.current[0] = heat_rate;
        }
        if cool_rate > self.current[1] {
            self.current[1] = cool_rate;
        }
        self.rolling[4] = self.rolling[2];
        self.rolling[5] = self.rolling[3];
        self.rolling[2] = self.rolling[0];
        self.rolling[3] = self.rolling[1];
        self.rolling[0] = ztav;
        let cool_load = cool_rate.abs();
        // Original ObjexxFCL::max selects the first operand on equality.
        self.rolling[1] = if heat_rate < cool_load {
            cool_load
        } else {
            heat_rate
        };
    }

    fn check(&mut self, temperature_tolerance: f64, load_tolerance: f64) -> bool {
        let mut failed = false;
        self.test_values[0] = (self.previous[2] - self.current[2]).abs();
        self.test_values[1] = (self.previous[3] - self.current[3]).abs();
        if self.test_values[0] <= temperature_tolerance {
            self.pass_flags[0] = 2;
        } else {
            failed = true;
            self.pass_flags[0] = 1;
        }
        if self.test_values[1] <= temperature_tolerance {
            self.pass_flags[1] = 2;
        } else {
            failed = true;
            self.pass_flags[1] = 1;
        }
        failed |= self.check_load(0, 2, load_tolerance);
        failed |= self.check_load(1, 3, load_tolerance);
        failed
    }

    fn check_load(&mut self, load_index: usize, test_index: usize, load_tolerance: f64) -> bool {
        if self.current[load_index] > 1.0e-4 {
            self.current[load_index] = self.current[load_index].max(100.0).abs();
            self.previous[load_index] = self.previous[load_index].max(100.0).abs();
            self.test_values[test_index] = ((self.current[load_index] - self.previous[load_index])
                / self.current[load_index])
                .abs();
            if self.test_values[test_index] <= load_tolerance {
                self.pass_flags[test_index] = 2;
            } else {
                self.pass_flags[test_index] = 1;
                return true;
            }
        } else {
            self.pass_flags[test_index] = 2;
        }
        false
    }

    fn retain_and_reset(&mut self) {
        self.previous[0] = self.current[0];
        self.previous[1] = self.current[1];
        self.previous[2] = self.current[2];
        self.previous[3] = self.current[3];
        self.current = [-9999.0, -9999.0, -9999.0, 1000.0];
    }
}

impl HeatBalanceWarmupConvergenceState {
    fn record_state(
        &mut self,
        zones: &[super::state::ZoneHeatBalanceState],
        next_timestep: usize,
    ) -> Result<(), RuntimeError> {
        if let Some(reason) = self.unavailable_reason {
            return Err(unavailable(reason));
        }
        if self.zones.len() != zones.len() {
            return Err(unavailable("incomplete daily Zone owner array"));
        }
        for (owner, actual) in self.zones.iter_mut().zip(zones) {
            if owner.zone_id != actual.zone_id
                || owner
                    .sensible_report_timestep
                    .and_then(|step| step.checked_add(1))
                    != Some(next_timestep)
            {
                return Err(unavailable("missing actual accepted sensible-load report"));
            }
            let (heat, cool) = owner
                .air_system_sensible_rates_w
                .ok_or_else(|| unavailable("unobserved sensible demand rates"))?;
            let ztav = actual.zone_timestep_average_air_temperature_c;
            if !ztav.is_finite()
                || !heat.is_finite()
                || !cool.is_finite()
                || heat < 0.0
                || cool < 0.0
            {
                return Err(unavailable(
                    "nonfinite or negative actual daily observation",
                ));
            }
            owner.record(ztav, heat, cool);
        }
        Ok(())
    }

    fn check_day(&mut self, day: u32, options: HeatBalanceWarmupOptions) {
        self.convergence_checks_failed = false;
        for zone in &mut self.zones {
            self.convergence_checks_failed |= zone.check(
                options.temperature_convergence_tolerance_delta_c,
                options.loads_convergence_tolerance_w,
            );
            // Native maximum-day severe/continue transport precedes this point.
            // That actual backend/environment prerequisite remains outside this bounded unit.
            zone.retain_and_reset();
        }
        self.checked_day = day;
        if !self.convergence_checks_failed {
            self.warmup_flag = day < options.minimum_days;
        }
        let maximum_days = options.maximum_days.max(options.minimum_days).max(1);
        self.maximum_day_forced_exit = day >= maximum_days && self.warmup_flag;
        if day >= maximum_days && self.warmup_flag {
            self.warmup_flag = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn begin_environment_previous_zeros_allow_real_first_day_check() {
        let mut zone = HeatBalanceWarmupZoneConvergence::new(ep_model::ZoneId(0));
        assert_eq!(zone.previous, [0.0; 4]);
        assert_eq!(zone.current, [-9999.0, -9999.0, -9999.0, 1000.0]);
        assert!(zone.air_system_sensible_rates_w.is_none());
        zone.record(0.0, 0.0, 0.0);
        assert!(!zone.check(0.0, 0.0));
        assert_eq!(zone.pass_flags, [2; 4]);
    }

    #[test]
    fn all_four_tests_are_per_zone_and_loads_are_normalized() {
        let mut zone = HeatBalanceWarmupZoneConvergence::new(ep_model::ZoneId(7));
        zone.previous = [100.0, 400.0, 20.0, 10.0];
        zone.record(20.0, 200.0, 200.0);
        zone.record(10.0, 0.0, 0.0);
        assert!(zone.check(0.0, 0.04));
        assert_eq!(zone.pass_flags, [2, 2, 1, 1]);
        assert_eq!(zone.test_values, [0.0, 0.0, 0.5, 1.0]);
        zone.retain_and_reset();
        assert_eq!(zone.previous, [200.0, 200.0, 20.0, 10.0]);
        assert_eq!(zone.current, [-9999.0, -9999.0, -9999.0, 1000.0]);
    }

    #[test]
    fn hundred_watt_floor_mutates_then_is_retained() {
        let mut zone = HeatBalanceWarmupZoneConvergence::new(ep_model::ZoneId(0));
        zone.record(0.0, 20.0, 99.0);
        assert!(!zone.check(0.0, 0.0));
        assert_eq!(zone.current[..2], [100.0, 100.0]);
        assert_eq!(zone.previous[..2], [100.0, 100.0]);
        zone.retain_and_reset();
        assert_eq!(zone.previous[..2], [100.0, 100.0]);
    }

    #[test]
    fn small_load_guard_retains_comparison_metric_and_uses_strict_greater_than() {
        let mut zone = HeatBalanceWarmupZoneConvergence::new(ep_model::ZoneId(0));
        zone.test_values[2] = 0.75;
        zone.test_values[3] = 0.25;
        zone.record(0.0, 1.0e-4, 0.0);
        assert!(!zone.check(0.0, 0.0));
        assert_eq!(zone.test_values, [0.0, 0.0, 0.75, 0.25]);
        assert_eq!(zone.current[..2], [1.0e-4, 0.0]);
    }

    #[test]
    fn earlier_zone_failure_does_not_skip_later_zone_check_or_reset() {
        let mut failed = HeatBalanceWarmupZoneConvergence::new(ep_model::ZoneId(1));
        failed.record(20.0, 0.0, 0.0);
        let mut passed = HeatBalanceWarmupZoneConvergence::new(ep_model::ZoneId(9));
        passed.previous = [0.0, 0.0, 12.0, 12.0];
        passed.record(12.0, 0.0, 0.0);
        let mut owner = HeatBalanceWarmupConvergenceState {
            zones: vec![failed, passed],
            checked_day: 0,
            convergence_checks_failed: false,
            warmup_flag: true,
            maximum_day_forced_exit: false,
            unavailable_reason: None,
            report: None,
        };
        owner.check_day(
            1,
            HeatBalanceWarmupOptions {
                enabled: true,
                minimum_days: 1,
                maximum_days: 3,
                temperature_convergence_tolerance_delta_c: 0.0,
                loads_convergence_tolerance_w: 0.0,
            },
        );
        assert!(owner.convergence_checks_failed);
        assert!(owner.warmup_flag);
        assert_eq!(owner.zones[0].pass_flags, [1, 1, 2, 2]);
        assert_eq!(owner.zones[1].pass_flags, [2; 4]);
        assert_eq!(owner.zones[1].previous, [0.0, 0.0, 12.0, 12.0]);
        assert_eq!(owner.zones[1].current, [-9999.0, -9999.0, -9999.0, 1000.0]);
    }

    #[test]
    fn forced_maximum_exit_is_not_convergence() {
        let mut owner = HeatBalanceWarmupConvergenceState {
            zones: vec![HeatBalanceWarmupZoneConvergence::new(ep_model::ZoneId(0))],
            checked_day: 0,
            convergence_checks_failed: false,
            warmup_flag: true,
            maximum_day_forced_exit: false,
            unavailable_reason: None,
            report: None,
        };
        owner.zones[0].record(20.0, 0.0, 0.0);
        owner.check_day(
            1,
            HeatBalanceWarmupOptions {
                enabled: true,
                minimum_days: 1,
                maximum_days: 1,
                temperature_convergence_tolerance_delta_c: 0.0,
                loads_convergence_tolerance_w: 0.0,
            },
        );
        assert!(owner.convergence_checks_failed);
        assert!(owner.maximum_day_forced_exit);
        assert!(!owner.warmup_flag);
        assert_eq!(owner.zones[0].previous, [0.0, 0.0, 20.0, 20.0]);
    }
}

#[cfg(test)]
#[path = "warmup_selected_reference.rs"]
mod selected_reference;

#[cfg(test)]
#[path = "warmup_rolling_tests.rs"]
mod rolling_tests;

#[path = "warmup_report.rs"]
pub(crate) mod report;

#[path = "warmup_ordinary.rs"]
pub(crate) mod ordinary;
