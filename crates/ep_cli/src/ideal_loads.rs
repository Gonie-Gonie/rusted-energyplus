use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use ep_compare::{
    SeriesAlignment, SeriesComparisonStatus, SeriesDivergenceKind, SeriesSample, Tolerance,
    compare_series_samples_v2, load_eso_time_series,
};
use ep_compiler::compile_raw_model;
use ep_conformance::{
    ComparisonClass, ConformanceCase, EvidenceDomain, OutputFrequency, OutputLevel, OutputRequest,
    SourceArtifact, VariableClass,
};
use ep_model::{
    DemandControlledVentilationType, DesignSpecificationOutdoorAirMethod, HeatRecoveryType,
    IdealLoadsAirSystem, IdealLoadsLimit, OutdoorAirEconomizerType, OutputHandle, SimulationModel,
};
use ep_raw_model::load_epjson_file;
use ep_runtime::{
    IdealLoadsOutdoorAirContext, IdealLoadsOutdoorAirNodeState, IdealLoadsOutdoorAirSensibleResult,
    IdealLoadsSensibleLimitContext, IdealLoadsSensibleMode, IdealLoadsSensibleResult,
    IdealLoadsUnsupportedFeature, IdealLoadsZoneState, OutputSeries, ResultStore,
    ZONE_IDEAL_LOADS_MIXED_AIR_HUMIDITY_RATIO, ZONE_IDEAL_LOADS_MIXED_AIR_TEMPERATURE,
    ZONE_IDEAL_LOADS_OUTDOOR_AIR_MASS_FLOW_RATE,
    ZONE_IDEAL_LOADS_OUTDOOR_AIR_SENSIBLE_COOLING_RATE,
    ZONE_IDEAL_LOADS_OUTDOOR_AIR_SENSIBLE_HEATING_RATE,
    ZONE_IDEAL_LOADS_OUTDOOR_AIR_STANDARD_DENSITY_VOLUME_FLOW_RATE,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_RATE, ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_RATE,
    ZONE_IDEAL_LOADS_ZONE_SENSIBLE_COOLING_RATE, ZONE_IDEAL_LOADS_ZONE_SENSIBLE_HEATING_RATE,
    ZONE_IDEAL_LOADS_ZONE_TOTAL_COOLING_RATE, ZONE_IDEAL_LOADS_ZONE_TOTAL_HEATING_RATE,
    ZONE_THERMOSTAT_COOLING_SETPOINT_TEMPERATURE, ZONE_THERMOSTAT_HEATING_SETPOINT_TEMPERATURE,
    ZoneSysEnergyDemand, calc_no_oa_no_limit_sensible_compat,
    calc_no_oa_sensible_with_limits_compat, calc_outdoor_air_sensible_report_rates_compat,
    calc_scheduled_outdoor_air_mass_flow_rate_kg_per_s, classify_no_oa_no_limit_sensible_subset,
    classify_no_oa_sensible_subset, ideal_loads_zone_equipment_stages,
    supply_node_update_from_result,
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
    zone_name: String,
    zone_air_node_name: String,
    system_name: String,
    supply_node_name: String,
    rows: Vec<IdealLoadsDiagnosticRow>,
    result_store: ResultStore,
    input_trace: IdealLoadsInputTrace,
    mode_counts: IdealLoadsModeCounts,
}

struct IdealLoadsOutdoorAirDiagnosticContext<'a> {
    manifest: &'a ConformanceCase,
    baseline: &'a BaselineSummary,
    branch: &'static str,
    zone_name: String,
    system_name: String,
    outdoor_air_spec_name: String,
    outdoor_air_node_name: String,
    standard_air_density_kg_per_m3: f64,
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
    active_demand: LoadedSeries,
    heating_demand: LoadedSeries,
    cooling_demand: LoadedSeries,
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
    Ok(())
}

