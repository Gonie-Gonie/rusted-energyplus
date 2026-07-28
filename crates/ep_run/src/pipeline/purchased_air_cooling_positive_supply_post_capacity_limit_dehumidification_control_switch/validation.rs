//! Fail-closed validation for CP346 direct-release evidence.

use ep_model::DehumidificationControlType;
use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE,
    PurchasedAirCalcCoolingDehumidificationFlowLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentLifecycleSummary,
    PurchasedAirInitLifecycleSummary,
};

pub(in crate::pipeline) struct DirectLifecyclePredecessors<'a> {
    pub(in crate::pipeline) post_capacity_assignment_cp345: Option<
        &'a PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentLifecycleSummary,
    >,
    pub(in crate::pipeline) dehumidification_flow_cp319:
        Option<&'a PurchasedAirCalcCoolingDehumidificationFlowLifecycleSummary>,
}

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<
        &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchLifecycleSummary,
    >,
    predecessors: DirectLifecyclePredecessors<'_>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose post-capacity-limit dehumidification-control switch evidence"
            .to_string()
    })?;
    let predecessor = predecessors.post_capacity_assignment_cp345.ok_or_else(|| {
        "direct-zone IdealLoads dehumidification-control switch has no CP345 evidence".to_string()
    })?;
    let corroborating = predecessors.dehumidification_flow_cp319.ok_or_else(|| {
        "direct-zone IdealLoads dehumidification-control switch has no CP319 evidence".to_string()
    })?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads dehumidification-control switch has no initialization evidence"
            .to_string()
    })?;
    let calls = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads dehumidification-control switch has no coupling call count"
            .to_string()
    })?;

    if calls == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || corroborating.source != PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SOURCE
        || corroborating.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE
    {
        return Err(
            "direct-zone IdealLoads dehumidification-control switch provenance is invalid"
                .to_string(),
        );
    }

    let state = &lifecycle.state;
    let predecessor_state = &predecessor.state;
    let corroborating_state = &corroborating.state;
    let switch_count = state.dehumidification_control_switch_count;
    let transition_partition = checked_sum(
        &[
            state.unit_off_skip_count,
            state.non_cooling_skip_count,
            state.positive_guard_false_fallthrough_skip_count,
            switch_count,
        ],
        "transition partition",
    )?;
    let predecessor_route_partition = checked_sum(
        &[
            predecessor_state.assignment_after_capacity_limit_guard_false_fallthrough_count,
            predecessor_state
                .assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count,
            predecessor_state
                .assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count,
        ],
        "CP345 route partition",
    )?;
    let case_partition = checked_sum(
        &[
            state.dehumidification_control_none_case_selection_count,
            state.dehumidification_control_constant_sensible_heat_ratio_case_selection_count,
            state.dehumidification_control_humidistat_case_selection_count,
            state.dehumidification_control_constant_supply_humidity_ratio_case_selection_count,
        ],
        "case partition",
    )?;
    let source_sites = switch_count
        .checked_mul(
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE_ORDER
                .len(),
        )
        .ok_or_else(|| {
            "direct-zone IdealLoads dehumidification-control switch source-site count overflowed"
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
            "corroborating_transition_count",
            corroborating_state.transition_count,
            state.transition_count,
        ),
        (
            "transition_partition",
            state.transition_count,
            transition_partition,
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
            "predecessor_assignment_count",
            predecessor_state.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count,
            switch_count,
        ),
        (
            "predecessor_route_partition",
            switch_count,
            predecessor_route_partition,
        ),
        ("case_partition", switch_count, case_partition),
        (
            "direct_none_case_selection_count",
            switch_count,
            state.dehumidification_control_none_case_selection_count,
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
            "dehumidification_control_type_read_count",
            switch_count,
            state.dehumidification_control_type_read_count,
        ),
        (
            "dehumidification_control_switch_dispatch_count",
            switch_count,
            state.dehumidification_control_switch_dispatch_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }

    let latest = state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads dehumidification-control switch has no latest snapshot".to_string()
    })?;
    let predecessor_latest = predecessor_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads dehumidification-control switch has no latest CP345 snapshot"
            .to_string()
    })?;
    let corroborating_latest = corroborating_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads dehumidification-control switch has no latest CP319 snapshot"
            .to_string()
    })?;
    let expected_system = init.declared_system_order.first().copied().ok_or_else(|| {
        "direct-zone IdealLoads dehumidification-control switch has no declared system".to_string()
    })?;
    let expected_zone = init.controlled_zone.ok_or_else(|| {
        "direct-zone IdealLoads dehumidification-control switch has no controlled Zone".to_string()
    })?;

    let active =
        predecessor_latest.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed;
    let active_route_count =
        usize::from(predecessor_latest.capacity_limit_guard_false_fallthrough_skipped)
            + usize::from(
                predecessor_latest.capacity_limit_sensible_output_guard_false_fallthrough,
            )
            + usize::from(
                predecessor_latest
                    .capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed,
            );
    let source_shape = if active {
        latest.dehumidification_control_type_read
            && latest.dehumidification_control_type == Some(DehumidificationControlType::None)
            && latest.dehumidification_control_switch_dispatched
            && corroborating_latest.dehumidification_control_type_read
            && corroborating_latest.dehumidification_control_type
                == Some(DehumidificationControlType::None)
    } else {
        !latest.dehumidification_control_type_read
            && latest.dehumidification_control_type.is_none()
            && !latest.dehumidification_control_switch_dispatched
    };
    if ![
        state.system,
        predecessor_state.system,
        corroborating_state.system,
        latest.system,
        predecessor_latest.system,
        corroborating_latest.system,
    ]
    .into_iter()
    .all(|system| system == expected_system)
        || ![
            latest.parent_call_ordinal,
            predecessor_latest.parent_call_ordinal,
            corroborating_latest.parent_call_ordinal,
        ]
        .into_iter()
        .all(|ordinal| ordinal == calls)
        || ![
            latest.controlled_zone,
            predecessor_latest.controlled_zone,
            corroborating_latest.controlled_zone,
        ]
        .into_iter()
        .all(|zone| zone == expected_zone)
        || latest.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE
        || latest.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_SOURCE
        || latest.source_order
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE_ORDER
        || latest.unit_body_entered != predecessor_latest.unit_body_entered
        || latest.predecessor_cooling_body_entered
            != predecessor_latest.predecessor_cooling_body_entered
        || latest.predecessor_no_outdoor_air_fallback_entered
            != predecessor_latest.predecessor_no_outdoor_air_fallback_entered
        || latest.predecessor_positive_supply_mass_flow_body_entered
            != predecessor_latest.predecessor_positive_supply_mass_flow_body_entered
        || latest.unit_off_skipped != predecessor_latest.unit_off_skipped
        || latest.non_cooling_skipped != predecessor_latest.non_cooling_skipped
        || latest.positive_guard_false_fallthrough_skipped
            != predecessor_latest.positive_guard_false_fallthrough_skipped
        || latest.predecessor_capacity_limit_guard_false_fallthrough
            != predecessor_latest.capacity_limit_guard_false_fallthrough_skipped
        || latest.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
            != predecessor_latest.capacity_limit_sensible_output_guard_false_fallthrough
        || latest
            .predecessor_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed
            != predecessor_latest
                .capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed
        || latest
            .predecessor_post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed
            != active
        || !option_bits_equal(
            latest.predecessor_assigned_supply_humidity_ratio,
            predecessor_latest.assigned_supply_humidity_ratio,
        )
        || active_route_count != usize::from(active)
        || !source_shape
    {
        return Err(
            "direct-zone IdealLoads dehumidification-control switch latest state is not release-ready"
                .to_string(),
        );
    }
    Ok(())
}

fn checked_sum(values: &[usize], label: &str) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value).ok_or_else(|| {
            format!("direct-zone IdealLoads dehumidification-control switch {label} overflowed")
        })
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads dehumidification-control switch invariant {field} expected {expected}, got {actual}"
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
mod tests {
    use super::*;

    #[test]
    fn lifecycle_partition_overflow_fails_closed() {
        let error = checked_sum(&[usize::MAX, 1], "test partition")
            .expect_err("partition overflow must fail closed");
        assert!(error.contains("overflowed"));
    }

    #[test]
    fn bit_comparison_distinguishes_signed_zero() {
        assert!(option_bits_equal(Some(-0.0), Some(-0.0)));
        assert!(!option_bits_equal(Some(-0.0), Some(0.0)));
    }
}
