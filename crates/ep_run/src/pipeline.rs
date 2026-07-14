//! Arbitrary-run pipeline implementation.

use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::hash::Hasher;
use std::path::{Path, PathBuf};
use std::time::Instant;

use ep_compare::{
    SeriesAlignment, SeriesComparisonStatus, Tolerance, compare_series_samples_v2,
    load_eso_time_series,
};
use ep_compiler::{CompileReport, compile_raw_model};
use ep_model::{SimulationModel, TypedModel};
use ep_oracle::default_oracle_release;
use ep_raw_model::{RawModel, load_epjson_file, load_epjson_file_with_idf_order};
use ep_runtime::{
    ExecutionPlan, ExecutionStep, HeatBalanceSimulationOptions, IdealLoadsCompatibilityOptions,
    NodeStateProjectionOptions, ResultStore, RuntimePrecomputedData, ScheduleValueSeries, TimeAxis,
    WeatherTimestepSeries, build_hourly_time_axis, build_hourly_time_axis_with_weather_metadata,
    load_epw_weather_file, precompute_runtime_data, precompute_schedule_value_series_for_time_axis,
    precompute_weather_timestep_series, select_epw_environment_weather,
    simulate_heat_balance_zone_air_temperatures_with_weather_series,
    simulate_ideal_loads_node_state_projection, simulate_ideal_loads_purchased_air_compat,
};
use serde::Serialize;
use serde_json::{Value, json};

use crate::oracle::{
    OracleBaselineSummary, OracleInputKind, OracleOutputRequest, convert_idf_to_epjson,
    resolve_oracle_paths, run_oracle_baseline,
};
use crate::outputs::{
    render_compatibility_boundary, render_eplusrs_err, render_run_report, render_support_report,
    result_store_json, write_empty_meters_csv, write_json, write_selected_outputs_csv, write_text,
};
use crate::{
    RunConfig, RunDiagnosticSeverity as Severity, RunDiagnostics, RunExitCode, RunResultState,
    RuntimeClass, SelectedAlgorithmLane, SupportAssessment, SupportStatus, TraceLevel,
    TraceSelection, assess_support,
};

/// Completed arbitrary-run outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RunOutcome {
    /// Process exit-code contract selected by the pipeline.
    pub exit_code: RunExitCode,
    /// Output directory.
    pub output_dir: PathBuf,
    /// Run summary JSON path.
    pub run_summary_path: PathBuf,
    /// Support assessment status.
    pub support_status: SupportStatus,
    /// User-facing run result state.
    pub run_result_state: RunResultState,
    /// Optional message for CLI display.
    pub message: String,
}

/// Error returned before a complete run summary can be written.
#[derive(Debug)]
pub struct RunError {
    /// Process exit-code contract.
    pub exit_code: RunExitCode,
    /// Error message.
    pub message: String,
}

impl RunError {
    fn new(exit_code: RunExitCode, message: impl Into<String>) -> Self {
        Self {
            exit_code,
            message: message.into(),
        }
    }
}

impl Display for RunError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.message)
    }
}

impl std::error::Error for RunError {}

#[derive(Clone, Debug, PartialEq, Serialize)]
struct PhaseTiming {
    name: String,
    engine: String,
    wall_seconds: f64,
    scope: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
struct PipelineTiming {
    schema_version: u32,
    measurement: String,
    phases: Vec<PhaseTiming>,
    total_wall_seconds: f64,
}

impl PipelineTiming {
    fn new() -> Self {
        Self {
            schema_version: 1,
            measurement: "wall-clock seconds measured inside ep_run pipeline".to_string(),
            phases: Vec::new(),
            total_wall_seconds: 0.0,
        }
    }

