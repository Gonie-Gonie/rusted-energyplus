use super::*;
use crate::ideal_loads::PurchasedAirRuntimeState;
use crate::ideal_loads::calc::cooling_mixed_air_call::release_tests::release_case;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_committed_latest_route;

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

fn completed_case() -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystemId,
    Snapshot,
) {
    let (mut runtime, system, _, _) = release_case();
    let (cp418_state, predecessor) = predecessor_fixture_with_state(4, false, false);
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
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment =
        cp419_state;
    (runtime, system.id, snapshot)
}
