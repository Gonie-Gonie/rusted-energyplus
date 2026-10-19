//! Source-ordered EnergyPlusData initialization on the caller's retained owners.
//! The 249-method closure is Source-audited; only literal-empty methods are omitted.
//! These call guards are set before dispatch and are not readiness certificates.

use super::ctf_initialization_context::CtfInitializationReports;
use super::error_backend::{SourceErrorBackend, SourceFatalStop};
use super::selected_fluid_properties::{SelectedFluidError, SelectedFluidProperties};
use super::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use super::selected_project_data::{ProjectDataError, ProjectDataOwners, SelectedProjectData};
use super::selected_schedule_manager::{SelectedScheduleError, SelectedScheduleManager};
use super::selected_simulation_startup::{SelectedSimulationStartup, StartupError};
use super::setup_postludes::{SelectedCurves, SelectedSetupServiceError};
use crate::weather::day::{
    ProductionWeatherTimestepSeries, WeatherDayError, WeatherDayState,
    WeatherEnvironmentConfiguration,
};
use ep_model::ReportInputs;
use std::io;

mod files;
pub(crate) use files::SourceCloseOutputFilesOwners;
mod kickoff_postlude;
pub(crate) use kickoff_postlude::SourceFaultsManager;
mod output_reporting;
mod produce_dictionaries;
mod simulation_parent;
pub(crate) use output_reporting::SourceSqliteState;
pub(crate) mod primary_startup_surfaces;
mod run_prefix;
mod unopened_weather;
pub(crate) use files::{
    ScheduleAuditFileHost, SourceRuntimeMeterStreams, SourceRuntimeReportStreams,
};
pub use files::{SourceInitializationFilePaths, SourceInitializationFiles};

/// Actual raw operands supplied after the same ordinary input admission.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SourceInitializationInput {
    /// Raw GetProjectData fields and authentic IDF/epJSON mode.
    pub project: super::selected_project_data::SelectedProjectDataInput,
    /// Actual source-ordered constant schedule inputs.
    pub schedules: super::selected_schedule_manager::SelectedScheduleInput,
    /// Actual ordered SummaryReports inputs for ManageSizing234.
    pub sizing: super::selected_sizing::SelectedSizingInput,
    /// Actual ordered Table:Style operands admitted by the same input processor.
    pub tabular: super::ordinary_tabular_reports::SelectedTabularInput,
    /// Caller-supplied genuine weather path for CheckEnvironment213 fileExists.
    pub input_weather_file_path: std::path::PathBuf,
    /// Genuine command-line DesignDay-only request, separate from process settings.
    pub cli_dd_only: bool,
    /// Genuine command-line annual request, separate from process settings.
    pub cli_annual: bool,
    /// Already captured sole ReverseDD process setting.
    pub reverse_design_days: bool,
}

/// EnergyPlusData.hh610-611 guards; true denotes that a call was entered.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SourceInitializationGuards {
    /// Set before the first constant-state member dispatch; retained after error.
    pub init_constant_state_called: bool,
    /// Set before SimulationManager input-state dispatch; retained after error.
    pub init_state_called: bool,
}

