//! Fail-closed validation for CP347 direct-release evidence.

use ep_model::DehumidificationControlType;
use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE,
    PurchasedAirCalcCoolingMixedAirCallLifecycleSummary,
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
    PurchasedAirInitLifecycleSummary,
};

pub(in crate::pipeline) struct DirectLifecyclePredecessors<'a> {
    pub(in crate::pipeline) control_switch_cp346: Option<
        &'a PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchLifecycleSummary,
    >,
    pub(in crate::pipeline) mixed_air_cp329:
        Option<&'a PurchasedAirCalcCoolingMixedAirCallLifecycleSummary>,
}

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<
        &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseLifecycleSummary,
    >,
    predecessors: DirectLifecyclePredecessors<'_>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose dehumidification-control None-case evidence"
            .to_string()
    })?;
    let predecessor = predecessors.control_switch_cp346.ok_or_else(|| {
        "direct-zone IdealLoads dehumidification-control None case has no CP346 evidence"
            .to_string()
    })?;
    let mixed_air = predecessors.mixed_air_cp329.ok_or_else(|| {
        "direct-zone IdealLoads dehumidification-control None case has no CP329 evidence"
            .to_string()
    })?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads dehumidification-control None case has no initialization evidence"
            .to_string()
    })?;
    let calls = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads dehumidification-control None case has no coupling call count"
            .to_string()
    })?;

    if calls == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_SOURCE
        || mixed_air.source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE
        || mixed_air.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE
    {
        return Err(
            "direct-zone IdealLoads dehumidification-control None-case provenance is invalid"
                .to_string(),
        );
    }

    let state = &lifecycle.state;
    let predecessor_state = &predecessor.state;
    let mixed_air_state = &mixed_air.state;
    let completed = state.dehumidification_control_none_case_completion_count;
    validate_route_partition(state)?;
    let source_sites = completed
        .checked_mul(
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE_ORDER
                .len(),
        )
        .ok_or_else(|| {
            "direct-zone IdealLoads dehumidification-control None-case source-site count overflowed"
                .to_string()
        })?;

    for (field, expected, actual) in [
        ("transition_count", calls, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor_state.transition_count,
            state.transition_count,
        ),
        (
            "mixed_air_transition_count",
            mixed_air_state.transition_count,
            state.transition_count,
        ),
        (
            "unit_off_skip_count",
            predecessor_state.unit_off_skip_count,
            state.unit_off_skip_count,
        ),
        (
            "non_cooling_skip_count",
            predecessor_state.non_cooling_skip_count,
            state.non_cooling_skip_count,
        ),
        (
            "positive_guard_false_fallthrough_skip_count",
            predecessor_state.positive_guard_false_fallthrough_skip_count,
            state.positive_guard_false_fallthrough_skip_count,
        ),
        (
            "none_case_completion_count",
            predecessor_state.dehumidification_control_none_case_selection_count,
            completed,
        ),
        (
            "direct_none_case_completion_count",
            predecessor_state.dehumidification_control_switch_count,
            completed,
        ),
        (
            "constant_sensible_heat_ratio_case_selection_count",
            predecessor_state
                .dehumidification_control_constant_sensible_heat_ratio_case_selection_count,
            state.dehumidification_control_constant_sensible_heat_ratio_case_selection_count,
        ),
        (
            "humidistat_case_selection_count",
            predecessor_state.dehumidification_control_humidistat_case_selection_count,
            state.dehumidification_control_humidistat_case_selection_count,
        ),
        (
            "constant_supply_humidity_ratio_case_selection_count",
            predecessor_state
                .dehumidification_control_constant_supply_humidity_ratio_case_selection_count,
            state.dehumidification_control_constant_supply_humidity_ratio_case_selection_count,
        ),
        (
            "direct_constant_sensible_heat_ratio_case_selection_count",
            0,
            state.dehumidification_control_constant_sensible_heat_ratio_case_selection_count,
        ),
        (
            "direct_humidistat_case_selection_count",
            0,
            state.dehumidification_control_humidistat_case_selection_count,
        ),
        (
            "direct_constant_supply_humidity_ratio_case_selection_count",
            0,
            state.dehumidification_control_constant_supply_humidity_ratio_case_selection_count,
        ),
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "none_case_entry_count",
            completed,
            state.dehumidification_control_none_case_entry_count,
        ),
        (
            "mixed_air_humidity_ratio_read_count",
            completed,
            state.mixed_air_humidity_ratio_read_count,
        ),
        (
            "supply_humidity_ratio_assignment_count",
            completed,
            state.supply_humidity_ratio_assignment_count,
        ),
        (
            "none_case_break_count",
            completed,
            state.dehumidification_control_none_case_break_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }

    let latest = state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads dehumidification-control None case has no latest snapshot"
            .to_string()
    })?;
    let predecessor_latest = predecessor_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads dehumidification-control None case has no latest CP346 snapshot"
            .to_string()
    })?;
    let mixed_air_latest = mixed_air_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads dehumidification-control None case has no latest CP329 snapshot"
            .to_string()
    })?;
    let expected_system = init.declared_system_order.first().copied().ok_or_else(|| {
        "direct-zone IdealLoads dehumidification-control None case has no declared system"
            .to_string()
    })?;
    let expected_zone = init.controlled_zone.ok_or_else(|| {
        "direct-zone IdealLoads dehumidification-control None case has no controlled Zone"
            .to_string()
    })?;

    if ![
        state.system,
        predecessor_state.system,
        mixed_air_state.system,
        latest.system,
        predecessor_latest.system,
        mixed_air_latest.system,
    ]
    .into_iter()
    .all(|system| system == expected_system)
        || ![
            latest.parent_call_ordinal,
            predecessor_latest.parent_call_ordinal,
            mixed_air_latest.parent_call_ordinal,
        ]
        .into_iter()
        .all(|ordinal| ordinal == calls)
        || ![
            latest.controlled_zone,
            predecessor_latest.controlled_zone,
            mixed_air_latest.controlled_zone,
        ]
        .into_iter()
        .all(|zone| zone == expected_zone)
        || !snapshot_shape(latest, predecessor_latest, mixed_air_latest)
    {
        return Err(
            "direct-zone IdealLoads dehumidification-control None-case latest state is not release-ready"
                .to_string(),
        );
    }
    Ok(())
}

