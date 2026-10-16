use super::*;
use crate::heat_balance::ctf_public_initialization::{
    ConstructionCtfPublicInitialization, CtfPublicConstructionReturn, CtfPublicReturnRoute,
};
use crate::heat_balance::error_backend::{SourceErrorState, SourceErrorTracking};
use crate::heat_balance::surface_manager::ctf_caller_stability::CtfCallerSharedFlags;
use crate::heat_balance::surface_manager::ctf_public_storage::CtfPublicCoefficientStorage;
use ep_model::{
    Construction, ConstructionId, ConstructionKind, Material, MaterialDefinition, MaterialId,
    MaterialSurfaceRoughness, NoMassMaterial, NormalizedName, OpaqueSurfaceProperties,
    ReportInputObject, ReportInputObjectType,
};
use std::io;

fn flags(errors_found: bool, do_ctf_error_report: bool) -> CtfCallerSharedFlags {
    CtfCallerSharedFlags {
        errors_found,
        do_ctf_error_report,
    }
}

// Explicit direct postlude input owners, not Native endpoints or production IDs.
fn fixture(do_report: bool) -> (TypedModel, OrderedCtfPublicInitialization) {
    let mut model = TypedModel::default();
    let material = Material {
        id: MaterialId(7),
        name: NormalizedName::new("DIRECT MATERIAL"),
        definition: MaterialDefinition::NoMass(NoMassMaterial {
            roughness: MaterialSurfaceRoughness::Smooth,
            thermal_resistance_m2_k_per_w: 0.25,
            surface: OpaqueSurfaceProperties::default(),
        }),
    };
    model
        .ctf_report_names
        .materials
        .push((material.id, "DIRECT MATERIAL".into()));
    model.materials.push(material);
    let mut public = OrderedCtfPublicInitialization::begin(true, 0);
    for (index, id) in [2, 9].into_iter().enumerate() {
        let construction = Construction {
            id: ConstructionId(id),
            name: NormalizedName(format!("DIRECT C{id}")),
            kind: ConstructionKind::Opaque,
            outside_layer: Some(MaterialId(7)),
            layers: vec![MaterialId(7)],
            thermochromic_master: None,
            ground_factor: None,
            air_boundary: None,
            complex_fenestration: None,
            window_equivalent_layer: None,
            internal_heat_source: None,
        };
        model
            .ctf_report_names
            .constructions
            .push((construction.id, construction.name.0.clone()));
        public
            .constructions
            .push(ConstructionCtfPublicInitialization {
                construction_id: construction.id,
                declared_index: index,
                original_material_ids: construction.layers.clone(),
                is_used_ctf: index == 1,
                public_initializer_invoked: true,
                retry_flags_on_entry: Some(flags(false, false)),
                retry_flags_on_return: Some(flags(false, false)),
                unused_reset_flags: None,
                result: Ok(CtfPublicConstructionReturn {
                    route: if index == 0 {
                        CtfPublicReturnRoute::UnusedReset
                    } else {
                        CtfPublicReturnRoute::AllResistive
                    },
                    storage: CtfPublicCoefficientStorage::reset(),
                }),
            });
        model.constructions.push(construction);
    }
    public.aggregate_after_returned_calls.returned_constructions = 2;
    public.finish_declared_loop_with_flags(2, flags(false, false));
    if do_report {
        model.report_inputs.objects.push(report_object(
            ReportInputObjectType::Constructions,
            "Constructions",
        ));
    }
    (model, public)
}

fn report_object(kind: ReportInputObjectType, first: &str) -> ReportInputObject {
    ReportInputObject {
        object_name: "direct authored input".into(),
        object_type: kind,
        alphas: [first.to_string(), String::new(), String::new()],
        alpha_blank: [false, true, true],
        default_applied: [false; 3],
        num_alphas: 1,
        is_epjson: true,
    }
}

fn error_state() -> SourceErrorState {
    SourceErrorState {
        output_error_header: true,
        version_string: "direct actual version operand".into(),
        idd_version_string: "direct actual IDD operand".into(),
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
            total_severe_errors: 3,
            total_severe_errors_during_warmup: 0,
            total_severe_errors_during_sizing: 0,
            last_severe_error: "incoming actual severe owner".into(),
            total_warning_errors: 0,
            total_warning_errors_during_warmup: 0,
            total_warning_errors_during_sizing: 0,
        },
    }
}

struct Outputs {
    state: SourceErrorState,
    error: Vec<u8>,
    console: Vec<u8>,
    eio: Vec<u8>,
    table: PredefinedStringTable,
    scan: ReportScanState,
    progress: CtfInitPostludeProgress,
}

