//! Exact CP389 route preservation for CP390.

use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentSnapshot as Predecessor;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) struct RetainedRoute {
    pub predecessor_index: usize,
    pub active: bool,
}

pub(in crate::ideal_loads::calc) fn predecessor_route(
    predecessor: Predecessor,
) -> Option<RetainedRoute> {
    let route = crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_snapshot_route(predecessor)?;
    Some(RetainedRoute {
        predecessor_index: route.predecessor_index,
        active: route.active,
    })
}

pub(in crate::ideal_loads::calc) const fn predecessor_index_is_active(index: usize) -> bool {
    matches!(index, 18 | 22 | 28)
}

pub(in crate::ideal_loads::calc) const fn predecessor_has_supply_temperature(index: usize) -> bool {
    index >= 3
}