/// Genuine reached initialization error; preceding owners and files stay retained.
#[derive(Debug)]
pub enum SourceInitializationError {
    /// A real GetProjectData diagnostic, formatter or output failed.
    Project(ProjectDataError),
    /// A selected member handler failed; the actual typed cause remains retained.
    Handler {
        /// Actual selected member whose call failed.
        handler: &'static str,
        /// Unmodified typed error retained behind the public boundary.
        source: Box<dyn std::error::Error>,
    },
    /// A real filesystem/write operation failed and retains its OS cause.
    Io(io::Error),
    /// Original ensure_open fatal bridge stopped after the actual open failure.
    Fatal {
        /// Actual filesystem failure that caused the fatal bridge.
        source: io::Error,
        /// Actual fatal stop after the backend retained its reached prefix.
        stop: SourceFatalStop,
    },
    /// A caller omitted a required actual owner; no successful state is invented.
    Association(&'static str),
    /// Later weather allocation failed while the same pre-weather state is retained.
    Weather(WeatherDayError),
}
// RuntimeError's established PartialEq API needs an explicit rule for this new
// typed branch. Defined value variants compare values; opaque/resource causes
// compare their actual identities. No error is formatted or replaced for equality.
impl PartialEq for SourceInitializationError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Association(a), Self::Association(b)) => a == b,
            (Self::Weather(a), Self::Weather(b)) => a == b,
            (
                Self::Project(ProjectDataError::Unavailable(a)),
                Self::Project(ProjectDataError::Unavailable(b)),
            ) => a == b,
            (
                Self::Project(ProjectDataError::Format(a)),
                Self::Project(ProjectDataError::Format(b)),
            ) => a == b,
            (Self::Project(a), Self::Project(b)) => std::ptr::eq(a, b),
            (Self::Io(a), Self::Io(b)) => std::ptr::eq(a, b),
            (
                Self::Handler { handler: a, source: x },
                Self::Handler { handler: b, source: y },
            ) => a == b && std::ptr::eq(x.as_ref(), y.as_ref()),
            (
                Self::Fatal { source: a, stop: x },
                Self::Fatal { source: b, stop: y },
            ) => std::ptr::eq(a, b) && std::ptr::eq(x, y),
            _ => false,
        }
    }
}

macro_rules! from_error {
    ($source:ty, $variant:ident) => {
        impl From<$source> for SourceInitializationError {
            fn from(value: $source) -> Self {
                Self::$variant(value)
            }
        }
    };
}
from_error!(ProjectDataError, Project);
impl From<StartupError> for SourceInitializationError {
    fn from(source: StartupError) -> Self {
        Self::Handler {
            handler: "EMS",
            source: Box::new(source),
        }
    }
}
impl From<SelectedFluidError> for SourceInitializationError {
    fn from(source: SelectedFluidError) -> Self {
        Self::Handler {
            handler: "Fluid",
            source: Box::new(source),
        }
    }
}
impl From<SelectedScheduleError> for SourceInitializationError {
    fn from(source: SelectedScheduleError) -> Self {
        Self::Handler {
            handler: "Schedule",
            source: Box::new(source),
        }
    }
}
impl From<SelectedSetupServiceError> for SourceInitializationError {
    fn from(source: SelectedSetupServiceError) -> Self {
        Self::Handler {
            handler: "Curve",
            source: Box::new(source),
        }
    }
}
from_error!(io::Error, Io);
from_error!(WeatherDayError, Weather);
impl std::fmt::Display for SourceInitializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Project(e) => std::fmt::Display::fmt(e, f),
            Self::Handler { handler, source } => write!(f, "{handler}: {source}"),
            Self::Io(e) => std::fmt::Display::fmt(e, f),
            Self::Fatal { stop, .. } => std::fmt::Display::fmt(stop, f),
            Self::Association(reason) => {
                write!(f, "actual initialization owner unavailable: {reason}")
            }
            Self::Weather(e) => std::fmt::Display::fmt(e, f),
        }
    }
}
impl std::error::Error for SourceInitializationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Project(e) => Some(e),
            Self::Handler { source, .. } => Some(source.as_ref()),
            Self::Io(e) => Some(e),
            Self::Fatal { source, .. } => Some(source),
            Self::Weather(e) => Some(e),
            Self::Association(_) => None,
        }
    }
}

fn open_output_files(
    files: &mut SourceInitializationFiles,
    backend: &mut SourceErrorBackend<'_>,
    record_count: &mut i64,
) -> Result<(), SourceInitializationError> {
    *record_count = 0; // SimulationManager1747, before the first ensure_open.
    for index in 0..4 {
        let path = files.output_path(index).to_owned();
        let stream = match files.ensure_output_open(index) {
            Ok(stream) => stream,
            Err(source) => {
                let message = format!(
                    "OpenOutputFiles: Could not open file {} for output (write).",
                    path.display()
                );
                return match backend.show_fatal_error(&message) {
                    Ok(never) => match never {},
                    Err(stop) => Err(SourceInitializationError::Fatal { source, stop }),
                };
            }
        };
        writeln!(stream, "Program Version,{}", backend.state.version_string)?;
    }
    Ok(())
}

