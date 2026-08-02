//! Exact CP394 retained-state validation for CP395.

use super::routes::{
    RetainedRoute, predecessor_has_supply_enthalpy, predecessor_has_supply_humidity_ratio,
    predecessor_has_supply_temperature,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseEntrySnapshot as Predecessor;

#[derive(Clone, Copy)]
pub(super) struct PreparedInput {
    pub predecessor_supply_humidity_ratio: Option<f64>,
    pub predecessor_supply_enthalpy_j_per_kg: Option<f64>,
    pub predecessor_supply_temperature_c: Option<f64>,
    pub active: Option<PreparedActive>,
}

#[derive(Clone, Copy)]
pub(super) struct PreparedActive {
    pub supply_temperature_c: f64,
    pub supply_enthalpy_j_per_kg: f64,
}

pub(super) fn prepare_exact_input(
    predecessor: Predecessor,
    route: RetainedRoute,
) -> Option<PreparedInput> {
    let index = route.predecessor_index;
    let predecessor_supply_humidity_ratio = predecessor.resulting_supply_humidity_ratio;
    let predecessor_supply_enthalpy_j_per_kg = predecessor.resulting_supply_enthalpy_j_per_kg;
    let predecessor_supply_temperature_c = predecessor.resulting_supply_temperature_c;
    if predecessor_supply_humidity_ratio.is_some() != predecessor_has_supply_humidity_ratio(index)
        || predecessor_supply_enthalpy_j_per_kg.is_some() != predecessor_has_supply_enthalpy(index)
        || predecessor_supply_temperature_c.is_some() != predecessor_has_supply_temperature(index)
    {
        return None;
    }
    let active = if route.active {
        Some(PreparedActive {
            supply_temperature_c: predecessor_supply_temperature_c?,
            supply_enthalpy_j_per_kg: predecessor_supply_enthalpy_j_per_kg?,
        })
    } else {
        None
    };
    Some(PreparedInput {
        predecessor_supply_humidity_ratio,
        predecessor_supply_enthalpy_j_per_kg,
        predecessor_supply_temperature_c,
        active,
    })
}
