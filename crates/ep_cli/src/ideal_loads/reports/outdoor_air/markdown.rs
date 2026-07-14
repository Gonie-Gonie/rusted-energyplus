//! Markdown serialization for the IdealLoads outdoor-air report.

use super::super::super::*;

pub(super) fn render_outdoor_air_markdown(
    context: &IdealLoadsOutdoorAirDiagnosticContext<'_>,
) -> String {
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
    report.push_str(&format!("timestep_source: {}\n", context.timestep.source));
    report.push_str(&format!(
        "nominal_system_timestep_substeps: {:.0}\n",
        context.timestep.nominal_system_timestep_substeps
    ));
    report.push_str(&format!(
        "nominal_system_timestep_seconds: {:.12}\n",
        context.timestep.nominal_system_timestep_seconds
    ));
    report.push_str(&format!(
        "zone_timestep_seconds: {:.12}\n",
        context.timestep.zone_timestep_seconds
    ));
    report.push_str(&format!(
        "adaptive_system_timestep_claim: {}\n",
        context.timestep.adaptive_system_timestep_claim
    ));
    report.push_str("sample_timestep_source: ESO timestamp duration with ep_runtime::TimeAxis integer-substep normalization and nominal fallback\n");
    let purchased_air_source_order = purchased_air_source_order_stages()
        .iter()
        .map(|stage| stage.source_routine)
        .collect::<Vec<_>>()
        .join(" -> ");
    let zone_equipment_dispatch_issues = context.zone_equipment_dispatch.issue_codes();
    let zone_equipment_dispatch_warnings = context.zone_equipment_dispatch.warning_codes();
    report.push_str(&format!(
        "source_order_wrapper: {}\n",
        IDEAL_LOADS_OUTDOOR_AIR_SOURCE_ORDER_WRAPPER
    ));
    report.push_str(&format!(
        "ideal_loads_invocation_path: {}\n",
        IDEAL_LOADS_INVOCATION_PATH
    ));
    report.push_str(&format!(
        "direct_calc_helper_invocation: {}\n",
        IDEAL_LOADS_DIRECT_CALC_HELPER_INVOCATION
    ));
    report.push_str(&format!(
        "zone_equipment_dispatch_execution_boundary: {}\n",
        IDEAL_LOADS_ZONE_EQUIPMENT_EXECUTION_BOUNDARY
    ));
    report.push_str(&format!(
        "ideal_loads_runtime_binding_source: {}\n",
        IDEAL_LOADS_RUNTIME_BINDING_SOURCE
    ));
    report.push_str(&format!(
        "purchased_air_name_lookup_policy: {}\n",
        IDEAL_LOADS_RUNTIME_STRING_LOOKUP_POLICY
    ));
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
        "purchased_air_source_order: {}\n",
        purchased_air_source_order
    ));
    report.push_str(&format!(
        "selected_purchased_air_branch: {}\n",
        outdoor_air_selected_purchased_air_branch()
    ));
    report.push_str(&format!(
        "declared_ideal_loads_branch: {}\n",
        outdoor_air_declared_ideal_loads_branch(context)
    ));
    report.push_str(&format!(
        "inactive_branches: {}\n",
        outdoor_air_inactive_branches(context).join(", ")
    ));
    report.push_str(&format!(
        "ideal_loads_feature_flags: {}\n",
        ideal_loads_feature_flags_label(context.feature_flags)
    ));
    report.push_str(&format!(
        "ideal_loads_feature_dispatch_policy: {}\n",
        IDEAL_LOADS_FEATURE_DISPATCH_POLICY
    ));
    report.push_str(&format!(
        "ideal_loads_prebound_id_contract: {}\n",
        IDEAL_LOADS_PREBOUND_ID_CONTRACT
    ));
    report.push_str(&format!(
        "ideal_loads_psychrometric_evaluation_policy: {}\n",
        IDEAL_LOADS_PSYCHROMETRIC_EVALUATION_POLICY
    ));
    report.push_str(&format!(
        "ideal_loads_psychrometric_cache_policy: {}\n",
        IDEAL_LOADS_PSYCHROMETRIC_CACHE_POLICY
    ));
    push_ideal_loads_output_handle_policy_markdown(&mut report);
    report.push_str(&format!(
        "trace_level: {}\n",
        ideal_loads_trace_level(context.manifest)
    ));
    report.push_str(&format!(
        "trace_level_source: {}\n",
        ideal_loads_trace_level_source(context.manifest)
    ));
    report.push_str(&format!(
        "trace_payload: {}\n",
        IDEAL_LOADS_OUTDOOR_AIR_TRACE_PAYLOAD
    ));
    report.push_str(&format!(
        "trace_side_effect_policy: {}\n",
        IDEAL_LOADS_TRACE_SIDE_EFFECT_POLICY
    ));
    report.push_str(&format!(
        "trace_result_invariance_policy: {}\n",
        IDEAL_LOADS_TRACE_RESULT_INVARIANCE_POLICY
    ));
    report.push_str(&format!(
        "trace_overhead_accounting: {}\n",
        IDEAL_LOADS_TRACE_OVERHEAD_ACCOUNTING
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
        "meter_aggregation_source: {}\n",
        IDEAL_LOADS_METER_AGGREGATION_SOURCE
    ));
    report.push_str(&format!(
        "meter_fuel_energy_binding_source: {}\n",
        IDEAL_LOADS_METER_FUEL_ENERGY_BINDING_SOURCE
    ));
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
    report.push_str(&format!(
        "outdoor_air_source: {}\n",
        outdoor_air_source_description(context)
    ));
    report.push_str("outdoor_air_schedule: blank-always-1.0\n");
    report.push_str(&format!(
        "demand_controlled_ventilation_type: {}\n",
        demand_controlled_ventilation_label(context.demand_controlled_ventilation_type)
    ));
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
    if let Some(recirculation_node_name) = &context.recirculation_node_name {
        report.push_str(&format!(
            "recirculation_node: {}\n",
            markdown_cell(recirculation_node_name)
        ));
        report.push_str(&format!(
            "recirculation_state_source: {}\n",
            IDEAL_LOADS_OUTDOOR_AIR_RECIRCULATION_STATE_SOURCE
        ));
    }
    report.push_str(&format!(
        "standard_air_density_kg_per_m3: {:.15}\n",
        context.standard_air_density_kg_per_m3
    ));
    report.push_str(&format!(
        "design_people_count: {:.15}\n",
        context.design_people_count
    ));
    report.push_str(&format!(
        "current_people_count_min: {:.15}\n",
        context.current_people_count_min
    ));
    report.push_str(&format!(
        "current_people_count_max: {:.15}\n",
        context.current_people_count_max
    ));
    report.push_str(&format!(
        "co2_setpoint_required_mass_flow_rate_min_kg_per_s: {:.15}\n",
        context.co2_setpoint_required_mass_flow_rate_min_kg_per_s
    ));
    report.push_str(&format!(
        "co2_setpoint_required_mass_flow_rate_max_kg_per_s: {:.15}\n",
        context.co2_setpoint_required_mass_flow_rate_max_kg_per_s
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
        "outdoor_air_mass_flow_rate_kg_per_s: {:.15}\n",
        context.outdoor_air_mass_flow_rate_kg_per_s
    ));
    report.push_str(&format!(
        "outdoor_air_mass_flow_rate_min_kg_per_s: {:.15}\n",
        context.outdoor_air_mass_flow_rate_min_kg_per_s
    ));
    report.push_str(&format!(
        "outdoor_air_mass_flow_rate_max_kg_per_s: {:.15}\n\n",
        context.outdoor_air_mass_flow_rate_max_kg_per_s
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
