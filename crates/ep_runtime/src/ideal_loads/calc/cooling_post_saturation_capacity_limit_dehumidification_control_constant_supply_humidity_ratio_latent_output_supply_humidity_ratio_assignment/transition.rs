//! Pure CP403-to-CP404 supply-humidity-ratio assignment.

use super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyHumidityRatioAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyHumidityRatioAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirAssignmentSnapshot as Predecessor;
use crate::psychrometrics::energyplus_psy_w_fn_tdb_h;

mod accounting;
mod owners;
pub(super) mod routes;
pub(super) mod snapshot;

use accounting::{increment_counts, next_transition_fits};
use owners::prepare_exact_input;
pub(in crate::ideal_loads::calc) use routes::RetainedRoute;
#[cfg(test)]
pub(in crate::ideal_loads::calc) use routes::logical_route_index;
use routes::predecessor_route;

pub(in crate::ideal_loads::calc) fn advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment_state(
    state: &mut State,
    predecessor: Predecessor,
) -> Option<Snapshot> {
    if state.system != predecessor.system {
        return None;
    }
    let route = predecessor_route(predecessor)?;
    let prepared = prepare_exact_input(predecessor, route)?;
    if !next_transition_fits(state, route) {
        return None;
    }

    let psychrometric_supply_humidity_ratio = prepared.active.map(|active| {
        source_assignment(active.supply_temperature_c, active.supply_enthalpy_j_per_kg)
    });
    let transition_ordinal = state.transition_count + 1;
    let result = snapshot::build_snapshot(predecessor, route, prepared, psychrometric_supply_humidity_ratio);
    increment_counts(state, route);
    state.latest = Some(result);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(transition_ordinal);
    Some(result)
}

pub(super) fn source_assignment(
    supply_temperature_c: f64,
    supply_enthalpy_j_per_kg: f64,
) -> f64 {
    energyplus_psy_w_fn_tdb_h(supply_temperature_c, supply_enthalpy_j_per_kg)
}

#[cfg(test)]
pub(super) use routes::{predecessor_index_is_active, predecessor_index_is_public};
