use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use ep_compare::{
    SeriesAlignment, SeriesComparisonStatus, SeriesDivergenceKind, SeriesSample, Tolerance,
    compare_series_samples_v2, load_eso_time_series, load_mtr_time_series,
};
use ep_compiler::compile_raw_model;
use ep_conformance::{
    ComparisonClass, ConformanceCase, EvidenceDomain, MeterRequest, OutputFrequency, OutputLevel,
    OutputRequest, SourceArtifact, VariableClass,
};
use ep_model::{
    AutoOrNumber, AutosizeOrNumber, DehumidificationControlType, DemandControlledVentilationType,
    DesignSpecificationOutdoorAirMethod, FirstHourInterpolationStartingValues, HeatRecoveryType,
    HumidificationControlType, IdealLoadsAirSystem, IdealLoadsLimit, NormalizedName,
    OutdoorAirEconomizerType, OutputHandle, PeopleNumberCalculationMethod, ScheduleId,
    SimulationModel, SurfaceType, TypedModel, Zone,
};
use ep_raw_model::load_epjson_file;
use ep_runtime::{
    EpwRecord, IDEAL_LOADS_ENERGY_OUTPUT_LEVEL_POLICY, IDEAL_LOADS_ENERGY_OUTPUT_TIMESTEP_SOURCE,
    IDEAL_LOADS_FUEL_ENERGY_OUTPUT_LEVEL_POLICY, IDEAL_LOADS_METER_AGGREGATION_SOURCE,
    IDEAL_LOADS_METER_FUEL_ENERGY_BINDING_SOURCE, IDEAL_LOADS_NODE_OUTPUT_REPORT_SOURCE,
    IDEAL_LOADS_NODE_OUTPUT_STATE_STRUCT, IDEAL_LOADS_NODE_OUTPUT_STORE_TYPE,
    IDEAL_LOADS_NODE_OUTPUT_UPDATE_SOURCE, IDEAL_LOADS_RATE_OUTPUT_SOURCE,
    IDEAL_LOADS_RATE_OUTPUT_TIMESTEP_SOURCE, IDEAL_LOADS_ZONE_EQUIPMENT_DISPATCH_PATH,
    IdealLoadsOutdoorAirContext, IdealLoadsOutdoorAirNodeState, IdealLoadsOutdoorAirSensibleResult,
    IdealLoadsSensibleLimitContext, IdealLoadsSensibleMode, IdealLoadsSensibleResult,
    IdealLoadsUnsupportedFeature, IdealLoadsZoneEquipmentDispatchValidation, IdealLoadsZoneState,
    OutputSeries, ResultStore, RuntimeMeterRequest, RuntimeOutputRegistry,
    SimPurchasedAirCompatInput, ZONE_IDEAL_LOADS_ECONOMIZER_ACTIVE_TIME,
    ZONE_IDEAL_LOADS_HEAT_RECOVERY_ACTIVE_TIME, ZONE_IDEAL_LOADS_HEAT_RECOVERY_LATENT_COOLING_RATE,
    ZONE_IDEAL_LOADS_HEAT_RECOVERY_LATENT_HEATING_RATE,
    ZONE_IDEAL_LOADS_HEAT_RECOVERY_SENSIBLE_COOLING_RATE,
    ZONE_IDEAL_LOADS_HEAT_RECOVERY_SENSIBLE_HEATING_RATE,
    ZONE_IDEAL_LOADS_HEAT_RECOVERY_TOTAL_COOLING_RATE,
    ZONE_IDEAL_LOADS_HEAT_RECOVERY_TOTAL_HEATING_RATE, ZONE_IDEAL_LOADS_MIXED_AIR_HUMIDITY_RATIO,
    ZONE_IDEAL_LOADS_MIXED_AIR_TEMPERATURE, ZONE_IDEAL_LOADS_OUTDOOR_AIR_LATENT_COOLING_RATE,
    ZONE_IDEAL_LOADS_OUTDOOR_AIR_LATENT_HEATING_RATE, ZONE_IDEAL_LOADS_OUTDOOR_AIR_MASS_FLOW_RATE,
    ZONE_IDEAL_LOADS_OUTDOOR_AIR_SENSIBLE_COOLING_RATE,
    ZONE_IDEAL_LOADS_OUTDOOR_AIR_SENSIBLE_HEATING_RATE,
    ZONE_IDEAL_LOADS_OUTDOOR_AIR_STANDARD_DENSITY_VOLUME_FLOW_RATE,
    ZONE_IDEAL_LOADS_OUTDOOR_AIR_TOTAL_COOLING_RATE,
    ZONE_IDEAL_LOADS_OUTDOOR_AIR_TOTAL_HEATING_RATE, ZONE_IDEAL_LOADS_SUPPLY_AIR_HUMIDITY_RATIO,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_COOLING_RATE,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_HEATING_RATE, ZONE_IDEAL_LOADS_SUPPLY_AIR_MASS_FLOW_RATE,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_SENSIBLE_COOLING_RATE,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_SENSIBLE_HEATING_RATE,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_STANDARD_DENSITY_VOLUME_FLOW_RATE,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_TEMPERATURE, ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_ENERGY,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_FUEL_ENERGY,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_FUEL_ENERGY_RATE,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_RATE,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_ENERGY,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_FUEL_ENERGY,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_FUEL_ENERGY_RATE,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_RATE, ZONE_IDEAL_LOADS_ZONE_COOLING_FUEL_ENERGY,
    ZONE_IDEAL_LOADS_ZONE_COOLING_FUEL_ENERGY_RATE, ZONE_IDEAL_LOADS_ZONE_HEATING_FUEL_ENERGY,
    ZONE_IDEAL_LOADS_ZONE_HEATING_FUEL_ENERGY_RATE, ZONE_IDEAL_LOADS_ZONE_LATENT_COOLING_RATE,
    ZONE_IDEAL_LOADS_ZONE_LATENT_HEATING_RATE, ZONE_IDEAL_LOADS_ZONE_SENSIBLE_COOLING_RATE,
    ZONE_IDEAL_LOADS_ZONE_SENSIBLE_HEATING_RATE, ZONE_IDEAL_LOADS_ZONE_TOTAL_COOLING_ENERGY,
    ZONE_IDEAL_LOADS_ZONE_TOTAL_COOLING_RATE, ZONE_IDEAL_LOADS_ZONE_TOTAL_HEATING_ENERGY,
    ZONE_IDEAL_LOADS_ZONE_TOTAL_HEATING_RATE, ZONE_SYS_ENERGY_DEMAND_COOLING_FIELD,
    ZONE_SYS_ENERGY_DEMAND_COOLING_SIGN_CONVENTION, ZONE_SYS_ENERGY_DEMAND_FIXTURE_MODE,
    ZONE_SYS_ENERGY_DEMAND_HEATING_FIELD, ZONE_SYS_ENERGY_DEMAND_HEATING_SIGN_CONVENTION,
    ZONE_SYS_ENERGY_DEMAND_INPUT_SOURCE, ZONE_SYS_ENERGY_DEMAND_MISMATCH_CLASSIFICATION,
    ZONE_SYS_ENERGY_DEMAND_SOURCE_FILE, ZONE_SYS_ENERGY_DEMAND_SOURCE_STRUCT,
    ZONE_SYSTEM_PREDICTED_DEHUMIDIFYING_MOISTURE_LOAD,
    ZONE_SYSTEM_PREDICTED_HUMIDIFYING_MOISTURE_LOAD, ZONE_THERMOSTAT_COOLING_SETPOINT_TEMPERATURE,
    ZONE_THERMOSTAT_HEATING_SETPOINT_TEMPERATURE, ZoneSysEnergyDemand,
    calc_outdoor_air_sensible_report_rates_compat,
    calc_scheduled_outdoor_air_mass_flow_rate_kg_per_s, classify_no_oa_no_limit_sensible_subset,
    classify_no_oa_sensible_subset, design_outdoor_air_volume_flow_components_m3_per_s,
    ideal_loads_facility_meter_binding, ideal_loads_zone_equipment_stages, load_epw_records,
    purchased_air_source_order_stages, select_purchased_air_branch, sim_purchased_air_compat,
    surface_area_m2, validate_ideal_loads_zone_equipment_dispatch,
};

use crate::conformance_artifacts::{BaselineSummary, generate_conformance_baseline_in_dir};
use crate::{
    comparison_class_label, evidence_domain_label, json_number, json_string, markdown_cell,
    output_frequency_label, output_level_label, source_artifact_label, variable_class_label,
};

const SYSTEM_NODE_TEMPERATURE: &str = "System Node Temperature";
const SYSTEM_NODE_HUMIDITY_RATIO: &str = "System Node Humidity Ratio";
const SYSTEM_NODE_MASS_FLOW_RATE: &str = "System Node Mass Flow Rate";
const ZONE_AIR_HUMIDITY_RATIO: &str = "Zone Air Humidity Ratio";
const ZONE_SYSTEM_PREDICTED_SETPOINT_LOAD: &str =
    "Zone System Predicted Sensible Load to Setpoint Heat Transfer Rate";
const ZONE_SYSTEM_PREDICTED_HEATING_LOAD: &str =
    "Zone System Predicted Sensible Load to Heating Setpoint Heat Transfer Rate";
const ZONE_SYSTEM_PREDICTED_COOLING_LOAD: &str =
    "Zone System Predicted Sensible Load to Cooling Setpoint Heat Transfer Rate";
const IDEAL_LOADS_NO_OA_ENERGY_SYSTEM_SUBSTEPS: f64 = 8.0;
const IDEAL_LOADS_OUTDOOR_AIR_SYSTEM_SUBSTEPS: f64 = 8.0;
const IDEAL_LOADS_OUTDOOR_AIR_FLOW_ZONE_CONFORMANCE_CASE_ID: &str =
    "ideal_loads_outdoor_air_flow_zone_conformance_candidate_001";
const IDEAL_LOADS_OUTDOOR_AIR_FLOW_PERSON_CONFORMANCE_CASE_ID: &str =
    "ideal_loads_outdoor_air_flow_person_conformance_candidate_001";
const IDEAL_LOADS_OUTDOOR_AIR_FLOW_AREA_CONFORMANCE_CASE_ID: &str =
    "ideal_loads_outdoor_air_flow_area_conformance_candidate_001";
const IDEAL_LOADS_OUTDOOR_AIR_AIR_CHANGES_CONFORMANCE_CASE_ID: &str =
    "ideal_loads_outdoor_air_air_changes_conformance_candidate_001";
const IDEAL_LOADS_OUTDOOR_AIR_SUM_CONFORMANCE_CASE_ID: &str =
    "ideal_loads_outdoor_air_sum_conformance_candidate_001";
const IDEAL_LOADS_BLANK_FUEL_EFFICIENCY_RATE_SOURCE: &str =
    "rust-ideal-loads-blank-fuel-efficiency";
const IDEAL_LOADS_BLANK_FUEL_EFFICIENCY_ENERGY_SOURCE: &str =
    "rust-ideal-loads-blank-fuel-efficiency-time-step-energy";
const IDEAL_LOADS_CONSTANT_FUEL_EFFICIENCY_RATE_SOURCE: &str =
    "rust-ideal-loads-constant-fuel-efficiency";
const IDEAL_LOADS_CONSTANT_FUEL_EFFICIENCY_ENERGY_SOURCE: &str =
    "rust-ideal-loads-constant-fuel-efficiency-time-step-energy";
const IDEAL_LOADS_BLANK_FUEL_EFFICIENCY_REPORT_SOURCE: &str =
    "EnergyPlus ReportPurchasedAir blank fuel-efficiency schedule branch; diagnostic-only";
const IDEAL_LOADS_CONSTANT_FUEL_EFFICIENCY_REPORT_SOURCE: &str = "EnergyPlus ReportPurchasedAir constant Schedule:Constant fuel-efficiency schedule branch; diagnostic-only";
const IDEAL_LOADS_FACILITY_METER_RUST_SOURCE: &str =
    "rust-ideal-loads-hourly-facility-meter-from-fuel-energy";
const IDEAL_LOADS_FACILITY_METER_REPORT_SOURCE: &str =
    "EnergyPlus Output:Meter hourly MTR vs Rust aggregated fuel-energy diagnostic";
const IDEAL_LOADS_FINITE_LIMIT_RECIRCULATION_STATE_SOURCE: &str = "EnergyPlus return/exhaust recirculation node same-call state for finite-limit no-OA mixed-air and report calculations";
const IDEAL_LOADS_HUMIDITY_CONTROL_RECIRCULATION_STATE_SOURCE: &str = "EnergyPlus return/exhaust recirculation node same-call state for no-OA humidity-control mixed-air calculations";
const IDEAL_LOADS_SOURCE_MAP_ANCHOR: &str = "docs/src/porting-map/ideal-loads-source-map.md";
const IDEAL_LOADS_NODE_OUTPUT_TIMESTAMP_ALIGNMENT: &str = "timestamp";

pub(crate) struct IdealLoadsDiagnosticReportSummary {
    pub(crate) baseline: BaselineSummary,
    pub(crate) report_dir: PathBuf,
    pub(crate) compare_report: PathBuf,
    pub(crate) compare_summary: PathBuf,
    pub(crate) selected_outputs: PathBuf,
    pub(crate) rust_result_store: PathBuf,
    pub(crate) variable_deltas: PathBuf,
    pub(crate) first_divergence: PathBuf,
    pub(crate) tolerance_failures: PathBuf,
    pub(crate) stage_summary: PathBuf,
    pub(crate) series_count: usize,
    pub(crate) compared_samples: usize,
    pub(crate) tolerance_failures_count: usize,
    pub(crate) tolerance_policy: &'static str,
    pub(crate) status: &'static str,
}

struct IdealLoadsDiagnosticContext<'a> {
    manifest: &'a ConformanceCase,
    baseline: &'a BaselineSummary,
    branch: &'static str,
    selected_purchased_air_branch: &'static str,
    declared_ideal_loads_branch: &'static str,
    inactive_branches: Vec<&'static str>,
    zone_equipment_dispatch: IdealLoadsZoneEquipmentDispatchValidation,
    constant_shr_conformance_claim: bool,
    constant_supply_humidity_cooling_conformance_claim: bool,
    constant_supply_humidity_heating_conformance_claim: bool,
    humidistat_dehumidification_conformance_claim: bool,
    humidistat_humidification_conformance_claim: bool,
    zone_name: String,
    zone_air_node_name: String,
    recirculation_node_name: Option<String>,
    system_name: String,
    supply_node_name: String,
    system_timestep_seconds: f64,
    energy_report_interval_seconds: f64,
    fuel_efficiency: IdealLoadsFuelEfficiencyContext,
    rows: Vec<IdealLoadsDiagnosticRow>,
    meter_rows: Vec<IdealLoadsMeterDiagnosticRow>,
    result_store: ResultStore,
    input_trace: IdealLoadsInputTrace,
    mode_counts: IdealLoadsModeCounts,
}

#[derive(Clone, Copy)]
struct IdealLoadsFuelEfficiencyContext {
    heating: f64,
    cooling: f64,
    report_source: &'static str,
    rate_rust_source: &'static str,
    energy_rust_source: &'static str,
}

impl IdealLoadsFuelEfficiencyContext {
    fn blank() -> Self {
        Self {
            heating: 1.0,
            cooling: 1.0,
            report_source: IDEAL_LOADS_BLANK_FUEL_EFFICIENCY_REPORT_SOURCE,
            rate_rust_source: IDEAL_LOADS_BLANK_FUEL_EFFICIENCY_RATE_SOURCE,
            energy_rust_source: IDEAL_LOADS_BLANK_FUEL_EFFICIENCY_ENERGY_SOURCE,
        }
    }

    fn constant(heating: f64, cooling: f64) -> Self {
        Self {
            heating,
            cooling,
            report_source: IDEAL_LOADS_CONSTANT_FUEL_EFFICIENCY_REPORT_SOURCE,
            rate_rust_source: IDEAL_LOADS_CONSTANT_FUEL_EFFICIENCY_RATE_SOURCE,
            energy_rust_source: IDEAL_LOADS_CONSTANT_FUEL_EFFICIENCY_ENERGY_SOURCE,
        }
    }
}

struct IdealLoadsOutdoorAirDiagnosticContext<'a> {
    manifest: &'a ConformanceCase,
    baseline: &'a BaselineSummary,
    branch: &'static str,
    zone_name: String,
    system_name: String,
    outdoor_air_spec_name: String,
    outdoor_air_method: DesignSpecificationOutdoorAirMethod,
    outdoor_air_node_name: String,
    outdoor_air_economizer_type: OutdoorAirEconomizerType,
    heat_recovery_type: HeatRecoveryType,
    standard_air_density_kg_per_m3: f64,
    design_people_count: f64,
    zone_floor_area_m2: f64,
    zone_volume_m3: f64,
    flow_per_person_m3_per_s: f64,
    flow_per_area_m3_per_s: f64,
    flow_per_zone_m3_per_s: f64,
    air_changes_m3_per_s: f64,
    design_volume_flow_rate_m3_per_s: f64,
    outdoor_air_mass_flow_rate_kg_per_s: f64,
    sample_count: usize,
    rows: Vec<IdealLoadsDiagnosticRow>,
    result_store: ResultStore,
}

struct IdealLoadsInputTrace {
    sample_count: usize,
    zone_node_temperature: LoadedSeries,
    zone_node_humidity_ratio: LoadedSeries,
    recirculation_node_temperature: LoadedSeries,
    recirculation_node_humidity_ratio: LoadedSeries,
    active_demand: LoadedSeries,
    heating_demand: LoadedSeries,
    cooling_demand: LoadedSeries,
    humidifying_moisture_demand: LoadedSeries,
    dehumidifying_moisture_demand: LoadedSeries,
}

#[derive(Clone)]
struct LoadedSeries {
    units: Option<String>,
    samples: Vec<SeriesSample>,
}

struct IdealLoadsDiagnosticRow {
    key: String,
    variable: String,
    frequency: OutputFrequency,
    variable_class: VariableClass,
    source: SourceArtifact,
    domain: Option<EvidenceDomain>,
    level: Option<OutputLevel>,
    units: String,
    oracle_units: Option<String>,
    rust_source: &'static str,
    tolerance: Tolerance,
    max_rmse_tolerance: Option<f64>,
    expected_samples: usize,
    observed_samples: usize,
    compared_samples: usize,
    max_abs_delta: f64,
    mean_abs_delta: f64,
    rmse_delta: f64,
    max_rel_delta: f64,
    alignment: SeriesAlignment,
    first_divergence: Option<ep_compare::SeriesDivergenceV2>,
    status: SeriesComparisonStatus,
}

struct IdealLoadsMeterDiagnosticRow {
    name: String,
    frequency: OutputFrequency,
    source: SourceArtifact,
    domain: EvidenceDomain,
    level: OutputLevel,
    units: String,
    oracle_units: Option<String>,
    rust_source: &'static str,
    tolerance: Tolerance,
    max_rmse_tolerance: Option<f64>,
    expected_samples: usize,
    observed_samples: usize,
    compared_samples: usize,
    max_abs_delta: f64,
    mean_abs_delta: f64,
    rmse_delta: f64,
    max_rel_delta: f64,
    alignment: SeriesAlignment,
    first_divergence: Option<ep_compare::SeriesDivergenceV2>,
    status: SeriesComparisonStatus,
}

#[derive(Clone, Copy, Debug, Default)]
struct IdealLoadsModeCounts {
    off: usize,
    deadband: usize,
    cooling: usize,
    heating: usize,
}

pub(crate) fn generate_ideal_loads_no_oa_sensible_report(
    case_path: &Path,
    manifest: &ConformanceCase,
    oracle_root: &Path,
    output_root: &Path,
) -> Result<IdealLoadsDiagnosticReportSummary, String> {
    validate_manifest(manifest)?;

    let case_output_dir = output_root.join(&manifest.id);
    let oracle_output_dir = case_output_dir.join("oracle");
    let compare_dir = case_output_dir.join("compare");

    let baseline =
        generate_conformance_baseline_in_dir(case_path, manifest, oracle_root, &oracle_output_dir)?;
    let (series_count, compared_samples, tolerance_failures_count, tolerance_policy, status) = {
        let context = build_context(manifest, &baseline)?;
        write_artifacts(&compare_dir, &context)?;

        let tolerance_failures_count = context
            .rows
            .iter()
            .filter(|row| row.status == SeriesComparisonStatus::Fail)
            .count();
        let status = overall_status(&context);
        (
            context.rows.len(),
            context.input_trace.sample_count,
            tolerance_failures_count,
            tolerance_policy(&context),
            status,
        )
    };

    Ok(IdealLoadsDiagnosticReportSummary {
        baseline,
        report_dir: compare_dir.clone(),
        compare_report: compare_dir.join("compare-report.md"),
        compare_summary: compare_dir.join("compare-summary.json"),
        selected_outputs: compare_dir.join("selected_outputs.json"),
        rust_result_store: compare_dir.join("rust-result-store.json"),
        variable_deltas: compare_dir.join("variable-deltas.csv"),
        first_divergence: compare_dir.join("first-divergence.csv"),
        tolerance_failures: compare_dir.join("tolerance-failures.csv"),
        stage_summary: compare_dir.join("stage-summary.json"),
        series_count,
        compared_samples,
        tolerance_failures_count,
        tolerance_policy,
        status,
    })
}

pub(crate) fn generate_ideal_loads_outdoor_air_design_flow_report(
    case_path: &Path,
    manifest: &ConformanceCase,
    oracle_root: &Path,
    output_root: &Path,
) -> Result<IdealLoadsDiagnosticReportSummary, String> {
    validate_outdoor_air_design_flow_manifest(manifest)?;

    let case_output_dir = output_root.join(&manifest.id);
    let oracle_output_dir = case_output_dir.join("oracle");
    let compare_dir = case_output_dir.join("compare");

    let baseline =
        generate_conformance_baseline_in_dir(case_path, manifest, oracle_root, &oracle_output_dir)?;
    let (series_count, compared_samples, tolerance_failures_count, tolerance_policy, status) = {
        let context = build_outdoor_air_design_flow_context(manifest, &baseline)?;
        write_outdoor_air_artifacts(&compare_dir, &context)?;

        let tolerance_failures_count = context
            .rows
            .iter()
            .filter(|row| row.status == SeriesComparisonStatus::Fail)
            .count();
        let status = outdoor_air_overall_status(&context);
        (
            context.rows.len(),
            context.sample_count,
            tolerance_failures_count,
            outdoor_air_tolerance_policy(&context),
            status,
        )
    };

    Ok(IdealLoadsDiagnosticReportSummary {
        baseline,
        report_dir: compare_dir.clone(),
        compare_report: compare_dir.join("compare-report.md"),
        compare_summary: compare_dir.join("compare-summary.json"),
        selected_outputs: compare_dir.join("selected_outputs.json"),
        rust_result_store: compare_dir.join("rust-result-store.json"),
        variable_deltas: compare_dir.join("variable-deltas.csv"),
        first_divergence: compare_dir.join("first-divergence.csv"),
        tolerance_failures: compare_dir.join("tolerance-failures.csv"),
        stage_summary: compare_dir.join("stage-summary.json"),
        series_count,
        compared_samples,
        tolerance_failures_count,
        tolerance_policy,
        status,
    })
}

fn validate_manifest(manifest: &ConformanceCase) -> Result<(), String> {
    if !matches!(
        manifest.comparison_class,
        ComparisonClass::DiagnosticOnly | ComparisonClass::Conformance
    ) {
        return Err(format!(
            "IdealLoads no-OA report requires diagnostic-only or conformance, got {}",
            comparison_class_label(manifest.comparison_class)
        ));
    }
    if manifest.comparison_class == ComparisonClass::DiagnosticOnly && manifest.conformance_claim {
        return Err(
            "diagnostic-only IdealLoads report must keep conformance_claim false".to_string(),
        );
    }
    if manifest.conformance_claim
        && !manifest
            .outputs
            .iter()
            .any(|output| output.level == Some(OutputLevel::Conformance))
    {
        return Err(
            "conformance IdealLoads report requires at least one conformance-level output"
                .to_string(),
        );
    }
    if manifest.outputs.is_empty() {
        return Err("IdealLoads no-OA report requires output requests".to_string());
    }
    for output in &manifest.outputs {
        if output.frequency != OutputFrequency::Detailed {
            return Err(format!(
                "IdealLoads no-OA report requires detailed outputs, got {} for {}",
                output_frequency_label(output.frequency),
                output.variable
            ));
        }
        if output.source != SourceArtifact::Eso {
            return Err(format!(
                "IdealLoads no-OA report requires ESO outputs, got {} for {}",
                source_artifact_label(output.source),
                output.variable
            ));
        }
    }
    for meter in &manifest.meters {
        if meter.frequency != OutputFrequency::Hourly {
            return Err(format!(
                "IdealLoads no-OA report requires hourly meter outputs, got {} for {}",
                output_frequency_label(meter.frequency),
                meter.name
            ));
        }
        if meter.source != SourceArtifact::Mtr {
            return Err(format!(
                "IdealLoads no-OA report requires MTR meter outputs, got {} for {}",
                source_artifact_label(meter.source),
                meter.name
            ));
        }
        if meter.level != OutputLevel::Diagnostic {
            return Err(format!(
                "IdealLoads no-OA report currently supports diagnostic-level meter outputs: {}",
                meter.name
            ));
        }
    }
    Ok(())
}

