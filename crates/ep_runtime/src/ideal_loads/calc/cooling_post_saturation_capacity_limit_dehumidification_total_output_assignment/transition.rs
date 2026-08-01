//! Pure CP381-to-CP382 post-saturation total-output assignment.

use super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardSnapshot as Predecessor;

mod accounting;
mod routes;

use accounting::{increment_route_counts, next_route_counters_fit};
use routes::retained_route;
pub(in crate::ideal_loads::calc) use routes::{
    PredecessorRoute, predecessor_route, predecessor_route_is_assignment,
};

/// Release-validated same-call numerical owners for line 2267.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads) struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentActiveInput
{
    pub supply_mass_flow_rate_kg_per_s: f64,
    pub mixed_air_enthalpy_j_per_kg: f64,
    pub supply_enthalpy_j_per_kg: f64,
    pub cp330_supply_mass_flow_rate_owned_read: bool,
    pub cp329_same_call_supply_mass_flow_rate_bit_corroborated: bool,
    pub cp339_same_call_supply_mass_flow_rate_bit_corroborated: bool,
    pub cp329_mixed_air_enthalpy_owned_read: bool,
    pub cp329_same_call_recirculation_enthalpy_bit_corroborated: bool,
    pub cp339_same_call_mixed_air_enthalpy_bit_corroborated: bool,
    pub cp379_post_saturation_supply_enthalpy_owned_read: bool,
    pub cp379_same_call_supply_enthalpy_bits_corroborated: bool,
}

struct PreparedAssignment {
    supply_mass_flow_rate_kg_per_s: Option<f64>,
    mixed_air_enthalpy_j_per_kg: Option<f64>,
    supply_enthalpy_j_per_kg: Option<f64>,
    enthalpy_difference_j_per_kg: Option<f64>,
    cooling_total_output_w: Option<f64>,
}

pub(in crate::ideal_loads::calc) fn advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_state(
    state: &mut State,
    predecessor: Predecessor,
    input: Option<
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentActiveInput,
    >,
) -> Option<Snapshot> {
    if state.system != predecessor.system {
        return None;
    }
    let predecessor_route = predecessor_route(predecessor)?;
    let assignment = predecessor_route_is_assignment(predecessor_route);
    let prepared = prepare_assignment(assignment, input)?;
    let route = retained_route(predecessor_route);
    if !next_transition_fits(state, predecessor_route, route, assignment) {
        return None;
    }

    state.transition_count += 1;
    increment_route_counts(state, predecessor_route, route);
    if assignment {
        state.dehumidification_total_output_assignment_count += 1;
        state.source_site_execution_count +=
            PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE_ORDER.len();
        for count in active_counters_mut(state) {
            *count += 1;
        }
    }

    let snapshot = Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
        system: state.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor
            .positive_guard_false_fallthrough_skipped,
        heating_availability_guard_false_fallthrough: predecessor
            .heating_availability_guard_false_fallthrough,
        humidification_control_guard_false_fallthrough: predecessor
            .humidification_control_guard_false_fallthrough,
        dehumidification_control_humidistat_maximum_assignment_executed: predecessor
            .dehumidification_control_humidistat_maximum_assignment_executed,
        dehumidification_control_none_maximum_assignment_executed: predecessor
            .dehumidification_control_none_maximum_assignment_executed,
        dehumidification_control_guard_false_fallthrough: predecessor
            .dehumidification_control_guard_false_fallthrough,
        predecessor_capacity_limit_guard_evaluated: predecessor
            .predecessor_capacity_limit_guard_evaluated,
        predecessor_capacity_limit_body_entered: predecessor
            .predecessor_capacity_limit_body_entered,
        predecessor_active_capacity_limit_guard_false_fallthrough: predecessor
            .predecessor_active_capacity_limit_guard_false_fallthrough,
        predecessor_dehumidification_guard_evaluated: predecessor
            .dehumidification_guard_evaluated,
        predecessor_dehumidification_body_entered: predecessor.dehumidification_body_entered,
        predecessor_dehumidification_guard_false_fallthrough: predecessor
            .dehumidification_guard_false_fallthrough,
        dehumidification_total_output_assignment_executed: assignment,
        cp330_supply_mass_flow_rate_owned_read: assignment,
        cp329_same_call_supply_mass_flow_rate_bit_corroborated: assignment,
        cp339_same_call_supply_mass_flow_rate_bit_corroborated: assignment,
        supply_mass_flow_rate_read: assignment,
        supply_mass_flow_rate_kg_per_s: prepared.supply_mass_flow_rate_kg_per_s,
        cp329_mixed_air_enthalpy_owned_read: assignment,
        cp329_same_call_recirculation_enthalpy_bit_corroborated: assignment,
        cp339_same_call_mixed_air_enthalpy_bit_corroborated: assignment,
        mixed_air_enthalpy_read: assignment,
        mixed_air_enthalpy_j_per_kg: prepared.mixed_air_enthalpy_j_per_kg,
        cp379_post_saturation_supply_enthalpy_owned_read: assignment,
        cp379_same_call_supply_enthalpy_bits_corroborated: assignment,
        supply_enthalpy_read: assignment,
        supply_enthalpy_j_per_kg: prepared.supply_enthalpy_j_per_kg,
        enthalpy_difference_calculated: assignment,
        mixed_air_minus_supply_enthalpy_j_per_kg: prepared.enthalpy_difference_j_per_kg,
        cooling_total_output_calculated: assignment,
        calculated_cooling_total_output_w: prepared.cooling_total_output_w,
        cooling_total_output_assigned: assignment,
        cooling_total_output_w: prepared.cooling_total_output_w,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    Some(snapshot)
}

