//! Zone-air correction and adaptive system-timestep helpers.

use crate::heat_balance::air_manager::{
    update_single_zone_air_heat_capacity_from_weather_context,
    update_zone_air_heat_capacities_from_weather_context,
    weather_proxy_zone_air_heat_capacity_j_per_k,
};
use crate::heat_balance::ctf::surface_outside_conduction_rate_w_for_report;
use crate::heat_balance::inside_convection::{
    zone_air_heat_balance_surface_convection_rate_w, zone_surface_convection_sums_for_indices,
};
use crate::heat_balance::state::{
    HeatBalanceSurfaceIndexes, SurfaceHeatBalanceState, ZoneHeatBalanceState,
};
use crate::heat_balance::zone_predictor_corrector::{
    correct_step_source_order_path, energyplus_analytical_zone_air_temperature_c,
    energyplus_third_order_zone_air_temperature_from_coefficients,
    energyplus_zone_air_temperature_coefficients, push_system_timestep_histories_source_order_path,
    revert_zone_timestep_histories_source_order_path,
};
use crate::psychrometrics::energyplus_psychrometric_humidity_ratio_from_rh;
use crate::weather::{
    HeatBalanceWeatherContext, energyplus_weather_atmospheric_pressure_for_context,
};
use ep_model::ZoneId;
use std::collections::BTreeMap;

/// EnergyPlus source-order owner for zone-air correction and history updates.
pub const ZONE_AIR_CORRECTION_OWNER_STAGE: &str = "ManageZoneAirUpdates";

pub(crate) const ENERGYPLUS_DEFAULT_ZONE_AIR_HUMIDITY_RATIO: f64 = 0.008;
const ENERGYPLUS_STANDARD_ATMOSPHERIC_PRESSURE_PA: f64 = 101_325.0;
const ENERGYPLUS_MAX_ZONE_TEMP_DIFF_C: f64 = 0.3;
const ENERGYPLUS_MIN_SYSTEM_TIMESTEP_SECONDS: f64 = 60.0;
const ENERGYPLUS_MIN_HUMIDITY_RATIO: f64 = 1.0e-5;

pub(crate) fn heat_balance_zone_temperature_map(
    zones: &[ZoneHeatBalanceState],
) -> BTreeMap<ZoneId, f64> {
    zones
        .iter()
        .map(|zone| (zone.zone_id, zone.mean_air_temperature_c))
        .collect()
}

pub(crate) fn correct_zone_air_temperatures_from_current_surfaces(
    surfaces: &[SurfaceHeatBalanceState],
    surface_indexes: &HeatBalanceSurfaceIndexes,
    zones: &mut [ZoneHeatBalanceState],
    timestep_seconds: f64,
    weather_context: Option<HeatBalanceWeatherContext<'_>>,
    fallback_dry_bulb_c: f64,
    update_mean_air_temperature: bool,
    use_third_order_zone_air_correction: bool,
    use_inside_ctf_outside_temperature_for_conduction_report: bool,
) {
    update_zone_air_heat_capacities_from_weather_context(
        zones,
        weather_context,
        fallback_dry_bulb_c,
    );

    for zone in zones {
        let zone_surface_indexes = surface_indexes.surfaces_for_zone(zone.zone_id);
        zone.opaque_surface_heat_gain_w = zone_surface_indexes
            .iter()
            .filter_map(|surface_index| surfaces.get(*surface_index))
            .map(|surface| surface.heat_gain_to_zone_w)
            .sum();
        zone.opaque_surface_outside_conduction_w = zone_surface_indexes
            .iter()
            .filter_map(|surface_index| surfaces.get(*surface_index))
            .map(|surface| {
                surface_outside_conduction_rate_w_for_report(
                    surface,
                    use_inside_ctf_outside_temperature_for_conduction_report,
                )
            })
            .sum();
        let (sum_ha_w_per_k, sum_hat_surf_w, sum_hat_ref_w) =
            zone_surface_convection_sums_for_indices(surfaces, zone_surface_indexes);
        zone.sum_ha_w_per_k = sum_ha_w_per_k;
        zone.sum_hat_surf_w = sum_hat_surf_w;
        zone.sum_hat_ref_w = sum_hat_ref_w;
        let coefficients = energyplus_zone_air_temperature_coefficients(
            zone.sum_ha_w_per_k,
            zone.sum_hat_surf_w,
            zone.sum_hat_ref_w,
            zone.convective_internal_gain_w,
            0.0,
            0.0,
            zone.air_heat_capacity_j_per_k,
            timestep_seconds,
            zone.previous_mean_air_temperatures_c,
        );
        if update_mean_air_temperature {
            zone.mean_air_temperature_c = if use_third_order_zone_air_correction {
                energyplus_third_order_zone_air_temperature_from_coefficients(
                    zone.previous_mean_air_temperatures_c[0],
                    coefficients,
                )
            } else {
                energyplus_analytical_zone_air_temperature_c(
                    zone.previous_mean_air_temperatures_c[0],
                    coefficients.temp_independent_coefficient_w,
                    coefficients.temp_dependent_coefficient_w_per_k,
                    zone.air_heat_capacity_j_per_k,
                    timestep_seconds,
                )
            };
        }
        zone.zone_air_temperature_coefficients = coefficients;
    }
}

