//! Copies caller-retained direct Init owners, including every error prefix.
use super::compile_report_inputs::{DirectReportInputs, write_project_eio};
use ep_model::ReportInputs;
use ep_runtime::heat_balance::ctf_direct_initialization::{
    CtfDirectInitializationError, CtfDirectInitializationState, CtfDirectRegularConstructionInput,
    initialize_direct_regular_ctf,
};
use ep_runtime::heat_balance::ctf_init_postlude::CtfInitPostludeError;
use ep_runtime::heat_balance::ctf_initialization_context::{
    CtfInitializationContext, CtfInitializationReports,
};
use ep_runtime::heat_balance::ctf_reports::CtfReportUnavailable;
use ep_runtime::heat_balance::ctf_source_diagnostics::CtfSourceDiagnosticStop;
use ep_runtime::heat_balance::error_backend::SourceErrorBackendFailure;
use ep_runtime::heat_balance::error_backend::{
    SourceErrorBackend, SourceErrorLevel, SourceErrorState, SourceErrorTracking, SourceFatalStop,
};
use ep_runtime::heat_balance::report_scan::{
    ReportScanDiagnostic, ReportScanFailure, try_scan_for_reports,
};
use ep_runtime::heat_balance::surface_manager::ctf_layer_preprocessing::CtfLayerCheckpoint;
use serde_json::{Value, json};
use std::cell::RefCell;
include!("../../../ep_runtime/src/heat_balance/predefined_tables_names.rs");

pub(super) fn scalar(value: f64) -> Value {
    let class = if value.is_nan() {
        "nan"
    } else if value.is_infinite() {
        if value.is_sign_negative() {
            "negative_infinity"
        } else {
            "positive_infinity"
        }
    } else if value == 0.0 {
        if value.is_sign_negative() {
            "negative_zero"
        } else {
            "positive_zero"
        }
    } else {
        "finite"
    };
    json!({"value":if value.is_finite() { Some(value) } else { None },
        "value_bits":format!("{:016x}",value.to_bits()),"value_class":class})
}

