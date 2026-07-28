//! Run-summary evidence for the bounded cooling positive-supply temperature minimum limit.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
    PurchasedAirCalcCoolingSensibleFlowLifecycleSummary, PurchasedAirInitLifecycleSummary,
};

mod serialization;
mod validation;

pub(super) use serialization::lifecycle_json;
use validation::{snapshot_shape, validate_source_counters};

#[allow(clippy::too_many_arguments)]
pub(super) fn validate_direct_lifecycle(
    lifecycle: Option<
        &PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitLifecycleSummary,
    >,
    predecessor_cp332: Option<
        &PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentLifecycleSummary,
    >,
    sensible_flow_cp318: Option<&PurchasedAirCalcCoolingSensibleFlowLifecycleSummary>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    typed_minimum_cooling_supply_air_temperature_c: Option<f64>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose cooling positive-supply temperature minimum-limit evidence"
            .to_string()
    })?;
    let predecessor = predecessor_cp332.ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply temperature minimum limit has no CP332 evidence"
            .to_string()
    })?;
    let sensible_flow = sensible_flow_cp318.ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply temperature minimum limit has no CP318 evidence"
            .to_string()
    })?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply temperature minimum limit has no initialization evidence"
            .to_string()
    })?;
    let typed_minimum_cooling_supply_air_temperature_c =
        typed_minimum_cooling_supply_air_temperature_c.ok_or_else(|| {
            "direct-zone IdealLoads cooling positive-supply temperature minimum limit has no typed-system minimum"
                .to_string()
        })?;
    let calls = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply temperature minimum limit has no coupling call count"
            .to_string()
    })?;

    if calls == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
    {
        return Err(
            "direct-zone IdealLoads cooling positive-supply temperature minimum-limit provenance is invalid"
                .to_string(),
        );
    }

    let state = &lifecycle.state;
    let predecessor_state = &predecessor.state;
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
        state.supply_temperature_minimum_limit_count,
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
            "supply_temperature_minimum_limit_count",
            predecessor_state.supply_temperature_assignment_count,
            state.supply_temperature_minimum_limit_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    validate_source_counters(state)?;

    let latest = state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply temperature minimum limit has no latest snapshot"
            .to_string()
    })?;
    let predecessor_latest = predecessor_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply temperature minimum limit has no latest CP332 snapshot"
            .to_string()
    })?;
    let sensible_flow_latest = sensible_flow.state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply temperature minimum limit has no latest CP318 snapshot"
            .to_string()
    })?;
    let expected_system = init.declared_system_order.first().copied().ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply temperature minimum limit has no declared system"
            .to_string()
    })?;
    let expected_zone = init.controlled_zone.ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply temperature minimum limit has no controlled Zone"
            .to_string()
    })?;
    if state.system != expected_system
        || predecessor_state.system != expected_system
        || sensible_flow.state.system != expected_system
        || !latest_matches_release(
            latest,
            predecessor_latest,
            sensible_flow_latest,
            typed_minimum_cooling_supply_air_temperature_c,
            expected_system,
            expected_zone,
            calls,
        )
    {
        return Err(
            "direct-zone IdealLoads cooling positive-supply temperature minimum-limit latest state is not release-ready"
                .to_string(),
        );
    }
    Ok(())
}

fn latest_matches_release(
    minimum_limit: &PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
    predecessor: &PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
    sensible_flow: &ep_runtime::PurchasedAirCalcCoolingSensibleFlowSnapshot,
    typed_minimum_cooling_supply_air_temperature_c: f64,
    expected_system: ep_model::IdealLoadsAirSystemId,
    expected_zone: ep_model::ZoneId,
    call_count: usize,
) -> bool {
    minimum_limit.source
        == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_SOURCE
        && minimum_limit.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_FIRST_EXCLUDED_SOURCE
        && minimum_limit.source_order
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_SOURCE_ORDER
        && predecessor.source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE
        && predecessor.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && predecessor.source_order
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER
        && minimum_limit.system == expected_system
        && predecessor.system == expected_system
        && sensible_flow.system == expected_system
        && minimum_limit.parent_call_ordinal == call_count
        && predecessor.parent_call_ordinal == call_count
        && sensible_flow.parent_call_ordinal == call_count
        && minimum_limit.controlled_zone == expected_zone
        && predecessor.controlled_zone == expected_zone
        && sensible_flow.controlled_zone == expected_zone
        && minimum_limit.unit_body_entered == predecessor.unit_body_entered
        && minimum_limit.predecessor_cooling_body_entered
            == predecessor.predecessor_cooling_body_entered
        && minimum_limit.predecessor_no_outdoor_air_fallback_entered
            == predecessor.predecessor_no_outdoor_air_fallback_entered
        && minimum_limit.predecessor_positive_supply_mass_flow_body_entered
            == predecessor.predecessor_positive_supply_mass_flow_body_entered
        && minimum_limit.predecessor_active_guard_false_fallthrough
            == predecessor.predecessor_active_guard_false_fallthrough
        && minimum_limit.unit_off_skipped == predecessor.unit_off_skipped
        && minimum_limit.non_cooling_skipped == predecessor.non_cooling_skipped
        && snapshot_shape(
            minimum_limit,
            predecessor,
            sensible_flow,
            typed_minimum_cooling_supply_air_temperature_c,
        )
}

fn checked_add(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_add(right).ok_or_else(|| {
        format!(
            "direct-zone IdealLoads cooling positive-supply temperature minimum limit {label} overflowed"
        )
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads cooling positive-supply temperature minimum-limit invariant {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests {
    use ep_model::{IdealLoadsAirSystemId, ZoneId};
    use ep_runtime::PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitRuntimeState;

    use super::*;

    fn lifecycle_with_values(
        values: [f64; 4],
    ) -> PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitLifecycleSummary {
        let snapshot = PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_SOURCE_ORDER,
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
            supply_temperature_minimum_limit_executed: true,
            supply_temperature_for_maximum_read: true,
            supply_temperature_before_minimum_limit_c: Some(values[0]),
            minimum_cooling_supply_air_temperature_for_maximum_read: true,
            minimum_cooling_supply_air_temperature_c: Some(values[1]),
            source_shaped_two_argument_maximum_evaluated: true,
            maximum_supply_temperature_c: Some(values[2]),
            supply_temperature_assignment_performed: true,
            assigned_supply_temperature_c: Some(values[3]),
        };
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitRuntimeState::new(
                snapshot.system,
            );
        state.latest = Some(snapshot);
        PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitLifecycleSummary {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_FIRST_EXCLUDED_SOURCE,
            state,
        }
    }

    #[test]
    fn json_preserves_every_cp333_ieee_value_bit_pattern() {
        let values = [-0.0, 13.0, -0.0, -0.0];
        let value = lifecycle_json(&lifecycle_with_values(values));
        let latest = &value["latest"];
        for (field, expected) in [
            (
                "supply_temperature_before_minimum_limit_c_ieee_bits",
                values[0],
            ),
            (
                "minimum_cooling_supply_air_temperature_c_ieee_bits",
                values[1],
            ),
            ("maximum_supply_temperature_c_ieee_bits", values[2]),
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
            "supply_temperature_before_minimum_limit_c",
            "minimum_cooling_supply_air_temperature_c",
            "maximum_supply_temperature_c",
            "assigned_supply_temperature_c",
        ] {
            assert!(latest[field].is_null(), "{field}");
        }
        for field in [
            "supply_temperature_before_minimum_limit_c_ieee_bits",
            "minimum_cooling_supply_air_temperature_c_ieee_bits",
            "maximum_supply_temperature_c_ieee_bits",
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
