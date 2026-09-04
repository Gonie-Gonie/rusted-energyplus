//! Focused CP440 retained-route seal and forgery tests.

use crate::ideal_loads::calc::{
    cp440_fixture_unit_for_successor_tests,
    heating_outdoor_air_maximum_flow_continue_warning_call_committed_latest_route as committed,
};

#[test]
fn cp440_committed_seal_is_retained_constant_time_and_active_call_exact() {
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
            .matches("heating_outdoor_air_maximum_flow_first_warning_call_committed_latest_route(")
            .count(),
        1,
    );
    let (unit, snapshot, route) = cp440_fixture_unit_for_successor_tests();
    assert!(route.continue_warning_call_site_reached);
    assert_eq!(committed(&unit, snapshot), Some(route));
}

#[test]
fn cp440_committed_seal_rejects_witness_route_and_accounting_forgeries() {
    let (mut unit, snapshot, _) = cp440_fixture_unit_for_successor_tests();

    let mut witness = snapshot;
    witness.heating_outdoor_air_maximum_flow_continue_warning_call_site_reached ^= true;
    assert!(committed(&unit, witness).is_none(), "witness forgery");

    unit
        .calc_heating_outdoor_air_maximum_flow_continue_warning_call
        .latest
        .as_mut()
        .expect("latest")
        .heating_outdoor_air_maximum_flow_continue_warning_call_site_reached ^= true;
    assert!(committed(&unit, snapshot).is_none(), "latest forgery");
    unit
        .calc_heating_outdoor_air_maximum_flow_continue_warning_call
        .latest
        .as_mut()
        .expect("latest")
        .heating_outdoor_air_maximum_flow_continue_warning_call_site_reached ^= true;

    unit
        .calc_heating_outdoor_air_maximum_flow_continue_warning_call
        .latest_route
        .as_mut()
        .expect("route")
        .continue_warning_call_site_reached ^= true;
    assert!(committed(&unit, snapshot).is_none(), "route forgery");
    unit
        .calc_heating_outdoor_air_maximum_flow_continue_warning_call
        .latest_route
        .as_mut()
        .expect("route")
        .continue_warning_call_site_reached ^= true;

    unit
        .calc_heating_outdoor_air_maximum_flow_continue_warning_call
        .source_site_execution_count += 1;
    assert!(committed(&unit, snapshot).is_none(), "accounting forgery");
    unit
        .calc_heating_outdoor_air_maximum_flow_continue_warning_call
        .source_site_execution_count -= 1;

    unit
        .calc_heating_outdoor_air_maximum_flow_first_warning_call
        .predecessor_route_counts[1] += 1;
    assert!(
        committed(&unit, snapshot).is_none(),
        "predecessor forgery"
    );
}
