//! Caller-retained owners for the ordinary construction initialization call.
//!
//! Report/table/Scan state and every actual sink outlive both success and error.
//! The caller explicitly supplies its backend; no optional owner is inferred.

use super::ctf_init_postlude::{
    CtfInitPostludeError, CtfInitPostludeProgress, complete_ctf_init_postlude,
};
use super::ctf_public_initialization::OrderedCtfPublicInitialization;
use super::ctf_reports::{CtfReportError, CtfReportUnavailable, PredefinedStringTable};
use super::ctf_source_diagnostics::{CtfSourceDiagnosticStop, try_emit_ctf_loading_diagnostic};
use super::error_backend::{SourceErrorBackend, SourceErrorBackendFailure, SourceFatalStop};
use super::predefined_tables::PredefinedRegistry;
use super::report_scan::ReportScanState;
use super::surface_manager::ctf_layer_preprocessing::CtfLayerDiagnostic;
use ep_model::{Construction, MaterialId, NormalizedName, TypedModel};
use std::io::{self, Write};

pub(crate) mod internal_heat_gains_input;
mod ordinary_surface_geometry;
mod post_building_input;
mod setup_geometry_reports;
mod setup_zone_geometry;
mod source_heat_balance_input;
mod world_surface_geometry;
pub use internal_heat_gains_input::observation::{
    OrdinaryGainDevice, OrdinaryGainDeviceStorage, OrdinaryGainDictionaryEntry,
    OrdinaryGainMiscObject, OrdinaryGainOutputRegistration, OrdinaryGainRateTarget,
    OrdinaryGainReport, OrdinaryGainSchedule, OrdinaryGainSchedules, OrdinaryGainSpace,
    OrdinaryGainZone, OrdinaryInternalGains, OrdinaryOtherEquipment,
};
pub use ordinary_surface_geometry::{
    OrdinaryGeometryConstruction, OrdinaryGeometryMaterial, OrdinaryGeometryProcess,
    OrdinaryGeometryShadeVertices, OrdinaryGeometrySpace, OrdinaryGeometrySurface,
    OrdinaryGeometryZone, OrdinarySurfaceGeometry,
};

