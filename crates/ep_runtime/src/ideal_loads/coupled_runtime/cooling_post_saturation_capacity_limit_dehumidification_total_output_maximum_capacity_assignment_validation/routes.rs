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

pub(super) fn inherited_lineage_route_counts(state: &State) -> [usize; 18] {
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

pub(super) fn predecessor_lineage_route_counts(state: &PredecessorState) -> [usize; 18] {
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

pub(super) fn guard_false_route_counts(state: &State) -> [usize; 5] {
    [
        state.heating_availability_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count,
        state.humidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count,
        state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_capacity_guard_false_fallthrough_count,
        state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_capacity_guard_false_fallthrough_count,
        state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count,
    ]
}

pub(super) fn predecessor_guard_false_route_counts(state: &PredecessorState) -> [usize; 5] {
    [
        state.heating_availability_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count,
        state.humidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count,
        state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_capacity_guard_false_fallthrough_count,
        state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_capacity_guard_false_fallthrough_count,
        state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count,
    ]
}

pub(super) fn assignment_route_counts(state: &State) -> [usize; 5] {
    [
        state.heating_availability_guard_false_fallthrough_dehumidification_total_output_maximum_capacity_assignment_count,
        state.humidification_control_guard_false_fallthrough_dehumidification_total_output_maximum_capacity_assignment_count,
        state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_maximum_capacity_assignment_count,
        state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_maximum_capacity_assignment_count,
        state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_maximum_capacity_assignment_count,
    ]
}

pub(super) fn predecessor_body_route_counts(state: &PredecessorState) -> [usize; 5] {
    [
        state.heating_availability_guard_false_fallthrough_dehumidification_total_output_capacity_adjustment_body_entry_count,
        state.humidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_adjustment_body_entry_count,
        state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_capacity_adjustment_body_entry_count,
        state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_capacity_adjustment_body_entry_count,
        state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_adjustment_body_entry_count,
    ]
}

pub(super) fn refined_route_counts(state: &State) -> [usize; 23] {
    let inherited = inherited_lineage_route_counts(state);
    let guard_false = guard_false_route_counts(state);
    let assignment = assignment_route_counts(state);
    [
        inherited[0],
        inherited[1],
        inherited[2],
        inherited[3],
        inherited[5],
        guard_false[0],
        assignment[0],
        inherited[6],
        inherited[8],
        guard_false[1],
        assignment[1],
        inherited[9],
        inherited[11],
        guard_false[2],
        assignment[2],
        inherited[12],
        inherited[14],
        guard_false[3],
        assignment[3],
        inherited[15],
        inherited[17],
        guard_false[4],
        assignment[4],
    ]
}

pub(super) fn predecessor_refined_route_counts(state: &PredecessorState) -> [usize; 23] {
    let inherited = predecessor_lineage_route_counts(state);
    let guard_false = predecessor_guard_false_route_counts(state);
    let body = predecessor_body_route_counts(state);
    [
        inherited[0],
        inherited[1],
        inherited[2],
        inherited[3],
        inherited[5],
        guard_false[0],
        body[0],
        inherited[6],
        inherited[8],
        guard_false[1],
        body[1],
        inherited[9],
        inherited[11],
        guard_false[2],
        body[2],
        inherited[12],
        inherited[14],
        guard_false[3],
        body[3],
        inherited[15],
        inherited[17],
        guard_false[4],
        body[4],
    ]
}

pub(super) fn refined_route_index(snapshot: Snapshot) -> Option<usize> {
    route_index(
        [
            snapshot.unit_off_skipped,
            snapshot.non_cooling_skipped,
            snapshot.positive_guard_false_fallthrough_skipped,
            snapshot.heating_availability_guard_false_fallthrough,
            snapshot.humidification_control_guard_false_fallthrough,
            snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
            snapshot.dehumidification_control_none_maximum_assignment_executed,
            snapshot.dehumidification_control_guard_false_fallthrough,
        ],
        snapshot.predecessor_active_capacity_limit_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_guard_false_fallthrough,
        snapshot.dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_maximum_capacity_assignment_executed,
    )
}

pub(super) fn predecessor_refined_route_index(snapshot: PredecessorSnapshot) -> Option<usize> {
    route_index(
        [
            snapshot.unit_off_skipped,
            snapshot.non_cooling_skipped,
            snapshot.positive_guard_false_fallthrough_skipped,
            snapshot.heating_availability_guard_false_fallthrough,
            snapshot.humidification_control_guard_false_fallthrough,
            snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
            snapshot.dehumidification_control_none_maximum_assignment_executed,
            snapshot.dehumidification_control_guard_false_fallthrough,
        ],
        snapshot.predecessor_active_capacity_limit_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_guard_false_fallthrough,
        snapshot.dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_capacity_adjustment_body_entered,
    )
}

fn route_index(
    base: [bool; 8],
    capacity_guard_false: bool,
    dehumidification_guard_false: bool,
    total_output_capacity_guard_false: bool,
    final_route: bool,
) -> Option<usize> {
    if base.into_iter().filter(|flag| *flag).count() != 1 {
        return None;
    }
    let base_index = base.into_iter().position(|flag| flag)?;
    if base_index < 3 {
        return Some(base_index);
    }
    let successors = [
        capacity_guard_false,
        dehumidification_guard_false,
        total_output_capacity_guard_false,
        final_route,
    ];
    if successors.into_iter().filter(|flag| *flag).count() != 1 {
        return None;
    }
    Some(3 + 4 * (base_index - 3) + successors.into_iter().position(|flag| flag)?)
}
