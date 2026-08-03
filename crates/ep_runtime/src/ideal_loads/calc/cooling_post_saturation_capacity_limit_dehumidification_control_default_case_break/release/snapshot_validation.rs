//! Exact CP410 compressed snapshot, route, and binary64 validation.

use super::super::transition::routes::{
    RetainedRoute, compressed_snapshot_route, cp409_shape, predecessor_index_is_public,
};
use super::super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_DEFAULT_CASE_BREAK_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_DEFAULT_CASE_BREAK_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_DEFAULT_CASE_BREAK_SOURCE_ORDER as ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlDefaultCaseBreakSnapshot as Snapshot,
};

pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break_snapshot_is_exact_direct_release(
    snapshot: Snapshot,
) -> bool {
    snapshot_route(snapshot)
        .is_some_and(|route| predecessor_index_is_public(route.predecessor_index))
}

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn snapshot_is_exact(snapshot: Snapshot) -> bool {
    snapshot_route(snapshot).is_some()
}

pub(in crate::ideal_loads::calc) fn snapshot_route(snapshot: Snapshot) -> Option<RetainedRoute> {
    if snapshot.source != SOURCE
        || snapshot.first_excluded_source != EXCLUDED
        || snapshot.source_order != ORDER
    {
        return None;
    }
    let route = compressed_snapshot_route(snapshot)?;
    let predecessor = cp409_shape(snapshot);
    if !carrier_is_exact(
        predecessor.resulting_supply_humidity_ratio,
        snapshot.resulting_supply_humidity_ratio,
    ) || !carrier_is_exact(
        predecessor.resulting_supply_enthalpy_j_per_kg,
        snapshot.resulting_supply_enthalpy_j_per_kg,
    ) || !carrier_is_exact(
        predecessor.resulting_supply_temperature_c,
        snapshot.resulting_supply_temperature_c,
    ) {
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
    let values_match = compare_clear!(predecessor_cp409_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp409_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp409_resulting_supply_temperature_c)
        && compare_clear!(resulting_supply_humidity_ratio)
        && compare_clear!(resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(resulting_supply_temperature_c);
    values_match && left == right
}

fn carrier_is_exact(predecessor: Option<f64>, resulting: Option<f64>) -> bool {
    option_bits_match(predecessor, resulting)
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
