//! Genuine first-weather GetNextEnvironment prefix and unopened Source attachment.
//! Full weather output/getter children are mandatory and no legacy phase is inferred.
use super::source_weather_initialization::SourceWeatherEio;
use super::{
    SourceWeatherInitializationError, WeatherDayError, WeatherDayState,
    WeatherEnvironmentConfiguration, WeatherSession,
};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::radiation::ordinary::source_children::SourceSolarReportFlags;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::SelectedHeatBalanceInputState;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, SelectedHeatBalanceInput,
};
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputProcessorShortCut,
};
use crate::heat_balance::selected_project_data::ProjectSystemVariables;
use crate::heat_balance::source_output_registration::SourceOutputRegistration;
use crate::heat_balance::source_output_registration::{
    SourceOutputDictionaryHost, SourceOutputReportStreams,
};
use crate::weather::raw::{
    HeaderContextConsumption, RawEpwCursorOwner, RawEpwHeaderState, RawEpwInput,
};
/// The same actual files/RF owner supplies both report-stream and dictionary
/// effects. Passing this one mutable capability prevents an aliased file host.
pub(crate) trait SourceWeatherReportFiles:
    SourceOutputReportStreams + SourceOutputDictionaryHost
{
}
impl<T: SourceOutputReportStreams + SourceOutputDictionaryHost + ?Sized> SourceWeatherReportFiles
    for T
{
}
pub(crate) struct SourceWeatherInitializationOwners<'a, 'backend> {
    pub(crate) raw: &'a SelectedHeatBalanceInput,
    pub(crate) system: &'a ProjectSystemVariables,
    pub(crate) reverse_design_days: &'a bool,
    pub(crate) globals: &'a SelectedHeatBalanceGlobals,
    pub(crate) control: &'a SelectedHeatBalanceInputState,
    /// Same SourceInitializationInput path populated from actual RunConfig.
    pub(crate) input_weather_file_path: &'a std::path::Path,
    pub(crate) input: &'a mut SelectedInputProcessor,
    pub(crate) shortcut: &'a mut SourceInputProcessorShortCut,
    pub(crate) registration: &'a mut SourceOutputRegistration,
    pub(crate) report_flags: &'a mut SourceSolarReportFlags,
    pub(crate) files: &'a mut dyn SourceWeatherReportFiles,
    pub(crate) eio: &'a mut dyn SourceWeatherEio,
    pub(crate) backend: &'a mut SourceErrorBackend<'backend>,
}
/// These are actual remaining Native children; implementors must borrow the same
/// passed owners and preserve full Source effects. No constructor/count substitute.
pub(crate) trait SourceWeatherInitializationChildren {
    fn register_source_weather_outputs(
        &mut self,
        session: &mut WeatherSession,
        owners: &mut SourceWeatherInitializationOwners<'_, '_>,
    ) -> Result<(), SourceWeatherInitializationError>;
    /// Genuine positive-DD GetDesignDayData child; a real same-owner implementation
    /// is mandatory before the actual A DD2 route can complete ReadUser.
    fn get_source_design_day_data(
        &mut self,
        session: &mut WeatherSession,
        owners: &mut SourceWeatherInitializationOwners<'_, '_>,
        errors_found: &mut bool,
    ) -> Result<(), SourceWeatherInitializationError>;
}
impl WeatherSession {
    /// Move genuine Project owners into an UNOPENED cursor. No Native location,
    /// environment list, weather arrays, interpolation or first-call flag is seeded.
    pub(crate) fn from_source_project_state(
        bytes: Vec<u8>,
        configuration: WeatherEnvironmentConfiguration,
        state: &mut WeatherDayState,
    ) -> Result<Self, WeatherDayError> {
        if state.global.time_steps_in_hour != configuration.steps() as i32 {
            return Err(WeatherDayError::admission(
                "actual Project/weather timestep association",
            ));
        }
        let cursor = RawEpwCursorOwner::from_open_input(
            RawEpwInput::new_unopened(bytes),
            RawEpwHeaderState::default(),
            false,
            HeaderContextConsumption::default(),
        );
        Ok(Self {
            state: std::mem::take(state),
            configuration,
            cursor,
            available: false,
            errors_found: false,
            print_environment_stamp: false,
            current_cycle: 0,
            set_week_days: false,
            today_raw: None,
            tomorrow_raw: None,
            today_produced: None,
            tomorrow_produced: None,
        })
    }
    /// Genuine GetNextEnvironment first-input prefix followed by its actual
    /// first-input finish and every-call advance on the same retained owners.
    pub(crate) fn get_next_source_environment(
        &mut self,
        owners: &mut SourceWeatherInitializationOwners<'_, '_>,
        children: &mut impl SourceWeatherInitializationChildren,
        available: &mut bool,
    ) -> Result<bool, SourceWeatherInitializationError> {
        if self.state.global.begin_sim_flag && self.state.weather.get_environment_first_call {
            owners.report_flags.print_end_data_dictionary = true;
            self.state.weather.report_source_output_file_headers(
                owners.registration,
                owners.files,
                owners.backend,
            )?;
            children.register_source_weather_outputs(self, owners)?;
            self.state.weather.get_environment_first_call = false;
        }
        if self.state.weather.get_branch_input_one_time_flag {
            self.state.setup_interpolation_values()?;
            self.state.weather.time_step_fraction =
                1.0 / f64::from(self.state.global.time_steps_in_hour);
            self.state.environment.rho_air_stp =
                crate::psychrometrics::energyplus_psy_rho_air_fn_pb_tdb_w(101325.0, 20.0, 0.0);
            self.open_source_weather_file(
                owners.input_weather_file_path,
                owners.input,
                owners.files,
                owners.backend,
            )?;
            self.cursor.close();
            self.read_source_user_weather_input(owners, children)?;
            let steps = usize::try_from(self.state.global.time_steps_in_hour)
                .map_err(|_| Error::Unavailable("actual AllocateWeatherData4124 dimension"))?;
            self.state.today_values =
                super::WeatherDayValues::allocated(steps).map_err(WeatherDayError::from)?;
            self.state.tomorrow_values =
                super::WeatherDayValues::allocated(steps).map_err(WeatherDayError::from)?;
            if self.cursor.header().intervals_per_hour != 1
                && self.cursor.header().intervals_per_hour != self.state.global.time_steps_in_hour
            {
                owners.backend.show_severe_error("GetNextEnvironment: Number of intervals per hour on Weather file does not match specified number of Time Steps Per Hour")?;
                self.errors_found = true;
            }
            self.state.weather.get_branch_input_one_time_flag = false;
            self.state.weather.envrn = 0;
            if self.state.weather.num_of_envrn > 0 {
                self.state.resolve_source_location_information(
                    self.cursor.header(),
                    owners.backend,
                    owners.eio,
                    &mut self.errors_found,
                )?;
                self.state.check_source_location_validity(owners.backend)?;
                let ordinal = usize::try_from(self.state.weather.num_of_envrn - 1)
                    .map_err(|_| Error::Unavailable("actual last Environment ordinal728"))?;
                let kind = self
                    .state
                    .weather
                    .source_environments
                    .as_deref()
                    .and_then(|rows| rows.get(ordinal))
                    .ok_or(Error::Unavailable("actual last Environment KindOfEnvrn728"))?
                    .kind_of_environment;
                if kind != 1 && kind != 4 {
                    self.check_source_weather_file_validity(owners.backend)?;
                }
                if self.errors_found {
                    owners.backend.show_severe_error(
                        "GetNextEnvironment: No location specified, program will terminate.",
                    )?;
                }
            } else {
                self.errors_found = true;
                owners.backend.show_severe_error("GetNextEnvironment: No Design Days or Run Period(s) specified, program will terminate.")?;
            }
            self.finish_source_environment_input(owners)?;
        }
        self.advance_source_environment(owners, available)
    }
    /// Genuine Native4585–4608 child: only the actual missing-file/weather-sim
    /// predicate emits severe then fatal; a present file requires no extra writes.
    fn check_source_weather_file_validity(
        &self,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), SourceWeatherInitializationError> {
        if !self.state.weather.weather_file_exists && self.state.global.do_weath_sim {
            backend.show_severe_error(
                "GetNextEnvironment: Weather Environment(s) requested, but no weather file found",
            )?;
            return match backend
                .show_fatal_error("Due to previous error condition, simulation terminated")
            {
                Ok(never) => match never {},
                Err(error) => Err(Error::Fatal(error).into()),
            };
        }
        Ok(())
    }
    /// Actual ReadUserWeatherInput4818?4828 contiguous child sequence, borrowing
    /// its real local ErrorsFound. Earlier readers and later SetupEnvironmentTypes
    /// remain mandatory to the parent ReadUserWeatherInput child.
    pub(crate) fn read_source_location_ground_and_auxiliary_inputs(
        &mut self,
        owners: &mut SourceWeatherInitializationOwners<'_, '_>,
        errors_found: &mut bool,
    ) -> Result<(), SourceWeatherInitializationError> {
        self.state.get_source_location_info(
            owners.raw,
            owners.input,
            owners.shortcut,
            owners.backend,
            errors_found,
        )?;
        self.initialize_source_ground_temperatures(
            owners.raw,
            owners.input,
            owners.shortcut,
            owners.backend,
            owners.eio,
        )?;
        self.initialize_source_weather_auxiliary_inputs(
            owners.input,
            owners.shortcut,
            owners.backend,
            owners.eio,
            errors_found,
        )?;
        owners.shortcut.current_module_object = "Site:WeatherStation".into();
        if owners
            .input
            .get_num_objects_found(&owners.shortcut.current_module_object)
            != 0
        {
            return Err(
                Error::Unavailable("actual positive GetWeatherStation object getter").into(),
            );
        }
        self.state.environment.outdoor_environment_parameters.initialize_absent_weather_station(owners.eio)
            .map_err(|error| match error {
                crate::heat_balance::convection::surface_environment::SurfaceOutdoorInitializationError::Io(error)=>Error::Io(error),
                crate::heat_balance::convection::surface_environment::SurfaceOutdoorInitializationError::Format(reason)=>Error::Format(reason),
            })?;
        // Rust call-return witness follows actual Station profile/EIO completion.
        // It never substitutes for the parent weather getter or Native flags.
        self.state.environment.source_weather_station_initialized = true;
        Ok(())
    }
}
