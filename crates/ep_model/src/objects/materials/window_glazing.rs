use crate::{AutoOrNumber, MaterialId};

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

/// One ordered optical state in a thermochromic glazing group.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowGlazingThermochromicState {
    /// Optical-data temperature in degrees Celsius.
    pub optical_data_temperature_c: f64,
    /// Resolved ordinary glazing material used at this temperature.
    pub glazing_material: MaterialId,
}

/// Range descriptor for a thermochromic group's states in the model arena.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WindowGlazingThermochromicGroupMaterial {
    /// Index of the first group state in the model arena.
    pub first_state: u32,
    /// Number of ordered states in the group.
    pub state_count: u32,
}
