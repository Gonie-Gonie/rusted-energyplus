//! Actual consumed EMS and HVAC startup owner slice; later checks remain held.
use super::error_backend::{SourceErrorBackend, SourceErrorBackendFailure, SourceFatalStop};
use super::report_scan::{
    ReportScanDiagnostic, ReportScanFailure, ReportScanState, ReportScanUnavailable,
    SourceReportScanInput, try_scan_for_reports, try_scan_for_reports_with_input,
};
use super::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use super::selected_input_processor::{SelectedInputProcessor, SourceInputProcessorShortCut};
use ep_model::ReportInputs;
#[derive(Debug)]
pub(crate) enum StartupError {
    Backend(SourceErrorBackendFailure),
    Scan(ReportScanUnavailable),
    Fatal(SourceFatalStop),
    Filesystem(std::io::Error),
    Unavailable(&'static str),
}
impl From<SourceErrorBackendFailure> for StartupError {
    fn from(value: SourceErrorBackendFailure) -> Self {
        Self::Backend(value)
    }
}
impl std::fmt::Display for StartupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Backend(e) => std::fmt::Display::fmt(e, f),
            Self::Scan(e) => write!(f, "startup report request unavailable: {e:?}"),
            Self::Fatal(e) => std::fmt::Display::fmt(e, f),
            Self::Filesystem(e) => std::fmt::Display::fmt(e, f),
            Self::Unavailable(e) => write!(f, "selected simulation startup unavailable: {e}"),
        }
    }
}
impl std::error::Error for StartupError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Backend(e) => Some(e),
            Self::Fatal(e) => Some(e),
            Self::Filesystem(e) => Some(e),
            Self::Scan(_) | Self::Unavailable(_) => None,
        }
    }
}
/// The sixteen actual RuntimeLanguage counts written on each CheckIfAnyEMS call.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct SelectedRuntimeLanguage {
    pub(crate) number_of_sensors: usize,
    pub(crate) number_of_actuators_used: usize,
    pub(crate) number_of_program_call_managers: usize,
    pub(crate) number_of_erl_programs: usize,
    pub(crate) number_of_erl_subroutines: usize,
    pub(crate) number_of_user_global_variables: usize,
    pub(crate) number_of_ems_output_variables: usize,
    pub(crate) number_of_ems_metered_output_variables: usize,
    pub(crate) number_of_ems_curve_indices: usize,
    pub(crate) number_of_external_interface_global_variables: usize,
    pub(crate) number_of_fmu_import_global_variables: usize,
    pub(crate) number_of_fmu_export_global_variables: usize,
    pub(crate) number_of_external_interface_actuators: usize,
    pub(crate) number_of_fmu_import_actuators: usize,
    pub(crate) number_of_fmu_export_actuators: usize,
    pub(crate) number_of_ems_construction_indices: usize,
    pub(crate) output_edd_file: bool,
}

/// Sole selected DataHVACGlobals scalars; constructors are +0, not the zone step.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SelectedHvacGlobals {
    /// DataHVACGlobals507 constructor; not DataSizing/NumAirLoops or a raw recount.
    pub(crate) number_of_primary_air_systems: i32,
    pub(crate) time_step_sys: f64,
    pub(crate) time_step_sys_seconds: f64,
    pub(crate) limit_num_sys_steps: i32,
    pub(crate) deviation_from_setpoint_heating: f64,
    pub(crate) deviation_from_setpoint_cooling: f64,
}
impl Default for SelectedHvacGlobals {
    fn default() -> Self {
        Self {
            number_of_primary_air_systems: 0,
            time_step_sys: 0.0,
            time_step_sys_seconds: 0.0,
            limit_num_sys_steps: 0,
            deviation_from_setpoint_heating: -0.2,
            deviation_from_setpoint_cooling: 0.2,
        }
    }
}