fn validate_outdoor_air_design_flow_manifest(manifest: &ConformanceCase) -> Result<(), String> {
    let conformance_method = outdoor_air_conformance_method_for_manifest(manifest);
    let outdoor_air_conformance = conformance_method.is_some();
    if outdoor_air_conformance {
        if manifest.comparison_class != ComparisonClass::Conformance {
            return Err(format!(
                "IdealLoads outdoor-air conformance requires comparison_class=conformance, got {}",
                comparison_class_label(manifest.comparison_class)
            ));
        }
        for variable in OUTDOOR_AIR_CONFORMANCE_VARIABLES {
            if !manifest.outputs.iter().any(|output| {
                output.variable == *variable && output.level == Some(OutputLevel::Conformance)
            }) {
                return Err(format!(
                    "IdealLoads outdoor-air conformance is missing conformance row for {variable}"
                ));
            }
        }
    } else if manifest.comparison_class != ComparisonClass::DiagnosticOnly {
        return Err(format!(
            "IdealLoads outdoor-air design-flow report requires diagnostic-only unless it is an approved outdoor-air conformance candidate, got {}",
            comparison_class_label(manifest.comparison_class)
        ));
    }
    if !outdoor_air_conformance && manifest.conformance_claim {
        return Err(
            "IdealLoads outdoor-air design-flow diagnostic must keep conformance_claim false"
                .to_string(),
        );
    }
    if manifest.outputs.is_empty() {
        return Err("IdealLoads outdoor-air design-flow report requires outputs".to_string());
    }
    for output in &manifest.outputs {
        if output.frequency != OutputFrequency::Detailed {
            return Err(format!(
                "IdealLoads outdoor-air design-flow report requires detailed outputs, got {} for {}",
                output_frequency_label(output.frequency),
                output.variable
            ));
        }
        if output.source != SourceArtifact::Eso {
            return Err(format!(
                "IdealLoads outdoor-air design-flow report requires ESO outputs, got {} for {}",
                source_artifact_label(output.source),
                output.variable
            ));
        }
        if outdoor_air_conformance {
            let expected_level = if outdoor_air_conformance_variable(output.variable.as_str()) {
                OutputLevel::Conformance
            } else {
                OutputLevel::Diagnostic
            };
            if output.level != Some(expected_level) {
                return Err(format!(
                    "IdealLoads outdoor-air conformance expects {} level for {}",
                    output_level_label(expected_level),
                    output.variable
                ));
            }
        } else if output.level != Some(OutputLevel::Diagnostic) {
            return Err(format!(
                "IdealLoads outdoor-air design-flow outputs must be diagnostic-level: {}",
                output.variable
            ));
        }
        if !matches!(
            output.variable.as_str(),
            ZONE_IDEAL_LOADS_OUTDOOR_AIR_MASS_FLOW_RATE
                | ZONE_IDEAL_LOADS_OUTDOOR_AIR_STANDARD_DENSITY_VOLUME_FLOW_RATE
                | ZONE_IDEAL_LOADS_OUTDOOR_AIR_SENSIBLE_HEATING_RATE
                | ZONE_IDEAL_LOADS_OUTDOOR_AIR_SENSIBLE_COOLING_RATE
                | ZONE_IDEAL_LOADS_OUTDOOR_AIR_LATENT_HEATING_RATE
                | ZONE_IDEAL_LOADS_OUTDOOR_AIR_LATENT_COOLING_RATE
                | ZONE_IDEAL_LOADS_OUTDOOR_AIR_TOTAL_HEATING_RATE
                | ZONE_IDEAL_LOADS_OUTDOOR_AIR_TOTAL_COOLING_RATE
                | ZONE_IDEAL_LOADS_SUPPLY_AIR_MASS_FLOW_RATE
                | ZONE_IDEAL_LOADS_SUPPLY_AIR_STANDARD_DENSITY_VOLUME_FLOW_RATE
                | ZONE_IDEAL_LOADS_SUPPLY_AIR_TEMPERATURE
                | ZONE_IDEAL_LOADS_SUPPLY_AIR_HUMIDITY_RATIO
                | ZONE_IDEAL_LOADS_MIXED_AIR_TEMPERATURE
                | ZONE_IDEAL_LOADS_MIXED_AIR_HUMIDITY_RATIO
                | ZONE_IDEAL_LOADS_HEAT_RECOVERY_SENSIBLE_HEATING_RATE
                | ZONE_IDEAL_LOADS_HEAT_RECOVERY_LATENT_HEATING_RATE
                | ZONE_IDEAL_LOADS_HEAT_RECOVERY_TOTAL_HEATING_RATE
                | ZONE_IDEAL_LOADS_HEAT_RECOVERY_SENSIBLE_COOLING_RATE
                | ZONE_IDEAL_LOADS_HEAT_RECOVERY_LATENT_COOLING_RATE
                | ZONE_IDEAL_LOADS_HEAT_RECOVERY_TOTAL_COOLING_RATE
                | ZONE_IDEAL_LOADS_ECONOMIZER_ACTIVE_TIME
                | ZONE_IDEAL_LOADS_HEAT_RECOVERY_ACTIVE_TIME
        ) {
            return Err(format!(
                "IdealLoads outdoor-air design-flow report cannot produce Rust series for {}",
                output.variable
            ));
        }
    }
    Ok(())
}

const OUTDOOR_AIR_CONFORMANCE_VARIABLES: &[&str] = &[
    ZONE_IDEAL_LOADS_OUTDOOR_AIR_MASS_FLOW_RATE,
    ZONE_IDEAL_LOADS_OUTDOOR_AIR_STANDARD_DENSITY_VOLUME_FLOW_RATE,
    ZONE_IDEAL_LOADS_OUTDOOR_AIR_SENSIBLE_HEATING_RATE,
    ZONE_IDEAL_LOADS_OUTDOOR_AIR_SENSIBLE_COOLING_RATE,
    ZONE_IDEAL_LOADS_OUTDOOR_AIR_LATENT_HEATING_RATE,
    ZONE_IDEAL_LOADS_OUTDOOR_AIR_LATENT_COOLING_RATE,
    ZONE_IDEAL_LOADS_OUTDOOR_AIR_TOTAL_HEATING_RATE,
    ZONE_IDEAL_LOADS_OUTDOOR_AIR_TOTAL_COOLING_RATE,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_MASS_FLOW_RATE,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_STANDARD_DENSITY_VOLUME_FLOW_RATE,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_TEMPERATURE,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_HUMIDITY_RATIO,
    ZONE_IDEAL_LOADS_MIXED_AIR_TEMPERATURE,
    ZONE_IDEAL_LOADS_MIXED_AIR_HUMIDITY_RATIO,
];

fn manifest_allows_outdoor_air_flow_zone_conformance_manifest(manifest: &ConformanceCase) -> bool {
    manifest.id == IDEAL_LOADS_OUTDOOR_AIR_FLOW_ZONE_CONFORMANCE_CASE_ID
        && manifest.comparison_class == ComparisonClass::Conformance
        && manifest.conformance_claim
}

fn manifest_allows_outdoor_air_flow_person_conformance_manifest(
    manifest: &ConformanceCase,
) -> bool {
    manifest.id == IDEAL_LOADS_OUTDOOR_AIR_FLOW_PERSON_CONFORMANCE_CASE_ID
        && manifest.comparison_class == ComparisonClass::Conformance
        && manifest.conformance_claim
}

fn manifest_allows_outdoor_air_flow_area_conformance_manifest(manifest: &ConformanceCase) -> bool {
    manifest.id == IDEAL_LOADS_OUTDOOR_AIR_FLOW_AREA_CONFORMANCE_CASE_ID
        && manifest.comparison_class == ComparisonClass::Conformance
        && manifest.conformance_claim
}

fn manifest_allows_outdoor_air_air_changes_conformance_manifest(
    manifest: &ConformanceCase,
) -> bool {
    manifest.id == IDEAL_LOADS_OUTDOOR_AIR_AIR_CHANGES_CONFORMANCE_CASE_ID
        && manifest.comparison_class == ComparisonClass::Conformance
        && manifest.conformance_claim
}

fn manifest_allows_outdoor_air_sum_conformance_manifest(manifest: &ConformanceCase) -> bool {
    manifest.id == IDEAL_LOADS_OUTDOOR_AIR_SUM_CONFORMANCE_CASE_ID
        && manifest.comparison_class == ComparisonClass::Conformance
        && manifest.conformance_claim
}

fn outdoor_air_conformance_method_for_manifest(
    manifest: &ConformanceCase,
) -> Option<DesignSpecificationOutdoorAirMethod> {
    if manifest_allows_outdoor_air_flow_zone_conformance_manifest(manifest) {
        Some(DesignSpecificationOutdoorAirMethod::FlowPerZone)
    } else if manifest_allows_outdoor_air_flow_person_conformance_manifest(manifest) {
        Some(DesignSpecificationOutdoorAirMethod::FlowPerPerson)
    } else if manifest_allows_outdoor_air_flow_area_conformance_manifest(manifest) {
        Some(DesignSpecificationOutdoorAirMethod::FlowPerArea)
    } else if manifest_allows_outdoor_air_air_changes_conformance_manifest(manifest) {
        Some(DesignSpecificationOutdoorAirMethod::AirChangesPerHour)
    } else if manifest_allows_outdoor_air_sum_conformance_manifest(manifest) {
        Some(DesignSpecificationOutdoorAirMethod::Sum)
    } else {
        None
    }
}

fn outdoor_air_conformance_variable(variable: &str) -> bool {
    OUTDOOR_AIR_CONFORMANCE_VARIABLES.contains(&variable)
}

fn build_outdoor_air_design_flow_context<'a>(
    manifest: &'a ConformanceCase,
    baseline: &'a BaselineSummary,
) -> Result<IdealLoadsOutdoorAirDiagnosticContext<'a>, String> {
    let raw_model = load_epjson_file(&baseline.epjson)
        .map_err(|error| format!("failed to load baseline epJSON: {error}"))?;
    let compile_result = compile_raw_model(&raw_model);
    let typed = compile_result.model.ok_or_else(|| {
        compile_result
            .report
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.message.as_str())
            .collect::<Vec<_>>()
            .join("; ")
    })?;
    let model = SimulationModel::from_typed(typed);
    if model.typed.zones.len() != 1 {
        return Err(format!(
            "IdealLoads outdoor-air design-flow report requires one zone, got {}",
            model.typed.zones.len()
        ));
    }
    if model.typed.ideal_loads_air_systems.len() != 1 {
        return Err(format!(
            "IdealLoads outdoor-air design-flow report requires one IdealLoads system, got {}",
            model.typed.ideal_loads_air_systems.len()
        ));
    }

    let edge = model
        .graph
        .zone_ideal_loads
        .first()
        .ok_or_else(|| "missing zone to IdealLoads graph edge".to_string())?;
    let zone = model
        .typed
        .zones
        .iter()
        .find(|zone| zone.id == edge.zone)
        .ok_or_else(|| "missing controlled zone for IdealLoads edge".to_string())?;
    let zone_air_node_edge = model
        .graph
        .zone_air_nodes
        .iter()
        .find(|candidate| candidate.zone == zone.id)
        .ok_or_else(|| "missing zone air-node edge".to_string())?;
    let zone_air_node = model
        .typed
        .nodes
        .iter()
        .find(|node| node.id == zone_air_node_edge.node)
        .ok_or_else(|| "missing zone air node".to_string())?;
    let system = model
        .typed
        .ideal_loads_air_systems
        .iter()
        .find(|system| system.id == edge.ideal_loads_air_system)
        .ok_or_else(|| "missing IdealLoads system for graph edge".to_string())?;
    let outdoor_air_edge = model
        .graph
        .ideal_loads_outdoor_air_specs
        .iter()
        .find(|candidate| candidate.ideal_loads_air_system == system.id)
        .ok_or_else(|| "missing IdealLoads outdoor-air design specification edge".to_string())?;
    let outdoor_air_specification = model
        .typed
        .design_specification_outdoor_air
        .iter()
        .find(|specification| specification.id == outdoor_air_edge.design_specification_outdoor_air)
        .ok_or_else(|| "missing IdealLoads outdoor-air design specification".to_string())?;
    let outdoor_air_node_name = system
        .outdoor_air_inlet_node_name
        .as_ref()
        .ok_or_else(|| "IdealLoads outdoor-air diagnostic requires an OA inlet node".to_string())?;
    if outdoor_air_specification.outdoor_air_schedule.is_some() {
        return Err(
            "IdealLoads outdoor-air design-flow diagnostic currently requires a blank OA schedule"
                .to_string(),
        );
    }

    validate_outdoor_air_design_flow_boundary(system, outdoor_air_specification.method)?;
    if let Some(conformance_method) = outdoor_air_conformance_method_for_manifest(manifest) {
        validate_outdoor_air_conformance_boundary(
            system,
            outdoor_air_specification.method,
            conformance_method,
        )?;
    }

    let site =
        model.typed.site.as_ref().ok_or_else(|| {
            "IdealLoads outdoor-air diagnostics require Site:Location".to_string()
        })?;
    let limit_context = IdealLoadsSensibleLimitContext::from_site_elevation_m(site.elevation_m)
        .ok_or_else(|| {
            format!(
                "failed to derive EnergyPlus StdRhoAir from site elevation {}",
                site.elevation_m
            )
        })?;
    let standard_air_density_kg_per_m3 = limit_context.standard_air_density_kg_per_m3;
    let outdoor_air_context = ideal_loads_outdoor_air_context(&model.typed, zone);
    let design_flow_components = design_outdoor_air_volume_flow_components_m3_per_s(
        outdoor_air_specification,
        outdoor_air_context,
    )
    .ok_or_else(|| {
        "failed to calculate IdealLoads outdoor-air design-flow components".to_string()
    })?;
    let outdoor_air_mass_flow_rate_kg_per_s = calc_scheduled_outdoor_air_mass_flow_rate_kg_per_s(
        outdoor_air_specification,
        outdoor_air_context,
        None,
        standard_air_density_kg_per_m3,
    )
    .ok_or_else(|| "failed to calculate IdealLoads outdoor-air mass flow".to_string())?;
    let design_volume_flow_rate_m3_per_s =
        outdoor_air_mass_flow_rate_kg_per_s / standard_air_density_kg_per_m3;

    let mut expected_series = Vec::with_capacity(manifest.outputs.len());
    for output in &manifest.outputs {
        expected_series.push(load_series(&baseline.eso, &output.key, &output.variable)?);
    }
    let zone_air_humidity_ratio =
        load_series(&baseline.eso, &zone.name.0, ZONE_AIR_HUMIDITY_RATIO)?;
    let zone_node_temperature = load_series(
        &baseline.eso,
        &zone_air_node.name.0,
        SYSTEM_NODE_TEMPERATURE,
    )?;
    let zone_node_humidity_ratio = load_series(
        &baseline.eso,
        &zone_air_node.name.0,
        SYSTEM_NODE_HUMIDITY_RATIO,
    )?;
    let outdoor_air_node_temperature = load_series(
        &baseline.eso,
        &outdoor_air_node_name.0,
        SYSTEM_NODE_TEMPERATURE,
    )?;
    let outdoor_air_node_humidity_ratio = load_series(
        &baseline.eso,
        &outdoor_air_node_name.0,
        SYSTEM_NODE_HUMIDITY_RATIO,
    )?;
    let heating_demand = load_series(
        &baseline.eso,
        &zone.name.0,
        ZONE_SYSTEM_PREDICTED_HEATING_LOAD,
    )?;
    let cooling_demand = load_series(
        &baseline.eso,
        &zone.name.0,
        ZONE_SYSTEM_PREDICTED_COOLING_LOAD,
    )?;
    let sample_count = expected_series
        .iter()
        .map(|series| series.samples.len())
        .chain([
            zone_air_humidity_ratio.samples.len(),
            zone_node_temperature.samples.len(),
            zone_node_humidity_ratio.samples.len(),
            outdoor_air_node_temperature.samples.len(),
            outdoor_air_node_humidity_ratio.samples.len(),
            heating_demand.samples.len(),
            cooling_demand.samples.len(),
        ])
        .min()
        .unwrap_or(0);
    if sample_count == 0 {
        return Err("IdealLoads outdoor-air diagnostic has no samples".to_string());
    }

    let zone_timestep_hours = ideal_loads_energy_report_interval_seconds(&model) / 3600.0;
    let sample_timestep_hours = expected_series
        .first()
        .map(|series| {
            series
                .samples
                .iter()
                .take(sample_count)
                .map(|sample| {
                    ideal_loads_outdoor_air_sample_timestep_hours(
                        sample.timestamp.as_deref(),
                        zone_timestep_hours,
                    )
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|| vec![zone_timestep_hours; sample_count]);
    let barometric_pressure_trace = ideal_loads_barometric_pressure_samples(
        &model,
        baseline.weather.as_deref(),
        expected_series
            .first()
            .map(|series| series.samples.as_slice())
            .unwrap_or(&[]),
        sample_count,
        limit_context,
    )?;

    let mut sensible_results = Vec::with_capacity(sample_count);
    for index in 0..sample_count {
        let calc_zone_state_index = index.saturating_sub(1);
        let zone_state = IdealLoadsOutdoorAirNodeState {
            air_temperature_c: zone_node_temperature.samples[calc_zone_state_index].value,
            air_humidity_ratio: zone_air_humidity_ratio.samples[index].value,
        };
        let recirculation_state = IdealLoadsOutdoorAirNodeState {
            air_temperature_c: zone_node_temperature.samples[calc_zone_state_index].value,
            air_humidity_ratio: zone_node_humidity_ratio.samples[calc_zone_state_index].value,
        };
        let outdoor_air_state = IdealLoadsOutdoorAirNodeState {
            air_temperature_c: outdoor_air_node_temperature.samples[index].value,
            air_humidity_ratio: outdoor_air_node_humidity_ratio.samples[index].value,
        };
        let demand = ZoneSysEnergyDemand::sensible_only(
            zone.id,
            heating_demand.samples[index].value,
            cooling_demand.samples[index].value,
        );
        sensible_results.push(calc_outdoor_air_sensible_report_rates_compat(
            system,
            zone_state,
            recirculation_state,
            outdoor_air_state,
            demand,
            outdoor_air_mass_flow_rate_kg_per_s,
            sample_timestep_hours[index],
            barometric_pressure_trace[index],
            true,
        ));
    }

    let mut rows = Vec::new();
    let mut result_store = ResultStore::new();
    for (output, expected) in manifest.outputs.iter().zip(expected_series.iter()) {
        let (rust_source, units, observed_values) = outdoor_air_observed_values(
            output,
            system.outdoor_air_economizer_type,
            system.heat_recovery_type,
            standard_air_density_kg_per_m3,
            &sensible_results,
            expected.samples.len(),
        )?;
        let timestamps = expected
            .samples
            .iter()
            .map(|sample| sample.timestamp.clone())
            .collect::<Vec<_>>();
        let observed_samples = samples_with_timestamps(&observed_values, &timestamps);
        let tolerance = tolerance_for_output(manifest, output)?;
        let max_rmse_tolerance = max_rmse_tolerance_for_output(manifest, output)?;
        let comparison = compare_series_samples_v2(&expected.samples, &observed_samples, tolerance);
        let mean_abs_delta = mean_abs_delta(&expected.samples, &observed_samples);
        let status = if comparison.status == SeriesComparisonStatus::Pass
            && max_rmse_tolerance.is_none_or(|max_rmse| comparison.rmse_delta <= max_rmse)
        {
            SeriesComparisonStatus::Pass
        } else {
            SeriesComparisonStatus::Fail
        };

        result_store.add_series(OutputSeries {
            handle: OutputHandle(result_store.series.len() as u32),
            key: output.key.clone(),
            variable_name: output.variable.clone(),
            units: units.to_string(),
            values: observed_values,
        });
        rows.push(IdealLoadsDiagnosticRow {
            key: output.key.clone(),
            variable: output.variable.clone(),
            frequency: output.frequency,
            variable_class: output.class,
            source: output.source,
            domain: output.domain,
            level: output.level,
            units: units.to_string(),
            oracle_units: expected.units.clone(),
            rust_source,
            tolerance,
            max_rmse_tolerance,
            expected_samples: comparison.expected_samples,
            observed_samples: comparison.observed_samples,
            compared_samples: comparison.compared_samples,
            max_abs_delta: comparison.max_abs_delta,
            mean_abs_delta,
            rmse_delta: comparison.rmse_delta,
            max_rel_delta: comparison.max_rel_delta,
            alignment: comparison.alignment,
            first_divergence: comparison.first_divergence,
            status,
        });
    }

    Ok(IdealLoadsOutdoorAirDiagnosticContext {
        manifest,
        baseline,
        branch: "outdoor-air-design-flow",
        zone_name: zone.name.0.clone(),
        system_name: system.name.0.clone(),
        outdoor_air_spec_name: outdoor_air_specification.name.0.clone(),
        outdoor_air_method: outdoor_air_specification.method,
        outdoor_air_node_name: outdoor_air_node_name.0.clone(),
        outdoor_air_economizer_type: system.outdoor_air_economizer_type,
        heat_recovery_type: system.heat_recovery_type,
        standard_air_density_kg_per_m3,
        design_people_count: outdoor_air_context.design_people_count,
        zone_floor_area_m2: outdoor_air_context.zone_floor_area_m2,
        zone_volume_m3: outdoor_air_context.zone_volume_m3,
        flow_per_person_m3_per_s: design_flow_components.flow_per_person_m3_per_s,
        flow_per_area_m3_per_s: design_flow_components.flow_per_area_m3_per_s,
        flow_per_zone_m3_per_s: design_flow_components.flow_per_zone_m3_per_s,
        air_changes_m3_per_s: design_flow_components.air_changes_m3_per_s,
        design_volume_flow_rate_m3_per_s,
        outdoor_air_mass_flow_rate_kg_per_s,
        sample_count,
        rows,
        result_store,
    })
}

fn validate_outdoor_air_design_flow_boundary(
    system: &IdealLoadsAirSystem,
    method: DesignSpecificationOutdoorAirMethod,
) -> Result<(), String> {
    if !matches!(
        method,
        DesignSpecificationOutdoorAirMethod::FlowPerPerson
            | DesignSpecificationOutdoorAirMethod::FlowPerZone
            | DesignSpecificationOutdoorAirMethod::FlowPerArea
            | DesignSpecificationOutdoorAirMethod::AirChangesPerHour
            | DesignSpecificationOutdoorAirMethod::Sum
            | DesignSpecificationOutdoorAirMethod::Maximum
    ) {
        return Err(
            "IdealLoads outdoor-air design-flow diagnostic currently requires Flow/Person, Flow/Zone, Flow/Area, AirChanges/Hour, Sum, or Maximum"
                .to_string(),
        );
    }
    if system.demand_controlled_ventilation_type != DemandControlledVentilationType::None {
        return Err("IdealLoads outdoor-air design-flow diagnostic excludes DCV".to_string());
    }
    if !matches!(
        system.outdoor_air_economizer_type,
        OutdoorAirEconomizerType::NoEconomizer
            | OutdoorAirEconomizerType::DifferentialDryBulb
            | OutdoorAirEconomizerType::DifferentialEnthalpy
    ) {
        return Err(
            "IdealLoads outdoor-air design-flow diagnostic currently supports NoEconomizer, DifferentialDryBulb, or DifferentialEnthalpy economizer".to_string(),
        );
    }
    if !matches!(
        system.heat_recovery_type,
        HeatRecoveryType::None | HeatRecoveryType::Sensible | HeatRecoveryType::Enthalpy
    ) {
        return Err(
            "IdealLoads outdoor-air design-flow diagnostic currently supports no heat recovery, Sensible heat recovery, or Enthalpy heat recovery".to_string(),
        );
    }
    if system.heat_recovery_type != HeatRecoveryType::None
        && system.outdoor_air_economizer_type != OutdoorAirEconomizerType::NoEconomizer
    {
        return Err(
            "IdealLoads outdoor-air heat-recovery diagnostic currently requires NoEconomizer"
                .to_string(),
        );
    }
    if system.heating_limit != IdealLoadsLimit::NoLimit
        || system.cooling_limit != IdealLoadsLimit::NoLimit
    {
        return Err(
            "IdealLoads outdoor-air design-flow diagnostic excludes finite flow/capacity limits"
                .to_string(),
        );
    }
    Ok(())
}

fn validate_outdoor_air_conformance_boundary(
    system: &IdealLoadsAirSystem,
    method: DesignSpecificationOutdoorAirMethod,
    expected_method: DesignSpecificationOutdoorAirMethod,
) -> Result<(), String> {
    if method != expected_method {
        return Err(format!(
            "IdealLoads outdoor-air {} conformance candidate requires {}",
            outdoor_air_method_label(expected_method),
            outdoor_air_method_label(expected_method)
        ));
    }
    if system.outdoor_air_economizer_type != OutdoorAirEconomizerType::NoEconomizer {
        return Err(
            "IdealLoads outdoor-air conformance candidate excludes active economizer".to_string(),
        );
    }
    if system.heat_recovery_type != HeatRecoveryType::None {
        return Err(
            "IdealLoads outdoor-air conformance candidate excludes heat recovery".to_string(),
        );
    }
    if system.dehumidification_control_type
        != DehumidificationControlType::ConstantSensibleHeatRatio
        || system.humidification_control_type != HumidificationControlType::None
    {
        return Err(
            "IdealLoads outdoor-air conformance candidate requires default ConstantSensibleHeatRatio dehumidification and no humidification control"
                .to_string(),
        );
    }
    Ok(())
}

fn outdoor_air_method_label(method: DesignSpecificationOutdoorAirMethod) -> &'static str {
    match method {
        DesignSpecificationOutdoorAirMethod::FlowPerPerson => "Flow/Person",
        DesignSpecificationOutdoorAirMethod::FlowPerArea => "Flow/Area",
        DesignSpecificationOutdoorAirMethod::FlowPerZone => "Flow/Zone",
        DesignSpecificationOutdoorAirMethod::AirChangesPerHour => "AirChanges/Hour",
        DesignSpecificationOutdoorAirMethod::Sum => "Sum",
        DesignSpecificationOutdoorAirMethod::Maximum => "Maximum",
        DesignSpecificationOutdoorAirMethod::IndoorAirQualityProcedure => {
            "IndoorAirQualityProcedure"
        }
        DesignSpecificationOutdoorAirMethod::ProportionalControlBasedOnDesignOccupancy => {
            "ProportionalControlBasedOnDesignOccupancy"
        }
        DesignSpecificationOutdoorAirMethod::ProportionalControlBasedOnOccupancySchedule => {
            "ProportionalControlBasedOnOccupancySchedule"
        }
    }
}

fn outdoor_air_economizer_label(economizer: OutdoorAirEconomizerType) -> &'static str {
    match economizer {
        OutdoorAirEconomizerType::NoEconomizer => "NoEconomizer",
        OutdoorAirEconomizerType::DifferentialDryBulb => "DifferentialDryBulb",
        OutdoorAirEconomizerType::DifferentialEnthalpy => "DifferentialEnthalpy",
    }
}

fn heat_recovery_label(heat_recovery: HeatRecoveryType) -> &'static str {
    match heat_recovery {
        HeatRecoveryType::None => "None",
        HeatRecoveryType::Sensible => "Sensible",
        HeatRecoveryType::Enthalpy => "Enthalpy",
    }
}

