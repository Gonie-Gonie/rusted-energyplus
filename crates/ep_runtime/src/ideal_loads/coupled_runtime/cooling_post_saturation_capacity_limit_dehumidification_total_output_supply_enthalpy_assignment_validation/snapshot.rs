use super::*;

pub(super) fn matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let operands = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment;
    let predecessor = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment;
    let snapshot = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment;

    snapshot.system == binding.ideal_loads_air_system
        && snapshot.parent_call_ordinal == call_ordinal
        && snapshot.controlled_zone == binding.zone
        && cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_snapshot_is_exact_direct_release(snapshot)
        && links_to_prefix(snapshot, predecessor, operands)
}

pub(super) fn links_to_prefix(
    snapshot: Snapshot,
    predecessor: PredecessorSnapshot,
    operands: OperandSnapshot,
) -> bool {
    if !cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_snapshot_is_exact_direct_release(predecessor)
        || !cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_snapshot_is_exact_direct_release(operands)
        || !same_call(snapshot, predecessor, operands)
        || !inherited_flags_match(snapshot, predecessor)
        || snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated
            != predecessor.predecessor_dehumidification_total_output_capacity_guard_evaluated
        || snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered
            != predecessor.predecessor_dehumidification_total_output_capacity_adjustment_body_entered
        || snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough
            != predecessor.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough
        || snapshot.dehumidification_total_output_capacity_guard_false_fallthrough
            != predecessor.dehumidification_total_output_capacity_guard_false_fallthrough
        || snapshot.dehumidification_total_output_maximum_capacity_assignment_executed
            != predecessor.dehumidification_total_output_maximum_capacity_assignment_executed
        || snapshot.supply_enthalpy_assignment_executed
            != predecessor.dehumidification_total_output_maximum_capacity_assignment_executed
    {
        return false;
    }

    let guard_false = predecessor.dehumidification_total_output_capacity_guard_false_fallthrough;
    let assignment = predecessor.dehumidification_total_output_maximum_capacity_assignment_executed;
    let retained = guard_false || assignment;
    if guard_false && assignment {
        return false;
    }
    if !retained {
        return outer_skip_shape(snapshot);
    }
    if !operands.dehumidification_total_output_assignment_executed
        || !snapshot.cp379_retained_supply_enthalpy_owned_read
        || !exact_optional_f64(
            snapshot.preexisting_supply_enthalpy_j_per_kg,
            operands.supply_enthalpy_j_per_kg,
        )
    {
        return false;
    }
    if guard_false {
        return guard_false_shape(snapshot);
    }
    assignment_shape(snapshot, predecessor, operands)
}

fn same_call(
    snapshot: Snapshot,
    predecessor: PredecessorSnapshot,
    operands: OperandSnapshot,
) -> bool {
    [predecessor.system, operands.system]
        .into_iter()
        .all(|system| system == snapshot.system)
        && [
            predecessor.parent_call_ordinal,
            operands.parent_call_ordinal,
        ]
        .into_iter()
        .all(|ordinal| ordinal == snapshot.parent_call_ordinal)
        && [predecessor.controlled_zone, operands.controlled_zone]
            .into_iter()
            .all(|zone| zone == snapshot.controlled_zone)
}

fn inherited_flags_match(snapshot: Snapshot, predecessor: PredecessorSnapshot) -> bool {
    [
        snapshot.unit_off_skipped,
        snapshot.non_cooling_skipped,
        snapshot.positive_guard_false_fallthrough_skipped,
        snapshot.heating_availability_guard_false_fallthrough,
        snapshot.humidification_control_guard_false_fallthrough,
        snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        snapshot.dehumidification_control_none_maximum_assignment_executed,
        snapshot.dehumidification_control_guard_false_fallthrough,
        snapshot.predecessor_capacity_limit_guard_evaluated,
        snapshot.predecessor_capacity_limit_body_entered,
        snapshot.predecessor_active_capacity_limit_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_guard_evaluated,
        snapshot.predecessor_dehumidification_body_entered,
        snapshot.predecessor_dehumidification_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_total_output_assignment_executed,
    ] == [
        predecessor.unit_off_skipped,
        predecessor.non_cooling_skipped,
        predecessor.positive_guard_false_fallthrough_skipped,
        predecessor.heating_availability_guard_false_fallthrough,
        predecessor.humidification_control_guard_false_fallthrough,
        predecessor.dehumidification_control_humidistat_maximum_assignment_executed,
        predecessor.dehumidification_control_none_maximum_assignment_executed,
        predecessor.dehumidification_control_guard_false_fallthrough,
        predecessor.predecessor_capacity_limit_guard_evaluated,
        predecessor.predecessor_capacity_limit_body_entered,
        predecessor.predecessor_active_capacity_limit_guard_false_fallthrough,
        predecessor.predecessor_dehumidification_guard_evaluated,
        predecessor.predecessor_dehumidification_body_entered,
        predecessor.predecessor_dehumidification_guard_false_fallthrough,
        predecessor.predecessor_dehumidification_total_output_assignment_executed,
    ]
}

