//! Exact CP407 route preservation for CP408.

use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentSnapshot as Predecessor;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) struct RetainedRoute {
    pub predecessor_index: usize,
    pub predecessor_guard_evaluated: bool,
    pub predecessor_maximum_capacity_assignment_executed: bool,
    pub active: bool,
}

pub(in crate::ideal_loads::calc) fn predecessor_route(
    predecessor: Predecessor,
) -> Option<RetainedRoute> {
    let route = crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_snapshot_route(predecessor)?;
    Some(RetainedRoute {
        predecessor_index: route.predecessor_index,
        predecessor_guard_evaluated: route.predecessor_guard_evaluated,
        predecessor_maximum_capacity_assignment_executed: route
            .predecessor_maximum_capacity_assignment_executed,
        active: route.assignment_executed,
    })
}

/// Stable flattened ordering of the 36 CP407/CP408 logical routes.
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
    route.predecessor_index
        + extra
        + if route.predecessor_maximum_capacity_assignment_executed {
            1
        } else {
            0
        }
}

#[cfg(test)]
pub(in crate::ideal_loads::calc) const fn predecessor_index_is_active(index: usize) -> bool {
    matches!(index, 20 | 21 | 24 | 25 | 27 | 29)
}

pub(in crate::ideal_loads::calc) const fn predecessor_index_is_public(index: usize) -> bool {
    matches!(index, 0..=8 | 20 | 24)
}

pub(in crate::ideal_loads::calc) const fn predecessor_has_supply_humidity_ratio(
    route: RetainedRoute,
) -> bool {
    route.active
        || route.predecessor_maximum_capacity_assignment_executed
        || matches!(route.predecessor_index, 18 | 19 | 22 | 23 | 26 | 28)
}

pub(in crate::ideal_loads::calc) const fn predecessor_has_supply_enthalpy(index: usize) -> bool {
    matches!(index, 5 | 8 | 11 | 14 | 17..=29)
}

pub(in crate::ideal_loads::calc) const fn predecessor_has_supply_temperature(index: usize) -> bool {
    index >= 3
}
