use super::MaterialSurfaceRoughness;

/// One side's normal-incidence beam properties for equivalent-layer shading.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WindowShadeEquivalentLayerSideOpticalProperties {
    /// Direct beam-to-beam transmittance.
    pub beam_beam_transmittance: f64,
    /// Beam-to-diffuse transmittance.
    pub beam_diffuse_transmittance: f64,
    /// Beam-to-diffuse reflectance.
    pub beam_diffuse_reflectance: f64,
}

/// Fully resolved `WindowMaterial:Shade:EquivalentLayer` payload.
///
/// EnergyPlus 26.1 writes the three visible inputs only to the front-side TAR
/// record while leaving the back-side visible record at its initialized zero.
/// This type preserves that observable source behavior instead of inferring
/// front/back symmetry from the schema notes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowShadeEquivalentLayerMaterial {
    /// Source-fixed surface roughness.
    pub roughness: MaterialSurfaceRoughness,
    /// Front-side solar beam properties.
    pub front_solar: WindowShadeEquivalentLayerSideOpticalProperties,
    /// Back-side solar beam properties.
    pub back_solar: WindowShadeEquivalentLayerSideOpticalProperties,
    /// Front-side visible beam properties.
    pub front_visible: WindowShadeEquivalentLayerSideOpticalProperties,
    /// Source-initialized back-side visible beam properties (all zero in 26.1).
    pub back_visible: WindowShadeEquivalentLayerSideOpticalProperties,
    /// Shared front/back infrared transmittance.
    pub infrared_transmittance: f64,
    /// Front-side infrared emissivity.
    pub front_infrared_emissivity: f64,
    /// Back-side infrared emissivity.
    pub back_infrared_emissivity: f64,
    /// Front-side thermal absorptance derived from infrared emissivity.
    pub front_thermal_absorptance: f64,
    /// Back-side thermal absorptance derived from infrared emissivity.
    pub back_thermal_absorptance: f64,
    /// Thermal transmittance derived from infrared transmittance.
    pub thermal_transmittance: f64,
}

impl WindowShadeEquivalentLayerMaterial {
    /// EnergyPlus fixes equivalent-layer shades to resistance-only behavior.
    #[must_use]
    pub const fn is_resistance_only(self) -> bool {
        true
    }

    /// EnergyPlus 26.1 does not assign a nominal resistance for this object.
    #[must_use]
    pub const fn nominal_thermal_resistance_m2_k_per_w(self) -> Option<f64> {
        None
    }
}
