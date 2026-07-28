//! CP329/CP330/CP336/CP338-to-CP339 lineage validation.

use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    cooling_mixed_air_call_snapshots_match_bit_exact,
};

pub(super) fn sensible_output_assignment_links_to_cp_air_assignment(
    assignment:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
) -> bool {
    assignment.system == predecessor.system
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
            == predecessor.capacity_limit_cp_air_assignment_executed
        && assignment.unit_off_skipped == predecessor.unit_off_skipped
        && assignment.non_cooling_skipped == predecessor.non_cooling_skipped
        && assignment.positive_guard_false_fallthrough_skipped
            == predecessor.positive_guard_false_fallthrough_skipped
        && assignment.capacity_limit_guard_false_fallthrough_skipped
            == predecessor.capacity_limit_guard_false_fallthrough_skipped
        && assignment.capacity_limit_sensible_output_assignment_executed
            == predecessor.capacity_limit_cp_air_assignment_executed
}

#[allow(clippy::too_many_arguments)]
pub(super) fn active_operands_link_to_retained_prefix(
    predecessor: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
    positive_guard: PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    positive_guard_witness: PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    mixed_air: PurchasedAirCalcCoolingMixedAirCallSnapshot,
    mixed_air_witness: PurchasedAirCalcCoolingMixedAirCallSnapshot,
    supply_enthalpy: PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    supply_enthalpy_witness: PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    supply_mass_flow_rate_kg_per_s: Option<f64>,
    mixed_air_enthalpy_j_per_kg: Option<f64>,
    supply_enthalpy_j_per_kg: Option<f64>,
) -> bool {
    let (
        Some(supply_mass_flow_rate_kg_per_s),
        Some(mixed_air_enthalpy_j_per_kg),
        Some(supply_enthalpy_j_per_kg),
    ) = (
        supply_mass_flow_rate_kg_per_s,
        mixed_air_enthalpy_j_per_kg,
        supply_enthalpy_j_per_kg,
    )
    else {
        return false;
    };
    let same_call = [
        (
            positive_guard.system,
            positive_guard.parent_call_ordinal,
            positive_guard.controlled_zone,
        ),
        (
            mixed_air.system,
            mixed_air.parent_call_ordinal,
            mixed_air.controlled_zone,
        ),
        (
            supply_enthalpy.system,
            supply_enthalpy.parent_call_ordinal,
            supply_enthalpy.controlled_zone,
        ),
    ]
    .into_iter()
    .all(|(system, ordinal, zone)| {
        system == predecessor.system
            && ordinal == predecessor.parent_call_ordinal
            && zone == predecessor.controlled_zone
    });
    let positive_guard_operand_matches = positive_guard
        .supply_mass_flow_rate_kg_per_s
        .is_some_and(|value| value.to_bits() == supply_mass_flow_rate_kg_per_s.to_bits());
    let mixed_air_flow_matches = mixed_air
        .supply_mass_flow_rate_kg_per_s
        .is_some_and(|value| value.to_bits() == supply_mass_flow_rate_kg_per_s.to_bits())
        && mixed_air
            .child_supply_mass_flow_rate_kg_per_s
            .is_some_and(|value| value.to_bits() == supply_mass_flow_rate_kg_per_s.to_bits());
    let mixed_air_enthalpy_matches = mixed_air
        .mixed_air_enthalpy_projection_j_per_kg
        .is_some_and(|value| value.to_bits() == mixed_air_enthalpy_j_per_kg.to_bits());
    let supply_enthalpy_matches = supply_enthalpy
        .supply_enthalpy_j_per_kg
        .is_some_and(|value| value.to_bits() == supply_enthalpy_j_per_kg.to_bits());

    predecessor.capacity_limit_cp_air_assignment_executed
        && same_call
        && positive_guard.supply_mass_flow_rate_read
        && positive_guard.supply_mass_flow_rate_strictly_positive_comparison_evaluated
        && positive_guard.supply_mass_flow_rate_strictly_positive == Some(true)
        && positive_guard.positive_supply_mass_flow_body_entered
        && !positive_guard.active_guard_false_fallthrough
        && positive_guard_snapshots_match_bit_exact(positive_guard, positive_guard_witness)
        && positive_guard_operand_matches
        && mixed_air_flow_matches
        && supply_mass_flow_rate_kg_per_s > 0.0
        && mixed_air.cooling_call_executed
        && mixed_air.no_outdoor_air_fallback_entered
        && mixed_air.mixed_air_enthalpy_projection_assigned
        && cooling_mixed_air_call_snapshots_match_bit_exact(mixed_air, mixed_air_witness)
        && mixed_air_enthalpy_matches
        && mixed_air_enthalpy_j_per_kg.is_finite()
        && supply_enthalpy.supply_enthalpy_assignment_executed
        && supply_enthalpy.supply_enthalpy_assigned
        && supply_enthalpy_snapshots_match_bit_exact(supply_enthalpy, supply_enthalpy_witness)
        && supply_enthalpy_matches
        && supply_enthalpy_j_per_kg.is_finite()
}

pub(super) fn cp_air_assignment_snapshots_match_bit_exact(
    mut left: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
    mut right: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
) -> bool {
    let values_match =
        option_bits_match(left.mixed_air_humidity_ratio, right.mixed_air_humidity_ratio)
            && option_bits_match(
                left.psychrometric_cp_air_result_j_per_kg_k,
                right.psychrometric_cp_air_result_j_per_kg_k,
            )
            && option_bits_match(left.cp_air_j_per_kg_k, right.cp_air_j_per_kg_k);
    for snapshot in [&mut left, &mut right] {
        snapshot.mixed_air_humidity_ratio = None;
        snapshot.psychrometric_cp_air_result_j_per_kg_k = None;
        snapshot.cp_air_j_per_kg_k = None;
    }
    values_match && left == right
}

fn positive_guard_snapshots_match_bit_exact(
    mut left: PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    mut right: PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
) -> bool {
    let values_match = option_bits_match(
        left.supply_mass_flow_rate_kg_per_s,
        right.supply_mass_flow_rate_kg_per_s,
    );
    left.supply_mass_flow_rate_kg_per_s = None;
    right.supply_mass_flow_rate_kg_per_s = None;
    values_match && left == right
}

fn supply_enthalpy_snapshots_match_bit_exact(
    mut left: PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    mut right: PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
) -> bool {
    let values_match = option_bits_match(left.supply_temperature_c, right.supply_temperature_c)
        && option_bits_match(left.supply_humidity_ratio, right.supply_humidity_ratio)
        && option_bits_match(
            left.psychrometric_supply_enthalpy_result_j_per_kg,
            right.psychrometric_supply_enthalpy_result_j_per_kg,
        )
        && option_bits_match(
            left.supply_enthalpy_j_per_kg,
            right.supply_enthalpy_j_per_kg,
        );
    for snapshot in [&mut left, &mut right] {
        snapshot.supply_temperature_c = None;
        snapshot.supply_humidity_ratio = None;
        snapshot.psychrometric_supply_enthalpy_result_j_per_kg = None;
        snapshot.supply_enthalpy_j_per_kg = None;
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
