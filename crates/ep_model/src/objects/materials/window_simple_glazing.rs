use super::MaterialSurfaceRoughness;

/// Source-effective payload for a `WindowMaterial:SimpleGlazingSystem` object.
///
/// The derived properties reproduce EnergyPlus 26.1
/// `MaterialGlass::SetupSimpleWindowGlazingSystem`. They describe the equivalent
/// single glass layer at normal incidence; the separate angular-dependence
/// algorithm in `WindowManager::TransAndReflAtPhi` is intentionally outside this
/// type's boundary.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowSimpleGlazingMaterial {
    /// User U-factor including film coefficients in W/m2-K.
    pub u_factor_with_film_coefficients_w_per_m2_k: f64,
    /// User solar heat-gain coefficient at normal incidence.
    pub solar_heat_gain_coefficient: f64,
    /// User visible transmittance, or `None` when EnergyPlus derives it from
    /// the solar properties.
    pub input_visible_transmittance_at_normal_incidence: Option<f64>,
    /// Source-fixed equivalent-layer roughness.
    pub roughness: MaterialSurfaceRoughness,
    /// Equivalent-layer area-normalized thermal resistance in m2-K/W.
    pub thermal_resistance_m2_k_per_w: f64,
    /// Equivalent-layer thickness in meters.
    pub thickness_m: f64,
    /// Equivalent-layer conductivity in W/m-K.
    pub conductivity_w_per_m_k: f64,
    /// Whether EnergyPlus replaced a non-positive layer resistance with
    /// 0.001 m2-K/W and emits its high-U-factor warning.
    pub film_resistance_clamped: bool,
    /// Solar transmittance at normal incidence.
    pub solar_transmittance_at_normal_incidence: f64,
    /// Front-side solar reflectance at normal incidence.
    pub front_side_solar_reflectance_at_normal_incidence: f64,
    /// Back-side solar reflectance at normal incidence.
    pub back_side_solar_reflectance_at_normal_incidence: f64,
    /// Visible transmittance at normal incidence after default application.
    pub visible_transmittance_at_normal_incidence: f64,
    /// Front-side visible reflectance at normal incidence.
    pub front_side_visible_reflectance_at_normal_incidence: f64,
    /// Back-side visible reflectance at normal incidence.
    pub back_side_visible_reflectance_at_normal_incidence: f64,
    /// Source-fixed infrared transmittance at normal incidence.
    pub infrared_transmittance_at_normal_incidence: f64,
    /// Source-fixed front-side infrared hemispherical emissivity.
    pub front_side_infrared_hemispherical_emissivity: f64,
    /// Source-fixed back-side infrared hemispherical emissivity.
    pub back_side_infrared_hemispherical_emissivity: f64,
    /// Source-effective thermal absorptance used by generic material reports.
    pub thermal_absorptance: f64,
    /// Source-effective solar absorptance used by generic material reports.
    pub solar_absorptance: f64,
    /// Source-effective visible absorptance used by generic material reports.
    pub visible_absorptance: f64,
    /// Source-fixed dirt correction factor for solar and visible
    /// transmittance.
    pub dirt_correction_factor_for_solar_and_visible_transmittance: f64,
    /// Source-fixed solar-diffusing flag.
    pub solar_diffusing: bool,
}

