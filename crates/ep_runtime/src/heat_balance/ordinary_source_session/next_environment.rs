//! Real GetNextEnvironment children and separate Setup/Main Available identities.
use super::OrdinarySourceSession;
use crate::RuntimeError;
use crate::heat_balance::ctf_initialization_context::{CtfInitializationReports, hb_input_error};
use crate::heat_balance::ctf_reports::PredefinedStringTable;
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::ordinary_hvac_manager::water_manager::SourceWaterManager;
use crate::heat_balance::predefined_tables::PredefinedRegistry;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_schedule_manager::SelectedScheduleManager;
use crate::heat_balance::source_initialization::SourceInitializationFiles;
use crate::heat_balance::source_output_prescan::SourceOutputPreScan;
use crate::heat_balance::source_output_registration::{
    SourceMtdFileHost, SourceOutputDictionaryHost, SourceOutputFileHost, SourceReportVariableInput,
};
use crate::heat_balance::state::HeatBalanceOrdinaryWarmupReportInputs;
use crate::weather::day::output_bindings::SourceWeatherOutputOwners;
use crate::weather::day::output_registration::{
    SourceWeatherOutputContext, register_source_weather_outputs,
};
use crate::weather::day::source_initialization::{
    SourceWeatherInitializationChildren, SourceWeatherInitializationOwners,
    SourceWeatherReportFiles,
};
use crate::weather::day::{SourceWeatherInitializationError as Failure, WeatherSession};
use ep_model::{ReportInputs, TypedModel};

