//! Completed CP313-through-CP315 and pending CP316 retained-state invariants.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId, IdealLoadsLimit};

use super::super::{
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE_ORDER,
    PurchasedAirCalcCoolingEconomizerConditionRetainedRoute,
    PurchasedAirCalcCoolingEconomizerConditionRuntimeState,
    PurchasedAirCalcCoolingEconomizerConditionSnapshot,
};
use super::predecessor_validation::{
    cooling_body_links_to_gate, cooling_body_snapshot_is_exact_direct_release,
    cooling_gate_snapshot_is_exact_direct_release, economizer_guard_links_to_body,
    economizer_guard_snapshot_is_exact_direct_release,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingEconomizerGuardRuntimeState,
    PurchasedAirCalcCoolingEconomizerGuardSnapshot,
    PurchasedAirCalcCoolingOaMaxFlowBodyRuntimeState,
    PurchasedAirCalcCoolingOaMaxFlowGateRuntimeState, PurchasedAirUnitRuntimeState,
};

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    selected: IdealLoadsAirSystemId,
) -> bool {
    unit.system == selected
        && unit.calc_entry.system == selected
        && unit.calc_minimum_oa_prefix.system == selected
        && unit.calc_cooling_entry_gate.system == selected
        && unit.calc_cooling_oa_max_flow_gate.system == selected
        && unit.calc_cooling_oa_max_flow_body.system == selected
        && unit.calc_cooling_economizer_guard.system == selected
        && unit.calc_cooling_economizer_condition.system == selected
}

pub(super) fn call_order_is_pending_condition(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingEconomizerGuardSnapshot,
) -> bool {
    unit.init_call_count == unit.calc_entry.call_count
        && unit.calc_entry.call_count == unit.calc_minimum_oa_prefix.transition_count
        && unit.calc_minimum_oa_prefix.transition_count
            == unit.calc_cooling_entry_gate.transition_count
        && unit.calc_cooling_entry_gate.transition_count
            == unit.calc_cooling_oa_max_flow_gate.transition_count
        && unit.calc_cooling_oa_max_flow_gate.transition_count
            == unit.calc_cooling_oa_max_flow_body.transition_count
        && unit.calc_cooling_oa_max_flow_body.transition_count
            == unit.calc_cooling_economizer_guard.transition_count
        && unit
            .calc_cooling_economizer_condition
            .transition_count
            .checked_add(1)
            == Some(unit.calc_cooling_economizer_guard.transition_count)
        && predecessor.parent_call_ordinal == unit.calc_cooling_economizer_guard.transition_count
}

pub(super) fn completed_cp313_through_cp315_prefix_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingEconomizerGuardSnapshot,
) -> bool {
    let (Some(gate_latest), Some(body_latest), Some(guard_latest)) = (
        unit.calc_cooling_oa_max_flow_gate.latest,
        unit.calc_cooling_oa_max_flow_body.latest,
        unit.calc_cooling_economizer_guard.latest,
    ) else {
        return false;
    };
    let count = unit.calc_cooling_economizer_guard.transition_count;
    let latest_chain = guard_latest == predecessor
        && unit.maximum_cooling_air_mass_flow_rate_kg_per_s.is_finite()
        && unit.maximum_cooling_air_mass_flow_rate_kg_per_s >= 0.0
        && gate_latest.parent_call_ordinal == count
        && body_latest.parent_call_ordinal == count
        && guard_latest.parent_call_ordinal == count
        && gate_latest.system == unit.system
        && body_latest.system == unit.system
        && guard_latest.system == unit.system
        && unit.controlled_zone == Some(gate_latest.controlled_zone)
        && gate_latest.controlled_zone == body_latest.controlled_zone
        && body_latest.controlled_zone == guard_latest.controlled_zone
        && cooling_gate_snapshot_is_exact_direct_release(
            gate_latest,
            system.cooling_limit,
            unit.maximum_cooling_air_mass_flow_rate_kg_per_s,
        )
        && cooling_body_snapshot_is_exact_direct_release(body_latest)
        && economizer_guard_snapshot_is_exact_direct_release(guard_latest)
        && cooling_body_links_to_gate(body_latest, gate_latest)
        && economizer_guard_links_to_body(guard_latest, body_latest);

    latest_chain
        && gate_history_is_consistent(&unit.calc_cooling_oa_max_flow_gate, system.cooling_limit)
        && body_history_is_consistent(
            &unit.calc_cooling_oa_max_flow_body,
            &unit.calc_cooling_oa_max_flow_gate,
        )
        && guard_history_is_consistent(
            &unit.calc_cooling_economizer_guard,
            &unit.calc_cooling_oa_max_flow_body,
        )
        && gate_latest_route_is_recorded(&unit.calc_cooling_oa_max_flow_gate, gate_latest)
        && body_latest_route_is_recorded(&unit.calc_cooling_oa_max_flow_body, body_latest)
        && guard_latest_route_is_recorded(&unit.calc_cooling_economizer_guard, guard_latest)
}

