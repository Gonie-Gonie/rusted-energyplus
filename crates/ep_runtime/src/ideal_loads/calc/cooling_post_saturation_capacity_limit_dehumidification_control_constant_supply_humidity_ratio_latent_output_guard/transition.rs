//! Pure CP401-to-CP402 shared-case latent-output capacity guard.

use super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardSnapshot as Snapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentSnapshot as Predecessor;

mod accounting;
pub(in crate::ideal_loads::calc) mod routes;
mod snapshot;

use accounting::{increment_counts, next_transition_fits};
use routes::{RetainedRoute, predecessor_route};

/// Release-validated same-call operand owners for line 2297.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads) struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardActiveInput {
    pub cooling_latent_output_w: f64,
    pub maximum_total_cooling_capacity_w: f64,
    pub cp401_cooling_latent_output_owned_read: bool,
    pub cp321_maximum_total_cooling_capacity_owned_read: bool,
    pub cp340_same_call_maximum_total_cooling_capacity_bit_corroborated: bool,
}

use PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardActiveInput as ActiveInput;

pub(in crate::ideal_loads::calc) fn advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_state(
    state: &mut State,
    predecessor: Predecessor,
    input: Option<ActiveInput>,
) -> Option<Snapshot> {
    if state.system != predecessor.system {
        return None;
    }
    let base_route = predecessor_route(predecessor)?;
    let prepared = prepare_guard(predecessor, base_route, input)?;
    let body_entered = prepared.comparison == Some(true);
    let route = RetainedRoute {
        body_entered,
        ..base_route
    };
    if !next_transition_fits(state, route) {
        return None;
    }

    let transition_ordinal = state.transition_count + 1;
    let snapshot = snapshot::build_snapshot(
        predecessor,
        route,
        prepared.cooling_latent_output_w,
        prepared.maximum_total_cooling_capacity_w,
        prepared.comparison,
    );
    state.transition_count = transition_ordinal;
    increment_counts(state, route);
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(transition_ordinal);
    Some(snapshot)
}

struct PreparedGuard {
    cooling_latent_output_w: Option<f64>,
    maximum_total_cooling_capacity_w: Option<f64>,
    comparison: Option<bool>,
}

fn prepare_guard(
    predecessor: Predecessor,
    route: RetainedRoute,
    input: Option<ActiveInput>,
) -> Option<PreparedGuard> {
    if !route.active {
        return (input.is_none() && predecessor.cooling_latent_output_w.is_none()).then_some(
            PreparedGuard {
                cooling_latent_output_w: None,
                maximum_total_cooling_capacity_w: None,
                comparison: None,
            },
        );
    }
    let input = input?;
    let predecessor_latent = predecessor.cooling_latent_output_w?;
    if !input.cp401_cooling_latent_output_owned_read
        || !input.cp321_maximum_total_cooling_capacity_owned_read
        || !input.cp340_same_call_maximum_total_cooling_capacity_bit_corroborated
        || input.cooling_latent_output_w.to_bits() != predecessor_latent.to_bits()
    {
        return None;
    }
    Some(PreparedGuard {
        cooling_latent_output_w: Some(input.cooling_latent_output_w),
        maximum_total_cooling_capacity_w: Some(input.maximum_total_cooling_capacity_w),
        comparison: Some(source_greater_than_or_equal(
            input.cooling_latent_output_w,
            input.maximum_total_cooling_capacity_w,
        )),
    })
}

pub(super) fn source_greater_than_or_equal(left: f64, right: f64) -> bool {
    left >= right
}
