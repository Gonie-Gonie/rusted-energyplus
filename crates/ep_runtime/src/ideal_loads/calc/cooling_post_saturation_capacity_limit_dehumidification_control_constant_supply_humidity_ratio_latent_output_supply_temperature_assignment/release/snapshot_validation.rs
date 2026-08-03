//! Exact CP407 compact snapshot, route, and binary64 validation.

use super::super::transition::routes::{
    RetainedRoute, compressed_snapshot_route, predecessor_has_supply_enthalpy,
    predecessor_has_supply_humidity_ratio, predecessor_has_supply_temperature,
    predecessor_index_is_public, resulting_has_supply_enthalpy,
    resulting_has_supply_humidity_ratio, resulting_has_supply_temperature,
};
use super::super::transition::source_assignment;
use super::super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER as ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentSnapshot as Snapshot,
};

pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_snapshot_is_exact_direct_release(
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
    let index = route.predecessor_index;
    if !carrier_is_exact(
        snapshot.predecessor_cp406_resulting_supply_humidity_ratio,
        predecessor_has_supply_humidity_ratio(route),
    ) || !carrier_is_exact(
        snapshot.predecessor_cp406_resulting_supply_enthalpy_j_per_kg,
        predecessor_has_supply_enthalpy(index),
    ) || !carrier_is_exact(
        snapshot.predecessor_cp406_resulting_supply_temperature_c,
        predecessor_has_supply_temperature(index),
    ) || snapshot.cp406_retained_supply_temperature_state_owned
        != predecessor_has_supply_temperature(index)
        || !option_bits_match(
            snapshot.preexisting_supply_temperature_c,
            snapshot.predecessor_cp406_resulting_supply_temperature_c,
        )
        || !carrier_is_exact(
            snapshot.resulting_supply_humidity_ratio,
            resulting_has_supply_humidity_ratio(route),
        )
        || !carrier_is_exact(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            resulting_has_supply_enthalpy(index),
        )
        || !carrier_is_exact(
            snapshot.resulting_supply_temperature_c,
            resulting_has_supply_temperature(index),
        )
    {
        return None;
    }
    let local_exact = if route.assignment_executed {
        active_snapshot_is_exact(snapshot)
    } else {
        inactive_snapshot_is_exact(snapshot)
    };
    local_exact.then_some(route)
}

fn active_snapshot_is_exact(snapshot: Snapshot) -> bool {
    let (Some(enthalpy), Some(humidity), Some(psychrometric), Some(assigned)) = (
        snapshot.supply_enthalpy_j_per_kg,
        snapshot.supply_humidity_ratio,
        snapshot.psychrometric_supply_temperature_result_c,
        snapshot.assigned_supply_temperature_c,
    ) else {
        return false;
    };
    let expected = source_assignment(enthalpy, humidity);
    snapshot.cp385_retained_supply_enthalpy_owned_read
        && snapshot.cp406_same_call_supply_enthalpy_bit_corroborated
        && snapshot.supply_enthalpy_for_dry_bulb_inversion_read
        && snapshot.cp378_retained_supply_humidity_ratio_owned_read
        && snapshot.supply_humidity_ratio_for_dry_bulb_inversion_read
        && snapshot.psychrometric_supply_temperature_evaluated
        && snapshot.supply_temperature_assigned
        && snapshot
            .predecessor_cp406_resulting_supply_humidity_ratio
            .is_none()
        && snapshot
            .predecessor_cp406_resulting_supply_enthalpy_j_per_kg
            .is_some_and(|value| value.to_bits() == enthalpy.to_bits())
        && psychrometric.to_bits() == expected.to_bits()
        && assigned.to_bits() == psychrometric.to_bits()
        && snapshot
            .resulting_supply_humidity_ratio
            .is_some_and(|value| value.to_bits() == humidity.to_bits())
        && snapshot
            .resulting_supply_enthalpy_j_per_kg
            .is_some_and(|value| value.to_bits() == enthalpy.to_bits())
        && snapshot
            .resulting_supply_temperature_c
            .is_some_and(|value| value.to_bits() == assigned.to_bits())
}

fn inactive_snapshot_is_exact(snapshot: Snapshot) -> bool {
    !snapshot.cp385_retained_supply_enthalpy_owned_read
        && !snapshot.cp406_same_call_supply_enthalpy_bit_corroborated
        && !snapshot.supply_enthalpy_for_dry_bulb_inversion_read
        && snapshot.supply_enthalpy_j_per_kg.is_none()
        && !snapshot.cp378_retained_supply_humidity_ratio_owned_read
        && !snapshot.supply_humidity_ratio_for_dry_bulb_inversion_read
        && snapshot.supply_humidity_ratio.is_none()
        && !snapshot.psychrometric_supply_temperature_evaluated
        && snapshot.psychrometric_supply_temperature_result_c.is_none()
        && !snapshot.supply_temperature_assigned
        && snapshot.assigned_supply_temperature_c.is_none()
        && option_bits_match(
            snapshot.resulting_supply_humidity_ratio,
            snapshot.predecessor_cp406_resulting_supply_humidity_ratio,
        )
        && option_bits_match(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            snapshot.predecessor_cp406_resulting_supply_enthalpy_j_per_kg,
        )
        && option_bits_match(
            snapshot.resulting_supply_temperature_c,
            snapshot.predecessor_cp406_resulting_supply_temperature_c,
        )
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
    let values_match = compare_clear!(predecessor_cp406_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp406_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp406_resulting_supply_temperature_c)
        && compare_clear!(supply_enthalpy_j_per_kg)
        && compare_clear!(supply_humidity_ratio)
        && compare_clear!(preexisting_supply_temperature_c)
        && compare_clear!(psychrometric_supply_temperature_result_c)
        && compare_clear!(assigned_supply_temperature_c)
        && compare_clear!(resulting_supply_humidity_ratio)
        && compare_clear!(resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(resulting_supply_temperature_c);
    values_match && left == right
}

fn carrier_is_exact(value: Option<f64>, present: bool) -> bool {
    present == value.is_some()
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