    fn push(&mut self, name: &str, engine: &str, wall_seconds: f64, scope: impl Into<String>) {
        self.phases.push(PhaseTiming {
            name: name.to_string(),
            engine: engine.to_string(),
            wall_seconds,
            scope: scope.into(),
        });
    }
}

struct PreparedInput {
    input_kind: OracleInputKind,
    original_path: PathBuf,
    converted_epjson_path: PathBuf,
}

struct RustRuntimeResult {
    results: ResultStore,
    runtime_class: RuntimeClass,
    sample_count: usize,
    source_order_gate: SourceOrderGateSummary,
}

struct PreparedRuntimeInputs {
    sample_count: usize,
    time_axis: TimeAxis,
    schedule_series: Vec<ScheduleValueSeries>,
    weather_series: Option<WeatherTimestepSeries>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct GraphAndPlanExportSummary {
    trace_wall_seconds: f64,
    trace_file_size_bytes: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
struct SourceOrderGateSummary {
    expected_source_order_stages: Vec<String>,
    actual_executed_source_order_stages: Vec<String>,
    matches: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
struct ComparisonSummary {
    schema_version: u32,
    status: String,
    conformance_claim: bool,
    oracle_version: String,
    series: Vec<ComparisonSeriesSummary>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
struct ComparisonSeriesSummary {
    key: String,
    variable_name: String,
    units: String,
    status: String,
    alignment: String,
    oracle_samples: usize,
    rust_samples: usize,
    compared_samples: usize,
    max_abs_delta: f64,
    rmse_delta: f64,
    max_rel_delta: f64,
    first_divergence: Option<Value>,
}

/// Runs an arbitrary IDF/epJSON through the Rust support gate and optional oracle comparison.
pub fn run_arbitrary_idf(config: &RunConfig) -> Result<RunOutcome, RunError> {
    let total_start = Instant::now();
    prepare_output_dir(&config.output_dir, config.overwrite)?;
    create_output_layout(&config.output_dir)
        .map_err(|error| RunError::new(RunExitCode::OutputExport, error))?;

    let mut timing = PipelineTiming::new();
    let mut diagnostics = RunDiagnostics::default();
    let mut oracle_status = "not-requested".to_string();
    let mut compare_status = "not-requested".to_string();
    let mut oracle_summary = None;
    let mut comparison_summary = None;

    let input_start = Instant::now();
    let prepared_input = match prepare_input(config, &mut diagnostics) {
        Ok(input) => input,
        Err(error) => {
            let message = error.to_string();
            let diagnostic_code = input_error_diagnostic_code(&message);
            let exit_code = error.exit_code;
            diagnostics.error(diagnostic_code, "input", message);
            return finish_early(
                config,
                diagnostics,
                timing,
                exit_code,
                SupportStatus::Unsupported,
                RunResultState::RunBlocked,
                "input import failed",
            );
        }
    };
    timing.push(
        "input_resolver",
        "ep_run",
        input_start.elapsed().as_secs_f64(),
        "stage original input, convert IDF to epJSON when needed, and write input hashes",
    );

    let raw_start = Instant::now();
    let raw_model_result = match prepared_input.input_kind {
        OracleInputKind::Idf => load_epjson_file_with_idf_order(
            &prepared_input.converted_epjson_path,
            &prepared_input.original_path,
        ),
        OracleInputKind::EpJson => load_epjson_file(&prepared_input.converted_epjson_path),
    };
    let raw_model = match raw_model_result {
        Ok(model) => model,
        Err(error) => {
            diagnostics.error(
                "RawModelParseFailed",
                "raw-model",
                format!(
                    "failed to load {} input into RawModel: {error}",
                    prepared_input.input_kind.id()
                ),
            );
            return finish_early(
                config,
                diagnostics,
                timing,
                RunExitCode::ImportParse,
                SupportStatus::Unsupported,
                RunResultState::RunBlocked,
                "raw model parse failed",
            );
        }
    };
    timing.push(
        "raw_model",
        "ep_raw_model",
        raw_start.elapsed().as_secs_f64(),
        "parse epJSON into RawModel, preserve unknown objects, and recover configured IDF order",
    );
    write_raw_model_summary(&config.output_dir, &raw_model)
        .map_err(|error| RunError::new(RunExitCode::OutputExport, error))?;

    let compile_start = Instant::now();
    let compile_result = compile_raw_model(&raw_model);
    let typed_model = compile_result.model.as_ref();
    timing.push(
        "typed_compile",
        "ep_compiler",
        compile_start.elapsed().as_secs_f64(),
        "convert RawModel into TypedModel, resolve references, and collect coverage",
    );
    write_compile_artifacts(&config.output_dir, &compile_result.report, typed_model)
        .map_err(|error| RunError::new(RunExitCode::OutputExport, error))?;

    let support_start = Instant::now();
    let assessment = assess_support(
        &raw_model,
        &compile_result.report,
        typed_model,
        config.mode,
        config.partial_policy,
        config.output_format,
        config.trace_level,
    );
    timing.push(
        "support_assessment",
        "ep_run",
        support_start.elapsed().as_secs_f64(),
        "classify object, topology, and algorithm support before Rust execution",
    );
    diagnostics = merge_diagnostics(diagnostics, assessment.diagnostics.clone());
    if !config.dry_run
        && assessment.allows_rust_runtime()
        && runtime_class_requires_weather(assessment.runtime_class)
        && config.weather_path.is_none()
    {
        diagnostics.error(
            "MissingWeatherFile",
            "input",
            "weather EPW path is required for heat-balance compatibility runtime",
        );
        write_support_artifacts(&config.output_dir, &assessment, &diagnostics)
            .map_err(|error| RunError::new(RunExitCode::OutputExport, error))?;
        return finish_successful_summary(
            config,
            &prepared_input,
            &assessment,
            diagnostics,
            timing,
            None,
            None,
            oracle_status,
            compare_status,
            None,
            RunExitCode::Args,
            "missing weather file",
        );
    }
    write_support_artifacts(&config.output_dir, &assessment, &diagnostics)
        .map_err(|error| RunError::new(RunExitCode::OutputExport, error))?;

    let mut simulation_model: Option<SimulationModel> = None;
    let mut runtime_precomputed: Option<RuntimePrecomputedData> = None;
    if assessment.allows_rust_runtime() {
        let graph_start = Instant::now();
        simulation_model = typed_model.cloned().map(SimulationModel::from_typed);
        timing.push(
            "graph_build",
            "ep_model",
            graph_start.elapsed().as_secs_f64(),
            "build SimulationModel and ModelGraph after support assessment allows runtime execution",
        );

        let plan_start = Instant::now();
        runtime_precomputed = simulation_model.as_ref().map(precompute_runtime_data);
        let mut graph_and_plan_export = GraphAndPlanExportSummary::default();
        if let (Some(model), Some(precomputed)) =
            (simulation_model.as_ref(), runtime_precomputed.as_ref())
        {
            graph_and_plan_export = write_graph_and_plan(
                &config.output_dir,
                model,
                precomputed,
                config.trace_level,
                &config.trace_selection,
            )
            .map_err(|error| RunError::new(RunExitCode::OutputExport, error))?;
        }
        timing.push(
            "execution_plan",
            "ep_runtime",
            plan_start.elapsed().as_secs_f64(),
            "precompute ExecutionPlan and OutputRegistry for supported typed subset, then write plan artifacts",
        );
        timing.push(
            "trace_overhead",
            "ep_run",
            graph_and_plan_export.trace_wall_seconds,
            format!(
                "generate trace metadata and optional source-order stage snapshot artifacts; trace_file_size_bytes={}",
                graph_and_plan_export.trace_file_size_bytes
            ),
        );
    }

    if config.dry_run {
        oracle_status = "skipped-dry-run".to_string();
        compare_status = "skipped-dry-run".to_string();
        return finish_successful_summary(
            config,
            &prepared_input,
            &assessment,
            diagnostics,
            timing,
            None,
            None,
            oracle_status,
            compare_status,
            None,
            if compile_result.model.is_none() {
                RunExitCode::CompileReference
            } else if assessment.allows_rust_runtime() {
                RunExitCode::Success
            } else {
                RunExitCode::Unsupported
            },
            "dry run completed",
        );
    }

    let mut rust_runtime_result = None;
    if assessment.allows_rust_runtime() {
        let source_order_gate = match runtime_precomputed
            .as_ref()
            .map(|precomputed| source_order_gate_summary(&precomputed.execution_plan))
        {
            Some(gate) if gate.matches => gate,
            Some(gate) => {
                diagnostics.error(
                    "ExecutionPlanSourceOrderMismatch",
                    "execution_plan",
                    format!(
                        "expected source-order stages {:?} but actual execution plan stages {:?}",
                        gate.expected_source_order_stages, gate.actual_executed_source_order_stages
                    ),
                );
                return finish_successful_summary(
                    config,
                    &prepared_input,
                    &assessment,
                    diagnostics,
                    timing,
                    None,
                    None,
                    oracle_status,
                    compare_status,
                    None,
                    RunExitCode::Plan,
                    "execution plan source-order gate failed",
                );
            }
            None => {
                diagnostics.error(
                    "ExecutionPlanMissing",
                    "execution_plan",
                    "Rust runtime was allowed but no execution plan was available",
                );
                return finish_successful_summary(
                    config,
                    &prepared_input,
                    &assessment,
                    diagnostics,
                    timing,
                    None,
                    None,
                    oracle_status,
                    compare_status,
                    None,
                    RunExitCode::Plan,
                    "execution plan was missing",
                );
            }
        };
        let runtime_setup_start = Instant::now();
        let runtime_inputs = match prepare_runtime_inputs(
            config,
            simulation_model.as_ref(),
            assessment.runtime_class,
        ) {
            Ok(inputs) => inputs,
            Err(error) => {
                diagnostics.error("RuntimeConvergenceFailure", "runtime", error);
                return finish_successful_summary(
                    config,
                    &prepared_input,
                    &assessment,
                    diagnostics,
                    timing,
                    None,
                    None,
                    oracle_status,
                    compare_status,
                    None,
                    RunExitCode::Runtime,
                    "Rust runtime failed",
                );
            }
        };
        timing.push(
            "rust_runtime_setup",
            "ep_run",
            runtime_setup_start.elapsed().as_secs_f64(),
            "resolve runtime inputs before the loop; weather runtimes load rich EPW metadata, build the metadata-aware time axis, select source-order records, and precompute weather timesteps",
        );

        let runtime_start = Instant::now();
        match execute_rust_runtime(
            simulation_model.as_ref(),
            assessment.runtime_class,
            source_order_gate,
            &runtime_inputs,
        ) {
            Ok(result) => {
                timing.push(
                    "rust_runtime",
                    "rusted-energyplus",
                    runtime_start.elapsed().as_secs_f64(),
                    format!(
                        "execute {} for {} hourly samples",
                        result.runtime_class.id(),
                        result.sample_count
                    ),
                );
                let export_start = Instant::now();
                write_runtime_artifacts(&config.output_dir, &result.results)
                    .map_err(|error| RunError::new(RunExitCode::OutputExport, error))?;
                timing.push(
                    "rust_output_export",
                    "ep_run",
                    export_start.elapsed().as_secs_f64(),
                    "write result-store.json, selected-outputs.csv, and meters.csv",
                );
                rust_runtime_result = Some(result);
            }
            Err(error) => {
                diagnostics.error("RuntimeConvergenceFailure", "runtime", error);
                return finish_successful_summary(
                    config,
                    &prepared_input,
                    &assessment,
                    diagnostics,
                    timing,
                    None,
                    None,
                    oracle_status,
                    compare_status,
                    None,
                    RunExitCode::Runtime,
                    "Rust runtime failed",
                );
            }
        }
    }

    let should_run_oracle = config.oracle_baseline || config.compare_oracle;
    if should_run_oracle {
        let oracle_start = Instant::now();
        match run_requested_oracle_baseline(
            config,
            &prepared_input,
            rust_runtime_result.as_ref().map(|result| &result.results),
        ) {
            Ok(summary) => {
                timing.push(
                    "energyplus_oracle",
                    "EnergyPlus",
                    oracle_start.elapsed().as_secs_f64(),
                    "stage and execute EnergyPlus oracle baseline in output/oracle",
                );
                oracle_status = "generated".to_string();
                oracle_summary = Some(summary);
            }
            Err(error) => {
                diagnostics.error("OracleBaselineFailed", "oracle", error.to_string());
                oracle_status = "failed".to_string();
                if assessment.allows_rust_runtime() {
                    return finish_successful_summary(
                        config,
                        &prepared_input,
                        &assessment,
                        diagnostics,
                        timing,
                        rust_runtime_result.as_ref(),
                        None,
                        oracle_status,
                        compare_status,
                        None,
                        RunExitCode::OracleCompare,
                        "oracle baseline failed",
                    );
                }
            }
        }
    }

    if config.compare_oracle {
        let compare_start = Instant::now();
        match (
            rust_runtime_result.as_ref().map(|result| &result.results),
            oracle_summary.as_ref(),
        ) {
            (Some(results), Some(summary)) => match compare_with_oracle(results, summary) {
                Ok(summary) => {
                    compare_status = summary.status.clone();
                    write_compare_artifacts(&config.output_dir, &summary)
                        .map_err(|error| RunError::new(RunExitCode::OutputExport, error))?;
                    timing.push(
                        "oracle_compare",
                        "ep_compare",
                        compare_start.elapsed().as_secs_f64(),
                        "compare Rust result-store time series with EnergyPlus ESO oracle series",
                    );
                    comparison_summary = Some(summary);
                }
                Err(error) => {
                    diagnostics.error("OracleCompareFailed", "compare", error);
                    compare_status = "failed".to_string();
                }
            },
            _ => {
                compare_status = "skipped-rust-unsupported-or-oracle-missing".to_string();
                let skipped = ComparisonSummary {
                    schema_version: 1,
                    status: compare_status.clone(),
                    conformance_claim: false,
                    oracle_version: default_oracle_release().version.to_string(),
                    series: Vec::new(),
                };
                write_compare_artifacts(&config.output_dir, &skipped)
                    .map_err(|error| RunError::new(RunExitCode::OutputExport, error))?;
                comparison_summary = Some(skipped);
            }
        }
    }

    let mut exit_code = if compile_result.model.is_none() {
        RunExitCode::CompileReference
    } else if assessment.allows_rust_runtime() {
        RunExitCode::Success
    } else {
        RunExitCode::Unsupported
    };
    if config.fail_on_warning && diagnostics.has_warnings() && exit_code == RunExitCode::Success {
        exit_code = RunExitCode::Unsupported;
    }
    if config.compare_oracle
        && matches!(compare_status.as_str(), "fail" | "failed")
        && exit_code == RunExitCode::Success
    {
        exit_code = RunExitCode::OracleCompare;
    }

    timing.total_wall_seconds = total_start.elapsed().as_secs_f64();
    finish_successful_summary(
        config,
        &prepared_input,
        &assessment,
        diagnostics,
        timing,
        rust_runtime_result.as_ref(),
        oracle_summary.as_ref(),
        oracle_status,
        compare_status,
        comparison_summary.as_ref(),
        exit_code,
        "arbitrary run completed",
    )
}

fn finish_early(
    config: &RunConfig,
    diagnostics: RunDiagnostics,
    mut timing: PipelineTiming,
    exit_code: RunExitCode,
    support_status: SupportStatus,
    run_result_state: RunResultState,
    message: &str,
) -> Result<RunOutcome, RunError> {
    timing.total_wall_seconds = timing.phases.iter().map(|phase| phase.wall_seconds).sum();
    write_json(&config.output_dir.join("diagnostics.json"), &diagnostics)
        .map_err(|error| RunError::new(RunExitCode::OutputExport, error))?;
    write_text(
        &config.output_dir.join("eplusrs.err"),
        &render_eplusrs_err(&diagnostics, exit_code),
    )
    .map_err(|error| RunError::new(RunExitCode::OutputExport, error))?;
    let summary = json!({
        "schema_version": 1,
        "status": exit_code.id(),
        "exit_code": exit_code.code(),
        "message": message,
        "support_status": support_status.id(),
        "run_result_state": run_result_state.id(),
        "selected_algorithm_lane": SelectedAlgorithmLane::none(),
        "conformance_claim": false,
        "diagnostics": diagnostic_counts(&diagnostics),
        "timing": timing,
    });
    let run_summary_path = config.output_dir.join("run-summary.json");
    write_json(&run_summary_path, &summary)
        .map_err(|error| RunError::new(RunExitCode::OutputExport, error))?;
    Ok(RunOutcome {
        exit_code,
        output_dir: config.output_dir.clone(),
        run_summary_path,
        support_status,
        run_result_state,
        message: message.to_string(),
    })
}

#[allow(clippy::too_many_arguments)]
fn finish_successful_summary(
    config: &RunConfig,
    prepared_input: &PreparedInput,
    assessment: &SupportAssessment,
    diagnostics: RunDiagnostics,
    mut timing: PipelineTiming,
    rust_runtime_result: Option<&RustRuntimeResult>,
    oracle_summary: Option<&OracleBaselineSummary>,
    oracle_status: String,
    compare_status: String,
    comparison_summary: Option<&ComparisonSummary>,
    exit_code: RunExitCode,
    message: &str,
) -> Result<RunOutcome, RunError> {
    write_json(&config.output_dir.join("diagnostics.json"), &diagnostics)
        .map_err(|error| RunError::new(RunExitCode::OutputExport, error))?;
    write_text(
        &config.output_dir.join("eplusrs.err"),
        &render_eplusrs_err(&diagnostics, exit_code),
    )
    .map_err(|error| RunError::new(RunExitCode::OutputExport, error))?;
    let report_start = Instant::now();
    let report = render_run_report(
        assessment,
        rust_runtime_result.is_some(),
        &oracle_status,
        &compare_status,
    );
    write_text(
        &config.output_dir.join("reports").join("run-report.md"),
        &report,
    )
    .map_err(|error| RunError::new(RunExitCode::OutputExport, error))?;
    write_text(
        &config
            .output_dir
            .join("reports")
            .join("compatibility-boundary.md"),
        &render_compatibility_boundary(assessment),
    )
    .map_err(|error| RunError::new(RunExitCode::OutputExport, error))?;
    write_text(
        &config.output_dir.join("logs").join("command.log"),
        "command: eplus-rs run <input> --weather <epw> --output-dir <dir>\n",
    )
    .map_err(|error| RunError::new(RunExitCode::OutputExport, error))?;
    timing.push(
        "report_generation",
        "ep_run",
        report_start.elapsed().as_secs_f64(),
        "render run report, compatibility boundary, and command log after runtime/oracle/compare phases",
    );
    timing.total_wall_seconds = timing
        .total_wall_seconds
        .max(timing.phases.iter().map(|phase| phase.wall_seconds).sum());

    let run_summary = json!({
        "schema_version": 1,
        "status": exit_code.id(),
        "exit_code": exit_code.code(),
        "message": message,
        "input": {
            "kind": prepared_input.input_kind.id(),
            "original": prepared_input.original_path.display().to_string(),
            "converted_epjson": prepared_input.converted_epjson_path.display().to_string(),
            "weather": config.weather_path.as_ref().map(|path| path.display().to_string()),
        },
        "config": {
            "mode": config.mode.id(),
            "partial_policy": config.partial_policy.id(),
            "output_format": config.output_format.id(),
            "keep_intermediate": config.keep_intermediate,
            "trace_level": config.trace_level.id(),
            "trace_selection": config.trace_selection,
            "fail_on_warning": config.fail_on_warning,
            "dry_run": config.dry_run,
            "oracle_baseline": config.oracle_baseline,
            "compare_oracle": config.compare_oracle,
            "hours": config.hours,
        },
        "selected_algorithm_lane": assessment.selected_algorithm_lane.clone(),
        "support": {
            "status": assessment.status.id(),
            "run_result_state": assessment.run_result_state.id(),
            "runtime_class": assessment.runtime_class.id(),
            "selected_algorithm_lane": assessment.selected_algorithm_lane.clone(),
            "runtime_selection_note": assessment.runtime_selection_note,
            "capability_registry_loaded": assessment.capability_registry_loaded,
            "matched_capability_ids": assessment.matched_capability_ids.clone(),
            "matched_capabilities": assessment.matched_capabilities.clone(),
            "failed_capability_ids": assessment.failed_capability_ids.clone(),
            "active_ideal_loads_branches": assessment.active_ideal_loads_branches.clone(),
            "inactive_ideal_loads_branches": assessment.inactive_ideal_loads_branches.clone(),
            "conformance_claim": false,
        },
        "rust_runtime": rust_runtime_result.as_ref().map(|result| json!({
            "runtime_class": result.runtime_class.id(),
            "samples": result.sample_count,
            "series": result.results.series.len(),
            "source_order_stages": result.source_order_gate.actual_executed_source_order_stages.clone(),
        })),
        "source_order_gate": rust_runtime_result.as_ref().map(|result| &result.source_order_gate),
        "oracle": oracle_summary,
        "comparison": comparison_summary,
        "oracle_status": oracle_status,
        "compare_status": compare_status,
        "diagnostics": diagnostic_counts(&diagnostics),
        "artifacts": artifact_map(&config.output_dir),
        "timing": timing,
    });
    let run_summary_path = config.output_dir.join("run-summary.json");
    write_json(&run_summary_path, &run_summary)
        .map_err(|error| RunError::new(RunExitCode::OutputExport, error))?;
    Ok(RunOutcome {
        exit_code,
        output_dir: config.output_dir.clone(),
        run_summary_path,
        support_status: assessment.status,
        run_result_state: assessment.run_result_state,
        message: message.to_string(),
    })
}

fn input_error_diagnostic_code(message: &str) -> &'static str {
    if message.starts_with("ConvertInputFormatFailed") {
        "ConvertInputFormatFailed"
    } else if message.starts_with("UnsupportedInputFormat") {
        "UnsupportedInputFormat"
    } else if message.starts_with("missing weather file") {
        "MissingWeatherFile"
    } else if message.starts_with("missing input file") {
        "MissingInputFile"
    } else {
        "RawModelParseFailed"
    }
}

fn prepare_output_dir(output_dir: &Path, overwrite: bool) -> Result<(), RunError> {
    if output_dir.exists() {
        if overwrite {
            assert_safe_output_dir(output_dir)?;
            std::fs::remove_dir_all(output_dir).map_err(|error| {
                RunError::new(
                    RunExitCode::OutputExport,
                    format!(
                        "failed to remove output directory {}: {error}",
                        output_dir.display()
                    ),
                )
            })?;
        } else if output_dir
            .read_dir()
            .map_err(|error| {
                RunError::new(
                    RunExitCode::OutputExport,
                    format!(
                        "failed to inspect output directory {}: {error}",
                        output_dir.display()
                    ),
                )
            })?
            .next()
            .is_some()
        {
            return Err(RunError::new(
                RunExitCode::Args,
                format!(
                    "output directory is not empty; pass --overwrite to replace it: {}",
                    output_dir.display()
                ),
            ));
        }
    }
    std::fs::create_dir_all(output_dir).map_err(|error| {
        RunError::new(
            RunExitCode::OutputExport,
            format!(
                "failed to create output directory {}: {error}",
                output_dir.display()
            ),
        )
    })
}

fn assert_safe_output_dir(output_dir: &Path) -> Result<(), RunError> {
    let full = std::fs::canonicalize(output_dir).unwrap_or_else(|_| output_dir.to_path_buf());
    let cwd = std::env::current_dir().map_err(|error| {
        RunError::new(
            RunExitCode::OutputExport,
            format!("failed to resolve current directory: {error}"),
        )
    })?;
    if full == cwd || full.parent().is_none() {
        return Err(RunError::new(
            RunExitCode::Args,
            format!(
                "refusing to overwrite unsafe output directory: {}",
                full.display()
            ),
        ));
    }
    Ok(())
}

fn create_output_layout(output_dir: &Path) -> Result<(), String> {
    for relative in [
        "input", "model", "results", "reports", "logs", "oracle", "compare",
    ] {
        std::fs::create_dir_all(output_dir.join(relative))
            .map_err(|error| format!("failed to create output/{relative}: {error}"))?;
    }
    Ok(())
}

fn prepare_input(
    config: &RunConfig,
    diagnostics: &mut RunDiagnostics,
) -> Result<PreparedInput, RunError> {
    if !config.input_path.is_file() {
        return Err(RunError::new(
            RunExitCode::ImportParse,
            format!("missing input file: {}", config.input_path.display()),
        ));
    }
    if let Some(weather_path) = config.weather_path.as_ref()
        && !weather_path.is_file()
    {
        return Err(RunError::new(
            RunExitCode::Args,
            format!("missing weather file: {}", weather_path.display()),
        ));
    }

    let input_kind = OracleInputKind::from_path(&config.input_path).ok_or_else(|| {
        RunError::new(
            RunExitCode::ImportParse,
            "UnsupportedInputFormat: input must be .idf or .epJSON",
        )
    })?;
    let input_dir = config.output_dir.join("input");
    let original_path = match input_kind {
        OracleInputKind::Idf => input_dir.join("original.idf"),
        OracleInputKind::EpJson => input_dir.join("original.epJSON"),
    };
    std::fs::copy(&config.input_path, &original_path).map_err(|error| {
        RunError::new(
            RunExitCode::ImportParse,
            format!("failed to stage original input: {error}"),
        )
    })?;

    let converted_epjson_path = input_dir.join("converted.epJSON");
    let conversion_seconds = match input_kind {
        OracleInputKind::Idf => {
            let oracle_paths =
                resolve_oracle_paths(config.oracle_root.as_deref()).map_err(|error| {
                    RunError::new(
                        RunExitCode::ImportParse,
                        format!("ConvertInputFormatFailed: {error}"),
                    )
                })?;
            convert_idf_to_epjson(
                &oracle_paths.convert_input_format_exe,
                &original_path,
                &converted_epjson_path,
            )
            .map_err(|error| {
                RunError::new(
                    RunExitCode::ImportParse,
                    format!("ConvertInputFormatFailed: {error}"),
                )
            })?
        }
        OracleInputKind::EpJson => {
            std::fs::copy(&original_path, &converted_epjson_path).map_err(|error| {
                RunError::new(
                    RunExitCode::ImportParse,
                    format!("failed to stage epJSON input: {error}"),
                )
            })?;
            0.0
        }
    };
    diagnostics.info(
        "InputResolved",
        "input",
        format!(
            "input kind {} staged; IDF conversion wall seconds {:.9}",
            input_kind.id(),
            conversion_seconds
        ),
    );

    write_input_hashes(
        &config.output_dir,
        &config.input_path,
        &original_path,
        &converted_epjson_path,
    )
    .map_err(|error| RunError::new(RunExitCode::OutputExport, error))?;

    Ok(PreparedInput {
        input_kind,
        original_path,
        converted_epjson_path,
    })
}

fn write_input_hashes(
    output_dir: &Path,
    source_path: &Path,
    original_path: &Path,
    converted_epjson_path: &Path,
) -> Result<(), String> {
    let hashes = json!({
        "schema_version": 1,
        "algorithm": "fnv-1a-64",
        "source": file_hash_json(source_path)?,
        "staged_original": file_hash_json(original_path)?,
        "converted_epjson": file_hash_json(converted_epjson_path)?,
    });
    write_json(&output_dir.join("input").join("input-hashes.json"), &hashes)
}

fn file_hash_json(path: &Path) -> Result<Value, String> {
    let bytes = std::fs::read(path)
        .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
    let hash = fnv1a_64(&bytes);
    Ok(json!({
        "path": path.display().to_string(),
        "bytes": bytes.len(),
        "hash": format!("{hash:016x}"),
    }))
}

fn fnv1a_64(bytes: &[u8]) -> u64 {
    let mut hash = Fnv1a64::default();
    hash.write(bytes);
    hash.finish()
}

struct Fnv1a64(u64);

impl Default for Fnv1a64 {
    fn default() -> Self {
        Self(0xcbf29ce484222325)
    }
}

impl Hasher for Fnv1a64 {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.0 ^= u64::from(*byte);
            self.0 = self.0.wrapping_mul(0x100000001b3);
        }
    }
}

fn write_raw_model_summary(output_dir: &Path, raw_model: &RawModel) -> Result<(), String> {
    let summary = raw_model.summary();
    let value = json!({
        "schema_version": 1,
        "version": summary.version,
        "object_type_count": summary.object_type_count,
        "object_count": summary.object_count,
        "object_type_counts": summary.object_type_counts,
    });
    write_json(
        &output_dir.join("model").join("raw-model-summary.json"),
        &value,
    )
}

fn write_compile_artifacts(
    output_dir: &Path,
    report: &CompileReport,
    typed_model: Option<&TypedModel>,
) -> Result<(), String> {
    let value = match typed_model {
        Some(model) => json!({
            "schema_version": 1,
            "status": "compiled",
            "raw_objects": report.raw_object_count,
            "typed_objects": report.typed_object_count,
            "version": model.version.to_string(),
            "counts": typed_counts(model),
            "diagnostics": report.diagnostics.iter().map(|diagnostic| json!({
                "severity": diagnostic.severity.to_string(),
                "code": diagnostic.code,
                "object_type": diagnostic.object_type,
                "object_name": diagnostic.object_name,
                "field": diagnostic.field,
                "message": diagnostic.message,
            })).collect::<Vec<_>>(),
            "coverage": report.coverage.iter().map(|coverage| json!({
                "object_type": coverage.object_type,
                "object_count": coverage.object_count,
                "status": coverage.status.to_string(),
            })).collect::<Vec<_>>(),
        }),
        None => json!({
            "schema_version": 1,
            "status": "failed",
            "raw_objects": report.raw_object_count,
            "typed_objects": report.typed_object_count,
            "diagnostics": report.diagnostics.iter().map(|diagnostic| json!({
                "severity": diagnostic.severity.to_string(),
                "code": diagnostic.code,
                "object_type": diagnostic.object_type,
                "object_name": diagnostic.object_name,
                "field": diagnostic.field,
                "message": diagnostic.message,
            })).collect::<Vec<_>>(),
        }),
    };
    write_json(
        &output_dir.join("model").join("typed-model-summary.json"),
        &value,
    )
}

fn typed_counts(model: &TypedModel) -> BTreeMap<&'static str, usize> {
    BTreeMap::from([
        ("zones", model.zones.len()),
        ("surfaces", model.surfaces.len()),
        ("constructions", model.constructions.len()),
        ("materials", model.materials.len()),
        ("constant_schedules", model.schedules.len()),
        ("compact_schedules", model.compact_schedules.len()),
        ("other_equipment", model.other_equipment.len()),
        ("people", model.people.len()),
        (
            "thermostat_dual_setpoints",
            model.thermostat_dual_setpoints.len(),
        ),
        ("zone_thermostats", model.zone_thermostats.len()),
        (
            "ideal_loads_air_systems",
            model.ideal_loads_air_systems.len(),
        ),
        ("zone_equipment_lists", model.zone_equipment_lists.len()),
        (
            "zone_equipment_connections",
            model.zone_equipment_connections.len(),
        ),
        ("nodes", model.nodes.len()),
        ("node_lists", model.node_lists.len()),
        ("plant_loops", model.plant_loops.len()),
        ("plant_branches", model.plant_branches.len()),
        ("plant_branch_lists", model.plant_branch_lists.len()),
        ("plant_connectors", model.plant_connectors.len()),
        ("plant_connector_lists", model.plant_connector_lists.len()),
        ("pumps_constant_speed", model.pumps_constant_speed.len()),
        ("boilers_hot_water", model.boilers_hot_water.len()),
        ("chillers_electric_eir", model.chillers_electric_eir.len()),
    ])
}

fn write_graph_and_plan(
    output_dir: &Path,
    model: &SimulationModel,
    precomputed: &RuntimePrecomputedData,
    trace_level: TraceLevel,
    trace_selection: &TraceSelection,
) -> Result<GraphAndPlanExportSummary, String> {
    let plan = &precomputed.execution_plan;
    let graph = &model.graph;
    let graph_summary = json!({
        "schema_version": 1,
        "zone_surface_edges": graph.zone_surfaces.len(),
        "construction_material_edges": graph.construction_materials.len(),
        "zone_thermostat_edges": graph.zone_thermostats.len(),
        "thermostat_setpoint_edges": graph.thermostat_setpoints.len(),
        "zone_ideal_loads_edges": graph.zone_ideal_loads.len(),
        "node_list_member_edges": graph.node_list_members.len(),
        "ideal_loads_supply_node_edges": graph.ideal_loads_supply_nodes.len(),
        "ideal_loads_outdoor_air_spec_edges": graph.ideal_loads_outdoor_air_specs.len(),
        "zone_air_node_edges": graph.zone_air_nodes.len(),
        "plant_loop_branch_list_edges": graph.plant_loop_branch_lists.len(),
        "plant_branch_list_member_edges": graph.plant_branch_list_members.len(),
        "plant_connector_list_member_edges": graph.plant_connector_list_members.len(),
        "plant_branch_component_edges": graph.plant_branch_components.len(),
    });
    write_json(
        &output_dir.join("model").join("graph-summary.json"),
        &graph_summary,
    )?;
    let source_order_gate = source_order_gate_summary(plan);
    let expected_source_order_stages = source_order_gate.expected_source_order_stages.clone();
    let actual_executed_source_order_stages = source_order_gate
        .actual_executed_source_order_stages
        .clone();
    let trace_enabled = trace_level_enables_stage_snapshots(trace_level);
    let trace_start = Instant::now();
    let stage_snapshots = execution_stage_snapshots(plan, trace_level);
    let trace_file_size_bytes =
        write_source_order_stage_state_snapshots(output_dir, plan, trace_level, trace_selection)?;
    let plan_json = json!({
        "schema_version": 1,
        "stage_count": plan.stages.len(),
        "step_count": plan.step_count(),
        "output_registry_count": precomputed.output_registry.len(),
        "output_meter_registry_count": precomputed.output_registry.meter_registry().len(),
        "runtime_lookup_policy": {
            "post_typed_model_object_lookup": plan.runtime_policy.post_typed_model_object_lookup,
            "stage_execution_string_comparison": plan.runtime_policy.stage_execution_string_comparison,
            "stage_execution_hash_map_lookup": plan.runtime_policy.stage_execution_hash_map_lookup,
            "compatibility_plan_order": plan.runtime_policy.compatibility_plan_order,
            "fast_mode_grouping_policy": plan.runtime_policy.fast_mode_grouping_policy,
        },
        "prebound_summary": {
            "surface_loop_targets": model.typed.surfaces.len(),
            "zone_loop_targets": model.typed.zones.len(),
            "construction_coefficient_references": model.typed.constructions.len(),
            "schedule_ids": model.typed.schedules.len() + model.typed.compact_schedules.len(),
            "weather_series_indices": 1,
            "output_handles": precomputed.output_registry.len(),
        },
        "source_order_gate": source_order_gate,
        "expected_source_order_stages": expected_source_order_stages,
        "actual_executed_source_order_stages": actual_executed_source_order_stages,
        "trace_level": trace_level.id(),
        "trace_selection": trace_selection,
        "selected_trace_enabled": selected_trace_enabled(trace_level, trace_selection),
        "selected_trace_policy": "zone/surface/ctf payloads are emitted only for explicitly requested names; source-order stage snapshots remain metadata-only",
        "ctf_split_trace_enabled": ctf_split_trace_enabled(trace_level, trace_selection),
        "full_surface_trace_opt_in": full_surface_trace_opt_in(trace_level, trace_selection),
        "trace_output_write_policy": "buffered-json-writer",
        "trace_variable_handle_policy": "trace handles are separate from RuntimeOutputRegistry output handles",
        "trace_file_size_bytes": trace_file_size_bytes,
        "stage_snapshots_enabled": trace_level_enables_stage_snapshots(trace_level),
        "stage_snapshot_policy": "metadata-only source-order snapshots generated from ExecutionPlan; no simulation values are read or mutated",
        "stage_snapshots": stage_snapshots,
        "stages": plan.stages.iter().map(|stage| json!({
            "kind": stage.kind.id(),
            "name": stage.name,
            "steps": stage.steps.iter().map(execution_step_label).collect::<Vec<_>>(),
            "dependencies": {
                "reads": stage.dependencies.reads.clone(),
                "writes": stage.dependencies.writes.clone(),
            },
            "prebound": {
                "output_handles": stage.prebound.output_handles.iter().map(|id| id.0).collect::<Vec<_>>(),
                "surface_ids": stage.prebound.surface_ids.iter().map(|id| id.0).collect::<Vec<_>>(),
                "zone_ids": stage.prebound.zone_ids.iter().map(|id| id.0).collect::<Vec<_>>(),
                "construction_ids": stage.prebound.construction_ids.iter().map(|id| id.0).collect::<Vec<_>>(),
                "schedule_ids": stage.prebound.schedule_ids.iter().map(|id| id.0).collect::<Vec<_>>(),
                "weather_series_indices": stage.prebound.weather_series_indices.clone(),
            },
        })).collect::<Vec<_>>(),
        "compatibility_stages": plan.compatibility_stages.iter().map(|stage| json!({
            "kind": stage.kind.id(),
            "stage_name": stage.stage_name,
            "source_file": stage.source_file,
            "source_routine": stage.source_routine,
        })).collect::<Vec<_>>(),
    });
    write_json(
        &output_dir.join("model").join("execution-plan.json"),
        &plan_json,
    )?;
    Ok(GraphAndPlanExportSummary {
        trace_wall_seconds: if trace_enabled {
            trace_start.elapsed().as_secs_f64()
        } else {
            0.0
        },
        trace_file_size_bytes,
    })
}

fn trace_level_enables_stage_snapshots(trace_level: TraceLevel) -> bool {
    matches!(
        trace_level,
        TraceLevel::Stage
            | TraceLevel::Zone
            | TraceLevel::Surface
            | TraceLevel::Ctf
            | TraceLevel::Full
    )
}

fn selected_trace_enabled(trace_level: TraceLevel, selection: &TraceSelection) -> bool {
    matches!(
        trace_level,
        TraceLevel::Zone | TraceLevel::Surface | TraceLevel::Ctf | TraceLevel::Full
    ) && !selection.is_empty()
}

fn ctf_split_trace_enabled(trace_level: TraceLevel, selection: &TraceSelection) -> bool {
    matches!(trace_level, TraceLevel::Ctf | TraceLevel::Full) && !selection.surface_names.is_empty()
}

fn full_surface_trace_opt_in(trace_level: TraceLevel, selection: &TraceSelection) -> bool {
    matches!(
        trace_level,
        TraceLevel::Surface | TraceLevel::Ctf | TraceLevel::Full
    ) && !selection.surface_names.is_empty()
}

fn write_source_order_stage_state_snapshots(
    output_dir: &Path,
    plan: &ExecutionPlan,
    trace_level: TraceLevel,
    trace_selection: &TraceSelection,
) -> Result<u64, String> {
    if !trace_level_enables_stage_snapshots(trace_level) {
        return Ok(0);
    }
    let snapshots = source_order_stage_state_snapshots(plan, trace_level);
    let trace_path = output_dir
        .join("logs")
        .join("source-order-stage-state-snapshots.json");
    let artifact = json!({
        "schema_version": 1,
        "snapshot_schema": "rusted-energyplus.source-order-stage-state-snapshot.v1",
        "artifact_class": "diagnostic-trace",
        "trace_level": trace_level.id(),
        "trace_selection": trace_selection,
        "selected_trace_enabled": selected_trace_enabled(trace_level, trace_selection),
        "ctf_split_trace_enabled": ctf_split_trace_enabled(trace_level, trace_selection),
        "full_surface_trace_opt_in": full_surface_trace_opt_in(trace_level, trace_selection),
        "selected_surface_count": trace_selection.surface_names.len(),
        "selected_node_count": trace_selection.node_names.len(),
        "selected_trace_policy": "zone/surface/ctf payloads are emitted only for explicitly requested names; this artifact records stage metadata only",
        "trace_output_write_policy": "buffered-json-writer",
        "trace_variable_handle_policy": "trace handles are separate from RuntimeOutputRegistry output handles",
        "snapshot_count": snapshots.len(),
        "mutation_policy": "diagnostic trace artifact only; runtime calculations never read this file",
        "snapshots": snapshots,
    });
    write_json(&trace_path, &artifact)?;
    Ok(std::fs::metadata(&trace_path)
        .map(|metadata| metadata.len())
        .unwrap_or(0))
}

fn execution_stage_snapshots(plan: &ExecutionPlan, trace_level: TraceLevel) -> Vec<Value> {
    if !trace_level_enables_stage_snapshots(trace_level) {
        return Vec::new();
    }

    plan.stages
        .iter()
        .enumerate()
        .map(|(index, stage)| {
            json!({
                "index": index,
                "kind": stage.kind.id(),
                "name": stage.name,
                "source_order_barrier": stage.kind.is_source_order_barrier(),
                "step_count": stage.steps.len(),
                "steps": stage.steps.iter().map(execution_step_label).collect::<Vec<_>>(),
            })
        })
        .collect()
}

fn source_order_stage_state_snapshot_targets()
-> &'static [(&'static str, &'static str, &'static str)] {
    &[
        ("init-heat-balance", "stage", "heat_balance"),
        ("calc-heat-balance-outside-surf", "stage", "heat_balance"),
        ("calc-heat-balance-inside-surf", "stage", "heat_balance"),
        ("manage-air-heat-balance", "stage", "heat_balance"),
        ("update-thermal-histories", "stage", "heat_balance"),
        ("report-surface-heat-balance", "stage", "heat_balance"),
        (
            "manage-zone-air-updates",
            "ZoneTempPredictorCorrector::PredictStep",
            "zone_temp_predictor_corrector",
        ),
        (
            "manage-zone-air-updates",
            "ZoneTempPredictorCorrector::CorrectStep",
            "zone_temp_predictor_corrector",
        ),
        ("sim-purchased-air", "stage", "ideal_loads"),
        ("calc-purch-air-loads", "stage", "ideal_loads"),
        ("update-purchased-air", "stage", "ideal_loads"),
        ("report-purchased-air", "stage", "ideal_loads"),
    ]
}

