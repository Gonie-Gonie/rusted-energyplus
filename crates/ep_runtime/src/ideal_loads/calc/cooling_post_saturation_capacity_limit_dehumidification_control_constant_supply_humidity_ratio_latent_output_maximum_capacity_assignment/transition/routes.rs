//! One-to-one CP404-to-CP405 retained-route classification.

use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputMaximumCapacityAssignmentSnapshot as Snapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyHumidityRatioAssignmentSnapshot as Predecessor,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment_snapshot_route as cp404_snapshot_route;

/// One compressed CP405 route. CP404 assignment execution is CP405 assignment execution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) struct RetainedRoute {
    pub predecessor_index: usize,
    pub guard_evaluated: bool,
    pub assignment_executed: bool,
}

pub(super) fn predecessor_route(predecessor: Predecessor) -> Option<RetainedRoute> {
    let route = cp404_snapshot_route(predecessor)?;
    Some(RetainedRoute {
        predecessor_index: route.predecessor_index,
        guard_evaluated: route.guard_evaluated,
        assignment_executed: route.assignment_executed,
    })
}

pub(in crate::ideal_loads::calc) fn snapshot_route(snapshot: Snapshot) -> Option<RetainedRoute> {
    let predecessor = super::snapshot::predecessor_snapshot(snapshot);
    let route = predecessor_route(predecessor)?;
    local_shape_is_exact(snapshot, route).then_some(route)
}

fn local_shape_is_exact(snapshot: Snapshot, route: RetainedRoute) -> bool {
    let assignment = route.assignment_executed;
    let index = route.predecessor_index;
    let predecessor = super::snapshot::predecessor_snapshot(snapshot);
    if snapshot
        .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed
        != assignment
        || snapshot.cp404_retained_supply_humidity_ratio_state_owned
            != predecessor_has_supply_humidity_ratio(route)
        || snapshot.cp404_retained_supply_temperature_state_owned
            != predecessor_has_supply_temperature(index)
        || snapshot.cp404_retained_supply_enthalpy_state_owned
            != predecessor_has_supply_enthalpy(index)
        || snapshot.preexisting_cooling_latent_output_w.is_some() != route.guard_evaluated
        || snapshot.resulting_cooling_latent_output_w.is_some() != route.guard_evaluated
        || snapshot.cp404_retained_maximum_total_cooling_capacity_owned_read != assignment
        || snapshot.maximum_total_cooling_capacity_read != assignment
        || snapshot.cooling_latent_output_assigned != assignment
        || !option_bits_match(
            snapshot.resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
        || !option_bits_match(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        || !option_bits_match(
            snapshot.resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
    {
        return false;
    }
    if assignment {
        let (Some(preexisting), Some(maximum), Some(assigned), Some(resulting)) = (
            snapshot.preexisting_cooling_latent_output_w,
            snapshot.maximum_total_cooling_capacity_w,
            snapshot.assigned_cooling_latent_output_w,
            snapshot.resulting_cooling_latent_output_w,
        ) else {
            return false;
        };
        predecessor
            .predecessor_cp402_cooling_latent_output_w
            .is_some_and(|value| value.to_bits() == preexisting.to_bits())
            && predecessor
                .predecessor_maximum_total_cooling_capacity_w
                .is_some_and(|value| value.to_bits() == maximum.to_bits())
            && assigned.to_bits() == super::source_assignment(maximum).to_bits()
            && resulting.to_bits() == assigned.to_bits()
    } else if route.guard_evaluated {
        snapshot.maximum_total_cooling_capacity_w.is_none()
            && snapshot.assigned_cooling_latent_output_w.is_none()
            && option_bits_match(
                snapshot.preexisting_cooling_latent_output_w,
                predecessor.predecessor_cp402_cooling_latent_output_w,
            )
            && option_bits_match(
                snapshot.resulting_cooling_latent_output_w,
                snapshot.preexisting_cooling_latent_output_w,
            )
    } else {
        [
            snapshot.preexisting_cooling_latent_output_w,
            snapshot.maximum_total_cooling_capacity_w,
            snapshot.assigned_cooling_latent_output_w,
            snapshot.resulting_cooling_latent_output_w,
        ]
        .into_iter()
        .all(|value| value.is_none())
    }
}

/// Stable flattened ordering of the 36 CP404/CP405 logical routes.
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

pub(super) const fn predecessor_has_supply_humidity_ratio(route: RetainedRoute) -> bool {
    route.assignment_executed
        || matches!(route.predecessor_index, 18 | 19 | 22 | 23 | 26 | 28)
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
