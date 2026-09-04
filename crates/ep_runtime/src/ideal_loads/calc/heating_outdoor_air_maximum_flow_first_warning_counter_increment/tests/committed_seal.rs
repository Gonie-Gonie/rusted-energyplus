//! Focused CP438 retained-route, counter, and forgery tests.

use crate::ideal_loads::calc::{
    cp438_fixture_unit_for_successor_tests,
    heating_outdoor_air_maximum_flow_first_warning_counter_increment_committed_latest_route_and_outdoor_air_flow_maximum_heating_output_error_count as committed,
};

#[test]
fn cp438_committed_seal_is_retained_constant_time_and_active_counter_exact() {
    let source = include_str!("../release/committed.rs");
    for forbidden in [
        "route_from_committed_predecessor(",
        "retained_route_matches_snapshot_bounded",
        "snapshot_route(",
        "private_characterization",
        "DirectZonePurchasedAirCouplingInput",
        "ShowWarningError",
    ] {
        assert!(!source.contains(forbidden), "{forbidden}");
    }
    let (unit, snapshot, route, _) = cp438_fixture_unit_for_successor_tests();
    assert!(route.counter_increment_executed);
    assert_eq!(committed(&unit, snapshot), Some((route, 1)));
}

#[test]
fn cp438_committed_seal_rejects_witness_route_accounting_and_counter_forgeries() {
    let (mut unit, snapshot, _, _) = cp438_fixture_unit_for_successor_tests();
    let mut witness = snapshot;
    witness.heating_outdoor_air_maximum_flow_first_warning_counter_increment_executed ^= true;
    assert!(committed(&unit, witness).is_none(), "witness forgery");

    unit
        .calc_heating_outdoor_air_maximum_flow_first_warning_counter_increment
        .latest
        .as_mut()
        .expect("latest")
        .outdoor_air_flow_maximum_heating_output_error_count_increment_performed ^= true;
    assert!(committed(&unit, snapshot).is_none(), "latest forgery");
    unit
        .calc_heating_outdoor_air_maximum_flow_first_warning_counter_increment
        .latest
        .as_mut()
        .expect("latest")
        .outdoor_air_flow_maximum_heating_output_error_count_increment_performed ^= true;

    unit
        .calc_heating_outdoor_air_maximum_flow_first_warning_counter_increment
        .latest_route
        .as_mut()
        .expect("route")
        .counter_increment_executed ^= true;
    assert!(committed(&unit, snapshot).is_none(), "route forgery");
    unit
        .calc_heating_outdoor_air_maximum_flow_first_warning_counter_increment
        .latest_route
        .as_mut()
        .expect("route")
        .counter_increment_executed ^= true;

    unit
        .calc_heating_outdoor_air_maximum_flow_first_warning_counter_increment
        .source_site_execution_count += 1;
    assert!(committed(&unit, snapshot).is_none(), "accounting forgery");
    unit
        .calc_heating_outdoor_air_maximum_flow_first_warning_counter_increment
        .source_site_execution_count -= 1;

    unit
        .calc_heating_outdoor_air_maximum_flow_first_warning_guard
        .predecessor_route_counts[1] += 1;
    assert!(committed(&unit, snapshot).is_none(), "predecessor forgery");
    unit
        .calc_heating_outdoor_air_maximum_flow_first_warning_guard
        .predecessor_route_counts[1] -= 1;

    let counter = unit
        .calc_heating_outdoor_air_maximum_flow_first_warning_guard
        .outdoor_air_flow_maximum_heating_output_error_count;
    unit
        .calc_heating_outdoor_air_maximum_flow_first_warning_guard
        .outdoor_air_flow_maximum_heating_output_error_count = 0;
    assert!(committed(&unit, snapshot).is_none(), "counter forgery");
    unit
        .calc_heating_outdoor_air_maximum_flow_first_warning_guard
        .outdoor_air_flow_maximum_heating_output_error_count = counter;
}