pub(crate) fn correct_zone_air_humidity_ratios_from_current_state(
    zones: &mut [ZoneHeatBalanceState],
    timestep_seconds: f64,
    context: Option<HeatBalanceWeatherContext<'_>>,
    use_third_order_zone_air_correction: bool,
) {
    for zone in zones {
        let humidity_ratio = if use_third_order_zone_air_correction {
            let history_term = 3.0 * zone.previous_air_humidity_ratios[0]
                - (3.0 / 2.0) * zone.previous_air_humidity_ratios[1]
                + (1.0 / 3.0) * zone.previous_air_humidity_ratios[2];
            history_term / (11.0 / 6.0)
        } else {
            zone.previous_air_humidity_ratios[0]
        };
        let saturation_humidity_ratio = context
            .and_then(|context| {
                let atmospheric_pressure_pa = energyplus_weather_atmospheric_pressure_for_context(
                    context,
                    context
                        .records
                        .get(context.record_index)?
                        .atmospheric_pressure_pa,
                );
                energyplus_psychrometric_humidity_ratio_from_rh(
                    zone.mean_air_temperature_c,
                    1.0,
                    atmospheric_pressure_pa,
                )
            })
            .or_else(|| {
                energyplus_psychrometric_humidity_ratio_from_rh(
                    zone.mean_air_temperature_c,
                    1.0,
                    ENERGYPLUS_STANDARD_ATMOSPHERIC_PRESSURE_PA,
                )
            })
            .unwrap_or(f64::INFINITY);
        zone.air_humidity_ratio = humidity_ratio
            .clamp(0.0, saturation_humidity_ratio)
            .max(ENERGYPLUS_MIN_HUMIDITY_RATIO);
        if timestep_seconds <= 0.0 || !zone.air_humidity_ratio.is_finite() {
            zone.air_humidity_ratio = ENERGYPLUS_DEFAULT_ZONE_AIR_HUMIDITY_RATIO;
        }
    }
}