impl WindowSimpleGlazingMaterial {
    /// Converts the validated EnergyPlus performance indices to the source's
    /// equivalent single-layer properties.
    ///
    /// EnergyPlus schema validation requires positive U-factor and SHGC values,
    /// and constrains SHGC and an explicit visible transmittance below one. This
    /// function intentionally assumes those parser-level constraints.
    #[must_use]
    pub fn from_performance_indices(
        u_factor_with_film_coefficients_w_per_m2_k: f64,
        solar_heat_gain_coefficient: f64,
        input_visible_transmittance_at_normal_incidence: Option<f64>,
    ) -> Self {
        let u_factor = u_factor_with_film_coefficients_w_per_m2_k;

        // Steps 1-3: remove the source-correlated winter film resistances,
        // determine block-layer thickness, and calculate effective conductivity.
        let interior_winter_film_resistance = if u_factor < 5.85 {
            1.0 / (0.359_073 * u_factor.ln() + 6.949_915)
        } else {
            1.0 / (1.788_041 * u_factor - 2.886_625)
        };
        let exterior_winter_film_resistance = 1.0 / (0.025_342 * u_factor + 29.163_853);
        let mut thermal_resistance_m2_k_per_w =
            1.0 / u_factor - interior_winter_film_resistance - exterior_winter_film_resistance;
        let film_resistance_clamped = thermal_resistance_m2_k_per_w <= 0.0;
        if film_resistance_clamped {
            thermal_resistance_m2_k_per_w = thermal_resistance_m2_k_per_w.max(0.001);
        }

        let thickness_m = if 1.0 / thermal_resistance_m2_k_per_w > 7.0 {
            0.002
        } else {
            0.059_14 - 0.007_14 / thermal_resistance_m2_k_per_w
        };
        let conductivity_w_per_m_k = thickness_m / thermal_resistance_m2_k_per_w;

        // Step 4: normal-incidence solar transmittance.
        let high_u_solar_transmittance = if solar_heat_gain_coefficient < 0.7206 {
            0.939_998 * squared(solar_heat_gain_coefficient)
                + 0.203_32 * solar_heat_gain_coefficient
        } else {
            1.304_15 * solar_heat_gain_coefficient - 0.305_15
        };
        let low_u_solar_transmittance = if solar_heat_gain_coefficient <= 0.15 {
            0.410_40 * solar_heat_gain_coefficient
        } else {
            0.085_775 * squared(solar_heat_gain_coefficient)
                + 0.963_954 * solar_heat_gain_coefficient
                - 0.084_958
        };
        let mut solar_transmittance_at_normal_incidence = if u_factor > 4.5 {
            high_u_solar_transmittance
        } else if u_factor < 3.4 {
            low_u_solar_transmittance
        } else {
            ((u_factor - 3.4) / (4.5 - 3.4))
                * (high_u_solar_transmittance - low_u_solar_transmittance)
                + low_u_solar_transmittance
        };
        if solar_transmittance_at_normal_incidence < 0.0 {
            solar_transmittance_at_normal_incidence = 0.0;
        }

        // Step 5: normal-incidence solar reflectance.
        let delta_shgc_and_solar_transmittance =
            solar_heat_gain_coefficient - solar_transmittance_at_normal_incidence;
        let high_u_interior_summer_film_resistance = 1.0
            / (29.436_546 * cubed(delta_shgc_and_solar_transmittance)
                - 21.943_415 * squared(delta_shgc_and_solar_transmittance)
                + 9.945_872 * delta_shgc_and_solar_transmittance
                + 7.426_151);
        let high_u_exterior_summer_film_resistance =
            1.0 / (2.225_824 * delta_shgc_and_solar_transmittance + 20.577_080);
        let low_u_interior_summer_film_resistance = 1.0
            / (199.820_812_8 * cubed(delta_shgc_and_solar_transmittance)
                - 90.639_733 * squared(delta_shgc_and_solar_transmittance)
                + 19.737_055 * delta_shgc_and_solar_transmittance
                + 6.766_575);
        let low_u_exterior_summer_film_resistance =
            1.0 / (5.763_355 * delta_shgc_and_solar_transmittance + 20.541_528);

        let (interior_summer_film_resistance, exterior_summer_film_resistance) = if u_factor > 4.5 {
            (
                high_u_interior_summer_film_resistance,
                high_u_exterior_summer_film_resistance,
            )
        } else if u_factor < 3.4 {
            (
                low_u_interior_summer_film_resistance,
                low_u_exterior_summer_film_resistance,
            )
        } else {
            let interpolation_fraction = (u_factor - 3.4) / (4.5 - 3.4);

            // EnergyPlus 26.1 uses `(low - high) * fraction + low` for
            // both terms. Preserve that source-actual direction exactly.
            (
                interpolation_fraction
                    * (low_u_interior_summer_film_resistance
                        - high_u_interior_summer_film_resistance)
                    + low_u_interior_summer_film_resistance,
                interpolation_fraction
                    * (low_u_exterior_summer_film_resistance
                        - high_u_exterior_summer_film_resistance)
                    + low_u_exterior_summer_film_resistance,
            )
        };
        let inward_flowing_fraction = (exterior_summer_film_resistance
            + 0.5 * thermal_resistance_m2_k_per_w)
            / (exterior_summer_film_resistance
                + thermal_resistance_m2_k_per_w
                + interior_summer_film_resistance);
        let solar_absorptance = (solar_heat_gain_coefficient
            - solar_transmittance_at_normal_incidence)
            / inward_flowing_fraction;
        let solar_reflectance_at_normal_incidence =
            1.0 - solar_transmittance_at_normal_incidence - solar_absorptance;

        // Step 6: optional user visible transmittance or the solar defaults.
        let (
            visible_transmittance_at_normal_incidence,
            front_side_visible_reflectance_at_normal_incidence,
            back_side_visible_reflectance_at_normal_incidence,
        ) = if let Some(visible_transmittance) = input_visible_transmittance_at_normal_incidence {
            let mut back_reflectance = -0.7409 * cubed(visible_transmittance)
                + 1.6531 * squared(visible_transmittance)
                - 1.2299 * visible_transmittance
                + 0.4545;
            if visible_transmittance + back_reflectance >= 1.0 {
                back_reflectance = 0.999 - visible_transmittance;
            }

            let mut front_reflectance = -0.0622 * cubed(visible_transmittance)
                + 0.4277 * squared(visible_transmittance)
                - 0.4169 * visible_transmittance
                + 0.2399;
            if visible_transmittance + front_reflectance >= 1.0 {
                front_reflectance = 0.999 - visible_transmittance;
            }

            (visible_transmittance, front_reflectance, back_reflectance)
        } else {
            (
                solar_transmittance_at_normal_incidence,
                solar_reflectance_at_normal_incidence,
                solar_reflectance_at_normal_incidence,
            )
        };

        Self {
            u_factor_with_film_coefficients_w_per_m2_k,
            solar_heat_gain_coefficient,
            input_visible_transmittance_at_normal_incidence,
            roughness: MaterialSurfaceRoughness::VerySmooth,
            thermal_resistance_m2_k_per_w,
            thickness_m,
            conductivity_w_per_m_k,
            film_resistance_clamped,
            solar_transmittance_at_normal_incidence,
            front_side_solar_reflectance_at_normal_incidence: solar_reflectance_at_normal_incidence,
            back_side_solar_reflectance_at_normal_incidence: solar_reflectance_at_normal_incidence,
            visible_transmittance_at_normal_incidence,
            front_side_visible_reflectance_at_normal_incidence,
            back_side_visible_reflectance_at_normal_incidence,
            infrared_transmittance_at_normal_incidence: 0.0,
            front_side_infrared_hemispherical_emissivity: 0.84,
            back_side_infrared_hemispherical_emissivity: 0.84,
            thermal_absorptance: 0.84,
            solar_absorptance: 0.0,
            visible_absorptance: 0.0,
            dirt_correction_factor_for_solar_and_visible_transmittance: 1.0,
            solar_diffusing: false,
        }
    }
}

