//! CP433 coupled-runtime accounting and no-feed contracts.

use crate::ideal_loads::PurchasedAirCalcHeatingModeGuardElseBranchEntryRuntimeState;
use ep_model::IdealLoadsAirSystemId;

#[test]
fn cp433_contract_locks_61_transitions_two_entries_and_current_binding() {
    assert_eq!(
        [61usize, 59, 2, 20, 41, 37, 42, 57],
        [61, 59, 2, 20, 41, 37, 42, 57]
    );
    assert_eq!(59usize + 2, 61, "inactive and else-entry partition");
    assert_eq!(20usize + 41, 61, "public and private route partition");
    let source = include_str!("calc/heating_mode_guard_else_branch_entry.rs");
    let snapshot = source
        .split_once("pub struct PurchasedAirCalcHeatingModeGuardElseBranchEntrySnapshot")
        .and_then(|(_, tail)| tail.split_once("/// Final selected-unit CP433"))
        .map(|(snapshot, _)| snapshot)
        .expect("CP433 snapshot declaration");
    assert_eq!(
        snapshot
            .lines()
            .filter(|line| line.trim_start().starts_with("pub "))
            .count(),
        352
    );
    assert_eq!(snapshot.matches("Option<f64>").count(), 125);
    assert_eq!(snapshot.matches("Option<bool>").count(), 4);
    assert_eq!(snapshot.matches("Option<").count() - 125 - 4, 3);
    assert_eq!(
        include_str!("binding/scheduled_output.rs")
            .matches("    pub calculation_")
            .count(),
        124
    );
}

#[test]
fn cp433_new_state_has_two_zeroed_lossless_route_partitions() {
    let state =
        PurchasedAirCalcHeatingModeGuardElseBranchEntryRuntimeState::new(IdealLoadsAirSystemId(0));
    for values in [
        state.predecessor_route_counts,
        state.heating_mode_guard_else_branch_entry_route_counts,
    ] {
        assert_eq!(values, [0; 36]);
    }
    assert_eq!(state.source_site_execution_count, 0);
    assert!(state.latest.is_none());
}

#[test]
fn cp433_is_ordered_after_cp432_and_cannot_feed_or_reconcile_deadband_numerics() {
    let binding = include_str!("binding.rs");
    let cp432 = binding
        .find("let calculation_heating_operating_mode_heat_assignment =")
        .expect("CP432 binding");
    let cp433 = binding
        .find("let calculation_heating_mode_guard_else_branch_entry =")
        .expect("CP433 binding");
    let coupling = binding
        .find("let coupling = complete_direct_zone_purchased_air_coupling(")
        .expect("numerical coupling");
    assert!(cp432 < cp433 && cp433 < coupling);
    assert!(!binding[cp433..coupling].contains("DirectZonePurchasedAirCouplingInput {"));
    let validator =
        include_str!("coupled_runtime/heating_mode_guard_else_branch_entry_validation.rs");
    let production = validator
        .split_once("#[cfg(test)]")
        .map_or(validator, |(production, _)| production);
    assert!(production.contains("predecessor_heating_mode_guard_false_fallthrough_route_counts"));
    for forbidden in [
        "IdealLoadsSensibleMode::Deadband",
        "calculation.mode",
        "DirectZonePurchasedAirCouplingInput",
        "private_characterization",
    ] {
        assert!(!production.contains(forbidden), "{forbidden}");
    }
}
