use super::*;

pub(super) fn metadata_is_exact(
    snapshot: Snapshot,
    predecessor: PredecessorSnapshot,
    expected_system: IdealLoadsAirSystemId,
    expected_zone: ZoneId,
    calls: usize,
) -> bool {
    snapshot.source
        == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE_ORDER
        && snapshot.system == expected_system
        && predecessor.system == expected_system
        && snapshot.controlled_zone == expected_zone
        && predecessor.controlled_zone == expected_zone
        && snapshot.parent_call_ordinal == calls
        && predecessor.parent_call_ordinal == calls
}

pub(super) fn links_exactly(snapshot: Snapshot, predecessor: PredecessorSnapshot) -> bool {
    inherited_lineage_is_exact(snapshot, predecessor)
        && assignment_shape_is_exact(snapshot, predecessor.dehumidification_body_entered)
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
            == predecessor.dehumidification_guard_evaluated
        && snapshot.predecessor_dehumidification_body_entered
            == predecessor.dehumidification_body_entered
        && snapshot.predecessor_dehumidification_guard_false_fallthrough
            == predecessor.dehumidification_guard_false_fallthrough
}

fn assignment_shape_is_exact(snapshot: Snapshot, active: bool) -> bool {
    if snapshot.dehumidification_total_output_assignment_executed != active
        || assignment_flags(snapshot)
            .into_iter()
            .any(|flag| flag != active)
    {
        return false;
    }
    if !active {
        return numeric_values(snapshot)
            .into_iter()
            .all(|value| value.is_none());
    }
    active_numeric_grouping_is_exact(snapshot)
}

fn assignment_flags(snapshot: Snapshot) -> [bool; 14] {
    [
        snapshot.cp330_supply_mass_flow_rate_owned_read,
        snapshot.cp329_same_call_supply_mass_flow_rate_bit_corroborated,
        snapshot.cp339_same_call_supply_mass_flow_rate_bit_corroborated,
        snapshot.supply_mass_flow_rate_read,
        snapshot.cp329_mixed_air_enthalpy_owned_read,
        snapshot.cp329_same_call_recirculation_enthalpy_bit_corroborated,
        snapshot.cp339_same_call_mixed_air_enthalpy_bit_corroborated,
        snapshot.mixed_air_enthalpy_read,
        snapshot.cp379_post_saturation_supply_enthalpy_owned_read,
        snapshot.cp379_same_call_supply_enthalpy_bits_corroborated,
        snapshot.supply_enthalpy_read,
        snapshot.enthalpy_difference_calculated,
        snapshot.cooling_total_output_calculated,
        snapshot.cooling_total_output_assigned,
    ]
}

fn numeric_values(snapshot: Snapshot) -> [Option<f64>; 6] {
    [
        snapshot.supply_mass_flow_rate_kg_per_s,
        snapshot.mixed_air_enthalpy_j_per_kg,
        snapshot.supply_enthalpy_j_per_kg,
        snapshot.mixed_air_minus_supply_enthalpy_j_per_kg,
        snapshot.calculated_cooling_total_output_w,
        snapshot.cooling_total_output_w,
    ]
}

fn active_numeric_grouping_is_exact(snapshot: Snapshot) -> bool {
    let Some(supply_mass_flow_rate) = snapshot.supply_mass_flow_rate_kg_per_s else {
        return false;
    };
    let Some(mixed_air_enthalpy) = snapshot.mixed_air_enthalpy_j_per_kg else {
        return false;
    };
    let Some(supply_enthalpy) = snapshot.supply_enthalpy_j_per_kg else {
        return false;
    };
    let expected_difference = mixed_air_enthalpy - supply_enthalpy;
    let expected_output = supply_mass_flow_rate * expected_difference;
    option_bits_equal(
        snapshot.mixed_air_minus_supply_enthalpy_j_per_kg,
        Some(expected_difference),
    ) && option_bits_equal(
        snapshot.calculated_cooling_total_output_w,
        Some(expected_output),
    ) && option_bits_equal(snapshot.cooling_total_output_w, Some(expected_output))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_bit_comparison_distinguishes_signed_zero() {
        assert!(!option_bits_equal(Some(0.0), Some(-0.0)));
    }

    #[test]
    fn raw_grouping_preserves_non_finite_ieee_result_bits() {
        let mixed = f64::from_bits(0x7ff8_0000_0000_0382);
        let difference = mixed - 1.0;
        let output = 2.0 * difference;
        assert!(option_bits_equal(Some(difference), Some(mixed - 1.0)));
        assert!(option_bits_equal(Some(output), Some(2.0 * difference)));
    }
}
