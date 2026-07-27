//! Heat-balance timestep advance and source-order state update.

use crate::diagnostic_probes::HeatBalanceZoneAirAlgorithm;
use crate::heat_balance::algorithm::{
    HeatBalanceInteriorLongwaveMode, HeatBalanceRuntimeConfig, HeatBalanceZoneAirUpdate,
};
use crate::heat_balance::convection::heat_balance_uses_doe2_outside_convection;
use crate::heat_balance::ctf::{
    advance_surface_ctf_histories, advance_surface_ctf_histories_with_outside_temperature_override,
    heat_balance_ctf_history_slot_samples,
};
use crate::heat_balance::inside_convection::zone_surface_convection_sums_for_indices;
use crate::heat_balance::radiation::InteriorLongwaveExchangeProbe;
use crate::heat_balance::state::{
    HeatBalanceState, HeatBalanceStepInput, HeatBalanceSurfaceLoopZoneAirCorrection,
};
use crate::heat_balance::surface_balance::heat_balance_surface_boundary_balance;
use crate::heat_balance::surface_boundary::{
    inside_ctf_outside_temperature_history_commit_override_c,
    sync_adiabatic_outside_faces_to_inside_faces,
};
use crate::heat_balance::surface_loop::{
    run_interleaved_surface_zone_balance, run_surface_balance_passes,
};
use crate::heat_balance::zone_air_correction::{
    apply_energyplus_adaptive_system_timestep_zone_air_correction,
    correct_zone_air_humidity_ratios_from_current_state,
    correct_zone_air_temperatures_from_current_surfaces,
    synchronize_single_system_timestep_history,
};
use crate::heat_balance::zone_predictor_corrector::{
    energyplus_analytical_zone_air_temperature_c,
    energyplus_third_order_zone_air_temperature_from_coefficients,
    energyplus_zone_air_temperature_coefficients, step_zone_air_temperature,
};
use crate::heat_balance::{air_manager, manager, surface_manager, zone_predictor_corrector};
use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirRuntimeStepError,
    DirectZonePurchasedAirScheduledCouplingInput, DirectZonePurchasedAirScheduledCouplingOutput,
    couple_model_bound_direct_zone_purchased_air,
};
use crate::schedules::{
    InternalGainSchedulePhaseOperations, ScheduleSeriesCache, convective_internal_gain_w,
    convective_internal_gain_w_from_cache, convective_internal_gain_w_from_cache_profiled,
    convective_internal_gain_w_live_profiled, update_surface_radiant_internal_gain_source_terms,
    update_surface_radiant_internal_gain_source_terms_from_cache,
    update_surface_radiant_internal_gain_source_terms_from_cache_profiled,
    update_surface_radiant_internal_gain_source_terms_live_profiled,
};
use crate::weather::HeatBalanceWeatherContext;
use ep_model::{OutsideBoundaryCondition, TypedModel};
use std::collections::BTreeMap;
use std::convert::Infallible;
/// Advances the heat-balance state by one timestep without making a
/// conformance claim.
///
/// This is the first zone-air predictor/corrector-shaped state update. It uses
/// the currently supported opaque surface conductance and internal convective
/// gains while keeping the public zone-temperature comparison diagnostic-only.
pub fn advance_heat_balance_state_one_timestep(
    model: &TypedModel,
    state: &mut HeatBalanceState,
    input: HeatBalanceStepInput,
) {
    advance_heat_balance_state_one_timestep_internal(
        model,
        state,
        input,
        None,
        HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical.runtime_config(),
        1,
        None,
        HeatBalanceSurfaceLoopZoneAirCorrection::EachSurfaceIteration,
    );
}

pub(crate) fn advance_heat_balance_state_one_timestep_internal(
    model: &TypedModel,
    state: &mut HeatBalanceState,
    input: HeatBalanceStepInput,
    weather_context: Option<HeatBalanceWeatherContext<'_>>,
    runtime_config: HeatBalanceRuntimeConfig,
    surface_iteration_count: u32,
    inside_hconv_reevaluation_interval: Option<u32>,
    surface_loop_zone_air_correction: HeatBalanceSurfaceLoopZoneAirCorrection,
) {
    advance_heat_balance_state_one_timestep_internal_with_optional_schedule_cache(
        model,
        None,
        None,
        state,
        input,
        weather_context,
        runtime_config,
        surface_iteration_count,
        inside_hconv_reevaluation_interval,
        surface_loop_zone_air_correction,
    );
}