impl CtfInitializationReports {
    /// Execute the real simulation wrapper operands before either parent guard.
    /// The concrete ordinary caller invokes this once, retaining reached state on error.
    ///
    /// # Errors
    /// Preserves partial environment effects if an actual environment value is invalid.
    pub fn prepare_source_environment(
        &mut self,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), SourceInitializationError> {
        self.selected_heat_balance_globals
            .get_or_insert_with(SelectedHeatBalanceGlobals::default);
        self.selected_simulation_startup
            .get_or_insert_with(SelectedSimulationStartup::default);
        self.selected_project_data
            .get_or_insert_with(SelectedProjectData::default);
        self.selected_fluid_properties
            .get_or_insert_with(SelectedFluidProperties::default);
        self.selected_schedule_manager
            .get_or_insert_with(SelectedScheduleManager::default);
        self.selected_curves
            .get_or_insert_with(SelectedCurves::default);
        self.source_initialization_weather
            .get_or_insert_with(WeatherDayState::default);
        let raw = self.source_initialization_input.as_ref().ok_or(
            SourceInitializationError::Association("actual raw initialization input"),
        )?;
        let project = self
            .selected_project_data
            .as_mut()
            .ok_or(SourceInitializationError::Association("project"))?;
        let globals = self
            .selected_heat_balance_globals
            .as_mut()
            .ok_or(SourceInitializationError::Association("DataGlobal"))?;
        let weather = self
            .source_initialization_weather
            .as_mut()
            .ok_or(SourceInitializationError::Association("pre-weather owner"))?;
        project.resolve_environment_operands(
            globals,
            &mut weather.environment,
            raw.cli_dd_only,
            raw.cli_annual,
            raw.reverse_design_days,
        )?;
        backend.state.developer_flag = project.system.developer_flag;
        Ok(())
    }

