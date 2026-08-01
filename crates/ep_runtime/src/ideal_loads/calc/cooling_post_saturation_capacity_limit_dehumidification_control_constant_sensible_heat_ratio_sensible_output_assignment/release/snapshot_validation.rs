//! Exact CP388 snapshot, route, owner, and IEEE-bit validation.

use super::super::transition::routes::{RetainedRoute, predecessor_route};
use super::super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot as Predecessor,
};

pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_snapshot_is_exact_direct_release(
    snapshot: Snapshot,
) -> bool {
    snapshot_route(snapshot).is_some_and(|route| {
        !route.active
            && crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_snapshot_is_exact_direct_release(
                predecessor_snapshot(snapshot),
            )
    })
}

#[allow(dead_code)] // Successor checkpoints consume the general exact-shape validator.
pub(in crate::ideal_loads::calc) fn snapshot_is_exact(snapshot: Snapshot) -> bool {
    snapshot_route(snapshot).is_some()
}

pub(super) fn snapshot_route(snapshot: Snapshot) -> Option<RetainedRoute> {
    if snapshot.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER
    {
        return None;
    }
    let predecessor = predecessor_snapshot(snapshot);
    let route = predecessor_route(predecessor)?;
    let local_flags = [
        snapshot.dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed,
        snapshot.cp384_retained_cooling_total_output_owned_read,
        snapshot.cp385_cooling_total_output_bit_corroborated,
        snapshot.cooling_total_output_read,
        snapshot.cooling_sensible_heat_ratio_read,
        snapshot.cooling_sensible_output_calculated,
        snapshot.cooling_sensible_output_assigned,
    ];
    if local_flags.into_iter().any(|flag| flag != route.active)
        || !option_bits_match(
            snapshot.predecessor_resulting_supply_enthalpy_j_per_kg,
            snapshot.resulting_supply_enthalpy_j_per_kg,
        )
    {
        return None;
    }
    if route.active {
        let (Some(total), Some(ratio), Some(calculated), Some(assigned)) = (
            snapshot.cooling_total_output_w,
            snapshot.cooling_sensible_heat_ratio,
            snapshot.calculated_cooling_sensible_output_w,
            snapshot.cooling_sensible_output_w,
        ) else {
            return None;
        };
        if calculated.to_bits() != (total * ratio).to_bits()
            || assigned.to_bits() != calculated.to_bits()
        {
            return None;
        }
    } else if snapshot.cooling_total_output_w.is_some()
        || snapshot.cooling_sensible_heat_ratio.is_some()
        || snapshot.calculated_cooling_sensible_output_w.is_some()
        || snapshot.cooling_sensible_output_w.is_some()
    {
        return None;
    }
    Some(route)
}

