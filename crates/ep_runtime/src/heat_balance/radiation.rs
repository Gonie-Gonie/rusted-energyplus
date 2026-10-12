//! Heat-balance radiation source-order ownership notes.

use crate::heat_balance::HeatBalanceSurfaceIndexes;
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

/// Actual selected diagnostic probe inputs are outside the implemented scope.
/// This is a Rust unavailable boundary, never a substitute Original fatal.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InteriorRadiationUnavailable {
    /// The zone surface domain is incomplete, duplicated or inconsistently associated.
    SurfaceLayout,
    /// A real probe operand is nonfinite or outside its admitted physical domain.
    SurfaceInput {
        /// Actual typed surface carrying the unsupported operand.
        surface_id: SurfaceId,
    },
    /// A real zone requires an unported Fix/ScriptF numeric or diagnostic branch.
    ZoneNumericDomain {
        /// Actual zero-based caller zone index; no enclosure owner is inferred.
        zone_index: usize,
    },
    /// A computed probe flux is nonfinite; no staged surface flux was committed.
    SurfaceOutput,
}

impl std::fmt::Display for InteriorRadiationUnavailable {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "selected interior radiation unavailable: {self:?}"
        )
    }
}
impl std::error::Error for InteriorRadiationUnavailable {}

fn validate_radiation_probe_inputs(
    surfaces: &[SurfaceHeatBalanceState],
    indexes: &HeatBalanceSurfaceIndexes,
    temperature_overrides: Option<&BTreeMap<SurfaceId, f64>>,
) -> Result<(), InteriorRadiationUnavailable> {
    let mut seen = vec![false; surfaces.len()];
    for (zone_index, indices) in indexes.surfaces_by_zone.iter().enumerate() {
        if indices.len().checked_mul(indices.len()).is_none() {
            return Err(InteriorRadiationUnavailable::SurfaceLayout);
        }
        for index in indices {
            let surface = surfaces
                .get(*index)
                .ok_or(InteriorRadiationUnavailable::SurfaceLayout)?;
            if seen[*index] || usize::try_from(surface.zone_id.0).ok() != Some(zone_index) {
                return Err(InteriorRadiationUnavailable::SurfaceLayout);
            }
            seen[*index] = true;
            let temperature = temperature_overrides
                .and_then(|values| values.get(&surface.surface_id).copied())
                .unwrap_or(surface.inside_face_temperature_c);
            if !surface.area_m2.is_finite()
                || surface.area_m2 <= 0.0
                || !surface.azimuth_deg.is_finite()
                || !surface.tilt_deg.is_finite()
                || !surface.inside_thermal_absorptance.is_finite()
                || !(0.0..=1.0).contains(&surface.inside_thermal_absorptance)
                || !temperature.is_finite()
                || !(temperature + KELVIN_OFFSET).max(0.0).powi(4).is_finite()
            {
                return Err(InteriorRadiationUnavailable::SurfaceInput {
                    surface_id: surface.surface_id,
                });
            }
        }
    }
    if seen.iter().any(|present| !present) {
        return Err(InteriorRadiationUnavailable::SurfaceLayout);
    }
    Ok(())
}