    /// Run the selected genuine constant/input-state handlers before weather allocation.
    /// The caller supplies actual files, report inputs and the same diagnostic backend.
    /// No sizing, SetupSimulation or downstream HB completion is implied.
    ///
    /// # Errors
    /// Leaves both guards, every reached owner and all actual output prefixes intact.
    pub fn initialize_source_state(
        &mut self,
        files: &mut SourceInitializationFiles,
        reports: &ReportInputs,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), SourceInitializationError> {
        if self.source_initialization_guards.init_constant_state_called
            && self.source_initialization_guards.init_state_called
        {
            return Ok(());
        }
        self.selected_heat_balance_globals
            .get_or_insert_with(SelectedHeatBalanceGlobals::default);
        self.selected_simulation_startup
            .get_or_insert_with(SelectedSimulationStartup::default);
        self.selected_project_data
            .get_or_insert_with(SelectedProjectData::default);
        self.selected_fluid_properties
            .get_or_insert_with(SelectedFluidProperties::default);
        self.selected_schedule_manager
            .get_or_insert_with(SelectedScheduleManager::default);
        self.selected_curves
            .get_or_insert_with(SelectedCurves::default);
        self.source_initialization_weather
            .get_or_insert_with(WeatherDayState::default);

        let input = self.selected_input_processor.as_mut().ok_or(
            SourceInitializationError::Association("selected_input_processor"),
        )?;
        let raw = self.source_initialization_input.as_ref().ok_or(
            SourceInitializationError::Association("actual raw initialization input"),
        )?;
        let globals = self
            .selected_heat_balance_globals
            .as_mut()
            .ok_or(SourceInitializationError::Association("DataGlobal"))?;
        let startup = self
            .selected_simulation_startup
            .as_mut()
            .ok_or(SourceInitializationError::Association("startup"))?;
        let project = self
            .selected_project_data
            .as_mut()
            .ok_or(SourceInitializationError::Association("project"))?;
        let fluid = self
            .selected_fluid_properties
            .as_mut()
            .ok_or(SourceInitializationError::Association("Fluid"))?;
        let schedules = self
            .selected_schedule_manager
            .as_mut()
            .ok_or(SourceInitializationError::Association("Schedule"))?;
        let curves = self
            .selected_curves
            .as_mut()
            .ok_or(SourceInitializationError::Association("Curve"))?;
        let weather = self
            .source_initialization_weather
            .as_mut()
            .ok_or(SourceInitializationError::Association("pre-weather owner"))?;
        // Actual RunSimulation205 precedes both EnergyPlusData guards. No
        // query is repeated when the parent guard already entered on an Err.
        if !self.source_initialization_guards.init_constant_state_called
            && !self.source_initialization_guards.init_state_called
        {
            startup.run_periods_in_input = input.get_num_objects_found("RunPeriod") > 0
                || input.get_num_objects_found("RunPeriod:CustomRange") > 0
                || project.system.full_annual_run;
            backend.state.tracking.ask_for_connections_report = false; // Actual208.
        }
        if !self.source_initialization_guards.init_constant_state_called {
            self.source_initialization_guards.init_constant_state_called = true;
            // Ordinals1/2 are literal-empty. Ordinal3 clears only the four caches.
            crate::psychrometrics::initialize_source_psychrometric_caches();
            fluid.init_constant_state()?; // Ordinal4.
            let steps = usize::try_from(weather.global.time_steps_in_hour)
                .map_err(|_| SourceInitializationError::Association("constructor timestep"))?;
            schedules.init_constant_state(steps)?; // Ordinal5; actual fresh value0.
            // All remaining244 direct member bodies are Source-proven literal-empty.
        }
        if !self.source_initialization_guards.init_state_called {
            self.source_initialization_guards.init_state_called = true;
            open_output_files(
                files,
                backend,
                &mut self.source_standard_output_record_count,
            )?;
            let mut owners = ProjectDataOwners {
                input: &mut *input,
                shortcut: &mut self.input_processor_shortcut,
                globals: &mut *globals,
                weather: &mut *weather,
                hvac: &mut startup.hvac_globals,
                heat_balance_current_module_object: &mut self.heat_balance_current_module_object,
            };
            project.get_project_data(&raw.project, &mut owners, backend, files.eio_writer()?)?;
            backend.state.developer_flag = project.system.developer_flag;
            // Ordinal2; no external HVAC manager, API or plugin callbacks exist in
            // this concrete CLI host. Active raw EMS remains an explicit stop.
            startup.check_if_any_ems(
                input,
                &mut self.input_processor_shortcut,
                globals,
                reports,
                &mut self.scan,
                false,
                false,
                0,
                backend,
            )?;
            // Ordinal3 Psych::init_state is literal-empty; do not clear its caches.
            fluid.get_fluid_properties_data(input)?;
            let steps = usize::try_from(weather.global.time_steps_in_hour)
                .map_err(|_| SourceInitializationError::Association("effective timestep"))?;
            match schedules.process_schedule_input(input, &raw.schedules, steps, files) {
                Err(SelectedScheduleError::AuditOpen { path, source }) => {
                    let message = format!(
                        "ProcessScheduleInput: Could not open file {} for output (write).",
                        path.display()
                    );
                    return match backend.show_fatal_error(&message) {
                        Ok(never) => match never {},
                        Err(stop) => Err(SourceInitializationError::Fatal { source, stop }),
                    };
                }
                result => result?,
            }
            curves.initialize_state(input)?;
            // Remaining243 direct input-state bodies are literal-empty.
        }
        Ok(())
    }

    /// Read the effective timestep from the same real pre-weather global owner.
    ///
    /// # Errors
    /// Fails before weather allocation when the actual owner or positive step is absent.
    pub fn source_zone_timesteps_per_hour(&self) -> Result<u32, SourceInitializationError> {
        let weather = self.source_initialization_weather.as_ref().ok_or(
            SourceInitializationError::Association("initialized weather owner"),
        )?;
        let steps = u32::try_from(weather.global.time_steps_in_hour)
            .map_err(|_| SourceInitializationError::Association("effective timestep"))?;
        if steps == 0 {
            return Err(SourceInitializationError::Association(
                "zero effective timestep",
            ));
        }
        Ok(steps)
    }

