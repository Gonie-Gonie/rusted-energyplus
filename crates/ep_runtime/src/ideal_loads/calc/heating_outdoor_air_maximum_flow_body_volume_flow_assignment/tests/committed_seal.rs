//! Focused CP436 retained-route seal and coordinated-forgery tests.

use crate::ideal_loads::calc::{
    cp436_fixture_unit_for_successor_tests,
    heating_outdoor_air_maximum_flow_body_volume_flow_assignment_committed_latest_route as committed,
};

#[test]
fn cp436_committed_seal_is_retained_constant_time_and_owner_lazy() {
    let source = include_str!("../release/committed.rs");
    for forbidden in [
        "heating_outdoor_air_maximum_flow_body_volume_flow_assignment_route_from_committed_predecessor",
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
            .matches("heating_outdoor_air_maximum_flow_guard_committed_latest_route(")
            .count(),
        1,
    );
    let (unit, snapshot, route, owner) = cp436_fixture_unit_for_successor_tests();
    assert_eq!(committed(&unit, snapshot, owner), Some(route));
    assert!(
        owner.is_none(),
        "heating path must not acquire cooling owner"
    );
}

#[test]
fn cp436_committed_seal_rejects_latest_witness_route_and_accounting_forgeries() {
    let (unit, snapshot, _, owner) = cp436_fixture_unit_for_successor_tests();
    let mut witness = snapshot;
    witness.heating_outdoor_air_maximum_flow_body_volume_flow_assignment_executed ^= true;
    assert!(committed(&unit, witness, owner).is_none(), "witness forgery");

    let mut forged = unit.clone();
    forged
        .calc_heating_outdoor_air_maximum_flow_body_volume_flow_assignment
        .latest
        .as_mut()
        .expect("latest")
        .local_outdoor_air_volume_flow_rate_assignment_performed ^= true;
    assert!(committed(&forged, snapshot, owner).is_none(), "latest forgery");

    forged = unit.clone();
    forged
        .calc_heating_outdoor_air_maximum_flow_body_volume_flow_assignment
        .latest_route
        .as_mut()
        .expect("route")
        .assignment_executed ^= true;
    assert!(committed(&forged, snapshot, owner).is_none(), "route forgery");

    forged = unit.clone();
    forged
        .calc_heating_outdoor_air_maximum_flow_body_volume_flow_assignment
        .source_site_execution_count += 1;
    assert!(
        committed(&forged, snapshot, owner).is_none(),
        "accounting forgery"
    );

    forged = unit.clone();
    forged
        .calc_heating_outdoor_air_maximum_flow_body_volume_flow_assignment
        .latest_transition_ordinal = Some(0);
    assert!(committed(&forged, snapshot, owner).is_none(), "ordinal forgery");

    forged = unit.clone();
    forged
        .calc_heating_outdoor_air_maximum_flow_guard
        .predecessor_route_counts[1] += 1;
    assert!(
        committed(&forged, snapshot, owner).is_none(),
        "predecessor forgery"
    );
}