pub(super) fn pending_condition_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingEconomizerGuardSnapshot,
    condition_consumer_latest_witness: Option<PurchasedAirCalcCoolingEconomizerConditionSnapshot>,
) -> bool {
    let state = &unit.calc_cooling_economizer_condition;
    let guard = &unit.calc_cooling_economizer_guard;
    let transition_partition = state
        .condition_evaluation_count
        .checked_add(state.unit_off_skip_count)
        .and_then(|count| count.checked_add(state.non_cooling_skip_count))
        .and_then(|count| count.checked_add(state.maximum_cooling_flow_body_sibling_skip_count))
        .and_then(|count| {
            count.checked_add(state.no_economizer_outer_guard_fallthrough_skip_count)
        })
        == Some(state.transition_count);
    let latest_is_valid = match (
        state.transition_count,
        state.latest,
        state.latest_route,
        state.latest_transition_ordinal,
        condition_consumer_latest_witness,
    ) {
        (0, None, None, None, None) => true,
        (
            count,
            Some(latest),
            Some(retained_route),
            Some(latest_transition_ordinal),
            Some(consumer_witness),
        ) if count > 0 => {
            latest_transition_ordinal == count
                && consumer_witness == latest
                && latest.parent_call_ordinal == count
                && latest.system == state.system
                && unit.controlled_zone == Some(latest.controlled_zone)
                && condition_snapshot_is_exact_direct_release(latest)
                && condition_snapshot_route(latest) == Some(retained_route)
        }
        _ => false,
    };
    let histories_link = state
        .unit_off_skip_count
        .checked_add(usize::from(predecessor.unit_off_skipped))
        == Some(guard.unit_off_skip_count)
        && state
            .non_cooling_skip_count
            .checked_add(usize::from(predecessor.non_cooling_skipped))
            == Some(guard.non_cooling_skip_count)
        && state
            .maximum_cooling_flow_body_sibling_skip_count
            .checked_add(usize::from(
                predecessor.maximum_cooling_flow_body_sibling_skipped,
            ))
            == Some(guard.maximum_cooling_flow_body_sibling_skip_count)
        && state
            .no_economizer_outer_guard_fallthrough_skip_count
            .checked_add(usize::from(predecessor.no_economizer_fallthrough))
            == Some(guard.no_economizer_fallthrough_count)
        && state
            .condition_evaluation_count
            .checked_add(usize::from(predecessor.economizer_body_entered))
            == Some(guard.economizer_body_entry_count);

    transition_partition
        && latest_is_valid
        && histories_link
        && state.condition_evaluation_count == 0
        && state.maximum_cooling_flow_body_sibling_skip_count == 0
        && condition_source_counters_are_zero(state)
}

