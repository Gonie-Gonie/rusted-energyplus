use super::MaterialSurfaceRoughness;

/// Fully resolved `WindowMaterial:Shade` payload.
///
/// EnergyPlus treats a shade as an ordinary fenestration material with fixed
/// medium-rough surfaces. The visible absorptance remains the source-initialized
/// zero; unlike solar absorptance, EnergyPlus 26.1 does not derive it from the
/// visible transmittance and reflectance inputs for this object.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowShadeMaterial {
    /// Source-fixed surface roughness.
    pub roughness: MaterialSurfaceRoughness,
    /// Hemispherical-diffuse solar transmittance.
    pub solar_transmittance: f64,
    /// Shared front/back hemispherical-diffuse solar reflectance.
    pub solar_reflectance: f64,
    /// Hemispherical-diffuse visible transmittance.
    pub visible_transmittance: f64,
    /// Shared front/back hemispherical-diffuse visible reflectance.
    pub visible_reflectance: f64,
    /// Shared front/back infrared hemispherical emissivity.
    pub infrared_hemispherical_emissivity: f64,
    /// Shared front/back infrared transmittance.
    pub infrared_transmittance: f64,
    /// Shade thickness in meters.
    pub thickness_m: f64,
    /// Shade conductivity in W/m-K.
    pub conductivity_w_per_m_k: f64,
    /// Source-derived solar absorptance, `max(0, 1 - Tsol - Rsol)`.
    pub solar_absorptance: f64,
    /// Source-initialized visible absorptance. EnergyPlus 26.1 leaves this zero.
    pub visible_absorptance: f64,
    /// Distance from the shade to the adjacent glazing in meters.
    pub shade_to_glass_distance_m: f64,
    /// Top-edge opening area multiplier.
    pub top_opening_multiplier: f64,
    /// Bottom-edge opening area multiplier.
    pub bottom_opening_multiplier: f64,
    /// Left-edge opening area multiplier.
    pub left_side_opening_multiplier: f64,
    /// Right-edge opening area multiplier.
    pub right_side_opening_multiplier: f64,
    /// Effective shade opening area divided by shade area.
    pub airflow_permeability: f64,
}

impl WindowShadeMaterial {
    /// Returns the source-order nominal resistance, thickness divided by
    /// conductivity.
    #[must_use]
    pub fn nominal_thermal_resistance_m2_k_per_w(self) -> Option<f64> {
        (self.thickness_m > 0.0 && self.conductivity_w_per_m_k > 0.0)
            .then_some(self.thickness_m / self.conductivity_w_per_m_k)
    }
}
