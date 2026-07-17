use super::MaterialSurfaceRoughness;

/// TARCOG layer classification for a `WindowMaterial:ComplexShade` object.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WindowComplexShadeLayerType {
    /// Generic diffuse shading layer (`DIFFSHADE` in TARCOG).
    OtherShadingType,
    /// Venetian blind whose slats run horizontally.
    VenetianHorizontal,
    /// Venetian blind whose slats run vertically.
    VenetianVertical,
    /// Woven shade layer.
    Woven,
    /// Perforated shade layer.
    Perforated,
    /// User-supplied BSDF shade layer.
    Bsdf,
}

impl WindowComplexShadeLayerType {
    /// Parses the six EnergyPlus 26.1 layer-type tokens case-insensitively.
    #[must_use]
    pub fn from_energyplus_name(value: &str) -> Option<Self> {
        match value.trim().to_ascii_uppercase().as_str() {
            "OTHERSHADINGTYPE" => Some(Self::OtherShadingType),
            "VENETIANHORIZONTAL" => Some(Self::VenetianHorizontal),
            "VENETIANVERTICAL" => Some(Self::VenetianVertical),
            "WOVEN" => Some(Self::Woven),
            "PERFORATED" => Some(Self::Perforated),
            "BSDF" => Some(Self::Bsdf),
            _ => None,
        }
    }

    /// Returns whether EnergyPlus applies the slat-curvature relationship.
    #[must_use]
    pub const fn is_venetian(self) -> bool {
        matches!(self, Self::VenetianHorizontal | Self::VenetianVertical)
    }
}

/// Source-effective `WindowMaterial:ComplexShade` payload.
///
/// The inherited base-material values are retained where EnergyPlus 26.1
/// leaves them at their zero initialization. In particular, this object is
/// resistance-only but does not receive either a base or nominal resistance.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowComplexShadeMaterial {
    /// Source-fixed surface roughness.
    pub roughness: MaterialSurfaceRoughness,
    /// TARCOG shade-layer classification.
    pub layer_type: WindowComplexShadeLayerType,
    /// Shade-layer thickness in meters.
    pub thickness_m: f64,
    /// Shade-layer conductivity in W/m-K.
    pub conductivity_w_per_m_k: f64,
    /// Infrared transmittance.
    pub infrared_transmittance: f64,
    /// Front-side infrared emissivity.
    pub front_infrared_emissivity: f64,
    /// Back-side infrared emissivity.
    pub back_infrared_emissivity: f64,
    /// Generic thermal absorptance copied from the back emissivity.
    pub thermal_absorptance: f64,
    /// Front-side thermal absorptance copied from the front emissivity.
    pub front_thermal_absorptance: f64,
    /// Back-side thermal absorptance copied from the back emissivity.
    pub back_thermal_absorptance: f64,
    /// Top-edge opening multiplier.
    pub top_opening_multiplier: f64,
    /// Bottom-edge opening multiplier.
    pub bottom_opening_multiplier: f64,
    /// Left-edge opening multiplier.
    pub left_side_opening_multiplier: f64,
    /// Right-edge opening multiplier.
    pub right_side_opening_multiplier: f64,
    /// Front opening area divided by shade area.
    pub front_opening_multiplier: f64,
    /// Slat width in meters.
    pub slat_width_m: f64,
    /// Distance between adjacent slat faces in meters.
    pub slat_spacing_m: f64,
    /// Slat thickness in meters.
    pub slat_thickness_m: f64,
    /// Slat angle in degrees.
    pub slat_angle_deg: f64,
    /// Slat conductivity in W/m-K.
    pub slat_conductivity_w_per_m_k: f64,
    /// Slat curvature radius in meters; zero denotes a flat slat.
    pub slat_curvature_radius_m: f64,
    /// Inherited density left at the source's zero initialization.
    pub density_kg_per_m3: f64,
    /// Inherited specific heat left at the source's zero initialization.
    pub specific_heat_j_per_kg_k: f64,
    /// Inherited base resistance left at the source's zero initialization.
    pub base_thermal_resistance_m2_k_per_w: f64,
    /// Inherited nominal resistance left at the source's zero initialization.
    pub base_nominal_thermal_resistance_m2_k_per_w: f64,
    /// Inherited solar absorptance left at the source's zero initialization.
    pub solar_absorptance: f64,
    /// Inherited visible absorptance left at the source's zero initialization.
    pub visible_absorptance: f64,
}

