//! CTF conduction source-order ownership notes.

use crate::heat_balance::state::{
    HeatBalanceCtfHistorySlotSample, SurfaceCtfState, SurfaceHeatBalanceState,
};
use ep_model::{NormalizedName, OutsideBoundaryCondition};
use std::collections::BTreeMap;

/// EnergyPlus source file for CTF conduction state used by heat balance.
pub const CTF_SOURCE_FILE: &str = "src/EnergyPlus/HeatBalanceSurfaceManager.cc";

/// Current Rust owner for inside/outside CTF history advancement.
pub const CTF_HISTORY_OWNER_STAGE: &str = "UpdateThermalHistories";

/// Current Rust owner for inside/outside conduction report timing.
pub const CTF_REPORT_OWNER_STAGE: &str = "ReportSurfaceHeatBalance";

const ENERGYPLUS_INSIDE_SURFACE_ITER_DAMP_W_PER_M2_K: f64 = 5.0;
const ENERGYPLUS_QUICK_CONDUCTION_CROSS_THRESHOLD_W_PER_M2_K: f64 = 0.01;

/// Per-construction CTF coefficient row used to seed diagnostic surface histories.
#[derive(Clone, Debug, PartialEq)]
pub struct ConstructionCtfCoefficientOverride {
    /// EnergyPlus-normalized construction name.
    pub construction_name: String,
    /// EnergyPlus CTF time/history index. Time zero is the current coefficient row.
    pub time_index: usize,
    /// CTF outside/X coefficient in W/m2-K.
    pub outside_w_per_m2_k: f64,
    /// CTF cross/Y coefficient in W/m2-K.
    pub cross_w_per_m2_k: f64,
    /// CTF inside/Z coefficient in W/m2-K.
    pub inside_w_per_m2_k: f64,
    /// CTF flux coefficient for history rows.
    pub flux: Option<f64>,
}

pub(crate) fn steady_ctf_coefficient_w_per_m2_k(
    area_m2: f64,
    thermal_resistance_m2_k_per_w: f64,
) -> f64 {
    if area_m2 > 0.0 && thermal_resistance_m2_k_per_w > 0.0 {
        1.0 / thermal_resistance_m2_k_per_w
    } else {
        0.0
    }
}

pub(crate) fn steady_surface_ctf_state(
    coefficient_w_per_m2_k: f64,
    initial_temperature_c: f64,
) -> SurfaceCtfState {
    SurfaceCtfState {
        outside_0_w_per_m2_k: coefficient_w_per_m2_k,
        cross_0_w_per_m2_k: coefficient_w_per_m2_k,
        inside_0_w_per_m2_k: coefficient_w_per_m2_k,
        flux_0: None,
        const_in_part_w_per_m2: 0.0,
        const_out_part_w_per_m2: 0.0,
        outside_history_w_per_m2_k: Vec::new(),
        cross_history_w_per_m2_k: Vec::new(),
        inside_history_w_per_m2_k: Vec::new(),
        flux_history: Vec::new(),
        outside_temperature_history_c: vec![initial_temperature_c],
        inside_temperature_history_c: vec![initial_temperature_c],
        outside_flux_history_w_per_m2: vec![0.0],
        inside_flux_history_w_per_m2: vec![0.0],
    }
}

pub(crate) fn construction_ctf_coefficients_by_name(
    coefficients: &[ConstructionCtfCoefficientOverride],
) -> BTreeMap<String, Vec<&ConstructionCtfCoefficientOverride>> {
    let mut by_construction = BTreeMap::new();
    for coefficient in coefficients {
        by_construction
            .entry(NormalizedName::new(&coefficient.construction_name).0)
            .or_insert_with(Vec::new)
            .push(coefficient);
    }
    for coefficients in by_construction.values_mut() {
        // EnergyPlus writes EIO CTF rows in descending array index, but the
        // surface balance consumes history terms as Term=1..NumCTFTerms.
        coefficients.sort_by_key(|coefficient| coefficient.time_index);
    }
    by_construction
}

