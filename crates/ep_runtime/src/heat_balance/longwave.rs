//! Exterior longwave exchange helpers for CalcHeatBalanceOutsideSurf.

use crate::heat_balance::radiation::{KELVIN_OFFSET, STEFAN_BOLTZMANN_W_PER_M2_K4};
use crate::heat_balance::solar::{
    surface_air_sky_radiation_split, surface_ground_view_factor, surface_sky_view_factor,
};
use crate::heat_balance::state::SurfaceHeatBalanceState;
use ep_model::Surface;

/// EnergyPlus source-order owner for exterior longwave exchange terms.
pub const EXTERIOR_LONGWAVE_OWNER_STAGE: &str = "CalcHeatBalanceOutsideSurf";

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ExteriorLongwaveTerms {
    pub(crate) sky_coefficient_w_per_m2_k: f64,
    pub(crate) air_coefficient_w_per_m2_k: f64,
    pub(crate) ground_coefficient_w_per_m2_k: f64,
    pub(crate) sky_temperature_c: f64,
    pub(crate) air_temperature_c: f64,
    pub(crate) ground_temperature_c: f64,
}

impl ExteriorLongwaveTerms {
    pub(crate) fn equivalent_coefficient_w_per_m2_k(self) -> f64 {
        self.sky_coefficient_w_per_m2_k
            + self.air_coefficient_w_per_m2_k
            + self.ground_coefficient_w_per_m2_k
    }

    pub(crate) fn equivalent_radiant_temperature_c(self, fallback_temperature_c: f64) -> f64 {
        let coefficient = self.equivalent_coefficient_w_per_m2_k();
        if coefficient.abs() <= f64::EPSILON {
            return fallback_temperature_c;
        }

        (self.sky_coefficient_w_per_m2_k * self.sky_temperature_c
            + self.air_coefficient_w_per_m2_k * self.air_temperature_c
            + self.ground_coefficient_w_per_m2_k * self.ground_temperature_c)
            / coefficient
    }

    pub(crate) fn net_heat_gain_per_area_w_per_m2(self, surface_temperature_c: f64) -> f64 {
        -(self.sky_coefficient_w_per_m2_k * (surface_temperature_c - self.sky_temperature_c)
            + self.air_coefficient_w_per_m2_k * (surface_temperature_c - self.air_temperature_c)
            + self.ground_coefficient_w_per_m2_k
                * (surface_temperature_c - self.ground_temperature_c))
    }
}

pub(crate) fn energyplus_exterior_longwave_terms(
    surface_state: &SurfaceHeatBalanceState,
    typed_surface: &Surface,
    horizontal_infrared_radiation_w_per_m2: f64,
    surface_temperature_c: f64,
    air_reference_temperature_c: f64,
    ground_temperature_c: f64,
    tilt_rad: f64,
) -> ExteriorLongwaveTerms {
    let thermal_absorptance = surface_state.thermal_absorptance.clamp(0.0, 1.0);
    let surface_temperature_k = surface_temperature_c + KELVIN_OFFSET;
    let sky_temperature_c = horizontal_infrared_sky_temperature_c(
        horizontal_infrared_radiation_w_per_m2,
        ground_temperature_c,
    );
    let sky_temperature_k = sky_temperature_c + KELVIN_OFFSET;
    let air_temperature_k = air_reference_temperature_c + KELVIN_OFFSET;
    let ground_temperature_k = ground_temperature_c + KELVIN_OFFSET;
    let sky_view_factor = surface_sky_view_factor(typed_surface, tilt_rad);
    let ground_view_factor = surface_ground_view_factor(typed_surface, tilt_rad);
    let air_sky_rad_split = surface_air_sky_radiation_split(tilt_rad);
    let sky_coefficient_w_per_m2_k = energyplus_linearized_radiation_coefficient_w_per_m2_k(
        thermal_absorptance * sky_view_factor * air_sky_rad_split,
        surface_temperature_k,
        sky_temperature_k,
    );
    let air_coefficient_w_per_m2_k = energyplus_linearized_radiation_coefficient_w_per_m2_k(
        thermal_absorptance * sky_view_factor * (1.0 - air_sky_rad_split),
        surface_temperature_k,
        air_temperature_k,
    );
    let ground_coefficient_w_per_m2_k = energyplus_linearized_radiation_coefficient_w_per_m2_k(
        thermal_absorptance * ground_view_factor,
        surface_temperature_k,
        ground_temperature_k,
    );

    ExteriorLongwaveTerms {
        sky_coefficient_w_per_m2_k,
        air_coefficient_w_per_m2_k,
        ground_coefficient_w_per_m2_k,
        sky_temperature_c,
        air_temperature_c: air_reference_temperature_c,
        ground_temperature_c,
    }
}

pub(crate) fn energyplus_linearized_radiation_coefficient_w_per_m2_k(
    exchange_factor: f64,
    surface_temperature_k: f64,
    reference_temperature_k: f64,
) -> f64 {
    if exchange_factor <= 0.0
        || !surface_temperature_k.is_finite()
        || !reference_temperature_k.is_finite()
        || (surface_temperature_k - reference_temperature_k).abs() <= f64::EPSILON
    {
        return 0.0;
    }

    STEFAN_BOLTZMANN_W_PER_M2_K4
        * exchange_factor
        * (surface_temperature_k.powi(4) - reference_temperature_k.powi(4))
        / (surface_temperature_k - reference_temperature_k)
}

pub(crate) fn horizontal_infrared_sky_temperature_c(
    horizontal_infrared_radiation_w_per_m2: f64,
    fallback_air_temperature_c: f64,
) -> f64 {
    if horizontal_infrared_radiation_w_per_m2 <= 0.0 {
        return fallback_air_temperature_c;
    }

    (horizontal_infrared_radiation_w_per_m2 / STEFAN_BOLTZMANN_W_PER_M2_K4).powf(0.25)
        - KELVIN_OFFSET
}
