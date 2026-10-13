//! Actual caller-retained Init reports and sinks; no Native values are inputs.
use super::{fields, unit_fields};
use ep_model::{SimulationModel, TypedModel};
use ep_runtime::error::RuntimeError;
use ep_runtime::heat_balance::ctf_init_postlude::CtfInitPostludeError;
use ep_runtime::heat_balance::ctf_initialization_context::{
    CtfInitializationContext, CtfInitializationError, CtfInitializationReports,
};
use ep_runtime::heat_balance::ctf_source_diagnostics::CtfSourceDiagnosticStop;
use ep_runtime::heat_balance::error_backend::{
    SourceErrorBackend, SourceErrorState, SourceErrorTracking, SourceFatalStop,
};
use ep_runtime::heat_balance::{
    HeatBalanceState, initialize_heat_balance_state_with_initialization_context,
};
use serde_json::{Value, json};

// Literal declaration order, not registry values or expected initialized IDs.
include!("../../../ep_runtime/src/heat_balance/predefined_tables_names.rs");

pub(super) fn api_paths() -> Vec<&'static str> {
    let mut paths = fields::API_PATHS.to_vec();
    paths.extend([
        "crates/ep_model/src/report_inputs.rs",
        "crates/ep_compiler/src/compiler/report_inputs.rs",
        "crates/ep_runtime/src/error.rs",
        "crates/ep_runtime/src/heat_balance/ctf_initialization_context.rs",
        "crates/ep_runtime/src/heat_balance/ctf_geometry_reports.rs",
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
    ]);
    paths
}
pub(super) fn observer_paths() -> Vec<&'static str> {
    let mut paths = vec![
        "crates/ep_run/examples/ctf10_init_postlude_candidate_production_observer.rs",
        "crates/ep_run/examples/ctf10_init_postlude_support/fields.rs",
    ];
    paths.extend_from_slice(&fields::OBSERVER_PATHS[1..]);
    paths
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
fn reports(source: &CtfInitializationReports) -> Value {
    let registry = &source.registry;
    let table = &source.table;
    let scan = &source.scan;
    let progress = &source.progress;
    json!({"registry":{
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
        "table":{"entries":allocated(&table.entries,|v|json!({"char_entry":v.char_entry,
            "object_name":v.object_name,"index_column":v.index_column,"sub_table_index":v.sub_table_index,
            "unique_object_name":v.unique_object_name,"original_real_entry":unit_fields::scalar(v.original_real_entry),
            "significant_digits":v.significant_digits,"original_entry_is_real":v.original_entry_is_real})),
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
            "final_fatal_invoked":progress.final_fatal_invoked,"postlude_returned":progress.postlude_returned}})
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
        "f_message_present":false,"message_callback_present":false,"error_callback_present":false,"sqlite_present":false,
        "Native_header_strings_or_optional_backend_parity_claimed":false,
        "Native_registry_real_totals_component_or_shadow_owners_paired":false})
}

