//! Exact CP399 snapshot, route, and binary64 validation.

use super::super::transition::routes::{
    RetainedRoute, compressed_snapshot_route, predecessor_has_supply_enthalpy,
    predecessor_has_supply_humidity_ratio, predecessor_has_supply_temperature,
    predecessor_snapshot,
};
use super::super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CP_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentSnapshot as Snapshot,
};
use crate::psychrometrics::energyplus_psy_cp_air_fn_w;

pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_snapshot_is_exact_direct_release(
    snapshot: Snapshot,
) -> bool {
    snapshot_route(snapshot).is_some_and(|route| {
        matches!(route.predecessor_index, 0..=8 | 20 | 24)
            && route.active == matches!(route.predecessor_index, 20 | 24)
            && crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry_snapshot_is_exact_direct_release(
                predecessor_snapshot(snapshot),
            )
    })
}

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn snapshot_is_exact(snapshot: Snapshot) -> bool {
    snapshot_route(snapshot).is_some()
}

pub(in crate::ideal_loads::calc) fn snapshot_route(snapshot: Snapshot) -> Option<RetainedRoute> {
    if snapshot.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CP_AIR_ASSIGNMENT_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CP_AIR_ASSIGNMENT_SOURCE_ORDER
    {
        return None;
    }
    let route = compressed_snapshot_route(snapshot)?;
    let index = route.predecessor_index;
    let local_flags = [
        snapshot.dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed,
        snapshot.mixed_air_humidity_ratio_read,
        snapshot.psychrometric_cp_air_evaluated,
        snapshot.cp_air_assigned,
    ];
    if local_flags.into_iter().any(|flag| flag != route.active)
        || !carrier_is_exact(
            snapshot.predecessor_cp398_resulting_supply_humidity_ratio,
            snapshot.resulting_supply_humidity_ratio,
            predecessor_has_supply_humidity_ratio(index),
        )
        || !carrier_is_exact(
            snapshot.predecessor_cp398_resulting_supply_enthalpy_j_per_kg,
            snapshot.resulting_supply_enthalpy_j_per_kg,
            predecessor_has_supply_enthalpy(index),
        )
        || !carrier_is_exact(
            snapshot.predecessor_cp398_resulting_supply_temperature_c,
            snapshot.resulting_supply_temperature_c,
            predecessor_has_supply_temperature(index),
        )
    {
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
        let expected = energyplus_psy_cp_air_fn_w(humidity);
        if !humidity.is_finite()
            || humidity < 0.0
            || !expected.is_finite()
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

pub(in crate::ideal_loads::calc) fn snapshots_match_bit_exact(
    mut left: Snapshot,
    mut right: Snapshot,
) -> bool {
    macro_rules! compare_clear {
        ($field:ident) => {{
            let matches = option_bits_match(left.$field, right.$field);
            left.$field = None;
            right.$field = None;
            matches
        }};
    }
    let values_match = compare_clear!(predecessor_cp397_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp397_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp397_resulting_supply_temperature_c)
        && compare_clear!(predecessor_cp398_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp398_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp398_resulting_supply_temperature_c)
        && compare_clear!(mixed_air_humidity_ratio)
        && compare_clear!(psychrometric_cp_air_result_j_per_kg_k)
        && compare_clear!(cp_air_j_per_kg_k)
        && compare_clear!(resulting_supply_humidity_ratio)
        && compare_clear!(resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(resulting_supply_temperature_c);
    values_match && left == right
}

fn carrier_is_exact(predecessor: Option<f64>, resulting: Option<f64>, present: bool) -> bool {
    present == predecessor.is_some()
        && present == resulting.is_some()
        && option_bits_match(predecessor, resulting)
}

pub(super) fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
