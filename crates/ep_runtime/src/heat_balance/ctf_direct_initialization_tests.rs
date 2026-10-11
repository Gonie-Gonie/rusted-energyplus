use super::*;
use crate::heat_balance::ctf_initialization_context::CtfInitializationReports;
use crate::heat_balance::error_backend::{
    SourceErrorBackend, SourceErrorState, SourceErrorTracking, SourceFatalStop,
};
use crate::heat_balance::surface_manager::ctf_layer_preprocessing::{
    CtfLayerIssue, try_preprocess_ctf_layers,
};
use ep_model::{MaterialId, ReportInputObject, ReportInputObjectType};
use std::io::{self, Write};

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn input() -> CtfDirectRegularConstructionInput<'static> {
    // Input-only constructor members used by the declared direct Regular material.
    // ROnly remains independently true, without constructing a NoMass model.
    CtfDirectRegularConstructionInput {
        construction_id: ConstructionId(1),
        construction_name: "UNIT CONSTRUCTION",
        material_name: "UNIT MATERIAL",
        layer: CtfLayerInput {
            material_id: MaterialId(1),
            thickness: 0.0,
            conductivity: 0.0,
            density: 0.0,
            specific_heat: 0.0,
            resistance: 0.5,
            resistance_only: true,
        },
        is_used_ctf: true,
        source_sink_present: false,
        solution_dimensions: 1,
        absorptance: [0.0; 4],
        outside_roughness: MaterialSurfaceRoughness::MediumRough,
        time_step_zone: 0.0,
        any_internal_heat_source_in_input: false,
    }
}

fn report_inputs() -> ReportInputs {
    // Authored public-API unit operand only. Actual direct6 transport must use
    // the genuine raw input parser/compiler helper, never this test constructor.
    ReportInputs {
        objects: vec![ReportInputObject {
            object_name: "authored report API operand".into(),
            object_type: ReportInputObjectType::Constructions,
            alphas: ["CONSTRUCTIONS".into(), "MATERIALS".into(), String::new()],
            alpha_blank: [false, false, true],
            default_applied: [false; 3],
            num_alphas: 2,
            is_epjson: true,
        }],
        legacy_variable_dictionary_sections: -1,
    }
}

fn error_state() -> SourceErrorState {
    SourceErrorState {
        output_error_header: true,
        version_string: "authored version operand".into(),
        idd_version_string: "authored IDD operand".into(),
        doing_input_processing: false,
        print_console_output: true,
        kickoff_simulation: false,
        developer_flag: false,
        warmup_flag: false,
        doing_sizing: false,
        tracking: SourceErrorTracking {
            match_counts: [0; 20],
            abort_processing: false,
            total_severe_errors: 0,
            total_severe_errors_during_warmup: 0,
            total_severe_errors_during_sizing: 0,
            last_severe_error: String::new(),
            total_warning_errors: 0,
            total_warning_errors_during_warmup: 0,
            total_warning_errors_during_sizing: 0,
        },
    }
}

struct Outputs {
    reports: CtfInitializationReports,
    errors: SourceErrorState,
    error: Vec<u8>,
    console: Vec<u8>,
}
impl Outputs {
    fn new() -> Result<Self, CtfReportUnavailable> {
        let mut reports = CtfInitializationReports::default();
        reports.registry.set_predefined_tables(false, false)?;
        Ok(Self {
            reports,
            errors: error_state(),
            error: Vec::new(),
            console: Vec::new(),
        })
    }
    fn run(
        &mut self,
        input: &CtfDirectRegularConstructionInput<'_>,
        reports: &ReportInputs,
        state: &mut CtfDirectInitializationState,
        eio: &mut dyn Write,
    ) -> Result<(), CtfDirectInitializationError> {
        let mut context = CtfInitializationContext {
            reports: &mut self.reports,
            eio,
            backend: SourceErrorBackend {
                state: &mut self.errors,
                error_stream: Some(&mut self.error),
                console: &mut self.console,
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: None,
                error_callback: None,
                sqlite: None,
            },
        };
        initialize_direct_regular_ctf(input, reports, state, &mut context)
    }
}