/// Full real registry/table/cache owners, retained even if initialization stops.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CtfInitializationReports {
    /// Sole previously unmodeled NumRecurringErrors/RecurringErrors cells from DataErrTracking.
    /// Actual recurring warning records share these persistent rows; summary is a separate child.
    pub(crate) source_recurring_errors: super::source_initialization::SourceRecurringErrorState,
    /// Genuine full registration state; repeated caller invocations append normally.
    pub registry: PredefinedRegistry,
    /// True only after the selected Setup caller reached SetPredefinedTables215.
    /// Legacy callers still register normally; this is not inferred from size.
    pub setup_simulation_registry_prepared: bool,
    /// Actual RunSimulation249 local, None before that assignment is reached.
    /// Retained across a later error; this is not weather/Setup readiness.
    pub source_run_available: Option<bool>,
    /// Actual RunSimulation236 caller local; None before that reached assignment.
    /// The primary loop writes true at410 and never resets it at368.
    pub source_run_sims_done: Option<bool>,
    /// Sole actual ExternalInterfaceData module; genuine native scalar constructors.
    pub(crate) source_external_interface:
        super::ordinary_external_interface::SourceExternalInterface,
    /// Sole actual raw/schema input owner, retained across runtime success and failure.
    /// Legacy callers that supply no raw projection retain None.
    pub selected_input_processor: Option<super::selected_input_processor::SelectedInputProcessor>,
    /// Sole shared input arrays allocated before the ordinary runtime getters.
    pub input_processor_shortcut: super::selected_input_processor::SourceInputProcessorShortCut,
    /// Raw direct-JSON output pre-scan operands, retained independently of getters.
    pub reporting_prescan_input: Option<super::source_output_prescan::SourceReportingPreScanInput>,
    /// Sole selected OutputsData pre-scan owner; not OutputProcessor registration.
    pub output_prescan: super::source_output_prescan::SourceOutputPreScan,
    /// Actual SimulationManager PreP_Fatal constructor member, initially false.
    pub simulation_preprocessor_fatal: bool,
    /// Actual parent guards, set before each member sequence.
    pub source_initialization_guards: super::source_initialization::SourceInitializationGuards,
    /// Real raw operands retained even if initialization stops.
    pub source_initialization_input:
        Option<super::source_initialization::SourceInitializationInput>,
    /// Sole pre-weather owner, moved into the real session after successful allocation.
    pub(crate) source_initialization_weather: Option<crate::weather::day::WeatherDayState>,
    /// Actual StdOutputRecordCount reset at OpenOutputFiles1747.
    pub source_standard_output_record_count: i64,
    /// Actual DataGlobals.hh116 Int64 constructor0; OpenOutputFiles does not reset it.
    pub source_standard_meter_record_count: i64,
    /// Sole actual partial IOFiles output-control owner, excluding existing SHD cell.
    pub(crate) source_output_control: super::ordinary_output_control::SourceOutputControl,
    /// Same ResultsFramework constructor/options and genuine report frames.
    pub(crate) source_results_framework:
        super::ordinary_output_control::SourceResultsFrameworkOptions,
    /// Sole ResultsFramework outputVariables metadata (no numerical snapshots).
    pub(crate) source_results_output_variables: Vec<String>,
    /// Sole actual nullable SQLite pointer; positive database construction unavailable.
    pub(crate) source_sqlite: super::source_initialization::SourceSqliteState,
    /// Sole actual DataReportingFlags subset shared by weather, solar and simulation.
    pub(crate) source_report_flags:
        super::radiation::ordinary::source_children::SourceSolarReportFlags,
    /// Sole genuine EconomicTariff subset, retaining first-call state across reports.
    pub(crate) economic_tariff: super::ordinary_utility_bills::SourceEconomicTariff,
    /// Sole actual CostEstimate monetary array and selected currency member.
    pub(crate) cost_estimate_monetary: super::ordinary_utility_bills::SourceCostEstimateMonetary,
    /// Sole genuine EconomicLifeCycleCostData input flags, counts and collections.
    pub(crate) source_life_cycle_cost: super::ordinary_life_cycle_cost::SourceEconomicLifeCycleCost,
    /// Same DataGlobal owner later moved into the first genuine HeatBalanceState.
    pub(crate) selected_heat_balance_globals:
        Option<super::selected_heat_balance_globals::SelectedHeatBalanceGlobals>,
    /// Sole startup/HVAC/RuntimeLanguage owner.
    pub(crate) selected_simulation_startup:
        Option<super::selected_simulation_startup::SelectedSimulationStartup>,
    /// Sole retained GetProjectData owner.
    pub(crate) selected_project_data: Option<super::selected_project_data::SelectedProjectData>,
    /// Sole Fluid constructor/constant/input owner.
    pub(crate) selected_fluid_properties:
        Option<super::selected_fluid_properties::SelectedFluidProperties>,
    /// Actual InternalHeatGains constructor and reached input collections.
    pub(crate) selected_internal_heat_gains: internal_heat_gains_input::SelectedInternalHeatGains,
    /// Sole real built-in and selected constant schedules.
    pub(crate) selected_schedule_manager:
        Option<super::selected_schedule_manager::SelectedScheduleManager>,
    /// Sole reached OutputProcessor time targets, extended by later registrations.
    pub(crate) output_registration: super::source_output_registration::SourceOutputRegistration,
    /// Actual222 electric constructor; later moved into the gain provider.
    pub(crate) facility_electric_service:
        Option<super::facility_electric_service::FacilityElectricService>,
    /// Actual223 coil report constructor storage.
    pub(crate) coil_selection_report: Option<super::run_input_prefix::SelectedCoilSelectionReport>,
    /// Actual227/228 BranchInputManager owner, retaining failed getter prefixes.
    pub(crate) selected_branches: super::run_input_prefix::SelectedBranches,
    /// Sole actual DataPluginManager trends vector, distinct from pluginManager ptr.
    pub(crate) source_plugin_data: super::ordinary_ems_plugin::SourcePluginData,
    /// Actual231 PluginManager constructor result, retained for the run.
    pub(crate) selected_plugin_manager: Option<super::run_input_prefix::SelectedPluginManager>,
    /// Same DataSizing/SizingManager owner through234 and later consumers.
    pub(crate) selected_sizing_manager: super::selected_sizing::SelectedSizingManager,
    /// Same three DataHeatBalance space-control fields before and after241.
    pub(crate) heat_balance_space_controls:
        super::selected_heat_balance_space_controls::SelectedHeatBalanceSpaceControls,
    /// Sole actual FaultsManager constructor, retaining getter counts and once flag.
    pub(crate) source_faults_manager: super::source_initialization::SourceFaultsManager,
    /// Sole selected CurveManager retained collections.
    pub(crate) selected_curves: Option<super::setup_postludes::SelectedCurves>,
    /// Raw HB getter operands from the same admitted source model.
    pub heat_balance_input: Option<super::selected_heat_balance_input::SelectedHeatBalanceInput>,
    /// Sole actual ProjectControl/Construction flags and argument state.
    pub(crate) heat_balance_input_state:
        super::selected_heat_balance_input::SelectedHeatBalanceInputState,
    pub(crate) heat_balance_input_errors_found: bool,
    /// Same actual Setup298 equipment owner, moved only on initializer success.
    pub(crate) source_zone_equipment: Option<super::ordinary_equipment::OrdinaryEquipment>,
    /// Actual partial GetSurfaceData input state, retained across every failure.
    pub(crate) surface_geometry_input:
        super::selected_heat_balance_input::surface::SurfaceGeometryInputState,
    /// Raw Material operands, retaining original loader format/order.
    pub selected_material_input: Option<super::selected_material_data::SelectedMaterialInput>,
    pub(crate) selected_material_data: super::selected_material_data::SelectedMaterialData,
    /// Same selected WindowManager constructor/once/report owner, retained on error.
    /// The real State assembly must move this owner, never reconstruct its flags.
    pub(crate) window_optical_initialization:
        super::ordinary_window_optical_initialization::SourceWindowOpticalInitialization,
    /// Rust failure custody after the Source-connected State has been moved.
    /// On a later outdoor/runtime error the actual State moves here without
    /// cloning its Zone/global/Solar owners; this is not an Original flag.
    pub(crate) failed_heat_balance_state: Option<Box<super::state::HeatBalanceState>>,
    /// Actual partial Zone/Space arrays, moved only after initializer success.
    pub(crate) selected_zone_input:
        Option<super::selected_heat_balance_input::zone::SelectedZoneInputOwners>,
    /// Sole reached GetShadowingInput result; later Solar must consume this owner.
    pub(crate) shadowing_input:
        Option<super::selected_heat_balance_input::shadowing::SourceShadowingInput>,
    /// Distinct HeatBalanceManager getter module carrier, not dataIPShortCut.
    pub(crate) heat_balance_current_module_object: String,
    /// Full allocated predefined string-entry owner.
    pub table: PredefinedStringTable,
    /// Genuine first-scan/cache state, not a requested result flag.
    pub scan: ReportScanState,
    /// Sole actual OutputReports module: Lines once flag and last option.
    pub(crate) source_output_reports: super::ctf_reports::SourceOutputReports,
    /// Sole SurfaceColorData constructor colors used by actual surface drawings.
    pub(crate) source_surface_colors:
        super::source_initialization::primary_startup_surfaces::SourceSurfaceColors,
    /// Reached ordinary postlude writes and return boundaries.
    pub progress: CtfInitPostludeProgress,
    /// Actual reached InitSolar prefix on failure; moved into State on success.
    pub ordinary_solar_radiation: Option<super::radiation::ordinary::OrdinarySolarRadiation>,
}

