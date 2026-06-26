//! Surface/zone coupled loop helpers for CalcHeatBalanceInsideSurf.

use crate::heat_balance::convection::energyplus_tarp_inside_convection_coefficient_w_per_m2_k;
use crate::heat_balance::ctf::{
    CtfInsideFaceBalanceInput, energyplus_ctf_inside_face_temperature_c_with_outside_temperature,
    surface_inside_conduction_rate_w_for_report, update_surface_ctf_history_constants,
};
use crate::heat_balance::inside_convection::{
    heat_balance_inside_convection_coefficient_inputs, heat_balance_inside_convection_coefficients,
};
use crate::heat_balance::radiation::{
    InteriorLongwaveExchangeProbe, update_surface_inside_longwave_exchange_probe,
    update_surface_inside_scriptf_flat_access_longwave_exchange_probe,
    update_surface_inside_scriptf_longwave_exchange_probe,
};
use crate::heat_balance::state::{
    HeatBalanceStepInput, HeatBalanceSurfaceIndexes, HeatBalanceSurfaceLoopZoneAirCorrection,
    InsideConvectionCoefficientInputState, SurfaceBoundaryBalanceResult, SurfaceHeatBalanceState,
    ZoneHeatBalanceState,
};
use crate::heat_balance::surface_balance::{
    QuickOutsideConductionContext, heat_balance_surface_boundary_balance,
    surface_inside_ctf_source_terms_w_per_m2,
};
use crate::heat_balance::zone_air_correction::{
    correct_zone_air_temperatures_from_current_surfaces, heat_balance_zone_temperature_map,
};
use crate::weather::HeatBalanceWeatherContext;
use ep_model::{OutsideBoundaryCondition, SurfaceId, TypedModel, ZoneId};
use std::collections::BTreeMap;

const ENERGYPLUS_MAX_ALLOWED_INSIDE_SURFACE_DELTA_C: f64 = 0.002;
#[derive(Default)]
pub(crate) struct InterleavedSurfaceZoneBalanceResult {
    pub(crate) inside_ctf_outside_temperature_snapshots: Option<BTreeMap<SurfaceId, f64>>,
    pub(crate) inside_surface_iteration_count: u32,
    pub(crate) max_inside_surface_delta_c: f64,
    pub(crate) max_delta_surface_name: Option<String>,
}

