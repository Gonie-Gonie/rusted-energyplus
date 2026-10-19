//! Selected genuine SimulationManager::GetProjectData and retained owners.
//! The real init_state caller supplies the same raw/IP/scratch/weather owners.

use super::error_backend::{SourceErrorBackend, SourceErrorBackendFailure, SourceFatalStop};
use super::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use super::selected_input_processor::{SelectedInputProcessor, SourceInputProcessorShortCut};
use super::selected_simulation_startup::SelectedHvacGlobals;
use crate::weather::day::{SetupSimulationControls, WeatherDayState};
use std::io;

mod input;
pub use input::{
    ProjectDiagnosticsObject, ProjectInputObject, ProjectInputValue, SelectedProjectDataInput,
};
mod environment;
mod threading;
mod getter;

/// Genuine backend/OS failure or explicit unsupported input boundary.
#[derive(Debug)]
pub enum ProjectDataError {
    /// Caller-owned diagnostic backend failed after its real reached prefix.
    Backend(SourceErrorBackendFailure),
    /// Original ShowFatalError terminated this getter.
    Fatal(SourceFatalStop),
    /// Caller-owned EIO failed; earlier writes remain in the same sink.
    Io(io::Error),
    /// A positive unsupported branch or actual owner association is unavailable.
    Unavailable(&'static str),
    /// The existing selected Source R formatter rejected a value.
    Format(String),
}
impl From<SourceErrorBackendFailure> for ProjectDataError {
    fn from(value: SourceErrorBackendFailure) -> Self {
        Self::Backend(value)
    }
}
impl From<io::Error> for ProjectDataError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}
impl std::fmt::Display for ProjectDataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Backend(e) => std::fmt::Display::fmt(e, f),
            Self::Fatal(e) => std::fmt::Display::fmt(e, f),
            Self::Io(e) => std::fmt::Display::fmt(e, f),
            Self::Unavailable(e) => write!(f, "selected GetProjectData unavailable: {e}"),
            Self::Format(e) => f.write_str(e),
        }
    }
}
impl std::error::Error for ProjectDataError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Backend(e) => Some(e),
            Self::Fatal(e) => Some(e),
            Self::Io(e) => Some(e),
            Self::Unavailable(_) | Self::Format(_) => None,
        }
    }
}
fn fatal(backend: &mut SourceErrorBackend<'_>, message: &str) -> Result<(), ProjectDataError> {
    match backend.show_fatal_error(message) {
        Ok(never) => match never {},
        Err(error) => Err(ProjectDataError::Fatal(error)),
    }
}

/// Actual retained SystemVariables members consumed/written by this selected getter.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct ProjectSystemVariables {
    /// Same SystemVarsData.runtimeTimer; actual commonInitialize ticks before Source IP.
    pub(crate) runtime_timer: super::source_initialization::SourceRuntimeTimer,
    // Grouped actual threading members; not another SysVars instance.
    pub(crate) threading: threading::SourceThreadingSystemVariables,
    pub(crate) ci_force_time_step: bool,
    pub(crate) minimal_shadowing: bool,
    pub(crate) minimum_report_frequency: String,
    pub(crate) shadowing_clipping:
        super::selected_heat_balance_input::shadowing::ShadowingSystemSettings,
    pub(crate) dd_only: bool,
    pub(crate) full_annual_run: bool,
    pub(crate) report_during_warmup: bool,
    /// Actual DataSystemVariables.hh130 constructor false.
    pub(crate) update_data_during_warmup_external_interface: bool,
    pub(crate) developer_flag: bool,
    pub(crate) timing_flag: bool,
    /// Actual DataSystemVariables.hh120 ctorfalse, read from TRACK_AIRLOOP.
    pub(crate) track_air_loop_env_flag: bool,
    pub(crate) report_detailed_warmup_convergence: bool,
    pub(crate) report_during_hvac_sizing_simulation: bool,
}

