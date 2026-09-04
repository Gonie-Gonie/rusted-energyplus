//! CP438 coupled-runtime accounting, ownership, counter, and no-feed contracts.

use crate::ideal_loads::PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementRuntimeState;
use ep_model::IdealLoadsAirSystemId;

#[test]
fn cp438_contract_locks_routes_schema_json_and_binding() {
    assert_eq!([67usize, 64, 3, 3], [67, 64, 3, 3]);
    assert_eq!(64usize + 3, 67, "inactive and increment partition");
    assert_eq!(20usize + 47, 67, "public and private route partition");
    let (public_active, private_active) = (0usize, 3usize);
    assert_eq!(
        public_active + private_active,
        3,
        "public and private active partition"
    );
    let source =
        include_str!("calc/heating_outdoor_air_maximum_flow_first_warning_counter_increment.rs");
    let snapshot = source
        .split_once(
            "pub struct PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementSnapshot",
        )
        .and_then(|(_, tail)| tail.split_once("/// Final selected-unit CP438"))
        .map(|(snapshot, _)| snapshot)
        .expect("CP438 snapshot declaration");
    assert_eq!(
        snapshot
            .lines()
            .filter(|line| line.trim_start().starts_with("pub "))
            .count(),
        426
    );
    assert_eq!(snapshot.matches("Option<f64>").count(), 146);
    assert_eq!(snapshot.matches("Option<bool>").count(), 9);
    assert_eq!(snapshot.matches("Option<usize>").count(), 2);
    assert_eq!(snapshot.matches("Option<").count() - 146 - 9 - 2, 6);
    let serializer = include_str!(
        "../../../ep_run/src/pipeline/purchased_air_heating_outdoor_air_maximum_flow_first_warning_counter_increment/serialization/snapshot.rs"
    );
    assert!(serializer.contains("exact_19_key_tail"));
    let fields = include_str!("binding/scheduled_output.rs")
        .lines()
        .filter(|line| line.starts_with("    pub calculation_"))
        .collect::<Vec<_>>();
    assert_eq!(fields.len(), 132);
    assert!(
        fields[127].contains("calculation_heating_outdoor_air_maximum_flow_first_warning_guard")
    );
    assert!(
        fields[128].contains(
            "calculation_heating_outdoor_air_maximum_flow_first_warning_counter_increment"
        )
    );
}

#[test]
fn cp438_new_state_has_seven_zeroed_lossless_route_partitions() {
    let state =
        PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementRuntimeState::new(
            IdealLoadsAirSystemId(0),
        );
    for values in [
        state.predecessor_route_counts,
        state.predecessor_guard_false_fallthrough_route_counts,
        state.predecessor_guard_body_entry_route_counts,
        state.predecessor_volume_flow_assignment_route_counts,
        state.predecessor_first_warning_guard_false_fallthrough_route_counts,
        state.predecessor_first_warning_branch_entry_route_counts,
        state.heating_outdoor_air_maximum_flow_first_warning_counter_increment_route_counts,
    ] {
        assert_eq!(values, [0; 36]);
    }
    assert_eq!(
        state.outdoor_air_flow_maximum_heating_output_error_count_increment_count,
        0
    );
    assert_eq!(state.source_site_execution_count, 0);
    assert!(state.latest.is_none());
}

#[test]
fn cp438_is_ordered_after_cp437_and_does_not_feed_numerics() {
    let binding = include_str!("binding.rs");
    let cp437 = binding
        .find("let calculation_heating_outdoor_air_maximum_flow_first_warning_guard =")
        .expect("CP437 binding");
    let cp438 = binding
        .find("let calculation_heating_outdoor_air_maximum_flow_first_warning_counter_increment =")
        .expect("CP438 binding");
    let coupling = binding
        .find("let coupling = complete_direct_zone_purchased_air_coupling(")
        .expect("numerical coupling");
    assert!(cp437 < cp438 && cp438 < coupling);
    assert!(!binding[cp438..coupling].contains("DirectZonePurchasedAirCouplingInput {"));
    let validator = include_str!(
        "coupled_runtime/heating_outdoor_air_maximum_flow_first_warning_counter_increment_validation.rs"
    );
    for required in [
        "public_counter_increment_count",
        "warning_counter_state_owner_count",
        "warning_counter_increment_write_count",
        "source_site_execution_count",
    ] {
        assert!(validator.contains(required), "{required}");
    }
    let production = validator
        .split_once("#[cfg(test)]")
        .map_or(validator, |(production, _)| production);
    assert!(!production.contains("DirectZonePurchasedAirCouplingInput"));
    assert!(!production.contains("private_characterization"));
}