fn source_order_stage_state_snapshots(plan: &ExecutionPlan, trace_level: TraceLevel) -> Vec<Value> {
    if !trace_level_enables_stage_snapshots(trace_level) {
        return Vec::new();
    }

    let mut snapshots = Vec::new();
    for (stage_name, substage, state_domain) in source_order_stage_state_snapshot_targets() {
        let Some((stage_index, stage)) = plan
            .stages
            .iter()
            .enumerate()
            .find(|(_, stage)| stage.name == *stage_name)
        else {
            continue;
        };
        let source_routine = plan
            .compatibility_stages
            .iter()
            .find(|compatibility_stage| compatibility_stage.stage_name == *stage_name)
            .map(|compatibility_stage| compatibility_stage.source_routine);
        for point in ["before", "after"] {
            snapshots.push(json!({
                "schema_version": 1,
                "stage_index": stage_index,
                "stage_kind": stage.kind.id(),
                "stage_name": stage.name,
                "source_routine": source_routine,
                "substage": substage,
                "point": point,
                "state_domain": state_domain,
                "trace_artifact_only": true,
                "state_observation": {
                    "step_count": stage.steps.len(),
                    "source_order_barrier": stage.kind.is_source_order_barrier(),
                    "capture_mode": "stage-boundary diagnostic snapshot",
                },
            }));
        }
    }
    snapshots
}

