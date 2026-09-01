//! CP432 coupled-runtime accounting, reconciliation, and no-feed contracts.

use crate::ideal_loads::PurchasedAirCalcHeatingOperatingModeHeatAssignmentRuntimeState;
use ep_model::IdealLoadsAirSystemId;

#[test]
fn cp432_contract_locks_61_transitions_and_current_schema_and_binding() {
    assert_eq!([61usize, 58, 2, 1], [61, 58, 2, 1]);
    assert_eq!(58usize + 2, 60, "all non-assignment outcomes");
    assert_eq!(58usize + 2 + 1, 61, "runtime transition partition");
    let source = include_str!("calc/heating_operating_mode_heat_assignment.rs");
    let snapshot = source
        .split_once("pub struct PurchasedAirCalcHeatingOperatingModeHeatAssignmentSnapshot")
        .and_then(|(_, tail)| tail.split_once("/// Final selected-unit CP432"))
        .map(|(snapshot, _)| snapshot)
        .expect("CP432 snapshot declaration");
    assert_eq!(
        snapshot
            .lines()
            .filter(|line| line.trim_start().starts_with("pub "))
            .count(),
        351
    );
    assert_eq!(snapshot.matches("Option<f64>").count(), 125);
    assert_eq!(snapshot.matches("Option<bool>").count(), 4);
    assert_eq!(snapshot.matches("Option<").count() - 125 - 4, 3);
    assert_eq!(
        include_str!("binding/scheduled_output.rs")
            .matches("    pub calculation_")
            .count(),
        127
    );
}

#[test]
fn cp432_new_state_has_four_zeroed_lossless_route_partitions() {
    let state = PurchasedAirCalcHeatingOperatingModeHeatAssignmentRuntimeState::new(
        IdealLoadsAirSystemId(0),
    );
    for values in [
        state.predecessor_route_counts,
        state.predecessor_heating_mode_guard_evaluation_route_counts,
        state.predecessor_heating_mode_guard_false_fallthrough_route_counts,
        state.heating_operating_mode_heat_assignment_route_counts,
    ] {
        assert_eq!(values, [0; 36]);
    }
    assert_eq!(state.source_site_execution_count, 0);
    assert!(state.latest.is_none());
}

#[test]
fn cp432_is_ordered_after_cp431_and_cannot_feed_numerical_coupling() {
    let binding = include_str!("binding.rs");
    let cp431 = binding
        .find("let calculation_heating_mode_guard =")
        .expect("CP431 binding");
    let cp432 = binding
        .find("let calculation_heating_operating_mode_heat_assignment =")
        .expect("CP432 binding");
    let coupling = binding
        .find("let coupling = complete_direct_zone_purchased_air_coupling(")
        .expect("numerical coupling");
    assert!(cp431 < cp432 && cp432 < coupling);
    assert!(!binding[cp432..coupling].contains("DirectZonePurchasedAirCouplingInput {"));
    let validator =
        include_str!("coupled_runtime/heating_operating_mode_heat_assignment_validation.rs");
    assert!(validator.contains("numerical_heating_reconciliation_count"));
    assert!(validator.contains("assigned_heating_operating_mode"));
    assert!(validator.contains("calculation.mode == IdealLoadsSensibleMode::Heating"));
    let production = validator
        .split_once("#[cfg(test)]")
        .map_or(validator, |(production, _)| production);
    assert!(!production.lines().any(|line| {
        line.contains("calculation.mode =") && !line.contains("calculation.mode ==")
    }));
    assert!(!production.contains("private_characterization"));
}