fn outdoor_air_claim_boundary(context: &IdealLoadsOutdoorAirDiagnosticContext<'_>) -> &'static str {
    if manifest_allows_outdoor_air_flow_zone_conformance_manifest(context.manifest) {
        return "conformance IdealLoads outdoor-air Flow/Zone branch for declared variables only";
    }
    if manifest_allows_outdoor_air_flow_person_conformance_manifest(context.manifest) {
        return "conformance IdealLoads outdoor-air Flow/Person branch for declared variables only";
    }
    if manifest_allows_outdoor_air_flow_area_conformance_manifest(context.manifest) {
        return "conformance IdealLoads outdoor-air Flow/Area branch for declared variables only";
    }
    if manifest_allows_outdoor_air_air_changes_conformance_manifest(context.manifest) {
        return "conformance IdealLoads outdoor-air AirChanges/Hour branch for declared variables only";
    }
    if manifest_allows_outdoor_air_sum_conformance_manifest(context.manifest) {
        return "conformance IdealLoads outdoor-air Sum branch for declared variables only";
    }
    if context.heat_recovery_type == HeatRecoveryType::Sensible {
        return "diagnostic-only IdealLoads outdoor-air Flow/Zone mass, standard-density volume, outdoor-air report rates, supply-air state, mixed-air state, and Sensible heat recovery active-time/rate parity; DCV, economizer, Enthalpy heat recovery, humidity controls, saturation-limit branches, and broad OA conformance remain outside the claim";
    }
    if context.heat_recovery_type == HeatRecoveryType::Enthalpy {
        return "diagnostic-only IdealLoads outdoor-air Flow/Zone mass, standard-density volume, outdoor-air report rates, supply-air state, mixed-air state, and Enthalpy heat recovery active-time/rate parity; DCV, economizer, humidity controls, saturation-limit branches, and broad OA conformance remain outside the claim";
    }
    match context.outdoor_air_economizer_type {
        OutdoorAirEconomizerType::DifferentialDryBulb => {
            "diagnostic-only IdealLoads outdoor-air Flow/Person, Flow/Zone, Flow/Area, AirChanges/Hour, Sum, and Maximum mass, standard-density volume, outdoor-air report rates, supply-air state, mixed-air state, and DifferentialDryBulb economizer active-time/flow parity; DCV, DifferentialEnthalpy economizer, heat recovery, humidity controls, saturation-limit branches, and broad OA conformance remain outside the claim"
        }
        OutdoorAirEconomizerType::DifferentialEnthalpy => {
            "diagnostic-only IdealLoads outdoor-air Flow/Person, Flow/Zone, Flow/Area, AirChanges/Hour, Sum, and Maximum mass, standard-density volume, outdoor-air report rates, supply-air state, mixed-air state, and DifferentialEnthalpy economizer active-time/flow parity; DCV, heat recovery, humidity controls, saturation-limit branches, and broad OA conformance remain outside the claim"
        }
        OutdoorAirEconomizerType::NoEconomizer => {
            "diagnostic-only IdealLoads outdoor-air Flow/Person, Flow/Zone, Flow/Area, AirChanges/Hour, Sum, and Maximum mass, standard-density volume, outdoor-air report rates, supply-air state, mixed-air state, and inactive economizer/heat recovery"
        }
    }
}

fn outdoor_air_source_description(context: &IdealLoadsOutdoorAirDiagnosticContext<'_>) -> String {
    let economizer_source = match context.outdoor_air_economizer_type {
        OutdoorAirEconomizerType::DifferentialDryBulb => {
            " plus EnergyPlus DifferentialDryBulb economizer OA flow reset when outdoor dry-bulb is below recirculation dry-bulb"
        }
        OutdoorAirEconomizerType::DifferentialEnthalpy => {
            " plus EnergyPlus DifferentialEnthalpy economizer OA flow reset when outdoor enthalpy is below recirculation enthalpy"
        }
        OutdoorAirEconomizerType::NoEconomizer => "",
    };
    let heat_recovery_source = match context.heat_recovery_type {
        HeatRecoveryType::Sensible => {
            " plus EnergyPlus Sensible heat recovery OA tempering when recirculation air can beneficially warm or cool outdoor air"
        }
        HeatRecoveryType::Enthalpy => {
            " plus EnergyPlus Enthalpy heat recovery OA tempering when recirculation enthalpy can beneficially warm or cool outdoor air"
        }
        HeatRecoveryType::None => "",
    };
    format!(
        "DesignSpecification:OutdoorAir {} with blank OA schedule, EnergyPlus StdRhoAir from Site:Location, and source-order zone/OA/mixed-air state proof rows{}{}",
        outdoor_air_method_label(context.outdoor_air_method),
        economizer_source,
        heat_recovery_source
    )
}

fn ideal_loads_outdoor_air_context(model: &TypedModel, zone: &Zone) -> IdealLoadsOutdoorAirContext {
    IdealLoadsOutdoorAirContext {
        design_people_count: ideal_loads_zone_design_people_count(model, zone),
        zone_floor_area_m2: ideal_loads_zone_floor_area_m2(model, zone),
        zone_volume_m3: ideal_loads_zone_volume_m3(model, zone).unwrap_or(0.0),
    }
}

fn ideal_loads_zone_design_people_count(model: &TypedModel, zone: &Zone) -> f64 {
    let zone_floor_area_m2 = ideal_loads_zone_floor_area_m2(model, zone);
    model
        .people
        .iter()
        .filter(|people| people.zone == zone.id)
        .map(|people| match people.number_of_people_calculation_method {
            PeopleNumberCalculationMethod::People => people.number_of_people,
            PeopleNumberCalculationMethod::PeoplePerArea => {
                people.people_per_floor_area * zone_floor_area_m2
            }
            PeopleNumberCalculationMethod::AreaPerPerson => {
                if people.floor_area_per_person > 0.0 {
                    zone_floor_area_m2 / people.floor_area_per_person
                } else {
                    0.0
                }
            }
        })
        .filter(|value| value.is_finite() && *value > 0.0)
        .sum()
}

fn ideal_loads_zone_floor_area_m2(model: &TypedModel, zone: &Zone) -> f64 {
    model
        .surfaces
        .iter()
        .filter(|surface| surface.zone == zone.id && surface.surface_type == SurfaceType::Floor)
        .map(|surface| surface_area_m2(&surface.vertices))
        .sum()
}

fn ideal_loads_zone_volume_m3(model: &TypedModel, zone: &Zone) -> Option<f64> {
    if let AutoOrNumber::Value(volume_m3) = zone.volume
        && volume_m3 > 0.0
    {
        return Some(volume_m3);
    }
    if let Some(volume_m3) = ideal_loads_bounding_box_volume_m3(model, zone)
        && volume_m3 > 0.0
    {
        return Some(volume_m3);
    }
    let AutoOrNumber::Value(ceiling_height_m) = zone.ceiling_height else {
        return None;
    };
    if ceiling_height_m <= 0.0 {
        return None;
    }
    let floor_area_m2 = ideal_loads_zone_floor_area_m2(model, zone);
    if floor_area_m2 > 0.0 {
        Some(floor_area_m2 * ceiling_height_m)
    } else {
        None
    }
}

fn ideal_loads_bounding_box_volume_m3(model: &TypedModel, zone: &Zone) -> Option<f64> {
    let mut bounds: Option<(f64, f64, f64, f64, f64, f64)> = None;
    for surface in model
        .surfaces
        .iter()
        .filter(|surface| surface.zone == zone.id)
    {
        for vertex in &surface.vertices {
            let x = vertex.x_m + zone.origin.x_m;
            let y = vertex.y_m + zone.origin.y_m;
            let z = vertex.z_m + zone.origin.z_m;
            bounds = Some(match bounds {
                Some((min_x, max_x, min_y, max_y, min_z, max_z)) => (
                    min_x.min(x),
                    max_x.max(x),
                    min_y.min(y),
                    max_y.max(y),
                    min_z.min(z),
                    max_z.max(z),
                ),
                None => (x, x, y, y, z, z),
            });
        }
    }
    let (min_x, max_x, min_y, max_y, min_z, max_z) = bounds?;
    let volume_m3 = (max_x - min_x) * (max_y - min_y) * (max_z - min_z);
    if volume_m3 > 0.0 {
        Some(volume_m3)
    } else {
        None
    }
}

fn outdoor_air_observed_values(
    output: &OutputRequest,
    outdoor_air_economizer_type: OutdoorAirEconomizerType,
    heat_recovery_type: HeatRecoveryType,
    standard_air_density_kg_per_m3: f64,
    sensible_results: &[IdealLoadsOutdoorAirSensibleResult],
    expected_samples: usize,
) -> Result<(&'static str, &'static str, Vec<f64>), String> {
    let outdoor_air_flow_source = if sensible_results
        .iter()
        .any(|result| result.economizer_active_time_hr > 0.0)
    {
        outdoor_air_economizer_source(outdoor_air_economizer_type)
    } else {
        "rust-ideal-loads-outdoor-air-design-flow"
    };
    match output.variable.as_str() {
        ZONE_IDEAL_LOADS_OUTDOOR_AIR_MASS_FLOW_RATE => Ok((
            outdoor_air_flow_source,
            "kg/s",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.outdoor_air_mass_flow_rate_kg_per_s)
                .collect(),
        )),
        ZONE_IDEAL_LOADS_OUTDOOR_AIR_STANDARD_DENSITY_VOLUME_FLOW_RATE => Ok((
            outdoor_air_flow_source,
            "m3/s",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| {
                    result.outdoor_air_mass_flow_rate_kg_per_s / standard_air_density_kg_per_m3
                })
                .collect(),
        )),
        ZONE_IDEAL_LOADS_OUTDOOR_AIR_SENSIBLE_HEATING_RATE => Ok((
            "rust-ideal-loads-outdoor-air-sensible-report",
            "W",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.outdoor_air_sensible_heating_rate_w)
                .collect(),
        )),
        ZONE_IDEAL_LOADS_OUTDOOR_AIR_SENSIBLE_COOLING_RATE => Ok((
            "rust-ideal-loads-outdoor-air-sensible-report",
            "W",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.outdoor_air_sensible_cooling_rate_w)
                .collect(),
        )),
        ZONE_IDEAL_LOADS_OUTDOOR_AIR_LATENT_HEATING_RATE => Ok((
            "rust-ideal-loads-outdoor-air-latent-report",
            "W",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.outdoor_air_latent_heating_rate_w)
                .collect(),
        )),
        ZONE_IDEAL_LOADS_OUTDOOR_AIR_LATENT_COOLING_RATE => Ok((
            "rust-ideal-loads-outdoor-air-latent-report",
            "W",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.outdoor_air_latent_cooling_rate_w)
                .collect(),
        )),
        ZONE_IDEAL_LOADS_OUTDOOR_AIR_TOTAL_HEATING_RATE => Ok((
            "rust-ideal-loads-outdoor-air-total-report",
            "W",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.outdoor_air_total_heating_rate_w)
                .collect(),
        )),
        ZONE_IDEAL_LOADS_OUTDOOR_AIR_TOTAL_COOLING_RATE => Ok((
            "rust-ideal-loads-outdoor-air-total-report",
            "W",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.outdoor_air_total_cooling_rate_w)
                .collect(),
        )),
        ZONE_IDEAL_LOADS_SUPPLY_AIR_MASS_FLOW_RATE => Ok((
            "rust-ideal-loads-outdoor-air-supply-state",
            "kg/s",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.supply_mass_flow_rate_kg_per_s)
                .collect(),
        )),
        ZONE_IDEAL_LOADS_SUPPLY_AIR_STANDARD_DENSITY_VOLUME_FLOW_RATE => Ok((
            "rust-ideal-loads-outdoor-air-supply-state",
            "m3/s",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| {
                    result.supply_mass_flow_rate_kg_per_s / standard_air_density_kg_per_m3
                })
                .collect(),
        )),
        ZONE_IDEAL_LOADS_SUPPLY_AIR_TEMPERATURE => Ok((
            "rust-ideal-loads-outdoor-air-supply-state",
            "C",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.supply_air_temperature_c)
                .collect(),
        )),
        ZONE_IDEAL_LOADS_SUPPLY_AIR_HUMIDITY_RATIO => Ok((
            "rust-ideal-loads-outdoor-air-supply-state",
            "kgWater/kgDryAir",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.supply_air_humidity_ratio)
                .collect(),
        )),
        ZONE_IDEAL_LOADS_MIXED_AIR_TEMPERATURE => Ok((
            "rust-ideal-loads-outdoor-air-mixed-air",
            "C",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.mixed_air_temperature_c)
                .collect(),
        )),
        ZONE_IDEAL_LOADS_MIXED_AIR_HUMIDITY_RATIO => Ok((
            "rust-ideal-loads-outdoor-air-mixed-air",
            "kgWater/kgDryAir",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.mixed_air_humidity_ratio)
                .collect(),
        )),
        ZONE_IDEAL_LOADS_HEAT_RECOVERY_SENSIBLE_HEATING_RATE => Ok((
            outdoor_air_heat_recovery_source(heat_recovery_type),
            "W",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.heat_recovery_sensible_heating_rate_w)
                .collect(),
        )),
        ZONE_IDEAL_LOADS_HEAT_RECOVERY_LATENT_HEATING_RATE => Ok((
            outdoor_air_heat_recovery_source(heat_recovery_type),
            "W",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.heat_recovery_latent_heating_rate_w)
                .collect(),
        )),
        ZONE_IDEAL_LOADS_HEAT_RECOVERY_TOTAL_HEATING_RATE => Ok((
            outdoor_air_heat_recovery_source(heat_recovery_type),
            "W",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.heat_recovery_total_heating_rate_w)
                .collect(),
        )),
        ZONE_IDEAL_LOADS_HEAT_RECOVERY_SENSIBLE_COOLING_RATE => Ok((
            outdoor_air_heat_recovery_source(heat_recovery_type),
            "W",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.heat_recovery_sensible_cooling_rate_w)
                .collect(),
        )),
        ZONE_IDEAL_LOADS_HEAT_RECOVERY_LATENT_COOLING_RATE => Ok((
            outdoor_air_heat_recovery_source(heat_recovery_type),
            "W",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.heat_recovery_latent_cooling_rate_w)
                .collect(),
        )),
        ZONE_IDEAL_LOADS_HEAT_RECOVERY_TOTAL_COOLING_RATE => Ok((
            outdoor_air_heat_recovery_source(heat_recovery_type),
            "W",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.heat_recovery_total_cooling_rate_w)
                .collect(),
        )),
        ZONE_IDEAL_LOADS_ECONOMIZER_ACTIVE_TIME => Ok((
            if sensible_results
                .iter()
                .any(|result| result.economizer_active_time_hr > 0.0)
            {
                outdoor_air_economizer_source(outdoor_air_economizer_type)
            } else {
                "rust-ideal-loads-outdoor-air-inactive-economizer"
            },
            "hr",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.economizer_active_time_hr)
                .collect(),
        )),
        ZONE_IDEAL_LOADS_HEAT_RECOVERY_ACTIVE_TIME => Ok((
            outdoor_air_heat_recovery_source(heat_recovery_type),
            "hr",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.heat_recovery_active_time_hr)
                .collect(),
        )),
        _ => Err(format!(
            "IdealLoads outdoor-air design-flow report cannot produce Rust series for {} / {}",
            output.key, output.variable
        )),
    }
}

fn outdoor_air_economizer_source(
    outdoor_air_economizer_type: OutdoorAirEconomizerType,
) -> &'static str {
    match outdoor_air_economizer_type {
        OutdoorAirEconomizerType::DifferentialDryBulb => {
            "rust-ideal-loads-outdoor-air-differential-dry-bulb-economizer"
        }
        OutdoorAirEconomizerType::DifferentialEnthalpy => {
            "rust-ideal-loads-outdoor-air-differential-enthalpy-economizer"
        }
        OutdoorAirEconomizerType::NoEconomizer => "rust-ideal-loads-outdoor-air-design-flow",
    }
}

fn outdoor_air_heat_recovery_source(heat_recovery_type: HeatRecoveryType) -> &'static str {
    match heat_recovery_type {
        HeatRecoveryType::None => "rust-ideal-loads-outdoor-air-inactive-heat-recovery",
        HeatRecoveryType::Sensible => "rust-ideal-loads-outdoor-air-sensible-heat-recovery",
        HeatRecoveryType::Enthalpy => "rust-ideal-loads-outdoor-air-enthalpy-heat-recovery",
    }
}

fn build_context<'a>(
    manifest: &'a ConformanceCase,
    baseline: &'a BaselineSummary,
) -> Result<IdealLoadsDiagnosticContext<'a>, String> {
    let raw_model = load_epjson_file(&baseline.epjson)
        .map_err(|error| format!("failed to load baseline epJSON: {error}"))?;
    let compile_result = compile_raw_model(&raw_model);
    let typed = compile_result.model.ok_or_else(|| {
        compile_result
            .report
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.message.as_str())
            .collect::<Vec<_>>()
            .join("; ")
    })?;
    let model = SimulationModel::from_typed(typed);
    if model.typed.zones.len() != 1 {
        return Err(format!(
            "IdealLoads no-OA report requires one zone, got {}",
            model.typed.zones.len()
        ));
    }
    if model.typed.ideal_loads_air_systems.len() != 1 {
        return Err(format!(
            "IdealLoads no-OA report requires one IdealLoads system, got {}",
            model.typed.ideal_loads_air_systems.len()
        ));
    }

    let edge = model
        .graph
        .zone_ideal_loads
        .first()
        .ok_or_else(|| "missing zone to IdealLoads graph edge".to_string())?;
    let zone = model
        .typed
        .zones
        .iter()
        .find(|zone| zone.id == edge.zone)
        .ok_or_else(|| "missing controlled zone for IdealLoads edge".to_string())?;
    let system = model
        .typed
        .ideal_loads_air_systems
        .iter()
        .find(|system| system.id == edge.ideal_loads_air_system)
        .ok_or_else(|| "missing IdealLoads system for graph edge".to_string())?;
    let zone_equipment_dispatch = validate_ideal_loads_zone_equipment_dispatch(&model, system.id);
    if !zone_equipment_dispatch.is_dispatchable() {
        return Err(format!(
            "IdealLoads zone equipment dispatch prerequisites failed: {}",
            label_list_or_none(&zone_equipment_dispatch.issue_codes())
        ));
    }
    if manifest.conformance_claim && !zone_equipment_dispatch.is_conformance_candidate() {
        return Err(format!(
            "IdealLoads conformance candidate requires single-zone/single-equipment dispatch scope: {}",
            label_list_or_none(&zone_equipment_dispatch.warning_codes())
        ));
    }
    let supply_edge = model
        .graph
        .ideal_loads_supply_nodes
        .iter()
        .find(|candidate| candidate.ideal_loads_air_system == system.id)
        .ok_or_else(|| "missing IdealLoads supply-node edge".to_string())?;
    let supply_node = model
        .typed
        .nodes
        .iter()
        .find(|node| node.id == supply_edge.node)
        .ok_or_else(|| "missing IdealLoads supply node".to_string())?;
    let zone_air_node_edge = model
        .graph
        .zone_air_nodes
        .iter()
        .find(|candidate| candidate.zone == zone.id)
        .ok_or_else(|| "missing zone air-node edge".to_string())?;
    let zone_air_node = model
        .typed
        .nodes
        .iter()
        .find(|node| node.id == zone_air_node_edge.node)
        .ok_or_else(|| "missing zone air node".to_string())?;

    let mut boundary = if manifest_allows_finite_limit_conformance(manifest, system) {
        classify_no_oa_sensible_subset(system)
    } else if manifest.conformance_claim {
        classify_no_oa_no_limit_sensible_subset(system)
    } else {
        classify_no_oa_sensible_subset(system)
    };
    if manifest_allows_constant_supply_humidity_diagnostic(manifest, system)
        || manifest_allows_constant_supply_humidity_cooling_conformance(manifest, system)
        || manifest_allows_humidistat_dehumidification_diagnostic(manifest, system)
        || manifest_allows_humidistat_dehumidification_conformance(manifest, system)
    {
        boundary
            .unsupported_features
            .retain(|feature| *feature != IdealLoadsUnsupportedFeature::Dehumidification);
    }
    if manifest_allows_constant_supply_humidity_humidification_diagnostic(manifest, system)
        || manifest_allows_constant_supply_humidity_heating_conformance(manifest, system)
        || manifest_allows_humidistat_humidification_diagnostic(manifest, system)
        || manifest_allows_humidistat_humidification_conformance(manifest, system)
    {
        boundary
            .unsupported_features
            .retain(|feature| *feature != IdealLoadsUnsupportedFeature::Humidification);
    }
    if !boundary.is_supported() {
        return Err(format!(
            "IdealLoads system is outside no-OA sensible subset: {}",
            unsupported_features_label(&boundary.unsupported_features)
        ));
    }

    let recirculation_node_name = if uses_finite_limits(system)
        || manifest_requests_ideal_loads_recirculation_node(manifest, &model, zone.id, system)?
    {
        Some(ideal_loads_recirculation_node_name(
            &model, zone.id, system,
        )?)
    } else {
        None
    };
    let input_trace = load_input_trace(
        &baseline.eso,
        &zone.name.0,
        &zone_air_node.name.0,
        recirculation_node_name.as_deref(),
    )?;
    let system_timestep_seconds = ideal_loads_system_timestep_seconds(&model);
    let energy_report_interval_seconds = ideal_loads_energy_report_interval_seconds(&model);
    let fuel_efficiency = ideal_loads_fuel_efficiency_context(&model, system)?;
    let mtr = baseline.output_dir.join("eplusout.mtr");
    let (rows, meter_rows, result_store, mode_counts) = evaluate_rows(
        manifest,
        &model,
        &baseline.eso,
        &mtr,
        baseline.weather.as_deref(),
        &input_trace,
        &zone.name.0,
        &zone_air_node.name.0,
        recirculation_node_name.as_deref(),
        &system.name.0,
        &supply_node.name.0,
        energy_report_interval_seconds,
        fuel_efficiency,
    )?;

    let zone_name = zone.name.0.clone();
    let zone_air_node_name = zone_air_node.name.0.clone();
    let system_name = system.name.0.clone();
    let supply_node_name = supply_node.name.0.clone();
    let branch = ideal_loads_sensible_branch(system);
    let selected_purchased_air_branch = select_purchased_air_branch(system).label();
    let declared_ideal_loads_branch = declared_ideal_loads_branch(manifest, system);
    let inactive_branches = inactive_ideal_loads_branches(system);
    let constant_shr_conformance_claim = manifest_allows_constant_shr_conformance(manifest, system);
    let constant_supply_humidity_cooling_conformance_claim =
        manifest_allows_constant_supply_humidity_cooling_conformance(manifest, system);
    let constant_supply_humidity_heating_conformance_claim =
        manifest_allows_constant_supply_humidity_heating_conformance(manifest, system);
    let humidistat_dehumidification_conformance_claim =
        manifest_allows_humidistat_dehumidification_conformance(manifest, system);
    let humidistat_humidification_conformance_claim =
        manifest_allows_humidistat_humidification_conformance(manifest, system);

    Ok(IdealLoadsDiagnosticContext {
        manifest,
        baseline,
        branch,
        selected_purchased_air_branch,
        declared_ideal_loads_branch,
        inactive_branches,
        zone_equipment_dispatch,
        constant_shr_conformance_claim,
        constant_supply_humidity_cooling_conformance_claim,
        constant_supply_humidity_heating_conformance_claim,
        humidistat_dehumidification_conformance_claim,
        humidistat_humidification_conformance_claim,
        zone_name,
        zone_air_node_name,
        recirculation_node_name,
        system_name,
        supply_node_name,
        system_timestep_seconds,
        energy_report_interval_seconds,
        fuel_efficiency,
        rows,
        meter_rows,
        result_store,
        input_trace,
        mode_counts,
    })
}

fn load_input_trace(
    eso: &Path,
    zone_name: &str,
    zone_air_node_name: &str,
    recirculation_node_name: Option<&str>,
) -> Result<IdealLoadsInputTrace, String> {
    let zone_node_temperature = load_series(eso, zone_air_node_name, SYSTEM_NODE_TEMPERATURE)?;
    let zone_node_humidity_ratio =
        load_series(eso, zone_air_node_name, SYSTEM_NODE_HUMIDITY_RATIO)?;
    let (recirculation_node_temperature, recirculation_node_humidity_ratio) =
        match recirculation_node_name {
            Some(recirculation_node_name) => (
                load_series(eso, recirculation_node_name, SYSTEM_NODE_TEMPERATURE)?,
                load_series(eso, recirculation_node_name, SYSTEM_NODE_HUMIDITY_RATIO)?,
            ),
            None => (
                zone_node_temperature.clone(),
                zone_node_humidity_ratio.clone(),
            ),
        };
    let active_demand = load_series(eso, zone_name, ZONE_SYSTEM_PREDICTED_SETPOINT_LOAD)?;
    let heating_demand = load_series(eso, zone_name, ZONE_SYSTEM_PREDICTED_HEATING_LOAD)?;
    let cooling_demand = load_series(eso, zone_name, ZONE_SYSTEM_PREDICTED_COOLING_LOAD)?;
    let humidifying_moisture_demand = load_optional_series_or_zero(
        eso,
        zone_name,
        ZONE_SYSTEM_PREDICTED_HUMIDIFYING_MOISTURE_LOAD,
        &active_demand,
        "kgWater/s",
    )?;
    let dehumidifying_moisture_demand = load_optional_series_or_zero(
        eso,
        zone_name,
        ZONE_SYSTEM_PREDICTED_DEHUMIDIFYING_MOISTURE_LOAD,
        &active_demand,
        "kgWater/s",
    )?;
    let sample_count = [
        zone_node_temperature.samples.len(),
        zone_node_humidity_ratio.samples.len(),
        recirculation_node_temperature.samples.len(),
        recirculation_node_humidity_ratio.samples.len(),
        active_demand.samples.len(),
        heating_demand.samples.len(),
        cooling_demand.samples.len(),
        humidifying_moisture_demand.samples.len(),
        dehumidifying_moisture_demand.samples.len(),
    ]
    .into_iter()
    .min()
    .unwrap_or(0);
    if sample_count == 0 {
        return Err("IdealLoads diagnostic input trace has no samples".to_string());
    }

    Ok(IdealLoadsInputTrace {
        sample_count,
        zone_node_temperature,
        zone_node_humidity_ratio,
        recirculation_node_temperature,
        recirculation_node_humidity_ratio,
        active_demand,
        heating_demand,
        cooling_demand,
        humidifying_moisture_demand,
        dehumidifying_moisture_demand,
    })
}

fn ideal_loads_system_timestep_seconds(model: &SimulationModel) -> f64 {
    let zone_timesteps_per_hour = model.typed.timestep.number_of_timesteps_per_hour.max(1);
    3600.0 / f64::from(zone_timesteps_per_hour) / IDEAL_LOADS_NO_OA_ENERGY_SYSTEM_SUBSTEPS
}

fn ideal_loads_energy_report_interval_seconds(model: &SimulationModel) -> f64 {
    let zone_timesteps_per_hour = model.typed.timestep.number_of_timesteps_per_hour.max(1);
    3600.0 / f64::from(zone_timesteps_per_hour)
}

fn ideal_loads_outdoor_air_sample_timestep_hours(
    timestamp: Option<&str>,
    zone_timestep_hours: f64,
) -> f64 {
    let Some(timestamp) = timestamp else {
        return zone_timestep_hours;
    };
    let Some(start_minute) = timestamp_numeric_field(timestamp, "start") else {
        return zone_timestep_hours;
    };
    let Some(end_minute) = timestamp_numeric_field(timestamp, "end") else {
        return zone_timestep_hours;
    };
    let duration_hours = (end_minute - start_minute) / 60.0;
    if duration_hours > 0.0 && duration_hours < zone_timestep_hours * 0.75 {
        zone_timestep_hours / IDEAL_LOADS_OUTDOOR_AIR_SYSTEM_SUBSTEPS
    } else {
        zone_timestep_hours
    }
}

fn timestamp_numeric_field(timestamp: &str, field_name: &str) -> Option<f64> {
    let prefix = format!("{field_name}=");
    timestamp
        .split(';')
        .find_map(|part| part.strip_prefix(&prefix))
        .and_then(|value| value.parse::<f64>().ok())
}

fn load_series(eso: &Path, key: &str, variable: &str) -> Result<LoadedSeries, String> {
    let series = load_eso_time_series(eso, key, variable)
        .map_err(|error| format!("failed to load ESO series {key}/{variable}: {error}"))?;
    Ok(LoadedSeries {
        units: series.metadata.units,
        samples: run_period_samples(series.samples),
    })
}

fn load_optional_series_or_zero(
    eso: &Path,
    key: &str,
    variable: &str,
    reference: &LoadedSeries,
    units: &str,
) -> Result<LoadedSeries, String> {
    match load_eso_time_series(eso, key, variable) {
        Ok(series) => Ok(LoadedSeries {
            units: series.metadata.units,
            samples: run_period_samples(series.samples),
        }),
        Err(_) => Ok(LoadedSeries {
            units: Some(units.to_string()),
            samples: reference
                .samples
                .iter()
                .enumerate()
                .map(|(index, sample)| SeriesSample {
                    index,
                    timestamp: sample.timestamp.clone(),
                    value: 0.0,
                })
                .collect(),
        }),
    }
}

fn run_period_samples(samples: Vec<SeriesSample>) -> Vec<SeriesSample> {
    let run_period = samples
        .iter()
        .filter(|sample| {
            sample
                .timestamp
                .as_deref()
                .is_some_and(|timestamp| timestamp.to_ascii_uppercase().contains("ENV=RUN PERIOD"))
        })
        .cloned()
        .collect::<Vec<_>>();
    if run_period.is_empty() {
        samples
    } else {
        run_period
    }
}

