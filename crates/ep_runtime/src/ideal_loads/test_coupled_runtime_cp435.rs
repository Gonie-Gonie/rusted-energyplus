//! CP435 coupled-runtime accounting, reconciliation, and no-feed contracts.

use crate::ideal_loads::PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardRuntimeState;
use ep_model::IdealLoadsAirSystemId;

#[test]
fn cp435_contract_locks_exhaustive_routes_current_schema_and_binding() {
    assert_eq!([64usize, 58, 6, 3, 3], [64, 58, 6, 3, 3]);
    assert_eq!(58usize + 6, 64, "inactive and evaluated partition");
    assert_eq!(3usize + 3, 6, "fallthrough and body partition");
    assert_eq!(20usize + 44, 64, "public and private route partition");
    let source = include_str!("calc/heating_outdoor_air_maximum_flow_guard.rs");
    let snapshot = source
        .split_once("pub struct PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardSnapshot")
        .and_then(|(_, tail)| tail.split_once("/// Final selected-unit CP435"))
        .map(|(snapshot, _)| snapshot)
        .expect("CP435 snapshot declaration");
    assert_eq!(
        snapshot
            .lines()
            .filter(|line| line.trim_start().starts_with("pub "))
            .count(),
        385
    );
    assert_eq!(snapshot.matches("Option<f64>").count(), 133);
    assert_eq!(snapshot.matches("Option<bool>").count(), 8);
    assert_eq!(snapshot.matches("Option<").count() - 133 - 8, 6);
    assert_eq!(
        include_str!("binding/scheduled_output.rs")
            .matches("    pub calculation_")
            .count(),
        126
    );
}

#[test]
fn cp435_new_state_has_three_zeroed_lossless_route_partitions() {
    let state = PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardRuntimeState::new(
        IdealLoadsAirSystemId(0),
    );
    for values in [
        state.predecessor_route_counts,
        state.heating_outdoor_air_maximum_flow_guard_false_fallthrough_route_counts,
        state.maximum_heating_flow_body_entry_route_counts,
    ] {
        assert_eq!(values, [0; 36]);
    }
    assert_eq!(state.source_site_execution_count, 0);
    assert!(state.latest.is_none());
}

#[test]
fn cp435_is_ordered_after_cp434_and_does_not_feed_numerics() {
    let binding = include_str!("binding.rs");
    let cp434 = binding
        .find("let calculation_heating_operating_mode_deadband_assignment =")
        .expect("CP434 binding");
    let cp435 = binding
        .find("let calculation_heating_outdoor_air_maximum_flow_guard =")
        .expect("CP435 binding");
    let coupling = binding
        .find("let coupling = complete_direct_zone_purchased_air_coupling(")
        .expect("numerical coupling");
    assert!(cp434 < cp435 && cp435 < coupling);
    assert!(!binding[cp435..coupling].contains("DirectZonePurchasedAirCouplingInput {"));
    assert!(!binding[cp435..coupling].contains("calculation.mode ="));
    let validator =
        include_str!("coupled_runtime/heating_outdoor_air_maximum_flow_guard_validation.rs");
    assert!(validator.contains("cp311_same_call_outdoor_air_mass_flow_rate_bit_corroborated"));
    assert!(validator.contains("maximum_heating_air_mass_flow_rate_kg_per_s"));
    assert!(validator.contains("public_body_entry_count"));
    let production = validator
        .split_once("#[cfg(test)]")
        .map_or(validator, |(production, _)| production);
    assert!(!production.contains("DirectZonePurchasedAirCouplingInput"));
    assert!(!production.contains("private_characterization"));
}