    /// Feed the genuine effective Project/environment controls to the reached HB caller.
    /// The existing reporting and radiation inputs remain the same retained values.
    ///
    /// # Errors
    /// Fails when the actual selected global or Project owner is missing.
    pub fn apply_source_reporting_controls(
        &self,
        reporting: &mut crate::HeatBalanceOrdinaryWarmupReportInputs,
    ) -> Result<(), SourceInitializationError> {
        let globals = self
            .selected_heat_balance_globals
            .as_ref()
            .ok_or(SourceInitializationError::Association("DataGlobal"))?;
        let project = self
            .selected_project_data
            .as_ref()
            .ok_or(SourceInitializationError::Association("project"))?;
        if let Some(input) = reporting.equipment_input.as_mut() {
            let steps = usize::try_from(self.source_zone_timesteps_per_hour()?).map_err(|_| {
                SourceInitializationError::Association("equipment timestep representation")
            })?;
            input.apply_source_timestep(steps).map_err(|source| {
                SourceInitializationError::Handler {
                    handler: "geometry equipment timestep",
                    source: Box::new(source),
                }
            })?;
        }
        reporting.detailed = project.system.report_detailed_warmup_convergence;
        reporting.advanced_report_variables = globals.display_advanced_report_variables;
        reporting.do_pure_load_calc = globals.do_pure_load_calc;
        if let Some(input) = reporting.radiation_input.as_mut() {
            input.apply_source_advanced_reporting(globals.display_advanced_report_variables);
        }
        Ok(())
    }

    /// Move the same initialized weather state into its real byte-backed session.
    /// The effective Project timestep controls allocation, configuration and clocks.
    ///
    /// # Errors
    /// Retains the partially prepared original state when allocation/header work fails.
    pub fn initialize_source_weather(
        &mut self,
        bytes: Vec<u8>,
        mut configuration: WeatherEnvironmentConfiguration,
    ) -> Result<ProductionWeatherTimestepSeries, SourceInitializationError> {
        let weather = self.source_initialization_weather.as_mut().ok_or(
            SourceInitializationError::Association("initialized weather owner"),
        )?;
        let project = self
            .selected_project_data
            .as_ref()
            .ok_or(SourceInitializationError::Association("project"))?;
        if !weather.global.do_weath_sim || project.system.dd_only || project.system.full_annual_run
        {
            return Err(SourceInitializationError::Association(
                "effective Project controls require a DesignDay-only or annual caller outside the current RunPeriod executor",
            ));
        }
        let globals = self
            .selected_heat_balance_globals
            .as_mut()
            .ok_or(SourceInitializationError::Association("DataGlobal"))?;
        let startup = self
            .selected_simulation_startup
            .as_mut()
            .ok_or(SourceInitializationError::Association("startup"))?;
        let input = self
            .selected_input_processor
            .as_mut()
            .ok_or(SourceInitializationError::Association("IP"))?;
        let owners = ProjectDataOwners {
            input,
            shortcut: &mut self.input_processor_shortcut,
            globals,
            weather,
            hvac: &mut startup.hvac_globals,
            heat_balance_current_module_object: &mut self.heat_balance_current_module_object,
        };
        configuration.setup_simulation_controls = Some(project.setup_controls(&owners));
        let steps = u32::try_from(owners.weather.global.time_steps_in_hour)
            .map_err(|_| SourceInitializationError::Association("effective timestep"))?;
        if steps == 0 {
            return Err(SourceInitializationError::Association(
                "zero effective timestep",
            ));
        }
        let profile = crate::time_axis::time_axis_timestep_profile_for_zone_timesteps(steps);
        configuration.time_axis.zone_timestep = profile.zone_timestep;
        configuration.time_axis.system_timestep = profile.system_timestep;
        configuration.solar_controls = crate::weather::day::WeatherSolarControls {
            display_weather_missing_data_warnings: owners
                .weather
                .environment
                .display_weather_missing_data_warnings,
            ignore_solar_radiation: owners.weather.environment.ignore_solar_radiation,
            ignore_beam_radiation: owners.weather.environment.ignore_beam_radiation,
            ignore_diffuse_radiation: owners.weather.environment.ignore_diffuse_radiation,
        };
        let series = ProductionWeatherTimestepSeries::from_initialized_project_state(
            bytes,
            configuration,
            owners.weather,
        )?;
        // Successful construction moved the owner; no second default owner is retained.
        self.source_initialization_weather = None;
        Ok(series)
    }
}
