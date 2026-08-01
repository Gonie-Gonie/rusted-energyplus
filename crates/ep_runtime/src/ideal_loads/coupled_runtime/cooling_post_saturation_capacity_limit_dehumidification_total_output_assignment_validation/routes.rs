use super::*;

pub(super) fn base_route_counts(state: &State) -> [usize; 8] {
    [
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.heating_availability_guard_false_fallthrough_count,
        state.humidification_control_guard_false_fallthrough_count,
        state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_guard_false_fallthrough_count,
    ]
}

pub(super) fn predecessor_base_route_counts(state: &PredecessorState) -> [usize; 8] {
    [
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.heating_availability_guard_false_fallthrough_count,
        state.humidification_control_guard_false_fallthrough_count,
        state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_guard_false_fallthrough_count,
    ]
}

pub(super) fn route_capacity_false_counts(state: &State) -> [usize; 5] {
    [
        state.heating_availability_guard_false_fallthrough_capacity_guard_false_count,
        state.humidification_control_guard_false_fallthrough_capacity_guard_false_count,
        state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count,
        state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count,
        state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count,
    ]
}

pub(super) fn route_body_entry_counts(state: &State) -> [usize; 5] {
    [
        state.heating_availability_guard_false_fallthrough_dehumidification_body_entry_count,
        state.humidification_control_guard_false_fallthrough_dehumidification_body_entry_count,
        state.dehumidification_control_humidistat_maximum_assignment_dehumidification_body_entry_count,
        state.dehumidification_control_none_maximum_assignment_dehumidification_body_entry_count,
        state.dehumidification_control_guard_false_fallthrough_dehumidification_body_entry_count,
    ]
}

pub(super) fn route_guard_false_counts(state: &State) -> [usize; 5] {
    [
        state.heating_availability_guard_false_fallthrough_dehumidification_guard_false_count,
        state.humidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
        state.dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count,
        state.dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count,
        state.dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
    ]
}

pub(super) fn route_assignment_counts(state: &State) -> [usize; 5] {
    [
        state.heating_availability_guard_false_fallthrough_dehumidification_total_output_assignment_count,
        state.humidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count,
        state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_assignment_count,
        state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_assignment_count,
        state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count,
    ]
}

pub(super) fn refined_route_counts(state: &State) -> [usize; 18] {
    let capacity_false = route_capacity_false_counts(state);
    let assigned = route_assignment_counts(state);
    let guard_false = route_guard_false_counts(state);
    [
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        capacity_false[0],
        assigned[0],
        guard_false[0],
        capacity_false[1],
        assigned[1],
        guard_false[1],
        capacity_false[2],
        assigned[2],
        guard_false[2],
        capacity_false[3],
        assigned[3],
        guard_false[3],
        capacity_false[4],
        assigned[4],
        guard_false[4],
    ]
}

pub(super) fn predecessor_refined_route_counts(state: &PredecessorState) -> [usize; 18] {
    [
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.heating_availability_guard_false_fallthrough_capacity_guard_false_count,
        state.heating_availability_guard_false_fallthrough_dehumidification_body_entry_count,
        state.heating_availability_guard_false_fallthrough_dehumidification_guard_false_count,
        state.humidification_control_guard_false_fallthrough_capacity_guard_false_count,
        state.humidification_control_guard_false_fallthrough_dehumidification_body_entry_count,
        state.humidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
        state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count,
        state.dehumidification_control_humidistat_maximum_assignment_dehumidification_body_entry_count,
        state.dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count,
        state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count,
        state.dehumidification_control_none_maximum_assignment_dehumidification_body_entry_count,
        state.dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count,
        state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count,
        state.dehumidification_control_guard_false_fallthrough_dehumidification_body_entry_count,
        state.dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
    ]
}

pub(super) fn refined_route_index(snapshot: Snapshot) -> Option<usize> {
    let base = [
        snapshot.unit_off_skipped,
        snapshot.non_cooling_skipped,
        snapshot.positive_guard_false_fallthrough_skipped,
        snapshot.heating_availability_guard_false_fallthrough,
        snapshot.humidification_control_guard_false_fallthrough,
        snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        snapshot.dehumidification_control_none_maximum_assignment_executed,
        snapshot.dehumidification_control_guard_false_fallthrough,
    ];
    if base.into_iter().filter(|flag| *flag).count() != 1 {
        return None;
    }
    let base_index = base.into_iter().position(|flag| flag)?;
    if base_index < 3 {
        return Some(base_index);
    }
    let successor = if snapshot.predecessor_active_capacity_limit_guard_false_fallthrough {
        0
    } else if snapshot.dehumidification_total_output_assignment_executed {
        1
    } else if snapshot.predecessor_dehumidification_guard_false_fallthrough {
        2
    } else {
        return None;
    };
    Some(3 + 3 * (base_index - 3) + successor)
}

pub(super) fn supply_mass_flow_route_counts(state: &SupplyMassFlowState) -> [usize; 4] {
    [
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.active_guard_false_fallthrough_count,
        state.positive_supply_mass_flow_body_entry_count,
    ]
}

pub(super) fn mixed_air_route_counts(state: &MixedAirState) -> [usize; 3] {
    [
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.no_outdoor_air_fallback_count,
    ]
}

pub(super) fn early_total_route_counts(state: &EarlyTotalState) -> [usize; 5] {
    [
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.capacity_limit_guard_false_fallthrough_skip_count,
        state.capacity_limit_sensible_output_assignment_count,
    ]
}

pub(super) fn supply_enthalpy_route_counts(state: &SupplyEnthalpyState) -> [usize; 8] {
    [
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.heating_availability_guard_false_fallthrough_count,
        state.humidification_control_guard_false_fallthrough_count,
        state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_guard_false_fallthrough_count,
    ]
}
