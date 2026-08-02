//! Exact CP392 snapshot, route, and binary64 validation.

mod predecessor;

use super::super::transition::routes::{
    RetainedRoute, predecessor_has_supply_enthalpy, predecessor_has_supply_temperature,
    predecessor_route,
};
use super::super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioAssignmentSnapshot as Snapshot,
};
use crate::psychrometrics::energyplus_psy_w_fn_tdb_h;
use predecessor::predecessor_snapshot;

pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_snapshot_is_exact_direct_release(
    snapshot: Snapshot,
) -> bool {
    snapshot_route(snapshot).is_some_and(|route| {
        !route.active
            && crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_snapshot_is_exact_direct_release(
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
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER
    {
        return None;
    }
    let predecessor = predecessor_snapshot(snapshot);
    let route = predecessor_route(predecessor)?;
    let active_flags = [
        snapshot
            .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_executed,
        snapshot.cp391_retained_supply_temperature_owned_read,
        snapshot.supply_temperature_for_humidity_ratio_inversion_read,
        snapshot.cp391_retained_supply_enthalpy_owned_read,
        snapshot.supply_enthalpy_for_humidity_ratio_inversion_read,
        snapshot.psychrometric_supply_humidity_ratio_evaluated,
        snapshot.supply_humidity_ratio_assignment_performed,
    ];
    let has_temperature = predecessor_has_supply_temperature(route.predecessor_index);
    let has_enthalpy = predecessor_has_supply_enthalpy(route.predecessor_index);
    if active_flags.into_iter().any(|flag| flag != route.active)
        || snapshot.cp391_retained_supply_temperature_state_owned != has_temperature
        || snapshot.cp391_retained_supply_enthalpy_state_owned != has_enthalpy
        || !option_bits_match(
            snapshot.resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
        || !option_bits_match(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
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
        if temperature.to_bits() != predecessor.resulting_supply_temperature_c?.to_bits()
            || enthalpy.to_bits() != predecessor.resulting_supply_enthalpy_j_per_kg?.to_bits()
            || psychrometric.to_bits() != expected.to_bits()
            || assigned.to_bits() != psychrometric.to_bits()
            || resulting.to_bits() != assigned.to_bits()
        {
            return None;
        }
    } else if [
        snapshot.supply_temperature_c,
        snapshot.supply_enthalpy_j_per_kg,
        snapshot.psychrometric_supply_humidity_ratio,
        snapshot.assigned_supply_humidity_ratio,
        snapshot.resulting_supply_humidity_ratio,
    ]
    .into_iter()
    .any(|value| value.is_some())
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
    let values_match = compare_clear!(predecessor_mixed_air_humidity_ratio)
        && compare_clear!(predecessor_psychrometric_cp_air_result_j_per_kg_k)
        && compare_clear!(predecessor_cp_air_j_per_kg_k)
        && compare_clear!(predecessor_cooling_total_output_w)
        && compare_clear!(predecessor_cooling_sensible_heat_ratio)
        && compare_clear!(predecessor_calculated_cooling_sensible_output_w)
        && compare_clear!(predecessor_cooling_sensible_output_w)
        && compare_clear!(predecessor_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_preexisting_supply_temperature_c)
        && compare_clear!(predecessor_mixed_air_temperature_c)
        && compare_clear!(predecessor_cp389_cooling_sensible_output_w)
        && compare_clear!(predecessor_cp389_cp_air_j_per_kg_k)
        && compare_clear!(predecessor_supply_mass_flow_rate_kg_per_s)
        && compare_clear!(predecessor_cp_air_times_supply_mass_flow_rate_w_per_k)
        && compare_clear!(predecessor_cooling_sensible_output_over_air_capacity_rate_k)
        && compare_clear!(predecessor_calculated_supply_temperature_c)
        && compare_clear!(predecessor_assigned_supply_temperature_c)
        && compare_clear!(predecessor_resulting_supply_temperature_c)
        && compare_clear!(predecessor_cp390_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(preexisting_supply_temperature_c)
        && compare_clear!(supply_temperature_before_mixed_air_limit_c)
        && compare_clear!(mixed_air_temperature_c)
        && compare_clear!(minimum_supply_temperature_c)
        && compare_clear!(assigned_supply_temperature_c)
        && compare_clear!(predecessor_cp390_resulting_supply_temperature_c)
        && compare_clear!(preexisting_supply_enthalpy_j_per_kg)
        && compare_clear!(supply_enthalpy_before_overdrying_limit_j_per_kg)
        && compare_clear!(predecessor_cp391_supply_temperature_c)
        && compare_clear!(psychrometric_minimum_supply_enthalpy_j_per_kg)
        && compare_clear!(maximum_supply_enthalpy_j_per_kg)
        && compare_clear!(assigned_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp391_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp391_resulting_supply_temperature_c)
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