fn gate_history_is_consistent(
    gate: &PurchasedAirCalcCoolingOaMaxFlowGateRuntimeState,
    cooling_limit: IdealLoadsLimit,
) -> bool {
    let selector_history = match cooling_limit {
        IdealLoadsLimit::LimitFlowRate => {
            gate.cooling_limit_flow_rate_match_count == gate.source_execution_count
                && gate.cooling_limit_flow_rate_and_capacity_comparison_count == 0
                && gate.cooling_limit_flow_rate_and_capacity_match_count == 0
        }
        IdealLoadsLimit::LimitFlowRateAndCapacity => {
            gate.cooling_limit_flow_rate_match_count == 0
                && gate.cooling_limit_flow_rate_and_capacity_comparison_count
                    == gate.source_execution_count
                && gate.cooling_limit_flow_rate_and_capacity_match_count
                    == gate.source_execution_count
        }
        IdealLoadsLimit::NoLimit | IdealLoadsLimit::LimitCapacity => {
            gate.cooling_limit_flow_rate_match_count == 0
                && gate.cooling_limit_flow_rate_and_capacity_comparison_count
                    == gate.source_execution_count
                && gate.cooling_limit_flow_rate_and_capacity_match_count == 0
        }
    };
    let expected_flow_reads = usize::from(matches!(
        cooling_limit,
        IdealLoadsLimit::LimitFlowRate | IdealLoadsLimit::LimitFlowRateAndCapacity
    )) * gate.source_execution_count;
    selector_history
        && gate
            .source_execution_count
            .checked_add(gate.unit_off_skip_count)
            .and_then(|count| count.checked_add(gate.non_cooling_skip_count))
            == Some(gate.transition_count)
        && gate.cooling_limit_flow_rate_comparison_count == gate.source_execution_count
        && gate
            .source_execution_count
            .checked_sub(gate.cooling_limit_flow_rate_match_count)
            == Some(gate.cooling_limit_flow_rate_and_capacity_comparison_count)
        && gate
            .cooling_limit_flow_rate_match_count
            .checked_add(gate.cooling_limit_flow_rate_and_capacity_match_count)
            == Some(gate.outdoor_air_mass_flow_rate_read_count)
        && gate.outdoor_air_mass_flow_rate_read_count == expected_flow_reads
        && gate.outdoor_air_mass_flow_rate_read_count
            == gate.maximum_cooling_air_mass_flow_rate_read_count
        && gate.maximum_cooling_air_mass_flow_rate_read_count
            == gate.strict_mass_flow_comparison_count
        && gate.strict_mass_flow_comparison_satisfied_count == 0
        && gate.maximum_cooling_flow_body_entry_count == 0
        && gate.active_fallthrough_count == gate.source_execution_count
}

fn body_history_is_consistent(
    body: &PurchasedAirCalcCoolingOaMaxFlowBodyRuntimeState,
    gate: &PurchasedAirCalcCoolingOaMaxFlowGateRuntimeState,
) -> bool {
    let skip_partition = body
        .unit_off_skip_count
        .checked_add(body.non_cooling_skip_count)
        .and_then(|count| count.checked_add(body.active_guard_false_economizer_fallthrough_count))
        == Some(body.body_skip_count);
    body.transition_count == gate.transition_count
        && body.transition_count == body.body_skip_count
        && body.body_entry_count == 0
        && skip_partition
        && body.unit_off_skip_count == gate.unit_off_skip_count
        && body.non_cooling_skip_count == gate.non_cooling_skip_count
        && body.active_guard_false_economizer_fallthrough_count == gate.source_execution_count
        && body_effect_counters_are_zero(body)
}

fn guard_history_is_consistent(
    guard: &PurchasedAirCalcCoolingEconomizerGuardRuntimeState,
    body: &PurchasedAirCalcCoolingOaMaxFlowBodyRuntimeState,
) -> bool {
    let partition = guard
        .guard_evaluation_count
        .checked_add(guard.unit_off_skip_count)
        .and_then(|count| count.checked_add(guard.non_cooling_skip_count))
        .and_then(|count| count.checked_add(guard.maximum_cooling_flow_body_sibling_skip_count))
        == Some(guard.transition_count);
    guard.transition_count == body.transition_count
        && partition
        && guard.unit_off_skip_count == body.unit_off_skip_count
        && guard.non_cooling_skip_count == body.non_cooling_skip_count
        && guard.maximum_cooling_flow_body_sibling_skip_count == body.body_entry_count
        && guard.guard_evaluation_count == body.active_guard_false_economizer_fallthrough_count
        && guard.guard_evaluation_count == guard.economizer_type_read_count
        && guard.economizer_type_read_count == guard.no_economizer_comparison_count
        && guard.economizer_body_entry_count == 0
        && guard.no_economizer_fallthrough_count == guard.guard_evaluation_count
}