fn source_order_gate_summary(plan: &ExecutionPlan) -> SourceOrderGateSummary {
    let expected_source_order_stages = plan
        .expected_source_order_stage_ids()
        .into_iter()
        .map(str::to_string)
        .collect::<Vec<_>>();
    let actual_executed_source_order_stages = plan
        .actual_source_order_stage_ids()
        .into_iter()
        .map(str::to_string)
        .collect::<Vec<_>>();
    let matches = expected_source_order_stages == actual_executed_source_order_stages;
    SourceOrderGateSummary {
        expected_source_order_stages,
        actual_executed_source_order_stages,
        matches,
    }
}

fn execution_step_label(step: &ExecutionStep) -> String {
    match step {
        ExecutionStep::UpdateWeather => "UpdateWeather".to_string(),
        ExecutionStep::EvaluateSchedule(id) => format!("EvaluateSchedule({})", id.0),
        ExecutionStep::EvaluateZoneThermostat(id) => {
            format!("EvaluateZoneThermostat({})", id.0)
        }
        ExecutionStep::SolveZone(id) => format!("SolveZone({})", id.0),
        ExecutionStep::ManageZoneEquipment(id) => {
            format!("ManageZoneEquipment({})", id.0)
        }
        ExecutionStep::SimZoneEquipment(id) => format!("SimZoneEquipment({})", id.0),
        ExecutionStep::SimPurchasedAir(id) => format!("SimPurchasedAir({})", id.0),
        ExecutionStep::GetIdealLoadsAirSystem(id) => {
            format!("GetIdealLoadsAirSystem({})", id.0)
        }
        ExecutionStep::InitIdealLoadsAirSystem(id) => {
            format!("InitIdealLoadsAirSystem({})", id.0)
        }
        ExecutionStep::EvaluateIdealLoadsAirSystem(id) => {
            format!("EvaluateIdealLoadsAirSystem({})", id.0)
        }
        ExecutionStep::UpdateIdealLoadsAirSystem(id) => {
            format!("UpdateIdealLoadsAirSystem({})", id.0)
        }
        ExecutionStep::ReportIdealLoadsAirSystem(id) => {
            format!("ReportIdealLoadsAirSystem({})", id.0)
        }
        ExecutionStep::WriteOutput(id) => format!("WriteOutput({})", id.0),
    }
}

