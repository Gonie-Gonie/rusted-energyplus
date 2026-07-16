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
    /// WindowMaterial:Glazing object using the SpectralAverage optical-data branch.
    WindowGlazing,
}

/// High-level material family used to keep opaque and fenestration consumers separate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MaterialFamily {
    /// Opaque heat-transfer material.
    Opaque,
    /// Fenestration material.
    Fenestration,
}

impl MaterialFamily {
    /// Stable diagnostic identifier.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Opaque => "opaque",
            Self::Fenestration => "fenestration",
        }
    }
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

/// Fully resolved `SpectralAverage` branch of a `WindowMaterial:Glazing` object.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowGlazingSpectralAverageMaterial {
    /// Glass thickness in meters.
    pub thickness_m: f64,
    /// Solar transmittance at normal incidence.
    pub solar_transmittance_at_normal_incidence: f64,
    /// Front-side solar reflectance at normal incidence.
    pub front_side_solar_reflectance_at_normal_incidence: f64,
    /// Back-side solar reflectance at normal incidence.
    pub back_side_solar_reflectance_at_normal_incidence: f64,
    /// Visible transmittance at normal incidence.
    pub visible_transmittance_at_normal_incidence: f64,
    /// Front-side visible reflectance at normal incidence.
    pub front_side_visible_reflectance_at_normal_incidence: f64,
    /// Back-side visible reflectance at normal incidence.
    pub back_side_visible_reflectance_at_normal_incidence: f64,
    /// Infrared transmittance at normal incidence.
    pub infrared_transmittance_at_normal_incidence: f64,
    /// Front-side infrared hemispherical emissivity.
    pub front_side_infrared_hemispherical_emissivity: f64,
    /// Back-side infrared hemispherical emissivity.
    pub back_side_infrared_hemispherical_emissivity: f64,
    /// Glass conductivity in W/m-K.
    pub conductivity_w_per_m_k: f64,
    /// Dirt correction factor for solar and visible transmittance.
    pub dirt_correction_factor_for_solar_and_visible_transmittance: f64,
    /// Whether the glazing is solar diffusing.
    pub solar_diffusing: bool,
    /// Young's modulus in Pa.
    pub youngs_modulus_pa: f64,
    /// Poisson's ratio.
    pub poissons_ratio: f64,
}

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
    /// Window glazing whose optical properties use the bounded `SpectralAverage` branch.
    WindowGlazingSpectralAverage(WindowGlazingSpectralAverageMaterial),
}

