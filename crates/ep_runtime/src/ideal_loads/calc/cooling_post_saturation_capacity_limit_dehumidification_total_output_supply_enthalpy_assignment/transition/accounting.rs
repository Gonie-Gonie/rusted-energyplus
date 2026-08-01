//! CP385 checked route and exact source-site accounting.

use super::routes::{
    predecessor_route_count, predecessor_route_is_guard_evaluated, successor_route_count,
};
use super::{PredecessorRoute, State};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentRetainedRoute as Route;

pub(super) fn increment_counts(
    state: &mut State,
    predecessor: PredecessorRoute,
    route: Route,
    guard_evaluated: bool,
    assignment: bool,
) {
    increment_inherited(state, predecessor);
    increment_predecessor(state, predecessor);
    if guard_evaluated {
        increment_successor(state, route);
        state.dehumidification_total_output_capacity_guard_evaluation_count += 1;
        state.cp379_retained_supply_enthalpy_owned_read_count += 1;
        if assignment {
            state.dehumidification_total_output_maximum_capacity_assignment_count += 1;
            state.post_saturation_capacity_limited_dehumidification_supply_enthalpy_assignment_count += 1;
            state.source_site_execution_count += 6;
            state.cp329_retained_mixed_air_enthalpy_owned_read_count += 1;
            state.mixed_air_enthalpy_read_count += 1;
            state.cp384_retained_cooling_total_output_owned_read_count += 1;
            state.cooling_total_output_read_count += 1;
            state.cp330_retained_supply_mass_flow_rate_owned_read_count += 1;
            state.supply_mass_flow_rate_read_count += 1;
            state.specific_cooling_output_calculation_count += 1;
            state.supply_enthalpy_difference_calculation_count += 1;
            state.supply_enthalpy_assignment_write_count += 1;
        } else {
            state.dehumidification_total_output_capacity_guard_false_fallthrough_count += 1;
        }
    }
}

pub(super) fn next_transition_fits(
    state: &State,
    predecessor: PredecessorRoute,
    route: Route,
    assignment: bool,
) -> bool {
    let guard_evaluated = predecessor_route_is_guard_evaluated(predecessor);
    if state.transition_count.checked_add(1).is_none()
        || inherited_count(state, predecessor).checked_add(1).is_none()
        || predecessor_route_count(state, predecessor).checked_add(1).is_none()
    {
        return false;
    }
    if !guard_evaluated {
        return true;
    }
    if successor_route_count(state, route).checked_add(1).is_none()
        || state.dehumidification_total_output_capacity_guard_evaluation_count.checked_add(1).is_none()
        || state.cp379_retained_supply_enthalpy_owned_read_count.checked_add(1).is_none()
    {
        return false;
    }
    if !assignment {
        return state.dehumidification_total_output_capacity_guard_false_fallthrough_count.checked_add(1).is_some();
    }
    state.source_site_execution_count.checked_add(6).is_some()
        && [
            state.dehumidification_total_output_maximum_capacity_assignment_count,
            state.post_saturation_capacity_limited_dehumidification_supply_enthalpy_assignment_count,
            state.cp329_retained_mixed_air_enthalpy_owned_read_count,
            state.mixed_air_enthalpy_read_count,
            state.cp384_retained_cooling_total_output_owned_read_count,
            state.cooling_total_output_read_count,
            state.cp330_retained_supply_mass_flow_rate_owned_read_count,
            state.supply_mass_flow_rate_read_count,
            state.specific_cooling_output_calculation_count,
            state.supply_enthalpy_difference_calculation_count,
            state.supply_enthalpy_assignment_write_count,
        ]
        .into_iter()
        .all(|count| count.checked_add(1).is_some())
}

pub(super) fn inherited_count(state: &State, route: PredecessorRoute) -> usize {
    use PredecessorRoute as P;
    match route {
        P::UnitOff => state.unit_off_skip_count,
        P::NonCooling => state.non_cooling_skip_count,
        P::PositiveGuardFalseFallthrough => state.positive_guard_false_fallthrough_skip_count,
        P::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
        | P::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough
        | P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough
        | P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned => state.heating_availability_guard_false_fallthrough_count,
        P::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
        | P::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough
        | P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough
        | P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned => state.humidification_control_guard_false_fallthrough_count,
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough
        | P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough
        | P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough
        | P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned => state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        P::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough
        | P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough
        | P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough
        | P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned => state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        P::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
        | P::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough
        | P::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough
        | P::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned => state.dehumidification_control_guard_false_fallthrough_count,
    }
}

fn increment_inherited(state: &mut State, route: PredecessorRoute) {
    use PredecessorRoute as P;
    match route {
        P::UnitOff => state.unit_off_skip_count += 1,
        P::NonCooling => state.non_cooling_skip_count += 1,
        P::PositiveGuardFalseFallthrough => state.positive_guard_false_fallthrough_skip_count += 1,
        P::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
        | P::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough
        | P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough
        | P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned => state.heating_availability_guard_false_fallthrough_count += 1,
        P::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
        | P::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough
        | P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough
        | P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned => state.humidification_control_guard_false_fallthrough_count += 1,
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough
        | P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough
        | P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough
        | P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned => state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count += 1,
        P::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough
        | P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough
        | P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough
        | P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned => state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count += 1,
        P::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
        | P::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough
        | P::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough
        | P::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned => state.dehumidification_control_guard_false_fallthrough_count += 1,
    }
}

fn increment_predecessor(state: &mut State, route: PredecessorRoute) {
    use PredecessorRoute as P;
    match route {
        P::UnitOff | P::NonCooling | P::PositiveGuardFalseFallthrough => {}
        P::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => state.heating_availability_guard_false_fallthrough_capacity_guard_false_count += 1,
        P::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough => state.heating_availability_guard_false_fallthrough_dehumidification_guard_false_count += 1,
        P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough | P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned => state.heating_availability_guard_false_fallthrough_dehumidification_total_output_assignment_count += 1,
        P::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => state.humidification_control_guard_false_fallthrough_capacity_guard_false_count += 1,
        P::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => state.humidification_control_guard_false_fallthrough_dehumidification_guard_false_count += 1,
        P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough | P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned => state.humidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count += 1,
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough => state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count += 1,
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => state.dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count += 1,
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough | P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned => state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_assignment_count += 1,
        P::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough => state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count += 1,
        P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => state.dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count += 1,
        P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough | P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned => state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_assignment_count += 1,
        P::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count += 1,
        P::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => state.dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count += 1,
        P::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough | P::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned => state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count += 1,
    }
}

fn increment_successor(state: &mut State, route: Route) {
    use Route as R;
    match route {
        R::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough => state.heating_availability_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count += 1,
        R::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned => state.heating_availability_guard_false_fallthrough_dehumidification_total_output_maximum_capacity_assignment_count += 1,
        R::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough => state.humidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count += 1,
        R::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned => state.humidification_control_guard_false_fallthrough_dehumidification_total_output_maximum_capacity_assignment_count += 1,
        R::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough => state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_capacity_guard_false_fallthrough_count += 1,
        R::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned => state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_maximum_capacity_assignment_count += 1,
        R::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough => state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_capacity_guard_false_fallthrough_count += 1,
        R::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned => state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_maximum_capacity_assignment_count += 1,
        R::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough => state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count += 1,
        R::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned => state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_maximum_capacity_assignment_count += 1,
        _ => {}
    }
}
