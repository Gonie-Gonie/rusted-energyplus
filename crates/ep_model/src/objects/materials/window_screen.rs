use super::MaterialSurfaceRoughness;

/// EnergyPlus accounting method for beam radiation reflected by a
/// `WindowMaterial:Screen`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WindowScreenBeamReflectanceModel {
    /// Do not model reflected beam radiation.
    DoNotModel,
    /// Treat reflected beam radiation as direct beam radiation.
    ModelAsDirectBeam,
    /// Treat reflected beam radiation as diffuse radiation.
    ModelAsDiffuse,
}

impl WindowScreenBeamReflectanceModel {
    /// Parses an EnergyPlus reflected-beam accounting token.
    #[must_use]
    pub fn from_energyplus_name(value: &str) -> Option<Self> {
        match value.trim().to_ascii_uppercase().as_str() {
            "DONOTMODEL" => Some(Self::DoNotModel),
            "MODELASDIRECTBEAM" => Some(Self::ModelAsDirectBeam),
            "MODELASDIFFUSE" => Some(Self::ModelAsDiffuse),
            _ => None,
        }
    }
}

/// Allowed angular resolution for the optional screen-transmittance map.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WindowScreenTransmittanceMapResolution {
    /// Disable transmittance-map output (`0`).
    Disabled,
    /// One-degree resolution.
    Degrees1,
    /// Two-degree resolution.
    Degrees2,
    /// Three-degree resolution.
    Degrees3,
    /// Five-degree resolution.
    Degrees5,
}

impl WindowScreenTransmittanceMapResolution {
    /// Returns the EnergyPlus numeric-enum value in degrees.
    #[must_use]
    pub const fn degrees(self) -> u8 {
        match self {
            Self::Disabled => 0,
            Self::Degrees1 => 1,
            Self::Degrees2 => 2,
            Self::Degrees3 => 3,
            Self::Degrees5 => 5,
        }
    }
}

/// Fully resolved `WindowMaterial:Screen` payload.
///
/// EnergyPlus represents the crossed cylindrical wires by their diameter to
/// spacing ratio. The open-area transmittance is `(1 - diameter / spacing)^2`;
/// the input reflectances and emissivity apply only to the complementary solid
/// fraction.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowScreenMaterial {
    /// Source-fixed surface roughness.
    pub roughness: MaterialSurfaceRoughness,
    /// Accounting method for reflected beam radiation.
    pub beam_reflectance_model: WindowScreenBeamReflectanceModel,
    /// User-supplied diffuse solar reflectance of the screen material.
    pub diffuse_solar_reflectance_input: f64,
    /// User-supplied diffuse visible reflectance of the screen material.
    pub diffuse_visible_reflectance_input: f64,
    /// User-supplied thermal hemispherical emissivity of the screen material.
    pub thermal_hemispherical_emissivity_input: f64,
    /// Screen-wire conductivity in W/m-K.
    pub conductivity_w_per_m_k: f64,
    /// Center-to-center screen-wire spacing in meters.
    pub screen_material_spacing_m: f64,
    /// Screen-wire diameter in meters.
    pub screen_material_diameter_m: f64,
    /// Distance from the screen to the adjacent glazing in meters.
    pub screen_to_glass_distance_m: f64,
    /// Top-edge opening area multiplier.
    pub top_opening_multiplier: f64,
    /// Bottom-edge opening area multiplier.
    pub bottom_opening_multiplier: f64,
    /// Left-edge opening area multiplier.
    pub left_side_opening_multiplier: f64,
    /// Right-edge opening area multiplier.
    pub right_side_opening_multiplier: f64,
    /// Angular resolution used by the optional screen transmittance map.
    pub transmittance_map_resolution: WindowScreenTransmittanceMapResolution,
    /// Source-derived direct-normal open-area transmittance.
    pub direct_normal_transmittance: f64,
    /// Source-derived diffuse solar reflectance over the total screen area.
    pub solar_reflectance: f64,
    /// Source-derived diffuse visible reflectance over the total screen area.
    pub visible_reflectance: f64,
    /// Source-derived visible transmittance.
    pub visible_transmittance: f64,
    /// Source-derived thermal transmittance.
    pub thermal_transmittance: f64,
    /// Source-derived open-area airflow permeability.
    pub airflow_permeability: f64,
    /// Source-derived solar absorptance over the total screen area.
    pub solar_absorptance: f64,
    /// Source-derived visible absorptance over the total screen area.
    pub visible_absorptance: f64,
    /// Source-derived thermal absorptance over the total screen area.
    pub thermal_absorptance: f64,
}

impl WindowScreenMaterial {
    /// Returns the source wire-diameter to wire-spacing ratio.
    #[must_use]
    pub fn diameter_to_spacing_ratio(self) -> f64 {
        self.screen_material_diameter_m / self.screen_material_spacing_m
    }

    /// EnergyPlus stores the screen-wire diameter as material thickness.
    #[must_use]
    pub const fn thickness_m(self) -> f64 {
        self.screen_material_diameter_m
    }

    /// EnergyPlus fixes ordinary screens to resistance-only behavior.
    #[must_use]
    pub const fn is_resistance_only(self) -> bool {
        true
    }

    /// Returns EnergyPlus's source-order nominal resistance: solid fraction
    /// times wire diameter divided by conductivity.
    #[must_use]
    pub fn nominal_thermal_resistance_m2_k_per_w(self) -> Option<f64> {
        if self.screen_material_spacing_m <= 0.0
            || self.screen_material_diameter_m <= 0.0
            || self.conductivity_w_per_m_k <= 0.0
        {
            return None;
        }
        let open_fraction = (1.0 - self.diameter_to_spacing_ratio()).powi(2);
        let solid_fraction = 1.0 - open_fraction;
        Some(solid_fraction * self.screen_material_diameter_m / self.conductivity_w_per_m_k)
    }
}
