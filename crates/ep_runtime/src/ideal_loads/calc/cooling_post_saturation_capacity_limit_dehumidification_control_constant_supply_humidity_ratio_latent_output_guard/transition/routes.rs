//! Compressed CP401 route refinement into CP402's 36 logical successors.

use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardSnapshot as Snapshot,
};

mod predecessor;

pub(in crate::ideal_loads::calc) use predecessor::predecessor_snapshot;

/// One compressed CP402 successor route.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) struct RetainedRoute {
    pub predecessor_index: usize,
    pub active: bool,
    pub body_entered: bool,
}

pub(in crate::ideal_loads::calc) fn predecessor_route(
    predecessor: Predecessor,
) -> Option<RetainedRoute> {
    let route = crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_snapshot_route(predecessor)?;
    Some(RetainedRoute {
        predecessor_index: route.predecessor_index,
        active: predecessor_index_is_active(route.predecessor_index),
        body_entered: false,
    })
}

pub(in crate::ideal_loads::calc) fn snapshot_route(snapshot: Snapshot) -> Option<RetainedRoute> {
    let predecessor = predecessor_snapshot(snapshot);
    let base = predecessor_route(predecessor)?;
    let result = snapshot
        .cooling_latent_output_greater_than_or_equal_to_maximum_total_cooling_capacity;
    let body_entered = result == Some(true);
    let route = RetainedRoute {
        body_entered,
        ..base
    };
    local_shape_is_exact(snapshot, predecessor, route).then_some(route)
}

fn local_shape_is_exact(
    snapshot: Snapshot,
    predecessor: Predecessor,
    route: RetainedRoute,
) -> bool {
    let result = snapshot
        .cooling_latent_output_greater_than_or_equal_to_maximum_total_cooling_capacity;
    let supply_is_preserved = option_bits_match(
        predecessor.resulting_supply_humidity_ratio,
        snapshot.resulting_supply_humidity_ratio,
    ) && option_bits_match(
        predecessor.resulting_supply_enthalpy_j_per_kg,
        snapshot.resulting_supply_enthalpy_j_per_kg,
    ) && option_bits_match(
        predecessor.resulting_supply_temperature_c,
        snapshot.resulting_supply_temperature_c,
    );
    let owner_shape = snapshot.cp401_retained_supply_humidity_ratio_state_owned
        == predecessor_has_supply_humidity_ratio(route.predecessor_index)
        && snapshot.cp401_retained_supply_enthalpy_state_owned
            == predecessor_has_supply_enthalpy(route.predecessor_index)
        && snapshot.cp401_retained_supply_temperature_state_owned
            == predecessor_has_supply_temperature(route.predecessor_index);
    if !supply_is_preserved || !owner_shape {
        return false;
    }
    if !route.active {
        return !snapshot
            .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_evaluated
            && !snapshot.cp401_retained_cooling_latent_output_owned_read
            && !snapshot.cooling_latent_output_read
            && snapshot.cooling_latent_output_w.is_none()
            && !snapshot.cp321_maximum_total_cooling_capacity_owned_read
            && !snapshot.cp340_same_call_maximum_total_cooling_capacity_bit_corroborated
            && !snapshot.maximum_total_cooling_capacity_read
            && snapshot.maximum_total_cooling_capacity_w.is_none()
            && !snapshot
                .cooling_latent_output_maximum_total_cooling_capacity_comparison_evaluated
            && result.is_none()
            && !snapshot
                .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_body_entered
            && !snapshot
                .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough;
    }
    let (Some(predecessor_latent), Some(latent), Some(capacity), Some(result)) = (
        predecessor.cooling_latent_output_w,
        snapshot.cooling_latent_output_w,
        snapshot.maximum_total_cooling_capacity_w,
        result,
    ) else {
        return false;
    };
    snapshot
        .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_evaluated
        && snapshot.cp401_retained_cooling_latent_output_owned_read
        && snapshot.cooling_latent_output_read
        && latent.to_bits() == predecessor_latent.to_bits()
        && snapshot.cp321_maximum_total_cooling_capacity_owned_read
        && snapshot.cp340_same_call_maximum_total_cooling_capacity_bit_corroborated
        && snapshot.maximum_total_cooling_capacity_read
        && snapshot
            .cooling_latent_output_maximum_total_cooling_capacity_comparison_evaluated
        && result == (latent >= capacity)
        && snapshot
            .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_body_entered
            == result
        && snapshot
            .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough
            != result
}

pub(in crate::ideal_loads::calc) const fn predecessor_index_is_active(index: usize) -> bool {
    matches!(index, 20 | 21 | 24 | 25 | 27 | 29)
}

pub(in crate::ideal_loads::calc) const fn predecessor_index_is_public(index: usize) -> bool {
    matches!(index, 0..=8 | 20 | 24)
}

pub(super) const fn predecessor_has_supply_humidity_ratio(index: usize) -> bool {
    matches!(index, 18 | 19 | 22 | 23 | 26 | 28)
}

pub(super) const fn predecessor_has_supply_enthalpy(index: usize) -> bool {
    matches!(index, 5 | 8 | 11 | 14 | 17..=29)
}

pub(super) const fn predecessor_has_supply_temperature(index: usize) -> bool {
    index >= 3
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