pub(crate) fn surface_ctf_state_from_coefficients(
    coefficients: &[&ConstructionCtfCoefficientOverride],
    initial_temperature_c: f64,
) -> Option<SurfaceCtfState> {
    let zero = coefficients
        .iter()
        .copied()
        .find(|coefficient| coefficient.time_index == 0)?;
    let history = coefficients
        .iter()
        .copied()
        .filter(|coefficient| coefficient.time_index > 0)
        .collect::<Vec<_>>();
    let history_terms = history.len();

    Some(SurfaceCtfState {
        outside_0_w_per_m2_k: zero.outside_w_per_m2_k,
        cross_0_w_per_m2_k: zero.cross_w_per_m2_k,
        inside_0_w_per_m2_k: zero.inside_w_per_m2_k,
        flux_0: zero.flux,
        const_in_part_w_per_m2: 0.0,
        const_out_part_w_per_m2: 0.0,
        outside_history_w_per_m2_k: history
            .iter()
            .map(|coefficient| coefficient.outside_w_per_m2_k)
            .collect(),
        cross_history_w_per_m2_k: history
            .iter()
            .map(|coefficient| coefficient.cross_w_per_m2_k)
            .collect(),
        inside_history_w_per_m2_k: history
            .iter()
            .map(|coefficient| coefficient.inside_w_per_m2_k)
            .collect(),
        flux_history: history
            .iter()
            .map(|coefficient| coefficient.flux.unwrap_or(0.0))
            .collect(),
        outside_temperature_history_c: vec![initial_temperature_c; history_terms],
        inside_temperature_history_c: vec![initial_temperature_c; history_terms],
        outside_flux_history_w_per_m2: vec![0.0; history_terms],
        inside_flux_history_w_per_m2: vec![0.0; history_terms],
    })
}

pub(crate) fn surface_ctf_state_from_coefficient_rows(
    coefficients: &[ConstructionCtfCoefficientOverride],
    initial_temperature_c: f64,
) -> Option<SurfaceCtfState> {
    let coefficients = coefficients.iter().collect::<Vec<_>>();
    surface_ctf_state_from_coefficients(&coefficients, initial_temperature_c)
}

/// Inputs for the EnergyPlus CTF inside-face temperature balance subset.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CtfInsideFaceBalanceInput {
    /// Reference zone air temperature used by inside convection in C.
    pub reference_air_temperature_c: f64,
    /// Inside convection coefficient in W/m2-K.
    pub inside_convection_coefficient_w_per_m2_k: f64,
    /// Previous inside-face temperature from the current inside-surface iteration in C.
    pub previous_inside_face_temperature_c: f64,
    /// Net inside radiant/source term in W/m2 from EnergyPlus `SurfTempTerm` inputs.
    pub net_inside_source_w_per_m2: f64,
}

/// Inputs for the EnergyPlus CTF outside-face environmental balance subset.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CtfOutsideFaceBalanceInput {
    /// Outdoor air temperature used by exterior convection in C.
    pub outdoor_air_temperature_c: f64,
    /// Linearized outside radiant temperature in C.
    pub radiant_temperature_c: f64,
    /// Outside convection coefficient in W/m2-K.
    pub outside_convection_coefficient_w_per_m2_k: f64,
    /// Linearized outside radiation coefficient in W/m2-K.
    pub outside_radiation_coefficient_w_per_m2_k: f64,
    /// Shortwave/source term absorbed at the outside face in W/m2.
    pub absorbed_outside_source_w_per_m2: f64,
}

/// Inputs for the EnergyPlus quick-conduction outside-face balance subset.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CtfOutsideQuickConductionBalanceInput {
    /// Outside environmental/source balance inputs.
    pub environmental: CtfOutsideFaceBalanceInput,
    /// Reference zone air temperature used by inside convection in C.
    pub reference_air_temperature_c: f64,
    /// Inside convection coefficient in W/m2-K.
    pub inside_convection_coefficient_w_per_m2_k: f64,
    /// Net inside radiant/source term in W/m2 from EnergyPlus `SurfTempTerm` inputs.
    pub net_inside_source_w_per_m2: f64,
}

