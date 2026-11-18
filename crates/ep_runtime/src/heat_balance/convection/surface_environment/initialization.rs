//! Selected absent-object site/station producers and their real caller-owned EIO.
use super::{
    ATMOSPHERIC_TEMPERATURE_GRADIENT_K_PER_M, EARTH_RADIUS_M, SurfaceOutdoorEnvironmentParameters,
};
use ep_model::Terrain;
use std::fmt::{Display, Formatter};
use std::io::{self, Write};

const SITE_HEADER: &str = "! <Environment:Site Atmospheric Variation>,Wind Speed Profile Exponent {},Wind Speed Profile Boundary Layer Thickness {m},Air Temperature Gradient Coefficient {K/m}\n";
const STATION_HEADER: &str = "! <Environment:Weather Station>,Wind Sensor Height Above Ground {m},Wind Speed Profile Exponent {},Wind Speed Profile Boundary Layer Thickness {m},Air Temperature Sensor Height Above Ground {m},Wind Speed Modifier Coefficient-Internal,Temperature Modifier Coefficient-Internal\n";

/// Real EIO failure or selected Source R-format boundary, without a fallback row.
#[derive(Debug)]
pub enum SurfaceOutdoorInitializationError {
    /// The actual caller's EIO sink failed; its written prefix remains owned.
    Io(io::Error),
    /// An actual scalar has no represented result from the selected R formatter.
    Format(String),
}
impl From<io::Error> for SurfaceOutdoorInitializationError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}
impl Display for SurfaceOutdoorInitializationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => Display::fmt(error, f),
            Self::Format(reason) => f.write_str(reason),
        }
    }
}
impl std::error::Error for SurfaceOutdoorInitializationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Format(_) => None,
        }
    }
}

fn rounded(value: f64, precision: usize) -> Result<String, SurfaceOutdoorInitializationError> {
    crate::heat_balance::source_round_format::round(value, precision).map_err(|error| {
        SurfaceOutdoorInitializationError::Format(format!(
            "site/station .{precision}R unavailable: {error:?}"
        ))
    })
}

// The pure numerical factory and the genuine EIO-producing station call share
// this sole equation body and literal absent-object station operands.
pub(super) fn assign_default_weather_station_coefficients(
    parameters: &mut SurfaceOutdoorEnvironmentParameters,
) -> [f64; 4] {
    match assign_default_weather_station_coefficients_with_pow(parameters, |base, exponent| {
        Ok::<f64, std::convert::Infallible>(base.powf(exponent))
    }) {
        Ok(fields) => fields,
        Err(never) => match never {},
    }
}

fn assign_default_weather_station_coefficients_with_pow<E>(
    parameters: &mut SurfaceOutdoorEnvironmentParameters,
    pow: impl FnOnce(f64, f64) -> Result<f64, E>,
) -> Result<[f64; 4], E> {
    let wind_sensor_height_m = 10.0;
    let wind_exponent = 0.14;
    let wind_boundary_layer_height_m: f64 = 270.0;
    let temperature_sensor_height_m =
        super::super::ENERGYPLUS_DEFAULT_WEATHER_FILE_TEMPERATURE_SENSOR_HEIGHT_M;
    parameters.weather_file_wind_modifier = pow(
        wind_boundary_layer_height_m / wind_sensor_height_m,
        wind_exponent,
    )?;
    parameters.weather_file_temperature_modifier_c =
        ATMOSPHERIC_TEMPERATURE_GRADIENT_K_PER_M * EARTH_RADIUS_M * temperature_sensor_height_m
            / (EARTH_RADIUS_M + temperature_sensor_height_m);
    Ok([
        wind_sensor_height_m,
        wind_exponent,
        wind_boundary_layer_height_m,
        temperature_sensor_height_m,
    ])
}

impl SurfaceOutdoorEnvironmentParameters {
    /// Selected DataEnvironment constructor values after a real Building terrain
    /// handoff. This neither parses input nor establishes object absence.
    #[must_use]
    pub fn for_building_terrain(terrain: Terrain) -> Self {
        let (site_wind_exponent, site_wind_boundary_layer_height_m) =
            super::super::energyplus_site_wind_profile(terrain);
        Self {
            site_temperature_gradient_k_per_m: 0.0065,
            weather_file_temperature_modifier_c: 0.0,
            site_wind_exponent,
            site_wind_boundary_layer_height_m,
            weather_file_wind_modifier: 1.5863,
        }
    }

    /// GetSiteAtmosphereData's NumObjects==0 branch and following EIO writes.
    /// The caller must establish absence; one/multiple-object branches are separate.
    /// Incoming Building wind operands are retained. State changes and successful
    /// EIO prefixes remain visible if a later formatting/write operation fails.
    pub fn initialize_absent_site_height_variation(
        &mut self,
        eio: &mut (impl Write + ?Sized),
    ) -> Result<(), SurfaceOutdoorInitializationError> {
        self.site_temperature_gradient_k_per_m = 0.0065;
        eio.write_all(SITE_HEADER.as_bytes())?;
        let row = format!(
            "Environment:Site Atmospheric Variation,{},{},{}\n",
            rounded(self.site_wind_exponent, 3)?,
            rounded(self.site_wind_boundary_layer_height_m, 3)?,
            rounded(self.site_temperature_gradient_k_per_m, 6)?
        );
        eio.write_all(row.as_bytes())?;
        Ok(())
    }

    /// GetWeatherStation's NumObjects==0 defaults, exact coefficient assignment
    /// order, then its real header and .3R row. No caller lifecycle is invented.
    pub fn initialize_absent_weather_station(
        &mut self,
        eio: &mut (impl Write + ?Sized),
    ) -> Result<(), SurfaceOutdoorInitializationError> {
        self.initialize_absent_weather_station_with_pow(
            eio,
            |base, exponent| Ok::<f64, std::convert::Infallible>(base.powf(exponent)),
            |never| match never {},
            |error| error,
        )
    }

    /// Keep the actual coefficient/EIO order while the caller selects its math
    /// transport and preserves its own existing provider/output error boundary.
    pub(crate) fn initialize_absent_weather_station_with_pow<P, E>(
        &mut self,
        eio: &mut (impl Write + ?Sized),
        pow: impl FnOnce(f64, f64) -> Result<f64, P>,
        map_pow_error: impl FnOnce(P) -> E,
        mut map_output_error: impl FnMut(SurfaceOutdoorInitializationError) -> E,
    ) -> Result<(), E> {
        let [wind_height, wind_exponent, wind_layer, temperature_height] =
            assign_default_weather_station_coefficients_with_pow(self, pow)
                .map_err(map_pow_error)?;
        eio.write_all(STATION_HEADER.as_bytes())
            .map_err(|error| map_output_error(error.into()))?;
        let row = format!(
            "Environment:Weather Station,{},{},{},{},{},{}\n",
            rounded(wind_height, 3).map_err(&mut map_output_error)?,
            rounded(wind_exponent, 3).map_err(&mut map_output_error)?,
            rounded(wind_layer, 3).map_err(&mut map_output_error)?,
            rounded(temperature_height, 3).map_err(&mut map_output_error)?,
            rounded(self.weather_file_wind_modifier, 3).map_err(&mut map_output_error)?,
            rounded(self.weather_file_temperature_modifier_c, 3).map_err(&mut map_output_error)?
        );
        eio.write_all(row.as_bytes())
            .map_err(|error| map_output_error(error.into()))?;
        Ok(())
    }
}

#[cfg(test)]
#[path = "initialization_tests.rs"]
mod tests;