pub(crate) fn update_surface_inside_longwave_exchange_probe(
    surfaces: &mut [SurfaceHeatBalanceState],
    surface_indexes: &HeatBalanceSurfaceIndexes,
    temperature_overrides: Option<&BTreeMap<SurfaceId, f64>>,
) -> Result<(), InteriorRadiationUnavailable> {
    validate_radiation_probe_inputs(surfaces, surface_indexes, temperature_overrides)?;
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

    let mut longwave_terms_w_per_m2 = vec![0.0; surfaces.len()];
    for (zone_index, surface_indices) in surface_indexes.surfaces_by_zone.iter().enumerate() {
        if surface_indices.is_empty() {
            continue;
        }
        // InitInteriorRadExchange576-588 explicitly initializes a one-surface
        // enclosure to zero without Fix/ScriptF; its net self-exchange is zero.
        // Keep this known caller branch separate from unsupported numeric input.
        if surface_indices.len() == 1 {
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
        let Some(view_factors) = fix_energyplus_approximate_view_factors(
            &areas,
            &energyplus_approximate_view_factors(&zone_snapshots),
        ) else {
            return Err(InteriorRadiationUnavailable::ZoneNumericDomain { zone_index });
        };
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

    if longwave_terms_w_per_m2
        .iter()
        .any(|value| !value.is_finite())
    {
        return Err(InteriorRadiationUnavailable::SurfaceOutput);
    }
    for (surface, net_longwave_w_per_m2) in surfaces.iter_mut().zip(longwave_terms_w_per_m2) {
        surface.inside_net_longwave_w_per_m2 = net_longwave_w_per_m2;
    }
    Ok(())
}

pub(crate) fn update_surface_inside_scriptf_longwave_exchange_probe(
    surfaces: &mut [SurfaceHeatBalanceState],
    surface_indexes: &HeatBalanceSurfaceIndexes,
    temperature_overrides: Option<&BTreeMap<SurfaceId, f64>>,
) -> Result<(), InteriorRadiationUnavailable> {
    update_surface_inside_scriptf_longwave_exchange_probe_with_access(
        surfaces,
        surface_indexes,
        temperature_overrides,
        false,
    )
}

pub(crate) fn update_surface_inside_scriptf_flat_access_longwave_exchange_probe(
    surfaces: &mut [SurfaceHeatBalanceState],
    surface_indexes: &HeatBalanceSurfaceIndexes,
    temperature_overrides: Option<&BTreeMap<SurfaceId, f64>>,
) -> Result<(), InteriorRadiationUnavailable> {
    update_surface_inside_scriptf_longwave_exchange_probe_with_access(
        surfaces,
        surface_indexes,
        temperature_overrides,
        true,
    )
}

fn update_surface_inside_scriptf_longwave_exchange_probe_with_access(
    surfaces: &mut [SurfaceHeatBalanceState],
    surface_indexes: &HeatBalanceSurfaceIndexes,
    temperature_overrides: Option<&BTreeMap<SurfaceId, f64>>,
    use_energyplus_flat_lsr_access: bool,
) -> Result<(), InteriorRadiationUnavailable> {
    validate_radiation_probe_inputs(surfaces, surface_indexes, temperature_overrides)?;
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

    let mut longwave_terms_w_per_m2 = vec![0.0; surfaces.len()];
    for (zone_index, surface_indices) in surface_indexes.surfaces_by_zone.iter().enumerate() {
        if surface_indices.is_empty() {
            continue;
        }
        // InitInteriorRadExchange576-588 explicitly initializes a one-surface
        // enclosure to zero without Fix/ScriptF; its net self-exchange is zero.
        // Keep this known caller branch separate from unsupported numeric input.
        if surface_indices.len() == 1 {
            continue;
        }
        let zone_snapshots = surface_indices
            .iter()
            .map(|surface_index| snapshots[*surface_index])
            .collect::<Vec<_>>();
        let script_f = energyplus_scriptf_longwave_matrix_w_per_m2_k4(&zone_snapshots)
            .ok_or(InteriorRadiationUnavailable::ZoneNumericDomain { zone_index })?;
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

    if longwave_terms_w_per_m2
        .iter()
        .any(|value| !value.is_finite())
    {
        return Err(InteriorRadiationUnavailable::SurfaceOutput);
    }
    for (surface, net_longwave_w_per_m2) in surfaces.iter_mut().zip(longwave_terms_w_per_m2) {
        surface.inside_net_longwave_w_per_m2 = net_longwave_w_per_m2;
    }
    Ok(())
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
    )?;
    let mut emissivities = surfaces
        .iter()
        .map(|surface| surface.thermal_absorptance)
        .collect::<Vec<_>>();
    let mut script_f =
        energyplus_scriptf_from_view_factors(&areas, &direct_view_factors, &mut emissivities)?;
    // CalcScriptF returns dimensionless coefficients. This existing probe is
    // the consumed scaling boundary, not the unported ordinary cache owner.
    for value in &mut script_f {
        *value *= STEFAN_BOLTZMANN_W_PER_M2_K4;
    }
    Some(script_f)
}

/// Caller-owned identity of the selected single-zone enclosure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ApproximateViewFactorContext {
    /// Actual caller enclosure index, copied without sorting or index conversion.
    pub enclosure_index: usize,
    /// Actual typed zone associated with every selected enclosure surface.
    pub zone_id: ZoneId,
}

/// One actual surface in the caller's enclosure pointer order.
#[derive(Clone, Debug, PartialEq)]
pub struct ApproximateViewFactorSurface {
    /// Actual typed surface identity; never a position reconstructed from its name.
    pub surface_id: SurfaceId,
    /// Actual surface name used by the source warning continuation.
    pub surface_name: String,
    /// Actual zone association of this surface.
    pub zone_id: ZoneId,
    /// Actual zone name used by the source warning continuation.
    pub zone_name: String,
    /// Selected opaque class; Ceiling and Roof both use the source non-floor path.
    pub surface_type: SurfaceType,
    /// Actual initialized enclosure area in square meters, without a clamp.
    pub area_m2: f64,
    /// Actual initialized azimuth in degrees, without angle normalization.
    pub azimuth_deg: f64,
    /// Actual initialized tilt in degrees, without geometry reconstruction.
    pub tilt_deg: f64,
}

/// One source zero-seen-area warning followed by its surface/zone continuation.
///
/// Vector order is the source outer-loop order. This record describes those two
/// diagnostic calls; it does not claim a global message index or backend delivery.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApproximateViewFactorWarning {
    /// Zero-based position in the unchanged input surface slice.
    pub surface_index: usize,
    /// Actual surface identity retained at that position.
    pub surface_id: SurfaceId,
    /// Actual surface name for `Happens for Surface=...`.
    pub surface_name: String,
    /// Actual zone identity of the warned surface.
    pub zone_id: ZoneId,
    /// Actual zone name for the continuation call.
    pub zone_name: String,
}

