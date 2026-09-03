//! Focused CP435 retained-route seal and coordinated-forgery tests.

use crate::ideal_loads::calc::{
    cp435_fixture_unit_for_successor_tests,
    heating_outdoor_air_maximum_flow_guard_committed_latest_route as committed,
};

#[test]
fn cp435_committed_seal_is_retained_constant_time_and_owner_lazy() {
    let source = include_str!("../release/committed.rs");
    for forbidden in [
        "heating_outdoor_air_maximum_flow_guard_route_from_committed_predecessor",
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
            .matches("heating_operating_mode_deadband_assignment_committed_latest_route(")
            .count(),
        1,
    );
    let (unit, snapshot, route, owner) = cp435_fixture_unit_for_successor_tests();
    assert_eq!(committed(&unit, snapshot, owner), Some(route));
    assert!(owner.is_none(), "heating path must not acquire cooling owner");
}

#[test]
fn cp435_committed_seal_rejects_latest_witness_route_and_accounting_forgeries() {
    let (mut unit, snapshot, _, owner) = cp435_fixture_unit_for_successor_tests();
    let mut witness = snapshot;
    witness.maximum_heating_flow_body_entered ^= true;
    assert!(committed(&unit, witness, owner).is_none(), "forgery 0");

    unit
        .calc_heating_outdoor_air_maximum_flow_guard
        .latest
        .as_mut()
        .expect("latest")
        .heating_outdoor_air_maximum_flow_guard_false_fallthrough ^= true;
    assert!(committed(&unit, snapshot, owner).is_none(), "forgery 1");
    unit
        .calc_heating_outdoor_air_maximum_flow_guard
        .latest
        .as_mut()
        .expect("latest")
        .heating_outdoor_air_maximum_flow_guard_false_fallthrough ^= true;

    unit
        .calc_heating_outdoor_air_maximum_flow_guard
        .latest_route
        .as_mut()
        .expect("route")
        .body_entered ^= true;
    assert!(committed(&unit, snapshot, owner).is_none(), "forgery 2");
    unit
        .calc_heating_outdoor_air_maximum_flow_guard
        .latest_route
        .as_mut()
        .expect("route")
        .body_entered ^= true;

    unit
        .calc_heating_outdoor_air_maximum_flow_guard
        .source_site_execution_count += 1;
    assert!(committed(&unit, snapshot, owner).is_none(), "forgery 3");
    unit
        .calc_heating_outdoor_air_maximum_flow_guard
        .source_site_execution_count -= 1;

    let ordinal = unit
        .calc_heating_outdoor_air_maximum_flow_guard
        .latest_transition_ordinal;
    unit.calc_heating_outdoor_air_maximum_flow_guard
        .latest_transition_ordinal = Some(0);
    assert!(committed(&unit, snapshot, owner).is_none(), "forgery 4");
    unit.calc_heating_outdoor_air_maximum_flow_guard
        .latest_transition_ordinal = ordinal;

    unit
        .calc_heating_operating_mode_deadband_assignment
        .predecessor_route_counts[1] += 1;
    assert!(committed(&unit, snapshot, owner).is_none(), "forgery 5");
}
