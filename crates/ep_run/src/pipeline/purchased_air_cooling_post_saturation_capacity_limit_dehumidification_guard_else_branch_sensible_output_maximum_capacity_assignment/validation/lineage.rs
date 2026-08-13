//! CP422 predecessor-prefix and local assignment shape validation.

use ep_runtime::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentSnapshot as Snapshot,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_predecessor_cp421_snapshot,
};

use crate::pipeline::purchased_air_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard::serialization::snapshot::snapshot_json as predecessor_json;

pub(super) fn links_to_predecessor(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    predecessor_json(
        cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_predecessor_cp421_snapshot(snapshot),
    ) == predecessor_json(predecessor)
}

pub(super) fn operation_shape_is_exact(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    let active = predecessor
        .post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_evaluated;
    let assignment = predecessor
        .post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_adjustment_body_entered;
    if snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_executed != assignment
        || snapshot.cp421_retained_supply_humidity_ratio_state_owned
            != predecessor.resulting_supply_humidity_ratio.is_some()
        || snapshot.cp421_retained_supply_enthalpy_state_owned
            != predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        || snapshot.cp421_retained_supply_temperature_state_owned
            != predecessor.resulting_supply_temperature_c.is_some()
        || !option_bits_equal(snapshot.predecessor_cp421_resulting_supply_humidity_ratio, predecessor.resulting_supply_humidity_ratio)
        || !option_bits_equal(snapshot.predecessor_cp421_resulting_supply_enthalpy_j_per_kg, predecessor.resulting_supply_enthalpy_j_per_kg)
        || !option_bits_equal(snapshot.predecessor_cp421_resulting_supply_temperature_c, predecessor.resulting_supply_temperature_c)
        || !option_bits_equal(snapshot.resulting_supply_humidity_ratio, predecessor.resulting_supply_humidity_ratio)
        || !option_bits_equal(snapshot.resulting_supply_enthalpy_j_per_kg, predecessor.resulting_supply_enthalpy_j_per_kg)
        || !option_bits_equal(snapshot.resulting_supply_temperature_c, predecessor.resulting_supply_temperature_c)
    {
        return false;
    }
    if !active {
        return snapshot
            .preexisting_cooling_sensible_output_for_maximum_capacity_assignment_w
            .is_none()
            && !snapshot.cp421_retained_maximum_total_cooling_capacity_owned_read
            && !snapshot.maximum_total_cooling_capacity_for_sensible_output_assignment_read
            && snapshot
                .maximum_total_cooling_capacity_for_sensible_output_assignment_w
                .is_none()
            && !snapshot.cooling_sensible_output_maximum_capacity_assignment_performed
            && snapshot
                .assigned_cooling_sensible_output_from_maximum_capacity_w
                .is_none()
            && snapshot
                .resulting_cooling_sensible_output_after_maximum_capacity_assignment_w
                .is_none();
    }
    let Some(preexisting) = predecessor.cp420_cooling_sensible_output_for_capacity_guard_w else {
        return false;
    };
    if !option_has_bits(
        snapshot.preexisting_cooling_sensible_output_for_maximum_capacity_assignment_w,
        preexisting,
    ) {
        return false;
    }
    if !assignment {
        return !snapshot.cp421_retained_maximum_total_cooling_capacity_owned_read
            && !snapshot.maximum_total_cooling_capacity_for_sensible_output_assignment_read
            && snapshot
                .maximum_total_cooling_capacity_for_sensible_output_assignment_w
                .is_none()
            && !snapshot.cooling_sensible_output_maximum_capacity_assignment_performed
            && snapshot
                .assigned_cooling_sensible_output_from_maximum_capacity_w
                .is_none()
            && option_has_bits(
                snapshot.resulting_cooling_sensible_output_after_maximum_capacity_assignment_w,
                preexisting,
            );
    }
    let Some(maximum) = predecessor.maximum_total_cooling_capacity_w else {
        return false;
    };
    snapshot.cp421_retained_maximum_total_cooling_capacity_owned_read
        && snapshot.maximum_total_cooling_capacity_for_sensible_output_assignment_read
        && option_has_bits(
            snapshot.maximum_total_cooling_capacity_for_sensible_output_assignment_w,
            maximum,
        )
        && snapshot.cooling_sensible_output_maximum_capacity_assignment_performed
        && option_has_bits(
            snapshot.assigned_cooling_sensible_output_from_maximum_capacity_w,
            maximum,
        )
        && option_has_bits(
            snapshot.resulting_cooling_sensible_output_after_maximum_capacity_assignment_w,
            maximum,
        )
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
