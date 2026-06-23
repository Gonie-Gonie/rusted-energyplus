//! CTF conduction source-order ownership notes.

use crate::heat_balance::state::SurfaceHeatBalanceState;
use ep_model::OutsideBoundaryCondition;

/// EnergyPlus source file for CTF conduction state used by heat balance.
pub const CTF_SOURCE_FILE: &str = "src/EnergyPlus/HeatBalanceSurfaceManager.cc";

/// Current Rust owner for inside/outside CTF history advancement.
pub const CTF_HISTORY_OWNER_STAGE: &str = "UpdateThermalHistories";

/// Current Rust owner for inside/outside conduction report timing.
pub const CTF_REPORT_OWNER_STAGE: &str = "ReportSurfaceHeatBalance";

const ENERGYPLUS_INSIDE_SURFACE_ITER_DAMP_W_PER_M2_K: f64 = 5.0;
const ENERGYPLUS_QUICK_CONDUCTION_CROSS_THRESHOLD_W_PER_M2_K: f64 = 0.01;

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
