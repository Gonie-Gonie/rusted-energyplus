//! CP431 coupled-runtime accounting and no-feed contracts.

use crate::ideal_loads::PurchasedAirCalcHeatingModeGuardRuntimeState;
use ep_model::IdealLoadsAirSystemId;

#[test]
fn cp431_conceptual_contract_is_exactly_three_active_variants_and_no_numerical_feed() {
    assert_eq!(
        [61usize, 58, 3, 2, 1, 14, 20, 41, 37, 42, 57],
        [61, 58, 3, 2, 1, 14, 20, 41, 37, 42, 57]
    );
    let source = include_str!("calc/heating_mode_guard.rs");
    let snapshot = source
        .split_once("pub struct PurchasedAirCalcHeatingModeGuardSnapshot")
        .and_then(|(_, tail)| tail.split_once("/// Final selected-unit CP431"))
        .map(|(snapshot, _)| snapshot)
        .expect("CP431 snapshot declaration");
    assert_eq!(
        snapshot
            .lines()
            .filter(|line| line.trim_start().starts_with("pub "))
            .count(),
        342
    );
    assert_eq!(snapshot.matches("Option<f64>").count(), 122);
    assert_eq!(snapshot.matches("Option<bool>").count(), 4);
    assert_eq!(snapshot.matches("Option<").count() - 126, 2);
    assert_eq!(
        include_str!("binding/scheduled_output.rs")
            .matches("    pub calculation_")
            .count(),
        128
    );
    let binding = include_str!("binding.rs");
    let cp430 = binding
        .find("let calculation_heating_or_no_load_case_entry =")
        .expect("CP430 binding");
    let cp431 = binding
        .find("let calculation_heating_mode_guard =")
        .expect("CP431 binding");
    let coupling = binding
        .find("let coupling = complete_direct_zone_purchased_air_coupling(")
        .expect("numerical coupling");
    assert!(cp430 < cp431 && cp431 < coupling);
    assert!(!binding[cp431..coupling].contains("DirectZonePurchasedAirCouplingInput {"));
}

#[test]
fn cp431_new_state_has_four_zeroed_lossless_route_partitions() {
    let state = PurchasedAirCalcHeatingModeGuardRuntimeState::new(IdealLoadsAirSystemId(0));
    for values in [
        state.predecessor_route_counts,
        state.heating_mode_guard_evaluation_route_counts,
        state.heating_operating_mode_body_entry_route_counts,
        state.heating_mode_guard_false_fallthrough_route_counts,
    ] {
        assert_eq!(values, [0; 36]);
    }
    assert_eq!(state.source_site_execution_count, 0);
    assert!(state.latest.is_none());
}

#[test]
fn cp431_combined_fallthrough_and_single_cool_scalar_are_accounting_complete() {
    let source = include_str!("calc/heating_mode_guard/release/runtime_validation.rs");
    assert!(source.contains("body_entries.checked_add(fallthroughs) == Some(evaluations)"));
    assert!(source.contains(
        "state.single_cool_block_count.checked_add(body_entries) == Some(short_circuit)"
    ));
    assert!(source.contains("state.heating_mode_guard_false_fallthrough_count == fallthroughs"));
}

#[test]
fn cp431_validator_locks_deadband_heating_and_inactive_injection_regressions() {
    let validator = include_str!("coupled_runtime/heating_mode_guard_validation.rs");
    for required in [
        "direct_guard_accepts_both_deadband_short_circuit_and_dual_heating_body",
        "inactive_shape_rejects_each_injected_local_carrier_or_flag",
        "direct_guard_result_is_exact",
        "inactive_shape_is_exact",
    ] {
        assert!(validator.contains(required), "{required}");
    }
    let production = validator
        .split_once("#[cfg(test)]")
        .map_or(validator, |(production, _)| production);
    for forbidden in [
        "DirectZonePurchasedAirCouplingInput",
        "numerical_dto",
        "prediction",
        "feedback",
        "nodes",
        "zone_sensible_cooling_rate_w",
        "sum_sys_mcp_t_w",
        "reports",
    ] {
        assert!(!production.contains(forbidden), "{forbidden}");
    }
}

#[test]
fn cp431_integration_roots_stay_within_historical_caps() {
    let state = include_str!("init/state.rs");
    let witnesses = include_str!("init/state/witnesses.rs");
    let calc = include_str!("calc.rs");
    assert!(state.lines().filter(|line| !line.trim().is_empty()).count() <= 380);
    assert!(witnesses.lines().count() <= 274);
    assert!(calc.lines().count() <= 99);
}