// Both are real Native caller locals, never copies/reflections of each other.
enum AvailableBorrow<'a> {
    Caller(&'a mut bool),
    PrimaryLocal,
}
struct DictionaryBorrow<'a> {
    files: &'a mut dyn SourceWeatherReportFiles,
}
impl SourceOutputDictionaryHost for DictionaryBorrow<'_> {
    fn eso_good(&self) -> bool {
        SourceOutputDictionaryHost::eso_good(&*self.files)
    }
    fn write_eso_dictionary(&mut self, line: &str) -> std::io::Result<()> {
        SourceOutputDictionaryHost::write_eso_dictionary(&mut *self.files, line)
    }
    fn sqlite_present(&self) -> bool {
        SourceOutputDictionaryHost::sqlite_present(&*self.files)
    }
    fn results_framework_output_variables(&mut self) -> &mut Vec<String> {
        SourceOutputDictionaryHost::results_framework_output_variables(&mut *self.files)
    }
}
struct FirstWeatherChildren<'a, H> {
    water: &'a SourceWaterManager,
    registry: &'a PredefinedRegistry,
    table: &'a mut PredefinedStringTable,
    output_host: &'a mut H,
    prescan: &'a SourceOutputPreScan,
    report_inputs: &'a ReportInputs,
    minimum_report_frequency: &'a str,
    schedules: &'a mut SelectedScheduleManager,
}
impl<H: SourceOutputFileHost> SourceWeatherInitializationChildren for FirstWeatherChildren<'_, H> {
    fn register_source_weather_outputs(
        &mut self,
        session: &mut WeatherSession,
        owners: &mut SourceWeatherInitializationOwners<'_, '_>,
    ) -> Result<(), Failure> {
        let env = &session.state.environment;
        let output_owners = SourceWeatherOutputOwners {
            current_weather: &env.current_weather,
            weather: &session.state.weather,
            ground_temp: &env.ground_temp,
            water_mains_temperature: &env.water_mains_temperature,
            rainfall_current_rate: &self.water.data.rainfall.current_rate,
            rainfall_current_amount: &self.water.data.rainfall.current_amount,
            dst_indicator: &env.dst_indicator,
            exterior_horizontal_sky_illuminance: &env.exterior_horizontal_sky_illuminance,
            exterior_horizontal_beam_illuminance: &env.exterior_horizontal_beam_illuminance,
            exterior_beam_normal_illuminance: &env.exterior_beam_normal_illuminance,
            diffuse_solar_luminous_efficacy: &env.diffuse_solar_luminous_efficacy,
            direct_solar_luminous_efficacy: &env.direct_solar_luminous_efficacy,
            daylighting_sky_clearness: &env.daylighting_sky_clearness,
            daylighting_sky_brightness: &env.daylighting_sky_brightness,
        };
        let mut input = SourceReportVariableInput::with_schedules(
            self.report_inputs,
            owners.input,
            self.minimum_report_frequency,
            self.schedules,
        );
        let mut dictionary_host = DictionaryBorrow {
            files: owners.files,
        };
        register_source_weather_outputs(
            &output_owners,
            owners.globals,
            &mut SourceWeatherOutputContext {
                registration: owners.registration,
                input: &mut input,
                prescan: self.prescan,
                minutes_in_time_step: session.state.global.minutes_in_time_step,
                host: self.output_host,
                dictionary_host: &mut dictionary_host,
                eio: owners.eio,
            },
        )?;
        Ok(())
    }
    fn get_source_design_day_data(
        &mut self,
        session: &mut WeatherSession,
        owners: &mut SourceWeatherInitializationOwners<'_, '_>,
        errors_found: &mut bool,
    ) -> Result<(), Failure> {
        session.read_source_design_day_input(owners, self.registry, self.table, errors_found)
    }
}
impl SourceInitializationFiles {
    /// Actual SetupSimulation local Available1979 is supplied directly by caller.
    /// It never updates the independent ManageSimulation local stored at249.
    pub(crate) fn source_next_environment_on_session(
        &mut self,
        session: &mut Option<OrdinarySourceSession>,
        reports: &mut CtfInitializationReports,
        weather: &mut WeatherSession,
        model: &TypedModel,
        reporting: &HeatBalanceOrdinaryWarmupReportInputs,
        output_host: &mut SourceMtdFileHost,
        backend: &mut SourceErrorBackend<'_>,
        available: &mut bool,
    ) -> Result<bool, RuntimeError> {
        self.next_source_environment_with_available(
            session,
            reports,
            weather,
            model,
            reporting,
            output_host,
            backend,
            AvailableBorrow::Caller(available),
        )
    }
    /// Actual primary-loop249 Available is split from its sole retained owner.
    pub(crate) fn source_next_primary_environment_on_session(
        &mut self,
        session: &mut Option<OrdinarySourceSession>,
        reports: &mut CtfInitializationReports,
        weather: &mut WeatherSession,
        model: &TypedModel,
        reporting: &HeatBalanceOrdinaryWarmupReportInputs,
        output_host: &mut SourceMtdFileHost,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<bool, RuntimeError> {
        self.next_source_environment_with_available(
            session,
            reports,
            weather,
            model,
            reporting,
            output_host,
            backend,
            AvailableBorrow::PrimaryLocal,
        )
    }
    fn next_source_environment_with_available(
        &mut self,
        session: &mut Option<OrdinarySourceSession>,
        reports: &mut CtfInitializationReports,
        weather: &mut WeatherSession,
        model: &TypedModel,
        reporting: &HeatBalanceOrdinaryWarmupReportInputs,
        output_host: &mut SourceMtdFileHost,
        backend: &mut SourceErrorBackend<'_>,
        available: AvailableBorrow<'_>,
    ) -> Result<bool, RuntimeError> {
        self.ensure_source_session(session, reports, &weather.state, model, reporting)?;
        backend.with_scoped(
            &mut (self, session, reports, weather, available),
            |mut call_backend, (files, session, reports, weather, available)| {
                let available = match available {
                    AvailableBorrow::Caller(value) => &mut **value,
                    AvailableBorrow::PrimaryLocal => {
                        reports.source_run_available.as_mut().ok_or_else(|| {
                            hb_input_error(Error::Unavailable(
                                "actual ManageSimulation249 Available local",
                            ))
                        })?
                    }
                };
                let session = session.as_mut().ok_or_else(|| {
                    hb_input_error(Error::Unavailable(
                        "actual module constructors before GetNextEnvironment",
                    ))
                })?;
                let project = reports.selected_project_data.as_ref().ok_or_else(|| {
                    hb_input_error(Error::Unavailable(
                        "actual same project before GetNextEnvironment",
                    ))
                })?;
                let input = reports
                    .source_initialization_input
                    .as_ref()
                    .ok_or_else(|| {
                        hb_input_error(Error::Unavailable(
                            "actual same raw initialization before GetNextEnvironment",
                        ))
                    })?;
                let (mut streams, eio) = files.source_weather_streams_and_eio(
                    &reports.source_sqlite,
                    &mut reports.source_results_output_variables,
                );
                let mut owners = SourceWeatherInitializationOwners {
                    raw: reports.heat_balance_input.as_ref().ok_or_else(|| {
                        hb_input_error(Error::Unavailable(
                            "same actual raw model before GetNextEnvironment",
                        ))
                    })?,
                    system: &project.system,
                    reverse_design_days: &input.reverse_design_days,
                    globals: reports
                        .selected_heat_balance_globals
                        .as_ref()
                        .ok_or_else(|| {
                            hb_input_error(Error::Unavailable(
                                "same actual globals before GetNextEnvironment",
                            ))
                        })?,
                    control: &reports.heat_balance_input_state,
                    input_weather_file_path: &input.input_weather_file_path,
                    input: reports.selected_input_processor.as_mut().ok_or_else(|| {
                        hb_input_error(Error::Unavailable(
                            "same actual IP before GetNextEnvironment",
                        ))
                    })?,
                    shortcut: &mut reports.input_processor_shortcut,
                    registration: &mut reports.output_registration,
                    report_flags: &mut reports.source_report_flags,
                    files: &mut streams,
                    eio,
                    backend: &mut call_backend,
                };
                let mut children = FirstWeatherChildren {
                    water: &session.hvac.ancillary.water,
                    registry: &reports.registry,
                    table: &mut reports.table,
                    output_host,
                    prescan: &reports.output_prescan,
                    report_inputs: &model.report_inputs,
                    minimum_report_frequency: &project.system.minimum_report_frequency,
                    schedules: reports.selected_schedule_manager.as_mut().ok_or_else(|| {
                        hb_input_error(Error::Unavailable(
                            "same actual schedules before output request registration",
                        ))
                    })?,
                };
                weather
                    .get_next_source_environment(&mut owners, &mut children, available)
                    .map_err(Failure::into_runtime)
            },
        )
    }
}