pub(super) fn public_storage(
    source: &ep_runtime::heat_balance::surface_manager::ctf_public_storage::CtfPublicCoefficientStorage,
) -> Value {
    let values = |source: &[f64]| source.iter().copied().map(scalar).collect::<Vec<_>>();
    json!({"CTFOutside":values(&source.outside),"CTFCross":values(&source.cross),
        "CTFInside":values(&source.inside),"CTFFlux":values(&source.flux),
        "CTFSourceIn":values(&source.source_in),"CTFSourceOut":values(&source.source_out),
        "CTFTSourceOut":values(&source.temperature_source_out),"CTFTSourceIn":values(&source.temperature_source_in),
        "CTFTSourceQ":values(&source.temperature_source_q),"CTFTUserOut":values(&source.temperature_user_out),
        "CTFTUserIn":values(&source.temperature_user_in),"CTFTUserSource":values(&source.temperature_user_source),
        "CTFTimeStep":scalar(source.ctf_time_step),"UValue":scalar(source.u_value),
        "NumHistories":source.num_histories,"NumCTFTerms":source.num_ctf_terms,
        "fixed_array_count":12,"actual_slots_per_array":source.outside.len(),
        "coefficients_recomputed_by_observer":false,"counted_as_PASS":false})
}
fn allocated<T>(owner: &Option<Vec<T>>, mut copy: impl FnMut(&T) -> Value) -> Value {
    match owner {
        None => json!({"allocated":false,"length":null,"values":null}),
        Some(values) => json!({"allocated":true,"length":values.len(),
            "values":values.iter().enumerate().map(|(index,value)|json!({
                "vector_index":index,"source_one_based_index":index+1,"value":copy(value)
            })).collect::<Vec<_>>()}),
    }
}
fn bytes(owner: &[u8]) -> Value {
    json!({"size_bytes":owner.len(),"hex":owner.iter().map(|v|format!("{v:02x}")).collect::<String>(),
        "utf8":std::str::from_utf8(owner).ok()})
}
fn table_entries(
    owner: &Option<Vec<ep_runtime::heat_balance::ctf_reports::PredefinedTableEntry>>,
) -> Result<Value, std::str::Utf8Error> {
    match owner {
        None => Ok(json!({"allocated":false,"length":null,"values":null})),
        Some(values) => {
            let rows = values.iter().enumerate().map(|(index, v)| {
                let char_entry = std::str::from_utf8(&v.char_entry)?;
                Ok(json!({"vector_index":index,"source_one_based_index":index+1,
                    "value":json!({"char_entry":char_entry,
            "object_name":v.object_name,"index_column":v.index_column,"sub_table_index":v.sub_table_index,
            "unique_object_name":v.unique_object_name,"original_real_entry":scalar(v.original_real_entry),
            "significant_digits":v.significant_digits,"original_entry_is_real":v.original_entry_is_real})}))
            }).collect::<Result<Vec<_>, std::str::Utf8Error>>()?;
            Ok(json!({"allocated":true,"length":values.len(),"values":rows}))
        }
    }
}
fn reports(source: &CtfInitializationReports) -> Result<Value, std::str::Utf8Error> {
    let registry = &source.registry;
    let table = &source.table;
    let scan = &source.scan;
    let progress = &source.progress;
    Ok(json!({"registry":{
        "named_indices":registry.named_indices.iter().map(|(name,value)|json!({"name":name,"value":value})).collect::<Vec<_>>(),
        "declared_named_indices":NATIVE_INDEX_NAMES.iter().enumerate().map(|(index,name)|json!({
            "declaration_index":index,"name":name,"value":registry.named_indices.get(*name)
        })).collect::<Vec<_>>(),
        "named_index_owner_count":registry.named_indices.len(),
        "reports":allocated(&registry.reports,|v|json!({"name":v.name,"name_with_spaces":v.name_with_spaces,"abbreviation":v.abbreviation,"show":v.show})),
        "report_capacity":registry.report_capacity,"report_count":registry.report_count,
        "subtables":allocated(&registry.subtables,|v|json!({"name":v.name,"report_index":v.report_index,"footnote":v.footnote})),
        "subtable_capacity":registry.subtable_capacity,"subtable_count":registry.subtable_count,
        "columns":allocated(&registry.columns,|v|json!({"heading":v.heading,"subtable_index":v.subtable_index})),
        "column_capacity":registry.column_capacity,"column_count":registry.column_count,
        "opaque_layer_columns":registry.opaque_layer_columns,
        "unrelated_Native_aggregate_owners_available":false},
        "table":{"entries":table_entries(&table.entries)?,
            "size_table_entry":table.size_table_entry,"num_table_entry":table.num_table_entry},
        "scan":{"get_report_input":scan.get_report_input,"surface_vertices":scan.surface_vertices,
            "surface_details":scan.surface_details,"surface_details_with_vertices":scan.surface_details_with_vertices,
            "lines":scan.lines,"line_option":scan.line_option,"cost_info":scan.cost_info,
            "view_factor_info":scan.view_factor_info,"view_factor_option":scan.view_factor_option,
            "dxf":scan.dxf,"dxf_options":scan.dxf_options,"dxf_wireframe":scan.dxf_wireframe,
            "dxf_wireframe_options":scan.dxf_wireframe_options,"vrml":scan.vrml,"vrml_options":scan.vrml_options,
            "variable_dictionary":scan.variable_dictionary,"variable_dictionary_options":scan.variable_dictionary_options,
            "constructions":scan.constructions,"materials":scan.materials,"ems_output":scan.ems_output,
            "show_decay_curves_in_eio":scan.show_decay_curves_in_eio,
            "ems_actuator_availability":scan.ems_actuator_availability,
            "ems_internal_variable_availability":scan.ems_internal_variable_availability,
            "ems_errors":scan.ems_errors,"ems_full_trace":scan.ems_full_trace},
        "progress":{"layer_report_declared_indices":progress.layer_report_declared_indices,
            "construction_scan_result":progress.construction_scan_result,
            "report_headers_written":progress.report_headers_written,
            "coefficient_report_declared_indices":progress.coefficient_report_declared_indices,
            "final_fatal_invoked":progress.final_fatal_invoked,"postlude_returned":progress.postlude_returned}}))
}
fn backend(source: &SourceErrorState) -> Value {
    let t = &source.tracking;
    json!({"output_error_header":source.output_error_header,"version_string":source.version_string,
        "idd_version_string":source.idd_version_string,"doing_input_processing":source.doing_input_processing,
        "print_console_output":source.print_console_output,"kickoff_simulation":source.kickoff_simulation,
        "developer_flag":source.developer_flag,"warmup_flag":source.warmup_flag,"doing_sizing":source.doing_sizing,
        "tracking":{"match_counts":t.match_counts,"abort_processing":t.abort_processing,
            "total_severe_errors":t.total_severe_errors,"total_severe_errors_during_warmup":t.total_severe_errors_during_warmup,
            "total_severe_errors_during_sizing":t.total_severe_errors_during_sizing,"last_severe_error":t.last_severe_error,
            "total_warning_errors":t.total_warning_errors,"total_warning_errors_during_warmup":t.total_warning_errors_during_warmup,
            "total_warning_errors_during_sizing":t.total_warning_errors_during_sizing},
        "error_stream_present":true,"console_present":true,"out_unit1_present":false,"out_unit2_present":false,
        "f_message_present":false,"message_callback_present":true,"error_callback_present":true,"sqlite_present":false,
        "Native_header_strings_or_optional_backend_parity_claimed":false,
        "Native_registry_real_totals_component_or_shadow_owners_paired":false})
}

