//! Fail-closed validation for CP386 direct-release evidence.

use ep_model::DehumidificationControlType;
use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_LEXICAL_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchSnapshot as Snapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot as PredecessorSnapshot,
    PurchasedAirInitLifecycleSummary,
};

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessor_cp385: Option<&PredecessorLifecycle>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose CP386 switch evidence".to_string()
    })?;
    let predecessor = predecessor_cp385
        .ok_or_else(|| "direct-zone IdealLoads CP386 has no CP385 evidence".to_string())?;
    let init = init_lifecycle
        .ok_or_else(|| "direct-zone IdealLoads CP386 has no initialization evidence".to_string())?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP386 has no coupling call count".to_string())?;
    let expected_system = init
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| "direct-zone IdealLoads CP386 has no declared system".to_string())?;
    let expected_zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP386 has no controlled Zone".to_string())?;

    let state = &lifecycle.state;
    let predecessor_state = &predecessor.state;
    let assignments = predecessor_state
        .post_saturation_capacity_limited_dehumidification_supply_enthalpy_assignment_count;
    let inactive = calls.checked_sub(assignments).ok_or_else(|| {
        "direct-zone IdealLoads CP386 inactive transition count underflowed".to_string()
    })?;
    let route_sum = checked_sum(
        &state.predecessor_route_counts,
        "predecessor route partition",
    )?;
    let source_sites = assignments
        .checked_mul(PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE_ORDER.len())
        .ok_or_else(|| "direct-zone IdealLoads CP386 source-site count overflowed".to_string())?;

    if calls == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE
        || lifecycle.first_excluded_lexical_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_LEXICAL_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || state.system != expected_system
        || predecessor_state.system != expected_system
        || !predecessor_routes_match(state, predecessor_state)
    {
        return Err("direct-zone IdealLoads CP386 provenance is invalid".to_string());
    }

    for (field, expected, actual) in [
        ("transition_count", calls, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor_state.transition_count,
            state.transition_count,
        ),
        ("predecessor_route_partition", calls, route_sum),
        (
            "inactive_transition_count",
            inactive,
            state.inactive_transition_count,
        ),
        (
            "dehumidification_control_switch_count",
            assignments,
            state.dehumidification_control_switch_count,
        ),
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "dehumidification_control_type_read_count",
            assignments,
            state.dehumidification_control_type_read_count,
        ),
        (
            "dehumidification_control_switch_dispatch_count",
            assignments,
            state.dehumidification_control_switch_dispatch_count,
        ),
        (
            "dehumidification_control_none_case_selection_count",
            assignments,
            state.dehumidification_control_none_case_selection_count,
        ),
        (
            "constant_sensible_heat_ratio_case_selection_count",
            0,
            state.dehumidification_control_constant_sensible_heat_ratio_case_selection_count,
        ),
        (
            "humidistat_case_selection_count",
            0,
            state.dehumidification_control_humidistat_case_selection_count,
        ),
        (
            "constant_supply_humidity_ratio_case_selection_count",
            0,
            state.dehumidification_control_constant_supply_humidity_ratio_case_selection_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }

    let latest = state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP386 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor_state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP386 CP385 latest evidence is missing".to_string()
    })?;
    if latest.system != expected_system
        || latest.parent_call_ordinal != calls
        || latest.controlled_zone != expected_zone
        || latest.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE
        || latest.first_excluded_lexical_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_LEXICAL_SOURCE
        || latest.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_SOURCE
        || latest.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE_ORDER
        || !links_to_predecessor(latest, predecessor_latest)
    {
        return Err("direct-zone IdealLoads CP386 latest lineage is invalid".to_string());
    }
    Ok(())
}

