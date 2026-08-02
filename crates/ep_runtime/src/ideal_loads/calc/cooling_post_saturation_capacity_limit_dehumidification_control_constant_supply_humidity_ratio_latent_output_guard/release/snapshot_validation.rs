//! Exact CP402 snapshot, direct-route, and binary64 validation.

use super::super::transition::routes::{
    RetainedRoute, predecessor_index_is_public, predecessor_snapshot,
    snapshot_route as compressed_snapshot_route,
};
use super::super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardSnapshot as Snapshot,
};

pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_snapshot_is_exact_direct_release(
    snapshot: Snapshot,
) -> bool {
    snapshot_route(snapshot).is_some_and(|route| {
        predecessor_index_is_public(route.predecessor_index)
            && (!route.active
                || snapshot.maximum_total_cooling_capacity_w.is_some_and(|capacity| {
                    capacity.is_finite() && capacity >= 0.0
                }))
            && crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_snapshot_is_exact_direct_release(
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
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_SOURCE_ORDER
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
    let values_match = compare_clear!(predecessor_cp397_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp397_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp397_resulting_supply_temperature_c)
        && compare_clear!(predecessor_cp398_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp398_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp398_resulting_supply_temperature_c)
        && compare_clear!(predecessor_mixed_air_humidity_ratio)
        && compare_clear!(predecessor_psychrometric_cp_air_result_j_per_kg_k)
        && compare_clear!(predecessor_cp_air_j_per_kg_k)
        && compare_clear!(predecessor_cp399_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp399_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp399_resulting_supply_temperature_c)
        && compare_clear!(predecessor_supply_mass_flow_rate_kg_per_s)
        && compare_clear!(predecessor_cp400_cp_air_j_per_kg_k)
        && compare_clear!(predecessor_supply_mass_flow_rate_times_cp_air_w_per_k)
        && compare_clear!(predecessor_mixed_air_temperature_c)
        && compare_clear!(predecessor_supply_temperature_c)
        && compare_clear!(predecessor_mixed_air_minus_supply_temperature_k)
        && compare_clear!(predecessor_calculated_cooling_sensible_output_w)
        && compare_clear!(predecessor_cooling_sensible_output_w)
        && compare_clear!(predecessor_cp400_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp400_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp400_resulting_supply_temperature_c)
        && compare_clear!(predecessor_cooling_total_output_w)
        && compare_clear!(predecessor_cp401_cooling_sensible_output_w)
        && compare_clear!(predecessor_calculated_cooling_latent_output_w)
        && compare_clear!(predecessor_cooling_latent_output_w)
        && compare_clear!(predecessor_cp401_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp401_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp401_resulting_supply_temperature_c)
        && compare_clear!(cooling_latent_output_w)
        && compare_clear!(maximum_total_cooling_capacity_w)
        && compare_clear!(resulting_supply_humidity_ratio)
        && compare_clear!(resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(resulting_supply_temperature_c);
    values_match && left == right
}

pub(super) fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
