//! Persistent CP382 runtime-state validation.

use ep_model::IdealLoadsAirSystemId;

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_state,
};
use super::snapshot_validation::{
    cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_snapshot_is_exact_direct_release,
    snapshot_route, snapshots_match_bit_exact,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment::transition::{
    PredecessorRoute, predecessor_route,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardSnapshot as Predecessor,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> bool {
    unit.system == system
        && unit.calc_entry.system == system
        && unit.calc_cooling_mixed_air_call.system == system
        && unit.calc_cooling_supply_mass_flow_positive_guard.system == system
        && unit
            .calc_cooling_positive_supply_capacity_limit_sensible_output_assignment
            .system
            == system
        && unit
            .calc_cooling_supply_enthalpy_post_saturation_assignment
            .system
            == system
        && unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_guard
            .system
            == system
        && unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment
            .system
            == system
}

pub(super) fn call_order_is_pending(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
) -> bool {
    let ordinal = predecessor.parent_call_ordinal;
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment
        .transition_count
        .checked_add(1)
        == Some(ordinal)
        && unit.calc_entry.call_count == ordinal
        && unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_guard
            .transition_count
            == ordinal
        && unit
            .calc_cooling_supply_enthalpy_post_saturation_assignment
            .transition_count
            == ordinal
        && unit.calc_cooling_mixed_air_call.transition_count == ordinal
        && unit
            .calc_cooling_supply_mass_flow_positive_guard
            .transition_count
            == ordinal
        && unit
            .calc_cooling_positive_supply_capacity_limit_sensible_output_assignment
            .transition_count
            == ordinal
}

pub(super) fn pending_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
    witness: Option<Snapshot>,
) -> bool {
    let state =
        &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment;
    let Some(route) = predecessor_route(predecessor) else {
        return false;
    };
    state_is_consistent(state, witness, predecessor.system)
        && state.transition_count.checked_add(1) == Some(predecessor.parent_call_ordinal)
        && state_is_one_transition_behind_predecessor(
            state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard,
            route,
        )
}

pub(super) fn prepare_next_transition(
    state: &State,
    predecessor: Predecessor,
    input: Option<ActiveInput>,
) -> Option<(State, Snapshot)> {
    let mut next = state.clone();
    let snapshot = advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_state(
        &mut next,
        predecessor,
        input,
    )?;
    Some((next, snapshot))
}

pub(super) fn prepared_completed_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    state: &State,
    snapshot: Snapshot,
) -> bool {
    completed_state_parts(unit, state, snapshot, Some(snapshot))
}

pub(in crate::ideal_loads::calc) fn completed_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    completed_state_parts(
        unit,
        &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment,
        snapshot,
        witness,
    )
}

pub(in crate::ideal_loads::calc) fn committed_latest_snapshot_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
    snapshot: Snapshot,
    witness: Snapshot,
) -> bool {
    unit.system == system
        && unit.calc_entry.system == system
        && unit.init_call_count != 0
        && unit.init_call_count == unit.calc_entry.call_count
        && snapshot.system == system
        && snapshot.parent_call_ordinal == unit.init_call_count
        && unit.controlled_zone == Some(snapshot.controlled_zone)
        && snapshots_match_bit_exact(snapshot, witness)
        && completed_state_is_consistent(unit, snapshot, Some(witness))
        && cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_snapshot_is_exact_direct_release(snapshot)
}

fn completed_state_parts(
    unit: &PurchasedAirUnitRuntimeState,
    state: &State,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let prior = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard;
    state_is_consistent(state, witness, snapshot.system)
        && state.transition_count == prior.transition_count
        && predecessor_counts_match(state, prior)
        && state
            .latest
            .is_some_and(|latest| snapshots_match_bit_exact(latest, snapshot))
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_latest_metadata_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    expected_transition_count: usize,
) -> bool {
    let state =
        &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment;
    state.transition_count == expected_transition_count
        && state_is_consistent(state, state.latest, state.system)
        && predecessor_counts_match(
            state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard,
        )
}

