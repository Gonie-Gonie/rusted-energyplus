//! CP339/CP341-to-CP342 retained-lineage validation.

use super::super::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentActiveOperands,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentRetainedInput,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
    cooling_positive_supply_capacity_limit_sensible_output_assignment_snapshot_is_exact_direct_release,
};

pub(super) fn supply_enthalpy_assignment_links_to_predecessors(
    assignment:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
    cp339: Option<
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    >,
) -> bool {
    let guard_false =
        predecessor.capacity_limit_sensible_output_guard_false_fallthrough;
    let assigned = predecessor
        .capacity_limit_sensible_output_maximum_capacity_assignment_executed;
    let inherited = assignment.system == predecessor.system
        && assignment.parent_call_ordinal == predecessor.parent_call_ordinal
        && assignment.controlled_zone == predecessor.controlled_zone
        && assignment.unit_body_entered == predecessor.unit_body_entered
        && assignment.predecessor_cooling_body_entered
            == predecessor.predecessor_cooling_body_entered
        && assignment.predecessor_no_outdoor_air_fallback_entered
            == predecessor.predecessor_no_outdoor_air_fallback_entered
        && assignment.predecessor_positive_supply_mass_flow_body_entered
            == predecessor.predecessor_positive_supply_mass_flow_body_entered
        && assignment.predecessor_active_guard_false_fallthrough
            == predecessor.predecessor_active_guard_false_fallthrough
        && assignment.predecessor_capacity_limit_guard_evaluated
            == predecessor.predecessor_capacity_limit_guard_evaluated
        && assignment.predecessor_capacity_limit_body_entered
            == predecessor.predecessor_capacity_limit_body_entered
        && assignment.predecessor_active_capacity_limit_guard_false_fallthrough
            == predecessor.predecessor_active_capacity_limit_guard_false_fallthrough
        && assignment.predecessor_capacity_limit_cp_air_assignment_executed
            == predecessor.predecessor_capacity_limit_cp_air_assignment_executed
        && assignment.predecessor_capacity_limit_sensible_output_assignment_executed
            == predecessor.predecessor_capacity_limit_sensible_output_assignment_executed
        && assignment.predecessor_capacity_limit_sensible_output_guard_evaluated
            == predecessor.predecessor_capacity_limit_sensible_output_guard_evaluated
        && assignment.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
            == predecessor.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
        && assignment.predecessor_capacity_limit_sensible_output_adjustment_body_entered
            == predecessor.predecessor_capacity_limit_sensible_output_adjustment_body_entered
        && assignment
            .predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed
            == assigned
        && assignment.unit_off_skipped == predecessor.unit_off_skipped
        && assignment.non_cooling_skipped == predecessor.non_cooling_skipped
        && assignment.positive_guard_false_fallthrough_skipped
            == predecessor.positive_guard_false_fallthrough_skipped
        && assignment.capacity_limit_guard_false_fallthrough_skipped
            == predecessor.capacity_limit_guard_false_fallthrough_skipped
        && assignment.capacity_limit_sensible_output_guard_false_fallthrough
            == guard_false
        && assignment
            .capacity_limit_sensible_output_supply_enthalpy_assignment_executed
            == assigned;
    if !inherited {
        return false;
    }

    let active = guard_false || assigned;
    if !active {
        return cp339.is_none()
            && assignment.preexisting_supply_enthalpy_j_per_kg.is_none()
            && assignment.resulting_supply_enthalpy_j_per_kg.is_none();
    }
    let Some(cp339) = cp339 else {
        return false;
    };
    let (Some(cp339_supply_enthalpy), Some(preexisting), Some(resulting)) = (
        cp339.supply_enthalpy_j_per_kg,
        assignment.preexisting_supply_enthalpy_j_per_kg,
        assignment.resulting_supply_enthalpy_j_per_kg,
    ) else {
        return false;
    };
    if cp339_supply_enthalpy.to_bits() != preexisting.to_bits() {
        return false;
    }
    if guard_false {
        return preexisting.to_bits() == resulting.to_bits();
    }

    let (
        Some(cp339_mixed_air),
        Some(cp339_flow),
        Some(predecessor_output),
        Some(mixed_air),
        Some(output),
        Some(flow),
        Some(quotient),
        Some(calculated),
        Some(assigned_value),
    ) = (
        cp339.mixed_air_enthalpy_j_per_kg,
        cp339.supply_mass_flow_rate_kg_per_s,
        predecessor.resulting_cooling_sensible_output_w,
        assignment.mixed_air_enthalpy_j_per_kg,
        assignment.cooling_sensible_output_w,
        assignment.supply_mass_flow_rate_kg_per_s,
        assignment.specific_cooling_output_j_per_kg,
        assignment.calculated_supply_enthalpy_j_per_kg,
        assignment.assigned_supply_enthalpy_j_per_kg,
    )
    else {
        return false;
    };
    cp339_mixed_air.to_bits() == mixed_air.to_bits()
        && cp339_flow.to_bits() == flow.to_bits()
        && predecessor_output.to_bits() == output.to_bits()
        && quotient.to_bits() == (output / flow).to_bits()
        && calculated.to_bits() == (mixed_air - quotient).to_bits()
        && calculated.to_bits() == assigned_value.to_bits()
        && assigned_value.to_bits() == resulting.to_bits()
}