/// Actual outputs of one selected approximate-view-factor helper call.
#[derive(Clone, Debug, PartialEq)]
pub struct ApproximateViewFactors {
    /// Unchanged caller identity; this helper does not construct an enclosure.
    pub context: ApproximateViewFactorContext,
    /// Actual surface IDs in input order, shared by both matrix axes.
    pub ordered_surface_ids: Vec<SurfaceId>,
    /// Actual `ZoneArea(i)` accumulation before its source-local deallocation.
    pub zone_area_seen_m2: Vec<f64>,
    /// Actual `F(j,i)` in row-first storage: `j * N + i` with zero-based indices.
    pub view_factors: Vec<f64>,
    /// Ordered zero-seen-area warning/continuation records; no severity fallback.
    pub warnings: Vec<ApproximateViewFactorWarning>,
}

/// Association/representation errors outside the selected single-zone API.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApproximateViewFactorScopeError {
    /// The actual surface is not associated with the supplied selected zone.
    ZoneMismatch {
        /// Position carrying the mismatched actual zone.
        surface_index: usize,
    },
    /// Two input positions claim the same actual surface identity.
    DuplicateSurfaceId {
        /// First occurrence of this actual surface identity.
        first_index: usize,
        /// Later occurrence of the same identity.
        duplicate_index: usize,
    },
    /// The square buffer length cannot be represented by this Rust API.
    MatrixSizeOverflow,
}

impl std::fmt::Display for ApproximateViewFactorScopeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "selected approximate view factors unavailable: {self:?}"
        )
    }
}

impl std::error::Error for ApproximateViewFactorScopeError {}