fn evaluate_rows(
    manifest: &ConformanceCase,
    model: &SimulationModel,
    eso: &Path,
    mtr: &Path,
    weather: Option<&Path>,
    input_trace: &IdealLoadsInputTrace,
    zone_name: &str,
    zone_air_node_name: &str,
    recirculation_node_name: Option<&str>,
    system_name: &str,
    supply_node_name: &str,
    energy_report_interval_seconds: f64,
    fuel_efficiency: IdealLoadsFuelEfficiencyContext,
) -> Result<
    (
        Vec<IdealLoadsDiagnosticRow>,
        Vec<IdealLoadsMeterDiagnosticRow>,
        ResultStore,
        IdealLoadsModeCounts,
    ),
    String,
> {
    let system = model
        .typed
        .ideal_loads_air_systems
        .first()
        .ok_or_else(|| "missing IdealLoads system".to_string())?;
    let zone = model
        .typed
        .zones
        .first()
        .ok_or_else(|| "missing controlled zone".to_string())?;
    let supply_node = model
        .typed
        .nodes
        .iter()
        .find(|node| node.name.0.eq_ignore_ascii_case(supply_node_name))
        .ok_or_else(|| "missing supply node".to_string())?;

    let heating_setpoint =
        thermostat_setpoint_values(model, zone.id, true, input_trace.sample_count)?;
    let cooling_setpoint =
        thermostat_setpoint_values(model, zone.id, false, input_trace.sample_count)?;
    let limit_context = ideal_loads_limit_context(model, system)?;
    let barometric_pressure_trace =
        ideal_loads_barometric_pressure_trace(model, weather, input_trace, limit_context)?;
    let mut calc_results = Vec::with_capacity(input_trace.sample_count);
    let mut mode_counts = IdealLoadsModeCounts::default();
    let source_order_trace_uses_recirculation = recirculation_node_name.is_some()
        && (uses_finite_limits(system) || uses_ideal_loads_humidity_control(system));
    for index in 0..input_trace.sample_count {
        let (zone_temperature, zone_humidity_ratio) = if source_order_trace_uses_recirculation {
            (
                input_trace.recirculation_node_temperature.samples[index].value,
                input_trace.recirculation_node_humidity_ratio.samples[index].value,
            )
        } else {
            // CalcPurchAirLoads sees the zone node before the same-timestamp node
            // output row is updated, so no-limit transition samples use the previous row.
            let calc_zone_state_index = index.saturating_sub(1);
            (
                input_trace.zone_node_temperature.samples[calc_zone_state_index].value,
                input_trace.zone_node_humidity_ratio.samples[calc_zone_state_index].value,
            )
        };
        let active_demand = input_trace.active_demand.samples[index].value;
        let heating_demand = active_demand.max(0.0);
        let cooling_demand = active_demand.min(0.0);
        let zone_state = IdealLoadsZoneState {
            air_temperature_c: zone_temperature,
            air_humidity_ratio: zone_humidity_ratio,
        };
        let recirculation_state = if recirculation_node_name.is_some() {
            IdealLoadsZoneState {
                air_temperature_c: input_trace.recirculation_node_temperature.samples[index].value,
                air_humidity_ratio: input_trace.recirculation_node_humidity_ratio.samples[index]
                    .value,
            }
        } else {
            zone_state
        };
        let mut demand =
            ZoneSysEnergyDemand::sensible_only(zone.id, heating_demand, cooling_demand);
        demand.remaining_output_req_to_humid_sp_kg_per_s =
            input_trace.humidifying_moisture_demand.samples[index].value;
        demand.remaining_output_req_to_dehumid_sp_kg_per_s =
            input_trace.dehumidifying_moisture_demand.samples[index].value;
        let purchased_air = sim_purchased_air_compat(SimPurchasedAirCompatInput {
            system,
            supply_node: supply_node.id,
            zone_state,
            recirculation_state,
            demand,
            unit_available: true,
            limit_context: limit_context
                .with_barometric_pressure_pa(barometric_pressure_trace[index]),
        })
        .map_err(|error| {
            format!(
                "IdealLoads SimPurchasedAir compatibility path rejected system {:?}: {:?}",
                error.system_id, error.unsupported_features
            )
        })?;
        let result = purchased_air.calculation;
        record_mode(&mut mode_counts, result.mode);
        calc_results.push(result);
    }

    let result_source = rust_result_source(system);
    let timestamps = input_trace
        .zone_node_temperature
        .samples
        .iter()
        .take(input_trace.sample_count)
        .map(|sample| sample.timestamp.clone())
        .collect::<Vec<_>>();

    let mut observed_by_variable = BTreeMap::new();
    observed_by_variable.insert(
        (
            zone_name.to_string(),
            ZONE_THERMOSTAT_HEATING_SETPOINT_TEMPERATURE.to_string(),
        ),
        ObservedSeries::new("rust-thermostat-schedule", "C", heating_setpoint),
    );
    observed_by_variable.insert(
        (
            zone_name.to_string(),
            ZONE_THERMOSTAT_COOLING_SETPOINT_TEMPERATURE.to_string(),
        ),
        ObservedSeries::new("rust-thermostat-schedule", "C", cooling_setpoint),
    );
    observed_by_variable.insert(
        (
            zone_air_node_name.to_string(),
            SYSTEM_NODE_TEMPERATURE.to_string(),
        ),
        ObservedSeries::new(
            "oracle-zone-air-node-input",
            "C",
            values_from_samples(
                &input_trace.zone_node_temperature.samples,
                input_trace.sample_count,
            ),
        ),
    );
    observed_by_variable.insert(
        (
            zone_air_node_name.to_string(),
            SYSTEM_NODE_HUMIDITY_RATIO.to_string(),
        ),
        ObservedSeries::new(
            "oracle-zone-air-node-input",
            "kgWater/kgDryAir",
            values_from_samples(
                &input_trace.zone_node_humidity_ratio.samples,
                input_trace.sample_count,
            ),
        ),
    );
    if let Some(recirculation_node_name) = recirculation_node_name {
        observed_by_variable.insert(
            (
                recirculation_node_name.to_string(),
                SYSTEM_NODE_TEMPERATURE.to_string(),
            ),
            ObservedSeries::new(
                "oracle-recirculation-node-input",
                "C",
                values_from_samples(
                    &input_trace.recirculation_node_temperature.samples,
                    input_trace.sample_count,
                ),
            ),
        );
        observed_by_variable.insert(
            (
                recirculation_node_name.to_string(),
                SYSTEM_NODE_HUMIDITY_RATIO.to_string(),
            ),
            ObservedSeries::new(
                "oracle-recirculation-node-input",
                "kgWater/kgDryAir",
                values_from_samples(
                    &input_trace.recirculation_node_humidity_ratio.samples,
                    input_trace.sample_count,
                ),
            ),
        );
    }
    observed_by_variable.insert(
        (
            zone_name.to_string(),
            ZONE_SYSTEM_PREDICTED_SETPOINT_LOAD.to_string(),
        ),
        ObservedSeries::new(
            "oracle-zone-system-active-demand-input",
            "W",
            values_from_samples(&input_trace.active_demand.samples, input_trace.sample_count),
        ),
    );
    observed_by_variable.insert(
        (
            zone_name.to_string(),
            ZONE_SYSTEM_PREDICTED_HEATING_LOAD.to_string(),
        ),
        ObservedSeries::new(
            "oracle-zone-system-demand-input",
            "W",
            values_from_samples(
                &input_trace.heating_demand.samples,
                input_trace.sample_count,
            ),
        ),
    );
    observed_by_variable.insert(
        (
            zone_name.to_string(),
            ZONE_SYSTEM_PREDICTED_COOLING_LOAD.to_string(),
        ),
        ObservedSeries::new(
            "oracle-zone-system-demand-input",
            "W",
            values_from_samples(
                &input_trace.cooling_demand.samples,
                input_trace.sample_count,
            ),
        ),
    );
    observed_by_variable.insert(
        (
            zone_name.to_string(),
            ZONE_SYSTEM_PREDICTED_HUMIDIFYING_MOISTURE_LOAD.to_string(),
        ),
        ObservedSeries::new(
            "oracle-zone-system-moisture-demand-input",
            "kgWater/s",
            values_from_samples(
                &input_trace.humidifying_moisture_demand.samples,
                input_trace.sample_count,
            ),
        ),
    );
    observed_by_variable.insert(
        (
            zone_name.to_string(),
            ZONE_SYSTEM_PREDICTED_DEHUMIDIFYING_MOISTURE_LOAD.to_string(),
        ),
        ObservedSeries::new(
            "oracle-zone-system-moisture-demand-input",
            "kgWater/s",
            values_from_samples(
                &input_trace.dehumidifying_moisture_demand.samples,
                input_trace.sample_count,
            ),
        ),
    );

    add_result_series(
        &mut observed_by_variable,
        system_name,
        &calc_results,
        ZONE_IDEAL_LOADS_ZONE_TOTAL_HEATING_RATE,
        "W",
        result_source,
        |result| result.zone_total_heating_rate_w,
    );
    add_result_series(
        &mut observed_by_variable,
        system_name,
        &calc_results,
        ZONE_IDEAL_LOADS_ZONE_TOTAL_COOLING_RATE,
        "W",
        result_source,
        |result| result.zone_total_cooling_rate_w,
    );
    add_result_series(
        &mut observed_by_variable,
        system_name,
        &calc_results,
        ZONE_IDEAL_LOADS_ZONE_SENSIBLE_HEATING_RATE,
        "W",
        result_source,
        |result| result.zone_sensible_heating_rate_w,
    );
    add_result_series(
        &mut observed_by_variable,
        system_name,
        &calc_results,
        ZONE_IDEAL_LOADS_ZONE_SENSIBLE_COOLING_RATE,
        "W",
        result_source,
        |result| result.zone_sensible_cooling_rate_w,
    );
    add_result_series(
        &mut observed_by_variable,
        system_name,
        &calc_results,
        ZONE_IDEAL_LOADS_ZONE_LATENT_HEATING_RATE,
        "W",
        result_source,
        |result| result.zone_latent_heating_rate_w,
    );
    add_result_series(
        &mut observed_by_variable,
        system_name,
        &calc_results,
        ZONE_IDEAL_LOADS_ZONE_LATENT_COOLING_RATE,
        "W",
        result_source,
        |result| result.zone_latent_cooling_rate_w,
    );
    add_result_series(
        &mut observed_by_variable,
        system_name,
        &calc_results,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_SENSIBLE_HEATING_RATE,
        "W",
        result_source,
        |result| result.supply_air_sensible_heating_rate_w,
    );
    add_result_series(
        &mut observed_by_variable,
        system_name,
        &calc_results,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_SENSIBLE_COOLING_RATE,
        "W",
        result_source,
        |result| result.supply_air_sensible_cooling_rate_w,
    );
    add_result_series(
        &mut observed_by_variable,
        system_name,
        &calc_results,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_HEATING_RATE,
        "W",
        result_source,
        |result| result.supply_air_latent_heating_rate_w,
    );
    add_result_series(
        &mut observed_by_variable,
        system_name,
        &calc_results,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_COOLING_RATE,
        "W",
        result_source,
        |result| result.supply_air_latent_cooling_rate_w,
    );
    add_result_series(
        &mut observed_by_variable,
        system_name,
        &calc_results,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_RATE,
        "W",
        result_source,
        |result| result.supply_air_total_heating_rate_w,
    );
    add_result_series(
        &mut observed_by_variable,
        system_name,
        &calc_results,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_RATE,
        "W",
        result_source,
        |result| result.supply_air_total_cooling_rate_w,
    );
    if manifest_requests_report_energies(manifest) {
        let energy_source = "rust-ideal-loads-report-time-step-energy";
        add_result_energy_series(
            &mut observed_by_variable,
            system_name,
            &calc_results,
            ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_ENERGY,
            energy_source,
            &timestamps,
            energy_report_interval_seconds,
            |result| result.supply_air_total_heating_rate_w,
        );
        add_result_energy_series(
            &mut observed_by_variable,
            system_name,
            &calc_results,
            ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_ENERGY,
            energy_source,
            &timestamps,
            energy_report_interval_seconds,
            |result| result.supply_air_total_cooling_rate_w,
        );
        add_result_energy_series(
            &mut observed_by_variable,
            system_name,
            &calc_results,
            ZONE_IDEAL_LOADS_ZONE_TOTAL_HEATING_ENERGY,
            energy_source,
            &timestamps,
            energy_report_interval_seconds,
            |result| result.zone_total_heating_rate_w,
        );
        add_result_energy_series(
            &mut observed_by_variable,
            system_name,
            &calc_results,
            ZONE_IDEAL_LOADS_ZONE_TOTAL_COOLING_ENERGY,
            energy_source,
            &timestamps,
            energy_report_interval_seconds,
            |result| result.zone_total_cooling_rate_w,
        );
    }
    if manifest_requests_fuel_energy_outputs(manifest) || !manifest.meters.is_empty() {
        let heating_efficiency = fuel_efficiency.heating;
        let cooling_efficiency = fuel_efficiency.cooling;
        let fuel_source = fuel_efficiency.rate_rust_source;
        add_result_series(
            &mut observed_by_variable,
            system_name,
            &calc_results,
            ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_FUEL_ENERGY_RATE,
            "W",
            fuel_source,
            |result| result.supply_air_total_heating_rate_w / heating_efficiency,
        );
        add_result_series(
            &mut observed_by_variable,
            system_name,
            &calc_results,
            ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_FUEL_ENERGY_RATE,
            "W",
            fuel_source,
            |result| result.supply_air_total_cooling_rate_w / cooling_efficiency,
        );
        add_result_series(
            &mut observed_by_variable,
            system_name,
            &calc_results,
            ZONE_IDEAL_LOADS_ZONE_HEATING_FUEL_ENERGY_RATE,
            "W",
            fuel_source,
            |result| result.zone_total_heating_rate_w / heating_efficiency,
        );
        add_result_series(
            &mut observed_by_variable,
            system_name,
            &calc_results,
            ZONE_IDEAL_LOADS_ZONE_COOLING_FUEL_ENERGY_RATE,
            "W",
            fuel_source,
            |result| result.zone_total_cooling_rate_w / cooling_efficiency,
        );
        let fuel_energy_source = fuel_efficiency.energy_rust_source;
        add_result_energy_series(
            &mut observed_by_variable,
            system_name,
            &calc_results,
            ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_FUEL_ENERGY,
            fuel_energy_source,
            &timestamps,
            energy_report_interval_seconds,
            |result| result.supply_air_total_heating_rate_w / heating_efficiency,
        );
        add_result_energy_series(
            &mut observed_by_variable,
            system_name,
            &calc_results,
            ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_FUEL_ENERGY,
            fuel_energy_source,
            &timestamps,
            energy_report_interval_seconds,
            |result| result.supply_air_total_cooling_rate_w / cooling_efficiency,
        );
        add_result_energy_series(
            &mut observed_by_variable,
            system_name,
            &calc_results,
            ZONE_IDEAL_LOADS_ZONE_HEATING_FUEL_ENERGY,
            fuel_energy_source,
            &timestamps,
            energy_report_interval_seconds,
            |result| result.zone_total_heating_rate_w / heating_efficiency,
        );
        add_result_energy_series(
            &mut observed_by_variable,
            system_name,
            &calc_results,
            ZONE_IDEAL_LOADS_ZONE_COOLING_FUEL_ENERGY,
            fuel_energy_source,
            &timestamps,
            energy_report_interval_seconds,
            |result| result.zone_total_cooling_rate_w / cooling_efficiency,
        );
    }
    add_result_series(
        &mut observed_by_variable,
        supply_node_name,
        &calc_results,
        SYSTEM_NODE_TEMPERATURE,
        "C",
        result_source,
        |result| result.supply_temperature_c,
    );
    add_result_series(
        &mut observed_by_variable,
        supply_node_name,
        &calc_results,
        SYSTEM_NODE_HUMIDITY_RATIO,
        "kgWater/kgDryAir",
        result_source,
        |result| result.supply_humidity_ratio,
    );
    add_result_series(
        &mut observed_by_variable,
        supply_node_name,
        &calc_results,
        SYSTEM_NODE_MASS_FLOW_RATE,
        "kg/s",
        result_source,
        |result| result.supply_mass_flow_rate_kg_per_s,
    );

    let meter_rows = evaluate_meter_rows(
        manifest,
        model,
        mtr,
        system_name,
        &observed_by_variable,
        &timestamps,
    )?;

    let mut rows = Vec::new();
    let mut result_store = ResultStore::new();
    for output in &manifest.outputs {
        let expected = load_series(eso, &output.key, &output.variable)?;
        let Some(observed) =
            observed_by_variable.get(&(output.key.clone(), output.variable.clone()))
        else {
            return Err(format!(
                "IdealLoads diagnostic report cannot produce Rust series for {} / {}",
                output.key, output.variable
            ));
        };
        let observed_samples = samples_with_timestamps(&observed.values, &timestamps);
        let tolerance = tolerance_for_output(manifest, output)?;
        let max_rmse_tolerance = max_rmse_tolerance_for_output(manifest, output)?;
        let comparison = compare_series_samples_v2(&expected.samples, &observed_samples, tolerance);
        let mean_abs_delta = mean_abs_delta(&expected.samples, &observed_samples);
        let status = if comparison.status == SeriesComparisonStatus::Pass
            && max_rmse_tolerance.is_none_or(|max_rmse| comparison.rmse_delta <= max_rmse)
        {
            SeriesComparisonStatus::Pass
        } else {
            SeriesComparisonStatus::Fail
        };

        result_store.add_series(OutputSeries {
            handle: OutputHandle(result_store.series.len() as u32),
            key: output.key.clone(),
            variable_name: output.variable.clone(),
            units: observed.units.to_string(),
            values: observed.values.clone(),
        });
        rows.push(IdealLoadsDiagnosticRow {
            key: output.key.clone(),
            variable: output.variable.clone(),
            frequency: output.frequency,
            variable_class: output.class,
            source: output.source,
            domain: output.domain,
            level: output.level,
            units: observed.units.to_string(),
            oracle_units: expected.units.clone(),
            rust_source: observed.source,
            tolerance,
            max_rmse_tolerance,
            expected_samples: comparison.expected_samples,
            observed_samples: comparison.observed_samples,
            compared_samples: comparison.compared_samples,
            max_abs_delta: comparison.max_abs_delta,
            mean_abs_delta,
            rmse_delta: comparison.rmse_delta,
            max_rel_delta: comparison.max_rel_delta,
            alignment: comparison.alignment,
            first_divergence: comparison.first_divergence,
            status,
        });
    }

    Ok((rows, meter_rows, result_store, mode_counts))
}

fn manifest_requests_report_energies(manifest: &ConformanceCase) -> bool {
    manifest
        .outputs
        .iter()
        .any(|output| ideal_loads_report_energy_variable(&output.variable))
}

fn ideal_loads_report_energy_variable(variable: &str) -> bool {
    matches!(
        variable,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_ENERGY
            | ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_ENERGY
            | ZONE_IDEAL_LOADS_ZONE_TOTAL_HEATING_ENERGY
            | ZONE_IDEAL_LOADS_ZONE_TOTAL_COOLING_ENERGY
    )
}

fn manifest_requests_fuel_energy_outputs(manifest: &ConformanceCase) -> bool {
    manifest
        .outputs
        .iter()
        .any(|output| ideal_loads_fuel_energy_variable(&output.variable))
}

fn ideal_loads_fuel_energy_variable(variable: &str) -> bool {
    matches!(
        variable,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_FUEL_ENERGY_RATE
            | ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_FUEL_ENERGY_RATE
            | ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_FUEL_ENERGY
            | ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_FUEL_ENERGY
            | ZONE_IDEAL_LOADS_ZONE_HEATING_FUEL_ENERGY
            | ZONE_IDEAL_LOADS_ZONE_COOLING_FUEL_ENERGY
            | ZONE_IDEAL_LOADS_ZONE_HEATING_FUEL_ENERGY_RATE
            | ZONE_IDEAL_LOADS_ZONE_COOLING_FUEL_ENERGY_RATE
    )
}

fn evaluate_meter_rows(
    manifest: &ConformanceCase,
    model: &SimulationModel,
    mtr: &Path,
    system_name: &str,
    observed_by_variable: &BTreeMap<(String, String), ObservedSeries>,
    timestamps: &[Option<String>],
) -> Result<Vec<IdealLoadsMeterDiagnosticRow>, String> {
    let meter_requests = manifest
        .meters
        .iter()
        .map(|meter| {
            if meter.frequency != OutputFrequency::Hourly {
                Err(format!(
                    "IdealLoads diagnostic meter aggregation currently requires hourly meters, got {} for {}",
                    output_frequency_label(meter.frequency),
                    meter.name
                ))
            } else {
                Ok(RuntimeMeterRequest::hourly(meter.name.clone()))
            }
        })
        .collect::<Result<Vec<_>, String>>()?;
    let meter_registry = RuntimeOutputRegistry::from_model(model);
    let meter_resolution = meter_registry
        .meter_registry()
        .resolve_meter_requests(&meter_requests);
    if meter_resolution.diagnostics.has_errors() {
        let diagnostics = meter_resolution
            .diagnostics
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.message.as_str())
            .collect::<Vec<_>>()
            .join("; ");
        return Err(format!(
            "IdealLoads diagnostic meter aggregation requires MeterRegistry-resolved meters: {diagnostics}"
        ));
    }
    let fuel_energy_bindings = ideal_loads_meter_fuel_energy_bindings(model);
    let mut rows = Vec::new();
    for meter in &manifest.meters {
        let resolved_meter = meter_resolution
            .resolved
            .iter()
            .find(|resolved| resolved.request.name.eq_ignore_ascii_case(&meter.name))
            .ok_or_else(|| {
                format!(
                    "IdealLoads diagnostic meter aggregation did not resolve {} through MeterRegistry",
                    meter.name
                )
            })?;
        let expected = load_meter_series(mtr, &meter.name)?;
        let resolved_meter_key = NormalizedName::new(&resolved_meter.definition.name).0;
        let fuel_energy_variable = fuel_energy_bindings
            .get(&resolved_meter_key)
            .copied()
            .ok_or_else(|| {
                format!(
                    "IdealLoads diagnostic meter aggregation has no fuel-energy binding for MeterRegistry meter {}",
                    resolved_meter.definition.name
                )
            })?;
        let Some(observed) =
            observed_by_variable.get(&(system_name.to_string(), fuel_energy_variable.to_string()))
        else {
            return Err(format!(
                "IdealLoads diagnostic report cannot produce Rust meter source series for {} from {}",
                meter.name, fuel_energy_variable
            ));
        };
        let observed_samples =
            hourly_meter_samples_from_detailed_energy(&observed.values, timestamps)?;
        let tolerance = tolerance_for_meter(meter);
        let max_rmse_tolerance = meter.rmse_tol;
        let comparison = compare_series_samples_v2(&expected.samples, &observed_samples, tolerance);
        let mean_abs_delta = mean_abs_delta(&expected.samples, &observed_samples);
        let status = if comparison.status == SeriesComparisonStatus::Pass
            && max_rmse_tolerance.is_none_or(|max_rmse| comparison.rmse_delta <= max_rmse)
        {
            SeriesComparisonStatus::Pass
        } else {
            SeriesComparisonStatus::Fail
        };

        rows.push(IdealLoadsMeterDiagnosticRow {
            name: meter.name.clone(),
            frequency: meter.frequency,
            source: meter.source,
            domain: meter.domain,
            level: meter.level,
            units: observed.units.to_string(),
            oracle_units: expected.units.clone(),
            rust_source: IDEAL_LOADS_FACILITY_METER_RUST_SOURCE,
            tolerance,
            max_rmse_tolerance,
            expected_samples: comparison.expected_samples,
            observed_samples: comparison.observed_samples,
            compared_samples: comparison.compared_samples,
            max_abs_delta: comparison.max_abs_delta,
            mean_abs_delta,
            rmse_delta: comparison.rmse_delta,
            max_rel_delta: comparison.max_rel_delta,
            alignment: comparison.alignment,
            first_divergence: comparison.first_divergence,
            status,
        });
    }

    Ok(rows)
}

fn load_meter_series(mtr: &Path, meter: &str) -> Result<LoadedSeries, String> {
    let series = load_mtr_time_series(mtr, meter)
        .map_err(|error| format!("failed to load MTR meter {meter}: {error}"))?;
    Ok(LoadedSeries {
        units: series.metadata.units,
        samples: run_period_samples(series.samples),
    })
}

fn ideal_loads_meter_fuel_energy_bindings(
    model: &SimulationModel,
) -> BTreeMap<String, &'static str> {
    let mut bindings = BTreeMap::new();
    for system in &model.typed.ideal_loads_air_systems {
        for fuel_type in [system.heating_fuel_type, system.cooling_fuel_type] {
            if let Some(binding) = ideal_loads_facility_meter_binding(fuel_type) {
                bindings.insert(
                    NormalizedName::new(binding.meter_name).0,
                    binding.fuel_energy_variable,
                );
            }
        }
    }
    bindings
}

fn hourly_meter_samples_from_detailed_energy(
    values: &[f64],
    timestamps: &[Option<String>],
) -> Result<Vec<SeriesSample>, String> {
    let mut hourly_values = Vec::<(String, f64)>::new();
    for (index, value) in values.iter().copied().enumerate() {
        let timestamp = timestamps
            .get(index)
            .and_then(|timestamp| timestamp.as_deref())
            .ok_or_else(|| {
                format!(
                    "IdealLoads meter diagnostic requires timestamped detailed fuel energy sample {index}"
                )
            })?;
        let hourly_timestamp = hourly_meter_timestamp_label(timestamp).ok_or_else(|| {
            format!("IdealLoads meter diagnostic cannot derive hourly timestamp from {timestamp}")
        })?;
        if let Some((_, total)) = hourly_values
            .iter_mut()
            .find(|(candidate, _)| candidate == &hourly_timestamp)
        {
            *total += value;
        } else {
            hourly_values.push((hourly_timestamp, value));
        }
    }

    Ok(hourly_values
        .into_iter()
        .enumerate()
        .map(|(index, (timestamp, value))| SeriesSample::timestamped(index, timestamp, value))
        .collect())
}

fn hourly_meter_timestamp_label(timestamp: &str) -> Option<String> {
    Some(format!(
        "env={};day={};month={};date={};dst={};hour={};start=0.00;end=60.00;day_type={}",
        timestamp_field(timestamp, "env")?,
        timestamp_field(timestamp, "day")?,
        timestamp_field(timestamp, "month")?,
        timestamp_field(timestamp, "date")?,
        timestamp_field(timestamp, "dst")?,
        timestamp_field(timestamp, "hour")?,
        timestamp_field(timestamp, "day_type")?
    ))
}

fn timestamp_field<'a>(timestamp: &'a str, name: &str) -> Option<&'a str> {
    for field in timestamp.split(';') {
        let Some((key, value)) = field.split_once('=') else {
            continue;
        };
        if key.trim().eq_ignore_ascii_case(name) {
            return Some(value.trim());
        }
    }
    None
}

fn tolerance_for_meter(meter: &MeterRequest) -> Tolerance {
    Tolerance {
        absolute: meter.abs_tol.unwrap_or(0.0),
        relative: meter.rel_tol.unwrap_or(0.0),
    }
}

