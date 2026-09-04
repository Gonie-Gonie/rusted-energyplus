//! Focused CP439 retained-route seal and forgery tests.

use crate::ideal_loads::calc::{
    cp439_fixture_unit_for_successor_tests,
    heating_outdoor_air_maximum_flow_first_warning_call_committed_latest_route as committed,
};

#[test]
fn cp439_committed_seal_is_retained_constant_time_and_active_call_exact() {
    let source = include_str!("../release/committed.rs");
    for forbidden in [
        "route_from_committed_predecessor(",
        "retained_route_matches_snapshot_bounded",
        "snapshot_route(",
        "private_characterization",
        "ShowWarningError",
        "ShowContinueError",
    ] {
        assert!(!source.contains(forbidden), "{forbidden}");
    }
    assert_eq!(
        source
            .matches("heating_outdoor_air_maximum_flow_first_warning_counter_increment_committed_latest_route_and_outdoor_air_flow_maximum_heating_output_error_count(")
            .count(),
        1,
    );
    let (unit, snapshot, route) = cp439_fixture_unit_for_successor_tests();
    assert!(route.first_warning_call_site_reached);
    assert_eq!(committed(&unit, snapshot), Some(route));
}

#[test]
fn cp439_committed_seal_rejects_witness_route_and_accounting_forgeries() {
    let (mut unit, snapshot, _) = cp439_fixture_unit_for_successor_tests();

    let mut witness = snapshot;
    witness.heating_outdoor_air_maximum_flow_first_warning_call_site_reached ^= true;
    assert!(committed(&unit, witness).is_none(), "witness forgery");

    unit
        .calc_heating_outdoor_air_maximum_flow_first_warning_call
        .latest
        .as_mut()
        .expect("latest")
        .heating_outdoor_air_maximum_flow_first_warning_call_site_reached ^= true;
    assert!(committed(&unit, snapshot).is_none(), "latest forgery");
    unit
        .calc_heating_outdoor_air_maximum_flow_first_warning_call
        .latest
        .as_mut()
        .expect("latest")
        .heating_outdoor_air_maximum_flow_first_warning_call_site_reached ^= true;

    unit
        .calc_heating_outdoor_air_maximum_flow_first_warning_call
        .latest_route
        .as_mut()
        .expect("route")
        .first_warning_call_site_reached ^= true;
    assert!(committed(&unit, snapshot).is_none(), "route forgery");
    unit
        .calc_heating_outdoor_air_maximum_flow_first_warning_call
        .latest_route
        .as_mut()
        .expect("route")
        .first_warning_call_site_reached ^= true;

    unit
        .calc_heating_outdoor_air_maximum_flow_first_warning_call
        .source_site_execution_count += 1;
    assert!(committed(&unit, snapshot).is_none(), "accounting forgery");
    unit
        .calc_heating_outdoor_air_maximum_flow_first_warning_call
        .source_site_execution_count -= 1;

    unit
        .calc_heating_outdoor_air_maximum_flow_first_warning_counter_increment
        .predecessor_route_counts[1] += 1;
    assert!(committed(&unit, snapshot).is_none(), "predecessor forgery");
}