fn state_is_consistent(
    state: &State,
    witness: Option<Snapshot>,
    system: IdealLoadsAirSystemId,
) -> bool {
    let inherited = inherited_counts(state);
    let partitions = predecessor_partition_counts(state);
    let assignments = assignment_route_counts(state);
    let Some(inherited_total) = checked_sum(&inherited) else {
        return false;
    };
    for index in 0..5 {
        let offset = index * 3;
        let Some(partition) = checked_sum(&partitions[offset..offset + 3]) else {
            return false;
        };
        if inherited[index + 3] != partition || partitions[offset + 1] != assignments[index] {
            return false;
        }
    }
    let Some(assignment_total) = checked_sum(&assignments) else {
        return false;
    };
    let Some(expected_sites) = assignment_total.checked_mul(6) else {
        return false;
    };
    let counters_match = state.system == system
        && state.transition_count == inherited_total
        && state.dehumidification_total_output_assignment_count == assignment_total
        && state.source_site_execution_count == expected_sites
        && active_counters(state)
            .into_iter()
            .all(|count| count == assignment_total);
    counters_match && latest_metadata_is_consistent(state, witness)
}

fn latest_metadata_is_consistent(state: &State, witness: Option<Snapshot>) -> bool {
    if state.transition_count == 0 {
        return state.latest.is_none()
            && state.latest_route.is_none()
            && state.latest_transition_ordinal.is_none()
            && witness.is_none();
    }
    let (Some(latest), Some(route), Some(ordinal), Some(witness)) = (
        state.latest,
        state.latest_route,
        state.latest_transition_ordinal,
        witness,
    ) else {
        return false;
    };
    ordinal == state.transition_count
        && latest.parent_call_ordinal == state.transition_count
        && latest.system == state.system
        && snapshot_route(latest) == Some(route)
        && cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_snapshot_is_exact_direct_release(latest)
        && snapshots_match_bit_exact(latest, witness)
        && route_count(state, route) > 0
}

fn predecessor_counts_match(state: &State, prior: &PredecessorState) -> bool {
    inherited_counts(state) == prior_inherited_counts(prior)
        && predecessor_partition_counts(state) == prior_partition_counts(prior)
}

fn state_is_one_transition_behind_predecessor(
    state: &State,
    prior: &PredecessorState,
    route: PredecessorRoute,
) -> bool {
    let ours_inherited = inherited_counts(state);
    let theirs_inherited = prior_inherited_counts(prior);
    let ours_partitions = predecessor_partition_counts(state);
    let theirs_partitions = prior_partition_counts(prior);
    ours_inherited
        .into_iter()
        .enumerate()
        .all(|(index, count)| {
            count.checked_add(usize::from(inherited_index(route) == index))
                == Some(theirs_inherited[index])
        })
        && ours_partitions
            .into_iter()
            .enumerate()
            .all(|(index, count)| {
                count.checked_add(usize::from(partition_index(route) == Some(index)))
                    == Some(theirs_partitions[index])
            })
}

fn inherited_index(route: PredecessorRoute) -> usize {
    use PredecessorRoute as P;
    match route {
        P::UnitOff => 0,
        P::NonCooling => 1,
        P::PositiveGuardFalseFallthrough => 2,
        P::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
        | P::HeatingAvailabilityGuardFalseFallthroughDehumidificationBodyEntered
        | P::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough => 3,
        P::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
        | P::HumidificationControlGuardFalseFallthroughDehumidificationBodyEntered
        | P::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => 4,
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough
        | P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationBodyEntered
        | P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => 5,
        P::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough
        | P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationBodyEntered
        | P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => 6,
        P::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
        | P::DehumidificationControlGuardFalseFallthroughDehumidificationBodyEntered
        | P::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => 7,
    }
}

fn partition_index(route: PredecessorRoute) -> Option<usize> {
    use PredecessorRoute as P;
    Some(match route {
        P::UnitOff | P::NonCooling | P::PositiveGuardFalseFallthrough => return None,
        P::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => 0,
        P::HeatingAvailabilityGuardFalseFallthroughDehumidificationBodyEntered => 1,
        P::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough => 2,
        P::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => 3,
        P::HumidificationControlGuardFalseFallthroughDehumidificationBodyEntered => 4,
        P::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => 5,
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough => 6,
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationBodyEntered => 7,
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => 8,
        P::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough => 9,
        P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationBodyEntered => 10,
        P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => 11,
        P::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => 12,
        P::DehumidificationControlGuardFalseFallthroughDehumidificationBodyEntered => 13,
        P::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => 14,
    })
}