impl Outputs {
    fn new() -> Self {
        Self {
            state: error_state(),
            error: Vec::new(),
            console: Vec::new(),
            eio: Vec::new(),
            table: PredefinedStringTable::default(),
            scan: ReportScanState::default(),
            progress: CtfInitPostludeProgress::default(),
        }
    }
    fn run(
        &mut self,
        model: &TypedModel,
        public: &mut OrderedCtfPublicInitialization,
        columns: &[i32],
        any_source: bool,
    ) -> Result<(), CtfInitPostludeError> {
        let mut backend = SourceErrorBackend {
            state: &mut self.state,
            error_stream: Some(&mut self.error),
            console: &mut self.console,
            out_unit1: None,
            out_unit2: None,
            f_message: None,
            message_callback: None,
            error_callback: None,
            sqlite: None,
        };
        complete_ctf_init_postlude(
            model,
            public,
            any_source,
            columns,
            &mut self.table,
            &mut self.scan,
            &mut self.eio,
            &mut backend,
            &mut self.progress,
        )
    }
}

#[test]
fn used_layers_precede_scan_and_eio_keeps_original_declared_counter()
-> Result<(), Box<dyn std::error::Error>> {
    let (model, mut public) = fixture(true);
    let mut actual = Outputs::new();
    actual.run(&model, &mut public, &[37], false)?;
    assert_eq!(actual.progress.layer_report_declared_indices, [1]);
    assert_eq!(actual.progress.construction_scan_result, Some(true));
    assert!(actual.progress.report_headers_written && actual.progress.postlude_returned);
    assert_eq!(actual.progress.coefficient_report_declared_indices, [1]);
    let entries = actual
        .table
        .entries
        .as_ref()
        .ok_or("missing actual table")?;
    assert_eq!(entries[0].index_column, 37);
    assert_eq!(entries[0].object_name, "DIRECT C9");
    assert_eq!(actual.table.num_table_entry, 1);
    let eio = String::from_utf8(actual.eio)?;
    assert!(
        eio.lines()
            .nth(4)
            .is_some_and(|line| line.starts_with(" Construction CTF,DIRECT C9,   2,"))
    );
    assert!(actual.error.is_empty());
    Ok(())
}

#[test]
fn completed_prefix_without_actual_final_flags_cannot_infer_last_row() {
    let (model, mut public) = fixture(true);
    public.final_shared_flags = None;
    let mut actual = Outputs::new();
    assert!(matches!(
        actual.run(&model, &mut public, &[37], false),
        Err(CtfInitPostludeError::Unavailable(
            CtfInitPostludeUnavailable::MissingFinalSharedFlags
        ))
    ));
    assert_eq!(actual.table.num_table_entry, 0);
    assert!(actual.scan.get_report_input && actual.eio.is_empty());
}

#[test]
fn actual_empty_ordinary_cache_retains_local_flags_with_no_last_row()
-> Result<(), Box<dyn std::error::Error>> {
    let model = TypedModel::default();
    let cache = crate::heat_balance::surface_manager::ConstructionThermalDataCache::build(
        &model,
        &std::collections::BTreeMap::new(),
    )?;
    let public = cache
        .public_initialization()
        .ok_or("missing actual public cache owner")?;
    assert!(public.constructions.is_empty() && public.selected_declared_loop_completed);
    assert_eq!(public.final_shared_flags, Some(flags(false, false)));
    Ok(())
}

#[test]
fn producer_final_flags_drive_report_then_fatal_even_if_last_row_flags_differ()
-> Result<(), Box<dyn std::error::Error>> {
    let (model, mut public) = fixture(false);
    public.finish_declared_loop_with_flags(2, flags(true, true));
    let mut actual = Outputs::new();
    assert!(matches!(
        actual.run(&model, &mut public, &[37], false),
        Err(CtfInitPostludeError::Fatal(SourceFatalStop::Fatal(_)))
    ));
    assert_eq!(actual.progress.construction_scan_result, Some(false));
    assert!(actual.progress.report_headers_written && actual.progress.final_fatal_invoked);
    assert!(!actual.progress.postlude_returned);
    assert_eq!(actual.progress.coefficient_report_declared_indices, [1]);
    assert!(String::from_utf8(actual.eio)?.contains(" Construction CTF,DIRECT C9,   2,"));
    let error = String::from_utf8(actual.error)?;
    assert!(error.contains("Reference severe error count=3"));
    assert!(error.contains("Last severe error=incoming actual severe owner"));
    assert_eq!(actual.state.tracking.total_severe_errors, 3);
    Ok(())
}

