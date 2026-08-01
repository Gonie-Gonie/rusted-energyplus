//! Exact CP387 snapshot, route, and IEEE-bit validation.

use super::super::transition::routes::{RetainedRoute, predecessor_route};
use super::super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_LEXICAL_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchSnapshot as Predecessor,
};
use crate::psychrometrics::energyplus_psy_cp_air_fn_w;

pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_snapshot_is_exact_direct_release(
    snapshot: Snapshot,
) -> bool {
    snapshot_route(snapshot).is_some_and(|route| {
        !route.active
            && crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_control_switch_snapshot_is_exact_direct_release(
                predecessor_snapshot(snapshot),
            )
    })
}

pub(in crate::ideal_loads::calc) fn snapshot_route(snapshot: Snapshot) -> Option<RetainedRoute> {
    if snapshot.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE_ORDER
    {
        return None;
    }
    let predecessor = predecessor_snapshot(snapshot);
    let route = predecessor_route(predecessor)?;
    let local_flags = [
        snapshot.dehumidification_control_constant_sensible_heat_ratio_case_entered,
        snapshot.dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed,
        snapshot.mixed_air_humidity_ratio_read,
        snapshot.psychrometric_cp_air_evaluated,
        snapshot.cp_air_assigned,
    ];
    if local_flags.into_iter().any(|flag| flag != route.active) {
        return None;
    }
    if !option_bits_match(
        snapshot.predecessor_resulting_supply_enthalpy_j_per_kg,
        snapshot.resulting_supply_enthalpy_j_per_kg,
    ) {
        return None;
    }
    if route.active {
        let (Some(humidity), Some(evaluated), Some(assigned)) = (
            snapshot.mixed_air_humidity_ratio,
            snapshot.psychrometric_cp_air_result_j_per_kg_k,
            snapshot.cp_air_j_per_kg_k,
        ) else {
            return None;
        };
        if !humidity.is_finite() || humidity < 0.0 {
            return None;
        }
        let expected = energyplus_psy_cp_air_fn_w(humidity);
        if !expected.is_finite()
            || evaluated.to_bits() != expected.to_bits()
            || assigned.to_bits() != evaluated.to_bits()
        {
            return None;
        }
    } else if snapshot.mixed_air_humidity_ratio.is_some()
        || snapshot.psychrometric_cp_air_result_j_per_kg_k.is_some()
        || snapshot.cp_air_j_per_kg_k.is_some()
    {
        return None;
    }
    Some(route)
}

pub(super) fn predecessor_snapshot(snapshot: Snapshot) -> Predecessor {
    Predecessor {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE,
        first_excluded_lexical_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_LEXICAL_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE_ORDER,
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
        predecessor_resulting_supply_enthalpy_j_per_kg: snapshot.predecessor_resulting_supply_enthalpy_j_per_kg,
        dehumidification_control_type_read: snapshot.predecessor_dehumidification_control_type_read,
        dehumidification_control_type: snapshot.predecessor_dehumidification_control_type,
        dehumidification_control_switch_dispatched: snapshot.predecessor_dehumidification_control_switch_dispatched,
        resulting_supply_enthalpy_j_per_kg: snapshot.predecessor_resulting_supply_enthalpy_j_per_kg,
    }
}

pub(in crate::ideal_loads::calc) fn snapshots_match_bit_exact(
    mut left: Snapshot,
    mut right: Snapshot,
) -> bool {
    let predecessor_enthalpy_matches = option_bits_match(
        left.predecessor_resulting_supply_enthalpy_j_per_kg,
        right.predecessor_resulting_supply_enthalpy_j_per_kg,
    );
    let humidity_matches = option_bits_match(
        left.mixed_air_humidity_ratio,
        right.mixed_air_humidity_ratio,
    );
    let evaluated_matches = option_bits_match(
        left.psychrometric_cp_air_result_j_per_kg_k,
        right.psychrometric_cp_air_result_j_per_kg_k,
    );
    let assigned_matches = option_bits_match(left.cp_air_j_per_kg_k, right.cp_air_j_per_kg_k);
    let resulting_enthalpy_matches = option_bits_match(
        left.resulting_supply_enthalpy_j_per_kg,
        right.resulting_supply_enthalpy_j_per_kg,
    );
    for snapshot in [&mut left, &mut right] {
        snapshot.predecessor_resulting_supply_enthalpy_j_per_kg = None;
        snapshot.mixed_air_humidity_ratio = None;
        snapshot.psychrometric_cp_air_result_j_per_kg_k = None;
        snapshot.cp_air_j_per_kg_k = None;
        snapshot.resulting_supply_enthalpy_j_per_kg = None;
    }
    predecessor_enthalpy_matches
        && humidity_matches
        && evaluated_matches
        && assigned_matches
        && resulting_enthalpy_matches
        && left == right
}

pub(in crate::ideal_loads::calc) fn snapshot_is_exact(snapshot: Snapshot) -> bool {
    snapshot_route(snapshot).is_some()
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