/// Evaluate the pinned `CalcApproximateViewFactors` opaque helper once.
///
/// The caller supplies actual initialized geometry and enclosure pointer order.
/// This API does not select solar/radiant reuse, repair view factors, calculate
/// ScriptF, emit diagnostics, initialize a runtime cache or certify CON admission.
/// Diagnostic unit inputs may exercise zero/negative areas; no finite-value guard
/// or epsilon replacement changes the source arithmetic/comparison semantics.
pub fn calculate_selected_approximate_view_factors(
    context: ApproximateViewFactorContext,
    surfaces: &[ApproximateViewFactorSurface],
) -> Result<ApproximateViewFactors, ApproximateViewFactorScopeError> {
    let matrix_size = surfaces
        .len()
        .checked_mul(surfaces.len())
        .ok_or(ApproximateViewFactorScopeError::MatrixSizeOverflow)?;
    for (index, surface) in surfaces.iter().enumerate() {
        if surface.zone_id != context.zone_id {
            return Err(ApproximateViewFactorScopeError::ZoneMismatch {
                surface_index: index,
            });
        }
        if let Some(first_index) = surfaces[..index]
            .iter()
            .position(|prior| prior.surface_id == surface.surface_id)
        {
            return Err(ApproximateViewFactorScopeError::DuplicateSurfaceId {
                first_index,
                duplicate_index: index,
            });
        }
    }
    let geometry = surfaces
        .iter()
        .map(|surface| ViewFactorGeometry {
            surface_type: surface.surface_type,
            area_m2: surface.area_m2,
            azimuth_deg: surface.azimuth_deg,
            tilt_deg: surface.tilt_deg,
        })
        .collect::<Vec<_>>();
    let actual = populate_approximate_view_factors(&geometry, matrix_size);
    let warnings = actual
        .warning_indices
        .into_iter()
        .map(|surface_index| {
            let surface = &surfaces[surface_index];
            ApproximateViewFactorWarning {
                surface_index,
                surface_id: surface.surface_id,
                surface_name: surface.surface_name.clone(),
                zone_id: surface.zone_id,
                zone_name: surface.zone_name.clone(),
            }
        })
        .collect();
    Ok(ApproximateViewFactors {
        context,
        ordered_surface_ids: surfaces.iter().map(|surface| surface.surface_id).collect(),
        zone_area_seen_m2: actual.zone_area_seen_m2,
        view_factors: actual.view_factors,
        warnings,
    })
}

#[derive(Clone, Copy)]
struct ViewFactorGeometry {
    surface_type: SurfaceType,
    area_m2: f64,
    azimuth_deg: f64,
    tilt_deg: f64,
}

struct ApproximateViewFactorKernel {
    zone_area_seen_m2: Vec<f64>,
    view_factors: Vec<f64>,
    warning_indices: Vec<usize>,
}

pub(crate) fn energyplus_approximate_view_factors(
    surfaces: &[InteriorLongwaveSurfaceSnapshot],
) -> Vec<f64> {
    let geometry = surfaces
        .iter()
        .map(|surface| ViewFactorGeometry {
            surface_type: surface.surface_type,
            area_m2: surface.area_m2,
            azimuth_deg: surface.azimuth_deg,
            tilt_deg: surface.tilt_deg,
        })
        .collect::<Vec<_>>();
    // The legacy probe has no actual IDs/names or diagnostic backend. It borrows
    // the same arithmetic, without inventing metadata for the new public owner.
    populate_approximate_view_factors(&geometry, surfaces.len() * surfaces.len()).view_factors
}

fn populate_approximate_view_factors(
    surfaces: &[ViewFactorGeometry],
    matrix_size: usize,
) -> ApproximateViewFactorKernel {
    let surface_count = surfaces.len();
    let mut zone_area_seen_m2 = vec![0.0; surface_count];
    let mut warning_indices = Vec::new();
    for (from_index, from_surface) in surfaces.iter().enumerate() {
        for (to_index, to_surface) in surfaces.iter().enumerate() {
            if energyplus_surface_sees_surface(from_surface, to_surface, from_index, to_index) {
                zone_area_seen_m2[from_index] += to_surface.area_m2;
            }
        }
        if zone_area_seen_m2[from_index] <= 0.0 {
            warning_indices.push(from_index);
        }
    }

    let mut view_factors = vec![0.0; matrix_size];
    for (from_index, from_surface) in surfaces.iter().enumerate() {
        for (to_index, to_surface) in surfaces.iter().enumerate() {
            if energyplus_surface_sees_surface(from_surface, to_surface, from_index, to_index)
                && zone_area_seen_m2[from_index] > 0.0
            {
                view_factors[to_index * surface_count + from_index] =
                    to_surface.area_m2 / zone_area_seen_m2[from_index];
            }
        }
    }
    ApproximateViewFactorKernel {
        zone_area_seen_m2,
        view_factors,
        warning_indices,
    }
}

fn energyplus_surface_sees_surface(
    from_surface: &ViewFactorGeometry,
    to_surface: &ViewFactorGeometry,
    from_index: usize,
    to_index: usize,
) -> bool {
    if from_index == to_index
        || (from_surface.surface_type == SurfaceType::Floor
            && to_surface.surface_type == SurfaceType::Floor)
    {
        return false;
    }

    // IntMass is absent from the CON-selected typed SurfaceType domain.
    to_surface.surface_type == SurfaceType::Floor
        || from_surface.surface_type == SurfaceType::Floor
        || ((from_surface.azimuth_deg - to_surface.azimuth_deg).abs() > 10.0
            && (from_surface.azimuth_deg - to_surface.azimuth_deg).abs() < 350.0)
        || (from_surface.tilt_deg - to_surface.tilt_deg).abs() > 10.0
}

