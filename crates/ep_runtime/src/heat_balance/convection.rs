//! Heat-balance convection source-order ownership notes.

use crate::heat_balance::algorithm::HeatBalanceZoneAirAlgorithm;
use crate::heat_balance::state::SurfaceHeatBalanceState;
use ep_model::{MaterialSurfaceRoughness, OutsideSurfaceConvectionAlgorithm, TypedModel};

/// Current inside convection routine family used by the compatibility lane.
pub const INSIDE_CONVECTION_SOURCE: &str =
    "src/EnergyPlus/HeatBalanceSurfaceManager.cc::CalcHeatBalanceInsideSurf";

/// Current outside convection routine family used by the compatibility lane.
pub const OUTSIDE_CONVECTION_SOURCE: &str =
    "src/EnergyPlus/HeatBalanceSurfaceManager.cc::CalcHeatBalanceOutsideSurf";

pub(crate) const ENERGYPLUS_LOW_CONVECTION_LIMIT_W_PER_M2_K: f64 = 0.1;
pub(crate) const ENERGYPLUS_HIGH_CONVECTION_LIMIT_W_PER_M2_K: f64 = 1000.0;

/// EnergyPlus ASHRAE TARP inside natural convection coefficient for one surface.
#[must_use]
pub fn energyplus_tarp_inside_convection_coefficient_w_per_m2_k(
    surface: &SurfaceHeatBalanceState,
    surface_temperature_c: f64,
    air_temperature_c: f64,
) -> f64 {
    let inside_cos_tilt = -surface.tilt_deg.to_radians().cos();
    let coefficient = energyplus_ashrae_tarp_natural_convection_w_per_m2_k(
        surface_temperature_c,
        air_temperature_c,
        inside_cos_tilt,
    );
    if !coefficient.is_finite() {
        return ENERGYPLUS_LOW_CONVECTION_LIMIT_W_PER_M2_K;
    }

    coefficient.clamp(
        ENERGYPLUS_LOW_CONVECTION_LIMIT_W_PER_M2_K,
        ENERGYPLUS_HIGH_CONVECTION_LIMIT_W_PER_M2_K,
    )
}

pub(crate) fn energyplus_ashrae_tarp_natural_convection_w_per_m2_k(
    surface_temperature_c: f64,
    air_temperature_c: f64,
    cos_tilt: f64,
) -> f64 {
    let delta_temperature_c = surface_temperature_c - air_temperature_c;
    if delta_temperature_c.abs() <= f64::EPSILON || cos_tilt.abs() <= 1.0e-12 {
        return energyplus_ashrae_vertical_wall_convection_w_per_m2_k(delta_temperature_c);
    }

    if (delta_temperature_c < 0.0 && cos_tilt < 0.0)
        || (delta_temperature_c > 0.0 && cos_tilt > 0.0)
    {
        energyplus_walton_unstable_horizontal_or_tilt_convection_w_per_m2_k(
            delta_temperature_c,
            cos_tilt,
        )
    } else {
        energyplus_walton_stable_horizontal_or_tilt_convection_w_per_m2_k(
            delta_temperature_c,
            cos_tilt,
        )
    }
}

fn energyplus_ashrae_vertical_wall_convection_w_per_m2_k(delta_temperature_c: f64) -> f64 {
    1.31 * delta_temperature_c.abs().powf(1.0 / 3.0)
}

fn energyplus_walton_unstable_horizontal_or_tilt_convection_w_per_m2_k(
    delta_temperature_c: f64,
    cos_tilt: f64,
) -> f64 {
    9.482 * delta_temperature_c.abs().powf(1.0 / 3.0) / (7.238 - cos_tilt.abs())
}

fn energyplus_walton_stable_horizontal_or_tilt_convection_w_per_m2_k(
    delta_temperature_c: f64,
    cos_tilt: f64,
) -> f64 {
    1.810 * delta_temperature_c.abs().powf(1.0 / 3.0) / (1.382 + cos_tilt.abs())
}

pub(crate) fn exterior_convection_coefficient_w_per_m2_k(wind_speed_m_per_s: f64) -> f64 {
    13.0 + 2.5 * wind_speed_m_per_s.max(0.0)
}

/// EnergyPlus DOE-2 outside convection coefficient for future exterior balance wiring.
#[must_use]
pub fn energyplus_doe2_outside_convection_coefficient_w_per_m2_k(
    surface_temperature_c: f64,
    air_temperature_c: f64,
    cos_tilt: f64,
    surface_azimuth_deg: f64,
    wind_direction_deg: f64,
    wind_speed_m_per_s: f64,
    roughness: MaterialSurfaceRoughness,
) -> f64 {
    let h_n = energyplus_ashrae_tarp_natural_convection_w_per_m2_k(
        surface_temperature_c,
        air_temperature_c,
        cos_tilt,
    );
    let h_f_smooth =
        if energyplus_surface_is_windward(cos_tilt, surface_azimuth_deg, wind_direction_deg) {
            energyplus_mowitt_forced_windward_w_per_m2_k(wind_speed_m_per_s)
        } else {
            energyplus_mowitt_forced_leeward_w_per_m2_k(wind_speed_m_per_s)
        };
    let h_c_smooth = (h_n.powi(2) + h_f_smooth.powi(2)).sqrt();
    let h_f = energyplus_roughness_multiplier(roughness) * (h_c_smooth - h_n);
    h_n + h_f
}

fn energyplus_surface_is_windward(
    cos_tilt: f64,
    surface_azimuth_deg: f64,
    wind_direction_deg: f64,
) -> bool {
    if cos_tilt.abs() >= 0.98 {
        return true;
    }

    let mut diff = (wind_direction_deg - surface_azimuth_deg).abs();
    if diff - 180.0 > 0.001 {
        diff -= 360.0;
    }
    diff.abs() - 90.0 <= 0.001
}

fn energyplus_mowitt_forced_windward_w_per_m2_k(wind_speed_m_per_s: f64) -> f64 {
    3.26 * wind_speed_m_per_s.max(0.0).powf(0.89)
}

fn energyplus_mowitt_forced_leeward_w_per_m2_k(wind_speed_m_per_s: f64) -> f64 {
    3.55 * wind_speed_m_per_s.max(0.0).powf(0.617)
}

fn energyplus_roughness_multiplier(roughness: MaterialSurfaceRoughness) -> f64 {
    match roughness {
        MaterialSurfaceRoughness::VeryRough => 2.17,
        MaterialSurfaceRoughness::Rough => 1.67,
        MaterialSurfaceRoughness::MediumRough => 1.52,
        MaterialSurfaceRoughness::MediumSmooth => 1.13,
        MaterialSurfaceRoughness::Smooth => 1.11,
        MaterialSurfaceRoughness::VerySmooth => 1.0,
    }
}

pub(crate) fn heat_balance_uses_doe2_outside_convection(
    model: &TypedModel,
    zone_air_algorithm: HeatBalanceZoneAirAlgorithm,
) -> bool {
    model_uses_doe2_outside_convection(model)
        || zone_air_algorithm_uses_doe2_outside_convection(zone_air_algorithm)
}

pub(crate) fn model_uses_doe2_outside_convection(model: &TypedModel) -> bool {
    matches!(
        model.surface_convection_algorithms.outside,
        Some(OutsideSurfaceConvectionAlgorithm::Doe2)
    )
}

pub(crate) fn zone_air_algorithm_uses_doe2_outside_convection(
    zone_air_algorithm: HeatBalanceZoneAirAlgorithm,
) -> bool {
    matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideDoe2Probe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2Probe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2InteriorLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2ScriptFInteriorLongwaveProbe
    )
}
