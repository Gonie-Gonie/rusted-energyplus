//! Exact CP412 lossless snapshot, route, and binary64 validation.

use super::super::transition::routes::{
    compressed_snapshot_route, cp411_shape, predecessor_index_is_public, route_is_active,
    RetainedRoute,
};
use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentSnapshot as Snapshot,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE_ORDER as ORDER,
};

pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release(
    snapshot: Snapshot,
) -> bool {
    snapshot_route(snapshot).is_some_and(|route| {
        predecessor_index_is_public(route.predecessor_index)
            && crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_snapshot_is_exact_direct_release(cp411_shape(snapshot))
            && (!route_is_active(route)
                || (snapshot
                    .supply_temperature_for_saturation_humidity_ratio_c
                    .is_some_and(f64::is_finite)
                    && snapshot
                        .outdoor_barometric_pressure_pa
                        .is_some_and(|pressure| pressure.is_finite() && pressure > 0.0)
                    && snapshot
                        .saturation_supply_humidity_ratio
                        .is_some_and(f64::is_finite)))
    })
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
    compressed_snapshot_route(snapshot)
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
        && compare_clear!(predecessor_cp410_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp410_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp410_resulting_supply_temperature_c)
        && compare_clear!(purchased_air_supply_humidity_ratio_before_saturation_check)
        && compare_clear!(assigned_supply_humidity_ratio_original)
        && compare_clear!(resulting_supply_humidity_ratio_original)
        && compare_clear!(predecessor_cp411_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp411_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp411_resulting_supply_temperature_c)
        && compare_clear!(supply_temperature_for_saturation_humidity_ratio_c)
        && compare_clear!(outdoor_barometric_pressure_pa)
        && compare_clear!(saturation_supply_humidity_ratio)
        && compare_clear!(assigned_saturation_supply_humidity_ratio)
        && compare_clear!(resulting_saturation_supply_humidity_ratio)
        && compare_clear!(resulting_supply_humidity_ratio)
        && compare_clear!(resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(resulting_supply_temperature_c);
    values_match && left == right
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
