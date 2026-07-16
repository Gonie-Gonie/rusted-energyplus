use super::MaterialSurfaceRoughness;

/// Slat-axis orientation shared by ordinary and equivalent-layer blinds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WindowBlindSlatOrientation {
    /// Slats run horizontally.
    Horizontal,
    /// Slats run vertically.
    Vertical,
}

impl WindowBlindSlatOrientation {
    /// Parses an EnergyPlus slat-orientation token.
    #[must_use]
    pub fn from_energyplus_name(value: &str) -> Option<Self> {
        match value.trim().to_ascii_uppercase().as_str() {
            "HORIZONTAL" => Some(Self::Horizontal),
            "VERTICAL" => Some(Self::Vertical),
            _ => None,
        }
    }
}

/// Source-initialized slat-angle behavior for a blind material definition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WindowBlindSlatAngleType {
    /// Material input always starts with a fixed slat angle; later shading
    /// controls may create a variable-angle copy.
    Fixed,
}

/// Transmittance and directional front/back reflectances for one slat optical
/// path.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WindowBlindDirectionalOpticalProperties {
    /// Slat transmittance for this optical path.
    pub transmittance: f64,
    /// Front-side slat reflectance.
    pub front_reflectance: f64,
    /// Back-side slat reflectance.
    pub back_reflectance: f64,
}

/// Source-order `WindowMaterial:Blind` payload before blind optical tables are
/// evaluated.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowBlindMaterial {
    /// Source-fixed surface roughness.
    pub roughness: MaterialSurfaceRoughness,
    /// Slat-axis orientation.
    pub slat_orientation: WindowBlindSlatOrientation,
    /// Slat width in meters.
    pub slat_width_m: f64,
    /// Clear separation between adjacent slat faces in meters.
    pub slat_separation_m: f64,
    /// Slat thickness in meters.
    pub slat_thickness_m: f64,
    /// Initial slat angle in degrees.
    pub slat_angle_deg: f64,
    /// Slat conductivity in W/m-K.
    pub slat_conductivity_w_per_m_k: f64,
    /// Solar beam-to-diffuse slat properties.
    pub solar_beam_diffuse: WindowBlindDirectionalOpticalProperties,
    /// Solar diffuse-to-diffuse slat properties.
    pub solar_diffuse_diffuse: WindowBlindDirectionalOpticalProperties,
    /// Visible beam-to-diffuse slat properties.
    pub visible_beam_diffuse: WindowBlindDirectionalOpticalProperties,
    /// Visible diffuse-to-diffuse slat properties.
    pub visible_diffuse_diffuse: WindowBlindDirectionalOpticalProperties,
    /// Front-side infrared hemispherical transmittance.
    pub front_infrared_transmittance: f64,
    /// Back-side infrared hemispherical transmittance.
    pub back_infrared_transmittance: f64,
    /// Front-side infrared hemispherical emissivity.
    pub front_infrared_emissivity: f64,
    /// Back-side infrared hemispherical emissivity.
    pub back_infrared_emissivity: f64,
    /// Distance from the blind to adjacent glazing in meters.
    pub blind_to_glass_distance_m: f64,
    /// Top-edge opening multiplier.
    pub top_opening_multiplier: f64,
    /// Bottom-edge opening multiplier.
    pub bottom_opening_multiplier: f64,
    /// Left-edge opening multiplier.
    pub left_side_opening_multiplier: f64,
    /// Right-edge opening multiplier.
    pub right_side_opening_multiplier: f64,
    /// User-supplied minimum slat angle in degrees.
    pub minimum_slat_angle_deg: f64,
    /// User-supplied maximum slat angle in degrees.
    pub maximum_slat_angle_deg: f64,
    /// Source-initialized slat-angle behavior.
    pub slat_angle_type: WindowBlindSlatAngleType,
    /// Source-initialized slat crown in meters.
    pub slat_crown_m: f64,
    /// Unassigned inherited material thickness, fixed at zero by the source.
    pub base_thickness_m: f64,
    /// Unassigned inherited material conductivity, fixed at zero by the source.
    pub base_conductivity_w_per_m_k: f64,
    /// Unassigned inherited nominal resistance, fixed at zero by the source.
    pub base_thermal_resistance_m2_k_per_w: f64,
    /// Unassigned inherited solar absorptance, fixed at zero by the source.
    pub base_solar_absorptance: f64,
    /// Unassigned inherited visible absorptance, fixed at zero by the source.
    pub base_visible_absorptance: f64,
    /// Unassigned inherited thermal absorptance, fixed at zero by the source.
    pub base_thermal_absorptance: f64,
}

impl WindowBlindMaterial {
    /// EnergyPlus fixes ordinary blinds to resistance-only behavior.
    #[must_use]
    pub const fn is_resistance_only(self) -> bool {
        true
    }

    /// Blind material input does not assign a nominal thermal resistance.
    #[must_use]
    pub const fn nominal_thermal_resistance_m2_k_per_w(self) -> Option<f64> {
        None
    }
}
