//! Bit-exact CP414/CP329-to-CP415 latest-snapshot lineage validation.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallSnapshot as MixedAirOwner,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitSnapshot as Snapshot,
};

macro_rules! plain_fields_match {
    ($snapshot:expr, $predecessor:expr, $($field:ident),+ $(,)?) => {
        true $(&& $snapshot.$field == $predecessor.$field)+
    };
}

macro_rules! option_fields_match {
    ($snapshot:expr, $predecessor:expr, $($field:ident),+ $(,)?) => {
        true $(&& option_bits_match($snapshot.$field, $predecessor.$field))+
    };
}

pub(super) fn lineage_is_exact(
    snapshot: Snapshot,
    predecessor: Predecessor,
    mixed_air_owner: Option<MixedAirOwner>,
) -> bool {
    snapshot.system == predecessor.system
        && snapshot.parent_call_ordinal == predecessor.parent_call_ordinal
        && snapshot.controlled_zone == predecessor.controlled_zone
        && inherited_plain_fields_match(snapshot, predecessor)
        && inherited_option_fields_match(snapshot, predecessor)
        && option_bits_match(
            snapshot.predecessor_cp414_resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
        && option_bits_match(
            snapshot.predecessor_cp414_resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && option_bits_match(
            snapshot.predecessor_cp414_resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
        && local_shape_is_exact(snapshot, predecessor, mixed_air_owner)
}

fn inherited_plain_fields_match(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    plain_fields_match!(
        snapshot,
        predecessor,
        unit_off_skipped,
        non_cooling_skipped,
        positive_guard_false_fallthrough_skipped,
        heating_availability_guard_false_fallthrough,
        humidification_control_guard_false_fallthrough,
        dehumidification_control_humidistat_maximum_assignment_executed,
        dehumidification_control_none_maximum_assignment_executed,
        dehumidification_control_guard_false_fallthrough,
        predecessor_capacity_limit_guard_evaluated,
        predecessor_capacity_limit_body_entered,
        predecessor_active_capacity_limit_guard_false_fallthrough,
        predecessor_dehumidification_guard_evaluated,
        predecessor_dehumidification_body_entered,
        predecessor_dehumidification_guard_false_fallthrough,
        predecessor_dehumidification_total_output_assignment_executed,
        predecessor_dehumidification_total_output_capacity_guard_evaluated,
        predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_maximum_capacity_assignment_executed,
        predecessor_supply_enthalpy_assignment_executed,
        predecessor_dehumidification_control_type_read,
        predecessor_dehumidification_control_type,
        predecessor_dehumidification_control_switch_dispatched,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break,
        predecessor_dehumidification_control_humidistat_case_entered,
        predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed,
        predecessor_dehumidification_control_humidistat_case_exited_via_break,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break,
        predecessor_dehumidification_control_default_case_exited_via_break,
        post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_executed,
        cp410_retained_supply_humidity_ratio_state_owned,
        cp410_retained_supply_enthalpy_state_owned,
        cp410_retained_supply_temperature_state_owned,
        cp410_retained_supply_humidity_ratio_owned_read,
        purchased_air_supply_humidity_ratio_read,
        local_supply_humidity_ratio_original_assignment_performed,
        post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_executed,
        cp411_retained_supply_humidity_ratio_state_owned,
        cp411_retained_supply_enthalpy_state_owned,
        cp411_retained_supply_temperature_state_owned,
        cp411_retained_supply_temperature_owned_read,
        purchased_air_supply_temperature_for_saturation_humidity_ratio_read,
        environment_outdoor_barometric_pressure_owned_read,
        environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read,
        psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluated,
        local_saturation_supply_humidity_ratio_assignment_performed,
        post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_evaluated,
        cp412_saturation_supply_humidity_ratio_owned_read,
        saturation_supply_humidity_ratio_for_guard_read,
        cp411_original_supply_humidity_ratio_owned_read,
        cp412_same_call_original_supply_humidity_ratio_bit_corroborated,
        original_supply_humidity_ratio_for_guard_read,
        saturation_original_supply_humidity_ratio_comparison_evaluated,
        saturation_supply_humidity_ratio_strictly_less_than_original_supply_humidity_ratio,
        saturation_supply_humidity_ratio_guard_body_entered,
        saturation_supply_humidity_ratio_guard_false_fallthrough,
        cp412_retained_supply_humidity_ratio_state_owned,
        cp412_retained_supply_enthalpy_state_owned,
        cp412_retained_supply_temperature_state_owned,
        post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_executed,
        cp413_retained_supply_humidity_ratio_state_owned,
        cp413_retained_supply_enthalpy_state_owned,
        cp413_retained_supply_temperature_state_owned,
        cp413_retained_supply_enthalpy_owned_read,
        supply_enthalpy_for_saturation_temperature_read,
        environment_outdoor_barometric_pressure_for_saturation_temperature_owned_read,
        environment_outdoor_barometric_pressure_for_saturation_temperature_read,
        psy_tsat_fn_h_pb_evaluated,
        purchased_air_supply_temperature_saturation_assignment_performed,
    )
}

fn inherited_option_fields_match(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    option_fields_match!(
        snapshot,
        predecessor,
        predecessor_cp409_resulting_supply_humidity_ratio,
        predecessor_cp409_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp409_resulting_supply_temperature_c,
        predecessor_cp410_resulting_supply_humidity_ratio,
        predecessor_cp410_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp410_resulting_supply_temperature_c,
        purchased_air_supply_humidity_ratio_before_saturation_check,
        assigned_supply_humidity_ratio_original,
        resulting_supply_humidity_ratio_original,
        predecessor_cp411_resulting_supply_humidity_ratio,
        predecessor_cp411_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp411_resulting_supply_temperature_c,
        supply_temperature_for_saturation_humidity_ratio_c,
        outdoor_barometric_pressure_pa,
        saturation_supply_humidity_ratio,
        assigned_saturation_supply_humidity_ratio,
        resulting_saturation_supply_humidity_ratio,
        predecessor_cp412_resulting_supply_humidity_ratio,
        predecessor_cp412_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp412_resulting_supply_temperature_c,
        saturation_supply_humidity_ratio_for_guard,
        original_supply_humidity_ratio_for_guard,
        predecessor_cp413_resulting_supply_humidity_ratio,
        predecessor_cp413_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp413_resulting_supply_temperature_c,
        supply_enthalpy_for_saturation_temperature_j_per_kg,
        outdoor_barometric_pressure_for_saturation_temperature_pa,
        psychrometric_saturation_supply_temperature_result_c,
        assigned_saturation_supply_temperature_c,
    )
}

fn local_shape_is_exact(
    snapshot: Snapshot,
    predecessor: Predecessor,
    mixed_air_owner: Option<MixedAirOwner>,
) -> bool {
    let active = predecessor
        .post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_executed;
    let supply_temperature = active
        .then_some(predecessor.resulting_supply_temperature_c)
        .flatten();
    let mixed_air_temperature = active
        .then_some(mixed_air_owner.and_then(|owner| owner.mixed_air_temperature_c))
        .flatten();
    let minimum = supply_temperature
        .zip(mixed_air_temperature)
        .map(|(left, right)| source_minimum(left, right));

    snapshot
        .post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_executed
        == active
        && snapshot.cp414_retained_supply_temperature_state_owned
            == predecessor.resulting_supply_temperature_c.is_some()
        && option_bits_match(
            snapshot.preexisting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
        && mixed_air_owner_is_exact(mixed_air_owner, predecessor, active)
        && direct_subset_values_are_valid(
            active,
            supply_temperature,
            mixed_air_temperature,
            minimum,
        )
        && snapshot.cp414_retained_supply_temperature_owned_read == active
        && snapshot.supply_temperature_for_minimum_read == active
        && option_bits_match(
            snapshot.supply_temperature_before_mixed_air_limit_c,
            supply_temperature,
        )
        && snapshot.cp329_retained_mixed_air_temperature_owned_read == active
        && snapshot.mixed_air_temperature_for_minimum_read == active
        && option_bits_match(snapshot.mixed_air_temperature_c, mixed_air_temperature)
        && snapshot.source_shaped_two_argument_minimum_evaluated == active
        && option_bits_match(snapshot.minimum_supply_temperature_c, minimum)
        && snapshot.supply_temperature_assignment_performed == active
        && option_bits_match(snapshot.assigned_supply_temperature_c, minimum)
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
            if active {
                minimum
            } else {
                predecessor.resulting_supply_temperature_c
            },
        )
}

fn mixed_air_owner_is_exact(
    owner: Option<MixedAirOwner>,
    predecessor: Predecessor,
    active: bool,
) -> bool {
    if !active {
        return owner.is_none();
    }
    let Some(owner) = owner else {
        return false;
    };
    owner.source == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE
        && owner.child_source == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE
        && owner.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE
        && owner.source_order == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER
        && owner.system == predecessor.system
        && owner.parent_call_ordinal == predecessor.parent_call_ordinal
        && owner.controlled_zone == predecessor.controlled_zone
        && owner.cooling_call_executed
        && owner.mixed_air_temperature_output_reference_bound
        && owner.mixed_air_temperature_assigned
        && owner.mixed_air_temperature_c.is_some_and(f64::is_finite)
}

fn direct_subset_values_are_valid(
    active: bool,
    supply_temperature: Option<f64>,
    mixed_air_temperature: Option<f64>,
    minimum: Option<f64>,
) -> bool {
    if !active {
        return supply_temperature.is_none()
            && mixed_air_temperature.is_none()
            && minimum.is_none();
    }
    supply_temperature.is_some_and(f64::is_finite)
        && mixed_air_temperature.is_some_and(f64::is_finite)
        && minimum.is_some_and(f64::is_finite)
}

fn source_minimum(left: f64, right: f64) -> f64 {
    if left < right { left } else { right }
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
    fn source_minimum_is_right_biased_for_ties_and_unordered_left() {
        assert_eq!(source_minimum(1.0, 2.0).to_bits(), 1.0f64.to_bits());
        assert_eq!(source_minimum(2.0, 1.0).to_bits(), 1.0f64.to_bits());
        assert_eq!(source_minimum(0.0, -0.0).to_bits(), (-0.0f64).to_bits());
        let right = f64::from_bits(0x7ff8_0000_0000_0415);
        assert_eq!(source_minimum(f64::NAN, right).to_bits(), right.to_bits());
    }

    #[test]
    fn active_direct_subset_rejects_nonfinite_operands_and_result() {
        assert!(direct_subset_values_are_valid(
            true,
            Some(14.0),
            Some(20.0),
            Some(14.0),
        ));
        for values in [
            (Some(f64::NAN), Some(20.0), Some(20.0)),
            (Some(f64::INFINITY), Some(20.0), Some(20.0)),
            (Some(14.0), Some(f64::NAN), Some(14.0)),
            (Some(14.0), Some(f64::NEG_INFINITY), Some(14.0)),
            (Some(14.0), Some(20.0), Some(f64::NAN)),
        ] {
            assert!(!direct_subset_values_are_valid(
                true, values.0, values.1, values.2,
            ));
        }
        assert!(direct_subset_values_are_valid(false, None, None, None));
        assert!(!direct_subset_values_are_valid(
            false,
            Some(14.0),
            None,
            None,
        ));
    }

    #[test]
    fn option_bits_distinguish_signed_zero() {
        assert!(option_bits_match(Some(-0.0), Some(-0.0)));
        assert!(!option_bits_match(Some(-0.0), Some(0.0)));
    }
}