/// Selected warning-free balanced N>3 numeric branch of Original FixViewFactors.
/// Diagnostic and persistent-owner branches are explicitly unavailable here.
pub(crate) fn fix_energyplus_approximate_view_factors(
    areas: &[f64],
    view_factors: &[f64],
) -> Option<Vec<f64>> {
    let surface_count = areas.len();
    let extent = surface_count.checked_mul(surface_count)?;
    if surface_count <= 3
        || view_factors.len() != extent
        || areas.iter().any(|area| !area.is_finite() || *area <= 0.0)
        || view_factors.iter().any(|value| !value.is_finite())
    {
        return None;
    }
    let original_check = (view_factors.iter().sum::<f64>() - surface_count as f64).abs();
    let total_area = areas.iter().sum::<f64>();
    let mut largest_area = areas[0];
    for area in &areas[1..] {
        if *area > largest_area {
            largest_area = *area;
        }
    }
    // The Original unbalanced branch writes persistent LargestSurf; the small
    // enclosure branch emits diagnostics and commits Zone.EnforcedReciprocity.
    // Neither owner is represented by this existing private numeric interface.
    if !total_area.is_finite() || largest_area > 0.99 * (total_area - largest_area) {
        return None;
    }
    let mut area_factors = vec![0.0; extent];
    for i in 0..surface_count {
        for j in 0..surface_count {
            area_factors[j * surface_count + i] = view_factors[j * surface_count + i] * areas[i];
        }
    }
    let mut fixed_area_factors = average_with_transpose(&area_factors, surface_count);
    let mut convergence_old = 10.0;
    let mut iterations = 0;
    loop {
        iterations += 1;
        for i in 0..surface_count {
            let sum = (0..surface_count)
                .map(|j| fixed_area_factors[j * surface_count + i])
                .sum::<f64>();
            let coefficient = if sum.abs() > 1.0e-10 {
                areas[i] / sum
            } else {
                1.0
            };
            for j in 0..surface_count {
                fixed_area_factors[j * surface_count + i] *= coefficient;
            }
        }
        fixed_area_factors = average_with_transpose(&fixed_area_factors, surface_count);
        let mut fixed_factors = area_factors_to_view_factors(&fixed_area_factors, areas);
        for i in 0..surface_count {
            for j in 0..surface_count {
                let offset = j * surface_count + i;
                if fixed_factors[offset].abs() < 1.0e-10 {
                    fixed_factors[offset] = 0.0;
                    fixed_area_factors[offset] = 0.0;
                }
            }
        }
        let convergence_new = (fixed_factors.iter().sum::<f64>() - surface_count as f64).abs();
        let converged =
            (convergence_old - convergence_new).abs() < 1.0e-5 || convergence_new <= 0.001;
        convergence_old = convergence_new;
        // Original increments before work, then tests >400 after convergence.
        // Its iteration401 diagnostic/failure path is an unavailable boundary.
        if iterations > 400 || fixed_factors.iter().any(|value| !value.is_finite()) {
            return None;
        }
        if converged {
            if convergence_new < original_check
                || (fixed_factors.iter().sum::<f64>() - surface_count as f64).abs() < 0.001
            {
                return Some(fixed_factors);
            }
            // Original emits a completeness warning for the remaining branch.
            return None;
        }
    }
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
        for to_index in 0..surface_count {
            view_factors[to_index * surface_count + from_index] =
                area_factors[to_index * surface_count + from_index] / areas[from_index];
        }
    }
    view_factors
}

