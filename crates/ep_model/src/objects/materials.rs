use crate::{ConstructionId, MaterialId, NormalizedName};

mod roof_vegetation;
mod window_blind;
mod window_blind_equivalent_layer;
mod window_complex_gap;
mod window_complex_shade;
mod window_drape_equivalent_layer;
mod window_gas;
mod window_glazing;
mod window_screen;
mod window_screen_equivalent_layer;
mod window_shade;
mod window_shade_equivalent_layer;
mod window_simple_glazing;

pub use roof_vegetation::{RoofVegetationMaterial, RoofVegetationMoistureDiffusionMethod};
pub use window_blind::{
    WindowBlindDirectionalOpticalProperties, WindowBlindMaterial, WindowBlindSlatAngleType,
    WindowBlindSlatOrientation,
};
pub use window_blind_equivalent_layer::{
    WindowBlindEquivalentLayerMaterial, WindowBlindEquivalentLayerSlatAngleControl,
};
pub use window_complex_gap::{
    WindowComplexGapGasComposition, WindowComplexGapMaterial, WindowComplexGapSupportPillar,
};
pub use window_complex_shade::{WindowComplexShadeLayerType, WindowComplexShadeMaterial};
pub use window_drape_equivalent_layer::WindowDrapeEquivalentLayerMaterial;
pub use window_gas::{
    WindowGapEquivalentLayerMaterial, WindowGapVentType, WindowGasMaterial, WindowGasMixture,
    WindowGasMixtureComponent, WindowGasMixtureMaterial, WindowGasPolynomialCoefficients,
    WindowGasProperties, WindowGasType, WindowStandardGasType,
};
pub use window_glazing::{
    WindowGlazingEquivalentLayerDiffuseProperties,
    WindowGlazingEquivalentLayerDirectionalProperties, WindowGlazingEquivalentLayerMaterial,
    WindowGlazingEquivalentLayerOpticalBand, WindowGlazingRefractionExtinctionMaterial,
    WindowGlazingRefractionExtinctionOpticalProperties, WindowGlazingSpectralAverageMaterial,
    WindowGlazingThermochromicGroupMaterial, WindowGlazingThermochromicState,
};
pub use window_screen::{
    WindowScreenBeamReflectanceModel, WindowScreenMaterial, WindowScreenTransmittanceMapResolution,
};
pub use window_screen_equivalent_layer::{
    WindowScreenEquivalentLayerMaterial, WindowScreenEquivalentLayerSolarProperties,
    WindowScreenEquivalentLayerVisibleProperties,
};
pub use window_shade::WindowShadeMaterial;
pub use window_shade_equivalent_layer::{
    WindowShadeEquivalentLayerMaterial, WindowShadeEquivalentLayerSideOpticalProperties,
};
pub use window_simple_glazing::WindowSimpleGlazingMaterial;

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
    /// WindowMaterial:Glazing:RefractionExtinctionMethod object.
    WindowGlazingRefractionExtinction,
    /// WindowMaterial:Glazing:EquivalentLayer object.
    WindowGlazingEquivalentLayer,
    /// WindowMaterial:GlazingGroup:Thermochromic object.
    WindowGlazingThermochromicGroup,
    /// WindowMaterial:SimpleGlazingSystem object.
    WindowSimpleGlazing,
    /// Complex-fenestration `WindowMaterial:Gap` object.
    WindowComplexGap,
    /// Complex-fenestration `WindowMaterial:ComplexShade` object.
    WindowComplexShade,
    /// WindowMaterial:Gas object.
    WindowGas,
    /// WindowMaterial:Gap:EquivalentLayer object.
    WindowGapEquivalentLayer,
    /// WindowMaterial:GasMixture object.
    WindowGasMixture,
    /// WindowMaterial:Shade object.
    WindowShade,
    /// WindowMaterial:Shade:EquivalentLayer object.
    WindowShadeEquivalentLayer,
    /// WindowMaterial:Drape:EquivalentLayer object.
    WindowDrapeEquivalentLayer,
    /// WindowMaterial:Screen object.
    WindowScreen,
    /// WindowMaterial:Screen:EquivalentLayer object.
    WindowScreenEquivalentLayer,
    /// WindowMaterial:Blind object.
    WindowBlind,
    /// WindowMaterial:Blind:EquivalentLayer object.
    WindowBlindEquivalentLayer,
    /// Material:RoofVegetation object.
    RoofVegetation,
}

/// High-level material family used to keep construction consumers separate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MaterialFamily {
    /// Opaque heat-transfer material.
    Opaque,
    /// Regular fenestration material consumed by `Construction`.
    Fenestration,
    /// Equivalent-layer fenestration material consumed only by
    /// `Construction:WindowEquivalentLayer`.
    EquivalentLayer,
    /// Non-executable thermochromic parent whose child generation/runtime consumer is deferred.
    ThermochromicGroup,
    /// Whole-system simple glazing accepted only as a sole ordinary construction layer.
    /// Window runtime and multi-layer source quirks remain deferred.
    SimpleGlazing,
    /// Complex-fenestration material whose dedicated construction consumer is deferred.
    ComplexFenestration,
}

