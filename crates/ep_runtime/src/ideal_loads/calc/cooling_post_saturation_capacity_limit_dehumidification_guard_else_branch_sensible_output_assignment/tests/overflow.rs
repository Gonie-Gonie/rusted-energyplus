use super::*;

type Predecessor = crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentSnapshot;
type Setter = fn(&mut State, usize);

#[test]
fn every_mutable_scalar_and_all_ten_route_arrays_overflow_transactionally() {
    let predecessors = cp419_all_snapshots_for_successor_tests();
    let active = find(&predecessors, |predecessor, route| {
        predecessor.post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_executed
            && route.active
    });
    let inactive = find(&predecessors, |_, route| !route.active);
    let guard_false = find(&predecessors, |_, route| {
        route.predecessor_guard_false_fallthrough
    });
    let guard_body = find(&predecessors, |_, route| {
        route.predecessor_guard_body_entered
            && route.predecessor_saturation_temperature_assignment_executed
            && route.predecessor_saturation_temperature_mixed_air_limit_executed
            && route.predecessor_supply_humidity_ratio_assignment_executed
            && route.predecessor_supply_enthalpy_assignment_executed
    });
    let all_carriers = find(&predecessors, |predecessor, _| {
        predecessor.resulting_supply_humidity_ratio.is_some()
            && predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
            && predecessor.resulting_supply_temperature_c.is_some()
    });

    let mut tested = 0;
    tested += assert_overflows(
        active,
        &[
            |state, _| state.transition_count = usize::MAX,
            |state, index| state.predecessor_route_counts[index] = usize::MAX,
        ],
    );
    tested += assert_overflows(
        inactive,
        &[|state, _| state.inactive_transition_count = usize::MAX],
    );
    tested += assert_overflows(
        guard_false,
        &[|state, index| {
            state.predecessor_guard_false_fallthrough_route_counts[index] = usize::MAX
        }],
    );
    tested += assert_overflows(
        guard_body,
        &[
            |state, _| {
                state.predecessor_supply_temperature_saturation_assignment_count = usize::MAX
            },
            |state, _| {
                state.predecessor_supply_temperature_saturation_mixed_air_limit_count = usize::MAX
            },
            |state, _| state.predecessor_supply_humidity_ratio_assignment_count = usize::MAX,
            |state, _| state.predecessor_supply_enthalpy_assignment_count = usize::MAX,
            |state, index| state.predecessor_guard_body_entry_route_counts[index] = usize::MAX,
            |state, index| {
                state.predecessor_supply_temperature_saturation_assignment_route_counts[index] =
                    usize::MAX
            },
            |state, index| {
                state.predecessor_supply_temperature_mixed_air_limit_route_counts[index] =
                    usize::MAX
            },
            |state, index| {
                state.predecessor_supply_humidity_ratio_assignment_route_counts[index] = usize::MAX
            },
            |state, index| {
                state.predecessor_supply_enthalpy_assignment_route_counts[index] = usize::MAX
            },
        ],
    );
    tested += assert_overflows(
        active,
        &[
            |state, _| {
                state.predecessor_dehumidification_guard_else_branch_entry_count = usize::MAX
            },
            |state, _| {
                state.predecessor_dehumidification_guard_else_branch_cp_air_assignment_count =
                    usize::MAX
            },
            |state, _| {
                state.dehumidification_guard_else_branch_sensible_output_assignment_count =
                    usize::MAX
            },
            |state, index| {
                state.predecessor_dehumidification_guard_else_branch_entry_route_counts[index] =
                    usize::MAX
            },
            |state, index| {
                state
                    .predecessor_dehumidification_guard_else_branch_cp_air_assignment_route_counts
                    [index] = usize::MAX
            },
            |state, index| {
                state.dehumidification_guard_else_branch_sensible_output_assignment_route_counts
                    [index] = usize::MAX
            },
            |state, _| state.source_site_execution_count = usize::MAX,
            |state, _| state.supply_mass_flow_rate_owned_read_count = usize::MAX,
            |state, _| state.supply_mass_flow_rate_bit_corroboration_count = usize::MAX,
            |state, _| state.supply_mass_flow_rate_read_count = usize::MAX,
            |state, _| state.cp_air_owned_read_count = usize::MAX,
            |state, _| state.cp_air_read_count = usize::MAX,
            |state, _| state.supply_mass_flow_rate_times_cp_air_calculation_count = usize::MAX,
            |state, _| state.mixed_air_temperature_owned_read_count = usize::MAX,
            |state, _| state.mixed_air_temperature_read_count = usize::MAX,
            |state, _| state.supply_temperature_owned_read_count = usize::MAX,
            |state, _| state.supply_temperature_read_count = usize::MAX,
            |state, _| state.mixed_air_minus_supply_temperature_calculation_count = usize::MAX,
            |state, _| state.cooling_sensible_output_calculation_count = usize::MAX,
            |state, _| state.cooling_sensible_output_assignment_write_count = usize::MAX,
        ],
    );
    tested += assert_overflows(
        all_carriers,
        &[
            |state, _| state.cp419_supply_humidity_ratio_state_owner_count = usize::MAX,
            |state, _| state.unchanged_supply_humidity_ratio_preservation_count = usize::MAX,
            |state, _| state.cp419_supply_enthalpy_state_owner_count = usize::MAX,
            |state, _| state.unchanged_supply_enthalpy_preservation_count = usize::MAX,
            |state, _| state.cp419_supply_temperature_state_owner_count = usize::MAX,
            |state, _| state.unchanged_supply_temperature_preservation_count = usize::MAX,
        ],
    );
    assert_eq!(tested, 39);
}

fn assert_overflows(predecessor: Predecessor, setters: &[Setter]) -> usize {
    let route = predecessor_route(predecessor).expect("CP420 route");
    for setter in setters {
        let mut state = State::new(predecessor.system);
        setter(&mut state, route.logical_index);
        let before = state.clone();
        let input = route.active.then_some(ActiveInput {
            supply_mass_flow_rate_kg_per_s: 0.25,
            mixed_air_temperature_c: 17.0,
        });
        assert!(advance(&mut state, predecessor, input).is_none());
        assert_eq!(state, before);
    }
    setters.len()
}

fn find(
    predecessors: &[Predecessor],
    predicate: impl Fn(Predecessor, super::super::transition::RetainedRoute) -> bool,
) -> Predecessor {
    predecessors
        .iter()
        .copied()
        .find(|predecessor| {
            let route = predecessor_route(*predecessor).expect("CP420 route");
            predicate(*predecessor, route)
        })
        .expect("requested CP420 predecessor")
}
