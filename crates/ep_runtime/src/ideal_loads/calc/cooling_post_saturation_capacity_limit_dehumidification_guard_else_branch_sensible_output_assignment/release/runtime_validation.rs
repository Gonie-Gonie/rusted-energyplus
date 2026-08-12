//! Exact CP420 route, owner, and counter validation.

use super::super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentRuntimeState as State;
use super::super::transition::RetainedRoute;
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentRuntimeState as PredecessorState;

pub(super) fn state_counts_are_consistent(state: &State) -> bool {
    let Some(transitions) = checked_sum(&state.predecessor_route_counts) else {
        return false;
    };
    let Some(saturation) =
        checked_sum(&state.predecessor_supply_temperature_saturation_assignment_route_counts)
    else {
        return false;
    };
    let Some(limits) =
        checked_sum(&state.predecessor_supply_temperature_mixed_air_limit_route_counts)
    else {
        return false;
    };
    let Some(humidity) =
        checked_sum(&state.predecessor_supply_humidity_ratio_assignment_route_counts)
    else {
        return false;
    };
    let Some(enthalpy) = checked_sum(&state.predecessor_supply_enthalpy_assignment_route_counts)
    else {
        return false;
    };
    let Some(entries) =
        checked_sum(&state.predecessor_dehumidification_guard_else_branch_entry_route_counts)
    else {
        return false;
    };
    let Some(cp_air_assignments) = checked_sum(
        &state.predecessor_dehumidification_guard_else_branch_cp_air_assignment_route_counts,
    ) else {
        return false;
    };
    let Some(assignments) = checked_sum(
        &state.dehumidification_guard_else_branch_sensible_output_assignment_route_counts,
    ) else {
        return false;
    };
    let Some(inactive) = transitions.checked_sub(assignments) else {
        return false;
    };
    let Some(w_owners) = checked_sum(&state.predecessor_route_counts[18..]) else {
        return false;
    };
    let Some(h_owners) = sum_predecessor_indices(&state.predecessor_route_counts, |index| {
        matches!(index, 5 | 8 | 11 | 14 | 17..=29)
    }) else {
        return false;
    };
    let Some(t_owners) =
        sum_predecessor_indices(&state.predecessor_route_counts, |index| index >= 3)
    else {
        return false;
    };

    for index in 0..36 {
        let Some(guard_outcomes) = state.predecessor_guard_false_fallthrough_route_counts[index]
            .checked_add(state.predecessor_guard_body_entry_route_counts[index])
        else {
            return false;
        };
        let expected_guard = if predecessor_index_for_logical(index) >= 18 {
            state.predecessor_route_counts[index]
        } else {
            0
        };
        let expected_active = if matches!(index, 4 | 7 | 10 | 13 | 16) {
            state.predecessor_route_counts[index]
        } else {
            0
        };
        if guard_outcomes != expected_guard
            || state.predecessor_supply_temperature_saturation_assignment_route_counts[index]
                != state.predecessor_guard_body_entry_route_counts[index]
            || state.predecessor_supply_temperature_mixed_air_limit_route_counts[index]
                != state.predecessor_supply_temperature_saturation_assignment_route_counts[index]
            || state.predecessor_supply_humidity_ratio_assignment_route_counts[index]
                != state.predecessor_supply_temperature_mixed_air_limit_route_counts[index]
            || state.predecessor_supply_enthalpy_assignment_route_counts[index]
                != state.predecessor_supply_humidity_ratio_assignment_route_counts[index]
            || state.predecessor_dehumidification_guard_else_branch_entry_route_counts[index]
                != expected_active
            || state.predecessor_dehumidification_guard_else_branch_cp_air_assignment_route_counts
                [index]
                != expected_active
            || state.dehumidification_guard_else_branch_sensible_output_assignment_route_counts
                [index]
                != expected_active
        {
            return false;
        }
    }
    let active_counters = [
        state.supply_mass_flow_rate_owned_read_count,
        state.supply_mass_flow_rate_bit_corroboration_count,
        state.supply_mass_flow_rate_read_count,
        state.cp_air_owned_read_count,
        state.cp_air_read_count,
        state.supply_mass_flow_rate_times_cp_air_calculation_count,
        state.mixed_air_temperature_owned_read_count,
        state.mixed_air_temperature_read_count,
        state.supply_temperature_owned_read_count,
        state.supply_temperature_read_count,
        state.mixed_air_minus_supply_temperature_calculation_count,
        state.cooling_sensible_output_calculation_count,
        state.cooling_sensible_output_assignment_write_count,
    ];
    state.transition_count == transitions
        && state.inactive_transition_count == inactive
        && state.predecessor_supply_temperature_saturation_assignment_count == saturation
        && state.predecessor_supply_temperature_saturation_mixed_air_limit_count == limits
        && state.predecessor_supply_humidity_ratio_assignment_count == humidity
        && state.predecessor_supply_enthalpy_assignment_count == enthalpy
        && saturation == limits
        && limits == humidity
        && humidity == enthalpy
        && state.predecessor_dehumidification_guard_else_branch_entry_count == entries
        && state.predecessor_dehumidification_guard_else_branch_cp_air_assignment_count
            == cp_air_assignments
        && state.dehumidification_guard_else_branch_sensible_output_assignment_count == assignments
        && entries == cp_air_assignments
        && cp_air_assignments == assignments
        && assignments
            .checked_mul(8)
            .is_some_and(|sites| state.source_site_execution_count == sites)
        && state.cp419_supply_humidity_ratio_state_owner_count == w_owners
        && state.unchanged_supply_humidity_ratio_preservation_count == w_owners
        && state.cp419_supply_enthalpy_state_owner_count == h_owners
        && state.unchanged_supply_enthalpy_preservation_count == h_owners
        && state.cp419_supply_temperature_state_owner_count == t_owners
        && state.unchanged_supply_temperature_preservation_count == t_owners
        && active_counters
            .into_iter()
            .all(|count| count == assignments)
}