/// Actual DataConvergParams.hh111-123; positive Node histories remain unwritten until Source allocation/write.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceZoneInletConvergence {
    pub(crate) zone_name: String,
    pub(crate) number_inlet_nodes: i32,
    pub(crate) inlet_nodes: Option<Vec<SourceNodeConvergenceLog>>,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceNodeConvergenceLog {
    pub(crate) node_number: i32,
    pub(crate) humidity_ratio: [Option<f64>; 10],
    pub(crate) mass_flow_rate: [Option<f64>; 10],
    pub(crate) temperature: [Option<f64>; 10],
}

/// Actual DataConvergParams constructor; absence writes the four getter defaults.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ProjectConvergence {
    /// DataConvergParams.hh183 actual EPVector; GetAirPath400 allocates it.
    pub(crate) air_loop_convergence: Option<Vec<std::convert::Infallible>>,
    pub(crate) zone_inlet_convergence: Option<Vec<SourceZoneInletConvergence>>,
    pub(crate) min_time_step_sys: f64,
    pub(crate) max_iter: i32,
    pub(crate) min_plant_sub_iterations: i32,
    pub(crate) max_plant_sub_iterations: i32,
    pub(crate) max_zone_temp_diff: f64,
}
impl Default for ProjectConvergence {
    fn default() -> Self {
        Self {
            air_loop_convergence: None,
            zone_inlet_convergence: None,
            min_time_step_sys: 1.0 / 60.0,
            max_iter: 20,
            min_plant_sub_iterations: 2,
            max_plant_sub_iterations: 8,
            max_zone_temp_diff: 0.3,
        }
    }
}

/// Sole retained selected portions of SimulationManager, ReportFlag and HB controls.
/// Input processing runs before Building changes the HB minimum warmup value.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SelectedProjectData {
    pub(crate) run_control_in_input: bool,
    pub(crate) debug_output: bool,
    pub(crate) even_during_warmup: bool,
    pub(crate) make_mirrored_detached_shading: bool,
    pub(crate) make_mirrored_attached_shading: bool,
    pub(crate) system: ProjectSystemVariables,
    pub(crate) convergence: ProjectConvergence,
    pub(crate) carroll_method: bool,
    pub(crate) override_zone_air_solution_algorithm: bool,
    pub(crate) min_number_of_warmup_days: i32,
    pub(crate) max_allowed_delta_temperature: f64,
}
impl Default for SelectedProjectData {
    fn default() -> Self {
        Self {
            run_control_in_input: false,
            debug_output: false,
            even_during_warmup: false,
            make_mirrored_detached_shading: true,
            make_mirrored_attached_shading: true,
            system: ProjectSystemVariables::default(),
            convergence: ProjectConvergence::default(),
            carroll_method: false,
            override_zone_air_solution_algorithm: false,
            min_number_of_warmup_days: 1,
            max_allowed_delta_temperature: 0.002,
        }
    }
}

/// Borrow every sole live owner; no copied readiness or timestep scalar.
pub(crate) struct ProjectDataOwners<'a> {
    pub(crate) input: &'a mut SelectedInputProcessor,
    pub(crate) shortcut: &'a mut SourceInputProcessorShortCut,
    pub(crate) globals: &'a mut SelectedHeatBalanceGlobals,
    pub(crate) weather: &'a mut WeatherDayState,
    pub(crate) hvac: &'a mut SelectedHvacGlobals,
    pub(crate) heat_balance_current_module_object: &'a mut String,
}

impl SelectedProjectData {
    pub(crate) fn setup_controls(&self, owners: &ProjectDataOwners<'_>) -> SetupSimulationControls {
        SetupSimulationControls::from_admitted_input_flags(
            owners.globals.do_zone_sizing,
            owners.globals.do_system_sizing,
            owners.globals.do_plant_sizing,
            owners.globals.do_hvac_sizing_simulation,
            owners.weather.global.do_des_day_sim,
            owners.weather.global.do_weath_sim,
        )
    }
}