fn ideal_loads_fuel_efficiency_context(
    model: &SimulationModel,
    system: &IdealLoadsAirSystem,
) -> Result<IdealLoadsFuelEfficiencyContext, String> {
    let heating = ideal_loads_fuel_efficiency_value(
        model,
        system.heating_fuel_efficiency_schedule,
        "heating",
    )?;
    let cooling = ideal_loads_fuel_efficiency_value(
        model,
        system.cooling_fuel_efficiency_schedule,
        "cooling",
    )?;
    if system.heating_fuel_efficiency_schedule.is_none()
        && system.cooling_fuel_efficiency_schedule.is_none()
    {
        Ok(IdealLoadsFuelEfficiencyContext::blank())
    } else {
        Ok(IdealLoadsFuelEfficiencyContext::constant(heating, cooling))
    }
}

fn ideal_loads_fuel_efficiency_value(
    model: &SimulationModel,
    schedule_id: Option<ScheduleId>,
    label: &str,
) -> Result<f64, String> {
    let Some(schedule_id) = schedule_id else {
        return Ok(1.0);
    };
    let schedule = model
        .typed
        .schedules
        .iter()
        .find(|schedule| schedule.id == schedule_id)
        .ok_or_else(|| {
            format!(
                "IdealLoads {label} fuel energy diagnostic currently supports only blank or Schedule:Constant fuel efficiency schedules"
            )
        })?;
    if !schedule.hourly_value.is_finite() || schedule.hourly_value <= 0.0 {
        return Err(format!(
            "IdealLoads {label} fuel efficiency schedule {} must have a positive finite value, got {}",
            schedule.name.0, schedule.hourly_value
        ));
    }
    Ok(schedule.hourly_value)
}

fn ideal_loads_limit_context(
    model: &SimulationModel,
    system: &IdealLoadsAirSystem,
) -> Result<IdealLoadsSensibleLimitContext, String> {
    if !uses_finite_limits(system) {
        return Ok(model
            .typed
            .site
            .as_ref()
            .and_then(|site| {
                IdealLoadsSensibleLimitContext::from_site_elevation_m(site.elevation_m)
            })
            .unwrap_or_default());
    }

    let site =
        model.typed.site.as_ref().ok_or_else(|| {
            "IdealLoads finite-limit diagnostics require Site:Location".to_string()
        })?;
    IdealLoadsSensibleLimitContext::from_site_elevation_m(site.elevation_m).ok_or_else(|| {
        format!(
            "failed to derive EnergyPlus StdRhoAir from site elevation {}",
            site.elevation_m
        )
    })
}

fn ideal_loads_barometric_pressure_trace(
    model: &SimulationModel,
    weather: Option<&Path>,
    input_trace: &IdealLoadsInputTrace,
    limit_context: IdealLoadsSensibleLimitContext,
) -> Result<Vec<f64>, String> {
    ideal_loads_barometric_pressure_samples(
        model,
        weather,
        &input_trace.zone_node_temperature.samples,
        input_trace.sample_count,
        limit_context,
    )
}

fn ideal_loads_barometric_pressure_samples(
    model: &SimulationModel,
    weather: Option<&Path>,
    samples: &[SeriesSample],
    sample_count: usize,
    limit_context: IdealLoadsSensibleLimitContext,
) -> Result<Vec<f64>, String> {
    let Some(weather) = weather else {
        return Ok(vec![limit_context.barometric_pressure_pa; sample_count]);
    };
    let weather_records =
        load_epw_records(weather).map_err(|error| format!("failed to load EPW: {error}"))?;
    if weather_records.is_empty() {
        return Ok(vec![limit_context.barometric_pressure_pa; sample_count]);
    }

    let zone_steps_per_hour = model.typed.timestep.number_of_timesteps_per_hour.max(1);
    let first_hour_interpolation_starting_values = model
        .typed
        .run_periods
        .first()
        .map(|run_period| run_period.first_hour_interpolation_starting_values)
        .unwrap_or_default();
    Ok(samples
        .iter()
        .take(sample_count)
        .map(|sample| {
            sample
                .timestamp
                .as_deref()
                .and_then(parse_ideal_loads_timestamp)
                .and_then(|timestamp| {
                    ideal_loads_weather_pressure_for_timestamp(
                        &weather_records,
                        timestamp,
                        zone_steps_per_hour,
                        first_hour_interpolation_starting_values,
                    )
                })
                .unwrap_or(limit_context.barometric_pressure_pa)
        })
        .collect())
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct IdealLoadsTimestampFields {
    month: u32,
    day_of_month: u32,
    hour: u32,
    end_minute: f64,
}

fn parse_ideal_loads_timestamp(timestamp: &str) -> Option<IdealLoadsTimestampFields> {
    let mut month = None;
    let mut day_of_month = None;
    let mut hour = None;
    let mut end_minute = None;
    for field in timestamp.split(';') {
        let (key, value) = field.split_once('=')?;
        match key.trim() {
            "month" => month = value.trim().parse::<u32>().ok(),
            "date" => day_of_month = value.trim().parse::<u32>().ok(),
            "hour" => hour = value.trim().parse::<u32>().ok(),
            "end" => end_minute = value.trim().parse::<f64>().ok(),
            _ => {}
        }
    }
    Some(IdealLoadsTimestampFields {
        month: month?,
        day_of_month: day_of_month?,
        hour: hour?,
        end_minute: end_minute?,
    })
}

fn ideal_loads_weather_pressure_for_timestamp(
    weather_records: &[EpwRecord],
    timestamp: IdealLoadsTimestampFields,
    zone_steps_per_hour: u32,
    first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
) -> Option<f64> {
    let record_index = weather_records.iter().position(|record| {
        record.month == timestamp.month
            && record.day == timestamp.day_of_month
            && record.hour == timestamp.hour
    })?;
    let record = weather_records.get(record_index)?;
    let previous_record = previous_ideal_loads_weather_record(
        weather_records,
        record_index,
        first_hour_interpolation_starting_values,
    )?;
    let weight = ideal_loads_weather_interpolation_weight(
        zone_steps_per_hour,
        ideal_loads_zone_timestep(timestamp.end_minute, zone_steps_per_hour),
    );
    Some(
        previous_record.atmospheric_pressure_pa * (1.0 - weight)
            + record.atmospheric_pressure_pa * weight,
    )
}

fn previous_ideal_loads_weather_record(
    weather_records: &[EpwRecord],
    record_index: usize,
    first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
) -> Option<&EpwRecord> {
    if record_index == 0 {
        let first_day_record_index = match first_hour_interpolation_starting_values {
            FirstHourInterpolationStartingValues::Hour1 => 0,
            FirstHourInterpolationStartingValues::Hour24 => weather_records.len().min(24) - 1,
        };
        weather_records.get(first_day_record_index)
    } else {
        weather_records.get(record_index - 1)
    }
}

fn ideal_loads_zone_timestep(end_minute: f64, zone_steps_per_hour: u32) -> u32 {
    let steps = zone_steps_per_hour.max(1);
    let minutes_per_step = 60.0 / f64::from(steps);
    (end_minute / minutes_per_step)
        .round()
        .clamp(1.0, f64::from(steps)) as u32
}

fn ideal_loads_weather_interpolation_weight(zone_steps_per_hour: u32, zone_timestep: u32) -> f64 {
    let steps = zone_steps_per_hour.max(1);
    if steps == 1 {
        return 1.0;
    }
    (f64::from(zone_timestep.clamp(1, steps)) / f64::from(steps)).min(1.0)
}

fn ideal_loads_recirculation_node_name(
    model: &SimulationModel,
    zone_id: ep_model::ZoneId,
    system: &IdealLoadsAirSystem,
) -> Result<String, String> {
    if let Some(exhaust_node_name) = system.zone_exhaust_air_node_name.as_ref() {
        return resolve_first_node_or_list_name(model, &exhaust_node_name.0).ok_or_else(|| {
            format!(
                "failed to resolve IdealLoads exhaust/recirculation node {}",
                exhaust_node_name.0
            )
        });
    }

    let connection = model
        .typed
        .zone_equipment_connections
        .iter()
        .find(|connection| connection.zone == zone_id)
        .ok_or_else(|| {
            "missing ZoneHVAC:EquipmentConnections for finite-limit recirculation node".to_string()
        })?;
    let Some(return_node_name) = connection.zone_return_air_node_or_nodelist_name.as_ref() else {
        return Err(
            "finite-limit IdealLoads diagnostic requires a zone return air node or node list"
                .to_string(),
        );
    };
    resolve_first_node_or_list_name(model, &return_node_name.0).ok_or_else(|| {
        format!(
            "failed to resolve IdealLoads return/recirculation node {}",
            return_node_name.0
        )
    })
}

fn manifest_requests_ideal_loads_recirculation_node(
    manifest: &ConformanceCase,
    model: &SimulationModel,
    zone_id: ep_model::ZoneId,
    system: &IdealLoadsAirSystem,
) -> Result<bool, String> {
    if !uses_ideal_loads_humidity_control(system) {
        return Ok(false);
    }
    let recirculation_node_name = ideal_loads_recirculation_node_name(model, zone_id, system)?;
    Ok(manifest
        .outputs
        .iter()
        .any(|output| output.key.eq_ignore_ascii_case(&recirculation_node_name)))
}

fn manifest_allows_constant_supply_humidity_diagnostic(
    manifest: &ConformanceCase,
    system: &IdealLoadsAirSystem,
) -> bool {
    !manifest.conformance_claim
        && system.dehumidification_control_type
            == DehumidificationControlType::ConstantSupplyHumidityRatio
        && system.humidification_control_type == HumidificationControlType::None
        && manifest.outputs.iter().any(|output| {
            output.variable == ZONE_IDEAL_LOADS_ZONE_LATENT_COOLING_RATE
                || output.variable == ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_COOLING_RATE
        })
}

fn manifest_allows_constant_supply_humidity_cooling_conformance(
    manifest: &ConformanceCase,
    system: &IdealLoadsAirSystem,
) -> bool {
    manifest.conformance_claim
        && manifest.id == "ideal_loads_constant_supply_humidity_cooling_conformance_candidate_001"
        && system.dehumidification_control_type
            == DehumidificationControlType::ConstantSupplyHumidityRatio
        && system.humidification_control_type == HumidificationControlType::None
        && manifest.outputs.iter().any(|output| {
            output.level == Some(OutputLevel::Conformance)
                && output.variable == ZONE_IDEAL_LOADS_ZONE_LATENT_COOLING_RATE
        })
        && manifest.outputs.iter().any(|output| {
            output.level == Some(OutputLevel::Conformance)
                && output.variable == ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_COOLING_RATE
        })
}

fn manifest_allows_humidistat_dehumidification_diagnostic(
    manifest: &ConformanceCase,
    system: &IdealLoadsAirSystem,
) -> bool {
    !manifest.conformance_claim
        && system.dehumidification_control_type == DehumidificationControlType::Humidistat
        && system.humidification_control_type == HumidificationControlType::None
        && manifest.outputs.iter().any(|output| {
            output.variable == ZONE_SYSTEM_PREDICTED_DEHUMIDIFYING_MOISTURE_LOAD
                || output.variable == ZONE_IDEAL_LOADS_ZONE_LATENT_COOLING_RATE
                || output.variable == ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_COOLING_RATE
        })
}

fn manifest_allows_humidistat_dehumidification_conformance(
    manifest: &ConformanceCase,
    system: &IdealLoadsAirSystem,
) -> bool {
    manifest.conformance_claim
        && manifest.id == "ideal_loads_humidistat_dehumidification_conformance_candidate_001"
        && system.dehumidification_control_type == DehumidificationControlType::Humidistat
        && system.humidification_control_type == HumidificationControlType::None
        && manifest.outputs.iter().any(|output| {
            output.level == Some(OutputLevel::Conformance)
                && output.variable == ZONE_IDEAL_LOADS_ZONE_LATENT_COOLING_RATE
        })
        && manifest.outputs.iter().any(|output| {
            output.level == Some(OutputLevel::Conformance)
                && output.variable == ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_COOLING_RATE
        })
}

fn manifest_allows_humidistat_humidification_diagnostic(
    manifest: &ConformanceCase,
    system: &IdealLoadsAirSystem,
) -> bool {
    !manifest.conformance_claim
        && system.dehumidification_control_type == DehumidificationControlType::None
        && system.humidification_control_type == HumidificationControlType::Humidistat
        && manifest.outputs.iter().any(|output| {
            output.variable == ZONE_SYSTEM_PREDICTED_HUMIDIFYING_MOISTURE_LOAD
                || output.variable == ZONE_IDEAL_LOADS_ZONE_LATENT_HEATING_RATE
                || output.variable == ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_HEATING_RATE
        })
}

fn manifest_allows_humidistat_humidification_conformance(
    manifest: &ConformanceCase,
    system: &IdealLoadsAirSystem,
) -> bool {
    manifest.conformance_claim
        && manifest.id == "ideal_loads_humidistat_humidification_conformance_candidate_001"
        && system.dehumidification_control_type == DehumidificationControlType::None
        && system.humidification_control_type == HumidificationControlType::Humidistat
        && manifest.outputs.iter().any(|output| {
            output.level == Some(OutputLevel::Conformance)
                && output.variable == ZONE_IDEAL_LOADS_ZONE_LATENT_HEATING_RATE
        })
        && manifest.outputs.iter().any(|output| {
            output.level == Some(OutputLevel::Conformance)
                && output.variable == ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_HEATING_RATE
        })
}

fn manifest_allows_constant_supply_humidity_humidification_diagnostic(
    manifest: &ConformanceCase,
    system: &IdealLoadsAirSystem,
) -> bool {
    !manifest.conformance_claim
        && system.dehumidification_control_type == DehumidificationControlType::None
        && system.humidification_control_type
            == HumidificationControlType::ConstantSupplyHumidityRatio
        && manifest.outputs.iter().any(|output| {
            output.variable == ZONE_IDEAL_LOADS_ZONE_LATENT_HEATING_RATE
                || output.variable == ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_HEATING_RATE
        })
}

fn manifest_allows_constant_supply_humidity_heating_conformance(
    manifest: &ConformanceCase,
    system: &IdealLoadsAirSystem,
) -> bool {
    manifest.conformance_claim
        && manifest.id == "ideal_loads_constant_supply_humidity_heating_conformance_candidate_001"
        && system.dehumidification_control_type == DehumidificationControlType::None
        && system.humidification_control_type
            == HumidificationControlType::ConstantSupplyHumidityRatio
        && manifest.outputs.iter().any(|output| {
            output.level == Some(OutputLevel::Conformance)
                && output.variable == ZONE_IDEAL_LOADS_ZONE_LATENT_HEATING_RATE
        })
        && manifest.outputs.iter().any(|output| {
            output.level == Some(OutputLevel::Conformance)
                && output.variable == ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_HEATING_RATE
        })
}

fn manifest_allows_finite_limit_conformance(
    manifest: &ConformanceCase,
    system: &IdealLoadsAirSystem,
) -> bool {
    if !manifest.conformance_claim {
        return false;
    }

    match manifest.id.as_str() {
        "ideal_loads_capacity_limit_conformance_001" => {
            system.heating_limit == IdealLoadsLimit::LimitCapacity
                && system.cooling_limit == IdealLoadsLimit::LimitCapacity
        }
        "ideal_loads_flow_limit_conformance_001" => {
            system.heating_limit == IdealLoadsLimit::LimitFlowRate
                && system.cooling_limit == IdealLoadsLimit::LimitFlowRate
        }
        "ideal_loads_flow_capacity_limit_conformance_001" => {
            system.heating_limit == IdealLoadsLimit::LimitFlowRateAndCapacity
                && system.cooling_limit == IdealLoadsLimit::LimitFlowRateAndCapacity
        }
        _ => false,
    }
}

fn manifest_allows_constant_shr_conformance(
    manifest: &ConformanceCase,
    system: &IdealLoadsAirSystem,
) -> bool {
    manifest.conformance_claim
        && manifest.id == "ideal_loads_constant_shr_conformance_001"
        && system.dehumidification_control_type
            == DehumidificationControlType::ConstantSensibleHeatRatio
        && system.humidification_control_type == HumidificationControlType::None
}

fn resolve_first_node_or_list_name(model: &SimulationModel, name: &str) -> Option<String> {
    if let Some(node_id) = model.typed.node_names.resolve(name) {
        return model
            .typed
            .nodes
            .iter()
            .find(|node| node.id == node_id)
            .map(|node| node.name.0.clone());
    }
    let node_list_id = model.typed.node_list_names.resolve(name)?;
    let node_list = model
        .typed
        .node_lists
        .iter()
        .find(|node_list| node_list.id == node_list_id)?;
    let node_id = node_list.nodes.first()?;
    model
        .typed
        .nodes
        .iter()
        .find(|node| node.id == *node_id)
        .map(|node| node.name.0.clone())
}

fn ideal_loads_sensible_branch(system: &IdealLoadsAirSystem) -> &'static str {
    if uses_finite_limits(system) {
        "no-oa-finite-limit-sensible"
    } else {
        "no-oa-no-limit-sensible"
    }
}

fn inactive_ideal_loads_branches(system: &IdealLoadsAirSystem) -> Vec<&'static str> {
    let mut branches = Vec::new();
    if !uses_outdoor_air(system) {
        branches.push("outdoor_air");
    }
    if system.outdoor_air_economizer_type == OutdoorAirEconomizerType::NoEconomizer {
        branches.push("economizer");
    }
    if system.heat_recovery_type == HeatRecoveryType::None {
        branches.push("heat_recovery");
    }
    if system.dehumidification_control_type != DehumidificationControlType::Humidistat
        && system.humidification_control_type != HumidificationControlType::Humidistat
    {
        branches.push("humidistat");
    }
    if system.demand_controlled_ventilation_type == DemandControlledVentilationType::None {
        branches.push("dcv");
    }
    if !uses_autosizing(system) {
        branches.push("autosizing");
    }
    branches.push("saturation_limit");
    branches
}

fn declared_ideal_loads_branch(
    manifest: &ConformanceCase,
    system: &IdealLoadsAirSystem,
) -> &'static str {
    if manifest.id.contains("constant_supply_humidity_heating") {
        "constant_supply_humidity_heating"
    } else if manifest.id.contains("constant_supply_humidity") {
        "constant_supply_humidity_cooling"
    } else if manifest.id.contains("humidistat_dehumidification") {
        "humidistat_dehumidification"
    } else if manifest.id.contains("humidistat_humidification") {
        "humidistat_humidification"
    } else if manifest.id.contains("constant_shr") {
        "constant_shr"
    } else if manifest.id.contains("flow_capacity_limit") {
        "flow_and_capacity"
    } else if manifest.id.contains("flow_limit") {
        "finite_flow"
    } else if manifest.id.contains("capacity_limit") {
        "finite_capacity"
    } else if uses_finite_limits(system) {
        select_purchased_air_branch(system).label()
    } else {
        "no_oa_sensible"
    }
}

fn ideal_loads_recirculation_state_source(branch: &str) -> &'static str {
    if branch == "no-oa-finite-limit-sensible" {
        IDEAL_LOADS_FINITE_LIMIT_RECIRCULATION_STATE_SOURCE
    } else {
        IDEAL_LOADS_HUMIDITY_CONTROL_RECIRCULATION_STATE_SOURCE
    }
}

fn rust_result_source(system: &IdealLoadsAirSystem) -> &'static str {
    if uses_finite_limits(system) {
        "rust-ideal-loads-no-oa-sensible-limited-calc"
    } else {
        "rust-ideal-loads-no-oa-sensible-calc"
    }
}

fn uses_finite_limits(system: &IdealLoadsAirSystem) -> bool {
    system.heating_limit != IdealLoadsLimit::NoLimit
        || system.cooling_limit != IdealLoadsLimit::NoLimit
}

fn uses_outdoor_air(system: &IdealLoadsAirSystem) -> bool {
    system
        .design_specification_outdoor_air_object_name
        .is_some()
        || system.outdoor_air_inlet_node_name.is_some()
}

fn uses_autosizing(system: &IdealLoadsAirSystem) -> bool {
    system
        .design_specification_zonehvac_sizing_object_name
        .is_some()
        || matches!(
            system.maximum_heating_air_flow_rate_m3_per_s,
            Some(AutosizeOrNumber::Autosize)
        )
        || matches!(
            system.maximum_sensible_heating_capacity_w,
            Some(AutosizeOrNumber::Autosize)
        )
        || matches!(
            system.maximum_cooling_air_flow_rate_m3_per_s,
            Some(AutosizeOrNumber::Autosize)
        )
        || matches!(
            system.maximum_total_cooling_capacity_w,
            Some(AutosizeOrNumber::Autosize)
        )
}

fn uses_ideal_loads_humidity_control(system: &IdealLoadsAirSystem) -> bool {
    system.dehumidification_control_type != DehumidificationControlType::None
        || system.humidification_control_type != HumidificationControlType::None
}

struct ObservedSeries {
    source: &'static str,
    units: &'static str,
    values: Vec<f64>,
}

impl ObservedSeries {
    fn new(source: &'static str, units: &'static str, values: Vec<f64>) -> Self {
        Self {
            source,
            units,
            values,
        }
    }
}

fn thermostat_setpoint_values(
    model: &SimulationModel,
    zone: ep_model::ZoneId,
    heating: bool,
    sample_count: usize,
) -> Result<Vec<f64>, String> {
    let thermostat_edge = model
        .graph
        .zone_thermostats
        .iter()
        .find(|edge| edge.zone == zone)
        .ok_or_else(|| "missing zone thermostat edge".to_string())?;
    let thermostat = model
        .typed
        .zone_thermostats
        .iter()
        .find(|thermostat| thermostat.id == thermostat_edge.thermostat)
        .ok_or_else(|| "missing zone thermostat".to_string())?;
    let control = thermostat
        .controls
        .first()
        .ok_or_else(|| "zone thermostat has no controls".to_string())?;
    let dual_setpoint = model
        .typed
        .thermostat_dual_setpoints
        .iter()
        .find(|setpoint| setpoint.id == control.dual_setpoint)
        .ok_or_else(|| "missing ThermostatSetpoint:DualSetpoint".to_string())?;
    let schedule_id = if heating {
        dual_setpoint.heating_setpoint_schedule
    } else {
        dual_setpoint.cooling_setpoint_schedule
    };
    let schedule = model
        .typed
        .schedules
        .iter()
        .find(|schedule| schedule.id == schedule_id)
        .ok_or_else(|| {
            "IdealLoads diagnostic currently requires constant thermostat setpoint schedules"
                .to_string()
        })?;
    Ok(vec![schedule.hourly_value; sample_count])
}

fn add_result_series(
    observed_by_variable: &mut BTreeMap<(String, String), ObservedSeries>,
    key: &str,
    results: &[IdealLoadsSensibleResult],
    variable: &str,
    units: &'static str,
    source: &'static str,
    value: impl Fn(IdealLoadsSensibleResult) -> f64,
) {
    observed_by_variable.insert(
        (key.to_string(), variable.to_string()),
        ObservedSeries::new(source, units, results.iter().copied().map(value).collect()),
    );
}

fn add_result_energy_series(
    observed_by_variable: &mut BTreeMap<(String, String), ObservedSeries>,
    key: &str,
    results: &[IdealLoadsSensibleResult],
    variable: &str,
    source: &'static str,
    timestamps: &[Option<String>],
    default_report_interval_seconds: f64,
    rate: impl Fn(IdealLoadsSensibleResult) -> f64,
) {
    let values = results
        .iter()
        .copied()
        .enumerate()
        .map(|(index, result)| {
            let interval_seconds = energy_report_seconds_from_timestamp(
                timestamps
                    .get(index)
                    .and_then(|timestamp| timestamp.as_deref()),
                default_report_interval_seconds,
            );
            rate(result) * interval_seconds
        })
        .collect();
    observed_by_variable.insert(
        (key.to_string(), variable.to_string()),
        ObservedSeries::new(source, "J", values),
    );
}

fn energy_report_seconds_from_timestamp(
    timestamp: Option<&str>,
    default_report_interval_seconds: f64,
) -> f64 {
    let Some(timestamp) = timestamp else {
        return default_report_interval_seconds;
    };
    let mut start_minutes = None;
    let mut end_minutes = None;
    for field in timestamp.split(';') {
        let Some((key, value)) = field.split_once('=') else {
            continue;
        };
        if key.eq_ignore_ascii_case("start") {
            start_minutes = value.trim().parse::<f64>().ok();
        } else if key.eq_ignore_ascii_case("end") {
            end_minutes = value.trim().parse::<f64>().ok();
        }
    }
    let (Some(start_minutes), Some(end_minutes)) = (start_minutes, end_minutes) else {
        return default_report_interval_seconds;
    };
    let duration_minutes = end_minutes - start_minutes;
    if duration_minutes <= 0.0 || !duration_minutes.is_finite() {
        return default_report_interval_seconds;
    }
    let default_report_interval_minutes = default_report_interval_seconds / 60.0;
    if default_report_interval_minutes <= 0.0 || !default_report_interval_minutes.is_finite() {
        return default_report_interval_seconds;
    }
    let substeps = (default_report_interval_minutes / duration_minutes)
        .round()
        .max(1.0);
    default_report_interval_seconds / substeps
}

fn values_from_samples(samples: &[SeriesSample], sample_count: usize) -> Vec<f64> {
    samples
        .iter()
        .take(sample_count)
        .map(|sample| sample.value)
        .collect()
}

fn samples_with_timestamps(values: &[f64], timestamps: &[Option<String>]) -> Vec<SeriesSample> {
    values
        .iter()
        .copied()
        .enumerate()
        .map(
            |(index, value)| match timestamps.get(index).cloned().flatten() {
                Some(timestamp) => SeriesSample::timestamped(index, timestamp, value),
                None => SeriesSample::indexed(index, value),
            },
        )
        .collect()
}

fn tolerance_for_output(
    manifest: &ConformanceCase,
    output: &OutputRequest,
) -> Result<Tolerance, String> {
    let rule = manifest
        .tolerances
        .iter()
        .find(|rule| rule.variable_class == output.class)
        .ok_or_else(|| {
            format!(
                "missing tolerance rule for {} output {}",
                variable_class_label(output.class),
                output.variable
            )
        })?;
    Ok(Tolerance {
        absolute: output.abs_tol.or(rule.max_abs).unwrap_or(0.0),
        relative: output.rel_tol.or(rule.max_rel).unwrap_or(0.0),
    })
}

fn max_rmse_tolerance_for_output(
    manifest: &ConformanceCase,
    output: &OutputRequest,
) -> Result<Option<f64>, String> {
    let rule = manifest
        .tolerances
        .iter()
        .find(|rule| rule.variable_class == output.class)
        .ok_or_else(|| {
            format!(
                "missing tolerance rule for {} output {}",
                variable_class_label(output.class),
                output.variable
            )
        })?;
    Ok(output.rmse_tol.or(rule.max_rmse))
}

fn mean_abs_delta(expected: &[SeriesSample], observed: &[SeriesSample]) -> f64 {
    let compared_samples = expected.len().min(observed.len());
    if compared_samples == 0 {
        return 0.0;
    }
    expected
        .iter()
        .zip(observed)
        .take(compared_samples)
        .map(|(left, right)| (left.value - right.value).abs())
        .sum::<f64>()
        / compared_samples as f64
}

