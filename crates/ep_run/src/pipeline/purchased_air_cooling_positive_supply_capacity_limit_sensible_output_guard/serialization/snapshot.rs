//! JSON serialization for one CP340 snapshot.

use ep_runtime::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
) -> Value {
    json!({
        "source": snapshot.source,
        "first_excluded_source": snapshot.first_excluded_source,
        "source_order": snapshot.source_order,
        "system": snapshot.system.0,
        "parent_call_ordinal": snapshot.parent_call_ordinal,
        "controlled_zone": snapshot.controlled_zone.0,
        "unit_body_entered": snapshot.unit_body_entered,
        "predecessor_cooling_body_entered": snapshot.predecessor_cooling_body_entered,
        "predecessor_no_outdoor_air_fallback_entered":
            snapshot.predecessor_no_outdoor_air_fallback_entered,
        "predecessor_positive_supply_mass_flow_body_entered":
            snapshot.predecessor_positive_supply_mass_flow_body_entered,
        "predecessor_active_guard_false_fallthrough":
            snapshot.predecessor_active_guard_false_fallthrough,
        "predecessor_capacity_limit_guard_evaluated":
            snapshot.predecessor_capacity_limit_guard_evaluated,
        "predecessor_capacity_limit_body_entered":
            snapshot.predecessor_capacity_limit_body_entered,
        "predecessor_active_capacity_limit_guard_false_fallthrough":
            snapshot.predecessor_active_capacity_limit_guard_false_fallthrough,
        "predecessor_capacity_limit_cp_air_assignment_executed":
            snapshot.predecessor_capacity_limit_cp_air_assignment_executed,
        "predecessor_capacity_limit_sensible_output_assignment_executed":
            snapshot.predecessor_capacity_limit_sensible_output_assignment_executed,
        "unit_off_skipped": snapshot.unit_off_skipped,
        "non_cooling_skipped": snapshot.non_cooling_skipped,
        "positive_guard_false_fallthrough_skipped":
            snapshot.positive_guard_false_fallthrough_skipped,
        "capacity_limit_guard_false_fallthrough_skipped":
            snapshot.capacity_limit_guard_false_fallthrough_skipped,
        "capacity_limit_sensible_output_guard_evaluated":
            snapshot.capacity_limit_sensible_output_guard_evaluated,
        "cooling_sensible_output_read": snapshot.cooling_sensible_output_read,
        "cooling_sensible_output_w": json_number(snapshot.cooling_sensible_output_w),
        "cooling_sensible_output_w_ieee_bits": ieee_bits(snapshot.cooling_sensible_output_w),
        "maximum_total_cooling_capacity_read":
            snapshot.maximum_total_cooling_capacity_read,
        "maximum_total_cooling_capacity_w":
            json_number(snapshot.maximum_total_cooling_capacity_w),
        "maximum_total_cooling_capacity_w_ieee_bits":
            ieee_bits(snapshot.maximum_total_cooling_capacity_w),
        "cooling_sensible_output_maximum_capacity_comparison_evaluated":
            snapshot.cooling_sensible_output_maximum_capacity_comparison_evaluated,
        "cooling_sensible_output_at_or_above_maximum_capacity":
            snapshot.cooling_sensible_output_at_or_above_maximum_capacity,
        "capacity_limit_sensible_output_guard_false_fallthrough":
            snapshot.capacity_limit_sensible_output_guard_false_fallthrough,
        "capacity_limit_sensible_output_adjustment_body_entered":
            snapshot.capacity_limit_sensible_output_adjustment_body_entered,
    })
}

fn json_number(value: Option<f64>) -> Value {
    value
        .filter(|value| value.is_finite())
        .map_or(Value::Null, |value| json!(value))
}

fn ieee_bits(value: Option<f64>) -> Option<String> {
    value.map(|value| format!("0x{:016x}", value.to_bits()))
}

#[cfg(test)]
mod tests {
    use ep_model::{IdealLoadsAirSystemId, ZoneId};
    use ep_runtime::{
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE_ORDER,
    };