#[test]
fn direct_regular_ronly_uses_actual_core_and_caller_zero_timestep() -> TestResult {
    let input = input();
    let mut outputs = Outputs::new().map_err(|value| format!("registry: {value:?}"))?;
    let mut state = CtfDirectInitializationState::new(true, 0);
    let mut eio = Vec::new();
    outputs.run(&input, &report_inputs(), &mut state, &mut eio)?;
    let actual = state
        .preprocessing
        .as_ref()
        .ok_or("actual preprocessing missing")?;
    let reference = try_preprocess_ctf_layers(
        &[input.layer],
        CtfLayerContext {
            is_used_ctf: true,
            errors_found: false,
            source_sink_present: false,
            solution_dimensions: 1,
        },
        |_| Ok::<(), std::convert::Infallible>(()),
    )
    .map_err(|value| format!("core: {value:?}"))?;
    assert_eq!(actual, &reference);
    let generated = state
        .all_resistive
        .as_ref()
        .ok_or("actual all-resistive missing")?;
    assert_eq!(
        state.storage,
        copy_all_resistive_public_coefficients(generated)
            .map_err(|value| format!("copy: {value:?}"))?
    );
    assert_eq!(
        state.storage.ctf_time_step.to_bits(),
        input.time_step_zone.to_bits()
    );
    assert_eq!(
        state.construction_return_route,
        Some(CtfPublicReturnRoute::AllResistive)
    );
    assert!(state.init_returned && outputs.reports.progress.postlude_returned);
    assert_eq!(outputs.reports.progress.layer_report_declared_indices, [0]);
    assert_eq!(
        outputs.reports.progress.coefficient_report_declared_indices,
        [0]
    );
    assert!(!eio.is_empty());
    Ok(())
}

#[test]
fn unused_return_does_not_read_layer_dimension_or_assign_timestep() -> TestResult {
    let mut input = input();
    input.is_used_ctf = false;
    input.time_step_zone = 4.0;
    input.solution_dimensions = 2;
    input.source_sink_present = true;
    input.layer.thickness = 4.0;
    let mut state = CtfDirectInitializationState::new(true, 7);
    let mut outputs = Outputs::new().map_err(|value| format!("registry: {value:?}"))?;
    outputs.run(
        &input,
        &ReportInputs::default(),
        &mut state,
        &mut Vec::new(),
    )?;
    assert_eq!(state.storage, CtfPublicCoefficientStorage::reset());
    assert!(state.preprocessing.is_none() && state.all_resistive.is_none());
    assert_eq!(
        state.construction_return_route,
        Some(CtfPublicReturnRoute::UnusedReset)
    );
    assert_eq!(state.aggregate.max_ctf_terms, 7);
    assert_eq!(state.aggregate.returned_constructions, 1);
    assert!(outputs.error.is_empty() && state.init_returned);
    Ok(())
}

#[test]
fn actual_loading_error_reset_and_report_prefix_survive_final_fatal() -> TestResult {
    let mut zero_resistance = input();
    zero_resistance.layer.resistance = 0.0;
    let mut thick = input();
    thick.layer.thickness = 4.0;
    thick.layer.conductivity = 1.0;
    thick.layer.density = 1000.0;
    thick.layer.specific_heat = 1000.0;
    thick.layer.resistance = 4.0;
    thick.layer.resistance_only = false;
    for input in [zero_resistance, thick] {
        let mut state = CtfDirectInitializationState::new(true, 0);
        let mut outputs = Outputs::new().map_err(|value| format!("registry: {value:?}"))?;
        let mut eio = Vec::new();
        let stop = outputs.run(&input, &report_inputs(), &mut state, &mut eio);
        assert!(matches!(
            stop,
            Err(CtfDirectInitializationError::Postlude(
                CtfInitPostludeError::Fatal(SourceFatalStop::Fatal(_))
            ))
        ));
        assert_eq!(
            state.construction_return_route,
            Some(CtfPublicReturnRoute::LoadingErrorReset)
        );
        assert!(state.shared_flags.errors_found && !state.shared_flags.do_ctf_error_report);
        assert!(state.all_resistive.is_none() && !state.init_returned);
        assert_eq!(state.storage, CtfPublicCoefficientStorage::reset());
        assert_eq!(state.aggregate.returned_constructions, 1);
        assert!(
            outputs.reports.progress.report_headers_written
                && outputs.reports.progress.final_fatal_invoked
        );
        assert!(
            !outputs.reports.progress.postlude_returned
                && !eio.is_empty()
                && !outputs.error.is_empty()
        );
    }
    Ok(())
}