fn write_artifacts(
    compare_dir: &Path,
    context: &IdealLoadsDiagnosticContext<'_>,
) -> Result<(), String> {
    std::fs::create_dir_all(compare_dir)
        .map_err(|error| format!("failed to create IdealLoads report directory: {error}"))?;
    std::fs::write(
        compare_dir.join("compare-report.md"),
        render_markdown(context),
    )
    .map_err(|error| format!("failed to write IdealLoads compare report: {error}"))?;
    std::fs::write(
        compare_dir.join("compare-summary.json"),
        render_summary_json(context),
    )
    .map_err(|error| format!("failed to write IdealLoads compare summary: {error}"))?;
    std::fs::write(
        compare_dir.join("selected_outputs.json"),
        render_selected_outputs_json(context),
    )
    .map_err(|error| format!("failed to write IdealLoads selected outputs: {error}"))?;
    std::fs::write(
        compare_dir.join("rust-result-store.json"),
        render_result_store_json(context),
    )
    .map_err(|error| format!("failed to write IdealLoads Rust result store: {error}"))?;
    std::fs::write(
        compare_dir.join("variable-deltas.csv"),
        render_variable_deltas_csv(context),
    )
    .map_err(|error| format!("failed to write IdealLoads variable deltas: {error}"))?;
    std::fs::write(
        compare_dir.join("first-divergence.csv"),
        render_first_divergence_csv(context),
    )
    .map_err(|error| format!("failed to write IdealLoads first divergence CSV: {error}"))?;
    std::fs::write(
        compare_dir.join("tolerance-failures.csv"),
        render_tolerance_failures_csv(context),
    )
    .map_err(|error| format!("failed to write IdealLoads tolerance failures CSV: {error}"))?;
    std::fs::write(
        compare_dir.join("stage-summary.json"),
        render_stage_summary_json(context),
    )
    .map_err(|error| format!("failed to write IdealLoads stage summary: {error}"))?;
    Ok(())
}

fn write_outdoor_air_artifacts(
    compare_dir: &Path,
    context: &IdealLoadsOutdoorAirDiagnosticContext<'_>,
) -> Result<(), String> {
    std::fs::create_dir_all(compare_dir).map_err(|error| {
        format!("failed to create IdealLoads outdoor-air report directory: {error}")
    })?;
    std::fs::write(
        compare_dir.join("compare-report.md"),
        render_outdoor_air_markdown(context),
    )
    .map_err(|error| format!("failed to write IdealLoads outdoor-air compare report: {error}"))?;
    std::fs::write(
        compare_dir.join("compare-summary.json"),
        render_outdoor_air_summary_json(context),
    )
    .map_err(|error| format!("failed to write IdealLoads outdoor-air compare summary: {error}"))?;
    std::fs::write(
        compare_dir.join("selected_outputs.json"),
        render_outdoor_air_selected_outputs_json(context),
    )
    .map_err(|error| format!("failed to write IdealLoads outdoor-air selected outputs: {error}"))?;
    std::fs::write(
        compare_dir.join("rust-result-store.json"),
        render_outdoor_air_result_store_json(context),
    )
    .map_err(|error| {
        format!("failed to write IdealLoads outdoor-air Rust result store: {error}")
    })?;
    std::fs::write(
        compare_dir.join("variable-deltas.csv"),
        render_outdoor_air_variable_deltas_csv(context),
    )
    .map_err(|error| format!("failed to write IdealLoads outdoor-air variable deltas: {error}"))?;
    std::fs::write(
        compare_dir.join("first-divergence.csv"),
        render_outdoor_air_first_divergence_csv(context),
    )
    .map_err(|error| {
        format!("failed to write IdealLoads outdoor-air first divergence CSV: {error}")
    })?;
    std::fs::write(
        compare_dir.join("tolerance-failures.csv"),
        render_outdoor_air_tolerance_failures_csv(context),
    )
    .map_err(|error| {
        format!("failed to write IdealLoads outdoor-air tolerance failures CSV: {error}")
    })?;
    std::fs::write(
        compare_dir.join("stage-summary.json"),
        render_outdoor_air_stage_summary_json(context),
    )
    .map_err(|error| format!("failed to write IdealLoads outdoor-air stage summary: {error}"))?;
    Ok(())
}

fn render_markdown(context: &IdealLoadsDiagnosticContext<'_>) -> String {
    let manifest = context.manifest;
    let mut report = String::new();
    report.push_str("# IdealLoads No-OA Sensible Report\n\n");
    report.push_str("## Manifest\n\n");
    report.push_str(&format!("case_id: {}\n", manifest.id));
    report.push_str(&format!(
        "comparison_class: {}\n",
        comparison_class_label(manifest.comparison_class)
    ));
    report.push_str(&format!(
        "conformance_claim: {}\n",
        manifest.conformance_claim
    ));
    report.push_str(&format!("claim_boundary: {}\n", claim_boundary(context)));
    report.push_str(&format!(
        "tolerance_policy: {}\n",
        tolerance_policy(context)
    ));
    report.push_str("timestamp_rule: EnergyPlus timestep ESO timestamps; Rust samples inherit oracle timestep labels\n");
    report.push_str(&format!(
        "zone_demand_source: {}\n",
        ZONE_SYS_ENERGY_DEMAND_INPUT_SOURCE
    ));
    report.push_str(&format!(
        "zone_demand_struct_source: {}::{}\n",
        ZONE_SYS_ENERGY_DEMAND_SOURCE_FILE, ZONE_SYS_ENERGY_DEMAND_SOURCE_STRUCT
    ));
    report.push_str(&format!(
        "zone_demand_heating_field: {}\n",
        ZONE_SYS_ENERGY_DEMAND_HEATING_FIELD
    ));
    report.push_str(&format!(
        "zone_demand_heating_sign_convention: {}\n",
        ZONE_SYS_ENERGY_DEMAND_HEATING_SIGN_CONVENTION
    ));
    report.push_str(&format!(
        "zone_demand_cooling_field: {}\n",
        ZONE_SYS_ENERGY_DEMAND_COOLING_FIELD
    ));
    report.push_str(&format!(
        "zone_demand_cooling_sign_convention: {}\n",
        ZONE_SYS_ENERGY_DEMAND_COOLING_SIGN_CONVENTION
    ));
    report.push_str(&format!(
        "zone_demand_mismatch_classification: {}\n",
        ZONE_SYS_ENERGY_DEMAND_MISMATCH_CLASSIFICATION
    ));
    report.push_str(&format!(
        "zone_demand_fixture_mode: {}\n",
        ZONE_SYS_ENERGY_DEMAND_FIXTURE_MODE
    ));
    report.push_str("zone_state_source: source-order pre-update zone air node state; same-timestamp zone air node outputs are diagnostic proof rows\n");
    report.push_str(&format!(
        "fuel_energy_rate_source: {}\n",
        context.fuel_efficiency.report_source
    ));
    report.push_str(&format!(
        "fuel_efficiency: heating={:.12} cooling={:.12}\n",
        context.fuel_efficiency.heating, context.fuel_efficiency.cooling
    ));
    report.push_str(&format!(
        "energy_source: EnergyPlus ReportPurchasedAir raw rate * TimeStepSysSec summed by OutputProcessor; diagnostic-only fixed_system_substeps={:.0} system_timestep_seconds={:.12} energy_report_interval_seconds={:.12}\n",
        IDEAL_LOADS_NO_OA_ENERGY_SYSTEM_SUBSTEPS,
        context.system_timestep_seconds,
        context.energy_report_interval_seconds
    ));
    report.push_str(&format!(
        "rate_output_source: {}\n",
        IDEAL_LOADS_RATE_OUTPUT_SOURCE
    ));
    report.push_str(&format!(
        "rate_output_timestep_source: {}\n",
        IDEAL_LOADS_RATE_OUTPUT_TIMESTEP_SOURCE
    ));
    report.push_str(&format!(
        "energy_output_timestep_source: {}\n",
        IDEAL_LOADS_ENERGY_OUTPUT_TIMESTEP_SOURCE
    ));
    report.push_str(&format!(
        "energy_output_level_policy: {}\n",
        IDEAL_LOADS_ENERGY_OUTPUT_LEVEL_POLICY
    ));
    report.push_str(&format!(
        "fuel_energy_output_level_policy: {}\n",
        IDEAL_LOADS_FUEL_ENERGY_OUTPUT_LEVEL_POLICY
    ));
    report.push_str(&format!(
        "meter_source: {}; rust_meter_time_series_comparison=true requested_meters={}\n",
        IDEAL_LOADS_FACILITY_METER_REPORT_SOURCE,
        manifest.meters.len()
    ));
    report.push_str(&format!(
        "meter_aggregation_source: {}\n",
        IDEAL_LOADS_METER_AGGREGATION_SOURCE
    ));
    report.push_str(&format!(
        "meter_fuel_energy_binding_source: {}\n",
        IDEAL_LOADS_METER_FUEL_ENERGY_BINDING_SOURCE
    ));
    if !manifest.meters.is_empty() {
        let meter_names = manifest
            .meters
            .iter()
            .map(|meter| meter.name.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        report.push_str(&format!(
            "meter_requests: {}\n",
            markdown_cell(&meter_names)
        ));
    }
    report.push_str("zone_demand_synthetic_rc_model: false\n");
    report.push_str(&format!("oracle_version: {}\n", manifest.oracle_version));
    report.push_str(&format!("zone: {}\n", markdown_cell(&context.zone_name)));
    report.push_str(&format!(
        "zone_air_node: {}\n",
        markdown_cell(&context.zone_air_node_name)
    ));
    if let Some(recirculation_node_name) = context.recirculation_node_name.as_ref() {
        report.push_str(&format!(
            "recirculation_node: {}\n",
            markdown_cell(recirculation_node_name)
        ));
        report.push_str(&format!(
            "recirculation_state_source: {}\n",
            ideal_loads_recirculation_state_source(context.branch)
        ));
    }
    report.push_str(&format!(
        "ideal_loads_system: {}\n",
        markdown_cell(&context.system_name)
    ));
    let purchased_air_source_order = purchased_air_source_order_stages()
        .iter()
        .map(|stage| stage.source_routine)
        .collect::<Vec<_>>()
        .join(" -> ");
    let zone_equipment_dispatch_issues = context.zone_equipment_dispatch.issue_codes();
    let zone_equipment_dispatch_warnings = context.zone_equipment_dispatch.warning_codes();
    report.push_str("source_order_wrapper: ep_runtime::ideal_loads::sim_purchased_air_compat\n");
    report.push_str(&format!(
        "zone_equipment_dispatch_path: {}\n",
        IDEAL_LOADS_ZONE_EQUIPMENT_DISPATCH_PATH
    ));
    report.push_str(&format!(
        "zone_equipment_dispatch_validation: {}\n",
        context.zone_equipment_dispatch.dispatch_status_label()
    ));
    report.push_str(&format!(
        "zone_equipment_conformance_candidate: {}\n",
        context
            .zone_equipment_dispatch
            .conformance_candidate_status_label()
    ));
    report.push_str(&format!(
        "zone_equipment_scope: {}\n",
        context.zone_equipment_dispatch.scope_label()
    ));
    report.push_str(&format!(
        "zone_equipment_dispatch_issues: {}\n",
        label_list_or_none(&zone_equipment_dispatch_issues)
    ));
    report.push_str(&format!(
        "zone_equipment_dispatch_warnings: {}\n",
        label_list_or_none(&zone_equipment_dispatch_warnings)
    ));
    report.push_str(&format!(
        "selected_purchased_air_branch: {}\n",
        context.selected_purchased_air_branch
    ));
    report.push_str(&format!(
        "declared_ideal_loads_branch: {}\n",
        context.declared_ideal_loads_branch
    ));
    report.push_str(&format!(
        "inactive_branches: {}\n",
        context.inactive_branches.join(", ")
    ));
    report.push_str(&format!(
        "source_map_anchor: {}\n",
        IDEAL_LOADS_SOURCE_MAP_ANCHOR
    ));
    report.push_str(&format!(
        "node_output_timestamp_alignment: {}\n",
        IDEAL_LOADS_NODE_OUTPUT_TIMESTAMP_ALIGNMENT
    ));
    report.push_str(&format!(
        "node_output_store_type: {}\n",
        IDEAL_LOADS_NODE_OUTPUT_STORE_TYPE
    ));
    report.push_str(&format!(
        "node_output_state_struct: {}\n",
        IDEAL_LOADS_NODE_OUTPUT_STATE_STRUCT
    ));
    report.push_str(&format!(
        "node_output_update_source: {}\n",
        IDEAL_LOADS_NODE_OUTPUT_UPDATE_SOURCE
    ));
    report.push_str(&format!(
        "node_output_report_source: {}\n",
        IDEAL_LOADS_NODE_OUTPUT_REPORT_SOURCE
    ));
    report.push_str(&format!(
        "purchased_air_source_order: {}\n",
        purchased_air_source_order
    ));
    report.push_str(&format!(
        "supply_node: {}\n\n",
        markdown_cell(&context.supply_node_name)
    ));

    report.push_str("## Result\n\n");
    report.push_str(&format!("status: {}\n", overall_status(context)));
    report.push_str(&format!("series: {}\n", context.rows.len()));
    report.push_str(&format!("samples: {}\n", context.input_trace.sample_count));
    report.push_str(&format!(
        "tolerance_failures: {}\n",
        context
            .rows
            .iter()
            .filter(|row| row.status == SeriesComparisonStatus::Fail)
            .count()
    ));
    report.push_str(&format!("meter_series: {}\n", context.meter_rows.len()));
    report.push_str(&format!(
        "meter_tolerance_failures: {}\n",
        context
            .meter_rows
            .iter()
            .filter(|row| row.status == SeriesComparisonStatus::Fail)
            .count()
    ));
    report.push_str(&format!(
        "mode_counts: off={} deadband={} cooling={} heating={}\n\n",
        context.mode_counts.off,
        context.mode_counts.deadband,
        context.mode_counts.cooling,
        context.mode_counts.heating
    ));

    report.push_str("## Artifacts\n\n");
    report.push_str("- selected_outputs.json\n");
    report.push_str("- rust-result-store.json\n");
    report.push_str("- compare-summary.json\n");
    report.push_str("- compare-report.md\n");
    report.push_str("- variable-deltas.csv\n");
    report.push_str("- first-divergence.csv\n");
    report.push_str("- tolerance-failures.csv\n");
    report.push_str("- stage-summary.json\n\n");

    report.push_str("## Series\n\n");
    report.push_str("| key | variable | level | domain | class | frequency | rust_source | units | unit_match | alignment | expected | observed | compared | max_abs_delta | mean_abs_delta | rmse_delta | max_rel_delta | tolerance | status | first_divergence |\n");
    report.push_str("|---|---|---|---|---|---|---|---|---|---|---:|---:|---:|---:|---:|---:|---:|---|---|---|\n");
    for row in &context.rows {
        report.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {:.12} | {:.12} | {:.12} | {:.12} | {} | {} | {} |\n",
            markdown_cell(&row.key),
            markdown_cell(&row.variable),
            optional_output_level_label(row.level),
            row.domain.map_or("unspecified", evidence_domain_label),
            variable_class_label(row.variable_class),
            output_frequency_label(row.frequency),
            row.rust_source,
            markdown_cell(&row.units),
            row.unit_match(),
            alignment_label(row.alignment),
            row.expected_samples,
            row.observed_samples,
            row.compared_samples,
            row.max_abs_delta,
            row.mean_abs_delta,
            row.rmse_delta,
            row.max_rel_delta,
            tolerance_label(row.tolerance, row.max_rmse_tolerance),
            status_label(row.status),
            first_divergence_label(row.first_divergence.as_ref())
        ));
    }
    if !context.meter_rows.is_empty() {
        report.push_str("\n## Meters\n\n");
        report.push_str("| meter | level | domain | frequency | source | rust_source | units | unit_match | alignment | expected | observed | compared | max_abs_delta | mean_abs_delta | rmse_delta | max_rel_delta | tolerance | status | first_divergence |\n");
        report.push_str("|---|---|---|---|---|---|---|---|---|---:|---:|---:|---:|---:|---:|---:|---|---|---|\n");
        for row in &context.meter_rows {
            report.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {:.12} | {:.12} | {:.12} | {:.12} | {} | {} | {} |\n",
                markdown_cell(&row.name),
                output_level_label(row.level),
                evidence_domain_label(row.domain),
                output_frequency_label(row.frequency),
                source_artifact_label(row.source),
                row.rust_source,
                markdown_cell(&row.units),
                row.unit_match(),
                alignment_label(row.alignment),
                row.expected_samples,
                row.observed_samples,
                row.compared_samples,
                row.max_abs_delta,
                row.mean_abs_delta,
                row.rmse_delta,
                row.max_rel_delta,
                tolerance_label(row.tolerance, row.max_rmse_tolerance),
                status_label(row.status),
                first_divergence_label(row.first_divergence.as_ref())
            ));
        }
    }
    report
}

fn render_outdoor_air_markdown(context: &IdealLoadsOutdoorAirDiagnosticContext<'_>) -> String {
    let manifest = context.manifest;
    let mut report = String::new();
    report.push_str("# IdealLoads Outdoor-Air Design-Flow Report\n\n");
    report.push_str("## Manifest\n\n");
    report.push_str(&format!("case_id: {}\n", manifest.id));
    report.push_str(&format!(
        "comparison_class: {}\n",
        comparison_class_label(manifest.comparison_class)
    ));
    report.push_str(&format!(
        "conformance_claim: {}\n",
        manifest.conformance_claim
    ));
    report.push_str(&format!(
        "claim_boundary: {}\n",
        outdoor_air_claim_boundary(context)
    ));
    report.push_str(&format!(
        "tolerance_policy: {}\n",
        outdoor_air_tolerance_policy(context)
    ));
    report.push_str("timestamp_rule: EnergyPlus timestep ESO timestamps; Rust samples inherit oracle timestep labels\n");
    report.push_str(&format!(
        "node_output_store_type: {}\n",
        IDEAL_LOADS_NODE_OUTPUT_STORE_TYPE
    ));
    report.push_str(&format!(
        "node_output_state_struct: {}\n",
        IDEAL_LOADS_NODE_OUTPUT_STATE_STRUCT
    ));
    report.push_str(&format!(
        "node_output_update_source: {}\n",
        IDEAL_LOADS_NODE_OUTPUT_UPDATE_SOURCE
    ));
    report.push_str(&format!(
        "node_output_report_source: {}\n",
        IDEAL_LOADS_NODE_OUTPUT_REPORT_SOURCE
    ));
    report.push_str(&format!(
        "outdoor_air_source: {}\n",
        outdoor_air_source_description(context)
    ));
    report.push_str("outdoor_air_schedule: blank-always-1.0\n");
    report.push_str(&format!("oracle_version: {}\n", manifest.oracle_version));
    report.push_str(&format!("zone: {}\n", markdown_cell(&context.zone_name)));
    report.push_str(&format!(
        "ideal_loads_system: {}\n",
        markdown_cell(&context.system_name)
    ));
    report.push_str(&format!(
        "outdoor_air_spec: {}\n",
        markdown_cell(&context.outdoor_air_spec_name)
    ));
    report.push_str(&format!(
        "outdoor_air_node: {}\n",
        markdown_cell(&context.outdoor_air_node_name)
    ));
    report.push_str(&format!(
        "standard_air_density_kg_per_m3: {:.15}\n",
        context.standard_air_density_kg_per_m3
    ));
    report.push_str(&format!(
        "design_people_count: {:.15}\n",
        context.design_people_count
    ));
    report.push_str(&format!(
        "zone_floor_area_m2: {:.15}\n",
        context.zone_floor_area_m2
    ));
    report.push_str(&format!("zone_volume_m3: {:.15}\n", context.zone_volume_m3));
    report.push_str(&format!(
        "outdoor_air_flow_per_person_m3_per_s: {:.15}\n",
        context.flow_per_person_m3_per_s
    ));
    report.push_str(&format!(
        "outdoor_air_flow_per_area_m3_per_s: {:.15}\n",
        context.flow_per_area_m3_per_s
    ));
    report.push_str(&format!(
        "outdoor_air_flow_per_zone_m3_per_s: {:.15}\n",
        context.flow_per_zone_m3_per_s
    ));
    report.push_str(&format!(
        "outdoor_air_air_changes_m3_per_s: {:.15}\n",
        context.air_changes_m3_per_s
    ));
    report.push_str(&format!(
        "design_volume_flow_rate_m3_per_s: {:.15}\n",
        context.design_volume_flow_rate_m3_per_s
    ));
    report.push_str(&format!(
        "outdoor_air_mass_flow_rate_kg_per_s: {:.15}\n\n",
        context.outdoor_air_mass_flow_rate_kg_per_s
    ));

    report.push_str("## Result\n\n");
    report.push_str(&format!(
        "status: {}\n",
        outdoor_air_overall_status(context)
    ));
    report.push_str(&format!("series: {}\n", context.rows.len()));
    report.push_str(&format!("samples: {}\n", context.sample_count));
    report.push_str(&format!(
        "tolerance_failures: {}\n\n",
        context
            .rows
            .iter()
            .filter(|row| row.status == SeriesComparisonStatus::Fail)
            .count()
    ));

    report.push_str("## Artifacts\n\n");
    report.push_str("- selected_outputs.json\n");
    report.push_str("- rust-result-store.json\n");
    report.push_str("- compare-summary.json\n");
    report.push_str("- compare-report.md\n");
    report.push_str("- variable-deltas.csv\n");
    report.push_str("- first-divergence.csv\n");
    report.push_str("- tolerance-failures.csv\n");
    report.push_str("- stage-summary.json\n\n");

    report.push_str("## Series\n\n");
    report.push_str("| key | variable | level | domain | class | frequency | rust_source | units | unit_match | alignment | expected | observed | compared | max_abs_delta | mean_abs_delta | rmse_delta | max_rel_delta | tolerance | status | first_divergence |\n");
    report.push_str("|---|---|---|---|---|---|---|---|---|---|---:|---:|---:|---:|---:|---:|---:|---|---|---|\n");
    for row in &context.rows {
        report.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {:.12} | {:.12} | {:.12} | {:.12} | {} | {} | {} |\n",
            markdown_cell(&row.key),
            markdown_cell(&row.variable),
            optional_output_level_label(row.level),
            row.domain.map_or("unspecified", evidence_domain_label),
            variable_class_label(row.variable_class),
            output_frequency_label(row.frequency),
            row.rust_source,
            markdown_cell(&row.units),
            row.unit_match(),
            alignment_label(row.alignment),
            row.expected_samples,
            row.observed_samples,
            row.compared_samples,
            row.max_abs_delta,
            row.mean_abs_delta,
            row.rmse_delta,
            row.max_rel_delta,
            tolerance_label(row.tolerance, row.max_rmse_tolerance),
            status_label(row.status),
            first_divergence_label(row.first_divergence.as_ref())
        ));
    }
    report
}

fn render_outdoor_air_summary_json(context: &IdealLoadsOutdoorAirDiagnosticContext<'_>) -> String {
    let manifest = context.manifest;
    let mut json = String::new();
    json.push_str("{\n");
    json.push_str("  \"schema_version\": 1,\n");
    json.push_str(&format!("  \"case_id\": {},\n", json_string(&manifest.id)));
    json.push_str(&format!(
        "  \"oracle_version\": {},\n",
        json_string(&manifest.oracle_version)
    ));
    json.push_str(&format!(
        "  \"comparison_class\": {},\n",
        json_string(comparison_class_label(manifest.comparison_class))
    ));
    json.push_str(&format!(
        "  \"conformance_claim\": {},\n",
        manifest.conformance_claim
    ));
    json.push_str(&format!(
        "  \"status\": {},\n",
        json_string(outdoor_air_overall_status(context))
    ));
    json.push_str(&format!(
        "  \"tolerance_policy\": {},\n",
        json_string(outdoor_air_tolerance_policy(context))
    ));
    json.push_str("  \"timestamp_rule\": \"EnergyPlus timestep ESO timestamps; Rust samples inherit oracle timestep labels\",\n");
    json.push_str(&format!(
        "  \"node_output_store_type\": {},\n",
        json_string(IDEAL_LOADS_NODE_OUTPUT_STORE_TYPE)
    ));
    json.push_str(&format!(
        "  \"node_output_state_struct\": {},\n",
        json_string(IDEAL_LOADS_NODE_OUTPUT_STATE_STRUCT)
    ));
    json.push_str(&format!(
        "  \"node_output_update_source\": {},\n",
        json_string(IDEAL_LOADS_NODE_OUTPUT_UPDATE_SOURCE)
    ));
    json.push_str(&format!(
        "  \"node_output_report_source\": {},\n",
        json_string(IDEAL_LOADS_NODE_OUTPUT_REPORT_SOURCE)
    ));
    json.push_str(&format!(
        "  \"outdoor_air_source\": {},\n",
        json_string(&outdoor_air_source_description(context))
    ));
    json.push_str(&format!(
        "  \"zone_demand_source\": {},\n",
        json_string(ZONE_SYS_ENERGY_DEMAND_INPUT_SOURCE)
    ));
    json.push_str(&format!(
        "  \"zone_demand_struct_source\": {},\n",
        json_string(&format!(
            "{}::{}",
            ZONE_SYS_ENERGY_DEMAND_SOURCE_FILE, ZONE_SYS_ENERGY_DEMAND_SOURCE_STRUCT
        ))
    ));
    json.push_str(&format!(
        "  \"zone_demand_heating_sign_convention\": {},\n",
        json_string(ZONE_SYS_ENERGY_DEMAND_HEATING_SIGN_CONVENTION)
    ));
    json.push_str(&format!(
        "  \"zone_demand_cooling_sign_convention\": {},\n",
        json_string(ZONE_SYS_ENERGY_DEMAND_COOLING_SIGN_CONVENTION)
    ));
    json.push_str(&format!(
        "  \"zone_demand_mismatch_classification\": {},\n",
        json_string(ZONE_SYS_ENERGY_DEMAND_MISMATCH_CLASSIFICATION)
    ));
    json.push_str(&format!(
        "  \"zone_demand_fixture_mode\": {},\n",
        json_string(ZONE_SYS_ENERGY_DEMAND_FIXTURE_MODE)
    ));
    json.push_str("  \"outdoor_air_schedule\": \"blank-always-1.0\",\n");
    json.push_str(&format!(
        "  \"economizer\": {},\n",
        json_string(outdoor_air_economizer_label(
            context.outdoor_air_economizer_type
        ))
    ));
    json.push_str(&format!(
        "  \"heat_recovery\": {},\n",
        json_string(heat_recovery_label(context.heat_recovery_type))
    ));
    json.push_str(&format!(
        "  \"zone\": {},\n",
        json_string(&context.zone_name)
    ));
    json.push_str(&format!(
        "  \"ideal_loads_system\": {},\n",
        json_string(&context.system_name)
    ));
    json.push_str(&format!(
        "  \"outdoor_air_spec\": {},\n",
        json_string(&context.outdoor_air_spec_name)
    ));
    json.push_str(&format!(
        "  \"outdoor_air_node\": {},\n",
        json_string(&context.outdoor_air_node_name)
    ));
    json.push_str(&format!(
        "  \"standard_air_density_kg_per_m3\": {},\n",
        json_number(context.standard_air_density_kg_per_m3)
    ));
    json.push_str(&format!(
        "  \"design_people_count\": {},\n",
        json_number(context.design_people_count)
    ));
    json.push_str(&format!(
        "  \"zone_floor_area_m2\": {},\n",
        json_number(context.zone_floor_area_m2)
    ));
    json.push_str(&format!(
        "  \"zone_volume_m3\": {},\n",
        json_number(context.zone_volume_m3)
    ));
    json.push_str(&format!(
        "  \"outdoor_air_flow_per_person_m3_per_s\": {},\n",
        json_number(context.flow_per_person_m3_per_s)
    ));
    json.push_str(&format!(
        "  \"outdoor_air_flow_per_area_m3_per_s\": {},\n",
        json_number(context.flow_per_area_m3_per_s)
    ));
    json.push_str(&format!(
        "  \"outdoor_air_flow_per_zone_m3_per_s\": {},\n",
        json_number(context.flow_per_zone_m3_per_s)
    ));
    json.push_str(&format!(
        "  \"outdoor_air_air_changes_m3_per_s\": {},\n",
        json_number(context.air_changes_m3_per_s)
    ));
    json.push_str(&format!(
        "  \"design_volume_flow_rate_m3_per_s\": {},\n",
        json_number(context.design_volume_flow_rate_m3_per_s)
    ));
    json.push_str(&format!(
        "  \"outdoor_air_mass_flow_rate_kg_per_s\": {},\n",
        json_number(context.outdoor_air_mass_flow_rate_kg_per_s)
    ));
    json.push_str(&format!("  \"samples\": {},\n", context.sample_count));
    json.push_str(&format!("  \"series_count\": {},\n", context.rows.len()));
    json.push_str(&format!(
        "  \"tolerance_failures\": {},\n",
        context
            .rows
            .iter()
            .filter(|row| row.status == SeriesComparisonStatus::Fail)
            .count()
    ));
    json.push_str("  \"artifacts\": {\n");
    json.push_str("    \"oracle_selected_outputs_json\": \"selected_outputs.json\",\n");
    json.push_str("    \"rust_result_store_json\": \"rust-result-store.json\",\n");
    json.push_str("    \"compare_summary_json\": \"compare-summary.json\",\n");
    json.push_str("    \"compare_report_md\": \"compare-report.md\",\n");
    json.push_str("    \"variable_deltas_csv\": \"variable-deltas.csv\",\n");
    json.push_str("    \"first_divergence_csv\": \"first-divergence.csv\",\n");
    json.push_str("    \"tolerance_failures_csv\": \"tolerance-failures.csv\",\n");
    json.push_str("    \"stage_summary_json\": \"stage-summary.json\"\n");
    json.push_str("  },\n");
    json.push_str(&format!(
        "  \"domains\": {},\n",
        domain_status_json(&context.rows)
    ));
    json.push_str("  \"series\": [\n");
    for (index, row) in context.rows.iter().enumerate() {
        json.push_str("    ");
        json.push_str(&row_json(row));
        if index + 1 < context.rows.len() {
            json.push(',');
        }
        json.push('\n');
    }
    json.push_str("  ]\n");
    json.push_str("}\n");
    json
}