#[allow(clippy::too_many_arguments)]
#[cfg(test)]
pub(crate) fn advance_heat_balance_state_one_timestep_internal_with_schedule_cache(
    model: &TypedModel,
    schedule_cache: &ScheduleSeriesCache,
    state: &mut HeatBalanceState,
    input: HeatBalanceStepInput,
    weather_context: Option<HeatBalanceWeatherContext<'_>>,
    runtime_config: HeatBalanceRuntimeConfig,
    surface_iteration_count: u32,
    inside_hconv_reevaluation_interval: Option<u32>,
    surface_loop_zone_air_correction: HeatBalanceSurfaceLoopZoneAirCorrection,
) {
    advance_heat_balance_state_one_timestep_internal_with_optional_schedule_cache(
        model,
        Some(schedule_cache),
        None,
        state,
        input,
        weather_context,
        runtime_config,
        surface_iteration_count,
        inside_hconv_reevaluation_interval,
        surface_loop_zone_air_correction,
    );
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn advance_heat_balance_state_one_timestep_internal_with_schedule_cache_profiled(
    model: &TypedModel,
    schedule_cache: &ScheduleSeriesCache,
    operations: &mut InternalGainSchedulePhaseOperations,
    state: &mut HeatBalanceState,
    input: HeatBalanceStepInput,
    weather_context: Option<HeatBalanceWeatherContext<'_>>,
    runtime_config: HeatBalanceRuntimeConfig,
    surface_iteration_count: u32,
    inside_hconv_reevaluation_interval: Option<u32>,
    surface_loop_zone_air_correction: HeatBalanceSurfaceLoopZoneAirCorrection,
) {
    advance_heat_balance_state_one_timestep_internal_with_optional_schedule_cache(
        model,
        Some(schedule_cache),
        Some(operations),
        state,
        input,
        weather_context,
        runtime_config,
        surface_iteration_count,
        inside_hconv_reevaluation_interval,
        surface_loop_zone_air_correction,
    );
}

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
                couple_model_bound_direct_zone_purchased_air(
                    DirectZonePurchasedAirScheduledCouplingInput {
                        binding,
                        schedule_cache: coupling_schedule_cache,
                        schedule_sample_index: coupling_schedule_sample_index,
                        zone_state,
                        system_timestep_seconds: input.timestep_seconds,
                    },
                )
                .map_err(DirectZonePurchasedAirRuntimeStepError::ScheduledCoupling)
            },
        )
    })
}

#[cfg(test)]
#[allow(clippy::too_many_arguments)]
pub(crate) fn advance_heat_balance_state_one_timestep_internal_with_live_schedule_profiled(
    model: &TypedModel,
    operations: &mut InternalGainSchedulePhaseOperations,
    state: &mut HeatBalanceState,
    input: HeatBalanceStepInput,
    weather_context: Option<HeatBalanceWeatherContext<'_>>,
    runtime_config: HeatBalanceRuntimeConfig,
    surface_iteration_count: u32,
    inside_hconv_reevaluation_interval: Option<u32>,
    surface_loop_zone_air_correction: HeatBalanceSurfaceLoopZoneAirCorrection,
) {
    advance_heat_balance_state_one_timestep_internal_with_optional_schedule_cache(
        model,
        None,
        Some(operations),
        state,
        input,
        weather_context,
        runtime_config,
        surface_iteration_count,
        inside_hconv_reevaluation_interval,
        surface_loop_zone_air_correction,
    );
}