impl CtfInitializationReports {
    /// Move the actual admitted input owner and allocate its shared arrays.
    /// This is the selected InputProcessor.cc315-320 boundary, before runtime.
    /// Later definition queries retain their presence mutations without reallocating.
    ///
    /// # Errors
    /// Retains the moved input and previous arrays if schema/count lookup fails.
    pub fn initialize_input_processor(
        &mut self,
        input: super::selected_input_processor::SelectedInputProcessor,
    ) -> Result<(), super::selected_input_processor::SelectedInputUnavailable> {
        self.selected_input_processor = Some(input);
        let input = self
            .selected_input_processor
            .as_ref()
            .ok_or(super::selected_input_processor::SelectedInputUnavailable::Association)?;
        self.input_processor_shortcut.allocate_from_input(input)
    }

    /// Full genuine process_idf315-322 handoff on the same IP and DataOutputs.
    /// The existing prefix-only method remains unchanged for explicit legacy
    /// callers; this production entry also performs the actual statistics child.
    ///
    /// # Errors
    /// Retains the moved IP, all reached shared allocations and every actual
    /// counter reset/increment before a typed child/domain failure.
    pub fn initialize_source_input_processor(
        &mut self,
        input: super::selected_input_processor::SelectedInputProcessor,
    ) -> Result<(), super::selected_input_processor::SelectedInputUnavailable> {
        self.initialize_input_processor(input)?;
        self.selected_input_processor
            .as_ref()
            .ok_or(super::selected_input_processor::SelectedInputUnavailable::Association)?
            .report_source_idf_records_stats(&mut self.output_prescan)
    }
}

