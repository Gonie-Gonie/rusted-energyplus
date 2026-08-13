//! Pure CP422-to-CP423 sensible-output supply-temperature transition.

use super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentSnapshot as Predecessor;
use crate::ideal_loads::calc::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentRetainedRoute as PredecessorRoute,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_retained_route_matches_snapshot_bounded,
};

mod accounting;
mod snapshot;

use accounting::{increment_counts, next_transition_fits};

/// One retained CP423 successor route over the exact 36-wide CP422 partition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentRetainedRoute {
    pub logical_index: usize,
    pub active: bool,
    pub assignment_executed: bool,
}

/// Release-sealed owners required only on the five CP423 assignment routes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads) struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentActiveInput {
    pub mixed_air_temperature_c: f64,
    pub cooling_sensible_output_w: f64,
    pub supply_mass_flow_rate_kg_per_s: f64,
    pub cp_air_j_per_kg_k: f64,
}

use PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentActiveInput as ActiveInput;
use PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentRetainedRoute as Route;

pub(in crate::ideal_loads::calc) fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_route_from_committed_predecessor(
    predecessor: Predecessor,
    predecessor_route: PredecessorRoute,
) -> Option<Route> {
    let active = predecessor_route.active;
    let assignment_executed = predecessor_route.assignment_executed;
    (predecessor_route.logical_index < 36
        && predecessor
            .post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_executed
            == assignment_executed)
        .then_some(Route {
            logical_index: predecessor_route.logical_index,
            active,
            assignment_executed,
        })
}

pub(in crate::ideal_loads::calc) fn advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_state(
    state: &mut State,
    predecessor: Predecessor,
    input: Option<ActiveInput>,
) -> Option<Snapshot> {
    let predecessor_route = crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_snapshot_route(predecessor)?;
    let route = cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_route_from_committed_predecessor(predecessor, predecessor_route)?;
    advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_state_with_validated_route(state, predecessor, route, input)
}

pub(in crate::ideal_loads::calc) fn advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_state_with_validated_route(
    state: &mut State,
    predecessor: Predecessor,
    route: Route,
    input: Option<ActiveInput>,
) -> Option<Snapshot> {
    if state.system != predecessor.system || !route_matches_predecessor(predecessor, route) {
        return None;
    }
    let input = prepare_exact_input(predecessor, route, input)?;
    if !next_transition_fits(state, predecessor, route) {
        return None;
    }
    let (denominator, drop, calculated) = if let Some(input) = input {
        let (denominator, drop, calculated) = calculate_supply_temperature(input);
        (Some(denominator), Some(drop), Some(calculated))
    } else {
        (None, None, None)
    };
    let snapshot = snapshot::build_snapshot(predecessor, route, input, denominator, drop, calculated);
    state.transition_count += 1;
    increment_counts(state, predecessor, route);
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    Some(snapshot)
}

pub(super) fn calculate_supply_temperature(input: ActiveInput) -> (f64, f64, f64) {
    let denominator = input.supply_mass_flow_rate_kg_per_s * input.cp_air_j_per_kg_k;
    let drop = input.cooling_sensible_output_w / denominator;
    let calculated = input.mixed_air_temperature_c - drop;
    (denominator, drop, calculated)
}

fn route_matches_predecessor(predecessor: Predecessor, route: Route) -> bool {
    let predecessor_route = PredecessorRoute {
        logical_index: route.logical_index,
        active: route.active,
        assignment_executed: route.assignment_executed,
    };
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_retained_route_matches_snapshot_bounded(
        predecessor,
        predecessor_route,
    ) && cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_route_from_committed_predecessor(
        predecessor,
        predecessor_route,
    ) == Some(route)
        && (!route.assignment_executed || route.active)
        && if route.assignment_executed {
            predecessor.resulting_cooling_sensible_output_after_maximum_capacity_assignment_w.is_some()
        } else {
            true
        }
}

fn prepare_exact_input(
    predecessor: Predecessor,
    route: Route,
    input: Option<ActiveInput>,
) -> Option<Option<ActiveInput>> {
    if !route.assignment_executed {
        return input.is_none().then_some(None);
    }
    let input = input?;
    let exact = [
        (input.mixed_air_temperature_c, predecessor.mixed_air_temperature_for_sensible_output_c?),
        (input.cooling_sensible_output_w, predecessor.resulting_cooling_sensible_output_after_maximum_capacity_assignment_w?),
        (input.supply_mass_flow_rate_kg_per_s, predecessor.supply_mass_flow_rate_kg_per_s?),
        (input.cp_air_j_per_kg_k, predecessor.cp_air_j_per_kg_k?),
    ]
    .into_iter()
    .all(|(actual, expected)| actual.to_bits() == expected.to_bits());
    exact.then_some(Some(input))
}