fn write_support_artifacts(
    output_dir: &Path,
    assessment: &SupportAssessment,
    diagnostics: &RunDiagnostics,
) -> Result<(), String> {
    write_json(&output_dir.join("support-assessment.json"), assessment)?;
    write_text(
        &output_dir.join("support-report.md"),
        &render_support_report(assessment),
    )?;
    write_json(&output_dir.join("diagnostics.json"), diagnostics)
}

fn prepare_runtime_inputs(
    config: &RunConfig,
    simulation_model: Option<&SimulationModel>,
    runtime_class: RuntimeClass,
) -> Result<PreparedRuntimeInputs, String> {
    let model = simulation_model.ok_or_else(|| "missing compiled simulation model".to_string())?;
    let (time_axis, weather_series) = if runtime_class_requires_weather(runtime_class) {
        let weather_path = config
            .weather_path
            .as_ref()
            .ok_or_else(|| "weather path is required for heat-balance runtime".to_string())?;
        let weather_file = load_epw_weather_file(weather_path)
            .map_err(|error| format!("failed to load EPW weather: {error}"))?;
        let time_axis = build_hourly_time_axis_with_weather_metadata(
            &model.typed,
            &weather_file.calendar_metadata,
        )
        .map_err(|error| format!("failed to build weather-aware time axis: {error}"))?;
        let environment_weather = select_epw_environment_weather(&weather_file, &time_axis)
            .map_err(|error| format!("failed to select EPW environment records: {error}"))?;
        let weather_series = precompute_weather_timestep_series(
            environment_weather.hourly_records(),
            time_axis.zone_timestep.timesteps_per_hour,
            time_axis.first_hour_interpolation_starting_values,
        );
        (time_axis, Some(weather_series))
    } else {
        (
            build_hourly_time_axis(&model.typed).map_err(|error| error.to_string())?,
            None,
        )
    };
    let sample_count = runtime_sample_count(
        config,
        &time_axis,
        runtime_class_requires_weather(runtime_class),
    )?;
    let schedule_series = precompute_schedule_value_series_for_time_axis(&model.typed, &time_axis);

    Ok(PreparedRuntimeInputs {
        sample_count,
        time_axis,
        schedule_series,
        weather_series,
    })
}

