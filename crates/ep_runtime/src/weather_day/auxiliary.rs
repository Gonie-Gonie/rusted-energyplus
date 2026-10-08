//! Source missing-value and counter storage, independent of the daily copy.

use super::WeatherVars;

/// Source `ExtWeatherVars`: complete WeatherVars plus its five additional fields.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ExtendedWeatherVars {
    /// The full source inherited weather storage.
    pub base: WeatherVars,
    /// Visibility.
    pub visibility: f64,
    /// Ceiling height.
    pub ceiling: f64,
    /// Aerosol optical depth.
    pub aer_opt_depth: f64,
    /// Snow depth.
    pub snow_depth: f64,
    /// Integer days since last snow.
    pub days_last_snow: i32,
}

impl ExtendedWeatherVars {
    /// Extra source real fields in declaration order, without changing their bits.
    #[must_use]
    pub fn extra_real_values(&self) -> [f64; 4] {
        [
            self.visibility,
            self.ceiling,
            self.aer_opt_depth,
            self.snow_depth,
        ]
    }
}

/// All nineteen source `WeatherVarCounts`, with integer constructor defaults.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WeatherVarCounts {
    /// Dry-bulb count.
    pub out_dry_bulb_temp: i32,
    /// Dew-point count.
    pub out_dew_point_temp: i32,
    /// Relative-humidity count.
    pub out_rel_hum: i32,
    /// Barometric-pressure count.
    pub out_baro_press: i32,
    /// Wind-direction count.
    pub wind_dir: i32,
    /// Wind-speed count.
    pub wind_speed: i32,
    /// Beam-radiation count.
    pub beam_solar_rad: i32,
    /// Diffuse-radiation count.
    pub dif_solar_rad: i32,
    /// Total-sky-cover count.
    pub total_sky_cover: i32,
    /// Opaque-sky-cover count.
    pub opaque_sky_cover: i32,
    /// Visibility count.
    pub visibility: i32,
    /// Ceiling count.
    pub ceiling: i32,
    /// Liquid-precipitation count.
    pub liquid_precip: i32,
    /// Water-precipitation count.
    pub water_precip: i32,
    /// Aerosol-optical-depth count.
    pub aer_opt_depth: i32,
    /// Snow-depth count.
    pub snow_depth: i32,
    /// Days-since-last-snow count.
    pub days_last_snow: i32,
    /// Weather-code count; parser synchronization belongs to its actual caller.
    pub weath_codes: i32,
    /// Albedo count.
    pub albedo: i32,
}

impl WeatherVarCounts {
    /// Returns all source counters in declaration order.
    #[must_use]
    pub fn integer_values(&self) -> [i32; 19] {
        [
            self.out_dry_bulb_temp,
            self.out_dew_point_temp,
            self.out_rel_hum,
            self.out_baro_press,
            self.wind_dir,
            self.wind_speed,
            self.beam_solar_rad,
            self.dif_solar_rad,
            self.total_sky_cover,
            self.opaque_sky_cover,
            self.visibility,
            self.ceiling,
            self.liquid_precip,
            self.water_precip,
            self.aer_opt_depth,
            self.snow_depth,
            self.days_last_snow,
            self.weath_codes,
            self.albedo,
        ]
    }
}