pub(super) struct Caller {
    pub(super) reports: CtfInitializationReports,
    state: SourceErrorState,
    error: Vec<u8>,
    console: Vec<u8>,
    eio: Vec<u8>,
}
impl Caller {
    pub(super) fn new(version: &str) -> Self {
        // Same authored Rust ordinary caller initialization as CtfRunOutputs::new.
        // These are not Native state/answers, and optional adapters remain absent.
        Self {
            reports: CtfInitializationReports::default(),
            state: SourceErrorState {
                output_error_header: true,
                version_string: concat!("rusted-energyplus ", env!("CARGO_PKG_VERSION")).into(),
                idd_version_string: format!("Rust typed input version {version}"),
                doing_input_processing: false,
                print_console_output: true,
                kickoff_simulation: false,
                developer_flag: false,
                warmup_flag: false,
                doing_sizing: false,
                tracking: SourceErrorTracking {
                    ask_for_surfaces_report: false,
                    total_coincident_vertices: 0,
                    total_degenerate_surfaces: 0,
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
        }
    }
    pub(super) fn snapshot(&self) -> Value {
        json!({"reports":reports(&self.reports),"backend":backend(&self.state),
            "prefixes":{"error":bytes(&self.error),"console":bytes(&self.console),"eio":bytes(&self.eio)},
            "actual_Rust_caller_owner_copied":true,"Native_intermediate_checkpoint_inferred":false})
    }
    fn context(&mut self) -> CtfInitializationContext<'_> {
        CtfInitializationContext {
            reports: &mut self.reports,
            eio: &mut self.eio,
            backend: SourceErrorBackend {
                state: &mut self.state,
                error_stream: Some(&mut self.error),
                console: &mut self.console,
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: None,
                error_callback: None,
                sqlite: None,
            },
        }
    }
    pub(super) fn prepare(&mut self, model: &TypedModel) -> Result<(), RuntimeError> {
        self.context()
            .prepare_report_input_cache(&model.report_inputs)?;
        Ok(())
    }
    pub(super) fn initialize(
        &mut self,
        model: &SimulationModel,
        initial: f64,
    ) -> Result<HeatBalanceState, RuntimeError> {
        initialize_heat_balance_state_with_initialization_context(
            model,
            initial,
            &mut self.context(),
        )
    }
}
pub(super) fn typed_report_inputs(model: &TypedModel) -> Value {
    json!({"report_inputs":{"objects":model.report_inputs.objects.iter().enumerate().map(|(index,v)|json!({
        "vector_index":index,"object_name":v.object_name,"object_type":v.object_type.source_name(),
        "alphas":v.alphas,"alpha_blank":v.alpha_blank,"default_applied":v.default_applied,
        "num_alphas":v.num_alphas,"is_epjson":v.is_epjson})).collect::<Vec<_>>(),
        "legacy_variable_dictionary_sections":model.report_inputs.legacy_variable_dictionary_sections},
        "predefined_table_sizing":model.predefined_table_sizing.first_object.as_ref().map(|v|json!({
            "object_name":v.object_name,"alphas":v.alphas,"alpha_blank":v.alpha_blank,"default_applied":v.default_applied})),
        "loader_material_names":model.ctf_report_names.materials.iter().map(|(id,name)|json!({"id":id.0,"name":name})).collect::<Vec<_>>(),
        "loader_construction_names":model.ctf_report_names.constructions.iter().map(|(id,name)|json!({"id":id.0,"name":name})).collect::<Vec<_>>()})
}
pub(super) fn public_owner(state: &HeatBalanceState) -> Value {
    match state.construction_ctf10_public_initialization.as_ref() {
        None => unavailable("returned State has no actual public owner"),
        Some(source) => json!({"owner_available":true,
            "constructions":source.constructions.iter().map(|row| {
                let mut value = unit_fields::public_construction(row);
                value["actual_unused_reset_flags"]=json!(row.unused_reset_flags.map(unit_fields::flags));
                value
            }).collect::<Vec<_>>(),
            "aggregate_after_returned_calls":{"SimpleCTFOnly":source.aggregate_after_returned_calls.simple_ctf_only,
                "MaxCTFTerms":source.aggregate_after_returned_calls.max_ctf_terms,
                "returned_constructions":source.aggregate_after_returned_calls.returned_constructions},
            "selected_stop":source.selected_stop.map(|value|format!("{value:?}")),
            "selected_declared_loop_completed":source.selected_declared_loop_completed,
            "actual_final_shared_flags":source.final_shared_flags.map(unit_fields::flags),
            "Native_whole_Init_return_inferred":false,"counted_as_PASS":false}),
    }
}
pub(super) fn unavailable(reason: &str) -> Value {
    json!({"owner_available":false,"snapshot":null,"reason":reason,"counted_as_PASS":false})
}
pub(super) fn error(error: &RuntimeError) -> Value {
    let source_fatal = matches!(error,RuntimeError::CtfInitialization(value) if matches!(value.as_ref(),
        CtfInitializationError::Diagnostic(CtfSourceDiagnosticStop::Fatal(SourceFatalStop::Fatal(_))) |
        CtfInitializationError::Postlude(CtfInitPostludeError::Fatal(SourceFatalStop::Fatal(_)))));
    let mut sources = Vec::new();
    let mut next = std::error::Error::source(error);
    while let Some(source) = next {
        sources.push(source.to_string());
        next = source.source();
    }
    json!({"status":if source_fatal {"actual-Source-fatal-returned"} else {"actual-Rust-error-returned"},
        "actual_type":format!("{error:?}"),"message":error.to_string(),"source_chain":sources,
        "actual_SourceFatalError_returned":source_fatal,"Native_process_outcome_inferred":false,"counted_as_PASS":false})
}
