//! Focused CP432 retained-route seal and coordinated-forgery tests.

use crate::ideal_loads::calc::{
    cp432_fixture_unit_for_successor_tests,
    heating_operating_mode_heat_assignment_committed_latest_route as committed,
};

#[test]
fn cp432_committed_seal_is_retained_constant_time_and_owner_lazy() {
    let source = include_str!("../release/committed.rs");
    for forbidden in [
        "cooling_entry_gate_committed_latest_heating_mode_guard_numeric_operands",
        "cooling_entry_gate_committed_latest_heating_mode_guard_temperature_control_type",
        "heating_operating_mode_heat_assignment_route_from_committed_predecessor",
        "retained_route_matches_snapshot_bounded",
        "snapshot_route(",
        "completed_",
        "private_characterization",
        " < ",
    ] {
        assert!(!source.contains(forbidden), "{forbidden}");
    }
    assert_eq!(
        source
            .matches("heating_mode_guard_committed_latest_route(")
            .count(),
        1,
    );
    let validation = include_str!("../release/snapshot_validation.rs");
    let start = validation
        .find("pub(super) fn committed_prefix_and_local_route_shape_match")
        .expect("committed local matcher");
    let end = validation[start..]
        .find("pub(super) fn prefix_and_local_shape_match")
        .map(|offset| start + offset)
        .expect("committed local matcher end");
    let committed_local = &validation[start..end];
    for forbidden in [
        "cooling_entry_gate_committed_latest_heating_mode_guard_numeric_operands",
        "cooling_entry_gate_committed_latest_heating_mode_guard_temperature_control_type",
        "SingleCool",
        " < ",
        "heating_operating_mode_heat_assignment_route_from_committed_predecessor",
        "local_shape_is_exact(",
        "snapshot_route(",
    ] {
        assert!(!committed_local.contains(forbidden), "{forbidden}");
    }

    let (unit, snapshot, route, owner) = cp432_fixture_unit_for_successor_tests();
    assert_eq!(committed(&unit, snapshot, owner), Some(route));
    assert!(owner.is_none(), "heating entry must not acquire the cooling owner");
}

#[test]
fn cp432_committed_seal_rejects_latest_witness_route_and_accounting_forgeries() {
    let (unit, snapshot, _, owner) = cp432_fixture_unit_for_successor_tests();
    let mut cases = Vec::new();

    let mut witness = snapshot;
    witness.heating_operating_mode_heat_assignment_performed ^= true;
    cases.push((unit.clone(), witness));

    let mut latest = unit.clone();
    latest
        .calc_heating_operating_mode_heat_assignment
        .latest
        .as_mut()
        .expect("latest")
        .heating_operating_mode_heat_assignment_executed ^= true;
    cases.push((latest, snapshot));

    let mut route = unit.clone();
    route
        .calc_heating_operating_mode_heat_assignment
        .latest_route
        .as_mut()
        .expect("route")
        .assignment_executed ^= true;
    cases.push((route, snapshot));

    let mut accounting = unit.clone();
    accounting
        .calc_heating_operating_mode_heat_assignment
        .source_site_execution_count += 1;
    cases.push((accounting, snapshot));

    let mut ordinal = unit.clone();
    ordinal
        .calc_heating_operating_mode_heat_assignment
        .latest_transition_ordinal = Some(0);
    cases.push((ordinal, snapshot));

    let mut predecessor = unit.clone();
    predecessor
        .calc_heating_mode_guard
        .predecessor_route_counts[1] += 1;
    cases.push((predecessor, snapshot));

    for (index, (forged, witness)) in cases.into_iter().enumerate() {
        assert!(
            committed(&forged, witness, owner).is_none(),
            "forgery {index}",
        );
    }
}