fn links_to_predecessor(snapshot: Snapshot, predecessor: PredecessorSnapshot) -> bool {
    let active = predecessor.supply_enthalpy_assignment_executed;
    snapshot.system == predecessor.system
        && snapshot.parent_call_ordinal == predecessor.parent_call_ordinal
        && snapshot.controlled_zone == predecessor.controlled_zone
        && inherited_flags(snapshot) == predecessor_flags(predecessor)
        && snapshot.predecessor_supply_enthalpy_assignment_executed == active
        && option_bits_equal(
            snapshot.predecessor_resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && option_bits_equal(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && snapshot.dehumidification_control_type_read == active
        && snapshot.dehumidification_control_type
            == active.then_some(DehumidificationControlType::None)
        && snapshot.dehumidification_control_switch_dispatched == active
}

fn inherited_flags(snapshot: Snapshot) -> [bool; 20] {
    [
        snapshot.unit_off_skipped,
        snapshot.non_cooling_skipped,
        snapshot.positive_guard_false_fallthrough_skipped,
        snapshot.heating_availability_guard_false_fallthrough,
        snapshot.humidification_control_guard_false_fallthrough,
        snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        snapshot.dehumidification_control_none_maximum_assignment_executed,
        snapshot.dehumidification_control_guard_false_fallthrough,
        snapshot.predecessor_capacity_limit_guard_evaluated,
        snapshot.predecessor_capacity_limit_body_entered,
        snapshot.predecessor_active_capacity_limit_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_guard_evaluated,
        snapshot.predecessor_dehumidification_body_entered,
        snapshot.predecessor_dehumidification_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_total_output_assignment_executed,
        snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated,
        snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_maximum_capacity_assignment_executed,
    ]
}

fn predecessor_flags(snapshot: PredecessorSnapshot) -> [bool; 20] {
    [
        snapshot.unit_off_skipped,
        snapshot.non_cooling_skipped,
        snapshot.positive_guard_false_fallthrough_skipped,
        snapshot.heating_availability_guard_false_fallthrough,
        snapshot.humidification_control_guard_false_fallthrough,
        snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        snapshot.dehumidification_control_none_maximum_assignment_executed,
        snapshot.dehumidification_control_guard_false_fallthrough,
        snapshot.predecessor_capacity_limit_guard_evaluated,
        snapshot.predecessor_capacity_limit_body_entered,
        snapshot.predecessor_active_capacity_limit_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_guard_evaluated,
        snapshot.predecessor_dehumidification_body_entered,
        snapshot.predecessor_dehumidification_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_total_output_assignment_executed,
        snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated,
        snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_maximum_capacity_assignment_executed,
    ]
}

fn predecessor_route_counts(state: &PredecessorState) -> [usize; 23] {
    [
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.heating_availability_guard_false_fallthrough_capacity_guard_false_count,
        state.heating_availability_guard_false_fallthrough_dehumidification_guard_false_count,
        state.heating_availability_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count,
        state.heating_availability_guard_false_fallthrough_dehumidification_total_output_maximum_capacity_assignment_count,
        state.humidification_control_guard_false_fallthrough_capacity_guard_false_count,
        state.humidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
        state.humidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count,
        state.humidification_control_guard_false_fallthrough_dehumidification_total_output_maximum_capacity_assignment_count,
        state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count,
        state.dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count,
        state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_capacity_guard_false_fallthrough_count,
        state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_maximum_capacity_assignment_count,
        state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count,
        state.dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count,
        state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_capacity_guard_false_fallthrough_count,
        state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_maximum_capacity_assignment_count,
        state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count,
        state.dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
        state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count,
        state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_maximum_capacity_assignment_count,
    ]
}

fn predecessor_routes_match(state: &State, predecessor: &PredecessorState) -> bool {
    state.predecessor_route_counts == predecessor_route_counts(predecessor)
}

fn option_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

fn checked_sum(values: &[usize], label: &str) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| format!("direct-zone IdealLoads CP386 {label} overflowed"))
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP386 invariant {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests {
    use ep_model::IdealLoadsAirSystemId;

    use super::*;

    #[test]
    fn route_partition_overflow_fails_closed() {
        assert!(checked_sum(&[usize::MAX, 1], "test partition").is_err());
    }

    #[test]
    fn bit_comparison_distinguishes_signed_zero() {
        assert!(option_bits_equal(Some(-0.0), Some(-0.0)));
        assert!(!option_bits_equal(Some(-0.0), Some(0.0)));
    }

    #[test]
    fn cp386_predecessor_route_mutation_is_rejected() {
        let predecessor = PredecessorState::new(IdealLoadsAirSystemId(0));
        let mut state = State::new(IdealLoadsAirSystemId(0));
        assert!(predecessor_routes_match(&state, &predecessor));
        state.predecessor_route_counts[0] = 1;
        assert!(!predecessor_routes_match(&state, &predecessor));
    }
}