pub(super) fn pending_predecessor_counts_match(
    state: &State,
    predecessor: &PredecessorState,
    route: RetainedRoute,
) -> bool {
    let index = route.logical_index;
    if index >= 36 {
        return false;
    }
    let mut routes = state.predecessor_route_counts;
    let mut guard_false = state.predecessor_guard_false_fallthrough_route_counts;
    let mut guard_body = state.predecessor_guard_body_entry_route_counts;
    let mut saturation = state.predecessor_supply_temperature_saturation_assignment_route_counts;
    let mut limits = state.predecessor_supply_temperature_mixed_air_limit_route_counts;
    let mut humidity = state.predecessor_supply_humidity_ratio_assignment_route_counts;
    let mut enthalpy = state.predecessor_supply_enthalpy_assignment_route_counts;
    let mut entries = state.predecessor_dehumidification_guard_else_branch_entry_route_counts;
    let mut cp_air =
        state.predecessor_dehumidification_guard_else_branch_cp_air_assignment_route_counts;
    if !checked_increment(&mut routes[index])
        || (route.predecessor_guard_false_fallthrough
            && !checked_increment(&mut guard_false[index]))
        || (route.predecessor_guard_body_entered && !checked_increment(&mut guard_body[index]))
        || (route.predecessor_saturation_temperature_assignment_executed
            && !checked_increment(&mut saturation[index]))
        || (route.predecessor_saturation_temperature_mixed_air_limit_executed
            && !checked_increment(&mut limits[index]))
        || (route.predecessor_supply_humidity_ratio_assignment_executed
            && !checked_increment(&mut humidity[index]))
        || (route.predecessor_supply_enthalpy_assignment_executed
            && !checked_increment(&mut enthalpy[index]))
        || (route.active
            && (!checked_increment(&mut entries[index]) || !checked_increment(&mut cp_air[index])))
    {
        return false;
    }
    routes == predecessor.predecessor_route_counts
        && guard_false == predecessor.predecessor_guard_false_fallthrough_route_counts
        && guard_body == predecessor.predecessor_guard_body_entry_route_counts
        && saturation
            == predecessor.predecessor_supply_temperature_saturation_assignment_route_counts
        && limits == predecessor.predecessor_supply_temperature_mixed_air_limit_route_counts
        && humidity == predecessor.predecessor_supply_humidity_ratio_assignment_route_counts
        && enthalpy == predecessor.predecessor_supply_enthalpy_assignment_route_counts
        && entries == predecessor.predecessor_dehumidification_guard_else_branch_entry_route_counts
        && cp_air == predecessor.dehumidification_guard_else_branch_cp_air_assignment_route_counts
}