pub(crate) fn synchronize_single_system_timestep_history(zone: &mut ZoneHeatBalanceState) {
    zone.previous_system_mean_air_temperatures_c = [
        zone.mean_air_temperature_c,
        zone.previous_mean_air_temperatures_c[0],
        zone.previous_mean_air_temperatures_c[1],
    ];
    zone.previous_system_air_humidity_ratios = [
        zone.air_humidity_ratio,
        zone.previous_air_humidity_ratios[0],
        zone.previous_air_humidity_ratios[1],
    ];
    zone.previous_system_timestep_count = 1;
}
pub(crate) fn apply_energyplus_adaptive_system_timestep_zone_air_correction(
    surfaces: &[SurfaceHeatBalanceState],
    surface_indexes: &HeatBalanceSurfaceIndexes,
    zones: &mut [ZoneHeatBalanceState],
    zone_timestep_seconds: f64,
    weather_context: Option<HeatBalanceWeatherContext<'_>>,
    fallback_dry_bulb_c: f64,
    use_inside_ctf_outside_temperature_for_conduction_report: bool,
) {
    if zone_timestep_seconds <= 0.0 {
        return;
    }

    let limit_system_timestep_count = (zone_timestep_seconds
        / ENERGYPLUS_MIN_SYSTEM_TIMESTEP_SECONDS)
        .floor()
        .max(1.0) as u32;

    for zone in zones {
        let zone_temp_change_c =
            (zone.mean_air_temperature_c - zone.previous_mean_air_temperatures_c[0]).abs();
        let system_timestep_count = if zone_temp_change_c > ENERGYPLUS_MAX_ZONE_TEMP_DIFF_C {
            let requested = (zone_temp_change_c / ENERGYPLUS_MAX_ZONE_TEMP_DIFF_C + 1.0) as u32;
            requested.clamp(1, limit_system_timestep_count)
        } else {
            1
        };

        if system_timestep_count <= 1 {
            zone.zone_timestep_average_air_temperature_c = zone.mean_air_temperature_c;
            zone.zone_timestep_average_air_humidity_ratio = zone.air_humidity_ratio;
            push_system_timestep_histories_source_order_path(|| {
                synchronize_single_system_timestep_history(zone);
            });
            zone.system_timestep_average_surface_convection_report_w = None;
            zone.system_timestep_average_air_storage_report_w = None;
            continue;
        }

        let system_timestep_seconds = zone_timestep_seconds / f64::from(system_timestep_count);
        let mut system_temperature_history =
            if system_timestep_count == zone.previous_system_timestep_count {
                zone.previous_system_mean_air_temperatures_c
            } else {
                energyplus_down_interpolate_three_history_values(
                    zone_timestep_seconds,
                    system_timestep_seconds,
                    zone.previous_mean_air_temperatures_c,
                )
            };
        let mut system_humidity_history =
            if system_timestep_count == zone.previous_system_timestep_count {
                zone.previous_system_air_humidity_ratios
            } else {
                energyplus_down_interpolate_three_history_values(
                    zone_timestep_seconds,
                    system_timestep_seconds,
                    zone.previous_air_humidity_ratios,
                )
            };

        let reset_zone_air_state_from_system_history =
            system_timestep_count != zone.previous_system_timestep_count;
        revert_zone_timestep_histories_source_order_path(|| {
            if reset_zone_air_state_from_system_history {
                zone.mean_air_temperature_c = system_temperature_history[0];
                zone.air_humidity_ratio = system_humidity_history[0];
            }
        });
        let mut zone_temperature_average_c = 0.0;
        let mut zone_humidity_average = 0.0;
        let mut surface_convection_report_average_w = 0.0;
        let mut air_storage_report_average_w = 0.0;
        let system_timestep_fraction = 1.0 / f64::from(system_timestep_count);

        for _ in 0..system_timestep_count {
            correct_step_source_order_path(|| {
                correct_single_zone_air_temperature_from_current_surfaces(
                    surfaces,
                    surface_indexes.surfaces_for_zone(zone.zone_id),
                    zone,
                    system_timestep_seconds,
                    system_temperature_history,
                    weather_context,
                    fallback_dry_bulb_c,
                    use_inside_ctf_outside_temperature_for_conduction_report,
                );
                correct_single_zone_air_humidity_ratio_from_history(
                    zone,
                    system_timestep_seconds,
                    system_humidity_history,
                    weather_context,
                );
            });
            let air_storage_rate_w = zone_air_system_timestep_storage_report_rate_w(
                zone,
                system_temperature_history[0],
                system_timestep_seconds,
                weather_context,
                fallback_dry_bulb_c,
            );
            let surface_convection_rate_w = zone_air_heat_balance_surface_convection_rate_w(zone);
            zone_temperature_average_c += zone.mean_air_temperature_c * system_timestep_fraction;
            zone_humidity_average += zone.air_humidity_ratio * system_timestep_fraction;
            surface_convection_report_average_w +=
                surface_convection_rate_w * system_timestep_fraction;
            air_storage_report_average_w += air_storage_rate_w * system_timestep_fraction;
            system_temperature_history = [
                zone.mean_air_temperature_c,
                system_temperature_history[0],
                system_temperature_history[1],
            ];
            system_humidity_history = [
                zone.air_humidity_ratio,
                system_humidity_history[0],
                system_humidity_history[1],
            ];
        }

        push_system_timestep_histories_source_order_path(|| {
            zone.zone_timestep_average_air_temperature_c = zone_temperature_average_c;
            zone.zone_timestep_average_air_humidity_ratio = zone_humidity_average;
            zone.previous_system_mean_air_temperatures_c = system_temperature_history;
            zone.previous_system_air_humidity_ratios = system_humidity_history;
            zone.previous_system_timestep_count = system_timestep_count;
            zone.system_timestep_average_surface_convection_report_w =
                Some(surface_convection_report_average_w);
            zone.system_timestep_average_air_storage_report_w = Some(air_storage_report_average_w);
        });
    }
}

pub(crate) fn zone_air_system_timestep_storage_report_rate_w(
    zone: &ZoneHeatBalanceState,
    previous_system_temperature_c: f64,
    system_timestep_seconds: f64,
    weather_context: Option<HeatBalanceWeatherContext<'_>>,
    fallback_dry_bulb_c: f64,
) -> f64 {
    if system_timestep_seconds <= 0.0 {
        return 0.0;
    }

    let report_air_heat_capacity_j_per_k =
        weather_proxy_zone_air_heat_capacity_j_per_k(zone, weather_context, fallback_dry_bulb_c)
            .unwrap_or(zone.air_heat_capacity_j_per_k);
    report_air_heat_capacity_j_per_k * (zone.mean_air_temperature_c - previous_system_temperature_c)
        / system_timestep_seconds
}

