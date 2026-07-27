//! Exact completed CP313-through-CP315 predecessor snapshot validation.

use ep_model::{IdealLoadsLimit, OutdoorAirEconomizerType};

use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_RECURRING_WARNING_CHILD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE,
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE_ORDER,
    PurchasedAirCalcCoolingEconomizerGuardSnapshot, PurchasedAirCalcCoolingOaMaxFlowBodySnapshot,
    PurchasedAirCalcCoolingOaMaxFlowGateSnapshot,
};

pub(super) fn economizer_guard_links_to_body(
    guard: PurchasedAirCalcCoolingEconomizerGuardSnapshot,
    body: PurchasedAirCalcCoolingOaMaxFlowBodySnapshot,
) -> bool {
    guard.system == body.system
        && guard.parent_call_ordinal == body.parent_call_ordinal
        && guard.controlled_zone == body.controlled_zone
        && guard.unit_body_entered == body.unit_body_entered
        && guard.predecessor_cooling_body_entered == body.predecessor_cooling_body_entered
        && guard.predecessor_maximum_cooling_flow_body_entered
            == body.predecessor_maximum_cooling_flow_body_entered
        && guard.predecessor_active_guard_false_economizer_fallthrough
            == body.active_guard_false_economizer_fallthrough
        && guard.unit_off_skipped == body.unit_off_skipped
        && guard.non_cooling_skipped == body.non_cooling_skipped
        && guard.maximum_cooling_flow_body_sibling_skipped
            == body.predecessor_maximum_cooling_flow_body_entered
        && guard.economizer_guard_evaluated == body.active_guard_false_economizer_fallthrough
}

pub(super) fn economizer_guard_snapshot_is_exact_direct_release(
    snapshot: PurchasedAirCalcCoolingEconomizerGuardSnapshot,
) -> bool {
    let provenance = snapshot.source == PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order == PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE_ORDER;
    if !provenance || snapshot.maximum_cooling_flow_body_sibling_skipped {
        return false;
    }
    if snapshot.economizer_guard_evaluated {
        snapshot.unit_body_entered
            && snapshot.predecessor_cooling_body_entered
            && !snapshot.predecessor_maximum_cooling_flow_body_entered
            && snapshot.predecessor_active_guard_false_economizer_fallthrough
            && !snapshot.unit_off_skipped
            && !snapshot.non_cooling_skipped
            && snapshot.economizer_type_read
            && snapshot.economizer_type == Some(OutdoorAirEconomizerType::NoEconomizer)
            && snapshot.no_economizer_comparison_evaluated
            && snapshot.economizer_not_no_economizer == Some(false)
            && !snapshot.economizer_body_entered
            && snapshot.no_economizer_fallthrough
    } else {
        let unit_off = snapshot.unit_off_skipped
            && !snapshot.unit_body_entered
            && !snapshot.predecessor_cooling_body_entered;
        let non_cooling = snapshot.non_cooling_skipped
            && snapshot.unit_body_entered
            && !snapshot.predecessor_cooling_body_entered;
        usize::from(snapshot.unit_off_skipped) + usize::from(snapshot.non_cooling_skipped) == 1
            && (unit_off || non_cooling)
            && !snapshot.predecessor_maximum_cooling_flow_body_entered
            && !snapshot.predecessor_active_guard_false_economizer_fallthrough
            && !snapshot.economizer_type_read
            && snapshot.economizer_type.is_none()
            && !snapshot.no_economizer_comparison_evaluated
            && snapshot.economizer_not_no_economizer.is_none()
            && !snapshot.economizer_body_entered
            && !snapshot.no_economizer_fallthrough
    }
}

