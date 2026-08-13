use super::*;
use crate::ideal_loads::PurchasedAirRuntimeState;
use crate::ideal_loads::calc::cooling_mixed_air_call::release_tests::release_case;
use crate::ideal_loads::calc::{
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_committed_latest_route,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_committed_latest_route_and_cp_air,
};

#[test]
fn cp419_route_owner_accepts_exact_latest_witness_count_route_ordinal_and_value() {
    let (runtime, key, snapshot) = completed_case();
    let route = cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_committed_latest_route(
        runtime.units.get(&key).expect("unit"),
        snapshot,
    )
    .expect("sealed CP419 route");
    assert!(route.active);
    assert_eq!(route.logical_index, 4);
}

#[test]
fn cp419_route_owner_rejects_latest_witness_count_route_ordinal_and_value_forgeries() {
    let (runtime, key, snapshot) = completed_case();
    let mut cases = Vec::new();

    let mut latest = runtime.clone();
    latest
        .units
        .get_mut(&key)
        .expect("unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment
        .latest = None;
    cases.push((latest, snapshot));

    let mut witness = snapshot;
    witness.cp_air_j_per_kg_k = witness
        .cp_air_j_per_kg_k
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    cases.push((runtime.clone(), witness));

    let mut count = runtime.clone();
    count
        .units
        .get_mut(&key)
        .expect("unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment
        .transition_count += 1;
    cases.push((count, snapshot));

    let mut route = runtime.clone();
    route
        .units
        .get_mut(&key)
        .expect("unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment
        .latest_route
        .as_mut()
        .expect("route")
        .active = false;
    cases.push((route, snapshot));

    let mut ordinal = runtime.clone();
    ordinal
        .units
        .get_mut(&key)
        .expect("unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment
        .latest_transition_ordinal = Some(0);
    cases.push((ordinal, snapshot));

    let mut value = runtime.clone();
    let latest = value
        .units
        .get_mut(&key)
        .expect("unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment
        .latest
        .as_mut()
        .expect("latest");
    latest.cp_air_j_per_kg_k = latest
        .cp_air_j_per_kg_k
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    let value_witness = *latest;
    cases.push((value, value_witness));

    for (case, witness) in cases {
        assert!(cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_committed_latest_route(
            case.units.get(&key).expect("unit"),
            witness,
        ).is_none());
    }
}

#[test]
fn cp419_route_owner_rejects_each_retained_route_component_forgery() {
    let (runtime, key, snapshot) = completed_case();
    let mut cases = Vec::new();

    let mut logical_index = runtime.clone();
    logical_index
        .units
        .get_mut(&key)
        .expect("unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment
        .latest_route
        .as_mut()
        .expect("route")
        .logical_index = 5;
    cases.push(logical_index);

    macro_rules! flip_route_marker {
        ($field:ident) => {{
            let mut case = runtime.clone();
            let route = case
                .units
                .get_mut(&key)
                .expect("unit")
                .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment
                .latest_route
                .as_mut()
                .expect("route");
            route.$field = !route.$field;
            cases.push(case);
        }};
    }
    flip_route_marker!(predecessor_guard_false_fallthrough);
    flip_route_marker!(predecessor_guard_body_entered);
    flip_route_marker!(predecessor_saturation_temperature_assignment_executed);
    flip_route_marker!(predecessor_saturation_temperature_mixed_air_limit_executed);
    flip_route_marker!(predecessor_supply_humidity_ratio_assignment_executed);
    flip_route_marker!(predecessor_supply_enthalpy_assignment_executed);
    flip_route_marker!(active);

    for case in cases {
        assert!(
            cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_committed_latest_route_and_cp_air(
                case.units.get(&key).expect("unit"),
                snapshot,
            )
            .is_none()
        );
    }
}

#[test]
fn cp419_route_owner_rejects_coordinated_latest_and_witness_identity_forgery() {
    let (runtime, key, snapshot) = completed_case();
    let mut cases = Vec::new();

    let mut system = runtime.clone();
    let latest = system
        .units
        .get_mut(&key)
        .expect("unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment
        .latest
        .as_mut()
        .expect("latest");
    latest.system = ep_model::IdealLoadsAirSystemId(latest.system.0.wrapping_add(1));
    let witness = *latest;
    cases.push((system, witness));

    let mut zone = runtime.clone();
    let latest = zone
        .units
        .get_mut(&key)
        .expect("unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment
        .latest
        .as_mut()
        .expect("latest");
    latest.controlled_zone = ep_model::ZoneId(latest.controlled_zone.0.wrapping_add(1));
    let witness = *latest;
    cases.push((zone, witness));

    let mut ordinal = runtime.clone();
    let latest = ordinal
        .units
        .get_mut(&key)
        .expect("unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment
        .latest
        .as_mut()
        .expect("latest");
    latest.parent_call_ordinal = latest.parent_call_ordinal.wrapping_add(1);
    let witness = *latest;
    cases.push((ordinal, witness));

    for (case, witness) in cases {
        assert!(
            cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_committed_latest_route_and_cp_air(
                case.units.get(&key).expect("unit"),
                witness,
            )
            .is_none()
        );
    }

    assert!(
        cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_committed_latest_route_and_cp_air(
            runtime.units.get(&key).expect("unit"),
            snapshot,
        )
        .is_some()
    );
}

fn completed_case() -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystemId,
    Snapshot,
) {
    let (mut runtime, system, _, _) = release_case();
    let (cp417_state, cp418_state, predecessor) =
        predecessor_fixture_with_state(4, false, false);
    let mut cp419_state = State::new(predecessor.system);
    let snapshot =
        advance(&mut cp419_state, predecessor, active_input(predecessor)).expect("CP419");
    let unit = runtime.units.get_mut(&system.id).expect("unit");
    unit.system = snapshot.system;
    unit.controlled_zone = Some(snapshot.controlled_zone);
    unit.init_call_count = snapshot.parent_call_ordinal;
    unit.calc_entry.call_count = snapshot.parent_call_ordinal;
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry =
        cp418_state;
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment =
        cp417_state;
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment =
        cp419_state;
    (runtime, system.id, snapshot)
}

pub(in crate::ideal_loads::calc) fn cp419_fixture_unit_for_successor_tests(
    snapshot: Snapshot,
) -> crate::ideal_loads::PurchasedAirUnitRuntimeState {
    let (mut runtime, system, _) = crate::ideal_loads::calc::cooling_positive_supply_capacity_limit_sensible_output_guard::tests::release_fixture::completed_cp340_case(-1_000.0, 1.0, true);
    let desired = crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_snapshot_route(snapshot).expect("CP419 route");
    let (mut cp417_state, mut cp418_state, _) = predecessor_fixture_with_state(desired.logical_index, snapshot.predecessor_post_saturation_capacity_limit_dehumidification_guard_else_branch_entered, desired.predecessor_guard_false_fallthrough);
    let predecessor = super::super::release::cp418_shape_for_test(snapshot);
    cp418_state.latest = Some(predecessor);
    let mut cp419_state = State::new(predecessor.system);
    let rebuilt = advance(&mut cp419_state, predecessor, active_input(predecessor)).expect("CP419");
    assert!(super::super::release::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_snapshots_match_bit_exact(rebuilt, snapshot));
    let unit = runtime.units.get_mut(&system.id).expect("unit");
    cp417_state.latest.as_mut().expect("CP417").system = snapshot.system;
    cp417_state.latest.as_mut().expect("CP417").controlled_zone = snapshot.controlled_zone;
    cp417_state.latest.as_mut().expect("CP417").parent_call_ordinal = snapshot.parent_call_ordinal;
    cp418_state.latest.as_mut().expect("CP418").system = snapshot.system;
    cp418_state.latest.as_mut().expect("CP418").controlled_zone = snapshot.controlled_zone;
    cp418_state.latest.as_mut().expect("CP418").parent_call_ordinal = snapshot.parent_call_ordinal;
    cp417_state.system = snapshot.system;
    cp418_state.system = snapshot.system;
    cp419_state.system = snapshot.system;
    cp419_state.latest = Some(snapshot);
    unit.system = snapshot.system;
    unit.controlled_zone = Some(snapshot.controlled_zone);
    unit.init_call_count = snapshot.parent_call_ordinal;
    unit.calc_entry.call_count = snapshot.parent_call_ordinal;
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry = cp418_state;
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment = cp417_state;
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment = cp419_state;
    unit.clone()
}
