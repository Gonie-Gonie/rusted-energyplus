use crate::AutoOrNumber;

use super::MaterialSurfaceRoughness;

/// Symmetric front/back solar properties for a
/// `WindowMaterial:Screen:EquivalentLayer`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowScreenEquivalentLayerSolarProperties {
    /// Beam-to-beam solar transmittance, or EnergyPlus `Autocalculate`.
    pub beam_beam_transmittance: AutoOrNumber,
    /// Beam-to-diffuse solar transmittance.
    pub beam_diffuse_transmittance: f64,
    /// Beam-to-diffuse solar reflectance.
    pub beam_diffuse_reflectance: f64,
}

/// Visible properties for one side of an equivalent-layer screen.
///
/// EnergyPlus 26.1 stores the visible-reflectance input in the
/// diffuse-to-diffuse slot, not the beam-to-diffuse slot.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WindowScreenEquivalentLayerVisibleProperties {
    /// Beam-to-beam visible transmittance.
    pub beam_beam_transmittance: f64,
    /// Beam-to-diffuse visible transmittance.
    pub beam_diffuse_transmittance: f64,
    /// Source-initialized beam-to-diffuse visible reflectance.
    pub beam_diffuse_reflectance: f64,
    /// Diffuse-to-diffuse visible reflectance.
    pub diffuse_diffuse_reflectance: f64,
}

/// Fully resolved `WindowMaterial:Screen:EquivalentLayer` payload.
///
/// EnergyPlus 26.1 mirrors the three solar inputs to both sides, writes the
/// visible inputs only to the front-side record, and leaves omitted wire
/// geometry at the class-initialized zero values.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowScreenEquivalentLayerMaterial {
    /// Source-fixed surface roughness.
    pub roughness: MaterialSurfaceRoughness,
    /// Front-side solar properties.
    pub front_solar: WindowScreenEquivalentLayerSolarProperties,
    /// Back-side solar properties, copied from the front side by EnergyPlus.
    pub back_solar: WindowScreenEquivalentLayerSolarProperties,
    /// Front-side visible properties.
    pub front_visible: WindowScreenEquivalentLayerVisibleProperties,
    /// Source-initialized back-side visible properties (all zero in 26.1).
    pub back_visible: WindowScreenEquivalentLayerVisibleProperties,
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
    /// Base-material thermal absorptance left at its class default.
    ///
    /// EnergyPlus's final thermal-sum check reads this zero-valued base field,
    /// rather than either side-specific absorptance above.
    pub base_thermal_absorptance: f64,
    /// Center-to-center screen-wire spacing in meters.
    pub wire_spacing_m: f64,
    /// Screen-wire diameter in meters.
    pub wire_diameter_m: f64,
}

impl WindowScreenEquivalentLayerMaterial {
    /// EnergyPlus fixes equivalent-layer screens to resistance-only behavior.
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
