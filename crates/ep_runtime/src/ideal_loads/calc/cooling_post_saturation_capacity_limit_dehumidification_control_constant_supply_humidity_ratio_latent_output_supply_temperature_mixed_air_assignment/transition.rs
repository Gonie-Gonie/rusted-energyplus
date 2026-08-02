//! Pure CP402-to-CP403 supply-temperature mixed-air assignment.

use super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardSnapshot as Predecessor;

mod accounting;
pub(super) mod routes;
pub(super) mod snapshot;

use accounting::{increment_counts, next_transition_fits};
pub(in crate::ideal_loads::calc) use routes::RetainedRoute;
#[cfg(test)]
pub(in crate::ideal_loads::calc) use routes::logical_route_index;
use routes::predecessor_route;

pub(in crate::ideal_loads::calc) fn advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment_state(
    state: &mut State,
    predecessor: Predecessor,
) -> Option<Snapshot> {
    if state.system != predecessor.system {
        return None;
    }
    let route = predecessor_route(predecessor)?;
    let mixed_air_temperature_c = prepare_assignment(predecessor, route)?;
    if !next_transition_fits(state, predecessor, route) {
        return None;
    }

    let transition_ordinal = state.transition_count + 1;
    let result = snapshot::build_snapshot(predecessor, route, mixed_air_temperature_c);
    increment_counts(state, predecessor, route);
    state.latest = Some(result);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(transition_ordinal);
    Some(result)
}

fn prepare_assignment(
    predecessor: Predecessor,
    route: RetainedRoute,
) -> Option<Option<f64>> {
    if !route.assignment_executed {
        return Some(None);
    }
    if !predecessor.predecessor_cp329_retained_mixed_air_temperature_owned_read
        || !predecessor.predecessor_mixed_air_temperature_read
    {
        return None;
    }
    Some(Some(predecessor.predecessor_mixed_air_temperature_c?))
}

pub(super) fn source_assignment(mixed_air_temperature_c: f64) -> f64 {
    mixed_air_temperature_c
}

#[cfg(test)]
pub(super) use routes::{predecessor_index_is_active, predecessor_index_is_public};