pub(super) fn cooling_body_links_to_gate(
    body: PurchasedAirCalcCoolingOaMaxFlowBodySnapshot,
    gate: PurchasedAirCalcCoolingOaMaxFlowGateSnapshot,
) -> bool {
    body.system == gate.system
        && body.parent_call_ordinal == gate.parent_call_ordinal
        && body.controlled_zone == gate.controlled_zone
        && body.unit_body_entered == gate.unit_body_entered
        && body.predecessor_cooling_body_entered == gate.predecessor_cooling_body_entered
        && body.predecessor_maximum_cooling_flow_body_entered
            == gate.maximum_cooling_flow_body_entered
        && body.body_skipped != gate.maximum_cooling_flow_body_entered
        && body.unit_off_skipped == gate.unit_off_skipped
        && body.non_cooling_skipped == gate.non_cooling_skipped
        && body.active_guard_false_economizer_fallthrough
            == (gate.predecessor_cooling_body_entered && !gate.maximum_cooling_flow_body_entered)
}

pub(super) fn cooling_body_snapshot_is_exact_direct_release(
    snapshot: PurchasedAirCalcCoolingOaMaxFlowBodySnapshot,
) -> bool {
    let provenance = snapshot.source == PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_FIRST_EXCLUDED_SOURCE
        && snapshot.recurring_warning_child_source
            == PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_RECURRING_WARNING_CHILD_SOURCE
        && snapshot.source_order == PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE_ORDER;
    if !provenance
        || !snapshot.body_skipped
        || snapshot.predecessor_maximum_cooling_flow_body_entered
        || !body_sites_are_skipped(snapshot)
    {
        return false;
    }
    match (
        snapshot.unit_off_skipped,
        snapshot.non_cooling_skipped,
        snapshot.active_guard_false_economizer_fallthrough,
    ) {
        (true, false, false) => {
            !snapshot.unit_body_entered && !snapshot.predecessor_cooling_body_entered
        }
        (false, true, false) => {
            snapshot.unit_body_entered && !snapshot.predecessor_cooling_body_entered
        }
        (false, false, true) => {
            snapshot.unit_body_entered && snapshot.predecessor_cooling_body_entered
        }
        _ => false,
    }
}

pub(super) fn cooling_gate_snapshot_is_exact_direct_release(
    snapshot: PurchasedAirCalcCoolingOaMaxFlowGateSnapshot,
    cooling_limit: IdealLoadsLimit,
    retained_maximum_mass_flow_rate_kg_per_s: f64,
) -> bool {
    let provenance = snapshot.source == PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order == PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE_ORDER;
    if !provenance || snapshot.maximum_cooling_flow_body_entered {
        return false;
    }
    if !snapshot.predecessor_cooling_body_entered {
        return snapshot.unit_off_skipped != snapshot.unit_body_entered
            && snapshot.non_cooling_skipped == snapshot.unit_body_entered
            && gate_sites_are_skipped(snapshot);
    }
    if snapshot.unit_off_skipped
        || snapshot.non_cooling_skipped
        || !snapshot.unit_body_entered
        || !snapshot.cooling_limit_flow_rate_comparison_evaluated
        || !snapshot.cooling_limit_flow_rate_read
        || snapshot.cooling_limit_flow_rate_value != Some(cooling_limit)
    {
        return false;
    }
    let first_match = cooling_limit == IdealLoadsLimit::LimitFlowRate;
    let second_match = cooling_limit == IdealLoadsLimit::LimitFlowRateAndCapacity;
    let flow_active = first_match || second_match;
    let selectors_match = snapshot.cooling_limit_flow_rate_comparison_satisfied
        == Some(first_match)
        && snapshot.cooling_limit_flow_rate_and_capacity_comparison_evaluated != first_match
        && snapshot.cooling_limit_flow_rate_and_capacity_read != first_match
        && snapshot.cooling_limit_flow_rate_and_capacity_value
            == (!first_match).then_some(cooling_limit)
        && snapshot.cooling_limit_flow_rate_and_capacity_comparison_satisfied
            == (!first_match).then_some(second_match)
        && snapshot.cooling_flow_limit_active == Some(flow_active);
    if !selectors_match {
        return false;
    }
    if flow_active {
        snapshot.outdoor_air_mass_flow_rate_read
            && option_f64_has_bits(snapshot.outdoor_air_mass_flow_rate_kg_per_s, 0.0)
            && snapshot.maximum_cooling_air_mass_flow_rate_read
            && option_f64_has_bits(
                snapshot.maximum_cooling_air_mass_flow_rate_kg_per_s,
                retained_maximum_mass_flow_rate_kg_per_s,
            )
            && snapshot.strict_mass_flow_comparison_evaluated
            && snapshot.outdoor_air_mass_flow_above_maximum == Some(false)
    } else {
        !snapshot.outdoor_air_mass_flow_rate_read
            && snapshot.outdoor_air_mass_flow_rate_kg_per_s.is_none()
            && !snapshot.maximum_cooling_air_mass_flow_rate_read
            && snapshot
                .maximum_cooling_air_mass_flow_rate_kg_per_s
                .is_none()
            && !snapshot.strict_mass_flow_comparison_evaluated
            && snapshot.outdoor_air_mass_flow_above_maximum.is_none()
    }
}

