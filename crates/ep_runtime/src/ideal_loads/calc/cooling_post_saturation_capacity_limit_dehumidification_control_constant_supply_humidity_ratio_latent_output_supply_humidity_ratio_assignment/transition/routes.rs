//! One-to-one CP403-to-CP404 retained-route classification.

use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyHumidityRatioAssignmentSnapshot as Snapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirAssignmentSnapshot as Predecessor,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment_snapshot_route as cp403_snapshot_route;

/// One compressed CP404 route. CP403 assignment execution is CP404 assignment execution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) struct RetainedRoute {
    pub predecessor_index: usize,
    pub guard_evaluated: bool,
    pub assignment_executed: bool,
}

pub(super) fn predecessor_route(predecessor: Predecessor) -> Option<RetainedRoute> {
    let route = cp403_snapshot_route(predecessor)?;
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
    let active_flags = [
        snapshot.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_supply_humidity_ratio_assignment_executed,
        snapshot.cp403_retained_supply_temperature_owned_read,
        snapshot.supply_temperature_for_humidity_ratio_inversion_read,
        snapshot.cp403_retained_supply_enthalpy_owned_read,
        snapshot.supply_enthalpy_for_humidity_ratio_inversion_read,
        snapshot.psychrometric_supply_humidity_ratio_evaluated,
        snapshot.supply_humidity_ratio_assignment_performed,
    ];
    if active_flags.into_iter().any(|flag| flag != assignment)
        || snapshot.cp403_retained_supply_humidity_ratio_state_owned
            != predecessor_has_supply_humidity_ratio(index)
        || snapshot.cp403_retained_supply_temperature_state_owned
            != predecessor_has_supply_temperature(index)
        || snapshot.cp403_retained_supply_enthalpy_state_owned
            != predecessor_has_supply_enthalpy(index)
        || snapshot.resulting_supply_humidity_ratio.is_some()
            != resulting_has_supply_humidity_ratio(route)
        || !option_bits_match(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            snapshot.predecessor_cp403_resulting_supply_enthalpy_j_per_kg,
        )
        || !option_bits_match(
            snapshot.resulting_supply_temperature_c,
            snapshot.predecessor_cp403_resulting_supply_temperature_c,
        )
    {
        return false;
    }
    if assignment {
        let (
            Some(temperature),
            Some(enthalpy),
            Some(psychrometric),
            Some(assigned),
            Some(resulting),
            Some(predecessor_temperature),
            Some(predecessor_enthalpy),
        ) = (
            snapshot.supply_temperature_c,
            snapshot.supply_enthalpy_j_per_kg,
            snapshot.psychrometric_supply_humidity_ratio,
            snapshot.assigned_supply_humidity_ratio,
            snapshot.resulting_supply_humidity_ratio,
            snapshot.predecessor_cp403_resulting_supply_temperature_c,
            snapshot.predecessor_cp403_resulting_supply_enthalpy_j_per_kg,
        ) else {
            return false;
        };
        let expected = super::source_assignment(temperature, enthalpy);
        temperature.to_bits() == predecessor_temperature.to_bits()
            && enthalpy.to_bits() == predecessor_enthalpy.to_bits()
            && psychrometric.to_bits() == expected.to_bits()
            && assigned.to_bits() == psychrometric.to_bits()
            && resulting.to_bits() == assigned.to_bits()
    } else {
        [
            snapshot.supply_temperature_c,
            snapshot.supply_enthalpy_j_per_kg,
            snapshot.psychrometric_supply_humidity_ratio,
            snapshot.assigned_supply_humidity_ratio,
        ]
        .into_iter()
        .all(|value| value.is_none())
            && option_bits_match(
                snapshot.resulting_supply_humidity_ratio,
                snapshot.predecessor_cp403_resulting_supply_humidity_ratio,
            )
    }
}

/// Stable flattened ordering of the 36 CP403/CP404 logical routes.
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

pub(super) const fn predecessor_has_supply_humidity_ratio(index: usize) -> bool {
    matches!(index, 18 | 19 | 22 | 23 | 26 | 28)
}

pub(super) const fn predecessor_has_supply_enthalpy(index: usize) -> bool {
    matches!(index, 5 | 8 | 11 | 14 | 17..=29)
}

pub(super) const fn predecessor_has_supply_temperature(index: usize) -> bool {
    index >= 3
}

const fn resulting_has_supply_humidity_ratio(route: RetainedRoute) -> bool {
    route.assignment_executed || predecessor_has_supply_humidity_ratio(route.predecessor_index)
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
