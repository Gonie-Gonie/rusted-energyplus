//! Run-summary evidence for the bounded post-capacity-limit humidity-ratio assignment.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardLifecycleSummary,
    PurchasedAirInitLifecycleSummary,
};

mod serialization;
mod validation;

pub(super) use serialization::lifecycle_json;
use validation::{snapshot_shape, validate_source_counters};

pub(super) struct DirectLifecyclePredecessors<'a> {
    pub(super) capacity_limit_temperature_cp344: Option<
        &'a PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitLifecycleSummary,
    >,
    pub(super) mixed_air_cp329: Option<&'a PurchasedAirCalcCoolingMixedAirCallLifecycleSummary>,
    pub(super) corroborating_cp335: Option<
        &'a PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentLifecycleSummary,
    >,
    pub(super) positive_guard_cp330:
        Option<&'a PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardLifecycleSummary>,
    pub(super) enthalpy_cp336:
        Option<&'a PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentLifecycleSummary>,
    pub(super) capacity_limit_guard_cp337:
        Option<&'a PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardLifecycleSummary>,
}

pub(super) fn validate_direct_lifecycle(
    lifecycle: Option<
        &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentLifecycleSummary,
    >,
    predecessors: DirectLifecyclePredecessors<'_>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose post-capacity-limit mixed-air humidity-ratio assignment evidence"
            .to_string()
    })?;
    let predecessor = predecessors.capacity_limit_temperature_cp344.ok_or_else(|| {
        "direct-zone IdealLoads post-capacity-limit humidity-ratio assignment has no CP344 evidence"
            .to_string()
    })?;
    let mixed_air = predecessors.mixed_air_cp329.ok_or_else(|| {
        "direct-zone IdealLoads post-capacity-limit humidity-ratio assignment has no CP329 evidence"
            .to_string()
    })?;
    let corroborating = predecessors.corroborating_cp335.ok_or_else(|| {
        "direct-zone IdealLoads post-capacity-limit humidity-ratio assignment has no CP335 evidence"
            .to_string()
    })?;
    let positive_guard = predecessors.positive_guard_cp330.ok_or_else(|| {
        "direct-zone IdealLoads post-capacity-limit humidity-ratio assignment has no CP330 evidence"
            .to_string()
    })?;
    let enthalpy = predecessors.enthalpy_cp336.ok_or_else(|| {
        "direct-zone IdealLoads post-capacity-limit humidity-ratio assignment has no CP336 evidence"
            .to_string()
    })?;
    let capacity_limit_guard = predecessors.capacity_limit_guard_cp337.ok_or_else(|| {
        "direct-zone IdealLoads post-capacity-limit humidity-ratio assignment has no CP337 evidence"
            .to_string()
    })?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads post-capacity-limit humidity-ratio assignment has no initialization evidence"
            .to_string()
    })?;
    let calls = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads post-capacity-limit humidity-ratio assignment has no coupling call count"
            .to_string()
    })?;

    if calls == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE
        || mixed_air.source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE
        || mixed_air.child_source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE
        || mixed_air.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE
        || corroborating.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE
        || corroborating.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || positive_guard.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE
        || positive_guard.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE
        || enthalpy.source != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE
        || enthalpy.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || capacity_limit_guard.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE
        || capacity_limit_guard.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE
    {
        return Err(
            "direct-zone IdealLoads post-capacity-limit humidity-ratio assignment provenance is invalid"
                .to_string(),
        );
    }

    let state = &lifecycle.state;
    let predecessor_state = &predecessor.state;
    let mixed_air_state = &mixed_air.state;
    let corroborating_state = &corroborating.state;
    let positive_guard_state = &positive_guard.state;
    let enthalpy_state = &enthalpy.state;
    let capacity_limit_guard_state = &capacity_limit_guard.state;
    let skipped = checked_add(
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        "skip partition",
    )?;
    let skipped = checked_add(
        skipped,
        state.positive_guard_false_fallthrough_skip_count,
        "positive-guard-false partition",
    )?;
    let assignment_routes = checked_add(
        state.assignment_after_capacity_limit_guard_false_fallthrough_count,
        state.assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count,
        "assignment-route partition",
    )?;
    let assignment_routes = checked_add(
        assignment_routes,
        state
            .assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count,
        "assignment-route partition",
    )?;
    let capacity_body_routes = checked_add(
        state.assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count,
        state
            .assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count,
        "capacity-body route partition",
    )?;
    let transition_partition = checked_add(
        skipped,
        state.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count,
        "transition partition",
    )?;

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
            "corroborating_transition_count",
            corroborating_state.transition_count,
            state.transition_count,
        ),
        (
            "positive_guard_transition_count",
            positive_guard_state.transition_count,
            state.transition_count,
        ),
        (
            "enthalpy_transition_count",
            enthalpy_state.transition_count,
            state.transition_count,
        ),
        (
            "capacity_limit_guard_transition_count",
            capacity_limit_guard_state.transition_count,
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
            "assignment_after_capacity_limit_guard_false_fallthrough_count",
            predecessor_state.capacity_limit_guard_false_fallthrough_skip_count,
            state.assignment_after_capacity_limit_guard_false_fallthrough_count,
        ),
        (
            "assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count",
            predecessor_state.capacity_limit_sensible_output_guard_false_fallthrough_count,
            state.assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count,
        ),
        (
            "assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count",
            predecessor_state
                .capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count,
            state
                .assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count,
        ),
        (
            "assignment_route_partition",
            assignment_routes,
            state.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count,
        ),
        (
            "corroborating_supply_humidity_ratio_mixed_air_assignment_count",
            corroborating_state.supply_humidity_ratio_mixed_air_assignment_count,
            state.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count,
        ),
        (
            "positive_guard_positive_supply_mass_flow_body_entry_count",
            positive_guard_state.positive_supply_mass_flow_body_entry_count,
            state.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count,
        ),
        (
            "enthalpy_supply_enthalpy_assignment_count",
            enthalpy_state.supply_enthalpy_assignment_count,
            state.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count,
        ),
        (
            "capacity_limit_guard_evaluation_count",
            capacity_limit_guard_state.capacity_limit_guard_evaluation_count,
            state.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count,
        ),
        (
            "capacity_limit_guard_active_guard_false_fallthrough_count",
            capacity_limit_guard_state.active_guard_false_fallthrough_count,
            state.assignment_after_capacity_limit_guard_false_fallthrough_count,
        ),
        (
            "capacity_limit_guard_body_entry_count",
            capacity_limit_guard_state.capacity_limit_body_entry_count,
            capacity_body_routes,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    validate_source_counters(state)?;

    let latest = state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads post-capacity-limit humidity-ratio assignment has no latest snapshot"
            .to_string()
    })?;
    let predecessor_latest = predecessor_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads post-capacity-limit humidity-ratio assignment has no latest CP344 snapshot"
            .to_string()
    })?;
    let mixed_air_latest = mixed_air_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads post-capacity-limit humidity-ratio assignment has no latest CP329 snapshot"
            .to_string()
    })?;
    let corroborating_latest = corroborating_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads post-capacity-limit humidity-ratio assignment has no latest CP335 snapshot"
            .to_string()
    })?;
    let positive_guard_latest = positive_guard_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads post-capacity-limit humidity-ratio assignment has no latest CP330 snapshot"
            .to_string()
    })?;
    let enthalpy_latest = enthalpy_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads post-capacity-limit humidity-ratio assignment has no latest CP336 snapshot"
            .to_string()
    })?;
    let capacity_limit_guard_latest =
        capacity_limit_guard_state.latest.as_ref().ok_or_else(|| {
            "direct-zone IdealLoads post-capacity-limit humidity-ratio assignment has no latest CP337 snapshot"
                .to_string()
        })?;
    let expected_system = init.declared_system_order.first().copied().ok_or_else(|| {
        "direct-zone IdealLoads post-capacity-limit humidity-ratio assignment has no declared system"
            .to_string()
    })?;
    let expected_zone = init.controlled_zone.ok_or_else(|| {
        "direct-zone IdealLoads post-capacity-limit humidity-ratio assignment has no controlled Zone"
            .to_string()
    })?;

    if ![
        state.system,
        predecessor_state.system,
        mixed_air_state.system,
        corroborating_state.system,
        positive_guard_state.system,
        enthalpy_state.system,
        capacity_limit_guard_state.system,
    ]
    .into_iter()
    .all(|system| system == expected_system)
        || latest.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE
        || latest.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || latest.source_order
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER
        || predecessor_latest.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE
        || predecessor_latest.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE
        || predecessor_latest.source_order
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER
        || mixed_air_latest.source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE
        || mixed_air_latest.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE
        || corroborating_latest.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE
        || corroborating_latest.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || corroborating_latest.source_order
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER
        || positive_guard_latest.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE
        || positive_guard_latest.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE
        || positive_guard_latest.source_order
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE_ORDER
        || enthalpy_latest.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE
        || enthalpy_latest.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || enthalpy_latest.source_order
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER
        || capacity_limit_guard_latest.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE
        || capacity_limit_guard_latest.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE
        || capacity_limit_guard_latest.source_order
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE_ORDER
        || ![
            latest.system,
            predecessor_latest.system,
            mixed_air_latest.system,
            corroborating_latest.system,
            positive_guard_latest.system,
            enthalpy_latest.system,
            capacity_limit_guard_latest.system,
        ]
        .into_iter()
        .all(|system| system == expected_system)
        || ![
            latest.parent_call_ordinal,
            predecessor_latest.parent_call_ordinal,
            mixed_air_latest.parent_call_ordinal,
            corroborating_latest.parent_call_ordinal,
            positive_guard_latest.parent_call_ordinal,
            enthalpy_latest.parent_call_ordinal,
            capacity_limit_guard_latest.parent_call_ordinal,
        ]
        .into_iter()
        .all(|ordinal| ordinal == calls)
        || ![
            latest.controlled_zone,
            predecessor_latest.controlled_zone,
            mixed_air_latest.controlled_zone,
            corroborating_latest.controlled_zone,
            positive_guard_latest.controlled_zone,
            enthalpy_latest.controlled_zone,
            capacity_limit_guard_latest.controlled_zone,
        ]
        .into_iter()
        .all(|zone| zone == expected_zone)
        || !snapshot_shape(
            latest,
            predecessor_latest,
            mixed_air_latest,
            corroborating_latest,
        )
    {
        return Err(
            "direct-zone IdealLoads post-capacity-limit humidity-ratio assignment latest state is not release-ready"
                .to_string(),
        );
    }
    Ok(())
}

fn checked_add(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_add(right).ok_or_else(|| {
        format!(
            "direct-zone IdealLoads post-capacity-limit humidity-ratio assignment {label} overflowed"
        )
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads post-capacity-limit humidity-ratio assignment invariant {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifecycle_partition_overflow_fails_closed() {
        let error = checked_add(usize::MAX, 1, "test partition")
            .expect_err("partition overflow must fail closed");
        assert!(error.contains("overflowed"));
    }
}