#[derive(Debug)]
pub(super) enum SetupError {
    Registry(CtfReportUnavailable),
    ProjectIo(std::io::Error),
    Scan(ReportScanFailure<SourceErrorBackendFailure>),
    UnknownAction,
}
impl std::fmt::Display for SetupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Registry(error) => write!(f, "actual registry unavailable: {error:?}"),
            Self::Scan(error) => write!(f, "actual Scan stopped: {error:?}"),
            Self::ProjectIo(error) => std::fmt::Display::fmt(error, f),
            Self::UnknownAction => f.write_str("undeclared setup action"),
        }
    }
}
impl std::error::Error for SetupError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Scan(ReportScanFailure::Diagnostic(error)) => Some(error),
            Self::ProjectIo(error) => Some(error),
            _ => None,
        }
    }
}
pub(super) fn setup_error(error: &SetupError) -> Value {
    let mut sources = Vec::new();
    let mut next = std::error::Error::source(error);
    while let Some(source) = next {
        sources.push(source.to_string());
        next = source.source();
    }
    json!({"actual_type":format!("{error:?}"),"message":error.to_string(),"source_chain":sources,"Native_outcome_inferred":false})
}
pub(super) struct Caller {
    reports: CtfInitializationReports,
    state: SourceErrorState,
    error: Vec<u8>,
    console: Vec<u8>,
    eio: Vec<u8>,
    callbacks: Vec<Value>,
}
impl Caller {
    pub(super) fn new() -> Self {
        // Fresh direct EnergyPlusData string owners are empty. The helper does
        // not call EnergyPlusPgm, which later constructs the CLI version string.
        Self {
            reports: CtfInitializationReports::default(),
            state: SourceErrorState {
                output_error_header: true,
                version_string: String::new(),
                idd_version_string: String::new(),
                doing_input_processing: false,
                print_console_output: true,
                kickoff_simulation: false,
                developer_flag: false,
                warmup_flag: false,
                doing_sizing: false,
                tracking: SourceErrorTracking {
                    ask_for_plant_check_on_abort: false,
                    exit_during_simulations: false,
                    ask_for_connections_report: false,
                    ask_for_surfaces_report: false,
                    total_multiplied_windows: 0,
                    total_coincident_vertices: 0,
                    total_degenerate_surfaces: 0,
                    total_receiving_non_convex_surfaces: 0,
                    total_casting_non_convex_surfaces: 0,
                    match_counts: [0; 20],
                    abort_processing: false,
                    total_severe_errors: 0,
                    last_severe_error: String::new(),
                    total_severe_errors_during_warmup: 0,
                    total_severe_errors_during_sizing: 0,
                    total_warning_errors: 0,
                    total_warning_errors_during_warmup: 0,
                    total_warning_errors_during_sizing: 0,
                },
            },
            error: Vec::new(),
            console: Vec::new(),
            eio: Vec::new(),
            callbacks: Vec::new(),
        }
    }
    fn with_context<T>(
        &mut self,
        action: impl FnOnce(&mut CtfInitializationContext<'_>) -> T,
    ) -> T {
        let callbacks = RefCell::new(&mut self.callbacks);
        let mut message = |_: &mut SourceErrorState, text: &str| {
            let mut rows = callbacks.borrow_mut();
            let ordinal = rows.len() + 1;
            rows.push(json!({"actual_ordinal":ordinal,"kind":"messageCallback","actual_level":null,"actual_message":text}));
        };
        let mut error = |_: &mut SourceErrorState, level: SourceErrorLevel, text: &str| {
            let mut rows = callbacks.borrow_mut();
            let ordinal = rows.len() + 1;
            rows.push(json!({"actual_ordinal":ordinal,"kind":"errorCallback","actual_level":level as i32,"actual_message":text}));
        };
        let mut context = CtfInitializationContext {
            reports: &mut self.reports,
            eio: &mut self.eio,
            backend: SourceErrorBackend {
                state: &mut self.state,
                error_stream: Some(&mut self.error),
                console: &mut self.console,
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: Some(&mut message),
                error_callback: Some(&mut error),
                sqlite: None,
            },
        };
        action(&mut context)
    }
    pub(super) fn snapshot(&self) -> Result<Value, std::str::Utf8Error> {
        Ok(json!({"reports":reports(&self.reports)?,"backend":backend(&self.state),
            "prefixes":{"error":bytes(&self.error),"console":bytes(&self.console),"eio":bytes(&self.eio)},
            "backend_callbacks":self.callbacks,"actual_backend_callback_count":self.callbacks.len(),
            "actual_Rust_caller_owner_copied":true,"Native_intermediate_checkpoint_inferred":false}))
    }
    pub(super) fn prepare(&mut self, input: &DirectReportInputs) -> Result<bool, SetupError> {
        self.with_context(|context| {
            write_project_eio(input, &context.backend.state.version_string, context.eio)
                .map_err(SetupError::ProjectIo)?;
            // EnergyPlusData::init_state invokes this after GetProjectData and
            // before the direct caller's first SetPredefinedTables observation.
            try_scan_for_reports(
                &input.reports,
                &mut context.reports.scan,
                "EnergyManagementSystem",
                None,
                None,
                None,
                |call| match call {
                    ReportScanDiagnostic::Warning(message) => {
                        context.backend.show_warning_error(&message)
                    }
                    ReportScanDiagnostic::Continue(message) => {
                        context.backend.show_continue_error(&message)
                    }
                },
            )
            .map_err(SetupError::Scan)
        })
    }
    pub(super) fn setup(
        &mut self,
        action: &str,
        inputs: &ReportInputs,
    ) -> Result<Option<bool>, SetupError> {
        self.with_context(|context| match action {
            "SetPredefinedTables" => {
                context
                    .reports
                    .registry
                    .set_predefined_tables(false, false)
                    .map_err(SetupError::Registry)?;
                Ok(None)
            }
            "ScanForReports(Constructions)" => {
                let result = try_scan_for_reports(
                    inputs,
                    &mut context.reports.scan,
                    "Constructions",
                    Some("Constructions"),
                    None,
                    None,
                    |call| match call {
                        ReportScanDiagnostic::Warning(message) => {
                            context.backend.show_warning_error(&message)
                        }
                        ReportScanDiagnostic::Continue(message) => {
                            context.backend.show_continue_error(&message)
                        }
                    },
                )
                .map_err(SetupError::Scan)?;
                Ok(Some(result))
            }
            _ => Err(SetupError::UnknownAction),
        })
    }
    pub(super) fn initialize(
        &mut self,
        input: &CtfDirectRegularConstructionInput<'_>,
        reports: &ReportInputs,
        state: &mut CtfDirectInitializationState,
    ) -> Result<(), CtfDirectInitializationError> {
        self.with_context(|context| initialize_direct_regular_ctf(input, reports, state, context))
    }
}
fn checkpoint(source: Option<&CtfLayerCheckpoint>) -> Value {
    source.map_or(Value::Null,|v|json!({"LayersInConstruct":v.layers.len(),"NumResLayers":v.num_res_layers,
        "active_prefix":v.layers.iter().map(|l|json!({"dl":scalar(l.dl),"rk":scalar(l.rk),"rho":scalar(l.rho),
            "cp":scalar(l.cp),"lr":scalar(l.lr),"ResLayer":l.res_layer})).collect::<Vec<_>>()}))
}
pub(super) fn direct_state(source: &CtfDirectInitializationState) -> Value {
    let preprocessing=source.preprocessing.as_ref().map(|v|json!({"skipped_unused":v.skipped_unused,"errors_found":v.errors_found,
        "issues_in_actual_order":format!("{:?}",v.issues),"after_load":checkpoint(v.after_load.as_ref()),
        "after_merge":checkpoint(v.after_merge.as_ref()),"after_conversion":v.after_conversion.as_ref().map(|c|json!({
            "active":checkpoint(Some(&c.active)),"dyn":scalar(c.dyn_spacing),"rs":scalar(c.total_resistance),"cnd":scalar(c.conductance)}))}));
    let values = |v: &[f64]| v.iter().copied().map(scalar).collect::<Vec<_>>();
    json!({"public_storage":public_storage(&source.storage),"retained_preprocessing":preprocessing,
        "all_resistive":source.all_resistive.as_ref().map(|v|json!({"CTFOutside":values(&v.outside),"CTFCross":values(&v.cross),
            "CTFInside":values(&v.inside),"CTFFlux":values(&v.flux),"CTFTimeStep":scalar(v.ctf_time_step),"UValue":scalar(v.u_value),
            "NumHistories":v.num_histories,"NumCTFTerms":v.num_ctf_terms})),
        "actual_shared_flags":{"ErrorsFound":source.shared_flags.errors_found,"DoCTFErrorReport":source.shared_flags.do_ctf_error_report},
        "aggregate_after_actual_returned_calls":{"SimpleCTFOnly":source.aggregate.simple_ctf_only,"MaxCTFTerms":source.aggregate.max_ctf_terms,
            "returned_constructions":source.aggregate.returned_constructions},
        "construction_invoked":source.construction_invoked,"construction_return_route":source.construction_return_route.map(|v|format!("{v:?}")),
        "init_returned":source.init_returned,"owner_retained_on_Err":true,"Native_return_inferred":false,"counted_as_PASS":false})
}
pub(super) fn error(error: &CtfDirectInitializationError) -> Value {
    let fatal = matches!(
        error,
        CtfDirectInitializationError::Diagnostic(CtfSourceDiagnosticStop::Fatal(
            SourceFatalStop::Fatal(_)
        )) | CtfDirectInitializationError::Postlude(CtfInitPostludeError::Fatal(
            SourceFatalStop::Fatal(_)
        ))
    );
    let mut sources = Vec::new();
    let mut next = std::error::Error::source(error);
    while let Some(source) = next {
        sources.push(source.to_string());
        next = source.source();
    }
    json!({"status":if fatal {"actual-Source-fatal-returned"} else {"actual-Rust-error-returned"},"actual_type":format!("{error:?}"),
        "message":error.to_string(),"source_chain":sources,"actual_SourceFatalError_returned":fatal,"Native_process_outcome_inferred":false,"counted_as_PASS":false})
}
pub(super) fn typed_report_inputs(inputs: &ReportInputs) -> Value {
    json!({"objects":inputs.objects.iter().enumerate().map(|(i,v)|json!({"vector_index":i,"object_name":v.object_name,
        "object_type":v.object_type.source_name(),"alphas":v.alphas,"alpha_blank":v.alpha_blank,"default_applied":v.default_applied,
        "num_alphas":v.num_alphas,"is_epjson":v.is_epjson})).collect::<Vec<_>>(),"legacy_variable_dictionary_sections":inputs.legacy_variable_dictionary_sections})
}

pub(super) fn observer_paths() -> Vec<&'static str> {
    vec![
        "crates/ep_run/examples/ctf10_init_postlude_candidate_direct_observer.rs",
        "crates/ep_run/examples/ctf10_init_postlude_direct_support/fields.rs",
        "crates/ep_run/examples/ctf10_init_postlude_direct_support/compile_report_inputs.rs",
    ]
}
pub(super) fn api_paths() -> Vec<&'static str> {
    vec![
        "crates/ep_run/Cargo.toml",
        "crates/ep_raw_model/src/lib.rs",
        "crates/ep_raw_model/src/idf_order.rs",
        "crates/ep_compiler/src/lib.rs",
        "crates/ep_compiler/src/compiler.rs",
        "crates/ep_model/src/lib.rs",
        "crates/ep_model/src/model.rs",
        "crates/ep_model/src/objects/materials.rs",
        "crates/ep_model/src/objects/construction.rs",
        "crates/ep_runtime/src/lib.rs",
        "crates/ep_runtime/src/heat_balance/mod.rs",
        "crates/ep_runtime/src/heat_balance/initialization.rs",
        "crates/ep_runtime/src/heat_balance/initialization/schedule_cache.rs",
        "crates/ep_runtime/src/heat_balance/initialization/state_shell.rs",
        "crates/ep_runtime/src/heat_balance/surface_manager.rs",
        "crates/ep_runtime/src/heat_balance/surface_manager/construction_cache.rs",
        "crates/ep_runtime/src/heat_balance/state.rs",
        "crates/ep_runtime/src/heat_balance/ctf.rs",
        "crates/ep_runtime/examples/clk02_probe_support/digest.rs",
        "crates/ep_runtime/src/heat_balance/surface_manager/ctf_layer_preprocessing.rs",
        "crates/ep_model/src/objects/building.rs",
        "crates/ep_runtime/src/heat_balance/surface_manager/ctf_all_resistive.rs",
        "crates/ep_runtime/src/heat_balance/surface_manager/ctf_initial_discretization.rs",
        "crates/ep_runtime/src/heat_balance/ctf_initial_owner.rs",
        "crates/ep_compiler/src/compiler/surface_space.rs",
        "crates/ep_runtime/src/heat_balance/surface_manager/ctf_state_space_assembly.rs",
        "crates/ep_runtime/src/heat_balance/ctf_first_assembly_owner.rs",
        "crates/ep_runtime/src/heat_balance/surface_manager/ctf_exponential_matrix.rs",
        "crates/ep_runtime/src/heat_balance/ctf_first_exponential_owner.rs",
        "crates/ep_runtime/src/heat_balance/surface_manager/ctf_inverse_matrix.rs",
        "crates/ep_runtime/src/heat_balance/ctf_first_inverse_owner.rs",
        "crates/ep_runtime/src/heat_balance/ctf_first_matrix_owner.rs",
        "crates/ep_runtime/src/heat_balance/surface_manager/ctf_gamma_matrix.rs",
        "crates/ep_runtime/src/heat_balance/ctf_first_gamma_owner.rs",
        "crates/ep_runtime/src/heat_balance/ctf_first_gamma_ingress.rs",
        "crates/ep_runtime/src/heat_balance/surface_manager/ctf_final_coefficients.rs",
        "crates/ep_runtime/src/heat_balance/ctf_first_final_coefficients_owner.rs",
        "crates/ep_runtime/src/heat_balance/ctf_first_final_coefficients_ingress.rs",
        "crates/ep_runtime/src/heat_balance/surface_manager/ctf_caller_stability.rs",
        "crates/ep_runtime/src/heat_balance/ctf_retry_driver.rs",
        "crates/ep_runtime/src/heat_balance/ctf_retry_initialization.rs",
        "crates/ep_runtime/src/heat_balance/ctf_public_initialization.rs",
        "crates/ep_runtime/src/heat_balance/surface_manager/ctf_public_storage.rs",
        "crates/ep_model/src/report_inputs.rs",
        "crates/ep_compiler/src/compiler/report_inputs.rs",
        "crates/ep_runtime/src/error.rs",
        "crates/ep_runtime/src/heat_balance/ctf_initialization_context.rs",
        "crates/ep_runtime/src/heat_balance/ctf_init_postlude.rs",
        "crates/ep_runtime/src/heat_balance/ctf_reports.rs",
        "crates/ep_runtime/src/heat_balance/ctf_reports_format.rs",
        "crates/ep_runtime/src/heat_balance/predefined_tables.rs",
        "crates/ep_runtime/src/heat_balance/predefined_tables_names.rs",
        "crates/ep_runtime/src/heat_balance/predefined_tables_catalog.rs",
        "crates/ep_runtime/src/heat_balance/report_scan.rs",
        "crates/ep_runtime/src/heat_balance/error_backend.rs",
        "crates/ep_runtime/src/heat_balance/ctf_source_diagnostics.rs",
        "crates/ep_runtime/src/heat_balance/ctf_surface_lifecycle.rs",
        "crates/ep_run/src/ctf_initialization_outputs.rs",
        "crates/ep_runtime/src/heat_balance/ctf_direct_initialization.rs",
    ]
}

