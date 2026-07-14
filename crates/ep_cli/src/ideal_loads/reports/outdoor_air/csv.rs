//! CSV serialization for IdealLoads outdoor-air comparison artifacts.

use super::super::super::*;

pub(super) fn render_outdoor_air_variable_deltas_csv(
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
pub(super) fn render_outdoor_air_first_divergence_csv(
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

pub(super) fn render_outdoor_air_tolerance_failures_csv(
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
