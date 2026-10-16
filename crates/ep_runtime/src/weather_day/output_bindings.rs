//! Actual disjoint live Weather Which refs, shared with timestamp-field view.
use super::{CurrentWeatherState, WeatherOwnerState};
use crate::heat_balance::radiation::ordinary::{
    OrdinaryRadiationError, OrdinaryRadiationUnavailable,
};
use crate::heat_balance::source_output_registration::{
    SourceIntegerOutputOwners, SourceIntegerOutputTarget, SourceRealOutputOwners,
    SourceRealOutputTarget, SourceWeatherIntegerOutputField as Integer,
    SourceWeatherRealOutputField as Real,
};
/// No whole-environment constructor: Root must split actual fields with timestamp
/// view. Mutable EndMonth belongs solely to OutputUpdateEnvironment.
pub(crate) struct SourceWeatherOutputOwners<'a> {
    pub(crate) current_weather: &'a CurrentWeatherState,
    pub(crate) weather: &'a WeatherOwnerState,
    pub(crate) ground_temp: &'a [f64; 4],
    pub(crate) water_mains_temperature: &'a f64,
    pub(crate) rainfall_current_rate: &'a f64,
    pub(crate) rainfall_current_amount: &'a f64,
    pub(crate) dst_indicator: &'a i32,
    pub(crate) exterior_horizontal_sky_illuminance: &'a f64,
    pub(crate) exterior_horizontal_beam_illuminance: &'a f64,
    pub(crate) exterior_beam_normal_illuminance: &'a f64,
    pub(crate) diffuse_solar_luminous_efficacy: &'a f64,
    pub(crate) direct_solar_luminous_efficacy: &'a f64,
    pub(crate) daylighting_sky_clearness: &'a f64,
    pub(crate) daylighting_sky_brightness: &'a f64,
}
impl SourceRealOutputOwners for SourceWeatherOutputOwners<'_> {
    fn real_output(&self, target: SourceRealOutputTarget) -> Result<&f64, OrdinaryRadiationError> {
        let SourceRealOutputTarget::Weather { field } = target else {
            return Err(OrdinaryRadiationUnavailable::StorageDomain.into());
        };
        Ok(match field {
            Real::OutDryBulbTemp => &self.current_weather.out_dry_bulb_temp,
            Real::OutDewPointTemp => &self.current_weather.out_dew_point_temp,
            Real::OutWetBulbTemp => &self.current_weather.out_wet_bulb_temp,
            Real::OutHumRat => &self.current_weather.out_hum_rat,
            Real::OutRelHum => &self.current_weather.out_rel_hum,
            Real::OutBaroPress => &self.current_weather.out_baro_press,
            Real::WindSpeed => &self.current_weather.wind_speed,
            Real::WindDir => &self.current_weather.wind_dir,
            Real::SkyTemp => &self.current_weather.sky_temp,
            Real::HorizIRSky => &self.weather.horizontal_infrared_sky,
            Real::DifSolarRad => &self.current_weather.dif_solar_rad,
            Real::BeamSolarRad => &self.current_weather.beam_solar_rad,
            Real::LiquidPrecipitation => &self.current_weather.liquid_precipitation,
            Real::RainfallCurrentRate => self.rainfall_current_rate,
            Real::RainfallCurrentAmount => self.rainfall_current_amount,
            Real::GndSolarRad => &self.current_weather.gnd_solar_rad,
            Real::GroundTemperatureBuildingSurface => &self.ground_temp[0],
            Real::GroundTemperatureShallow => &self.ground_temp[1],
            Real::GroundTemperatureDeep => &self.ground_temp[2],
            Real::GroundTemperatureFCFactorMethod => &self.ground_temp[3],
            Real::TotalCloudCover => &self.current_weather.total_cloud_cover,
            Real::OpaqueCloudCover => &self.current_weather.opaque_cloud_cover,
            Real::OutEnthalpy => &self.current_weather.out_enthalpy,
            Real::OutAirDensity => &self.current_weather.out_air_density,
            Real::SolarAzimuthAngle => &self.weather.solar_azimuth_angle,
            Real::SolarAltitudeAngle => &self.weather.solar_altitude_angle,
            Real::HrAngle => &self.weather.hour_angle,
            Real::HorizontalSkyIlluminance => self.exterior_horizontal_sky_illuminance,
            Real::HorizontalBeamIlluminance => self.exterior_horizontal_beam_illuminance,
            Real::BeamNormalIlluminance => self.exterior_beam_normal_illuminance,
            Real::DiffuseLuminousEfficacy => self.diffuse_solar_luminous_efficacy,
            Real::DirectLuminousEfficacy => self.direct_solar_luminous_efficacy,
            Real::SkyClearness => self.daylighting_sky_clearness,
            Real::SkyBrightness => self.daylighting_sky_brightness,
            Real::WaterMainsTemperature => self.water_mains_temperature,
        })
    }
}
impl SourceIntegerOutputOwners for SourceWeatherOutputOwners<'_> {
    fn integer_output(
        &self,
        target: SourceIntegerOutputTarget,
    ) -> Result<&i32, OrdinaryRadiationError> {
        let SourceIntegerOutputTarget::Weather { field } = target else {
            return Err(OrdinaryRadiationUnavailable::StorageDomain.into());
        };
        Ok(match field {
            Integer::RainStatus => &self.weather.rpt_is_rain,
            Integer::SnowStatus => &self.weather.rpt_is_snow,
            Integer::DstIndicator => self.dst_indicator,
            Integer::DayTypeIndex => &self.weather.rpt_day_type,
        })
    }
}