fn inherited_counts(state: &State) -> [usize; 8] {
    [
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.heating_availability_guard_false_fallthrough_count,
        state.humidification_control_guard_false_fallthrough_count,
        state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_guard_false_fallthrough_count,
    ]
}

fn prior_inherited_counts(state: &PredecessorState) -> [usize; 8] {
    [
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.heating_availability_guard_false_fallthrough_count,
        state.humidification_control_guard_false_fallthrough_count,
        state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_guard_false_fallthrough_count,
    ]
}

fn predecessor_partition_counts(state: &State) -> [usize; 15] {
    [
        state.heating_availability_guard_false_fallthrough_capacity_guard_false_count,
        state.heating_availability_guard_false_fallthrough_dehumidification_body_entry_count,
        state.heating_availability_guard_false_fallthrough_dehumidification_guard_false_count,
        state.humidification_control_guard_false_fallthrough_capacity_guard_false_count,
        state.humidification_control_guard_false_fallthrough_dehumidification_body_entry_count,
        state.humidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
        state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count,
        state.dehumidification_control_humidistat_maximum_assignment_dehumidification_body_entry_count,
        state.dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count,
        state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count,
        state.dehumidification_control_none_maximum_assignment_dehumidification_body_entry_count,
        state.dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count,
        state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count,
        state.dehumidification_control_guard_false_fallthrough_dehumidification_body_entry_count,
        state.dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
    ]
}

fn prior_partition_counts(state: &PredecessorState) -> [usize; 15] {
    [
        state.heating_availability_guard_false_fallthrough_capacity_guard_false_count,
        state.heating_availability_guard_false_fallthrough_dehumidification_body_entry_count,
        state.heating_availability_guard_false_fallthrough_dehumidification_guard_false_count,
        state.humidification_control_guard_false_fallthrough_capacity_guard_false_count,
        state.humidification_control_guard_false_fallthrough_dehumidification_body_entry_count,
        state.humidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
        state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count,
        state.dehumidification_control_humidistat_maximum_assignment_dehumidification_body_entry_count,
        state.dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count,
        state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count,
        state.dehumidification_control_none_maximum_assignment_dehumidification_body_entry_count,
        state.dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count,
        state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count,
        state.dehumidification_control_guard_false_fallthrough_dehumidification_body_entry_count,
        state.dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
    ]
}

fn assignment_route_counts(state: &State) -> [usize; 5] {
    [
        state.heating_availability_guard_false_fallthrough_dehumidification_total_output_assignment_count,
        state.humidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count,
        state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_assignment_count,
        state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_assignment_count,
        state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count,
    ]
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

fn route_count(state: &State, route: Route) -> usize {
    use Route as R;
    match route {
        R::UnitOff => state.unit_off_skip_count,
        R::NonCooling => state.non_cooling_skip_count,
        R::PositiveGuardFalseFallthrough => state.positive_guard_false_fallthrough_skip_count,
        R::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => state.heating_availability_guard_false_fallthrough_capacity_guard_false_count,
        R::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputAssigned => state.heating_availability_guard_false_fallthrough_dehumidification_total_output_assignment_count,
        R::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough => state.heating_availability_guard_false_fallthrough_dehumidification_guard_false_count,
        R::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => state.humidification_control_guard_false_fallthrough_capacity_guard_false_count,
        R::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputAssigned => state.humidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count,
        R::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => state.humidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
        R::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough => state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count,
        R::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputAssigned => state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_assignment_count,
        R::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => state.dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count,
        R::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough => state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count,
        R::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputAssigned => state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_assignment_count,
        R::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => state.dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count,
        R::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count,
        R::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputAssigned => state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count,
        R::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => state.dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
    }
}

fn checked_sum(values: &[usize]) -> Option<usize> {
    values
        .iter()
        .try_fold(0usize, |sum, value| sum.checked_add(*value))
}
