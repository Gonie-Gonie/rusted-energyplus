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
use ep_model::{AutosizeOrNumber, SimulationModel, TypedModel};
use ep_oracle::default_oracle_release;
use ep_raw_model::{RawModel, load_epjson_file, load_epjson_file_with_idf_order};
use ep_runtime::{
    DIRECT_ZONE_PURCHASED_AIR_DEMAND_SOURCE, DirectZonePurchasedAirCoupledOptions, ExecutionPlan,
    ExecutionStep, HeatBalanceSimulationOptions, IDEAL_LOADS_FIXTURE_DEMAND_DIAGNOSTIC_SOURCE,
    IdealLoadsCompatibilityOptions, NodeStateProjectionOptions,
    PURCHASED_AIR_CALC_ENTRY_RESET_TARGETS, PURCHASED_AIR_CALC_ENTRY_SOURCE,
    PURCHASED_AIR_CALC_ENTRY_SOURCE_ORDER, PURCHASED_AIR_INIT_LIFECYCLE_SOURCE,
    PurchasedAirAvailabilityStatus, PurchasedAirCalcCoolingCapacityZeroFlowResetLifecycleSummary,
    PurchasedAirCalcCoolingDehumidificationFlowLifecycleSummary,
    PurchasedAirCalcCoolingEconomizerBodyLifecycleSummary,
    PurchasedAirCalcCoolingEconomizerConditionLifecycleSummary,
    PurchasedAirCalcCoolingEconomizerGuardLifecycleSummary,
    PurchasedAirCalcCoolingEntryGateLifecycleSummary,
    PurchasedAirCalcCoolingHumidificationFlowLifecycleSummary,
    PurchasedAirCalcCoolingMixedAirCallLifecycleSummary,
    PurchasedAirCalcCoolingOaMaxFlowBodyLifecycleSummary,
    PurchasedAirCalcCoolingOaMaxFlowGateLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntryLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitLifecycleSummary,
    PurchasedAirCalcCoolingSensibleFlowLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodyLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowMaximumLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardLifecycleSummary,
    PurchasedAirCalcEntryLifecycleSummary, PurchasedAirCalcMinimumOaPrefixLifecycleSummary,
    PurchasedAirHardSizeField, PurchasedAirHardSizeLegacyRoute, PurchasedAirInitDiagnosticKind,
    PurchasedAirInitLifecycleSummary, PurchasedAirInitTopologyDiagnosticKind,
    PurchasedAirInitTopologyDiagnosticSeverity, PurchasedAirInitTopologyError,
    PurchasedAirRecirculationSource, PurchasedAirSupplyTemperatureDiagnosticKind,
    PurchasedAirSupplyTemperatureInitialMessageApi, ResultStore, RuntimePrecomputedData,
    ScheduleCacheProfile, ScheduleSeriesCache, ScheduleSeriesIndexKind, TimeAxis,
    WeatherTimestepSeries, ZoneSensibleDemandInputKind,
    build_environment_time_axes_with_weather_metadata, build_hourly_time_axis,
    build_hourly_time_axis_with_weather_metadata, load_epw_weather_file, precompute_runtime_data,
    precompute_schedule_cache_for_environment_time_axis, precompute_schedule_cache_for_time_axis,
    precompute_weather_timestep_series, select_epw_environment_weather,
    simulate_direct_zone_purchased_air_coupled_heat_balance,
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

mod purchased_air_cooling_capacity_zero_flow_reset;
mod purchased_air_cooling_dehumidification_flow;
mod purchased_air_cooling_economizer_body;
mod purchased_air_cooling_economizer_condition;
mod purchased_air_cooling_economizer_guard;
mod purchased_air_cooling_entry_gate;
mod purchased_air_cooling_humidification_flow;
mod purchased_air_cooling_mixed_air_call;
mod purchased_air_cooling_oa_max_flow;
mod purchased_air_cooling_oa_max_flow_body;
mod purchased_air_cooling_positive_supply_capacity_limit_cp_air_assignment;
mod purchased_air_cooling_positive_supply_capacity_limit_guard;
mod purchased_air_cooling_positive_supply_capacity_limit_sensible_output_assignment;
mod purchased_air_cooling_positive_supply_capacity_limit_sensible_output_guard;
mod purchased_air_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment;
mod purchased_air_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment;
mod purchased_air_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment;
mod purchased_air_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit;
mod purchased_air_cooling_positive_supply_cp_air_assignment;
mod purchased_air_cooling_positive_supply_enthalpy_assignment;
mod purchased_air_cooling_positive_supply_humidity_ratio_mixed_air_assignment;
mod purchased_air_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry;
mod purchased_air_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment;
mod purchased_air_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment;
mod purchased_air_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment;
mod purchased_air_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment;
mod purchased_air_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case;
mod purchased_air_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch;
mod purchased_air_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment;
mod purchased_air_cooling_positive_supply_temperature_assignment;
mod purchased_air_cooling_positive_supply_temperature_minimum_limit;
mod purchased_air_cooling_positive_supply_temperature_mixed_air_limit;
mod purchased_air_cooling_sensible_flow;
mod purchased_air_cooling_supply_mass_flow_ems_override_body;
mod purchased_air_cooling_supply_mass_flow_ems_override_guard;
mod purchased_air_cooling_supply_mass_flow_limit_body;
mod purchased_air_cooling_supply_mass_flow_limit_guard;
mod purchased_air_cooling_supply_mass_flow_maximum;
mod purchased_air_cooling_supply_mass_flow_positive_guard;
mod purchased_air_cooling_supply_mass_flow_very_small_guard;
mod purchased_air_cooling_supply_mass_flow_very_small_guard_body;
mod purchased_air_minimum_oa;

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
    schedule_cache_sample_count: usize,
    schedule_cache_profile: ScheduleCacheProfile,
    source_order_gate: SourceOrderGateSummary,
    zone_demand_source: Option<String>,
    fixture_demand_injection_used: Option<bool>,
    purchased_air_branch: Option<String>,
    recirculation_node: Option<String>,
    recirculation_state_source: Option<String>,
    actual_coupled_source_order: Option<Vec<String>>,
    purchased_air_coupling_call_count: Option<usize>,
    purchased_air_init_lifecycle: Option<PurchasedAirInitLifecycleSummary>,
    purchased_air_calc_entry_lifecycle: Option<PurchasedAirCalcEntryLifecycleSummary>,
    purchased_air_calc_minimum_oa_prefix_lifecycle:
        Option<PurchasedAirCalcMinimumOaPrefixLifecycleSummary>,
    purchased_air_calc_cooling_entry_gate_lifecycle:
        Option<PurchasedAirCalcCoolingEntryGateLifecycleSummary>,
    purchased_air_calc_cooling_oa_max_flow_gate_lifecycle:
        Option<PurchasedAirCalcCoolingOaMaxFlowGateLifecycleSummary>,
    purchased_air_calc_cooling_oa_max_flow_body_lifecycle:
        Option<PurchasedAirCalcCoolingOaMaxFlowBodyLifecycleSummary>,
    purchased_air_calc_cooling_economizer_guard_lifecycle:
        Option<PurchasedAirCalcCoolingEconomizerGuardLifecycleSummary>,
    purchased_air_calc_cooling_economizer_condition_lifecycle:
        Option<PurchasedAirCalcCoolingEconomizerConditionLifecycleSummary>,
    purchased_air_calc_cooling_economizer_body_lifecycle:
        Option<PurchasedAirCalcCoolingEconomizerBodyLifecycleSummary>,
    purchased_air_calc_cooling_sensible_flow_lifecycle:
        Option<PurchasedAirCalcCoolingSensibleFlowLifecycleSummary>,
    purchased_air_calc_cooling_dehumidification_flow_lifecycle:
        Option<PurchasedAirCalcCoolingDehumidificationFlowLifecycleSummary>,
    purchased_air_calc_cooling_humidification_flow_lifecycle:
        Option<PurchasedAirCalcCoolingHumidificationFlowLifecycleSummary>,
    purchased_air_calc_cooling_capacity_zero_flow_reset_lifecycle:
        Option<PurchasedAirCalcCoolingCapacityZeroFlowResetLifecycleSummary>,
    purchased_air_calc_cooling_supply_mass_flow_maximum_lifecycle:
        Option<PurchasedAirCalcCoolingSupplyMassFlowMaximumLifecycleSummary>,
    purchased_air_calc_cooling_supply_mass_flow_ems_override_guard_lifecycle:
        Option<PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardLifecycleSummary>,
    purchased_air_calc_cooling_supply_mass_flow_ems_override_body_lifecycle:
        Option<PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyLifecycleSummary>,
    purchased_air_calc_cooling_supply_mass_flow_limit_guard_lifecycle:
        Option<PurchasedAirCalcCoolingSupplyMassFlowLimitGuardLifecycleSummary>,
    purchased_air_calc_cooling_supply_mass_flow_limit_body_lifecycle:
        Option<PurchasedAirCalcCoolingSupplyMassFlowLimitBodyLifecycleSummary>,
    purchased_air_calc_cooling_supply_mass_flow_very_small_guard_lifecycle:
        Option<PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardLifecycleSummary>,
    purchased_air_calc_cooling_supply_mass_flow_very_small_guard_body_lifecycle:
        Option<PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyLifecycleSummary>,
    purchased_air_calc_cooling_mixed_air_call_lifecycle:
        Option<PurchasedAirCalcCoolingMixedAirCallLifecycleSummary>,
    purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle:
        Option<PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardLifecycleSummary>,
    purchased_air_calc_cooling_positive_supply_cp_air_assignment_lifecycle:
        Option<PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentLifecycleSummary>,
    purchased_air_calc_cooling_positive_supply_temperature_assignment_lifecycle:
        Option<PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentLifecycleSummary>,
    purchased_air_calc_cooling_positive_supply_temperature_minimum_limit_lifecycle:
        Option<PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitLifecycleSummary>,
    purchased_air_calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle:
        Option<PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitLifecycleSummary>,
    purchased_air_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle:
        Option<
            PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentLifecycleSummary,
        >,
    purchased_air_calc_cooling_positive_supply_enthalpy_assignment_lifecycle:
        Option<PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentLifecycleSummary>,
    purchased_air_calc_cooling_positive_supply_capacity_limit_guard_lifecycle:
        Option<PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardLifecycleSummary>,
    purchased_air_calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle:
        Option<PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentLifecycleSummary>,
    purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle:
        Option<
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentLifecycleSummary,
        >,
    purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle:
        Option<
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardLifecycleSummary,
        >,
    purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_lifecycle:
        Option<
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentLifecycleSummary,
        >,
    purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_lifecycle:
        Option<
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentLifecycleSummary,
        >,
    purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_lifecycle:
        Option<
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentLifecycleSummary,
        >,
    purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle:
        Option<
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitLifecycleSummary,
        >,
    purchased_air_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle:
        Option<
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentLifecycleSummary,
        >,
    purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_lifecycle:
        Option<
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchLifecycleSummary,
        >,
    purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_lifecycle:
        Option<
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseLifecycleSummary,
        >,
    purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_lifecycle:
        Option<
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntryLifecycleSummary,
        >,
    purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_lifecycle:
        Option<
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentLifecycleSummary,
        >,
    purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_lifecycle:
        Option<
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentLifecycleSummary,
        >,
    purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_lifecycle:
        Option<
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignmentLifecycleSummary,
        >,
    purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_lifecycle:
        Option<
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentLifecycleSummary,
        >,
}