/// EnergyPlus-shaped CTF inside-face temperature balance for the opaque subset.
///
/// This covers the no-pool/no-movable-insulation branch documented in
/// `CalcHeatBalanceInsideSurf2CTFOnly`. Inside shortwave, radiant, additional
/// heat-source, HVAC radiant, and net longwave terms are passed through the
/// source-map slots on `SurfaceHeatBalanceState`.
#[must_use]
pub fn energyplus_ctf_inside_face_temperature_c(
    surface: &SurfaceHeatBalanceState,
    input: CtfInsideFaceBalanceInput,
) -> f64 {
    energyplus_ctf_inside_face_temperature_c_with_outside_temperature(surface, input, None)
}

pub(crate) fn energyplus_ctf_inside_face_temperature_c_with_outside_temperature(
    surface: &SurfaceHeatBalanceState,
    input: CtfInsideFaceBalanceInput,
    outside_face_temperature_override_c: Option<f64>,
) -> f64 {
    let adiabatic_cross =
        if surface.outside_boundary_condition == OutsideBoundaryCondition::Adiabatic {
            surface.ctf.cross_0_w_per_m2_k
        } else {
            0.0
        };
    let outside_face_temperature_c =
        outside_face_temperature_override_c.unwrap_or(surface.outside_face_temperature_c);
    let outside_temperature_term =
        if surface.outside_boundary_condition == OutsideBoundaryCondition::Adiabatic {
            0.0
        } else {
            surface.ctf.cross_0_w_per_m2_k * outside_face_temperature_c
        };
    let denominator = surface.ctf.inside_0_w_per_m2_k - adiabatic_cross
        + input.inside_convection_coefficient_w_per_m2_k
        + ENERGYPLUS_INSIDE_SURFACE_ITER_DAMP_W_PER_M2_K;
    if denominator.abs() <= f64::EPSILON {
        return surface.inside_face_temperature_c;
    }

    (surface.ctf.const_in_part_w_per_m2
        + input.net_inside_source_w_per_m2
        + input.inside_convection_coefficient_w_per_m2_k * input.reference_air_temperature_c
        + ENERGYPLUS_INSIDE_SURFACE_ITER_DAMP_W_PER_M2_K * input.previous_inside_face_temperature_c
        + outside_temperature_term)
        / denominator
}

/// EnergyPlus-shaped CTF outside-face environmental balance for the opaque subset.
#[must_use]
pub fn energyplus_ctf_outside_face_temperature_c(
    surface: &SurfaceHeatBalanceState,
    input: CtfOutsideFaceBalanceInput,
) -> f64 {
    let denominator = surface.ctf.outside_0_w_per_m2_k
        + input.outside_convection_coefficient_w_per_m2_k
        + input.outside_radiation_coefficient_w_per_m2_k;
    if denominator.abs() <= f64::EPSILON {
        return input.outdoor_air_temperature_c;
    }

    (-surface.ctf.const_out_part_w_per_m2
        + input.absorbed_outside_source_w_per_m2
        + input.outside_convection_coefficient_w_per_m2_k * input.outdoor_air_temperature_c
        + input.outside_radiation_coefficient_w_per_m2_k * input.radiant_temperature_c
        + surface.ctf.cross_0_w_per_m2_k * surface.inside_face_temperature_c)
        / denominator
}

