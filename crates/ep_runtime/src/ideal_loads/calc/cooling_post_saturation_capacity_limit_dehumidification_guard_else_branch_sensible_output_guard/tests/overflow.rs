use super::*;

type Setter = fn(&mut State, usize);

#[test]
fn every_mutable_scalar_and_all_three_route_arrays_overflow_transactionally() {
    let predecessors = cp420_predecessors();
    let active = find(&predecessors, |predecessor| predecessor.cooling_sensible_output_w.is_some());
    let inactive = find(&predecessors, |predecessor| predecessor.cooling_sensible_output_w.is_none());
    let w_owner = find(&predecessors, |predecessor| {
        predecessor.resulting_supply_humidity_ratio.is_some()
            && predecessor.cooling_sensible_output_w.is_none()
    });
    let h_owner = find(&predecessors, |predecessor| {
        predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
            && predecessor.cooling_sensible_output_w.is_none()
    });
    let t_owner = find(&predecessors, |predecessor| {
        predecessor.resulting_supply_temperature_c.is_some()
            && predecessor.cooling_sensible_output_w.is_none()
    });

    let mut tested = 0;
    tested += assert_overflows(
        inactive,
        None,
        &[
            |state, _| state.transition_count = usize::MAX,
            |state, index| state.predecessor_route_counts[index] = usize::MAX,
            |state, _| state.inactive_transition_count = usize::MAX,
        ],
    );
    tested += assert_overflows(
        active,
        Some(false),
        &[
            |state, _| {
                state.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_evaluation_count = usize::MAX
            },
            |state, _| state.source_site_execution_count = usize::MAX,
            |state, _| state.cp420_cooling_sensible_output_owned_read_count = usize::MAX,
            |state, _| state.cooling_sensible_output_read_count = usize::MAX,
            |state, _| state.cp321_maximum_total_cooling_capacity_owned_read_count = usize::MAX,
            |state, _| {
                state.cp340_same_call_maximum_total_cooling_capacity_bit_corroboration_count =
                    usize::MAX
            },
            |state, _| state.maximum_total_cooling_capacity_read_count = usize::MAX,
            |state, _| {
                state.cooling_sensible_output_maximum_total_cooling_capacity_comparison_count =
                    usize::MAX
            },
            |state, _| {
                state.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_false_fallthrough_count = usize::MAX
            },
            |state, index| state.guard_false_fallthrough_route_counts[index] = usize::MAX,
        ],
    );
    tested += assert_overflows(
        active,
        Some(true),
        &[
            |state, _| {
                state.cooling_sensible_output_greater_than_or_equal_to_maximum_total_cooling_capacity_count = usize::MAX
            },
            |state, _| {
                state.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_adjustment_body_entry_count = usize::MAX
            },
            |state, index| state.adjustment_body_entry_route_counts[index] = usize::MAX,
        ],
    );
    tested += assert_overflows(
        w_owner,
        None,
        &[
            |state, _| state.cp420_supply_humidity_ratio_state_owner_count = usize::MAX,
            |state, _| state.unchanged_supply_humidity_ratio_preservation_count = usize::MAX,
        ],
    );
    tested += assert_overflows(
        h_owner,
        None,
        &[
            |state, _| state.cp420_supply_enthalpy_state_owner_count = usize::MAX,
            |state, _| state.unchanged_supply_enthalpy_preservation_count = usize::MAX,
        ],
    );
    tested += assert_overflows(
        t_owner,
        None,
        &[
            |state, _| state.cp420_supply_temperature_state_owner_count = usize::MAX,
            |state, _| state.unchanged_supply_temperature_preservation_count = usize::MAX,
        ],
    );
    assert_eq!(tested, 22);
}

fn assert_overflows(
    predecessor: Predecessor,
    body: Option<bool>,
    setters: &[Setter],
) -> usize {
    let route = successor_route_for(predecessor);
    let input = body.and_then(|body| active_input(predecessor, body));
    for setter in setters {
        let mut state = State::new(predecessor.system);
        setter(&mut state, route.logical_index);
        let before = state.clone();
        assert!(advance_validated(&mut state, predecessor, route, input).is_none());
        assert_eq!(state, before);
    }
    setters.len()
}

fn find(
    predecessors: &[Predecessor],
    predicate: impl Fn(Predecessor) -> bool,
) -> Predecessor {
    predecessors
        .iter()
        .copied()
        .find(|predecessor| predicate(*predecessor))
        .expect("requested predecessor")
}
