//! Direct Zone PurchasedAir insertion at the source-order predictor boundary.

use super::advance_heat_balance_state_one_timestep_source_order_path;
use crate::heat_balance::algorithm::HeatBalanceRuntimeConfig;
use crate::heat_balance::manager;
use crate::heat_balance::state::{
    HeatBalanceState, HeatBalanceStepInput, HeatBalanceSurfaceLoopZoneAirCorrection,
};
use crate::heat_balance::zone_air_correction::correct_zone_air_temperatures_from_current_surfaces;
use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirRuntimeStepError,
    DirectZonePurchasedAirScheduledCouplingInput, DirectZonePurchasedAirScheduledCouplingOutput,
    PurchasedAirRuntimeState, couple_model_bound_direct_zone_purchased_air,
};
use crate::psychrometrics::production_trace::system_call as trace_system_call;
use crate::schedules::{InternalGainSchedulePhaseOperations, ScheduleSeriesCache};
use crate::weather::{
    HeatBalanceWeatherContext, energyplus_weather_atmospheric_pressure_for_context,
};
use ep_model::TypedModel;

/// Advances one fixed ThirdOrder timestep with CP301 inserted inside
/// `PredictSystemLoads` before the existing surface/corrector tail.
#[allow(clippy::too_many_arguments)]
pub(crate) fn advance_heat_balance_state_one_timestep_with_direct_zone_purchased_air(
    model: &TypedModel,
    internal_gain_schedule_cache: &ScheduleSeriesCache,
    internal_gain_schedule_operations: &mut InternalGainSchedulePhaseOperations,
    state: &mut HeatBalanceState,
    input: HeatBalanceStepInput,
    weather_context: Option<HeatBalanceWeatherContext<'_>>,
    runtime_config: HeatBalanceRuntimeConfig,
    surface_iteration_count: u32,
    inside_hconv_reevaluation_interval: Option<u32>,
    surface_loop_zone_air_correction: HeatBalanceSurfaceLoopZoneAirCorrection,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
    purchased_air_runtime_state: &mut PurchasedAirRuntimeState,
    begin_environment: bool,
    coupling_schedule_cache: &ScheduleSeriesCache,
    coupling_schedule_sample_index: usize,
) -> Result<DirectZonePurchasedAirScheduledCouplingOutput, DirectZonePurchasedAirRuntimeStepError> {
    manager::manage_heat_balance_source_order_path(|| {
        advance_heat_balance_state_one_timestep_source_order_path(
            model,
            Some(internal_gain_schedule_cache),
            Some(internal_gain_schedule_operations),
            state,
            input,
            weather_context,
            runtime_config,
            surface_iteration_count,
            inside_hconv_reevaluation_interval,
            surface_loop_zone_air_correction,
            |state| {
                // CP299 consumes current non-system predictor terms. Materialize
                // them after history/internal-gain preparation, without
                // correcting MAT, before CP301 writes the system-air terms.
                correct_zone_air_temperatures_from_current_surfaces(
                    &state.surfaces,
                    &state.surface_indexes,
                    &mut state.zones,
                    input.timestep_seconds,
                    weather_context,
                    input.outdoor_dry_bulb_c,
                    false,
                    true,
                    runtime_config.use_inside_ctf_outside_temperature_for_conduction_report,
                );
                let zone_state = state
                    .zones
                    .iter_mut()
                    .find(|zone| zone.zone_id == binding.zone)
                    .ok_or(
                        DirectZonePurchasedAirRuntimeStepError::MissingBoundZoneState {
                            zone: binding.zone,
                        },
                    )?;
                let limit_context = weather_context
                    .and_then(|context| {
                        context.current_record().map(|record| {
                            binding.limit_context.with_barometric_pressure_pa(
                                energyplus_weather_atmospheric_pressure_for_context(
                                    context,
                                    record.atmospheric_pressure_pa,
                                ),
                            )
                        })
                    })
                    .unwrap_or(binding.limit_context);
                let _trace_scope = trace_system_call(
                    coupling_schedule_sample_index,
                    begin_environment,
                    input.timestep_seconds,
                );
                couple_model_bound_direct_zone_purchased_air(
                    DirectZonePurchasedAirScheduledCouplingInput {
                        binding,
                        schedule_cache: coupling_schedule_cache,
                        schedule_sample_index: coupling_schedule_sample_index,
                        zone_state,
                        purchased_air_runtime_state,
                        begin_environment,
                        barometric_pressure_pa: limit_context.barometric_pressure_pa,
                        system_timestep_seconds: input.timestep_seconds,
                    },
                )
                .map_err(DirectZonePurchasedAirRuntimeStepError::ScheduledCoupling)
            },
        )
    })
}
