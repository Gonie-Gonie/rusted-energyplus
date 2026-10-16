//! Native Weather4667–4731,8250–8339,8816–8871 reporting on the actual streams/counters.
use super::{Error, Failure, SourceWeatherEio, ordinal};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::weather::day::source_initialization::SourceWeatherInitializationOwners;
use crate::weather::day::{WeatherDayState, WeatherOwnerState, WeatherSession};

impl WeatherSession {
    pub(crate) fn report_source_weather_and_time(
        &mut self,
        owners: &mut SourceWeatherInitializationOwners<'_, '_>,
    ) -> Result<(), Failure> {
        if !self.state.global.warmup_flag && !self.state.weather.rp_read_all_weather_data {
            if self.print_environment_stamp {
                if owners.report_flags.print_end_data_dictionary
                    && self.state.global.do_output_reporting
                {
                    owners
                        .files
                        .write_eso(b"End of Data Dictionary\n")
                        .map_err(Error::Io)?;
                    owners
                        .files
                        .write_mtr(b"End of Data Dictionary\n")
                        .map_err(Error::Io)?;
                    owners.report_flags.print_end_data_dictionary = false;
                }
                if self.state.global.do_output_reporting {
                    let title = &self
                        .state
                        .weather
                        .source_environments
                        .as_ref()
                        .and_then(|v| v.get(ordinal(self.state.weather.envrn).ok()?))
                        .ok_or(Error::Unavailable(
                            "actual weather environment stamp Title4708",
                        ))?
                        .title;
                    let stamp = || {
                        let fixed=crate::heat_balance::ordinary_heat_balance_initialization::report_warmup::source_fixed_7_2_upper;
                        format!(
                            "{},{},{},{},{},{}\n",
                            self.state.weather.environment_report_character,
                            title,
                            fixed(self.state.environment.latitude),
                            fixed(self.state.environment.longitude),
                            fixed(self.state.environment.time_zone_number),
                            fixed(self.state.environment.elevation)
                        )
                    };
                    owners
                        .files
                        .write_eso(stamp().as_bytes())
                        .map_err(Error::Io)?;
                    owners
                        .files
                        .write_mtr(stamp().as_bytes())
                        .map_err(Error::Io)?;
                    self.print_environment_stamp = false;
                }
            }
        }
        Ok(())
    }
}
pub(super) fn report_source_water_mains(
    weather: &WeatherOwnerState,
    eio: &mut dyn SourceWeatherEio,
) -> Result<(), Failure> {
    if !eio.good() {
        return Ok(());
    }
    let mut text = String::from(
        "! <Site Water Mains Temperature Information>,Calculation Method{},Water Mains Temperature Schedule Name{},Annual Average Outdoor Air Temperature{C},Maximum Difference In Monthly Average Outdoor Air Temperatures{deltaC},Fixed Default Water Mains Temperature{C}\n",
    );
    match weather.water_mains_temperature_method {
        0 => return Err(Error::Unavailable("actual waterMainsTempSched Name8839").into()),
        1 => {
            text.push_str("Site Water Mains Temperature Information,Correlation,NA,");
            text.push_str(&rounded(
                weather.water_mains_annual_average_air_temperature,
                2,
            )?);
            text.push(',');
            text.push_str(&rounded(
                weather.water_mains_maximum_air_temperature_difference,
                2,
            )?);
            text.push_str(",NA\n");
        }
        2 => {
            return Err(
                Error::Unavailable("actual OADryBulbAverage processed predicate8852").into(),
            );
        }
        _ => {
            text.push_str("Site Water Mains Temperature Information,FixedDefault,NA,NA,NA,");
            text.push_str(&rounded(10.0, 1)?);
            text.push('\n');
        }
    }
    eio.write_all(text.as_bytes()).map_err(Error::Io)?;
    Ok(())
}
fn rounded(value: f64, precision: usize) -> Result<String, Error> {
    crate::heat_balance::source_round_format::round(value, precision)
        .map_err(|_| Error::Format("actual water mains R formatter".to_owned()))
}
fn missed_header_check(
    value: f64,
    description: &str,
    missed_header: &mut bool,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), Failure> {
    if value > 0.0 {
        if !*missed_header {
            backend.show_warning_error("Missing Data Found on Weather Data File")?;
            *missed_header = true;
        }
        backend.show_message(&format!(
            "Missing \"{description}\", Number of items={value:5}"
        ))?;
    }
    Ok(())
}
pub(super) fn report_source_missing_range(
    state: &WeatherDayState,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), Failure> {
    if !state.environment.display_weather_missing_data_warnings {
        return Ok(());
    }
    let mut missed_header = false;
    missed_header_check(
        f64::from(state.missed_counts.out_dry_bulb_temp),
        "Dry Bulb Temperature",
        &mut missed_header,
        backend,
    )?;
    missed_header_check(
        f64::from(state.missed_counts.out_baro_press),
        "Atmospheric Pressure",
        &mut missed_header,
        backend,
    )?;
    missed_header_check(
        f64::from(state.missed_counts.out_rel_hum),
        "Relative Humidity",
        &mut missed_header,
        backend,
    )?;
    missed_header_check(
        f64::from(state.missed_counts.out_dew_point_temp),
        "Dew Point Temperatures",
        &mut missed_header,
        backend,
    )?;
    missed_header_check(
        f64::from(state.missed_counts.wind_speed),
        "Wind Speed",
        &mut missed_header,
        backend,
    )?;
    missed_header_check(
        f64::from(state.missed_counts.wind_dir),
        "Wind Direction",
        &mut missed_header,
        backend,
    )?;
    missed_header_check(
        f64::from(state.missed_counts.beam_solar_rad),
        "Direct Radiation",
        &mut missed_header,
        backend,
    )?;
    missed_header_check(
        f64::from(state.missed_counts.dif_solar_rad),
        "Diffuse Radiation",
        &mut missed_header,
        backend,
    )?;
    missed_header_check(
        f64::from(state.missed_counts.total_sky_cover),
        "Total Sky Cover",
        &mut missed_header,
        backend,
    )?;
    missed_header_check(
        f64::from(state.missed_counts.opaque_sky_cover),
        "Opaque Sky Cover",
        &mut missed_header,
        backend,
    )?;
    missed_header_check(
        f64::from(state.missed_counts.snow_depth),
        "Snow Depth",
        &mut missed_header,
        backend,
    )?;
    if state.missed_counts.weath_codes > 0 {
        // The same parser counter was mutated directly during actual getline.
        // It is never reset/read through the legacy raw header counter.
        backend.show_warning_error("Invalid Data Found on Weather Data File")?;
        backend.show_message(&format!(
            "Invalid \"Weather Codes\" (not equal 9 digits), Number of items={:5}",
            state.missed_counts.weath_codes
        ))?;
    }
    missed_header_check(
        f64::from(state.missed_counts.liquid_precip),
        "Liquid Precipitation Depth",
        &mut missed_header,
        backend,
    )?;
    let mut out_of_range_header = false;
    let mut range = |value: i32,
                     description: &str,
                     low: &str,
                     high: &str,
                     extra: &str|
     -> Result<(), Failure> {
        if value > 0 {
            if !out_of_range_header {
                backend.show_warning_error("Out of Range Data Found on Weather Data File")?;
                out_of_range_header = true;
            }
            backend.show_message(&format!(
                "Out of Range {description} [{low},{high}], Number of items={:5}",
                f64::from(value)
            ))?;
            if !extra.is_empty() {
                backend.show_message(extra)?;
            }
        }
        Ok(())
    };
    range(
        state.out_of_range_counts.out_dry_bulb_temp,
        "Dry Bulb Temperatures",
        ">=-90",
        "<=70",
        "",
    )?;
    range(
        state.out_of_range_counts.out_baro_press,
        "Atmospheric Pressure",
        ">31000",
        "<=120000",
        "Out of Range values set to last good value",
    )?;
    range(
        state.out_of_range_counts.out_rel_hum,
        "Relative Humidity",
        ">=0",
        "<=110",
        "",
    )?;
    range(
        state.out_of_range_counts.out_dew_point_temp,
        "Dew Point Temperatures",
        ">=-90",
        "<=70",
        "",
    )?;
    range(
        state.out_of_range_counts.wind_speed,
        "Wind Speed",
        ">=0",
        "<=40",
        "",
    )?;
    range(
        state.out_of_range_counts.wind_dir,
        "Wind Direction",
        ">=0",
        "<=360",
        "",
    )?;
    range(
        state.out_of_range_counts.beam_solar_rad,
        "Direct Radiation",
        ">=0",
        "NoLimit",
        "",
    )?;
    range(
        state.out_of_range_counts.dif_solar_rad,
        "Diffuse Radiation",
        ">=0",
        "NoLimit",
        "",
    )?;
    Ok(())
}
