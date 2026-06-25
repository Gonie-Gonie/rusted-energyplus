use crate::{ConstructionId, MaterialId, NormalizedName};

/// Material flavor tracked by the first typed subset.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MaterialKind {
    /// Material object with mass.
    Mass,
    /// Material:NoMass object.
    NoMass,
}

/// EnergyPlus material surface roughness.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MaterialSurfaceRoughness {
    /// `VeryRough`.
    VeryRough,
    /// `Rough`.
    Rough,
    /// `MediumRough`.
    MediumRough,
    /// `MediumSmooth`.
    MediumSmooth,
    /// `Smooth`.
    Smooth,
    /// `VerySmooth`.
    VerySmooth,
}

impl MaterialSurfaceRoughness {
    /// Parses an EnergyPlus roughness token.
    #[must_use]
    pub fn from_energyplus_name(value: &str) -> Option<Self> {
        match value.trim().to_ascii_uppercase().as_str() {
            "VERYROUGH" => Some(Self::VeryRough),
            "ROUGH" => Some(Self::Rough),
            "MEDIUMROUGH" => Some(Self::MediumRough),
            "MEDIUMSMOOTH" => Some(Self::MediumSmooth),
            "SMOOTH" => Some(Self::Smooth),
            "VERYSMOOTH" => Some(Self::VerySmooth),
            _ => None,
        }
    }
}

/// Minimal material identity and thermal properties.
#[derive(Clone, Debug, PartialEq)]
pub struct Material {
    /// Typed ID.
    pub id: MaterialId,
    /// Material name.
    pub name: NormalizedName,
    /// Material object kind.
    pub kind: MaterialKind,
    /// Surface roughness used by exterior convection algorithms.
    pub roughness: Option<MaterialSurfaceRoughness>,
    /// Conductivity for Material objects in W/m-K.
    pub conductivity_w_per_m_k: Option<f64>,
    /// Density for Material objects in kg/m3.
    pub density_kg_per_m3: Option<f64>,
    /// Specific heat for Material objects in J/kg-K.
    pub specific_heat_j_per_kg_k: Option<f64>,
    /// Thickness for Material objects in meters.
    pub thickness_m: Option<f64>,
    /// Thermal resistance for Material:NoMass objects in m2-K/W.
    pub thermal_resistance_m2_k_per_w: Option<f64>,
    /// Thermal absorptance for exterior longwave heat-balance diagnostics.
    pub thermal_absorptance: Option<f64>,
    /// Solar absorptance for exterior solar heat-balance diagnostics.
    pub solar_absorptance: Option<f64>,
    /// Visible absorptance.
    pub visible_absorptance: Option<f64>,
}

impl Material {
    /// Returns the area-normalized thermal resistance when available.
    #[must_use]
    pub fn thermal_resistance(&self) -> Option<f64> {
        if let Some(resistance) = self.thermal_resistance_m2_k_per_w
            && resistance > 0.0
        {
            return Some(resistance);
        }

        let (Some(thickness), Some(conductivity)) = (self.thickness_m, self.conductivity_w_per_m_k)
        else {
            return None;
        };
        if thickness > 0.0 && conductivity > 0.0 {
            Some(thickness / conductivity)
        } else {
            None
        }
    }

    /// Returns the area-normalized heat capacity when available.
    #[must_use]
    pub fn heat_capacity_per_area(&self) -> Option<f64> {
        let (Some(thickness), Some(density), Some(specific_heat)) = (
            self.thickness_m,
            self.density_kg_per_m3,
            self.specific_heat_j_per_kg_k,
        ) else {
            return None;
        };
        if thickness > 0.0 && density > 0.0 && specific_heat > 0.0 {
            Some(thickness * density * specific_heat)
        } else {
            None
        }
    }
}

/// Construction resolved to an ordered material layer stack.
#[derive(Clone, Debug, PartialEq)]
pub struct Construction {
    /// Typed ID.
    pub id: ConstructionId,
    /// Construction name.
    pub name: NormalizedName,
    /// Outside layer material.
    pub outside_layer: MaterialId,
    /// Ordered material layers from outside to inside.
    pub layers: Vec<MaterialId>,
}
