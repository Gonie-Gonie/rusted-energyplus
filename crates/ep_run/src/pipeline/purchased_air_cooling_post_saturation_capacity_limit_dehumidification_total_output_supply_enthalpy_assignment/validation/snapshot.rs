//! CP385 snapshot lineage and null/preserve/arithmetic shape validation.

use super::*;

pub(super) fn metadata_is_exact(
    snapshot: Snapshot,
    predecessor: PredecessorSnapshot,
    operands: OperandSnapshot,
    expected_system: IdealLoadsAirSystemId,
    expected_zone: ZoneId,
    calls: usize,
) -> bool {
    snapshot.source
        == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER
        && snapshot.system == expected_system
        && predecessor.system == expected_system
        && operands.system == expected_system
        && snapshot.controlled_zone == expected_zone
        && predecessor.controlled_zone == expected_zone
        && operands.controlled_zone == expected_zone
        && snapshot.parent_call_ordinal == calls
        && predecessor.parent_call_ordinal == calls
        && operands.parent_call_ordinal == calls
}

pub(super) fn links_exactly(
    snapshot: Snapshot,
    predecessor: PredecessorSnapshot,
    operands: OperandSnapshot,
) -> bool {
    inherited_lineage_is_exact(snapshot, predecessor)
        && assignment_shape_is_exact(snapshot, predecessor, operands)
}

fn inherited_lineage_is_exact(snapshot: Snapshot, predecessor: PredecessorSnapshot) -> bool {
    snapshot.unit_off_skipped == predecessor.unit_off_skipped
        && snapshot.non_cooling_skipped == predecessor.non_cooling_skipped
        && snapshot.positive_guard_false_fallthrough_skipped
            == predecessor.positive_guard_false_fallthrough_skipped
        && snapshot.heating_availability_guard_false_fallthrough
            == predecessor.heating_availability_guard_false_fallthrough
        && snapshot.humidification_control_guard_false_fallthrough
            == predecessor.humidification_control_guard_false_fallthrough
        && snapshot.dehumidification_control_humidistat_maximum_assignment_executed
            == predecessor.dehumidification_control_humidistat_maximum_assignment_executed
        && snapshot.dehumidification_control_none_maximum_assignment_executed
            == predecessor.dehumidification_control_none_maximum_assignment_executed
        && snapshot.dehumidification_control_guard_false_fallthrough
            == predecessor.dehumidification_control_guard_false_fallthrough
        && snapshot.predecessor_capacity_limit_guard_evaluated
            == predecessor.predecessor_capacity_limit_guard_evaluated
        && snapshot.predecessor_capacity_limit_body_entered
            == predecessor.predecessor_capacity_limit_body_entered
        && snapshot.predecessor_active_capacity_limit_guard_false_fallthrough
            == predecessor.predecessor_active_capacity_limit_guard_false_fallthrough
        && snapshot.predecessor_dehumidification_guard_evaluated
            == predecessor.predecessor_dehumidification_guard_evaluated
        && snapshot.predecessor_dehumidification_body_entered
            == predecessor.predecessor_dehumidification_body_entered
        && snapshot.predecessor_dehumidification_guard_false_fallthrough
            == predecessor.predecessor_dehumidification_guard_false_fallthrough
        && snapshot.predecessor_dehumidification_total_output_assignment_executed
            == predecessor.predecessor_dehumidification_total_output_assignment_executed
        && snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated
            == predecessor.predecessor_dehumidification_total_output_capacity_guard_evaluated
        && snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered
            == predecessor
                .predecessor_dehumidification_total_output_capacity_adjustment_body_entered
        && snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough
            == predecessor
                .predecessor_dehumidification_total_output_capacity_guard_false_fallthrough
        && snapshot.dehumidification_total_output_capacity_guard_false_fallthrough
            == predecessor.dehumidification_total_output_capacity_guard_false_fallthrough
        && snapshot.dehumidification_total_output_maximum_capacity_assignment_executed
            == predecessor.dehumidification_total_output_maximum_capacity_assignment_executed
}

