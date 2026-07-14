//! Outdoor-air report artifact writer.

mod csv;
mod json;
mod markdown;

use std::path::Path;

use super::super::IdealLoadsOutdoorAirDiagnosticContext;
use csv::{
    render_outdoor_air_first_divergence_csv, render_outdoor_air_tolerance_failures_csv,
    render_outdoor_air_variable_deltas_csv,
};
use json::{
    render_outdoor_air_result_store_json, render_outdoor_air_selected_outputs_json,
    render_outdoor_air_stage_summary_json, render_outdoor_air_summary_json,
};
use markdown::render_outdoor_air_markdown;

pub(in crate::ideal_loads) fn write_outdoor_air_artifacts(
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
