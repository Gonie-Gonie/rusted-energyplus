//! Native3795–3957 temperature/humidity/sky/solar writes on actual Tomorrow slots.
use super::properties;
use super::{
    DailyWeatherVariables, Error, SourceDesignDayData, SourceEnvironmentData,
    SourcePsychrometricAccess, WeatherDayValues, WeatherEnvironmentState, ordinal,
};
use crate::weather::day::WeatherVars;
use crate::weather::day::design_day::solar::{Coefficients, clear_sky_radiation};
use crate::weather::day::source_design_day::state::SourceDesignDayModifierGrid;
#[derive(Clone, Copy)]
pub(super) enum HumidityState {
    Constant(f64),
    Variable,
    Uninitialized,
}
pub(super) fn slot(
    values: &WeatherDayValues,
    hour: i32,
    timestep: i32,
) -> Result<&WeatherVars, Error> {
    values
        .hour(
            usize::try_from(hour)
                .map_err(|_| Error::Unavailable("actual DD Tomorrow hour domain"))?,
        )
        .map_err(|_| Error::Unavailable("actual DD Tomorrow allocated hour"))?
        .get(ordinal(timestep)?)
        .ok_or(Error::Unavailable("actual DD Tomorrow timestep domain"))
}
pub(super) fn slot_mut(
    values: &mut WeatherDayValues,
    hour: i32,
    timestep: i32,
) -> Result<&mut WeatherVars, Error> {
    values
        .hour_mut(
            usize::try_from(hour)
                .map_err(|_| Error::Unavailable("actual DD Tomorrow hour domain"))?,
        )
        .map_err(|_| Error::Unavailable("actual DD Tomorrow allocated hour"))?
        .get_mut(ordinal(timestep)?)
        .ok_or(Error::Unavailable("actual DD Tomorrow timestep domain"))
}
fn sun_direction_cosines(
    time: f64,
    day: &DailyWeatherVariables,
    environment: &WeatherEnvironmentState,
) -> [f64; 3] {
    let hour_angle = (15.0 * (12.0 - (time + day.equation_of_time))
        + (environment.time_zone_meridian - environment.longitude))
        * (std::f64::consts::PI / 180.0);
    let cosine = hour_angle.cos();
    let z = day.sin_solar_declin_angle * environment.sin_latitude
        + day.cos_solar_declin_angle * environment.cos_latitude * cosine;
    let (x, y);
    if z >= 0.00001 {
        y = day.sin_solar_declin_angle * environment.cos_latitude
            - day.cos_solar_declin_angle * environment.sin_latitude * cosine;
        x = day.cos_solar_declin_angle * hour_angle.sin();
    } else {
        x = 0.0;
        y = 0.0;
    }
    [x, y, z]
}
// These are the real independently borrowed Native operands; do not hide them in a copied owner DTO.
#[allow(clippy::too_many_arguments)]
pub(super) fn write(
    input: &SourceDesignDayData,
    modifiers: &SourceDesignDayModifierGrid,
    day: &DailyWeatherVariables,
    coefficients: Coefficients,
    environment_row: &SourceEnvironmentData,
    opaque_sky: f64,
    constant_humidity: HumidityState,
    tomorrow: &mut WeatherDayValues,
    steps: i32,
    time_step_fraction: &f64,
    environment: &WeatherEnvironmentState,
    psy: &mut SourcePsychrometricAccess,
) -> Result<(), Error> {
    let dry_bulb_range = if input.dry_bulb_range_type == 2 {
        1.0
    } else if input.dry_bulb_range_type == 3 {
        0.0
    } else {
        input.daily_dry_bulb_range
    };
    let wet_bulb_range = if input.humidity_indicator_type == 6 {
        1.0
    } else {
        input.daily_wet_bulb_range
    };
    for hour in 1..=24 {
        for timestep in 1..=steps {
            let modifier = modifiers.cell(timestep, hour)?;
            let slot = slot_mut(tomorrow, hour, timestep)?;
            if input.dry_bulb_range_type != 3 {
                slot.out_dry_bulb_temp =
                    input.maximum_dry_bulb - modifier.outdoor_dry_bulb_temperature * dry_bulb_range;
            } else {
                slot.out_dry_bulb_temp = modifier.outdoor_dry_bulb_temperature;
            }
            if matches!(input.humidity_indicator_type, 5..=7) {
                let mut wet_bulb = input.humidity_indicator_value
                    - modifier.outdoor_relative_humidity * wet_bulb_range;
                wet_bulb = properties::source_min(wet_bulb, slot.out_dry_bulb_temp);
                let humidity = properties::from_wet_bulb(
                    psy,
                    slot.out_dry_bulb_temp,
                    wet_bulb,
                    input.barometric_pressure,
                )?;
                slot.out_dew_point_temp =
                    properties::dew_point(psy, humidity, input.barometric_pressure)?;
                slot.out_rel_hum = properties::relative_humidity(
                    psy,
                    slot.out_dry_bulb_temp,
                    humidity,
                    input.barometric_pressure,
                )? * 100.0;
            } else if let HumidityState::Constant(humidity) = constant_humidity {
                let mut wet_bulb = properties::wet_bulb(
                    psy,
                    slot.out_dry_bulb_temp,
                    humidity,
                    input.barometric_pressure,
                )?;
                let mut out_humidity = properties::from_dew_point(
                    psy,
                    slot.out_dry_bulb_temp,
                    input.barometric_pressure,
                )?;
                if humidity > out_humidity {
                    wet_bulb = slot.out_dry_bulb_temp;
                } else {
                    out_humidity = properties::from_wet_bulb(
                        psy,
                        slot.out_dry_bulb_temp,
                        wet_bulb,
                        input.barometric_pressure,
                    )?;
                }
                let _ = wet_bulb; // Source local write retained; saturation branch does not recalculate W.
                slot.out_dew_point_temp =
                    properties::dew_point(psy, out_humidity, input.barometric_pressure)?;
                slot.out_rel_hum = properties::relative_humidity(
                    psy,
                    slot.out_dry_bulb_temp,
                    out_humidity,
                    input.barometric_pressure,
                )? * 100.0;
            } else {
                if matches!(constant_humidity, HumidityState::Uninitialized) {
                    return Err(Error::Unavailable(
                        "actual SetUpDesignDay uninitialized ConstantHumidityRatio read after current dry-bulb write3814",
                    ));
                }
                let humidity = properties::from_relative_humidity(
                    psy,
                    slot.out_dry_bulb_temp,
                    modifier.outdoor_relative_humidity / 100.0,
                    input.barometric_pressure,
                )?;
                slot.out_rel_hum = properties::relative_humidity(
                    psy,
                    slot.out_dry_bulb_temp,
                    humidity,
                    input.barometric_pressure,
                )? * 100.0;
                slot.out_dew_point_temp =
                    properties::dew_point(psy, humidity, input.barometric_pressure)?;
            }
            if environment_row.sky_temperature_model != 0 {
                return Err(Error::Unavailable(
                    "actual SetUpDesignDay positive alternate CalcSkyEmissivity child3842",
                ));
            }
            crate::weather::day::sky::initialize_design_day_sky(slot, opaque_sky);
            let (mut beam, mut diffuse) = if input.solar_model == 2 {
                (
                    Some(modifier.beam_solar_radiation),
                    Some(modifier.diffuse_solar_radiation),
                )
            } else {
                let current_time = if steps != 1 {
                    f64::from(hour - 1) + f64::from(timestep) * *time_step_fraction
                } else {
                    f64::from(hour) + environment.one_step_solar_time_offset
                };
                let sun = sun_direction_cosines(current_time, day, environment);
                let cosine_zenith = sun[2];
                if cosine_zenith < 0.00001 {
                    (Some(0.0), Some(0.0))
                } else {
                    match input.solar_model {
                        0 => {
                            let (beam, diffuse) = clear_sky_radiation(
                                cosine_zenith,
                                input.sky_clearness,
                                coefficients,
                            );
                            (Some(beam), Some(diffuse))
                        }
                        3 | 4 => {
                            return Err(Error::Unavailable(
                                "actual SetUpDesignDay ASHRAETauModel child3905",
                            ));
                        }
                        1 => {
                            return Err(Error::Unavailable(
                                "actual SetUpDesignDay ZhangHuang prior-hour weather read3911",
                            ));
                        }
                        _ => (None, None), // Native default leaves both locals uninitialized.
                    }
                }
            };
            if environment.ignore_solar_radiation || environment.ignore_beam_radiation {
                beam = Some(0.0);
            }
            if environment.ignore_solar_radiation || environment.ignore_diffuse_radiation {
                diffuse = Some(0.0);
            }
            slot.beam_solar_rad = beam.ok_or(Error::Unavailable(
                "actual SetUpDesignDay uninitialized BeamRad read3951",
            ))?;
            slot.dif_solar_rad = diffuse.ok_or(Error::Unavailable(
                "actual SetUpDesignDay uninitialized DiffRad read3952",
            ))?;
        }
    }
    Ok(())
}

// Reuse the actual production CalculateSunDirectionCosines body; no test math.
#[cfg(all(test, feature = "native-controlled-air-cohort"))]
impl crate::weather::day::WeatherDayState {
    pub(crate) fn native_gap_sun_direction_cosines(&self, time: f64) -> [f64; 3] {
        sun_direction_cosines(time, &self.today_variables, &self.environment)
    }
}
