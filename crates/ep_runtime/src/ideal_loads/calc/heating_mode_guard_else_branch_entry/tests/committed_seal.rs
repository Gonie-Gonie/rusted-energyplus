//! Focused CP433 retained-route seal and coordinated-forgery tests.

use crate::ideal_loads::calc::{
    cp433_fixture_unit_for_successor_tests,
    heating_mode_guard_else_branch_entry_committed_latest_route as committed,
};

#[test]
fn cp433_committed_seal_is_retained_constant_time_and_owner_lazy() {
    let source = include_str!("../release/committed.rs");
    for forbidden in [
        "heating_mode_guard_else_branch_entry_route_from_committed_predecessor",
        "retained_route_matches_snapshot_bounded",
        "snapshot_route(",
        "private_characterization",
        "DirectZonePurchasedAirCouplingInput",
        "calculation.mode",
        "IdealLoadsSensibleMode::Deadband",
    ] {
        assert!(!source.contains(forbidden), "{forbidden}");
    }
    assert_eq!(
        source
            .matches("heating_operating_mode_heat_assignment_committed_latest_route(")
            .count(),
        1,
    );
    let (unit, snapshot, route, owner) = cp433_fixture_unit_for_successor_tests();
    assert_eq!(committed(&unit, snapshot, owner), Some(route));
    assert!(
        owner.is_none(),
        "heating entry must not acquire the cooling owner"
    );
}

#[test]
fn cp433_committed_seal_rejects_latest_witness_route_and_accounting_forgeries() {
    let (unit, snapshot, _, owner) = cp433_fixture_unit_for_successor_tests();
    let mut cases = Vec::new();

    let mut witness = snapshot;
    witness.heating_mode_guard_else_branch_entered ^= true;
    cases.push((unit.clone(), witness));

    let mut latest = unit.clone();
    latest
        .calc_heating_mode_guard_else_branch_entry
        .latest
        .as_mut()
        .expect("latest")
        .heating_mode_guard_else_branch_entered ^= true;
    cases.push((latest, snapshot));

    let mut route = unit.clone();
    route
        .calc_heating_mode_guard_else_branch_entry
        .latest_route
        .as_mut()
        .expect("route")
        .entered ^= true;
    cases.push((route, snapshot));

    let mut accounting = unit.clone();
    accounting
        .calc_heating_mode_guard_else_branch_entry
        .source_site_execution_count += 1;
    cases.push((accounting, snapshot));

    let mut ordinal = unit.clone();
    ordinal
        .calc_heating_mode_guard_else_branch_entry
        .latest_transition_ordinal = Some(0);
    cases.push((ordinal, snapshot));

    let mut predecessor = unit.clone();
    predecessor
        .calc_heating_operating_mode_heat_assignment
        .predecessor_route_counts[1] += 1;
    cases.push((predecessor, snapshot));

    for (index, (forged, witness)) in cases.into_iter().enumerate() {
        assert!(
            committed(&forged, witness, owner).is_none(),
            "forgery {index}",
        );
    }
}
