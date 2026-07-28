//! Run-summary evidence for the bounded cooling positive-supply enthalpy assignment.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
    PurchasedAirInitLifecycleSummary,
};

mod serialization;
mod validation;

pub(super) use serialization::lifecycle_json;
use validation::{snapshot_shape, validate_source_counters};

pub(super) fn validate_direct_lifecycle(
    lifecycle: Option<&PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentLifecycleSummary>,
    predecessor_cp335: Option<
        &PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentLifecycleSummary,
    >,
    temperature_cp334: Option<
        &PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitLifecycleSummary,
    >,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose cooling positive-supply enthalpy assignment evidence"
            .to_string()
    })?;
    let predecessor = predecessor_cp335.ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply enthalpy assignment has no CP335 evidence"
            .to_string()
    })?;
    let temperature = temperature_cp334.ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply enthalpy assignment has no CP334 evidence"
            .to_string()
    })?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply enthalpy assignment has no initialization evidence"
            .to_string()
    })?;
    let calls = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply enthalpy assignment has no coupling call count"
            .to_string()
    })?;

    if calls == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || temperature.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE
        || temperature.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE
    {
        return Err(
            "direct-zone IdealLoads cooling positive-supply enthalpy assignment provenance is invalid"
                .to_string(),
        );
    }

    let state = &lifecycle.state;
    let predecessor_state = &predecessor.state;
    let temperature_state = &temperature.state;
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
        state.supply_enthalpy_assignment_count,
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
            "temperature_transition_count",
            temperature_state.transition_count,
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
            "supply_enthalpy_assignment_count",
            predecessor_state.supply_humidity_ratio_mixed_air_assignment_count,
            state.supply_enthalpy_assignment_count,
        ),
        (
            "temperature_assignment_count",
            temperature_state.supply_temperature_mixed_air_limit_count,
            state.supply_enthalpy_assignment_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    validate_source_counters(state)?;

    let latest = state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply enthalpy assignment has no latest snapshot"
            .to_string()
    })?;
    let predecessor_latest = predecessor_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply enthalpy assignment has no latest CP335 snapshot"
            .to_string()
    })?;
    let temperature_latest = temperature_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply enthalpy assignment has no latest CP334 snapshot"
            .to_string()
    })?;
    let expected_system = init.declared_system_order.first().copied().ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply enthalpy assignment has no declared system"
            .to_string()
    })?;
    let expected_zone = init.controlled_zone.ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply enthalpy assignment has no controlled Zone"
            .to_string()
    })?;
    if state.system != expected_system
        || predecessor_state.system != expected_system
        || temperature_state.system != expected_system
        || !latest_matches_release(
            latest,
            predecessor_latest,
            temperature_latest,
            expected_system,
            expected_zone,
            calls,
        )
    {
        return Err(
            "direct-zone IdealLoads cooling positive-supply enthalpy assignment latest state is not release-ready"
                .to_string(),
        );
    }
    Ok(())
}

fn latest_matches_release(
    assignment: &PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    predecessor: &PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    temperature: &PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
    expected_system: ep_model::IdealLoadsAirSystemId,
    expected_zone: ep_model::ZoneId,
    call_count: usize,
) -> bool {
    assignment.source == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE
        && assignment.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && assignment.source_order
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER
        && predecessor.source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE
        && predecessor.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && predecessor.source_order
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER
        && temperature.source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE
        && temperature.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE
        && temperature.source_order
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER
        && assignment.system == expected_system
        && predecessor.system == expected_system
        && temperature.system == expected_system
        && assignment.parent_call_ordinal == call_count
        && predecessor.parent_call_ordinal == call_count
        && temperature.parent_call_ordinal == call_count
        && assignment.controlled_zone == expected_zone
        && predecessor.controlled_zone == expected_zone
        && temperature.controlled_zone == expected_zone
        && assignment.unit_body_entered == predecessor.unit_body_entered
        && assignment.predecessor_cooling_body_entered
            == predecessor.predecessor_cooling_body_entered
        && assignment.predecessor_no_outdoor_air_fallback_entered
            == predecessor.predecessor_no_outdoor_air_fallback_entered
        && assignment.predecessor_positive_supply_mass_flow_body_entered
            == predecessor.predecessor_positive_supply_mass_flow_body_entered
        && assignment.predecessor_active_guard_false_fallthrough
            == predecessor.predecessor_active_guard_false_fallthrough
        && assignment.unit_off_skipped == predecessor.unit_off_skipped
        && assignment.non_cooling_skipped == predecessor.non_cooling_skipped
        && snapshot_shape(assignment, predecessor, temperature)
}

fn checked_add(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_add(right).ok_or_else(|| {
        format!(
            "direct-zone IdealLoads cooling positive-supply enthalpy assignment {label} overflowed"
        )
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads cooling positive-supply enthalpy assignment invariant {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests {
    use ep_model::{IdealLoadsAirSystemId, ZoneId};
    use ep_runtime::PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentRuntimeState;
    use ep_runtime::psychrometrics::energyplus_psy_h_fn_tdb_w;

    use super::*;

    fn lifecycle_with_values(
        values: [f64; 4],
    ) -> PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentLifecycleSummary {
        let snapshot = PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot {
            source: PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
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
            supply_enthalpy_assignment_executed: true,
            supply_temperature_for_enthalpy_read: true,
            supply_temperature_c: Some(values[0]),
            supply_humidity_ratio_for_enthalpy_read: true,
            supply_humidity_ratio: Some(values[1]),
            psychrometric_supply_enthalpy_evaluated: true,
            psychrometric_supply_enthalpy_result_j_per_kg: Some(values[2]),
            supply_enthalpy_assigned: true,
            supply_enthalpy_j_per_kg: Some(values[3]),
        };
        let mut state = PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentRuntimeState::new(
            snapshot.system,
        );
        state.latest = Some(snapshot);
        PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            state,
        }
    }

    #[test]
    fn json_preserves_signed_zero_humidity_and_the_canonical_floor_result() {
        let expected = energyplus_psy_h_fn_tdb_w(24.0, -0.0);
        let value = lifecycle_json(&lifecycle_with_values([24.0, -0.0, expected, expected]));
        let latest = &value["latest"];

        assert_eq!(
            latest["supply_humidity_ratio_ieee_bits"],
            "0x8000000000000000"
        );
        assert_eq!(
            latest["psychrometric_supply_enthalpy_result_j_per_kg_ieee_bits"],
            format!("0x{:016x}", expected.to_bits())
        );
        assert_eq!(
            latest["supply_enthalpy_j_per_kg_ieee_bits"],
            format!("0x{:016x}", expected.to_bits())
        );
    }

    #[test]
    fn json_keeps_non_finite_bits_when_raw_values_are_null() {
        let nan = f64::from_bits(0x7ff8_0000_0000_1234);
        let value = lifecycle_json(&lifecycle_with_values([nan; 4]));
        let latest = &value["latest"];
        for field in [
            "supply_temperature_c",
            "supply_humidity_ratio",
            "psychrometric_supply_enthalpy_result_j_per_kg",
            "supply_enthalpy_j_per_kg",
        ] {
            assert!(latest[field].is_null(), "{field}");
        }
        for field in [
            "supply_temperature_c_ieee_bits",
            "supply_humidity_ratio_ieee_bits",
            "psychrometric_supply_enthalpy_result_j_per_kg_ieee_bits",
            "supply_enthalpy_j_per_kg_ieee_bits",
        ] {
            assert_eq!(
                latest[field],
                format!("0x{:016x}", nan.to_bits()),
                "{field}"
            );
        }
    }
}
