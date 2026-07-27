//! Completed CP316 and pending CP317 retained-state invariants.

use ep_model::IdealLoadsAirSystemId;

use super::super::{
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE_ORDER,
    PurchasedAirCalcCoolingEconomizerBodyRetainedRoute,
    PurchasedAirCalcCoolingEconomizerBodyRuntimeState,
    PurchasedAirCalcCoolingEconomizerBodySnapshot,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE_ORDER,
    PurchasedAirCalcCoolingEconomizerConditionSnapshot, PurchasedAirUnitRuntimeState,
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
        && unit.calc_cooling_economizer_body.system == selected
}

pub(super) fn call_order_is_pending_body(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingEconomizerConditionSnapshot,
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
        && unit.calc_cooling_economizer_guard.transition_count
            == unit.calc_cooling_economizer_condition.transition_count
        && unit
            .calc_cooling_economizer_body
            .transition_count
            .checked_add(1)
            == Some(unit.calc_cooling_economizer_condition.transition_count)
        && predecessor.parent_call_ordinal
            == unit.calc_cooling_economizer_condition.transition_count
}

pub(super) fn economizer_condition_snapshot_is_exact_direct_release(
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
        && condition_sites_are_skipped(snapshot)
}

pub(super) fn pending_body_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingEconomizerConditionSnapshot,
    body_consumer_latest_witness: Option<PurchasedAirCalcCoolingEconomizerBodySnapshot>,
) -> bool {
    let state = &unit.calc_cooling_economizer_body;
    let condition = &unit.calc_cooling_economizer_condition;
    let transition_partition = state
        .body_execution_count
        .checked_add(state.unit_off_skip_count)
        .and_then(|count| count.checked_add(state.non_cooling_skip_count))
        .and_then(|count| count.checked_add(state.maximum_cooling_flow_body_sibling_skip_count))
        .and_then(|count| count.checked_add(state.no_economizer_outer_guard_fallthrough_skip_count))
        .and_then(|count| count.checked_add(state.economizer_condition_fallthrough_skip_count))
        == Some(state.transition_count);
    let latest_is_valid = match (
        state.transition_count,
        state.latest,
        state.latest_route,
        state.latest_transition_ordinal,
        body_consumer_latest_witness,
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
                && body_snapshot_is_exact_direct_release(latest)
                && body_snapshot_route(latest) == Some(retained_route)
        }
        _ => false,
    };
    let histories_link = state
        .unit_off_skip_count
        .checked_add(usize::from(predecessor.unit_off_skipped))
        == Some(condition.unit_off_skip_count)
        && state
            .non_cooling_skip_count
            .checked_add(usize::from(predecessor.non_cooling_skipped))
            == Some(condition.non_cooling_skip_count)
        && state
            .maximum_cooling_flow_body_sibling_skip_count
            .checked_add(usize::from(
                predecessor.maximum_cooling_flow_body_sibling_skipped,
            ))
            == Some(condition.maximum_cooling_flow_body_sibling_skip_count)
        && state
            .no_economizer_outer_guard_fallthrough_skip_count
            .checked_add(usize::from(
                predecessor.no_economizer_outer_guard_fallthrough_skipped,
            ))
            == Some(condition.no_economizer_outer_guard_fallthrough_skip_count)
        && state
            .body_execution_count
            .checked_add(usize::from(predecessor.economizer_calculation_body_entered))
            == Some(condition.economizer_calculation_body_entry_count)
        && state
            .economizer_condition_fallthrough_skip_count
            .checked_add(usize::from(predecessor.economizer_condition_fallthrough))
            .and_then(|fallthrough| {
                state
                    .body_execution_count
                    .checked_add(usize::from(predecessor.economizer_calculation_body_entered))
                    .and_then(|entered| entered.checked_add(fallthrough))
            })
            == Some(condition.condition_evaluation_count);

    transition_partition
        && latest_is_valid
        && histories_link
        && state.body_execution_count == 0
        && state.maximum_cooling_flow_body_sibling_skip_count == 0
        && state.economizer_condition_fallthrough_skip_count == 0
        && body_source_counters_are_zero(state)
}

