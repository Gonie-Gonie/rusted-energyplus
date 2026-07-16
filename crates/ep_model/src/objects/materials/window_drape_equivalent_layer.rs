use super::{MaterialSurfaceRoughness, WindowShadeEquivalentLayerSideOpticalProperties};

/// Fully resolved `WindowMaterial:Drape:EquivalentLayer` payload.
///
/// EnergyPlus 26.1 writes the three visible inputs only to the front-side TAR
/// record while leaving the back-side visible record at its initialized zero.
/// Pleat dimensions are source-effective: both values are retained only when
/// both inputs are nonzero; otherwise both remain zero.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowDrapeEquivalentLayerMaterial {
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
    /// Effective width of the pleated fabric section in meters.
    pub pleated_width_m: f64,
    /// Effective length of the pleated fabric section in meters.
    pub pleated_length_m: f64,
}

impl WindowDrapeEquivalentLayerMaterial {
    /// EnergyPlus fixes equivalent-layer drapes to resistance-only behavior.
    #[must_use]
    pub const fn is_resistance_only(self) -> bool {
        true
    }

    /// EnergyPlus 26.1 does not assign a nominal resistance for this object.
    #[must_use]
    pub const fn nominal_thermal_resistance_m2_k_per_w(self) -> Option<f64> {
        None
    }

    /// Returns whether both source-effective pleat dimensions are nonzero.
    #[must_use]
    pub const fn is_pleated(self) -> bool {
        self.pleated_width_m != 0.0 && self.pleated_length_m != 0.0
    }
}