fn validate_outdoor_air_design_flow_manifest(manifest: &ConformanceCase) -> Result<(), String> {
    if manifest.comparison_class != ComparisonClass::DiagnosticOnly {
        return Err(format!(
            "IdealLoads outdoor-air design-flow report requires diagnostic-only, got {}",
            comparison_class_label(manifest.comparison_class)
        ));
    }
    if manifest.conformance_claim {
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
        if output.level != Some(OutputLevel::Diagnostic) {
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
                | ZONE_IDEAL_LOADS_MIXED_AIR_TEMPERATURE
                | ZONE_IDEAL_LOADS_MIXED_AIR_HUMIDITY_RATIO
        ) {
            return Err(format!(
                "IdealLoads outdoor-air design-flow report cannot produce Rust series for {}",
                output.variable
            ));
        }
    }
    Ok(())
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

    let site =
        model.typed.site.as_ref().ok_or_else(|| {
            "IdealLoads outdoor-air diagnostics require Site:Location".to_string()
        })?;
    let standard_air_density_kg_per_m3 =
        IdealLoadsSensibleLimitContext::from_site_elevation_m(site.elevation_m)
            .ok_or_else(|| {
                format!(
                    "failed to derive EnergyPlus StdRhoAir from site elevation {}",
                    site.elevation_m
                )
            })?
            .standard_air_density_kg_per_m3;
    let outdoor_air_context = IdealLoadsOutdoorAirContext {
        design_people_count: 0.0,
        zone_floor_area_m2: 0.0,
        zone_volume_m3: 0.0,
    };
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
            true,
        ));
    }

    let mut rows = Vec::new();
    let mut result_store = ResultStore::new();
    for (output, expected) in manifest.outputs.iter().zip(expected_series.iter()) {
        let (rust_source, units, observed_values) = outdoor_air_observed_values(
            output,
            outdoor_air_mass_flow_rate_kg_per_s,
            design_volume_flow_rate_m3_per_s,
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
        outdoor_air_node_name: outdoor_air_node_name.0.clone(),
        standard_air_density_kg_per_m3,
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
    if method != DesignSpecificationOutdoorAirMethod::FlowPerZone {
        return Err(
            "IdealLoads outdoor-air design-flow diagnostic currently requires Flow/Zone"
                .to_string(),
        );
    }
    if system.demand_controlled_ventilation_type != DemandControlledVentilationType::None {
        return Err("IdealLoads outdoor-air design-flow diagnostic excludes DCV".to_string());
    }
    if system.outdoor_air_economizer_type != OutdoorAirEconomizerType::NoEconomizer {
        return Err(
            "IdealLoads outdoor-air design-flow diagnostic excludes economizer".to_string(),
        );
    }
    if system.heat_recovery_type != HeatRecoveryType::None {
        return Err(
            "IdealLoads outdoor-air design-flow diagnostic excludes heat recovery".to_string(),
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

fn outdoor_air_observed_values(
    output: &OutputRequest,
    outdoor_air_mass_flow_rate_kg_per_s: f64,
    design_volume_flow_rate_m3_per_s: f64,
    sensible_results: &[IdealLoadsOutdoorAirSensibleResult],
    expected_samples: usize,
) -> Result<(&'static str, &'static str, Vec<f64>), String> {
    match output.variable.as_str() {
        ZONE_IDEAL_LOADS_OUTDOOR_AIR_MASS_FLOW_RATE => Ok((
            "rust-ideal-loads-outdoor-air-design-flow",
            "kg/s",
            vec![outdoor_air_mass_flow_rate_kg_per_s; expected_samples],
        )),
        ZONE_IDEAL_LOADS_OUTDOOR_AIR_STANDARD_DENSITY_VOLUME_FLOW_RATE => Ok((
            "rust-ideal-loads-outdoor-air-design-flow",
            "m3/s",
            vec![design_volume_flow_rate_m3_per_s; expected_samples],
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
        _ => Err(format!(
            "IdealLoads outdoor-air design-flow report cannot produce Rust series for {} / {}",
            output.key, output.variable
        )),
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

    let boundary = if manifest.conformance_claim {
        classify_no_oa_no_limit_sensible_subset(system)
    } else {
        classify_no_oa_sensible_subset(system)
    };
    if !boundary.is_supported() {
        return Err(format!(
            "IdealLoads system is outside no-OA sensible subset: {}",
            unsupported_features_label(&boundary.unsupported_features)
        ));
    }

    let input_trace = load_input_trace(&baseline.eso, &zone.name.0, &zone_air_node.name.0)?;
    let (rows, result_store, mode_counts) = evaluate_rows(
        manifest,
        &model,
        &baseline.eso,
        &input_trace,
        &zone.name.0,
        &zone_air_node.name.0,
        &system.name.0,
        &supply_node.name.0,
    )?;

    let zone_name = zone.name.0.clone();
    let zone_air_node_name = zone_air_node.name.0.clone();
    let system_name = system.name.0.clone();
    let supply_node_name = supply_node.name.0.clone();
    let branch = ideal_loads_sensible_branch(system);

    Ok(IdealLoadsDiagnosticContext {
        manifest,
        baseline,
        branch,
        zone_name,
        zone_air_node_name,
        system_name,
        supply_node_name,
        rows,
        result_store,
        input_trace,
        mode_counts,
    })
}

fn load_input_trace(
    eso: &Path,
    zone_name: &str,
    zone_air_node_name: &str,
) -> Result<IdealLoadsInputTrace, String> {
    let zone_node_temperature = load_series(eso, zone_air_node_name, SYSTEM_NODE_TEMPERATURE)?;
    let zone_node_humidity_ratio =
        load_series(eso, zone_air_node_name, SYSTEM_NODE_HUMIDITY_RATIO)?;
    let active_demand = load_series(eso, zone_name, ZONE_SYSTEM_PREDICTED_SETPOINT_LOAD)?;
    let heating_demand = load_series(eso, zone_name, ZONE_SYSTEM_PREDICTED_HEATING_LOAD)?;
    let cooling_demand = load_series(eso, zone_name, ZONE_SYSTEM_PREDICTED_COOLING_LOAD)?;
    let sample_count = [
        zone_node_temperature.samples.len(),
        zone_node_humidity_ratio.samples.len(),
        active_demand.samples.len(),
        heating_demand.samples.len(),
        cooling_demand.samples.len(),
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
        active_demand,
        heating_demand,
        cooling_demand,
    })
}

fn load_series(eso: &Path, key: &str, variable: &str) -> Result<LoadedSeries, String> {
    let series = load_eso_time_series(eso, key, variable)
        .map_err(|error| format!("failed to load ESO series {key}/{variable}: {error}"))?;
    Ok(LoadedSeries {
        units: series.metadata.units,
        samples: run_period_samples(series.samples),
    })
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
    input_trace: &IdealLoadsInputTrace,
    zone_name: &str,
    zone_air_node_name: &str,
    system_name: &str,
    supply_node_name: &str,
) -> Result<
    (
        Vec<IdealLoadsDiagnosticRow>,
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
    let mut calc_results = Vec::with_capacity(input_trace.sample_count);
    let mut mode_counts = IdealLoadsModeCounts::default();
    for index in 0..input_trace.sample_count {
        // CalcPurchAirLoads sees the zone node before the same-timestamp node
        // output row is updated, so transition samples use the previous row.
        let calc_zone_state_index = index.saturating_sub(1);
        let zone_temperature =
            input_trace.zone_node_temperature.samples[calc_zone_state_index].value;
        let zone_humidity_ratio =
            input_trace.zone_node_humidity_ratio.samples[calc_zone_state_index].value;
        let active_demand = input_trace.active_demand.samples[index].value;
        let heating_demand = active_demand.max(0.0);
        let cooling_demand = active_demand.min(0.0);
        let zone_state = IdealLoadsZoneState {
            air_temperature_c: zone_temperature,
            air_humidity_ratio: zone_humidity_ratio,
        };
        let demand = ZoneSysEnergyDemand::sensible_only(zone.id, heating_demand, cooling_demand);
        let result = calc_ideal_loads_sensible_compat(system, zone_state, demand, limit_context);
        record_mode(&mut mode_counts, result.mode);
        let _node_update = supply_node_update_from_result(supply_node.id, result);
        calc_results.push(result);
    }

    let result_source = rust_result_source(system);

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

    let timestamps = input_trace
        .zone_node_temperature
        .samples
        .iter()
        .take(input_trace.sample_count)
        .map(|sample| sample.timestamp.clone())
        .collect::<Vec<_>>();

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

    Ok((rows, result_store, mode_counts))
}

fn calc_ideal_loads_sensible_compat(
    system: &IdealLoadsAirSystem,
    zone_state: IdealLoadsZoneState,
    demand: ZoneSysEnergyDemand,
    limit_context: IdealLoadsSensibleLimitContext,
) -> IdealLoadsSensibleResult {
    if uses_finite_limits(system) {
        calc_no_oa_sensible_with_limits_compat(system, zone_state, demand, true, limit_context)
    } else {
        calc_no_oa_no_limit_sensible_compat(system, zone_state, demand, true)
    }
}

fn ideal_loads_limit_context(
    model: &SimulationModel,
    system: &IdealLoadsAirSystem,
) -> Result<IdealLoadsSensibleLimitContext, String> {
    if !uses_finite_limits(system) {
        return Ok(IdealLoadsSensibleLimitContext::default());
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

fn ideal_loads_sensible_branch(system: &IdealLoadsAirSystem) -> &'static str {
    if uses_finite_limits(system) {
        "no-oa-finite-limit-sensible"
    } else {
        "no-oa-no-limit-sensible"
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
    value: fn(IdealLoadsSensibleResult) -> f64,
) {
    observed_by_variable.insert(
        (key.to_string(), variable.to_string()),
        ObservedSeries::new(source, units, results.iter().copied().map(value).collect()),
    );
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
    report.push_str("zone_demand_source: EnergyPlus Zone System Predicted Sensible Load to Setpoint output split into active heat/cool ZoneSysEnergyDemand inputs\n");
    report.push_str("zone_state_source: source-order pre-update zone air node state; same-timestamp zone air node outputs are diagnostic proof rows\n");
    report.push_str("zone_demand_synthetic_rc_model: false\n");
    report.push_str(&format!("oracle_version: {}\n", manifest.oracle_version));
    report.push_str(&format!("zone: {}\n", markdown_cell(&context.zone_name)));
    report.push_str(&format!(
        "zone_air_node: {}\n",
        markdown_cell(&context.zone_air_node_name)
    ));
    report.push_str(&format!(
        "ideal_loads_system: {}\n",
        markdown_cell(&context.system_name)
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
    report.push_str("claim_boundary: diagnostic-only IdealLoads outdoor-air Flow/Zone mass, standard-density volume, sensible report rates, and mixed-air state\n");
    report.push_str(&format!(
        "tolerance_policy: {}\n",
        outdoor_air_tolerance_policy(context)
    ));
    report.push_str("timestamp_rule: EnergyPlus timestep ESO timestamps; Rust samples inherit oracle timestep labels\n");
    report.push_str("outdoor_air_source: DesignSpecification:OutdoorAir Flow/Zone with blank OA schedule, EnergyPlus StdRhoAir from Site:Location, and source-order zone/OA/mixed-air state proof rows\n");
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
    json.push_str("  \"outdoor_air_source\": \"DesignSpecification:OutdoorAir Flow/Zone with blank OA schedule, EnergyPlus StdRhoAir from Site:Location, and source-order zone/OA/mixed-air state proof rows\",\n");
    json.push_str("  \"outdoor_air_schedule\": \"blank-always-1.0\",\n");
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
    json.push_str("  \"outdoor_air_method\": \"Flow/Zone\",\n");
    json.push_str("  \"outdoor_air_schedule\": \"blank-always-1.0\",\n");
    json.push_str("  \"economizer\": \"NoEconomizer\",\n");
    json.push_str("  \"heat_recovery\": \"None\",\n");
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
    json.push_str("  ]\n");
    json.push_str("}\n");
    json
}

fn render_summary_json(context: &IdealLoadsDiagnosticContext<'_>) -> String {
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
        json_string(overall_status(context))
    ));
    json.push_str(&format!(
        "  \"tolerance_policy\": {},\n",
        json_string(tolerance_policy(context))
    ));
    json.push_str("  \"timestamp_rule\": \"EnergyPlus timestep ESO timestamps; Rust samples inherit oracle timestep labels\",\n");
    json.push_str("  \"zone_demand_source\": \"EnergyPlus Zone System Predicted Sensible Load to Setpoint output split into active heat/cool ZoneSysEnergyDemand inputs\",\n");
    json.push_str("  \"zone_state_source\": \"source-order pre-update zone air node state; same-timestamp zone air node outputs are diagnostic proof rows\",\n");
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
    let mut json = String::new();
    json.push_str("{\n");
    json.push_str("  \"schema_version\": 1,\n");
    json.push_str(&format!(
        "  \"case_id\": {},\n",
        json_string(&context.manifest.id)
    ));
    json.push_str(&format!("  \"branch\": {},\n", json_string(context.branch)));
    json.push_str("  \"outdoor_air\": false,\n");
    json.push_str("  \"economizer\": \"NoEconomizer\",\n");
    json.push_str("  \"heat_recovery\": \"None\",\n");
    json.push_str("  \"humidity_control_conformance\": false,\n");
    json.push_str("  \"zone_demand_source\": \"EnergyPlus Zone System Predicted Sensible Load to Setpoint output split into active heat/cool ZoneSysEnergyDemand inputs\",\n");
    json.push_str("  \"zone_state_source\": \"source-order pre-update zone air node state\",\n");
    json.push_str(&format!(
        "  \"zone_air_node\": {},\n",
        json_string(&context.zone_air_node_name)
    ));
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
    if context.rows.is_empty() {
        return "fail";
    }
    if context
        .rows
        .iter()
        .all(|row| row.status == SeriesComparisonStatus::Pass)
    {
        "diagnostic"
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
    if context.manifest.conformance_claim {
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
