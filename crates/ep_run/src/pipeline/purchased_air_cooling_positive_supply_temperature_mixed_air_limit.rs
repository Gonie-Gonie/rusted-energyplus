//! Run-summary evidence for the bounded cooling positive-supply mixed-air limit.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallLifecycleSummary,
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
    PurchasedAirInitLifecycleSummary,
};

mod serialization;
mod validation;

pub(super) use serialization::lifecycle_json;
use validation::{snapshot_shape, validate_source_counters};

pub(super) fn validate_direct_lifecycle(
    lifecycle: Option<
        &PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitLifecycleSummary,
    >,
    predecessor_cp333: Option<
        &PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitLifecycleSummary,
    >,
    mixed_air_cp329: Option<&PurchasedAirCalcCoolingMixedAirCallLifecycleSummary>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose cooling positive-supply mixed-air-temperature limit evidence"
            .to_string()
    })?;
    let predecessor = predecessor_cp333.ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply mixed-air-temperature limit has no CP333 evidence"
            .to_string()
    })?;
    let mixed_air = mixed_air_cp329.ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply mixed-air-temperature limit has no CP329 evidence"
            .to_string()
    })?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply mixed-air-temperature limit has no initialization evidence"
            .to_string()
    })?;
    let calls = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply mixed-air-temperature limit has no coupling call count"
            .to_string()
    })?;

    if calls == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_FIRST_EXCLUDED_SOURCE
        || mixed_air.source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE
        || mixed_air.child_source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE
        || mixed_air.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE
    {
        return Err(
            "direct-zone IdealLoads cooling positive-supply mixed-air-temperature limit provenance is invalid"
                .to_string(),
        );
    }

    let state = &lifecycle.state;
    let predecessor_state = &predecessor.state;
    let mixed_air_state = &mixed_air.state;
    let skipped = checked_add(
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        "skip partition",
    )?;
    let skipped = checked_add(
        skipped,
        state.positive_guard_false_fallthrough_skip_count,
        "guard-false partition",
    )?;
    let transition_partition = checked_add(
        skipped,
        state.supply_temperature_mixed_air_limit_count,
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
            "supply_temperature_mixed_air_limit_count",
            predecessor_state.supply_temperature_minimum_limit_count,
            state.supply_temperature_mixed_air_limit_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    validate_source_counters(state)?;

    let latest = state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply mixed-air-temperature limit has no latest snapshot"
            .to_string()
    })?;
    let predecessor_latest = predecessor_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply mixed-air-temperature limit has no latest CP333 snapshot"
            .to_string()
    })?;
    let mixed_air_latest = mixed_air_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply mixed-air-temperature limit has no latest CP329 snapshot"
            .to_string()
    })?;
    let expected_system = init.declared_system_order.first().copied().ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply mixed-air-temperature limit has no declared system"
            .to_string()
    })?;
    let expected_zone = init.controlled_zone.ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply mixed-air-temperature limit has no controlled Zone"
            .to_string()
    })?;
    if state.system != expected_system
        || predecessor_state.system != expected_system
        || mixed_air_state.system != expected_system
        || !latest_matches_release(
            latest,
            predecessor_latest,
            mixed_air_latest,
            expected_system,
            expected_zone,
            calls,
        )
    {
        return Err(
            "direct-zone IdealLoads cooling positive-supply mixed-air-temperature limit latest state is not release-ready"
                .to_string(),
        );
    }
    Ok(())
}