#[cfg(test)]
mod preparation_tests {
    use super::*;
    use ep_raw_model::parse_epjson_str_with_idf_order;

    #[test]
    fn genuine_preparation_reads_report_cache_before_registry_and_retains_prefix()
    -> Result<(), Box<dyn std::error::Error>> {
        let raw = parse_epjson_str_with_idf_order(
            r#"{"Version":{"Version 1":{"version_identifier":"26.1"}},"Timestep":{"Timestep 1":{"number_of_timesteps_per_hour":4}},"Output:Constructions":{"Output:Constructions 1":{"details_type_1":"Constructions","details_type_2":"Materials"}}}"#,
            "Version,26.1;Timestep,4;Output:Constructions,Constructions,Materials;",
        )?;
        let (input, _) = super::super::compile_report_inputs::compile_direct_report_inputs(&raw);
        let input = input.ok_or("genuine direct compilation missing")?;
        super::super::compile_report_inputs::admit_project_defaults(&raw, &input)?;
        let mut caller = Caller::new();
        assert!(caller.reports.scan.get_report_input);
        assert!(!caller.prepare(&input)?);
        assert!(!caller.reports.scan.get_report_input);
        assert!(caller.reports.scan.constructions && caller.reports.scan.materials);
        assert_eq!(
            caller.reports.registry,
            ep_runtime::heat_balance::predefined_tables::PredefinedRegistry::default()
        );
        assert!(caller.callbacks.is_empty());
        let prefix = caller.eio.clone();
        caller.setup("SetPredefinedTables", &input.reports)?;
        assert_eq!(caller.eio, prefix);
        assert!(!caller.reports.scan.get_report_input);
        assert!(caller.reports.scan.constructions && caller.reports.scan.materials);
        assert!(prefix.starts_with(b"Program Version,\n"));
        Ok(())
    }
}
