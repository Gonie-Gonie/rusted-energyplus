//! Run-period wrapper for the standalone heat-balance timestep driver.

use super::sample_heat_balance_run_period_with_step_driver;
use crate::error::RuntimeError;
use crate::heat_balance::algorithm::HeatBalanceRuntimeConfig;
use crate::heat_balance::state::{HeatBalanceSimulationOptions, HeatBalanceState};
use crate::heat_balance::timestep::advance_heat_balance_state_one_timestep_internal_with_schedule_cache_profiled;
use crate::heat_balance::trace::HeatBalanceRunPeriodSamples;
use crate::heat_balance::weather_driver::HeatBalanceWeatherDriver;
use crate::schedules::{InternalGainSchedulePhaseOperations, ScheduleSeriesCache};
use crate::weather::EpwRecord;
use ep_model::{FirstHourInterpolationStartingValues, SimulationModel};

pub(crate) fn sample_heat_balance_run_period(
    model: &SimulationModel,
    schedule_cache: &ScheduleSeriesCache,
    schedule_operations: &mut InternalGainSchedulePhaseOperations,
    state: &mut HeatBalanceState,
    weather_dry_bulb_c: &[f64],
    weather_records: Option<&[EpwRecord]>,
    weather_driver: Option<HeatBalanceWeatherDriver<'_>>,
    options: HeatBalanceSimulationOptions,
    runtime_config: HeatBalanceRuntimeConfig,
    zone_steps_per_hour: u32,
    seconds_per_timestep: f64,
    first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
) -> Result<HeatBalanceRunPeriodSamples, RuntimeError> {
    let sampled = sample_heat_balance_run_period_with_step_driver(
        model,
        state,
        weather_dry_bulb_c,
        weather_records,
        weather_driver,
        options,
        runtime_config,
        zone_steps_per_hour,
        seconds_per_timestep,
        first_hour_interpolation_starting_values,
        |state, input, weather_context, _hour_index, _substep| {
            advance_heat_balance_state_one_timestep_internal_with_schedule_cache_profiled(
                &model.typed,
                schedule_cache,
                schedule_operations,
                state,
                input,
                weather_context,
                runtime_config,
                options.surface_iteration_count,
                options.inside_hconv_reevaluation_interval,
                options.surface_loop_zone_air_correction,
            );
            Ok::<(), RuntimeError>(())
        },
    );
    sampled.map(|(samples, _)| samples)
}
