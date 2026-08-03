//! Exact CP378/CP385/CP406-to-CP407 snapshot lineage validation.

use ep_runtime::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntrySnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentSnapshot as Snapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot as EnthalpyOwner,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot as HumidityOwner,
};

pub(super) fn lineage_is_exact(
    snapshot: Snapshot,
    predecessor: Predecessor,
    enthalpy_owner: EnthalpyOwner,
    humidity_owner: HumidityOwner,
) -> bool {
    inherited_shape_matches(snapshot, predecessor)
        && predecessor_carriers_match(snapshot, predecessor)
        && local_shape_is_exact(snapshot, predecessor, enthalpy_owner, humidity_owner)
}

fn inherited_shape_matches(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    snapshot.system == predecessor.system
        && snapshot.parent_call_ordinal == predecessor.parent_call_ordinal
        && snapshot.controlled_zone == predecessor.controlled_zone
        && snapshot.predecessor_dehumidification_control_type
            == predecessor.predecessor_dehumidification_control_type
        && inherited_flags(snapshot) == predecessor_flags(predecessor)
}

fn predecessor_carriers_match(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    [
        (
            snapshot.predecessor_cp406_resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        ),
        (
            snapshot.predecessor_cp406_resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        ),
        (
            snapshot.predecessor_cp406_resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        ),
    ]
    .into_iter()
    .all(|(left, right)| option_bits_equal(left, right))
}

fn local_shape_is_exact(
    snapshot: Snapshot,
    predecessor: Predecessor,
    enthalpy_owner: EnthalpyOwner,
    humidity_owner: HumidityOwner,
) -> bool {
    let executed = predecessor
        .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entered;
    if snapshot
        .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_executed
        != executed
    {
        return false;
    }
    if !executed {
        return inactive_shape_is_exact(snapshot, predecessor);
    }
    let (Some(enthalpy), Some(humidity), Some(preexisting_temperature)) = (
        enthalpy_owner.resulting_supply_enthalpy_j_per_kg,
        humidity_owner.resulting_supply_humidity_ratio,
        predecessor.resulting_supply_temperature_c,
    ) else {
        return false;
    };
    let expected = ep_runtime::psychrometrics::energyplus_psy_tdb_fn_h_w(enthalpy, humidity);
    predecessor.resulting_supply_humidity_ratio.is_none()
        && option_has_bits(predecessor.resulting_supply_enthalpy_j_per_kg, enthalpy)
        && snapshot.cp385_retained_supply_enthalpy_owned_read
        && snapshot.cp406_same_call_supply_enthalpy_bit_corroborated
        && snapshot.supply_enthalpy_for_dry_bulb_inversion_read
        && option_has_bits(snapshot.supply_enthalpy_j_per_kg, enthalpy)
        && snapshot.cp378_retained_supply_humidity_ratio_owned_read
        && snapshot.supply_humidity_ratio_for_dry_bulb_inversion_read
        && option_has_bits(snapshot.supply_humidity_ratio, humidity)
        && snapshot.cp406_retained_supply_temperature_state_owned
        && option_has_bits(
            snapshot.preexisting_supply_temperature_c,
            preexisting_temperature,
        )
        && snapshot.psychrometric_supply_temperature_evaluated
        && option_has_bits(snapshot.psychrometric_supply_temperature_result_c, expected)
        && snapshot.supply_temperature_assigned
        && option_has_bits(snapshot.assigned_supply_temperature_c, expected)
        && option_has_bits(snapshot.resulting_supply_humidity_ratio, humidity)
        && option_has_bits(snapshot.resulting_supply_enthalpy_j_per_kg, enthalpy)
        && option_has_bits(snapshot.resulting_supply_temperature_c, expected)
}

fn inactive_shape_is_exact(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    !snapshot.cp385_retained_supply_enthalpy_owned_read
        && !snapshot.cp406_same_call_supply_enthalpy_bit_corroborated
        && !snapshot.supply_enthalpy_for_dry_bulb_inversion_read
        && snapshot.supply_enthalpy_j_per_kg.is_none()
        && !snapshot.cp378_retained_supply_humidity_ratio_owned_read
        && !snapshot.supply_humidity_ratio_for_dry_bulb_inversion_read
        && snapshot.supply_humidity_ratio.is_none()
        && snapshot.cp406_retained_supply_temperature_state_owned
            == predecessor.resulting_supply_temperature_c.is_some()
        && option_bits_equal(
            snapshot.preexisting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
        && !snapshot.psychrometric_supply_temperature_evaluated
        && snapshot.psychrometric_supply_temperature_result_c.is_none()
        && !snapshot.supply_temperature_assigned
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
            .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entered,
    ]
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
    fn exact_bits_distinguish_signed_zero() {
        assert!(option_bits_equal(Some(-0.0), Some(-0.0)));
        assert!(!option_bits_equal(Some(-0.0), Some(0.0)));
    }
}