fn prepare_assignment(
    active: bool,
    input: Option<
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentActiveInput,
    >,
) -> Option<PreparedAssignment> {
    if !active {
        return input.is_none().then_some(PreparedAssignment {
            supply_mass_flow_rate_kg_per_s: None,
            mixed_air_enthalpy_j_per_kg: None,
            supply_enthalpy_j_per_kg: None,
            enthalpy_difference_j_per_kg: None,
            cooling_total_output_w: None,
        });
    }
    let input = input?;
    if ![
        input.cp330_supply_mass_flow_rate_owned_read,
        input.cp329_same_call_supply_mass_flow_rate_bit_corroborated,
        input.cp339_same_call_supply_mass_flow_rate_bit_corroborated,
        input.cp329_mixed_air_enthalpy_owned_read,
        input.cp329_same_call_recirculation_enthalpy_bit_corroborated,
        input.cp339_same_call_mixed_air_enthalpy_bit_corroborated,
        input.cp379_post_saturation_supply_enthalpy_owned_read,
        input.cp379_same_call_supply_enthalpy_bits_corroborated,
    ]
    .into_iter()
    .all(|evidence| evidence)
    {
        return None;
    }
    let enthalpy_difference_j_per_kg =
        input.mixed_air_enthalpy_j_per_kg - input.supply_enthalpy_j_per_kg;
    let cooling_total_output_w =
        input.supply_mass_flow_rate_kg_per_s * enthalpy_difference_j_per_kg;
    Some(PreparedAssignment {
        supply_mass_flow_rate_kg_per_s: Some(input.supply_mass_flow_rate_kg_per_s),
        mixed_air_enthalpy_j_per_kg: Some(input.mixed_air_enthalpy_j_per_kg),
        supply_enthalpy_j_per_kg: Some(input.supply_enthalpy_j_per_kg),
        enthalpy_difference_j_per_kg: Some(enthalpy_difference_j_per_kg),
        cooling_total_output_w: Some(cooling_total_output_w),
    })
}

fn next_transition_fits(
    state: &State,
    predecessor_route: PredecessorRoute,
    route: Route,
    assignment: bool,
) -> bool {
    state.transition_count.checked_add(1).is_some()
        && next_route_counters_fit(state, predecessor_route, route)
        && (!assignment
            || state
                .dehumidification_total_output_assignment_count
                .checked_add(1)
                .is_some()
                && state
                    .source_site_execution_count
                    .checked_add(
                        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE_ORDER.len(),
                    )
                    .is_some()
                && active_counters(state)
                    .into_iter()
                    .all(|count| count.checked_add(1).is_some()))
}

fn active_counters(state: &State) -> [usize; 14] {
    [
        state.cp330_supply_mass_flow_rate_owned_read_count,
        state.cp329_same_call_supply_mass_flow_rate_bit_corroboration_count,
        state.cp339_same_call_supply_mass_flow_rate_bit_corroboration_count,
        state.supply_mass_flow_rate_read_count,
        state.cp329_mixed_air_enthalpy_owned_read_count,
        state.cp329_same_call_recirculation_enthalpy_bit_corroboration_count,
        state.cp339_same_call_mixed_air_enthalpy_bit_corroboration_count,
        state.mixed_air_enthalpy_read_count,
        state.cp379_post_saturation_supply_enthalpy_owned_read_count,
        state.cp379_same_call_supply_enthalpy_bits_corroboration_count,
        state.supply_enthalpy_read_count,
        state.enthalpy_difference_calculation_count,
        state.cooling_total_output_calculation_count,
        state.cooling_total_output_assignment_write_count,
    ]
}

fn active_counters_mut(state: &mut State) -> [&mut usize; 14] {
    [
        &mut state.cp330_supply_mass_flow_rate_owned_read_count,
        &mut state.cp329_same_call_supply_mass_flow_rate_bit_corroboration_count,
        &mut state.cp339_same_call_supply_mass_flow_rate_bit_corroboration_count,
        &mut state.supply_mass_flow_rate_read_count,
        &mut state.cp329_mixed_air_enthalpy_owned_read_count,
        &mut state.cp329_same_call_recirculation_enthalpy_bit_corroboration_count,
        &mut state.cp339_same_call_mixed_air_enthalpy_bit_corroboration_count,
        &mut state.mixed_air_enthalpy_read_count,
        &mut state.cp379_post_saturation_supply_enthalpy_owned_read_count,
        &mut state.cp379_same_call_supply_enthalpy_bits_corroboration_count,
        &mut state.supply_enthalpy_read_count,
        &mut state.enthalpy_difference_calculation_count,
        &mut state.cooling_total_output_calculation_count,
        &mut state.cooling_total_output_assignment_write_count,
    ]
}