pub(super) fn completed_predecessor_counts_match(
    state: &State,
    predecessor: &PredecessorState,
) -> bool {
    state.transition_count == predecessor.transition_count
        && state.predecessor_supply_temperature_saturation_assignment_count
            == predecessor.predecessor_supply_temperature_saturation_assignment_count
        && state.predecessor_supply_temperature_saturation_mixed_air_limit_count
            == predecessor.predecessor_supply_temperature_saturation_mixed_air_limit_count
        && state.predecessor_supply_humidity_ratio_assignment_count
            == predecessor.predecessor_supply_humidity_ratio_assignment_count
        && state.predecessor_supply_enthalpy_assignment_count
            == predecessor.predecessor_supply_enthalpy_assignment_count
        && state.predecessor_dehumidification_guard_else_branch_entry_count
            == predecessor.predecessor_dehumidification_guard_else_branch_entry_count
        && state.predecessor_dehumidification_guard_else_branch_cp_air_assignment_count
            == predecessor.dehumidification_guard_else_branch_cp_air_assignment_count
        && state.cp419_supply_humidity_ratio_state_owner_count
            == predecessor.cp418_supply_humidity_ratio_state_owner_count
        && state.cp419_supply_enthalpy_state_owner_count
            == predecessor.cp418_supply_enthalpy_state_owner_count
        && state.cp419_supply_temperature_state_owner_count
            == predecessor.cp418_supply_temperature_state_owner_count
        && state.predecessor_route_counts == predecessor.predecessor_route_counts
        && state.predecessor_guard_false_fallthrough_route_counts
            == predecessor.predecessor_guard_false_fallthrough_route_counts
        && state.predecessor_guard_body_entry_route_counts
            == predecessor.predecessor_guard_body_entry_route_counts
        && state.predecessor_supply_temperature_saturation_assignment_route_counts
            == predecessor.predecessor_supply_temperature_saturation_assignment_route_counts
        && state.predecessor_supply_temperature_mixed_air_limit_route_counts
            == predecessor.predecessor_supply_temperature_mixed_air_limit_route_counts
        && state.predecessor_supply_humidity_ratio_assignment_route_counts
            == predecessor.predecessor_supply_humidity_ratio_assignment_route_counts
        && state.predecessor_supply_enthalpy_assignment_route_counts
            == predecessor.predecessor_supply_enthalpy_assignment_route_counts
        && state.predecessor_dehumidification_guard_else_branch_entry_route_counts
            == predecessor.predecessor_dehumidification_guard_else_branch_entry_route_counts
        && state.predecessor_dehumidification_guard_else_branch_cp_air_assignment_route_counts
            == predecessor.dehumidification_guard_else_branch_cp_air_assignment_route_counts
}

fn checked_increment(value: &mut usize) -> bool {
    let Some(next) = value.checked_add(1) else {
        return false;
    };
    *value = next;
    true
}

fn checked_sum(values: &[usize]) -> Option<usize> {
    values
        .iter()
        .try_fold(0usize, |sum, value| sum.checked_add(*value))
}

fn sum_predecessor_indices(values: &[usize; 36], include: impl Fn(usize) -> bool) -> Option<usize> {
    let mut logical_index = 0;
    let mut total = 0usize;
    for predecessor_index in 0..30 {
        let width = 1 + usize::from(matches!(predecessor_index, 20 | 21 | 24 | 25 | 27 | 29));
        if include(predecessor_index) {
            total = values[logical_index..logical_index + width]
                .iter()
                .try_fold(total, |sum, value| sum.checked_add(*value))?;
        }
        logical_index += width;
    }
    (logical_index == 36).then_some(total)
}

fn predecessor_index_for_logical(logical_index: usize) -> usize {
    let mut offset = 0;
    for predecessor_index in 0..30 {
        let width = 1 + usize::from(matches!(predecessor_index, 20 | 21 | 24 | 25 | 27 | 29));
        if logical_index < offset + width {
            return predecessor_index;
        }
        offset += width;
    }
    usize::MAX
}
