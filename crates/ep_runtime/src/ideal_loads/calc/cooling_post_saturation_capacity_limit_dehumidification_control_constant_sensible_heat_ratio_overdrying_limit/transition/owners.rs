//! Exact CP390 retained-state validation for CP391.

use super::routes::{
    RetainedRoute, predecessor_has_supply_enthalpy, predecessor_has_supply_temperature,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitSnapshot as Predecessor;

#[derive(Clone, Copy)]
pub(super) struct PreparedInput {
    pub preexisting_supply_enthalpy_j_per_kg: Option<f64>,
    pub resulting_supply_temperature_c: Option<f64>,
    pub active: Option<PreparedActive>,
}

#[derive(Clone, Copy)]
pub(super) struct PreparedActive {
    pub supply_enthalpy_before_overdrying_limit_j_per_kg: f64,
    pub supply_temperature_c: f64,
}

pub(super) fn prepare_exact_input(
    predecessor: Predecessor,
    route: RetainedRoute,
) -> Option<PreparedInput> {
    let preexisting_supply_enthalpy_j_per_kg = predecessor.resulting_supply_enthalpy_j_per_kg;
    let resulting_supply_temperature_c = predecessor.resulting_supply_temperature_c;
    if preexisting_supply_enthalpy_j_per_kg.is_some()
        != predecessor_has_supply_enthalpy(route.predecessor_index)
        || resulting_supply_temperature_c.is_some()
            != predecessor_has_supply_temperature(route.predecessor_index)
    {
        return None;
    }
    let active = if route.active {
        Some(PreparedActive {
            supply_enthalpy_before_overdrying_limit_j_per_kg: preexisting_supply_enthalpy_j_per_kg?,
            supply_temperature_c: resulting_supply_temperature_c?,
        })
    } else {
        None
    };
    Some(PreparedInput {
        preexisting_supply_enthalpy_j_per_kg,
        resulting_supply_temperature_c,
        active,
    })
}