fn execute_rust_runtime(
    simulation_model: Option<&SimulationModel>,
    runtime_class: RuntimeClass,
    source_order_gate: SourceOrderGateSummary,
    runtime_inputs: &PreparedRuntimeInputs,
) -> Result<RustRuntimeResult, String> {
    let model = simulation_model.ok_or_else(|| "missing compiled simulation model".to_string())?;
    let sample_count = runtime_inputs.sample_count;
    match runtime_class {
        RuntimeClass::OneZoneHeatBalanceCompatibility
        | RuntimeClass::HeatBalanceZoneAirDiagnostic => {
            let weather_series = runtime_inputs.weather_series.as_ref().ok_or_else(|| {
                "weather records are required for heat-balance runtime".to_string()
            })?;
            let _runtime_time_axis_samples = runtime_inputs.time_axis.sample_count();
            let _runtime_precomputed_schedule_count = runtime_inputs.schedule_series.len();
            let options = HeatBalanceSimulationOptions::hourly_samples(sample_count);
            let simulation = simulate_heat_balance_zone_air_temperatures_with_weather_series(
                model,
                weather_series,
                options,
            )
            .map_err(|error| error.to_string())?;
            Ok(RustRuntimeResult {
                results: simulation.results,
                runtime_class,
                sample_count,
                source_order_gate,
            })
        }
        RuntimeClass::IdealLoadsNoOaSensibleCompatibility
        | RuntimeClass::IdealLoadsFiniteLimitCompatibility
        | RuntimeClass::IdealLoadsConstantShrCompatibility
        | RuntimeClass::IdealLoadsHumiditySelectedBranchesCompatibility
        | RuntimeClass::IdealLoadsOutdoorAirSelectedBranchesCompatibility
        | RuntimeClass::IdealLoadsMixedDeclaredCompatibility => {
            let simulation = simulate_ideal_loads_purchased_air_compat(
                model,
                IdealLoadsCompatibilityOptions::hourly_samples(sample_count),
            )
            .map_err(|error| error.to_string())?;
            Ok(RustRuntimeResult {
                results: simulation.results,
                runtime_class,
                sample_count,
                source_order_gate,
            })
        }
        RuntimeClass::IdealLoadsNodeStateProjection => {
            let projection = simulate_ideal_loads_node_state_projection(
                model,
                NodeStateProjectionOptions::hourly_samples(sample_count),
            )
            .map_err(|error| error.to_string())?;
            Ok(RustRuntimeResult {
                results: projection.results,
                runtime_class,
                sample_count,
                source_order_gate,
            })
        }
        RuntimeClass::None => Err("no runtime selected".to_string()),
    }
}

fn runtime_class_requires_weather(runtime_class: RuntimeClass) -> bool {
    matches!(
        runtime_class,
        RuntimeClass::OneZoneHeatBalanceCompatibility | RuntimeClass::HeatBalanceZoneAirDiagnostic
    )
}

fn runtime_sample_count(
    config: &RunConfig,
    time_axis: &TimeAxis,
    constrain_to_time_axis: bool,
) -> Result<usize, String> {
    let available_samples = time_axis.sample_count();
    let requested_samples = config.hours.unwrap_or(available_samples);
    if constrain_to_time_axis && requested_samples > available_samples {
        return Err(format!(
            "requested {requested_samples} runtime hours but the resolved time axis contains only {available_samples} hourly samples"
        ));
    }
    Ok(requested_samples)
}

fn write_runtime_artifacts(output_dir: &Path, results: &ResultStore) -> Result<(), String> {
    write_json(
        &output_dir.join("results").join("result-store.json"),
        &result_store_json(results),
    )?;
    let mut selected_results = ResultStore::new();
    for series in results
        .series
        .iter()
        .filter(|series| is_primary_compare_series(&series.variable_name))
    {
        selected_results.add_series(series.clone());
    }
    write_selected_outputs_csv(
        &output_dir.join("results").join("selected-outputs.csv"),
        &selected_results,
    )?;
    write_empty_meters_csv(&output_dir.join("results").join("meters.csv"))
}

fn run_requested_oracle_baseline(
    config: &RunConfig,
    prepared_input: &PreparedInput,
    rust_results: Option<&ResultStore>,
) -> Result<OracleBaselineSummary, String> {
    let oracle_paths =
        resolve_oracle_paths(config.oracle_root.as_deref()).map_err(|error| error.to_string())?;
    let output_requests = rust_results
        .map(output_requests_from_results)
        .unwrap_or_else(default_output_requests);
    run_oracle_baseline(
        &oracle_paths,
        &prepared_input.original_path,
        prepared_input.input_kind,
        config.weather_path.as_deref(),
        &config.output_dir.join("oracle"),
        &output_requests,
    )
    .map_err(|error| error.to_string())
}

fn output_requests_from_results(results: &ResultStore) -> Vec<OracleOutputRequest> {
    let mut requests = results
        .series
        .iter()
        .filter(|series| is_primary_compare_series(&series.variable_name))
        .map(|series| OracleOutputRequest::hourly(&series.key, &series.variable_name))
        .collect::<Vec<_>>();
    if requests.is_empty() {
        requests = default_output_requests();
    }
    requests
}

