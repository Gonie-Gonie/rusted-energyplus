use super::*;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_committed_latest_route_and_cooling_sensible_output as committed_output;

#[test]
fn cp420_output_owner_rejects_latest_route_count_ordinal_witness_and_value_forgeries() {
    let predecessors = cp419_all_snapshots_for_successor_tests();
    for active in [false, true] {
        let mut predecessor = predecessors
            .iter()
            .copied()
            .find(|snapshot| {
                snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_executed
                    == active
            })
            .expect("route");
        predecessor.parent_call_ordinal = 1;
        let mut state = State::new(predecessor.system);
        let input = active.then_some(ActiveInput {
            supply_mass_flow_rate_kg_per_s: 0.25,
            mixed_air_temperature_c: 17.0,
        });
        let snapshot = advance(&mut state, predecessor, input).expect("CP420");
        let unit = fixture_unit(state.clone(), snapshot);
        let (route, output) = committed_output(&unit, snapshot).expect("sealed CP420");
        assert_eq!(route.active, active);
        assert_eq!(output.is_some(), active);

        let mut cases = Vec::new();
        let mut missing = unit.clone();
        missing
            .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment
            .latest = None;
        cases.push((missing, snapshot));

        let mut witness = snapshot;
        if active {
            witness.cooling_sensible_output_w = witness
                .cooling_sensible_output_w
                .map(|value| f64::from_bits(value.to_bits() ^ 1));
        } else {
            witness.parent_call_ordinal += 1;
        }
        cases.push((unit.clone(), witness));

        let mut count = unit.clone();
        count
            .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment
            .transition_count += 1;
        cases.push((count, snapshot));

        let mut ordinal = unit.clone();
        ordinal
            .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment
            .latest_transition_ordinal = Some(0);
        cases.push((ordinal, snapshot));

        let mut route_forged = unit.clone();
        route_forged
            .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment
            .latest_route
            .as_mut()
            .expect("route")
            .logical_index = (route.logical_index + 1) % 36;
        cases.push((route_forged, snapshot));

        let route_mutations: [fn(
            &mut super::super::transition::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentCommittedRoute,
        ); 7] = [
            |route| {
                route.predecessor_guard_false_fallthrough =
                    !route.predecessor_guard_false_fallthrough
            },
            |route| {
                route.predecessor_guard_body_entered = !route.predecessor_guard_body_entered
            },
            |route| {
                route.predecessor_saturation_temperature_assignment_executed =
                    !route.predecessor_saturation_temperature_assignment_executed
            },
            |route| {
                route.predecessor_saturation_temperature_mixed_air_limit_executed =
                    !route.predecessor_saturation_temperature_mixed_air_limit_executed
            },
            |route| {
                route.predecessor_supply_humidity_ratio_assignment_executed =
                    !route.predecessor_supply_humidity_ratio_assignment_executed
            },
            |route| {
                route.predecessor_supply_enthalpy_assignment_executed =
                    !route.predecessor_supply_enthalpy_assignment_executed
            },
            |route| route.active = !route.active,
        ];
        for mutate in route_mutations {
            let mut forged = unit.clone();
            mutate(
                forged
                    .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment
                    .latest_route
                    .as_mut()
                    .expect("route"),
            );
            cases.push((forged, snapshot));
        }

        if active {
            let mut value = unit.clone();
            let latest = value
                .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment
                .latest
                .as_mut()
                .expect("latest");
            latest.cooling_sensible_output_w = latest
                .cooling_sensible_output_w
                .map(|value| f64::from_bits(value.to_bits() ^ 1));
            let coordinated = *latest;
            cases.push((value, coordinated));
        }

        for (case, forged_witness) in cases {
            assert!(committed_output(&case, forged_witness).is_none());
        }
    }
}

fn fixture_unit(
    state: State,
    snapshot: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentSnapshot,
) -> crate::ideal_loads::PurchasedAirUnitRuntimeState {
    let (runtime, system, _, _) =
        crate::ideal_loads::calc::cooling_mixed_air_call::release_tests::release_case();
    let mut unit = runtime.units.get(&system.id).expect("unit").clone();
    unit.system = snapshot.system;
    unit.controlled_zone = Some(snapshot.controlled_zone);
    unit.init_call_count = snapshot.parent_call_ordinal;
    unit.calc_entry.call_count = snapshot.parent_call_ordinal;
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment =
        state;
    let predecessor = &mut unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment;
    predecessor.system = snapshot.system;
    predecessor.transition_count = snapshot.parent_call_ordinal;
    predecessor.predecessor_supply_temperature_saturation_assignment_count = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment
        .predecessor_supply_temperature_saturation_assignment_count;
    predecessor.predecessor_supply_temperature_saturation_mixed_air_limit_count = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment
        .predecessor_supply_temperature_saturation_mixed_air_limit_count;
    predecessor.predecessor_supply_humidity_ratio_assignment_count = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment
        .predecessor_supply_humidity_ratio_assignment_count;
    predecessor.predecessor_supply_enthalpy_assignment_count = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment
        .predecessor_supply_enthalpy_assignment_count;
    predecessor.predecessor_dehumidification_guard_else_branch_entry_count = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment
        .predecessor_dehumidification_guard_else_branch_entry_count;
    predecessor.dehumidification_guard_else_branch_cp_air_assignment_count = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment
        .predecessor_dehumidification_guard_else_branch_cp_air_assignment_count;
    predecessor.cp418_supply_humidity_ratio_state_owner_count = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment
        .cp419_supply_humidity_ratio_state_owner_count;
    predecessor.cp418_supply_enthalpy_state_owner_count = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment
        .cp419_supply_enthalpy_state_owner_count;
    predecessor.cp418_supply_temperature_state_owner_count = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment
        .cp419_supply_temperature_state_owner_count;
    predecessor.predecessor_route_counts = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment
        .predecessor_route_counts;
    predecessor.predecessor_guard_false_fallthrough_route_counts = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment
        .predecessor_guard_false_fallthrough_route_counts;
    predecessor.predecessor_guard_body_entry_route_counts = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment
        .predecessor_guard_body_entry_route_counts;
    predecessor.predecessor_supply_temperature_saturation_assignment_route_counts = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment
        .predecessor_supply_temperature_saturation_assignment_route_counts;
    predecessor.predecessor_supply_temperature_mixed_air_limit_route_counts = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment
        .predecessor_supply_temperature_mixed_air_limit_route_counts;
    predecessor.predecessor_supply_humidity_ratio_assignment_route_counts = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment
        .predecessor_supply_humidity_ratio_assignment_route_counts;
    predecessor.predecessor_supply_enthalpy_assignment_route_counts = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment
        .predecessor_supply_enthalpy_assignment_route_counts;
    predecessor.predecessor_dehumidification_guard_else_branch_entry_route_counts = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment
        .predecessor_dehumidification_guard_else_branch_entry_route_counts;
    predecessor.dehumidification_guard_else_branch_cp_air_assignment_route_counts = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment
        .predecessor_dehumidification_guard_else_branch_cp_air_assignment_route_counts;
    unit
}