/// Distinct actual initialization failures, retaining the original backend errors.
#[derive(Debug)]
pub enum CtfInitializationError {
    /// A supplied registry or original loader-name owner is not representable.
    Owner(CtfReportUnavailable),
    /// The genuine selected chain stopped; no later construction callbacks run.
    SelectedChain(super::ctf_retry_initialization::CtfRetryChainStop),
    /// A real public suffix was unavailable; no later construction is invoked.
    PublicStop(super::ctf_public_initialization::CtfPublicInitStop),
    /// A reached loading/caller diagnostic stopped before later assignments.
    Diagnostic(CtfSourceDiagnosticStop),
    /// The reached report/Scan/final-fatal suffix stopped.
    Postlude(CtfInitPostludeError),
}

impl std::fmt::Display for CtfInitializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Owner(error) => {
                write!(f, "ordinary CTF input/report owner unavailable: {error:?}")
            }
            Self::SelectedChain(error) => {
                write!(f, "ordinary CTF selected chain stopped: {error:?}")
            }
            Self::PublicStop(error) => write!(f, "ordinary CTF public suffix stopped: {error:?}"),
            Self::Diagnostic(error) => std::fmt::Display::fmt(error, f),
            Self::Postlude(error) => std::fmt::Display::fmt(error, f),
        }
    }
}
impl std::error::Error for CtfInitializationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Owner(_) | Self::SelectedChain(_) | Self::PublicStop(_) => None,
            Self::Diagnostic(e) => Some(e),
            Self::Postlude(e) => Some(e),
        }
    }
}