/// Sole actual preconstruction bound, initialized before any window allocation.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct SelectedConstructionBounds {
    pub(crate) max_solid_window_layers: i32,
}
impl SelectedConstructionBounds {
    pub(crate) fn set_preconstruction_input_parameters(
        &mut self,
        input: &SelectedInputProcessor,
    ) -> Result<(), StartupError> {
        self.max_solid_window_layers = 7;
        if input.get_num_objects_found("Construction:ComplexFenestrationState") > 0 {
            self.max_solid_window_layers = self.max_solid_window_layers.max(10);
        }
        let constructions = input.get_num_objects_found("Construction:WindowEquivalentLayer");
        if constructions > 0 {
            // The real positive loop must read getObjectItem; this admitted route
            // does not replace its layer fields with counts or guessed bounds.
            return Err(StartupError::Unavailable(
                "positive WindowEquivalentLayer getter",
            ));
        }
        Ok(())
    }
}

mod checks;
pub(crate) use checks::{check_environment_specifications, check_for_requested_reporting};

fn fatal(backend: &mut SourceErrorBackend<'_>, message: &str) -> Result<(), StartupError> {
    match backend.show_fatal_error(message) {
        Ok(never) => match never {},
        Err(error) => Err(StartupError::Fatal(error)),
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SelectedSimulationStartup {
    pub(crate) runtime_language: SelectedRuntimeLanguage,
    pub(crate) construction_bounds: SelectedConstructionBounds,
    pub(crate) plant_loops: Option<Vec<()>>,
    /// Actual SimulationManager205 result, written before either init guard.
    pub(crate) run_periods_in_input: bool,
    pub(crate) hvac_globals: SelectedHvacGlobals,
}
impl SelectedSimulationStartup {
    pub(crate) fn check_if_any_ems(
        &mut self,
        input: &mut SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        globals: &mut SelectedHeatBalanceGlobals,
        report_inputs: &ReportInputs,
        scan: &mut ReportScanState,
        external_hvac_manager: bool,
        running_via_api: bool,
        active_callback_count: usize,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), StartupError> {
        let r = &mut self.runtime_language;
        r.number_of_sensors = input.get_num_objects_found("EnergyManagementSystem:Sensor");
        r.number_of_actuators_used = input.get_num_objects_found("EnergyManagementSystem:Actuator");
        r.number_of_program_call_managers =
            input.get_num_objects_found("EnergyManagementSystem:ProgramCallingManager");
        r.number_of_erl_programs = input.get_num_objects_found("EnergyManagementSystem:Program");
        r.number_of_erl_subroutines =
            input.get_num_objects_found("EnergyManagementSystem:Subroutine");
        r.number_of_user_global_variables =
            input.get_num_objects_found("EnergyManagementSystem:GlobalVariable");
        r.number_of_ems_output_variables =
            input.get_num_objects_found("EnergyManagementSystem:OutputVariable");
        r.number_of_ems_metered_output_variables =
            input.get_num_objects_found("EnergyManagementSystem:MeteredOutputVariable");
        r.number_of_ems_curve_indices =
            input.get_num_objects_found("EnergyManagementSystem:CurveOrTableIndexVariable");
        r.number_of_external_interface_global_variables =
            input.get_num_objects_found("ExternalInterface:Variable");
        r.number_of_fmu_import_global_variables =
            input.get_num_objects_found("ExternalInterface:FunctionalMockupUnitImport:To:Variable");
        r.number_of_fmu_export_global_variables =
            input.get_num_objects_found("ExternalInterface:FunctionalMockupUnitExport:To:Variable");
        r.number_of_external_interface_actuators =
            input.get_num_objects_found("ExternalInterface:Actuator");
        r.number_of_fmu_import_actuators =
            input.get_num_objects_found("ExternalInterface:FunctionalMockupUnitImport:To:Actuator");
        r.number_of_fmu_export_actuators =
            input.get_num_objects_found("ExternalInterface:FunctionalMockupUnitExport:To:Actuator");
        r.number_of_ems_construction_indices =
            input.get_num_objects_found("EnergyManagementSystem:ConstructionIndexVariable");
        let output_ems_objects = input.get_num_objects_found("Output:EnergyManagementSystem");
        let python_plugins = input.get_num_objects_found("PythonPlugin:Instance");
        globals.any_energy_management_system_in_model = [
            r.number_of_sensors,
            r.number_of_actuators_used,
            r.number_of_program_call_managers,
            r.number_of_erl_programs,
            r.number_of_erl_subroutines,
            r.number_of_user_global_variables,
            r.number_of_ems_output_variables,
            r.number_of_ems_curve_indices,
            r.number_of_external_interface_global_variables,
            r.number_of_external_interface_actuators,
            r.number_of_ems_construction_indices,
            r.number_of_ems_metered_output_variables,
            r.number_of_fmu_import_actuators,
            r.number_of_fmu_import_global_variables,
            r.number_of_fmu_export_actuators,
            r.number_of_fmu_export_global_variables,
            output_ems_objects,
            python_plugins,
            active_callback_count,
        ]
        .iter()
        .any(|&count| count > 0);
        globals.any_energy_management_system_in_model = globals
            .any_energy_management_system_in_model
            || external_hvac_manager
            || running_via_api;
        let mut diagnostic = |diagnostic| match diagnostic {
            ReportScanDiagnostic::Warning(message) => backend.show_warning_error(&message),
            ReportScanDiagnostic::Continue(message) => backend.show_continue_error(&message),
        };
        let result = if scan.get_report_input {
            let mut source = SourceReportScanInput::new(input, shortcut, report_inputs)
                .map_err(StartupError::Scan)?;
            try_scan_for_reports_with_input(
                report_inputs,
                scan,
                "EnergyManagementSystem",
                None,
                None,
                None,
                &mut source,
                &mut diagnostic,
            )
        } else {
            // A cached request does not inspect or rewrite inactive getter operands.
            try_scan_for_reports(
                report_inputs,
                scan,
                "EnergyManagementSystem",
                None,
                None,
                None,
                &mut diagnostic,
            )
        };
        r.output_edd_file = result.map_err(|error| match error {
            ReportScanFailure::Unavailable(error) => StartupError::Scan(error),
            ReportScanFailure::Diagnostic(error) => StartupError::Backend(error),
        })?;
        if globals.any_energy_management_system_in_model {
            return Err(StartupError::Unavailable(
                "active EMS/API/plugin/EDD producer outside selected route",
            ));
        }
        if r.output_edd_file {
            backend.show_warning_error("CheckIFAnyEMS: No EnergyManagementSystem has been set up in the input file but output is requested.")?;
            backend.show_continue_error("No EDD file will be produced. Refer to EMS Application Guide and/or InputOutput Reference to set up your EnergyManagementSystem.")?;
        }
        Ok(())
    }
    pub(crate) fn is_input_object_used(
        &mut self,
        input: &mut SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        globals: &mut SelectedHeatBalanceGlobals,
        report_inputs: &ReportInputs,
        scan: &mut ReportScanState,
        external_hvac_manager: bool,
        running_via_api: bool,
        active_callback_count: usize,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), StartupError> {
        globals.air_loop_hvac_doas_used_in_sim =
            input.get_num_objects_found("AirLoopHVAC:DedicatedOutdoorAirSystem") > 0;
        self.check_if_any_ems(
            input,
            shortcut,
            globals,
            report_inputs,
            scan,
            external_hvac_manager,
            running_via_api,
            active_callback_count,
            backend,
        )?;
        shortcut.current_module_object = "PlantLoop".into();
        let plant = input.get_num_objects_found(&shortcut.current_module_object);
        shortcut.current_module_object = "CondenserLoop".into();
        let condenser = input.get_num_objects_found(&shortcut.current_module_object);
        globals.any_plant_in_model = plant > 0 || condenser > 0;
        if !globals.any_plant_in_model {
            self.plant_loops = Some(Vec::new());
        }
        globals.any_slabs_in_model = input.get_num_objects_found("Site:GroundDomain:Slab") > 0;
        globals.any_basements_in_model =
            input.get_num_objects_found("Site:GroundDomain:Basement") > 0;
        globals.any_ideal_cond_ent_setpoint_in_model =
            input.get_num_objects_found("SetpointManager:CondenserEnteringReset:Ideal") > 0;
        Ok(())
    }
}