impl WindowComplexShadeMaterial {
    /// EnergyPlus fixes complex shades to resistance-only behavior.
    #[must_use]
    pub const fn is_resistance_only(self) -> bool {
        true
    }

    /// EnergyPlus 26.1 does not assign a nominal resistance to this object.
    #[must_use]
    pub const fn nominal_thermal_resistance_m2_k_per_w(self) -> Option<f64> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Material, MaterialDefinition, MaterialFamily, MaterialKind};
    use super::*;
    use crate::{MaterialId, NormalizedName};

    fn payload() -> WindowComplexShadeMaterial {
        WindowComplexShadeMaterial {
            roughness: MaterialSurfaceRoughness::Rough,
            layer_type: WindowComplexShadeLayerType::VenetianHorizontal,
            thickness_m: 0.002,
            conductivity_w_per_m_k: 1.0,
            infrared_transmittance: 0.1,
            front_infrared_emissivity: 0.7,
            back_infrared_emissivity: 0.8,
            thermal_absorptance: 0.8,
            front_thermal_absorptance: 0.7,
            back_thermal_absorptance: 0.8,
            top_opening_multiplier: 0.1,
            bottom_opening_multiplier: 0.2,
            left_side_opening_multiplier: 0.3,
            right_side_opening_multiplier: 0.4,
            front_opening_multiplier: 0.5,
            slat_width_m: 0.016,
            slat_spacing_m: 0.012,
            slat_thickness_m: 0.0006,
            slat_angle_deg: 45.0,
            slat_conductivity_w_per_m_k: 160.0,
            slat_curvature_radius_m: 0.008,
            density_kg_per_m3: 0.0,
            specific_heat_j_per_kg_k: 0.0,
            base_thermal_resistance_m2_k_per_w: 0.0,
            base_nominal_thermal_resistance_m2_k_per_w: 0.0,
            solar_absorptance: 0.0,
            visible_absorptance: 0.0,
        }
    }

    #[test]
    fn layer_type_parses_all_source_tokens_case_insensitively() {
        for (token, expected) in [
            (
                "OtherShadingType",
                WindowComplexShadeLayerType::OtherShadingType,
            ),
            (
                "venetianhorizontal",
                WindowComplexShadeLayerType::VenetianHorizontal,
            ),
            (
                "VENETIANVERTICAL",
                WindowComplexShadeLayerType::VenetianVertical,
            ),
            ("Woven", WindowComplexShadeLayerType::Woven),
            ("perforated", WindowComplexShadeLayerType::Perforated),
            ("bsdf", WindowComplexShadeLayerType::Bsdf),
        ] {
            assert_eq!(
                WindowComplexShadeLayerType::from_energyplus_name(token),
                Some(expected)
            );
        }
        assert_eq!(
            WindowComplexShadeLayerType::from_energyplus_name("Venetian"),
            None
        );
        assert!(WindowComplexShadeLayerType::VenetianHorizontal.is_venetian());
        assert!(WindowComplexShadeLayerType::VenetianVertical.is_venetian());
        assert!(!WindowComplexShadeLayerType::OtherShadingType.is_venetian());
    }

    #[test]
    fn material_identity_and_source_base_state_are_explicit() {
        let payload = payload();
        let material = Material {
            id: MaterialId(2),
            name: NormalizedName::new("Complex Shade"),
            definition: MaterialDefinition::WindowComplexShade(payload),
        };

        assert_eq!(material.kind(), MaterialKind::WindowComplexShade);
        assert_eq!(material.family(), MaterialFamily::ComplexFenestration);
        assert_eq!(material.as_window_complex_shade(), Some(&payload));
        assert_eq!(material.as_opaque(), None);
        assert!(payload.is_resistance_only());
        assert_eq!(payload.nominal_thermal_resistance_m2_k_per_w(), None);
        assert_eq!(
            payload.thermal_absorptance,
            payload.back_infrared_emissivity
        );
        assert_eq!(
            payload.front_thermal_absorptance,
            payload.front_infrared_emissivity
        );
        assert_eq!(
            payload.back_thermal_absorptance,
            payload.back_infrared_emissivity
        );
        assert_eq!(payload.density_kg_per_m3, 0.0);
        assert_eq!(payload.specific_heat_j_per_kg_k, 0.0);
        assert_eq!(payload.base_thermal_resistance_m2_k_per_w, 0.0);
        assert_eq!(payload.base_nominal_thermal_resistance_m2_k_per_w, 0.0);
        assert_eq!(payload.solar_absorptance, 0.0);
        assert_eq!(payload.visible_absorptance, 0.0);
    }
}
