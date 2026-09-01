//! CP434 coupled-runtime accounting, reconciliation, and no-feed contracts.

use crate::ideal_loads::PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentRuntimeState;
use ep_model::IdealLoadsAirSystemId;

#[test]
fn cp434_contract_locks_exhaustive_routes_current_schema_and_binding() {
    assert_eq!([61usize, 59, 2, 2], [61, 59, 2, 2]);
    assert_eq!(59usize + 2, 61, "inactive and assignment partition");
    assert_eq!(20usize + 41, 61, "public and private route partition");
    assert_eq!(1usize + 1, 2, "public and private active partition");
    let source = include_str!("calc/heating_operating_mode_deadband_assignment.rs");
    let snapshot = source
        .split_once("pub struct PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentSnapshot")
        .and_then(|(_, tail)| tail.split_once("/// Final selected-unit CP434"))
        .map(|(snapshot, _)| snapshot)
        .expect("CP434 snapshot declaration");
    assert_eq!(
        snapshot
            .lines()
            .filter(|line| line.trim_start().starts_with("pub "))
            .count(),
        361
    );
    assert_eq!(snapshot.matches("Option<f64>").count(), 128);
    assert_eq!(snapshot.matches("Option<bool>").count(), 4);
    assert_eq!(snapshot.matches("Option<").count() - 128 - 4, 4);
    assert_eq!(
        include_str!("binding/scheduled_output.rs")
            .matches("    pub calculation_")
            .count(),
        128
    );
}

#[test]
fn cp434_new_state_has_two_zeroed_lossless_route_partitions() {
    let state = PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentRuntimeState::new(
        IdealLoadsAirSystemId(0),
    );
    for values in [
        state.predecessor_route_counts,
        state.heating_operating_mode_deadband_assignment_route_counts,
    ] {
        assert_eq!(values, [0; 36]);
    }
    assert_eq!(state.source_site_execution_count, 0);
    assert!(state.latest.is_none());
}

#[test]
fn cp434_is_ordered_after_cp433_and_reconciles_without_feeding_numerics() {
    let binding = include_str!("binding.rs");
    let cp433 = binding
        .find("let calculation_heating_mode_guard_else_branch_entry =")
        .expect("CP433 binding");
    let cp434 = binding
        .find("let calculation_heating_operating_mode_deadband_assignment =")
        .expect("CP434 binding");
    let coupling = binding
        .find("let coupling = complete_direct_zone_purchased_air_coupling(")
        .expect("numerical coupling");
    assert!(cp433 < cp434 && cp434 < coupling);
    assert!(!binding[cp434..coupling].contains("DirectZonePurchasedAirCouplingInput {"));
    assert!(!binding[cp434..coupling].contains("calculation.mode ="));
    let validator =
        include_str!("coupled_runtime/heating_operating_mode_deadband_assignment_validation.rs");
    assert!(validator.contains("numerical_deadband_reconciliation_count"));
    assert!(validator.contains("assigned_heating_operating_mode_deadband"));
    assert!(validator.contains("calculation.mode == IdealLoadsSensibleMode::Deadband"));
    let production = validator
        .split_once("#[cfg(test)]")
        .map_or(validator, |(production, _)| production);
    assert!(!production.contains("DirectZonePurchasedAirCouplingInput"));
    assert!(!production.contains("private_characterization"));
    assert!(!production.lines().any(|line| {
        line.contains("calculation.mode =") && !line.contains("calculation.mode ==")
    }));
}