#[test]
fn incomplete_loop_never_reaches_aggregate_layer_scan_or_fatal() {
    let (model, mut public) = fixture(true);
    public.selected_declared_loop_completed = false;
    let mut actual = Outputs::new();
    assert!(matches!(
        actual.run(&model, &mut public, &[37], true),
        Err(CtfInitPostludeError::Unavailable(
            CtfInitPostludeUnavailable::IncompleteConstructionLoop
        ))
    ));
    assert!(public.aggregate_after_returned_calls.simple_ctf_only);
    assert_eq!(actual.progress, CtfInitPostludeProgress::default());
    assert!(actual.scan.get_report_input);
}

#[test]
fn actual_global_internal_source_owner_changes_aggregate_before_reports()
-> Result<(), Box<dyn std::error::Error>> {
    let (model, mut public) = fixture(false);
    let mut actual = Outputs::new();
    actual.run(&model, &mut public, &[37], true)?;
    assert!(!public.aggregate_after_returned_calls.simple_ctf_only);
    assert!(actual.eio.is_empty());
    assert_eq!(actual.progress.layer_report_declared_indices, [1]);
    assert!(actual.progress.postlude_returned);
    Ok(())
}

#[test]
fn later_layer_failure_keeps_prior_table_entries_and_never_scans() {
    let (mut model, mut public) = fixture(true);
    public.constructions[0].is_used_ctf = true;
    model
        .ctf_report_names
        .constructions
        .retain(|(id, _)| *id != ConstructionId(9));
    let mut actual = Outputs::new();
    assert!(matches!(
        actual.run(&model, &mut public, &[37], false),
        Err(CtfInitPostludeError::Report(_))
    ));
    assert_eq!(actual.table.num_table_entry, 1);
    assert_eq!(actual.progress.layer_report_declared_indices, [0]);
    assert!(actual.scan.get_report_input && actual.eio.is_empty());
}

struct FailWrite;
impl Write for FailWrite {
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
        Err(io::Error::other("direct sink failure"))
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn warning_backend_failure_retains_layers_and_stops_scan_before_continue() {
    let (mut model, mut public) = fixture(true);
    model.report_inputs.objects.insert(
        0,
        report_object(ReportInputObjectType::SurfacesList, "UNKNOWN"),
    );
    let mut actual = Outputs::new();
    let mut failed = FailWrite;
    let mut backend = SourceErrorBackend {
        state: &mut actual.state,
        error_stream: Some(&mut failed),
        console: &mut actual.console,
        out_unit1: None,
        out_unit2: None,
        f_message: None,
        message_callback: None,
        error_callback: None,
        sqlite: None,
    };
    let result = complete_ctf_init_postlude(
        &model,
        &mut public,
        false,
        &[37],
        &mut actual.table,
        &mut actual.scan,
        &mut actual.eio,
        &mut backend,
        &mut actual.progress,
    );
    assert!(matches!(result, Err(CtfInitPostludeError::Backend(_))));
    assert_eq!(actual.state.tracking.total_warning_errors, 1);
    assert_eq!(actual.table.num_table_entry, 1);
    assert!(actual.scan.get_report_input && !actual.scan.constructions);
    assert_eq!(actual.progress.construction_scan_result, None);
    assert!(!actual.progress.final_fatal_invoked && actual.eio.is_empty());
}

#[test]
fn eio_failure_after_scan_never_reaches_final_fatal() {
    let (model, mut public) = fixture(true);
    public.finish_declared_loop_with_flags(2, flags(true, true));
    let mut actual = Outputs::new();
    let mut failed = FailWrite;
    let mut backend = SourceErrorBackend {
        state: &mut actual.state,
        error_stream: Some(&mut actual.error),
        console: &mut actual.console,
        out_unit1: None,
        out_unit2: None,
        f_message: None,
        message_callback: None,
        error_callback: None,
        sqlite: None,
    };
    let result = complete_ctf_init_postlude(
        &model,
        &mut public,
        false,
        &[37],
        &mut actual.table,
        &mut actual.scan,
        &mut failed,
        &mut backend,
        &mut actual.progress,
    );
    assert!(matches!(
        result,
        Err(CtfInitPostludeError::Report(CtfReportError::Io(_)))
    ));
    assert_eq!(actual.progress.construction_scan_result, Some(true));
    assert!(!actual.progress.report_headers_written && !actual.progress.final_fatal_invoked);
    assert!(actual.error.is_empty());
}
