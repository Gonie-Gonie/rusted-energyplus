//! Focused CP434 retained-route seal and coordinated-forgery tests.

use crate::ideal_loads::calc::{
    cp434_fixture_unit_for_successor_tests,
    heating_operating_mode_deadband_assignment_committed_latest_route as committed,
};

#[test]
fn cp434_committed_seal_is_retained_constant_time_and_owner_lazy() {
    let source = include_str!("../release/committed.rs");
    for forbidden in [
        "heating_operating_mode_deadband_assignment_route_from_committed_predecessor",
        "retained_route_matches_snapshot_bounded",
        "snapshot_route(",
        "private_characterization",
        "DirectZonePurchasedAirCouplingInput",
        "numerical_dto",
    ] {
        assert!(!source.contains(forbidden), "{forbidden}");
    }
    assert_eq!(
        source
            .matches("heating_mode_guard_else_branch_entry_committed_latest_route(")
            .count(),
        1,
    );
    let (unit, snapshot, route, owner) = cp434_fixture_unit_for_successor_tests();
    assert_eq!(committed(&unit, snapshot, owner), Some(route));
    assert!(owner.is_none(), "heating path must not acquire cooling owner");
}

#[test]
fn cp434_committed_seal_rejects_latest_witness_route_and_accounting_forgeries() {
    let (unit, snapshot, _, owner) = cp434_fixture_unit_for_successor_tests();
    let mut witness = snapshot;
    witness.heating_operating_mode_deadband_assignment_executed ^= true;
    assert!(committed(&unit, witness, owner).is_none(), "forgery 0");

    {
        let mut forged = unit.clone();
        forged
            .calc_heating_operating_mode_deadband_assignment
            .latest
            .as_mut()
            .expect("latest")
            .heating_operating_mode_deadband_assignment_performed ^= true;
        assert!(committed(&forged, snapshot, owner).is_none(), "forgery 1");
    }

    {
        let mut forged = unit.clone();
        forged
            .calc_heating_operating_mode_deadband_assignment
            .latest_route
            .as_mut()
            .expect("route")
            .assignment_executed ^= true;
        assert!(committed(&forged, snapshot, owner).is_none(), "forgery 2");
    }

    {
        let mut forged = unit.clone();
        forged
            .calc_heating_operating_mode_deadband_assignment
            .source_site_execution_count += 1;
        assert!(committed(&forged, snapshot, owner).is_none(), "forgery 3");
    }

    {
        let mut forged = unit.clone();
        forged
            .calc_heating_operating_mode_deadband_assignment
            .latest_transition_ordinal = Some(0);
        assert!(committed(&forged, snapshot, owner).is_none(), "forgery 4");
    }

    {
        let mut forged = unit.clone();
        forged
            .calc_heating_mode_guard_else_branch_entry
            .predecessor_route_counts[1] += 1;
        assert!(committed(&forged, snapshot, owner).is_none(), "forgery 5");
    }
}
