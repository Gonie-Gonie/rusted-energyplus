//! Exact CP401 snapshot, route, carrier, and binary64 validation.

use super::super::transition::routes::{
    RetainedRoute, compressed_snapshot_route, predecessor_has_supply_enthalpy,
    predecessor_has_supply_humidity_ratio, predecessor_has_supply_temperature,
    predecessor_snapshot,
};
use super::super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentSnapshot as Snapshot,
};

pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_snapshot_is_exact_direct_release(
    snapshot: Snapshot,
) -> bool {
    snapshot_route(snapshot).is_some_and(|route| {
        matches!(route.predecessor_index, 0..=8 | 20 | 24)
            && route.active == matches!(route.predecessor_index, 20 | 24)
            && crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_snapshot_is_exact_direct_release(
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
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_SOURCE_ORDER
    {
        return None;
    }
    let route = compressed_snapshot_route(snapshot)?;
    let index = route.predecessor_index;
    let active_flags = [
        snapshot.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_assignment_executed,
        snapshot.cp384_retained_cooling_total_output_owned_read,
        snapshot.cp385_cooling_total_output_bit_corroborated,
        snapshot.cooling_total_output_read,
        snapshot.cp400_retained_cooling_sensible_output_owned_read,
        snapshot.cooling_sensible_output_read,
        snapshot.cooling_latent_output_calculated,
        snapshot.cooling_latent_output_assigned,
    ];
    if active_flags.into_iter().any(|flag| flag != route.active)
        || snapshot.cp400_retained_supply_humidity_ratio_state_owned
            != predecessor_has_supply_humidity_ratio(index)
        || snapshot.cp400_retained_supply_enthalpy_state_owned
            != predecessor_has_supply_enthalpy(index)
        || snapshot.cp400_retained_supply_temperature_state_owned
            != predecessor_has_supply_temperature(index)
        || !carrier_is_exact(
            snapshot.predecessor_cp400_resulting_supply_humidity_ratio,
            snapshot.resulting_supply_humidity_ratio,
            predecessor_has_supply_humidity_ratio(index),
        )
        || !carrier_is_exact(
            snapshot.predecessor_cp400_resulting_supply_enthalpy_j_per_kg,
            snapshot.resulting_supply_enthalpy_j_per_kg,
            predecessor_has_supply_enthalpy(index),
        )
        || !carrier_is_exact(
            snapshot.predecessor_cp400_resulting_supply_temperature_c,
            snapshot.resulting_supply_temperature_c,
            predecessor_has_supply_temperature(index),
        )
    {
        return None;
    }
    let local_values = [
        snapshot.cooling_total_output_w,
        snapshot.cooling_sensible_output_w,
        snapshot.calculated_cooling_latent_output_w,
        snapshot.cooling_latent_output_w,
    ];
    if route.active {
        let [Some(total), Some(sensible), Some(calculated), Some(assigned)] = local_values else {
            return None;
        };
        if !option_bits_match(
            snapshot.predecessor_cooling_sensible_output_w,
            Some(sensible),
        ) || calculated.to_bits() != (total - sensible).to_bits()
            || assigned.to_bits() != calculated.to_bits()
        {
            return None;
        }
    } else if local_values.into_iter().any(|value| value.is_some()) {
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
        && compare_clear!(cooling_total_output_w)
        && compare_clear!(cooling_sensible_output_w)
        && compare_clear!(calculated_cooling_latent_output_w)
        && compare_clear!(cooling_latent_output_w)
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