pub(super) fn condition_snapshot_is_exact_direct_release(
    snapshot: PurchasedAirCalcCoolingEconomizerConditionSnapshot,
) -> bool {
    let provenance = snapshot.source == PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order == PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE_ORDER;
    let unit_off = snapshot.unit_off_skipped
        && !snapshot.unit_body_entered
        && !snapshot.predecessor_cooling_body_entered
        && !snapshot.predecessor_active_guard_false_economizer_fallthrough
        && !snapshot.predecessor_economizer_guard_evaluated
        && !snapshot.predecessor_no_economizer_fallthrough;
    let non_cooling = snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && !snapshot.predecessor_cooling_body_entered
        && !snapshot.predecessor_active_guard_false_economizer_fallthrough
        && !snapshot.predecessor_economizer_guard_evaluated
        && !snapshot.predecessor_no_economizer_fallthrough;
    let no_economizer = snapshot.no_economizer_outer_guard_fallthrough_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.predecessor_active_guard_false_economizer_fallthrough
        && snapshot.predecessor_economizer_guard_evaluated
        && snapshot.predecessor_no_economizer_fallthrough;
    provenance
        && usize::from(snapshot.unit_off_skipped)
            + usize::from(snapshot.non_cooling_skipped)
            + usize::from(snapshot.no_economizer_outer_guard_fallthrough_skipped)
            == 1
        && !snapshot.maximum_cooling_flow_body_sibling_skipped
        && !snapshot.predecessor_maximum_cooling_flow_body_entered
        && !snapshot.predecessor_economizer_body_entered
        && (unit_off || non_cooling || no_economizer)
        && condition_snapshot_sites_are_skipped(snapshot)
}

fn condition_snapshot_sites_are_skipped(
    snapshot: PurchasedAirCalcCoolingEconomizerConditionSnapshot,
) -> bool {
    !snapshot.economizer_condition_evaluated
        && !snapshot.differential_dry_bulb_economizer_type_read
        && snapshot.differential_dry_bulb_economizer_type.is_none()
        && !snapshot.differential_dry_bulb_selector_comparison_evaluated
        && snapshot.differential_dry_bulb_selector_matched.is_none()
        && !snapshot.outdoor_air_temperature_read
        && snapshot.outdoor_air_temperature_c.is_none()
        && !snapshot.recirculation_air_temperature_read
        && snapshot.recirculation_air_temperature_c.is_none()
        && !snapshot.dry_bulb_temperature_comparison_evaluated
        && snapshot
            .outdoor_air_temperature_below_recirculation_temperature
            .is_none()
        && !snapshot.differential_enthalpy_economizer_type_read
        && snapshot.differential_enthalpy_economizer_type.is_none()
        && !snapshot.differential_enthalpy_selector_comparison_evaluated
        && snapshot.differential_enthalpy_selector_matched.is_none()
        && !snapshot.outdoor_air_enthalpy_read
        && snapshot.outdoor_air_enthalpy_j_per_kg.is_none()
        && !snapshot.recirculation_air_enthalpy_read
        && snapshot.recirculation_air_enthalpy_j_per_kg.is_none()
        && !snapshot.enthalpy_comparison_evaluated
        && snapshot
            .outdoor_air_enthalpy_below_recirculation_enthalpy
            .is_none()
        && snapshot.economizer_condition_satisfied.is_none()
        && !snapshot.economizer_calculation_body_entered
        && !snapshot.economizer_condition_fallthrough
}

pub(super) fn condition_source_counters_are_zero(
    state: &PurchasedAirCalcCoolingEconomizerConditionRuntimeState,
) -> bool {
    state.differential_dry_bulb_economizer_type_read_count == 0
        && state.differential_dry_bulb_selector_comparison_count == 0
        && state.differential_dry_bulb_selector_match_count == 0
        && state.outdoor_air_temperature_read_count == 0
        && state.recirculation_air_temperature_read_count == 0
        && state.dry_bulb_temperature_comparison_count == 0
        && state.dry_bulb_temperature_comparison_satisfied_count == 0
        && state.differential_enthalpy_economizer_type_read_count == 0
        && state.differential_enthalpy_selector_comparison_count == 0
        && state.differential_enthalpy_selector_match_count == 0
        && state.outdoor_air_enthalpy_read_count == 0
        && state.recirculation_air_enthalpy_read_count == 0
        && state.enthalpy_comparison_count == 0
        && state.enthalpy_comparison_satisfied_count == 0
        && state.economizer_calculation_body_entry_count == 0
        && state.economizer_condition_fallthrough_count == 0
}

