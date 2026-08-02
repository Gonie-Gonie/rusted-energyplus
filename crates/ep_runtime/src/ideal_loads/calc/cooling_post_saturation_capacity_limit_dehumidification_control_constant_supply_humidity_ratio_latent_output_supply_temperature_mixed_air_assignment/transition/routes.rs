//! One-to-one CP402-to-CP403 retained-route classification.

use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_snapshot_route as cp402_snapshot_route;

/// One compressed CP403 route. CP402 body entry is CP403 assignment execution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) struct RetainedRoute {
    pub predecessor_index: usize,
    pub guard_evaluated: bool,
    pub assignment_executed: bool,
}

pub(super) fn predecessor_route(predecessor: Predecessor) -> Option<RetainedRoute> {
    let route = cp402_snapshot_route(predecessor)?;
    Some(RetainedRoute {
        predecessor_index: route.predecessor_index,
        guard_evaluated: route.active,
        assignment_executed: route.body_entered,
    })
}

pub(in crate::ideal_loads::calc) fn snapshot_route(snapshot: Snapshot) -> Option<RetainedRoute> {
    let predecessor = super::snapshot::predecessor_snapshot(snapshot);
    let route = predecessor_route(predecessor)?;
    local_shape_is_exact(snapshot, predecessor, route).then_some(route)
}

pub(super) fn local_shape_is_exact(
    snapshot: Snapshot,
    predecessor: Predecessor,
    route: RetainedRoute,
) -> bool {
    let assignment = route.assignment_executed;
    let active_flags = [
        snapshot.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_supply_temperature_mixed_air_assignment_executed,
        snapshot.cp329_retained_mixed_air_temperature_owned_read,
        snapshot.cp402_same_call_mixed_air_temperature_bit_corroborated,
        snapshot.mixed_air_temperature_read,
        snapshot.supply_temperature_assigned,
    ];
    if active_flags.into_iter().any(|flag| flag != assignment)
        || snapshot.cp402_retained_supply_humidity_ratio_state_owned
            != predecessor.resulting_supply_humidity_ratio.is_some()
        || snapshot.cp402_retained_supply_enthalpy_state_owned
            != predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        || snapshot.cp402_retained_supply_temperature_state_owned
            != predecessor.resulting_supply_temperature_c.is_some()
        || !option_bits_match(
            snapshot.resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
        || !option_bits_match(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
    {
        return false;
    }
    if assignment {
        let (Some(carried), Some(read), Some(assigned), Some(resulting)) = (
            predecessor.predecessor_mixed_air_temperature_c,
            snapshot.mixed_air_temperature_c,
            snapshot.assigned_supply_temperature_c,
            snapshot.resulting_supply_temperature_c,
        ) else {
            return false;
        };
        predecessor.predecessor_cp329_retained_mixed_air_temperature_owned_read
            && predecessor.predecessor_mixed_air_temperature_read
            && carried.to_bits() == read.to_bits()
            && read.to_bits() == assigned.to_bits()
            && assigned.to_bits() == resulting.to_bits()
    } else {
        snapshot.mixed_air_temperature_c.is_none()
            && snapshot.assigned_supply_temperature_c.is_none()
            && option_bits_match(
                snapshot.resulting_supply_temperature_c,
                predecessor.resulting_supply_temperature_c,
            )
    }
}

/// Stable flattened ordering of the 36 CP402/CP403 logical routes.
#[cfg(test)]
pub(in crate::ideal_loads::calc) const fn logical_route_index(route: RetainedRoute) -> usize {
    let mut extra = 0;
    let mut index = 0;
    while index < route.predecessor_index {
        if predecessor_index_is_active(index) {
            extra += 1;
        }
        index += 1;
    }
    route.predecessor_index + extra + if route.assignment_executed { 1 } else { 0 }
}

#[cfg(test)]
pub(in crate::ideal_loads::calc) const fn predecessor_index_is_active(index: usize) -> bool {
    matches!(index, 20 | 21 | 24 | 25 | 27 | 29)
}

pub(in crate::ideal_loads::calc) const fn predecessor_index_is_public(index: usize) -> bool {
    matches!(index, 0..=8 | 20 | 24)
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
