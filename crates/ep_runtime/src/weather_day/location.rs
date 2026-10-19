//! Actual GetLocationInfo, ResolveLocationInformation and CheckLocationValidity.
use super::{SourceWeatherInitializationError, WeatherDayError, WeatherDayState};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::selected_heat_balance_input::SourceLocalObjectArguments;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, SelectedHeatBalanceInput,
};
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputProcessorShortCut,
};
use crate::weather::raw::RawEpwHeaderState;
use std::io::Write;

fn real(value: f64, precision: usize) -> Result<String, Error> {
    crate::heat_balance::source_round_format::round(value, precision)
        .map_err(|error| Error::Format(format!("actual location Source .{precision}R: {error:?}")))
}
fn fatal(backend: &mut SourceErrorBackend<'_>, text: &str) -> Error {
    match backend.show_fatal_error(text) {
        Ok(never) => match never {},
        Err(e) => Error::Fatal(e),
    }
}
fn weather_error(error: WeatherDayError) -> SourceWeatherInitializationError {
    SourceWeatherInitializationError::Weather(error)
}
impl WeatherDayState {
    pub(crate) fn get_source_location_info(
        &mut self,
        raw: &SelectedHeatBalanceInput,
        input: &mut SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        backend: &mut SourceErrorBackend<'_>,
        errors_found: &mut bool,
    ) -> Result<(), Error> {
        shortcut.current_module_object = "Site:Location".into();
        let count = input.get_num_objects_found(&shortcut.current_module_object);
        if count > 1 {
            backend.show_severe_error(&format!(
                "{}: Too many objects entered. Only one allowed.",
                shortcut.current_module_object
            ))?;
            *errors_found = true;
        }
        if count == 1 {
            let mut args = SourceLocalObjectArguments::<2, 4>::allocated_unwritten();
            raw.get_no_optional_source_object_item(
                &shortcut.current_module_object,
                0,
                input,
                &mut args,
            )?;
            self.weather.location_title.clone_from(&args.alphas[0]);
            self.environment.latitude =
                args.numbers[0].ok_or(Error::Unavailable("actual Location Latitude unwritten"))?;
            self.environment.longitude =
                args.numbers[1].ok_or(Error::Unavailable("actual Location Longitude unwritten"))?;
            self.environment.time_zone_number = args.numbers[2].ok_or(Error::Unavailable(
                "actual Location TimeZoneNumber unwritten",
            ))?;
            self.environment.elevation =
                args.numbers[3].ok_or(Error::Unavailable("actual Location Elevation unwritten"))?;
            if args.alphas[1].eq_ignore_ascii_case("Yes") {
                self.weather.keep_user_site_location_definition = true;
            }
            self.weather.location_gathered = true;
        }
        Ok(())
    }
    // Retain Original nested guards and their diagnostic/output sequencing.
    #[allow(clippy::collapsible_if)]
    pub(crate) fn resolve_source_location_information(
        &mut self,
        header: &RawEpwHeaderState,
        backend: &mut SourceErrorBackend<'_>,
        eio: &mut dyn Write,
        errors_found: &mut bool,
    ) -> Result<(), SourceWeatherInitializationError> {
        // Genuine Environment(NumOfEnvrn), never the selected/current KindOfSim.
        let at = self
            .weather
            .num_of_envrn
            .checked_sub(1)
            .and_then(|n| usize::try_from(n).ok())
            .ok_or(Error::Unavailable(
                "actual positive NumOfEnvrn location ordinal",
            ))?;
        let last = self
            .weather
            .source_environments
            .as_deref()
            .and_then(|r| r.get(at))
            .ok_or(Error::Unavailable(
                "actual allocated Environment(NumOfEnvrn)",
            ))?;
        if last.kind_of_environment == 3 && self.weather.weather_file_exists {
            if self.weather.location_gathered && !self.weather.keep_user_site_location_definition {
                if (self.environment.latitude - header.latitude).abs() > 1.0
                    || (self.environment.longitude - header.longitude).abs() > 1.0
                    || (self.environment.time_zone_number - header.time_zone).abs() > 0.0
                    || (self.environment.elevation - header.elevation).abs()
                        / source_max(self.environment.elevation, 1.0)
                        > 0.10
                {
                    backend.show_warning_error("Weather file location will be used rather than entered (IDF) Location object.")?;
                    backend.show_continue_error(&format!(
                        "..Location object={}",
                        self.weather.location_title
                    ))?;
                    backend.show_continue_error(&format!(
                        "..Weather File Location={}",
                        header.weather_location_title
                    ))?;
                    backend.show_continue_error(&format!("..due to location differences, Latitude difference=[{}] degrees, Longitude difference=[{}] degrees.",real((self.environment.latitude-header.latitude).abs(),2)?,real((self.environment.longitude-header.longitude).abs(),2)?))?;
                    backend.show_continue_error(&format!("..Time Zone difference=[{}] hour(s), Elevation difference=[{}] percent, [{}] meters.",real((self.environment.time_zone_number-header.time_zone).abs(),1)?,real(((self.environment.elevation-header.elevation)/source_max(self.environment.elevation,1.0)*100.0).abs(),2)?,real((self.environment.elevation-header.elevation).abs(),2)?))?;
                }
            }
            if !self.weather.keep_user_site_location_definition {
                self.weather
                    .location_title
                    .clone_from(&header.weather_location_title);
                self.environment.latitude = header.latitude;
                self.environment.longitude = header.longitude;
                self.environment.time_zone_number = header.time_zone;
                self.environment.elevation = header.elevation;
            }
        } else if !self.weather.location_gathered {
            self.weather.location_title = "Not Entered".into();
            backend.show_severe_error(
                "No Location given. Must have location information for simulation.",
            )?;
            *errors_found = true;
        }
        if !*errors_found {
            self.environment
                .resolve_source_standard_conditions(&self.weather.location_title, eio)
                .map_err(weather_error)?;
        }
        Ok(())
    }
    pub(crate) fn check_source_location_validity(
        &mut self,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        let mut location_error = false;
        if self.environment.latitude == -999.0
            && self.environment.longitude == -999.0
            && self.environment.time_zone_number != -999.0
        {
            backend.show_severe_error("No location specified")?;
            location_error = true;
        }
        if self.environment.latitude < -90.0 || self.environment.latitude > 90.0 {
            backend.show_severe_error(&format!(
                "Latitude must be between -90 and 90; Entered={}",
                real(self.environment.latitude, 2)?
            ))?;
            location_error = true;
        }
        if self.environment.longitude < -180.0 || self.environment.longitude > 180.0 {
            backend.show_severe_error(&format!(
                "Longitude must be between -180 and 180; Entered={}",
                real(self.environment.longitude, 2)?
            ))?;
            location_error = true;
        }
        if self.environment.time_zone_number < -12.0 || self.environment.time_zone_number > 14.0 {
            backend.show_severe_error(&format!(
                "Time Zone must be between -12 and +14; Entered={}",
                real(self.environment.time_zone_number, 2)?
            ))?;
            location_error = true;
        }
        let meridian = get_source_standard_time_meridian(self.environment.longitude)?;
        if self
            .environment
            .varying_location_latitude_schedule
            .is_some()
            || self
                .environment
                .varying_location_longitude_schedule
                .is_some()
        {
            // Source moving-building branch intentionally issues no meridian warning.
        } else if (-12.0..=12.0).contains(&meridian) {
            if self.environment.time_zone_number != meridian {
                let difference = (self.environment.time_zone_number - meridian).abs();
                if difference > 1.0 && difference < 24.0 {
                    if difference < 3.0 {
                        backend.show_warning_error(&format!("Standard Time Meridian and Time Zone differ by more than 1, Difference=\"{}\"",real(difference,1)?))?;
                        backend.show_continue_error("Solar Positions may be incorrect")?;
                    } else {
                        backend.show_severe_error(&format!("Standard Time Meridian and Time Zone differ by more than 2, Difference=\"{}\"",real(difference,1)?))?;
                        backend.show_continue_error("Solar Positions will be incorrect")?;
                    }
                }
            }
        } else {
            backend.show_severe_error("Unable to calculate the standard time meridian")?;
            location_error = true;
        }
        if location_error {
            return Err(fatal(
                backend,
                "Due to previous error condition, simulation terminated",
            ));
        }
        self.environment.time_zone_meridian = if self.environment.time_zone_number <= 12.0 {
            self.environment.time_zone_number * 15.0
        } else {
            self.environment.time_zone_number * 15.0 - 360.0
        };
        self.environment.sin_latitude =
            (std::f64::consts::PI / 180.0 * self.environment.latitude).sin();
        self.environment.cos_latitude =
            (std::f64::consts::PI / 180.0 * self.environment.latitude).cos();
        if self.environment.latitude == 0.0
            && self.environment.longitude == 0.0
            && self.environment.time_zone_number == 0.0
        {
            backend.show_warning_error("Did you realize that you have Latitude=0.0, Longitude=0.0 and TimeZone=0.0?  Your building site is in the middle of the Atlantic Ocean.")?;
        }
        Ok(())
    }
}
fn get_source_standard_time_meridian(longitude: f64) -> Result<f64, Error> {
    let mut low: [Option<f64>; 25] = [None; 25];
    let mut high: [Option<f64>; 25] = [None; 25];
    let mut result = 0.0;
    low[12] = Some(-7.5);
    high[12] = Some(7.5);
    for i in 1..=12 {
        low[12 + i] = Some(
            low[12 + i - 1].ok_or(Error::Unavailable("actual GetSTM previous positive low"))?
                + 15.0,
        );
        high[12 + i] = Some(
            high[12 + i - 1].ok_or(Error::Unavailable("actual GetSTM previous positive high"))?
                + 15.0,
        );
    }
    for i in 1..=12 {
        low[12 - i] = Some(
            low[12 - i + 1].ok_or(Error::Unavailable("actual GetSTM previous negative low"))?
                - 15.0,
        );
        high[12 - i] = Some(
            high[12 - i + 1].ok_or(Error::Unavailable("actual GetSTM previous negative high"))?
                - 15.0,
        );
    }
    let mut temp = longitude % 360.0;
    if temp > 180.0 {
        temp -= 180.0;
    }
    for i in -12_i32..=12 {
        let at =
            usize::try_from(i + 12).map_err(|_| Error::Unavailable("actual GetSTM bound index"))?;
        if temp > low[at].ok_or(Error::Unavailable("actual GetSTM low cell"))?
            && temp <= high[at].ok_or(Error::Unavailable("actual GetSTM high cell"))?
        {
            let time_zone = f64::from(i) % 24.0;
            result = time_zone;
            break;
        }
    }
    Ok(result)
}

fn source_max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}
