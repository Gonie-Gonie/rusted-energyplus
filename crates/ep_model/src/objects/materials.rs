use crate::{ConstructionId, MaterialId, NormalizedName};

/// Material flavor tracked by the first typed subset.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MaterialKind {
    /// Material object with mass.
    Mass,
    /// Material:NoMass object.
    NoMass,
    /// Material:AirGap object.
    AirGap,
    /// Material:InfraredTransparent object.
    InfraredTransparent,
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

/// Default-applied opaque surface properties shared by regular and no-mass materials.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OpaqueSurfaceProperties {
    /// Thermal absorptance for exterior longwave heat-balance diagnostics.
    pub thermal_absorptance: f64,
    /// Solar absorptance for exterior solar heat-balance diagnostics.
    pub solar_absorptance: f64,
    /// Visible absorptance.
    pub visible_absorptance: f64,
}

impl Default for OpaqueSurfaceProperties {
    fn default() -> Self {
        Self {
            thermal_absorptance: 0.9,
            solar_absorptance: 0.7,
            visible_absorptance: 0.7,
        }
    }
}

/// Required and default-applied fields for a regular `Material` object.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RegularMaterial {
    /// Surface roughness used by exterior convection algorithms.
    pub roughness: MaterialSurfaceRoughness,
    /// Material thickness in meters.
    pub thickness_m: f64,
    /// Thermal conductivity in W/m-K.
    pub conductivity_w_per_m_k: f64,
    /// Density in kg/m3.
    pub density_kg_per_m3: f64,
    /// Specific heat in J/kg-K.
    pub specific_heat_j_per_kg_k: f64,
    /// Shared opaque surface properties.
    pub surface: OpaqueSurfaceProperties,
}

/// Required and default-applied fields for a `Material:NoMass` object.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NoMassMaterial {
    /// Surface roughness used by exterior convection algorithms.
    pub roughness: MaterialSurfaceRoughness,
    /// Area-normalized thermal resistance in m2-K/W.
    pub thermal_resistance_m2_k_per_w: f64,
    /// Shared opaque surface properties.
    pub surface: OpaqueSurfaceProperties,
}

/// User-supplied fields for a `Material:AirGap` object.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AirGapMaterial {
    /// Area-normalized thermal resistance in m2-K/W.
    pub thermal_resistance_m2_k_per_w: f64,
}

/// Name-only `Material:InfraredTransparent` payload.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct InfraredTransparentMaterial;

/// Object-specific material payload.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MaterialDefinition {
    /// Regular opaque material with mass.
    Regular(RegularMaterial),
    /// Opaque material with resistance but no heat capacity.
    NoMass(NoMassMaterial),
    /// Opaque air gap with source-fixed roughness and resistance-only behavior.
    AirGap(AirGapMaterial),
    /// Infrared-transparent material whose thermal properties are fixed by EnergyPlus.
    InfraredTransparent(InfraredTransparentMaterial),
}

const AIR_GAP_SURFACE_PROPERTIES: OpaqueSurfaceProperties = OpaqueSurfaceProperties {
    thermal_absorptance: 0.0,
    solar_absorptance: 0.0,
    visible_absorptance: 0.0,
};

const INFRARED_TRANSPARENT_SURFACE_PROPERTIES: OpaqueSurfaceProperties = OpaqueSurfaceProperties {
    thermal_absorptance: 0.9999,
    solar_absorptance: 1.0,
    visible_absorptance: 1.0,
};

const INFRARED_TRANSPARENT_THERMAL_RESISTANCE_M2_K_PER_W: f64 = 0.01;

/// Minimal material identity plus an object-specific definition.
#[derive(Clone, Debug, PartialEq)]
pub struct Material {
    /// Typed ID.
    pub id: MaterialId,
    /// Material name.
    pub name: NormalizedName,
    /// Object-specific material fields.
    pub definition: MaterialDefinition,
}

impl Material {
    /// Returns the EnergyPlus object flavor.
    #[must_use]
    pub const fn kind(&self) -> MaterialKind {
        match self.definition {
            MaterialDefinition::Regular(_) => MaterialKind::Mass,
            MaterialDefinition::NoMass(_) => MaterialKind::NoMass,
            MaterialDefinition::AirGap(_) => MaterialKind::AirGap,
            MaterialDefinition::InfraredTransparent(_) => MaterialKind::InfraredTransparent,
        }
    }

    /// Returns the surface-roughness projection when applicable.
    #[must_use]
    pub const fn roughness(&self) -> Option<MaterialSurfaceRoughness> {
        match self.definition {
            MaterialDefinition::Regular(material) => Some(material.roughness),
            MaterialDefinition::NoMass(material) => Some(material.roughness),
            MaterialDefinition::AirGap(_) => Some(MaterialSurfaceRoughness::MediumRough),
            MaterialDefinition::InfraredTransparent(_) => None,
        }
    }

