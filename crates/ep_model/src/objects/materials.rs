use crate::{AutoOrNumber, ConstructionId, MaterialId, NormalizedName};

mod window_blind;
mod window_drape_equivalent_layer;
mod window_gas;
mod window_screen;
mod window_screen_equivalent_layer;
mod window_shade;
mod window_shade_equivalent_layer;

pub use window_blind::{
    WindowBlindDirectionalOpticalProperties, WindowBlindMaterial, WindowBlindSlatAngleType,
    WindowBlindSlatOrientation,
};
pub use window_drape_equivalent_layer::WindowDrapeEquivalentLayerMaterial;
pub use window_gas::{
    WindowGapEquivalentLayerMaterial, WindowGapVentType, WindowGasMaterial, WindowGasMixture,
    WindowGasMixtureComponent, WindowGasMixtureMaterial, WindowGasPolynomialCoefficients,
    WindowGasProperties, WindowGasType, WindowStandardGasType,
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
}

impl MaterialFamily {
    /// Stable diagnostic identifier.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Opaque => "opaque",
            Self::Fenestration => "fenestration",
            Self::EquivalentLayer => "equivalent-layer",
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

/// Normal-incidence solar and visible properties derived by EnergyPlus from
/// refraction indices and extinction coefficients.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowGlazingRefractionExtinctionOpticalProperties {
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
}

/// Required and default-applied fields for a
/// `WindowMaterial:Glazing:RefractionExtinctionMethod` object.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowGlazingRefractionExtinctionMaterial {
    /// Glass thickness in meters.
    pub thickness_m: f64,
    /// Solar index of refraction.
    pub solar_index_of_refraction: f64,
    /// Solar extinction coefficient in 1/m.
    pub solar_extinction_coefficient_per_m: f64,
    /// Visible index of refraction.
    pub visible_index_of_refraction: f64,
    /// Visible extinction coefficient in 1/m.
    pub visible_extinction_coefficient_per_m: f64,
    /// Infrared transmittance at normal incidence.
    pub infrared_transmittance_at_normal_incidence: f64,
    /// Shared front/back infrared hemispherical emissivity.
    pub infrared_hemispherical_emissivity: f64,
    /// Glass conductivity in W/m-K.
    pub conductivity_w_per_m_k: f64,
    /// Dirt correction factor for solar and visible transmittance.
    pub dirt_correction_factor_for_solar_and_visible_transmittance: f64,
    /// Whether the glazing is solar diffusing.
    pub solar_diffusing: bool,
}

impl WindowGlazingRefractionExtinctionMaterial {
    /// Applies the EnergyPlus 26.1 normal-incidence refraction/extinction
    /// formulas.
    ///
    /// The visible back reflectance intentionally copies the solar front
    /// reflectance. EnergyPlus 26.1 does that assignment in `GetMaterialData`,
    /// even though the visible front value is calculated separately.
    #[must_use]
    pub fn normal_incidence_optical_properties(
        self,
    ) -> WindowGlazingRefractionExtinctionOpticalProperties {
        let (solar_transmittance, solar_reflectance) = refraction_extinction_band_properties(
            self.thickness_m,
            self.solar_index_of_refraction,
            self.solar_extinction_coefficient_per_m,
        );
        let (visible_transmittance, visible_front_reflectance) =
            refraction_extinction_band_properties(
                self.thickness_m,
                self.visible_index_of_refraction,
                self.visible_extinction_coefficient_per_m,
            );

        WindowGlazingRefractionExtinctionOpticalProperties {
            solar_transmittance_at_normal_incidence: solar_transmittance,
            front_side_solar_reflectance_at_normal_incidence: solar_reflectance,
            back_side_solar_reflectance_at_normal_incidence: solar_reflectance,
            visible_transmittance_at_normal_incidence: visible_transmittance,
            front_side_visible_reflectance_at_normal_incidence: visible_front_reflectance,
            back_side_visible_reflectance_at_normal_incidence: solar_reflectance,
        }
    }
}