fn render_outdoor_air_selected_outputs_json(
    context: &IdealLoadsOutdoorAirDiagnosticContext<'_>,
) -> String {
    let mut json = String::new();
    json.push_str("{\n");
    json.push_str("  \"schema_version\": 1,\n");
    json.push_str(&format!(
        "  \"case_id\": {},\n",
        json_string(&context.manifest.id)
    ));
    json.push_str(&format!(
        "  \"eso\": {},\n",
        json_string(&context.baseline.eso.display().to_string())
    ));
    json.push_str("  \"series\": [\n");
    for (index, row) in context.rows.iter().enumerate() {
        json.push_str("    {\n");
        json.push_str(&format!("      \"key\": {},\n", json_string(&row.key)));
        json.push_str(&format!(
            "      \"variable\": {},\n",
            json_string(&row.variable)
        ));
        json.push_str(&format!(
            "      \"frequency\": {},\n",
            json_string(output_frequency_label(row.frequency))
        ));
        json.push_str(&format!(
            "      \"units\": {},\n",
            row.oracle_units
                .as_ref()
                .map_or_else(|| "null".to_string(), |units| json_string(units))
        ));
        json.push_str(&format!("      \"samples\": {}\n", row.expected_samples));
        json.push_str("    }");
        if index + 1 < context.rows.len() {
            json.push(',');
        }
        json.push('\n');
    }
    json.push_str("  ]\n");
    json.push_str("}\n");
    json
}

fn render_outdoor_air_result_store_json(
    context: &IdealLoadsOutdoorAirDiagnosticContext<'_>,
) -> String {
    let mut json = String::new();
    json.push_str("{\n");
    json.push_str("  \"schema_version\": 1,\n");
    json.push_str(&format!(
        "  \"case_id\": {},\n",
        json_string(&context.manifest.id)
    ));
    json.push_str(&format!(
        "  \"series_count\": {},\n",
        context.result_store.series.len()
    ));
    json.push_str(&format!(
        "  \"sample_count\": {},\n",
        context.result_store.sample_count()
    ));
    json.push_str("  \"series\": [\n");
    for (index, series) in context.result_store.series.iter().enumerate() {
        json.push_str("    {\n");
        json.push_str(&format!("      \"handle\": {},\n", series.handle.0));
        json.push_str(&format!("      \"key\": {},\n", json_string(&series.key)));
        json.push_str(&format!(
            "      \"variable_name\": {},\n",
            json_string(&series.variable_name)
        ));
        json.push_str(&format!(
            "      \"units\": {},\n",
            json_string(&series.units)
        ));
        json.push_str("      \"values\": [");
        for (value_index, value) in series.values.iter().enumerate() {
            if value_index > 0 {
                json.push_str(", ");
            }
            json.push_str(&json_number(*value));
        }
        json.push_str("]\n");
        json.push_str("    }");
        if index + 1 < context.result_store.series.len() {
            json.push(',');
        }
        json.push('\n');
    }
    json.push_str("  ]\n");
    json.push_str("}\n");
    json
}

fn render_outdoor_air_variable_deltas_csv(
    context: &IdealLoadsOutdoorAirDiagnosticContext<'_>,
) -> String {
    let mut csv = String::from(
        "key,variable,domain,class,level,expected_samples,observed_samples,compared_samples,max_abs_delta,mean_abs_delta,rmse_delta,max_rel_delta,status\n",
    );
    for row in &context.rows {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            csv_cell(&row.key),
            csv_cell(&row.variable),
            row.domain.map_or("unspecified", evidence_domain_label),
            variable_class_label(row.variable_class),
            optional_output_level_label(row.level),
            row.expected_samples,
            row.observed_samples,
            row.compared_samples,
            json_number(row.max_abs_delta),
            json_number(row.mean_abs_delta),
            json_number(row.rmse_delta),
            json_number(row.max_rel_delta),
            status_label(row.status)
        ));
    }
    csv
}

fn render_outdoor_air_first_divergence_csv(
    context: &IdealLoadsOutdoorAirDiagnosticContext<'_>,
) -> String {
    let mut csv =
        String::from("key,variable,index,timestamp,kind,expected,observed,abs_delta,rel_delta\n");
    for row in &context.rows {
        let Some(divergence) = row.first_divergence.as_ref() else {
            continue;
        };
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{}\n",
            csv_cell(&row.key),
            csv_cell(&row.variable),
            divergence.index,
            csv_cell(divergence.timestamp.as_deref().unwrap_or("")),
            divergence_kind_label(divergence.kind),
            optional_number_csv(divergence.expected),
            optional_number_csv(divergence.observed),
            optional_number_csv(divergence.abs_delta),
            optional_number_csv(divergence.rel_delta)
        ));
    }
    csv
}

fn render_outdoor_air_tolerance_failures_csv(
    context: &IdealLoadsOutdoorAirDiagnosticContext<'_>,
) -> String {
    let mut csv = String::from(
        "key,variable,domain,class,level,max_abs_delta,rmse_delta,max_abs_tolerance,max_rmse_tolerance,status\n",
    );
    for row in &context.rows {
        if row.status == SeriesComparisonStatus::Pass {
            continue;
        }
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{}\n",
            csv_cell(&row.key),
            csv_cell(&row.variable),
            row.domain.map_or("unspecified", evidence_domain_label),
            variable_class_label(row.variable_class),
            optional_output_level_label(row.level),
            json_number(row.max_abs_delta),
            json_number(row.rmse_delta),
            json_number(row.tolerance.absolute),
            row.max_rmse_tolerance
                .map_or_else(|| "null".to_string(), json_number),
            status_label(row.status)
        ));
    }
    csv
}

fn render_outdoor_air_stage_summary_json(
    context: &IdealLoadsOutdoorAirDiagnosticContext<'_>,
) -> String {
    let mut json = String::new();
    json.push_str("{\n");
    json.push_str("  \"schema_version\": 1,\n");
    json.push_str(&format!(
        "  \"case_id\": {},\n",
        json_string(&context.manifest.id)
    ));
    json.push_str(&format!("  \"branch\": {},\n", json_string(context.branch)));
    json.push_str("  \"outdoor_air\": true,\n");
    json.push_str(&format!(
        "  \"node_output_store_type\": {},\n",
        json_string(IDEAL_LOADS_NODE_OUTPUT_STORE_TYPE)
    ));
    json.push_str(&format!(
        "  \"node_output_state_struct\": {},\n",
        json_string(IDEAL_LOADS_NODE_OUTPUT_STATE_STRUCT)
    ));
    json.push_str(&format!(
        "  \"node_output_update_source\": {},\n",
        json_string(IDEAL_LOADS_NODE_OUTPUT_UPDATE_SOURCE)
    ));
    json.push_str(&format!(
        "  \"node_output_report_source\": {},\n",
        json_string(IDEAL_LOADS_NODE_OUTPUT_REPORT_SOURCE)
    ));
    json.push_str(&format!(
        "  \"outdoor_air_method\": {},\n",
        json_string(outdoor_air_method_label(context.outdoor_air_method))
    ));
    json.push_str("  \"outdoor_air_schedule\": \"blank-always-1.0\",\n");
    json.push_str(&format!(
        "  \"economizer\": {},\n",
        json_string(outdoor_air_economizer_label(
            context.outdoor_air_economizer_type
        ))
    ));
    json.push_str(&format!(
        "  \"heat_recovery\": {},\n",
        json_string(heat_recovery_label(context.heat_recovery_type))
    ));
    json.push_str(&format!(
        "  \"zone_demand_source\": {},\n",
        json_string(ZONE_SYS_ENERGY_DEMAND_INPUT_SOURCE)
    ));
    json.push_str(&format!(
        "  \"zone_demand_struct_source\": {},\n",
        json_string(&format!(
            "{}::{}",
            ZONE_SYS_ENERGY_DEMAND_SOURCE_FILE, ZONE_SYS_ENERGY_DEMAND_SOURCE_STRUCT
        ))
    ));
    json.push_str(&format!(
        "  \"zone_demand_heating_sign_convention\": {},\n",
        json_string(ZONE_SYS_ENERGY_DEMAND_HEATING_SIGN_CONVENTION)
    ));
    json.push_str(&format!(
        "  \"zone_demand_cooling_sign_convention\": {},\n",
        json_string(ZONE_SYS_ENERGY_DEMAND_COOLING_SIGN_CONVENTION)
    ));
    json.push_str(&format!(
        "  \"zone_demand_mismatch_classification\": {},\n",
        json_string(ZONE_SYS_ENERGY_DEMAND_MISMATCH_CLASSIFICATION)
    ));
    json.push_str(&format!(
        "  \"zone_demand_fixture_mode\": {},\n",
        json_string(ZONE_SYS_ENERGY_DEMAND_FIXTURE_MODE)
    ));
    json.push_str("  \"humidity_control_conformance\": false,\n");
    json.push_str("  \"finite_limit_conformance\": false,\n");
    json.push_str(&format!(
        "  \"outdoor_air_spec\": {},\n",
        json_string(&context.outdoor_air_spec_name)
    ));
    json.push_str(&format!(
        "  \"outdoor_air_node\": {},\n",
        json_string(&context.outdoor_air_node_name)
    ));
    json.push_str(&format!(
        "  \"standard_air_density_kg_per_m3\": {},\n",
        json_number(context.standard_air_density_kg_per_m3)
    ));
    json.push_str(&format!(
        "  \"design_people_count\": {},\n",
        json_number(context.design_people_count)
    ));
    json.push_str(&format!(
        "  \"zone_floor_area_m2\": {},\n",
        json_number(context.zone_floor_area_m2)
    ));
    json.push_str(&format!(
        "  \"zone_volume_m3\": {},\n",
        json_number(context.zone_volume_m3)
    ));
    json.push_str(&format!(
        "  \"outdoor_air_flow_per_person_m3_per_s\": {},\n",
        json_number(context.flow_per_person_m3_per_s)
    ));
    json.push_str(&format!(
        "  \"outdoor_air_flow_per_area_m3_per_s\": {},\n",
        json_number(context.flow_per_area_m3_per_s)
    ));
    json.push_str(&format!(
        "  \"outdoor_air_flow_per_zone_m3_per_s\": {},\n",
        json_number(context.flow_per_zone_m3_per_s)
    ));
    json.push_str(&format!(
        "  \"outdoor_air_air_changes_m3_per_s\": {},\n",
        json_number(context.air_changes_m3_per_s)
    ));
    json.push_str(&format!(
        "  \"design_volume_flow_rate_m3_per_s\": {},\n",
        json_number(context.design_volume_flow_rate_m3_per_s)
    ));
    json.push_str(&format!(
        "  \"outdoor_air_mass_flow_rate_kg_per_s\": {},\n",
        json_number(context.outdoor_air_mass_flow_rate_kg_per_s)
    ));
    json.push_str("  \"stages\": [\n");
    let stages = ideal_loads_zone_equipment_stages();
    for (index, stage) in stages.iter().enumerate() {
        json.push_str("    {\n");
        json.push_str(&format!(
            "      \"stage_name\": {},\n",
            json_string(stage.stage_name)
        ));
        json.push_str(&format!(
            "      \"source_file\": {},\n",
            json_string(stage.source_file)
        ));
        json.push_str(&format!(
            "      \"source_routine\": {}\n",
            json_string(stage.source_routine)
        ));
        json.push_str("    }");
        if index + 1 < stages.len() {
            json.push(',');
        }
        json.push('\n');
    }
    json.push_str("  ],\n");
    json.push_str("  \"purchased_air_stages\": [\n");
    let purchased_air_stages = purchased_air_source_order_stages();
    for (index, stage) in purchased_air_stages.iter().enumerate() {
        json.push_str("    {\n");
        json.push_str(&format!(
            "      \"stage_name\": {},\n",
            json_string(stage.stage_name)
        ));
        json.push_str(&format!(
            "      \"source_file\": {},\n",
            json_string(stage.source_file)
        ));
        json.push_str(&format!(
            "      \"source_routine\": {},\n",
            json_string(stage.source_routine)
        ));
        json.push_str(&format!(
            "      \"rust_equivalent\": {}\n",
            json_string(stage.rust_equivalent)
        ));
        json.push_str("    }");
        if index + 1 < purchased_air_stages.len() {
            json.push(',');
        }
        json.push('\n');
    }
    json.push_str("  ]\n");
    json.push_str("}\n");
    json
}

fn render_summary_json(context: &IdealLoadsDiagnosticContext<'_>) -> String {
    let manifest = context.manifest;
    let zone_equipment_dispatch_issues = context.zone_equipment_dispatch.issue_codes();
    let zone_equipment_dispatch_warnings = context.zone_equipment_dispatch.warning_codes();
    let mut json = String::new();
    json.push_str("{\n");
    json.push_str("  \"schema_version\": 1,\n");
    json.push_str(&format!("  \"case_id\": {},\n", json_string(&manifest.id)));
    json.push_str(&format!(
        "  \"oracle_version\": {},\n",
        json_string(&manifest.oracle_version)
    ));
    json.push_str(&format!(
        "  \"comparison_class\": {},\n",
        json_string(comparison_class_label(manifest.comparison_class))
    ));
    json.push_str(&format!(
        "  \"conformance_claim\": {},\n",
        manifest.conformance_claim
    ));
    json.push_str(&format!(
        "  \"status\": {},\n",
        json_string(overall_status(context))
    ));
    json.push_str(&format!(
        "  \"tolerance_policy\": {},\n",
        json_string(tolerance_policy(context))
    ));
    json.push_str("  \"timestamp_rule\": \"EnergyPlus timestep ESO timestamps; Rust samples inherit oracle timestep labels\",\n");
    json.push_str(&format!(
        "  \"zone_demand_source\": {},\n",
        json_string(ZONE_SYS_ENERGY_DEMAND_INPUT_SOURCE)
    ));
    json.push_str(&format!(
        "  \"zone_demand_struct_source\": {},\n",
        json_string(&format!(
            "{}::{}",
            ZONE_SYS_ENERGY_DEMAND_SOURCE_FILE, ZONE_SYS_ENERGY_DEMAND_SOURCE_STRUCT
        ))
    ));
    json.push_str(&format!(
        "  \"zone_demand_heating_field\": {},\n",
        json_string(ZONE_SYS_ENERGY_DEMAND_HEATING_FIELD)
    ));
    json.push_str(&format!(
        "  \"zone_demand_heating_sign_convention\": {},\n",
        json_string(ZONE_SYS_ENERGY_DEMAND_HEATING_SIGN_CONVENTION)
    ));
    json.push_str(&format!(
        "  \"zone_demand_cooling_field\": {},\n",
        json_string(ZONE_SYS_ENERGY_DEMAND_COOLING_FIELD)
    ));
    json.push_str(&format!(
        "  \"zone_demand_cooling_sign_convention\": {},\n",
        json_string(ZONE_SYS_ENERGY_DEMAND_COOLING_SIGN_CONVENTION)
    ));
    json.push_str(&format!(
        "  \"zone_demand_mismatch_classification\": {},\n",
        json_string(ZONE_SYS_ENERGY_DEMAND_MISMATCH_CLASSIFICATION)
    ));
    json.push_str(&format!(
        "  \"zone_demand_fixture_mode\": {},\n",
        json_string(ZONE_SYS_ENERGY_DEMAND_FIXTURE_MODE)
    ));
    json.push_str("  \"zone_state_source\": \"source-order pre-update zone air node state; same-timestamp zone air node outputs are diagnostic proof rows\",\n");
    json.push_str(&format!(
        "  \"source_map_anchor\": {},\n",
        json_string(IDEAL_LOADS_SOURCE_MAP_ANCHOR)
    ));
    json.push_str(&format!(
        "  \"node_output_timestamp_alignment\": {},\n",
        json_string(IDEAL_LOADS_NODE_OUTPUT_TIMESTAMP_ALIGNMENT)
    ));
    json.push_str(&format!(
        "  \"node_output_store_type\": {},\n",
        json_string(IDEAL_LOADS_NODE_OUTPUT_STORE_TYPE)
    ));
    json.push_str(&format!(
        "  \"node_output_state_struct\": {},\n",
        json_string(IDEAL_LOADS_NODE_OUTPUT_STATE_STRUCT)
    ));
    json.push_str(&format!(
        "  \"node_output_update_source\": {},\n",
        json_string(IDEAL_LOADS_NODE_OUTPUT_UPDATE_SOURCE)
    ));
    json.push_str(&format!(
        "  \"node_output_report_source\": {},\n",
        json_string(IDEAL_LOADS_NODE_OUTPUT_REPORT_SOURCE)
    ));
    json.push_str(&format!(
        "  \"selected_purchased_air_branch\": {},\n",
        json_string(context.selected_purchased_air_branch)
    ));
    json.push_str(&format!(
        "  \"declared_ideal_loads_branch\": {},\n",
        json_string(context.declared_ideal_loads_branch)
    ));
    json.push_str(&format!(
        "  \"inactive_branches\": {},\n",
        json_string_array(&context.inactive_branches)
    ));
    json.push_str(&format!(
        "  \"zone_equipment_dispatch_path\": {},\n",
        json_string(IDEAL_LOADS_ZONE_EQUIPMENT_DISPATCH_PATH)
    ));
    json.push_str(&format!(
        "  \"zone_equipment_dispatch_validation\": {},\n",
        json_string(context.zone_equipment_dispatch.dispatch_status_label())
    ));
    json.push_str(&format!(
        "  \"zone_equipment_conformance_candidate\": {},\n",
        json_string(
            context
                .zone_equipment_dispatch
                .conformance_candidate_status_label()
        )
    ));
    json.push_str(&format!(
        "  \"zone_equipment_scope\": {},\n",
        json_string(context.zone_equipment_dispatch.scope_label())
    ));
    json.push_str(&format!(
        "  \"zone_equipment_dispatch_issues\": {},\n",
        json_string_array(&zone_equipment_dispatch_issues)
    ));
    json.push_str(&format!(
        "  \"zone_equipment_dispatch_warnings\": {},\n",
        json_string_array(&zone_equipment_dispatch_warnings)
    ));
    json.push_str(&format!(
        "  \"fuel_energy_rate_source\": {},\n",
        json_string(context.fuel_efficiency.report_source)
    ));
    json.push_str(&format!(
        "  \"heating_fuel_efficiency\": {},\n",
        json_number(context.fuel_efficiency.heating)
    ));
    json.push_str(&format!(
        "  \"cooling_fuel_efficiency\": {},\n",
        json_number(context.fuel_efficiency.cooling)
    ));
    json.push_str(&format!(
        "  \"fuel_energy_rate_rust_source\": {},\n",
        json_string(context.fuel_efficiency.rate_rust_source)
    ));
    json.push_str(&format!(
        "  \"fuel_energy_rust_source\": {},\n",
        json_string(context.fuel_efficiency.energy_rust_source)
    ));
    json.push_str("  \"energy_source\": \"EnergyPlus ReportPurchasedAir raw rate * TimeStepSysSec summed by OutputProcessor; diagnostic-only fixed 8-substep fixture branch\",\n");
    json.push_str(&format!(
        "  \"rate_output_source\": {},\n",
        json_string(IDEAL_LOADS_RATE_OUTPUT_SOURCE)
    ));
    json.push_str(&format!(
        "  \"rate_output_timestep_source\": {},\n",
        json_string(IDEAL_LOADS_RATE_OUTPUT_TIMESTEP_SOURCE)
    ));
    json.push_str(&format!(
        "  \"energy_output_timestep_source\": {},\n",
        json_string(IDEAL_LOADS_ENERGY_OUTPUT_TIMESTEP_SOURCE)
    ));
    json.push_str(&format!(
        "  \"energy_output_level_policy\": {},\n",
        json_string(IDEAL_LOADS_ENERGY_OUTPUT_LEVEL_POLICY)
    ));
    json.push_str(&format!(
        "  \"fuel_energy_output_level_policy\": {},\n",
        json_string(IDEAL_LOADS_FUEL_ENERGY_OUTPUT_LEVEL_POLICY)
    ));
    json.push_str(&format!(
        "  \"meter_source\": {},\n",
        json_string(IDEAL_LOADS_FACILITY_METER_REPORT_SOURCE)
    ));
    json.push_str(&format!(
        "  \"meter_aggregation_source\": {},\n",
        json_string(IDEAL_LOADS_METER_AGGREGATION_SOURCE)
    ));
    json.push_str(&format!(
        "  \"meter_fuel_energy_binding_source\": {},\n",
        json_string(IDEAL_LOADS_METER_FUEL_ENERGY_BINDING_SOURCE)
    ));
    json.push_str("  \"rust_meter_time_series_comparison\": true,\n");
    json.push_str(&format!(
        "  \"requested_meter_count\": {},\n",
        manifest.meters.len()
    ));
    json.push_str("  \"requested_meters\": [\n");
    for (index, meter) in manifest.meters.iter().enumerate() {
        json.push_str(&format!(
            "    {{\"name\": {}, \"frequency\": {}, \"source\": {}, \"domain\": {}, \"level\": {}}}",
            json_string(&meter.name),
            json_string(output_frequency_label(meter.frequency)),
            json_string(source_artifact_label(meter.source)),
            json_string(evidence_domain_label(meter.domain)),
            json_string(optional_output_level_label(Some(meter.level)))
        ));
        if index + 1 < manifest.meters.len() {
            json.push(',');
        }
        json.push('\n');
    }
    json.push_str("  ],\n");
    json.push_str(&format!(
        "  \"meter_series_count\": {},\n",
        context.meter_rows.len()
    ));
    json.push_str(&format!(
        "  \"meter_tolerance_failures\": {},\n",
        context
            .meter_rows
            .iter()
            .filter(|row| row.status == SeriesComparisonStatus::Fail)
            .count()
    ));
    json.push_str("  \"meter_series\": [\n");
    for (index, row) in context.meter_rows.iter().enumerate() {
        json.push_str("    ");
        json.push_str(&meter_row_json(row));
        if index + 1 < context.meter_rows.len() {
            json.push(',');
        }
        json.push('\n');
    }
    json.push_str("  ],\n");
    json.push_str(&format!(
        "  \"system_timestep_substeps\": {},\n",
        json_number(IDEAL_LOADS_NO_OA_ENERGY_SYSTEM_SUBSTEPS)
    ));
    json.push_str(&format!(
        "  \"system_timestep_seconds\": {},\n",
        json_number(context.system_timestep_seconds)
    ));
    json.push_str(&format!(
        "  \"energy_report_interval_seconds\": {},\n",
        json_number(context.energy_report_interval_seconds)
    ));
    json.push_str("  \"zone_demand_synthetic_rc_model\": false,\n");
    json.push_str(&format!(
        "  \"zone\": {},\n",
        json_string(&context.zone_name)
    ));
    json.push_str(&format!(
        "  \"zone_air_node\": {},\n",
        json_string(&context.zone_air_node_name)
    ));
    json.push_str(&format!(
        "  \"recirculation_node\": {},\n",
        context
            .recirculation_node_name
            .as_ref()
            .map_or_else(|| "null".to_string(), |name| json_string(name))
    ));
    if context.recirculation_node_name.is_some() {
        json.push_str(&format!(
            "  \"recirculation_state_source\": {},\n",
            json_string(ideal_loads_recirculation_state_source(context.branch))
        ));
    }
    json.push_str(&format!(
        "  \"ideal_loads_system\": {},\n",
        json_string(&context.system_name)
    ));
    json.push_str(&format!(
        "  \"supply_node\": {},\n",
        json_string(&context.supply_node_name)
    ));
    json.push_str(&format!(
        "  \"samples\": {},\n",
        context.input_trace.sample_count
    ));
    json.push_str(&format!("  \"series_count\": {},\n", context.rows.len()));
    json.push_str(&format!(
        "  \"tolerance_failures\": {},\n",
        context
            .rows
            .iter()
            .filter(|row| row.status == SeriesComparisonStatus::Fail)
            .count()
    ));
    json.push_str(&format!(
        "  \"mode_counts\": {{\"off\": {}, \"deadband\": {}, \"cooling\": {}, \"heating\": {}}},\n",
        context.mode_counts.off,
        context.mode_counts.deadband,
        context.mode_counts.cooling,
        context.mode_counts.heating
    ));
    json.push_str("  \"artifacts\": {\n");
    json.push_str("    \"oracle_selected_outputs_json\": \"selected_outputs.json\",\n");
    json.push_str("    \"rust_result_store_json\": \"rust-result-store.json\",\n");
    json.push_str("    \"compare_summary_json\": \"compare-summary.json\",\n");
    json.push_str("    \"compare_report_md\": \"compare-report.md\",\n");
    json.push_str("    \"variable_deltas_csv\": \"variable-deltas.csv\",\n");
    json.push_str("    \"first_divergence_csv\": \"first-divergence.csv\",\n");
    json.push_str("    \"tolerance_failures_csv\": \"tolerance-failures.csv\",\n");
    json.push_str("    \"stage_summary_json\": \"stage-summary.json\"\n");
    json.push_str("  },\n");
    json.push_str(&format!(
        "  \"domains\": {},\n",
        domain_status_json(&context.rows)
    ));
    json.push_str("  \"series\": [\n");
    for (index, row) in context.rows.iter().enumerate() {
        json.push_str("    ");
        json.push_str(&row_json(row));
        if index + 1 < context.rows.len() {
            json.push(',');
        }
        json.push('\n');
    }
    json.push_str("  ]\n");
    json.push_str("}\n");
    json
}

