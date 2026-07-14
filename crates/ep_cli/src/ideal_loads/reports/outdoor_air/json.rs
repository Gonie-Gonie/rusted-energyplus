//! JSON serialization for IdealLoads outdoor-air report artifacts.

use super::super::super::*;

pub(super) fn render_outdoor_air_summary_json(
    context: &IdealLoadsOutdoorAirDiagnosticContext<'_>,
) -> String {
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
        json_string(outdoor_air_overall_status(context))
    ));
    json.push_str(&format!(
        "  \"tolerance_policy\": {},\n",
        json_string(outdoor_air_tolerance_policy(context))
    ));
    json.push_str("  \"timestamp_rule\": \"EnergyPlus timestep ESO timestamps; Rust samples inherit oracle timestep labels\",\n");
    json.push_str(&format!(
        "  \"source_order_wrapper\": {},\n",
        json_string(IDEAL_LOADS_OUTDOOR_AIR_SOURCE_ORDER_WRAPPER)
    ));
    json.push_str(&format!(
        "  \"ideal_loads_invocation_path\": {},\n",
        json_string(IDEAL_LOADS_INVOCATION_PATH)
    ));
    json.push_str(&format!(
        "  \"direct_calc_helper_invocation\": {},\n",
        IDEAL_LOADS_DIRECT_CALC_HELPER_INVOCATION
    ));
    json.push_str(&format!(
        "  \"zone_equipment_dispatch_execution_boundary\": {},\n",
        json_string(IDEAL_LOADS_ZONE_EQUIPMENT_EXECUTION_BOUNDARY)
    ));
    json.push_str(&format!(
        "  \"ideal_loads_runtime_binding_source\": {},\n",
        json_string(IDEAL_LOADS_RUNTIME_BINDING_SOURCE)
    ));
    json.push_str(&format!(
        "  \"purchased_air_name_lookup_policy\": {},\n",
        json_string(IDEAL_LOADS_RUNTIME_STRING_LOOKUP_POLICY)
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
        "  \"selected_purchased_air_branch\": {},\n",
        json_string(outdoor_air_selected_purchased_air_branch())
    ));
    json.push_str(&format!(
        "  \"declared_ideal_loads_branch\": {},\n",
        json_string(outdoor_air_declared_ideal_loads_branch(context))
    ));
    json.push_str(&format!(
        "  \"inactive_branches\": {},\n",
        json_string_array(&outdoor_air_inactive_branches(context))
    ));
    json.push_str(&format!(
        "  \"ideal_loads_feature_flags\": {},\n",
        ideal_loads_feature_flags_json(context.feature_flags)
    ));
    json.push_str(&format!(
        "  \"ideal_loads_feature_dispatch_policy\": {},\n",
        json_string(IDEAL_LOADS_FEATURE_DISPATCH_POLICY)
    ));
    json.push_str(&format!(
        "  \"ideal_loads_prebound_id_contract\": {},\n",
        json_string(IDEAL_LOADS_PREBOUND_ID_CONTRACT)
    ));
    json.push_str(&format!(
        "  \"ideal_loads_psychrometric_evaluation_policy\": {},\n",
        json_string(IDEAL_LOADS_PSYCHROMETRIC_EVALUATION_POLICY)
    ));
    json.push_str(&format!(
        "  \"ideal_loads_psychrometric_cache_policy\": {},\n",
        json_string(IDEAL_LOADS_PSYCHROMETRIC_CACHE_POLICY)
    ));
    push_ideal_loads_output_handle_policy_json(&mut json);
    json.push_str(&format!(
        "  \"trace_level\": {},\n",
        json_string(ideal_loads_trace_level(manifest))
    ));
    json.push_str(&format!(
        "  \"trace_level_source\": {},\n",
        json_string(ideal_loads_trace_level_source(manifest))
    ));
    json.push_str(&format!(
        "  \"trace_payload\": {},\n",
        json_string(IDEAL_LOADS_OUTDOOR_AIR_TRACE_PAYLOAD)
    ));
    json.push_str(&format!(
        "  \"trace_side_effect_policy\": {},\n",
        json_string(IDEAL_LOADS_TRACE_SIDE_EFFECT_POLICY)
    ));
    json.push_str(&format!(
        "  \"trace_result_invariance_policy\": {},\n",
        json_string(IDEAL_LOADS_TRACE_RESULT_INVARIANCE_POLICY)
    ));
    json.push_str(&format!(
        "  \"trace_overhead_accounting\": {},\n",
        json_string(IDEAL_LOADS_TRACE_OVERHEAD_ACCOUNTING)
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
    json.push_str(&format!(
        "  \"outdoor_air_source\": {},\n",
        json_string(&outdoor_air_source_description(context))
    ));
    json.push_str(&format!(
        "  \"demand_controlled_ventilation_type\": {},\n",
        json_string(demand_controlled_ventilation_label(
            context.demand_controlled_ventilation_type
        ))
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
    if let Some(recirculation_node_name) = &context.recirculation_node_name {
        json.push_str(&format!(
            "  \"recirculation_node\": {},\n",
            json_string(recirculation_node_name)
        ));
        json.push_str(&format!(
            "  \"recirculation_state_source\": {},\n",
            json_string(IDEAL_LOADS_OUTDOOR_AIR_RECIRCULATION_STATE_SOURCE)
        ));
    }
    json.push_str(&format!(
        "  \"standard_air_density_kg_per_m3\": {},\n",
        json_number(context.standard_air_density_kg_per_m3)
    ));
    json.push_str(&format!(
        "  \"design_people_count\": {},\n",
        json_number(context.design_people_count)
    ));
    json.push_str(&format!(
        "  \"current_people_count_min\": {},\n",
        json_number(context.current_people_count_min)
    ));
    json.push_str(&format!(
        "  \"current_people_count_max\": {},\n",
        json_number(context.current_people_count_max)
    ));
    json.push_str(&format!(
        "  \"co2_setpoint_required_mass_flow_rate_min_kg_per_s\": {},\n",
        json_number(context.co2_setpoint_required_mass_flow_rate_min_kg_per_s)
    ));
    json.push_str(&format!(
        "  \"co2_setpoint_required_mass_flow_rate_max_kg_per_s\": {},\n",
        json_number(context.co2_setpoint_required_mass_flow_rate_max_kg_per_s)
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
    json.push_str(&format!(
        "  \"outdoor_air_mass_flow_rate_min_kg_per_s\": {},\n",
        json_number(context.outdoor_air_mass_flow_rate_min_kg_per_s)
    ));
    json.push_str(&format!(
        "  \"outdoor_air_mass_flow_rate_max_kg_per_s\": {},\n",
        json_number(context.outdoor_air_mass_flow_rate_max_kg_per_s)
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
pub(super) fn render_outdoor_air_selected_outputs_json(
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

pub(super) fn render_outdoor_air_result_store_json(
    context: &IdealLoadsOutdoorAirDiagnosticContext<'_>,
) -> String {
    let diagnostics = context.result_store.diagnostics();
    let profile = context.result_store.profile();
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
    json.push_str("  \"profile\": {\n");
    json.push_str(&format!(
        "    \"series_count\": {},\n",
        profile.series_count
    ));
    json.push_str(&format!(
        "    \"sample_count\": {},\n",
        profile.sample_count
    ));
    json.push_str(&format!(
        "    \"empty_series_count\": {}\n",
        profile.empty_series_count
    ));
    json.push_str("  },\n");
    json.push_str("  \"duplicate_guard\": \"ep_runtime::ResultStore::diagnostics\",\n");
    json.push_str(&format!(
        "  \"diagnostic_count\": {},\n",
        diagnostics.diagnostics.len()
    ));
    json.push_str("  \"diagnostics\": [\n");
    for (index, diagnostic) in diagnostics.diagnostics.iter().enumerate() {
        json.push_str("    {\n");
        json.push_str(&format!(
            "      \"code\": {},\n",
            json_string(&format!("{:?}", diagnostic.code))
        ));
        json.push_str(&format!(
            "      \"message\": {},\n",
            json_string(&diagnostic.message)
        ));
        json.push_str(&format!(
            "      \"handle\": {}\n",
            diagnostic
                .handle
                .map_or_else(|| "null".to_string(), |handle| handle.0.to_string())
        ));
        json.push_str("    }");
        if index + 1 < diagnostics.diagnostics.len() {
            json.push(',');
        }
        json.push('\n');
    }
    json.push_str("  ],\n");
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

pub(super) fn render_outdoor_air_stage_summary_json(
    context: &IdealLoadsOutdoorAirDiagnosticContext<'_>,
) -> String {
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
    json.push_str("  \"outdoor_air\": true,\n");
    json.push_str(&format!(
        "  \"source_order_wrapper\": {},\n",
        json_string(IDEAL_LOADS_OUTDOOR_AIR_SOURCE_ORDER_WRAPPER)
    ));
    json.push_str(&format!(
        "  \"ideal_loads_invocation_path\": {},\n",
        json_string(IDEAL_LOADS_INVOCATION_PATH)
    ));
    json.push_str(&format!(
        "  \"direct_calc_helper_invocation\": {},\n",
        IDEAL_LOADS_DIRECT_CALC_HELPER_INVOCATION
    ));
    json.push_str(&format!(
        "  \"zone_equipment_dispatch_execution_boundary\": {},\n",
        json_string(IDEAL_LOADS_ZONE_EQUIPMENT_EXECUTION_BOUNDARY)
    ));
    json.push_str(&format!(
        "  \"ideal_loads_runtime_binding_source\": {},\n",
        json_string(IDEAL_LOADS_RUNTIME_BINDING_SOURCE)
    ));
    json.push_str(&format!(
        "  \"purchased_air_name_lookup_policy\": {},\n",
        json_string(IDEAL_LOADS_RUNTIME_STRING_LOOKUP_POLICY)
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
        "  \"selected_purchased_air_branch\": {},\n",
        json_string(outdoor_air_selected_purchased_air_branch())
    ));
    json.push_str(&format!(
        "  \"declared_ideal_loads_branch\": {},\n",
        json_string(outdoor_air_declared_ideal_loads_branch(context))
    ));
    json.push_str(&format!(
        "  \"inactive_branches\": {},\n",
        json_string_array(&outdoor_air_inactive_branches(context))
    ));
    json.push_str(&format!(
        "  \"ideal_loads_feature_flags\": {},\n",
        ideal_loads_feature_flags_json(context.feature_flags)
    ));
    json.push_str(&format!(
        "  \"ideal_loads_feature_dispatch_policy\": {},\n",
        json_string(IDEAL_LOADS_FEATURE_DISPATCH_POLICY)
    ));
    json.push_str(&format!(
        "  \"ideal_loads_prebound_id_contract\": {},\n",
        json_string(IDEAL_LOADS_PREBOUND_ID_CONTRACT)
    ));
    json.push_str(&format!(
        "  \"ideal_loads_psychrometric_evaluation_policy\": {},\n",
        json_string(IDEAL_LOADS_PSYCHROMETRIC_EVALUATION_POLICY)
    ));
    json.push_str(&format!(
        "  \"ideal_loads_psychrometric_cache_policy\": {},\n",
        json_string(IDEAL_LOADS_PSYCHROMETRIC_CACHE_POLICY)
    ));
    push_ideal_loads_output_handle_policy_json(&mut json);
    json.push_str(&format!(
        "  \"trace_level\": {},\n",
        json_string(ideal_loads_trace_level(context.manifest))
    ));
    json.push_str(&format!(
        "  \"trace_level_source\": {},\n",
        json_string(ideal_loads_trace_level_source(context.manifest))
    ));
    json.push_str(&format!(
        "  \"trace_payload\": {},\n",
        json_string(IDEAL_LOADS_OUTDOOR_AIR_TRACE_PAYLOAD)
    ));
    json.push_str(&format!(
        "  \"trace_side_effect_policy\": {},\n",
        json_string(IDEAL_LOADS_TRACE_SIDE_EFFECT_POLICY)
    ));
    json.push_str(&format!(
        "  \"trace_result_invariance_policy\": {},\n",
        json_string(IDEAL_LOADS_TRACE_RESULT_INVARIANCE_POLICY)
    ));
    json.push_str(&format!(
        "  \"trace_overhead_accounting\": {},\n",
        json_string(IDEAL_LOADS_TRACE_OVERHEAD_ACCOUNTING)
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
    json.push_str(&format!(
        "  \"outdoor_air_method\": {},\n",
        json_string(outdoor_air_method_label(context.outdoor_air_method))
    ));
    json.push_str("  \"outdoor_air_schedule\": \"blank-always-1.0\",\n");
    json.push_str(&format!(
        "  \"demand_controlled_ventilation_type\": {},\n",
        json_string(demand_controlled_ventilation_label(
            context.demand_controlled_ventilation_type
        ))
    ));
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
    if let Some(recirculation_node_name) = &context.recirculation_node_name {
        json.push_str(&format!(
            "  \"recirculation_node\": {},\n",
            json_string(recirculation_node_name)
        ));
        json.push_str(&format!(
            "  \"recirculation_state_source\": {},\n",
            json_string(IDEAL_LOADS_OUTDOOR_AIR_RECIRCULATION_STATE_SOURCE)
        ));
    }
    json.push_str(&format!(
        "  \"standard_air_density_kg_per_m3\": {},\n",
        json_number(context.standard_air_density_kg_per_m3)
    ));
    json.push_str(&format!(
        "  \"design_people_count\": {},\n",
        json_number(context.design_people_count)
    ));
    json.push_str(&format!(
        "  \"current_people_count_min\": {},\n",
        json_number(context.current_people_count_min)
    ));
    json.push_str(&format!(
        "  \"current_people_count_max\": {},\n",
        json_number(context.current_people_count_max)
    ));
    json.push_str(&format!(
        "  \"co2_setpoint_required_mass_flow_rate_min_kg_per_s\": {},\n",
        json_number(context.co2_setpoint_required_mass_flow_rate_min_kg_per_s)
    ));
    json.push_str(&format!(
        "  \"co2_setpoint_required_mass_flow_rate_max_kg_per_s\": {},\n",
        json_number(context.co2_setpoint_required_mass_flow_rate_max_kg_per_s)
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
    json.push_str(&format!(
        "  \"outdoor_air_mass_flow_rate_min_kg_per_s\": {},\n",
        json_number(context.outdoor_air_mass_flow_rate_min_kg_per_s)
    ));
    json.push_str(&format!(
        "  \"outdoor_air_mass_flow_rate_max_kg_per_s\": {},\n",
        json_number(context.outdoor_air_mass_flow_rate_max_kg_per_s)
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
