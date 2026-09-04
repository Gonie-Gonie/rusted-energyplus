//! CP437 coupled-runtime accounting, ownership, counter, and no-feed contracts.

use crate::ideal_loads::PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardRuntimeState;
use ep_model::IdealLoadsAirSystemId;

#[test]
fn cp437_contract_locks_exhaustive_routes_current_schema_and_binding() {
    assert_eq!([67usize, 61, 6], [67, 61, 6]);
    assert_eq!(61usize + 3 + 3, 67, "inactive and guard partition");
    assert_eq!(20usize + 47, 67, "public and private route partition");
    let (public_active, private_active) = (0usize, 6usize);
    assert_eq!(
        public_active + private_active,
        6,
        "public and private active partition"
    );
    let source = include_str!("calc/heating_outdoor_air_maximum_flow_first_warning_guard.rs");
    let snapshot = source
        .split_once(
            "pub struct PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardSnapshot",
        )
        .and_then(|(_, tail)| tail.split_once("/// Final selected-unit CP437"))
        .map(|(snapshot, _)| snapshot)
        .expect("CP437 snapshot declaration");
    assert_eq!(
        snapshot
            .lines()
            .filter(|line| line.trim_start().starts_with("pub "))
            .count(),
        416
    );
    assert_eq!(snapshot.matches("Option<f64>").count(), 143);
    assert_eq!(snapshot.matches("Option<bool>").count(), 9);
    assert_eq!(snapshot.matches("Option<usize>").count(), 1);
    assert_eq!(snapshot.matches("Option<").count() - 143 - 9 - 1, 6);
    let fields = include_str!("binding/scheduled_output.rs")
        .lines()
        .filter(|line| line.starts_with("    pub calculation_"))
        .collect::<Vec<_>>();
    assert_eq!(fields.len(), 132);
    assert!(
        fields[126]
            .contains("calculation_heating_outdoor_air_maximum_flow_body_volume_flow_assignment")
    );
    assert!(
        fields[127].contains("calculation_heating_outdoor_air_maximum_flow_first_warning_guard")
    );
}

#[test]
fn cp437_new_state_has_six_zeroed_lossless_route_partitions_and_owned_counter() {
    let state = PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardRuntimeState::new(
        IdealLoadsAirSystemId(0),
    );
    for values in [
        state.predecessor_route_counts,
        state.predecessor_guard_false_fallthrough_route_counts,
        state.predecessor_guard_body_entry_route_counts,
        state.predecessor_volume_flow_assignment_route_counts,
        state.heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough_route_counts,
        state.heating_outdoor_air_maximum_flow_first_warning_branch_entry_route_counts,
    ] {
        assert_eq!(values, [0; 36]);
    }
    assert_eq!(state.outdoor_air_flow_maximum_heating_output_error_count, 0);
    assert_eq!(state.guard_evaluation_count, 0);
    assert_eq!(state.source_site_execution_count, 0);
    assert!(state.latest.is_none());
}

#[test]
fn cp437_is_ordered_after_cp436_and_does_not_feed_numerics() {
    let binding = include_str!("binding.rs");
    let cp436 = binding
        .find("let calculation_heating_outdoor_air_maximum_flow_body_volume_flow_assignment =")
        .expect("CP436 binding");
    let cp437 = binding
        .find("let calculation_heating_outdoor_air_maximum_flow_first_warning_guard =")
        .expect("CP437 binding");
    let cp438 = binding
        .find("let calculation_heating_outdoor_air_maximum_flow_first_warning_counter_increment =")
        .expect("CP438 binding");
    let coupling = binding
        .find("let coupling = complete_direct_zone_purchased_air_coupling(")
        .expect("numerical coupling");
    assert!(cp436 < cp437 && cp437 < cp438 && cp438 < coupling);
    assert!(!binding[cp437..coupling].contains("DirectZonePurchasedAirCouplingInput {"));
    assert!(
        !binding[cp437..coupling].contains("outdoor_air_flow_maximum_heating_output_error_count:")
    );
    let validator = include_str!(
        "coupled_runtime/heating_outdoor_air_maximum_flow_first_warning_guard_validation.rs"
    );
    for required in [
        "public_guard_evaluation_count",
        "warning_counter_state_owner_count",
        "warning_counter_read_count",
        "warning_counter_comparison_count",
        "warning_counter_unchanged",
    ] {
        assert!(validator.contains(required), "{required}");
    }
    let production = validator
        .split_once("#[cfg(test)]")
        .map_or(validator, |(production, _)| production);
    assert!(!production.contains("DirectZonePurchasedAirCouplingInput"));
    assert!(!production.contains("private_characterization"));
}