pub(super) fn predecessor_snapshot(snapshot: Snapshot) -> Predecessor {
    Predecessor {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
        system: snapshot.system,
        parent_call_ordinal: snapshot.parent_call_ordinal,
        controlled_zone: snapshot.controlled_zone,
        unit_off_skipped: snapshot.unit_off_skipped,
        non_cooling_skipped: snapshot.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: snapshot.positive_guard_false_fallthrough_skipped,
        heating_availability_guard_false_fallthrough: snapshot.heating_availability_guard_false_fallthrough,
        humidification_control_guard_false_fallthrough: snapshot.humidification_control_guard_false_fallthrough,
        dehumidification_control_humidistat_maximum_assignment_executed: snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        dehumidification_control_none_maximum_assignment_executed: snapshot.dehumidification_control_none_maximum_assignment_executed,
        dehumidification_control_guard_false_fallthrough: snapshot.dehumidification_control_guard_false_fallthrough,
        predecessor_capacity_limit_guard_evaluated: snapshot.predecessor_capacity_limit_guard_evaluated,
        predecessor_capacity_limit_body_entered: snapshot.predecessor_capacity_limit_body_entered,
        predecessor_active_capacity_limit_guard_false_fallthrough: snapshot.predecessor_active_capacity_limit_guard_false_fallthrough,
        predecessor_dehumidification_guard_evaluated: snapshot.predecessor_dehumidification_guard_evaluated,
        predecessor_dehumidification_body_entered: snapshot.predecessor_dehumidification_body_entered,
        predecessor_dehumidification_guard_false_fallthrough: snapshot.predecessor_dehumidification_guard_false_fallthrough,
        predecessor_dehumidification_total_output_assignment_executed: snapshot.predecessor_dehumidification_total_output_assignment_executed,
        predecessor_dehumidification_total_output_capacity_guard_evaluated: snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated,
        predecessor_dehumidification_total_output_capacity_adjustment_body_entered: snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        predecessor_dehumidification_total_output_capacity_guard_false_fallthrough: snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_capacity_guard_false_fallthrough: snapshot.dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_maximum_capacity_assignment_executed: snapshot.dehumidification_total_output_maximum_capacity_assignment_executed,
        predecessor_supply_enthalpy_assignment_executed: snapshot.predecessor_supply_enthalpy_assignment_executed,
        predecessor_dehumidification_control_type_read: snapshot.predecessor_dehumidification_control_type_read,
        predecessor_dehumidification_control_type: snapshot.predecessor_dehumidification_control_type,
        predecessor_dehumidification_control_switch_dispatched: snapshot.predecessor_dehumidification_control_switch_dispatched,
        predecessor_resulting_supply_enthalpy_j_per_kg: snapshot.predecessor_resulting_supply_enthalpy_j_per_kg,
        dehumidification_control_constant_sensible_heat_ratio_case_entered: snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered,
        dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed: snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed,
        mixed_air_humidity_ratio_read: snapshot.predecessor_mixed_air_humidity_ratio_read,
        mixed_air_humidity_ratio: snapshot.predecessor_mixed_air_humidity_ratio,
        psychrometric_cp_air_evaluated: snapshot.predecessor_psychrometric_cp_air_evaluated,
        psychrometric_cp_air_result_j_per_kg_k: snapshot.predecessor_psychrometric_cp_air_result_j_per_kg_k,
        cp_air_assigned: snapshot.predecessor_cp_air_assigned,
        cp_air_j_per_kg_k: snapshot.predecessor_cp_air_j_per_kg_k,
        resulting_supply_enthalpy_j_per_kg: snapshot.predecessor_resulting_supply_enthalpy_j_per_kg,
    }
}

pub(in crate::ideal_loads::calc) fn snapshots_match_bit_exact(
    mut left: Snapshot,
    mut right: Snapshot,
) -> bool {
    let values_match = [
        (left.predecessor_resulting_supply_enthalpy_j_per_kg, right.predecessor_resulting_supply_enthalpy_j_per_kg),
        (left.predecessor_mixed_air_humidity_ratio, right.predecessor_mixed_air_humidity_ratio),
        (left.predecessor_psychrometric_cp_air_result_j_per_kg_k, right.predecessor_psychrometric_cp_air_result_j_per_kg_k),
        (left.predecessor_cp_air_j_per_kg_k, right.predecessor_cp_air_j_per_kg_k),
        (left.cooling_total_output_w, right.cooling_total_output_w),
        (left.cooling_sensible_heat_ratio, right.cooling_sensible_heat_ratio),
        (left.calculated_cooling_sensible_output_w, right.calculated_cooling_sensible_output_w),
        (left.cooling_sensible_output_w, right.cooling_sensible_output_w),
        (left.resulting_supply_enthalpy_j_per_kg, right.resulting_supply_enthalpy_j_per_kg),
    ]
    .into_iter()
    .all(|(left, right)| option_bits_match(left, right));
    for snapshot in [&mut left, &mut right] {
        snapshot.predecessor_resulting_supply_enthalpy_j_per_kg = None;
        snapshot.predecessor_mixed_air_humidity_ratio = None;
        snapshot.predecessor_psychrometric_cp_air_result_j_per_kg_k = None;
        snapshot.predecessor_cp_air_j_per_kg_k = None;
        snapshot.cooling_total_output_w = None;
        snapshot.cooling_sensible_heat_ratio = None;
        snapshot.calculated_cooling_sensible_output_w = None;
        snapshot.cooling_sensible_output_w = None;
        snapshot.resulting_supply_enthalpy_j_per_kg = None;
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
