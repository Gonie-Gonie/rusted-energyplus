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

pub(super) fn predecessor_lineage_route_counts(state: &State) -> [usize; 18] {
    [
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.heating_availability_guard_false_fallthrough_capacity_guard_false_count,
        state.heating_availability_guard_false_fallthrough_dehumidification_total_output_assignment_count,
        state.heating_availability_guard_false_fallthrough_dehumidification_guard_false_count,
        state.humidification_control_guard_false_fallthrough_capacity_guard_false_count,
        state.humidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count,
        state.humidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
        state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count,
        state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_assignment_count,
        state.dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count,
        state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count,
        state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_assignment_count,
        state.dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count,
        state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count,
        state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count,
        state.dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
    ]
}

pub(super) fn predecessor_refined_route_counts(state: &PredecessorState) -> [usize; 18] {
    [
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.heating_availability_guard_false_fallthrough_capacity_guard_false_count,
        state.heating_availability_guard_false_fallthrough_dehumidification_total_output_assignment_count,
        state.heating_availability_guard_false_fallthrough_dehumidification_guard_false_count,
        state.humidification_control_guard_false_fallthrough_capacity_guard_false_count,
        state.humidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count,
        state.humidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
        state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count,
        state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_assignment_count,
        state.dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count,
        state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count,
        state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_assignment_count,
        state.dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count,
        state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count,
        state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count,
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

pub(super) fn route_guard_false_counts(state: &State) -> [usize; 5] {
    [
        state.heating_availability_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count,
        state.humidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count,
        state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_capacity_guard_false_fallthrough_count,
        state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_capacity_guard_false_fallthrough_count,
        state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count,
    ]
}

pub(super) fn route_body_entry_counts(state: &State) -> [usize; 5] {
    [
        state.heating_availability_guard_false_fallthrough_dehumidification_total_output_capacity_adjustment_body_entry_count,
        state.humidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_adjustment_body_entry_count,
        state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_capacity_adjustment_body_entry_count,
        state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_capacity_adjustment_body_entry_count,
        state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_adjustment_body_entry_count,
    ]
}

pub(super) fn refined_route_counts(state: &State) -> [usize; 23] {
    let predecessor = predecessor_lineage_route_counts(state);
    let guard_false = route_guard_false_counts(state);
    let body = route_body_entry_counts(state);
    [
        predecessor[0],
        predecessor[1],
        predecessor[2],
        predecessor[3],
        predecessor[5],
        guard_false[0],
        body[0],
        predecessor[6],
        predecessor[8],
        guard_false[1],
        body[1],
        predecessor[9],
        predecessor[11],
        guard_false[2],
        body[2],
        predecessor[12],
        predecessor[14],
        guard_false[3],
        body[3],
        predecessor[15],
        predecessor[17],
        guard_false[4],
        body[4],
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
    } else if snapshot.predecessor_dehumidification_guard_false_fallthrough {
        1
    } else if snapshot.dehumidification_total_output_capacity_guard_false_fallthrough {
        2
    } else if snapshot.dehumidification_total_output_capacity_adjustment_body_entered {
        3
    } else {
        return None;
    };
    Some(3 + 4 * (base_index - 3) + successor)
}

pub(super) fn predecessor_route_index(cp383_index: usize) -> Option<usize> {
    if cp383_index < 3 {
        return Some(cp383_index);
    }
    let relative = cp383_index - 3;
    let base = relative / 4;
    let successor = match relative % 4 {
        0 => 0,
        1 => 2,
        2 | 3 => 1,
        _ => return None,
    };
    Some(3 + 3 * base + successor)
}

pub(super) fn corroborator_latest_route_has_count(
    state: &CapacityCorroboratorState,
    latest: CapacityCorroboratorSnapshot,
) -> bool {
    if latest.capacity_limit_sensible_output_guard_false_fallthrough {
        state.capacity_limit_sensible_output_guard_false_fallthrough_count > 0
    } else if latest.capacity_limit_sensible_output_adjustment_body_entered {
        state.capacity_limit_sensible_output_adjustment_body_entry_count > 0
    } else {
        false
    }
}
