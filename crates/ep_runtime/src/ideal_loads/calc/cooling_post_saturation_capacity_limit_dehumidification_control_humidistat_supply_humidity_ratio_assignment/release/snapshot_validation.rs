//! Exact CP395 snapshot, route, ownership, and binary64 validation.

use super::super::transition::routes::{
    RetainedRoute, compressed_snapshot_route, predecessor_has_supply_enthalpy,
    predecessor_has_supply_humidity_ratio, predecessor_has_supply_temperature,
    predecessor_snapshot, resulting_has_supply_humidity_ratio,
};
use super::super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatSupplyHumidityRatioAssignmentSnapshot as Snapshot,
};
use crate::psychrometrics::energyplus_psy_w_fn_tdb_h;

pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_snapshot_is_exact_direct_release(
    snapshot: Snapshot,
) -> bool {
    snapshot_route(snapshot).is_some_and(|route| {
        !route.active
            && crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry_snapshot_is_exact_direct_release(
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
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER
    {
        return None;
    }
    let route = compressed_snapshot_route(snapshot)?;
    let index = route.predecessor_index;
    let active_flags = [
        snapshot.dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed,
        snapshot.cp394_retained_supply_temperature_owned_read,
        snapshot.supply_temperature_for_humidity_ratio_inversion_read,
        snapshot.cp394_retained_supply_enthalpy_owned_read,
        snapshot.supply_enthalpy_for_humidity_ratio_inversion_read,
        snapshot.psychrometric_supply_humidity_ratio_evaluated,
        snapshot.supply_humidity_ratio_assignment_performed,
    ];
    if active_flags.into_iter().any(|flag| flag != route.active)
        || snapshot.cp394_retained_supply_humidity_ratio_state_owned
            != predecessor_has_supply_humidity_ratio(index)
        || snapshot.cp394_retained_supply_temperature_state_owned
            != predecessor_has_supply_temperature(index)
        || snapshot.cp394_retained_supply_enthalpy_state_owned
            != predecessor_has_supply_enthalpy(index)
        || snapshot.resulting_supply_humidity_ratio.is_some()
            != resulting_has_supply_humidity_ratio(index)
        || !option_bits_match(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            snapshot.predecessor_cp394_resulting_supply_enthalpy_j_per_kg,
        )
        || !option_bits_match(
            snapshot.resulting_supply_temperature_c,
            snapshot.predecessor_cp394_resulting_supply_temperature_c,
        )
    {
        return None;
    }
    if route.active {
        let (
            Some(temperature),
            Some(enthalpy),
            Some(psychrometric),
            Some(assigned),
            Some(resulting),
        ) = (
            snapshot.supply_temperature_c,
            snapshot.supply_enthalpy_j_per_kg,
            snapshot.psychrometric_supply_humidity_ratio,
            snapshot.assigned_supply_humidity_ratio,
            snapshot.resulting_supply_humidity_ratio,
        )
        else {
            return None;
        };
        let expected = energyplus_psy_w_fn_tdb_h(temperature, enthalpy);
        if temperature.to_bits()
            != snapshot
                .predecessor_cp394_resulting_supply_temperature_c?
                .to_bits()
            || enthalpy.to_bits()
                != snapshot
                    .predecessor_cp394_resulting_supply_enthalpy_j_per_kg?
                    .to_bits()
            || psychrometric.to_bits() != expected.to_bits()
            || assigned.to_bits() != psychrometric.to_bits()
            || resulting.to_bits() != assigned.to_bits()
        {
            return None;
        }
    } else {
        if [
            snapshot.supply_temperature_c,
            snapshot.supply_enthalpy_j_per_kg,
            snapshot.psychrometric_supply_humidity_ratio,
            snapshot.assigned_supply_humidity_ratio,
        ]
        .into_iter()
        .any(|value| value.is_some())
        {
            return None;
        }
        if !option_bits_match(
            snapshot.resulting_supply_humidity_ratio,
            snapshot.predecessor_cp394_resulting_supply_humidity_ratio,
        ) {
            return None;
        }
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
    let values_match = compare_clear!(predecessor_cp393_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp393_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp393_resulting_supply_temperature_c)
        && compare_clear!(predecessor_cp394_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp394_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp394_resulting_supply_temperature_c)
        && compare_clear!(supply_temperature_c)
        && compare_clear!(supply_enthalpy_j_per_kg)
        && compare_clear!(psychrometric_supply_humidity_ratio)
        && compare_clear!(assigned_supply_humidity_ratio)
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