// RuntimeError has an established PartialEq API. Compare failure identity and
// observable OS error facts without replacing or dropping the actual io::Error.
fn io_equal(a: &io::Error, b: &io::Error) -> bool {
    a.kind() == b.kind() && a.raw_os_error() == b.raw_os_error() && a.to_string() == b.to_string()
}
fn backend_equal(a: &SourceErrorBackendFailure, b: &SourceErrorBackendFailure) -> bool {
    match (a, b) {
        (
            SourceErrorBackendFailure::Effect {
                operation: a,
                source: x,
            },
            SourceErrorBackendFailure::Effect {
                operation: b,
                source: y,
            },
        ) => a == b && io_equal(x, y),
        (
            SourceErrorBackendFailure::UndefinedCounterIncrement { counter: a },
            SourceErrorBackendFailure::UndefinedCounterIncrement { counter: b },
        ) => a == b,
        _ => false,
    }
}
fn fatal_equal(a: &SourceFatalStop, b: &SourceFatalStop) -> bool {
    match (a, b) {
        (SourceFatalStop::Fatal(a), SourceFatalStop::Fatal(b)) => a == b,
        (SourceFatalStop::Backend(a), SourceFatalStop::Backend(b)) => backend_equal(a, b),
        _ => false,
    }
}
impl PartialEq for CtfInitializationError {
    fn eq(&self, other: &Self) -> bool {
        use CtfInitPostludeError as P;
        use CtfSourceDiagnosticStop as D;
        match (self, other) {
            (Self::Owner(a), Self::Owner(b)) => a == b,
            (Self::SelectedChain(a), Self::SelectedChain(b)) => a == b,
            (Self::PublicStop(a), Self::PublicStop(b)) => a == b,
            (Self::Diagnostic(D::Unavailable(a)), Self::Diagnostic(D::Unavailable(b))) => a == b,
            (Self::Diagnostic(D::Backend(a)), Self::Diagnostic(D::Backend(b))) => {
                backend_equal(a, b)
            }
            (Self::Diagnostic(D::Fatal(a)), Self::Diagnostic(D::Fatal(b))) => fatal_equal(a, b),
            (Self::Postlude(P::Unavailable(a)), Self::Postlude(P::Unavailable(b))) => a == b,
            (Self::Postlude(P::Scan(a)), Self::Postlude(P::Scan(b))) => a == b,
            (
                Self::Postlude(P::Report(CtfReportError::Unavailable(a))),
                Self::Postlude(P::Report(CtfReportError::Unavailable(b))),
            ) => a == b,
            (
                Self::Postlude(P::Report(CtfReportError::Io(a))),
                Self::Postlude(P::Report(CtfReportError::Io(b))),
            ) => io_equal(a, b),
            (Self::Postlude(P::Backend(a)), Self::Postlude(P::Backend(b))) => backend_equal(a, b),
            (Self::Postlude(P::Fatal(a)), Self::Postlude(P::Fatal(b))) => fatal_equal(a, b),
            _ => false,
        }
    }
}

/// Borrow actual caller-owned report outputs and a fully explicit backend once.
pub struct CtfInitializationContext<'a> {
    /// Retained full registry/table/Scan/progress owner.
    pub reports: &'a mut CtfInitializationReports,
    /// Actual EIO writer; errors preserve its already-written prefix.
    pub eio: &'a mut dyn Write,
    /// Actual state, sinks, optional callbacks and optional SQLite supplied by caller.
    pub backend: SourceErrorBackend<'a>,
}