    /// Returns the regular-material thickness projection when applicable.
    #[must_use]
    pub const fn thickness_m(&self) -> Option<f64> {
        match self.definition {
            MaterialDefinition::Regular(material) => Some(material.thickness_m),
            MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_)
            | MaterialDefinition::InfraredTransparent(_) => None,
        }
    }

    /// Returns the regular-material conductivity projection when applicable.
    #[must_use]
    pub const fn conductivity_w_per_m_k(&self) -> Option<f64> {
        match self.definition {
            MaterialDefinition::Regular(material) => Some(material.conductivity_w_per_m_k),
            MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_)
            | MaterialDefinition::InfraredTransparent(_) => None,
        }
    }

    /// Returns the regular-material density projection when applicable.
    #[must_use]
    pub const fn density_kg_per_m3(&self) -> Option<f64> {
        match self.definition {
            MaterialDefinition::Regular(material) => Some(material.density_kg_per_m3),
            MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_)
            | MaterialDefinition::InfraredTransparent(_) => None,
        }
    }

    /// Returns the regular-material specific-heat projection when applicable.
    #[must_use]
    pub const fn specific_heat_j_per_kg_k(&self) -> Option<f64> {
        match self.definition {
            MaterialDefinition::Regular(material) => Some(material.specific_heat_j_per_kg_k),
            MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_)
            | MaterialDefinition::InfraredTransparent(_) => None,
        }
    }

    /// Returns the no-mass resistance projection when applicable.
    #[must_use]
    pub const fn no_mass_thermal_resistance_m2_k_per_w(&self) -> Option<f64> {
        match self.definition {
            MaterialDefinition::Regular(_) => None,
            MaterialDefinition::NoMass(material) => Some(material.thermal_resistance_m2_k_per_w),
            MaterialDefinition::AirGap(_) | MaterialDefinition::InfraredTransparent(_) => None,
        }
    }

    /// Returns whether EnergyPlus treats the material as resistance-only.
    #[must_use]
    pub const fn is_resistance_only(&self) -> bool {
        match self.definition {
            MaterialDefinition::Regular(_) => false,
            MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_)
            | MaterialDefinition::InfraredTransparent(_) => true,
        }
    }

    /// Returns the thermal absorptance.
    #[must_use]
    pub const fn thermal_absorptance(&self) -> f64 {
        self.surface_properties().thermal_absorptance
    }

    /// Returns the solar absorptance.
    #[must_use]
    pub const fn solar_absorptance(&self) -> f64 {
        self.surface_properties().solar_absorptance
    }

    /// Returns the visible absorptance.
    #[must_use]
    pub const fn visible_absorptance(&self) -> f64 {
        self.surface_properties().visible_absorptance
    }

    /// Returns the shared opaque surface properties.
    #[must_use]
    pub const fn surface_properties(&self) -> &OpaqueSurfaceProperties {
        match &self.definition {
            MaterialDefinition::Regular(material) => &material.surface,
            MaterialDefinition::NoMass(material) => &material.surface,
            MaterialDefinition::AirGap(_) => &AIR_GAP_SURFACE_PROPERTIES,
            MaterialDefinition::InfraredTransparent(_) => &INFRARED_TRANSPARENT_SURFACE_PROPERTIES,
        }
    }

    /// Returns the area-normalized thermal resistance when available.
    #[must_use]
    pub fn thermal_resistance(&self) -> Option<f64> {
        match self.definition {
            MaterialDefinition::Regular(material)
                if material.thickness_m > 0.0 && material.conductivity_w_per_m_k > 0.0 =>
            {
                Some(material.thickness_m / material.conductivity_w_per_m_k)
            }
            MaterialDefinition::NoMass(material)
                if material.thermal_resistance_m2_k_per_w > 0.0 =>
            {
                Some(material.thermal_resistance_m2_k_per_w)
            }
            MaterialDefinition::AirGap(material)
                if material.thermal_resistance_m2_k_per_w > 0.0 =>
            {
                Some(material.thermal_resistance_m2_k_per_w)
            }
            MaterialDefinition::InfraredTransparent(_) => {
                Some(INFRARED_TRANSPARENT_THERMAL_RESISTANCE_M2_K_PER_W)
            }
            MaterialDefinition::Regular(_)
            | MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_) => None,
        }
    }

    /// Returns the area-normalized heat capacity when available.
    #[must_use]
    pub fn heat_capacity_per_area(&self) -> Option<f64> {
        match self.definition {
            MaterialDefinition::Regular(material)
                if material.thickness_m > 0.0
                    && material.density_kg_per_m3 > 0.0
                    && material.specific_heat_j_per_kg_k > 0.0 =>
            {
                Some(
                    material.thickness_m
                        * material.density_kg_per_m3
                        * material.specific_heat_j_per_kg_k,
                )
            }
            MaterialDefinition::Regular(_)
            | MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_)
            | MaterialDefinition::InfraredTransparent(_) => None,
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
