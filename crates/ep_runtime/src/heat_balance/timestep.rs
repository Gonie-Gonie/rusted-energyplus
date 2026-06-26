//! Heat-balance timestep advance and source-order state update.

use crate::heat_balance::algorithm::{
    HeatBalanceZoneAirAlgorithm,
    heat_balance_preserves_surface_inside_temperature_for_first_longwave,
    heat_balance_timestep_algorithm_flags, heat_balance_uses_third_order_zone_air_correction,
    heat_balance_zone_air_algorithm_execution_variant,
    heat_balance_zone_air_algorithm_feature_base,
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
use crate::schedules::{
    convective_internal_gain_w, update_surface_radiant_internal_gain_source_terms,
};
use crate::weather::HeatBalanceWeatherContext;
use ep_model::{OutsideBoundaryCondition, TypedModel};
use std::collections::BTreeMap;
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
        HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical,
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
    zone_air_algorithm: HeatBalanceZoneAirAlgorithm,
    surface_iteration_count: u32,
    inside_hconv_reevaluation_interval: Option<u32>,
    surface_loop_zone_air_correction: HeatBalanceSurfaceLoopZoneAirCorrection,
) {
    manager::manage_heat_balance_source_order_path(|| {
        advance_heat_balance_state_one_timestep_source_order_path(
            model,
            state,
            input,
            weather_context,
            zone_air_algorithm,
            surface_iteration_count,
            inside_hconv_reevaluation_interval,
            surface_loop_zone_air_correction,
        );
    });
}