impl MaterialFamily {
    /// Stable diagnostic identifier.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Opaque => "opaque",
            Self::Fenestration => "fenestration",
            Self::EquivalentLayer => "equivalent-layer",
            Self::ThermochromicGroup => "thermochromic-group",
            Self::SimpleGlazing => "simple-glazing",
            Self::ComplexFenestration => "complex-fenestration",
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
    /// Window glazing whose optical properties are derived from refraction and
    /// extinction inputs.
    WindowGlazingRefractionExtinction(WindowGlazingRefractionExtinctionMaterial),
    /// Equivalent-layer glazing with directional and diffuse optical inputs.
    WindowGlazingEquivalentLayer(WindowGlazingEquivalentLayerMaterial),
    /// Ordered thermochromic glazing states held in the model's flat state arena.
    WindowGlazingThermochromicGroup(WindowGlazingThermochromicGroupMaterial),
    /// Simple glazing performance indices and their source-derived equivalent layer.
    WindowSimpleGlazing(WindowSimpleGlazingMaterial),
    /// Complex-fenestration gap with source-copied gas state.
    WindowComplexGap(WindowComplexGapMaterial),
    /// Complex-fenestration shade with source-effective inherited state.
    WindowComplexShade(WindowComplexShadeMaterial),
    /// Single-gas ordinary window gap.
    WindowGas(WindowGasMaterial),
    /// Single-gas equivalent-layer window gap.
    WindowGapEquivalentLayer(WindowGapEquivalentLayerMaterial),
    /// Ordinary window gap containing one to four built-in gases.
    WindowGasMixture(WindowGasMixtureMaterial),
    /// Ordinary window shade.
    WindowShade(WindowShadeMaterial),
    /// Equivalent-layer window shade.
    WindowShadeEquivalentLayer(WindowShadeEquivalentLayerMaterial),
    /// Equivalent-layer window drape.
    WindowDrapeEquivalentLayer(WindowDrapeEquivalentLayerMaterial),
    /// Ordinary exterior window screen.
    WindowScreen(WindowScreenMaterial),
    /// Equivalent-layer window screen.
    WindowScreenEquivalentLayer(WindowScreenEquivalentLayerMaterial),
    /// Ordinary window blind.
    WindowBlind(WindowBlindMaterial),
    /// Equivalent-layer window blind.
    WindowBlindEquivalentLayer(WindowBlindEquivalentLayerMaterial),
    /// Vegetated-roof plant and dry-soil material.
    RoofVegetation(RoofVegetationMaterial),
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
    /// Vegetated-roof plant and dry-soil material.
    RoofVegetation(&'a RoofVegetationMaterial),
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
            Self::RoofVegetation(material) => Some(material.roughness),
            Self::NoMass(material) => Some(material.roughness),
            Self::AirGap(_) => Some(MaterialSurfaceRoughness::MediumRough),
            Self::InfraredTransparent(_) => None,
        }
    }

    /// Returns the mass-bearing opaque-material thickness when applicable.
    #[must_use]
    pub const fn thickness_m(self) -> Option<f64> {
        match self {
            Self::Regular(material) => Some(material.thickness_m),
            Self::RoofVegetation(material) => Some(material.thickness_m),
            Self::NoMass(_) | Self::AirGap(_) | Self::InfraredTransparent(_) => None,
        }
    }

    /// Returns the mass-bearing opaque-material conductivity when applicable.
    #[must_use]
    pub const fn conductivity_w_per_m_k(self) -> Option<f64> {
        match self {
            Self::Regular(material) => Some(material.conductivity_w_per_m_k),
            Self::RoofVegetation(material) => Some(material.dry_soil_conductivity_w_per_m_k),
            Self::NoMass(_) | Self::AirGap(_) | Self::InfraredTransparent(_) => None,
        }
    }

    /// Returns the mass-bearing opaque-material density when applicable.
    #[must_use]
    pub const fn density_kg_per_m3(self) -> Option<f64> {
        match self {
            Self::Regular(material) => Some(material.density_kg_per_m3),
            Self::RoofVegetation(material) => Some(material.dry_soil_density_kg_per_m3),
            Self::NoMass(_) | Self::AirGap(_) | Self::InfraredTransparent(_) => None,
        }
    }

    /// Returns the mass-bearing opaque-material specific heat when applicable.
    #[must_use]
    pub const fn specific_heat_j_per_kg_k(self) -> Option<f64> {
        match self {
            Self::Regular(material) => Some(material.specific_heat_j_per_kg_k),
            Self::RoofVegetation(material) => Some(material.dry_soil_specific_heat_j_per_kg_k),
            Self::NoMass(_) | Self::AirGap(_) | Self::InfraredTransparent(_) => None,
        }
    }

    /// Returns the no-mass resistance projection when applicable.
    #[must_use]
    pub const fn no_mass_thermal_resistance_m2_k_per_w(self) -> Option<f64> {
        match self {
            Self::NoMass(material) => Some(material.thermal_resistance_m2_k_per_w),
            Self::Regular(_)
            | Self::AirGap(_)
            | Self::InfraredTransparent(_)
            | Self::RoofVegetation(_) => None,
        }
    }

    /// Returns whether EnergyPlus treats the opaque material as resistance-only.
    #[must_use]
    pub const fn is_resistance_only(self) -> bool {
        !matches!(self, Self::Regular(_) | Self::RoofVegetation(_))
    }

    /// Returns the shared opaque surface properties.
    #[must_use]
    pub const fn surface_properties(self) -> &'a OpaqueSurfaceProperties {
        match self {
            Self::Regular(material) => &material.surface,
            Self::RoofVegetation(material) => &material.surface,
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
            Self::RoofVegetation(material)
                if material.thickness_m > 0.0 && material.dry_soil_conductivity_w_per_m_k > 0.0 =>
            {
                Some(material.thickness_m / material.dry_soil_conductivity_w_per_m_k)
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
            Self::Regular(_) | Self::NoMass(_) | Self::AirGap(_) | Self::RoofVegetation(_) => None,
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
            Self::RoofVegetation(material)
                if material.thickness_m > 0.0
                    && material.dry_soil_density_kg_per_m3 > 0.0
                    && material.dry_soil_specific_heat_j_per_kg_k > 0.0 =>
            {
                Some(
                    material.thickness_m
                        * material.dry_soil_density_kg_per_m3
                        * material.dry_soil_specific_heat_j_per_kg_k,
                )
            }
            Self::Regular(_)
            | Self::NoMass(_)
            | Self::AirGap(_)
            | Self::InfraredTransparent(_)
            | Self::RoofVegetation(_) => None,
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
            MaterialDefinition::WindowGlazingRefractionExtinction(_) => {
                MaterialKind::WindowGlazingRefractionExtinction
            }
            MaterialDefinition::WindowGlazingEquivalentLayer(_) => {
                MaterialKind::WindowGlazingEquivalentLayer
            }
            MaterialDefinition::WindowGlazingThermochromicGroup(_) => {
                MaterialKind::WindowGlazingThermochromicGroup
            }
            MaterialDefinition::WindowSimpleGlazing(_) => MaterialKind::WindowSimpleGlazing,
            MaterialDefinition::WindowComplexGap(_) => MaterialKind::WindowComplexGap,
            MaterialDefinition::WindowComplexShade(_) => MaterialKind::WindowComplexShade,
            MaterialDefinition::WindowGas(_) => MaterialKind::WindowGas,
            MaterialDefinition::WindowGapEquivalentLayer(_) => {
                MaterialKind::WindowGapEquivalentLayer
            }
            MaterialDefinition::WindowGasMixture(_) => MaterialKind::WindowGasMixture,
            MaterialDefinition::WindowShade(_) => MaterialKind::WindowShade,
            MaterialDefinition::WindowShadeEquivalentLayer(_) => {
                MaterialKind::WindowShadeEquivalentLayer
            }
            MaterialDefinition::WindowDrapeEquivalentLayer(_) => {
                MaterialKind::WindowDrapeEquivalentLayer
            }
            MaterialDefinition::WindowScreen(_) => MaterialKind::WindowScreen,
            MaterialDefinition::WindowScreenEquivalentLayer(_) => {
                MaterialKind::WindowScreenEquivalentLayer
            }
            MaterialDefinition::WindowBlind(_) => MaterialKind::WindowBlind,
            MaterialDefinition::WindowBlindEquivalentLayer(_) => {
                MaterialKind::WindowBlindEquivalentLayer
            }
            MaterialDefinition::RoofVegetation(_) => MaterialKind::RoofVegetation,
        }
    }

    /// Returns the high-level consumer family.
    #[must_use]
    pub const fn family(&self) -> MaterialFamily {
        match self.definition {
            MaterialDefinition::Regular(_)
            | MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_)
            | MaterialDefinition::InfraredTransparent(_)
            | MaterialDefinition::RoofVegetation(_) => MaterialFamily::Opaque,
            MaterialDefinition::WindowGlazingSpectralAverage(_)
            | MaterialDefinition::WindowGlazingRefractionExtinction(_)
            | MaterialDefinition::WindowGas(_)
            | MaterialDefinition::WindowGasMixture(_)
            | MaterialDefinition::WindowShade(_)
            | MaterialDefinition::WindowScreen(_)
            | MaterialDefinition::WindowBlind(_) => MaterialFamily::Fenestration,
            MaterialDefinition::WindowGlazingEquivalentLayer(_)
            | MaterialDefinition::WindowGapEquivalentLayer(_)
            | MaterialDefinition::WindowShadeEquivalentLayer(_)
            | MaterialDefinition::WindowDrapeEquivalentLayer(_)
            | MaterialDefinition::WindowScreenEquivalentLayer(_)
            | MaterialDefinition::WindowBlindEquivalentLayer(_) => MaterialFamily::EquivalentLayer,
            MaterialDefinition::WindowGlazingThermochromicGroup(_) => {
                MaterialFamily::ThermochromicGroup
            }
            MaterialDefinition::WindowSimpleGlazing(_) => MaterialFamily::SimpleGlazing,
            MaterialDefinition::WindowComplexGap(_) | MaterialDefinition::WindowComplexShade(_) => {
                MaterialFamily::ComplexFenestration
            }
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
            MaterialDefinition::RoofVegetation(material) => {
                Some(OpaqueMaterialRef::RoofVegetation(material))
            }
            MaterialDefinition::WindowGlazingSpectralAverage(_)
            | MaterialDefinition::WindowGlazingRefractionExtinction(_)
            | MaterialDefinition::WindowGlazingEquivalentLayer(_)
            | MaterialDefinition::WindowGlazingThermochromicGroup(_)
            | MaterialDefinition::WindowSimpleGlazing(_)
            | MaterialDefinition::WindowComplexGap(_)
            | MaterialDefinition::WindowComplexShade(_)
            | MaterialDefinition::WindowGas(_)
            | MaterialDefinition::WindowGasMixture(_)
            | MaterialDefinition::WindowShade(_)
            | MaterialDefinition::WindowGapEquivalentLayer(_)
            | MaterialDefinition::WindowShadeEquivalentLayer(_)
            | MaterialDefinition::WindowDrapeEquivalentLayer(_)
            | MaterialDefinition::WindowScreen(_)
            | MaterialDefinition::WindowScreenEquivalentLayer(_)
            | MaterialDefinition::WindowBlind(_)
            | MaterialDefinition::WindowBlindEquivalentLayer(_) => None,
        }
    }

    /// Borrows the vegetated-roof payload when applicable.
    #[must_use]
    pub const fn as_roof_vegetation(&self) -> Option<&RoofVegetationMaterial> {
        match &self.definition {
            MaterialDefinition::RoofVegetation(material) => Some(material),
            MaterialDefinition::Regular(_)
            | MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_)
            | MaterialDefinition::InfraredTransparent(_)
            | MaterialDefinition::WindowGlazingSpectralAverage(_)
            | MaterialDefinition::WindowGlazingRefractionExtinction(_)
            | MaterialDefinition::WindowGlazingEquivalentLayer(_)
            | MaterialDefinition::WindowGlazingThermochromicGroup(_)
            | MaterialDefinition::WindowSimpleGlazing(_)
            | MaterialDefinition::WindowComplexGap(_)
            | MaterialDefinition::WindowComplexShade(_)
            | MaterialDefinition::WindowGas(_)
            | MaterialDefinition::WindowGasMixture(_)
            | MaterialDefinition::WindowShade(_)
            | MaterialDefinition::WindowGapEquivalentLayer(_)
            | MaterialDefinition::WindowShadeEquivalentLayer(_)
            | MaterialDefinition::WindowDrapeEquivalentLayer(_)
            | MaterialDefinition::WindowScreen(_)
            | MaterialDefinition::WindowScreenEquivalentLayer(_)
            | MaterialDefinition::WindowBlind(_)
            | MaterialDefinition::WindowBlindEquivalentLayer(_) => None,
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
            | MaterialDefinition::RoofVegetation(_)
            | MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_)
            | MaterialDefinition::InfraredTransparent(_)
            | MaterialDefinition::WindowGlazingRefractionExtinction(_)
            | MaterialDefinition::WindowGlazingEquivalentLayer(_)
            | MaterialDefinition::WindowGlazingThermochromicGroup(_)
            | MaterialDefinition::WindowSimpleGlazing(_)
            | MaterialDefinition::WindowComplexGap(_)
            | MaterialDefinition::WindowComplexShade(_)
            | MaterialDefinition::WindowGas(_)
            | MaterialDefinition::WindowGasMixture(_)
            | MaterialDefinition::WindowShade(_)
            | MaterialDefinition::WindowGapEquivalentLayer(_)
            | MaterialDefinition::WindowShadeEquivalentLayer(_)
            | MaterialDefinition::WindowDrapeEquivalentLayer(_)
            | MaterialDefinition::WindowScreen(_)
            | MaterialDefinition::WindowScreenEquivalentLayer(_)
            | MaterialDefinition::WindowBlind(_)
            | MaterialDefinition::WindowBlindEquivalentLayer(_) => None,
        }
    }

    /// Borrows the refraction/extinction glazing payload when applicable.
    #[must_use]
    pub const fn as_window_glazing_refraction_extinction(
        &self,
    ) -> Option<&WindowGlazingRefractionExtinctionMaterial> {
        match &self.definition {
            MaterialDefinition::WindowGlazingRefractionExtinction(material) => Some(material),
            MaterialDefinition::Regular(_)
            | MaterialDefinition::RoofVegetation(_)
            | MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_)
            | MaterialDefinition::InfraredTransparent(_)
            | MaterialDefinition::WindowGlazingSpectralAverage(_)
            | MaterialDefinition::WindowGlazingEquivalentLayer(_)
            | MaterialDefinition::WindowGlazingThermochromicGroup(_)
            | MaterialDefinition::WindowSimpleGlazing(_)
            | MaterialDefinition::WindowComplexGap(_)
            | MaterialDefinition::WindowComplexShade(_)
            | MaterialDefinition::WindowGas(_)
            | MaterialDefinition::WindowGasMixture(_)
            | MaterialDefinition::WindowShade(_)
            | MaterialDefinition::WindowGapEquivalentLayer(_)
            | MaterialDefinition::WindowShadeEquivalentLayer(_)
            | MaterialDefinition::WindowDrapeEquivalentLayer(_)
            | MaterialDefinition::WindowScreen(_)
            | MaterialDefinition::WindowScreenEquivalentLayer(_)
            | MaterialDefinition::WindowBlind(_)
            | MaterialDefinition::WindowBlindEquivalentLayer(_) => None,
        }
    }

    /// Borrows the equivalent-layer glazing payload when applicable.
    #[must_use]
    pub const fn as_window_glazing_equivalent_layer(
        &self,
    ) -> Option<&WindowGlazingEquivalentLayerMaterial> {
        match &self.definition {
            MaterialDefinition::WindowGlazingEquivalentLayer(material) => Some(material),
            MaterialDefinition::Regular(_)
            | MaterialDefinition::RoofVegetation(_)
            | MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_)
            | MaterialDefinition::InfraredTransparent(_)
            | MaterialDefinition::WindowGlazingSpectralAverage(_)
            | MaterialDefinition::WindowGlazingRefractionExtinction(_)
            | MaterialDefinition::WindowGlazingThermochromicGroup(_)
            | MaterialDefinition::WindowSimpleGlazing(_)
            | MaterialDefinition::WindowComplexGap(_)
            | MaterialDefinition::WindowComplexShade(_)
            | MaterialDefinition::WindowGas(_)
            | MaterialDefinition::WindowGasMixture(_)
            | MaterialDefinition::WindowShade(_)
            | MaterialDefinition::WindowGapEquivalentLayer(_)
            | MaterialDefinition::WindowShadeEquivalentLayer(_)
            | MaterialDefinition::WindowDrapeEquivalentLayer(_)
            | MaterialDefinition::WindowScreen(_)
            | MaterialDefinition::WindowScreenEquivalentLayer(_)
            | MaterialDefinition::WindowBlind(_)
            | MaterialDefinition::WindowBlindEquivalentLayer(_) => None,
        }
    }

    /// Borrows the thermochromic glazing-group descriptor when applicable.
    #[must_use]
    pub const fn as_window_glazing_thermochromic_group(
        &self,
    ) -> Option<&WindowGlazingThermochromicGroupMaterial> {
        match &self.definition {
            MaterialDefinition::WindowGlazingThermochromicGroup(material) => Some(material),
            MaterialDefinition::Regular(_)
            | MaterialDefinition::RoofVegetation(_)
            | MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_)
            | MaterialDefinition::InfraredTransparent(_)
            | MaterialDefinition::WindowGlazingSpectralAverage(_)
            | MaterialDefinition::WindowGlazingRefractionExtinction(_)
            | MaterialDefinition::WindowGlazingEquivalentLayer(_)
            | MaterialDefinition::WindowSimpleGlazing(_)
            | MaterialDefinition::WindowComplexGap(_)
            | MaterialDefinition::WindowComplexShade(_)
            | MaterialDefinition::WindowGas(_)
            | MaterialDefinition::WindowGasMixture(_)
            | MaterialDefinition::WindowShade(_)
            | MaterialDefinition::WindowGapEquivalentLayer(_)
            | MaterialDefinition::WindowShadeEquivalentLayer(_)
            | MaterialDefinition::WindowDrapeEquivalentLayer(_)
            | MaterialDefinition::WindowScreen(_)
            | MaterialDefinition::WindowScreenEquivalentLayer(_)
            | MaterialDefinition::WindowBlind(_)
            | MaterialDefinition::WindowBlindEquivalentLayer(_) => None,
        }
    }

    /// Borrows the simple glazing-system payload when applicable.
    #[must_use]
    pub const fn as_window_simple_glazing(&self) -> Option<&WindowSimpleGlazingMaterial> {
        match &self.definition {
            MaterialDefinition::WindowSimpleGlazing(material) => Some(material),
            MaterialDefinition::Regular(_)
            | MaterialDefinition::RoofVegetation(_)
            | MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_)
            | MaterialDefinition::InfraredTransparent(_)
            | MaterialDefinition::WindowGlazingSpectralAverage(_)
            | MaterialDefinition::WindowGlazingRefractionExtinction(_)
            | MaterialDefinition::WindowGlazingEquivalentLayer(_)
            | MaterialDefinition::WindowGlazingThermochromicGroup(_)
            | MaterialDefinition::WindowComplexGap(_)
            | MaterialDefinition::WindowComplexShade(_)
            | MaterialDefinition::WindowGas(_)
            | MaterialDefinition::WindowGasMixture(_)
            | MaterialDefinition::WindowShade(_)
            | MaterialDefinition::WindowGapEquivalentLayer(_)
            | MaterialDefinition::WindowShadeEquivalentLayer(_)
            | MaterialDefinition::WindowDrapeEquivalentLayer(_)
            | MaterialDefinition::WindowScreen(_)
            | MaterialDefinition::WindowScreenEquivalentLayer(_)
            | MaterialDefinition::WindowBlind(_)
            | MaterialDefinition::WindowBlindEquivalentLayer(_) => None,
        }
    }

    /// Borrows the complex-fenestration window-gap payload when applicable.
    #[must_use]
    pub const fn as_window_complex_gap(&self) -> Option<&WindowComplexGapMaterial> {
        match &self.definition {
            MaterialDefinition::WindowComplexGap(material) => Some(material),
            MaterialDefinition::Regular(_)
            | MaterialDefinition::RoofVegetation(_)
            | MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_)
            | MaterialDefinition::InfraredTransparent(_)
            | MaterialDefinition::WindowGlazingSpectralAverage(_)
            | MaterialDefinition::WindowGlazingRefractionExtinction(_)
            | MaterialDefinition::WindowGlazingEquivalentLayer(_)
            | MaterialDefinition::WindowGlazingThermochromicGroup(_)
            | MaterialDefinition::WindowSimpleGlazing(_)
            | MaterialDefinition::WindowComplexShade(_)
            | MaterialDefinition::WindowGas(_)
            | MaterialDefinition::WindowGasMixture(_)
            | MaterialDefinition::WindowShade(_)
            | MaterialDefinition::WindowGapEquivalentLayer(_)
            | MaterialDefinition::WindowShadeEquivalentLayer(_)
            | MaterialDefinition::WindowDrapeEquivalentLayer(_)
            | MaterialDefinition::WindowScreen(_)
            | MaterialDefinition::WindowScreenEquivalentLayer(_)
            | MaterialDefinition::WindowBlind(_)
            | MaterialDefinition::WindowBlindEquivalentLayer(_) => None,
        }
    }

    /// Borrows the complex-fenestration shade payload when applicable.
    #[must_use]
    pub const fn as_window_complex_shade(&self) -> Option<&WindowComplexShadeMaterial> {
        match &self.definition {
            MaterialDefinition::WindowComplexShade(material) => Some(material),
            MaterialDefinition::Regular(_)
            | MaterialDefinition::RoofVegetation(_)
            | MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_)
            | MaterialDefinition::InfraredTransparent(_)
            | MaterialDefinition::WindowGlazingSpectralAverage(_)
            | MaterialDefinition::WindowGlazingRefractionExtinction(_)
            | MaterialDefinition::WindowGlazingEquivalentLayer(_)
            | MaterialDefinition::WindowGlazingThermochromicGroup(_)
            | MaterialDefinition::WindowSimpleGlazing(_)
            | MaterialDefinition::WindowComplexGap(_)
            | MaterialDefinition::WindowGas(_)
            | MaterialDefinition::WindowGasMixture(_)
            | MaterialDefinition::WindowShade(_)
            | MaterialDefinition::WindowGapEquivalentLayer(_)
            | MaterialDefinition::WindowShadeEquivalentLayer(_)
            | MaterialDefinition::WindowDrapeEquivalentLayer(_)
            | MaterialDefinition::WindowScreen(_)
            | MaterialDefinition::WindowScreenEquivalentLayer(_)
            | MaterialDefinition::WindowBlind(_)
            | MaterialDefinition::WindowBlindEquivalentLayer(_) => None,
        }
    }

    /// Borrows the ordinary window-gas payload when applicable.
    #[must_use]
    pub const fn as_window_gas(&self) -> Option<&WindowGasMaterial> {
        match &self.definition {
            MaterialDefinition::WindowGas(material) => Some(material),
            MaterialDefinition::Regular(_)
            | MaterialDefinition::RoofVegetation(_)
            | MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_)
            | MaterialDefinition::InfraredTransparent(_)
            | MaterialDefinition::WindowGlazingSpectralAverage(_)
            | MaterialDefinition::WindowGlazingRefractionExtinction(_)
            | MaterialDefinition::WindowGlazingEquivalentLayer(_)
            | MaterialDefinition::WindowGlazingThermochromicGroup(_)
            | MaterialDefinition::WindowSimpleGlazing(_)
            | MaterialDefinition::WindowComplexGap(_)
            | MaterialDefinition::WindowComplexShade(_)
            | MaterialDefinition::WindowGasMixture(_)
            | MaterialDefinition::WindowShade(_)
            | MaterialDefinition::WindowGapEquivalentLayer(_)
            | MaterialDefinition::WindowShadeEquivalentLayer(_)
            | MaterialDefinition::WindowDrapeEquivalentLayer(_)
            | MaterialDefinition::WindowScreen(_)
            | MaterialDefinition::WindowScreenEquivalentLayer(_)
            | MaterialDefinition::WindowBlind(_)
            | MaterialDefinition::WindowBlindEquivalentLayer(_) => None,
        }
    }

    /// Borrows the equivalent-layer window-gap payload when applicable.
    #[must_use]
    pub const fn as_window_gap_equivalent_layer(
        &self,
    ) -> Option<&WindowGapEquivalentLayerMaterial> {
        match &self.definition {
            MaterialDefinition::WindowGapEquivalentLayer(material) => Some(material),
            MaterialDefinition::Regular(_)
            | MaterialDefinition::RoofVegetation(_)
            | MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_)
            | MaterialDefinition::InfraredTransparent(_)
            | MaterialDefinition::WindowGlazingSpectralAverage(_)
            | MaterialDefinition::WindowGlazingRefractionExtinction(_)
            | MaterialDefinition::WindowGlazingEquivalentLayer(_)
            | MaterialDefinition::WindowGlazingThermochromicGroup(_)
            | MaterialDefinition::WindowSimpleGlazing(_)
            | MaterialDefinition::WindowComplexGap(_)
            | MaterialDefinition::WindowComplexShade(_)
            | MaterialDefinition::WindowGas(_)
            | MaterialDefinition::WindowGasMixture(_)
            | MaterialDefinition::WindowShade(_)
            | MaterialDefinition::WindowShadeEquivalentLayer(_)
            | MaterialDefinition::WindowDrapeEquivalentLayer(_)
            | MaterialDefinition::WindowScreen(_)
            | MaterialDefinition::WindowScreenEquivalentLayer(_)
            | MaterialDefinition::WindowBlind(_)
            | MaterialDefinition::WindowBlindEquivalentLayer(_) => None,
        }
    }

    /// Borrows the ordinary window gas-mixture payload when applicable.
    #[must_use]
    pub const fn as_window_gas_mixture(&self) -> Option<&WindowGasMixtureMaterial> {
        match &self.definition {
            MaterialDefinition::WindowGasMixture(material) => Some(material),
            MaterialDefinition::Regular(_)
            | MaterialDefinition::RoofVegetation(_)
            | MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_)
            | MaterialDefinition::InfraredTransparent(_)
            | MaterialDefinition::WindowGlazingSpectralAverage(_)
            | MaterialDefinition::WindowGlazingRefractionExtinction(_)
            | MaterialDefinition::WindowGlazingEquivalentLayer(_)
            | MaterialDefinition::WindowGlazingThermochromicGroup(_)
            | MaterialDefinition::WindowSimpleGlazing(_)
            | MaterialDefinition::WindowComplexGap(_)
            | MaterialDefinition::WindowComplexShade(_)
            | MaterialDefinition::WindowGas(_)
            | MaterialDefinition::WindowShade(_)
            | MaterialDefinition::WindowGapEquivalentLayer(_)
            | MaterialDefinition::WindowShadeEquivalentLayer(_)
            | MaterialDefinition::WindowDrapeEquivalentLayer(_)
            | MaterialDefinition::WindowScreen(_)
            | MaterialDefinition::WindowScreenEquivalentLayer(_)
            | MaterialDefinition::WindowBlind(_)
            | MaterialDefinition::WindowBlindEquivalentLayer(_) => None,
        }
    }

    /// Borrows the ordinary window-shade payload when applicable.
    #[must_use]
    pub const fn as_window_shade(&self) -> Option<&WindowShadeMaterial> {
        match &self.definition {
            MaterialDefinition::WindowShade(material) => Some(material),
            MaterialDefinition::Regular(_)
            | MaterialDefinition::RoofVegetation(_)
            | MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_)
            | MaterialDefinition::InfraredTransparent(_)
            | MaterialDefinition::WindowGlazingSpectralAverage(_)
            | MaterialDefinition::WindowGlazingRefractionExtinction(_)
            | MaterialDefinition::WindowGlazingEquivalentLayer(_)
            | MaterialDefinition::WindowGlazingThermochromicGroup(_)
            | MaterialDefinition::WindowSimpleGlazing(_)
            | MaterialDefinition::WindowComplexGap(_)
            | MaterialDefinition::WindowComplexShade(_)
            | MaterialDefinition::WindowGas(_)
            | MaterialDefinition::WindowGasMixture(_)
            | MaterialDefinition::WindowGapEquivalentLayer(_)
            | MaterialDefinition::WindowShadeEquivalentLayer(_)
            | MaterialDefinition::WindowDrapeEquivalentLayer(_)
            | MaterialDefinition::WindowScreen(_)
            | MaterialDefinition::WindowScreenEquivalentLayer(_)
            | MaterialDefinition::WindowBlind(_)
            | MaterialDefinition::WindowBlindEquivalentLayer(_) => None,
        }
    }

    /// Borrows the equivalent-layer window-shade payload when applicable.
    #[must_use]
    pub const fn as_window_shade_equivalent_layer(
        &self,
    ) -> Option<&WindowShadeEquivalentLayerMaterial> {
        match &self.definition {
            MaterialDefinition::WindowShadeEquivalentLayer(material) => Some(material),
            MaterialDefinition::Regular(_)
            | MaterialDefinition::RoofVegetation(_)
            | MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_)
            | MaterialDefinition::InfraredTransparent(_)
            | MaterialDefinition::WindowGlazingSpectralAverage(_)
            | MaterialDefinition::WindowGlazingRefractionExtinction(_)
            | MaterialDefinition::WindowGlazingEquivalentLayer(_)
            | MaterialDefinition::WindowGlazingThermochromicGroup(_)
            | MaterialDefinition::WindowSimpleGlazing(_)
            | MaterialDefinition::WindowComplexGap(_)
            | MaterialDefinition::WindowComplexShade(_)
            | MaterialDefinition::WindowGas(_)
            | MaterialDefinition::WindowGapEquivalentLayer(_)
            | MaterialDefinition::WindowGasMixture(_)
            | MaterialDefinition::WindowShade(_)
            | MaterialDefinition::WindowDrapeEquivalentLayer(_)
            | MaterialDefinition::WindowScreen(_)
            | MaterialDefinition::WindowScreenEquivalentLayer(_)
            | MaterialDefinition::WindowBlind(_)
            | MaterialDefinition::WindowBlindEquivalentLayer(_) => None,
        }
    }

    /// Borrows the equivalent-layer window-drape payload when applicable.
    #[must_use]
    pub const fn as_window_drape_equivalent_layer(
        &self,
    ) -> Option<&WindowDrapeEquivalentLayerMaterial> {
        match &self.definition {
            MaterialDefinition::WindowDrapeEquivalentLayer(material) => Some(material),
            MaterialDefinition::Regular(_)
            | MaterialDefinition::RoofVegetation(_)
            | MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_)
            | MaterialDefinition::InfraredTransparent(_)
            | MaterialDefinition::WindowGlazingSpectralAverage(_)
            | MaterialDefinition::WindowGlazingRefractionExtinction(_)
            | MaterialDefinition::WindowGlazingEquivalentLayer(_)
            | MaterialDefinition::WindowGlazingThermochromicGroup(_)
            | MaterialDefinition::WindowSimpleGlazing(_)
            | MaterialDefinition::WindowComplexGap(_)
            | MaterialDefinition::WindowComplexShade(_)
            | MaterialDefinition::WindowGas(_)
            | MaterialDefinition::WindowGapEquivalentLayer(_)
            | MaterialDefinition::WindowGasMixture(_)
            | MaterialDefinition::WindowShade(_)
            | MaterialDefinition::WindowShadeEquivalentLayer(_)
            | MaterialDefinition::WindowScreen(_)
            | MaterialDefinition::WindowScreenEquivalentLayer(_)
            | MaterialDefinition::WindowBlind(_)
            | MaterialDefinition::WindowBlindEquivalentLayer(_) => None,
        }
    }

    /// Borrows the ordinary window-screen payload when applicable.
    #[must_use]
    pub const fn as_window_screen(&self) -> Option<&WindowScreenMaterial> {
        match &self.definition {
            MaterialDefinition::WindowScreen(material) => Some(material),
            MaterialDefinition::Regular(_)
            | MaterialDefinition::RoofVegetation(_)
            | MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_)
            | MaterialDefinition::InfraredTransparent(_)
            | MaterialDefinition::WindowGlazingSpectralAverage(_)
            | MaterialDefinition::WindowGlazingRefractionExtinction(_)
            | MaterialDefinition::WindowGlazingEquivalentLayer(_)
            | MaterialDefinition::WindowGlazingThermochromicGroup(_)
            | MaterialDefinition::WindowSimpleGlazing(_)
            | MaterialDefinition::WindowComplexGap(_)
            | MaterialDefinition::WindowComplexShade(_)
            | MaterialDefinition::WindowGas(_)
            | MaterialDefinition::WindowGapEquivalentLayer(_)
            | MaterialDefinition::WindowGasMixture(_)
            | MaterialDefinition::WindowShade(_)
            | MaterialDefinition::WindowShadeEquivalentLayer(_)
            | MaterialDefinition::WindowDrapeEquivalentLayer(_)
            | MaterialDefinition::WindowScreenEquivalentLayer(_)
            | MaterialDefinition::WindowBlind(_)
            | MaterialDefinition::WindowBlindEquivalentLayer(_) => None,
        }
    }

    /// Borrows the equivalent-layer window-screen payload when applicable.
    #[must_use]
    pub const fn as_window_screen_equivalent_layer(
        &self,
    ) -> Option<&WindowScreenEquivalentLayerMaterial> {
        match &self.definition {
            MaterialDefinition::WindowScreenEquivalentLayer(material) => Some(material),
            MaterialDefinition::Regular(_)
            | MaterialDefinition::RoofVegetation(_)
            | MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_)
            | MaterialDefinition::InfraredTransparent(_)
            | MaterialDefinition::WindowGlazingSpectralAverage(_)
            | MaterialDefinition::WindowGlazingRefractionExtinction(_)
            | MaterialDefinition::WindowGlazingEquivalentLayer(_)
            | MaterialDefinition::WindowGlazingThermochromicGroup(_)
            | MaterialDefinition::WindowSimpleGlazing(_)
            | MaterialDefinition::WindowComplexGap(_)
            | MaterialDefinition::WindowComplexShade(_)
            | MaterialDefinition::WindowGas(_)
            | MaterialDefinition::WindowGapEquivalentLayer(_)
            | MaterialDefinition::WindowGasMixture(_)
            | MaterialDefinition::WindowShade(_)
            | MaterialDefinition::WindowShadeEquivalentLayer(_)
            | MaterialDefinition::WindowDrapeEquivalentLayer(_)
            | MaterialDefinition::WindowScreen(_)
            | MaterialDefinition::WindowBlind(_)
            | MaterialDefinition::WindowBlindEquivalentLayer(_) => None,
        }
    }

    /// Borrows the ordinary window-blind payload when applicable.
    #[must_use]
    pub const fn as_window_blind(&self) -> Option<&WindowBlindMaterial> {
        match &self.definition {
            MaterialDefinition::WindowBlind(material) => Some(material),
            MaterialDefinition::Regular(_)
            | MaterialDefinition::RoofVegetation(_)
            | MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_)
            | MaterialDefinition::InfraredTransparent(_)
            | MaterialDefinition::WindowGlazingSpectralAverage(_)
            | MaterialDefinition::WindowGlazingRefractionExtinction(_)
            | MaterialDefinition::WindowGlazingEquivalentLayer(_)
            | MaterialDefinition::WindowGlazingThermochromicGroup(_)
            | MaterialDefinition::WindowSimpleGlazing(_)
            | MaterialDefinition::WindowComplexGap(_)
            | MaterialDefinition::WindowComplexShade(_)
            | MaterialDefinition::WindowGas(_)
            | MaterialDefinition::WindowGapEquivalentLayer(_)
            | MaterialDefinition::WindowGasMixture(_)
            | MaterialDefinition::WindowShade(_)
            | MaterialDefinition::WindowShadeEquivalentLayer(_)
            | MaterialDefinition::WindowDrapeEquivalentLayer(_)
            | MaterialDefinition::WindowScreen(_)
            | MaterialDefinition::WindowScreenEquivalentLayer(_)
            | MaterialDefinition::WindowBlindEquivalentLayer(_) => None,
        }
    }

    /// Borrows the equivalent-layer window-blind payload when applicable.
    #[must_use]
    pub const fn as_window_blind_equivalent_layer(
        &self,
    ) -> Option<&WindowBlindEquivalentLayerMaterial> {
        match &self.definition {
            MaterialDefinition::WindowBlindEquivalentLayer(material) => Some(material),
            MaterialDefinition::Regular(_)
            | MaterialDefinition::RoofVegetation(_)
            | MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_)
            | MaterialDefinition::InfraredTransparent(_)
            | MaterialDefinition::WindowGlazingSpectralAverage(_)
            | MaterialDefinition::WindowGlazingRefractionExtinction(_)
            | MaterialDefinition::WindowGlazingEquivalentLayer(_)
            | MaterialDefinition::WindowGlazingThermochromicGroup(_)
            | MaterialDefinition::WindowSimpleGlazing(_)
            | MaterialDefinition::WindowComplexGap(_)
            | MaterialDefinition::WindowComplexShade(_)
            | MaterialDefinition::WindowGas(_)
            | MaterialDefinition::WindowGapEquivalentLayer(_)
            | MaterialDefinition::WindowGasMixture(_)
            | MaterialDefinition::WindowShade(_)
            | MaterialDefinition::WindowShadeEquivalentLayer(_)
            | MaterialDefinition::WindowDrapeEquivalentLayer(_)
            | MaterialDefinition::WindowScreen(_)
            | MaterialDefinition::WindowScreenEquivalentLayer(_)
            | MaterialDefinition::WindowBlind(_) => None,
        }
    }

    /// Returns the opaque surface-roughness projection when applicable.
    #[must_use]
    pub fn roughness(&self) -> Option<MaterialSurfaceRoughness> {
        self.as_opaque().and_then(OpaqueMaterialRef::roughness)
    }

    /// Returns the mass-bearing opaque-material thickness when applicable.
    #[must_use]
    pub fn thickness_m(&self) -> Option<f64> {
        self.as_opaque().and_then(OpaqueMaterialRef::thickness_m)
    }

    /// Returns the mass-bearing opaque-material conductivity when applicable.
    #[must_use]
    pub fn conductivity_w_per_m_k(&self) -> Option<f64> {
        self.as_opaque()
            .and_then(OpaqueMaterialRef::conductivity_w_per_m_k)
    }

    /// Returns the mass-bearing opaque-material density when applicable.
    #[must_use]
    pub fn density_kg_per_m3(&self) -> Option<f64> {
        self.as_opaque()
            .and_then(OpaqueMaterialRef::density_kg_per_m3)
    }

    /// Returns the mass-bearing opaque-material specific heat when applicable.
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

/// Thermochromic parent metadata retained on an effective construction stack.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConstructionThermochromicMaster {
    /// Thermochromic glazing-group material replaced by its first typed state.
    pub parent_material: MaterialId,
    /// Zero-based construction layer index (EnergyPlus `TCLayerNum` is one-based).
    pub layer_index: u32,
    /// Zero-based source glass-layer ordinal (EnergyPlus `TCGlassNum` is one-based).
    pub glazing_layer_index: u32,
}

/// Construction resolved to an ordered, effective material layer stack.
#[derive(Clone, Debug, PartialEq)]
pub struct Construction {
    /// Typed ID.
    pub id: ConstructionId,
    /// Construction name.
    pub name: NormalizedName,
    /// Consumer family for this construction.
    pub kind: ConstructionKind,
    /// Effective outside layer material (including first-state TC substitution).
    pub outside_layer: MaterialId,
    /// Ordered material layers from outside to inside.
    pub layers: Vec<MaterialId>,
    /// Source-style thermochromic master metadata for the last group parent in the stack.
    ///
    /// The effective layer stack contains the parent's first glazing state. Generating
    /// thermochromic child constructions and selecting states at runtime remain deferred.
    pub thermochromic_master: Option<ConstructionThermochromicMaster>,
}
