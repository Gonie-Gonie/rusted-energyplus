//! Actual sequential Weather and Exterior calls before the first HB input.
use super::OrdinarySourceSession;
use crate::RuntimeError;
use crate::heat_balance::ctf_initialization_context::{CtfInitializationReports, hb_input_error};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::ordinary_exterior_energy_use::SourceExteriorEnergyOwners;
use crate::heat_balance::ordinary_hvac_manager::water_manager::SourceWaterManager;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_schedule_manager::SelectedScheduleManager;
use crate::heat_balance::source_initialization::SourceInitializationFiles;
use crate::heat_balance::source_output_registration::SourceOutputFileHost;
use crate::psychrometrics::SourcePsychrometricAccess;
use crate::weather::day::source_current_weather::{SourceCurrentWeatherOwners, SourceEcoRoofData};
use crate::weather::day::source_initialization::SourceWeatherInitializationOwners;
use crate::weather::day::source_weather_initialization::SourceManageWeatherChildren;
use crate::weather::day::{SourceWeatherInitializationError as Failure, WeatherSession};
use ep_model::TypedModel;

struct CurrentWeatherChild<'a> {
    schedules: &'a mut SelectedScheduleManager,
    water: &'a mut SourceWaterManager,
    eco_roof: &'a mut SourceEcoRoofData,
}
impl SourceManageWeatherChildren for CurrentWeatherChild<'_> {
    fn set_current_source_weather(
        &mut self,
        session: &mut WeatherSession,
        owners: &mut SourceWeatherInitializationOwners<'_, '_>,
        _psychrometrics: &mut SourcePsychrometricAccess,
    ) -> Result<(), Failure> {
        // The stateless passed PSY handle and Current's accepted wrappers reach
        // the same PROPERTY_STATE. This creates no fresh scope/cache/owner.
        session.set_source_current_weather(&mut SourceCurrentWeatherOwners {
            schedules: self.schedules,
            water: self.water,
            eco_roof: self.eco_roof,
            backend: owners.backend,
        })
    }
}
impl OrdinarySourceSession {
    pub(super) fn run_source_weather(
        &mut self,
        reports: &mut CtfInitializationReports,
        files: &mut SourceInitializationFiles,
        weather: &mut WeatherSession,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), RuntimeError> {
        backend.with_scoped(
            &mut (self, reports, files, weather),
            |mut call_backend, (session, reports, files, weather)| {
                let input = reports
                    .source_initialization_input
                    .as_ref()
                    .ok_or_else(|| {
                        hb_input_error(Error::Unavailable(
                            "actual raw initialization owners before ManageWeather",
                        ))
                    })?;
                let (mut streams, eio) = files.source_weather_streams_and_eio(
                    &reports.source_sqlite,
                    &mut reports.source_results_output_variables,
                );
                let mut owners = SourceWeatherInitializationOwners {
                    raw: reports.heat_balance_input.as_ref().ok_or_else(|| {
                        hb_input_error(Error::Unavailable("same raw source input before Weather"))
                    })?,
                    system: &reports
                        .selected_project_data
                        .as_ref()
                        .ok_or_else(|| {
                            hb_input_error(Error::Unavailable(
                                "same actual Project system before Weather",
                            ))
                        })?
                        .system,
                    reverse_design_days: &input.reverse_design_days,
                    globals: reports
                        .selected_heat_balance_globals
                        .as_ref()
                        .ok_or_else(|| {
                            hb_input_error(Error::Unavailable("same actual globals before Weather"))
                        })?,
                    control: &reports.heat_balance_input_state,
                    input_weather_file_path: &input.input_weather_file_path,
                    input: reports.selected_input_processor.as_mut().ok_or_else(|| {
                        hb_input_error(Error::Unavailable("same actual IP before Weather"))
                    })?,
                    shortcut: &mut reports.input_processor_shortcut,
                    registration: &mut reports.output_registration,
                    report_flags: &mut reports.source_report_flags,
                    files: &mut streams,
                    eio,
                    backend: &mut call_backend,
                };
                let mut children = CurrentWeatherChild {
                    schedules: reports.selected_schedule_manager.as_mut().ok_or_else(|| {
                        hb_input_error(Error::Unavailable(
                            "same actual schedule owner before Weather",
                        ))
                    })?,
                    water: &mut session.hvac.ancillary.water,
                    eco_roof: &mut session.eco_roof,
                };
                let mut psychrometrics = SourcePsychrometricAccess;
                weather
                    .manage_source_weather(&mut owners, &mut psychrometrics, &mut children)
                    .map_err(Failure::into_runtime)
            },
        )
    }
    pub(super) fn run_source_exterior<H: SourceOutputFileHost>(
        &mut self,
        reports: &mut CtfInitializationReports,
        files: &mut SourceInitializationFiles,
        weather: &mut WeatherSession,
        model: &TypedModel,
        output_host: &mut H,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), RuntimeError> {
        backend.with_scoped(
            &mut (self, reports, files, weather),
            |mut call_backend, (session, reports, files, weather)| {
                let (mut dictionary_host, eio) = files.source_report_streams_and_eio(
                    &reports.source_sqlite,
                    &mut reports.source_results_output_variables,
                );
                let mut owners = SourceExteriorEnergyOwners {
                    raw: reports.heat_balance_input.as_ref().ok_or_else(|| {
                        hb_input_error(Error::Unavailable("same actual raw source before Exterior"))
                    })?,
                    input: reports.selected_input_processor.as_mut().ok_or_else(|| {
                        hb_input_error(Error::Unavailable("same actual IP before Exterior"))
                    })?,
                    shortcut: &mut reports.input_processor_shortcut,
                    schedules: reports.selected_schedule_manager.as_mut().ok_or_else(|| {
                        hb_input_error(Error::Unavailable("same actual schedules before Exterior"))
                    })?,
                    globals: reports
                        .selected_heat_balance_globals
                        .as_ref()
                        .ok_or_else(|| {
                            hb_input_error(Error::Unavailable(
                                "same actual globals before Exterior",
                            ))
                        })?,
                    clock: &weather.state.global,
                    environment: &weather.state.environment,
                    registration: &mut reports.output_registration,
                    prescan: &reports.output_prescan,
                    reports: &model.report_inputs,
                    minimum_report_frequency: &reports
                        .selected_project_data
                        .as_ref()
                        .ok_or_else(|| {
                            hb_input_error(Error::Unavailable(
                                "same Project frequency before Exterior",
                            ))
                        })?
                        .system
                        .minimum_report_frequency,
                    host: output_host,
                    dictionary_host: &mut dictionary_host,
                    eio,
                    registry: &reports.registry,
                    table: &mut reports.table,
                    backend: &mut call_backend,
                };
                session
                    .exterior
                    .manage_source_exterior_energy_use(&mut owners)
                    .map_err(hb_input_error)
            },
        )
    }
}