pub(super) fn retained_cp339_lineage_is_exact(
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
    cp339: Option<
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    >,
    cp339_witness: Option<
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    >,
) -> bool {
    let active = predecessor
        .capacity_limit_sensible_output_guard_false_fallthrough
        || predecessor
            .capacity_limit_sensible_output_maximum_capacity_assignment_executed;
    if !active {
        return cp339.is_none() && cp339_witness.is_none();
    }
    let (Some(cp339), Some(cp339_witness)) = (cp339, cp339_witness) else {
        return false;
    };
    let same_call = cp339.system == predecessor.system
        && cp339.parent_call_ordinal == predecessor.parent_call_ordinal
        && cp339.controlled_zone == predecessor.controlled_zone;
    let (Some(flow), Some(mixed_air), Some(preexisting)) = (
        cp339.supply_mass_flow_rate_kg_per_s,
        cp339.mixed_air_enthalpy_j_per_kg,
        cp339.supply_enthalpy_j_per_kg,
    ) else {
        return false;
    };
    let active_output_is_reachable = if predecessor
        .capacity_limit_sensible_output_maximum_capacity_assignment_executed
    {
        predecessor
            .resulting_cooling_sensible_output_w
            .is_some_and(|value| value.is_finite() && value > 0.0)
    } else {
        true
    };
    same_call
        && cp339.capacity_limit_sensible_output_assignment_executed
        && cooling_positive_supply_capacity_limit_sensible_output_assignment_snapshot_is_exact_direct_release(
            cp339,
        )
        && sensible_output_assignment_snapshots_match_bit_exact(cp339, cp339_witness)
        && flow > 0.0
        && mixed_air.is_finite()
        && preexisting.is_finite()
        && active_output_is_reachable
}

pub(super) fn retained_input_from_prefix(
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
    cp339: Option<
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    >,
) -> Option<
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentRetainedInput,
> {
    let active = predecessor
        .capacity_limit_sensible_output_guard_false_fallthrough
        || predecessor
            .capacity_limit_sensible_output_maximum_capacity_assignment_executed;
    if !active {
        return None;
    }
    let cp339 = cp339?;
    let preexisting_supply_enthalpy_j_per_kg =
        cp339.supply_enthalpy_j_per_kg?;
    let active_operands = if predecessor
        .capacity_limit_sensible_output_maximum_capacity_assignment_executed
    {
        Some(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentActiveOperands {
                mixed_air_enthalpy_j_per_kg: cp339.mixed_air_enthalpy_j_per_kg?,
                cooling_sensible_output_w: predecessor.resulting_cooling_sensible_output_w?,
                supply_mass_flow_rate_kg_per_s: cp339.supply_mass_flow_rate_kg_per_s?,
            },
        )
    } else {
        None
    };
    Some(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentRetainedInput {
            preexisting_supply_enthalpy_j_per_kg,
            active_operands,
        },
    )
}

pub(super) fn maximum_capacity_assignment_snapshots_match_bit_exact(
    mut left:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
    mut right:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
) -> bool {
    let values_match = [
        option_bits_match(
            left.preexisting_cooling_sensible_output_w,
            right.preexisting_cooling_sensible_output_w,
        ),
        option_bits_match(
            left.maximum_total_cooling_capacity_w,
            right.maximum_total_cooling_capacity_w,
        ),
        option_bits_match(
            left.assigned_cooling_sensible_output_w,
            right.assigned_cooling_sensible_output_w,
        ),
        option_bits_match(
            left.resulting_cooling_sensible_output_w,
            right.resulting_cooling_sensible_output_w,
        ),
    ]
    .into_iter()
    .all(|matches| matches);
    for snapshot in [&mut left, &mut right] {
        snapshot.preexisting_cooling_sensible_output_w = None;
        snapshot.maximum_total_cooling_capacity_w = None;
        snapshot.assigned_cooling_sensible_output_w = None;
        snapshot.resulting_cooling_sensible_output_w = None;
    }
    values_match && left == right
}

fn sensible_output_assignment_snapshots_match_bit_exact(
    mut left:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    mut right:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
) -> bool {
    let values_match = [
        option_bits_match(
            left.supply_mass_flow_rate_kg_per_s,
            right.supply_mass_flow_rate_kg_per_s,
        ),
        option_bits_match(
            left.mixed_air_enthalpy_j_per_kg,
            right.mixed_air_enthalpy_j_per_kg,
        ),
        option_bits_match(
            left.supply_enthalpy_j_per_kg,
            right.supply_enthalpy_j_per_kg,
        ),
        option_bits_match(
            left.mixed_air_minus_supply_enthalpy_j_per_kg,
            right.mixed_air_minus_supply_enthalpy_j_per_kg,
        ),
        option_bits_match(
            left.calculated_cooling_sensible_output_w,
            right.calculated_cooling_sensible_output_w,
        ),
        option_bits_match(
            left.cooling_sensible_output_w,
            right.cooling_sensible_output_w,
        ),
    ]
    .into_iter()
    .all(|matches| matches);
    for snapshot in [&mut left, &mut right] {
        snapshot.supply_mass_flow_rate_kg_per_s = None;
        snapshot.mixed_air_enthalpy_j_per_kg = None;
        snapshot.supply_enthalpy_j_per_kg = None;
        snapshot.mixed_air_minus_supply_enthalpy_j_per_kg = None;
        snapshot.calculated_cooling_sensible_output_w = None;
        snapshot.cooling_sensible_output_w = None;
    }
    values_match && left == right
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