#[allow(clippy::too_many_arguments)]
fn advance_heat_balance_state_one_timestep_internal_with_optional_schedule_cache(
    model: &TypedModel,
    schedule_cache: Option<&ScheduleSeriesCache>,
    schedule_operations: Option<&mut InternalGainSchedulePhaseOperations>,
    state: &mut HeatBalanceState,
    input: HeatBalanceStepInput,
    weather_context: Option<HeatBalanceWeatherContext<'_>>,
    runtime_config: HeatBalanceRuntimeConfig,
    surface_iteration_count: u32,
    inside_hconv_reevaluation_interval: Option<u32>,
    surface_loop_zone_air_correction: HeatBalanceSurfaceLoopZoneAirCorrection,
) {
    let result = manager::manage_heat_balance_source_order_path(|| {
        advance_heat_balance_state_one_timestep_source_order_path(
            model,
            schedule_cache,
            schedule_operations,
            state,
            input,
            weather_context,
            runtime_config,
            surface_iteration_count,
            inside_hconv_reevaluation_interval,
            surface_loop_zone_air_correction,
            |_state| Ok::<(), Infallible>(()),
        )
    });
    match result {
        Ok(()) => {}
        Err(error) => match error {},
    }
}

fn advance_heat_balance_state_one_timestep_source_order_path<PredictorHook, Output, Error>(
    model: &TypedModel,
    schedule_cache: Option<&ScheduleSeriesCache>,
    mut schedule_operations: Option<&mut InternalGainSchedulePhaseOperations>,
    state: &mut HeatBalanceState,
    input: HeatBalanceStepInput,
    weather_context: Option<HeatBalanceWeatherContext<'_>>,
    runtime_config: HeatBalanceRuntimeConfig,
    surface_iteration_count: u32,
    inside_hconv_reevaluation_interval: Option<u32>,
    surface_loop_zone_air_correction: HeatBalanceSurfaceLoopZoneAirCorrection,
    predictor_hook: PredictorHook,
) -> Result<Output, Error>
where
    PredictorHook: FnOnce(&mut HeatBalanceState) -> Result<Output, Error>,
{
    let hour_ending = input.hour_ending.clamp(1, 24);
    let previous_zone_temperatures = state
        .zones
        .iter()
        .map(|zone| (zone.zone_id, zone.mean_air_temperature_c))
        .collect::<BTreeMap<_, _>>();
    let previous_surface_inside_temperatures = state
        .surfaces
        .iter()
        .map(|surface| (surface.surface_id, surface.inside_face_temperature_c))
        .collect::<BTreeMap<_, _>>();
    let previous_surface_outside_temperatures = state
        .surfaces
        .iter()
        .map(|surface| (surface.surface_id, surface.outside_face_temperature_c))
        .collect::<BTreeMap<_, _>>();
    let use_energyplus_adaptive_system_timestep_zone_air_correction =
        runtime_config.use_energyplus_adaptive_system_timestep_zone_air_correction;
    let use_doe2_outside_convection =
        heat_balance_uses_doe2_outside_convection(model, runtime_config);
    let sync_adiabatic_outside_to_current_inside_before_history =
        runtime_config.sync_adiabatic_outside_to_current_inside_before_history;
    let sync_adiabatic_outside_to_current_inside_for_report_only =
        runtime_config.sync_adiabatic_outside_to_current_inside_for_report_only;
    let commit_adiabatic_current_inside_to_history_only =
        runtime_config.commit_adiabatic_current_inside_to_history_only;
    let interior_longwave_exchange_probe = match runtime_config.interior_longwave_mode {
        HeatBalanceInteriorLongwaveMode::None => InteriorLongwaveExchangeProbe::None,
        HeatBalanceInteriorLongwaveMode::GreyEnergyPlusDirectViewFactor => {
            InteriorLongwaveExchangeProbe::GreyEnergyPlusDirectViewFactor
        }
        HeatBalanceInteriorLongwaveMode::EnergyPlusScriptF => {
            InteriorLongwaveExchangeProbe::EnergyPlusScriptF
        }
        HeatBalanceInteriorLongwaveMode::EnergyPlusScriptFFlatAccess => {
            InteriorLongwaveExchangeProbe::EnergyPlusScriptFFlatAccess
        }
    };

    let preserve_surface_inside_temperature_for_first_longwave =
        runtime_config.preserve_surface_inside_temperature_for_first_longwave;

    surface_manager::manage_surface_heat_balance_source_order_path(|| {
        surface_manager::init_surface_heat_balance_source_order_path(|| {
            surface_manager::calc_heat_balance_outside_surf_source_order_path(|| {
                for surface in &mut state.surfaces {
                    let zone_temperature_c = previous_zone_temperatures
                        .get(&surface.zone_id)
                        .copied()
                        .unwrap_or(surface.inside_face_temperature_c);

                    let initial_inside_face_temperature_c =
                        if preserve_surface_inside_temperature_for_first_longwave {
                            previous_surface_inside_temperatures
                                .get(&surface.surface_id)
                                .copied()
                                .unwrap_or(zone_temperature_c)
                        } else {
                            zone_temperature_c
                        };
                    surface.inside_face_temperature_c = initial_inside_face_temperature_c;
                    let boundary_balance = heat_balance_surface_boundary_balance(
                        model,
                        surface,
                        &previous_zone_temperatures,
                        input.outdoor_dry_bulb_c,
                        initial_inside_face_temperature_c,
                        weather_context,
                        None,
                        use_doe2_outside_convection,
                    );
                    surface.outside_face_temperature_c = boundary_balance.temperature_c;
                    surface.outside_report_terms = boundary_balance.exterior_report_terms;
                    surface.outside_balance_diagnostics =
                        boundary_balance.outside_balance_diagnostics;
                }
            });
        });
    });
    let predictor_output = air_manager::manage_air_heat_balance_compat(|| {
        air_manager::init_air_heat_balance_compat(|| {});
        air_manager::calc_heat_balance_air_compat(|| {
            zone_predictor_corrector::manage_zone_air_updates_compat(
                zone_predictor_corrector::PredictorCorrectorCtrl::PredictStep,
                || {
                    zone_predictor_corrector::get_zone_air_set_points_compat(|| {
                        zone_predictor_corrector::init_zone_air_set_points_compat(|| {
                            zone_predictor_corrector::calc_zone_air_temp_set_points_compat(|| {})
                        })
                    });
                    zone_predictor_corrector::push_zone_timestep_histories_compat(|| {
                        zone_predictor_corrector::predict_system_loads_compat(|| {
                            for zone in &mut state.zones {
                                zone.use_zone_timestep_history =
                                    use_energyplus_adaptive_system_timestep_zone_air_correction;
                                zone.shorten_timestep_sys = false;
                                zone.prior_timestep_seconds = input.timestep_seconds;
                                let previous_temperature_c = zone.mean_air_temperature_c;
                                let previous_zone_history_temperature_c =
                                    if use_energyplus_adaptive_system_timestep_zone_air_correction {
                                        zone.zone_timestep_average_air_temperature_c
                                    } else {
                                        previous_temperature_c
                                    };
                                zone.previous_mean_air_temperatures_c = [
                                    previous_zone_history_temperature_c,
                                    zone.previous_mean_air_temperatures_c[0],
                                    zone.previous_mean_air_temperatures_c[1],
                                ];
                                let previous_humidity_ratio = zone.air_humidity_ratio;
                                let previous_zone_history_humidity_ratio =
                                    if use_energyplus_adaptive_system_timestep_zone_air_correction {
                                        zone.zone_timestep_average_air_humidity_ratio
                                    } else {
                                        previous_humidity_ratio
                                    };
                                zone.previous_air_humidity_ratios = [
                                    previous_zone_history_humidity_ratio,
                                    zone.previous_air_humidity_ratios[0],
                                    zone.previous_air_humidity_ratios[1],
                                ];
                                zone.convective_internal_gain_w =
                                    match (schedule_cache, schedule_operations.as_deref_mut()) {
                                        (Some(schedule_cache), Some(operations)) => {
                                            convective_internal_gain_w_from_cache_profiled(
                                                model,
                                                schedule_cache,
                                                zone.zone_id,
                                                hour_ending,
                                                operations,
                                            )
                                        }
                                        (Some(schedule_cache), None) => {
                                            convective_internal_gain_w_from_cache(
                                                model,
                                                schedule_cache,
                                                zone.zone_id,
                                                hour_ending,
                                            )
                                        }
                                        (None, Some(operations)) => {
                                            convective_internal_gain_w_live_profiled(
                                                model,
                                                zone.zone_id,
                                                hour_ending,
                                                operations,
                                            )
                                        }
                                        (None, None) => convective_internal_gain_w(
                                            model,
                                            zone.zone_id,
                                            hour_ending,
                                        ),
                                    };

                                let zone_surface_indexes =
                                    state.surface_indexes.surfaces_for_zone(zone.zone_id);
                                let conductance_w_per_k = zone_surface_indexes
                                    .iter()
                                    .filter_map(|surface_index| state.surfaces.get(*surface_index))
                                    .map(|surface| surface.conductance_w_per_k)
                                    .sum::<f64>();
                                let conductance_weighted_outside_temperature = zone_surface_indexes
                                    .iter()
                                    .filter_map(|surface_index| state.surfaces.get(*surface_index))
                                    .map(|surface| {
                                        surface.conductance_w_per_k
                                            * surface.outside_face_temperature_c
                                    })
                                    .sum::<f64>();
                                let equivalent_outside_temperature_c = if conductance_w_per_k > 0.0
                                {
                                    conductance_weighted_outside_temperature / conductance_w_per_k
                                } else {
                                    previous_temperature_c
                                };

                                zone.opaque_surface_conductance_w_per_k = conductance_w_per_k;
                                zone.mean_air_temperature_c = match runtime_config.zone_air_update {
                                    HeatBalanceZoneAirUpdate::SimplifiedAnalytical => {
                                        step_zone_air_temperature(
                                            previous_temperature_c,
                                            equivalent_outside_temperature_c,
                                            zone.convective_internal_gain_w,
                                            conductance_w_per_k,
                                            zone.air_heat_capacity_j_per_k,
                                            input.timestep_seconds,
                                        )
                                    }
                                    HeatBalanceZoneAirUpdate::Deferred => previous_temperature_c,
                                    HeatBalanceZoneAirUpdate::EnergyPlusAnalytical => {
                                        let (sum_ha_w_per_k, sum_hat_surf_w, sum_hat_ref_w) =
                                            zone_surface_convection_sums_for_indices(
                                                &state.surfaces,
                                                state
                                                    .surface_indexes
                                                    .surfaces_for_zone(zone.zone_id),
                                            );
                                        let coefficients =
                                            energyplus_zone_air_temperature_coefficients(
                                                sum_ha_w_per_k,
                                                sum_hat_surf_w,
                                                sum_hat_ref_w,
                                                zone.convective_internal_gain_w,
                                                zone.sum_mcp_w_per_k + zone.sum_sys_mcp_w_per_k,
                                                zone.sum_mcp_t_w + zone.sum_sys_mcp_t_w,
                                                zone.air_heat_capacity_j_per_k,
                                                input.timestep_seconds,
                                                zone.previous_mean_air_temperatures_c,
                                            );
                                        energyplus_analytical_zone_air_temperature_c(
                                            previous_temperature_c,
                                            coefficients.temp_independent_coefficient_w,
                                            coefficients.temp_dependent_coefficient_w_per_k,
                                            zone.air_heat_capacity_j_per_k,
                                            input.timestep_seconds,
                                        )
                                    }
                                    HeatBalanceZoneAirUpdate::EnergyPlusThirdOrder => {
                                        let (sum_ha_w_per_k, sum_hat_surf_w, sum_hat_ref_w) =
                                            zone_surface_convection_sums_for_indices(
                                                &state.surfaces,
                                                state
                                                    .surface_indexes
                                                    .surfaces_for_zone(zone.zone_id),
                                            );
                                        let coefficients =
                                            energyplus_zone_air_temperature_coefficients(
                                                sum_ha_w_per_k,
                                                sum_hat_surf_w,
                                                sum_hat_ref_w,
                                                zone.convective_internal_gain_w,
                                                zone.sum_mcp_w_per_k + zone.sum_sys_mcp_w_per_k,
                                                zone.sum_mcp_t_w + zone.sum_sys_mcp_t_w,
                                                zone.air_heat_capacity_j_per_k,
                                                input.timestep_seconds,
                                                zone.previous_mean_air_temperatures_c,
                                            );
                                        energyplus_third_order_zone_air_temperature_from_coefficients(
                    previous_temperature_c,
                    coefficients,
                )
                                    }
                                };
                            }
                            predictor_hook(state)
                        })
                    })
                },
            )
        })
    })?;
    match (schedule_cache, schedule_operations) {
        (Some(schedule_cache), Some(operations)) => {
            update_surface_radiant_internal_gain_source_terms_from_cache_profiled(
                model,
                schedule_cache,
                &mut state.surfaces,
                hour_ending,
                operations,
            );
        }
        (Some(schedule_cache), None) => {
            update_surface_radiant_internal_gain_source_terms_from_cache(
                model,
                schedule_cache,
                &mut state.surfaces,
                hour_ending,
            );
        }
        (None, Some(operations)) => {
            update_surface_radiant_internal_gain_source_terms_live_profiled(
                model,
                &mut state.surfaces,
                hour_ending,
                operations,
            );
        }
        (None, None) => {
            update_surface_radiant_internal_gain_source_terms(
                model,
                &mut state.surfaces,
                hour_ending,
            );
        }
    }
    let use_current_inside_for_first_longwave =
        runtime_config.use_current_inside_for_first_longwave;
    let converge_interleaved_surface_iterations_to_energyplus_tolerance =
        runtime_config.converge_interleaved_surface_iterations_to_energyplus_tolerance;
    let freeze_outside_balance_for_surface_iterations =
        runtime_config.freeze_outside_balance_for_surface_iterations;
    let freeze_inside_ctf_outside_temperature_for_surface_iterations =
        runtime_config.freeze_inside_ctf_outside_temperature_for_surface_iterations;
    let use_inside_ctf_outside_temperature_for_conduction_report =
        runtime_config.use_inside_ctf_outside_temperature_for_conduction_report;
    let commit_inside_ctf_outside_temperature_to_history =
        runtime_config.commit_inside_ctf_outside_temperature_to_history;

    let interleaved_surface_zone_balance_result =
        surface_manager::calc_heat_balance_inside_surf_source_order_path(|| {
            if runtime_config.timestep.interleave_zone_air_surface_passes {
                Some(run_interleaved_surface_zone_balance(
                    model,
                    &mut state.surfaces,
                    &state.surface_indexes,
                    &mut state.zones,
                    Some(&previous_surface_inside_temperatures),
                    input,
                    weather_context,
                    surface_iteration_count,
                    runtime_config
                        .timestep
                        .use_previous_inside_for_outdoor_boundary,
                    runtime_config
                        .timestep
                        .use_previous_inside_for_adiabatic_boundary,
                    runtime_config.timestep.use_quick_outside_conduction,
                    Some(&previous_surface_outside_temperatures),
                    use_doe2_outside_convection,
                    interior_longwave_exchange_probe,
                    runtime_config.freeze_inside_convection_coefficients,
                    use_current_inside_for_first_longwave,
                    runtime_config.use_third_order_zone_air_correction,
                    runtime_config.freeze_surface_reference_air,
                    converge_interleaved_surface_iterations_to_energyplus_tolerance,
                    freeze_outside_balance_for_surface_iterations,
                    freeze_inside_ctf_outside_temperature_for_surface_iterations,
                    use_inside_ctf_outside_temperature_for_conduction_report,
                    inside_hconv_reevaluation_interval,
                    surface_loop_zone_air_correction,
                ))
            } else {
                let current_zone_temperatures = state
                    .zones
                    .iter()
                    .map(|zone| (zone.zone_id, zone.mean_air_temperature_c))
                    .collect::<BTreeMap<_, _>>();
                run_surface_balance_passes(
                    model,
                    &mut state.surfaces,
                    &state.surface_indexes,
                    Some(&previous_surface_inside_temperatures),
                    Some(&previous_surface_inside_temperatures),
                    None,
                    &current_zone_temperatures,
                    input,
                    weather_context,
                    surface_iteration_count,
                    runtime_config
                        .timestep
                        .use_previous_inside_for_outdoor_boundary,
                    runtime_config
                        .timestep
                        .use_previous_inside_for_adiabatic_boundary,
                    runtime_config.timestep.use_quick_outside_conduction,
                    if runtime_config.timestep.use_quick_outside_conduction {
                        Some(&previous_surface_outside_temperatures)
                    } else {
                        None
                    },
                    use_doe2_outside_convection,
                    interior_longwave_exchange_probe,
                    None,
                    None,
                    None,
                    None,
                    false,
                );

                if runtime_config
                    .timestep
                    .rebalance_surfaces_after_zone_air_correction
                {
                    zone_predictor_corrector::correct_step_source_order_path(|| {
                        correct_zone_air_temperatures_from_current_surfaces(
                            &state.surfaces,
                            &state.surface_indexes,
                            &mut state.zones,
                            input.timestep_seconds,
                            weather_context,
                            input.outdoor_dry_bulb_c,
                            true,
                            runtime_config.use_third_order_zone_air_correction,
                            false,
                        );
                    });
                    let corrected_zone_temperatures = state
                        .zones
                        .iter()
                        .map(|zone| (zone.zone_id, zone.mean_air_temperature_c))
                        .collect::<BTreeMap<_, _>>();
                    run_surface_balance_passes(
                        model,
                        &mut state.surfaces,
                        &state.surface_indexes,
                        None,
                        None,
                        None,
                        &corrected_zone_temperatures,
                        input,
                        weather_context,
                        surface_iteration_count,
                        runtime_config
                            .timestep
                            .use_previous_inside_for_outdoor_boundary,
                        runtime_config
                            .timestep
                            .use_previous_inside_for_adiabatic_boundary,
                        runtime_config.timestep.use_quick_outside_conduction,
                        if runtime_config.timestep.use_quick_outside_conduction {
                            Some(&previous_surface_outside_temperatures)
                        } else {
                            None
                        },
                        use_doe2_outside_convection,
                        interior_longwave_exchange_probe,
                        None,
                        None,
                        None,
                        None,
                        false,
                    );
                }
                None
            }
        });

    if runtime_config.timestep.interleave_zone_air_surface_passes
        && matches!(
            surface_loop_zone_air_correction,
            HeatBalanceSurfaceLoopZoneAirCorrection::AfterSurfaceLoop
        )
    {
        zone_predictor_corrector::correct_step_source_order_path(|| {
            correct_zone_air_temperatures_from_current_surfaces(
                &state.surfaces,
                &state.surface_indexes,
                &mut state.zones,
                input.timestep_seconds,
                weather_context,
                input.outdoor_dry_bulb_c,
                true,
                runtime_config.use_third_order_zone_air_correction,
                use_inside_ctf_outside_temperature_for_conduction_report,
            );
        });
    }

    let adiabatic_report_history_outside_temperature_snapshots =
        surface_manager::update_final_surface_heat_balance_source_order_path(|| {
            if sync_adiabatic_outside_to_current_inside_before_history {
                sync_adiabatic_outside_faces_to_inside_faces(&mut state.surfaces);
            }
            if sync_adiabatic_outside_to_current_inside_for_report_only {
                let snapshots = state
                    .surfaces
                    .iter()
                    .filter(|surface| {
                        surface.outside_boundary_condition == OutsideBoundaryCondition::Adiabatic
                    })
                    .map(|surface| (surface.surface_id, surface.outside_face_temperature_c))
                    .collect::<BTreeMap<_, _>>();
                sync_adiabatic_outside_faces_to_inside_faces(&mut state.surfaces);
                Some(snapshots)
            } else {
                None
            }
        });

    surface_manager::update_thermal_histories_source_order_path(|| {
        state.last_ctf_history_slot_terms = heat_balance_ctf_history_slot_samples(&state.surfaces);
        let inside_ctf_outside_temperature_history_commit_snapshots =
            interleaved_surface_zone_balance_result
                .as_ref()
                .and_then(|result| result.inside_ctf_outside_temperature_snapshots.as_ref());
        for surface in &mut state.surfaces {
            if commit_adiabatic_current_inside_to_history_only
                && surface.outside_boundary_condition == OutsideBoundaryCondition::Adiabatic
            {
                advance_surface_ctf_histories_with_outside_temperature_override(
                    surface,
                    Some(surface.inside_face_temperature_c),
                );
            } else if let Some(outside_temperature_c) =
                adiabatic_report_history_outside_temperature_snapshots
                    .as_ref()
                    .and_then(|snapshots| snapshots.get(&surface.surface_id).copied())
            {
                advance_surface_ctf_histories_with_outside_temperature_override(
                    surface,
                    Some(outside_temperature_c),
                );
            } else if let Some(outside_temperature_c) =
                inside_ctf_outside_temperature_history_commit_override_c(
                    surface,
                    commit_inside_ctf_outside_temperature_to_history,
                    inside_ctf_outside_temperature_history_commit_snapshots,
                )
            {
                advance_surface_ctf_histories_with_outside_temperature_override(
                    surface,
                    Some(outside_temperature_c),
                );
            } else {
                advance_surface_ctf_histories(surface);
            }
        }
        state.last_ctf_history_slot_terms_after_advance =
            heat_balance_ctf_history_slot_samples(&state.surfaces);
    });

    zone_predictor_corrector::manage_zone_air_updates_compat(
        zone_predictor_corrector::PredictorCorrectorCtrl::CorrectStep,
        || {
            zone_predictor_corrector::correct_zone_air_temps_compat(|| {
                correct_zone_air_temperatures_from_current_surfaces(
                    &state.surfaces,
                    &state.surface_indexes,
                    &mut state.zones,
                    input.timestep_seconds,
                    weather_context,
                    input.outdoor_dry_bulb_c,
                    runtime_config.timestep.correct_zone_air_after_surface_pass
                        && !runtime_config.timestep.interleave_zone_air_surface_passes,
                    runtime_config.use_third_order_zone_air_correction,
                    use_inside_ctf_outside_temperature_for_conduction_report,
                );
                correct_zone_air_humidity_ratios_from_current_state(
                    &mut state.zones,
                    input.timestep_seconds,
                    weather_context,
                    runtime_config.use_third_order_zone_air_correction,
                );
                if use_energyplus_adaptive_system_timestep_zone_air_correction {
                    apply_energyplus_adaptive_system_timestep_zone_air_correction(
                        &state.surfaces,
                        &state.surface_indexes,
                        &mut state.zones,
                        input.timestep_seconds,
                        weather_context,
                        input.outdoor_dry_bulb_c,
                        use_inside_ctf_outside_temperature_for_conduction_report,
                    );
                } else {
                    for zone in &mut state.zones {
                        zone.zone_timestep_average_air_temperature_c = zone.mean_air_temperature_c;
                        zone.zone_timestep_average_air_humidity_ratio = zone.air_humidity_ratio;
                        zone.shorten_timestep_sys = false;
                        zone.prior_timestep_seconds = input.timestep_seconds;
                        zone_predictor_corrector::push_system_timestep_histories_compat(|| {
                            synchronize_single_system_timestep_history(zone);
                        });
                        zone.system_timestep_average_surface_convection_report_w = None;
                        zone.system_timestep_average_air_storage_report_w = None;
                    }
                }
            });
        },
    );
    state.last_inside_surface_iteration_count = interleaved_surface_zone_balance_result
        .as_ref()
        .map(|result| result.inside_surface_iteration_count)
        .unwrap_or_else(|| surface_iteration_count.max(1));
    state.last_inside_surface_iteration_max_delta_c = interleaved_surface_zone_balance_result
        .as_ref()
        .map(|result| result.max_inside_surface_delta_c)
        .unwrap_or(f64::NAN);
    state.last_inside_surface_iteration_max_delta_surface_name =
        interleaved_surface_zone_balance_result
            .as_ref()
            .and_then(|result| result.max_delta_surface_name.clone());
    state.timestep_index += 1;
    Ok(predictor_output)
}