fn json_string_array(values: &[&str]) -> String {
    let mut json = String::from("[");
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            json.push_str(", ");
        }
        json.push_str(&json_string(value));
    }
    json.push(']');
    json
}

fn label_list_or_none(values: &[&str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(", ")
    }
}

fn row_json(row: &IdealLoadsDiagnosticRow) -> String {
    format!(
        concat!(
            "{{\"key\": {}, \"variable\": {}, \"level\": {}, \"domain\": {}, ",
            "\"class\": {}, \"frequency\": {}, \"source\": {}, \"rust_source\": {}, ",
            "\"units\": {}, \"oracle_units\": {}, \"unit_match\": {}, ",
            "\"alignment\": {}, \"expected_samples\": {}, \"observed_samples\": {}, ",
            "\"compared_samples\": {}, \"max_abs_delta\": {}, \"mean_abs_delta\": {}, ",
            "\"rmse_delta\": {}, \"max_rel_delta\": {}, \"max_abs_tolerance\": {}, ",
            "\"max_rel_tolerance\": {}, \"max_rmse_tolerance\": {}, \"status\": {}, ",
            "\"first_divergence\": {}}}"
        ),
        json_string(&row.key),
        json_string(&row.variable),
        json_string(optional_output_level_label(row.level)),
        json_string(row.domain.map_or("unspecified", evidence_domain_label)),
        json_string(variable_class_label(row.variable_class)),
        json_string(output_frequency_label(row.frequency)),
        json_string(source_artifact_label(row.source)),
        json_string(row.rust_source),
        json_string(&row.units),
        row.oracle_units
            .as_ref()
            .map_or_else(|| "null".to_string(), |units| json_string(units)),
        row.unit_match(),
        json_string(alignment_label(row.alignment)),
        row.expected_samples,
        row.observed_samples,
        row.compared_samples,
        json_number(row.max_abs_delta),
        json_number(row.mean_abs_delta),
        json_number(row.rmse_delta),
        json_number(row.max_rel_delta),
        json_number(row.tolerance.absolute),
        json_number(row.tolerance.relative),
        row.max_rmse_tolerance
            .map_or_else(|| "null".to_string(), json_number),
        json_string(status_label(row.status)),
        first_divergence_json(row.first_divergence.as_ref())
    )
}

fn meter_row_json(row: &IdealLoadsMeterDiagnosticRow) -> String {
    format!(
        concat!(
            "{{\"name\": {}, \"level\": {}, \"domain\": {}, \"frequency\": {}, ",
            "\"source\": {}, \"rust_source\": {}, \"units\": {}, \"oracle_units\": {}, ",
            "\"unit_match\": {}, \"alignment\": {}, \"expected_samples\": {}, ",
            "\"observed_samples\": {}, \"compared_samples\": {}, \"max_abs_delta\": {}, ",
            "\"mean_abs_delta\": {}, \"rmse_delta\": {}, \"max_rel_delta\": {}, ",
            "\"max_abs_tolerance\": {}, \"max_rel_tolerance\": {}, ",
            "\"max_rmse_tolerance\": {}, \"status\": {}, \"first_divergence\": {}}}"
        ),
        json_string(&row.name),
        json_string(output_level_label(row.level)),
        json_string(evidence_domain_label(row.domain)),
        json_string(output_frequency_label(row.frequency)),
        json_string(source_artifact_label(row.source)),
        json_string(row.rust_source),
        json_string(&row.units),
        row.oracle_units
            .as_ref()
            .map_or_else(|| "null".to_string(), |units| json_string(units)),
        row.unit_match(),
        json_string(alignment_label(row.alignment)),
        row.expected_samples,
        row.observed_samples,
        row.compared_samples,
        json_number(row.max_abs_delta),
        json_number(row.mean_abs_delta),
        json_number(row.rmse_delta),
        json_number(row.max_rel_delta),
        json_number(row.tolerance.absolute),
        json_number(row.tolerance.relative),
        row.max_rmse_tolerance
            .map_or_else(|| "null".to_string(), json_number),
        json_string(status_label(row.status)),
        first_divergence_json(row.first_divergence.as_ref())
    )
}

fn render_selected_outputs_json(context: &IdealLoadsDiagnosticContext<'_>) -> String {
    let mut json = String::new();
    json.push_str("{\n");
    json.push_str("  \"schema_version\": 1,\n");
    json.push_str(&format!(
        "  \"case_id\": {},\n",
        json_string(&context.manifest.id)
    ));
    json.push_str(&format!(
        "  \"eso\": {},\n",
        json_string(&context.baseline.eso.display().to_string())
    ));
    json.push_str("  \"series\": [\n");
    for (index, row) in context.rows.iter().enumerate() {
        json.push_str("    {\n");
        json.push_str(&format!("      \"key\": {},\n", json_string(&row.key)));
        json.push_str(&format!(
            "      \"variable\": {},\n",
            json_string(&row.variable)
        ));
        json.push_str(&format!(
            "      \"frequency\": {},\n",
            json_string(output_frequency_label(row.frequency))
        ));
        json.push_str(&format!(
            "      \"units\": {},\n",
            row.oracle_units
                .as_ref()
                .map_or_else(|| "null".to_string(), |units| json_string(units))
        ));
        json.push_str(&format!("      \"samples\": {}\n", row.expected_samples));
        json.push_str("    }");
        if index + 1 < context.rows.len() {
            json.push(',');
        }
        json.push('\n');
    }
    json.push_str("  ]\n");
    json.push_str("}\n");
    json
}

fn render_result_store_json(context: &IdealLoadsDiagnosticContext<'_>) -> String {
    let mut json = String::new();
    json.push_str("{\n");
    json.push_str("  \"schema_version\": 1,\n");
    json.push_str(&format!(
        "  \"case_id\": {},\n",
        json_string(&context.manifest.id)
    ));
    json.push_str(&format!(
        "  \"series_count\": {},\n",
        context.result_store.series.len()
    ));
    json.push_str(&format!(
        "  \"sample_count\": {},\n",
        context.result_store.sample_count()
    ));
    json.push_str("  \"series\": [\n");
    for (index, series) in context.result_store.series.iter().enumerate() {
        json.push_str("    {\n");
        json.push_str(&format!("      \"handle\": {},\n", series.handle.0));
        json.push_str(&format!("      \"key\": {},\n", json_string(&series.key)));
        json.push_str(&format!(
            "      \"variable_name\": {},\n",
            json_string(&series.variable_name)
        ));
        json.push_str(&format!(
            "      \"units\": {},\n",
            json_string(&series.units)
        ));
        json.push_str("      \"values\": [");
        for (value_index, value) in series.values.iter().enumerate() {
            if value_index > 0 {
                json.push_str(", ");
            }
            json.push_str(&json_number(*value));
        }
        json.push_str("]\n");
        json.push_str("    }");
        if index + 1 < context.result_store.series.len() {
            json.push(',');
        }
        json.push('\n');
    }
    json.push_str("  ]\n");
    json.push_str("}\n");
    json
}

fn render_variable_deltas_csv(context: &IdealLoadsDiagnosticContext<'_>) -> String {
    let mut csv = String::from(
        "key,variable,domain,class,level,expected_samples,observed_samples,compared_samples,max_abs_delta,mean_abs_delta,rmse_delta,max_rel_delta,status\n",
    );
    for row in &context.rows {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            csv_cell(&row.key),
            csv_cell(&row.variable),
            row.domain.map_or("unspecified", evidence_domain_label),
            variable_class_label(row.variable_class),
            optional_output_level_label(row.level),
            row.expected_samples,
            row.observed_samples,
            row.compared_samples,
            json_number(row.max_abs_delta),
            json_number(row.mean_abs_delta),
            json_number(row.rmse_delta),
            json_number(row.max_rel_delta),
            status_label(row.status)
        ));
    }
    csv
}

fn render_first_divergence_csv(context: &IdealLoadsDiagnosticContext<'_>) -> String {
    let mut csv =
        String::from("key,variable,index,timestamp,kind,expected,observed,abs_delta,rel_delta\n");
    for row in &context.rows {
        let Some(divergence) = row.first_divergence.as_ref() else {
            continue;
        };
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{}\n",
            csv_cell(&row.key),
            csv_cell(&row.variable),
            divergence.index,
            csv_cell(divergence.timestamp.as_deref().unwrap_or("")),
            divergence_kind_label(divergence.kind),
            optional_number_csv(divergence.expected),
            optional_number_csv(divergence.observed),
            optional_number_csv(divergence.abs_delta),
            optional_number_csv(divergence.rel_delta)
        ));
    }
    csv
}

fn render_tolerance_failures_csv(context: &IdealLoadsDiagnosticContext<'_>) -> String {
    let mut csv = String::from(
        "key,variable,domain,class,level,max_abs_delta,rmse_delta,max_abs_tolerance,max_rmse_tolerance,status\n",
    );
    for row in &context.rows {
        if row.status == SeriesComparisonStatus::Pass {
            continue;
        }
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{}\n",
            csv_cell(&row.key),
            csv_cell(&row.variable),
            row.domain.map_or("unspecified", evidence_domain_label),
            variable_class_label(row.variable_class),
            optional_output_level_label(row.level),
            json_number(row.max_abs_delta),
            json_number(row.rmse_delta),
            json_number(row.tolerance.absolute),
            row.max_rmse_tolerance
                .map_or_else(|| "null".to_string(), json_number),
            status_label(row.status)
        ));
    }
    csv
}

fn render_stage_summary_json(context: &IdealLoadsDiagnosticContext<'_>) -> String {
    let zone_equipment_dispatch_issues = context.zone_equipment_dispatch.issue_codes();
    let zone_equipment_dispatch_warnings = context.zone_equipment_dispatch.warning_codes();
    let mut json = String::new();
    json.push_str("{\n");
    json.push_str("  \"schema_version\": 1,\n");
    json.push_str(&format!(
        "  \"case_id\": {},\n",
        json_string(&context.manifest.id)
    ));
    json.push_str(&format!("  \"branch\": {},\n", json_string(context.branch)));
    json.push_str(&format!(
        "  \"selected_purchased_air_branch\": {},\n",
        json_string(context.selected_purchased_air_branch)
    ));
    json.push_str(&format!(
        "  \"declared_ideal_loads_branch\": {},\n",
        json_string(context.declared_ideal_loads_branch)
    ));
    json.push_str(&format!(
        "  \"inactive_branches\": {},\n",
        json_string_array(&context.inactive_branches)
    ));
    json.push_str(&format!(
        "  \"zone_equipment_dispatch_path\": {},\n",
        json_string(IDEAL_LOADS_ZONE_EQUIPMENT_DISPATCH_PATH)
    ));
    json.push_str(&format!(
        "  \"zone_equipment_dispatch_validation\": {},\n",
        json_string(context.zone_equipment_dispatch.dispatch_status_label())
    ));
    json.push_str(&format!(
        "  \"zone_equipment_conformance_candidate\": {},\n",
        json_string(
            context
                .zone_equipment_dispatch
                .conformance_candidate_status_label()
        )
    ));
    json.push_str(&format!(
        "  \"zone_equipment_scope\": {},\n",
        json_string(context.zone_equipment_dispatch.scope_label())
    ));
    json.push_str(&format!(
        "  \"zone_equipment_dispatch_issues\": {},\n",
        json_string_array(&zone_equipment_dispatch_issues)
    ));
    json.push_str(&format!(
        "  \"zone_equipment_dispatch_warnings\": {},\n",
        json_string_array(&zone_equipment_dispatch_warnings)
    ));
    json.push_str(&format!(
        "  \"source_map_anchor\": {},\n",
        json_string(IDEAL_LOADS_SOURCE_MAP_ANCHOR)
    ));
    json.push_str(&format!(
        "  \"node_output_timestamp_alignment\": {},\n",
        json_string(IDEAL_LOADS_NODE_OUTPUT_TIMESTAMP_ALIGNMENT)
    ));
    json.push_str(&format!(
        "  \"node_output_store_type\": {},\n",
        json_string(IDEAL_LOADS_NODE_OUTPUT_STORE_TYPE)
    ));
    json.push_str(&format!(
        "  \"node_output_state_struct\": {},\n",
        json_string(IDEAL_LOADS_NODE_OUTPUT_STATE_STRUCT)
    ));
    json.push_str(&format!(
        "  \"node_output_update_source\": {},\n",
        json_string(IDEAL_LOADS_NODE_OUTPUT_UPDATE_SOURCE)
    ));
    json.push_str(&format!(
        "  \"node_output_report_source\": {},\n",
        json_string(IDEAL_LOADS_NODE_OUTPUT_REPORT_SOURCE)
    ));
    json.push_str("  \"outdoor_air\": false,\n");
    json.push_str("  \"economizer\": \"NoEconomizer\",\n");
    json.push_str("  \"heat_recovery\": \"None\",\n");
    json.push_str("  \"humidity_control_conformance\": false,\n");
    json.push_str(&format!(
        "  \"finite_limit_conformance\": {},\n",
        context.manifest.conformance_claim && context.branch == "no-oa-finite-limit-sensible"
    ));
    json.push_str(&format!(
        "  \"heating_fuel_efficiency\": {},\n",
        json_number(context.fuel_efficiency.heating)
    ));
    json.push_str(&format!(
        "  \"cooling_fuel_efficiency\": {},\n",
        json_number(context.fuel_efficiency.cooling)
    ));
    json.push_str(&format!(
        "  \"fuel_energy_rate_source\": {},\n",
        json_string(context.fuel_efficiency.report_source)
    ));
    json.push_str(&format!(
        "  \"rate_output_source\": {},\n",
        json_string(IDEAL_LOADS_RATE_OUTPUT_SOURCE)
    ));
    json.push_str(&format!(
        "  \"rate_output_timestep_source\": {},\n",
        json_string(IDEAL_LOADS_RATE_OUTPUT_TIMESTEP_SOURCE)
    ));
    json.push_str(&format!(
        "  \"energy_output_timestep_source\": {},\n",
        json_string(IDEAL_LOADS_ENERGY_OUTPUT_TIMESTEP_SOURCE)
    ));
    json.push_str(&format!(
        "  \"energy_output_level_policy\": {},\n",
        json_string(IDEAL_LOADS_ENERGY_OUTPUT_LEVEL_POLICY)
    ));
    json.push_str(&format!(
        "  \"fuel_energy_output_level_policy\": {},\n",
        json_string(IDEAL_LOADS_FUEL_ENERGY_OUTPUT_LEVEL_POLICY)
    ));
    json.push_str(&format!(
        "  \"meter_aggregation_source\": {},\n",
        json_string(IDEAL_LOADS_METER_AGGREGATION_SOURCE)
    ));
    json.push_str(&format!(
        "  \"meter_fuel_energy_binding_source\": {},\n",
        json_string(IDEAL_LOADS_METER_FUEL_ENERGY_BINDING_SOURCE)
    ));
    json.push_str("  \"meter_time_series_comparison\": true,\n");
    json.push_str(&format!(
        "  \"meter_series_count\": {},\n",
        context.meter_rows.len()
    ));
    json.push_str(&format!(
        "  \"zone_demand_source\": {},\n",
        json_string(ZONE_SYS_ENERGY_DEMAND_INPUT_SOURCE)
    ));
    json.push_str(&format!(
        "  \"zone_demand_struct_source\": {},\n",
        json_string(&format!(
            "{}::{}",
            ZONE_SYS_ENERGY_DEMAND_SOURCE_FILE, ZONE_SYS_ENERGY_DEMAND_SOURCE_STRUCT
        ))
    ));
    json.push_str(&format!(
        "  \"zone_demand_heating_field\": {},\n",
        json_string(ZONE_SYS_ENERGY_DEMAND_HEATING_FIELD)
    ));
    json.push_str(&format!(
        "  \"zone_demand_heating_sign_convention\": {},\n",
        json_string(ZONE_SYS_ENERGY_DEMAND_HEATING_SIGN_CONVENTION)
    ));
    json.push_str(&format!(
        "  \"zone_demand_cooling_field\": {},\n",
        json_string(ZONE_SYS_ENERGY_DEMAND_COOLING_FIELD)
    ));
    json.push_str(&format!(
        "  \"zone_demand_cooling_sign_convention\": {},\n",
        json_string(ZONE_SYS_ENERGY_DEMAND_COOLING_SIGN_CONVENTION)
    ));
    json.push_str(&format!(
        "  \"zone_demand_mismatch_classification\": {},\n",
        json_string(ZONE_SYS_ENERGY_DEMAND_MISMATCH_CLASSIFICATION)
    ));
    json.push_str(&format!(
        "  \"zone_demand_fixture_mode\": {},\n",
        json_string(ZONE_SYS_ENERGY_DEMAND_FIXTURE_MODE)
    ));
    json.push_str("  \"zone_state_source\": \"source-order pre-update zone air node state\",\n");
    json.push_str(&format!(
        "  \"zone_air_node\": {},\n",
        json_string(&context.zone_air_node_name)
    ));
    json.push_str(&format!(
        "  \"recirculation_node\": {},\n",
        context
            .recirculation_node_name
            .as_ref()
            .map_or_else(|| "null".to_string(), |name| json_string(name))
    ));
    if context.recirculation_node_name.is_some() {
        json.push_str(&format!(
            "  \"recirculation_state_source\": {},\n",
            json_string(ideal_loads_recirculation_state_source(context.branch))
        ));
    }
    json.push_str("  \"zone_demand_synthetic_rc_model\": false,\n");
    json.push_str("  \"stages\": [\n");
    let stages = ideal_loads_zone_equipment_stages();
    for (index, stage) in stages.iter().enumerate() {
        json.push_str("    {\n");
        json.push_str(&format!(
            "      \"stage_name\": {},\n",
            json_string(stage.stage_name)
        ));
        json.push_str(&format!(
            "      \"source_file\": {},\n",
            json_string(stage.source_file)
        ));
        json.push_str(&format!(
            "      \"source_routine\": {}\n",
            json_string(stage.source_routine)
        ));
        json.push_str("    }");
        if index + 1 < stages.len() {
            json.push(',');
        }
        json.push('\n');
    }
    json.push_str("  ],\n");
    json.push_str("  \"purchased_air_stages\": [\n");
    let purchased_air_stages = purchased_air_source_order_stages();
    for (index, stage) in purchased_air_stages.iter().enumerate() {
        json.push_str("    {\n");
        json.push_str(&format!(
            "      \"stage_name\": {},\n",
            json_string(stage.stage_name)
        ));
        json.push_str(&format!(
            "      \"source_file\": {},\n",
            json_string(stage.source_file)
        ));
        json.push_str(&format!(
            "      \"source_routine\": {},\n",
            json_string(stage.source_routine)
        ));
        json.push_str(&format!(
            "      \"rust_equivalent\": {}\n",
            json_string(stage.rust_equivalent)
        ));
        json.push_str("    }");
        if index + 1 < purchased_air_stages.len() {
            json.push(',');
        }
        json.push('\n');
    }
    json.push_str("  ]\n");
    json.push_str("}\n");
    json
}

fn domain_status_json(rows: &[IdealLoadsDiagnosticRow]) -> String {
    let mut domains: BTreeMap<&'static str, (usize, usize)> = BTreeMap::new();
    for row in rows {
        let entry = domains
            .entry(row.domain.map_or("unspecified", evidence_domain_label))
            .or_insert((0, 0));
        entry.0 += 1;
        if row.status == SeriesComparisonStatus::Fail {
            entry.1 += 1;
        }
    }
    let mut json = String::from("{");
    for (index, (domain, (series, failures))) in domains.iter().enumerate() {
        if index > 0 {
            json.push_str(", ");
        }
        json.push_str(&format!(
            "{}: {{\"series\": {}, \"failures\": {}, \"status\": {}}}",
            json_string(domain),
            series,
            failures,
            json_string(if *failures == 0 { "pass" } else { "fail" })
        ));
    }
    json.push('}');
    json
}

impl IdealLoadsDiagnosticRow {
    fn unit_match(&self) -> bool {
        self.oracle_units
            .as_ref()
            .is_some_and(|oracle_units| oracle_units == &self.units)
    }
}

impl IdealLoadsMeterDiagnosticRow {
    fn unit_match(&self) -> bool {
        self.oracle_units
            .as_ref()
            .is_some_and(|oracle_units| oracle_units == &self.units)
    }
}

fn overall_status(context: &IdealLoadsDiagnosticContext<'_>) -> &'static str {
    let conformance_rows = context
        .rows
        .iter()
        .filter(|row| row.level == Some(OutputLevel::Conformance))
        .collect::<Vec<_>>();
    if conformance_rows.is_empty() && !context.manifest.conformance_claim {
        "diagnostic"
    } else if conformance_rows.is_empty() {
        "fail"
    } else if conformance_rows
        .iter()
        .all(|row| row.status == SeriesComparisonStatus::Pass)
    {
        "pass"
    } else {
        "fail"
    }
}

fn outdoor_air_overall_status(context: &IdealLoadsOutdoorAirDiagnosticContext<'_>) -> &'static str {
    let conformance_rows = context
        .rows
        .iter()
        .filter(|row| row.level == Some(OutputLevel::Conformance))
        .collect::<Vec<_>>();
    if conformance_rows.is_empty() && !context.manifest.conformance_claim {
        "diagnostic"
    } else if conformance_rows.is_empty() {
        "fail"
    } else if conformance_rows
        .iter()
        .all(|row| row.status == SeriesComparisonStatus::Pass)
    {
        "pass"
    } else {
        "fail"
    }
}

fn tolerance_policy(context: &IdealLoadsDiagnosticContext<'_>) -> &'static str {
    if context.manifest.conformance_claim {
        "conformance-gate"
    } else {
        "diagnostic-draft"
    }
}

fn outdoor_air_tolerance_policy(
    context: &IdealLoadsOutdoorAirDiagnosticContext<'_>,
) -> &'static str {
    if context.manifest.conformance_claim {
        "conformance-gate"
    } else {
        "diagnostic-draft"
    }
}

fn claim_boundary(context: &IdealLoadsDiagnosticContext<'_>) -> &'static str {
    if context.manifest.conformance_claim && context.branch == "no-oa-finite-limit-sensible" {
        "conformance no-OA finite-limit sensible IdealLoads branch for declared variables only"
    } else if context.constant_shr_conformance_claim {
        "conformance no-OA ConstantSensibleHeatRatio cooling IdealLoads branch for declared variables only"
    } else if context.constant_supply_humidity_cooling_conformance_claim {
        "conformance no-OA ConstantSupplyHumidityRatio cooling IdealLoads branch for declared variables only"
    } else if context.constant_supply_humidity_heating_conformance_claim {
        "conformance no-OA ConstantSupplyHumidityRatio heating IdealLoads branch for declared variables only"
    } else if context.humidistat_dehumidification_conformance_claim {
        "conformance no-OA Humidistat dehumidification IdealLoads branch for declared variables only"
    } else if context.humidistat_humidification_conformance_claim {
        "conformance no-OA Humidistat humidification IdealLoads branch for declared variables only"
    } else if context.manifest.conformance_claim {
        "conformance no-OA/no-limit sensible IdealLoads branch for declared variables only"
    } else if context.branch == "no-oa-finite-limit-sensible" {
        "diagnostic-only no-OA finite-limit sensible IdealLoads branch"
    } else {
        "diagnostic-only no-OA/no-limit sensible IdealLoads branch"
    }
}

fn record_mode(counts: &mut IdealLoadsModeCounts, mode: IdealLoadsSensibleMode) {
    match mode {
        IdealLoadsSensibleMode::Off => counts.off += 1,
        IdealLoadsSensibleMode::Deadband => counts.deadband += 1,
        IdealLoadsSensibleMode::Cooling => counts.cooling += 1,
        IdealLoadsSensibleMode::Heating => counts.heating += 1,
    }
}

fn optional_output_level_label(level: Option<OutputLevel>) -> &'static str {
    level.map_or("unspecified", output_level_label)
}

fn alignment_label(alignment: SeriesAlignment) -> &'static str {
    match alignment {
        SeriesAlignment::Index => "index",
        SeriesAlignment::Timestamp => "timestamp",
    }
}

fn status_label(status: SeriesComparisonStatus) -> &'static str {
    match status {
        SeriesComparisonStatus::Pass => "pass",
        SeriesComparisonStatus::Fail => "fail",
    }
}

fn tolerance_label(tolerance: Tolerance, max_rmse: Option<f64>) -> String {
    format!(
        "abs={} rel={} rmse={}",
        json_number(tolerance.absolute),
        json_number(tolerance.relative),
        max_rmse.map_or_else(|| "none".to_string(), json_number)
    )
}

fn first_divergence_label(divergence: Option<&ep_compare::SeriesDivergenceV2>) -> String {
    let Some(divergence) = divergence else {
        return "none".to_string();
    };
    format!(
        "{} index={} timestamp={} expected={} observed={} abs_delta={}",
        divergence_kind_label(divergence.kind),
        divergence.index,
        divergence.timestamp.as_deref().unwrap_or("none"),
        optional_number_csv(divergence.expected),
        optional_number_csv(divergence.observed),
        optional_number_csv(divergence.abs_delta)
    )
}

fn first_divergence_json(divergence: Option<&ep_compare::SeriesDivergenceV2>) -> String {
    let Some(divergence) = divergence else {
        return "null".to_string();
    };
    format!(
        "{{\"index\": {}, \"timestamp\": {}, \"kind\": {}, \"expected\": {}, \"observed\": {}, \"abs_delta\": {}, \"rel_delta\": {}}}",
        divergence.index,
        divergence
            .timestamp
            .as_ref()
            .map_or_else(|| "null".to_string(), |value| json_string(value)),
        json_string(divergence_kind_label(divergence.kind)),
        optional_number_json(divergence.expected),
        optional_number_json(divergence.observed),
        optional_number_json(divergence.abs_delta),
        optional_number_json(divergence.rel_delta)
    )
}

fn divergence_kind_label(kind: SeriesDivergenceKind) -> &'static str {
    match kind {
        SeriesDivergenceKind::Tolerance => "tolerance",
        SeriesDivergenceKind::MissingExpectedSample => "missing-expected-sample",
        SeriesDivergenceKind::MissingObservedSample => "missing-observed-sample",
    }
}

fn optional_number_json(value: Option<f64>) -> String {
    value.map_or_else(|| "null".to_string(), json_number)
}

fn optional_number_csv(value: Option<f64>) -> String {
    value.map_or_else(|| "".to_string(), json_number)
}

fn csv_cell(value: &str) -> String {
    if value.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

fn unsupported_features_label(features: &[IdealLoadsUnsupportedFeature]) -> String {
    if features.is_empty() {
        return "none".to_string();
    }
    features
        .iter()
        .map(|feature| match feature {
            IdealLoadsUnsupportedFeature::OutdoorAir => "outdoor-air",
            IdealLoadsUnsupportedFeature::DemandControlledVentilation => "dcv",
            IdealLoadsUnsupportedFeature::Economizer => "economizer",
            IdealLoadsUnsupportedFeature::HeatRecovery => "heat-recovery",
            IdealLoadsUnsupportedFeature::HeatingLimit => "heating-limit",
            IdealLoadsUnsupportedFeature::CoolingLimit => "cooling-limit",
            IdealLoadsUnsupportedFeature::Humidification => "humidification",
            IdealLoadsUnsupportedFeature::Dehumidification => "dehumidification",
            IdealLoadsUnsupportedFeature::UnresolvedHeatingLimit => "unresolved-heating-limit",
            IdealLoadsUnsupportedFeature::UnresolvedCoolingLimit => "unresolved-cooling-limit",
        })
        .collect::<Vec<_>>()
        .join(",")
}