fn snapshot_shape(
    snapshot:
        &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
    predecessor:
        &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
    mixed_air_owner: &PurchasedAirCalcCoolingMixedAirCallSnapshot,
) -> bool {
    if snapshot.source
        != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE_ORDER
        || snapshot.system != predecessor.system
        || snapshot.parent_call_ordinal != predecessor.parent_call_ordinal
        || snapshot.controlled_zone != predecessor.controlled_zone
        || snapshot.unit_body_entered != predecessor.unit_body_entered
        || snapshot.predecessor_cooling_body_entered
            != predecessor.predecessor_cooling_body_entered
        || snapshot.predecessor_no_outdoor_air_fallback_entered
            != predecessor.predecessor_no_outdoor_air_fallback_entered
        || snapshot.predecessor_positive_supply_mass_flow_body_entered
            != predecessor.predecessor_positive_supply_mass_flow_body_entered
        || snapshot.unit_off_skipped != predecessor.unit_off_skipped
        || snapshot.non_cooling_skipped != predecessor.non_cooling_skipped
        || snapshot.positive_guard_false_fallthrough_skipped
            != predecessor.positive_guard_false_fallthrough_skipped
        || snapshot.predecessor_capacity_limit_guard_false_fallthrough
            != predecessor.predecessor_capacity_limit_guard_false_fallthrough
        || snapshot.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
            != predecessor.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
        || snapshot
            .predecessor_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed
            != predecessor
                .predecessor_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed
        || snapshot
            .predecessor_post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed
            != predecessor
                .predecessor_post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed
        || !option_bits_equal(
            snapshot.predecessor_assigned_supply_humidity_ratio,
            predecessor.predecessor_assigned_supply_humidity_ratio,
        )
        || snapshot.predecessor_dehumidification_control_type_read
            != predecessor.dehumidification_control_type_read
        || snapshot.predecessor_dehumidification_control_type
            != predecessor.dehumidification_control_type
        || snapshot.predecessor_dehumidification_control_switch_dispatched
            != predecessor.dehumidification_control_switch_dispatched
    {
        return false;
    }

    let active = predecessor.dehumidification_control_switch_dispatched
        && predecessor.dehumidification_control_type == Some(DehumidificationControlType::None);
    if !active {
        return !snapshot.dehumidification_control_none_case_entered
            && !snapshot.mixed_air_humidity_ratio_read
            && snapshot.mixed_air_humidity_ratio.is_none()
            && !snapshot.supply_humidity_ratio_assignment_performed
            && snapshot.assigned_supply_humidity_ratio.is_none()
            && snapshot.resulting_supply_humidity_ratio.is_none()
            && !snapshot.dehumidification_control_none_case_exited_via_break;
    }

    snapshot.dehumidification_control_none_case_entered
        && snapshot.mixed_air_humidity_ratio_read
        && snapshot.supply_humidity_ratio_assignment_performed
        && snapshot.dehumidification_control_none_case_exited_via_break
        && mixed_air_owner.mixed_air_humidity_ratio_assigned
        && [
            snapshot.predecessor_assigned_supply_humidity_ratio,
            snapshot.mixed_air_humidity_ratio,
            snapshot.assigned_supply_humidity_ratio,
            snapshot.resulting_supply_humidity_ratio,
        ]
        .into_iter()
        .all(|value| option_bits_equal(value, mixed_air_owner.mixed_air_humidity_ratio))
}

fn validate_route_partition(
    state:
        &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseRuntimeState,
) -> Result<(), String> {
    let transition_partition = checked_sum(
        &[
            state.unit_off_skip_count,
            state.non_cooling_skip_count,
            state.positive_guard_false_fallthrough_skip_count,
            state.dehumidification_control_none_case_completion_count,
            state.dehumidification_control_constant_sensible_heat_ratio_case_selection_count,
            state.dehumidification_control_humidistat_case_selection_count,
            state.dehumidification_control_constant_supply_humidity_ratio_case_selection_count,
        ],
        "transition partition",
    )?;
    ensure_count(
        transition_partition,
        state.transition_count,
        "transition_partition",
    )
}

fn checked_sum(values: &[usize], label: &str) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value).ok_or_else(|| {
            format!("direct-zone IdealLoads dehumidification-control None case {label} overflowed")
        })
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads dehumidification-control None-case invariant {field} expected {expected}, got {actual}"
        ))
    }
}

fn option_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests;