impl CtfInitializationContext<'_> {
    /// Reach actual PostIPProcessing for the raw-admitted ordinary caller.
    ///
    /// # Errors
    /// Missing owners and reached Source failures retain all preceding state.
    pub fn post_input_processing(
        &mut self,
    ) -> Result<(), super::source_output_prescan::SourcePreScanError> {
        use super::source_output_prescan::{SourcePreScanError, post_input_processing};
        let input = self.reports.selected_input_processor.as_ref().ok_or(
            SourcePreScanError::Unavailable("missing sole admitted input"),
        )?;
        let raw = self.reports.reporting_prescan_input.as_ref().ok_or(
            SourcePreScanError::Unavailable("missing raw reporting pre-scan operands"),
        )?;
        post_input_processing(
            input,
            &mut self.reports.input_processor_shortcut,
            raw,
            &mut self.reports.output_prescan,
            self.reports.simulation_preprocessor_fatal,
            &mut self.backend,
        )
    }

    /// Run the first report-cache query reached by ordinary no-EMS init_state.
    /// A genuinely already-scanned cache is retained without replaying its query
    /// or diagnostics; it is not relabelled as a newly reached preparation.
    /// With the admitted input owner, real reached getter writes use its sole
    /// complete shortcut; legacy callers without that owner keep their API.
    pub fn prepare_report_input_cache(
        &mut self,
        input: &ep_model::ReportInputs,
    ) -> Result<(), CtfInitializationError> {
        use super::report_scan::{
            ReportScanDiagnostic, ReportScanFailure, SourceReportScanInput, try_scan_for_reports,
            try_scan_for_reports_with_input,
        };
        if !self.reports.scan.get_report_input {
            return Ok(());
        }
        let mut diagnostic = |diagnostic| match diagnostic {
            ReportScanDiagnostic::Warning(message) => self.backend.show_warning_error(&message),
            ReportScanDiagnostic::Continue(message) => self.backend.show_continue_error(&message),
        };
        let result = if let Some(actual_input) = self.reports.selected_input_processor.as_mut() {
            let mut source = SourceReportScanInput::new(
                actual_input,
                &mut self.reports.input_processor_shortcut,
                input,
            )
            .map_err(|value| CtfInitializationError::Postlude(CtfInitPostludeError::Scan(value)))?;
            try_scan_for_reports_with_input(
                input,
                &mut self.reports.scan,
                "EnergyManagementSystem",
                None,
                None,
                None,
                &mut source,
                &mut diagnostic,
            )
        } else {
            try_scan_for_reports(
                input,
                &mut self.reports.scan,
                "EnergyManagementSystem",
                None,
                None,
                None,
                &mut diagnostic,
            )
        };
        let requested = result.map_err(|error| {
            CtfInitializationError::Postlude(match error {
                ReportScanFailure::Unavailable(value) => CtfInitPostludeError::Scan(value),
                ReportScanFailure::Diagnostic(value) => CtfInitPostludeError::Backend(value),
            })
        })?;
        // CON's ordinary caller excludes EMS/API/external-HVAC producers. This
        // is the actual no-EMS requested-report warning suffix, not an EMS flag
        // inferred from the existence of Output:EnergyManagementSystem itself.
        if requested {
            self.backend.show_warning_error("CheckIFAnyEMS: No EnergyManagementSystem has been set up in the input file but output is requested.")
                .map_err(|error| CtfInitializationError::Postlude(CtfInitPostludeError::Backend(error)))?;
            self.backend.show_continue_error("No EDD file will be produced. Refer to EMS Application Guide and/or InputOutput Reference to set up your EnergyManagementSystem.")
                .map_err(|error| CtfInitializationError::Postlude(CtfInitPostludeError::Backend(error)))?;
        }
        Ok(())
    }

    /// Perform the real early SetPredefinedTables prefix for the selected weather host.
    /// The success-only retained flag prevents only this route's later duplicate.
    /// Legacy initialization keeps its established repeated registration behavior.
    ///
    /// # Errors
    /// Returns the actual registry/backend failure with the completed prefix retained.
    pub fn prepare_setup_weather_registry(
        &mut self,
        model: &TypedModel,
    ) -> Result<(), CtfInitializationError> {
        self.prepare_registry(model)?;
        self.reports.setup_simulation_registry_prepared = true;
        Ok(())
    }

    pub(crate) fn prepare_registry(
        &mut self,
        model: &TypedModel,
    ) -> Result<(), CtfInitializationError> {
        let (zone, system) = model.predefined_table_sizing.actual_flags();
        self.reports
            .registry
            .set_predefined_tables(zone, system)
            .map_err(CtfInitializationError::Owner)
    }

    pub(crate) fn loading_diagnostic(
        &mut self,
        model: &TypedModel,
        construction: &Construction,
        call: CtfLayerDiagnostic,
    ) -> Result<(), CtfInitializationError> {
        try_emit_ctf_loading_diagnostic(&mut self.backend, call, |ordinal| {
            let id = ordinal.checked_sub(1).and_then(|index| construction.effective_layers().get(index))
                .ok_or(super::ctf_source_diagnostics::CtfSourceDiagnosticUnavailable::OriginalLayerName(ordinal))?;
            original_material_name(model, *id).map_err(|_| super::ctf_source_diagnostics::CtfSourceDiagnosticUnavailable::OriginalLayerName(ordinal))
        }).map_err(CtfInitializationError::Diagnostic)
    }

    pub(crate) fn complete(
        &mut self,
        model: &TypedModel,
        public: &mut OrderedCtfPublicInitialization,
    ) -> Result<(), CtfInitializationError> {
        // This is the complete typed declaration set, not used-surface filtering.
        let any_source = model
            .constructions
            .iter()
            .any(Construction::has_internal_heat_source);
        complete_ctf_init_postlude(
            model,
            public,
            any_source,
            self.reports.registry.opaque_layer_columns(),
            &mut self.reports.table,
            &mut self.reports.scan,
            self.eio,
            &mut self.backend,
            &mut self.reports.progress,
        )
        .map_err(CtfInitializationError::Postlude)
    }
}

