//! Run-period warmup loop and CheckWarmupConvergence helpers.

use crate::heat_balance::algorithm::HeatBalanceRuntimeConfig;
use crate::heat_balance::state::{
    HeatBalanceState, HeatBalanceStepInput, HeatBalanceSurfaceLoopZoneAirCorrection,
    HeatBalanceWarmupDayEndZoneAirStateSample, HeatBalanceWarmupOptions,
};
use crate::heat_balance::summary::HeatBalanceWarmupSummary;
use crate::heat_balance::trace::heat_balance_zone_air_state_sample;
use crate::weather::{
    EpwRecord, HeatBalanceWeatherContext, WeatherTimestepSeries,
    energyplus_weather_dry_bulb_at_timestep_with_starting_values,
    heat_balance_weather_context_for_timestep,
};
use ep_model::{FirstHourInterpolationStartingValues, TypedModel};

pub(crate) fn run_heat_balance_run_period_warmup<F>(
    model: &TypedModel,
    state: &mut HeatBalanceState,
    weather_dry_bulb_c: &[f64],
    weather_records: Option<&[EpwRecord]>,
    weather_series: Option<&WeatherTimestepSeries>,
    zone_steps_per_hour: u32,
    seconds_per_timestep: f64,
    options: HeatBalanceWarmupOptions,
    runtime_config: HeatBalanceRuntimeConfig,
    surface_iteration_count: u32,
    inside_hconv_reevaluation_interval: Option<u32>,
    surface_loop_zone_air_correction: HeatBalanceSurfaceLoopZoneAirCorrection,
    first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
    day_end_zone_air_states: &mut Vec<HeatBalanceWarmupDayEndZoneAirStateSample>,
    mut advance_timestep: F,
) -> HeatBalanceWarmupSummary
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
    ),
{
    if !options.enabled || options.maximum_days == 0 || weather_dry_bulb_c.is_empty() {
        return HeatBalanceWarmupSummary::disabled();
    }

    let hours_per_day = weather_dry_bulb_c.len().min(24);
    let maximum_days = options.maximum_days.max(options.minimum_days).max(1);
    let tolerance = options.temperature_convergence_tolerance_delta_c.max(0.0);
    let timestep_start = state.timestep_index;
    let mut previous_day_extrema: Option<HeatBalanceWarmupDayTemperatureExtrema> = None;
    let mut final_delta = f64::INFINITY;

    for day in 1..=maximum_days {
        let mut day_extrema = HeatBalanceWarmupDayTemperatureExtrema::new(state.zones.len());
        for (hour_index, outdoor_dry_bulb_c) in weather_dry_bulb_c
            .iter()
            .copied()
            .take(hours_per_day)
            .enumerate()
        {
            let hour_ending = u32::try_from(hour_index % 24 + 1).unwrap_or(24);
            let steps = zone_steps_per_hour.max(1);
            for substep in 1..=steps {
                let weather_context = heat_balance_weather_context_for_timestep(
                    weather_series,
                    hour_index,
                    steps,
                    substep,
                    first_hour_interpolation_starting_values,
                );
                let timestep_outdoor_dry_bulb_c = weather_context
                    .and_then(|context| context.sample.map(|sample| sample.dry_bulb_c))
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
                );
                day_extrema.record_state(state);
            }
        }

        let day_end_temperatures = heat_balance_zone_temperature_snapshot(state);
        day_end_zone_air_states.extend(state.zones.iter().map(|zone| {
            HeatBalanceWarmupDayEndZoneAirStateSample {
                day_index: day,
                state: heat_balance_zone_air_state_sample(zone),
            }
        }));
        if let Some(previous_extrema) = &previous_day_extrema {
            final_delta = day_extrema.max_abs_delta(previous_extrema);
            if day >= options.minimum_days && final_delta <= tolerance {
                return HeatBalanceWarmupSummary {
                    enabled: true,
                    minimum_days: options.minimum_days,
                    maximum_days,
                    temperature_convergence_tolerance_delta_c: tolerance,
                    loads_convergence_tolerance_w: options.loads_convergence_tolerance_w,
                    day_count: day,
                    timestep_count: state.timestep_index - timestep_start,
                    hours_per_day,
                    converged: true,
                    final_max_zone_temperature_delta_c: final_delta,
                };
            }
        }
        if day_extrema.is_empty() {
            day_extrema.record_temperatures(&day_end_temperatures);
        }
        previous_day_extrema = Some(day_extrema);
    }

    HeatBalanceWarmupSummary {
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
    }
}

#[derive(Clone, Debug, PartialEq)]
struct HeatBalanceWarmupDayTemperatureExtrema {
    max_temperatures_c: Vec<f64>,
    min_temperatures_c: Vec<f64>,
}

impl HeatBalanceWarmupDayTemperatureExtrema {
    fn new(zone_count: usize) -> Self {
        Self {
            max_temperatures_c: vec![f64::NEG_INFINITY; zone_count],
            min_temperatures_c: vec![f64::INFINITY; zone_count],
        }
    }

    fn record_state(&mut self, state: &HeatBalanceState) {
        let temperatures = state
            .zones
            .iter()
            .map(|zone| {
                if zone.zone_timestep_average_air_temperature_c.is_finite() {
                    zone.zone_timestep_average_air_temperature_c
                } else {
                    zone.mean_air_temperature_c
                }
            })
            .collect::<Vec<_>>();
        self.record_temperatures(&temperatures);
    }

    fn record_temperatures(&mut self, temperatures_c: &[f64]) {
        for (index, temperature_c) in temperatures_c.iter().copied().enumerate() {
            if let Some(max_temperature_c) = self.max_temperatures_c.get_mut(index) {
                *max_temperature_c = max_temperature_c.max(temperature_c);
            }
            if let Some(min_temperature_c) = self.min_temperatures_c.get_mut(index) {
                *min_temperature_c = min_temperature_c.min(temperature_c);
            }
        }
    }

    fn is_empty(&self) -> bool {
        self.max_temperatures_c
            .iter()
            .all(|temperature_c| !temperature_c.is_finite())
            && self
                .min_temperatures_c
                .iter()
                .all(|temperature_c| !temperature_c.is_finite())
    }

    fn max_abs_delta(&self, previous: &Self) -> f64 {
        max_abs_pair_delta(
            self.max_temperatures_c.as_slice(),
            previous.max_temperatures_c.as_slice(),
        )
        .max(max_abs_pair_delta(
            self.min_temperatures_c.as_slice(),
            previous.min_temperatures_c.as_slice(),
        ))
    }
}

fn heat_balance_zone_temperature_snapshot(state: &HeatBalanceState) -> Vec<f64> {
    state
        .zones
        .iter()
        .map(|zone| zone.mean_air_temperature_c)
        .collect()
}

fn max_abs_pair_delta(left: &[f64], right: &[f64]) -> f64 {
    left.iter()
        .zip(right.iter())
        .map(|(left, right)| (left - right).abs())
        .fold(0.0, f64::max)
}