/// Literal dimensionless CalcScriptF flat walks on the admitted finite domain.
/// Above-limit emissivity requires an unported warning/mutation owner and is
/// rejected before mutation; this helper does not activate ordinary radiation.
pub(crate) fn energyplus_scriptf_from_view_factors(
    areas: &[f64],
    view_factors: &[f64],
    emissivities: &mut [f64],
) -> Option<Vec<f64>> {
    let surface_count = areas.len();
    let extent = surface_count.checked_mul(surface_count)?;
    if surface_count == 0
        || view_factors.len() != extent
        || emissivities.len() != surface_count
        || areas.iter().any(|area| !area.is_finite() || *area <= 0.0)
        || view_factors.iter().any(|value| !value.is_finite())
        || emissivities
            .iter()
            .any(|value| !value.is_finite() || *value < 0.0 || *value > 0.99999)
    {
        return None;
    }
    let mut coefficient_matrix = vec![0.0; extent];
    let mut offset = 0;
    for _j in 0..surface_count {
        for area in areas {
            coefficient_matrix[offset] = *area * view_factors[offset];
            offset += 1;
        }
    }
    let mut excitation = vec![0.0; surface_count];
    offset = 0;
    for i in 0..surface_count {
        let emissivity = emissivities[i];
        let factor = areas[i] / (1.0 - emissivity);
        excitation[i] = -emissivity * factor;
        coefficient_matrix[offset] -= factor;
        offset += surface_count + 1;
    }
    let mut inverse = invert_square_matrix(&mut coefficient_matrix, surface_count)?;
    offset = 0;
    for excite in excitation {
        for _i in 0..surface_count {
            inverse[offset] *= excite;
            offset += 1;
        }
    }
    let mut script_f = vec![0.0; extent];
    let mut output_offset = 0;
    for (i, emissivity) in emissivities.iter().copied().enumerate() {
        let factor = emissivity / (1.0 - emissivity);
        offset = i;
        for j in 0..surface_count {
            script_f[output_offset] = if i == j {
                factor * (inverse[offset] - emissivity)
            } else {
                factor * inverse[offset]
            };
            offset += surface_count;
            output_offset += 1;
        }
    }
    if script_f.iter().any(|value| !value.is_finite()) {
        return None;
    }
    Some(script_f)
}

fn invert_square_matrix(matrix: &mut [f64], dimension: usize) -> Option<Vec<f64>> {
    let matrix_extent = dimension.checked_mul(dimension)?;
    if dimension == 0 || matrix.len() != matrix_extent {
        return None;
    }

    let mut inverse = vec![0.0; matrix_extent];
    for i in 0..dimension {
        inverse[i * dimension + i] = 1.0;
    }

    // CalcMatrixInverse uses these literal row-major Array2 indices. Preserve
    // its flat walks rather than reinterpreting the Source's row labels.
    for i in 0..dimension {
        let mut pivot_index = i;
        let mut pivot_magnitude = matrix[i * dimension + i].abs();
        for k in i + 1..dimension {
            let candidate = matrix[i * dimension + k].abs();
            if candidate > pivot_magnitude {
                pivot_index = k;
                pivot_magnitude = candidate;
            }
        }
        if pivot_magnitude == 0.0 {
            return None;
        }

        if pivot_index != i {
            for j in 0..dimension {
                matrix.swap(j * dimension + i, j * dimension + pivot_index);
                inverse.swap(j * dimension + i, j * dimension + pivot_index);
            }
        }

        let diagonal_inverse = 1.0 / matrix[i * dimension + i];
        for k in i + 1..dimension {
            let multiplier = matrix[i * dimension + k] * diagonal_inverse;
            matrix[i * dimension + k] = multiplier;
            if multiplier != 0.0 {
                for j in i + 1..dimension {
                    matrix[j * dimension + k] -= multiplier * matrix[j * dimension + i];
                }
                for j in 0..dimension {
                    let inverse_ji = inverse[j * dimension + i];
                    if inverse_ji != 0.0 {
                        inverse[j * dimension + k] -= multiplier * inverse_ji;
                    }
                }
            }
        }
    }

    for k in (0..dimension).rev() {
        let diagonal_inverse = 1.0 / matrix[k * dimension + k];
        for j in 0..dimension {
            inverse[j * dimension + k] *= diagonal_inverse;
        }
        for i in 0..k {
            let coefficient = matrix[k * dimension + i];
            for j in 0..dimension {
                inverse[j * dimension + i] -= coefficient * inverse[j * dimension + k];
            }
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

#[cfg(test)]
#[path = "radiation_approximate_view_factor_tests.rs"]
mod approximate_view_factor_tests;
