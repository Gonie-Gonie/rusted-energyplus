//! Heat-balance convection source-order ownership notes.

mod geometry_operands;
pub mod simple_inside;
pub mod surface_environment;
pub mod tarp_inside;
use crate::geometry::production_trace::{self, GeometryConsumer, GeometryOperand};
use crate::heat_balance::algorithm::HeatBalanceRuntimeConfig;
use crate::heat_balance::state::SurfaceHeatBalanceState;
use ep_model::{
    MaterialSurfaceRoughness, OutsideBoundaryCondition, OutsideSurfaceConvectionAlgorithm, Surface,
    Terrain, TypedModel, WindExposure,
};
use geometry_operands::surface_centroid_z_m;
pub(crate) use geometry_operands::{
    energyplus_stored_surface_outdoor_air_temperature_c,
    energyplus_stored_surface_outdoor_wet_bulb_temperature_c,
    energyplus_stored_surface_outside_wind_speed_m_per_s,
};
pub(crate) use tarp_inside::{
    branch as energyplus_ashrae_tarp_natural_convection_branch,
    natural_coefficient_w_per_m2_k as energyplus_ashrae_tarp_natural_convection_w_per_m2_k,
};

/// Current inside convection routine family used by the compatibility lane.
pub const INSIDE_CONVECTION_SOURCE: &str =
    "src/EnergyPlus/HeatBalanceSurfaceManager.cc::CalcHeatBalanceInsideSurf";

/// Current outside convection routine family used by the compatibility lane.
pub const OUTSIDE_CONVECTION_SOURCE: &str =
    "src/EnergyPlus/HeatBalanceSurfaceManager.cc::CalcHeatBalanceOutsideSurf";

pub(crate) const ENERGYPLUS_LOW_CONVECTION_LIMIT_W_PER_M2_K: f64 = 0.1;
pub(crate) const ENERGYPLUS_HIGH_CONVECTION_LIMIT_W_PER_M2_K: f64 = 1000.0;
pub(crate) const ENERGYPLUS_DEFAULT_WEATHER_FILE_TEMPERATURE_SENSOR_HEIGHT_M: f64 = 1.5;

/// EnergyPlus non-Kiva detailed inside coefficient using the actual stored
/// outward cosine and fixed CON01 LowHConvLimit default. Full InitIntConvCoeff
/// traversal/reference-air/cadence ownership remains a separate caller contract.
#[must_use]
pub fn energyplus_tarp_inside_convection_coefficient_w_per_m2_k(
    surface: &SurfaceHeatBalanceState,
    surface_temperature_c: f64,
    air_temperature_c: f64,
) -> f64 {
    tarp_inside::detailed_inside_coefficient_w_per_m2_k(
        surface_temperature_c,
        air_temperature_c,
        surface.geometry.cos_tilt,
        ENERGYPLUS_LOW_CONVECTION_LIMIT_W_PER_M2_K,
    )
}

#[must_use]
pub(crate) fn energyplus_tarp_inside_convection_branch_id(
    surface: &SurfaceHeatBalanceState,
    surface_temperature_c: f64,
    air_temperature_c: f64,
) -> &'static str {
    energyplus_ashrae_tarp_natural_convection_branch(
        surface_temperature_c,
        air_temperature_c,
        -surface.geometry.cos_tilt,
    )
    .id()
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

pub(crate) fn energyplus_surface_is_windward(
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

#[must_use]
pub(crate) fn energyplus_outside_convection_branch_id(
    surface_state: &SurfaceHeatBalanceState,
    typed_surface: Option<&Surface>,
    wind_direction_deg: f64,
    use_doe2_outside_convection: bool,
) -> &'static str {
    if surface_state.outside_boundary_condition != OutsideBoundaryCondition::Outdoors
        || surface_state.area_m2 <= 0.0
    {
        return "not-outdoors";
    }
    if !use_doe2_outside_convection {
        return "simple-combined";
    }
    let Some(_typed_surface) = typed_surface else {
        return "missing-surface";
    };

    if energyplus_surface_is_windward(
        surface_state.tilt_deg.to_radians().cos(),
        surface_state.azimuth_deg,
        wind_direction_deg,
    ) {
        "doe2-windward"
    } else {
        "doe2-leeward"
    }
}