fn assignment_shape_is_exact(
    snapshot: Snapshot,
    predecessor: PredecessorSnapshot,
    operands: OperandSnapshot,
) -> bool {
    let evaluated = predecessor.predecessor_dehumidification_total_output_capacity_guard_evaluated;
    let guard_false = predecessor.dehumidification_total_output_capacity_guard_false_fallthrough;
    let assignment = predecessor.dehumidification_total_output_maximum_capacity_assignment_executed;
    if snapshot.supply_enthalpy_assignment_executed != assignment
        || evaluated != (guard_false || assignment)
        || (evaluated && guard_false == assignment)
    {
        return false;
    }
    if !evaluated {
        return numeric_values(snapshot)
            .into_iter()
            .all(|value| value.is_none())
            && provenance_flags(snapshot).into_iter().all(|flag| !flag);
    }

    let Some(preexisting) = operands.supply_enthalpy_j_per_kg else {
        return false;
    };
    if !snapshot.cp379_retained_supply_enthalpy_owned_read
        || !option_bits_equal(
            snapshot.preexisting_supply_enthalpy_j_per_kg,
            Some(preexisting),
        )
    {
        return false;
    }
    if guard_false {
        return !snapshot.cp329_retained_mixed_air_enthalpy_owned_read
            && !snapshot.mixed_air_enthalpy_read
            && snapshot.mixed_air_enthalpy_j_per_kg.is_none()
            && !snapshot.cp384_retained_cooling_total_output_owned_read
            && !snapshot.cooling_total_output_read
            && snapshot.cooling_total_output_w.is_none()
            && !snapshot.cp330_retained_supply_mass_flow_rate_owned_read
            && !snapshot.supply_mass_flow_rate_read
            && snapshot.supply_mass_flow_rate_kg_per_s.is_none()
            && !snapshot.specific_cooling_output_calculated
            && snapshot.specific_cooling_output_j_per_kg.is_none()
            && !snapshot.supply_enthalpy_difference_calculated
            && snapshot.calculated_supply_enthalpy_j_per_kg.is_none()
            && !snapshot.supply_enthalpy_assigned
            && snapshot.assigned_supply_enthalpy_j_per_kg.is_none()
            && option_bits_equal(
                snapshot.resulting_supply_enthalpy_j_per_kg,
                Some(preexisting),
            );
    }

    let (Some(mixed), Some(output), Some(flow)) = (
        operands.mixed_air_enthalpy_j_per_kg,
        predecessor.resulting_cooling_total_output_w,
        operands.supply_mass_flow_rate_kg_per_s,
    ) else {
        return false;
    };
    let specific = output / flow;
    let calculated = mixed - specific;
    snapshot.cp329_retained_mixed_air_enthalpy_owned_read
        && snapshot.mixed_air_enthalpy_read
        && option_bits_equal(snapshot.mixed_air_enthalpy_j_per_kg, Some(mixed))
        && snapshot.cp384_retained_cooling_total_output_owned_read
        && snapshot.cooling_total_output_read
        && option_bits_equal(snapshot.cooling_total_output_w, Some(output))
        && snapshot.cp330_retained_supply_mass_flow_rate_owned_read
        && snapshot.supply_mass_flow_rate_read
        && option_bits_equal(snapshot.supply_mass_flow_rate_kg_per_s, Some(flow))
        && snapshot.specific_cooling_output_calculated
        && option_bits_equal(snapshot.specific_cooling_output_j_per_kg, Some(specific))
        && snapshot.supply_enthalpy_difference_calculated
        && option_bits_equal(
            snapshot.calculated_supply_enthalpy_j_per_kg,
            Some(calculated),
        )
        && snapshot.supply_enthalpy_assigned
        && option_bits_equal(snapshot.assigned_supply_enthalpy_j_per_kg, Some(calculated))
        && option_bits_equal(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            Some(calculated),
        )
}

fn provenance_flags(snapshot: Snapshot) -> [bool; 10] {
    [
        snapshot.cp379_retained_supply_enthalpy_owned_read,
        snapshot.cp329_retained_mixed_air_enthalpy_owned_read,
        snapshot.mixed_air_enthalpy_read,
        snapshot.cp384_retained_cooling_total_output_owned_read,
        snapshot.cooling_total_output_read,
        snapshot.cp330_retained_supply_mass_flow_rate_owned_read,
        snapshot.supply_mass_flow_rate_read,
        snapshot.specific_cooling_output_calculated,
        snapshot.supply_enthalpy_difference_calculated,
        snapshot.supply_enthalpy_assigned,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_bits_distinguish_signed_zero_and_keep_nan_payload() {
        let nan = f64::from_bits(0x7ff8_0000_0000_0385);
        assert!(option_bits_equal(Some(nan), Some(nan)));
        assert!(!option_bits_equal(Some(0.0), Some(-0.0)));
    }
}