pub(crate) fn correct_single_zone_air_temperature_from_current_surfaces(
    surfaces: &[SurfaceHeatBalanceState],
    surface_indices: &[usize],
    zone: &mut ZoneHeatBalanceState,
    timestep_seconds: f64,
    previous_mean_air_temperatures_c: [f64; 3],
    weather_context: Option<HeatBalanceWeatherContext<'_>>,
    fallback_dry_bulb_c: f64,
    use_inside_ctf_outside_temperature_for_conduction_report: bool,
) {
    update_single_zone_air_heat_capacity_from_weather_context(
        zone,
        weather_context,
        fallback_dry_bulb_c,
    );
    zone.opaque_surface_heat_gain_w = surface_indices
        .iter()
        .filter_map(|surface_index| surfaces.get(*surface_index))
        .map(|surface| surface.heat_gain_to_zone_w)
        .sum();
    zone.opaque_surface_outside_conduction_w = surface_indices
        .iter()
        .filter_map(|surface_index| surfaces.get(*surface_index))
        .map(|surface| {
            surface_outside_conduction_rate_w_for_report(
                surface,
                use_inside_ctf_outside_temperature_for_conduction_report,
            )
        })
        .sum();
    let (sum_ha_w_per_k, sum_hat_surf_w, sum_hat_ref_w) =
        zone_surface_convection_sums_for_indices(surfaces, surface_indices);
    zone.sum_ha_w_per_k = sum_ha_w_per_k;
    zone.sum_hat_surf_w = sum_hat_surf_w;
    zone.sum_hat_ref_w = sum_hat_ref_w;
    let coefficients = energyplus_zone_air_temperature_coefficients(
        zone.sum_ha_w_per_k,
        zone.sum_hat_surf_w,
        zone.sum_hat_ref_w,
        zone.convective_internal_gain_w,
        0.0,
        0.0,
        zone.air_heat_capacity_j_per_k,
        timestep_seconds,
        previous_mean_air_temperatures_c,
    );
    zone.mean_air_temperature_c = energyplus_third_order_zone_air_temperature_from_coefficients(
        previous_mean_air_temperatures_c[0],
        coefficients,
    );
    zone.zone_air_temperature_coefficients = coefficients;
}

pub(crate) fn correct_single_zone_air_humidity_ratio_from_history(
    zone: &mut ZoneHeatBalanceState,
    timestep_seconds: f64,
    previous_air_humidity_ratios: [f64; 3],
    context: Option<HeatBalanceWeatherContext<'_>>,
) {
    let history_term = 3.0 * previous_air_humidity_ratios[0]
        - (3.0 / 2.0) * previous_air_humidity_ratios[1]
        + (1.0 / 3.0) * previous_air_humidity_ratios[2];
    let humidity_ratio = history_term / (11.0 / 6.0);
    let atmospheric_pressure_pa = context
        .and_then(|context| {
            Some(energyplus_weather_atmospheric_pressure_for_context(
                context,
                context
                    .records
                    .get(context.record_index)?
                    .atmospheric_pressure_pa,
            ))
        })
        .unwrap_or(ENERGYPLUS_STANDARD_ATMOSPHERIC_PRESSURE_PA);
    let saturation_humidity_ratio = energyplus_psychrometric_humidity_ratio_from_rh(
        zone.mean_air_temperature_c,
        1.0,
        atmospheric_pressure_pa,
    )
    .unwrap_or(f64::INFINITY);
    zone.air_humidity_ratio = humidity_ratio
        .clamp(0.0, saturation_humidity_ratio)
        .max(ENERGYPLUS_MIN_HUMIDITY_RATIO);
    if timestep_seconds <= 0.0 || !zone.air_humidity_ratio.is_finite() {
        zone.air_humidity_ratio = ENERGYPLUS_DEFAULT_ZONE_AIR_HUMIDITY_RATIO;
    }
}

pub(crate) fn energyplus_down_interpolate_three_history_values(
    old_timestep_seconds: f64,
    new_timestep_seconds: f64,
    old_values: [f64; 3],
) -> [f64; 3] {
    if old_timestep_seconds <= 0.0 || new_timestep_seconds <= 0.0 {
        return old_values;
    }

    let down_step_ratio = old_timestep_seconds / new_timestep_seconds;
    let mut new_values = [old_values[0]; 3];
    if (down_step_ratio - 2.0).abs() < 0.01 {
        new_values[1] = (old_values[0] + old_values[1]) / 2.0;
        new_values[2] = old_values[1];
    } else if (down_step_ratio - 3.0).abs() < 0.01 {
        let delta = (old_values[1] - old_values[0]) / 3.0;
        new_values[1] = old_values[0] + delta;
        new_values[2] = new_values[1] + delta;
    } else {
        let delta = (old_values[1] - old_values[0]) / down_step_ratio;
        new_values[1] = old_values[0] + delta;
        new_values[2] = new_values[1] + delta;
    }
    new_values
}
