//! DataHeatBalance144–203 and302–339 on the sole live SelectedZoneData.
use super::*;
use crate::heat_balance::convection::surface_environment::ordinary::{
    SurfaceOutdoorEnvironmentError, check_out_bulb_temperatures,
};

pub(super) fn set_bulb_temperatures(
    owners: &mut SelectedZoneInputOwners,
    env: &WeatherEnvironmentState,
) -> Result<(), Error> {
    let rows = owners.zones.as_mut().ok_or(Error::Unavailable(
        "same actual Zone array for outdoor temperatures",
    ))?;
    for row in rows {
        let parameters = env.outdoor_environment_parameters;
        if parameters.site_temperature_gradient_k_per_m == 0.0 {
            row.dry_bulb = env.current_weather.out_dry_bulb_temp;
            row.wet_bulb = env.current_weather.out_wet_bulb_temp;
        } else {
            let base_dry = env.current_weather.out_dry_bulb_temp
                + parameters.weather_file_temperature_modifier_c;
            let base_wet = env.current_weather.out_wet_bulb_temp
                + parameters.weather_file_temperature_modifier_c;
            let z = row
                .centroid_m
                .ok_or(Error::Unavailable("actual Zone Centroid.z"))?[2];
            if z <= 0.0 {
                row.dry_bulb = base_dry;
                row.wet_bulb = base_wet;
            } else {
                row.dry_bulb = base_dry
                    - parameters.site_temperature_gradient_k_per_m * 6_356_000.0 * z
                        / (6_356_000.0 + z);
                row.wet_bulb = base_wet
                    - parameters.site_temperature_gradient_k_per_m * 6_356_000.0 * z
                        / (6_356_000.0 + z);
            }
        }
    }
    Ok(())
}
pub(super) fn check_bulb_temperatures(
    owners: &SelectedZoneInputOwners,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), Error> {
    let rows = owners.zones.as_ref().ok_or(Error::Unavailable(
        "same actual Zone array for outdoor check",
    ))?;
    let mut min_bulb = 0.0;
    for row in rows {
        // The same original min(a,b,c) helper retains operand/NaN order.
        min_bulb = crate::heat_balance::convection::surface_environment::ordinary::minimum_three(
            min_bulb,
            row.dry_bulb,
            row.wet_bulb,
        );
        if min_bulb < -100.0 {
            let z = row
                .centroid_m
                .ok_or(Error::Unavailable("actual checked Zone Centroid.z"))?[2];
            // The common diagnostic body is called only at the actual failing
            // Zone; later centroid/name reads are never prevalidated.
            check_out_bulb_temperatures("Zone", [(min_bulb, 0.0, z, row.name.as_str())], backend)
                .map_err(|error| match error {
                SurfaceOutdoorEnvironmentError::Unavailable(reason) => Error::Unavailable(reason),
                SurfaceOutdoorEnvironmentError::Format(reason) => Error::Format(reason),
                SurfaceOutdoorEnvironmentError::Backend(error) => Error::Backend(error),
                SurfaceOutdoorEnvironmentError::Fatal(error) => Error::Fatal(error),
            })?;
        }
    }
    Ok(())
}
pub(super) fn set_wind_speeds(
    owners: &mut SelectedZoneInputOwners,
    env: &WeatherEnvironmentState,
) -> Result<(), Error> {
    let parameters = env.outdoor_environment_parameters;
    let fac = env.current_weather.wind_speed
        * parameters.weather_file_wind_modifier
        * parameters
            .site_wind_boundary_layer_height_m
            .powf(-parameters.site_wind_exponent);
    let rows = owners.zones.as_mut().ok_or(Error::Unavailable(
        "same actual Zone array for outdoor wind",
    ))?;
    for row in rows {
        if parameters.site_wind_exponent == 0.0 {
            row.wind_speed = env.current_weather.wind_speed;
        } else {
            let z = row
                .centroid_m
                .ok_or(Error::Unavailable("actual wind Zone Centroid.z"))?[2];
            row.wind_speed = if z <= 0.0 {
                0.0
            } else {
                fac * z.powf(parameters.site_wind_exponent)
            };
        }
    }
    Ok(())
}
pub(super) fn set_wind_directions(
    owners: &mut SelectedZoneInputOwners,
    env: &WeatherEnvironmentState,
) -> Result<(), Error> {
    let fac = env.current_weather.wind_dir;
    let rows = owners.zones.as_mut().ok_or(Error::Unavailable(
        "same actual Zone array for wind direction",
    ))?;
    for row in rows {
        row.wind_direction = fac;
    }
    Ok(())
}