fn squared(value: f64) -> f64 {
    value * value
}

fn cubed(value: f64) -> f64 {
    value * value * value
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Material, MaterialDefinition, MaterialFamily, MaterialId, MaterialKind, NormalizedName,
    };

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() <= 1.0e-12,
            "expected {expected:.15}, got {actual:.15}"
        );
    }

    #[test]
    fn low_u_branch_defaults_visible_properties_to_solar_properties() {
        let material = WindowSimpleGlazingMaterial::from_performance_indices(3.0, 0.1, None);

        assert_eq!(material.u_factor_with_film_coefficients_w_per_m2_k, 3.0);
        assert_eq!(material.solar_heat_gain_coefficient, 0.1);
        assert_eq!(
            material.input_visible_transmittance_at_normal_incidence,
            None
        );
        assert_eq!(material.roughness, MaterialSurfaceRoughness::VerySmooth);
        assert!(!material.film_resistance_clamped);
        assert_close(
            material.thermal_resistance_m2_k_per_w,
            0.162_975_248_660_524_9,
        );
        assert_close(material.thickness_m, 0.015_329_666_475_843_107);
        assert_close(material.conductivity_w_per_m_k, 0.094_061_316_683_581_7);
        assert_close(material.solar_transmittance_at_normal_incidence, 0.041_04);
        assert_close(
            material.front_side_solar_reflectance_at_normal_incidence,
            0.803_340_674_201_101_6,
        );
        assert_eq!(
            material.visible_transmittance_at_normal_incidence,
            material.solar_transmittance_at_normal_incidence
        );
        assert_eq!(
            material.front_side_visible_reflectance_at_normal_incidence,
            material.front_side_solar_reflectance_at_normal_incidence
        );
        assert_eq!(
            material.back_side_visible_reflectance_at_normal_incidence,
            material.back_side_solar_reflectance_at_normal_incidence
        );
    }

    #[test]
    fn interpolated_u_branch_preserves_source_actual_resistance_direction() {
        let material = WindowSimpleGlazingMaterial::from_performance_indices(4.0, 0.4, None);

        assert!(!material.film_resistance_clamped);
        assert_close(
            material.thermal_resistance_m2_k_per_w,
            0.081_560_032_537_565_39,
        );
        assert_close(material.thickness_m, 0.002);
        assert_close(material.conductivity_w_per_m_k, 0.024_521_814_640_998_684);
        assert_close(
            material.solar_transmittance_at_normal_incidence,
            0.269_282_189_090_909_1,
        );
        assert_close(
            material.front_side_solar_reflectance_at_normal_incidence,
            0.355_040_580_286_554_6,
        );
        assert_eq!(
            material.front_side_solar_reflectance_at_normal_incidence,
            material.back_side_solar_reflectance_at_normal_incidence
        );
    }

    #[test]
    fn high_u_branch_exposes_film_clamp_and_applies_explicit_visible_input() {
        let material = WindowSimpleGlazingMaterial::from_performance_indices(100.0, 0.4, Some(0.6));

        assert!(material.film_resistance_clamped);
        assert_close(material.thermal_resistance_m2_k_per_w, 0.001);
        assert_close(material.thickness_m, 0.002);
        assert_close(material.conductivity_w_per_m_k, 2.0);
        assert_close(
            material.solar_transmittance_at_normal_incidence,
            0.231_727_680_000_000_05,
        );
        assert_close(
            material.front_side_solar_reflectance_at_normal_incidence,
            0.193_433_983_866_041_71,
        );
        assert_eq!(
            material.input_visible_transmittance_at_normal_incidence,
            Some(0.6)
        );
        assert_close(material.visible_transmittance_at_normal_incidence, 0.6);
        assert_close(
            material.front_side_visible_reflectance_at_normal_incidence,
            0.130_296_800_000_000_02,
        );
        assert_close(
            material.back_side_visible_reflectance_at_normal_incidence,
            0.151_641_600_000_000_04,
        );
    }

    #[test]
    fn source_fixed_infrared_and_reporting_defaults_are_preserved() {
        let material = WindowSimpleGlazingMaterial::from_performance_indices(5.0, 0.8, None);

        assert_close(material.infrared_transmittance_at_normal_incidence, 0.0);
        assert_close(material.front_side_infrared_hemispherical_emissivity, 0.84);
        assert_close(material.back_side_infrared_hemispherical_emissivity, 0.84);
        assert_close(material.thermal_absorptance, 0.84);
        assert_close(material.solar_absorptance, 0.0);
        assert_close(material.visible_absorptance, 0.0);
        assert_close(
            material.dirt_correction_factor_for_solar_and_visible_transmittance,
            1.0,
        );
        assert!(!material.solar_diffusing);
    }

    #[test]
    fn material_identity_uses_the_dedicated_fail_closed_family() {
        let payload = WindowSimpleGlazingMaterial::from_performance_indices(2.7, 0.4, None);
        let material = Material {
            id: MaterialId(0),
            name: NormalizedName::new("Simple Glazing"),
            definition: MaterialDefinition::WindowSimpleGlazing(payload),
        };

        assert_eq!(material.kind(), MaterialKind::WindowSimpleGlazing);
        assert_eq!(material.family(), MaterialFamily::SimpleGlazing);
        assert_eq!(material.family().id(), "simple-glazing");
        assert_eq!(material.as_window_simple_glazing(), Some(&payload));
        assert_eq!(material.as_opaque(), None);
    }
}