fn refraction_extinction_band_properties(
    thickness_m: f64,
    index_of_refraction: f64,
    extinction_coefficient_per_m: f64,
) -> (f64, f64) {
    let interface_ratio = (index_of_refraction - 1.0) / (index_of_refraction + 1.0);
    let reflectivity = interface_ratio * interface_ratio;
    let transmittivity = (-extinction_coefficient_per_m * thickness_m).exp();
    let one_minus_reflectivity = 1.0 - reflectivity;
    let one_minus_reflectivity_squared = one_minus_reflectivity * one_minus_reflectivity;
    let reflectivity_times_transmittivity = reflectivity * transmittivity;
    let denominator = 1.0 - reflectivity_times_transmittivity * reflectivity_times_transmittivity;
    let transmittance = transmittivity * one_minus_reflectivity_squared / denominator;
    let transmittivity_squared = transmittivity * transmittivity;
    let reflectance = reflectivity
        * (1.0 + one_minus_reflectivity_squared * transmittivity_squared / denominator);
    (transmittance, reflectance)
}

/// Front/back beam optical properties for one equivalent-layer band.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowGlazingEquivalentLayerDirectionalProperties {
    /// Front-side transmittance.
    pub front_transmittance: f64,
    /// Back-side transmittance.
    pub back_transmittance: f64,
    /// Front-side reflectance.
    pub front_reflectance: f64,
    /// Back-side reflectance.
    pub back_reflectance: f64,
}

/// Diffuse-diffuse optical properties for one equivalent-layer band.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowGlazingEquivalentLayerDiffuseProperties {
    /// Shared front/back transmittance, or EnergyPlus `Autocalculate`.
    pub transmittance: AutoOrNumber,
    /// Front-side reflectance, or EnergyPlus `Autocalculate`.
    pub front_reflectance: AutoOrNumber,
    /// Back-side reflectance, or EnergyPlus `Autocalculate`.
    pub back_reflectance: AutoOrNumber,
}

/// Beam and diffuse optical properties for one equivalent-layer band.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowGlazingEquivalentLayerOpticalBand {
    /// Direct beam-to-beam properties.
    pub beam_beam: WindowGlazingEquivalentLayerDirectionalProperties,
    /// Beam-to-diffuse properties.
    pub beam_diffuse: WindowGlazingEquivalentLayerDirectionalProperties,
    /// Diffuse-to-diffuse properties.
    pub diffuse_diffuse: WindowGlazingEquivalentLayerDiffuseProperties,
}