fn is_primary_compare_series(variable_name: &str) -> bool {
    matches!(
        variable_name,
        "Zone Mean Air Temperature"
            | "Site Outdoor Air Drybulb Temperature"
            | "Surface Inside Face Temperature"
            | "Surface Outside Face Temperature"
            | "System Node Temperature"
            | "System Node Humidity Ratio"
            | "System Node Mass Flow Rate"
    ) || variable_name.contains("Ideal Loads")
}

fn default_output_requests() -> Vec<OracleOutputRequest> {
    vec![
        OracleOutputRequest::hourly("*", "Zone Mean Air Temperature"),
        OracleOutputRequest::hourly("*", "Site Outdoor Air Drybulb Temperature"),
        OracleOutputRequest::hourly("*", "Schedule Value"),
    ]
}

fn compare_with_oracle(
    rust_results: &ResultStore,
    oracle_summary: &OracleBaselineSummary,
) -> Result<ComparisonSummary, String> {
    let tolerance = Tolerance::default();
    let mut series_summaries = Vec::new();
    let mut failed = false;
    for series in &rust_results.series {
        if !is_primary_compare_series(&series.variable_name) {
            continue;
        }
        match load_eso_time_series(&oracle_summary.eso_path, &series.key, &series.variable_name) {
            Ok(oracle_series) => {
                let oracle_samples = run_period_samples(&oracle_series.samples);
                let rust_samples = series
                    .values
                    .iter()
                    .copied()
                    .enumerate()
                    .map(|(index, value)| ep_compare::SeriesSample::indexed(index, value))
                    .collect::<Vec<_>>();
                let comparison =
                    compare_series_samples_v2(&oracle_samples, &rust_samples, tolerance);
                if comparison.status == SeriesComparisonStatus::Fail {
                    failed = true;
                }
                series_summaries.push(ComparisonSeriesSummary {
                    key: series.key.clone(),
                    variable_name: series.variable_name.clone(),
                    units: series.units.clone(),
                    status: comparison_status_label(comparison.status).to_string(),
                    alignment: alignment_label(comparison.alignment).to_string(),
                    oracle_samples: comparison.expected_samples,
                    rust_samples: comparison.observed_samples,
                    compared_samples: comparison.compared_samples,
                    max_abs_delta: comparison.max_abs_delta,
                    rmse_delta: comparison.rmse_delta,
                    max_rel_delta: comparison.max_rel_delta,
                    first_divergence: comparison.first_divergence.map(|divergence| {
                        json!({
                            "index": divergence.index,
                            "timestamp": divergence.timestamp,
                            "kind": format!("{:?}", divergence.kind),
                            "expected": divergence.expected,
                            "observed": divergence.observed,
                            "abs_delta": divergence.abs_delta,
                            "rel_delta": divergence.rel_delta,
                        })
                    }),
                });
            }
            Err(error) => {
                failed = true;
                series_summaries.push(ComparisonSeriesSummary {
                    key: series.key.clone(),
                    variable_name: series.variable_name.clone(),
                    units: series.units.clone(),
                    status: "missing-oracle-series".to_string(),
                    alignment: "none".to_string(),
                    oracle_samples: 0,
                    rust_samples: series.values.len(),
                    compared_samples: 0,
                    max_abs_delta: 0.0,
                    rmse_delta: 0.0,
                    max_rel_delta: 0.0,
                    first_divergence: Some(json!({
                        "kind": "MissingOracleSeries",
                        "message": error.to_string(),
                    })),
                });
            }
        }
    }

    Ok(ComparisonSummary {
        schema_version: 1,
        status: if failed { "fail" } else { "pass" }.to_string(),
        conformance_claim: false,
        oracle_version: oracle_summary.oracle_version.clone(),
        series: series_summaries,
    })
}

fn run_period_samples(samples: &[ep_compare::SeriesSample]) -> Vec<ep_compare::SeriesSample> {
    let filtered = samples
        .iter()
        .filter(|sample| {
            sample
                .timestamp
                .as_deref()
                .is_some_and(|timestamp| timestamp.to_ascii_uppercase().contains("ENV=RUN PERIOD"))
        })
        .cloned()
        .collect::<Vec<_>>();
    if filtered.is_empty() {
        samples.to_vec()
    } else {
        filtered
    }
}

fn comparison_status_label(status: SeriesComparisonStatus) -> &'static str {
    match status {
        SeriesComparisonStatus::Pass => "pass",
        SeriesComparisonStatus::Fail => "fail",
    }
}

fn alignment_label(alignment: SeriesAlignment) -> &'static str {
    match alignment {
        SeriesAlignment::Index => "index",
        SeriesAlignment::Timestamp => "timestamp",
    }
}

fn write_compare_artifacts(output_dir: &Path, summary: &ComparisonSummary) -> Result<(), String> {
    write_json(
        &output_dir.join("compare").join("compare-summary.json"),
        summary,
    )?;
    let mut report = String::new();
    report.push_str("# Oracle Compare Report\n\n");
    report.push_str(&format!("status: {}\n", summary.status));
    report.push_str("conformance_claim: false\n\n");
    report.push_str("| key | variable | status | oracle_samples | rust_samples | max_abs_delta | rmse_delta |\n");
    report.push_str("| --- | --- | --- | ---: | ---: | ---: | ---: |\n");
    for series in &summary.series {
        report.push_str(&format!(
            "| {} | {} | {} | {} | {} | {:.12} | {:.12} |\n",
            markdown_cell(&series.key),
            markdown_cell(&series.variable_name),
            series.status,
            series.oracle_samples,
            series.rust_samples,
            series.max_abs_delta,
            series.rmse_delta
        ));
    }
    write_text(
        &output_dir.join("compare").join("compare-report.md"),
        &report,
    )
}

fn merge_diagnostics(mut left: RunDiagnostics, right: RunDiagnostics) -> RunDiagnostics {
    left.diagnostics.extend(right.diagnostics);
    left
}

fn diagnostic_counts(diagnostics: &RunDiagnostics) -> Value {
    json!({
        "total": diagnostics.diagnostics.len(),
        "info": diagnostics.count_by_severity(Severity::Info),
        "warning": diagnostics.count_by_severity(Severity::Warning),
        "error": diagnostics.count_by_severity(Severity::Error),
    })
}

fn artifact_map(output_dir: &Path) -> Value {
    json!({
        "diagnostics_json": output_dir.join("diagnostics.json").display().to_string(),
        "eplusrs_err": output_dir.join("eplusrs.err").display().to_string(),
        "run_summary_json": output_dir.join("run-summary.json").display().to_string(),
        "support_assessment_json": output_dir.join("support-assessment.json").display().to_string(),
        "support_report_md": output_dir.join("support-report.md").display().to_string(),
        "original_idf": output_dir.join("input").join("original.idf").display().to_string(),
        "original_epjson": output_dir.join("input").join("original.epJSON").display().to_string(),
        "converted_epjson": output_dir.join("input").join("converted.epJSON").display().to_string(),
        "input_hashes_json": output_dir.join("input").join("input-hashes.json").display().to_string(),
        "raw_model_summary_json": output_dir.join("model").join("raw-model-summary.json").display().to_string(),
        "typed_model_summary_json": output_dir.join("model").join("typed-model-summary.json").display().to_string(),
        "graph_summary_json": output_dir.join("model").join("graph-summary.json").display().to_string(),
        "execution_plan_json": output_dir.join("model").join("execution-plan.json").display().to_string(),
        "source_order_stage_state_snapshots_json": output_dir.join("logs").join("source-order-stage-state-snapshots.json").display().to_string(),
        "result_store_json": output_dir.join("results").join("result-store.json").display().to_string(),
        "selected_outputs_csv": output_dir.join("results").join("selected-outputs.csv").display().to_string(),
        "meters_csv": output_dir.join("results").join("meters.csv").display().to_string(),
        "run_report_md": output_dir.join("reports").join("run-report.md").display().to_string(),
        "compatibility_boundary_md": output_dir.join("reports").join("compatibility-boundary.md").display().to_string(),
        "command_log": output_dir.join("logs").join("command.log").display().to_string(),
        "oracle_dir": output_dir.join("oracle").display().to_string(),
        "compare_summary_json": output_dir.join("compare").join("compare-summary.json").display().to_string(),
        "compare_report_md": output_dir.join("compare").join("compare-report.md").display().to_string(),
    })
}

