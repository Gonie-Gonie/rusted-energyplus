//! Pure CP404-to-CP405 latent-output maximum-capacity assignment.

use super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputMaximumCapacityAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputMaximumCapacityAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyHumidityRatioAssignmentSnapshot as Predecessor;

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

pub(in crate::ideal_loads::calc) fn advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_state(
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

    let assigned_cooling_latent_output_w = prepared
        .maximum_total_cooling_capacity_w
        .map(source_assignment);
    let transition_ordinal = state.transition_count + 1;
    let result = snapshot::build_snapshot(
        predecessor,
        route,
        prepared,
        assigned_cooling_latent_output_w,
    );
    increment_counts(state, route);
    state.latest = Some(result);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(transition_ordinal);
    Some(result)
}

pub(super) const fn source_assignment(maximum_total_cooling_capacity_w: f64) -> f64 {
    maximum_total_cooling_capacity_w
}

#[cfg(test)]
pub(in crate::ideal_loads::calc) fn test_increment_counts(
    state: &mut State,
    route: RetainedRoute,
) {
    accounting::increment_counts(state, route);
}

#[cfg(test)]
pub(in crate::ideal_loads::calc) fn test_next_transition_fits(
    state: &State,
    route: RetainedRoute,
) -> bool {
    accounting::next_transition_fits(state, route)
}

#[cfg(test)]
pub(super) use routes::{predecessor_index_is_active, predecessor_index_is_public};