fn outer_skip_shape(snapshot: Snapshot) -> bool {
    !snapshot.cp379_retained_supply_enthalpy_owned_read
        && active_flags(snapshot).into_iter().all(|flag| !flag)
        && numeric_values(snapshot)
            .into_iter()
            .all(|value| value.is_none())
}

fn guard_false_shape(snapshot: Snapshot) -> bool {
    active_flags(snapshot).into_iter().all(|flag| !flag)
        && [
            snapshot.mixed_air_enthalpy_j_per_kg,
            snapshot.cooling_total_output_w,
            snapshot.supply_mass_flow_rate_kg_per_s,
            snapshot.specific_cooling_output_j_per_kg,
            snapshot.calculated_supply_enthalpy_j_per_kg,
            snapshot.assigned_supply_enthalpy_j_per_kg,
        ]
        .into_iter()
        .all(|value| value.is_none())
        && exact_optional_f64(
            snapshot.preexisting_supply_enthalpy_j_per_kg,
            snapshot.resulting_supply_enthalpy_j_per_kg,
        )
}

fn assignment_shape(
    snapshot: Snapshot,
    predecessor: PredecessorSnapshot,
    operands: OperandSnapshot,
) -> bool {
    if active_flags(snapshot).into_iter().any(|flag| !flag) {
        return false;
    }
    let (
        Some(mixed),
        Some(total),
        Some(flow),
        Some(specific),
        Some(calculated),
        Some(assigned),
        Some(resulting),
    ) = (
        snapshot.mixed_air_enthalpy_j_per_kg,
        snapshot.cooling_total_output_w,
        snapshot.supply_mass_flow_rate_kg_per_s,
        snapshot.specific_cooling_output_j_per_kg,
        snapshot.calculated_supply_enthalpy_j_per_kg,
        snapshot.assigned_supply_enthalpy_j_per_kg,
        snapshot.resulting_supply_enthalpy_j_per_kg,
    )
    else {
        return false;
    };
    exact_optional_f64(Some(mixed), operands.mixed_air_enthalpy_j_per_kg)
        && exact_optional_f64(Some(total), predecessor.resulting_cooling_total_output_w)
        && exact_optional_f64(Some(flow), operands.supply_mass_flow_rate_kg_per_s)
        && specific.to_bits() == (total / flow).to_bits()
        && calculated.to_bits() == (mixed - specific).to_bits()
        && assigned.to_bits() == calculated.to_bits()
        && resulting.to_bits() == assigned.to_bits()
}

fn active_flags(snapshot: Snapshot) -> [bool; 10] {
    [
        snapshot.cp329_retained_mixed_air_enthalpy_owned_read,
        snapshot.mixed_air_enthalpy_read,
        snapshot.cp384_retained_cooling_total_output_owned_read,
        snapshot.cooling_total_output_read,
        snapshot.cp330_retained_supply_mass_flow_rate_owned_read,
        snapshot.supply_mass_flow_rate_read,
        snapshot.specific_cooling_output_calculated,
        snapshot.supply_enthalpy_difference_calculated,
        snapshot.supply_enthalpy_assigned,
        snapshot.supply_enthalpy_assignment_executed,
    ]
}

fn numeric_values(snapshot: Snapshot) -> [Option<f64>; 8] {
    [
        snapshot.preexisting_supply_enthalpy_j_per_kg,
        snapshot.mixed_air_enthalpy_j_per_kg,
        snapshot.cooling_total_output_w,
        snapshot.supply_mass_flow_rate_kg_per_s,
        snapshot.specific_cooling_output_j_per_kg,
        snapshot.calculated_supply_enthalpy_j_per_kg,
        snapshot.assigned_supply_enthalpy_j_per_kg,
        snapshot.resulting_supply_enthalpy_j_per_kg,
    ]
}

pub(super) fn snapshots_match_exact_bits(left: Snapshot, right: Snapshot) -> bool {
    if !numeric_values(left)
        .into_iter()
        .zip(numeric_values(right))
        .all(|(left, right)| exact_optional_f64(left, right))
    {
        return false;
    }
    let mut left = left;
    let mut right = right;
    for snapshot in [&mut left, &mut right] {
        snapshot.preexisting_supply_enthalpy_j_per_kg = None;
        snapshot.mixed_air_enthalpy_j_per_kg = None;
        snapshot.cooling_total_output_w = None;
        snapshot.supply_mass_flow_rate_kg_per_s = None;
        snapshot.specific_cooling_output_j_per_kg = None;
        snapshot.calculated_supply_enthalpy_j_per_kg = None;
        snapshot.assigned_supply_enthalpy_j_per_kg = None;
        snapshot.resulting_supply_enthalpy_j_per_kg = None;
    }
    left == right
}
