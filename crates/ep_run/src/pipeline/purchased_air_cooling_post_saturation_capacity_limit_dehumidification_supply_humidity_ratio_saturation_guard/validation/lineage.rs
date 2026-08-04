//! Bit-exact CP412-to-CP413 latest-snapshot lineage validation.

use ep_runtime::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardSnapshot as Snapshot,
};

pub(super) fn lineage_is_exact(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    let active = predecessor
        .post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_executed;
    let saturation = active
        .then_some(predecessor.resulting_saturation_supply_humidity_ratio)
        .flatten();
    let original = active
        .then_some(predecessor.resulting_supply_humidity_ratio_original)
        .flatten();
    let comparison = saturation
        .zip(original)
        .map(|(saturation, original)| saturation < original);

    snapshot.system == predecessor.system
        && snapshot.parent_call_ordinal == predecessor.parent_call_ordinal
        && snapshot.controlled_zone == predecessor.controlled_zone
        && snapshot.unit_off_skipped == predecessor.unit_off_skipped
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
        && snapshot
            .predecessor_dehumidification_total_output_capacity_adjustment_body_entered
            == predecessor
                .predecessor_dehumidification_total_output_capacity_adjustment_body_entered
        && snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough
            == predecessor
                .predecessor_dehumidification_total_output_capacity_guard_false_fallthrough
        && snapshot.dehumidification_total_output_capacity_guard_false_fallthrough
            == predecessor.dehumidification_total_output_capacity_guard_false_fallthrough
        && snapshot.dehumidification_total_output_maximum_capacity_assignment_executed
            == predecessor.dehumidification_total_output_maximum_capacity_assignment_executed
        && snapshot.predecessor_supply_enthalpy_assignment_executed
            == predecessor.predecessor_supply_enthalpy_assignment_executed
        && snapshot.predecessor_dehumidification_control_type_read
            == predecessor.predecessor_dehumidification_control_type_read
        && snapshot.predecessor_dehumidification_control_type
            == predecessor.predecessor_dehumidification_control_type
        && snapshot.predecessor_dehumidification_control_switch_dispatched
            == predecessor.predecessor_dehumidification_control_switch_dispatched
        && snapshot
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered
            == predecessor
                .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered
        && snapshot
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break
            == predecessor
                .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break
        && snapshot.predecessor_dehumidification_control_humidistat_case_entered
            == predecessor.predecessor_dehumidification_control_humidistat_case_entered
        && snapshot
            .predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed
            == predecessor
                .predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed
        && snapshot
            .predecessor_dehumidification_control_humidistat_case_exited_via_break
            == predecessor
                .predecessor_dehumidification_control_humidistat_case_exited_via_break
        && snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered
            == predecessor
                .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered
        && snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough
            == predecessor
                .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough
        && snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed
            == predecessor
                .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed
        && snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break
            == predecessor
                .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break
        && option_bits_match(
            snapshot.predecessor_cp409_resulting_supply_humidity_ratio,
            predecessor.predecessor_cp409_resulting_supply_humidity_ratio,
        )
        && option_bits_match(
            snapshot.predecessor_cp409_resulting_supply_enthalpy_j_per_kg,
            predecessor.predecessor_cp409_resulting_supply_enthalpy_j_per_kg,
        )
        && option_bits_match(
            snapshot.predecessor_cp409_resulting_supply_temperature_c,
            predecessor.predecessor_cp409_resulting_supply_temperature_c,
        )
        && snapshot.predecessor_dehumidification_control_default_case_exited_via_break
            == predecessor.predecessor_dehumidification_control_default_case_exited_via_break
        && option_bits_match(
            snapshot.predecessor_cp410_resulting_supply_humidity_ratio,
            predecessor.predecessor_cp410_resulting_supply_humidity_ratio,
        )
        && option_bits_match(
            snapshot.predecessor_cp410_resulting_supply_enthalpy_j_per_kg,
            predecessor.predecessor_cp410_resulting_supply_enthalpy_j_per_kg,
        )
        && option_bits_match(
            snapshot.predecessor_cp410_resulting_supply_temperature_c,
            predecessor.predecessor_cp410_resulting_supply_temperature_c,
        )
        && snapshot
            .post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_executed
            == predecessor
                .post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_executed
        && snapshot.cp410_retained_supply_humidity_ratio_state_owned
            == predecessor.cp410_retained_supply_humidity_ratio_state_owned
        && snapshot.cp410_retained_supply_enthalpy_state_owned
            == predecessor.cp410_retained_supply_enthalpy_state_owned
        && snapshot.cp410_retained_supply_temperature_state_owned
            == predecessor.cp410_retained_supply_temperature_state_owned
        && snapshot.cp410_retained_supply_humidity_ratio_owned_read
            == predecessor.cp410_retained_supply_humidity_ratio_owned_read
        && snapshot.purchased_air_supply_humidity_ratio_read
            == predecessor.purchased_air_supply_humidity_ratio_read
        && option_bits_match(
            snapshot.purchased_air_supply_humidity_ratio_before_saturation_check,
            predecessor.purchased_air_supply_humidity_ratio_before_saturation_check,
        )
        && snapshot.local_supply_humidity_ratio_original_assignment_performed
            == predecessor.local_supply_humidity_ratio_original_assignment_performed
        && option_bits_match(
            snapshot.assigned_supply_humidity_ratio_original,
            predecessor.assigned_supply_humidity_ratio_original,
        )
        && option_bits_match(
            snapshot.resulting_supply_humidity_ratio_original,
            predecessor.resulting_supply_humidity_ratio_original,
        )
        && option_bits_match(
            snapshot.predecessor_cp411_resulting_supply_humidity_ratio,
            predecessor.predecessor_cp411_resulting_supply_humidity_ratio,
        )
        && option_bits_match(
            snapshot.predecessor_cp411_resulting_supply_enthalpy_j_per_kg,
            predecessor.predecessor_cp411_resulting_supply_enthalpy_j_per_kg,
        )
        && option_bits_match(
            snapshot.predecessor_cp411_resulting_supply_temperature_c,
            predecessor.predecessor_cp411_resulting_supply_temperature_c,
        )
        && snapshot
            .post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_executed
            == active
        && snapshot.cp411_retained_supply_humidity_ratio_state_owned
            == predecessor.cp411_retained_supply_humidity_ratio_state_owned
        && snapshot.cp411_retained_supply_enthalpy_state_owned
            == predecessor.cp411_retained_supply_enthalpy_state_owned
        && snapshot.cp411_retained_supply_temperature_state_owned
            == predecessor.cp411_retained_supply_temperature_state_owned
        && snapshot.cp411_retained_supply_temperature_owned_read
            == predecessor.cp411_retained_supply_temperature_owned_read
        && snapshot.purchased_air_supply_temperature_for_saturation_humidity_ratio_read
            == predecessor.purchased_air_supply_temperature_for_saturation_humidity_ratio_read
        && option_bits_match(
            snapshot.supply_temperature_for_saturation_humidity_ratio_c,
            predecessor.supply_temperature_for_saturation_humidity_ratio_c,
        )
        && snapshot.environment_outdoor_barometric_pressure_owned_read
            == predecessor.environment_outdoor_barometric_pressure_owned_read
        && snapshot.environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read
            == predecessor.environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read
        && option_bits_match(
            snapshot.outdoor_barometric_pressure_pa,
            predecessor.outdoor_barometric_pressure_pa,
        )
        && snapshot.psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluated
            == predecessor.psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluated
        && option_bits_match(
            snapshot.saturation_supply_humidity_ratio,
            predecessor.saturation_supply_humidity_ratio,
        )
        && snapshot.local_saturation_supply_humidity_ratio_assignment_performed
            == predecessor.local_saturation_supply_humidity_ratio_assignment_performed
        && option_bits_match(
            snapshot.assigned_saturation_supply_humidity_ratio,
            predecessor.assigned_saturation_supply_humidity_ratio,
        )
        && option_bits_match(
            snapshot.resulting_saturation_supply_humidity_ratio,
            predecessor.resulting_saturation_supply_humidity_ratio,
        )
        && option_bits_match(
            snapshot.predecessor_cp412_resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
        && option_bits_match(
            snapshot.predecessor_cp412_resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && option_bits_match(
            snapshot.predecessor_cp412_resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
        && snapshot
            .post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_evaluated
            == active
        && snapshot.cp412_saturation_supply_humidity_ratio_owned_read == active
        && snapshot.saturation_supply_humidity_ratio_for_guard_read == active
        && option_bits_match(snapshot.saturation_supply_humidity_ratio_for_guard, saturation)
        && snapshot.cp411_original_supply_humidity_ratio_owned_read == active
        && snapshot.cp412_same_call_original_supply_humidity_ratio_bit_corroborated == active
        && snapshot.original_supply_humidity_ratio_for_guard_read == active
        && option_bits_match(snapshot.original_supply_humidity_ratio_for_guard, original)
        && snapshot.saturation_original_supply_humidity_ratio_comparison_evaluated == active
        && snapshot.saturation_supply_humidity_ratio_strictly_less_than_original_supply_humidity_ratio
            == comparison
        && snapshot.saturation_supply_humidity_ratio_guard_body_entered
            == (comparison == Some(true))
        && snapshot.saturation_supply_humidity_ratio_guard_false_fallthrough
            == (comparison == Some(false))
        && snapshot.cp412_retained_supply_humidity_ratio_state_owned
            == predecessor.resulting_supply_humidity_ratio.is_some()
        && snapshot.cp412_retained_supply_enthalpy_state_owned
            == predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        && snapshot.cp412_retained_supply_temperature_state_owned
            == predecessor.resulting_supply_temperature_c.is_some()
        && option_bits_match(
            snapshot.resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
        && option_bits_match(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && option_bits_match(
            snapshot.resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
        && (!active
            || option_bits_match(
                predecessor.resulting_supply_humidity_ratio_original,
                predecessor.predecessor_cp411_resulting_supply_humidity_ratio,
            ))
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_bits_distinguish_signed_zero() {
        assert!(option_bits_match(Some(-0.0), Some(-0.0)));
        assert!(!option_bits_match(Some(-0.0), Some(0.0)));
    }
}
