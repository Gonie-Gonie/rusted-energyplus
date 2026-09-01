//! Focused CP437 retained-route, accounting, and canonical-counter seal tests.

use crate::ideal_loads::calc::{
    cp437_fixture_unit_for_successor_tests,
    heating_outdoor_air_maximum_flow_first_warning_guard_committed_latest_route_and_outdoor_air_flow_maximum_heating_output_error_count as committed,
};

#[test]
fn cp437_committed_seal_is_retained_constant_time_counter_exact_and_owner_lazy() {
    let source = include_str!("../release/committed.rs");
    for forbidden in [
        "heating_outdoor_air_maximum_flow_first_warning_guard_route_from_committed_predecessor",
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
            .matches(
                "heating_outdoor_air_maximum_flow_body_volume_flow_assignment_committed_latest_route(",
            )
            .count(),
        1,
    );
    assert!(source.contains("counter <= 1"));
    let (unit, snapshot, route, owner) = cp437_fixture_unit_for_successor_tests();
    assert_eq!(committed(&unit, snapshot, owner), Some((route, 0)));
    assert!(
        owner.is_none(),
        "heating path must not acquire cooling owner"
    );
}

#[test]
fn cp437_committed_seal_rejects_latest_witness_route_accounting_and_counter_forgeries() {
    let (unit, snapshot, _, owner) = cp437_fixture_unit_for_successor_tests();
    let mut witness = snapshot;
    witness.heating_outdoor_air_maximum_flow_first_warning_branch_entered ^= true;
    assert!(committed(&unit, witness, owner).is_none(), "witness forgery");

    let mut forged = unit.clone();
    forged
        .calc_heating_outdoor_air_maximum_flow_first_warning_guard
        .latest
        .as_mut()
        .expect("latest")
        .outdoor_air_flow_maximum_heating_output_error_count_read ^= true;
    assert!(committed(&forged, snapshot, owner).is_none(), "latest forgery");

    forged = unit.clone();
    forged
        .calc_heating_outdoor_air_maximum_flow_first_warning_guard
        .latest_route
        .as_mut()
        .expect("route")
        .first_warning_branch_entered ^= true;
    assert!(committed(&forged, snapshot, owner).is_none(), "route forgery");

    forged = unit.clone();
    forged
        .calc_heating_outdoor_air_maximum_flow_first_warning_guard
        .source_site_execution_count += 1;
    assert!(
        committed(&forged, snapshot, owner).is_none(),
        "accounting forgery"
    );

    forged = unit.clone();
    forged
        .calc_heating_outdoor_air_maximum_flow_first_warning_guard
        .latest_transition_ordinal = Some(0);
    assert!(committed(&forged, snapshot, owner).is_none(), "ordinal forgery");

    forged = unit.clone();
    forged
        .calc_heating_outdoor_air_maximum_flow_body_volume_flow_assignment
        .predecessor_route_counts[1] += 1;
    assert!(
        committed(&forged, snapshot, owner).is_none(),
        "predecessor forgery"
    );

    forged = unit.clone();
    forged
        .calc_heating_outdoor_air_maximum_flow_first_warning_guard
        .outdoor_air_flow_maximum_heating_output_error_count = 1;
    assert!(
        committed(&forged, snapshot, owner).is_none(),
        "latest-owner counter mismatch"
    );

    forged = unit.clone();
    forged
        .calc_heating_outdoor_air_maximum_flow_first_warning_guard
        .outdoor_air_flow_maximum_heating_output_error_count = usize::MAX;
    assert!(
        committed(&forged, snapshot, owner).is_none(),
        "out-of-range counter"
    );
}