struct PreparedRuntimeInputs {
    sample_count: usize,
    time_axis: TimeAxis,
    schedule_cache: ScheduleSeriesCache,
    zone_timestep_schedule_cache: Option<ScheduleSeriesCache>,
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
        match validate_runtime_selection(assessment.run_result_state, assessment.runtime_class)
            .and_then(|()| {
                execute_rust_runtime(
                    simulation_model.as_ref(),
                    assessment.runtime_class,
                    source_order_gate,
                    &runtime_inputs,
                )
            })
            .and_then(|result| {
                validate_runtime_demand_provenance(
                    assessment.run_result_state,
                    &result,
                    simulation_model.as_ref(),
                )?;
                Ok(result)
            }) {
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

/// Deterministic cache storage/index metadata, not a runtime timing measurement.
fn schedule_cache_json(sample_count: usize, profile: ScheduleCacheProfile) -> Value {
    let index_kind = match profile.index_kind {
        ScheduleSeriesIndexKind::DenseIdentity => "dense_identity",
        ScheduleSeriesIndexKind::Sparse => "sparse",
    };
    json!({
        "sample_count": sample_count,
        "profile": {
            "scalar_series_count": profile.scalar_series_count,
            "dense_series_count": profile.dense_series_count,
            "logical_sample_count": profile.logical_sample_count,
            "allocated_dense_sample_count": profile.allocated_dense_sample_count,
            "index_kind": index_kind,
            "ambiguous_id_count": profile.ambiguous_id_count,
        },
    })
}

fn purchased_air_init_lifecycle_json(lifecycle: &PurchasedAirInitLifecycleSummary) -> Value {
    let declared_system_order: Vec<_> = lifecycle
        .declared_system_order
        .iter()
        .map(|system| system.0)
        .collect();
    let equipment_list_scan_order: Vec<_> = lifecycle
        .equipment_list_scan_order
        .iter()
        .map(|system| system.0)
        .collect();
    let equipment_list_diagnostics: Vec<_> = lifecycle
        .equipment_list_diagnostics
        .iter()
        .map(|diagnostic| {
            let kind = match diagnostic.kind {
                PurchasedAirInitDiagnosticKind::EquipmentListMembershipMissing => {
                    "equipment_list_membership_missing"
                }
            };
            json!({
                "system": diagnostic.system.0,
                "scan_ordinal": diagnostic.scan_ordinal,
                "kind": kind,
            })
        })
        .collect();
    let topology_diagnostics: Vec<_> = lifecycle
        .topology_diagnostics
        .iter()
        .map(|diagnostic| {
            let severity = match diagnostic.severity {
                PurchasedAirInitTopologyDiagnosticSeverity::Severe => "severe",
                PurchasedAirInitTopologyDiagnosticSeverity::Warning => "warning",
                PurchasedAirInitTopologyDiagnosticSeverity::Fatal => "fatal",
            };
            let kind = match diagnostic.kind {
                PurchasedAirInitTopologyDiagnosticKind::SupplyNodeNotZoneInlet => {
                    "supply_node_not_zone_inlet"
                }
                PurchasedAirInitTopologyDiagnosticKind::ExhaustNodeNotZoneExhaust => {
                    "exhaust_node_not_zone_exhaust"
                }
                PurchasedAirInitTopologyDiagnosticKind::MultipleReturnNodesUnassigned => {
                    "multiple_return_nodes_unassigned"
                }
                PurchasedAirInitTopologyDiagnosticKind::NoRecirculationNode => {
                    "no_recirculation_node"
                }
                PurchasedAirInitTopologyDiagnosticKind::EconomizerWithoutCoolingFlowLimit => {
                    "economizer_without_cooling_flow_limit"
                }
            };
            json!({
                "system": diagnostic.system.0,
                "ordinal": diagnostic.ordinal,
                "severity": severity,
                "kind": kind,
            })
        })
        .collect();
    let recirculation_source = lifecycle.recirculation_source.map(|source| match source {
        PurchasedAirRecirculationSource::ConfiguredZoneExhaust => "configured_zone_exhaust",
        PurchasedAirRecirculationSource::SingleZoneReturn => "single_zone_return",
        PurchasedAirRecirculationSource::MultipleZoneReturnsUnassigned => {
            "multiple_zone_returns_unassigned"
        }
    });
    let topology_failure = lifecycle.topology_failure.map(|failure| match failure {
        PurchasedAirInitTopologyError::SupplyNodeNotZoneInlet { .. } => {
            "supply_node_not_zone_inlet"
        }
        PurchasedAirInitTopologyError::NoRecirculationNode { .. } => "no_recirculation_node",
    });
    let supply_temperature_diagnostics: Vec<_> = lifecycle
        .supply_temperature_diagnostics
        .iter()
        .map(|diagnostic| {
            let kind = match diagnostic.kind {
                PurchasedAirSupplyTemperatureDiagnosticKind::CoolingMinimumAboveSetpoint => {
                    "cooling_minimum_above_setpoint"
                }
                PurchasedAirSupplyTemperatureDiagnosticKind::HeatingMaximumBelowSetpoint => {
                    "heating_maximum_below_setpoint"
                }
            };
            let initial_message_api = match diagnostic.initial_message_api {
                PurchasedAirSupplyTemperatureInitialMessageApi::ShowSevereError => {
                    "show_severe_error"
                }
                PurchasedAirSupplyTemperatureInitialMessageApi::ShowSevereMessage => {
                    "show_severe_message"
                }
            };
            json!({
                "system": diagnostic.system.0,
                "registry_registration_ordinal": diagnostic.registry_registration_ordinal,
                "first_init_call_ordinal": diagnostic.first_init_call_ordinal,
                "last_init_call_ordinal": diagnostic.last_init_call_ordinal,
                "source_order_ordinal": diagnostic.source_order_ordinal,
                "kind": kind,
                "recurring_index": diagnostic.recurring_index,
                "first_detailed_diagnostic_count": diagnostic.first_detailed_diagnostic_count,
                "initial_message_api": initial_message_api,
                "first_detail_primary_message_count": diagnostic.first_detail_primary_message_count,
                "first_detail_continue_message_count": diagnostic.first_detail_continue_message_count,
                "first_detail_timestamp_count": diagnostic.first_detail_timestamp_count,
                "recurring_severe_call_count": diagnostic.recurring_severe_call_count,
                "characterized_severe_error_count_increment": diagnostic.characterized_severe_error_count_increment,
                "latest_supply_temperature_c": diagnostic.latest_supply_temperature_c,
                "latest_thermostat_setpoint_c": diagnostic.latest_thermostat_setpoint_c,
                "recurring_minimum_c": diagnostic.recurring_minimum_c,
                "recurring_maximum_c": diagnostic.recurring_maximum_c,
                "temperature_unit": diagnostic.temperature_unit,
            })
        })
        .collect();
    let recurring_index_json = |index: usize| {
        (index > 0)
            .then_some(index)
            .map_or(Value::Null, Value::from)
    };
    let sized_value_json = |value: Option<AutosizeOrNumber>| match value {
        Some(AutosizeOrNumber::Value(value)) => json!(value),
        Some(AutosizeOrNumber::Autosize) => json!("autosize"),
        None => Value::Null,
    };
    let sized_limits = lifecycle.sized_limits.map(|limits| {
        json!({
            "maximum_heating_air_flow_rate_m3_per_s": sized_value_json(
                limits.maximum_heating_air_flow_rate_m3_per_s
            ),
            "maximum_sensible_heating_capacity_w": sized_value_json(
                limits.maximum_sensible_heating_capacity_w
            ),
            "maximum_cooling_air_flow_rate_m3_per_s": sized_value_json(
                limits.maximum_cooling_air_flow_rate_m3_per_s
            ),
            "maximum_total_cooling_capacity_w": sized_value_json(
                limits.maximum_total_cooling_capacity_w
            ),
        })
    });
    let sizing_outcome = lifecycle.sizing_outcome.map(|outcome| {
        let route = match outcome.route {
            PurchasedAirHardSizeLegacyRoute::NoCurrentZoneEquipment => "no_current_zone_equipment",
            PurchasedAirHardSizeLegacyRoute::DirectHardSizedNoSizingRun => {
                "direct_hard_sized_no_sizing_run"
            }
        };
        let fields: Vec<_> = outcome
            .fields
            .iter()
            .flatten()
            .map(|field| {
                let name = match field.field {
                    PurchasedAirHardSizeField::MaximumHeatingAirFlowRate => {
                        "maximum_heating_air_flow_rate_m3_per_s"
                    }
                    PurchasedAirHardSizeField::MaximumSensibleHeatingCapacity => {
                        "maximum_sensible_heating_capacity_w"
                    }
                    PurchasedAirHardSizeField::MaximumCoolingAirFlowRate => {
                        "maximum_cooling_air_flow_rate_m3_per_s"
                    }
                    PurchasedAirHardSizeField::MaximumTotalCoolingCapacity => {
                        "maximum_total_cooling_capacity_w"
                    }
                };
                json!({
                    "field": name,
                    "child_sizer_called": field.child_sizer_called,
                    "object_writeback": field.object_writeback,
                    "local_design_value": field.local_design_value,
                    "child_user_report_records": field.child_user_report_records,
                    "outer_report_records": field.outer_report_records,
                    "child_sizing_label_unit": field.child_sizing_label_unit,
                })
            })
            .collect();
        json!({
            "route": route,
            "entry_fan_flags_cleared": outcome.entry_fan_flags_cleared,
            "fields": fields,
        })
    });
    let mut value = json!({
        "source": lifecycle.source,
        "flags": {
            "state_machine_used": lifecycle.flags.state_machine_used,
            "one_time_checked": lifecycle.flags.one_time_checked,
            "topology_ready": lifecycle.flags.topology_ready,
            "environment_initialized": lifecycle.flags.environment_initialized,
            "environment_initialization_needed": lifecycle.flags.environment_initialization_needed,
            "sizing_checked": lifecycle.flags.sizing_checked,
            "equipment_list_checked": lifecycle.flags.equipment_list_checked,
            "return_plenum_inactive": lifecycle.flags.return_plenum_inactive,
        },
        "module_initialization_count": lifecycle.module_initialization_count,
        "equipment_list_check_count": lifecycle.equipment_list_check_count,
        "declared_system_order": declared_system_order,
        "equipment_list_scan_order": equipment_list_scan_order,
        "equipment_list_scanned_unit_count": lifecycle.equipment_list_scanned_unit_count,
        "equipment_list_missing_unit_count": lifecycle.equipment_list_missing_unit_count,
        "equipment_list_diagnostics": equipment_list_diagnostics,
        "equipment_list_scan_ordinal": lifecycle.equipment_list_scan_ordinal,
        "first_matching_equipment_list": lifecycle.first_matching_equipment_list.map(|list| list.0),
        "equipment_list_membership_found": lifecycle.equipment_list_membership_found,
        "controlled_zone": lifecycle.controlled_zone.map(|zone| zone.0),
        "equipment_list": lifecycle.equipment_list.map(|list| list.0),
        "supply_node": lifecycle.supply_node.map(|node| node.0),
        "recirculation_node": lifecycle.recirculation_node.map(|node| node.0),
        "recirculation_source": recirculation_source,
        "rejected_exhaust_node": lifecycle.rejected_exhaust_node.map(|node| node.0),
        "reported_first_return_node": lifecycle.reported_first_return_node.map(|node| node.0),
        "topology_diagnostics": topology_diagnostics,
        "topology_failure": topology_failure,
        "init_call_count": lifecycle.init_call_count,
        "one_time_initialization_count": lifecycle.one_time_initialization_count,
        "topology_completion_count": lifecycle.topology_completion_count,
        "sizing_attempt_count": lifecycle.sizing_attempt_count,
        "sizing_check_count": lifecycle.sizing_check_count,
        "sized_limits": sized_limits,
        "sizing_outcome": sizing_outcome,
        "environment_initialization_count": lifecycle.environment_initialization_count,
        "environment_rearm_count": lifecycle.environment_rearm_count,
        "maximum_heating_air_mass_flow_rate_kg_per_s": lifecycle.maximum_heating_air_mass_flow_rate_kg_per_s,
        "maximum_cooling_air_mass_flow_rate_kg_per_s": lifecycle.maximum_cooling_air_mass_flow_rate_kg_per_s,
        "standard_air_density_kg_per_m3": lifecycle.standard_air_density_kg_per_m3,
        "cooling_supply_temperature_warning_count": lifecycle.cooling_supply_temperature_warning_count,
        "heating_supply_temperature_warning_count": lifecycle.heating_supply_temperature_warning_count,
        "economizer_flow_limit_warning_count": lifecycle.economizer_flow_limit_warning_count,
    });
    if let Value::Object(object) = &mut value {
        object.insert(
            "supply_temperature_diagnostic_registry".to_string(),
            json!({
                "registered_recurring_diagnostic_count": lifecycle.supply_temperature_registered_recurring_diagnostic_count,
                "event_count": lifecycle.supply_temperature_diagnostic_event_count,
                "characterized_severe_error_count_increment": lifecycle.supply_temperature_characterized_severe_error_count_increment,
                "cooling_error_index": recurring_index_json(lifecycle.cooling_supply_temperature_error_index),
                "heating_error_index": recurring_index_json(lifecycle.heating_supply_temperature_error_index),
                "cooling_first_diagnostic_count": lifecycle.cooling_supply_temperature_first_diagnostic_count,
                "heating_first_diagnostic_count": lifecycle.heating_supply_temperature_first_diagnostic_count,
                "identities": supply_temperature_diagnostics,
            }),
        );
    }
    value
}

fn purchased_air_calc_entry_lifecycle_json(
    lifecycle: &PurchasedAirCalcEntryLifecycleSummary,
) -> Value {
    let availability_status = |status| match status {
        PurchasedAirAvailabilityStatus::Invalid => "invalid",
        PurchasedAirAvailabilityStatus::NoAction => "no_action",
        PurchasedAirAvailabilityStatus::ForceOff => "force_off",
        PurchasedAirAvailabilityStatus::CycleOn => "cycle_on",
        PurchasedAirAvailabilityStatus::CycleOnZoneFansOnly => "cycle_on_zone_fans_only",
    };
    let demand_kind = |kind| match kind {
        ZoneSensibleDemandInputKind::ActiveLoadSplitCompatibility => {
            "active_load_split_compatibility"
        }
        ZoneSensibleDemandInputKind::SourceSetpointThresholds => "source_setpoint_thresholds",
    };
    let latest = lifecycle.state.latest.map(|snapshot| {
        json!({
            "source": snapshot.source,
            "system": snapshot.system.0,
            "call_ordinal": snapshot.call_ordinal,
            "source_order": snapshot.source_order,
            "controlled_zone": snapshot.controlled_zone.0,
            "supply_node": snapshot.supply_node.0,
            "zone_node": snapshot.zone_node.0,
            "outdoor_air_node": snapshot.outdoor_air_node.map(|node| node.0),
            "recirculation_node": snapshot.recirculation_node.0,
            "reset": {
                "targets": PURCHASED_AIR_CALC_ENTRY_RESET_TARGETS,
                "field_count": PURCHASED_AIR_CALC_ENTRY_RESET_TARGETS.len(),
                "all_zero": snapshot.reset.all_zero(),
                "supply_mass_flow_rate_kg_per_s": snapshot.reset.supply_mass_flow_rate_kg_per_s,
                "outdoor_air_mass_flow_rate_kg_per_s": snapshot.reset.outdoor_air_mass_flow_rate_kg_per_s,
                "minimum_outdoor_air_mass_flow_rate_kg_per_s": snapshot.reset.minimum_outdoor_air_mass_flow_rate_kg_per_s,
                "economizer_active_time_hours": snapshot.reset.economizer_active_time_hours,
                "heat_recovery_active_time_hours": snapshot.reset.heat_recovery_active_time_hours,
                "system_output_provided_w": snapshot.reset.system_output_provided_w,
                "moisture_output_provided_kg_per_s": snapshot.reset.moisture_output_provided_kg_per_s,
                "cooling_sensible_output_w": snapshot.reset.cooling_sensible_output_w,
                "cooling_latent_output_w": snapshot.reset.cooling_latent_output_w,
                "cooling_total_output_w": snapshot.reset.cooling_total_output_w,
                "heating_sensible_output_w": snapshot.reset.heating_sensible_output_w,
                "latent_output_w": snapshot.reset.latent_output_w,
            },
            "demand": {
                "zone": snapshot.demand.zone.0,
                "sensible_input_kind": demand_kind(snapshot.demand.sensible_input_kind),
                "remaining_output_req_to_heat_sp_w": snapshot.demand.remaining_output_req_to_heat_sp_w,
                "remaining_output_req_to_cool_sp_w": snapshot.demand.remaining_output_req_to_cool_sp_w,
            },
            "unit_defaulted_on": snapshot.unit_defaulted_on,
            "economizer_defaulted_on": snapshot.economizer_defaulted_on,
            "availability_manager_read_site_visited": snapshot.availability_manager_read_site_visited,
            "availability_manager_zone_written": snapshot.availability_manager_zone_written,
            "copied_availability_status": snapshot.copied_availability_status.map(availability_status),
            "force_off_applied": snapshot.force_off_applied,
            "overall_availability_read_site_visited": snapshot.overall_availability_read_site_visited,
            "heating_availability_read_site_visited": snapshot.heating_availability_read_site_visited,
            "cooling_availability_read_site_visited": snapshot.cooling_availability_read_site_visited,
            "overall_availability": snapshot.overall_availability,
            "heating_availability": snapshot.heating_availability,
            "cooling_availability": snapshot.cooling_availability,
            "unit_on": snapshot.unit_on,
            "heating_on": snapshot.heating_on,
            "cooling_on": snapshot.cooling_on,
            "unit_body_entered": snapshot.unit_body_entered,
        })
    });
    json!({
        "source": lifecycle.source,
        "system": lifecycle.state.system.0,
        "call_count": lifecycle.state.call_count,
        "reset_count": lifecycle.state.reset_count,
        "demand_read_count": lifecycle.state.demand_read_count,
        "availability_manager_read_count": lifecycle.state.availability_manager_read_count,
        "availability_manager_zone_write_count": lifecycle.state.availability_manager_zone_write_count,
        "availability_status_copy_count": lifecycle.state.availability_status_copy_count,
        "overall_availability_read_count": lifecycle.state.overall_availability_read_count,
        "heating_availability_read_count": lifecycle.state.heating_availability_read_count,
        "cooling_availability_read_count": lifecycle.state.cooling_availability_read_count,
        "force_off_count": lifecycle.state.force_off_count,
        "overall_schedule_off_count": lifecycle.state.overall_schedule_off_count,
        "unit_body_entry_count": lifecycle.state.unit_body_entry_count,
        "unit_off_count": lifecycle.state.unit_off_count,
        "heating_on_count": lifecycle.state.heating_on_count,
        "cooling_on_count": lifecycle.state.cooling_on_count,
        "availability_manager_zone": lifecycle.state.availability_manager_zone.map(|zone| zone.0),
        "availability_status": availability_status(lifecycle.state.availability_status),
        "minimum_outdoor_air_mass_flow_rate_kg_per_s": lifecycle.state.minimum_outdoor_air_mass_flow_rate_kg_per_s,
        "economizer_active_time_hours": lifecycle.state.economizer_active_time_hours,
        "heat_recovery_active_time_hours": lifecycle.state.heat_recovery_active_time_hours,
        "latest": latest,
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
            "schedule_cache": schedule_cache_json(
                result.schedule_cache_sample_count,
                result.schedule_cache_profile,
            ),
            "source_order_stages": result.source_order_gate.actual_executed_source_order_stages.clone(),
            "zone_demand_source": result.zone_demand_source.as_deref(),
            "fixture_demand_injection_used": result.fixture_demand_injection_used,
            "purchased_air_branch": result.purchased_air_branch.as_deref(),
            "recirculation_node": result.recirculation_node.as_deref(),
            "recirculation_state_source": result.recirculation_state_source.as_deref(),
            "actual_coupled_source_order": result.actual_coupled_source_order.as_deref(),
            "purchased_air_coupling_call_count": result.purchased_air_coupling_call_count,
            "purchased_air_init_lifecycle": result.purchased_air_init_lifecycle
                .as_ref()
                .map(purchased_air_init_lifecycle_json),
            "purchased_air_calc_entry_lifecycle": result.purchased_air_calc_entry_lifecycle
                .as_ref()
                .map(purchased_air_calc_entry_lifecycle_json),
            "purchased_air_calc_minimum_oa_prefix_lifecycle": result
                .purchased_air_calc_minimum_oa_prefix_lifecycle
                .as_ref()
                .map(purchased_air_minimum_oa::lifecycle_json),
            "purchased_air_calc_cooling_entry_gate_lifecycle": result
                .purchased_air_calc_cooling_entry_gate_lifecycle
                .as_ref()
                .map(purchased_air_cooling_entry_gate::lifecycle_json),
            "purchased_air_calc_cooling_oa_max_flow_gate_lifecycle": result
                .purchased_air_calc_cooling_oa_max_flow_gate_lifecycle
                .as_ref()
                .map(purchased_air_cooling_oa_max_flow::lifecycle_json),
            "purchased_air_calc_cooling_oa_max_flow_body_lifecycle": result
                .purchased_air_calc_cooling_oa_max_flow_body_lifecycle
                .as_ref()
                .map(purchased_air_cooling_oa_max_flow_body::lifecycle_json),
            "purchased_air_calc_cooling_economizer_guard_lifecycle": result
                .purchased_air_calc_cooling_economizer_guard_lifecycle
                .as_ref()
                .map(purchased_air_cooling_economizer_guard::lifecycle_json),
            "purchased_air_calc_cooling_economizer_condition_lifecycle": result
                .purchased_air_calc_cooling_economizer_condition_lifecycle
                .as_ref()
                .map(purchased_air_cooling_economizer_condition::lifecycle_json),
            "purchased_air_calc_cooling_economizer_body_lifecycle": result
                .purchased_air_calc_cooling_economizer_body_lifecycle
                .as_ref()
                .map(purchased_air_cooling_economizer_body::lifecycle_json),
            "purchased_air_calc_cooling_sensible_flow_lifecycle": result
                .purchased_air_calc_cooling_sensible_flow_lifecycle
                .as_ref()
                .map(purchased_air_cooling_sensible_flow::lifecycle_json),
            "purchased_air_calc_cooling_dehumidification_flow_lifecycle": result
                .purchased_air_calc_cooling_dehumidification_flow_lifecycle
                .as_ref()
                .map(purchased_air_cooling_dehumidification_flow::lifecycle_json),
            "purchased_air_calc_cooling_humidification_flow_lifecycle": result
                .purchased_air_calc_cooling_humidification_flow_lifecycle
                .as_ref()
                .map(purchased_air_cooling_humidification_flow::lifecycle_json),
            "purchased_air_calc_cooling_capacity_zero_flow_reset_lifecycle": result
                .purchased_air_calc_cooling_capacity_zero_flow_reset_lifecycle
                .as_ref()
                .map(purchased_air_cooling_capacity_zero_flow_reset::lifecycle_json),
            "purchased_air_calc_cooling_supply_mass_flow_maximum_lifecycle": result
                .purchased_air_calc_cooling_supply_mass_flow_maximum_lifecycle
                .as_ref()
                .map(purchased_air_cooling_supply_mass_flow_maximum::lifecycle_json),
            "purchased_air_calc_cooling_supply_mass_flow_ems_override_guard_lifecycle": result
                .purchased_air_calc_cooling_supply_mass_flow_ems_override_guard_lifecycle
                .as_ref()
                .map(purchased_air_cooling_supply_mass_flow_ems_override_guard::lifecycle_json),
            "purchased_air_calc_cooling_supply_mass_flow_ems_override_body_lifecycle": result
                .purchased_air_calc_cooling_supply_mass_flow_ems_override_body_lifecycle
                .as_ref()
                .map(purchased_air_cooling_supply_mass_flow_ems_override_body::lifecycle_json),
            "purchased_air_calc_cooling_supply_mass_flow_limit_guard_lifecycle": result
                .purchased_air_calc_cooling_supply_mass_flow_limit_guard_lifecycle
                .as_ref()
                .map(purchased_air_cooling_supply_mass_flow_limit_guard::lifecycle_json),
            "purchased_air_calc_cooling_supply_mass_flow_limit_body_lifecycle": result
                .purchased_air_calc_cooling_supply_mass_flow_limit_body_lifecycle
                .as_ref()
                .map(purchased_air_cooling_supply_mass_flow_limit_body::lifecycle_json),
            "purchased_air_calc_cooling_supply_mass_flow_very_small_guard_lifecycle": result
                .purchased_air_calc_cooling_supply_mass_flow_very_small_guard_lifecycle
                .as_ref()
                .map(purchased_air_cooling_supply_mass_flow_very_small_guard::lifecycle_json),
            "purchased_air_calc_cooling_supply_mass_flow_very_small_guard_body_lifecycle": result
                .purchased_air_calc_cooling_supply_mass_flow_very_small_guard_body_lifecycle
                .as_ref()
                .map(purchased_air_cooling_supply_mass_flow_very_small_guard_body::lifecycle_json),
            "purchased_air_calc_cooling_mixed_air_call_lifecycle": result
                .purchased_air_calc_cooling_mixed_air_call_lifecycle
                .as_ref()
                .map(purchased_air_cooling_mixed_air_call::lifecycle_json),
            "purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle": result
                .purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle
                .as_ref()
                .map(purchased_air_cooling_supply_mass_flow_positive_guard::lifecycle_json),
            "purchased_air_calc_cooling_positive_supply_cp_air_assignment_lifecycle": result
                .purchased_air_calc_cooling_positive_supply_cp_air_assignment_lifecycle
                .as_ref()
                .map(purchased_air_cooling_positive_supply_cp_air_assignment::lifecycle_json),
            "purchased_air_calc_cooling_positive_supply_temperature_assignment_lifecycle": result
                .purchased_air_calc_cooling_positive_supply_temperature_assignment_lifecycle
                .as_ref()
                .map(purchased_air_cooling_positive_supply_temperature_assignment::lifecycle_json),
            "purchased_air_calc_cooling_positive_supply_temperature_minimum_limit_lifecycle": result
                .purchased_air_calc_cooling_positive_supply_temperature_minimum_limit_lifecycle
                .as_ref()
                .map(purchased_air_cooling_positive_supply_temperature_minimum_limit::lifecycle_json),
            "purchased_air_calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle": result
                .purchased_air_calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle
                .as_ref()
                .map(purchased_air_cooling_positive_supply_temperature_mixed_air_limit::lifecycle_json),
            "purchased_air_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle": result
                .purchased_air_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle
                .as_ref()
                .map(purchased_air_cooling_positive_supply_humidity_ratio_mixed_air_assignment::lifecycle_json),
            "purchased_air_calc_cooling_positive_supply_enthalpy_assignment_lifecycle": result
                .purchased_air_calc_cooling_positive_supply_enthalpy_assignment_lifecycle
                .as_ref()
                .map(purchased_air_cooling_positive_supply_enthalpy_assignment::lifecycle_json),
            "purchased_air_calc_cooling_positive_supply_capacity_limit_guard_lifecycle": result
                .purchased_air_calc_cooling_positive_supply_capacity_limit_guard_lifecycle
                .as_ref()
                .map(purchased_air_cooling_positive_supply_capacity_limit_guard::lifecycle_json),
            "purchased_air_calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle": result
                .purchased_air_calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle
                .as_ref()
                .map(purchased_air_cooling_positive_supply_capacity_limit_cp_air_assignment::lifecycle_json),
            "purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle": result
                .purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle
                .as_ref()
                .map(purchased_air_cooling_positive_supply_capacity_limit_sensible_output_assignment::lifecycle_json),
            "purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle": result
                .purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle
                .as_ref()
                .map(purchased_air_cooling_positive_supply_capacity_limit_sensible_output_guard::lifecycle_json),
            "purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_lifecycle": result
                .purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_lifecycle
                .as_ref()
                .map(purchased_air_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment::lifecycle_json),
            "purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_lifecycle": result
                .purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_lifecycle
                .as_ref()
                .map(purchased_air_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment::lifecycle_json),
            "purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_lifecycle": result
                .purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_lifecycle
                .as_ref()
                .map(purchased_air_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment::lifecycle_json),
            "purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle": result
                .purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle
                .as_ref()
                .map(purchased_air_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit::lifecycle_json),
            "purchased_air_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle": result
                .purchased_air_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle
                .as_ref()
                .map(purchased_air_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment::lifecycle_json),
            "purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_lifecycle": result
                .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_lifecycle
                .as_ref()
                .map(purchased_air_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch::lifecycle_json),
            "purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_lifecycle": result
                .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_lifecycle
                .as_ref()
                .map(purchased_air_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case::lifecycle_json),
            "purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_lifecycle": result
                .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_lifecycle
                .as_ref()
                .map(purchased_air_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry::lifecycle_json),
            "purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_lifecycle": result
                .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_lifecycle
                .as_ref()
                .map(purchased_air_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment::lifecycle_json),
            "purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_lifecycle": result
                .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_lifecycle
                .as_ref()
                .map(purchased_air_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment::lifecycle_json),
            "purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_lifecycle": result
                .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_lifecycle
                .as_ref()
                .map(purchased_air_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment::lifecycle_json),
            "purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_lifecycle": result
                .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_lifecycle
                .as_ref()
                .map(purchased_air_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment::lifecycle_json),
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
    let file_shading_generated_schedules = model
        .file_shading_schedule
        .as_ref()
        .map_or(0, |schedule| schedule.columns.len());
    BTreeMap::from([
        ("zones", model.zones.len()),
        ("zone_lists", model.zone_lists.len()),
        ("zone_groups", model.zone_groups.len()),
        (
            "zone_local_environments",
            model.zone_local_environments.len(),
        ),
        ("spaces", model.spaces.len()),
        (
            "authored_spaces",
            model
                .spaces
                .iter()
                .filter(|space| space.origin == ep_model::SpaceOrigin::Authored)
                .count(),
        ),
        ("space_lists", model.space_lists.len()),
        ("space_types", model.space_type_names.len()),
        ("surfaces", model.surfaces.len()),
        ("constructions", model.constructions.len()),
        ("materials", model.materials.len()),
        ("constant_schedules", model.schedules.len()),
        ("compact_schedules", model.compact_schedules.len()),
        ("file_schedules", model.file_schedules.len()),
        (
            "external_interface_schedules",
            model.external_interface_schedules.len(),
        ),
        (
            "external_interface_fmu_import_schedules",
            model.external_interface_fmu_import_schedules.len(),
        ),
        (
            "external_interface_fmu_export_schedules",
            model.external_interface_fmu_export_schedules.len(),
        ),
        (
            "file_shading_schedule_objects",
            usize::from(model.file_shading_schedule.is_some()),
        ),
        (
            "file_shading_generated_schedules",
            file_shading_generated_schedules,
        ),
        (
            "schedules",
            model.schedules.len()
                + model.compact_schedules.len()
                + model.file_schedules.len()
                + file_shading_generated_schedules
                + model.year_schedules.len()
                + model.external_interface_schedules.len()
                + model.external_interface_fmu_import_schedules.len()
                + model.external_interface_fmu_export_schedules.len(),
        ),
        (
            "day_schedules",
            model.day_schedules.len()
                + model.day_interval_schedules.len()
                + model.day_list_schedules.len(),
        ),
        ("day_hourly_schedules", model.day_schedules.len()),
        ("day_interval_schedules", model.day_interval_schedules.len()),
        ("day_list_schedules", model.day_list_schedules.len()),
        (
            "week_schedules",
            model.week_schedules.len() + model.week_compact_schedules.len(),
        ),
        ("week_daily_schedules", model.week_schedules.len()),
        ("week_compact_schedules", model.week_compact_schedules.len()),
        ("year_schedules", model.year_schedules.len()),
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
            "schedule_ids": model.typed.schedules.len()
                + model.typed.compact_schedules.len()
                + model.typed.file_schedules.len()
                + model
                    .typed
                    .file_shading_schedule
                    .as_ref()
                    .map_or(0, |schedule| schedule.columns.len())
                + model.typed.year_schedules.len()
                + model.typed.external_interface_schedules.len()
                + model.typed.external_interface_fmu_import_schedules.len()
                + model.typed.external_interface_fmu_export_schedules.len(),
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
    let (time_axis, weather_series, zone_timestep_schedule_cache) =
        if runtime_class_requires_weather(runtime_class) {
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
            let zone_timestep_schedule_cache =
                if runtime_class == RuntimeClass::IdealLoadsDirectZoneCoupledCompatibility {
                    let environment_axis = build_environment_time_axes_with_weather_metadata(
                        &model.typed,
                        &weather_file.calendar_metadata,
                    )
                    .map_err(|error| {
                        format!("failed to build zone-timestep environment axis: {error}")
                    })?
                    .into_iter()
                    .next()
                    .ok_or_else(|| "no zone-timestep environment axis was available".to_string())?;
                    Some(precompute_schedule_cache_for_environment_time_axis(
                        &model.typed,
                        &environment_axis,
                    ))
                } else {
                    None
                };
            (
                time_axis,
                Some(weather_series),
                zone_timestep_schedule_cache,
            )
        } else {
            (
                build_hourly_time_axis(&model.typed).map_err(|error| error.to_string())?,
                None,
                None,
            )
        };
    let sample_count = runtime_sample_count(
        config,
        &time_axis,
        runtime_class_requires_weather(runtime_class),
    )?;
    let schedule_cache = precompute_schedule_cache_for_time_axis(&model.typed, &time_axis);

    Ok(PreparedRuntimeInputs {
        sample_count,
        time_axis,
        schedule_cache,
        zone_timestep_schedule_cache,
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
    let schedule_cache_sample_count = runtime_inputs.schedule_cache.sample_count();
    let schedule_cache_profile = runtime_inputs.schedule_cache.profile();
    match runtime_class {
        RuntimeClass::OneZoneHeatBalanceCompatibility
        | RuntimeClass::HeatBalanceZoneAirDiagnostic => {
            let weather_series = runtime_inputs.weather_series.as_ref().ok_or_else(|| {
                "weather records are required for heat-balance runtime".to_string()
            })?;
            let _runtime_time_axis_samples = runtime_inputs.time_axis.sample_count();
            let _runtime_precomputed_schedule_count = runtime_inputs.schedule_cache.len();
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
                schedule_cache_sample_count,
                schedule_cache_profile,
                source_order_gate,
                zone_demand_source: None,
                fixture_demand_injection_used: None,
                purchased_air_branch: None,
                recirculation_node: None,
                recirculation_state_source: None,
                actual_coupled_source_order: None,
                purchased_air_coupling_call_count: None,
                purchased_air_init_lifecycle: None,
                purchased_air_calc_entry_lifecycle: None,
                purchased_air_calc_minimum_oa_prefix_lifecycle: None,
                purchased_air_calc_cooling_entry_gate_lifecycle: None,
                purchased_air_calc_cooling_oa_max_flow_gate_lifecycle: None,
                purchased_air_calc_cooling_oa_max_flow_body_lifecycle: None,
                purchased_air_calc_cooling_economizer_guard_lifecycle: None,
                purchased_air_calc_cooling_economizer_condition_lifecycle: None,
                purchased_air_calc_cooling_economizer_body_lifecycle: None,
                purchased_air_calc_cooling_sensible_flow_lifecycle: None,
                purchased_air_calc_cooling_dehumidification_flow_lifecycle: None,
                purchased_air_calc_cooling_humidification_flow_lifecycle: None,
                purchased_air_calc_cooling_capacity_zero_flow_reset_lifecycle: None,
                purchased_air_calc_cooling_supply_mass_flow_maximum_lifecycle: None,
                purchased_air_calc_cooling_supply_mass_flow_ems_override_guard_lifecycle: None,
                purchased_air_calc_cooling_supply_mass_flow_ems_override_body_lifecycle: None,
                purchased_air_calc_cooling_supply_mass_flow_limit_guard_lifecycle: None,
                purchased_air_calc_cooling_supply_mass_flow_limit_body_lifecycle: None,
                purchased_air_calc_cooling_supply_mass_flow_very_small_guard_lifecycle: None,
                purchased_air_calc_cooling_supply_mass_flow_very_small_guard_body_lifecycle: None,
                purchased_air_calc_cooling_mixed_air_call_lifecycle: None,
                purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle: None,
                purchased_air_calc_cooling_positive_supply_cp_air_assignment_lifecycle: None,
                purchased_air_calc_cooling_positive_supply_temperature_assignment_lifecycle: None,
                purchased_air_calc_cooling_positive_supply_temperature_minimum_limit_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_enthalpy_assignment_lifecycle: None,
                purchased_air_calc_cooling_positive_supply_capacity_limit_guard_lifecycle: None,
                purchased_air_calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_lifecycle:
                    None,
            })
        }
        RuntimeClass::IdealLoadsDirectZoneCoupledCompatibility => {
            let weather_series = runtime_inputs.weather_series.as_ref().ok_or_else(|| {
                "weather records are required for direct-zone coupled runtime".to_string()
            })?;
            let schedule_cache = runtime_inputs
                .zone_timestep_schedule_cache
                .as_ref()
                .ok_or_else(|| {
                    "zone-timestep schedule cache is required for direct-zone coupled runtime"
                        .to_string()
                })?;
            let schedule_cache_sample_count = schedule_cache.sample_count();
            let schedule_cache_profile = schedule_cache.profile();
            let simulation = simulate_direct_zone_purchased_air_coupled_heat_balance(
                model,
                weather_series,
                schedule_cache,
                DirectZonePurchasedAirCoupledOptions::hourly_samples(sample_count),
            )
            .map_err(|error| error.to_string())?;
            let zone_demand_source = Some(simulation.summary.zone_demand_source.to_string());
            let fixture_demand_injection_used =
                Some(simulation.summary.fixture_demand_injection_used);
            let purchased_air_branch = Some(simulation.summary.branch.label().to_string());
            let recirculation_node = Some(simulation.summary.return_node_name.clone());
            let recirculation_state_source =
                Some(simulation.summary.recirculation_state_source.to_string());
            let actual_coupled_source_order = Some(
                simulation
                    .summary
                    .actual_coupled_source_order
                    .iter()
                    .map(|stage| (*stage).to_string())
                    .collect(),
            );
            let purchased_air_coupling_call_count = Some(simulation.summary.coupling_call_count);
            let purchased_air_init_lifecycle = Some(simulation.summary.init_lifecycle);
            let purchased_air_calc_entry_lifecycle = Some(simulation.summary.calc_entry_lifecycle);
            let purchased_air_calc_minimum_oa_prefix_lifecycle =
                Some(simulation.summary.calc_minimum_oa_prefix_lifecycle);
            let purchased_air_calc_cooling_entry_gate_lifecycle =
                Some(simulation.summary.calc_cooling_entry_gate_lifecycle);
            let purchased_air_calc_cooling_oa_max_flow_gate_lifecycle =
                Some(simulation.summary.calc_cooling_oa_max_flow_gate_lifecycle);
            let purchased_air_calc_cooling_oa_max_flow_body_lifecycle =
                Some(simulation.summary.calc_cooling_oa_max_flow_body_lifecycle);
            let purchased_air_calc_cooling_economizer_guard_lifecycle =
                Some(simulation.summary.calc_cooling_economizer_guard_lifecycle);
            let purchased_air_calc_cooling_economizer_condition_lifecycle = Some(
                simulation
                    .summary
                    .calc_cooling_economizer_condition_lifecycle,
            );
            let purchased_air_calc_cooling_economizer_body_lifecycle =
                Some(simulation.summary.calc_cooling_economizer_body_lifecycle);
            let purchased_air_calc_cooling_sensible_flow_lifecycle =
                Some(simulation.summary.calc_cooling_sensible_flow_lifecycle);
            let purchased_air_calc_cooling_dehumidification_flow_lifecycle = Some(
                simulation
                    .summary
                    .calc_cooling_dehumidification_flow_lifecycle,
            );
            let purchased_air_calc_cooling_humidification_flow_lifecycle = Some(
                simulation
                    .summary
                    .calc_cooling_humidification_flow_lifecycle,
            );
            let purchased_air_calc_cooling_capacity_zero_flow_reset_lifecycle = Some(
                simulation
                    .summary
                    .calc_cooling_capacity_zero_flow_reset_lifecycle,
            );
            let purchased_air_calc_cooling_supply_mass_flow_maximum_lifecycle = Some(
                simulation
                    .summary
                    .calc_cooling_supply_mass_flow_maximum_lifecycle,
            );
            let purchased_air_calc_cooling_supply_mass_flow_ems_override_guard_lifecycle = Some(
                simulation
                    .summary
                    .calc_cooling_supply_mass_flow_ems_override_guard_lifecycle,
            );
            let purchased_air_calc_cooling_supply_mass_flow_ems_override_body_lifecycle = Some(
                simulation
                    .summary
                    .calc_cooling_supply_mass_flow_ems_override_body_lifecycle,
            );
            let purchased_air_calc_cooling_supply_mass_flow_limit_guard_lifecycle = Some(
                simulation
                    .summary
                    .calc_cooling_supply_mass_flow_limit_guard_lifecycle,
            );
            let purchased_air_calc_cooling_supply_mass_flow_limit_body_lifecycle = Some(
                simulation
                    .summary
                    .calc_cooling_supply_mass_flow_limit_body_lifecycle,
            );
            let purchased_air_calc_cooling_supply_mass_flow_very_small_guard_lifecycle = Some(
                simulation
                    .summary
                    .calc_cooling_supply_mass_flow_very_small_guard_lifecycle,
            );
            let purchased_air_calc_cooling_supply_mass_flow_very_small_guard_body_lifecycle = Some(
                simulation
                    .summary
                    .calc_cooling_supply_mass_flow_very_small_guard_body_lifecycle,
            );
            let purchased_air_calc_cooling_mixed_air_call_lifecycle =
                Some(simulation.summary.calc_cooling_mixed_air_call_lifecycle);
            let purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle = Some(
                simulation
                    .summary
                    .calc_cooling_supply_mass_flow_positive_guard_lifecycle,
            );
            let purchased_air_calc_cooling_positive_supply_cp_air_assignment_lifecycle = Some(
                simulation
                    .summary
                    .calc_cooling_positive_supply_cp_air_assignment_lifecycle,
            );
            let purchased_air_calc_cooling_positive_supply_temperature_assignment_lifecycle = Some(
                simulation
                    .summary
                    .calc_cooling_positive_supply_temperature_assignment_lifecycle,
            );
            let purchased_air_calc_cooling_positive_supply_temperature_minimum_limit_lifecycle =
                Some(
                    simulation
                        .summary
                        .calc_cooling_positive_supply_temperature_minimum_limit_lifecycle,
                );
            let purchased_air_calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle =
                Some(
                    simulation
                        .summary
                        .calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle,
                );
            let purchased_air_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle =
                Some(
                    simulation
                        .summary
                        .calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle,
                );
            let purchased_air_calc_cooling_positive_supply_enthalpy_assignment_lifecycle = Some(
                simulation
                    .summary
                    .calc_cooling_positive_supply_enthalpy_assignment_lifecycle,
            );
            let purchased_air_calc_cooling_positive_supply_capacity_limit_guard_lifecycle = Some(
                simulation
                    .summary
                    .calc_cooling_positive_supply_capacity_limit_guard_lifecycle,
            );
            let purchased_air_calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle =
                Some(
                    simulation
                        .summary
                        .calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle,
                );
            let purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle =
                Some(
                    simulation
                        .summary
                        .calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle,
                );
            let purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle =
                Some(
                    simulation
                        .summary
                        .calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle,
                );
            let purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_lifecycle =
                Some(
                    simulation
                        .summary
                        .calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_lifecycle,
                );
            let purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_lifecycle =
                Some(
                    simulation
                        .summary
                        .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_lifecycle,
                );
            let purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_lifecycle =
                Some(
                    simulation
                        .summary
                        .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_lifecycle,
                );
            let purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle =
                Some(
                    simulation
                        .summary
                        .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle,
                );
            let purchased_air_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle =
                Some(
                    simulation
                        .summary
                        .calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle,
                );
            let purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_lifecycle =
                Some(
                    simulation
                        .summary
                        .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_lifecycle,
                );
            let purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_lifecycle =
                Some(
                    simulation
                        .summary
                        .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_lifecycle,
                );
            let purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_lifecycle =
                Some(
                    simulation
                        .summary
                        .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_lifecycle,
                );
            let purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_lifecycle =
                Some(
                    simulation
                        .summary
                        .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_lifecycle,
                );
            let purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_lifecycle =
                Some(
                    simulation
                        .summary
                        .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_lifecycle,
                );
            let purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_lifecycle =
                Some(
                    simulation
                        .summary
                        .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_lifecycle,
                );
            let purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_lifecycle =
                Some(
                    simulation
                        .summary
                        .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_lifecycle,
                );
            Ok(RustRuntimeResult {
                results: simulation.results,
                runtime_class,
                sample_count,
                schedule_cache_sample_count,
                schedule_cache_profile,
                source_order_gate,
                zone_demand_source,
                fixture_demand_injection_used,
                purchased_air_branch,
                recirculation_node,
                recirculation_state_source,
                actual_coupled_source_order,
                purchased_air_coupling_call_count,
                purchased_air_init_lifecycle,
                purchased_air_calc_entry_lifecycle,
                purchased_air_calc_minimum_oa_prefix_lifecycle,
                purchased_air_calc_cooling_entry_gate_lifecycle,
                purchased_air_calc_cooling_oa_max_flow_gate_lifecycle,
                purchased_air_calc_cooling_oa_max_flow_body_lifecycle,
                purchased_air_calc_cooling_economizer_guard_lifecycle,
                purchased_air_calc_cooling_economizer_condition_lifecycle,
                purchased_air_calc_cooling_economizer_body_lifecycle,
                purchased_air_calc_cooling_sensible_flow_lifecycle,
                purchased_air_calc_cooling_dehumidification_flow_lifecycle,
                purchased_air_calc_cooling_humidification_flow_lifecycle,
                purchased_air_calc_cooling_capacity_zero_flow_reset_lifecycle,
                purchased_air_calc_cooling_supply_mass_flow_maximum_lifecycle,
                purchased_air_calc_cooling_supply_mass_flow_ems_override_guard_lifecycle,
                purchased_air_calc_cooling_supply_mass_flow_ems_override_body_lifecycle,
                purchased_air_calc_cooling_supply_mass_flow_limit_guard_lifecycle,
                purchased_air_calc_cooling_supply_mass_flow_limit_body_lifecycle,
                purchased_air_calc_cooling_supply_mass_flow_very_small_guard_lifecycle,
                purchased_air_calc_cooling_supply_mass_flow_very_small_guard_body_lifecycle,
                purchased_air_calc_cooling_mixed_air_call_lifecycle,
                purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle,
                purchased_air_calc_cooling_positive_supply_cp_air_assignment_lifecycle,
                purchased_air_calc_cooling_positive_supply_temperature_assignment_lifecycle,
                purchased_air_calc_cooling_positive_supply_temperature_minimum_limit_lifecycle,
                purchased_air_calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle,
                purchased_air_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle,
                purchased_air_calc_cooling_positive_supply_enthalpy_assignment_lifecycle,
                purchased_air_calc_cooling_positive_supply_capacity_limit_guard_lifecycle,
                purchased_air_calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle,
                purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle,
                purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle,
                purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_lifecycle,
                purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_lifecycle,
                purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_lifecycle,
                purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle,
                purchased_air_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle,
                purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_lifecycle,
                purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_lifecycle,
                purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_lifecycle,
                purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_lifecycle,
                purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_lifecycle,
                purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_lifecycle,
                purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_lifecycle,
            })
        }
        RuntimeClass::IdealLoadsFixtureDemandDiagnostic => {
            let simulation = simulate_ideal_loads_purchased_air_compat(
                model,
                IdealLoadsCompatibilityOptions::hourly_samples(sample_count),
            )
            .map_err(|error| error.to_string())?;
            Ok(RustRuntimeResult {
                results: simulation.results,
                runtime_class,
                sample_count,
                schedule_cache_sample_count,
                schedule_cache_profile,
                source_order_gate,
                zone_demand_source: Some(simulation.summary.zone_demand_source.to_string()),
                fixture_demand_injection_used: Some(
                    simulation.summary.fixture_demand_injection_used,
                ),
                purchased_air_branch: None,
                recirculation_node: None,
                recirculation_state_source: None,
                actual_coupled_source_order: None,
                purchased_air_coupling_call_count: None,
                purchased_air_init_lifecycle: None,
                purchased_air_calc_entry_lifecycle: None,
                purchased_air_calc_minimum_oa_prefix_lifecycle: None,
                purchased_air_calc_cooling_entry_gate_lifecycle: None,
                purchased_air_calc_cooling_oa_max_flow_gate_lifecycle: None,
                purchased_air_calc_cooling_oa_max_flow_body_lifecycle: None,
                purchased_air_calc_cooling_economizer_guard_lifecycle: None,
                purchased_air_calc_cooling_economizer_condition_lifecycle: None,
                purchased_air_calc_cooling_economizer_body_lifecycle: None,
                purchased_air_calc_cooling_sensible_flow_lifecycle: None,
                purchased_air_calc_cooling_dehumidification_flow_lifecycle: None,
                purchased_air_calc_cooling_humidification_flow_lifecycle: None,
                purchased_air_calc_cooling_capacity_zero_flow_reset_lifecycle: None,
                purchased_air_calc_cooling_supply_mass_flow_maximum_lifecycle: None,
                purchased_air_calc_cooling_supply_mass_flow_ems_override_guard_lifecycle: None,
                purchased_air_calc_cooling_supply_mass_flow_ems_override_body_lifecycle: None,
                purchased_air_calc_cooling_supply_mass_flow_limit_guard_lifecycle: None,
                purchased_air_calc_cooling_supply_mass_flow_limit_body_lifecycle: None,
                purchased_air_calc_cooling_supply_mass_flow_very_small_guard_lifecycle: None,
                purchased_air_calc_cooling_supply_mass_flow_very_small_guard_body_lifecycle: None,
                purchased_air_calc_cooling_mixed_air_call_lifecycle: None,
                purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle: None,
                purchased_air_calc_cooling_positive_supply_cp_air_assignment_lifecycle: None,
                purchased_air_calc_cooling_positive_supply_temperature_assignment_lifecycle: None,
                purchased_air_calc_cooling_positive_supply_temperature_minimum_limit_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_enthalpy_assignment_lifecycle: None,
                purchased_air_calc_cooling_positive_supply_capacity_limit_guard_lifecycle: None,
                purchased_air_calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_lifecycle:
                    None,
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
                schedule_cache_sample_count,
                schedule_cache_profile,
                source_order_gate,
                zone_demand_source: None,
                fixture_demand_injection_used: None,
                purchased_air_branch: None,
                recirculation_node: None,
                recirculation_state_source: None,
                actual_coupled_source_order: None,
                purchased_air_coupling_call_count: None,
                purchased_air_init_lifecycle: None,
                purchased_air_calc_entry_lifecycle: None,
                purchased_air_calc_minimum_oa_prefix_lifecycle: None,
                purchased_air_calc_cooling_entry_gate_lifecycle: None,
                purchased_air_calc_cooling_oa_max_flow_gate_lifecycle: None,
                purchased_air_calc_cooling_oa_max_flow_body_lifecycle: None,
                purchased_air_calc_cooling_economizer_guard_lifecycle: None,
                purchased_air_calc_cooling_economizer_condition_lifecycle: None,
                purchased_air_calc_cooling_economizer_body_lifecycle: None,
                purchased_air_calc_cooling_sensible_flow_lifecycle: None,
                purchased_air_calc_cooling_dehumidification_flow_lifecycle: None,
                purchased_air_calc_cooling_humidification_flow_lifecycle: None,
                purchased_air_calc_cooling_capacity_zero_flow_reset_lifecycle: None,
                purchased_air_calc_cooling_supply_mass_flow_maximum_lifecycle: None,
                purchased_air_calc_cooling_supply_mass_flow_ems_override_guard_lifecycle: None,
                purchased_air_calc_cooling_supply_mass_flow_ems_override_body_lifecycle: None,
                purchased_air_calc_cooling_supply_mass_flow_limit_guard_lifecycle: None,
                purchased_air_calc_cooling_supply_mass_flow_limit_body_lifecycle: None,
                purchased_air_calc_cooling_supply_mass_flow_very_small_guard_lifecycle: None,
                purchased_air_calc_cooling_supply_mass_flow_very_small_guard_body_lifecycle: None,
                purchased_air_calc_cooling_mixed_air_call_lifecycle: None,
                purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle: None,
                purchased_air_calc_cooling_positive_supply_cp_air_assignment_lifecycle: None,
                purchased_air_calc_cooling_positive_supply_temperature_assignment_lifecycle: None,
                purchased_air_calc_cooling_positive_supply_temperature_minimum_limit_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_enthalpy_assignment_lifecycle: None,
                purchased_air_calc_cooling_positive_supply_capacity_limit_guard_lifecycle: None,
                purchased_air_calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_lifecycle:
                    None,
                purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_lifecycle:
                    None,
            })
        }
        RuntimeClass::None => Err("no runtime selected".to_string()),
    }
}

fn validate_runtime_demand_provenance(
    run_result_state: RunResultState,
    result: &RustRuntimeResult,
    simulation_model: Option<&SimulationModel>,
) -> Result<(), String> {
    if result.runtime_class == RuntimeClass::IdealLoadsDirectZoneCoupledCompatibility
        && (result.zone_demand_source.as_deref() != Some(DIRECT_ZONE_PURCHASED_AIR_DEMAND_SOURCE)
            || result.fixture_demand_injection_used != Some(false))
    {
        return Err(
            "direct-zone IdealLoads runtime did not prove state-backed source-setpoint demand"
                .to_string(),
        );
    }
    if result.runtime_class == RuntimeClass::IdealLoadsDirectZoneCoupledCompatibility {
        let init_lifecycle = result.purchased_air_init_lifecycle.as_ref();
        validate_direct_purchased_air_init_lifecycle(
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        validate_direct_purchased_air_calc_entry_lifecycle(
            result.purchased_air_calc_entry_lifecycle.as_ref(),
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_minimum_oa::validate_direct_lifecycle(
            result
                .purchased_air_calc_minimum_oa_prefix_lifecycle
                .as_ref(),
            result.purchased_air_calc_entry_lifecycle.as_ref(),
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_entry_gate::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_entry_gate_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_minimum_oa_prefix_lifecycle
                .as_ref(),
            result.purchased_air_calc_entry_lifecycle.as_ref(),
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_oa_max_flow::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_oa_max_flow_gate_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_entry_gate_lifecycle
                .as_ref(),
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_oa_max_flow_body::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_oa_max_flow_body_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_oa_max_flow_gate_lifecycle
                .as_ref(),
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_economizer_guard::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_economizer_guard_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_oa_max_flow_body_lifecycle
                .as_ref(),
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_economizer_condition::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_economizer_condition_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_economizer_guard_lifecycle
                .as_ref(),
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_economizer_body::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_economizer_body_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_economizer_condition_lifecycle
                .as_ref(),
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_sensible_flow::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_sensible_flow_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_economizer_body_lifecycle
                .as_ref(),
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_dehumidification_flow::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_dehumidification_flow_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_sensible_flow_lifecycle
                .as_ref(),
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_humidification_flow::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_humidification_flow_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_dehumidification_flow_lifecycle
                .as_ref(),
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_capacity_zero_flow_reset::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_capacity_zero_flow_reset_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_humidification_flow_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_dehumidification_flow_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_sensible_flow_lifecycle
                .as_ref(),
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_supply_mass_flow_maximum::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_supply_mass_flow_maximum_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_capacity_zero_flow_reset_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_minimum_oa_prefix_lifecycle
                .as_ref(),
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_supply_mass_flow_ems_override_guard::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_supply_mass_flow_ems_override_guard_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_supply_mass_flow_maximum_lifecycle
                .as_ref(),
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_supply_mass_flow_ems_override_body::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_supply_mass_flow_ems_override_body_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_supply_mass_flow_ems_override_guard_lifecycle
                .as_ref(),
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        let model_cooling_limit = init_lifecycle
            .and_then(|lifecycle| lifecycle.declared_system_order.first())
            .and_then(|selected| {
                simulation_model.and_then(|model| {
                    model
                        .typed
                        .ideal_loads_air_systems
                        .iter()
                        .find(|system| system.id == *selected)
                })
            })
            .map(|system| system.cooling_limit);
        purchased_air_cooling_supply_mass_flow_limit_guard::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_supply_mass_flow_limit_guard_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_supply_mass_flow_ems_override_body_lifecycle
                .as_ref(),
            init_lifecycle,
            model_cooling_limit,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_supply_mass_flow_limit_body::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_supply_mass_flow_limit_body_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_supply_mass_flow_limit_guard_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_supply_mass_flow_maximum_lifecycle
                .as_ref(),
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_supply_mass_flow_very_small_guard::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_supply_mass_flow_very_small_guard_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_supply_mass_flow_limit_body_lifecycle
                .as_ref(),
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_supply_mass_flow_very_small_guard_body::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_supply_mass_flow_very_small_guard_body_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_supply_mass_flow_very_small_guard_lifecycle
                .as_ref(),
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_mixed_air_call::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_mixed_air_call_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_supply_mass_flow_very_small_guard_body_lifecycle
                .as_ref(),
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_supply_mass_flow_positive_guard::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_mixed_air_call_lifecycle
                .as_ref(),
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_positive_supply_cp_air_assignment::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_positive_supply_cp_air_assignment_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_mixed_air_call_lifecycle
                .as_ref(),
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_positive_supply_temperature_assignment::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_positive_supply_temperature_assignment_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_positive_supply_cp_air_assignment_lifecycle
                .as_ref(),
            result.purchased_air_calc_entry_lifecycle.as_ref(),
            result
                .purchased_air_calc_cooling_sensible_flow_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_mixed_air_call_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle
                .as_ref(),
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        let typed_minimum_cooling_supply_air_temperature_c = init_lifecycle
            .and_then(|lifecycle| lifecycle.declared_system_order.first().copied())
            .and_then(|system_id| {
                simulation_model.and_then(|model| {
                    model
                        .typed
                        .ideal_loads_air_systems
                        .iter()
                        .find(|system| system.id == system_id)
                })
            })
            .map(|system| system.minimum_cooling_supply_air_temperature_c);
        purchased_air_cooling_positive_supply_temperature_minimum_limit::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_positive_supply_temperature_minimum_limit_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_positive_supply_temperature_assignment_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_sensible_flow_lifecycle
                .as_ref(),
            init_lifecycle,
            typed_minimum_cooling_supply_air_temperature_c,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_positive_supply_temperature_mixed_air_limit::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_positive_supply_temperature_minimum_limit_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_mixed_air_call_lifecycle
                .as_ref(),
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_positive_supply_humidity_ratio_mixed_air_assignment::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_mixed_air_call_lifecycle
                .as_ref(),
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_positive_supply_enthalpy_assignment::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_positive_supply_enthalpy_assignment_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle
                .as_ref(),
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_positive_supply_capacity_limit_guard::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_positive_supply_capacity_limit_guard_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_positive_supply_enthalpy_assignment_lifecycle
                .as_ref(),
            init_lifecycle,
            model_cooling_limit,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_positive_supply_capacity_limit_cp_air_assignment::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_positive_supply_capacity_limit_guard_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_mixed_air_call_lifecycle
                .as_ref(),
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_positive_supply_capacity_limit_sensible_output_assignment::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_positive_supply_capacity_limit_guard_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_mixed_air_call_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_positive_supply_enthalpy_assignment_lifecycle
                .as_ref(),
            init_lifecycle,
            model_cooling_limit,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_positive_supply_capacity_limit_sensible_output_guard::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_capacity_zero_flow_reset_lifecycle
                .as_ref(),
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle
                .as_ref(),
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle
                .as_ref(),
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_positive_supply_enthalpy_assignment_lifecycle
                .as_ref(),
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_mixed_air_call_lifecycle
                .as_ref(),
            result
                .purchased_air_calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle
                .as_ref(),
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle
                .as_ref(),
            purchased_air_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment::DirectLifecyclePredecessors {
                capacity_limit_temperature_cp344: result
                    .purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle
                    .as_ref(),
                mixed_air_cp329: result
                    .purchased_air_calc_cooling_mixed_air_call_lifecycle
                    .as_ref(),
                corroborating_cp335: result
                    .purchased_air_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle
                    .as_ref(),
                positive_guard_cp330: result
                    .purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle
                    .as_ref(),
                enthalpy_cp336: result
                    .purchased_air_calc_cooling_positive_supply_enthalpy_assignment_lifecycle
                    .as_ref(),
                capacity_limit_guard_cp337: result
                    .purchased_air_calc_cooling_positive_supply_capacity_limit_guard_lifecycle
                    .as_ref(),
            },
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_lifecycle
                .as_ref(),
            purchased_air_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch::DirectLifecyclePredecessors {
                post_capacity_assignment_cp345: result
                    .purchased_air_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle
                    .as_ref(),
                dehumidification_flow_cp319: result
                    .purchased_air_calc_cooling_dehumidification_flow_lifecycle
                    .as_ref(),
            },
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_lifecycle
                .as_ref(),
            purchased_air_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case::DirectLifecyclePredecessors {
                control_switch_cp346: result
                    .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_lifecycle
                    .as_ref(),
                mixed_air_cp329: result
                    .purchased_air_calc_cooling_mixed_air_call_lifecycle
                    .as_ref(),
            },
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_lifecycle
                .as_ref(),
            purchased_air_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry::DirectLifecyclePredecessors {
                none_case_cp347: result
                    .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_lifecycle
                    .as_ref(),
            },
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_lifecycle
                .as_ref(),
            purchased_air_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment::DirectLifecyclePredecessors {
                case_entry_cp348: result
                    .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_lifecycle
                    .as_ref(),
                mixed_air_cp329: result
                    .purchased_air_calc_cooling_mixed_air_call_lifecycle
                    .as_ref(),
            },
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_lifecycle
                .as_ref(),
            purchased_air_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment::DirectLifecyclePredecessors {
                cp_air_assignment_cp349: result
                    .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_lifecycle
                    .as_ref(),
            },
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_lifecycle
                .as_ref(),
            purchased_air_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment::DirectLifecyclePredecessors {
                sensible_output_assignment_cp350: result
                    .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_lifecycle
                    .as_ref(),
            },
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
        purchased_air_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment::validate_direct_lifecycle(
            result
                .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_lifecycle
                .as_ref(),
            purchased_air_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment::DirectLifecyclePredecessors {
                total_output_assignment_cp351: result
                    .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_lifecycle
                    .as_ref(),
            },
            init_lifecycle,
            result.purchased_air_coupling_call_count,
        )?;
    } else if result.purchased_air_init_lifecycle.is_some()
        || result.purchased_air_calc_entry_lifecycle.is_some()
        || result
            .purchased_air_calc_minimum_oa_prefix_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_entry_gate_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_oa_max_flow_gate_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_oa_max_flow_body_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_economizer_guard_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_economizer_condition_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_economizer_body_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_sensible_flow_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_dehumidification_flow_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_humidification_flow_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_capacity_zero_flow_reset_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_supply_mass_flow_maximum_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_supply_mass_flow_ems_override_guard_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_supply_mass_flow_ems_override_body_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_supply_mass_flow_limit_guard_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_supply_mass_flow_limit_body_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_supply_mass_flow_very_small_guard_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_supply_mass_flow_very_small_guard_body_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_mixed_air_call_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_positive_supply_cp_air_assignment_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_positive_supply_temperature_assignment_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_positive_supply_temperature_minimum_limit_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_positive_supply_enthalpy_assignment_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_positive_supply_capacity_limit_guard_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_lifecycle
            .is_some()
        || result
            .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_lifecycle
            .is_some()
        || result.purchased_air_coupling_call_count.is_some()
    {
        return Err(
            "persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime"
                .to_string(),
        );
    }
    if result.runtime_class == RuntimeClass::IdealLoadsFixtureDemandDiagnostic
        && (result.zone_demand_source.as_deref()
            != Some(IDEAL_LOADS_FIXTURE_DEMAND_DIAGNOSTIC_SOURCE)
            || result.fixture_demand_injection_used != Some(true))
    {
        return Err(
            "IdealLoads fixture-demand diagnostic did not expose its active-split provenance"
                .to_string(),
        );
    }
    if run_result_state == RunResultState::SupportedCompatibilityRun
        && result.fixture_demand_injection_used == Some(true)
    {
        return Err(
            "fixture/default IdealLoads demand cannot execute in a release compatibility run"
                .to_string(),
        );
    }
    Ok(())
}

fn validate_direct_purchased_air_init_lifecycle(
    lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose persistent initialization evidence"
            .to_string()
    })?;
    let coupling_call_count = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose its coupling call count".to_string()
    })?;
    if lifecycle.source != PURCHASED_AIR_INIT_LIFECYCLE_SOURCE {
        return Err("direct-zone IdealLoads initialization provenance is invalid".to_string());
    }
    if coupling_call_count == 0 || lifecycle.init_call_count != coupling_call_count {
        return Err(
            "direct-zone IdealLoads initialization call count does not match coupling execution"
                .to_string(),
        );
    }
    let manager_sweep_ready = lifecycle.declared_system_order.len() == 1
        && lifecycle.equipment_list_scan_order == lifecycle.declared_system_order
        && lifecycle.equipment_list_scanned_unit_count == 1
        && lifecycle.equipment_list_missing_unit_count == 0
        && lifecycle.equipment_list_diagnostics.is_empty()
        && lifecycle.equipment_list_scan_ordinal == Some(1)
        && lifecycle.first_matching_equipment_list.is_some()
        && lifecycle.equipment_list_membership_found == Some(true);
    if !manager_sweep_ready {
        return Err(
            "direct-zone IdealLoads manager-wide equipment-list sweep is not release-ready"
                .to_string(),
        );
    }
    for (field, expected, actual) in [
        (
            "module_initialization_count",
            1,
            lifecycle.module_initialization_count,
        ),
        (
            "equipment_list_check_count",
            1,
            lifecycle.equipment_list_check_count,
        ),
        (
            "one_time_initialization_count",
            1,
            lifecycle.one_time_initialization_count,
        ),
        (
            "topology_completion_count",
            1,
            lifecycle.topology_completion_count,
        ),
        ("sizing_attempt_count", 1, lifecycle.sizing_attempt_count),
        ("sizing_check_count", 1, lifecycle.sizing_check_count),
        (
            "environment_initialization_count",
            1,
            lifecycle.environment_initialization_count,
        ),
        (
            "environment_rearm_count",
            usize::from(coupling_call_count > 1),
            lifecycle.environment_rearm_count,
        ),
    ] {
        if actual != expected {
            return Err(format!(
                "direct-zone IdealLoads initialization invariant {field} expected {expected}, got {actual}"
            ));
        }
    }
    let flags = lifecycle.flags;
    if !flags.state_machine_used
        || !flags.one_time_checked
        || !flags.topology_ready
        || !flags.environment_initialized
        || !flags.sizing_checked
        || !flags.equipment_list_checked
        || !flags.return_plenum_inactive
        || flags.environment_initialization_needed != (coupling_call_count > 1)
    {
        return Err(
            "direct-zone IdealLoads persistent initialization flags are not release-ready"
                .to_string(),
        );
    }
    let topology_ready = lifecycle.controlled_zone.is_some()
        && lifecycle.equipment_list.is_some()
        && lifecycle.equipment_list == lifecycle.first_matching_equipment_list
        && lifecycle.supply_node.is_some()
        && lifecycle.recirculation_node.is_some()
        && lifecycle.recirculation_source
            == Some(PurchasedAirRecirculationSource::SingleZoneReturn)
        && lifecycle.rejected_exhaust_node.is_none()
        && lifecycle.reported_first_return_node.is_none()
        && lifecycle.topology_diagnostics.is_empty()
        && lifecycle.topology_failure.is_none()
        && lifecycle.economizer_flow_limit_warning_count == 0;
    if !topology_ready {
        return Err(
            "direct-zone IdealLoads selected-unit topology is not release-ready".to_string(),
        );
    }
    let supply_temperature_diagnostics_clear =
        lifecycle.supply_temperature_registered_recurring_diagnostic_count == 0
            && lifecycle.supply_temperature_diagnostic_event_count == 0
            && lifecycle.supply_temperature_characterized_severe_error_count_increment == 0
            && lifecycle.cooling_supply_temperature_error_index == 0
            && lifecycle.heating_supply_temperature_error_index == 0
            && lifecycle.cooling_supply_temperature_first_diagnostic_count == 0
            && lifecycle.heating_supply_temperature_first_diagnostic_count == 0
            && lifecycle.supply_temperature_diagnostics.is_empty()
            && lifecycle.cooling_supply_temperature_warning_count == 0
            && lifecycle.heating_supply_temperature_warning_count == 0;
    if !supply_temperature_diagnostics_clear {
        return Err(
            "direct-zone IdealLoads supply-temperature diagnostic registry is not release-ready"
                .to_string(),
        );
    }
    let sized_limits = lifecycle
        .sized_limits
        .ok_or_else(|| "direct-zone IdealLoads sizing overlay is missing".to_string())?;
    let sized_values_valid = [
        sized_limits.maximum_heating_air_flow_rate_m3_per_s,
        sized_limits.maximum_sensible_heating_capacity_w,
        sized_limits.maximum_cooling_air_flow_rate_m3_per_s,
        sized_limits.maximum_total_cooling_capacity_w,
    ]
    .into_iter()
    .all(|value| match value {
        Some(AutosizeOrNumber::Value(value)) => value.is_finite() && value >= 0.0,
        None => true,
        Some(AutosizeOrNumber::Autosize) => false,
    });
    let sizing_outcome_ready = lifecycle.sizing_outcome.is_some_and(|outcome| {
        outcome.route == PurchasedAirHardSizeLegacyRoute::DirectHardSizedNoSizingRun
            && outcome.sized_limits == sized_limits
            && outcome.entry_fan_flags_cleared
            && outcome
                .fields
                .iter()
                .zip([
                    PurchasedAirHardSizeField::MaximumHeatingAirFlowRate,
                    PurchasedAirHardSizeField::MaximumSensibleHeatingCapacity,
                    PurchasedAirHardSizeField::MaximumCoolingAirFlowRate,
                    PurchasedAirHardSizeField::MaximumTotalCoolingCapacity,
                ])
                .all(|(field, expected)| field.is_some_and(|field| field.field == expected))
    });
    if !sized_values_valid || !sizing_outcome_ready {
        return Err(
            "direct-zone IdealLoads hard-size sizing state is not release-ready".to_string(),
        );
    }
    let density_valid = lifecycle
        .standard_air_density_kg_per_m3
        .is_some_and(|value| value.is_finite() && value > 0.0);
    let flow_caches_valid = lifecycle
        .maximum_heating_air_mass_flow_rate_kg_per_s
        .is_finite()
        && lifecycle.maximum_heating_air_mass_flow_rate_kg_per_s >= 0.0
        && lifecycle
            .maximum_cooling_air_mass_flow_rate_kg_per_s
            .is_finite()
        && lifecycle.maximum_cooling_air_mass_flow_rate_kg_per_s >= 0.0;
    if !density_valid || !flow_caches_valid {
        return Err(
            "direct-zone IdealLoads begin-environment initialization cache is invalid".to_string(),
        );
    }
    Ok(())
}

fn validate_direct_purchased_air_calc_entry_lifecycle(
    lifecycle: Option<&PurchasedAirCalcEntryLifecycleSummary>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose persistent Calc-entry evidence".to_string()
    })?;
    let init_lifecycle = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads Calc-entry evidence has no initialization evidence".to_string()
    })?;
    let coupling_call_count = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads Calc-entry evidence has no coupling call count".to_string()
    })?;
    let state = &lifecycle.state;
    if lifecycle.source != PURCHASED_AIR_CALC_ENTRY_SOURCE
        || coupling_call_count == 0
        || state.call_count != coupling_call_count
    {
        return Err(
            "direct-zone IdealLoads Calc-entry provenance or call count is invalid".to_string(),
        );
    }
    for (field, actual) in [
        ("reset_count", state.reset_count),
        ("demand_read_count", state.demand_read_count),
        (
            "overall_availability_read_count",
            state.overall_availability_read_count,
        ),
        (
            "heating_availability_read_count",
            state.heating_availability_read_count,
        ),
        (
            "cooling_availability_read_count",
            state.cooling_availability_read_count,
        ),
        (
            "availability_manager_read_count",
            state.availability_manager_read_count,
        ),
        (
            "availability_manager_zone_write_count",
            state.availability_manager_zone_write_count,
        ),
        (
            "availability_status_copy_count",
            state.availability_status_copy_count,
        ),
        ("heating_on_count", state.heating_on_count),
        ("cooling_on_count", state.cooling_on_count),
    ] {
        if actual != coupling_call_count {
            return Err(format!(
                "direct-zone IdealLoads Calc-entry invariant {field} expected {coupling_call_count}, got {actual}"
            ));
        }
    }
    let manager_ready = state.force_off_count == 0
        && state.availability_manager_zone == init_lifecycle.controlled_zone
        && state.availability_status == PurchasedAirAvailabilityStatus::NoAction;
    let partitions_reconcile = state
        .unit_body_entry_count
        .checked_add(state.unit_off_count)
        == Some(coupling_call_count)
        && state
            .unit_body_entry_count
            .checked_add(state.overall_schedule_off_count)
            == Some(coupling_call_count);
    let retained_resets_clear = state.minimum_outdoor_air_mass_flow_rate_kg_per_s == 0.0
        && state.economizer_active_time_hours == 0.0
        && state.heat_recovery_active_time_hours == 0.0;
    if !manager_ready || !partitions_reconcile || !retained_resets_clear {
        return Err(
            "direct-zone IdealLoads Calc-entry aggregate state is not release-ready".to_string(),
        );
    }
    let latest = state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads Calc-entry evidence has no latest snapshot".to_string()
    })?;
    let expected_system = init_lifecycle
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| {
            "direct-zone IdealLoads Calc-entry evidence has no declared system".to_string()
        })?;
    let latest_ready = latest.source == PURCHASED_AIR_CALC_ENTRY_SOURCE
        && latest.source_order == PURCHASED_AIR_CALC_ENTRY_SOURCE_ORDER
        && PURCHASED_AIR_CALC_ENTRY_RESET_TARGETS.len() == 12
        && latest.reset.all_zero()
        && state.system == expected_system
        && latest.system == expected_system
        && latest.call_ordinal == coupling_call_count
        && Some(latest.controlled_zone) == init_lifecycle.controlled_zone
        && latest.demand.zone == latest.controlled_zone
        && latest.demand.sensible_input_kind
            == ZoneSensibleDemandInputKind::SourceSetpointThresholds
        && latest.demand.remaining_output_req_to_heat_sp_w.is_finite()
        && latest.demand.remaining_output_req_to_cool_sp_w.is_finite()
        && Some(latest.supply_node) == init_lifecycle.supply_node
        && Some(latest.recirculation_node) == init_lifecycle.recirculation_node
        && latest.outdoor_air_node.is_none()
        && latest.unit_defaulted_on
        && !latest.economizer_defaulted_on
        && latest.availability_manager_read_site_visited
        && latest.availability_manager_zone_written
        && latest.copied_availability_status == Some(PurchasedAirAvailabilityStatus::NoAction)
        && !latest.force_off_applied
        && latest.overall_availability_read_site_visited
        && latest.heating_availability_read_site_visited
        && latest.cooling_availability_read_site_visited
        && latest.overall_availability.is_finite()
        && latest.heating_availability == 1.0
        && latest.cooling_availability == 1.0
        && latest.unit_on == (latest.overall_availability > 0.0)
        && latest.heating_on
        && latest.cooling_on
        && latest.unit_body_entered == latest.unit_on;
    if !latest_ready {
        return Err(
            "direct-zone IdealLoads latest Calc-entry snapshot is not release-ready".to_string(),
        );
    }
    Ok(())
}

