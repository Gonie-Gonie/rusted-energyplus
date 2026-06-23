//! Heat-balance radiation source-order ownership notes.

use crate::geometry::{surface_azimuth_deg, surface_tilt_deg};
use crate::heat_balance::state::{SurfaceHeatBalanceState, SurfaceIncidentSolarComponents};
use crate::time_axis::{DEFAULT_RUN_PERIOD_YEAR, day_of_year};
use crate::weather::{
    EpwRecord, next_weather_record, previous_weather_record_with_first_hour_starting_values,
};
use crate::{OutputSeries, ResultStore};
use ep_model::{
    AutoOrNumber, FirstHourInterpolationStartingValues, OutputHandle, OutsideBoundaryCondition,
    SimulationModel, SiteLocation, SunExposure, Surface, SurfaceId, SurfaceType, ZoneId,
};
use std::collections::BTreeMap;

/// Current longwave/solar source-order owner for outside-face inputs.
pub const EXTERIOR_RADIATION_OWNER_STAGE: &str = "CalcHeatBalanceOutsideSurf";

/// Current longwave source-order owner for inside-face inputs.
pub const INTERIOR_RADIATION_OWNER_STAGE: &str = "CalcHeatBalanceInsideSurf";

const ENERGYPLUS_SUN_IS_UP_COS_ZENITH: f64 = 0.00001;
const ENERGYPLUS_SHADOWING_CALC_FREQUENCY_DAYS: usize = 20;
const DEFAULT_SOLAR_GROUND_REFLECTANCE: f64 = 0.2;
pub(crate) const STEFAN_BOLTZMANN_W_PER_M2_K4: f64 = 5.6697e-8;
pub(crate) const KELVIN_OFFSET: f64 = 273.15;

