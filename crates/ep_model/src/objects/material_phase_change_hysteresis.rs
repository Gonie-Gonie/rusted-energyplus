use crate::{MaterialId, MaterialPhaseChangeHysteresisId, NormalizedName};

/// Conductivity, density, and specific heat for one fully resolved phase.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PhaseChangeHysteresisThermalState {
    /// Thermal conductivity in W/m-K.
    pub conductivity_w_per_m_k: f64,
    /// Density in kg/m3.
    pub density_kg_per_m3: f64,
    /// Specific heat in J/kg-K.
    pub specific_heat_j_per_kg_k: f64,
}

/// Peak temperature and the asymmetric temperature widths around it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PhaseChangeHysteresisCurve {
    /// Temperature width above the peak in delta C.
    pub high_temperature_difference_c: f64,
    /// Curve peak temperature in C.
    pub peak_temperature_c: f64,
    /// Temperature width below the peak in delta C.
    pub low_temperature_difference_c: f64,
}

/// Typed `MaterialProperty:PhaseChangeHysteresis` attachment.
#[derive(Clone, Debug, PartialEq)]
pub struct MaterialPhaseChangeHysteresis {
    /// Stable typed object ID.
    pub id: MaterialPhaseChangeHysteresisId,
    /// Normalized object key, which is also the referenced material name.
    pub name: NormalizedName,
    /// Referenced regular or no-mass material.
    pub reference_material: MaterialId,
    /// Total latent heat for either complete phase transition in J/kg.
    pub total_latent_heat_j_per_kg: f64,
    /// Fully liquid thermal properties.
    pub liquid_state: PhaseChangeHysteresisThermalState,
    /// Melting curve parameters in source field order.
    pub melting_curve: PhaseChangeHysteresisCurve,
    /// Fully solid thermal properties.
    pub solid_state: PhaseChangeHysteresisThermalState,
    /// Freezing curve parameters in source field order.
    pub freezing_curve: PhaseChangeHysteresisCurve,
    /// Source-initialized transition specific heat, the mean of solid and liquid values.
    pub transition_specific_heat_j_per_kg_k: f64,
    /// Source-initialized prior specific heat, equal to the solid value.
    pub initial_specific_heat_j_per_kg_k: f64,
}