#[test]
fn genuine_repeat_registry_and_pre_scan_remain_caller_owned_setup() -> TestResult {
    use crate::heat_balance::report_scan::try_scan_for_reports;
    let mut outputs = Outputs::new().map_err(|value| format!("registry: {value:?}"))?;
    outputs
        .reports
        .registry
        .set_predefined_tables(false, false)
        .map_err(|value| format!("repeat registry: {value:?}"))?;
    let columns = outputs.reports.registry.opaque_layer_columns().to_vec();
    assert_eq!(columns.len(), 20);
    let inputs = report_inputs();
    let assigned = try_scan_for_reports(
        &inputs,
        &mut outputs.reports.scan,
        "Constructions",
        Some("Constructions"),
        None,
        None,
        |_| Ok::<(), std::convert::Infallible>(()),
    )
    .map_err(|value| format!("setup scan: {value:?}"))?;
    assert!(assigned);
    let prepared_scan = outputs.reports.scan.clone();
    let mut state = CtfDirectInitializationState::new(true, 0);
    outputs.run(&input(), &inputs, &mut state, &mut Vec::new())?;
    assert_eq!(
        outputs.reports.registry.opaque_layer_columns(),
        columns.as_slice()
    );
    assert_eq!(outputs.reports.scan, prepared_scan);
    assert!(state.init_returned);
    Ok(())
}

#[test]
fn retained_preprocessing_keeps_prior_sticky_assignment_after_later_callback_failure() {
    let mut layer = input().layer;
    layer.thickness = 4.0;
    layer.resistance = 0.0;
    let mut owner = None;
    let mut calls = 0;
    let result = try_preprocess_ctf_layers_retained(
        &[layer],
        CtfLayerContext {
            is_used_ctf: true,
            errors_found: false,
            source_sink_present: false,
            solution_dimensions: 1,
        },
        &mut owner,
        |_| {
            calls += 1;
            if calls == 3 {
                Err("actual callback stop")
            } else {
                Ok(())
            }
        },
    );
    assert_eq!(
        result,
        Err(CtfLayerFailure::Diagnostic("actual callback stop"))
    );
    assert!(owner.as_ref().is_some_and(|actual| actual.errors_found
        && actual.issues == [CtfLayerIssue::MaterialTooThick(1)]
        && actual.after_load.is_none()
        && actual.after_conversion.is_none()));
    assert_eq!(calls, 3);
}

struct FailedEio;
impl Write for FailedEio {
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
        Err(io::Error::other("actual EIO stop"))
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn actual_eio_failure_keeps_public_and_table_owners_without_init_return() -> TestResult {
    let mut outputs = Outputs::new().map_err(|value| format!("registry: {value:?}"))?;
    let mut state = CtfDirectInitializationState::new(true, 0);
    let result = outputs.run(&input(), &report_inputs(), &mut state, &mut FailedEio);
    assert!(matches!(
        result,
        Err(CtfDirectInitializationError::Postlude(
            CtfInitPostludeError::Report(super::super::ctf_reports::CtfReportError::Io(_))
        ))
    ));
    assert_eq!(
        state.construction_return_route,
        Some(CtfPublicReturnRoute::AllResistive)
    );
    assert!(state.all_resistive.is_some() && !state.init_returned);
    assert_eq!(outputs.reports.table.num_table_entry, 1);
    assert_eq!(
        outputs.reports.progress.construction_scan_result,
        Some(true)
    );
    assert!(
        !outputs.reports.progress.report_headers_written
            && !outputs.reports.progress.postlude_returned
    );
    Ok(())
}
