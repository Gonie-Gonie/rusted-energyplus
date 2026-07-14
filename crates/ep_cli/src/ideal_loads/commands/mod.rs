//! IdealLoads CLI command entry points and their artifact summary.

use super::*;

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

pub(crate) fn generate_ideal_loads_no_oa_sensible_report(
    case_path: &Path,
    manifest: &ConformanceCase,
    oracle_root: &Path,
    output_root: &Path,
) -> Result<IdealLoadsDiagnosticReportSummary, String> {
    let total_start = Instant::now();
    validate_manifest(manifest)?;

    let case_output_dir = output_root.join(&manifest.id);
    let oracle_output_dir = case_output_dir.join("oracle");
    let compare_dir = case_output_dir.join("compare");

    let baseline =
        generate_conformance_baseline_in_dir(case_path, manifest, oracle_root, &oracle_output_dir)?;
    let (series_count, compared_samples, tolerance_failures_count, tolerance_policy, status) = {
        let rust_context_start = Instant::now();
        let context = build_context(manifest, &baseline)?;
        let rust_context_wall_seconds = elapsed_seconds_since(rust_context_start);
        let rust_artifact_start = Instant::now();
        let timing = ReportTimingSummary {
            baseline: baseline.timing,
            rust_context_wall_seconds,
            rust_artifact_write_wall_seconds: 0.0,
            rust_compare_report_wall_seconds: 0.0,
            total_wall_seconds: 0.0,
        };
        write_artifacts(&compare_dir, &context, &timing)?;
        let rust_artifact_write_wall_seconds = elapsed_seconds_since(rust_artifact_start);
        let timing = ReportTimingSummary {
            baseline: baseline.timing,
            rust_context_wall_seconds,
            rust_artifact_write_wall_seconds,
            rust_compare_report_wall_seconds: rust_context_wall_seconds
                + rust_artifact_write_wall_seconds,
            total_wall_seconds: elapsed_seconds_since(total_start),
        };
        write_artifacts(&compare_dir, &context, &timing)?;

        let tolerance_failures_count = tolerance_failures_count(&context);
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
