//! Heat-balance radiation source-order ownership notes.

use crate::heat_balance::state::SurfaceHeatBalanceState;
use ep_model::{SurfaceId, SurfaceType, ZoneId};
use std::collections::BTreeMap;

/// Current longwave/solar source-order owner for outside-face inputs.
pub const EXTERIOR_RADIATION_OWNER_STAGE: &str = "CalcHeatBalanceOutsideSurf";

/// Current longwave source-order owner for inside-face inputs.
pub const INTERIOR_RADIATION_OWNER_STAGE: &str = "CalcHeatBalanceInsideSurf";

pub(crate) const STEFAN_BOLTZMANN_W_PER_M2_K4: f64 = 5.6697e-8;
pub(crate) const KELVIN_OFFSET: f64 = 273.15;

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