fn advance_heat_balance_state_one_timestep_source_order_path(
    model: &TypedModel,
    state: &mut HeatBalanceState,
    input: HeatBalanceStepInput,
    weather_context: Option<HeatBalanceWeatherContext<'_>>,
    zone_air_algorithm: HeatBalanceZoneAirAlgorithm,
    surface_iteration_count: u32,
    inside_hconv_reevaluation_interval: Option<u32>,
    surface_loop_zone_air_correction: HeatBalanceSurfaceLoopZoneAirCorrection,
) {
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
    let requested_zone_air_algorithm = zone_air_algorithm;
    let zone_air_algorithm = heat_balance_zone_air_algorithm_execution_variant(zone_air_algorithm);
    let feature_zone_air_algorithm =
        heat_balance_zone_air_algorithm_feature_base(zone_air_algorithm);
    let use_energyplus_adaptive_system_timestep_zone_air_correction = matches!(
        requested_zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusHeatBalanceCompatCandidate
            | HeatBalanceZoneAirAlgorithm::EnergyPlusSourceOrder1ZoneOpaqueCompatibility
    );
    let algorithm_flags = heat_balance_timestep_algorithm_flags(zone_air_algorithm);
    let use_doe2_outside_convection =
        heat_balance_uses_doe2_outside_convection(model, feature_zone_air_algorithm);
    let sync_adiabatic_outside_to_current_inside_before_history = matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionCurrentAdiabaticHistoryProbe
    );
    let sync_adiabatic_outside_to_current_inside_for_report_only = matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatAdiabaticReportProbe
    );
    let commit_adiabatic_current_inside_to_history_only = matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe
    );
    let interior_longwave_exchange_probe = if matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFProbe
    ) {
        InteriorLongwaveExchangeProbe::EnergyPlusScriptF
    } else if matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusSourceOrder1ZoneOpaqueCompatibility
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveReferenceAirProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveHconvProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatSurfaceReferenceAirReportProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatFinalHconvReportProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatInsideCtfReportProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatAdiabaticReportProbe
    ) {
        InteriorLongwaveExchangeProbe::EnergyPlusScriptFFlatAccess
    } else {
        match feature_zone_air_algorithm {
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInteriorLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2InteriorLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentAdiabaticProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStoragePreviousMatSurfaceConvectionProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionCurrentAdiabaticHistoryProbe => {
                InteriorLongwaveExchangeProbe::GreyEnergyPlusDirectViewFactor
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedScriptFInteriorLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideScriptFInteriorLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2ScriptFInteriorLongwaveProbe => {
                InteriorLongwaveExchangeProbe::EnergyPlusScriptF
            }
            _ => InteriorLongwaveExchangeProbe::None,
        }
    };

    let preserve_surface_inside_temperature_for_first_longwave =
        heat_balance_preserves_surface_inside_temperature_for_first_longwave(
            feature_zone_air_algorithm,
        );

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
    air_manager::manage_air_heat_balance_source_order_path(|| {
        zone_predictor_corrector::manage_zone_air_updates_source_order_path(|| {
            zone_predictor_corrector::push_zone_timestep_histories_source_order_path(|| {
                zone_predictor_corrector::predict_step_source_order_path(|| {
                    for zone in &mut state.zones {
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
                            convective_internal_gain_w(model, zone.zone_id, hour_ending);

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
                                surface.conductance_w_per_k * surface.outside_face_temperature_c
                            })
                            .sum::<f64>();
                        let equivalent_outside_temperature_c = if conductance_w_per_k > 0.0 {
                            conductance_weighted_outside_temperature / conductance_w_per_k
                        } else {
                            previous_temperature_c
                        };

                        zone.opaque_surface_conductance_w_per_k = conductance_w_per_k;
                        zone.mean_air_temperature_c = match feature_zone_air_algorithm {
            HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical => step_zone_air_temperature(
                previous_temperature_c,
                equivalent_outside_temperature_c,
                zone.convective_internal_gain_w,
                conductance_w_per_k,
                zone.air_heat_capacity_j_per_k,
                input.timestep_seconds,
            ),
            HeatBalanceZoneAirAlgorithm::EnergyPlusSourceOrder1ZoneOpaqueCompatibility => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusHeatBalanceCompatCandidate => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalSurfaceFirstProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledProbe => previous_temperature_c,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideDoe2Probe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentLongwaveProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentAdiabaticProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStoragePreviousMatSurfaceConvectionProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryCommitProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveReferenceAirProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveHconvProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatSurfaceReferenceAirReportProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatFinalHconvReportProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatInsideCtfReportProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatAdiabaticReportProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionCurrentAdiabaticHistoryProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedScriptFInteriorLongwaveProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2Probe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInteriorLongwaveProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2InteriorLongwaveProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideScriptFInteriorLongwaveProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2ScriptFInteriorLongwaveProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousBoundaryProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalProbe => {
                let (sum_ha_w_per_k, sum_hat_surf_w, sum_hat_ref_w) =
                    zone_surface_convection_sums_for_indices(
                        &state.surfaces,
                        state.surface_indexes.surfaces_for_zone(zone.zone_id),
                    );
                let coefficients = energyplus_zone_air_temperature_coefficients(
                    sum_ha_w_per_k,
                    sum_hat_surf_w,
                    sum_hat_ref_w,
                    zone.convective_internal_gain_w,
                    0.0,
                    0.0,
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
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderProbe => {
                let (sum_ha_w_per_k, sum_hat_surf_w, sum_hat_ref_w) =
                    zone_surface_convection_sums_for_indices(
                        &state.surfaces,
                        state.surface_indexes.surfaces_for_zone(zone.zone_id),
                    );
                let coefficients = energyplus_zone_air_temperature_coefficients(
                    sum_ha_w_per_k,
                    sum_hat_surf_w,
                    sum_hat_ref_w,
                    zone.convective_internal_gain_w,
                    0.0,
                    0.0,
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
                });
            });
        });
    });
    update_surface_radiant_internal_gain_source_terms(model, &mut state.surfaces, hour_ending);
    let use_current_inside_for_first_longwave = matches!(
        feature_zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe
    );
    let converge_interleaved_surface_iterations_to_energyplus_tolerance = matches!(
        feature_zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe
    );
    let freeze_outside_balance_for_surface_iterations = matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusSourceOrder1ZoneOpaqueCompatibility
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatProbe
    );
    let freeze_inside_ctf_outside_temperature_for_surface_iterations = matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusSourceOrder1ZoneOpaqueCompatibility
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryCommitProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveReferenceAirProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveHconvProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatSurfaceReferenceAirReportProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatFinalHconvReportProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatInsideCtfReportProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatAdiabaticReportProbe
    );
    let use_inside_ctf_outside_temperature_for_conduction_report = matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatInsideCtfReportProbe
    );
    let commit_inside_ctf_outside_temperature_to_history = matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryCommitProbe
    );

    let interleaved_surface_zone_balance_result =
        surface_manager::calc_heat_balance_inside_surf_source_order_path(|| {
            if algorithm_flags.interleave_zone_air_surface_passes {
                Some(run_interleaved_surface_zone_balance(
            model,
            &mut state.surfaces,
            &state.surface_indexes,
            &mut state.zones,
            Some(&previous_surface_inside_temperatures),
            input,
            weather_context,
            surface_iteration_count,
            algorithm_flags.use_previous_inside_for_outdoor_boundary,
            algorithm_flags.use_previous_inside_for_adiabatic_boundary,
            algorithm_flags.use_quick_outside_conduction,
            Some(&previous_surface_outside_temperatures),
            use_doe2_outside_convection,
            interior_longwave_exchange_probe,
            matches!(
                feature_zone_air_algorithm,
                HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
                    | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentLongwaveProbe
                    | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentAdiabaticProbe
                    | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
                    | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageProbe
                    | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStoragePreviousMatSurfaceConvectionProbe
                    | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionProbe
                    | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirProbe
                    | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe
                    | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe
                    | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe
                    | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe
                    | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionCurrentAdiabaticHistoryProbe
            ) && !matches!(
                zone_air_algorithm,
                HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveHconvProbe
            ),
            use_current_inside_for_first_longwave,
            heat_balance_uses_third_order_zone_air_correction(feature_zone_air_algorithm),
            matches!(
                feature_zone_air_algorithm,
                HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirProbe
                    | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe
                    | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe
                    | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe
                    | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe
            ) && !matches!(
                zone_air_algorithm,
                HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveReferenceAirProbe
            ),
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
                    algorithm_flags.use_previous_inside_for_outdoor_boundary,
                    algorithm_flags.use_previous_inside_for_adiabatic_boundary,
                    algorithm_flags.use_quick_outside_conduction,
                    if algorithm_flags.use_quick_outside_conduction {
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

                if algorithm_flags.rebalance_surfaces_after_zone_air_correction {
                    zone_predictor_corrector::correct_step_source_order_path(|| {
                        correct_zone_air_temperatures_from_current_surfaces(
                            &state.surfaces,
                            &state.surface_indexes,
                            &mut state.zones,
                            input.timestep_seconds,
                            weather_context,
                            input.outdoor_dry_bulb_c,
                            true,
                            heat_balance_uses_third_order_zone_air_correction(
                                feature_zone_air_algorithm,
                            ),
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
                        algorithm_flags.use_previous_inside_for_outdoor_boundary,
                        algorithm_flags.use_previous_inside_for_adiabatic_boundary,
                        algorithm_flags.use_quick_outside_conduction,
                        if algorithm_flags.use_quick_outside_conduction {
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

    if algorithm_flags.interleave_zone_air_surface_passes
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
                heat_balance_uses_third_order_zone_air_correction(feature_zone_air_algorithm),
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

    zone_predictor_corrector::correct_step_source_order_path(|| {
        correct_zone_air_temperatures_from_current_surfaces(
            &state.surfaces,
            &state.surface_indexes,
            &mut state.zones,
            input.timestep_seconds,
            weather_context,
            input.outdoor_dry_bulb_c,
            algorithm_flags.correct_zone_air_after_surface_pass
                && !algorithm_flags.interleave_zone_air_surface_passes,
            heat_balance_uses_third_order_zone_air_correction(feature_zone_air_algorithm),
            use_inside_ctf_outside_temperature_for_conduction_report,
        );
        correct_zone_air_humidity_ratios_from_current_state(
            &mut state.zones,
            input.timestep_seconds,
            weather_context,
            heat_balance_uses_third_order_zone_air_correction(feature_zone_air_algorithm),
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
                zone_predictor_corrector::push_system_timestep_histories_source_order_path(|| {
                    synchronize_single_system_timestep_history(zone);
                });
                zone.system_timestep_average_surface_convection_report_w = None;
                zone.system_timestep_average_air_storage_report_w = None;
            }
        }
    });
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
}