pub(crate) fn original_material_name(
    model: &TypedModel,
    id: MaterialId,
) -> Result<&str, CtfReportUnavailable> {
    let material = model
        .materials
        .iter()
        .find(|material| material.id == id)
        .ok_or(CtfReportUnavailable::Material(id))?;
    original_loader_name(&model.ctf_report_names.materials, id, &material.name)
}
pub(crate) fn original_loader_name<'a, I: Copy + Eq>(
    names: &'a [(I, String)],
    id: I,
    normalized: &NormalizedName,
) -> Result<&'a str, CtfReportUnavailable> {
    let mut found = names.iter().filter(|(actual, _)| *actual == id);
    let (_, name) = found
        .next()
        .ok_or(CtfReportUnavailable::LoaderNameUnavailable)?;
    if found.next().is_some() || NormalizedName::new(name) != *normalized {
        return Err(CtfReportUnavailable::Association);
    }
    Ok(name)
}

impl CtfInitializationContext<'_> {
    pub(crate) fn get_selected_project_control_data(
        &mut self,
        environment: &mut crate::weather::day::WeatherEnvironmentState,
    ) -> Result<(), crate::error::RuntimeError> {
        use super::selected_heat_balance_input::{
            HeatBalanceInputError as E, HeatBalanceInputOwners,
        };
        let reports = &mut *self.reports;
        let raw = reports
            .heat_balance_input
            .as_ref()
            .ok_or_else(|| hb_input_error(E::Unavailable("missing genuine HB raw operands")))?;
        let input = reports
            .selected_input_processor
            .as_mut()
            .ok_or_else(|| hb_input_error(E::Unavailable("missing sole actual IP")))?;
        reports.heat_balance_input_errors_found = false;
        let mut owners = HeatBalanceInputOwners {
            input,
            shortcut: &mut reports.input_processor_shortcut,
            project: reports
                .selected_project_data
                .as_mut()
                .ok_or_else(|| hb_input_error(E::Unavailable("missing sole project owner")))?,
            space_controls: &mut reports.heat_balance_space_controls,
            environment,
            current_module_object: &mut reports.heat_balance_current_module_object,
        };
        reports
            .heat_balance_input_state
            .get_project_control_data(
                raw,
                &mut owners,
                &mut self.backend,
                self.eio,
                &mut reports.heat_balance_input_errors_found,
            )
            .map_err(hb_input_error)
    }
    pub(crate) fn read_selected_construction_arguments(
        &mut self,
        model: &TypedModel,
    ) -> Result<(), crate::error::RuntimeError> {
        use super::selected_heat_balance_input::HeatBalanceInputError as E;
        let reports = &mut *self.reports;
        let raw = reports.heat_balance_input.as_ref().ok_or_else(|| {
            hb_input_error(E::Unavailable("missing genuine Construction raw operands"))
        })?;
        let input = reports
            .selected_input_processor
            .as_mut()
            .ok_or_else(|| hb_input_error(E::Unavailable("missing sole actual IP")))?;
        reports
            .heat_balance_input_state
            .read_regular_construction_arguments(
                raw,
                super::selected_heat_balance_input::ConstructionMaterialOwners {
                    model,
                    data: &reports.selected_material_data,
                    bounds: &reports
                        .selected_simulation_startup
                        .as_ref()
                        .ok_or_else(|| {
                            hb_input_error(E::Unavailable("actual preconstruction window bound"))
                        })?
                        .construction_bounds,
                },
                input,
                &mut reports.input_processor_shortcut,
                &mut reports.heat_balance_current_module_object,
                &mut self.backend,
                &mut reports.heat_balance_input_errors_found,
            )
            .map_err(hb_input_error)
    }
}
pub(crate) fn hb_input_error(
    error: super::selected_heat_balance_input::HeatBalanceInputError,
) -> crate::error::RuntimeError {
    use super::selected_heat_balance_input::HeatBalanceInputError as E;
    use crate::error::{RuntimeError, WarmupOutputError};
    let output = match error {
        E::Backend(e) => WarmupOutputError::Backend(e),
        E::Fatal(e) => return RuntimeError::HeatBalanceInput(Box::new(E::Fatal(e))),
        E::CtfInitialization(e) => {
            return RuntimeError::HeatBalanceInput(Box::new(E::CtfInitialization(e)));
        }
        E::ThermalHistory(e) => {
            return RuntimeError::HeatBalanceInput(Box::new(E::ThermalHistory(e)));
        }
        E::SurfaceInitialization(e) => {
            return RuntimeError::HeatBalanceInput(Box::new(E::SurfaceInitialization(e)));
        }
        E::PredefinedReport(e) => {
            return RuntimeError::HeatBalanceInput(Box::new(E::PredefinedReport(e)));
        }
        E::SourceInput(e) => return *e,
        E::Io(e) => WarmupOutputError::Io(e),
        E::Output(e) => return e.into(),
        E::Unavailable(e) => WarmupOutputError::Unavailable(format!("GetHeatBalanceInput: {e}")),
        E::Format(e) => WarmupOutputError::Format(e),
    };
    RuntimeError::WarmupOutput(Box::new(output))
}