fn condition_sites_are_skipped(
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

pub(in crate::ideal_loads) fn body_snapshot_is_exact_direct_release(
    snapshot: PurchasedAirCalcCoolingEconomizerBodySnapshot,
) -> bool {
    let provenance = snapshot.source == PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order == PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE_ORDER;
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
        && !snapshot.economizer_condition_fallthrough_skipped
        && !snapshot.predecessor_maximum_cooling_flow_body_entered
        && !snapshot.predecessor_economizer_body_entered
        && !snapshot.predecessor_economizer_condition_evaluated
        && snapshot
            .predecessor_economizer_condition_satisfied
            .is_none()
        && !snapshot.predecessor_economizer_calculation_body_entered
        && (unit_off || non_cooling || no_economizer)
        && body_sites_are_skipped(snapshot)
}

fn body_sites_are_skipped(snapshot: PurchasedAirCalcCoolingEconomizerBodySnapshot) -> bool {
    !snapshot.economizer_calculation_body_executed
        && !snapshot.zone_humidity_ratio_read
        && snapshot.zone_humidity_ratio.is_none()
        && !snapshot.psychrometric_cp_air_evaluated
        && snapshot.psychrometric_cp_air_result_j_per_kg_k.is_none()
        && !snapshot.cp_air_assigned
        && snapshot.cp_air_j_per_kg_k.is_none()
        && !snapshot.outdoor_air_temperature_read
        && snapshot.outdoor_air_temperature_c.is_none()
        && !snapshot.zone_temperature_read
        && snapshot.zone_temperature_c.is_none()
        && !snapshot.delta_temperature_calculated
        && snapshot.delta_temperature_c.is_none()
        && !snapshot.delta_temperature_assigned
        && snapshot.assigned_delta_temperature_c.is_none()
        && !snapshot.delta_temperature_for_gate_read
        && snapshot.delta_temperature_for_gate_c.is_none()
        && !snapshot.delta_temperature_comparison_evaluated
        && snapshot
            .delta_temperature_below_negative_small_temp_diff
            .is_none()
        && !snapshot.delta_temperature_body_entered
        && !snapshot.zone_cooling_setpoint_load_read
        && snapshot.zone_cooling_setpoint_load_w.is_none()
        && !snapshot.cp_air_for_first_division_read
        && snapshot.cp_air_for_first_division_j_per_kg_k.is_none()
        && !snapshot.zone_cooling_setpoint_load_over_cp_air_calculated
        && snapshot
            .zone_cooling_setpoint_load_over_cp_air_kg_k_per_s
            .is_none()
        && !snapshot.delta_temperature_for_second_division_read
        && snapshot.delta_temperature_for_second_division_c.is_none()
        && !snapshot.supply_mass_flow_rate_calculated
        && snapshot.calculated_supply_mass_flow_rate_kg_per_s.is_none()
        && !snapshot.initial_supply_mass_flow_rate_assigned
        && snapshot.initial_supply_mass_flow_rate_kg_per_s.is_none()
        && !snapshot.cooling_limit_flow_rate_comparison_evaluated
        && !snapshot.cooling_limit_flow_rate_read
        && snapshot.cooling_limit_flow_rate_value.is_none()
        && snapshot
            .cooling_limit_flow_rate_comparison_satisfied
            .is_none()
        && !snapshot.cooling_limit_flow_rate_and_capacity_comparison_evaluated
        && !snapshot.cooling_limit_flow_rate_and_capacity_read
        && snapshot
            .cooling_limit_flow_rate_and_capacity_value
            .is_none()
        && snapshot
            .cooling_limit_flow_rate_and_capacity_comparison_satisfied
            .is_none()
        && snapshot.cooling_flow_limit_active.is_none()
        && !snapshot.maximum_cooling_air_mass_flow_rate_read
        && snapshot
            .maximum_cooling_air_mass_flow_rate_kg_per_s
            .is_none()
        && !snapshot.maximum_cooling_air_mass_flow_rate_positive_comparison_evaluated
        && snapshot
            .maximum_cooling_air_mass_flow_rate_positive
            .is_none()
        && !snapshot.maximum_flow_clamp_body_entered
        && !snapshot.maximum_cooling_air_mass_flow_rate_clamp_upper_bound_read
        && snapshot
            .maximum_cooling_air_mass_flow_rate_clamp_upper_bound_kg_per_s
            .is_none()
        && !snapshot.supply_mass_flow_rate_for_clamp_read
        && snapshot.supply_mass_flow_rate_for_clamp_kg_per_s.is_none()
        && !snapshot.inner_max_evaluated
        && !snapshot.supply_mass_flow_rate_clamped
        && snapshot
            .nonnegative_supply_mass_flow_rate_kg_per_s
            .is_none()
        && !snapshot.outer_min_evaluated
        && snapshot.clamped_supply_mass_flow_rate_kg_per_s.is_none()
        && !snapshot.clamped_supply_mass_flow_rate_assigned
        && snapshot.resulting_supply_mass_flow_rate_kg_per_s.is_none()
        && !snapshot.resulting_supply_mass_flow_rate_read
        && !snapshot.outdoor_air_mass_flow_rate_read
        && snapshot.outdoor_air_mass_flow_rate_kg_per_s.is_none()
        && !snapshot.supply_above_outdoor_air_mass_flow_comparison_evaluated
        && snapshot
            .supply_mass_flow_above_outdoor_air_mass_flow
            .is_none()
        && !snapshot.economizer_activation_body_entered
        && !snapshot.economizer_on_assigned
        && snapshot.economizer_on.is_none()
        && !snapshot.supply_mass_flow_rate_for_outdoor_air_assignment_read
        && snapshot
            .supply_mass_flow_rate_for_outdoor_air_assignment_kg_per_s
            .is_none()
        && !snapshot.outdoor_air_mass_flow_rate_assigned
        && snapshot
            .assigned_outdoor_air_mass_flow_rate_kg_per_s
            .is_none()
        && !snapshot.system_time_step_read
        && snapshot.system_time_step_hours.is_none()
        && !snapshot.economizer_active_time_assigned
        && snapshot.assigned_economizer_active_time_hours.is_none()
}

pub(super) fn body_source_counters_are_zero(
    state: &PurchasedAirCalcCoolingEconomizerBodyRuntimeState,
) -> bool {
    state.zone_humidity_ratio_read_count == 0
        && state.psychrometric_cp_air_evaluation_count == 0
        && state.cp_air_assignment_count == 0
        && state.outdoor_air_temperature_read_count == 0
        && state.zone_temperature_read_count == 0
        && state.delta_temperature_calculation_count == 0
        && state.delta_temperature_assignment_count == 0
        && state.delta_temperature_for_gate_read_count == 0
        && state.delta_temperature_comparison_count == 0
        && state.delta_temperature_comparison_satisfied_count == 0
        && state.delta_temperature_body_entry_count == 0
        && state.delta_temperature_fallthrough_count == 0
        && state.zone_cooling_setpoint_load_read_count == 0
        && state.cp_air_for_first_division_read_count == 0
        && state.zone_cooling_setpoint_load_over_cp_air_calculation_count == 0
        && state.delta_temperature_for_second_division_read_count == 0
        && state.supply_mass_flow_rate_calculation_count == 0
        && state.initial_supply_mass_flow_rate_assignment_count == 0
        && state.cooling_limit_flow_rate_read_count == 0
        && state.cooling_limit_flow_rate_comparison_count == 0
        && state.cooling_limit_flow_rate_match_count == 0
        && state.cooling_limit_flow_rate_and_capacity_comparison_count == 0
        && state.cooling_limit_flow_rate_and_capacity_read_count == 0
        && state.cooling_limit_flow_rate_and_capacity_match_count == 0
        && state.maximum_cooling_air_mass_flow_rate_read_count == 0
        && state.maximum_cooling_air_mass_flow_rate_positive_comparison_count == 0
        && state.maximum_cooling_air_mass_flow_rate_positive_count == 0
        && state.maximum_flow_clamp_body_entry_count == 0
        && state.supply_mass_flow_rate_for_clamp_read_count == 0
        && state.inner_max_evaluation_count == 0
        && state.maximum_cooling_air_mass_flow_rate_clamp_upper_bound_read_count == 0
        && state.supply_mass_flow_rate_clamp_count == 0
        && state.outer_min_evaluation_count == 0
        && state.clamped_supply_mass_flow_rate_assignment_count == 0
        && state.resulting_supply_mass_flow_rate_read_count == 0
        && state.outdoor_air_mass_flow_rate_read_count == 0
        && state.supply_above_outdoor_air_mass_flow_comparison_count == 0
        && state.supply_above_outdoor_air_mass_flow_comparison_satisfied_count == 0
        && state.economizer_activation_body_entry_count == 0
        && state.outdoor_air_mass_flow_comparison_fallthrough_count == 0
        && state.economizer_on_assignment_count == 0
        && state.supply_mass_flow_rate_for_outdoor_air_assignment_read_count == 0
        && state.outdoor_air_mass_flow_rate_assignment_count == 0
        && state.system_time_step_read_count == 0
        && state.economizer_active_time_assignment_count == 0
}

pub(super) fn body_snapshot_route(
    latest: PurchasedAirCalcCoolingEconomizerBodySnapshot,
) -> Option<PurchasedAirCalcCoolingEconomizerBodyRetainedRoute> {
    match (
        latest.unit_off_skipped,
        latest.non_cooling_skipped,
        latest.maximum_cooling_flow_body_sibling_skipped,
        latest.no_economizer_outer_guard_fallthrough_skipped,
        latest.economizer_condition_fallthrough_skipped,
        latest.economizer_calculation_body_executed,
    ) {
        (true, false, false, false, false, false) => {
            Some(PurchasedAirCalcCoolingEconomizerBodyRetainedRoute::UnitOff)
        }
        (false, true, false, false, false, false) => {
            Some(PurchasedAirCalcCoolingEconomizerBodyRetainedRoute::NonCooling)
        }
        (false, false, true, false, false, false) => {
            Some(PurchasedAirCalcCoolingEconomizerBodyRetainedRoute::MaximumCoolingFlowBodySibling)
        }
        (false, false, false, true, false, false) => Some(
            PurchasedAirCalcCoolingEconomizerBodyRetainedRoute::NoEconomizerOuterGuardFallthrough,
        ),
        (false, false, false, false, true, false) => {
            Some(PurchasedAirCalcCoolingEconomizerBodyRetainedRoute::EconomizerConditionFallthrough)
        }
        (false, false, false, false, false, true) => {
            Some(PurchasedAirCalcCoolingEconomizerBodyRetainedRoute::Executed)
        }
        _ => None,
    }
}