pub(crate) fn run_interleaved_surface_zone_balance(
    model: &TypedModel,
    surfaces: &mut [SurfaceHeatBalanceState],
    surface_indexes: &HeatBalanceSurfaceIndexes,
    zones: &mut [ZoneHeatBalanceState],
    first_pass_inside_temperatures: Option<&BTreeMap<SurfaceId, f64>>,
    input: HeatBalanceStepInput,
    weather_context: Option<HeatBalanceWeatherContext<'_>>,
    surface_iteration_count: u32,
    use_previous_inside_for_outdoor_boundary: bool,
    use_previous_inside_for_adiabatic_boundary: bool,
    use_quick_outside_conduction: bool,
    exterior_coefficient_surface_temperatures: Option<&BTreeMap<SurfaceId, f64>>,
    use_doe2_outside_convection: bool,
    interior_longwave_exchange_probe: InteriorLongwaveExchangeProbe,
    freeze_inside_convection_for_timestep: bool,
    use_current_inside_for_first_longwave: bool,
    use_third_order_zone_air_correction: bool,
    freeze_surface_reference_air_for_timestep: bool,
    converge_surface_iterations_to_energyplus_tolerance: bool,
    freeze_outside_balance_for_surface_iterations: bool,
    freeze_inside_ctf_outside_temperature_for_surface_iterations: bool,
    use_inside_ctf_outside_temperature_for_conduction_report: bool,
    inside_hconv_reevaluation_interval: Option<u32>,
    surface_loop_zone_air_correction: HeatBalanceSurfaceLoopZoneAirCorrection,
) -> InterleavedSurfaceZoneBalanceResult {
    let inside_hconv_reevaluation_interval =
        inside_hconv_reevaluation_interval.filter(|interval| *interval > 0);
    let mut inside_convection_coefficients =
        if freeze_inside_convection_for_timestep || inside_hconv_reevaluation_interval.is_some() {
            let zone_temperatures = heat_balance_zone_temperature_map(zones);
            Some(heat_balance_inside_convection_coefficients(
                surfaces,
                &zone_temperatures,
                first_pass_inside_temperatures,
            ))
        } else {
            None
        };
    let mut inside_convection_coefficient_inputs =
        if freeze_inside_convection_for_timestep || inside_hconv_reevaluation_interval.is_some() {
            let zone_temperatures = heat_balance_zone_temperature_map(zones);
            Some(heat_balance_inside_convection_coefficient_inputs(
                surfaces,
                &zone_temperatures,
                first_pass_inside_temperatures,
            ))
        } else {
            None
        };
    let frozen_surface_reference_air_temperatures = if freeze_surface_reference_air_for_timestep {
        Some(
            zones
                .iter()
                .map(|zone| (zone.zone_id, zone.mean_air_temperature_c))
                .collect::<BTreeMap<_, _>>(),
        )
    } else {
        None
    };
    let mut frozen_outside_boundary_balances: Option<
        BTreeMap<SurfaceId, SurfaceBoundaryBalanceResult>,
    > = None;
    let mut frozen_inside_ctf_outside_temperatures: Option<BTreeMap<SurfaceId, f64>> = None;
    let mut inside_surface_iteration_count = 0;
    let mut final_max_inside_surface_delta_c = f64::NAN;
    let mut final_max_delta_surface_name = None;

    for surface_iteration_index in 0..surface_iteration_count.max(1) {
        inside_surface_iteration_count = surface_iteration_index + 1;
        let pass_start_inside_temperatures = if converge_surface_iterations_to_energyplus_tolerance
        {
            Some(
                surfaces
                    .iter()
                    .map(|surface| (surface.surface_id, surface.inside_face_temperature_c))
                    .collect::<BTreeMap<_, _>>(),
            )
        } else {
            None
        };
        let current_zone_temperatures = heat_balance_zone_temperature_map(zones);
        if let Some(interval) = inside_hconv_reevaluation_interval {
            if surface_iteration_index > 0 && surface_iteration_index % interval == 0 {
                inside_convection_coefficients = Some(heat_balance_inside_convection_coefficients(
                    surfaces,
                    &current_zone_temperatures,
                    None,
                ));
                inside_convection_coefficient_inputs =
                    Some(heat_balance_inside_convection_coefficient_inputs(
                        surfaces,
                        &current_zone_temperatures,
                        None,
                    ));
            }
        }
        let zone_temperatures = frozen_surface_reference_air_temperatures
            .as_ref()
            .unwrap_or(&current_zone_temperatures);
        let first_pass_temperatures = if surface_iteration_index == 0 {
            first_pass_inside_temperatures
        } else {
            None
        };
        let first_pass_longwave_temperatures = if use_current_inside_for_first_longwave {
            None
        } else {
            first_pass_temperatures
        };
        run_surface_balance_passes(
            model,
            surfaces,
            surface_indexes,
            first_pass_temperatures,
            first_pass_longwave_temperatures,
            // EnergyPlus sets regular adiabatic/partition outside-face CTF state
            // during the outside balance before the inside surface loop, then
            // reports outside flux from that frozen state in UpdateThermalHistories.
            if use_previous_inside_for_adiabatic_boundary {
                first_pass_inside_temperatures
            } else {
                None
            },
            zone_temperatures,
            input,
            weather_context,
            1,
            use_previous_inside_for_outdoor_boundary,
            use_previous_inside_for_adiabatic_boundary,
            use_quick_outside_conduction,
            exterior_coefficient_surface_temperatures,
            use_doe2_outside_convection,
            interior_longwave_exchange_probe,
            inside_convection_coefficients.as_ref(),
            inside_convection_coefficient_inputs.as_ref(),
            frozen_outside_boundary_balances.as_ref(),
            frozen_inside_ctf_outside_temperatures.as_ref(),
            use_inside_ctf_outside_temperature_for_conduction_report,
        );
        if freeze_outside_balance_for_surface_iterations
            && frozen_outside_boundary_balances.is_none()
        {
            frozen_outside_boundary_balances = Some(
                surfaces
                    .iter()
                    .map(|surface| {
                        (
                            surface.surface_id,
                            SurfaceBoundaryBalanceResult {
                                temperature_c: surface.outside_face_temperature_c,
                                exterior_report_terms: surface.outside_report_terms,
                                outside_balance_diagnostics: surface.outside_balance_diagnostics,
                            },
                        )
                    })
                    .collect(),
            );
        }
        if freeze_inside_ctf_outside_temperature_for_surface_iterations
            && frozen_inside_ctf_outside_temperatures.is_none()
        {
            frozen_inside_ctf_outside_temperatures = Some(
                surfaces
                    .iter()
                    .map(|surface| (surface.surface_id, surface.outside_face_temperature_c))
                    .collect(),
            );
        }
        let (max_inside_surface_delta_c, max_delta_surface_name) = pass_start_inside_temperatures
            .as_ref()
            .map(|temperatures| {
                surfaces.iter().fold((0.0, None), |best, surface| {
                    let Some(previous) = temperatures.get(&surface.surface_id) else {
                        return best;
                    };
                    let delta = (surface.inside_face_temperature_c - previous).abs();
                    if delta > best.0 {
                        (delta, Some(surface.surface_name.clone()))
                    } else {
                        best
                    }
                })
            })
            .unwrap_or((f64::INFINITY, None));
        final_max_inside_surface_delta_c = max_inside_surface_delta_c;
        final_max_delta_surface_name = max_delta_surface_name;
        if matches!(
            surface_loop_zone_air_correction,
            HeatBalanceSurfaceLoopZoneAirCorrection::EachSurfaceIteration
        ) {
            correct_zone_air_temperatures_from_current_surfaces(
                surfaces,
                surface_indexes,
                zones,
                input.timestep_seconds,
                weather_context,
                input.outdoor_dry_bulb_c,
                true,
                use_third_order_zone_air_correction,
                use_inside_ctf_outside_temperature_for_conduction_report,
            );
        }
        if converge_surface_iterations_to_energyplus_tolerance
            && max_inside_surface_delta_c <= ENERGYPLUS_MAX_ALLOWED_INSIDE_SURFACE_DELTA_C
        {
            break;
        }
    }

    InterleavedSurfaceZoneBalanceResult {
        inside_ctf_outside_temperature_snapshots: frozen_inside_ctf_outside_temperatures,
        inside_surface_iteration_count,
        max_inside_surface_delta_c: final_max_inside_surface_delta_c,
        max_delta_surface_name: final_max_delta_surface_name,
    }
}

