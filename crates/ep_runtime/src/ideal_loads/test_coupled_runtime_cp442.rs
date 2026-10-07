//! CP442 coupled-runtime structural entry, binding, and no-feed contracts.

use crate::ideal_loads::PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardElseBranchEntryRuntimeState;
use ep_model::IdealLoadsAirSystemId;

#[test]
fn cp442_contract_locks_routes_schema_json_and_current_binding() {
    assert_eq!([67usize, 64, 3, 3], [67, 64, 3, 3]);
    assert_eq!(64usize + 3, 67, "inactive and else-entry partition");
    assert_eq!(20usize + 47, 67, "public and private route partition");
    assert_eq!(
        [0usize, 3].into_iter().sum::<usize>(),
        3,
        "public and private active partition"
    );
    let source = include_str!(
        "calc/heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry.rs"
    );
    let snapshot = source
        .split_once(
            "pub struct PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardElseBranchEntrySnapshot",
        )
        .and_then(|(_, tail)| tail.split_once("/// Final selected-unit CP442"))
        .map(|(snapshot, _)| snapshot)
        .expect("CP442 snapshot declaration");
    assert_eq!(
        snapshot
            .lines()
            .filter(|line| line.trim_start().starts_with("pub "))
            .count(),
        430
    );
    assert_eq!(snapshot.matches("Option<f64>").count(), 146);
    assert_eq!(snapshot.matches("Option<bool>").count(), 9);
    assert_eq!(snapshot.matches("Option<usize>").count(), 2);
    assert_eq!(snapshot.matches("Option<").count() - 146 - 9 - 2, 6);
    assert!(
        snapshot
            .contains("heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entered")
    );
    let serializer = include_str!(
        "../../../ep_run/src/pipeline/purchased_air_heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry/serialization/snapshot.rs"
    );
    let serializer_production = serializer
        .split_once("#[cfg(test)]")
        .map_or(serializer, |(production, _)| production);
    assert!(serializer_production.contains("cp441_snapshot_json(predecessor)"));
    assert_eq!(serializer_production.matches("target.insert(").count(), 4);
    assert!(
        serializer_production
            .contains("heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entered")
    );
    let fields = include_str!("binding/scheduled_output.rs")
        .lines()
        .filter(|line| line.contains("pub calculation_"))
        .collect::<Vec<_>>();
    assert_eq!(fields.len(), 133);
    assert!(
        fields[131].contains(
            "calculation_heating_outdoor_air_maximum_flow_continue_warning_timestamp_call"
        )
    );
    assert!(fields[132].contains(
        "calculation_heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry"
    ));
}

#[test]
fn cp442_new_state_has_two_zeroed_lossless_route_partitions() {
    let state =
        PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardElseBranchEntryRuntimeState::new(
            IdealLoadsAirSystemId(0),
        );
    for values in [
        state.predecessor_route_counts,
        state.heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_route_counts,
    ] {
        assert_eq!(values, [0; 36]);
    }
    assert_eq!(
        state.heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_count,
        0
    );
    assert_eq!(state.source_site_execution_count, 0);
    assert!(state.latest.is_none());
}

#[test]
fn cp442_is_ordered_after_cp441_and_does_not_feed_numerics_or_services() {
    let binding = include_str!("binding.rs");
    let cp441 = binding
        .find("let calculation_heating_outdoor_air_maximum_flow_continue_warning_timestamp_call =")
        .expect("CP441 binding");
    let cp442 = binding
        .find("let calculation_heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry =")
        .expect("CP442 binding");
    let coupling = binding
        .find("let coupling = complete_direct_zone_purchased_air_coupling(")
        .expect("numerical coupling");
    assert!(cp441 < cp442 && cp442 < coupling);
    assert!(!binding[cp442..coupling].contains("DirectZonePurchasedAirCouplingInput {"));
    let validator = include_str!(
        "coupled_runtime/heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_validation.rs"
    );
    for required in [
        "predecessor_first_warning_guard_false_fallthrough_route_counts",
        "first_warning_guard_else_entry_route_alias",
        "public_else_entry_count",
        "source_site_execution_count",
    ] {
        assert!(validator.contains(required), "{required}");
    }
    let production = validator
        .split_once("#[cfg(test)]")
        .map_or(validator, |(production, _)| production);
    assert!(!production.contains("DirectZonePurchasedAirCouplingInput"));
    assert!(!production.contains("private_characterization"));
    for forbidden in [
        "message",
        "sink",
        "sqlite",
        "callback",
        "warning_counter_owner",
    ] {
        assert!(
            !production.to_ascii_lowercase().contains(forbidden),
            "{forbidden}"
        );
    }
}
