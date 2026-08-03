//! Exact CP407/CP329-to-CP408 snapshot lineage validation.

use ep_runtime::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot as MixedAirOwner,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirLimitSnapshot as Snapshot,
};

pub(super) fn lineage_is_exact(
    snapshot: Snapshot,
    predecessor: Predecessor,
    mixed_air_owner: Option<MixedAirOwner>,
) -> bool {
    snapshot.system == predecessor.system
        && snapshot.parent_call_ordinal == predecessor.parent_call_ordinal
        && snapshot.controlled_zone == predecessor.controlled_zone
        && snapshot.predecessor_dehumidification_control_type
            == predecessor.predecessor_dehumidification_control_type
        && inherited_flags(snapshot) == predecessor_flags(predecessor)
        && predecessor_local_flags(snapshot) == local_flags(predecessor)
        && predecessor_values(snapshot)
            .into_iter()
            .zip(local_values(predecessor))
            .all(|(left, right)| option_bits_equal(left, right))
        && local_shape_is_exact(snapshot, predecessor, mixed_air_owner)
}

fn local_shape_is_exact(
    snapshot: Snapshot,
    predecessor: Predecessor,
    mixed_air_owner: Option<MixedAirOwner>,
) -> bool {
    let executed = predecessor
        .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_executed;
    if snapshot
        .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_executed
        != executed
    {
        return false;
    }
    if !executed {
        return inactive_shape_is_exact(snapshot, predecessor) && mixed_air_owner.is_none();
    }
    let (Some(owner), Some(supply_temperature), Some(mixed_air_temperature)) = (
        mixed_air_owner,
        predecessor.resulting_supply_temperature_c,
        mixed_air_owner.and_then(|owner| owner.mixed_air_temperature_c),
    ) else {
        return false;
    };
    let expected = source_minimum(supply_temperature, mixed_air_temperature);

    owner.system == predecessor.system
        && owner.parent_call_ordinal == predecessor.parent_call_ordinal
        && owner.controlled_zone == predecessor.controlled_zone
        && owner.cooling_call_executed
        && owner.mixed_air_temperature_assigned
        && snapshot.cp407_retained_supply_temperature_state_owned
        && option_has_bits(
            snapshot.preexisting_supply_temperature_c,
            supply_temperature,
        )
        && snapshot.cp407_retained_supply_temperature_owned_read
        && snapshot.supply_temperature_for_minimum_read
        && option_has_bits(
            snapshot.supply_temperature_before_mixed_air_limit_c,
            supply_temperature,
        )
        && snapshot.cp329_retained_mixed_air_temperature_owned_read
        && snapshot.mixed_air_temperature_for_minimum_read
        && option_has_bits(snapshot.mixed_air_temperature_c, mixed_air_temperature)
        && snapshot.source_shaped_two_argument_minimum_evaluated
        && option_has_bits(snapshot.minimum_supply_temperature_c, expected)
        && snapshot.supply_temperature_assignment_performed
        && option_has_bits(snapshot.assigned_supply_temperature_c, expected)
        && option_bits_equal(
            snapshot.resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
        && option_bits_equal(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && option_has_bits(snapshot.resulting_supply_temperature_c, expected)
}

fn inactive_shape_is_exact(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    snapshot.cp407_retained_supply_temperature_state_owned
        == predecessor.resulting_supply_temperature_c.is_some()
        && option_bits_equal(
            snapshot.preexisting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
        && !snapshot.cp407_retained_supply_temperature_owned_read
        && !snapshot.supply_temperature_for_minimum_read
        && snapshot
            .supply_temperature_before_mixed_air_limit_c
            .is_none()
        && !snapshot.cp329_retained_mixed_air_temperature_owned_read
        && !snapshot.mixed_air_temperature_for_minimum_read
        && snapshot.mixed_air_temperature_c.is_none()
        && !snapshot.source_shaped_two_argument_minimum_evaluated
        && snapshot.minimum_supply_temperature_c.is_none()
        && !snapshot.supply_temperature_assignment_performed
        && snapshot.assigned_supply_temperature_c.is_none()
        && option_bits_equal(
            snapshot.resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
        && option_bits_equal(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && option_bits_equal(
            snapshot.resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
}

fn inherited_flags(snapshot: Snapshot) -> [bool; 33] {
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
        snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated,
        snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_maximum_capacity_assignment_executed,
        snapshot.predecessor_supply_enthalpy_assignment_executed,
        snapshot.predecessor_dehumidification_control_type_read,
        snapshot.predecessor_dehumidification_control_switch_dispatched,
        snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered,
        snapshot
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break,
        snapshot.predecessor_dehumidification_control_humidistat_case_entered,
        snapshot
            .predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed,
        snapshot.predecessor_dehumidification_control_humidistat_case_exited_via_break,
        snapshot.predecessor_dehumidification_control_none_case_entered,
        snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered,
        snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough,
        snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed,
        snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entered,
    ]
}

fn predecessor_flags(snapshot: Predecessor) -> [bool; 33] {
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
        snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated,
        snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_maximum_capacity_assignment_executed,
        snapshot.predecessor_supply_enthalpy_assignment_executed,
        snapshot.predecessor_dehumidification_control_type_read,
        snapshot.predecessor_dehumidification_control_switch_dispatched,
        snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered,
        snapshot
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break,
        snapshot.predecessor_dehumidification_control_humidistat_case_entered,
        snapshot
            .predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed,
        snapshot.predecessor_dehumidification_control_humidistat_case_exited_via_break,
        snapshot.predecessor_dehumidification_control_none_case_entered,
        snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered,
        snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough,
        snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed,
        snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entered,
    ]
}

fn predecessor_local_flags(snapshot: Snapshot) -> [bool; 9] {
    [
        snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_executed,
        snapshot.predecessor_cp385_retained_supply_enthalpy_owned_read,
        snapshot.predecessor_cp406_same_call_supply_enthalpy_bit_corroborated,
        snapshot.predecessor_supply_enthalpy_for_dry_bulb_inversion_read,
        snapshot.predecessor_cp378_retained_supply_humidity_ratio_owned_read,
        snapshot.predecessor_supply_humidity_ratio_for_dry_bulb_inversion_read,
        snapshot.predecessor_cp406_retained_supply_temperature_state_owned,
        snapshot.predecessor_psychrometric_supply_temperature_evaluated,
        snapshot.predecessor_supply_temperature_assigned,
    ]
}

fn local_flags(snapshot: Predecessor) -> [bool; 9] {
    [
        snapshot.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_executed,
        snapshot.cp385_retained_supply_enthalpy_owned_read,
        snapshot.cp406_same_call_supply_enthalpy_bit_corroborated,
        snapshot.supply_enthalpy_for_dry_bulb_inversion_read,
        snapshot.cp378_retained_supply_humidity_ratio_owned_read,
        snapshot.supply_humidity_ratio_for_dry_bulb_inversion_read,
        snapshot.cp406_retained_supply_temperature_state_owned,
        snapshot.psychrometric_supply_temperature_evaluated,
        snapshot.supply_temperature_assigned,
    ]
}

fn predecessor_values(snapshot: Snapshot) -> [Option<f64>; 11] {
    [
        snapshot.predecessor_cp406_resulting_supply_humidity_ratio,
        snapshot.predecessor_cp406_resulting_supply_enthalpy_j_per_kg,
        snapshot.predecessor_cp406_resulting_supply_temperature_c,
        snapshot.predecessor_supply_enthalpy_j_per_kg,
        snapshot.predecessor_supply_humidity_ratio,
        snapshot.predecessor_preexisting_supply_temperature_c,
        snapshot.predecessor_psychrometric_supply_temperature_result_c,
        snapshot.predecessor_assigned_supply_temperature_c,
        snapshot.predecessor_resulting_supply_humidity_ratio,
        snapshot.predecessor_resulting_supply_enthalpy_j_per_kg,
        snapshot.predecessor_resulting_supply_temperature_c,
    ]
}

fn local_values(snapshot: Predecessor) -> [Option<f64>; 11] {
    [
        snapshot.predecessor_cp406_resulting_supply_humidity_ratio,
        snapshot.predecessor_cp406_resulting_supply_enthalpy_j_per_kg,
        snapshot.predecessor_cp406_resulting_supply_temperature_c,
        snapshot.supply_enthalpy_j_per_kg,
        snapshot.supply_humidity_ratio,
        snapshot.preexisting_supply_temperature_c,
        snapshot.psychrometric_supply_temperature_result_c,
        snapshot.assigned_supply_temperature_c,
        snapshot.resulting_supply_humidity_ratio,
        snapshot.resulting_supply_enthalpy_j_per_kg,
        snapshot.resulting_supply_temperature_c,
    ]
}

fn source_minimum(left: f64, right: f64) -> f64 {
    if left < right { left } else { right }
}

fn option_has_bits(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}

fn option_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
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
    fn source_minimum_is_right_biased_for_ties_and_unordered_left() {
        assert_eq!(source_minimum(1.0, 2.0).to_bits(), 1.0f64.to_bits());
        assert_eq!(source_minimum(2.0, 1.0).to_bits(), 1.0f64.to_bits());
        assert_eq!(source_minimum(0.0, -0.0).to_bits(), (-0.0f64).to_bits());
        let right = f64::from_bits(0x7ff8_0000_0000_0408);
        assert_eq!(source_minimum(f64::NAN, right).to_bits(), right.to_bits());
    }

    #[test]
    fn exact_bits_distinguish_signed_zero() {
        assert!(option_bits_equal(Some(-0.0), Some(-0.0)));
        assert!(!option_bits_equal(Some(-0.0), Some(0.0)));
    }
}