fn energyplus_mowitt_forced_windward_w_per_m2_k(wind_speed_m_per_s: f64) -> f64 {
    3.26 * wind_speed_m_per_s.max(0.0).powf(0.89)
}

fn energyplus_mowitt_forced_leeward_w_per_m2_k(wind_speed_m_per_s: f64) -> f64 {
    3.55 * wind_speed_m_per_s.max(0.0).powf(0.617)
}

pub(crate) fn energyplus_roughness_multiplier(roughness: MaterialSurfaceRoughness) -> f64 {
    match roughness {
        MaterialSurfaceRoughness::VeryRough => 2.17,
        MaterialSurfaceRoughness::Rough => 1.67,
        MaterialSurfaceRoughness::MediumRough => 1.52,
        MaterialSurfaceRoughness::MediumSmooth => 1.13,
        MaterialSurfaceRoughness::Smooth => 1.11,
        MaterialSurfaceRoughness::VerySmooth => 1.0,
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ExteriorConvectionTerms {
    pub(crate) coefficient_w_per_m2_k: f64,
    pub(crate) reference_temperature_c: f64,
}

pub(crate) fn energyplus_exterior_convection_terms(
    surface_state: &SurfaceHeatBalanceState,
    typed_surface: &Surface,
    surface_temperature_c: f64,
    outdoor_dry_bulb_c: f64,
    tilt_rad: f64,
    terrain: Terrain,
    weather_file_wind_speed_m_per_s: f64,
    wind_direction_deg: f64,
    use_doe2_outside_convection: bool,
    wet_reference_temperature_c: f64,
    wet_timestep_fraction: f64,
) -> ExteriorConvectionTerms {
    let dry_coefficient_w_per_m2_k = energyplus_dry_exterior_convection_coefficient_w_per_m2_k(
        surface_state,
        typed_surface,
        surface_temperature_c,
        outdoor_dry_bulb_c,
        tilt_rad,
        terrain,
        weather_file_wind_speed_m_per_s,
        wind_direction_deg,
        use_doe2_outside_convection,
    );
    let wet_timestep_fraction = wet_timestep_fraction.clamp(0.0, 1.0);
    if wet_timestep_fraction <= f64::EPSILON {
        return ExteriorConvectionTerms {
            coefficient_w_per_m2_k: dry_coefficient_w_per_m2_k,
            reference_temperature_c: outdoor_dry_bulb_c,
        };
    }

    let coefficient_w_per_m2_k = wet_timestep_fraction
        * ENERGYPLUS_HIGH_CONVECTION_LIMIT_W_PER_M2_K
        + (1.0 - wet_timestep_fraction) * dry_coefficient_w_per_m2_k;
    let reference_temperature_c = if coefficient_w_per_m2_k.abs() <= f64::EPSILON {
        outdoor_dry_bulb_c
    } else {
        (wet_timestep_fraction
            * ENERGYPLUS_HIGH_CONVECTION_LIMIT_W_PER_M2_K
            * wet_reference_temperature_c
            + (1.0 - wet_timestep_fraction) * dry_coefficient_w_per_m2_k * outdoor_dry_bulb_c)
            / coefficient_w_per_m2_k
    };

    ExteriorConvectionTerms {
        coefficient_w_per_m2_k,
        reference_temperature_c,
    }
}

pub(crate) fn energyplus_dry_exterior_convection_coefficient_w_per_m2_k(
    surface_state: &SurfaceHeatBalanceState,
    typed_surface: &Surface,
    surface_temperature_c: f64,
    outdoor_dry_bulb_c: f64,
    tilt_rad: f64,
    terrain: Terrain,
    weather_file_wind_speed_m_per_s: f64,
    wind_direction_deg: f64,
    use_doe2_outside_convection: bool,
) -> f64 {
    let wind_speed_m_per_s = energyplus_stored_surface_outside_wind_speed_m_per_s(
        surface_state,
        typed_surface,
        terrain,
        weather_file_wind_speed_m_per_s,
    );
    if use_doe2_outside_convection {
        let cos_tilt = tilt_rad.cos();
        production_trace::record(
            GeometryConsumer::OutsideConvection,
            surface_state.surface_id,
            surface_state.zone_id,
            GeometryOperand::ConvectionOrientation {
                azimuth_deg: surface_state.azimuth_deg,
                tilt_deg: surface_state.tilt_deg,
                cos_tilt,
            },
        );
        energyplus_doe2_outside_convection_coefficient_w_per_m2_k(
            surface_temperature_c,
            outdoor_dry_bulb_c,
            cos_tilt,
            surface_state.azimuth_deg,
            wind_direction_deg,
            wind_speed_m_per_s,
            surface_state.outside_layer_roughness,
        )
    } else {
        exterior_convection_coefficient_w_per_m2_k(wind_speed_m_per_s)
    }
}

pub(crate) fn energyplus_building_terrain(model: &TypedModel) -> Terrain {
    model
        .building
        .as_ref()
        .map(|building| building.terrain)
        .unwrap_or(Terrain::Suburbs)
}

pub(crate) fn energyplus_surface_outside_wind_speed_m_per_s(
    surface: &Surface,
    terrain: Terrain,
    weather_file_wind_speed_m_per_s: f64,
) -> f64 {
    if surface.wind_exposure != WindExposure::WindExposed {
        return 0.0;
    }

    energyplus_wind_speed_at_height_m_per_s(
        terrain,
        weather_file_wind_speed_m_per_s,
        surface_centroid_z_m(&surface.vertices),
    )
}

pub(crate) fn energyplus_wind_speed_at_height_m_per_s(
    terrain: Terrain,
    weather_file_wind_speed_m_per_s: f64,
    height_m: f64,
) -> f64 {
    surface_environment::SurfaceOutdoorEnvironmentParameters::default_weather_station(terrain)
        .wind_speed_at_height(weather_file_wind_speed_m_per_s, height_m)
}

pub(crate) fn energyplus_site_wind_profile(terrain: Terrain) -> (f64, f64) {
    match terrain {
        Terrain::Country => (0.14, 270.0),
        Terrain::Suburbs | Terrain::Urban => (0.22, 370.0),
        Terrain::City => (0.33, 460.0),
        Terrain::Ocean => (0.10, 210.0),
    }
}

pub(crate) fn energyplus_surface_outdoor_air_temperature_c(
    surface: &Surface,
    weather_file_temperature_c: f64,
) -> f64 {
    energyplus_air_temperature_at_height_c(
        weather_file_temperature_c,
        surface_centroid_z_m(&surface.vertices),
    )
}

pub(crate) fn energyplus_air_temperature_at_height_c(
    weather_file_temperature_c: f64,
    height_m: f64,
) -> f64 {
    // Terrain does not change the admitted temperature-gradient/station defaults.
    surface_environment::SurfaceOutdoorEnvironmentParameters::default_weather_station(
        Terrain::Suburbs,
    )
    .temperature_at_height(weather_file_temperature_c, height_m)
}

pub(crate) fn heat_balance_uses_doe2_outside_convection(
    model: &TypedModel,
    runtime_config: HeatBalanceRuntimeConfig,
) -> bool {
    model_uses_doe2_outside_convection(model) || runtime_config.use_doe2_outside_convection
}

pub(crate) fn model_uses_doe2_outside_convection(model: &TypedModel) -> bool {
    matches!(
        model.surface_convection_algorithms.outside,
        Some(OutsideSurfaceConvectionAlgorithm::Doe2)
    )
}
