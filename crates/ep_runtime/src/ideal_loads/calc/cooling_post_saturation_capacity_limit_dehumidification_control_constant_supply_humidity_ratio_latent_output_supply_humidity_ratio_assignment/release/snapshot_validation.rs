//! Exact CP404 snapshot, direct-route, and binary64 validation.

use super::super::transition::routes::{
    RetainedRoute, predecessor_index_is_public, snapshot_route as compressed_snapshot_route,
};
pub(in crate::ideal_loads::calc) use super::super::transition::snapshot::predecessor_snapshot;
use super::super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER as ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyHumidityRatioAssignmentSnapshot as Snapshot,
};

pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment_snapshot_is_exact_direct_release(
    snapshot: Snapshot,
) -> bool {
    snapshot_route(snapshot).is_some_and(|route| {
        predecessor_index_is_public(route.predecessor_index)
            && crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment_snapshot_is_exact_direct_release(
                predecessor_snapshot(snapshot),
            )
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
        && compare_clear!(predecessor_cp402_cooling_latent_output_w)
        && compare_clear!(predecessor_maximum_total_cooling_capacity_w)
        && compare_clear!(predecessor_cp402_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp402_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp402_resulting_supply_temperature_c)
        && compare_clear!(predecessor_cp403_mixed_air_temperature_c)
        && compare_clear!(predecessor_cp403_assigned_supply_temperature_c)
        && compare_clear!(predecessor_cp403_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp403_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp403_resulting_supply_temperature_c)
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
