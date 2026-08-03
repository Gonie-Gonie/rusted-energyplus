//! Exact CP404 retained-state validation for CP405.

use super::routes::{
    RetainedRoute, predecessor_has_supply_enthalpy, predecessor_has_supply_humidity_ratio,
    predecessor_has_supply_temperature,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyHumidityRatioAssignmentSnapshot as Predecessor;

#[derive(Clone, Copy)]
pub(super) struct PreparedInput {
    pub predecessor_supply_humidity_ratio: Option<f64>,
    pub predecessor_supply_enthalpy_j_per_kg: Option<f64>,
    pub predecessor_supply_temperature_c: Option<f64>,
    pub preexisting_cooling_latent_output_w: Option<f64>,
    pub maximum_total_cooling_capacity_w: Option<f64>,
}

pub(super) fn prepare_exact_input(
    predecessor: Predecessor,
    route: RetainedRoute,
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

    let preexisting_cooling_latent_output_w = if route.guard_evaluated {
        Some(predecessor.predecessor_cp402_cooling_latent_output_w?)
    } else {
        if predecessor.predecessor_cp402_cooling_latent_output_w.is_some() {
            return None;
        }
        None
    };
    let maximum_total_cooling_capacity_w = if route.assignment_executed {
        if !predecessor.predecessor_cp321_maximum_total_cooling_capacity_owned_read
            || !predecessor
                .predecessor_cp340_same_call_maximum_total_cooling_capacity_bit_corroborated
            || !predecessor.predecessor_maximum_total_cooling_capacity_read
        {
            return None;
        }
        Some(predecessor.predecessor_maximum_total_cooling_capacity_w?)
    } else {
        None
    };
    Some(PreparedInput {
        predecessor_supply_humidity_ratio,
        predecessor_supply_enthalpy_j_per_kg,
        predecessor_supply_temperature_c,
        preexisting_cooling_latent_output_w,
        maximum_total_cooling_capacity_w,
    })
}