impl CtfInitializationContext<'_> {
    pub(crate) fn get_selected_zone_data(
        &mut self,
        model: &TypedModel,
        minutes_in_time_step: i32,
        host: &mut dyn super::source_output_registration::SourceOutputFileHost,
    ) -> Result<(), crate::error::RuntimeError> {
        use super::selected_heat_balance_input::{
            HeatBalanceInputError as E,
            zone::{SelectedZoneInputOwners, ZoneGetterOwners},
        };
        let reports = &mut *self.reports;
        let raw = reports
            .heat_balance_input
            .as_ref()
            .ok_or_else(|| hb_input_error(E::Unavailable("missing genuine Zone raw operands")))?;
        let input = reports
            .selected_input_processor
            .as_mut()
            .ok_or_else(|| hb_input_error(E::Unavailable("missing sole Zone IP")))?;
        let globals = reports
            .selected_heat_balance_globals
            .as_mut()
            .ok_or_else(|| hb_input_error(E::Unavailable("missing sole early globals")))?;
        let zone = reports
            .selected_zone_input
            .get_or_insert_with(SelectedZoneInputOwners::default);
        let mut owners = ZoneGetterOwners {
            input,
            shortcut: &mut reports.input_processor_shortcut,
            globals,
            output: &mut reports.output_registration,
            prescan: &reports.output_prescan,
            report_input: &model.report_inputs,
            minimum_report_frequency: &reports
                .selected_project_data
                .as_ref()
                .ok_or_else(|| hb_input_error(E::Unavailable("missing sole system variables")))?
                .system
                .minimum_report_frequency,
            minutes_in_time_step,
            host,
            eio: self.eio,
        };
        zone.get_zone_data(raw, &reports.heat_balance_input_state, model, &mut owners)
            .map_err(hb_input_error)
    }
    pub(crate) fn mark_selected_surface_construction_used(
        &mut self,
        id: ep_model::ConstructionId,
    ) -> Result<(), crate::error::RuntimeError> {
        self.reports
            .heat_balance_input_state
            .mark_opaque_surface_construction_used(id)
            .map_err(hb_input_error)
    }
    pub(crate) fn mark_selected_surface_ctf_construction_used(
        &mut self,
        id: ep_model::ConstructionId,
    ) -> Result<(), crate::error::RuntimeError> {
        self.reports
            .heat_balance_input_state
            .mark_surface_ctf_construction_used(id)
            .map_err(hb_input_error)
    }
}

impl CtfInitializationContext<'_> {
    pub(crate) fn initialize_selected_heat_balance_input(
        &mut self,
        model: &TypedModel,
        environment: &mut crate::weather::day::WeatherEnvironmentState,
        minutes: i32,
        host: &mut dyn super::source_output_registration::SourceOutputFileHost,
    ) -> Result<(), crate::RuntimeError> {
        self.get_selected_project_control_data(environment)?;
        self.initialize_source_site_atmosphere(environment)?;
        self.get_selected_material_data(model)?;
        self.read_selected_construction_arguments(model)?;
        let mut owner = super::selected_heat_balance_input::shadowing::SourceShadowingInput::new();
        let result = owner.get_at_building_boundary(self);
        self.reports.shadowing_input = Some(owner);
        result?;
        self.get_selected_zone_data(model, minutes, host)
    }
}
