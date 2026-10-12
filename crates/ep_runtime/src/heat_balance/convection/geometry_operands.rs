//! State-bound physical height operands; diagnostic vertex helpers are separate.
use super::{energyplus_air_temperature_at_height_c, energyplus_wind_speed_at_height_m_per_s};
use crate::geometry::production_trace::{self, GeometryConsumer, GeometryOperand};
use crate::heat_balance::state::SurfaceHeatBalanceState;
use ep_model::{Point3, Surface, Terrain, WindExposure};

/// Physical height consumer: copies the initialized centroid, without deriving
/// a second centroid from the typed input geometry.
pub(crate) fn energyplus_stored_surface_outside_wind_speed_m_per_s(
    state: &SurfaceHeatBalanceState,
    surface: &Surface,
    terrain: Terrain,
    weather_file_wind_speed_m_per_s: f64,
) -> f64 {
    if let Some(actual) = state.outdoor_environment.current() {
        return actual.wind_speed_m_per_s;
    }
    if surface.wind_exposure != WindExposure::WindExposed {
        return 0.0;
    }
    let centroid = state.geometry.centroid_m;
    let height_m = centroid.z_m;
    production_trace::record(
        GeometryConsumer::OutsideWindSpeed,
        state.surface_id,
        state.zone_id,
        GeometryOperand::CentroidHeight {
            centroid_m: [centroid.x_m, centroid.y_m, centroid.z_m],
            height_m,
        },
    );
    energyplus_wind_speed_at_height_m_per_s(terrain, weather_file_wind_speed_m_per_s, height_m)
}

/// Physical temperature-height consumer using the one stored geometry owner.
pub(crate) fn energyplus_stored_surface_outdoor_air_temperature_c(
    state: &SurfaceHeatBalanceState,
    weather_file_temperature_c: f64,
) -> f64 {
    if let Some(actual) = state.outdoor_environment.current() {
        return actual.dry_bulb_c;
    }
    let centroid = state.geometry.centroid_m;
    let height_m = centroid.z_m;
    production_trace::record(
        GeometryConsumer::OutdoorAirTemperature,
        state.surface_id,
        state.zone_id,
        GeometryOperand::CentroidHeight {
            centroid_m: [centroid.x_m, centroid.y_m, centroid.z_m],
            height_m,
        },
    );
    energyplus_air_temperature_at_height_c(weather_file_temperature_c, height_m)
}

/// Wet-bulb reads its distinct actual array rather than the dry-bulb owner.
pub(crate) fn energyplus_stored_surface_outdoor_wet_bulb_temperature_c(
    state: &SurfaceHeatBalanceState,
    weather_file_temperature_c: f64,
) -> f64 {
    if let Some(actual) = state.outdoor_environment.current() {
        actual.wet_bulb_c
    } else {
        energyplus_stored_surface_outdoor_air_temperature_c(state, weather_file_temperature_c)
    }
}

pub(crate) fn surface_centroid_z_m(vertices: &[Point3]) -> f64 {
    crate::geometry::surface_geometry_properties(vertices)
        .map_or(0.0, |geometry| geometry.centroid_m.z_m)
}
