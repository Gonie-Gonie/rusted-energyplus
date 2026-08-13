//! CP423 predecessor-prefix and local assignment shape validation.

use ep_runtime::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentSnapshot as Snapshot,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_predecessor_cp422_snapshot,
};

use crate::pipeline::purchased_air_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment::serialization::snapshot::snapshot_json as predecessor_json;

pub(super) fn links_to_predecessor(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    predecessor_json(
        cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_predecessor_cp422_snapshot(snapshot),
    ) == predecessor_json(predecessor)
}

pub(super) fn operation_shape_is_exact(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    let assignment = predecessor
        .post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_executed;
    if snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_executed != assignment
        || snapshot.cp422_retained_supply_humidity_ratio_state_owned
            != predecessor.resulting_supply_humidity_ratio.is_some()
        || snapshot.cp422_retained_supply_enthalpy_state_owned
            != predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        || snapshot.cp422_retained_supply_temperature_state_owned
            != predecessor.resulting_supply_temperature_c.is_some()
        || !option_bits_equal(snapshot.predecessor_cp422_resulting_supply_humidity_ratio, predecessor.resulting_supply_humidity_ratio)
        || !option_bits_equal(snapshot.predecessor_cp422_resulting_supply_enthalpy_j_per_kg, predecessor.resulting_supply_enthalpy_j_per_kg)
        || !option_bits_equal(snapshot.predecessor_cp422_resulting_supply_temperature_c, predecessor.resulting_supply_temperature_c)
        || !option_bits_equal(snapshot.resulting_supply_humidity_ratio, predecessor.resulting_supply_humidity_ratio)
        || !option_bits_equal(snapshot.resulting_supply_enthalpy_j_per_kg, predecessor.resulting_supply_enthalpy_j_per_kg)
    {
        return false;
    }
    if !assignment {
        return rhs_is_empty(snapshot)
            && option_bits_equal(
                snapshot.resulting_supply_temperature_c,
                predecessor.resulting_supply_temperature_c,
            );
    }
    let (Some(mixed), Some(cooling), Some(mass_flow), Some(cp_air)) = (
        predecessor.mixed_air_temperature_for_sensible_output_c,
        predecessor.resulting_cooling_sensible_output_after_maximum_capacity_assignment_w,
        predecessor.supply_mass_flow_rate_kg_per_s,
        predecessor.cp_air_j_per_kg_k,
    ) else {
        return false;
    };
    let capacity_rate = mass_flow * cp_air;
    let drop = cooling / capacity_rate;
    let calculated = mixed - drop;
    snapshot.cp329_retained_mixed_air_temperature_for_sensible_output_supply_temperature_owned_read
        && snapshot.mixed_air_temperature_for_sensible_output_supply_temperature_read
        && option_has_bits(
            snapshot.mixed_air_temperature_for_sensible_output_supply_temperature_c,
            mixed,
        )
        && snapshot.cp422_retained_cooling_sensible_output_owned_read
        && snapshot.cooling_sensible_output_for_supply_temperature_read
        && option_has_bits(
            snapshot.cooling_sensible_output_for_supply_temperature_w,
            cooling,
        )
        && snapshot
            .cp330_retained_supply_mass_flow_rate_for_sensible_output_supply_temperature_owned_read
        && snapshot
            .cp329_supply_mass_flow_rate_for_sensible_output_supply_temperature_bit_corroborated
        && snapshot.supply_mass_flow_rate_for_sensible_output_supply_temperature_read
        && option_has_bits(
            snapshot.supply_mass_flow_rate_for_sensible_output_supply_temperature_kg_per_s,
            mass_flow,
        )
        && snapshot.cp419_retained_cp_air_for_sensible_output_supply_temperature_owned_read
        && snapshot.cp_air_for_sensible_output_supply_temperature_read
        && option_has_bits(
            snapshot.cp_air_for_sensible_output_supply_temperature_j_per_kg_k,
            cp_air,
        )
        && snapshot
            .supply_mass_flow_rate_times_cp_air_for_sensible_output_supply_temperature_calculated
        && option_has_bits(
            snapshot
                .supply_mass_flow_rate_times_cp_air_for_sensible_output_supply_temperature_w_per_k,
            capacity_rate,
        )
        && snapshot.cooling_sensible_output_over_air_capacity_rate_calculated
        && option_has_bits(
            snapshot.cooling_sensible_output_over_air_capacity_rate_k,
            drop,
        )
        && snapshot.sensible_output_supply_temperature_calculated
        && option_has_bits(
            snapshot.calculated_sensible_output_supply_temperature_c,
            calculated,
        )
        && snapshot.sensible_output_supply_temperature_assignment_performed
        && option_has_bits(
            snapshot.assigned_sensible_output_supply_temperature_c,
            calculated,
        )
        && option_has_bits(snapshot.resulting_supply_temperature_c, calculated)
}

fn rhs_is_empty(snapshot: Snapshot) -> bool {
    !snapshot.cp329_retained_mixed_air_temperature_for_sensible_output_supply_temperature_owned_read
        && !snapshot.mixed_air_temperature_for_sensible_output_supply_temperature_read
        && snapshot
            .mixed_air_temperature_for_sensible_output_supply_temperature_c
            .is_none()
        && !snapshot.cp422_retained_cooling_sensible_output_owned_read
        && !snapshot.cooling_sensible_output_for_supply_temperature_read
        && snapshot
            .cooling_sensible_output_for_supply_temperature_w
            .is_none()
        && !snapshot
            .cp330_retained_supply_mass_flow_rate_for_sensible_output_supply_temperature_owned_read
        && !snapshot
            .cp329_supply_mass_flow_rate_for_sensible_output_supply_temperature_bit_corroborated
        && !snapshot.supply_mass_flow_rate_for_sensible_output_supply_temperature_read
        && snapshot
            .supply_mass_flow_rate_for_sensible_output_supply_temperature_kg_per_s
            .is_none()
        && !snapshot.cp419_retained_cp_air_for_sensible_output_supply_temperature_owned_read
        && !snapshot.cp_air_for_sensible_output_supply_temperature_read
        && snapshot
            .cp_air_for_sensible_output_supply_temperature_j_per_kg_k
            .is_none()
        && !snapshot
            .supply_mass_flow_rate_times_cp_air_for_sensible_output_supply_temperature_calculated
        && snapshot
            .supply_mass_flow_rate_times_cp_air_for_sensible_output_supply_temperature_w_per_k
            .is_none()
        && !snapshot.cooling_sensible_output_over_air_capacity_rate_calculated
        && snapshot
            .cooling_sensible_output_over_air_capacity_rate_k
            .is_none()
        && !snapshot.sensible_output_supply_temperature_calculated
        && snapshot
            .calculated_sensible_output_supply_temperature_c
            .is_none()
        && !snapshot.sensible_output_supply_temperature_assignment_performed
        && snapshot
            .assigned_sensible_output_supply_temperature_c
            .is_none()
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