fn latest_matches_release(
    mixed_air_limit: &PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
    predecessor: &PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
    mixed_air: &PurchasedAirCalcCoolingMixedAirCallSnapshot,
    expected_system: ep_model::IdealLoadsAirSystemId,
    expected_zone: ep_model::ZoneId,
    call_count: usize,
) -> bool {
    mixed_air_limit.source
        == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE
        && mixed_air_limit.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE
        && mixed_air_limit.source_order
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER
        && predecessor.source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_SOURCE
        && predecessor.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_FIRST_EXCLUDED_SOURCE
        && predecessor.source_order
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_SOURCE_ORDER
        && mixed_air.source == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE
        && mixed_air.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE
        && mixed_air_limit.system == expected_system
        && predecessor.system == expected_system
        && mixed_air.system == expected_system
        && mixed_air_limit.parent_call_ordinal == call_count
        && predecessor.parent_call_ordinal == call_count
        && mixed_air.parent_call_ordinal == call_count
        && mixed_air_limit.controlled_zone == expected_zone
        && predecessor.controlled_zone == expected_zone
        && mixed_air.controlled_zone == expected_zone
        && mixed_air_limit.unit_body_entered == predecessor.unit_body_entered
        && mixed_air_limit.predecessor_cooling_body_entered
            == predecessor.predecessor_cooling_body_entered
        && mixed_air_limit.predecessor_no_outdoor_air_fallback_entered
            == predecessor.predecessor_no_outdoor_air_fallback_entered
        && mixed_air_limit.predecessor_positive_supply_mass_flow_body_entered
            == predecessor.predecessor_positive_supply_mass_flow_body_entered
        && mixed_air_limit.predecessor_active_guard_false_fallthrough
            == predecessor.predecessor_active_guard_false_fallthrough
        && mixed_air_limit.unit_off_skipped == predecessor.unit_off_skipped
        && mixed_air_limit.non_cooling_skipped == predecessor.non_cooling_skipped
        && snapshot_shape(mixed_air_limit, predecessor, mixed_air)
}

fn checked_add(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_add(right).ok_or_else(|| {
        format!(
            "direct-zone IdealLoads cooling positive-supply mixed-air-temperature limit {label} overflowed"
        )
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads cooling positive-supply mixed-air-temperature limit invariant {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests {
    use ep_model::{IdealLoadsAirSystemId, ZoneId};
    use ep_runtime::PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitRuntimeState;

    use super::*;

    fn lifecycle_with_values(
        values: [f64; 4],
    ) -> PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitLifecycleSummary {
        let snapshot = PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER,
            system: IdealLoadsAirSystemId(0),
            parent_call_ordinal: 1,
            controlled_zone: ZoneId(0),
            unit_body_entered: true,
            predecessor_cooling_body_entered: true,
            predecessor_no_outdoor_air_fallback_entered: true,
            predecessor_positive_supply_mass_flow_body_entered: true,
            predecessor_active_guard_false_fallthrough: false,
            unit_off_skipped: false,
            non_cooling_skipped: false,
            positive_guard_false_fallthrough_skipped: false,
            supply_temperature_mixed_air_limit_executed: true,
            supply_temperature_for_minimum_read: true,
            supply_temperature_before_mixed_air_limit_c: Some(values[0]),
            mixed_air_temperature_for_minimum_read: true,
            mixed_air_temperature_c: Some(values[1]),
            source_shaped_two_argument_minimum_evaluated: true,
            minimum_supply_temperature_c: Some(values[2]),
            supply_temperature_assignment_performed: true,
            assigned_supply_temperature_c: Some(values[3]),
        };
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitRuntimeState::new(
                snapshot.system,
            );
        state.latest = Some(snapshot);
        PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitLifecycleSummary {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
            state,
        }
    }

    #[test]
    fn json_preserves_every_cp334_ieee_value_bit_pattern() {
        let values = [-0.0, 0.0, 0.0, 0.0];
        let value = lifecycle_json(&lifecycle_with_values(values));
        let latest = &value["latest"];
        for (field, expected) in [
            (
                "supply_temperature_before_mixed_air_limit_c_ieee_bits",
                values[0],
            ),
            ("mixed_air_temperature_c_ieee_bits", values[1]),
            ("minimum_supply_temperature_c_ieee_bits", values[2]),
            ("assigned_supply_temperature_c_ieee_bits", values[3]),
        ] {
            assert_eq!(latest[field], format!("0x{:016x}", expected.to_bits()));
        }
    }

    #[test]
    fn json_keeps_non_finite_bits_when_raw_value_is_null() {
        let nan = f64::from_bits(0x7ff8_0000_0000_1234);
        let value = lifecycle_json(&lifecycle_with_values([nan; 4]));
        let latest = &value["latest"];
        for field in [
            "supply_temperature_before_mixed_air_limit_c",
            "mixed_air_temperature_c",
            "minimum_supply_temperature_c",
            "assigned_supply_temperature_c",
        ] {
            assert!(latest[field].is_null(), "{field}");
        }
        for field in [
            "supply_temperature_before_mixed_air_limit_c_ieee_bits",
            "mixed_air_temperature_c_ieee_bits",
            "minimum_supply_temperature_c_ieee_bits",
            "assigned_supply_temperature_c_ieee_bits",
        ] {
            assert_eq!(
                latest[field],
                format!("0x{:016x}", nan.to_bits()),
                "{field}"
            );
        }
    }
}