/// EnergyPlus-shaped quick-conduction outside-face balance for the opaque subset.
#[must_use]
pub fn energyplus_ctf_outside_face_temperature_quick_conduction_c(
    surface: &SurfaceHeatBalanceState,
    input: CtfOutsideQuickConductionBalanceInput,
) -> f64 {
    energyplus_ctf_outside_face_temperature_quick_conduction_calculation(surface, input)
        .temperature_c
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct CtfOutsideQuickConductionBalanceCalculation {
    pub(crate) temperature_c: f64,
    pub(crate) coupling_factor: f64,
    pub(crate) denominator_w_per_m2_k: f64,
    pub(crate) numerator_w_per_m2: f64,
    pub(crate) inside_balance_term_w_per_m2: f64,
}

pub(crate) fn energyplus_ctf_outside_face_temperature_quick_conduction_calculation(
    surface: &SurfaceHeatBalanceState,
    input: CtfOutsideQuickConductionBalanceInput,
) -> CtfOutsideQuickConductionBalanceCalculation {
    let inside_denominator =
        surface.ctf.inside_0_w_per_m2_k + input.inside_convection_coefficient_w_per_m2_k;
    if surface.ctf.cross_0_w_per_m2_k <= ENERGYPLUS_QUICK_CONDUCTION_CROSS_THRESHOLD_W_PER_M2_K
        || inside_denominator.abs() <= f64::EPSILON
    {
        return CtfOutsideQuickConductionBalanceCalculation {
            temperature_c: energyplus_ctf_outside_face_temperature_c(surface, input.environmental),
            ..CtfOutsideQuickConductionBalanceCalculation::default()
        };
    }

    let f1 = surface.ctf.cross_0_w_per_m2_k / inside_denominator;
    let denominator = surface.ctf.outside_0_w_per_m2_k
        + input
            .environmental
            .outside_convection_coefficient_w_per_m2_k
        + input.environmental.outside_radiation_coefficient_w_per_m2_k
        - f1 * surface.ctf.cross_0_w_per_m2_k;
    if denominator.abs() <= f64::EPSILON {
        return CtfOutsideQuickConductionBalanceCalculation {
            temperature_c: energyplus_ctf_outside_face_temperature_c(surface, input.environmental),
            coupling_factor: f1,
            denominator_w_per_m2_k: denominator,
            ..CtfOutsideQuickConductionBalanceCalculation::default()
        };
    }

    let inside_balance_term = surface.ctf.const_in_part_w_per_m2
        + input.net_inside_source_w_per_m2
        + input.inside_convection_coefficient_w_per_m2_k * input.reference_air_temperature_c;
    let numerator = -surface.ctf.const_out_part_w_per_m2
        + input.environmental.absorbed_outside_source_w_per_m2
        + input
            .environmental
            .outside_convection_coefficient_w_per_m2_k
            * input.environmental.outdoor_air_temperature_c
        + input.environmental.outside_radiation_coefficient_w_per_m2_k
            * input.environmental.radiant_temperature_c
        + f1 * inside_balance_term;
    CtfOutsideQuickConductionBalanceCalculation {
        temperature_c: numerator / denominator,
        coupling_factor: f1,
        denominator_w_per_m2_k: denominator,
        numerator_w_per_m2: numerator,
        inside_balance_term_w_per_m2: inside_balance_term,
    }
}

pub(crate) fn surface_inside_conduction_rate_w(surface: &SurfaceHeatBalanceState) -> f64 {
    surface.area_m2 * surface_inside_conduction_flux_w_per_m2(surface)
}

pub(crate) fn surface_inside_conduction_rate_w_for_report(
    surface: &SurfaceHeatBalanceState,
    use_inside_ctf_outside_temperature_for_conduction_report: bool,
) -> f64 {
    if use_inside_ctf_outside_temperature_for_conduction_report {
        surface.area_m2
            * surface_inside_conduction_flux_w_per_m2_with_outside_temperature(
                surface,
                surface.inside_ctf_outside_temperature_c,
            )
    } else {
        surface_inside_conduction_rate_w(surface)
    }
}

pub(crate) fn surface_outside_conduction_rate_w(surface: &SurfaceHeatBalanceState) -> f64 {
    -surface.area_m2 * surface_outside_conduction_flux_w_per_m2(surface)
}

pub(crate) fn surface_outside_conduction_rate_w_for_report(
    surface: &SurfaceHeatBalanceState,
    use_inside_ctf_outside_temperature_for_conduction_report: bool,
) -> f64 {
    if use_inside_ctf_outside_temperature_for_conduction_report {
        -surface.area_m2
            * surface_outside_conduction_flux_w_per_m2_with_outside_temperature(
                surface,
                surface.inside_ctf_outside_temperature_c,
            )
    } else {
        surface_outside_conduction_rate_w(surface)
    }
}

pub(crate) fn surface_heat_storage_rate_w(inside_rate_w: f64, outside_rate_w: f64) -> f64 {
    -(inside_rate_w + outside_rate_w)
}

pub(crate) fn surface_inside_conduction_flux_w_per_m2(surface: &SurfaceHeatBalanceState) -> f64 {
    surface_inside_conduction_flux_w_per_m2_with_outside_temperature(
        surface,
        surface.outside_face_temperature_c,
    )
}

pub(crate) fn surface_inside_conduction_flux_w_per_m2_with_outside_temperature(
    surface: &SurfaceHeatBalanceState,
    outside_temperature_c: f64,
) -> f64 {
    if surface.area_m2 <= 0.0 {
        return 0.0;
    }

    outside_temperature_c * surface.ctf.cross_0_w_per_m2_k
        - surface.inside_face_temperature_c * surface.ctf.inside_0_w_per_m2_k
        + surface.ctf.const_in_part_w_per_m2
}

pub(crate) fn surface_outside_conduction_flux_w_per_m2(surface: &SurfaceHeatBalanceState) -> f64 {
    surface_outside_conduction_flux_w_per_m2_with_outside_temperature(
        surface,
        surface.outside_face_temperature_c,
    )
}

pub(crate) fn surface_outside_conduction_flux_w_per_m2_with_outside_temperature(
    surface: &SurfaceHeatBalanceState,
    outside_temperature_c: f64,
) -> f64 {
    if surface.area_m2 <= 0.0 {
        return 0.0;
    }

    outside_temperature_c * surface.ctf.outside_0_w_per_m2_k
        - surface.inside_face_temperature_c * surface.ctf.cross_0_w_per_m2_k
        + surface.ctf.const_out_part_w_per_m2
}

pub(crate) fn surface_ctf_inside_current_outside_term_rate_w(
    surface: &SurfaceHeatBalanceState,
) -> f64 {
    surface_ctf_inside_current_outside_term_rate_w_with_outside_temperature(
        surface,
        surface.outside_face_temperature_c,
    )
}

pub(crate) fn surface_ctf_inside_current_outside_term_rate_w_for_report(
    surface: &SurfaceHeatBalanceState,
    use_inside_ctf_outside_temperature_for_conduction_report: bool,
) -> f64 {
    if use_inside_ctf_outside_temperature_for_conduction_report {
        surface_ctf_inside_current_outside_term_rate_w_with_outside_temperature(
            surface,
            surface.inside_ctf_outside_temperature_c,
        )
    } else {
        surface_ctf_inside_current_outside_term_rate_w(surface)
    }
}

pub(crate) fn surface_ctf_inside_current_outside_term_rate_w_with_outside_temperature(
    surface: &SurfaceHeatBalanceState,
    outside_temperature_c: f64,
) -> f64 {
    if surface.area_m2 <= 0.0 {
        return 0.0;
    }
    surface.area_m2 * outside_temperature_c * surface.ctf.cross_0_w_per_m2_k
}

pub(crate) fn surface_ctf_inside_current_inside_term_rate_w(
    surface: &SurfaceHeatBalanceState,
) -> f64 {
    surface_ctf_inside_current_inside_term_rate_w_from_sources(
        surface.area_m2,
        surface.ctf.inside_0_w_per_m2_k,
        surface.inside_face_temperature_c,
    )
}

pub(crate) fn surface_ctf_inside_current_inside_term_rate_w_from_sources(
    area_m2: f64,
    ctf_inside_0_w_per_m2_k: f64,
    inside_face_temperature_c: f64,
) -> f64 {
    if area_m2 <= 0.0 {
        return 0.0;
    }
    -area_m2 * inside_face_temperature_c * ctf_inside_0_w_per_m2_k
}

pub(crate) fn surface_ctf_inside_history_term_rate_w(surface: &SurfaceHeatBalanceState) -> f64 {
    if surface.area_m2 <= 0.0 {
        return 0.0;
    }
    surface.area_m2 * surface.ctf.const_in_part_w_per_m2
}

pub(crate) fn surface_ctf_outside_current_outside_term_rate_w(
    surface: &SurfaceHeatBalanceState,
) -> f64 {
    surface_ctf_outside_current_outside_term_rate_w_with_outside_temperature(
        surface,
        surface.outside_face_temperature_c,
    )
}

pub(crate) fn surface_ctf_outside_current_outside_term_rate_w_for_report(
    surface: &SurfaceHeatBalanceState,
    use_inside_ctf_outside_temperature_for_conduction_report: bool,
) -> f64 {
    if use_inside_ctf_outside_temperature_for_conduction_report {
        surface_ctf_outside_current_outside_term_rate_w_with_outside_temperature(
            surface,
            surface.inside_ctf_outside_temperature_c,
        )
    } else {
        surface_ctf_outside_current_outside_term_rate_w(surface)
    }
}

pub(crate) fn surface_ctf_outside_current_outside_term_rate_w_with_outside_temperature(
    surface: &SurfaceHeatBalanceState,
    outside_temperature_c: f64,
) -> f64 {
    if surface.area_m2 <= 0.0 {
        return 0.0;
    }
    -surface.area_m2 * outside_temperature_c * surface.ctf.outside_0_w_per_m2_k
}

pub(crate) fn surface_ctf_outside_current_inside_term_rate_w(
    surface: &SurfaceHeatBalanceState,
) -> f64 {
    if surface.area_m2 <= 0.0 {
        return 0.0;
    }
    surface.area_m2 * surface.inside_face_temperature_c * surface.ctf.cross_0_w_per_m2_k
}

pub(crate) fn surface_ctf_outside_history_term_rate_w(surface: &SurfaceHeatBalanceState) -> f64 {
    if surface.area_m2 <= 0.0 {
        return 0.0;
    }
    -surface.area_m2 * surface.ctf.const_out_part_w_per_m2
}

pub(crate) fn surface_ctf_history_term_count(surface: &SurfaceHeatBalanceState) -> usize {
    surface
        .ctf
        .outside_history_w_per_m2_k
        .len()
        .max(surface.ctf.cross_history_w_per_m2_k.len())
        .max(surface.ctf.inside_history_w_per_m2_k.len())
        .max(surface.ctf.flux_history.len())
}

pub(crate) fn surface_ctf_history_slot_samples(
    surface: &SurfaceHeatBalanceState,
) -> Vec<HeatBalanceCtfHistorySlotSample> {
    (0..surface_ctf_history_term_count(surface))
        .map(|term| surface_ctf_history_slot_sample(surface, term))
        .collect()
}

fn surface_ctf_history_slot_sample(
    surface: &SurfaceHeatBalanceState,
    term: usize,
) -> HeatBalanceCtfHistorySlotSample {
    let outside_temperature_history_c = surface
        .ctf
        .outside_temperature_history_c
        .get(term)
        .copied()
        .unwrap_or(surface.outside_face_temperature_c);
    let inside_temperature_history_c = surface
        .ctf
        .inside_temperature_history_c
        .get(term)
        .copied()
        .unwrap_or(surface.inside_face_temperature_c);
    let outside_flux_history_w_per_m2 = surface
        .ctf
        .outside_flux_history_w_per_m2
        .get(term)
        .copied()
        .unwrap_or(0.0);
    let inside_flux_history_w_per_m2 = surface
        .ctf
        .inside_flux_history_w_per_m2
        .get(term)
        .copied()
        .unwrap_or(0.0);
    let outside_history_coefficient_w_per_m2_k = surface
        .ctf
        .outside_history_w_per_m2_k
        .get(term)
        .copied()
        .unwrap_or(0.0);
    let cross_history_coefficient_w_per_m2_k = surface
        .ctf
        .cross_history_w_per_m2_k
        .get(term)
        .copied()
        .unwrap_or(0.0);
    let inside_history_coefficient_w_per_m2_k = surface
        .ctf
        .inside_history_w_per_m2_k
        .get(term)
        .copied()
        .unwrap_or(0.0);
    let flux_history_coefficient = surface.ctf.flux_history.get(term).copied().unwrap_or(0.0);

    let inside_temperature_term_w = surface.area_m2
        * (cross_history_coefficient_w_per_m2_k * outside_temperature_history_c
            - inside_history_coefficient_w_per_m2_k * inside_temperature_history_c);
    let inside_flux_term_w =
        surface.area_m2 * flux_history_coefficient * inside_flux_history_w_per_m2;
    let outside_temperature_term_w = -surface.area_m2
        * (outside_history_coefficient_w_per_m2_k * outside_temperature_history_c
            - cross_history_coefficient_w_per_m2_k * inside_temperature_history_c);
    let outside_flux_term_w =
        -surface.area_m2 * flux_history_coefficient * outside_flux_history_w_per_m2;

    HeatBalanceCtfHistorySlotSample {
        surface_name: surface.surface_name.clone(),
        construction_name: surface.construction_name.clone(),
        slot_index: term + 1,
        area_m2: surface.area_m2,
        outside_history_coefficient_w_per_m2_k,
        cross_history_coefficient_w_per_m2_k,
        inside_history_coefficient_w_per_m2_k,
        flux_history_coefficient,
        outside_temperature_history_c,
        inside_temperature_history_c,
        outside_flux_history_w_per_m2,
        inside_flux_history_w_per_m2,
        inside_temperature_term_w,
        inside_flux_term_w,
        inside_total_term_w: inside_temperature_term_w + inside_flux_term_w,
        outside_temperature_term_w,
        outside_flux_term_w,
        outside_total_term_w: outside_temperature_term_w + outside_flux_term_w,
    }
}

pub(crate) fn heat_balance_ctf_history_slot_samples(
    surfaces: &[SurfaceHeatBalanceState],
) -> Vec<HeatBalanceCtfHistorySlotSample> {
    surfaces
        .iter()
        .flat_map(surface_ctf_history_slot_samples)
        .collect()
}

pub(crate) fn heat_balance_ctf_history_slot_inside_temperature_term_rate_w(
    samples: &[HeatBalanceCtfHistorySlotSample],
    surface_name: &str,
) -> f64 {
    samples
        .iter()
        .filter(|sample| sample.surface_name == surface_name)
        .map(|sample| sample.inside_temperature_term_w)
        .sum()
}

pub(crate) fn heat_balance_ctf_history_slot_inside_flux_term_rate_w(
    samples: &[HeatBalanceCtfHistorySlotSample],
    surface_name: &str,
) -> f64 {
    samples
        .iter()
        .filter(|sample| sample.surface_name == surface_name)
        .map(|sample| sample.inside_flux_term_w)
        .sum()
}

pub(crate) fn update_surface_ctf_history_constants(surface: &mut SurfaceHeatBalanceState) {
    surface.ctf.const_in_part_w_per_m2 = surface_ctf_const_in_part_w_per_m2(surface);
    surface.ctf.const_out_part_w_per_m2 = surface_ctf_const_out_part_w_per_m2(surface);
}

pub(crate) fn surface_ctf_const_in_part_w_per_m2(surface: &SurfaceHeatBalanceState) -> f64 {
    let mut const_in_part_w_per_m2 = 0.0;
    let terms = surface_ctf_history_term_count(surface);

    for term in 0..terms {
        let outside_temperature_c = surface
            .ctf
            .outside_temperature_history_c
            .get(term)
            .copied()
            .unwrap_or(surface.outside_face_temperature_c);
        let inside_temperature_c = surface
            .ctf
            .inside_temperature_history_c
            .get(term)
            .copied()
            .unwrap_or(surface.inside_face_temperature_c);
        let inside_flux_w_per_m2 = surface
            .ctf
            .inside_flux_history_w_per_m2
            .get(term)
            .copied()
            .unwrap_or(0.0);
        let cross = surface
            .ctf
            .cross_history_w_per_m2_k
            .get(term)
            .copied()
            .unwrap_or(0.0);
        let inside = surface
            .ctf
            .inside_history_w_per_m2_k
            .get(term)
            .copied()
            .unwrap_or(0.0);
        let flux = surface.ctf.flux_history.get(term).copied().unwrap_or(0.0);

        const_in_part_w_per_m2 += cross * outside_temperature_c - inside * inside_temperature_c
            + flux * inside_flux_w_per_m2;
    }

    const_in_part_w_per_m2
}

pub(crate) fn surface_ctf_const_out_part_w_per_m2(surface: &SurfaceHeatBalanceState) -> f64 {
    let mut const_out_part_w_per_m2 = 0.0;
    let terms = surface_ctf_history_term_count(surface);

    for term in 0..terms {
        let outside_temperature_c = surface
            .ctf
            .outside_temperature_history_c
            .get(term)
            .copied()
            .unwrap_or(surface.outside_face_temperature_c);
        let inside_temperature_c = surface
            .ctf
            .inside_temperature_history_c
            .get(term)
            .copied()
            .unwrap_or(surface.inside_face_temperature_c);
        let outside_flux_w_per_m2 = surface
            .ctf
            .outside_flux_history_w_per_m2
            .get(term)
            .copied()
            .unwrap_or(0.0);
        let cross = surface
            .ctf
            .cross_history_w_per_m2_k
            .get(term)
            .copied()
            .unwrap_or(0.0);
        let outside = surface
            .ctf
            .outside_history_w_per_m2_k
            .get(term)
            .copied()
            .unwrap_or(0.0);
        let flux = surface.ctf.flux_history.get(term).copied().unwrap_or(0.0);

        const_out_part_w_per_m2 += outside * outside_temperature_c - cross * inside_temperature_c
            + flux * outside_flux_w_per_m2;
    }

    const_out_part_w_per_m2
}

pub(crate) fn advance_surface_ctf_histories(surface: &mut SurfaceHeatBalanceState) {
    advance_surface_ctf_histories_with_outside_temperature_override(surface, None);
}

pub(crate) fn advance_surface_ctf_histories_with_outside_temperature_override(
    surface: &mut SurfaceHeatBalanceState,
    outside_temperature_override_c: Option<f64>,
) {
    let history_terms = surface_ctf_history_term_count(surface);
    if history_terms == 0 {
        return;
    }

    let outside_temperature_c =
        outside_temperature_override_c.unwrap_or(surface.outside_face_temperature_c);
    let inside_flux_w_per_m2 = surface_inside_conduction_flux_w_per_m2_with_outside_temperature(
        surface,
        outside_temperature_c,
    );
    let outside_flux_w_per_m2 = surface_outside_conduction_flux_w_per_m2_with_outside_temperature(
        surface,
        outside_temperature_c,
    );
    push_surface_history(
        &mut surface.ctf.outside_temperature_history_c,
        outside_temperature_c,
        history_terms,
    );
    push_surface_history(
        &mut surface.ctf.inside_temperature_history_c,
        surface.inside_face_temperature_c,
        history_terms,
    );
    push_surface_history(
        &mut surface.ctf.inside_flux_history_w_per_m2,
        inside_flux_w_per_m2,
        history_terms,
    );
    push_surface_history(
        &mut surface.ctf.outside_flux_history_w_per_m2,
        outside_flux_w_per_m2,
        history_terms,
    );
}

fn push_surface_history(history: &mut Vec<f64>, value: f64, limit: usize) {
    history.insert(0, value);
    history.truncate(limit);
}

pub(crate) fn surface_rate_per_area_w_per_m2(rate_w: f64, area_m2: f64) -> f64 {
    if area_m2 > 0.0 { rate_w / area_m2 } else { 0.0 }
}
