//! IdealLoads arbitrary-run integration tests.

#![allow(clippy::expect_used, clippy::unwrap_used)]

use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use ep_run::{
    PartialRunPolicy, RunConfig, RunExitCode, RunMode, RunOutputFormat, RunResultState,
    SupportStatus, TraceLevel, TraceSelection, run_arbitrary_idf,
};
use serde_json::Value;

#[allow(dead_code)]
#[path = "arbitrary_run/fixtures.rs"]
mod fixtures;

use fixtures::*;

#[allow(dead_code)]
#[path = "arbitrary_run/output_manifest.rs"]
mod output_manifest;

use output_manifest::{SUPPORTED_RUNTIME_MANIFEST, assert_output_manifest};

#[test]
fn ideal_loads_no_oa_branch_runs_declared_compatibility_runtime()
-> Result<(), Box<dyn std::error::Error>> {
    let fixture = active_cooling_ideal_loads_fixture();
    let summary = assert_direct_ideal_loads_fixture_runs(
        "ideal-loads-no-oa",
        &fixture,
        "ideal-loads-direct-zone-coupled-compatibility",
        "ideal_loads_no_oa_sensible",
    )?;

    assert_eq!(summary["rust_runtime"]["samples"], 1);
    assert_eq!(
        summary["rust_runtime"]["zone_demand_source"],
        "rust-predictor-source-setpoint-thresholds"
    );
    assert_eq!(
        summary["rust_runtime"]["fixture_demand_injection_used"],
        false
    );
    assert_eq!(
        summary["rust_runtime"]["recirculation_state_source"],
        "rust-direct-zone-return-projection"
    );
    assert!(
        summary["rust_runtime"]["source_order_stages"]
            .as_array()
            .expect("source order stages should be an array")
            .iter()
            .any(|stage| stage == "calc-purch-air-loads")
    );
    let cp343 = &summary["rust_runtime"]["purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_lifecycle"];
    assert_eq!(
        cp343["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2201"
    );
    assert_eq!(
        cp343["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2203"
    );
    assert!(cp343["latest"].is_object());
    let cp344 = &summary["rust_runtime"]["purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle"];
    assert_eq!(
        cp344["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2203"
    );
    assert_eq!(
        cp344["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2208"
    );
    assert_eq!(
        cp344["latest"]["source_order"]
            .as_array()
            .expect("CP344 source order"),
        &[
            "read-purchased-air-supply-temperature-for-minimum",
            "read-purchased-air-mixed-air-temperature-for-minimum",
            "apply-source-shaped-two-argument-minimum",
            "assign-purchased-air-supply-temperature",
        ]
    );
    let cp345 = &summary["rust_runtime"]["purchased_air_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle"];
    assert_eq!(
        cp345["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2208"
    );
    assert_eq!(
        cp345["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2209"
    );
    assert_eq!(
        cp345["latest"]["source_order"]
            .as_array()
            .expect("CP345 source order"),
        &[
            "read-purchased-air-mixed-air-humidity-ratio",
            "assign-purchased-air-supply-humidity-ratio",
        ]
    );
    for field in [
        "capacity_limit_guard_false_fallthrough_skipped",
        "capacity_limit_sensible_output_guard_false_fallthrough",
        "capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed",
    ] {
        assert_eq!(
            cp345["latest"][field], cp344["latest"][field],
            "CP345 must retain CP344 {field} provenance"
        );
    }
    let cp329 =
        &summary["rust_runtime"]["purchased_air_calc_cooling_mixed_air_call_lifecycle"]["latest"];
    let cp335 = &summary["rust_runtime"]["purchased_air_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle"]
        ["latest"];
    let cp345_mixed_air_bits = cp345["latest"]["mixed_air_humidity_ratio_ieee_bits"]
        .as_str()
        .expect("active CP345 mixed-air humidity-ratio bits");
    let cp329_mixed_air_bits = cp329["mixed_air_humidity_ratio_ieee_bits"]
        .as_str()
        .expect("active CP329 mixed-air humidity-ratio bits");
    assert_eq!(
        cp345_mixed_air_bits, cp329_mixed_air_bits,
        "CP345 must read the CP329-owned mixed-air humidity-ratio bits"
    );
    let cp345_assigned_bits = cp345["latest"]["assigned_supply_humidity_ratio_ieee_bits"]
        .as_str()
        .expect("active CP345 assigned humidity-ratio bits");
    let cp335_assigned_bits = cp335["assigned_supply_humidity_ratio_ieee_bits"]
        .as_str()
        .expect("active CP335 assigned humidity-ratio bits");
    assert_eq!(
        cp345_assigned_bits, cp335_assigned_bits,
        "CP335 must corroborate the CP345 assigned humidity-ratio bits"
    );
    let cp345_assignment_route_count = [
        "capacity_limit_guard_false_fallthrough_skipped",
        "capacity_limit_sensible_output_guard_false_fallthrough",
        "capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed",
    ]
    .into_iter()
    .filter(|field| cp345["latest"][field].as_bool() == Some(true))
    .count();
    let cp345_skip_route_count = [
        "unit_off_skipped",
        "non_cooling_skipped",
        "positive_guard_false_fallthrough_skipped",
    ]
    .into_iter()
    .filter(|field| cp345["latest"][field].as_bool() == Some(true))
    .count();
    assert_eq!(
        cp345_assignment_route_count, 1,
        "fixture must execute one active CP345 G/F/L assignment route"
    );
    assert_eq!(
        cp345_skip_route_count, 0,
        "active CP345 fixture must not take an inherited skip route"
    );
    assert_eq!(
        cp345["latest"]["capacity_limit_guard_false_fallthrough_skipped"], true,
        "no-limit fixture must exercise the CP345 G assignment route"
    );
    let cp345_assignment_executed = cp345_assignment_route_count == 1;
    assert_eq!(
        cp345["latest"]["post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed"],
        cp345_assignment_executed
    );
    for field in ["mixed_air_humidity_ratio", "assigned_supply_humidity_ratio"] {
        assert_eq!(
            cp345["latest"][field].is_number(),
            cp345_assignment_executed,
            "{field}"
        );
        assert_eq!(
            cp345["latest"][format!("{field}_ieee_bits")].is_string(),
            cp345_assignment_executed,
            "{field} bits"
        );
    }
    let cp346 = &summary["rust_runtime"]["purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_lifecycle"];
    assert_eq!(
        cp346["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2209"
    );
    assert_eq!(
        cp346["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2211"
    );
    assert_eq!(
        cp346["latest"]["source_order"]
            .as_array()
            .expect("CP346 source order"),
        &[
            "read-purchased-air-dehumidification-control-type",
            "dispatch-dehumidification-control-switch",
        ]
    );
    for (cp346_field, cp345_field) in [
        (
            "predecessor_capacity_limit_guard_false_fallthrough",
            "capacity_limit_guard_false_fallthrough_skipped",
        ),
        (
            "predecessor_capacity_limit_sensible_output_guard_false_fallthrough",
            "capacity_limit_sensible_output_guard_false_fallthrough",
        ),
        (
            "predecessor_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed",
            "capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed",
        ),
    ] {
        assert_eq!(
            cp346["latest"][cp346_field], cp345["latest"][cp345_field],
            "CP346 must retain CP345 {cp345_field} provenance"
        );
    }
    assert_eq!(
        cp346["latest"]["predecessor_post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed"],
        true
    );
    assert_eq!(
        cp346["latest"]["predecessor_assigned_supply_humidity_ratio"],
        cp345["latest"]["assigned_supply_humidity_ratio"]
    );
    assert_eq!(cp346["latest"]["dehumidification_control_type_read"], true);
    assert_eq!(cp346["latest"]["dehumidification_control_type"], "None");
    assert_eq!(
        cp346["latest"]["dehumidification_control_switch_dispatched"],
        true
    );
    assert_eq!(
        cp346["dehumidification_control_switch_count"],
        cp345["post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count"]
    );
    assert_eq!(
        cp346["dehumidification_control_none_case_selection_count"],
        cp346["dehumidification_control_switch_count"]
    );
    let cp319 = &summary["rust_runtime"]["purchased_air_calc_cooling_dehumidification_flow_lifecycle"]
        ["latest"];
    assert_eq!(
        cp319["dehumidification_control_type"], cp346["latest"]["dehumidification_control_type"],
        "same-call CP319 None selector is corroboration, not CP346 operand ownership"
    );
    let cp347 = &summary["rust_runtime"]["purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_lifecycle"];
    assert_eq!(
        cp347["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2210-2212"
    );
    assert_eq!(
        cp347["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2216"
    );
    assert_eq!(
        cp347["latest"]["source_order"]
            .as_array()
            .expect("CP347 source order"),
        &[
            "enter-purchased-air-dehumidification-control-none-case",
            "read-purchased-air-mixed-air-humidity-ratio-for-none-case",
            "assign-purchased-air-supply-humidity-ratio-in-none-case",
            "exit-purchased-air-dehumidification-control-none-case-via-break",
        ]
    );
    assert_eq!(
        cp347["latest"]["predecessor_dehumidification_control_type_read"],
        cp346["latest"]["dehumidification_control_type_read"]
    );
    assert_eq!(
        cp347["latest"]["predecessor_dehumidification_control_type"],
        "None"
    );
    assert_eq!(
        cp347["latest"]["predecessor_dehumidification_control_switch_dispatched"],
        cp346["latest"]["dehumidification_control_switch_dispatched"]
    );
    for field in [
        "dehumidification_control_none_case_entered",
        "mixed_air_humidity_ratio_read",
        "supply_humidity_ratio_assignment_performed",
        "dehumidification_control_none_case_exited_via_break",
    ] {
        assert_eq!(cp347["latest"][field], true, "{field}");
    }
    assert_eq!(
        cp347["latest"]["predecessor_assigned_supply_humidity_ratio"],
        cp346["latest"]["predecessor_assigned_supply_humidity_ratio"],
        "CP347 must retain its immediate CP346 humidity-ratio lineage"
    );
    assert_eq!(
        cp347["latest"]["predecessor_assigned_supply_humidity_ratio_ieee_bits"],
        cp345["latest"]["assigned_supply_humidity_ratio_ieee_bits"],
        "CP347 must preserve the CP345/CP346 predecessor value's exact bits"
    );
    for field in [
        "mixed_air_humidity_ratio_ieee_bits",
        "assigned_supply_humidity_ratio_ieee_bits",
        "resulting_supply_humidity_ratio_ieee_bits",
    ] {
        assert_eq!(
            cp347["latest"][field], cp329["mixed_air_humidity_ratio_ieee_bits"],
            "CP347 {field} must retain the CP329-owned humidity-ratio bits"
        );
    }
    assert_eq!(
        cp347["dehumidification_control_none_case_completion_count"],
        cp346["dehumidification_control_none_case_selection_count"]
    );
    assert_eq!(
        cp347["source_site_execution_count"],
        cp347["dehumidification_control_none_case_completion_count"]
            .as_u64()
            .expect("CP347 completion count")
            * 4
    );
    for field in [
        "dehumidification_control_none_case_entry_count",
        "mixed_air_humidity_ratio_read_count",
        "supply_humidity_ratio_assignment_count",
        "dehumidification_control_none_case_break_count",
    ] {
        assert_eq!(
            cp347[field], cp347["dehumidification_control_none_case_completion_count"],
            "{field}"
        );
    }
    let cp348 = &summary["rust_runtime"]["purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_lifecycle"];
    assert_eq!(
        cp348["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2213"
    );
    assert_eq!(
        cp348["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2216"
    );
    assert_eq!(
        cp348["latest"]["source_order"]
            .as_array()
            .expect("CP348 source order"),
        &["enter-purchased-air-dehumidification-control-constant-sensible-heat-ratio-case"]
    );
    assert_eq!(
        cp348["latest"]["predecessor_dehumidification_control_type"],
        cp347["latest"]["predecessor_dehumidification_control_type"],
        "CP348 must retain the immediate CP347 selector lineage"
    );
    assert_eq!(
        cp348["latest"]["predecessor_dehumidification_control_none_case_completed"],
        cp347["latest"]["dehumidification_control_none_case_exited_via_break"],
        "CP348 must retain the completed CP347 None-case route"
    );
    assert_eq!(
        cp348["latest"]["dehumidification_control_none_case_completed_skip"],
        true
    );
    assert_eq!(
        cp348["latest"]["dehumidification_control_constant_sensible_heat_ratio_case_entered"],
        false
    );
    assert_eq!(
        cp348["latest"]["dehumidification_control_humidistat_case_selected_skip"],
        false
    );
    assert_eq!(
        cp348["latest"]["dehumidification_control_constant_supply_humidity_ratio_case_selected_skip"],
        false
    );
    assert_eq!(
        cp348["dehumidification_control_none_case_completed_skip_count"],
        cp347["dehumidification_control_none_case_completion_count"]
    );
    assert_eq!(
        cp348["dehumidification_control_constant_sensible_heat_ratio_case_entry_count"],
        0
    );
    assert_eq!(cp348["source_site_execution_count"], 0);
    assert_eq!(
        cp348["dehumidification_control_constant_sensible_heat_ratio_case_entry_site_count"],
        0
    );
    Ok(())
}

#[test]
fn ideal_loads_fixture_demand_fallbacks_fail_closed_in_compatibility_mode()
-> Result<(), Box<dyn std::error::Error>> {
    for (name, fixture) in [
        (
            "ideal-loads-constant-shr",
            IDEAL_LOADS_CONSTANT_SHR_EPJSON.to_string(),
        ),
        (
            "ideal-loads-constant-supply-humidity",
            IDEAL_LOADS_CONSTANT_SUPPLY_HUMIDITY_EPJSON.to_string(),
        ),
        (
            "ideal-loads-outdoor-air",
            IDEAL_LOADS_OUTDOOR_AIR_EPJSON.to_string(),
        ),
        (
            "ideal-loads-mixed",
            IDEAL_LOADS_MIXED_BRANCH_EPJSON.to_string(),
        ),
        (
            "ideal-loads-finite-hysteresis",
            finite_limit_hysteresis_fixture(),
        ),
    ] {
        let case_dir = unique_case_dir(name)?;
        let input_path = case_dir.join("ideal-loads.epJSON");
        let output_dir = case_dir.join("out");
        write_text(&input_path, &fixture)?;

        let outcome = run_arbitrary_idf(&RunConfig {
            input_path,
            weather_path: None,
            output_dir: output_dir.clone(),
            mode: RunMode::Compatibility,
            partial_policy: PartialRunPolicy::Deny,
            output_format: RunOutputFormat::RustNative,
            overwrite: true,
            keep_intermediate: true,
            trace_level: TraceLevel::Normal,
            trace_selection: TraceSelection::default(),
            fail_on_warning: false,
            dry_run: false,
            oracle_baseline: false,
            compare_oracle: false,
            json_stdout: false,
            oracle_root: None,
            hours: Some(1),
        })?;

        assert_eq!(outcome.exit_code, RunExitCode::Unsupported, "{name}");
        assert_eq!(
            outcome.support_status,
            SupportStatus::SupportedDiagnosticOnly,
            "{name}"
        );
        assert_eq!(
            outcome.run_result_state,
            RunResultState::RunBlocked,
            "{name}"
        );

        let summary = read_json(&output_dir.join("run-summary.json"))?;
        assert_eq!(summary["status"], "unsupported", "{name}");
        assert_eq!(
            summary["support"]["status"], "supported-diagnostic-only",
            "{name}"
        );
        assert_eq!(
            summary["support"]["run_result_state"], "run_blocked",
            "{name}"
        );
        assert_eq!(
            summary["support"]["runtime_class"], "ideal-loads-fixture-demand-diagnostic",
            "{name}"
        );
        assert_eq!(
            summary["support"]["matched_capability_ids"]
                .as_array()
                .expect("matched capability ids should be an array")
                .len(),
            0,
            "{name}"
        );
        assert_eq!(
            summary["support"]["selected_algorithm_lane"]["id"], "diagnostic-probe",
            "{name}"
        );
        assert_eq!(
            summary["support"]["selected_algorithm_lane"]["conformance_promotion_allowed"], false,
            "{name}"
        );
        assert!(summary["rust_runtime"].is_null(), "{name}");
        assert!(
            !output_dir
                .join("results")
                .join("result-store.json")
                .exists(),
            "{name}"
        );
        let diagnostics = read_json(&output_dir.join("diagnostics.json"))?;
        let diagnostic_entries = diagnostics["diagnostics"]
            .as_array()
            .expect("diagnostics should be an array");
        assert!(
            diagnostic_entries.iter().any(|diagnostic| {
                diagnostic["code"] == "IdealLoadsFixtureDemandDiagnosticOnly"
            }),
            "{name}"
        );
        assert!(
            diagnostic_entries
                .iter()
                .any(|diagnostic| diagnostic["code"] == "DiagnosticOnlyRuntimeBlocked"),
            "{name}"
        );
    }
    Ok(())
}

#[test]
fn ideal_loads_fixture_demand_runs_only_as_explicit_diagnostic_with_provenance()
-> Result<(), Box<dyn std::error::Error>> {
    let case_dir = unique_case_dir("ideal-loads-fixture-demand-diagnostic")?;
    let input_path = case_dir.join("ideal-loads-constant-shr.epJSON");
    let output_dir = case_dir.join("out");
    write_text(&input_path, IDEAL_LOADS_CONSTANT_SHR_EPJSON)?;

    let outcome = run_arbitrary_idf(&RunConfig {
        input_path,
        weather_path: None,
        output_dir: output_dir.clone(),
        mode: RunMode::Diagnostic,
        partial_policy: PartialRunPolicy::Allow,
        output_format: RunOutputFormat::RustNative,
        overwrite: true,
        keep_intermediate: true,
        trace_level: TraceLevel::Normal,
        trace_selection: TraceSelection::default(),
        fail_on_warning: false,
        dry_run: false,
        oracle_baseline: false,
        compare_oracle: false,
        json_stdout: false,
        oracle_root: None,
        hours: Some(1),
    })?;

    assert_eq!(outcome.exit_code, RunExitCode::Success);
    assert_eq!(
        outcome.support_status,
        SupportStatus::SupportedDiagnosticOnly
    );
    assert_eq!(
        outcome.run_result_state,
        RunResultState::PartialSupportedRun
    );

    let summary = read_json(&output_dir.join("run-summary.json"))?;
    assert_eq!(summary["status"], "success");
    assert_eq!(summary["support"]["status"], "supported-diagnostic-only");
    assert_eq!(
        summary["support"]["run_result_state"],
        "partial_supported_run"
    );
    assert_eq!(
        summary["support"]["runtime_class"],
        "ideal-loads-fixture-demand-diagnostic"
    );
    assert_eq!(
        summary["support"]["matched_capability_ids"]
            .as_array()
            .expect("matched capability ids should be an array")
            .len(),
        0
    );
    assert_eq!(summary["support"]["conformance_claim"], false);
    assert_eq!(
        summary["support"]["selected_algorithm_lane"]["id"],
        "diagnostic-probe"
    );
    assert_eq!(
        summary["support"]["selected_algorithm_lane"]["diagnostic_probe_used"],
        true
    );
    assert_eq!(
        summary["support"]["selected_algorithm_lane"]["conformance_promotion_allowed"],
        false
    );
    assert_eq!(
        summary["rust_runtime"]["runtime_class"],
        "ideal-loads-fixture-demand-diagnostic"
    );
    assert_eq!(
        summary["rust_runtime"]["zone_demand_source"],
        "rust-diagnostic-default-active-load-split"
    );
    assert_eq!(
        summary["rust_runtime"]["fixture_demand_injection_used"],
        true
    );
    assert_eq!(summary["rust_runtime"]["samples"], 1);
    let rust_runtime = summary["rust_runtime"]
        .as_object()
        .expect("diagnostic Rust runtime should be an object");
    assert!(rust_runtime.contains_key("purchased_air_calc_cooling_economizer_condition_lifecycle"));
    assert!(rust_runtime["purchased_air_calc_cooling_economizer_condition_lifecycle"].is_null());
    assert!(rust_runtime.contains_key("purchased_air_calc_cooling_economizer_body_lifecycle"));
    assert!(rust_runtime["purchased_air_calc_cooling_economizer_body_lifecycle"].is_null());
    assert!(rust_runtime.contains_key("purchased_air_calc_cooling_sensible_flow_lifecycle"));
    assert!(rust_runtime["purchased_air_calc_cooling_sensible_flow_lifecycle"].is_null());
    assert!(
        rust_runtime.contains_key("purchased_air_calc_cooling_dehumidification_flow_lifecycle")
    );
    assert!(rust_runtime["purchased_air_calc_cooling_dehumidification_flow_lifecycle"].is_null());
    assert!(rust_runtime.contains_key("purchased_air_calc_cooling_humidification_flow_lifecycle"));
    assert!(rust_runtime["purchased_air_calc_cooling_humidification_flow_lifecycle"].is_null());
    assert!(
        rust_runtime.contains_key("purchased_air_calc_cooling_capacity_zero_flow_reset_lifecycle")
    );
    assert!(
        rust_runtime["purchased_air_calc_cooling_capacity_zero_flow_reset_lifecycle"].is_null()
    );
    assert!(
        rust_runtime.contains_key("purchased_air_calc_cooling_supply_mass_flow_maximum_lifecycle")
    );
    assert!(
        rust_runtime["purchased_air_calc_cooling_supply_mass_flow_maximum_lifecycle"].is_null()
    );
    assert!(
        rust_runtime.contains_key(
            "purchased_air_calc_cooling_supply_mass_flow_ems_override_guard_lifecycle"
        )
    );
    assert!(
        rust_runtime["purchased_air_calc_cooling_supply_mass_flow_ems_override_guard_lifecycle"]
            .is_null()
    );
    assert!(
        rust_runtime.contains_key(
            "purchased_air_calc_cooling_supply_mass_flow_ems_override_body_lifecycle"
        )
    );
    assert!(
        rust_runtime["purchased_air_calc_cooling_supply_mass_flow_ems_override_body_lifecycle"]
            .is_null()
    );
    assert!(
        rust_runtime
            .contains_key("purchased_air_calc_cooling_supply_mass_flow_limit_guard_lifecycle")
    );
    assert!(
        rust_runtime["purchased_air_calc_cooling_supply_mass_flow_limit_guard_lifecycle"].is_null()
    );
    assert!(
        rust_runtime
            .contains_key("purchased_air_calc_cooling_supply_mass_flow_limit_body_lifecycle")
    );
    assert!(
        rust_runtime["purchased_air_calc_cooling_supply_mass_flow_limit_body_lifecycle"].is_null()
    );
    assert!(
        rust_runtime
            .contains_key("purchased_air_calc_cooling_supply_mass_flow_very_small_guard_lifecycle")
    );
    assert!(
        rust_runtime["purchased_air_calc_cooling_supply_mass_flow_very_small_guard_lifecycle"]
            .is_null()
    );
    assert!(rust_runtime.contains_key(
        "purchased_air_calc_cooling_supply_mass_flow_very_small_guard_body_lifecycle"
    ));
    assert!(
        rust_runtime["purchased_air_calc_cooling_supply_mass_flow_very_small_guard_body_lifecycle"]
            .is_null()
    );
    assert!(rust_runtime.contains_key("purchased_air_calc_cooling_mixed_air_call_lifecycle"));
    assert!(rust_runtime["purchased_air_calc_cooling_mixed_air_call_lifecycle"].is_null());
    assert!(
        rust_runtime
            .contains_key("purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle")
    );
    assert!(
        rust_runtime["purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle"]
            .is_null()
    );
    assert!(
        rust_runtime
            .contains_key("purchased_air_calc_cooling_positive_supply_cp_air_assignment_lifecycle")
    );
    assert!(
        rust_runtime["purchased_air_calc_cooling_positive_supply_cp_air_assignment_lifecycle"]
            .is_null()
    );
    assert!(rust_runtime.contains_key(
        "purchased_air_calc_cooling_positive_supply_temperature_assignment_lifecycle"
    ));
    assert!(
        rust_runtime["purchased_air_calc_cooling_positive_supply_temperature_assignment_lifecycle"]
            .is_null()
    );
    assert!(rust_runtime.contains_key(
        "purchased_air_calc_cooling_positive_supply_temperature_minimum_limit_lifecycle"
    ));
    assert!(
        rust_runtime
            ["purchased_air_calc_cooling_positive_supply_temperature_minimum_limit_lifecycle"]
            .is_null()
    );
    assert!(rust_runtime.contains_key(
        "purchased_air_calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle"
    ));
    assert!(
        rust_runtime
            ["purchased_air_calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle"]
            .is_null()
    );
    assert!(rust_runtime.contains_key(
        "purchased_air_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle"
    ));
    assert!(
        rust_runtime
            ["purchased_air_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle"]
            .is_null()
    );
    assert!(
        rust_runtime.contains_key(
            "purchased_air_calc_cooling_positive_supply_enthalpy_assignment_lifecycle"
        )
    );
    assert!(
        rust_runtime["purchased_air_calc_cooling_positive_supply_enthalpy_assignment_lifecycle"]
            .is_null()
    );
    assert!(
        rust_runtime.contains_key(
            "purchased_air_calc_cooling_positive_supply_capacity_limit_guard_lifecycle"
        )
    );
    assert!(
        rust_runtime["purchased_air_calc_cooling_positive_supply_capacity_limit_guard_lifecycle"]
            .is_null()
    );
    assert!(rust_runtime.contains_key(
        "purchased_air_calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle"
    ));
    assert!(
        rust_runtime
            ["purchased_air_calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle"]
            .is_null()
    );
    assert!(rust_runtime.contains_key(
        "purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle"
    ));
    assert!(
        rust_runtime
            ["purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle"]
            .is_null()
    );
    assert!(rust_runtime.contains_key(
        "purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle"
    ));
    assert!(
        rust_runtime
            ["purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle"]
            .is_null()
    );
    assert!(rust_runtime.contains_key(
        "purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_lifecycle"
    ));
    assert!(
        rust_runtime
            ["purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_lifecycle"]
            .is_null()
    );
    assert!(rust_runtime.contains_key(
        "purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle"
    ));
    assert!(
        rust_runtime
            ["purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle"]
            .is_null()
    );
    assert!(rust_runtime.contains_key(
        "purchased_air_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle"
    ));
    assert!(
        rust_runtime
            ["purchased_air_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle"]
            .is_null()
    );
    assert!(rust_runtime.contains_key(
        "purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_lifecycle"
    ));
    assert!(
        rust_runtime
            ["purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_lifecycle"]
            .is_null()
    );
    assert!(rust_runtime.contains_key(
        "purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_lifecycle"
    ));
    assert!(
        rust_runtime
            ["purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_lifecycle"]
            .is_null()
    );
    assert!(rust_runtime.contains_key(
        "purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_lifecycle"
    ));
    assert!(
        rust_runtime
            ["purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_lifecycle"]
            .is_null()
    );
    assert_eq!(summary["source_order_gate"]["matches"], true);
    assert_output_manifest(&output_dir, SUPPORTED_RUNTIME_MANIFEST)?;
    assert!(
        summary["rust_runtime"]["source_order_stages"]
            .as_array()
            .expect("source order stages should be an array")
            .iter()
            .any(|stage| stage == "calc-purch-air-loads")
    );
    let results = std::fs::read_to_string(output_dir.join("results").join("result-store.json"))?;
    assert!(results.contains("System Node Humidity Ratio"));
    Ok(())
}

fn unique_case_dir(name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let path = std::env::temp_dir().join(format!(
        "rusted-energyplus-ep-run-{name}-{}-{timestamp}",
        std::process::id()
    ));
    if path.exists() {
        std::fs::remove_dir_all(&path)?;
    }
    std::fs::create_dir_all(&path)?;
    Ok(path)
}

fn write_text(path: &Path, contents: &str) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::write(path, contents)?;
    Ok(())
}

fn finite_limit_hysteresis_fixture() -> String {
    IDEAL_LOADS_EPJSON
        .replace(
            r#""control_1_name": "Dual Setpoints""#,
            r#""control_1_name": "Dual Setpoints",
      "temperature_difference_between_cutout_and_setpoint": 0.5"#,
        )
        .replace(
            r#""zone_supply_air_node_name": "Zone Inlets","#,
            r#""zone_supply_air_node_name": "Zone Inlets",
      "heating_limit": "LimitFlowRateAndCapacity",
      "maximum_heating_air_flow_rate": 0.01,
      "maximum_sensible_heating_capacity": 300.0,
      "cooling_limit": "LimitFlowRateAndCapacity",
      "maximum_cooling_air_flow_rate": 0.01,
      "maximum_total_cooling_capacity": 300.0,"#,
        )
}

fn active_cooling_ideal_loads_fixture() -> String {
    IDEAL_LOADS_EPJSON
        .replace(
            r#""Version": {"Version 1": {"version_identifier": "26.1"}},"#,
            r#""Version": {"Version 1": {"version_identifier": "26.1"}},
  "Timestep": {"Timestep 1": {"number_of_timesteps_per_hour": 1}},"#,
        )
        .replace(
            r#""Heating Setpoint": {"hourly_value": 21}"#,
            r#""Heating Setpoint": {"hourly_value": 0}"#,
        )
        .replace(
            r#""Cooling Setpoint": {"hourly_value": 24}"#,
            r#""Cooling Setpoint": {"hourly_value": 15}"#,
        )
}

fn assert_direct_ideal_loads_fixture_runs(
    name: &str,
    fixture: &str,
    expected_runtime_class: &str,
    expected_capability_id: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    let case_dir = unique_case_dir(name)?;
    let input_path = case_dir.join("ideal-loads.epJSON");
    let weather_path = case_dir.join("weather.epw");
    let output_dir = case_dir.join("out");
    write_text(&input_path, fixture)?;
    let weather_path = if expected_runtime_class == "ideal-loads-direct-zone-coupled-compatibility"
    {
        write_text(&weather_path, ONE_DAY_EPW)?;
        Some(weather_path)
    } else {
        None
    };

    let outcome = run_arbitrary_idf(&RunConfig {
        input_path,
        weather_path,
        output_dir: output_dir.clone(),
        mode: RunMode::Compatibility,
        partial_policy: PartialRunPolicy::Deny,
        output_format: RunOutputFormat::RustNative,
        overwrite: true,
        keep_intermediate: true,
        trace_level: TraceLevel::Normal,
        trace_selection: TraceSelection::default(),
        fail_on_warning: false,
        dry_run: false,
        oracle_baseline: false,
        compare_oracle: false,
        json_stdout: false,
        oracle_root: None,
        hours: Some(1),
    })?;

    assert_eq!(outcome.exit_code, RunExitCode::Success);
    assert_eq!(
        outcome.support_status,
        SupportStatus::SupportedCompatibility
    );
    assert_eq!(
        outcome.run_result_state,
        RunResultState::SupportedCompatibilityRun
    );
    assert_output_manifest(&output_dir, SUPPORTED_RUNTIME_MANIFEST)?;

    let summary = read_json(&output_dir.join("run-summary.json"))?;
    assert_eq!(summary["status"], "success");
    assert_eq!(summary["support"]["status"], "supported-compatibility");
    assert_eq!(
        summary["support"]["run_result_state"],
        "supported_compatibility_run"
    );
    assert_eq!(summary["support"]["runtime_class"], expected_runtime_class);
    assert_eq!(
        summary["support"]["matched_capability_ids"][0],
        expected_capability_id
    );
    assert_eq!(
        summary["rust_runtime"]["runtime_class"],
        expected_runtime_class
    );
    assert_eq!(summary["source_order_gate"]["matches"], true);
    Ok(summary)
}

fn read_json(path: &Path) -> Result<Value, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&contents)?)
}
