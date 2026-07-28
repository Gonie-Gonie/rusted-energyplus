//! Run-summary evidence for the bounded capacity-limit supply-temperature mixed-air limit.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitLifecycleSummary,
    PurchasedAirInitLifecycleSummary,
};

mod serialization;
mod validation;

pub(super) use serialization::lifecycle_json;
use validation::{snapshot_shape, validate_source_counters};

pub(super) fn validate_direct_lifecycle(
    lifecycle: Option<
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitLifecycleSummary,
    >,
    predecessor_cp343: Option<
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentLifecycleSummary,
    >,
    mixed_air_cp329: Option<&PurchasedAirCalcCoolingMixedAirCallLifecycleSummary>,
    corroborating_cp334: Option<
        &PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitLifecycleSummary,
    >,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose capacity-limit supply-temperature mixed-air-limit evidence"
            .to_string()
    })?;
    let predecessor = predecessor_cp343.ok_or_else(|| {
        "direct-zone IdealLoads capacity-limit supply-temperature mixed-air limit has no CP343 evidence"
            .to_string()
    })?;
    let mixed_air = mixed_air_cp329.ok_or_else(|| {
        "direct-zone IdealLoads capacity-limit supply-temperature mixed-air limit has no CP329 evidence"
            .to_string()
    })?;
    let corroborating = corroborating_cp334.ok_or_else(|| {
        "direct-zone IdealLoads capacity-limit supply-temperature mixed-air limit has no CP334 evidence"
            .to_string()
    })?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads capacity-limit supply-temperature mixed-air limit has no initialization evidence"
            .to_string()
    })?;
    let calls = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads capacity-limit supply-temperature mixed-air limit has no coupling call count"
            .to_string()
    })?;

    if calls == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || mixed_air.source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE
        || mixed_air.child_source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE
        || mixed_air.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE
        || corroborating.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE
        || corroborating.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE
    {
        return Err(
            "direct-zone IdealLoads capacity-limit supply-temperature mixed-air-limit provenance is invalid"
                .to_string(),
        );
    }

    let state = &lifecycle.state;
    let predecessor_state = &predecessor.state;
    let mixed_air_state = &mixed_air.state;
    let corroborating_state = &corroborating.state;
    let inherited_skips = checked_add(
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        "skip partition",
    )?;
    let inherited_skips = checked_add(
        inherited_skips,
        state.positive_guard_false_fallthrough_skip_count,
        "positive-guard-false partition",
    )?;
    let inherited_skips = checked_add(
        inherited_skips,
        state.capacity_limit_guard_false_fallthrough_skip_count,
        "capacity-limit-guard-false partition",
    )?;
    let active_partition = checked_add(
        state.capacity_limit_sensible_output_guard_false_fallthrough_count,
        state.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count,
        "active partition",
    )?;
    let predecessor_active_partition = checked_add(
        predecessor_state.capacity_limit_sensible_output_guard_false_fallthrough_count,
        predecessor_state.capacity_limit_sensible_output_supply_temperature_assignment_count,
        "predecessor active partition",
    )?;
    let transition_partition =
        checked_add(inherited_skips, active_partition, "transition partition")?;

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
            "capacity_limit_guard_false_fallthrough_skip_count",
            predecessor_state.capacity_limit_guard_false_fallthrough_skip_count,
            state.capacity_limit_guard_false_fallthrough_skip_count,
        ),
        (
            "capacity_limit_sensible_output_guard_false_fallthrough_count",
            predecessor_state.capacity_limit_sensible_output_guard_false_fallthrough_count,
            state.capacity_limit_sensible_output_guard_false_fallthrough_count,
        ),
        (
            "capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count",
            predecessor_state.capacity_limit_sensible_output_supply_temperature_assignment_count,
            state.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count,
        ),
        (
            "active_partition",
            predecessor_active_partition,
            active_partition,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    validate_source_counters(state)?;

    let latest = state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads capacity-limit supply-temperature mixed-air limit has no latest snapshot"
            .to_string()
    })?;
    let predecessor_latest = predecessor_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads capacity-limit supply-temperature mixed-air limit has no latest CP343 snapshot"
            .to_string()
    })?;
    let mixed_air_latest = mixed_air_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads capacity-limit supply-temperature mixed-air limit has no latest CP329 snapshot"
            .to_string()
    })?;
    let corroborating_latest = corroborating_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads capacity-limit supply-temperature mixed-air limit has no latest CP334 snapshot"
            .to_string()
    })?;
    let expected_system = init.declared_system_order.first().copied().ok_or_else(|| {
        "direct-zone IdealLoads capacity-limit supply-temperature mixed-air limit has no declared system"
            .to_string()
    })?;
    let expected_zone = init.controlled_zone.ok_or_else(|| {
        "direct-zone IdealLoads capacity-limit supply-temperature mixed-air limit has no controlled Zone"
            .to_string()
    })?;

    if ![
        state.system,
        predecessor_state.system,
        mixed_air_state.system,
        corroborating_state.system,
    ]
    .into_iter()
    .all(|system| system == expected_system)
        || latest.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE
        || latest.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE
        || latest.source_order
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER
        || predecessor_latest.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE
        || predecessor_latest.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor_latest.source_order
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER
        || mixed_air_latest.source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE
        || mixed_air_latest.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE
        || corroborating_latest.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE
        || corroborating_latest.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE
        || corroborating_latest.source_order
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER
        || ![
            latest.system,
            predecessor_latest.system,
            mixed_air_latest.system,
            corroborating_latest.system,
        ]
        .into_iter()
        .all(|system| system == expected_system)
        || ![
            latest.parent_call_ordinal,
            predecessor_latest.parent_call_ordinal,
            mixed_air_latest.parent_call_ordinal,
            corroborating_latest.parent_call_ordinal,
        ]
        .into_iter()
        .all(|ordinal| ordinal == calls)
        || ![
            latest.controlled_zone,
            predecessor_latest.controlled_zone,
            mixed_air_latest.controlled_zone,
            corroborating_latest.controlled_zone,
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
            "direct-zone IdealLoads capacity-limit supply-temperature mixed-air limit latest state is not release-ready"
                .to_string(),
        );
    }
    Ok(())
}

fn checked_add(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_add(right).ok_or_else(|| {
        format!(
            "direct-zone IdealLoads capacity-limit supply-temperature mixed-air limit {label} overflowed"
        )
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads capacity-limit supply-temperature mixed-air limit invariant {field} expected {expected}, got {actual}"
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