fn body_sites_are_skipped(snapshot: PurchasedAirCalcCoolingOaMaxFlowBodySnapshot) -> bool {
    !snapshot.outdoor_air_mass_flow_rate_read
        && snapshot
            .outdoor_air_mass_flow_rate_before_clamp_kg_per_s
            .is_none()
        && !snapshot.standard_air_density_read
        && snapshot.standard_air_density_kg_per_m3.is_none()
        && !snapshot.outdoor_air_volume_flow_rate_calculated
        && snapshot.outdoor_air_volume_flow_rate_m3_per_s.is_none()
        && !snapshot.warning_counter_read
        && snapshot.warning_counter_before.is_none()
        && snapshot.first_warning_predicate_satisfied.is_none()
        && !snapshot.first_warning_branch_entered
        && !snapshot.warning_counter_incremented
        && snapshot.warning_counter_after.is_none()
        && !snapshot.first_warning_call_site_reached
        && !snapshot.maximum_cooling_air_volume_flow_rate_read
        && snapshot
            .maximum_cooling_air_volume_flow_rate_m3_per_s
            .is_none()
        && !snapshot.continue_warning_call_site_reached
        && !snapshot.continue_warning_timestamp_call_site_reached
        && !snapshot.recurring_warning_branch_entered
        && !snapshot.recurring_warning_call_site_reached
        && snapshot
            .recurring_warning_report_maximum_input_m3_per_s
            .is_none()
        && !snapshot.characterized_recurring_warning_index_allocated_on_call
        && !snapshot.characterized_recurring_warning_index_reused_on_call
        && snapshot
            .characterized_recurring_warning_index_before
            .is_none()
        && snapshot
            .characterized_recurring_warning_index_after
            .is_none()
        && snapshot
            .characterized_recurring_warning_occurrence_ordinal
            .is_none()
        && snapshot
            .characterized_recurring_warning_report_maximum_m3_per_s
            .is_none()
        && !snapshot.characterized_total_warning_error_incremented
        && !snapshot.maximum_cooling_air_mass_flow_rate_read
        && snapshot
            .maximum_cooling_air_mass_flow_rate_kg_per_s
            .is_none()
        && !snapshot.outdoor_air_mass_flow_clamp_assignment_performed
        && snapshot
            .outdoor_air_mass_flow_rate_after_clamp_kg_per_s
            .is_none()
}

fn gate_sites_are_skipped(snapshot: PurchasedAirCalcCoolingOaMaxFlowGateSnapshot) -> bool {
    !snapshot.cooling_limit_flow_rate_comparison_evaluated
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
        && !snapshot.outdoor_air_mass_flow_rate_read
        && snapshot.outdoor_air_mass_flow_rate_kg_per_s.is_none()
        && !snapshot.maximum_cooling_air_mass_flow_rate_read
        && snapshot
            .maximum_cooling_air_mass_flow_rate_kg_per_s
            .is_none()
        && !snapshot.strict_mass_flow_comparison_evaluated
        && snapshot.outdoor_air_mass_flow_above_maximum.is_none()
}

fn option_f64_has_bits(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}