/// Borrowed opaque material payload used by opaque-only consumers.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OpaqueMaterialRef<'a> {
    /// Regular opaque material with mass.
    Regular(&'a RegularMaterial),
    /// Opaque material with resistance but no heat capacity.
    NoMass(&'a NoMassMaterial),
    /// Opaque air gap.
    AirGap(&'a AirGapMaterial),
    /// Infrared-transparent opaque material.
    InfraredTransparent(&'a InfraredTransparentMaterial),
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

impl<'a> OpaqueMaterialRef<'a> {
    /// Returns the surface-roughness projection when applicable.
    #[must_use]
    pub const fn roughness(self) -> Option<MaterialSurfaceRoughness> {
        match self {
            Self::Regular(material) => Some(material.roughness),
            Self::NoMass(material) => Some(material.roughness),
            Self::AirGap(_) => Some(MaterialSurfaceRoughness::MediumRough),
            Self::InfraredTransparent(_) => None,
        }
    }

    /// Returns the regular-material thickness projection when applicable.
    #[must_use]
    pub const fn thickness_m(self) -> Option<f64> {
        match self {
            Self::Regular(material) => Some(material.thickness_m),
            Self::NoMass(_) | Self::AirGap(_) | Self::InfraredTransparent(_) => None,
        }
    }

    /// Returns the regular-material conductivity projection when applicable.
    #[must_use]
    pub const fn conductivity_w_per_m_k(self) -> Option<f64> {
        match self {
            Self::Regular(material) => Some(material.conductivity_w_per_m_k),
            Self::NoMass(_) | Self::AirGap(_) | Self::InfraredTransparent(_) => None,
        }
    }

    /// Returns the regular-material density projection when applicable.
    #[must_use]
    pub const fn density_kg_per_m3(self) -> Option<f64> {
        match self {
            Self::Regular(material) => Some(material.density_kg_per_m3),
            Self::NoMass(_) | Self::AirGap(_) | Self::InfraredTransparent(_) => None,
        }
    }

    /// Returns the regular-material specific-heat projection when applicable.
    #[must_use]
    pub const fn specific_heat_j_per_kg_k(self) -> Option<f64> {
        match self {
            Self::Regular(material) => Some(material.specific_heat_j_per_kg_k),
            Self::NoMass(_) | Self::AirGap(_) | Self::InfraredTransparent(_) => None,
        }
    }

    /// Returns the no-mass resistance projection when applicable.
    #[must_use]
    pub const fn no_mass_thermal_resistance_m2_k_per_w(self) -> Option<f64> {
        match self {
            Self::NoMass(material) => Some(material.thermal_resistance_m2_k_per_w),
            Self::Regular(_) | Self::AirGap(_) | Self::InfraredTransparent(_) => None,
        }
    }

    /// Returns whether EnergyPlus treats the opaque material as resistance-only.
    #[must_use]
    pub const fn is_resistance_only(self) -> bool {
        !matches!(self, Self::Regular(_))
    }

    /// Returns the shared opaque surface properties.
    #[must_use]
    pub const fn surface_properties(self) -> &'a OpaqueSurfaceProperties {
        match self {
            Self::Regular(material) => &material.surface,
            Self::NoMass(material) => &material.surface,
            Self::AirGap(_) => &AIR_GAP_SURFACE_PROPERTIES,
            Self::InfraredTransparent(_) => &INFRARED_TRANSPARENT_SURFACE_PROPERTIES,
        }
    }

    /// Returns the area-normalized thermal resistance when available.
    #[must_use]
    pub fn thermal_resistance(self) -> Option<f64> {
        match self {
            Self::Regular(material)
                if material.thickness_m > 0.0 && material.conductivity_w_per_m_k > 0.0 =>
            {
                Some(material.thickness_m / material.conductivity_w_per_m_k)
            }
            Self::NoMass(material) if material.thermal_resistance_m2_k_per_w > 0.0 => {
                Some(material.thermal_resistance_m2_k_per_w)
            }
            Self::AirGap(material) if material.thermal_resistance_m2_k_per_w > 0.0 => {
                Some(material.thermal_resistance_m2_k_per_w)
            }
            Self::InfraredTransparent(_) => {
                Some(INFRARED_TRANSPARENT_THERMAL_RESISTANCE_M2_K_PER_W)
            }
            Self::Regular(_) | Self::NoMass(_) | Self::AirGap(_) => None,
        }
    }

    /// Returns the area-normalized heat capacity when available.
    #[must_use]
    pub fn heat_capacity_per_area(self) -> Option<f64> {
        match self {
            Self::Regular(material)
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
            Self::Regular(_) | Self::NoMass(_) | Self::AirGap(_) | Self::InfraredTransparent(_) => {
                None
            }
        }
    }
}

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
            MaterialDefinition::WindowGlazingSpectralAverage(_) => MaterialKind::WindowGlazing,
        }
    }

    /// Returns the high-level consumer family.
    #[must_use]
    pub const fn family(&self) -> MaterialFamily {
        match self.definition {
            MaterialDefinition::Regular(_)
            | MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_)
            | MaterialDefinition::InfraredTransparent(_) => MaterialFamily::Opaque,
            MaterialDefinition::WindowGlazingSpectralAverage(_) => MaterialFamily::Fenestration,
        }
    }

    /// Borrows the payload through the opaque-only material boundary.
    #[must_use]
    pub const fn as_opaque(&self) -> Option<OpaqueMaterialRef<'_>> {
        match &self.definition {
            MaterialDefinition::Regular(material) => Some(OpaqueMaterialRef::Regular(material)),
            MaterialDefinition::NoMass(material) => Some(OpaqueMaterialRef::NoMass(material)),
            MaterialDefinition::AirGap(material) => Some(OpaqueMaterialRef::AirGap(material)),
            MaterialDefinition::InfraredTransparent(material) => {
                Some(OpaqueMaterialRef::InfraredTransparent(material))
            }
            MaterialDefinition::WindowGlazingSpectralAverage(_) => None,
        }
    }

    /// Borrows the bounded `SpectralAverage` glazing payload when applicable.
    #[must_use]
    pub const fn as_window_glazing_spectral_average(
        &self,
    ) -> Option<&WindowGlazingSpectralAverageMaterial> {
        match &self.definition {
            MaterialDefinition::WindowGlazingSpectralAverage(material) => Some(material),
            MaterialDefinition::Regular(_)
            | MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_)
            | MaterialDefinition::InfraredTransparent(_) => None,
        }
    }

    /// Returns the opaque surface-roughness projection when applicable.
    #[must_use]
    pub fn roughness(&self) -> Option<MaterialSurfaceRoughness> {
        self.as_opaque().and_then(OpaqueMaterialRef::roughness)
    }

    /// Returns the opaque regular-material thickness projection when applicable.
    #[must_use]
    pub fn thickness_m(&self) -> Option<f64> {
        self.as_opaque().and_then(OpaqueMaterialRef::thickness_m)
    }

    /// Returns the opaque regular-material conductivity projection when applicable.
    #[must_use]
    pub fn conductivity_w_per_m_k(&self) -> Option<f64> {
        self.as_opaque()
            .and_then(OpaqueMaterialRef::conductivity_w_per_m_k)
    }

    /// Returns the opaque regular-material density projection when applicable.
    #[must_use]
    pub fn density_kg_per_m3(&self) -> Option<f64> {
        self.as_opaque()
            .and_then(OpaqueMaterialRef::density_kg_per_m3)
    }

    /// Returns the opaque regular-material specific-heat projection when applicable.
    #[must_use]
    pub fn specific_heat_j_per_kg_k(&self) -> Option<f64> {
        self.as_opaque()
            .and_then(OpaqueMaterialRef::specific_heat_j_per_kg_k)
    }

    /// Returns the opaque no-mass resistance projection when applicable.
    #[must_use]
    pub fn no_mass_thermal_resistance_m2_k_per_w(&self) -> Option<f64> {
        self.as_opaque()
            .and_then(OpaqueMaterialRef::no_mass_thermal_resistance_m2_k_per_w)
    }

    /// Returns whether EnergyPlus treats an opaque material as resistance-only.
    #[must_use]
    pub fn is_resistance_only(&self) -> Option<bool> {
        self.as_opaque().map(OpaqueMaterialRef::is_resistance_only)
    }

    /// Returns the opaque thermal absorptance.
    #[must_use]
    pub fn thermal_absorptance(&self) -> Option<f64> {
        self.surface_properties()
            .map(|surface| surface.thermal_absorptance)
    }

    /// Returns the opaque solar absorptance.
    #[must_use]
    pub fn solar_absorptance(&self) -> Option<f64> {
        self.surface_properties()
            .map(|surface| surface.solar_absorptance)
    }

    /// Returns the opaque visible absorptance.
    #[must_use]
    pub fn visible_absorptance(&self) -> Option<f64> {
        self.surface_properties()
            .map(|surface| surface.visible_absorptance)
    }

    /// Returns the shared opaque surface properties when applicable.
    #[must_use]
    pub fn surface_properties(&self) -> Option<&OpaqueSurfaceProperties> {
        self.as_opaque().map(OpaqueMaterialRef::surface_properties)
    }

    /// Returns the opaque area-normalized thermal resistance when available.
    #[must_use]
    pub fn thermal_resistance(&self) -> Option<f64> {
        self.as_opaque()
            .and_then(OpaqueMaterialRef::thermal_resistance)
    }

    /// Returns the opaque area-normalized heat capacity when available.
    #[must_use]
    pub fn heat_capacity_per_area(&self) -> Option<f64> {
        self.as_opaque()
            .and_then(OpaqueMaterialRef::heat_capacity_per_area)
    }
}

/// Consumer family for an ordered construction layer stack.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConstructionKind {
    /// Opaque construction consumed by the existing surface heat-balance path.
    Opaque,
    /// Fenestration construction reserved for a dedicated window heat-balance path.
    Fenestration,
}

impl ConstructionKind {
    /// Stable diagnostic identifier.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Opaque => "opaque",
            Self::Fenestration => "fenestration",
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
    /// Consumer family for this construction.
    pub kind: ConstructionKind,
    /// Outside layer material.
    pub outside_layer: MaterialId,
    /// Ordered material layers from outside to inside.
    pub layers: Vec<MaterialId>,
}
