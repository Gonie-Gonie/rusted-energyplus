use super::{
    MaterialSurfaceRoughness, WindowBlindDirectionalOpticalProperties, WindowBlindSlatOrientation,
    WindowShadeEquivalentLayerSideOpticalProperties,
};

/// Slat-angle control embedded in a
/// `WindowMaterial:Blind:EquivalentLayer` material definition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WindowBlindEquivalentLayerSlatAngleControl {
    /// Preserve the material's input slat angle.
    FixedSlatAngle,
    /// Track the profile angle to maximize transmitted solar radiation.
    MaximizeSolar,
    /// Adjust the slats to exclude direct beam while retaining visibility.
    BlockBeamSolar,
}

impl WindowBlindEquivalentLayerSlatAngleControl {
    /// Parses an EnergyPlus equivalent-layer blind control token.
    #[must_use]
    pub fn from_energyplus_name(value: &str) -> Option<Self> {
        match value.trim().to_ascii_uppercase().as_str() {
            "FIXEDSLATANGLE" => Some(Self::FixedSlatAngle),
            "MAXIMIZESOLAR" => Some(Self::MaximizeSolar),
            "BLOCKBEAMSOLAR" => Some(Self::BlockBeamSolar),
            _ => None,
        }
    }
}

/// Source-effective `WindowMaterial:Blind:EquivalentLayer` payload before the
/// ASHWAT equivalent-layer calculations run.
///
/// When all three visible diffuse fields are present, EnergyPlus 26.1 copies
/// raw numeric arguments 13--15 into the visible diffuse record instead of
/// 16--18. That guard is independent of the solar diffuse assignment guard,
/// so a blank argument 13 can leave solar diffuse state zero while visible
/// diffuse retains arguments 14 and 15.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowBlindEquivalentLayerMaterial {
    /// Source-fixed surface roughness.
    pub roughness: MaterialSurfaceRoughness,
    /// Slat-axis orientation.
    pub slat_orientation: WindowBlindSlatOrientation,
    /// Source-recovered slat width in meters.
    pub slat_width_m: f64,
    /// Source-recovered clear slat separation in meters.
    pub slat_separation_m: f64,
    /// Source-recovered slat crown in meters.
    pub slat_crown_m: f64,
    /// Source-recovered initial slat angle in degrees.
    pub slat_angle_deg: f64,
    /// Front-side solar beam properties.
    pub front_solar: WindowShadeEquivalentLayerSideOpticalProperties,
    /// Back-side solar beam properties.
    pub back_solar: WindowShadeEquivalentLayerSideOpticalProperties,
    /// Front-side visible beam properties.
    pub front_visible: WindowShadeEquivalentLayerSideOpticalProperties,
    /// Back-side visible beam properties.
    pub back_visible: WindowShadeEquivalentLayerSideOpticalProperties,
    /// Solar diffuse-to-diffuse slat properties.
    pub solar_diffuse_diffuse: WindowBlindDirectionalOpticalProperties,
    /// Visible diffuse-to-diffuse slat properties after the 26.1 copy quirk.
    pub visible_diffuse_diffuse: WindowBlindDirectionalOpticalProperties,
    /// Shared front/back infrared transmittance.
    pub infrared_transmittance: f64,
    /// Front-side infrared emissivity.
    pub front_infrared_emissivity: f64,
    /// Back-side infrared emissivity.
    pub back_infrared_emissivity: f64,
    /// Front-side thermal absorptance copied from infrared emissivity.
    pub front_thermal_absorptance: f64,
    /// Back-side thermal absorptance copied from infrared emissivity.
    pub back_thermal_absorptance: f64,
    /// Thermal transmittance copied from infrared transmittance.
    pub thermal_transmittance: f64,
    /// Material-embedded slat-angle control.
    pub slat_angle_control: WindowBlindEquivalentLayerSlatAngleControl,
}

impl WindowBlindEquivalentLayerMaterial {
    /// EnergyPlus fixes equivalent-layer blinds to resistance-only behavior.
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