fn body_effect_counters_are_zero(body: &PurchasedAirCalcCoolingOaMaxFlowBodyRuntimeState) -> bool {
    body.outdoor_air_mass_flow_rate_read_count == 0
        && body.standard_air_density_read_count == 0
        && body.outdoor_air_volume_flow_calculation_count == 0
        && body.warning_counter_read_count == 0
        && body.outdoor_air_flow_max_cooling_output_error_count == 0
        && body.first_warning_branch_count == 0
        && body.warning_counter_increment_count == 0
        && body.first_warning_call_site_count == 0
        && body.maximum_cooling_air_volume_flow_rate_read_count == 0
        && body.continue_warning_call_site_count == 0
        && body.continue_warning_timestamp_call_site_count == 0
        && body.recurring_warning_branch_count == 0
        && body.recurring_warning_call_site_count == 0
        && body.characterized_recurring_warning_index_allocation_count == 0
        && body.characterized_recurring_warning_index_reuse_count == 0
        && body.characterized_recurring_warning_occurrence_count == 0
        && !body.characterized_recurring_warning_index_allocated
        && body.outdoor_air_flow_max_cooling_output_index == 0
        && body
            .characterized_recurring_warning_report_maximum_m3_per_s
            .is_none()
        && body.characterized_total_warning_error_increment_count == 0
        && body.maximum_cooling_air_mass_flow_rate_read_count == 0
        && body.outdoor_air_mass_flow_clamp_assignment_count == 0
}

fn gate_latest_route_is_recorded(
    state: &PurchasedAirCalcCoolingOaMaxFlowGateRuntimeState,
    latest: crate::ideal_loads::PurchasedAirCalcCoolingOaMaxFlowGateSnapshot,
) -> bool {
    if latest.unit_off_skipped {
        state.unit_off_skip_count > 0
    } else if latest.non_cooling_skipped {
        state.non_cooling_skip_count > 0
    } else {
        latest.predecessor_cooling_body_entered && state.source_execution_count > 0
    }
}

fn body_latest_route_is_recorded(
    state: &PurchasedAirCalcCoolingOaMaxFlowBodyRuntimeState,
    latest: crate::ideal_loads::PurchasedAirCalcCoolingOaMaxFlowBodySnapshot,
) -> bool {
    if latest.unit_off_skipped {
        state.unit_off_skip_count > 0
    } else if latest.non_cooling_skipped {
        state.non_cooling_skip_count > 0
    } else {
        latest.active_guard_false_economizer_fallthrough
            && state.active_guard_false_economizer_fallthrough_count > 0
    }
}

fn guard_latest_route_is_recorded(
    state: &PurchasedAirCalcCoolingEconomizerGuardRuntimeState,
    latest: PurchasedAirCalcCoolingEconomizerGuardSnapshot,
) -> bool {
    if latest.economizer_guard_evaluated {
        state.guard_evaluation_count > 0
    } else if latest.unit_off_skipped {
        state.unit_off_skip_count > 0
    } else {
        latest.non_cooling_skipped && state.non_cooling_skip_count > 0
    }
}

pub(super) fn condition_snapshot_route(
    latest: PurchasedAirCalcCoolingEconomizerConditionSnapshot,
) -> Option<PurchasedAirCalcCoolingEconomizerConditionRetainedRoute> {
    match (
        latest.unit_off_skipped,
        latest.non_cooling_skipped,
        latest.maximum_cooling_flow_body_sibling_skipped,
        latest.no_economizer_outer_guard_fallthrough_skipped,
        latest.economizer_condition_evaluated,
    ) {
        (true, false, false, false, false) => {
            Some(PurchasedAirCalcCoolingEconomizerConditionRetainedRoute::UnitOff)
        }
        (false, true, false, false, false) => {
            Some(PurchasedAirCalcCoolingEconomizerConditionRetainedRoute::NonCooling)
        }
        (false, false, true, false, false) => Some(
            PurchasedAirCalcCoolingEconomizerConditionRetainedRoute::
                MaximumCoolingFlowBodySibling,
        ),
        (false, false, false, true, false) => Some(
            PurchasedAirCalcCoolingEconomizerConditionRetainedRoute::
                NoEconomizerOuterGuardFallthrough,
        ),
        (false, false, false, false, true) => {
            Some(PurchasedAirCalcCoolingEconomizerConditionRetainedRoute::Evaluated)
        }
        _ => None,
    }
}