/// Appends diagnostic surface incident solar radiation series for sun-exposed
/// surfaces with a declared site location.
///
/// The calculation is intentionally a forcing diagnostic: direct normal
/// radiation is projected with EnergyPlus-style weather timestep interpolation
/// and shadowing-period solar position coefficients. Diffuse sky uses the
/// EnergyPlus Perez anisotropic multiplier, and ground reflection uses a fixed
/// default reflectance. It is not a full EnergyPlus solar distribution or
/// shadowing claim.
pub fn append_surface_incident_solar_radiation_series(
    results: &mut ResultStore,
    model: &SimulationModel,
    weather_records: &[EpwRecord],
    sample_count: usize,
) -> usize {
    let Some(site) = model.typed.site.as_ref() else {
        return 0;
    };
    if weather_records.is_empty() || sample_count == 0 {
        return 0;
    }

    let mut added = 0;
    let mut handle_index = results
        .series
        .iter()
        .map(|series| series.handle.0)
        .max()
        .map_or(0, |handle| handle + 1);

    let zone_steps_per_hour = model.typed.timestep.number_of_timesteps_per_hour.max(1);
    for surface in &model.typed.surfaces {
        if surface.sun_exposure != SunExposure::SunExposed
            || surface.outside_boundary_condition != OutsideBoundaryCondition::Outdoors
        {
            continue;
        }
        let components = weather_records
            .iter()
            .enumerate()
            .take(sample_count)
            .map(|(record_index, _record)| {
                surface_incident_solar_components_hourly_average_w_per_m2(
                    surface,
                    site,
                    weather_records,
                    record_index,
                    zone_steps_per_hour,
                )
            })
            .collect::<Vec<_>>();
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: surface.name.0.clone(),
            variable_name: "Surface Outside Face Incident Solar Radiation Rate per Area"
                .to_string(),
            units: "W/m2".to_string(),
            values: components
                .iter()
                .map(|component| component.total_w_per_m2())
                .collect(),
        });
        handle_index += 1;
        added += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: surface.name.0.clone(),
            variable_name: "Surface Outside Face Incident Beam Solar Radiation Rate per Area"
                .to_string(),
            units: "W/m2".to_string(),
            values: components
                .iter()
                .map(|component| component.beam_w_per_m2)
                .collect(),
        });
        handle_index += 1;
        added += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: surface.name.0.clone(),
            variable_name:
                "Surface Outside Face Incident Sky Diffuse Solar Radiation Rate per Area"
                    .to_string(),
            units: "W/m2".to_string(),
            values: components
                .iter()
                .map(|component| component.sky_diffuse_w_per_m2)
                .collect(),
        });
        handle_index += 1;
        added += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: surface.name.0.clone(),
            variable_name:
                "Surface Outside Face Incident Ground Diffuse Solar Radiation Rate per Area"
                    .to_string(),
            units: "W/m2".to_string(),
            values: components
                .iter()
                .map(|component| component.ground_diffuse_w_per_m2)
                .collect(),
        });
        handle_index += 1;
        added += 1;
    }

    added
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum InteriorLongwaveExchangeProbe {
    None,
    GreyEnergyPlusDirectViewFactor,
    EnergyPlusScriptF,
    EnergyPlusScriptFFlatAccess,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct InteriorLongwaveSurfaceSnapshot {
    pub(crate) zone_id: ZoneId,
    pub(crate) surface_type: SurfaceType,
    pub(crate) area_m2: f64,
    pub(crate) azimuth_deg: f64,
    pub(crate) tilt_deg: f64,
    pub(crate) temperature_k4: f64,
    pub(crate) thermal_absorptance: f64,
}

pub(crate) fn update_surface_inside_longwave_exchange_probe(
    surfaces: &mut [SurfaceHeatBalanceState],
    temperature_overrides: Option<&BTreeMap<SurfaceId, f64>>,
) {
    let snapshots = surfaces
        .iter()
        .map(|surface| {
            let temperature_c = temperature_overrides
                .and_then(|temperatures| temperatures.get(&surface.surface_id).copied())
                .unwrap_or(surface.inside_face_temperature_c);
            let temperature_k = (temperature_c + KELVIN_OFFSET).max(0.0);
            InteriorLongwaveSurfaceSnapshot {
                zone_id: surface.zone_id,
                surface_type: surface.surface_type,
                area_m2: surface.area_m2.max(0.0),
                azimuth_deg: surface.azimuth_deg,
                tilt_deg: surface.tilt_deg,
                temperature_k4: temperature_k.powi(4),
                thermal_absorptance: surface.inside_thermal_absorptance.clamp(0.0, 1.0),
            }
        })
        .collect::<Vec<_>>();
    let mut surfaces_by_zone = BTreeMap::<ZoneId, Vec<usize>>::new();
    for (surface_index, snapshot) in snapshots.iter().enumerate() {
        surfaces_by_zone
            .entry(snapshot.zone_id)
            .or_default()
            .push(surface_index);
    }

    let mut longwave_terms_w_per_m2 = vec![0.0; surfaces.len()];
    for surface_indices in surfaces_by_zone.values() {
        if surface_indices.len() <= 1 {
            continue;
        }
        let zone_snapshots = surface_indices
            .iter()
            .map(|surface_index| snapshots[*surface_index])
            .collect::<Vec<_>>();
        let areas = zone_snapshots
            .iter()
            .map(|surface| surface.area_m2)
            .collect::<Vec<_>>();
        if areas.iter().any(|area| *area <= f64::EPSILON) {
            continue;
        }
        let view_factors = fix_energyplus_approximate_view_factors(
            &areas,
            &energyplus_approximate_view_factors(&zone_snapshots),
        );
        let surface_count = zone_snapshots.len();

        for (receiver_zone_index, receiver) in zone_snapshots.iter().enumerate() {
            let mut net_longwave_w_per_m2 = 0.0;
            for (sender_zone_index, sender) in zone_snapshots.iter().enumerate() {
                if sender_zone_index == receiver_zone_index {
                    continue;
                }
                let exchange_emissivity = grey_pair_exchange_emissivity(
                    receiver.thermal_absorptance,
                    sender.thermal_absorptance,
                );
                if exchange_emissivity <= f64::EPSILON {
                    continue;
                }
                net_longwave_w_per_m2 += STEFAN_BOLTZMANN_W_PER_M2_K4
                    * exchange_emissivity
                    * view_factors[sender_zone_index * surface_count + receiver_zone_index]
                    * (sender.temperature_k4 - receiver.temperature_k4);
            }
            longwave_terms_w_per_m2[surface_indices[receiver_zone_index]] = net_longwave_w_per_m2;
        }
    }

    for (surface, net_longwave_w_per_m2) in surfaces.iter_mut().zip(longwave_terms_w_per_m2) {
        surface.inside_net_longwave_w_per_m2 = net_longwave_w_per_m2;
    }
}

pub(crate) fn update_surface_inside_scriptf_longwave_exchange_probe(
    surfaces: &mut [SurfaceHeatBalanceState],
    temperature_overrides: Option<&BTreeMap<SurfaceId, f64>>,
) {
    update_surface_inside_scriptf_longwave_exchange_probe_with_access(
        surfaces,
        temperature_overrides,
        false,
    );
}

pub(crate) fn update_surface_inside_scriptf_flat_access_longwave_exchange_probe(
    surfaces: &mut [SurfaceHeatBalanceState],
    temperature_overrides: Option<&BTreeMap<SurfaceId, f64>>,
) {
    update_surface_inside_scriptf_longwave_exchange_probe_with_access(
        surfaces,
        temperature_overrides,
        true,
    );
}

fn update_surface_inside_scriptf_longwave_exchange_probe_with_access(
    surfaces: &mut [SurfaceHeatBalanceState],
    temperature_overrides: Option<&BTreeMap<SurfaceId, f64>>,
    use_energyplus_flat_lsr_access: bool,
) {
    let snapshots = surfaces
        .iter()
        .map(|surface| {
            let temperature_c = temperature_overrides
                .and_then(|temperatures| temperatures.get(&surface.surface_id).copied())
                .unwrap_or(surface.inside_face_temperature_c);
            let temperature_k = (temperature_c + KELVIN_OFFSET).max(0.0);
            InteriorLongwaveSurfaceSnapshot {
                zone_id: surface.zone_id,
                surface_type: surface.surface_type,
                area_m2: surface.area_m2.max(0.0),
                azimuth_deg: surface.azimuth_deg,
                tilt_deg: surface.tilt_deg,
                temperature_k4: temperature_k.powi(4),
                thermal_absorptance: surface.inside_thermal_absorptance.clamp(0.0, 1.0),
            }
        })
        .collect::<Vec<_>>();

    let mut surfaces_by_zone = BTreeMap::<ZoneId, Vec<usize>>::new();
    for (surface_index, snapshot) in snapshots.iter().enumerate() {
        surfaces_by_zone
            .entry(snapshot.zone_id)
            .or_default()
            .push(surface_index);
    }

    let mut longwave_terms_w_per_m2 = vec![0.0; surfaces.len()];
    for surface_indices in surfaces_by_zone.values() {
        if surface_indices.len() <= 1 {
            continue;
        }
        let zone_snapshots = surface_indices
            .iter()
            .map(|surface_index| snapshots[*surface_index])
            .collect::<Vec<_>>();
        let Some(script_f) = energyplus_scriptf_longwave_matrix_w_per_m2_k4(&zone_snapshots) else {
            continue;
        };
        let surface_count = zone_snapshots.len();
        for (receiver_zone_index, receiver) in zone_snapshots.iter().enumerate() {
            let mut net_longwave_w_per_m2 = 0.0;
            for (sender_zone_index, sender) in zone_snapshots.iter().enumerate() {
                if sender_zone_index == receiver_zone_index {
                    continue;
                }
                let script_f_index = if use_energyplus_flat_lsr_access {
                    receiver_zone_index * surface_count + sender_zone_index
                } else {
                    sender_zone_index * surface_count + receiver_zone_index
                };
                net_longwave_w_per_m2 +=
                    script_f[script_f_index] * (sender.temperature_k4 - receiver.temperature_k4);
            }
            longwave_terms_w_per_m2[surface_indices[receiver_zone_index]] = net_longwave_w_per_m2;
        }
    }

    for (surface, net_longwave_w_per_m2) in surfaces.iter_mut().zip(longwave_terms_w_per_m2) {
        surface.inside_net_longwave_w_per_m2 = net_longwave_w_per_m2;
    }
}

fn energyplus_scriptf_longwave_matrix_w_per_m2_k4(
    surfaces: &[InteriorLongwaveSurfaceSnapshot],
) -> Option<Vec<f64>> {
    let surface_count = surfaces.len();
    if surface_count <= 1
        || surfaces
            .iter()
            .any(|surface| surface.area_m2 <= f64::EPSILON)
    {
        return None;
    }

    let areas = surfaces
        .iter()
        .map(|surface| surface.area_m2)
        .collect::<Vec<_>>();
    let direct_view_factors = fix_energyplus_approximate_view_factors(
        &areas,
        &energyplus_approximate_view_factors(surfaces),
    );
    let mut emissivities = surfaces
        .iter()
        .map(|surface| surface.thermal_absorptance.clamp(0.0, 0.99999))
        .collect::<Vec<_>>();
    energyplus_scriptf_from_view_factors(&areas, &direct_view_factors, &mut emissivities)
}

pub(crate) fn energyplus_approximate_view_factors(
    surfaces: &[InteriorLongwaveSurfaceSnapshot],
) -> Vec<f64> {
    let surface_count = surfaces.len();
    let mut zone_area_seen_m2 = vec![0.0; surface_count];
    for (from_index, from_surface) in surfaces.iter().enumerate() {
        for (to_index, to_surface) in surfaces.iter().enumerate() {
            if energyplus_surface_sees_surface(from_surface, to_surface, from_index, to_index) {
                zone_area_seen_m2[from_index] += to_surface.area_m2;
            }
        }
    }

    let mut view_factors = vec![0.0; surface_count * surface_count];
    for (from_index, from_surface) in surfaces.iter().enumerate() {
        if zone_area_seen_m2[from_index] <= f64::EPSILON {
            continue;
        }
        for (to_index, to_surface) in surfaces.iter().enumerate() {
            if energyplus_surface_sees_surface(from_surface, to_surface, from_index, to_index) {
                view_factors[to_index * surface_count + from_index] =
                    to_surface.area_m2 / zone_area_seen_m2[from_index];
            }
        }
    }
    view_factors
}

fn energyplus_surface_sees_surface(
    from_surface: &InteriorLongwaveSurfaceSnapshot,
    to_surface: &InteriorLongwaveSurfaceSnapshot,
    from_index: usize,
    to_index: usize,
) -> bool {
    if from_index == to_index
        || (from_surface.surface_type == SurfaceType::Floor
            && to_surface.surface_type == SurfaceType::Floor)
    {
        return false;
    }

    let azimuth_difference_deg = (from_surface.azimuth_deg - to_surface.azimuth_deg).abs();
    let tilt_difference_deg = (from_surface.tilt_deg - to_surface.tilt_deg).abs();
    to_surface.surface_type == SurfaceType::Floor
        || from_surface.surface_type == SurfaceType::Floor
        || (azimuth_difference_deg > 10.0 && azimuth_difference_deg < 350.0)
        || tilt_difference_deg > 10.0
}

pub(crate) fn fix_energyplus_approximate_view_factors(
    areas: &[f64],
    view_factors: &[f64],
) -> Vec<f64> {
    let surface_count = areas.len();
    if surface_count == 0 || view_factors.len() != surface_count * surface_count {
        return view_factors.to_vec();
    }

    let original_check = (view_factors.iter().sum::<f64>() - surface_count as f64).abs();
    let mut fixed_area_factors = view_factors.to_vec();
    let total_area = areas.iter().sum::<f64>();
    if surface_count > 3 && total_area > f64::EPSILON {
        if let Some((largest_index, largest_area)) = areas
            .iter()
            .copied()
            .enumerate()
            .max_by(|left, right| left.1.total_cmp(&right.1))
        {
            if largest_area > 0.99 * (total_area - largest_area) {
                fixed_area_factors[largest_index * surface_count + largest_index] =
                    (1.2 * largest_area / total_area).min(0.9);
            }
        }
    }

    let mut area_factor_matrix = vec![0.0; surface_count * surface_count];
    for from_index in 0..surface_count {
        for to_index in 0..surface_count {
            area_factor_matrix[to_index * surface_count + from_index] =
                fixed_area_factors[to_index * surface_count + from_index] * areas[from_index];
        }
    }
    fixed_area_factors = average_with_transpose(&area_factor_matrix, surface_count);

    if surface_count <= 3 {
        let mut fixed_factors = area_factors_to_view_factors(&fixed_area_factors, areas);
        let row_sum = fixed_factors.iter().sum::<f64>();
        if row_sum > surface_count as f64 + 0.01 {
            let max_surface_sum = max_surface_view_factor_sum(&fixed_factors, surface_count);
            if max_surface_sum > 1.0 {
                for factor in &mut fixed_factors {
                    *factor /= max_surface_sum;
                }
            }
        }
        return fixed_factors;
    }

    let mut convergence_old = 10.0;
    let mut fixed_factors = view_factors.to_vec();
    for _ in 0..400 {
        for from_index in 0..surface_count {
            let column_sum = (0..surface_count)
                .map(|to_index| fixed_area_factors[to_index * surface_count + from_index])
                .sum::<f64>();
            let coefficient = if column_sum.abs() > 1.0e-10 {
                areas[from_index] / column_sum
            } else {
                1.0
            };
            for to_index in 0..surface_count {
                fixed_area_factors[to_index * surface_count + from_index] *= coefficient;
            }
        }

        fixed_area_factors = average_with_transpose(&fixed_area_factors, surface_count);
        fixed_factors = area_factors_to_view_factors(&fixed_area_factors, areas);
        for (view_factor, area_factor) in
            fixed_factors.iter_mut().zip(fixed_area_factors.iter_mut())
        {
            if view_factor.abs() < 1.0e-10 {
                *view_factor = 0.0;
                *area_factor = 0.0;
            }
        }

        let convergence_new = (fixed_factors.iter().sum::<f64>() - surface_count as f64).abs();
        if (convergence_old - convergence_new).abs() < 1.0e-5 || convergence_new <= 0.001 {
            let row_sum = fixed_factors.iter().sum::<f64>();
            if convergence_new < original_check || (row_sum - surface_count as f64).abs() < 0.001 {
                return fixed_factors;
            }
            return view_factors.to_vec();
        }
        convergence_old = convergence_new;
    }

    fixed_factors
}

fn average_with_transpose(matrix: &[f64], surface_count: usize) -> Vec<f64> {
    let mut averaged = vec![0.0; matrix.len()];
    for row in 0..surface_count {
        for col in 0..surface_count {
            averaged[row * surface_count + col] =
                0.5 * (matrix[row * surface_count + col] + matrix[col * surface_count + row]);
        }
    }
    averaged
}

fn area_factors_to_view_factors(area_factors: &[f64], areas: &[f64]) -> Vec<f64> {
    let surface_count = areas.len();
    let mut view_factors = vec![0.0; area_factors.len()];
    for from_index in 0..surface_count {
        if areas[from_index] <= f64::EPSILON {
            continue;
        }
        for to_index in 0..surface_count {
            view_factors[to_index * surface_count + from_index] =
                area_factors[to_index * surface_count + from_index] / areas[from_index];
        }
    }
    view_factors
}

fn max_surface_view_factor_sum(view_factors: &[f64], surface_count: usize) -> f64 {
    (0..surface_count)
        .map(|from_index| {
            (0..surface_count)
                .map(|to_index| view_factors[to_index * surface_count + from_index])
                .sum::<f64>()
        })
        .fold(0.0, f64::max)
}

pub(crate) fn energyplus_scriptf_from_view_factors(
    areas: &[f64],
    view_factors: &[f64],
    emissivities: &mut [f64],
) -> Option<Vec<f64>> {
    let surface_count = areas.len();
    if surface_count == 0
        || view_factors.len() != surface_count * surface_count
        || emissivities.len() != surface_count
    {
        return None;
    }

    let mut coefficient_matrix = vec![0.0; surface_count * surface_count];
    for row in 0..surface_count {
        for col in 0..surface_count {
            coefficient_matrix[row * surface_count + col] =
                areas[row] * view_factors[row * surface_count + col];
        }
    }

    let mut excitation = vec![0.0; surface_count];
    for index in 0..surface_count {
        emissivities[index] = emissivities[index].clamp(0.0, 0.99999);
        let emissivity = emissivities[index];
        let emissivity_area_factor = areas[index] / (1.0 - emissivity);
        excitation[index] = -emissivity * emissivity_area_factor;
        coefficient_matrix[index * surface_count + index] -= emissivity_area_factor;
    }

    let mut inverse = invert_square_matrix(&coefficient_matrix, surface_count)?;
    for col in 0..surface_count {
        for row in 0..surface_count {
            inverse[row * surface_count + col] *= excitation[col];
        }
    }

    let mut script_f = vec![0.0; surface_count * surface_count];
    for receiver_index in 0..surface_count {
        let emissivity = emissivities[receiver_index];
        let emissivity_factor = emissivity / (1.0 - emissivity);
        for sender_index in 0..surface_count {
            let inverse_value = inverse[receiver_index * surface_count + sender_index];
            let script_f_value = if receiver_index == sender_index {
                emissivity_factor * (inverse_value - emissivity)
            } else {
                emissivity_factor * inverse_value
            };
            script_f[sender_index * surface_count + receiver_index] =
                script_f_value * STEFAN_BOLTZMANN_W_PER_M2_K4;
        }
    }
    Some(script_f)
}

fn invert_square_matrix(matrix: &[f64], dimension: usize) -> Option<Vec<f64>> {
    if dimension == 0 || matrix.len() != dimension * dimension {
        return None;
    }

    let augmented_width = dimension * 2;
    let mut augmented = vec![0.0; dimension * augmented_width];
    for row in 0..dimension {
        for col in 0..dimension {
            augmented[row * augmented_width + col] = matrix[row * dimension + col];
        }
        augmented[row * augmented_width + dimension + row] = 1.0;
    }

    for pivot_col in 0..dimension {
        let pivot_row = (pivot_col..dimension).max_by(|left, right| {
            augmented[*left * augmented_width + pivot_col]
                .abs()
                .total_cmp(&augmented[*right * augmented_width + pivot_col].abs())
        })?;
        let pivot = augmented[pivot_row * augmented_width + pivot_col];
        if pivot.abs() <= 1.0e-12 {
            return None;
        }
        if pivot_row != pivot_col {
            for col in 0..augmented_width {
                augmented.swap(
                    pivot_col * augmented_width + col,
                    pivot_row * augmented_width + col,
                );
            }
        }

        let pivot = augmented[pivot_col * augmented_width + pivot_col];
        for col in 0..augmented_width {
            augmented[pivot_col * augmented_width + col] /= pivot;
        }
        for row in 0..dimension {
            if row == pivot_col {
                continue;
            }
            let factor = augmented[row * augmented_width + pivot_col];
            if factor.abs() <= 1.0e-15 {
                continue;
            }
            for col in 0..augmented_width {
                augmented[row * augmented_width + col] -=
                    factor * augmented[pivot_col * augmented_width + col];
            }
        }
    }

    let mut inverse = vec![0.0; dimension * dimension];
    for row in 0..dimension {
        for col in 0..dimension {
            inverse[row * dimension + col] = augmented[row * augmented_width + dimension + col];
        }
    }
    Some(inverse)
}

fn grey_pair_exchange_emissivity(receiver_emissivity: f64, sender_emissivity: f64) -> f64 {
    let receiver = receiver_emissivity.clamp(0.0, 1.0);
    let sender = sender_emissivity.clamp(0.0, 1.0);
    if receiver <= f64::EPSILON || sender <= f64::EPSILON {
        return 0.0;
    }
    1.0 / ((1.0 / receiver) + (1.0 / sender) - 1.0)
}

pub(crate) fn weighted_solar_value(
    previous: f64,
    current: f64,
    next: f64,
    previous_weight: f64,
    current_weight: f64,
    next_weight: f64,
) -> f64 {
    previous.max(0.0) * previous_weight
        + current.max(0.0) * current_weight
        + next.max(0.0) * next_weight
}

pub(crate) fn solar_weather_interpolation_weights(
    zone_steps_per_hour: u32,
    timestep: u32,
) -> (f64, f64, f64) {
    let steps = zone_steps_per_hour.max(1);
    let timestep = timestep.clamp(1, steps);
    let current_weight = solar_interpolation_weight(steps, timestep);
    if steps == 1 {
        return (0.0, current_weight, 0.0);
    }
    let timestep_fraction = 1.0 / f64::from(steps);
    if (current_weight - 1.0).abs() <= f64::EPSILON {
        (0.0, current_weight, 0.0)
    } else if f64::from(timestep) * timestep_fraction < 0.5 {
        (1.0 - current_weight, current_weight, 0.0)
    } else {
        (0.0, current_weight, 1.0 - current_weight)
    }
}

fn solar_interpolation_weight(zone_steps_per_hour: u32, timestep: u32) -> f64 {
    let steps = zone_steps_per_hour.max(1);
    let timestep = timestep.clamp(1, steps);
    if steps.is_multiple_of(2) {
        let halfpoint = steps / 2;
        let distance = timestep.abs_diff(halfpoint);
        return 1.0 - f64::from(distance) / f64::from(steps);
    }

    if steps == 1 {
        0.5
    } else if steps == 3 {
        match timestep {
            1 | 2 => 5.0 / 6.0,
            _ => 0.5,
        }
    } else {
        let timestep_weight = 1.0 / f64::from(steps);
        let halfpoint = steps / 2;
        let peak_weight = 1.0 - timestep_weight / 2.0;
        if timestep == halfpoint || timestep == halfpoint + 1 {
            peak_weight
        } else if timestep > halfpoint + 1 {
            peak_weight - f64::from(timestep - (halfpoint + 1)) * timestep_weight
        } else {
            peak_weight - f64::from(halfpoint - timestep) * timestep_weight
        }
    }
}

pub(crate) fn surface_ground_view_factor(surface: &Surface, tilt_rad: f64) -> f64 {
    match surface.view_factor_to_ground {
        AutoOrNumber::Value(value) => value.clamp(0.0, 1.0),
        AutoOrNumber::AutoCalculate => ((1.0 - tilt_rad.cos()) * 0.5).clamp(0.0, 1.0),
    }
}

pub(crate) fn surface_sky_view_factor(surface: &Surface, tilt_rad: f64) -> f64 {
    match surface.view_factor_to_ground {
        AutoOrNumber::Value(value) => (1.0 - value).clamp(0.0, 1.0),
        AutoOrNumber::AutoCalculate => ((1.0 + tilt_rad.cos()) * 0.5).clamp(0.0, 1.0),
    }
}

pub(crate) fn surface_air_sky_radiation_split(tilt_rad: f64) -> f64 {
    ((1.0 + tilt_rad.cos()) * 0.5)
        .max(0.0)
        .sqrt()
        .clamp(0.0, 1.0)
}

pub(crate) fn solar_position_rad_at_local_hour(
    site: &SiteLocation,
    record: &EpwRecord,
    local_hour: f64,
) -> Option<(f64, f64)> {
    let day = energyplus_weather_record_day_of_year(record)?;
    let (sin_declination, cos_declination, equation_of_time_hours) =
        energyplus_daily_solar_coefficients(day);
    solar_position_rad_from_coefficients(
        site,
        local_hour,
        sin_declination,
        cos_declination,
        equation_of_time_hours,
    )
}

pub(crate) fn solar_position_rad_from_coefficients(
    site: &SiteLocation,
    local_hour: f64,
    sin_declination: f64,
    cos_declination: f64,
    equation_of_time_hours: f64,
) -> Option<(f64, f64)> {
    let latitude_rad = site.latitude_deg.to_radians();
    let sin_latitude = latitude_rad.sin();
    let cos_latitude = latitude_rad.cos();
    let time_zone_meridian_deg = 15.0 * site.time_zone_hours;
    let hour_angle_deg = 15.0 * (12.0 - (local_hour + equation_of_time_hours))
        + (time_zone_meridian_deg - site.longitude_deg);
    let hour_angle_rad = hour_angle_deg.to_radians();

    let cos_zenith =
        sin_declination * sin_latitude + cos_declination * cos_latitude * hour_angle_rad.cos();
    if cos_zenith < ENERGYPLUS_SUN_IS_UP_COS_ZENITH {
        return None;
    }

    let altitude_rad = cos_zenith.clamp(-1.0, 1.0).asin();
    let solar_zenith_rad = cos_zenith.clamp(-1.0, 1.0).acos();
    let azimuth_denominator = cos_latitude * solar_zenith_rad.sin();
    let mut azimuth_rad = if azimuth_denominator.abs() > 1.0e-12 {
        let cos_azimuth = -((sin_latitude * cos_zenith - sin_declination) / azimuth_denominator);
        cos_azimuth.clamp(-1.0, 1.0).acos()
    } else {
        0.0
    };
    if hour_angle_deg < 0.0 {
        azimuth_rad = 2.0 * std::f64::consts::PI - azimuth_rad;
    }

    Some((altitude_rad, azimuth_rad))
}

pub(crate) fn energyplus_shadowing_period_solar_coefficients(
    weather_records: &[EpwRecord],
    record_index: usize,
) -> Option<(f64, f64, f64)> {
    if weather_records.is_empty() {
        return None;
    }

    let total_days = weather_records.len().div_ceil(24);
    let day_of_sim_zero = record_index / 24;
    let period_start_day_zero = (day_of_sim_zero / ENERGYPLUS_SHADOWING_CALC_FREQUENCY_DAYS)
        * ENERGYPLUS_SHADOWING_CALC_FREQUENCY_DAYS;
    let period_length = ENERGYPLUS_SHADOWING_CALC_FREQUENCY_DAYS
        .min(total_days.saturating_sub(period_start_day_zero))
        .max(1);
    let period_start_record = weather_records.get(period_start_day_zero * 24)?;
    let period_start_day_of_year = energyplus_weather_record_day_of_year(period_start_record)?;

    Some(energyplus_average_solar_coefficients(
        period_start_day_of_year,
        period_length,
    ))
}

pub(crate) fn energyplus_average_solar_coefficients(
    start_day_of_year: u32,
    day_count: usize,
) -> (f64, f64, f64) {
    let day_count = day_count.max(1);
    let mut sin_declination_sum = 0.0;
    let mut equation_of_time_sum = 0.0;
    for offset in 0..day_count {
        let (sin_declination, _cos_declination, equation_of_time_hours) =
            energyplus_daily_solar_coefficients(start_day_of_year + offset as u32);
        sin_declination_sum += sin_declination;
        equation_of_time_sum += equation_of_time_hours;
    }

    let sin_declination = sin_declination_sum / day_count as f64;
    let cos_declination = (1.0 - sin_declination.powi(2)).sqrt();
    let equation_of_time_hours = equation_of_time_sum / day_count as f64;

    (sin_declination, cos_declination, equation_of_time_hours)
}

pub(crate) fn energyplus_weather_record_day_of_year(record: &EpwRecord) -> Option<u32> {
    day_of_year(DEFAULT_RUN_PERIOD_YEAR, record.month, record.day)
}

pub(crate) fn energyplus_daily_solar_coefficients(day_of_year: u32) -> (f64, f64, f64) {
    const SINE_SOLAR_DECLINATION_COEFFICIENTS: [f64; 9] = [
        0.00561800,
        0.0657911,
        -0.392779,
        0.00064440,
        -0.00618495,
        -0.00010101,
        -0.00007951,
        -0.00011691,
        0.00002096,
    ];
    const EQUATION_OF_TIME_COEFFICIENTS: [f64; 9] = [
        0.00021971,
        -0.122649,
        0.00762856,
        -0.156308,
        -0.0530028,
        -0.00388702,
        -0.00123978,
        -0.00270502,
        -0.00167992,
    ];

    let angle = 2.0 * std::f64::consts::PI * f64::from(day_of_year) / 366.0;
    let sin_x = angle.sin();
    let cos_x = angle.cos();
    let sin_2x = sin_x * cos_x * 2.0;
    let cos_2x = cos_x.powi(2) - sin_x.powi(2);
    let sin_3x = sin_x * cos_2x + cos_x * sin_2x;
    let cos_3x = cos_x * cos_2x - sin_x * sin_2x;
    let sin_4x = 2.0 * sin_2x * cos_2x;
    let cos_4x = cos_2x.powi(2) - sin_2x.powi(2);
    let basis = [
        1.0, sin_x, cos_x, sin_2x, cos_2x, sin_3x, cos_3x, sin_4x, cos_4x,
    ];

    let sin_declination = SINE_SOLAR_DECLINATION_COEFFICIENTS
        .iter()
        .zip(basis)
        .map(|(coefficient, term)| coefficient * term)
        .sum::<f64>();
    let cos_declination = (1.0 - sin_declination.powi(2)).sqrt();
    let equation_of_time_hours = EQUATION_OF_TIME_COEFFICIENTS
        .iter()
        .zip(basis)
        .map(|(coefficient, term)| coefficient * term)
        .sum::<f64>();

    (sin_declination, cos_declination, equation_of_time_hours)
}

pub(crate) fn surface_incident_solar_components_hourly_average_w_per_m2(
    surface: &Surface,
    site: &SiteLocation,
    weather_records: &[EpwRecord],
    record_index: usize,
    zone_steps_per_hour: u32,
) -> SurfaceIncidentSolarComponents {
    surface_incident_solar_components_for_weather_context_w_per_m2(
        surface,
        site,
        weather_records,
        record_index,
        zone_steps_per_hour,
        None,
        FirstHourInterpolationStartingValues::Hour24,
    )
}

pub(crate) fn surface_incident_solar_radiation_for_weather_context_w_per_m2(
    surface: &Surface,
    site: &SiteLocation,
    weather_records: &[EpwRecord],
    record_index: usize,
    zone_steps_per_hour: u32,
    zone_timestep: Option<u32>,
    first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
) -> f64 {
    surface_incident_solar_components_for_weather_context_w_per_m2(
        surface,
        site,
        weather_records,
        record_index,
        zone_steps_per_hour,
        zone_timestep,
        first_hour_interpolation_starting_values,
    )
    .total_w_per_m2()
}

fn surface_incident_solar_components_for_weather_context_w_per_m2(
    surface: &Surface,
    site: &SiteLocation,
    weather_records: &[EpwRecord],
    record_index: usize,
    zone_steps_per_hour: u32,
    zone_timestep: Option<u32>,
    first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
) -> SurfaceIncidentSolarComponents {
    if weather_records.get(record_index).is_none() {
        return SurfaceIncidentSolarComponents::default();
    }
    let Some((sin_declination, cos_declination, equation_of_time_hours)) =
        energyplus_shadowing_period_solar_coefficients(weather_records, record_index)
    else {
        return SurfaceIncidentSolarComponents::default();
    };
    let steps = zone_steps_per_hour.max(1);
    if let Some(timestep) = zone_timestep {
        return surface_incident_solar_components_at_weather_timestep_w_per_m2(
            surface,
            site,
            weather_records,
            record_index,
            steps,
            timestep,
            first_hour_interpolation_starting_values,
            sin_declination,
            cos_declination,
            equation_of_time_hours,
        );
    }

    let mut components = SurfaceIncidentSolarComponents::default();
    for timestep in 1..=steps {
        let timestep_components = surface_incident_solar_components_at_weather_timestep_w_per_m2(
            surface,
            site,
            weather_records,
            record_index,
            steps,
            timestep,
            first_hour_interpolation_starting_values,
            sin_declination,
            cos_declination,
            equation_of_time_hours,
        );
        components.beam_w_per_m2 += timestep_components.beam_w_per_m2;
        components.sky_diffuse_w_per_m2 += timestep_components.sky_diffuse_w_per_m2;
        components.ground_diffuse_w_per_m2 += timestep_components.ground_diffuse_w_per_m2;
    }

    let divisor = f64::from(steps);
    SurfaceIncidentSolarComponents {
        beam_w_per_m2: components.beam_w_per_m2 / divisor,
        sky_diffuse_w_per_m2: components.sky_diffuse_w_per_m2 / divisor,
        ground_diffuse_w_per_m2: components.ground_diffuse_w_per_m2 / divisor,
    }
}

fn surface_incident_solar_components_at_weather_timestep_w_per_m2(
    surface: &Surface,
    site: &SiteLocation,
    weather_records: &[EpwRecord],
    record_index: usize,
    zone_steps_per_hour: u32,
    zone_timestep: u32,
    first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
    sin_declination: f64,
    cos_declination: f64,
    equation_of_time_hours: f64,
) -> SurfaceIncidentSolarComponents {
    let Some(record) = weather_records.get(record_index) else {
        return SurfaceIncidentSolarComponents::default();
    };
    let steps = zone_steps_per_hour.max(1);
    let timestep = zone_timestep.clamp(1, steps);
    let (previous_weight, current_weight, next_weight) =
        solar_weather_interpolation_weights(steps, timestep);
    let previous = previous_weather_record_with_first_hour_starting_values(
        weather_records,
        record_index,
        first_hour_interpolation_starting_values,
    );
    let next = next_weather_record(weather_records, record_index);
    let direct_normal = weighted_solar_value(
        previous.direct_normal_radiation_wh_per_m2,
        record.direct_normal_radiation_wh_per_m2,
        next.direct_normal_radiation_wh_per_m2,
        previous_weight,
        current_weight,
        next_weight,
    );
    let diffuse_horizontal = weighted_solar_value(
        previous.diffuse_horizontal_radiation_wh_per_m2,
        record.diffuse_horizontal_radiation_wh_per_m2,
        next.diffuse_horizontal_radiation_wh_per_m2,
        previous_weight,
        current_weight,
        next_weight,
    );
    let local_hour =
        f64::from(record.hour.saturating_sub(1)) + f64::from(timestep) / f64::from(steps);
    let actual_solar_position_rad = solar_position_rad_at_local_hour(site, record, local_hour);

    surface_incident_solar_components_at_local_hour_w_per_m2(
        surface,
        site,
        SurfaceSolarTimestepInput {
            local_hour,
            actual_solar_position_rad,
            sin_declination,
            cos_declination,
            equation_of_time_hours,
            direct_normal_radiation_w_per_m2: direct_normal,
            diffuse_horizontal_radiation_w_per_m2: diffuse_horizontal,
        },
    )
}

#[derive(Clone, Copy)]
struct SurfaceSolarTimestepInput {
    local_hour: f64,
    actual_solar_position_rad: Option<(f64, f64)>,
    sin_declination: f64,
    cos_declination: f64,
    equation_of_time_hours: f64,
    direct_normal_radiation_w_per_m2: f64,
    diffuse_horizontal_radiation_w_per_m2: f64,
}

fn surface_incident_solar_components_at_local_hour_w_per_m2(
    surface: &Surface,
    site: &SiteLocation,
    input: SurfaceSolarTimestepInput,
) -> SurfaceIncidentSolarComponents {
    let Some((actual_solar_altitude_rad, actual_solar_azimuth_rad)) =
        input.actual_solar_position_rad
    else {
        return SurfaceIncidentSolarComponents::default();
    };

    let tilt_rad = surface_tilt_deg(surface.surface_type, &surface.vertices).to_radians();
    let direct_normal = input.direct_normal_radiation_w_per_m2.max(0.0);
    let diffuse_horizontal = input.diffuse_horizontal_radiation_w_per_m2.max(0.0);

    // EnergyPlus reports beam with the shadowing-period SurfCosIncAng table,
    // while Perez sky diffuse and ground-reflected solar use current SOLCOS.
    let shadowing_period_solar_position_rad = solar_position_rad_from_coefficients(
        site,
        input.local_hour,
        input.sin_declination,
        input.cos_declination,
        input.equation_of_time_hours,
    );

    let surface_azimuth_rad = surface_azimuth_deg(&surface.vertices).to_radians();

    let shadowing_period_cos_incidence =
        shadowing_period_solar_position_rad.map(|(solar_altitude_rad, solar_azimuth_rad)| {
            solar_altitude_rad.sin() * tilt_rad.cos()
                + solar_altitude_rad.cos()
                    * tilt_rad.sin()
                    * (solar_azimuth_rad - surface_azimuth_rad).cos()
        });
    let beam = shadowing_period_solar_position_rad
        .zip(shadowing_period_cos_incidence)
        .filter(
            |((solar_altitude_rad, _solar_azimuth_rad), _cos_incidence)| *solar_altitude_rad > 0.0,
        )
        .map(
            |((_solar_altitude_rad, _solar_azimuth_rad), cos_incidence)| {
                direct_normal * cos_incidence.max(0.0)
            },
        )
        .unwrap_or(0.0);
    let actual_cos_incidence = actual_solar_altitude_rad.sin() * tilt_rad.cos()
        + actual_solar_altitude_rad.cos()
            * tilt_rad.sin()
            * (actual_solar_azimuth_rad - surface_azimuth_rad).cos();
    let circumsolar_sunlit_fraction = shadowing_period_cos_incidence
        .map(|cos_incidence| {
            if cos_incidence > ENERGYPLUS_SUN_IS_UP_COS_ZENITH {
                1.0
            } else {
                0.0
            }
        })
        .unwrap_or(0.0);
    let sky_diffuse = diffuse_horizontal
        * energyplus_anisotropic_sky_multiplier(
            surface,
            site,
            tilt_rad,
            actual_solar_altitude_rad,
            direct_normal,
            diffuse_horizontal,
            actual_cos_incidence,
            circumsolar_sunlit_fraction,
        );
    let ground_horizontal =
        (direct_normal * actual_solar_altitude_rad.sin() + diffuse_horizontal).max(0.0);
    let ground_reflected = ground_horizontal
        * DEFAULT_SOLAR_GROUND_REFLECTANCE
        * surface_ground_view_factor(surface, tilt_rad);

    SurfaceIncidentSolarComponents {
        beam_w_per_m2: beam,
        sky_diffuse_w_per_m2: sky_diffuse,
        ground_diffuse_w_per_m2: ground_reflected,
    }
}

pub(crate) fn energyplus_anisotropic_sky_multiplier(
    surface: &Surface,
    site: &SiteLocation,
    tilt_rad: f64,
    solar_altitude_rad: f64,
    direct_normal_w_per_m2: f64,
    diffuse_horizontal_w_per_m2: f64,
    cos_incidence: f64,
    circumsolar_sunlit_fraction: f64,
) -> f64 {
    const EPSILON_LIMIT: [f64; 7] = [1.065, 1.23, 1.5, 1.95, 2.8, 4.5, 6.2];
    const F11R: [f64; 8] = [
        -0.0083117, 0.1299457, 0.3296958, 0.5682053, 0.8730280, 1.1326077, 1.0601591, 0.6777470,
    ];
    const F12R: [f64; 8] = [
        0.5877285, 0.6825954, 0.4868735, 0.1874525, -0.3920403, -1.2367284, -1.5999137, -0.3272588,
    ];
    const F13R: [f64; 8] = [
        -0.0620636, -0.1513752, -0.2210958, -0.2951290, -0.3616149, -0.4118494, -0.3589221,
        -0.2504286,
    ];
    const F21R: [f64; 8] = [
        -0.0596012, -0.0189325, 0.0554140, 0.1088631, 0.2255647, 0.2877813, 0.2642124, 0.1561313,
    ];
    const F22R: [f64; 8] = [
        0.0721249, 0.0659650, -0.0639588, -0.1519229, -0.4620442, -0.8230357, -1.1272340,
        -1.3765031,
    ];
    const F23R: [f64; 8] = [
        -0.0220216, -0.0288748, -0.0260542, -0.0139754, 0.0012448, 0.0558651, 0.1310694, 0.2506212,
    ];

    let diffuse_horizontal = diffuse_horizontal_w_per_m2.max(0.0);
    if diffuse_horizontal <= f64::EPSILON {
        return surface_sky_view_factor(surface, tilt_rad);
    }

    let direct_normal = direct_normal_w_per_m2.max(0.0);
    let cos_zenith = solar_altitude_rad.sin().clamp(0.0, 1.0);
    if cos_zenith < ENERGYPLUS_SUN_IS_UP_COS_ZENITH {
        return surface_sky_view_factor(surface, tilt_rad);
    }

    let zenith_rad = cos_zenith.acos();
    let zenith_deg = zenith_rad.to_degrees();
    let air_mass_height = 1.0 - 0.1 * site.elevation_m / 1000.0;
    let air_mass = if zenith_deg <= 75.0 {
        air_mass_height / cos_zenith
    } else {
        air_mass_height / (cos_zenith + 0.15 * (93.9 - zenith_deg).powf(-1.253))
    };
    let kappa_z3 = 1.041 * zenith_rad.powi(3);
    let epsilon =
        ((direct_normal + diffuse_horizontal) / diffuse_horizontal + kappa_z3) / (1.0 + kappa_z3);
    let delta = diffuse_horizontal * air_mass / 1353.0;
    let epsilon_bin = EPSILON_LIMIT
        .iter()
        .position(|limit| epsilon < *limit)
        .unwrap_or(F11R.len() - 1);
    let f1 =
        (F11R[epsilon_bin] + F12R[epsilon_bin] * delta + F13R[epsilon_bin] * zenith_rad).max(0.0);
    let f2 = F21R[epsilon_bin] + F22R[epsilon_bin] * delta + F23R[epsilon_bin] * zenith_rad;

    let mut circumsolar_factor = cos_incidence.max(0.0) / cos_zenith.max(0.0871557);
    if circumsolar_factor > 0.0 && cos_zenith < 0.0871557 && tilt_rad.to_degrees() < 2.0 {
        circumsolar_factor = 1.0;
    }

    let view_factor_sky = surface_sky_view_factor(surface, tilt_rad);
    let multiplier = view_factor_sky * (1.0 - f1)
        + f1 * circumsolar_factor * circumsolar_sunlit_fraction.clamp(0.0, 1.0)
        + f2 * tilt_rad.sin();
    multiplier.max(0.0)
}