fn validate_runtime_selection(
    run_result_state: RunResultState,
    runtime_class: RuntimeClass,
) -> Result<(), String> {
    if run_result_state == RunResultState::SupportedCompatibilityRun
        && !runtime_class.conformance_promotion_allowed()
    {
        return Err(format!(
            "diagnostic runtime '{}' cannot execute as a release compatibility run",
            runtime_class.id()
        ));
    }
    Ok(())
}

fn runtime_class_requires_weather(runtime_class: RuntimeClass) -> bool {
    matches!(
        runtime_class,
        RuntimeClass::OneZoneHeatBalanceCompatibility
            | RuntimeClass::HeatBalanceZoneAirDiagnostic
            | RuntimeClass::IdealLoadsDirectZoneCoupledCompatibility
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
        RustRuntimeResult, SourceOrderGateSummary, artifact_map, ctf_split_trace_enabled,
        execution_stage_snapshots, full_surface_trace_opt_in, input_error_diagnostic_code,
        purchased_air_calc_entry_lifecycle_json, purchased_air_cooling_capacity_zero_flow_reset,
        purchased_air_cooling_dehumidification_flow, purchased_air_cooling_economizer_body,
        purchased_air_cooling_economizer_condition, purchased_air_cooling_economizer_guard,
        purchased_air_cooling_entry_gate, purchased_air_cooling_humidification_flow,
        purchased_air_cooling_oa_max_flow, purchased_air_cooling_oa_max_flow_body,
        purchased_air_cooling_sensible_flow, purchased_air_init_lifecycle_json,
        purchased_air_minimum_oa, runtime_class_requires_weather, schedule_cache_json,
        selected_trace_enabled, source_order_gate_summary, source_order_stage_state_snapshots,
        trace_level_enables_stage_snapshots, typed_counts,
        validate_direct_purchased_air_calc_entry_lifecycle,
        validate_direct_purchased_air_init_lifecycle, validate_runtime_demand_provenance,
        validate_runtime_selection,
    };
    use ep_compiler::compile_raw_model;
    use ep_model::{
        DehumidificationControlType, ExternalInterfaceFmuExportSchedule,
        ExternalInterfaceFmuImportSchedule, ExternalInterfaceSchedule, HumidificationControlType,
        IdealLoadsAirSystemId, IdealLoadsLimit, NodeId, NormalizedName, OutdoorAirEconomizerType,
        ScheduleFileShading, ScheduleFileShadingColumn, ScheduleId, TypedModel,
        ZoneEquipmentListId, ZoneId,
    };
    use ep_raw_model::parse_epjson_str_with_idf_order;
    use ep_runtime::{
        DayType, EnergyPlusCompatibilityStage, ExecutionPlan, ExecutionStage, ExecutionStageKind,
        ExecutionStep, IDEAL_LOADS_FIXTURE_DEMAND_DIAGNOSTIC_SOURCE, IdealLoadsInitFlags,
        IdealLoadsSensibleMode,
        PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE,
        PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE_ORDER,
        PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SOURCE,
        PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SOURCE_ORDER,
        PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE,
        PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE_ORDER,
        PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE,
        PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE_ORDER,
        PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE,
        PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE_ORDER,
        PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE,
        PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE_ORDER,
        PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE,
        PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE_ORDER,
        PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_RECURRING_WARNING_CHILD_SOURCE,
        PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE,
        PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE_ORDER,
        PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE,
        PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE_ORDER,
        PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE,
        PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE_ORDER, PURCHASED_AIR_CALC_ENTRY_SOURCE,
        PURCHASED_AIR_CALC_ENTRY_SOURCE_ORDER, PURCHASED_AIR_CALC_MINIMUM_OA_CHILD_SOURCE,
        PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE,
        PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE_ORDER, PURCHASED_AIR_INIT_LIFECYCLE_SOURCE,
        PurchasedAirAvailabilityStatus,
        PurchasedAirCalcCoolingCapacityZeroFlowResetLifecycleSummary,
        PurchasedAirCalcCoolingCapacityZeroFlowResetRuntimeState,
        PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
        PurchasedAirCalcCoolingDehumidificationFlowLifecycleSummary,
        PurchasedAirCalcCoolingDehumidificationFlowRuntimeState,
        PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
        PurchasedAirCalcCoolingEconomizerBodyLifecycleSummary,
        PurchasedAirCalcCoolingEconomizerBodyRuntimeState,
        PurchasedAirCalcCoolingEconomizerBodySnapshot,
        PurchasedAirCalcCoolingEconomizerConditionLifecycleSummary,
        PurchasedAirCalcCoolingEconomizerConditionRuntimeState,
        PurchasedAirCalcCoolingEconomizerConditionSnapshot,
        PurchasedAirCalcCoolingEconomizerGuardLifecycleSummary,
        PurchasedAirCalcCoolingEconomizerGuardRuntimeState,
        PurchasedAirCalcCoolingEconomizerGuardSnapshot,
        PurchasedAirCalcCoolingEntryGateLifecycleSummary,
        PurchasedAirCalcCoolingEntryGateRuntimeState, PurchasedAirCalcCoolingEntryGateSnapshot,
        PurchasedAirCalcCoolingHumidificationFlowLifecycleSummary,
        PurchasedAirCalcCoolingHumidificationFlowRuntimeState,
        PurchasedAirCalcCoolingHumidificationFlowSnapshot,
        PurchasedAirCalcCoolingOaMaxFlowBodyLifecycleSummary,
        PurchasedAirCalcCoolingOaMaxFlowBodyRuntimeState,
        PurchasedAirCalcCoolingOaMaxFlowBodySnapshot,
        PurchasedAirCalcCoolingOaMaxFlowGateLifecycleSummary,
        PurchasedAirCalcCoolingOaMaxFlowGateRuntimeState,
        PurchasedAirCalcCoolingOaMaxFlowGateSnapshot,
        PurchasedAirCalcCoolingSensibleFlowLifecycleSummary,
        PurchasedAirCalcCoolingSensibleFlowRuntimeState,
        PurchasedAirCalcCoolingSensibleFlowSnapshot, PurchasedAirCalcEntryDemandSnapshot,
        PurchasedAirCalcEntryLifecycleSummary, PurchasedAirCalcEntryResetSnapshot,
        PurchasedAirCalcEntryRuntimeState, PurchasedAirCalcEntrySnapshot,
        PurchasedAirCalcMinimumOaPrefixLifecycleSummary,
        PurchasedAirCalcMinimumOaPrefixRuntimeState, PurchasedAirCalcMinimumOaPrefixSnapshot,
        PurchasedAirHardSizeField, PurchasedAirHardSizeFieldOutcome,
        PurchasedAirHardSizeLegacyOutcome, PurchasedAirHardSizeLegacyRoute,
        PurchasedAirInitLifecycleSummary, PurchasedAirRecirculationSource, PurchasedAirSizedLimits,
        PurchasedAirSupplyTemperatureDiagnostic, PurchasedAirSupplyTemperatureDiagnosticKind,
        PurchasedAirSupplyTemperatureInitialMessageApi, PurchasedAirTemperatureControlType,
        ResultStore, ScheduleCacheProfile, ScheduleSeriesIndexKind, ZoneSensibleDemandInputKind,
        build_hourly_time_axis,
    };

    use crate::{RunResultState, RuntimeClass, TraceLevel, TraceSelection};

    #[test]
    fn direct_zone_coupled_runtime_requires_weather_axis() {
        assert!(runtime_class_requires_weather(
            RuntimeClass::IdealLoadsDirectZoneCoupledCompatibility
        ));
        assert!(!runtime_class_requires_weather(
            RuntimeClass::IdealLoadsFixtureDemandDiagnostic
        ));
    }

    #[test]
    fn release_compatibility_rejects_diagnostic_runtime_before_execution() {
        assert!(
            validate_runtime_selection(
                RunResultState::SupportedCompatibilityRun,
                RuntimeClass::IdealLoadsFixtureDemandDiagnostic,
            )
            .is_err()
        );
        assert!(
            validate_runtime_selection(
                RunResultState::SupportedCompatibilityRun,
                RuntimeClass::IdealLoadsDirectZoneCoupledCompatibility,
            )
            .is_ok()
        );
        assert!(
            validate_runtime_selection(
                RunResultState::PartialSupportedRun,
                RuntimeClass::IdealLoadsFixtureDemandDiagnostic,
            )
            .is_ok()
        );
    }

    #[test]
    fn direct_release_lifecycle_validation_rejects_missing_or_disconnected_evidence() {
        let valid = valid_init_lifecycle(2);
        assert!(validate_direct_purchased_air_init_lifecycle(Some(&valid), Some(2)).is_ok());
        assert!(validate_direct_purchased_air_init_lifecycle(None, Some(2)).is_err());
        assert!(validate_direct_purchased_air_init_lifecycle(Some(&valid), None).is_err());
        assert!(validate_direct_purchased_air_init_lifecycle(Some(&valid), Some(0)).is_err());
        assert!(validate_direct_purchased_air_init_lifecycle(Some(&valid), Some(3)).is_err());

        let mut wrong_source = valid.clone();
        wrong_source.source = "diagnostic-init-marker";
        assert!(
            validate_direct_purchased_air_init_lifecycle(Some(&wrong_source), Some(2)).is_err()
        );
        let mut disconnected = valid;
        disconnected.flags.state_machine_used = false;
        assert!(
            validate_direct_purchased_air_init_lifecycle(Some(&disconnected), Some(2)).is_err()
        );

        let valid = valid_init_lifecycle(2);
        let mut latched_but_unusable = valid.clone();
        latched_but_unusable.flags.topology_ready = false;
        assert!(
            validate_direct_purchased_air_init_lifecycle(Some(&latched_but_unusable), Some(2))
                .is_err()
        );
        let mut wrong_branch = valid.clone();
        wrong_branch.recirculation_source =
            Some(PurchasedAirRecirculationSource::ConfiguredZoneExhaust);
        assert!(
            validate_direct_purchased_air_init_lifecycle(Some(&wrong_branch), Some(2)).is_err()
        );
        let mut missing_recirculation = valid.clone();
        missing_recirculation.recirculation_node = None;
        assert!(
            validate_direct_purchased_air_init_lifecycle(Some(&missing_recirculation), Some(2))
                .is_err()
        );
        let mut incomplete = valid.clone();
        incomplete.topology_completion_count = 0;
        assert!(validate_direct_purchased_air_init_lifecycle(Some(&incomplete), Some(2)).is_err());
        let mut advisory = valid.clone();
        advisory.economizer_flow_limit_warning_count = 1;
        assert!(validate_direct_purchased_air_init_lifecycle(Some(&advisory), Some(2)).is_err());
        let mut supply_temperature_diagnostic = valid.clone();
        supply_temperature_diagnostic.supply_temperature_diagnostic_event_count = 1;
        assert!(
            validate_direct_purchased_air_init_lifecycle(
                Some(&supply_temperature_diagnostic),
                Some(2)
            )
            .is_err()
        );
        let mut equipment_mismatch = valid;
        equipment_mismatch.equipment_list = Some(ZoneEquipmentListId(1));
        assert!(
            validate_direct_purchased_air_init_lifecycle(Some(&equipment_mismatch), Some(2))
                .is_err()
        );
    }

    #[test]
    fn direct_release_lifecycle_json_exposes_selected_topology_evidence() {
        let lifecycle = valid_init_lifecycle(2);
        let value = purchased_air_init_lifecycle_json(&lifecycle);

        assert_eq!(value["flags"]["topology_ready"], true);
        assert_eq!(value["controlled_zone"], 0);
        assert_eq!(value["equipment_list"], 0);
        assert_eq!(value["supply_node"], 3);
        assert_eq!(value["recirculation_node"], 4);
        assert_eq!(value["recirculation_source"], "single_zone_return");
        assert_eq!(value["topology_diagnostics"], serde_json::json!([]));
        assert!(value["topology_failure"].is_null());
        assert_eq!(value["topology_completion_count"], 1);
        assert_eq!(value["sizing_attempt_count"], 1);
        assert_eq!(
            value["sizing_outcome"]["route"],
            "direct_hard_sized_no_sizing_run"
        );
        assert_eq!(
            value["sizing_outcome"]["fields"].as_array().map(Vec::len),
            Some(4)
        );
        assert!(value["sized_limits"].is_object());
        assert_eq!(
            value["supply_temperature_diagnostic_registry"]["registered_recurring_diagnostic_count"],
            0
        );
        assert_eq!(
            value["supply_temperature_diagnostic_registry"]["event_count"],
            0
        );
        assert_eq!(
            value["supply_temperature_diagnostic_registry"]["identities"],
            serde_json::json!([])
        );
        assert!(value["supply_temperature_diagnostic_registry"]["cooling_error_index"].is_null());
        assert!(value["supply_temperature_diagnostic_registry"]["heating_error_index"].is_null());
        assert_eq!(value["economizer_flow_limit_warning_count"], 0);
    }

    #[test]
    fn direct_release_calc_entry_validation_rejects_disconnected_evidence() {
        let init = valid_init_lifecycle(2);
        let valid = valid_calc_entry_lifecycle(2);
        assert!(
            validate_direct_purchased_air_calc_entry_lifecycle(Some(&valid), Some(&init), Some(2))
                .is_ok()
        );
        assert!(
            validate_direct_purchased_air_calc_entry_lifecycle(None, Some(&init), Some(2)).is_err()
        );
        assert!(
            validate_direct_purchased_air_calc_entry_lifecycle(Some(&valid), Some(&init), Some(3))
                .is_err()
        );

        let mut wrong_source = valid.clone();
        wrong_source.source = "diagnostic-calc-entry-marker";
        assert!(
            validate_direct_purchased_air_calc_entry_lifecycle(
                Some(&wrong_source),
                Some(&init),
                Some(2)
            )
            .is_err()
        );
        let mut wrong_manager_count = valid.clone();
        wrong_manager_count.state.availability_manager_read_count = 1;
        assert!(
            validate_direct_purchased_air_calc_entry_lifecycle(
                Some(&wrong_manager_count),
                Some(&init),
                Some(2)
            )
            .is_err()
        );
        let mut wrong_mode_count = valid.clone();
        wrong_mode_count.state.heating_on_count = 1;
        assert!(
            validate_direct_purchased_air_calc_entry_lifecycle(
                Some(&wrong_mode_count),
                Some(&init),
                Some(2)
            )
            .is_err()
        );
        let mut wrong_identity = valid;
        wrong_identity
            .state
            .latest
            .as_mut()
            .expect("valid latest snapshot")
            .supply_node = NodeId(99);
        assert!(
            validate_direct_purchased_air_calc_entry_lifecycle(
                Some(&wrong_identity),
                Some(&init),
                Some(2)
            )
            .is_err()
        );
    }

    #[test]
    fn direct_release_calc_entry_json_exposes_reset_demand_and_gates() {
        let lifecycle = valid_calc_entry_lifecycle(2);
        let value = purchased_air_calc_entry_lifecycle_json(&lifecycle);

        assert_eq!(value["source"], PURCHASED_AIR_CALC_ENTRY_SOURCE);
        assert_eq!(value["call_count"], 2);
        assert_eq!(value["reset_count"], 2);
        assert_eq!(value["availability_manager_read_count"], 2);
        assert_eq!(value["availability_manager_zone_write_count"], 2);
        assert_eq!(value["availability_status_copy_count"], 2);
        assert_eq!(value["availability_manager_zone"], 0);
        assert_eq!(value["availability_status"], "no_action");
        assert_eq!(value["latest"]["call_ordinal"], 2);
        assert_eq!(value["latest"]["reset"]["field_count"], 12);
        assert_eq!(value["latest"]["reset"]["all_zero"], true);
        assert_eq!(
            value["latest"]["demand"]["sensible_input_kind"],
            "source_setpoint_thresholds"
        );
        assert_eq!(value["latest"]["heating_availability"], 1.0);
        assert_eq!(value["latest"]["cooling_availability"], 1.0);
        assert_eq!(value["latest"]["unit_body_entered"], true);
    }

    #[test]
    fn direct_release_minimum_oa_prefix_validation_rejects_disconnected_evidence() {
        let init = valid_init_lifecycle(2);
        let entry = valid_calc_entry_lifecycle(2);
        let valid = valid_minimum_oa_prefix_lifecycle(2);
        assert!(
            purchased_air_minimum_oa::validate_direct_lifecycle(
                Some(&valid),
                Some(&entry),
                Some(&init),
                Some(2)
            )
            .is_ok()
        );
        assert!(
            purchased_air_minimum_oa::validate_direct_lifecycle(
                None,
                Some(&entry),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut wrong_child = valid.clone();
        wrong_child.minimum_oa_child_source = "diagnostic-minimum-oa-child";
        assert!(
            purchased_air_minimum_oa::validate_direct_lifecycle(
                Some(&wrong_child),
                Some(&entry),
                Some(&init),
                Some(2)
            )
            .is_err()
        );
        let mut overflowed_partition = valid.clone();
        overflowed_partition.state.source_execution_count = usize::MAX;
        overflowed_partition.state.unit_off_skip_count = 1;
        assert!(
            purchased_air_minimum_oa::validate_direct_lifecycle(
                Some(&overflowed_partition),
                Some(&entry),
                Some(&init),
                Some(2)
            )
            .is_err()
        );
        let mut active_outdoor_air = valid;
        active_outdoor_air.state.outdoor_air_effect_count = 1;
        active_outdoor_air
            .state
            .latest
            .as_mut()
            .expect("valid latest minimum-OA prefix")
            .outdoor_air_enabled = Some(true);
        assert!(
            purchased_air_minimum_oa::validate_direct_lifecycle(
                Some(&active_outdoor_air),
                Some(&entry),
                Some(&init),
                Some(2)
            )
            .is_err()
        );
    }

    #[test]
    fn direct_release_minimum_oa_prefix_json_exposes_zero_no_oa_route() {
        let lifecycle = valid_minimum_oa_prefix_lifecycle(2);
        let value = purchased_air_minimum_oa::lifecycle_json(&lifecycle);

        assert_eq!(value["source"], PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE);
        assert_eq!(
            value["minimum_oa_child_source"],
            PURCHASED_AIR_CALC_MINIMUM_OA_CHILD_SOURCE
        );
        assert_eq!(value["transition_count"], 2);
        assert_eq!(value["source_execution_count"], 2);
        assert_eq!(value["ems_override_apply_count"], 0);
        assert_eq!(value["outdoor_air_effect_count"], 0);
        assert_eq!(value["no_outdoor_air_zero_branch_count"], 2);
        assert_eq!(value["latest"]["parent_call_ordinal"], 2);
        assert_eq!(value["latest"]["ems_override_enabled"], false);
        assert_eq!(value["latest"]["outdoor_air_enabled"], false);
        assert_eq!(
            value["latest"]["retained_minimum_outdoor_air_mass_flow_rate_kg_per_s"],
            0.0
        );
        assert_eq!(
            value["latest"]["working_outdoor_air_mass_flow_rate_kg_per_s"],
            0.0
        );
        assert_eq!(
            value["latest"]["minimum_outdoor_air_sensible_output_w"],
            0.0
        );
        assert_eq!(
            value["latest"]["minimum_outdoor_air_moisture_output_kg_per_s"],
            0.0
        );
    }

    #[test]
    fn direct_release_cooling_entry_gate_validation_rejects_disconnected_evidence() {
        let init = valid_init_lifecycle(2);
        let entry = valid_calc_entry_lifecycle(2);
        let minimum_oa = valid_minimum_oa_prefix_lifecycle(2);
        let valid = valid_cooling_entry_gate_lifecycle(2);
        assert!(
            purchased_air_cooling_entry_gate::validate_direct_lifecycle(
                Some(&valid),
                Some(&minimum_oa),
                Some(&entry),
                Some(&init),
                Some(2)
            )
            .is_ok()
        );
        assert!(
            purchased_air_cooling_entry_gate::validate_direct_lifecycle(
                None,
                Some(&minimum_oa),
                Some(&entry),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut wrong_boundary = valid.clone();
        wrong_boundary.first_excluded_source = "EnergyPlus 26.1 PurchasedAirManager.cc:2348";
        assert!(
            purchased_air_cooling_entry_gate::validate_direct_lifecycle(
                Some(&wrong_boundary),
                Some(&minimum_oa),
                Some(&entry),
                Some(&init),
                Some(2)
            )
            .is_err()
        );
        let mut overflowed_partition = valid.clone();
        overflowed_partition.state.cooling_body_entry_count = usize::MAX;
        overflowed_partition.state.active_fallthrough_count = 1;
        assert!(
            purchased_air_cooling_entry_gate::validate_direct_lifecycle(
                Some(&overflowed_partition),
                Some(&minimum_oa),
                Some(&entry),
                Some(&init),
                Some(2)
            )
            .is_err()
        );
        let mut single_heat = valid;
        single_heat.state.single_heat_block_count = 1;
        let latest = single_heat
            .state
            .latest
            .as_mut()
            .expect("valid latest cooling-entry gate");
        latest.single_heat_blocked = true;
        latest.temperature_control_type = Some(PurchasedAirTemperatureControlType::SingleHeat);
        assert!(
            purchased_air_cooling_entry_gate::validate_direct_lifecycle(
                Some(&single_heat),
                Some(&minimum_oa),
                Some(&entry),
                Some(&init),
                Some(2)
            )
            .is_err()
        );
    }

    #[test]
    fn direct_release_cooling_entry_gate_json_exposes_inclusive_cooling_route() {
        let lifecycle = valid_cooling_entry_gate_lifecycle(2);
        let value = purchased_air_cooling_entry_gate::lifecycle_json(&lifecycle);

        assert_eq!(
            value["source"],
            PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE
        );
        assert_eq!(
            value["first_excluded_source"],
            PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_FIRST_EXCLUDED_SOURCE
        );
        assert_eq!(value["transition_count"], 2);
        assert_eq!(value["sensible_comparison_satisfied_count"], 2);
        assert_eq!(value["temperature_control_type_read_count"], 2);
        assert_eq!(value["single_heat_block_count"], 0);
        assert_eq!(value["cooling_body_entry_count"], 2);
        assert_eq!(value["latest"]["parent_call_ordinal"], 2);
        assert_eq!(value["latest"]["cooling_setpoint_demand_w"], -50.0);
        assert_eq!(value["latest"]["sensible_comparison_satisfied"], true);
        assert_eq!(value["latest"]["temperature_control_type"], "DualHeatCool");
        assert_eq!(value["latest"]["cooling_body_entered"], true);
        assert_eq!(value["latest"]["assigned_operating_mode"], "Cooling");
    }

    #[test]
    fn direct_release_cooling_oa_max_flow_validation_rejects_disconnected_evidence() {
        let init = valid_init_lifecycle(2);
        let predecessor = valid_cooling_entry_gate_lifecycle(2);
        let valid = valid_cooling_oa_max_flow_gate_lifecycle(2, IdealLoadsLimit::LimitFlowRate);
        assert!(
            purchased_air_cooling_oa_max_flow::validate_direct_lifecycle(
                Some(&valid),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_ok()
        );
        assert!(
            purchased_air_cooling_oa_max_flow::validate_direct_lifecycle(
                None,
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut wrong_boundary = valid.clone();
        wrong_boundary.first_excluded_source = "EnergyPlus 26.1 PurchasedAirManager.cc:2082";
        assert!(
            purchased_air_cooling_oa_max_flow::validate_direct_lifecycle(
                Some(&wrong_boundary),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut wrong_source_order = valid.clone();
        wrong_source_order
            .state
            .latest
            .as_mut()
            .expect("valid latest cooling OA maximum-flow gate")
            .source_order = &["compare-cooling-limit-to-flow-rate"];
        assert!(
            purchased_air_cooling_oa_max_flow::validate_direct_lifecycle(
                Some(&wrong_source_order),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut wrong_limit_shape = valid.clone();
        wrong_limit_shape
            .state
            .latest
            .as_mut()
            .expect("valid latest cooling OA maximum-flow gate")
            .cooling_limit_flow_rate_value = Some(IdealLoadsLimit::LimitCapacity);
        assert!(
            purchased_air_cooling_oa_max_flow::validate_direct_lifecycle(
                Some(&wrong_limit_shape),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut overflowed_partition = valid.clone();
        overflowed_partition.state.source_execution_count = usize::MAX;
        overflowed_partition.state.unit_off_skip_count = 1;
        assert!(
            purchased_air_cooling_oa_max_flow::validate_direct_lifecycle(
                Some(&overflowed_partition),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut entered_excluded_body = valid;
        entered_excluded_body
            .state
            .maximum_cooling_flow_body_entry_count = 1;
        entered_excluded_body
            .state
            .latest
            .as_mut()
            .expect("valid latest cooling OA maximum-flow gate")
            .maximum_cooling_flow_body_entered = true;
        assert!(
            purchased_air_cooling_oa_max_flow::validate_direct_lifecycle(
                Some(&entered_excluded_body),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );
    }

    #[test]
    fn direct_release_cooling_oa_max_flow_json_exposes_limit_short_circuit_routes() {
        for (limit, expected_name, second_evaluated, flow_selected) in [
            (IdealLoadsLimit::NoLimit, "NoLimit", true, false),
            (IdealLoadsLimit::LimitFlowRate, "LimitFlowRate", false, true),
            (IdealLoadsLimit::LimitCapacity, "LimitCapacity", true, false),
            (
                IdealLoadsLimit::LimitFlowRateAndCapacity,
                "LimitFlowRateAndCapacity",
                true,
                true,
            ),
        ] {
            let lifecycle = valid_cooling_oa_max_flow_gate_lifecycle(2, limit);
            let value = purchased_air_cooling_oa_max_flow::lifecycle_json(&lifecycle);

            assert_eq!(
                value["source"],
                PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE
            );
            assert_eq!(
                value["first_excluded_source"],
                PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_FIRST_EXCLUDED_SOURCE
            );
            assert_eq!(value["transition_count"], 2);
            assert_eq!(value["source_execution_count"], 2);
            assert_eq!(value["maximum_cooling_flow_body_entry_count"], 0);
            assert_eq!(
                value["latest"]["cooling_limit_flow_rate_value"],
                expected_name
            );
            assert_eq!(
                value["latest"]["cooling_limit_flow_rate_and_capacity_comparison_evaluated"],
                second_evaluated
            );
            assert_eq!(value["latest"]["cooling_flow_limit_active"], flow_selected);
            assert_eq!(
                value["latest"]["strict_mass_flow_comparison_evaluated"],
                flow_selected
            );
            assert_eq!(value["latest"]["maximum_cooling_flow_body_entered"], false);
        }
    }

    #[test]
    fn direct_release_cooling_oa_max_flow_body_validation_rejects_malformed_evidence() {
        let init = valid_init_lifecycle(2);
        let predecessor =
            valid_cooling_oa_max_flow_gate_lifecycle(2, IdealLoadsLimit::LimitFlowRate);
        let valid = valid_cooling_oa_max_flow_body_lifecycle(2);
        assert!(
            purchased_air_cooling_oa_max_flow_body::validate_direct_lifecycle(
                Some(&valid),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_ok()
        );
        assert!(
            purchased_air_cooling_oa_max_flow_body::validate_direct_lifecycle(
                None,
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut wrong_provenance = valid.clone();
        wrong_provenance.recurring_warning_child_source = "process-global-message-sink";
        assert!(
            purchased_air_cooling_oa_max_flow_body::validate_direct_lifecycle(
                Some(&wrong_provenance),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut wrong_count = valid.clone();
        wrong_count.state.warning_counter_read_count = 1;
        assert!(
            purchased_air_cooling_oa_max_flow_body::validate_direct_lifecycle(
                Some(&wrong_count),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut wrong_clamp = valid.clone();
        wrong_clamp
            .state
            .outdoor_air_mass_flow_clamp_assignment_count = 1;
        assert!(
            purchased_air_cooling_oa_max_flow_body::validate_direct_lifecycle(
                Some(&wrong_clamp),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut wrong_index = valid.clone();
        wrong_index.state.outdoor_air_flow_max_cooling_output_index = 1;
        assert!(
            purchased_air_cooling_oa_max_flow_body::validate_direct_lifecycle(
                Some(&wrong_index),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut wrong_latest = valid.clone();
        wrong_latest
            .state
            .latest
            .as_mut()
            .expect("valid latest cooling OA maximum-flow body")
            .warning_counter_read = true;
        assert!(
            purchased_air_cooling_oa_max_flow_body::validate_direct_lifecycle(
                Some(&wrong_latest),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut overflowed_partition = valid;
        overflowed_partition.state.unit_off_skip_count = usize::MAX;
        overflowed_partition.state.non_cooling_skip_count = 1;
        assert!(
            purchased_air_cooling_oa_max_flow_body::validate_direct_lifecycle(
                Some(&overflowed_partition),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );
    }

    #[test]
    fn direct_release_cooling_oa_max_flow_body_json_exposes_zero_effect_skip() {
        let lifecycle = valid_cooling_oa_max_flow_body_lifecycle(2);
        let value = purchased_air_cooling_oa_max_flow_body::lifecycle_json(&lifecycle);

        assert_eq!(
            value["source"],
            PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE
        );
        assert_eq!(
            value["first_excluded_source"],
            PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_FIRST_EXCLUDED_SOURCE
        );
        assert_eq!(
            value["recurring_warning_child_source"],
            PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_RECURRING_WARNING_CHILD_SOURCE
        );
        assert_eq!(value["transition_count"], 2);
        assert_eq!(value["body_entry_count"], 0);
        assert_eq!(value["body_skip_count"], 2);
        assert_eq!(value["active_guard_false_economizer_fallthrough_count"], 2);
        assert_eq!(value["warning_counter_read_count"], 0);
        assert_eq!(value["outdoor_air_flow_max_cooling_output_index"], 0);
        assert_eq!(
            value["characterized_total_warning_error_increment_count"],
            0
        );
        assert_eq!(value["outdoor_air_mass_flow_clamp_assignment_count"], 0);
        assert_eq!(value["latest"]["body_skipped"], true);
        assert_eq!(
            value["latest"]["active_guard_false_economizer_fallthrough"],
            true
        );
        assert_eq!(value["latest"]["warning_counter_read"], false);
        assert!(value["latest"]["warning_counter_before"].is_null());
        assert_eq!(
            value["latest"]["outdoor_air_mass_flow_clamp_assignment_performed"],
            false
        );
    }

    #[test]
    fn direct_release_cooling_economizer_guard_validation_rejects_malformed_evidence() {
        let init = valid_init_lifecycle(2);
        let predecessor = valid_cooling_oa_max_flow_body_lifecycle(2);
        let valid = valid_cooling_economizer_guard_lifecycle(2);
        assert!(
            purchased_air_cooling_economizer_guard::validate_direct_lifecycle(
                Some(&valid),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_ok()
        );
        assert!(
            purchased_air_cooling_economizer_guard::validate_direct_lifecycle(
                None,
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut wrong_provenance = valid.clone();
        wrong_provenance.first_excluded_source = PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE;
        assert!(
            purchased_air_cooling_economizer_guard::validate_direct_lifecycle(
                Some(&wrong_provenance),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut wrong_count = valid.clone();
        wrong_count.state.economizer_body_entry_count = 1;
        assert!(
            purchased_air_cooling_economizer_guard::validate_direct_lifecycle(
                Some(&wrong_count),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut wrong_latest = valid.clone();
        wrong_latest
            .state
            .latest
            .as_mut()
            .expect("valid latest cooling economizer guard")
            .economizer_type = Some(OutdoorAirEconomizerType::DifferentialDryBulb);
        assert!(
            purchased_air_cooling_economizer_guard::validate_direct_lifecycle(
                Some(&wrong_latest),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut overflowed_partition = valid;
        overflowed_partition.state.unit_off_skip_count = usize::MAX;
        overflowed_partition.state.non_cooling_skip_count = 1;
        assert!(
            purchased_air_cooling_economizer_guard::validate_direct_lifecycle(
                Some(&overflowed_partition),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );
    }

    #[test]
    fn direct_release_cooling_economizer_guard_json_exposes_no_economizer_fallthrough() {
        let lifecycle = valid_cooling_economizer_guard_lifecycle(2);
        let value = purchased_air_cooling_economizer_guard::lifecycle_json(&lifecycle);

        assert_eq!(
            value["source"],
            PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE
        );
        assert_eq!(
            value["first_excluded_source"],
            PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_FIRST_EXCLUDED_SOURCE
        );
        assert_eq!(value["transition_count"], 2);
        assert_eq!(value["guard_evaluation_count"], 2);
        assert_eq!(value["economizer_type_read_count"], 2);
        assert_eq!(value["no_economizer_comparison_count"], 2);
        assert_eq!(value["economizer_body_entry_count"], 0);
        assert_eq!(value["no_economizer_fallthrough_count"], 2);
        assert_eq!(value["latest"]["economizer_type"], "NoEconomizer");
        assert_eq!(value["latest"]["economizer_not_no_economizer"], false);
        assert_eq!(value["latest"]["economizer_body_entered"], false);
        assert_eq!(value["latest"]["no_economizer_fallthrough"], true);
    }

    #[test]
    fn direct_release_cooling_economizer_condition_validation_rejects_malformed_evidence() {
        let init = valid_init_lifecycle(2);
        let predecessor = valid_cooling_economizer_guard_lifecycle(2);
        let valid = valid_cooling_economizer_condition_lifecycle(2);
        assert!(
            purchased_air_cooling_economizer_condition::validate_direct_lifecycle(
                Some(&valid),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_ok()
        );
        assert!(
            purchased_air_cooling_economizer_condition::validate_direct_lifecycle(
                None,
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut wrong_provenance = valid.clone();
        wrong_provenance.first_excluded_source =
            PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE;
        assert!(
            purchased_air_cooling_economizer_condition::validate_direct_lifecycle(
                Some(&wrong_provenance),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut wrong_count = valid.clone();
        wrong_count.state.condition_evaluation_count = 1;
        assert!(
            purchased_air_cooling_economizer_condition::validate_direct_lifecycle(
                Some(&wrong_count),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut wrong_latest = valid.clone();
        wrong_latest
            .state
            .latest
            .as_mut()
            .expect("valid latest cooling economizer condition")
            .differential_dry_bulb_economizer_type_read = true;
        assert!(
            purchased_air_cooling_economizer_condition::validate_direct_lifecycle(
                Some(&wrong_latest),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut wrong_predecessor_link = valid.clone();
        wrong_predecessor_link
            .state
            .latest
            .as_mut()
            .expect("valid latest cooling economizer condition")
            .predecessor_no_economizer_fallthrough = false;
        assert!(
            purchased_air_cooling_economizer_condition::validate_direct_lifecycle(
                Some(&wrong_predecessor_link),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut overflowed_partition = valid;
        overflowed_partition.state.unit_off_skip_count = usize::MAX;
        overflowed_partition.state.non_cooling_skip_count = 1;
        assert!(
            purchased_air_cooling_economizer_condition::validate_direct_lifecycle(
                Some(&overflowed_partition),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );
    }

    #[test]
    fn non_direct_runtime_rejects_cp316_through_cp352_lifecycle_evidence() {
        let mut result = RustRuntimeResult {
            results: ResultStore::new(),
            runtime_class: RuntimeClass::IdealLoadsFixtureDemandDiagnostic,
            sample_count: 1,
            schedule_cache_sample_count: 1,
            schedule_cache_profile: ScheduleCacheProfile {
                scalar_series_count: 0,
                dense_series_count: 0,
                logical_sample_count: 0,
                allocated_dense_sample_count: 0,
                index_kind: ScheduleSeriesIndexKind::DenseIdentity,
                ambiguous_id_count: 0,
            },
            source_order_gate: SourceOrderGateSummary {
                expected_source_order_stages: Vec::new(),
                actual_executed_source_order_stages: Vec::new(),
                matches: true,
            },
            zone_demand_source: Some(IDEAL_LOADS_FIXTURE_DEMAND_DIAGNOSTIC_SOURCE.to_string()),
            fixture_demand_injection_used: Some(true),
            purchased_air_branch: None,
            recirculation_node: None,
            recirculation_state_source: None,
            actual_coupled_source_order: None,
            purchased_air_coupling_call_count: None,
            purchased_air_init_lifecycle: None,
            purchased_air_calc_entry_lifecycle: None,
            purchased_air_calc_minimum_oa_prefix_lifecycle: None,
            purchased_air_calc_cooling_entry_gate_lifecycle: None,
            purchased_air_calc_cooling_oa_max_flow_gate_lifecycle: None,
            purchased_air_calc_cooling_oa_max_flow_body_lifecycle: None,
            purchased_air_calc_cooling_economizer_guard_lifecycle: None,
            purchased_air_calc_cooling_economizer_condition_lifecycle: None,
            purchased_air_calc_cooling_economizer_body_lifecycle: None,
            purchased_air_calc_cooling_sensible_flow_lifecycle: None,
            purchased_air_calc_cooling_dehumidification_flow_lifecycle: None,
            purchased_air_calc_cooling_humidification_flow_lifecycle: None,
            purchased_air_calc_cooling_capacity_zero_flow_reset_lifecycle: None,
            purchased_air_calc_cooling_supply_mass_flow_maximum_lifecycle: None,
            purchased_air_calc_cooling_supply_mass_flow_ems_override_guard_lifecycle: None,
            purchased_air_calc_cooling_supply_mass_flow_ems_override_body_lifecycle: None,
            purchased_air_calc_cooling_supply_mass_flow_limit_guard_lifecycle: None,
            purchased_air_calc_cooling_supply_mass_flow_limit_body_lifecycle: None,
            purchased_air_calc_cooling_supply_mass_flow_very_small_guard_lifecycle: None,
            purchased_air_calc_cooling_supply_mass_flow_very_small_guard_body_lifecycle: None,
            purchased_air_calc_cooling_mixed_air_call_lifecycle: None,
            purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle: None,
            purchased_air_calc_cooling_positive_supply_cp_air_assignment_lifecycle: None,
            purchased_air_calc_cooling_positive_supply_temperature_assignment_lifecycle: None,
            purchased_air_calc_cooling_positive_supply_temperature_minimum_limit_lifecycle: None,
            purchased_air_calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle: None,
            purchased_air_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle:
                None,
            purchased_air_calc_cooling_positive_supply_enthalpy_assignment_lifecycle: None,
            purchased_air_calc_cooling_positive_supply_capacity_limit_guard_lifecycle: None,
            purchased_air_calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle:
                None,
            purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle:
                None,
            purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle:
                None,
            purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_lifecycle:
                None,
            purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_lifecycle:
                None,
            purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_lifecycle:
                None,
            purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle:
                None,
            purchased_air_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle:
                None,
            purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_lifecycle:
                None,
            purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_lifecycle:
                None,
            purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_lifecycle:
                None,
            purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_lifecycle:
                None,
            purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_lifecycle:
                None,
            purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_lifecycle:
                None,
            purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_lifecycle:
                None,
        };
        assert!(
            validate_runtime_demand_provenance(RunResultState::PartialSupportedRun, &result, None)
                .is_ok()
        );

        result.purchased_air_calc_cooling_economizer_condition_lifecycle =
            Some(valid_cooling_economizer_condition_lifecycle(1));
        assert_eq!(
            validate_runtime_demand_provenance(RunResultState::PartialSupportedRun, &result, None),
            Err(
                "persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime"
                    .to_string()
            )
        );

        result.purchased_air_calc_cooling_economizer_condition_lifecycle = None;
        result.purchased_air_calc_cooling_economizer_body_lifecycle =
            Some(valid_cooling_economizer_body_lifecycle(1));
        assert_eq!(
            validate_runtime_demand_provenance(RunResultState::PartialSupportedRun, &result, None),
            Err(
                "persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime"
                    .to_string()
            )
        );

        result.purchased_air_calc_cooling_economizer_body_lifecycle = None;
        result.purchased_air_calc_cooling_sensible_flow_lifecycle =
            Some(valid_cooling_sensible_flow_lifecycle(1));
        assert_eq!(
            validate_runtime_demand_provenance(RunResultState::PartialSupportedRun, &result, None),
            Err(
                "persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime"
                    .to_string()
            )
        );

        result.purchased_air_calc_cooling_sensible_flow_lifecycle = None;
        result.purchased_air_calc_cooling_dehumidification_flow_lifecycle =
            Some(valid_cooling_dehumidification_flow_lifecycle(1));
        assert_eq!(
            validate_runtime_demand_provenance(RunResultState::PartialSupportedRun, &result, None),
            Err(
                "persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime"
                    .to_string()
            )
        );

        result.purchased_air_calc_cooling_dehumidification_flow_lifecycle = None;
        result.purchased_air_calc_cooling_humidification_flow_lifecycle =
            Some(valid_cooling_humidification_flow_lifecycle(1));
        assert_eq!(
            validate_runtime_demand_provenance(RunResultState::PartialSupportedRun, &result, None),
            Err(
                "persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime"
                    .to_string()
            )
        );

        result.purchased_air_calc_cooling_humidification_flow_lifecycle = None;
        result.purchased_air_calc_cooling_supply_mass_flow_maximum_lifecycle =
            Some(ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowMaximumLifecycleSummary {
                source: ep_runtime::PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE,
                first_excluded_source:
                    ep_runtime::PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_FIRST_EXCLUDED_SOURCE,
                state: ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowMaximumRuntimeState::new(
                    IdealLoadsAirSystemId(0),
                ),
            });
        assert_eq!(
            validate_runtime_demand_provenance(RunResultState::PartialSupportedRun, &result, None),
            Err(
                "persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime"
                    .to_string()
            )
        );

        result.purchased_air_calc_cooling_supply_mass_flow_maximum_lifecycle = None;
        result.purchased_air_calc_cooling_supply_mass_flow_ems_override_guard_lifecycle =
            Some(
                ep_runtime::
                    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardLifecycleSummary {
                    source: ep_runtime::
                        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE,
                    first_excluded_source: ep_runtime::
                        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_FIRST_EXCLUDED_SOURCE,
                    state: ep_runtime::
                        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRuntimeState::new(
                            IdealLoadsAirSystemId(0),
                        ),
                },
            );
        assert_eq!(
            validate_runtime_demand_provenance(RunResultState::PartialSupportedRun, &result, None),
            Err(
                "persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime"
                    .to_string()
            )
        );

        result.purchased_air_calc_cooling_supply_mass_flow_ems_override_guard_lifecycle = None;
        result.purchased_air_calc_cooling_supply_mass_flow_ems_override_body_lifecycle =
            Some(
                ep_runtime::
                    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyLifecycleSummary {
                    source: ep_runtime::
                        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE,
                    first_excluded_source: ep_runtime::
                        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_FIRST_EXCLUDED_SOURCE,
                    state: ep_runtime::
                        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRuntimeState::new(
                            IdealLoadsAirSystemId(0),
                        ),
                },
            );
        assert_eq!(
            validate_runtime_demand_provenance(RunResultState::PartialSupportedRun, &result, None),
            Err(
                "persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime"
                    .to_string()
            )
        );

        result.purchased_air_calc_cooling_supply_mass_flow_ems_override_body_lifecycle = None;
        result.purchased_air_calc_cooling_supply_mass_flow_limit_guard_lifecycle = Some(
            ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowLimitGuardLifecycleSummary {
                source:
                    ep_runtime::PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE,
                first_excluded_source: ep_runtime::
                    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
                state:
                    ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRuntimeState::new(
                        IdealLoadsAirSystemId(0),
                    ),
            },
        );
        assert_eq!(
            validate_runtime_demand_provenance(RunResultState::PartialSupportedRun, &result, None),
            Err(
                "persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime"
                    .to_string()
            )
        );

        result.purchased_air_calc_cooling_supply_mass_flow_limit_guard_lifecycle = None;
        result.purchased_air_calc_cooling_supply_mass_flow_limit_body_lifecycle = Some(
            ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowLimitBodyLifecycleSummary {
                source: ep_runtime::PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE,
                first_excluded_source: ep_runtime::
                    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_FIRST_EXCLUDED_SOURCE,
                state: ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRuntimeState::new(
                    IdealLoadsAirSystemId(0),
                ),
            },
        );
        assert_eq!(
            validate_runtime_demand_provenance(RunResultState::PartialSupportedRun, &result, None),
            Err(
                "persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime"
                    .to_string()
            )
        );

        result.purchased_air_calc_cooling_supply_mass_flow_limit_body_lifecycle = None;
        result.purchased_air_calc_cooling_supply_mass_flow_very_small_guard_lifecycle = Some(
            ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardLifecycleSummary {
                source: ep_runtime::
                    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE,
                first_excluded_source: ep_runtime::
                    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_FIRST_EXCLUDED_SOURCE,
                state: ep_runtime::
                    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRuntimeState::new(
                        IdealLoadsAirSystemId(0),
                    ),
            },
        );
        assert_eq!(
            validate_runtime_demand_provenance(RunResultState::PartialSupportedRun, &result, None),
            Err(
                "persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime"
                    .to_string()
            )
        );

        result.purchased_air_calc_cooling_supply_mass_flow_very_small_guard_lifecycle = None;
        result.purchased_air_calc_cooling_supply_mass_flow_very_small_guard_body_lifecycle = Some(
            ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyLifecycleSummary {
                source: ep_runtime::
                    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_SOURCE,
                first_excluded_source: ep_runtime::
                    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_FIRST_EXCLUDED_SOURCE,
                state: ep_runtime::
                    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyRuntimeState::new(
                        IdealLoadsAirSystemId(0),
                    ),
            },
        );
        assert_eq!(
            validate_runtime_demand_provenance(RunResultState::PartialSupportedRun, &result, None),
            Err(
                "persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime"
                    .to_string()
            )
        );

        result.purchased_air_calc_cooling_supply_mass_flow_very_small_guard_body_lifecycle = None;
        result.purchased_air_calc_cooling_mixed_air_call_lifecycle = Some(
            ep_runtime::PurchasedAirCalcCoolingMixedAirCallLifecycleSummary {
                source: ep_runtime::PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE,
                child_source: ep_runtime::PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE,
                first_excluded_source:
                    ep_runtime::PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE,
                state: ep_runtime::PurchasedAirCalcCoolingMixedAirCallRuntimeState::new(
                    IdealLoadsAirSystemId(0),
                ),
            },
        );
        assert_eq!(
            validate_runtime_demand_provenance(RunResultState::PartialSupportedRun, &result, None),
            Err(
                "persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime"
                    .to_string()
            )
        );

        result.purchased_air_calc_cooling_mixed_air_call_lifecycle = None;
        result.purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle = Some(
            ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardLifecycleSummary {
                source: ep_runtime::
                    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE,
                first_excluded_source: ep_runtime::
                    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE,
                state: ep_runtime::
                    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRuntimeState::new(
                        IdealLoadsAirSystemId(0),
                    ),
            },
        );
        assert_eq!(
            validate_runtime_demand_provenance(RunResultState::PartialSupportedRun, &result, None),
            Err(
                "persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime"
                    .to_string()
            )
        );

        result.purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle = None;
        result.purchased_air_calc_cooling_positive_supply_cp_air_assignment_lifecycle = Some(
            ep_runtime::PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentLifecycleSummary {
                source: ep_runtime::
                    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_SOURCE,
                first_excluded_source: ep_runtime::
                    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
                state: ep_runtime::
                    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentRuntimeState::new(
                        IdealLoadsAirSystemId(0),
                    ),
            },
        );
        assert_eq!(
            validate_runtime_demand_provenance(RunResultState::PartialSupportedRun, &result, None),
            Err(
                "persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime"
                    .to_string()
            )
        );

        result.purchased_air_calc_cooling_positive_supply_cp_air_assignment_lifecycle = None;
        result.purchased_air_calc_cooling_positive_supply_temperature_assignment_lifecycle = Some(
            ep_runtime::PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentLifecycleSummary {
                source: ep_runtime::
                    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
                first_excluded_source: ep_runtime::
                    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
                state: ep_runtime::
                    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRuntimeState::new(
                        IdealLoadsAirSystemId(0),
                    ),
            },
        );
        assert_eq!(
            validate_runtime_demand_provenance(RunResultState::PartialSupportedRun, &result, None),
            Err(
                "persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime"
                    .to_string()
            )
        );

        result.purchased_air_calc_cooling_positive_supply_temperature_assignment_lifecycle = None;
        result
            .purchased_air_calc_cooling_positive_supply_temperature_minimum_limit_lifecycle = Some(
            ep_runtime::PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitLifecycleSummary {
                source: ep_runtime::
                    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_SOURCE,
                first_excluded_source: ep_runtime::
                    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_FIRST_EXCLUDED_SOURCE,
                state: ep_runtime::
                    PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitRuntimeState::new(
                        IdealLoadsAirSystemId(0),
                    ),
            },
        );
        assert_eq!(
            validate_runtime_demand_provenance(RunResultState::PartialSupportedRun, &result, None),
            Err(
                "persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime"
                    .to_string()
            )
        );

        result.purchased_air_calc_cooling_positive_supply_temperature_minimum_limit_lifecycle =
            None;
        result
            .purchased_air_calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle =
            Some(
                ep_runtime::PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitLifecycleSummary {
                    source: ep_runtime::
                        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
                    first_excluded_source: ep_runtime::
                        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
                    state: ep_runtime::
                        PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitRuntimeState::new(
                            IdealLoadsAirSystemId(0),
                        ),
                },
            );
        assert_eq!(
            validate_runtime_demand_provenance(RunResultState::PartialSupportedRun, &result, None),
            Err(
                "persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime"
                    .to_string()
            )
        );

        result.purchased_air_calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle =
            None;
        result
            .purchased_air_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle =
            Some(
                ep_runtime::PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentLifecycleSummary {
                    source: ep_runtime::
                        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE,
                    first_excluded_source: ep_runtime::
                        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
                    state: ep_runtime::
                        PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentRuntimeState::new(
                            IdealLoadsAirSystemId(0),
                        ),
                },
            );
        assert_eq!(
            validate_runtime_demand_provenance(RunResultState::PartialSupportedRun, &result, None),
            Err(
                "persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime"
                    .to_string()
            )
        );

        result
            .purchased_air_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle =
            None;
        result.purchased_air_calc_cooling_positive_supply_enthalpy_assignment_lifecycle = Some(
            ep_runtime::PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentLifecycleSummary {
                source: ep_runtime::
                    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
                first_excluded_source: ep_runtime::
                    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
                state: ep_runtime::
                    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentRuntimeState::new(
                        IdealLoadsAirSystemId(0),
                    ),
            },
        );
        assert_eq!(
            validate_runtime_demand_provenance(RunResultState::PartialSupportedRun, &result, None),
            Err(
                "persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime"
                    .to_string()
            )
        );

        result.purchased_air_calc_cooling_positive_supply_enthalpy_assignment_lifecycle = None;
        result.purchased_air_calc_cooling_positive_supply_capacity_limit_guard_lifecycle = Some(
            ep_runtime::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardLifecycleSummary {
                source: ep_runtime::
                    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE,
                first_excluded_source: ep_runtime::
                    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
                state: ep_runtime::
                    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRuntimeState::new(
                        IdealLoadsAirSystemId(0),
                    ),
            },
        );
        assert_eq!(
            validate_runtime_demand_provenance(RunResultState::PartialSupportedRun, &result, None),
            Err(
                "persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime"
                    .to_string()
            )
        );

        result.purchased_air_calc_cooling_positive_supply_capacity_limit_guard_lifecycle = None;
        result
            .purchased_air_calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle =
            Some(
                ep_runtime::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentLifecycleSummary {
                    source: ep_runtime::
                        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_SOURCE,
                    first_excluded_source: ep_runtime::
                        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
                    state: ep_runtime::
                        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentRuntimeState::new(
                            IdealLoadsAirSystemId(0),
                        ),
                },
            );
        assert_eq!(
            validate_runtime_demand_provenance(RunResultState::PartialSupportedRun, &result, None),
            Err(
                "persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime"
                    .to_string()
            )
        );

        result
            .purchased_air_calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle =
            None;
        result
            .purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle =
            Some(
                ep_runtime::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentLifecycleSummary {
                    source: ep_runtime::
                        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
                    first_excluded_source: ep_runtime::
                        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
                    state: ep_runtime::
                        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRuntimeState::new(
                            IdealLoadsAirSystemId(0),
                        ),
                },
            );
        assert_eq!(
            validate_runtime_demand_provenance(RunResultState::PartialSupportedRun, &result, None),
            Err(
                "persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime"
                    .to_string()
            )
        );

        result
            .purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle =
            None;
        result
            .purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle =
            Some(
                ep_runtime::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardLifecycleSummary {
                    source: ep_runtime::
                        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE,
                    first_excluded_source: ep_runtime::
                        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
                    state: ep_runtime::
                        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardRuntimeState::new(
                            IdealLoadsAirSystemId(0),
                        ),
                },
            );
        assert_eq!(
            validate_runtime_demand_provenance(RunResultState::PartialSupportedRun, &result, None),
            Err(
                "persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime"
                    .to_string()
            )
        );

        result
            .purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle =
            None;
        result
            .purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_lifecycle =
            Some(
                ep_runtime::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentLifecycleSummary {
                    source: ep_runtime::
                        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
                    first_excluded_source: ep_runtime::
                        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
                    state: ep_runtime::
                        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentRuntimeState::new(
                            IdealLoadsAirSystemId(0),
                        ),
                },
            );
        assert_eq!(
            validate_runtime_demand_provenance(RunResultState::PartialSupportedRun, &result, None),
            Err(
                "persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime"
                    .to_string()
            )
        );

        result
            .purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_lifecycle =
            None;
        result
            .purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_lifecycle =
            Some(
                ep_runtime::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentLifecycleSummary {
                    source: ep_runtime::
                        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
                    first_excluded_source: ep_runtime::
                        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
                    state: ep_runtime::
                        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentRuntimeState::new(
                            IdealLoadsAirSystemId(0),
                        ),
                },
            );
        assert_eq!(
            validate_runtime_demand_provenance(RunResultState::PartialSupportedRun, &result, None),
            Err(
                "persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime"
                    .to_string()
            )
        );

        result
            .purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_lifecycle =
            None;
        result
            .purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_lifecycle =
            Some(
                ep_runtime::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentLifecycleSummary {
                    source: ep_runtime::
                        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
                    first_excluded_source: ep_runtime::
                        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
                    state: ep_runtime::
                        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentRuntimeState::new(
                            IdealLoadsAirSystemId(0),
                        ),
                },
            );
        assert_eq!(
            validate_runtime_demand_provenance(RunResultState::PartialSupportedRun, &result, None),
            Err(
                "persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime"
                    .to_string()
            )
        );

        result
            .purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_lifecycle =
            None;
        result
            .purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle =
            Some(
                ep_runtime::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitLifecycleSummary {
                    source: ep_runtime::
                        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
                    first_excluded_source: ep_runtime::
                        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
                    state: ep_runtime::
                        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRuntimeState::new(
                            IdealLoadsAirSystemId(0),
                        ),
                },
            );
        assert_eq!(
            validate_runtime_demand_provenance(RunResultState::PartialSupportedRun, &result, None),
            Err(
                "persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime"
                    .to_string()
            )
        );

        result
            .purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle =
            None;
        result
            .purchased_air_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle =
            Some(
                ep_runtime::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentLifecycleSummary {
                    source: ep_runtime::
                        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE,
                    first_excluded_source: ep_runtime::
                        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
                    state: ep_runtime::
                        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentRuntimeState::new(
                            IdealLoadsAirSystemId(0),
                        ),
                },
            );
        assert_eq!(
            validate_runtime_demand_provenance(RunResultState::PartialSupportedRun, &result, None),
            Err(
                "persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime"
                    .to_string()
            )
        );

        result
            .purchased_air_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle =
            None;
        result
            .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_lifecycle =
            Some(
                ep_runtime::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchLifecycleSummary {
                    source: ep_runtime::
                        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE,
                    first_excluded_source: ep_runtime::
                        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_SOURCE,
                    state: ep_runtime::
                        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchRuntimeState::new(
                            IdealLoadsAirSystemId(0),
                        ),
                },
            );
        assert_eq!(
            validate_runtime_demand_provenance(RunResultState::PartialSupportedRun, &result, None),
            Err(
                "persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime"
                    .to_string()
            )
        );

        result
            .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_lifecycle =
            None;
        result
            .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_lifecycle =
            Some(
                ep_runtime::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseLifecycleSummary {
                    source: ep_runtime::
                        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE,
                    first_excluded_source: ep_runtime::
                        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_FIRST_EXCLUDED_SOURCE,
                    state: ep_runtime::
                        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseRuntimeState::new(
                            IdealLoadsAirSystemId(0),
                        ),
                },
            );
        assert_eq!(
            validate_runtime_demand_provenance(RunResultState::PartialSupportedRun, &result, None),
            Err(
                "persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime"
                    .to_string()
            )
        );

        result
            .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_lifecycle =
            None;
        result
            .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_lifecycle =
            Some(
                ep_runtime::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntryLifecycleSummary {
                    source: ep_runtime::
                        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CASE_ENTRY_SOURCE,
                    first_excluded_source: ep_runtime::
                        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CASE_ENTRY_FIRST_EXCLUDED_SOURCE,
                    state: ep_runtime::
                        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntryRuntimeState::new(
                            IdealLoadsAirSystemId(0),
                        ),
                },
            );
        assert_eq!(
            validate_runtime_demand_provenance(RunResultState::PartialSupportedRun, &result, None),
            Err(
                "persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime"
                    .to_string()
            )
        );

        result
            .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_lifecycle =
            None;
        result
            .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_lifecycle =
            Some(
                ep_runtime::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentLifecycleSummary {
                    source: ep_runtime::
                        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE,
                    first_excluded_source: ep_runtime::
                        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
                    state: ep_runtime::
                        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentRuntimeState::new(
                            IdealLoadsAirSystemId(0),
                        ),
                },
            );
        assert_eq!(
            validate_runtime_demand_provenance(RunResultState::PartialSupportedRun, &result, None),
            Err(
                "persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime"
                    .to_string()
            )
        );
        result
            .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_lifecycle =
            None;
        result
            .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_lifecycle =
            Some(
                ep_runtime::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentLifecycleSummary {
                    source: ep_runtime::
                        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
                    first_excluded_source: ep_runtime::
                        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
                    state: ep_runtime::
                        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentRuntimeState::new(
                            IdealLoadsAirSystemId(0),
                        ),
                },
            );
        assert_eq!(
            validate_runtime_demand_provenance(RunResultState::PartialSupportedRun, &result, None),
            Err(
                "persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime"
                    .to_string()
            )
        );
        result
            .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_lifecycle =
            None;
        result
            .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_lifecycle =
            Some(
                ep_runtime::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignmentLifecycleSummary {
                    source: ep_runtime::
                        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_TOTAL_OUTPUT_ASSIGNMENT_SOURCE,
                    first_excluded_source: ep_runtime::
                        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_TOTAL_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
                    state: ep_runtime::
                        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignmentRuntimeState::new(
                            IdealLoadsAirSystemId(0),
                        ),
                },
            );
        assert_eq!(
            validate_runtime_demand_provenance(RunResultState::PartialSupportedRun, &result, None),
            Err(
                "persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime"
                    .to_string()
            )
        );
        result
            .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_lifecycle =
            None;
        result
            .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_lifecycle =
            Some(
                ep_runtime::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentLifecycleSummary {
                    source: ep_runtime::
                        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
                    first_excluded_source: ep_runtime::
                        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
                    state: ep_runtime::
                        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentRuntimeState::new(
                            IdealLoadsAirSystemId(0),
                        ),
                },
            );
        assert_eq!(
            validate_runtime_demand_provenance(RunResultState::PartialSupportedRun, &result, None),
            Err(
                "persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime"
                    .to_string()
            )
        );
        result
            .purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_lifecycle =
            None;
    }

    #[test]
    fn direct_release_cooling_economizer_condition_json_exposes_zero_evidence_skip() {
        let lifecycle = valid_cooling_economizer_condition_lifecycle(2);
        let value = purchased_air_cooling_economizer_condition::lifecycle_json(&lifecycle);

        assert_eq!(
            value["source"],
            PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE
        );
        assert_eq!(
            value["first_excluded_source"],
            PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_FIRST_EXCLUDED_SOURCE
        );
        assert_eq!(value["system"], 0);
        assert_eq!(value["transition_count"], 2);
        assert_eq!(value["condition_evaluation_count"], 0);
        assert_eq!(value["no_economizer_outer_guard_fallthrough_skip_count"], 2);
        assert_eq!(value["differential_dry_bulb_economizer_type_read_count"], 0);
        assert_eq!(value["differential_enthalpy_economizer_type_read_count"], 0);
        assert_eq!(value["outdoor_air_temperature_read_count"], 0);
        assert_eq!(value["outdoor_air_enthalpy_read_count"], 0);
        assert_eq!(value["economizer_calculation_body_entry_count"], 0);
        assert_eq!(value["economizer_condition_fallthrough_count"], 0);
        for field in [
            "differential_dry_bulb_selector_comparison_count",
            "differential_dry_bulb_selector_match_count",
            "recirculation_air_temperature_read_count",
            "dry_bulb_temperature_comparison_count",
            "dry_bulb_temperature_comparison_satisfied_count",
            "differential_enthalpy_selector_comparison_count",
            "differential_enthalpy_selector_match_count",
            "recirculation_air_enthalpy_read_count",
            "enthalpy_comparison_count",
            "enthalpy_comparison_satisfied_count",
        ] {
            assert_eq!(value[field], 0, "{field}");
        }
        let latest = &value["latest"];
        assert_eq!(latest["system"], 0);
        assert_eq!(latest["controlled_zone"], 0);
        assert_eq!(
            latest["source_order"],
            serde_json::json!(PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE_ORDER)
        );
        assert_eq!(latest["unit_body_entered"], true);
        assert_eq!(latest["predecessor_cooling_body_entered"], true);
        assert_eq!(
            latest["predecessor_maximum_cooling_flow_body_entered"],
            false
        );
        assert_eq!(
            latest["predecessor_active_guard_false_economizer_fallthrough"],
            true
        );
        assert_eq!(latest["predecessor_economizer_guard_evaluated"], true);
        assert_eq!(latest["predecessor_economizer_body_entered"], false);
        assert_eq!(latest["predecessor_no_economizer_fallthrough"], true);
        assert_eq!(
            latest["no_economizer_outer_guard_fallthrough_skipped"],
            true
        );
        assert_eq!(latest["economizer_condition_evaluated"], false);
        for field in [
            "differential_dry_bulb_economizer_type_read",
            "differential_dry_bulb_selector_comparison_evaluated",
            "outdoor_air_temperature_read",
            "recirculation_air_temperature_read",
            "dry_bulb_temperature_comparison_evaluated",
            "differential_enthalpy_economizer_type_read",
            "differential_enthalpy_selector_comparison_evaluated",
            "outdoor_air_enthalpy_read",
            "recirculation_air_enthalpy_read",
            "enthalpy_comparison_evaluated",
        ] {
            assert_eq!(latest[field], false, "{field}");
        }
        assert!(latest["differential_dry_bulb_economizer_type"].is_null());
        assert!(latest["differential_enthalpy_economizer_type"].is_null());
        assert!(latest["economizer_condition_satisfied"].is_null());
        assert_eq!(latest["economizer_calculation_body_entered"], false);
        assert_eq!(latest["economizer_condition_fallthrough"], false);
    }

    #[test]
    fn direct_release_cooling_economizer_body_validation_rejects_malformed_evidence() {
        let init = valid_init_lifecycle(2);
        let predecessor = valid_cooling_economizer_condition_lifecycle(2);
        let valid = valid_cooling_economizer_body_lifecycle(2);
        assert!(
            purchased_air_cooling_economizer_body::validate_direct_lifecycle(
                Some(&valid),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_ok()
        );
        assert!(
            purchased_air_cooling_economizer_body::validate_direct_lifecycle(
                None,
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut wrong_provenance = valid.clone();
        wrong_provenance.first_excluded_source = PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE;
        assert!(
            purchased_air_cooling_economizer_body::validate_direct_lifecycle(
                Some(&wrong_provenance),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut wrong_count = valid.clone();
        wrong_count.state.zone_humidity_ratio_read_count = 1;
        assert!(
            purchased_air_cooling_economizer_body::validate_direct_lifecycle(
                Some(&wrong_count),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut wrong_expanded_count = valid.clone();
        wrong_expanded_count
            .state
            .maximum_cooling_air_mass_flow_rate_clamp_upper_bound_read_count = 1;
        assert!(
            purchased_air_cooling_economizer_body::validate_direct_lifecycle(
                Some(&wrong_expanded_count),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut wrong_latest = valid.clone();
        wrong_latest
            .state
            .latest
            .as_mut()
            .expect("valid latest cooling economizer body")
            .zone_humidity_ratio_read = true;
        assert!(
            purchased_air_cooling_economizer_body::validate_direct_lifecycle(
                Some(&wrong_latest),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut wrong_expanded_latest = valid.clone();
        wrong_expanded_latest
            .state
            .latest
            .as_mut()
            .expect("valid latest cooling economizer body")
            .cp_air_assigned = true;
        assert!(
            purchased_air_cooling_economizer_body::validate_direct_lifecycle(
                Some(&wrong_expanded_latest),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut wrong_expanded_value = valid.clone();
        wrong_expanded_value
            .state
            .latest
            .as_mut()
            .expect("valid latest cooling economizer body")
            .maximum_cooling_air_mass_flow_rate_clamp_upper_bound_kg_per_s = Some(1.0);
        assert!(
            purchased_air_cooling_economizer_body::validate_direct_lifecycle(
                Some(&wrong_expanded_value),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut wrong_predecessor_link = valid.clone();
        wrong_predecessor_link
            .state
            .latest
            .as_mut()
            .expect("valid latest cooling economizer body")
            .predecessor_no_economizer_fallthrough = false;
        assert!(
            purchased_air_cooling_economizer_body::validate_direct_lifecycle(
                Some(&wrong_predecessor_link),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut overflowed_partition = valid;
        overflowed_partition.state.unit_off_skip_count = usize::MAX;
        overflowed_partition.state.non_cooling_skip_count = 1;
        assert!(
            purchased_air_cooling_economizer_body::validate_direct_lifecycle(
                Some(&overflowed_partition),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );
    }

    #[test]
    fn direct_release_cooling_economizer_body_json_exposes_zero_evidence_skip() {
        let lifecycle = valid_cooling_economizer_body_lifecycle(2);
        let value = purchased_air_cooling_economizer_body::lifecycle_json(&lifecycle);

        assert_eq!(
            value["source"],
            PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE
        );
        assert_eq!(
            value["first_excluded_source"],
            PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_FIRST_EXCLUDED_SOURCE
        );
        assert_eq!(value["system"], 0);
        assert_eq!(value["transition_count"], 2);
        assert_eq!(value["body_execution_count"], 0);
        assert_eq!(value["no_economizer_outer_guard_fallthrough_skip_count"], 2);
        for field in [
            "economizer_condition_fallthrough_skip_count",
            "zone_humidity_ratio_read_count",
            "psychrometric_cp_air_evaluation_count",
            "cp_air_assignment_count",
            "outdoor_air_temperature_read_count",
            "zone_temperature_read_count",
            "delta_temperature_calculation_count",
            "delta_temperature_assignment_count",
            "delta_temperature_for_gate_read_count",
            "delta_temperature_comparison_count",
            "delta_temperature_comparison_satisfied_count",
            "delta_temperature_body_entry_count",
            "delta_temperature_fallthrough_count",
            "zone_cooling_setpoint_load_read_count",
            "cp_air_for_first_division_read_count",
            "zone_cooling_setpoint_load_over_cp_air_calculation_count",
            "delta_temperature_for_second_division_read_count",
            "supply_mass_flow_rate_calculation_count",
            "initial_supply_mass_flow_rate_assignment_count",
            "cooling_limit_flow_rate_read_count",
            "cooling_limit_flow_rate_comparison_count",
            "cooling_limit_flow_rate_match_count",
            "cooling_limit_flow_rate_and_capacity_read_count",
            "cooling_limit_flow_rate_and_capacity_comparison_count",
            "cooling_limit_flow_rate_and_capacity_match_count",
            "maximum_cooling_air_mass_flow_rate_read_count",
            "maximum_cooling_air_mass_flow_rate_positive_comparison_count",
            "maximum_cooling_air_mass_flow_rate_positive_count",
            "maximum_flow_clamp_body_entry_count",
            "supply_mass_flow_rate_for_clamp_read_count",
            "inner_max_evaluation_count",
            "maximum_cooling_air_mass_flow_rate_clamp_upper_bound_read_count",
            "outer_min_evaluation_count",
            "supply_mass_flow_rate_clamp_count",
            "clamped_supply_mass_flow_rate_assignment_count",
            "resulting_supply_mass_flow_rate_read_count",
            "outdoor_air_mass_flow_rate_read_count",
            "supply_above_outdoor_air_mass_flow_comparison_count",
            "supply_above_outdoor_air_mass_flow_comparison_satisfied_count",
            "economizer_activation_body_entry_count",
            "outdoor_air_mass_flow_comparison_fallthrough_count",
            "economizer_on_assignment_count",
            "supply_mass_flow_rate_for_outdoor_air_assignment_read_count",
            "outdoor_air_mass_flow_rate_assignment_count",
            "system_time_step_read_count",
            "economizer_active_time_assignment_count",
        ] {
            assert_eq!(value[field], 0, "{field}");
        }

        let latest = &value["latest"];
        assert_eq!(latest["system"], 0);
        assert_eq!(latest["controlled_zone"], 0);
        assert_eq!(
            latest["source_order"],
            serde_json::json!(PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE_ORDER)
        );
        assert_eq!(latest["predecessor_no_economizer_fallthrough"], true);
        assert_eq!(
            latest["no_economizer_outer_guard_fallthrough_skipped"],
            true
        );
        assert_eq!(latest["economizer_calculation_body_executed"], false);
        for field in [
            "zone_humidity_ratio_read",
            "psychrometric_cp_air_evaluated",
            "cp_air_assigned",
            "outdoor_air_temperature_read",
            "zone_temperature_read",
            "delta_temperature_calculated",
            "delta_temperature_assigned",
            "delta_temperature_for_gate_read",
            "delta_temperature_comparison_evaluated",
            "delta_temperature_body_entered",
            "zone_cooling_setpoint_load_read",
            "cp_air_for_first_division_read",
            "zone_cooling_setpoint_load_over_cp_air_calculated",
            "delta_temperature_for_second_division_read",
            "supply_mass_flow_rate_calculated",
            "initial_supply_mass_flow_rate_assigned",
            "cooling_limit_flow_rate_comparison_evaluated",
            "cooling_limit_flow_rate_read",
            "cooling_limit_flow_rate_and_capacity_comparison_evaluated",
            "cooling_limit_flow_rate_and_capacity_read",
            "maximum_cooling_air_mass_flow_rate_read",
            "maximum_cooling_air_mass_flow_rate_positive_comparison_evaluated",
            "maximum_flow_clamp_body_entered",
            "supply_mass_flow_rate_clamped",
            "supply_mass_flow_rate_for_clamp_read",
            "inner_max_evaluated",
            "maximum_cooling_air_mass_flow_rate_clamp_upper_bound_read",
            "outer_min_evaluated",
            "clamped_supply_mass_flow_rate_assigned",
            "resulting_supply_mass_flow_rate_read",
            "outdoor_air_mass_flow_rate_read",
            "supply_above_outdoor_air_mass_flow_comparison_evaluated",
            "economizer_activation_body_entered",
            "economizer_on_assigned",
            "supply_mass_flow_rate_for_outdoor_air_assignment_read",
            "outdoor_air_mass_flow_rate_assigned",
            "system_time_step_read",
            "economizer_active_time_assigned",
        ] {
            assert_eq!(latest[field], false, "{field}");
        }
        for field in [
            "zone_humidity_ratio",
            "psychrometric_cp_air_result_j_per_kg_k",
            "cp_air_j_per_kg_k",
            "outdoor_air_temperature_c",
            "zone_temperature_c",
            "delta_temperature_c",
            "assigned_delta_temperature_c",
            "delta_temperature_for_gate_c",
            "delta_temperature_below_negative_small_temp_diff",
            "zone_cooling_setpoint_load_w",
            "cp_air_for_first_division_j_per_kg_k",
            "zone_cooling_setpoint_load_over_cp_air_kg_k_per_s",
            "delta_temperature_for_second_division_c",
            "calculated_supply_mass_flow_rate_kg_per_s",
            "initial_supply_mass_flow_rate_kg_per_s",
            "cooling_limit_flow_rate_value",
            "cooling_limit_flow_rate_comparison_satisfied",
            "cooling_limit_flow_rate_and_capacity_value",
            "cooling_limit_flow_rate_and_capacity_comparison_satisfied",
            "cooling_flow_limit_active",
            "maximum_cooling_air_mass_flow_rate_kg_per_s",
            "maximum_cooling_air_mass_flow_rate_positive",
            "supply_mass_flow_rate_for_clamp_kg_per_s",
            "nonnegative_supply_mass_flow_rate_kg_per_s",
            "maximum_cooling_air_mass_flow_rate_clamp_upper_bound_kg_per_s",
            "clamped_supply_mass_flow_rate_kg_per_s",
            "resulting_supply_mass_flow_rate_kg_per_s",
            "outdoor_air_mass_flow_rate_kg_per_s",
            "supply_mass_flow_above_outdoor_air_mass_flow",
            "economizer_on",
            "supply_mass_flow_rate_for_outdoor_air_assignment_kg_per_s",
            "assigned_outdoor_air_mass_flow_rate_kg_per_s",
            "system_time_step_hours",
            "assigned_economizer_active_time_hours",
        ] {
            assert!(latest[field].is_null(), "{field}");
        }
    }

    #[test]
    fn direct_release_cooling_sensible_flow_validation_rejects_malformed_evidence() {
        let init = valid_init_lifecycle(2);
        let predecessor = valid_cooling_economizer_body_lifecycle(2);
        let valid = valid_cooling_sensible_flow_lifecycle(2);
        assert!(
            purchased_air_cooling_sensible_flow::validate_direct_lifecycle(
                Some(&valid),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_ok()
        );
        assert!(
            purchased_air_cooling_sensible_flow::validate_direct_lifecycle(
                None,
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut wrong_provenance = valid.clone();
        wrong_provenance.first_excluded_source = PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE;
        assert!(
            purchased_air_cooling_sensible_flow::validate_direct_lifecycle(
                Some(&wrong_provenance),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut wrong_count = valid.clone();
        wrong_count.state.cp_air_assignment_count = 1;
        assert!(
            purchased_air_cooling_sensible_flow::validate_direct_lifecycle(
                Some(&wrong_count),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut wrong_latest = valid.clone();
        wrong_latest
            .state
            .latest
            .as_mut()
            .expect("valid latest cooling sensible-flow snapshot")
            .assigned_supply_mass_flow_rate_for_cool_kg_per_s = Some(0.0);
        assert!(
            purchased_air_cooling_sensible_flow::validate_direct_lifecycle(
                Some(&wrong_latest),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut wrong_predecessor_link = valid.clone();
        wrong_predecessor_link
            .state
            .latest
            .as_mut()
            .expect("valid latest cooling sensible-flow snapshot")
            .predecessor_no_economizer_outer_guard_fallthrough_skipped = false;
        assert!(
            purchased_air_cooling_sensible_flow::validate_direct_lifecycle(
                Some(&wrong_predecessor_link),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut overflowed_partition = valid;
        overflowed_partition.state.unit_off_skip_count = usize::MAX;
        overflowed_partition.state.non_cooling_skip_count = 1;
        assert!(
            purchased_air_cooling_sensible_flow::validate_direct_lifecycle(
                Some(&overflowed_partition),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );
    }

    #[test]
    fn direct_release_cooling_sensible_flow_json_exposes_all_source_sites() {
        let lifecycle = valid_cooling_sensible_flow_lifecycle(2);
        let value = purchased_air_cooling_sensible_flow::lifecycle_json(&lifecycle);

        assert_eq!(
            value["source"],
            PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE
        );
        assert_eq!(
            value["first_excluded_source"],
            PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_FIRST_EXCLUDED_SOURCE
        );
        assert_eq!(value["system"], 0);
        assert_eq!(value["transition_count"], 2);
        assert_eq!(value["cooling_body_entry_count"], 2);
        assert_eq!(
            value["supply_mass_flow_rate_for_cool_reset_assignment_count"],
            2
        );
        assert_eq!(value["cooling_on_read_count"], 2);
        assert_eq!(value["cooling_on_body_entry_count"], 2);
        assert_eq!(value["cooling_on_fallthrough_count"], 0);
        assert_eq!(value["delta_temperature_comparison_satisfied_count"], 2);
        assert_eq!(value["delta_temperature_fallthrough_count"], 0);
        assert_eq!(value["supply_mass_flow_rate_for_cool_assignment_count"], 2);
        assert_eq!(value.as_object().map(serde_json::Map::len), Some(30));

        let latest = &value["latest"];
        assert_eq!(
            latest["source_order"],
            serde_json::json!(PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE_ORDER)
        );
        assert_eq!(latest["cooling_body_entered"], true);
        assert_eq!(
            latest["supply_mass_flow_rate_for_cool_reset_assigned"],
            true
        );
        assert_eq!(latest["cooling_on"], true);
        assert_eq!(
            latest["delta_temperature_below_negative_small_temp_diff"],
            true
        );
        assert_eq!(latest["supply_mass_flow_rate_for_cool_assigned"], true);
        assert_eq!(
            latest["calculated_supply_mass_flow_rate_for_cool_kg_per_s"],
            latest["assigned_supply_mass_flow_rate_for_cool_kg_per_s"]
        );
        assert_eq!(
            latest["assigned_supply_mass_flow_rate_for_cool_kg_per_s"],
            latest["resulting_supply_mass_flow_rate_for_cool_kg_per_s"]
        );
        assert_eq!(latest.as_object().map(serde_json::Map::len), Some(52));
    }

    #[test]
    fn direct_release_cooling_dehumidification_flow_validation_rejects_malformed_evidence() {
        let init = valid_init_lifecycle(2);
        let predecessor = valid_cooling_sensible_flow_lifecycle(2);
        let valid = valid_cooling_dehumidification_flow_lifecycle(2);
        assert!(
            purchased_air_cooling_dehumidification_flow::validate_direct_lifecycle(
                Some(&valid),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_ok()
        );
        assert!(
            purchased_air_cooling_dehumidification_flow::validate_direct_lifecycle(
                None,
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut wrong_provenance = valid.clone();
        wrong_provenance.first_excluded_source =
            PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SOURCE;
        assert!(
            purchased_air_cooling_dehumidification_flow::validate_direct_lifecycle(
                Some(&wrong_provenance),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut wrong_count = valid.clone();
        wrong_count
            .state
            .dehumidification_control_type_fallthrough_count = 1;
        assert!(
            purchased_air_cooling_dehumidification_flow::validate_direct_lifecycle(
                Some(&wrong_count),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut wrong_zero = valid.clone();
        wrong_zero
            .state
            .latest
            .as_mut()
            .expect("valid latest cooling dehumidification-flow snapshot")
            .reset_supply_mass_flow_rate_for_dehumidification_kg_per_s = Some(-0.0);
        assert!(
            purchased_air_cooling_dehumidification_flow::validate_direct_lifecycle(
                Some(&wrong_zero),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut wrong_selector = valid.clone();
        wrong_selector
            .state
            .latest
            .as_mut()
            .expect("valid latest cooling dehumidification-flow snapshot")
            .dehumidification_control_type = Some(DehumidificationControlType::Humidistat);
        assert!(
            purchased_air_cooling_dehumidification_flow::validate_direct_lifecycle(
                Some(&wrong_selector),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut wrong_predecessor_link = valid.clone();
        wrong_predecessor_link
            .state
            .latest
            .as_mut()
            .expect("valid latest cooling dehumidification-flow snapshot")
            .predecessor_supply_mass_flow_rate_for_cool_assigned = false;
        assert!(
            purchased_air_cooling_dehumidification_flow::validate_direct_lifecycle(
                Some(&wrong_predecessor_link),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut overflowed_partition = valid;
        overflowed_partition.state.unit_off_skip_count = usize::MAX;
        overflowed_partition.state.non_cooling_skip_count = 1;
        assert!(
            purchased_air_cooling_dehumidification_flow::validate_direct_lifecycle(
                Some(&overflowed_partition),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );
    }

    #[test]
    fn direct_release_cooling_dehumidification_flow_json_exposes_all_source_sites() {
        let lifecycle = valid_cooling_dehumidification_flow_lifecycle(2);
        let value = purchased_air_cooling_dehumidification_flow::lifecycle_json(&lifecycle);

        assert_eq!(
            value["source"],
            PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SOURCE
        );
        assert_eq!(
            value["first_excluded_source"],
            PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE
        );
        assert_eq!(value["system"], 0);
        assert_eq!(value["transition_count"], 2);
        assert_eq!(value["cooling_body_entry_count"], 2);
        assert_eq!(
            value["supply_mass_flow_rate_for_dehumidification_reset_assignment_count"],
            2
        );
        assert_eq!(value["cooling_on_read_count"], 2);
        assert_eq!(value["cooling_on_body_entry_count"], 2);
        assert_eq!(value["cooling_on_fallthrough_count"], 0);
        assert_eq!(value["dehumidification_control_type_read_count"], 2);
        assert_eq!(value["dehumidification_control_type_humidistat_count"], 0);
        assert_eq!(value["dehumidification_control_type_fallthrough_count"], 2);
        assert_eq!(value["dehumidification_control_body_entry_count"], 0);
        assert_eq!(
            value["supply_mass_flow_rate_for_dehumidification_assignment_count"],
            0
        );
        assert_eq!(value.as_object().map(serde_json::Map::len), Some(35));

        let latest = &value["latest"];
        assert_eq!(
            latest["source_order"],
            serde_json::json!(PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SOURCE_ORDER)
        );
        assert_eq!(latest["cooling_body_entered"], true);
        assert_eq!(
            latest["supply_mass_flow_rate_for_dehumidification_reset_assigned"],
            true
        );
        assert_eq!(latest["cooling_on"], true);
        assert_eq!(latest["dehumidification_control_type"], "None");
        assert_eq!(latest["dehumidification_control_type_humidistat"], false);
        assert_eq!(latest["dehumidification_control_body_entered"], false);
        assert_eq!(
            latest["reset_supply_mass_flow_rate_for_dehumidification_kg_per_s"]
                .as_f64()
                .map(f64::to_bits),
            Some(0.0_f64.to_bits())
        );
        assert_eq!(
            latest["resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s"]
                .as_f64()
                .map(f64::to_bits),
            Some(0.0_f64.to_bits())
        );
        assert_eq!(latest.as_object().map(serde_json::Map::len), Some(53));
    }

    #[test]
    fn direct_release_cooling_humidification_flow_validation_rejects_malformed_evidence() {
        let init = valid_init_lifecycle(2);
        let predecessor = valid_cooling_dehumidification_flow_lifecycle(2);
        let valid = valid_cooling_humidification_flow_lifecycle(2);
        assert!(
            purchased_air_cooling_humidification_flow::validate_direct_lifecycle(
                Some(&valid),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_ok()
        );
        assert!(
            purchased_air_cooling_humidification_flow::validate_direct_lifecycle(
                None,
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut wrong_provenance = valid.clone();
        wrong_provenance.first_excluded_source =
            PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE;
        assert!(
            purchased_air_cooling_humidification_flow::validate_direct_lifecycle(
                Some(&wrong_provenance),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut wrong_count = valid.clone();
        wrong_count
            .state
            .humidification_control_type_fallthrough_count = 1;
        assert!(
            purchased_air_cooling_humidification_flow::validate_direct_lifecycle(
                Some(&wrong_count),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut wrong_zero = valid.clone();
        wrong_zero
            .state
            .latest
            .as_mut()
            .expect("valid latest cooling humidification-flow snapshot")
            .reset_supply_mass_flow_rate_for_humidification_kg_per_s = Some(-0.0);
        assert!(
            purchased_air_cooling_humidification_flow::validate_direct_lifecycle(
                Some(&wrong_zero),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut wrong_selector = valid.clone();
        wrong_selector
            .state
            .latest
            .as_mut()
            .expect("valid latest cooling humidification-flow snapshot")
            .humidification_control_type = Some(HumidificationControlType::Humidistat);
        assert!(
            purchased_air_cooling_humidification_flow::validate_direct_lifecycle(
                Some(&wrong_selector),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut wrong_predecessor_link = valid.clone();
        wrong_predecessor_link
            .state
            .latest
            .as_mut()
            .expect("valid latest cooling humidification-flow snapshot")
            .predecessor_cooling_body_entered = false;
        assert!(
            purchased_air_cooling_humidification_flow::validate_direct_lifecycle(
                Some(&wrong_predecessor_link),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );

        let mut overflowed_partition = valid;
        overflowed_partition.state.unit_off_skip_count = usize::MAX;
        overflowed_partition.state.non_cooling_skip_count = 1;
        assert!(
            purchased_air_cooling_humidification_flow::validate_direct_lifecycle(
                Some(&overflowed_partition),
                Some(&predecessor),
                Some(&init),
                Some(2)
            )
            .is_err()
        );
    }

    #[test]
    fn direct_release_cooling_humidification_flow_json_exposes_all_source_sites() {
        let lifecycle = valid_cooling_humidification_flow_lifecycle(2);
        let value = purchased_air_cooling_humidification_flow::lifecycle_json(&lifecycle);
        assert_eq!(
            value["source"],
            PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE
        );
        assert_eq!(
            value["first_excluded_source"],
            PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE
        );
        assert_eq!(value["transition_count"], 2);
        assert_eq!(value["reset_assignment_count"], 2);
        assert_eq!(value["heating_on_read_count"], 2);
        assert_eq!(value["heating_on_body_entry_count"], 2);
        assert_eq!(value["humidification_control_type_read_count"], 2);
        assert_eq!(value["humidification_control_type_fallthrough_count"], 2);
        assert_eq!(value["dehumidification_control_type_first_read_count"], 0);
        assert_eq!(value["moisture_demand_read_count"], 0);
        assert_eq!(value["assignment_count"], 0);
        assert_eq!(value.as_object().map(serde_json::Map::len), Some(41));

        let latest = &value["latest"];
        assert_eq!(
            latest["source_order"],
            serde_json::json!(PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE_ORDER)
        );
        assert_eq!(latest["source_order"].as_array().map(Vec::len), Some(26));
        assert_eq!(latest["cooling_body_entered"], true);
        assert_eq!(latest["heating_on"], true);
        assert_eq!(latest["humidification_control_type"], "None");
        assert_eq!(latest["humidification_control_type_humidistat"], false);
        assert_eq!(latest["dehumidification_control_type_first_read"], false);
        assert_eq!(
            latest["zone_humidifying_setpoint_moisture_demand_read"],
            false
        );
        assert_eq!(
            latest["reset_supply_mass_flow_rate_for_humidification_kg_per_s"]
                .as_f64()
                .map(f64::to_bits),
            Some(0.0_f64.to_bits())
        );
        assert_eq!(
            latest["resulting_supply_mass_flow_rate_for_humidification_kg_per_s"]
                .as_f64()
                .map(f64::to_bits),
            Some(0.0_f64.to_bits())
        );
        assert_eq!(latest.as_object().map(serde_json::Map::len), Some(57));
    }

    #[test]
    fn direct_release_cooling_capacity_zero_reset_rejects_malformed_evidence() {
        let init = valid_init_lifecycle(2);
        let cp318 = valid_cooling_sensible_flow_lifecycle(2);
        let cp319 = valid_cooling_dehumidification_flow_lifecycle(2);
        let cp320 = valid_cooling_humidification_flow_lifecycle(2);
        let valid =
            valid_cooling_capacity_zero_flow_reset_lifecycle(2, IdealLoadsLimit::NoLimit, None);
        let validate =
            |lifecycle: Option<&PurchasedAirCalcCoolingCapacityZeroFlowResetLifecycleSummary>| {
                purchased_air_cooling_capacity_zero_flow_reset::validate_direct_lifecycle(
                    lifecycle,
                    Some(&cp320),
                    Some(&cp319),
                    Some(&cp318),
                    Some(&init),
                    Some(2),
                )
            };
        assert!(validate(Some(&valid)).is_ok());
        assert!(validate(None).is_err());

        let mut wrong_provenance = valid.clone();
        wrong_provenance.first_excluded_source =
            PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE;
        assert!(validate(Some(&wrong_provenance)).is_err());

        let mut wrong_selector_count = valid.clone();
        wrong_selector_count.state.cooling_limit_rejected_count = 1;
        assert!(validate(Some(&wrong_selector_count)).is_err());

        let mut mixed_capacity_and_combined = valid_cooling_capacity_zero_flow_reset_lifecycle(
            2,
            IdealLoadsLimit::LimitCapacity,
            Some(300.0),
        );
        mixed_capacity_and_combined
            .state
            .cooling_limit_capacity_count = 1;
        mixed_capacity_and_combined
            .state
            .second_cooling_limit_read_count = 1;
        mixed_capacity_and_combined
            .state
            .cooling_limit_flow_rate_and_capacity_count = 1;
        assert!(validate(Some(&mixed_capacity_and_combined)).is_err());

        let mut latest_selector_disagrees_with_cumulative =
            valid_cooling_capacity_zero_flow_reset_lifecycle(
                2,
                IdealLoadsLimit::LimitCapacity,
                Some(300.0),
            );
        latest_selector_disagrees_with_cumulative
            .state
            .cooling_limit_capacity_count = 0;
        latest_selector_disagrees_with_cumulative
            .state
            .second_cooling_limit_read_count = 2;
        latest_selector_disagrees_with_cumulative
            .state
            .cooling_limit_flow_rate_and_capacity_count = 2;
        assert!(validate(Some(&latest_selector_disagrees_with_cumulative)).is_err());

        let mut wrong_candidate_lineage = valid.clone();
        wrong_candidate_lineage
            .state
            .latest
            .as_mut()
            .expect("valid CP321 latest snapshot")
            .predecessor_supply_mass_flow_rate_for_cool_kg_per_s = Some(-0.0);
        assert!(validate(Some(&wrong_candidate_lineage)).is_err());

        let mut partial_candidate_lineage = valid.clone();
        partial_candidate_lineage
            .state
            .latest
            .as_mut()
            .expect("valid CP321 latest snapshot")
            .predecessor_supply_mass_flow_rate_for_dehumidification_kg_per_s = None;
        assert!(validate(Some(&partial_candidate_lineage)).is_err());

        let mut wrong_result = valid.clone();
        wrong_result
            .state
            .latest
            .as_mut()
            .expect("valid CP321 latest snapshot")
            .resulting_supply_mass_flow_rate_for_humidification_kg_per_s = Some(-0.0);
        assert!(validate(Some(&wrong_result)).is_err());

        let mut wrong_cp318 = cp318.clone();
        wrong_cp318
            .state
            .latest
            .as_mut()
            .expect("valid CP318 latest snapshot")
            .resulting_supply_mass_flow_rate_for_cool_kg_per_s = Some(-0.0);
        assert!(
            purchased_air_cooling_capacity_zero_flow_reset::validate_direct_lifecycle(
                Some(&valid),
                Some(&cp320),
                Some(&cp319),
                Some(&wrong_cp318),
                Some(&init),
                Some(2),
            )
            .is_err()
        );

        let mut wrong_cp319 = cp319.clone();
        wrong_cp319
            .state
            .latest
            .as_mut()
            .expect("valid CP319 latest snapshot")
            .resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s = Some(-0.0);
        assert!(
            purchased_air_cooling_capacity_zero_flow_reset::validate_direct_lifecycle(
                Some(&valid),
                Some(&cp320),
                Some(&wrong_cp319),
                Some(&cp318),
                Some(&init),
                Some(2),
            )
            .is_err()
        );

        let mut wrong_cp320 = cp320.clone();
        wrong_cp320
            .state
            .latest
            .as_mut()
            .expect("valid CP320 latest snapshot")
            .resulting_supply_mass_flow_rate_for_humidification_kg_per_s = Some(-0.0);
        assert!(
            purchased_air_cooling_capacity_zero_flow_reset::validate_direct_lifecycle(
                Some(&valid),
                Some(&wrong_cp320),
                Some(&cp319),
                Some(&cp318),
                Some(&init),
                Some(2),
            )
            .is_err()
        );

        let mut overflowed_partition = valid;
        overflowed_partition.state.unit_off_skip_count = usize::MAX;
        overflowed_partition.state.non_cooling_skip_count = 1;
        assert!(validate(Some(&overflowed_partition)).is_err());
    }

    #[test]
    fn direct_release_cooling_capacity_zero_reset_json_exposes_all_source_sites() {
        let lifecycle = valid_cooling_capacity_zero_flow_reset_lifecycle(
            2,
            IdealLoadsLimit::LimitCapacity,
            Some(-0.0),
        );
        let value = purchased_air_cooling_capacity_zero_flow_reset::lifecycle_json(&lifecycle);
        assert_eq!(
            value["source"],
            PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE
        );
        assert_eq!(
            value["first_excluded_source"],
            PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_FIRST_EXCLUDED_SOURCE
        );
        assert_eq!(value["transition_count"], 2);
        assert_eq!(value["first_cooling_limit_read_count"], 2);
        assert_eq!(value["cooling_limit_capacity_count"], 2);
        assert_eq!(value["second_cooling_limit_read_count"], 0);
        assert_eq!(value["maximum_total_cooling_capacity_read_count"], 2);
        assert_eq!(value["maximum_total_cooling_capacity_zero_count"], 2);
        assert_eq!(value["zero_cooling_capacity_body_entry_count"], 2);
        assert_eq!(
            value["supply_mass_flow_rate_for_cool_zero_assignment_count"],
            2
        );
        assert_eq!(value.as_object().map(serde_json::Map::len), Some(21));

        let latest = &value["latest"];
        assert_eq!(
            latest["source_order"],
            serde_json::json!(PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE_ORDER)
        );
        assert_eq!(latest["source_order"].as_array().map(Vec::len), Some(10));
        assert_eq!(latest["first_cooling_limit"], "LimitCapacity");
        assert_eq!(latest["cooling_limit_capacity"], true);
        assert_eq!(latest["second_cooling_limit_read"], false);
        assert_eq!(
            latest["maximum_total_cooling_capacity_w"]
                .as_f64()
                .map(f64::to_bits),
            Some((-0.0_f64).to_bits())
        );
        assert_eq!(latest["maximum_total_cooling_capacity_equal_to_zero"], true);
        assert_eq!(latest["zero_cooling_capacity_body_entered"], true);
        for field in [
            "assigned_supply_mass_flow_rate_for_cool_kg_per_s",
            "assigned_supply_mass_flow_rate_for_dehumidification_kg_per_s",
            "assigned_supply_mass_flow_rate_for_humidification_kg_per_s",
            "resulting_supply_mass_flow_rate_for_cool_kg_per_s",
            "resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s",
            "resulting_supply_mass_flow_rate_for_humidification_kg_per_s",
        ] {
            assert_eq!(
                latest[field].as_f64().map(f64::to_bits),
                Some(0.0_f64.to_bits()),
                "{field}"
            );
        }
        assert_eq!(latest.as_object().map(serde_json::Map::len), Some(35));
    }

    #[test]
    fn lifecycle_json_serializes_structured_supply_temperature_diagnostics() {
        let mut lifecycle = valid_init_lifecycle(1);
        lifecycle.supply_temperature_registered_recurring_diagnostic_count = 1;
        lifecycle.supply_temperature_diagnostic_event_count = 1;
        lifecycle.supply_temperature_characterized_severe_error_count_increment = 2;
        lifecycle.cooling_supply_temperature_error_index = 1;
        lifecycle.cooling_supply_temperature_first_diagnostic_count = 1;
        lifecycle.cooling_supply_temperature_warning_count = 1;
        lifecycle
            .supply_temperature_diagnostics
            .push(PurchasedAirSupplyTemperatureDiagnostic {
                system: IdealLoadsAirSystemId(0),
                registry_registration_ordinal: 1,
                first_init_call_ordinal: 1,
                last_init_call_ordinal: 1,
                source_order_ordinal: 1,
                kind: PurchasedAirSupplyTemperatureDiagnosticKind::CoolingMinimumAboveSetpoint,
                recurring_index: 1,
                first_detailed_diagnostic_count: 1,
                initial_message_api:
                    PurchasedAirSupplyTemperatureInitialMessageApi::ShowSevereError,
                first_detail_primary_message_count: 1,
                first_detail_continue_message_count: 5,
                first_detail_timestamp_count: 1,
                recurring_severe_call_count: 1,
                characterized_severe_error_count_increment: 2,
                latest_supply_temperature_c: 25.0,
                latest_thermostat_setpoint_c: 24.0,
                recurring_minimum_c: 25.0,
                recurring_maximum_c: 25.0,
                temperature_unit: "C",
            });

        let value = purchased_air_init_lifecycle_json(&lifecycle);
        let registry = &value["supply_temperature_diagnostic_registry"];
        assert_eq!(registry["registered_recurring_diagnostic_count"], 1);
        assert_eq!(registry["event_count"], 1);
        assert_eq!(registry["characterized_severe_error_count_increment"], 2);
        assert_eq!(registry["cooling_error_index"], 1);
        assert!(registry["heating_error_index"].is_null());
        assert_eq!(registry["identities"][0]["source_order_ordinal"], 1);
        assert_eq!(
            registry["identities"][0]["kind"],
            "cooling_minimum_above_setpoint"
        );
        assert_eq!(
            registry["identities"][0]["initial_message_api"],
            "show_severe_error"
        );
        assert_eq!(
            registry["identities"][0]["first_detail_continue_message_count"],
            5
        );
        assert_eq!(registry["identities"][0]["temperature_unit"], "C");
    }

    fn valid_cooling_entry_gate_lifecycle(
        call_count: usize,
    ) -> PurchasedAirCalcCoolingEntryGateLifecycleSummary {
        let system = IdealLoadsAirSystemId(0);
        let mut state = PurchasedAirCalcCoolingEntryGateRuntimeState::new(system);
        state.transition_count = call_count;
        state.source_execution_count = call_count;
        state.sensible_comparison_count = call_count;
        state.sensible_comparison_satisfied_count = call_count;
        state.temperature_control_type_read_count = call_count;
        state.cooling_body_entry_count = call_count;
        state.operating_mode_assignment_count = call_count;
        state.latest = Some(PurchasedAirCalcCoolingEntryGateSnapshot {
            source: PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE,
            first_excluded_source: PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_FIRST_EXCLUDED_SOURCE,
            system,
            parent_call_ordinal: call_count,
            source_order: PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE_ORDER,
            controlled_zone: ZoneId(0),
            unit_body_entered: true,
            minimum_outdoor_air_sensible_output_w: Some(0.0),
            cooling_setpoint_demand_w: Some(-50.0),
            sensible_comparison_evaluated: true,
            sensible_comparison_satisfied: Some(true),
            temperature_control_type_read: true,
            temperature_control_type: Some(PurchasedAirTemperatureControlType::DualHeatCool),
            temperature_control_type_permits_cooling: Some(true),
            single_heat_blocked: false,
            cooling_body_entered: true,
            assigned_operating_mode: Some(IdealLoadsSensibleMode::Cooling),
        });
        PurchasedAirCalcCoolingEntryGateLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE,
            first_excluded_source: PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_FIRST_EXCLUDED_SOURCE,
            state,
        }
    }

    fn valid_cooling_oa_max_flow_gate_lifecycle(
        call_count: usize,
        limit: IdealLoadsLimit,
    ) -> PurchasedAirCalcCoolingOaMaxFlowGateLifecycleSummary {
        let system = IdealLoadsAirSystemId(0);
        let first_match = limit == IdealLoadsLimit::LimitFlowRate;
        let second_evaluated = !first_match;
        let second_match = limit == IdealLoadsLimit::LimitFlowRateAndCapacity;
        let flow_selected = first_match || second_match;
        let mut state = PurchasedAirCalcCoolingOaMaxFlowGateRuntimeState::new(system);
        state.transition_count = call_count;
        state.source_execution_count = call_count;
        state.cooling_limit_flow_rate_comparison_count = call_count;
        state.cooling_limit_flow_rate_match_count = usize::from(first_match) * call_count;
        state.cooling_limit_flow_rate_and_capacity_comparison_count =
            usize::from(second_evaluated) * call_count;
        state.cooling_limit_flow_rate_and_capacity_match_count =
            usize::from(second_match) * call_count;
        state.outdoor_air_mass_flow_rate_read_count = usize::from(flow_selected) * call_count;
        state.maximum_cooling_air_mass_flow_rate_read_count =
            usize::from(flow_selected) * call_count;
        state.strict_mass_flow_comparison_count = usize::from(flow_selected) * call_count;
        state.active_fallthrough_count = call_count;
        state.latest = Some(PurchasedAirCalcCoolingOaMaxFlowGateSnapshot {
            source: PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_FIRST_EXCLUDED_SOURCE,
            system,
            parent_call_ordinal: call_count,
            source_order: PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE_ORDER,
            controlled_zone: ZoneId(0),
            unit_body_entered: true,
            predecessor_cooling_body_entered: true,
            unit_off_skipped: false,
            non_cooling_skipped: false,
            cooling_limit_flow_rate_comparison_evaluated: true,
            cooling_limit_flow_rate_read: true,
            cooling_limit_flow_rate_value: Some(limit),
            cooling_limit_flow_rate_comparison_satisfied: Some(first_match),
            cooling_limit_flow_rate_and_capacity_comparison_evaluated: second_evaluated,
            cooling_limit_flow_rate_and_capacity_read: second_evaluated,
            cooling_limit_flow_rate_and_capacity_value: second_evaluated.then_some(limit),
            cooling_limit_flow_rate_and_capacity_comparison_satisfied: second_evaluated
                .then_some(second_match),
            cooling_flow_limit_active: Some(flow_selected),
            outdoor_air_mass_flow_rate_read: flow_selected,
            outdoor_air_mass_flow_rate_kg_per_s: flow_selected.then_some(0.0),
            maximum_cooling_air_mass_flow_rate_read: flow_selected,
            maximum_cooling_air_mass_flow_rate_kg_per_s: flow_selected.then_some(0.0),
            strict_mass_flow_comparison_evaluated: flow_selected,
            outdoor_air_mass_flow_above_maximum: flow_selected.then_some(false),
            maximum_cooling_flow_body_entered: false,
        });
        PurchasedAirCalcCoolingOaMaxFlowGateLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_FIRST_EXCLUDED_SOURCE,
            state,
        }
    }

    fn valid_cooling_oa_max_flow_body_lifecycle(
        call_count: usize,
    ) -> PurchasedAirCalcCoolingOaMaxFlowBodyLifecycleSummary {
        let system = IdealLoadsAirSystemId(0);
        let mut state = PurchasedAirCalcCoolingOaMaxFlowBodyRuntimeState::new(system);
        state.transition_count = call_count;
        state.body_skip_count = call_count;
        state.active_guard_false_economizer_fallthrough_count = call_count;
        state.latest = Some(PurchasedAirCalcCoolingOaMaxFlowBodySnapshot {
            source: PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_FIRST_EXCLUDED_SOURCE,
            recurring_warning_child_source:
                PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_RECURRING_WARNING_CHILD_SOURCE,
            system,
            parent_call_ordinal: call_count,
            source_order: PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE_ORDER,
            controlled_zone: ZoneId(0),
            unit_body_entered: true,
            predecessor_cooling_body_entered: true,
            predecessor_maximum_cooling_flow_body_entered: false,
            body_skipped: true,
            unit_off_skipped: false,
            non_cooling_skipped: false,
            active_guard_false_economizer_fallthrough: true,
            outdoor_air_mass_flow_rate_read: false,
            outdoor_air_mass_flow_rate_before_clamp_kg_per_s: None,
            standard_air_density_read: false,
            standard_air_density_kg_per_m3: None,
            outdoor_air_volume_flow_rate_calculated: false,
            outdoor_air_volume_flow_rate_m3_per_s: None,
            warning_counter_read: false,
            warning_counter_before: None,
            first_warning_predicate_satisfied: None,
            first_warning_branch_entered: false,
            warning_counter_incremented: false,
            warning_counter_after: None,
            first_warning_call_site_reached: false,
            maximum_cooling_air_volume_flow_rate_read: false,
            maximum_cooling_air_volume_flow_rate_m3_per_s: None,
            continue_warning_call_site_reached: false,
            continue_warning_timestamp_call_site_reached: false,
            recurring_warning_branch_entered: false,
            recurring_warning_call_site_reached: false,
            recurring_warning_report_maximum_input_m3_per_s: None,
            characterized_recurring_warning_index_allocated_on_call: false,
            characterized_recurring_warning_index_reused_on_call: false,
            characterized_recurring_warning_index_before: None,
            characterized_recurring_warning_index_after: None,
            characterized_recurring_warning_occurrence_ordinal: None,
            characterized_recurring_warning_report_maximum_m3_per_s: None,
            characterized_total_warning_error_incremented: false,
            maximum_cooling_air_mass_flow_rate_read: false,
            maximum_cooling_air_mass_flow_rate_kg_per_s: None,
            outdoor_air_mass_flow_clamp_assignment_performed: false,
            outdoor_air_mass_flow_rate_after_clamp_kg_per_s: None,
        });
        PurchasedAirCalcCoolingOaMaxFlowBodyLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_FIRST_EXCLUDED_SOURCE,
            recurring_warning_child_source:
                PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_RECURRING_WARNING_CHILD_SOURCE,
            state,
        }
    }

    fn valid_cooling_economizer_guard_lifecycle(
        call_count: usize,
    ) -> PurchasedAirCalcCoolingEconomizerGuardLifecycleSummary {
        let system = IdealLoadsAirSystemId(0);
        let mut state = PurchasedAirCalcCoolingEconomizerGuardRuntimeState::new(system);
        state.transition_count = call_count;
        state.guard_evaluation_count = call_count;
        state.economizer_type_read_count = call_count;
        state.no_economizer_comparison_count = call_count;
        state.no_economizer_fallthrough_count = call_count;
        state.latest = Some(PurchasedAirCalcCoolingEconomizerGuardSnapshot {
            source: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_FIRST_EXCLUDED_SOURCE,
            system,
            parent_call_ordinal: call_count,
            source_order: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE_ORDER,
            controlled_zone: ZoneId(0),
            unit_body_entered: true,
            predecessor_cooling_body_entered: true,
            predecessor_maximum_cooling_flow_body_entered: false,
            predecessor_active_guard_false_economizer_fallthrough: true,
            unit_off_skipped: false,
            non_cooling_skipped: false,
            maximum_cooling_flow_body_sibling_skipped: false,
            economizer_guard_evaluated: true,
            economizer_type_read: true,
            economizer_type: Some(OutdoorAirEconomizerType::NoEconomizer),
            no_economizer_comparison_evaluated: true,
            economizer_not_no_economizer: Some(false),
            economizer_body_entered: false,
            no_economizer_fallthrough: true,
        });
        PurchasedAirCalcCoolingEconomizerGuardLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_FIRST_EXCLUDED_SOURCE,
            state,
        }
    }

    fn valid_cooling_economizer_condition_lifecycle(
        call_count: usize,
    ) -> PurchasedAirCalcCoolingEconomizerConditionLifecycleSummary {
        let system = IdealLoadsAirSystemId(0);
        let mut state = PurchasedAirCalcCoolingEconomizerConditionRuntimeState::new(system);
        state.transition_count = call_count;
        state.no_economizer_outer_guard_fallthrough_skip_count = call_count;
        state.latest = Some(PurchasedAirCalcCoolingEconomizerConditionSnapshot {
            source: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_FIRST_EXCLUDED_SOURCE,
            system,
            parent_call_ordinal: call_count,
            source_order: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE_ORDER,
            controlled_zone: ZoneId(0),
            unit_body_entered: true,
            predecessor_cooling_body_entered: true,
            predecessor_maximum_cooling_flow_body_entered: false,
            predecessor_active_guard_false_economizer_fallthrough: true,
            predecessor_economizer_guard_evaluated: true,
            predecessor_economizer_body_entered: false,
            predecessor_no_economizer_fallthrough: true,
            unit_off_skipped: false,
            non_cooling_skipped: false,
            maximum_cooling_flow_body_sibling_skipped: false,
            no_economizer_outer_guard_fallthrough_skipped: true,
            economizer_condition_evaluated: false,
            differential_dry_bulb_economizer_type_read: false,
            differential_dry_bulb_economizer_type: None,
            differential_dry_bulb_selector_comparison_evaluated: false,
            differential_dry_bulb_selector_matched: None,
            outdoor_air_temperature_read: false,
            outdoor_air_temperature_c: None,
            recirculation_air_temperature_read: false,
            recirculation_air_temperature_c: None,
            dry_bulb_temperature_comparison_evaluated: false,
            outdoor_air_temperature_below_recirculation_temperature: None,
            differential_enthalpy_economizer_type_read: false,
            differential_enthalpy_economizer_type: None,
            differential_enthalpy_selector_comparison_evaluated: false,
            differential_enthalpy_selector_matched: None,
            outdoor_air_enthalpy_read: false,
            outdoor_air_enthalpy_j_per_kg: None,
            recirculation_air_enthalpy_read: false,
            recirculation_air_enthalpy_j_per_kg: None,
            enthalpy_comparison_evaluated: false,
            outdoor_air_enthalpy_below_recirculation_enthalpy: None,
            economizer_condition_satisfied: None,
            economizer_calculation_body_entered: false,
            economizer_condition_fallthrough: false,
        });
        PurchasedAirCalcCoolingEconomizerConditionLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_FIRST_EXCLUDED_SOURCE,
            state,
        }
    }

    fn valid_cooling_economizer_body_lifecycle(
        call_count: usize,
    ) -> PurchasedAirCalcCoolingEconomizerBodyLifecycleSummary {
        let system = IdealLoadsAirSystemId(0);
        let mut state = PurchasedAirCalcCoolingEconomizerBodyRuntimeState::new(system);
        state.transition_count = call_count;
        state.no_economizer_outer_guard_fallthrough_skip_count = call_count;
        state.latest = Some(PurchasedAirCalcCoolingEconomizerBodySnapshot {
            source: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE,
            first_excluded_source: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_FIRST_EXCLUDED_SOURCE,
            system,
            parent_call_ordinal: call_count,
            source_order: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE_ORDER,
            controlled_zone: ZoneId(0),
            unit_body_entered: true,
            predecessor_cooling_body_entered: true,
            predecessor_maximum_cooling_flow_body_entered: false,
            predecessor_active_guard_false_economizer_fallthrough: true,
            predecessor_economizer_guard_evaluated: true,
            predecessor_economizer_body_entered: false,
            predecessor_no_economizer_fallthrough: true,
            predecessor_economizer_condition_evaluated: false,
            predecessor_economizer_condition_satisfied: None,
            predecessor_economizer_calculation_body_entered: false,
            unit_off_skipped: false,
            non_cooling_skipped: false,
            maximum_cooling_flow_body_sibling_skipped: false,
            no_economizer_outer_guard_fallthrough_skipped: true,
            economizer_condition_fallthrough_skipped: false,
            economizer_calculation_body_executed: false,
            zone_humidity_ratio_read: false,
            zone_humidity_ratio: None,
            psychrometric_cp_air_evaluated: false,
            psychrometric_cp_air_result_j_per_kg_k: None,
            cp_air_assigned: false,
            cp_air_j_per_kg_k: None,
            outdoor_air_temperature_read: false,
            outdoor_air_temperature_c: None,
            zone_temperature_read: false,
            zone_temperature_c: None,
            delta_temperature_calculated: false,
            delta_temperature_c: None,
            delta_temperature_assigned: false,
            assigned_delta_temperature_c: None,
            delta_temperature_for_gate_read: false,
            delta_temperature_for_gate_c: None,
            delta_temperature_comparison_evaluated: false,
            delta_temperature_below_negative_small_temp_diff: None,
            delta_temperature_body_entered: false,
            zone_cooling_setpoint_load_read: false,
            zone_cooling_setpoint_load_w: None,
            cp_air_for_first_division_read: false,
            cp_air_for_first_division_j_per_kg_k: None,
            zone_cooling_setpoint_load_over_cp_air_calculated: false,
            zone_cooling_setpoint_load_over_cp_air_kg_k_per_s: None,
            delta_temperature_for_second_division_read: false,
            delta_temperature_for_second_division_c: None,
            supply_mass_flow_rate_calculated: false,
            calculated_supply_mass_flow_rate_kg_per_s: None,
            initial_supply_mass_flow_rate_assigned: false,
            initial_supply_mass_flow_rate_kg_per_s: None,
            cooling_limit_flow_rate_comparison_evaluated: false,
            cooling_limit_flow_rate_read: false,
            cooling_limit_flow_rate_value: None,
            cooling_limit_flow_rate_comparison_satisfied: None,
            cooling_limit_flow_rate_and_capacity_comparison_evaluated: false,
            cooling_limit_flow_rate_and_capacity_read: false,
            cooling_limit_flow_rate_and_capacity_value: None,
            cooling_limit_flow_rate_and_capacity_comparison_satisfied: None,
            cooling_flow_limit_active: None,
            maximum_cooling_air_mass_flow_rate_read: false,
            maximum_cooling_air_mass_flow_rate_kg_per_s: None,
            maximum_cooling_air_mass_flow_rate_positive_comparison_evaluated: false,
            maximum_cooling_air_mass_flow_rate_positive: None,
            maximum_flow_clamp_body_entered: false,
            supply_mass_flow_rate_clamped: false,
            supply_mass_flow_rate_for_clamp_read: false,
            supply_mass_flow_rate_for_clamp_kg_per_s: None,
            inner_max_evaluated: false,
            nonnegative_supply_mass_flow_rate_kg_per_s: None,
            maximum_cooling_air_mass_flow_rate_clamp_upper_bound_read: false,
            maximum_cooling_air_mass_flow_rate_clamp_upper_bound_kg_per_s: None,
            outer_min_evaluated: false,
            clamped_supply_mass_flow_rate_kg_per_s: None,
            clamped_supply_mass_flow_rate_assigned: false,
            resulting_supply_mass_flow_rate_kg_per_s: None,
            resulting_supply_mass_flow_rate_read: false,
            outdoor_air_mass_flow_rate_read: false,
            outdoor_air_mass_flow_rate_kg_per_s: None,
            supply_above_outdoor_air_mass_flow_comparison_evaluated: false,
            supply_mass_flow_above_outdoor_air_mass_flow: None,
            economizer_activation_body_entered: false,
            economizer_on_assigned: false,
            economizer_on: None,
            supply_mass_flow_rate_for_outdoor_air_assignment_read: false,
            supply_mass_flow_rate_for_outdoor_air_assignment_kg_per_s: None,
            outdoor_air_mass_flow_rate_assigned: false,
            assigned_outdoor_air_mass_flow_rate_kg_per_s: None,
            system_time_step_read: false,
            system_time_step_hours: None,
            economizer_active_time_assigned: false,
            assigned_economizer_active_time_hours: None,
        });
        PurchasedAirCalcCoolingEconomizerBodyLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE,
            first_excluded_source: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_FIRST_EXCLUDED_SOURCE,
            state,
        }
    }

    fn valid_cooling_sensible_flow_lifecycle(
        call_count: usize,
    ) -> PurchasedAirCalcCoolingSensibleFlowLifecycleSummary {
        let system = IdealLoadsAirSystemId(0);
        let humidity_ratio = 0.008;
        let cp_air = ep_runtime::psychrometrics::energyplus_psy_cp_air_fn_w(humidity_ratio);
        let minimum_supply_temperature = 13.0;
        let zone_temperature = 25.0;
        let delta_temperature = minimum_supply_temperature - zone_temperature;
        let cooling_load = -2_400.0;
        let first_division = cooling_load / cp_air;
        let calculated_flow = first_division / delta_temperature;
        let mut state = PurchasedAirCalcCoolingSensibleFlowRuntimeState::new(system);
        state.transition_count = call_count;
        state.cooling_body_entry_count = call_count;
        state.supply_mass_flow_rate_for_cool_reset_assignment_count = call_count;
        state.cooling_on_read_count = call_count;
        state.cooling_on_body_entry_count = call_count;
        state.zone_humidity_ratio_read_count = call_count;
        state.psychrometric_cp_air_evaluation_count = call_count;
        state.cp_air_assignment_count = call_count;
        state.minimum_cooling_supply_air_temperature_read_count = call_count;
        state.zone_temperature_read_count = call_count;
        state.delta_temperature_calculation_count = call_count;
        state.delta_temperature_assignment_count = call_count;
        state.delta_temperature_for_gate_read_count = call_count;
        state.delta_temperature_comparison_count = call_count;
        state.delta_temperature_comparison_satisfied_count = call_count;
        state.delta_temperature_body_entry_count = call_count;
        state.zone_cooling_setpoint_load_read_count = call_count;
        state.cp_air_for_first_division_read_count = call_count;
        state.zone_cooling_setpoint_load_over_cp_air_calculation_count = call_count;
        state.delta_temperature_for_second_division_read_count = call_count;
        state.supply_mass_flow_rate_for_cool_calculation_count = call_count;
        state.supply_mass_flow_rate_for_cool_assignment_count = call_count;
        state.latest = Some(PurchasedAirCalcCoolingSensibleFlowSnapshot {
            source: PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE,
            first_excluded_source: PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_FIRST_EXCLUDED_SOURCE,
            system,
            parent_call_ordinal: call_count,
            source_order: PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE_ORDER,
            controlled_zone: ZoneId(0),
            unit_body_entered: true,
            predecessor_cooling_body_entered: true,
            predecessor_maximum_cooling_flow_body_sibling_skipped: false,
            predecessor_no_economizer_outer_guard_fallthrough_skipped: true,
            predecessor_economizer_condition_fallthrough_skipped: false,
            predecessor_economizer_calculation_body_executed: false,
            unit_off_skipped: false,
            non_cooling_skipped: false,
            cooling_body_entered: true,
            supply_mass_flow_rate_for_cool_reset_assigned: true,
            reset_supply_mass_flow_rate_for_cool_kg_per_s: Some(0.0),
            cooling_on_read: true,
            cooling_on: Some(true),
            cooling_on_body_entered: true,
            zone_humidity_ratio_read: true,
            zone_humidity_ratio: Some(humidity_ratio),
            psychrometric_cp_air_evaluated: true,
            psychrometric_cp_air_result_j_per_kg_k: Some(cp_air),
            cp_air_assigned: true,
            cp_air_j_per_kg_k: Some(cp_air),
            minimum_cooling_supply_air_temperature_read: true,
            minimum_cooling_supply_air_temperature_c: Some(minimum_supply_temperature),
            zone_temperature_read: true,
            zone_temperature_c: Some(zone_temperature),
            delta_temperature_calculated: true,
            delta_temperature_c: Some(delta_temperature),
            delta_temperature_assigned: true,
            assigned_delta_temperature_c: Some(delta_temperature),
            delta_temperature_for_gate_read: true,
            delta_temperature_for_gate_c: Some(delta_temperature),
            delta_temperature_comparison_evaluated: true,
            delta_temperature_below_negative_small_temp_diff: Some(true),
            delta_temperature_body_entered: true,
            zone_cooling_setpoint_load_read: true,
            zone_cooling_setpoint_load_w: Some(cooling_load),
            cp_air_for_first_division_read: true,
            cp_air_for_first_division_j_per_kg_k: Some(cp_air),
            zone_cooling_setpoint_load_over_cp_air_calculated: true,
            zone_cooling_setpoint_load_over_cp_air_kg_k_per_s: Some(first_division),
            delta_temperature_for_second_division_read: true,
            delta_temperature_for_second_division_c: Some(delta_temperature),
            supply_mass_flow_rate_for_cool_calculated: true,
            calculated_supply_mass_flow_rate_for_cool_kg_per_s: Some(calculated_flow),
            supply_mass_flow_rate_for_cool_assigned: true,
            assigned_supply_mass_flow_rate_for_cool_kg_per_s: Some(calculated_flow),
            resulting_supply_mass_flow_rate_for_cool_kg_per_s: Some(calculated_flow),
        });
        PurchasedAirCalcCoolingSensibleFlowLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE,
            first_excluded_source: PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_FIRST_EXCLUDED_SOURCE,
            state,
        }
    }

    fn valid_cooling_dehumidification_flow_lifecycle(
        call_count: usize,
    ) -> PurchasedAirCalcCoolingDehumidificationFlowLifecycleSummary {
        let system = IdealLoadsAirSystemId(0);
        let mut state = PurchasedAirCalcCoolingDehumidificationFlowRuntimeState::new(system);
        state.transition_count = call_count;
        state.cooling_body_entry_count = call_count;
        state.supply_mass_flow_rate_for_dehumidification_reset_assignment_count = call_count;
        state.cooling_on_read_count = call_count;
        state.cooling_on_body_entry_count = call_count;
        state.dehumidification_control_type_read_count = call_count;
        state.dehumidification_control_type_fallthrough_count = call_count;
        state.latest = Some(PurchasedAirCalcCoolingDehumidificationFlowSnapshot {
            source: PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE,
            system,
            parent_call_ordinal: call_count,
            source_order: PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SOURCE_ORDER,
            controlled_zone: ZoneId(0),
            unit_body_entered: true,
            predecessor_cooling_body_entered: true,
            predecessor_cooling_on_body_entered: true,
            predecessor_delta_temperature_body_entered: true,
            predecessor_supply_mass_flow_rate_for_cool_assigned: true,
            unit_off_skipped: false,
            non_cooling_skipped: false,
            cooling_body_entered: true,
            supply_mass_flow_rate_for_dehumidification_reset_assigned: true,
            reset_supply_mass_flow_rate_for_dehumidification_kg_per_s: Some(0.0),
            cooling_on_read: true,
            cooling_on: Some(true),
            cooling_on_body_entered: true,
            dehumidification_control_type_read: true,
            dehumidification_control_type: Some(DehumidificationControlType::None),
            dehumidification_control_type_humidistat: Some(false),
            dehumidification_control_body_entered: false,
            zone_dehumidifying_setpoint_moisture_demand_read: false,
            zone_dehumidifying_setpoint_moisture_demand_kg_per_s: None,
            zone_dehumidifying_setpoint_moisture_demand_assigned: false,
            assigned_zone_dehumidifying_setpoint_moisture_demand_kg_per_s: None,
            minimum_cooling_supply_air_humidity_ratio_read: false,
            minimum_cooling_supply_air_humidity_ratio_kg_water_per_kg_dry_air: None,
            zone_humidity_ratio_read: false,
            zone_humidity_ratio_kg_water_per_kg_dry_air: None,
            delta_humidity_ratio_calculated: false,
            delta_humidity_ratio_kg_water_per_kg_dry_air: None,
            delta_humidity_ratio_assigned: false,
            assigned_delta_humidity_ratio_kg_water_per_kg_dry_air: None,
            delta_humidity_ratio_for_gate_read: false,
            delta_humidity_ratio_for_gate_kg_water_per_kg_dry_air: None,
            delta_humidity_ratio_comparison_evaluated: false,
            delta_humidity_ratio_below_negative_small_delta: None,
            zone_dehumidifying_setpoint_moisture_demand_for_gate_read: false,
            zone_dehumidifying_setpoint_moisture_demand_for_gate_kg_per_s: None,
            zone_dehumidifying_setpoint_moisture_demand_comparison_evaluated: false,
            zone_dehumidifying_setpoint_moisture_demand_below_zero: None,
            dehumidification_flow_body_entered: false,
            zone_dehumidifying_setpoint_moisture_demand_for_division_read: false,
            zone_dehumidifying_setpoint_moisture_demand_for_division_kg_per_s: None,
            delta_humidity_ratio_for_division_read: false,
            delta_humidity_ratio_for_division_kg_water_per_kg_dry_air: None,
            supply_mass_flow_rate_for_dehumidification_calculated: false,
            calculated_supply_mass_flow_rate_for_dehumidification_kg_per_s: None,
            supply_mass_flow_rate_for_dehumidification_assigned: false,
            assigned_supply_mass_flow_rate_for_dehumidification_kg_per_s: None,
            resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s: Some(0.0),
        });
        PurchasedAirCalcCoolingDehumidificationFlowLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE,
            state,
        }
    }

    fn valid_cooling_humidification_flow_lifecycle(
        call_count: usize,
    ) -> PurchasedAirCalcCoolingHumidificationFlowLifecycleSummary {
        let system = IdealLoadsAirSystemId(0);
        let mut state = PurchasedAirCalcCoolingHumidificationFlowRuntimeState::new(system);
        state.transition_count = call_count;
        state.cooling_body_entry_count = call_count;
        state.reset_assignment_count = call_count;
        state.heating_on_read_count = call_count;
        state.heating_on_body_entry_count = call_count;
        state.humidification_control_type_read_count = call_count;
        state.humidification_control_type_fallthrough_count = call_count;
        state.latest = Some(PurchasedAirCalcCoolingHumidificationFlowSnapshot {
            source: PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE,
            source_order: PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE_ORDER,
            system,
            parent_call_ordinal: call_count,
            controlled_zone: ZoneId(0),
            unit_body_entered: true,
            predecessor_cooling_body_entered: true,
            unit_off_skipped: false,
            non_cooling_skipped: false,
            cooling_body_entered: true,
            supply_mass_flow_rate_for_humidification_reset_assigned: true,
            reset_supply_mass_flow_rate_for_humidification_kg_per_s: Some(0.0),
            heating_on_read: true,
            heating_on: Some(true),
            heating_on_body_entered: true,
            humidification_control_type_read: true,
            humidification_control_type: Some(HumidificationControlType::None),
            humidification_control_type_humidistat: Some(false),
            humidification_control_body_entered: false,
            dehumidification_control_type_first_read: false,
            first_dehumidification_control_type: None,
            dehumidification_control_type_humidistat: None,
            dehumidification_control_type_second_read: false,
            second_dehumidification_control_type: None,
            dehumidification_control_type_none: None,
            humidification_control_condition_admitted: false,
            zone_humidifying_setpoint_moisture_demand_read: false,
            zone_humidifying_setpoint_moisture_demand_kg_per_s: None,
            zone_humidifying_setpoint_moisture_demand_assigned: false,
            assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s: None,
            maximum_heating_supply_air_humidity_ratio_read: false,
            maximum_heating_supply_air_humidity_ratio_kg_water_per_kg_dry_air: None,
            zone_humidity_ratio_read: false,
            zone_humidity_ratio_kg_water_per_kg_dry_air: None,
            delta_humidity_ratio_calculated: false,
            delta_humidity_ratio_kg_water_per_kg_dry_air: None,
            delta_humidity_ratio_assigned: false,
            assigned_delta_humidity_ratio_kg_water_per_kg_dry_air: None,
            delta_humidity_ratio_for_gate_read: false,
            delta_humidity_ratio_for_gate_kg_water_per_kg_dry_air: None,
            delta_humidity_ratio_comparison_evaluated: false,
            delta_humidity_ratio_above_small_delta: None,
            zone_humidifying_setpoint_moisture_demand_for_gate_read: false,
            zone_humidifying_setpoint_moisture_demand_for_gate_kg_per_s: None,
            zone_humidifying_setpoint_moisture_demand_comparison_evaluated: false,
            zone_humidifying_setpoint_moisture_demand_above_zero: None,
            humidification_flow_body_entered: false,
            zone_humidifying_setpoint_moisture_demand_for_division_read: false,
            zone_humidifying_setpoint_moisture_demand_for_division_kg_per_s: None,
            delta_humidity_ratio_for_division_read: false,
            delta_humidity_ratio_for_division_kg_water_per_kg_dry_air: None,
            supply_mass_flow_rate_for_humidification_calculated: false,
            calculated_supply_mass_flow_rate_for_humidification_kg_per_s: None,
            supply_mass_flow_rate_for_humidification_assigned: false,
            assigned_supply_mass_flow_rate_for_humidification_kg_per_s: None,
            resulting_supply_mass_flow_rate_for_humidification_kg_per_s: Some(0.0),
        });
        PurchasedAirCalcCoolingHumidificationFlowLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE,
            state,
        }
    }

    fn valid_cooling_capacity_zero_flow_reset_lifecycle(
        call_count: usize,
        limit: IdealLoadsLimit,
        selected_capacity_w: Option<f64>,
    ) -> PurchasedAirCalcCoolingCapacityZeroFlowResetLifecycleSummary {
        let system = IdealLoadsAirSystemId(0);
        let first_matched = limit == IdealLoadsLimit::LimitCapacity;
        let second_executed = !first_matched;
        let second_matched = limit == IdealLoadsLimit::LimitFlowRateAndCapacity;
        let selected = first_matched || second_matched;
        let capacity = if selected {
            Some(selected_capacity_w.expect("selected CP321 limit needs a capacity"))
        } else {
            None
        };
        let zero_body = capacity.is_some_and(|value| value == 0.0);
        let cp318 = valid_cooling_sensible_flow_lifecycle(call_count);
        let prior_cool = cp318
            .state
            .latest
            .expect("valid CP318 latest snapshot")
            .resulting_supply_mass_flow_rate_for_cool_kg_per_s;
        let prior_dehumidification = Some(0.0);
        let prior_humidification = Some(0.0);
        let assigned = zero_body.then_some(0.0_f64);
        let resulting_cool = if zero_body { assigned } else { prior_cool };
        let resulting_dehumidification = if zero_body {
            assigned
        } else {
            prior_dehumidification
        };
        let resulting_humidification = if zero_body {
            assigned
        } else {
            prior_humidification
        };

        let mut state = PurchasedAirCalcCoolingCapacityZeroFlowResetRuntimeState::new(system);
        state.transition_count = call_count;
        state.cooling_body_entry_count = call_count;
        state.first_cooling_limit_read_count = call_count;
        state.cooling_limit_capacity_count = usize::from(first_matched) * call_count;
        state.second_cooling_limit_read_count = usize::from(second_executed) * call_count;
        state.cooling_limit_flow_rate_and_capacity_count = usize::from(second_matched) * call_count;
        state.cooling_limit_rejected_count =
            usize::from(second_executed && !second_matched) * call_count;
        state.maximum_total_cooling_capacity_read_count = usize::from(selected) * call_count;
        state.maximum_total_cooling_capacity_comparison_count = usize::from(selected) * call_count;
        state.maximum_total_cooling_capacity_zero_count = usize::from(zero_body) * call_count;
        state.maximum_total_cooling_capacity_nonzero_count =
            usize::from(selected && !zero_body) * call_count;
        state.zero_cooling_capacity_body_entry_count = usize::from(zero_body) * call_count;
        state.supply_mass_flow_rate_for_cool_zero_assignment_count =
            usize::from(zero_body) * call_count;
        state.supply_mass_flow_rate_for_dehumidification_zero_assignment_count =
            usize::from(zero_body) * call_count;
        state.supply_mass_flow_rate_for_humidification_zero_assignment_count =
            usize::from(zero_body) * call_count;
        state.latest = Some(PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot {
            source: PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_FIRST_EXCLUDED_SOURCE,
            source_order: PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE_ORDER,
            system,
            parent_call_ordinal: call_count,
            controlled_zone: ZoneId(0),
            unit_body_entered: true,
            predecessor_cooling_body_entered: true,
            unit_off_skipped: false,
            non_cooling_skipped: false,
            cooling_body_entered: true,
            first_cooling_limit_read: true,
            first_cooling_limit: Some(limit),
            cooling_limit_capacity: Some(first_matched),
            second_cooling_limit_read: second_executed,
            second_cooling_limit: second_executed.then_some(limit),
            cooling_limit_flow_rate_and_capacity: second_executed.then_some(second_matched),
            cooling_limit_condition_satisfied: Some(selected),
            maximum_total_cooling_capacity_read: selected,
            maximum_total_cooling_capacity_w: capacity,
            maximum_total_cooling_capacity_comparison_evaluated: selected,
            maximum_total_cooling_capacity_equal_to_zero: selected.then_some(zero_body),
            zero_cooling_capacity_body_entered: zero_body,
            predecessor_supply_mass_flow_rate_for_cool_kg_per_s: prior_cool,
            predecessor_supply_mass_flow_rate_for_dehumidification_kg_per_s: prior_dehumidification,
            predecessor_supply_mass_flow_rate_for_humidification_kg_per_s: prior_humidification,
            supply_mass_flow_rate_for_cool_zero_assigned: zero_body,
            assigned_supply_mass_flow_rate_for_cool_kg_per_s: assigned,
            supply_mass_flow_rate_for_dehumidification_zero_assigned: zero_body,
            assigned_supply_mass_flow_rate_for_dehumidification_kg_per_s: assigned,
            supply_mass_flow_rate_for_humidification_zero_assigned: zero_body,
            assigned_supply_mass_flow_rate_for_humidification_kg_per_s: assigned,
            resulting_supply_mass_flow_rate_for_cool_kg_per_s: resulting_cool,
            resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s:
                resulting_dehumidification,
            resulting_supply_mass_flow_rate_for_humidification_kg_per_s: resulting_humidification,
        });
        PurchasedAirCalcCoolingCapacityZeroFlowResetLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_FIRST_EXCLUDED_SOURCE,
            state,
        }
    }

    fn valid_minimum_oa_prefix_lifecycle(
        call_count: usize,
    ) -> PurchasedAirCalcMinimumOaPrefixLifecycleSummary {
        let system = IdealLoadsAirSystemId(0);
        let mut state = PurchasedAirCalcMinimumOaPrefixRuntimeState::new(system);
        state.transition_count = call_count;
        state.source_execution_count = call_count;
        state.zone_heat_balance_reference_count = call_count;
        state.minimum_oa_child_call_count = call_count;
        state.minimum_oa_child_no_outdoor_air_count = call_count;
        state.retained_minimum_outdoor_air_write_count = call_count;
        state.ems_override_flag_read_count = call_count;
        state.outdoor_air_flag_read_count = call_count;
        state.no_outdoor_air_zero_branch_count = call_count;
        state.latest = Some(PurchasedAirCalcMinimumOaPrefixSnapshot {
            source: PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE,
            minimum_oa_child_source: PURCHASED_AIR_CALC_MINIMUM_OA_CHILD_SOURCE,
            system,
            parent_call_ordinal: call_count,
            source_order: PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE_ORDER,
            controlled_zone: ZoneId(0),
            unit_body_entered: true,
            zone_heat_balance_reference_bound: true,
            minimum_oa_child_called: true,
            minimum_oa_child_no_outdoor_air_route: true,
            retained_minimum_outdoor_air_mass_flow_rate_kg_per_s: Some(0.0),
            retained_minimum_outdoor_air_write_performed: true,
            ems_override_flag_read: true,
            ems_override_enabled: Some(false),
            ems_override_applied: false,
            working_outdoor_air_mass_flow_rate_kg_per_s: Some(0.0),
            outdoor_air_flag_read: true,
            outdoor_air_enabled: Some(false),
            no_outdoor_air_zero_branch_entered: true,
            psychrometric_call_count: 0,
            minimum_outdoor_air_sensible_output_w: Some(0.0),
            minimum_outdoor_air_moisture_output_kg_per_s: Some(0.0),
        });
        PurchasedAirCalcMinimumOaPrefixLifecycleSummary {
            source: PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE,
            minimum_oa_child_source: PURCHASED_AIR_CALC_MINIMUM_OA_CHILD_SOURCE,
            state,
        }
    }

    fn valid_calc_entry_lifecycle(call_count: usize) -> PurchasedAirCalcEntryLifecycleSummary {
        let system = IdealLoadsAirSystemId(0);
        let mut state = PurchasedAirCalcEntryRuntimeState::new(system);
        state.call_count = call_count;
        state.reset_count = call_count;
        state.demand_read_count = call_count;
        state.overall_availability_read_count = call_count;
        state.heating_availability_read_count = call_count;
        state.cooling_availability_read_count = call_count;
        state.availability_manager_read_count = call_count;
        state.availability_manager_zone_write_count = call_count;
        state.availability_status_copy_count = call_count;
        state.availability_manager_zone = Some(ZoneId(0));
        state.unit_body_entry_count = call_count;
        state.heating_on_count = call_count;
        state.cooling_on_count = call_count;
        state.latest = Some(PurchasedAirCalcEntrySnapshot {
            source: PURCHASED_AIR_CALC_ENTRY_SOURCE,
            system,
            call_ordinal: call_count,
            source_order: PURCHASED_AIR_CALC_ENTRY_SOURCE_ORDER,
            controlled_zone: ZoneId(0),
            supply_node: NodeId(3),
            zone_node: NodeId(5),
            outdoor_air_node: None,
            recirculation_node: NodeId(4),
            reset: PurchasedAirCalcEntryResetSnapshot::default(),
            demand: PurchasedAirCalcEntryDemandSnapshot {
                zone: ZoneId(0),
                sensible_input_kind: ZoneSensibleDemandInputKind::SourceSetpointThresholds,
                remaining_output_req_to_heat_sp_w: 100.0,
                remaining_output_req_to_cool_sp_w: -50.0,
            },
            unit_defaulted_on: true,
            economizer_defaulted_on: false,
            availability_manager_read_site_visited: true,
            availability_manager_zone_written: true,
            copied_availability_status: Some(PurchasedAirAvailabilityStatus::NoAction),
            force_off_applied: false,
            overall_availability_read_site_visited: true,
            heating_availability_read_site_visited: true,
            cooling_availability_read_site_visited: true,
            overall_availability: 1.0,
            heating_availability: 1.0,
            cooling_availability: 1.0,
            unit_on: true,
            heating_on: true,
            cooling_on: true,
            unit_body_entered: true,
        });
        assert_eq!(
            state.availability_status,
            PurchasedAirAvailabilityStatus::NoAction
        );
        PurchasedAirCalcEntryLifecycleSummary {
            source: PURCHASED_AIR_CALC_ENTRY_SOURCE,
            state,
        }
    }

    fn valid_init_lifecycle(call_count: usize) -> PurchasedAirInitLifecycleSummary {
        let sized_limits = PurchasedAirSizedLimits {
            maximum_heating_air_flow_rate_m3_per_s: None,
            maximum_sensible_heating_capacity_w: None,
            maximum_cooling_air_flow_rate_m3_per_s: None,
            maximum_total_cooling_capacity_w: None,
        };
        let skipped_field = |field| {
            Some(PurchasedAirHardSizeFieldOutcome {
                field,
                input_value: None,
                child_sizer_called: false,
                child_result: None,
                object_writeback: false,
                local_design_value: 0.0,
                child_user_report_records: 0,
                outer_report_records: 0,
                child_sizing_label_unit: "m3/s",
            })
        };
        PurchasedAirInitLifecycleSummary {
            source: PURCHASED_AIR_INIT_LIFECYCLE_SOURCE,
            flags: IdealLoadsInitFlags {
                state_machine_used: true,
                one_time_checked: true,
                topology_ready: true,
                environment_initialized: true,
                environment_initialization_needed: call_count > 1,
                sizing_checked: true,
                equipment_list_checked: true,
                return_plenum_inactive: true,
            },
            module_initialization_count: 1,
            equipment_list_check_count: 1,
            declared_system_order: vec![IdealLoadsAirSystemId(0)],
            equipment_list_scan_order: vec![IdealLoadsAirSystemId(0)],
            equipment_list_scanned_unit_count: 1,
            equipment_list_missing_unit_count: 0,
            equipment_list_diagnostics: Vec::new(),
            equipment_list_scan_ordinal: Some(1),
            first_matching_equipment_list: Some(ZoneEquipmentListId(0)),
            equipment_list_membership_found: Some(true),
            controlled_zone: Some(ep_model::ZoneId(0)),
            equipment_list: Some(ZoneEquipmentListId(0)),
            supply_node: Some(ep_model::NodeId(3)),
            recirculation_node: Some(ep_model::NodeId(4)),
            recirculation_source: Some(PurchasedAirRecirculationSource::SingleZoneReturn),
            rejected_exhaust_node: None,
            reported_first_return_node: None,
            topology_diagnostics: Vec::new(),
            topology_failure: None,
            init_call_count: call_count,
            one_time_initialization_count: 1,
            topology_completion_count: 1,
            sizing_attempt_count: 1,
            sizing_check_count: 1,
            sized_limits: Some(sized_limits),
            sizing_outcome: Some(PurchasedAirHardSizeLegacyOutcome {
                route: PurchasedAirHardSizeLegacyRoute::DirectHardSizedNoSizingRun,
                sized_limits,
                fields: [
                    skipped_field(PurchasedAirHardSizeField::MaximumHeatingAirFlowRate),
                    skipped_field(PurchasedAirHardSizeField::MaximumSensibleHeatingCapacity),
                    skipped_field(PurchasedAirHardSizeField::MaximumCoolingAirFlowRate),
                    skipped_field(PurchasedAirHardSizeField::MaximumTotalCoolingCapacity),
                ],
                entry_fan_flags_cleared: true,
            }),
            environment_initialization_count: 1,
            environment_rearm_count: usize::from(call_count > 1),
            maximum_heating_air_mass_flow_rate_kg_per_s: 0.0,
            maximum_cooling_air_mass_flow_rate_kg_per_s: 0.0,
            standard_air_density_kg_per_m3: Some(1.2),
            supply_temperature_registered_recurring_diagnostic_count: 0,
            supply_temperature_diagnostic_event_count: 0,
            supply_temperature_characterized_severe_error_count_increment: 0,
            cooling_supply_temperature_error_index: 0,
            heating_supply_temperature_error_index: 0,
            cooling_supply_temperature_first_diagnostic_count: 0,
            heating_supply_temperature_first_diagnostic_count: 0,
            supply_temperature_diagnostics: Vec::new(),
            cooling_supply_temperature_warning_count: 0,
            heating_supply_temperature_warning_count: 0,
            economizer_flow_limit_warning_count: 0,
        }
    }

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
    fn schedule_cache_json_exposes_stable_structural_profile() {
        let profile = ScheduleCacheProfile {
            scalar_series_count: 2,
            dense_series_count: 3,
            logical_sample_count: 120,
            allocated_dense_sample_count: 72,
            index_kind: ScheduleSeriesIndexKind::DenseIdentity,
            ambiguous_id_count: 0,
        };

        let cache = schedule_cache_json(24, profile);
        assert_eq!(cache["sample_count"], 24);
        assert_eq!(cache["profile"]["scalar_series_count"], 2);
        assert_eq!(cache["profile"]["dense_series_count"], 3);
        assert_eq!(cache["profile"]["logical_sample_count"], 120);
        assert_eq!(cache["profile"]["allocated_dense_sample_count"], 72);
        assert_eq!(cache["profile"]["index_kind"], "dense_identity");
        assert_eq!(cache["profile"]["ambiguous_id_count"], 0);

        let sparse = schedule_cache_json(
            24,
            ScheduleCacheProfile {
                index_kind: ScheduleSeriesIndexKind::Sparse,
                ambiguous_id_count: 1,
                ..profile
            },
        );
        assert_eq!(sparse["profile"]["index_kind"], "sparse");
        assert_eq!(sparse["profile"]["ambiguous_id_count"], 1);
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

    #[test]
    fn typed_counts_separate_file_shading_object_and_generated_schedules() {
        let model = TypedModel {
            file_shading_schedule: Some(ScheduleFileShading {
                file_name: "shading.csv".to_string(),
                timesteps_per_hour: 1,
                source_day_count: 365,
                columns: [7_u32, 11]
                    .into_iter()
                    .map(|id| ScheduleFileShadingColumn {
                        id: ScheduleId(id),
                        surface_header: format!("Surface {id}"),
                        schedule_name: NormalizedName::new(&format!("Surface {id}_shading")),
                        values: Vec::new(),
                    })
                    .collect(),
            }),
            ..TypedModel::default()
        };

        let counts = typed_counts(&model);
        assert_eq!(counts["file_shading_schedule_objects"], 1);
        assert_eq!(counts["file_shading_generated_schedules"], 2);
        assert_eq!(counts["schedules"], 2);
    }

    #[test]
    fn typed_counts_include_external_interface_schedule_families() {
        let model = TypedModel {
            external_interface_schedules: vec![ExternalInterfaceSchedule {
                id: ScheduleId(0),
                name: NormalizedName::new("External"),
                schedule_type_limits: None,
                initial_value: 0.375,
            }],
            external_interface_fmu_import_schedules: vec![ExternalInterfaceFmuImportSchedule {
                id: ScheduleId(1),
                name: NormalizedName::new("FMU Import"),
                schedule_type_limits: None,
                fmu_file_name: "unused.fmu".to_string(),
                fmu_instance_name: "UnusedInstance".to_string(),
                fmu_variable_name: "UnusedOutput".to_string(),
                initial_value: 0.625,
            }],
            external_interface_fmu_export_schedules: vec![ExternalInterfaceFmuExportSchedule {
                id: ScheduleId(2),
                name: NormalizedName::new("FMU Export"),
                schedule_type_limits: None,
                fmu_variable_name: "UnusedInput".to_string(),
                initial_value: 0.875,
            }],
            ..TypedModel::default()
        };

        let counts = typed_counts(&model);
        assert_eq!(counts["external_interface_schedules"], 1);
        assert_eq!(counts["external_interface_fmu_import_schedules"], 1);
        assert_eq!(counts["external_interface_fmu_export_schedules"], 1);
        assert_eq!(counts["schedules"], 3);
    }
}
