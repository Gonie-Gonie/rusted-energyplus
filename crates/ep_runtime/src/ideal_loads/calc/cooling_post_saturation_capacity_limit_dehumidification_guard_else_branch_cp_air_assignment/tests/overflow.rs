//! CP419 transactional counter-overflow tests.

use super::*;

#[test]
fn entry_counter_overflow_is_transactional() {
    let predecessor = predecessor_fixture(4, false, false);
    let route = predecessor_route(predecessor).expect("entry route");
    assert!(route.active);
    let setters: &[fn(&mut State, usize)] = &[
        |state, _| state.transition_count = usize::MAX,
        |state, index| state.predecessor_route_counts[index] = usize::MAX,
        |state, _| state.dehumidification_guard_else_branch_cp_air_assignment_count = usize::MAX,
        |state, _| state.predecessor_dehumidification_guard_else_branch_entry_count = usize::MAX,
        |state, index| {
            state.dehumidification_guard_else_branch_cp_air_assignment_route_counts[index] =
                usize::MAX
        },
        |state, index| {
            state.predecessor_dehumidification_guard_else_branch_entry_route_counts[index] =
                usize::MAX
        },
        |state, _| state.source_site_execution_count = usize::MAX,
        |state, _| state.cp418_supply_temperature_state_owner_count = usize::MAX,
        |state, _| state.unchanged_supply_temperature_preservation_count = usize::MAX,
        |state, _| state.cp419_psychrometric_cp_air_state_owner_count = usize::MAX,
        |state, _| state.cp329_retained_mixed_air_humidity_ratio_owned_read_count = usize::MAX,
        |state, _| state.mixed_air_humidity_ratio_for_cp_air_read_count = usize::MAX,
        |state, _| state.psychrometric_cp_air_evaluation_count = usize::MAX,
        |state, _| state.cp_air_assignment_write_count = usize::MAX,
    ];
    assert_overflows_transactionally(predecessor, route.logical_index, setters);
}

#[test]
fn deep_inactive_counter_overflow_is_transactional() {
    let predecessor = predecessor_fixture(20, true, true);
    let route = predecessor_route(predecessor).expect("deep inactive route");
    assert!(!route.active && route.predecessor_supply_enthalpy_assignment_executed);
    #[rustfmt::skip]
    let setters: &[fn(&mut State, usize)] = &[
        |state, _| state.transition_count = usize::MAX,
        |state, index| state.predecessor_route_counts[index] = usize::MAX,
        |state, index| state.predecessor_guard_body_entry_route_counts[index] = usize::MAX,
        |state, _| state.predecessor_supply_temperature_saturation_assignment_count = usize::MAX,
        |state, index| state.predecessor_supply_temperature_saturation_assignment_route_counts[index] = usize::MAX,
        |state, _| state.predecessor_supply_temperature_saturation_mixed_air_limit_count = usize::MAX,
        |state, index| state.predecessor_supply_temperature_mixed_air_limit_route_counts[index] = usize::MAX,
        |state, _| state.predecessor_supply_humidity_ratio_assignment_count = usize::MAX,
        |state, index| state.predecessor_supply_humidity_ratio_assignment_route_counts[index] = usize::MAX,
        |state, _| state.predecessor_supply_enthalpy_assignment_count = usize::MAX,
        |state, index| state.predecessor_supply_enthalpy_assignment_route_counts[index] = usize::MAX,
        |state, _| state.inactive_transition_count = usize::MAX,
        |state, _| state.cp418_supply_humidity_ratio_state_owner_count = usize::MAX,
        |state, _| state.unchanged_supply_humidity_ratio_preservation_count = usize::MAX,
        |state, _| state.cp418_supply_enthalpy_state_owner_count = usize::MAX,
        |state, _| state.unchanged_supply_enthalpy_preservation_count = usize::MAX,
        |state, _| state.cp418_supply_temperature_state_owner_count = usize::MAX,
        |state, _| state.unchanged_supply_temperature_preservation_count = usize::MAX,
    ];
    assert_overflows_transactionally(predecessor, route.logical_index, setters);
}

#[test]
fn later_guard_false_counter_overflow_is_transactional() {
    let predecessor = predecessor_fixture(20, false, true);
    let route = predecessor_route(predecessor).expect("later guard-false route");
    assert!(!route.active && route.predecessor_guard_false_fallthrough);
    let setters: &[fn(&mut State, usize)] = &[
        |state, index| state.predecessor_guard_false_fallthrough_route_counts[index] = usize::MAX,
        |state, _| state.inactive_transition_count = usize::MAX,
        |state, _| state.cp418_supply_humidity_ratio_state_owner_count = usize::MAX,
        |state, _| state.unchanged_supply_humidity_ratio_preservation_count = usize::MAX,
        |state, _| state.cp418_supply_enthalpy_state_owner_count = usize::MAX,
        |state, _| state.unchanged_supply_enthalpy_preservation_count = usize::MAX,
        |state, _| state.cp418_supply_temperature_state_owner_count = usize::MAX,
        |state, _| state.unchanged_supply_temperature_preservation_count = usize::MAX,
    ];
    assert_overflows_transactionally(predecessor, route.logical_index, setters);
}