pub(crate) fn run_surface_balance_passes(
    model: &TypedModel,
    surfaces: &mut [SurfaceHeatBalanceState],
    surface_indexes: &HeatBalanceSurfaceIndexes,
    first_pass_inside_temperatures: Option<&BTreeMap<SurfaceId, f64>>,
    first_pass_longwave_temperatures: Option<&BTreeMap<SurfaceId, f64>>,
    adiabatic_boundary_inside_temperatures: Option<&BTreeMap<SurfaceId, f64>>,
    zone_temperatures: &BTreeMap<ZoneId, f64>,
    input: HeatBalanceStepInput,
    weather_context: Option<HeatBalanceWeatherContext<'_>>,
    surface_iteration_count: u32,
    use_previous_inside_for_outdoor_boundary: bool,
    use_previous_inside_for_adiabatic_boundary: bool,
    use_quick_outside_conduction: bool,
    exterior_coefficient_surface_temperatures: Option<&BTreeMap<SurfaceId, f64>>,
    use_doe2_outside_convection: bool,
    interior_longwave_exchange_probe: InteriorLongwaveExchangeProbe,
    inside_convection_coefficient_overrides: Option<&BTreeMap<SurfaceId, f64>>,
    inside_convection_coefficient_input_overrides: Option<
        &BTreeMap<SurfaceId, InsideConvectionCoefficientInputState>,
    >,
    outside_boundary_balance_snapshots: Option<&BTreeMap<SurfaceId, SurfaceBoundaryBalanceResult>>,
    inside_ctf_outside_temperature_snapshots: Option<&BTreeMap<SurfaceId, f64>>,
    use_inside_ctf_outside_temperature_for_conduction_report: bool,
) {
    for surface_iteration_index in 0..surface_iteration_count.max(1) {
        let temperature_overrides = if surface_iteration_index == 0 {
            first_pass_longwave_temperatures
        } else {
            None
        };
        match interior_longwave_exchange_probe {
            InteriorLongwaveExchangeProbe::None => {}
            InteriorLongwaveExchangeProbe::GreyEnergyPlusDirectViewFactor => {
                update_surface_inside_longwave_exchange_probe(
                    surfaces,
                    surface_indexes,
                    temperature_overrides,
                );
            }
            InteriorLongwaveExchangeProbe::EnergyPlusScriptF => {
                update_surface_inside_scriptf_longwave_exchange_probe(
                    surfaces,
                    surface_indexes,
                    temperature_overrides,
                );
            }
            InteriorLongwaveExchangeProbe::EnergyPlusScriptFFlatAccess => {
                update_surface_inside_scriptf_flat_access_longwave_exchange_probe(
                    surfaces,
                    surface_indexes,
                    temperature_overrides,
                );
            }
        }
        for surface in surfaces.iter_mut() {
            let previous_inside_face_temperature_c = if surface_iteration_index == 0 {
                first_pass_inside_temperatures
                    .and_then(|temperatures| temperatures.get(&surface.surface_id).copied())
                    .unwrap_or(surface.inside_face_temperature_c)
            } else {
                surface.inside_face_temperature_c
            };
            let zone_temperature_c = zone_temperatures
                .get(&surface.zone_id)
                .copied()
                .unwrap_or(surface.inside_face_temperature_c);
            surface.inside_reference_air_temperature_c = zone_temperature_c;
            let inside_convection_coefficient_w_per_m2_k = inside_convection_coefficient_overrides
                .and_then(|coefficients| coefficients.get(&surface.surface_id).copied())
                .unwrap_or_else(|| {
                    energyplus_tarp_inside_convection_coefficient_w_per_m2_k(
                        surface,
                        previous_inside_face_temperature_c,
                        zone_temperature_c,
                    )
                });
            surface.inside_convection_coefficient_w_per_m2_k =
                inside_convection_coefficient_w_per_m2_k;
            let inside_convection_input = inside_convection_coefficient_input_overrides
                .and_then(|inputs| inputs.get(&surface.surface_id).copied())
                .unwrap_or(InsideConvectionCoefficientInputState {
                    inside_face_temperature_c: previous_inside_face_temperature_c,
                    reference_air_temperature_c: zone_temperature_c,
                });
            surface.inside_convection_input_inside_face_temperature_c =
                inside_convection_input.inside_face_temperature_c;
            surface.inside_convection_input_reference_air_temperature_c =
                inside_convection_input.reference_air_temperature_c;

            update_surface_ctf_history_constants(surface);
            let use_previous_inside_for_boundary = (use_previous_inside_for_outdoor_boundary
                && surface.outside_boundary_condition == OutsideBoundaryCondition::Outdoors)
                || (use_previous_inside_for_adiabatic_boundary
                    && surface.outside_boundary_condition == OutsideBoundaryCondition::Adiabatic);
            let outside_balance_inside_temperature_c = if use_previous_inside_for_adiabatic_boundary
                && surface.outside_boundary_condition == OutsideBoundaryCondition::Adiabatic
            {
                adiabatic_boundary_inside_temperatures
                    .and_then(|temperatures| temperatures.get(&surface.surface_id).copied())
                    .unwrap_or(previous_inside_face_temperature_c)
            } else if use_previous_inside_for_boundary {
                previous_inside_face_temperature_c
            } else {
                zone_temperature_c
            };
            surface.inside_face_temperature_c = outside_balance_inside_temperature_c;
            let net_inside_source_w_per_m2 = surface_inside_ctf_source_terms_w_per_m2(surface);
            let quick_outside_conduction = if use_quick_outside_conduction
                && surface.outside_boundary_condition == OutsideBoundaryCondition::Outdoors
            {
                Some(QuickOutsideConductionContext {
                    reference_air_temperature_c: zone_temperature_c,
                    inside_convection_coefficient_w_per_m2_k,
                    net_inside_source_w_per_m2,
                    exterior_coefficient_surface_temperature_c:
                        exterior_coefficient_surface_temperatures
                            .and_then(|temperatures| temperatures.get(&surface.surface_id))
                            .copied(),
                    use_doe2_outside_convection,
                })
            } else {
                None
            };
            let boundary_balance = outside_boundary_balance_snapshots
                .and_then(|snapshots| snapshots.get(&surface.surface_id).copied())
                .unwrap_or_else(|| {
                    heat_balance_surface_boundary_balance(
                        model,
                        surface,
                        zone_temperatures,
                        input.outdoor_dry_bulb_c,
                        outside_balance_inside_temperature_c,
                        weather_context,
                        quick_outside_conduction,
                        use_doe2_outside_convection,
                    )
                });
            surface.outside_face_temperature_c = boundary_balance.temperature_c;
            surface.outside_report_terms = boundary_balance.exterior_report_terms;
            surface.outside_balance_diagnostics = boundary_balance.outside_balance_diagnostics;
            let inside_ctf_outside_temperature_c = inside_ctf_outside_temperature_snapshots
                .and_then(|snapshots| snapshots.get(&surface.surface_id).copied());
            surface.inside_ctf_outside_temperature_c =
                inside_ctf_outside_temperature_c.unwrap_or(surface.outside_face_temperature_c);
            surface.inside_face_temperature_c =
                energyplus_ctf_inside_face_temperature_c_with_outside_temperature(
                    surface,
                    CtfInsideFaceBalanceInput {
                        reference_air_temperature_c: zone_temperature_c,
                        inside_convection_coefficient_w_per_m2_k,
                        previous_inside_face_temperature_c,
                        net_inside_source_w_per_m2,
                    },
                    inside_ctf_outside_temperature_c,
                );
            if surface.outside_boundary_condition == OutsideBoundaryCondition::Adiabatic
                && !use_previous_inside_for_adiabatic_boundary
            {
                surface.outside_face_temperature_c = surface.inside_face_temperature_c;
                surface.inside_ctf_outside_temperature_c = surface.inside_face_temperature_c;
            }
            surface.heat_gain_to_zone_w = surface_inside_conduction_rate_w_for_report(
                surface,
                use_inside_ctf_outside_temperature_for_conduction_report,
            );
        }
    }
}
