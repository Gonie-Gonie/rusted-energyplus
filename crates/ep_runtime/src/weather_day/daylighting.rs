//! Genuine WeatherManager DayltgCurrentExtHorizIllum/DayltgLuminousEfficacy.
use super::{WeatherDayError, WeatherEnvironmentState};
impl WeatherEnvironmentState {
    pub(crate) fn calculate_source_exterior_daylight(&mut self) -> Result<(), WeatherDayError> {
        if self.current_weather.sun_is_up {
            let direct_horizontal = self.current_weather.beam_solar_rad
                * self.current_weather.solar_direction_cosines[2];
            let diffuse_horizontal = self.current_weather.dif_solar_rad;
            let ratio = diffuse_horizontal / (direct_horizontal + diffuse_horizontal + 0.0001);
            self.cloud_fraction = ratio * ratio;
            self.calculate_source_daylight_luminous_efficacy()?;
            self.exterior_horizontal_sky_illuminance =
                diffuse_horizontal * self.diffuse_solar_luminous_efficacy;
            self.exterior_horizontal_beam_illuminance =
                direct_horizontal * self.direct_solar_luminous_efficacy;
            self.exterior_beam_normal_illuminance =
                self.current_weather.beam_solar_rad * self.direct_solar_luminous_efficacy;
        } else {
            self.cloud_fraction = 0.0;
            self.diffuse_solar_luminous_efficacy = 0.0;
            self.direct_solar_luminous_efficacy = 0.0;
            self.exterior_horizontal_sky_illuminance = 0.0;
            self.exterior_horizontal_beam_illuminance = 0.0;
            self.exterior_beam_normal_illuminance = 0.0;
            self.daylighting_sky_clearness = 0.0;
            self.daylighting_sky_brightness = 0.0;
        }
        Ok(())
    }
    fn calculate_source_daylight_luminous_efficacy(&mut self) -> Result<(), WeatherDayError> {
        const A_DIFF: [f64; 8] = [
            97.24, 107.22, 104.97, 102.39, 100.71, 106.42, 141.88, 152.23,
        ];
        const B_DIFF: [f64; 8] = [-0.46, 1.15, 2.96, 5.59, 5.94, 3.83, 1.90, 0.35];
        const C_DIFF: [f64; 8] = [12.00, 0.59, -5.53, -13.95, -22.75, -36.15, -53.24, -45.27];
        const D_DIFF: [f64; 8] = [-8.91, -3.95, -8.77, -13.90, -23.74, -28.83, -14.03, -7.98];
        const A_DIR: [f64; 8] = [57.20, 98.99, 109.83, 110.34, 106.36, 107.19, 105.75, 101.18];
        const B_DIR: [f64; 8] = [-4.55, -3.46, -4.90, -5.84, -3.97, -1.25, 0.77, 1.58];
        const C_DIR: [f64; 8] = [-2.98, -1.21, -1.71, -1.99, -1.75, -1.51, -1.26, -1.10];
        const D_DIR: [f64; 8] = [117.12, 12.38, -8.81, -4.56, -6.16, -26.73, -34.44, -8.29];
        const EXTRA_DIRECT_NORMAL: [f64; 12] = [
            131153.0, 130613.0, 128992.0, 126816.0, 124731.0, 123240.0, 122652.0, 123120.0,
            124576.0, 126658.0, 128814.0, 130471.0,
        ];
        let sun_zenith = self.current_weather.solar_direction_cosines[2].acos();
        let sun_altitude = std::f64::consts::PI / 2.0 - sun_zenith;
        let sin_sun_altitude = sun_altitude.sin();
        let zeta = 1.041 * ((sun_zenith * sun_zenith) * sun_zenith);
        self.daylighting_sky_clearness = ((self.current_weather.dif_solar_rad
            + self.current_weather.beam_solar_rad)
            / (self.current_weather.dif_solar_rad + 0.0001)
            + zeta)
            / (1.0 + zeta);
        let relative_air_mass = (1.0 - 0.1 * self.elevation / 1000.0)
            / (sin_sun_altitude
                + 0.15 / (sun_altitude / (std::f64::consts::PI / 180.0) + 3.885).powf(1.253));
        let month = self
            .month
            .checked_sub(1)
            .and_then(|n| usize::try_from(n).ok())
            .filter(|&n| n < 12)
            .ok_or_else(|| WeatherDayError::admission("actual ExtraDirNormIll Month7400 index"))?;
        self.daylighting_sky_brightness = (self.current_weather.dif_solar_rad * 93.73)
            * relative_air_mass
            / EXTRA_DIRECT_NORMAL[month];
        let bin = if self.daylighting_sky_clearness <= 1.065 {
            0
        } else if self.daylighting_sky_clearness <= 1.23 {
            1
        } else if self.daylighting_sky_clearness <= 1.50 {
            2
        } else if self.daylighting_sky_clearness <= 1.95 {
            3
        } else if self.daylighting_sky_clearness <= 2.80 {
            4
        } else if self.daylighting_sky_clearness <= 4.50 {
            5
        } else if self.daylighting_sky_clearness <= 6.20 {
            6
        } else {
            7
        };
        let atmospheric_moisture = (0.07 * self.current_weather.out_dew_point_temp - 0.075).exp();
        if self.daylighting_sky_brightness <= 0.0 {
            self.diffuse_solar_luminous_efficacy = 0.0;
        } else {
            self.diffuse_solar_luminous_efficacy = A_DIFF[bin]
                + B_DIFF[bin] * atmospheric_moisture
                + C_DIFF[bin] * self.current_weather.solar_direction_cosines[2]
                + D_DIFF[bin] * self.daylighting_sky_brightness.ln();
        }
        if self.daylighting_sky_brightness <= 0.0 {
            self.direct_solar_luminous_efficacy = 0.0;
        } else {
            let value = A_DIR[bin]
                + B_DIR[bin] * atmospheric_moisture
                + C_DIR[bin] * (5.73 * sun_zenith - 5.0).exp()
                + D_DIR[bin] * self.daylighting_sky_brightness;
            // ObjexxFCL double max(0,value): first operand wins unordered comparison.
            self.direct_solar_luminous_efficacy = if 0.0 < value { value } else { 0.0 };
        }
        Ok(())
    }
}