fn markdown_cell(value: &str) -> String {
    value.replace('|', "\\|").replace('\n', " ")
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{
        artifact_map, ctf_split_trace_enabled, execution_stage_snapshots,
        full_surface_trace_opt_in, input_error_diagnostic_code, selected_trace_enabled,
        source_order_gate_summary, source_order_stage_state_snapshots,
        trace_level_enables_stage_snapshots,
    };
    use ep_compiler::compile_raw_model;
    use ep_raw_model::parse_epjson_str_with_idf_order;
    use ep_runtime::{
        DayType, EnergyPlusCompatibilityStage, ExecutionPlan, ExecutionStage, ExecutionStageKind,
        ExecutionStep, build_hourly_time_axis,
    };

    use crate::{TraceLevel, TraceSelection};

    #[test]
    fn input_error_diagnostic_code_preserves_converter_failures() {
        assert_eq!(
            input_error_diagnostic_code("ConvertInputFormatFailed: IDF conversion failed"),
            "ConvertInputFormatFailed"
        );
        assert_eq!(
            input_error_diagnostic_code("UnsupportedInputFormat: input must be .idf or .epJSON"),
            "UnsupportedInputFormat"
        );
        assert_eq!(
            input_error_diagnostic_code("missing weather file: weather.epw"),
            "MissingWeatherFile"
        );
        assert_eq!(
            input_error_diagnostic_code("failed to stage epJSON input: denied"),
            "RawModelParseFailed"
        );
    }

    #[test]
    fn compiled_idf_declaration_order_drives_later_special_day_overwrite()
    -> Result<(), Box<dyn std::error::Error>> {
        let epjson = r#"{
            "RunPeriod": {
                "Ordered Special Days": {
                    "begin_month": 6,
                    "begin_day_of_month": 15,
                    "begin_year": 2017,
                    "end_month": 6,
                    "end_day_of_month": 15,
                    "end_year": 2017,
                    "day_of_week_for_start_day": "Thursday",
                    "use_weather_file_holidays_and_special_days": "No",
                    "apply_weekend_holiday_rule": "No"
                }
            },
            "RunPeriodControl:SpecialDays": {
                "Zulu Earlier Holiday": {
                    "start_date": "6/15",
                    "duration": 1,
                    "special_day_type": "Holiday"
                },
                "Alpha Later Custom": {
                    "start_date": "6/15",
                    "duration": 1,
                    "special_day_type": "CustomDay2"
                }
            }
        }"#;
        let idf = r#"
            RunPeriod,
              Ordered Special Days,
              6,
              15,
              2017,
              6,
              15,
              2017,
              Thursday;
            RunPeriodControl:SpecialDays,
              Zulu Earlier Holiday,
              6/15,
              1,
              Holiday;
            RunPeriodControl:SpecialDays,
              Alpha Later Custom,
              6/15,
              1,
              CustomDay2;
        "#;
        let raw_model = parse_epjson_str_with_idf_order(epjson, idf)?;
        let compile_result = compile_raw_model(&raw_model);
        let Some(model) = compile_result.model else {
            return Err(std::io::Error::other(
                "expected declaration-ordered special-day model to compile",
            )
            .into());
        };

        let axis = build_hourly_time_axis(&model)?;
        assert_eq!(axis.special_days.resolved_days.len(), 2);
        assert_eq!(
            axis.special_days.resolved_days[0].name,
            "ZULU EARLIER HOLIDAY"
        );
        assert_eq!(
            axis.special_days.resolved_days[1].name,
            "ALPHA LATER CUSTOM"
        );
        assert!(axis.points.iter().all(|point| {
            point.day_type == DayType::CustomDay2
                && point.special_day_type == Some(DayType::CustomDay2)
        }));
        Ok(())
    }

    #[test]
    fn source_order_gate_summary_detects_stage_mismatch() {
        let expected = EnergyPlusCompatibilityStage {
            kind: ExecutionStageKind::GetHeatBalanceInput,
            stage_name: "get-heat-balance-input",
            source_file: "src/EnergyPlus/HeatBalanceManager.cc",
            source_routine: "GetHeatBalanceInput",
        };
        let actual = ExecutionStage::new(
            ExecutionStageKind::InitHeatBalance,
            "init-heat-balance",
            Vec::new(),
        );
        let plan = ExecutionPlan::new(vec![actual], vec![expected]);

        let gate = source_order_gate_summary(&plan);

        assert!(!gate.matches);
        assert_eq!(
            gate.expected_source_order_stages,
            vec!["get-heat-balance-input"]
        );
        assert_eq!(
            gate.actual_executed_source_order_stages,
            vec!["init-heat-balance"]
        );
    }

    #[test]
    fn trace_level_controls_metadata_only_stage_snapshots() {
        let expected = EnergyPlusCompatibilityStage {
            kind: ExecutionStageKind::GetHeatBalanceInput,
            stage_name: "get-heat-balance-input",
            source_file: "src/EnergyPlus/HeatBalanceManager.cc",
            source_routine: "GetHeatBalanceInput",
        };
        let stage = ExecutionStage::new(
            ExecutionStageKind::GetHeatBalanceInput,
            "get-heat-balance-input",
            vec![ExecutionStep::UpdateWeather],
        );
        let plan = ExecutionPlan::new(vec![stage], vec![expected]);

        assert!(!trace_level_enables_stage_snapshots(TraceLevel::Normal));
        assert!(trace_level_enables_stage_snapshots(TraceLevel::Detailed));
        assert!(trace_level_enables_stage_snapshots(TraceLevel::Debug));
        assert!(execution_stage_snapshots(&plan, TraceLevel::Normal).is_empty());

        let snapshots = execution_stage_snapshots(&plan, TraceLevel::Detailed);
        assert_eq!(snapshots.len(), 1);
        assert_eq!(
            snapshots[0]["kind"].as_str(),
            Some("get_heat_balance_input")
        );
        assert_eq!(
            snapshots[0]["name"].as_str(),
            Some("get-heat-balance-input")
        );
        assert_eq!(snapshots[0]["source_order_barrier"].as_bool(), Some(true));
        assert_eq!(snapshots[0]["step_count"].as_u64(), Some(1));
        assert_eq!(snapshots[0]["steps"][0].as_str(), Some("UpdateWeather"));
    }

    #[test]
    fn selected_trace_requires_explicit_surface_or_node_names() {
        let empty = TraceSelection::default();
        let mut selected = TraceSelection::default();
        selected.push_surface("FLOOR");
        selected.push_node("ZONE ONE INLET");

        assert!(!selected_trace_enabled(TraceLevel::Normal, &empty));
        assert!(!selected_trace_enabled(TraceLevel::Detailed, &empty));
        assert!(!selected_trace_enabled(TraceLevel::Debug, &empty));
        assert!(!selected_trace_enabled(TraceLevel::Normal, &selected));
        assert!(!selected_trace_enabled(TraceLevel::Detailed, &selected));
        assert!(selected_trace_enabled(TraceLevel::Surface, &selected));
        assert!(selected_trace_enabled(TraceLevel::Debug, &selected));
        assert!(ctf_split_trace_enabled(TraceLevel::Ctf, &selected));
        assert!(full_surface_trace_opt_in(TraceLevel::Surface, &selected));
    }

    #[test]
    fn detailed_trace_records_before_after_stage_state_snapshots() {
        let expected = EnergyPlusCompatibilityStage {
            kind: ExecutionStageKind::InitHeatBalance,
            stage_name: "init-heat-balance",
            source_file: "src/EnergyPlus/HeatBalanceManager.cc",
            source_routine: "InitHeatBalance",
        };
        let stage = ExecutionStage::new(
            ExecutionStageKind::InitHeatBalance,
            "init-heat-balance",
            vec![ExecutionStep::UpdateWeather],
        );
        let plan = ExecutionPlan::new(vec![stage], vec![expected]);

        assert!(source_order_stage_state_snapshots(&plan, TraceLevel::Normal).is_empty());

        let snapshots = source_order_stage_state_snapshots(&plan, TraceLevel::Detailed);
        assert_eq!(snapshots.len(), 2);
        assert_eq!(snapshots[0]["schema_version"].as_u64(), Some(1));
        assert_eq!(
            snapshots[0]["stage_name"].as_str(),
            Some("init-heat-balance")
        );
        assert_eq!(
            snapshots[0]["source_routine"].as_str(),
            Some("InitHeatBalance")
        );
        assert_eq!(snapshots[0]["point"].as_str(), Some("before"));
        assert_eq!(snapshots[1]["point"].as_str(), Some("after"));
        assert_eq!(snapshots[0]["state_domain"].as_str(), Some("heat_balance"));
        assert_eq!(snapshots[0]["trace_artifact_only"].as_bool(), Some(true));
    }

    #[test]
    fn artifact_map_lists_c4_output_contract_paths() {
        let artifacts = artifact_map(Path::new("out"));
        let root = Path::new("out");
        for (key, expected) in [
            ("diagnostics_json", root.join("diagnostics.json")),
            ("eplusrs_err", root.join("eplusrs.err")),
            ("run_summary_json", root.join("run-summary.json")),
            (
                "support_assessment_json",
                root.join("support-assessment.json"),
            ),
            ("support_report_md", root.join("support-report.md")),
            ("original_idf", root.join("input").join("original.idf")),
            (
                "original_epjson",
                root.join("input").join("original.epJSON"),
            ),
            (
                "converted_epjson",
                root.join("input").join("converted.epJSON"),
            ),
            (
                "input_hashes_json",
                root.join("input").join("input-hashes.json"),
            ),
            (
                "raw_model_summary_json",
                root.join("model").join("raw-model-summary.json"),
            ),
            (
                "typed_model_summary_json",
                root.join("model").join("typed-model-summary.json"),
            ),
            (
                "graph_summary_json",
                root.join("model").join("graph-summary.json"),
            ),
            (
                "execution_plan_json",
                root.join("model").join("execution-plan.json"),
            ),
            (
                "source_order_stage_state_snapshots_json",
                root.join("logs")
                    .join("source-order-stage-state-snapshots.json"),
            ),
            (
                "result_store_json",
                root.join("results").join("result-store.json"),
            ),
            (
                "selected_outputs_csv",
                root.join("results").join("selected-outputs.csv"),
            ),
            ("meters_csv", root.join("results").join("meters.csv")),
            ("run_report_md", root.join("reports").join("run-report.md")),
            (
                "compatibility_boundary_md",
                root.join("reports").join("compatibility-boundary.md"),
            ),
            ("command_log", root.join("logs").join("command.log")),
            ("oracle_dir", root.join("oracle")),
            (
                "compare_summary_json",
                root.join("compare").join("compare-summary.json"),
            ),
            (
                "compare_report_md",
                root.join("compare").join("compare-report.md"),
            ),
        ] {
            assert_eq!(artifacts[key], expected.display().to_string());
        }
    }
}