    use super::*;

    #[test]
    fn active_nonfinite_left_retains_bits_and_non_null_comparison() {
        for (left, expected_comparison) in [
            (f64::INFINITY, true),
            (f64::NEG_INFINITY, false),
            (f64::from_bits(0x7ff8_0000_0000_0042), false),
        ] {
            let value = snapshot_json(snapshot(Some((left, 10.0))));
            assert!(value["cooling_sensible_output_w"].is_null());
            assert_eq!(
                value["cooling_sensible_output_w_ieee_bits"],
                format!("0x{:016x}", left.to_bits())
            );
            assert_eq!(
                value["cooling_sensible_output_at_or_above_maximum_capacity"],
                expected_comparison
            );
            assert!(!value["cooling_sensible_output_at_or_above_maximum_capacity"].is_null());
        }
    }

    #[test]
    fn generic_serializer_preserves_signed_zero_capacity_bits() {
        let value = snapshot_json(snapshot(Some((1.0, -0.0))));
        assert_eq!(value["maximum_total_cooling_capacity_w"], json!(-0.0));
        assert_eq!(
            value["maximum_total_cooling_capacity_w_ieee_bits"],
            "0x8000000000000000"
        );
        assert_eq!(
            value["cooling_sensible_output_at_or_above_maximum_capacity"],
            true
        );
    }

    #[test]
    fn skipped_snapshot_serializes_optional_evidence_as_null() {
        let value = snapshot_json(snapshot(None));
        for field in [
            "cooling_sensible_output_w",
            "cooling_sensible_output_w_ieee_bits",
            "maximum_total_cooling_capacity_w",
            "maximum_total_cooling_capacity_w_ieee_bits",
            "cooling_sensible_output_at_or_above_maximum_capacity",
        ] {
            assert!(value[field].is_null(), "{field}");
        }
        for field in [
            "capacity_limit_sensible_output_guard_evaluated",
            "cooling_sensible_output_read",
            "maximum_total_cooling_capacity_read",
            "cooling_sensible_output_maximum_capacity_comparison_evaluated",
            "capacity_limit_sensible_output_guard_false_fallthrough",
            "capacity_limit_sensible_output_adjustment_body_entered",
        ] {
            assert_eq!(value[field], false, "{field}");
        }
    }

    fn snapshot(
        values: Option<(f64, f64)>,
    ) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot {
        let active = values.is_some();
        let comparison = values.map(|(left, right)| left >= right);
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE_ORDER,
            system: IdealLoadsAirSystemId(0),
            parent_call_ordinal: 1,
            controlled_zone: ZoneId(0),
            unit_body_entered: true,
            predecessor_cooling_body_entered: true,
            predecessor_no_outdoor_air_fallback_entered: true,
            predecessor_positive_supply_mass_flow_body_entered: true,
            predecessor_active_guard_false_fallthrough: false,
            predecessor_capacity_limit_guard_evaluated: active,
            predecessor_capacity_limit_body_entered: active,
            predecessor_active_capacity_limit_guard_false_fallthrough: false,
            predecessor_capacity_limit_cp_air_assignment_executed: active,
            predecessor_capacity_limit_sensible_output_assignment_executed: active,
            unit_off_skipped: false,
            non_cooling_skipped: false,
            positive_guard_false_fallthrough_skipped: false,
            capacity_limit_guard_false_fallthrough_skipped: !active,
            capacity_limit_sensible_output_guard_evaluated: active,
            cooling_sensible_output_read: active,
            cooling_sensible_output_w: values.map(|values| values.0),
            maximum_total_cooling_capacity_read: active,
            maximum_total_cooling_capacity_w: values.map(|values| values.1),
            cooling_sensible_output_maximum_capacity_comparison_evaluated: active,
            cooling_sensible_output_at_or_above_maximum_capacity: comparison,
            capacity_limit_sensible_output_guard_false_fallthrough:
                comparison == Some(false),
            capacity_limit_sensible_output_adjustment_body_entered:
                comparison == Some(true),
        }
    }
}