/// Required and default-applied fields for a
/// `WindowMaterial:Glazing:EquivalentLayer` object.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowGlazingEquivalentLayerMaterial {
    /// Solar-band properties.
    pub solar: WindowGlazingEquivalentLayerOpticalBand,
    /// Visible-band properties.
    pub visible: WindowGlazingEquivalentLayerOpticalBand,
    /// Shared front/back infrared transmittance.
    pub infrared_transmittance: f64,
    /// Front-side infrared emissivity.
    pub front_infrared_emissivity: f64,
    /// Back-side infrared emissivity.
    pub back_infrared_emissivity: f64,
    /// Area-normalized thermal resistance in m2-K/W.
    pub thermal_resistance_m2_k_per_w: f64,
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
    /// Window glazing whose optical properties are derived from refraction and
    /// extinction inputs.
    WindowGlazingRefractionExtinction(WindowGlazingRefractionExtinctionMaterial),
    /// Equivalent-layer glazing with directional and diffuse optical inputs.
    WindowGlazingEquivalentLayer(WindowGlazingEquivalentLayerMaterial),
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
            MaterialDefinition::WindowGlazingRefractionExtinction(_) => {
                MaterialKind::WindowGlazingRefractionExtinction
            }
            MaterialDefinition::WindowGlazingEquivalentLayer(_) => {
                MaterialKind::WindowGlazingEquivalentLayer
            }
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
            | MaterialDefinition::WindowScreenEquivalentLayer(_) => MaterialFamily::EquivalentLayer,
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
            MaterialDefinition::WindowGlazingSpectralAverage(_)
            | MaterialDefinition::WindowGlazingRefractionExtinction(_)
            | MaterialDefinition::WindowGlazingEquivalentLayer(_)
            | MaterialDefinition::WindowGas(_)
            | MaterialDefinition::WindowGasMixture(_)
            | MaterialDefinition::WindowShade(_)
            | MaterialDefinition::WindowGapEquivalentLayer(_)
            | MaterialDefinition::WindowShadeEquivalentLayer(_)
            | MaterialDefinition::WindowDrapeEquivalentLayer(_)
            | MaterialDefinition::WindowScreen(_)
            | MaterialDefinition::WindowScreenEquivalentLayer(_)
            | MaterialDefinition::WindowBlind(_) => None,
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
            | MaterialDefinition::InfraredTransparent(_)
            | MaterialDefinition::WindowGlazingRefractionExtinction(_)
            | MaterialDefinition::WindowGlazingEquivalentLayer(_)
            | MaterialDefinition::WindowGas(_)
            | MaterialDefinition::WindowGasMixture(_)
            | MaterialDefinition::WindowShade(_)
            | MaterialDefinition::WindowGapEquivalentLayer(_)
            | MaterialDefinition::WindowShadeEquivalentLayer(_)
            | MaterialDefinition::WindowDrapeEquivalentLayer(_)
            | MaterialDefinition::WindowScreen(_)
            | MaterialDefinition::WindowScreenEquivalentLayer(_)
            | MaterialDefinition::WindowBlind(_) => None,
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
            | MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_)
            | MaterialDefinition::InfraredTransparent(_)
            | MaterialDefinition::WindowGlazingSpectralAverage(_)
            | MaterialDefinition::WindowGlazingEquivalentLayer(_)
            | MaterialDefinition::WindowGas(_)
            | MaterialDefinition::WindowGasMixture(_)
            | MaterialDefinition::WindowShade(_)
            | MaterialDefinition::WindowGapEquivalentLayer(_)
            | MaterialDefinition::WindowShadeEquivalentLayer(_)
            | MaterialDefinition::WindowDrapeEquivalentLayer(_)
            | MaterialDefinition::WindowScreen(_)
            | MaterialDefinition::WindowScreenEquivalentLayer(_)
            | MaterialDefinition::WindowBlind(_) => None,
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
            | MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_)
            | MaterialDefinition::InfraredTransparent(_)
            | MaterialDefinition::WindowGlazingSpectralAverage(_)
            | MaterialDefinition::WindowGlazingRefractionExtinction(_)
            | MaterialDefinition::WindowGas(_)
            | MaterialDefinition::WindowGasMixture(_)
            | MaterialDefinition::WindowShade(_)
            | MaterialDefinition::WindowGapEquivalentLayer(_)
            | MaterialDefinition::WindowShadeEquivalentLayer(_)
            | MaterialDefinition::WindowDrapeEquivalentLayer(_)
            | MaterialDefinition::WindowScreen(_)
            | MaterialDefinition::WindowScreenEquivalentLayer(_)
            | MaterialDefinition::WindowBlind(_) => None,
        }
    }

    /// Borrows the ordinary window-gas payload when applicable.
    #[must_use]
    pub const fn as_window_gas(&self) -> Option<&WindowGasMaterial> {
        match &self.definition {
            MaterialDefinition::WindowGas(material) => Some(material),
            MaterialDefinition::Regular(_)
            | MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_)
            | MaterialDefinition::InfraredTransparent(_)
            | MaterialDefinition::WindowGlazingSpectralAverage(_)
            | MaterialDefinition::WindowGlazingRefractionExtinction(_)
            | MaterialDefinition::WindowGlazingEquivalentLayer(_)
            | MaterialDefinition::WindowGasMixture(_)
            | MaterialDefinition::WindowShade(_)
            | MaterialDefinition::WindowGapEquivalentLayer(_)
            | MaterialDefinition::WindowShadeEquivalentLayer(_)
            | MaterialDefinition::WindowDrapeEquivalentLayer(_)
            | MaterialDefinition::WindowScreen(_)
            | MaterialDefinition::WindowScreenEquivalentLayer(_)
            | MaterialDefinition::WindowBlind(_) => None,
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
            | MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_)
            | MaterialDefinition::InfraredTransparent(_)
            | MaterialDefinition::WindowGlazingSpectralAverage(_)
            | MaterialDefinition::WindowGlazingRefractionExtinction(_)
            | MaterialDefinition::WindowGlazingEquivalentLayer(_)
            | MaterialDefinition::WindowGas(_)
            | MaterialDefinition::WindowGasMixture(_)
            | MaterialDefinition::WindowShade(_)
            | MaterialDefinition::WindowShadeEquivalentLayer(_)
            | MaterialDefinition::WindowDrapeEquivalentLayer(_)
            | MaterialDefinition::WindowScreen(_)
            | MaterialDefinition::WindowScreenEquivalentLayer(_)
            | MaterialDefinition::WindowBlind(_) => None,
        }
    }

    /// Borrows the ordinary window gas-mixture payload when applicable.
    #[must_use]
    pub const fn as_window_gas_mixture(&self) -> Option<&WindowGasMixtureMaterial> {
        match &self.definition {
            MaterialDefinition::WindowGasMixture(material) => Some(material),
            MaterialDefinition::Regular(_)
            | MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_)
            | MaterialDefinition::InfraredTransparent(_)
            | MaterialDefinition::WindowGlazingSpectralAverage(_)
            | MaterialDefinition::WindowGlazingRefractionExtinction(_)
            | MaterialDefinition::WindowGlazingEquivalentLayer(_)
            | MaterialDefinition::WindowGas(_)
            | MaterialDefinition::WindowShade(_)
            | MaterialDefinition::WindowGapEquivalentLayer(_)
            | MaterialDefinition::WindowShadeEquivalentLayer(_)
            | MaterialDefinition::WindowDrapeEquivalentLayer(_)
            | MaterialDefinition::WindowScreen(_)
            | MaterialDefinition::WindowScreenEquivalentLayer(_)
            | MaterialDefinition::WindowBlind(_) => None,
        }
    }

    /// Borrows the ordinary window-shade payload when applicable.
    #[must_use]
    pub const fn as_window_shade(&self) -> Option<&WindowShadeMaterial> {
        match &self.definition {
            MaterialDefinition::WindowShade(material) => Some(material),
            MaterialDefinition::Regular(_)
            | MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_)
            | MaterialDefinition::InfraredTransparent(_)
            | MaterialDefinition::WindowGlazingSpectralAverage(_)
            | MaterialDefinition::WindowGlazingRefractionExtinction(_)
            | MaterialDefinition::WindowGlazingEquivalentLayer(_)
            | MaterialDefinition::WindowGas(_)
            | MaterialDefinition::WindowGasMixture(_)
            | MaterialDefinition::WindowGapEquivalentLayer(_)
            | MaterialDefinition::WindowShadeEquivalentLayer(_)
            | MaterialDefinition::WindowDrapeEquivalentLayer(_)
            | MaterialDefinition::WindowScreen(_)
            | MaterialDefinition::WindowScreenEquivalentLayer(_)
            | MaterialDefinition::WindowBlind(_) => None,
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
            | MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_)
            | MaterialDefinition::InfraredTransparent(_)
            | MaterialDefinition::WindowGlazingSpectralAverage(_)
            | MaterialDefinition::WindowGlazingRefractionExtinction(_)
            | MaterialDefinition::WindowGlazingEquivalentLayer(_)
            | MaterialDefinition::WindowGas(_)
            | MaterialDefinition::WindowGapEquivalentLayer(_)
            | MaterialDefinition::WindowGasMixture(_)
            | MaterialDefinition::WindowShade(_)
            | MaterialDefinition::WindowDrapeEquivalentLayer(_)
            | MaterialDefinition::WindowScreen(_)
            | MaterialDefinition::WindowScreenEquivalentLayer(_)
            | MaterialDefinition::WindowBlind(_) => None,
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
            | MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_)
            | MaterialDefinition::InfraredTransparent(_)
            | MaterialDefinition::WindowGlazingSpectralAverage(_)
            | MaterialDefinition::WindowGlazingRefractionExtinction(_)
            | MaterialDefinition::WindowGlazingEquivalentLayer(_)
            | MaterialDefinition::WindowGas(_)
            | MaterialDefinition::WindowGapEquivalentLayer(_)
            | MaterialDefinition::WindowGasMixture(_)
            | MaterialDefinition::WindowShade(_)
            | MaterialDefinition::WindowShadeEquivalentLayer(_)
            | MaterialDefinition::WindowScreen(_)
            | MaterialDefinition::WindowScreenEquivalentLayer(_)
            | MaterialDefinition::WindowBlind(_) => None,
        }
    }

    /// Borrows the ordinary window-screen payload when applicable.
    #[must_use]
    pub const fn as_window_screen(&self) -> Option<&WindowScreenMaterial> {
        match &self.definition {
            MaterialDefinition::WindowScreen(material) => Some(material),
            MaterialDefinition::Regular(_)
            | MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_)
            | MaterialDefinition::InfraredTransparent(_)
            | MaterialDefinition::WindowGlazingSpectralAverage(_)
            | MaterialDefinition::WindowGlazingRefractionExtinction(_)
            | MaterialDefinition::WindowGlazingEquivalentLayer(_)
            | MaterialDefinition::WindowGas(_)
            | MaterialDefinition::WindowGapEquivalentLayer(_)
            | MaterialDefinition::WindowGasMixture(_)
            | MaterialDefinition::WindowShade(_)
            | MaterialDefinition::WindowShadeEquivalentLayer(_)
            | MaterialDefinition::WindowDrapeEquivalentLayer(_)
            | MaterialDefinition::WindowScreenEquivalentLayer(_)
            | MaterialDefinition::WindowBlind(_) => None,
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
            | MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_)
            | MaterialDefinition::InfraredTransparent(_)
            | MaterialDefinition::WindowGlazingSpectralAverage(_)
            | MaterialDefinition::WindowGlazingRefractionExtinction(_)
            | MaterialDefinition::WindowGlazingEquivalentLayer(_)
            | MaterialDefinition::WindowGas(_)
            | MaterialDefinition::WindowGapEquivalentLayer(_)
            | MaterialDefinition::WindowGasMixture(_)
            | MaterialDefinition::WindowShade(_)
            | MaterialDefinition::WindowShadeEquivalentLayer(_)
            | MaterialDefinition::WindowDrapeEquivalentLayer(_)
            | MaterialDefinition::WindowScreen(_)
            | MaterialDefinition::WindowBlind(_) => None,
        }
    }

    /// Borrows the ordinary window-blind payload when applicable.
    #[must_use]
    pub const fn as_window_blind(&self) -> Option<&WindowBlindMaterial> {
        match &self.definition {
            MaterialDefinition::WindowBlind(material) => Some(material),
            MaterialDefinition::Regular(_)
            | MaterialDefinition::NoMass(_)
            | MaterialDefinition::AirGap(_)
            | MaterialDefinition::InfraredTransparent(_)
            | MaterialDefinition::WindowGlazingSpectralAverage(_)
            | MaterialDefinition::WindowGlazingRefractionExtinction(_)
            | MaterialDefinition::WindowGlazingEquivalentLayer(_)
            | MaterialDefinition::WindowGas(_)
            | MaterialDefinition::WindowGapEquivalentLayer(_)
            | MaterialDefinition::WindowGasMixture(_)
            | MaterialDefinition::WindowShade(_)
            | MaterialDefinition::WindowShadeEquivalentLayer(_)
            | MaterialDefinition::WindowDrapeEquivalentLayer(_)
            | MaterialDefinition::WindowScreen(_)
            | MaterialDefinition::WindowScreenEquivalentLayer(_) => None,
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
