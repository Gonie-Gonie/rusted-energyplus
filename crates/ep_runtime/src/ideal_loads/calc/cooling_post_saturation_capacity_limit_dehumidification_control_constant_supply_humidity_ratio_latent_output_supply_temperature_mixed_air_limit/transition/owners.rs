//! Exact CP407 state and active CP329 mixed-air owner validation for CP408.

use super::routes::{
    RetainedRoute, predecessor_has_supply_enthalpy, predecessor_has_supply_humidity_ratio,
    predecessor_has_supply_temperature,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot as MixedAirOwner,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentSnapshot as Predecessor,
    cooling_mixed_air_call_snapshot_is_exact_direct_release,
};

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
    pub mixed_air_temperature_c: f64,
}

pub(super) fn prepare_exact_input(
    predecessor: Predecessor,
    route: RetainedRoute,
    mixed_air_owner: Option<MixedAirOwner>,
) -> Option<PreparedInput> {
    let index = route.predecessor_index;
    let predecessor_supply_humidity_ratio = predecessor.resulting_supply_humidity_ratio;
    let predecessor_supply_enthalpy_j_per_kg = predecessor.resulting_supply_enthalpy_j_per_kg;
    let predecessor_supply_temperature_c = predecessor.resulting_supply_temperature_c;
    if predecessor_supply_humidity_ratio.is_some() != predecessor_has_supply_humidity_ratio(route)
        || predecessor_supply_enthalpy_j_per_kg.is_some() != predecessor_has_supply_enthalpy(index)
        || predecessor_supply_temperature_c.is_some() != predecessor_has_supply_temperature(index)
    {
        return None;
    }
    let active = match (route.active, mixed_air_owner) {
        (false, None) => None,
        (true, Some(owner)) => Some(exact_active_values(predecessor, owner)?),
        _ => return None,
    };
    Some(PreparedInput {
        predecessor_supply_humidity_ratio,
        predecessor_supply_enthalpy_j_per_kg,
        predecessor_supply_temperature_c,
        active,
    })
}

fn exact_active_values(predecessor: Predecessor, owner: MixedAirOwner) -> Option<PreparedActive> {
    if !cooling_mixed_air_call_snapshot_is_exact_direct_release(owner)
        || predecessor.system != owner.system
        || predecessor.parent_call_ordinal != owner.parent_call_ordinal
        || predecessor.controlled_zone != owner.controlled_zone
        || !owner.mixed_air_temperature_assigned
    {
        return None;
    }
    Some(PreparedActive {
        supply_temperature_c: predecessor.resulting_supply_temperature_c?,
        mixed_air_temperature_c: owner.mixed_air_temperature_c?,
    })
}
