//! Actual SetUpDesignDay3613–3679 EIO operations and PrintDDHeader consumption.
use super::*;
use crate::weather::day::design_day::solar::Coefficients;
const RANGE_NAMES: [&str; 4] = [
    "DefaultMultipliers,",
    "MultiplierSchedule,",
    "DifferenceSchedule,",
    "TemperatureProfile,",
];
const SOLAR_NAMES: [&str; 5] = [
    "ASHRAEClearSky",
    "ZhangHuang",
    "User supplied beam/diffuse from schedules",
    "ASHRAETau",
    "ASHRAETau2017",
];
pub(super) fn write(
    input: &SourceDesignDayData,
    inputs: &[SourceDesignDayData],
    current_environment: i32,
    day: &DailyWeatherVariables,
    coefficients: Coefficients,
    print_header: &mut bool,
    flags: &SourceSolarReportFlags,
    eio: &mut dyn Write,
) -> Result<(), Error> {
    if *print_header && flags.do_weather_init_reporting {
        eio.write_all(b"! <Environment:Design Day Data>, Max Dry-Bulb Temp {C}, Temp Range {dC}, Temp Range Ind Type, Hum Ind Type, Hum Ind Value at Max Temp, Hum Ind Units, Pressure {Pa}, Wind Direction {deg CW from N}, Wind Speed {m/s}, Clearness, Rain, Snow\n")?;
        eio.write_all(b"! <Environment:Design Day Misc>,DayOfYear,ASHRAE A Coeff,ASHRAE B Coeff,ASHRAE C Coeff,Solar Constant-Annual Variation,Eq of Time {minutes}, Solar Declination Angle {deg}, Solar Model\n")?;
        *print_header = false;
    }
    if flags.do_weather_init_reporting {
        let rain = if input.rain_indicator == 1 {
            "Yes"
        } else {
            "No"
        };
        let snow = if input.snow_indicator == 1 {
            "Yes"
        } else {
            "No"
        };
        eio.write_all(b"Environment:Design Day Data,")?;
        write!(eio, "{},", rounded(input.maximum_dry_bulb, 2)?)?;
        write!(eio, "{},", rounded(input.daily_dry_bulb_range, 2)?)?;
        let range = RANGE_NAMES
            .get(size(input.dry_bulb_range_type)?)
            .ok_or(Error::Unavailable("actual DD EIO range-name index3630"))?;
        eio.write_all(range.as_bytes())?;
        match input.humidity_indicator_type {
            4 => eio.write_all(b"Schedule,<schedule values from 0.0 to 100.0>,{percent},")?,
            0 | 1 | 2 | 3 | 5 | 6 | 7 => {
                let value = if input.humidity_indicator_type == 5 {
                    inputs
                        .get(ordinal(current_environment)?)
                        .ok_or(Error::Unavailable(
                            "actual DD EIO global-Envrn input read3650",
                        ))?
                        .humidity_indicator_value
                } else {
                    input.humidity_indicator_value
                };
                let (name, precision, units) = match input.humidity_indicator_type {
                    0 => ("Wetbulb", 2, "C"),
                    1 => ("Dewpoint", 2, "C"),
                    2 => ("Enthalpy", 2, "J/kgDryAir"),
                    3 => ("HumidityRatio", 4, "kgWater/kgDryAir"),
                    5 => ("WetBulbProfileDefaultMultipliers", 2, "C"),
                    6 => ("WetBulbProfileDifferenceSchedule", 2, "C"),
                    7 => ("WetBulbProfileMultiplierSchedule", 2, "C"),
                    _ => return Err(Error::Unavailable("actual DD humidity report enum")),
                };
                write!(eio, "{name},{},{{{units}}},", rounded(value, precision)?)?;
            }
            _ => {
                return Err(Error::Unavailable(
                    "actual DD EIO humidity format index3640",
                ));
            }
        }
        write!(eio, "{},", rounded(input.barometric_pressure, 0)?)?;
        write!(eio, "{},", rounded(input.wind_direction, 0)?)?;
        write!(eio, "{},", rounded(input.wind_speed, 1)?)?;
        write!(eio, "{},", rounded(input.sky_clearness, 2)?)?;
        writeln!(eio, "{rain},{snow}")?;
        write!(eio, "Environment:Design Day Misc,{:3},", day.day_of_year)?;
        write!(eio, "{},", rounded(coefficients.a, 1)?)?;
        write!(eio, "{},", rounded(coefficients.b, 4)?)?;
        write!(eio, "{},", rounded(coefficients.c, 4)?)?;
        write!(eio, "{},", rounded(coefficients.annual_variation, 1)?)?;
        write!(eio, "{},", rounded(day.equation_of_time * 60.0, 2)?)?;
        write!(
            eio,
            "{},",
            rounded(
                day.sin_solar_declin_angle.asin() / (std::f64::consts::PI / 180.0),
                1
            )?
        )?;
        let model = SOLAR_NAMES
            .get(size(input.solar_model)?)
            .ok_or(Error::Unavailable("actual DD EIO solar-model name3678"))?;
        writeln!(eio, "{model}")?;
    }
    Ok(())
}
