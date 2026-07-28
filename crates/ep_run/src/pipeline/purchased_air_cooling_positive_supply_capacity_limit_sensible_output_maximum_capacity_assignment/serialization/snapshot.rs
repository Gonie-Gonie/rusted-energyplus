//! JSON serialization for one CP341 snapshot.

use ep_runtime::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
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
        "predecessor_capacity_limit_sensible_output_guard_evaluated":
            snapshot.predecessor_capacity_limit_sensible_output_guard_evaluated,
        "predecessor_capacity_limit_sensible_output_guard_false_fallthrough":
            snapshot.predecessor_capacity_limit_sensible_output_guard_false_fallthrough,
        "predecessor_capacity_limit_sensible_output_adjustment_body_entered":
            snapshot.predecessor_capacity_limit_sensible_output_adjustment_body_entered,
        "unit_off_skipped": snapshot.unit_off_skipped,
        "non_cooling_skipped": snapshot.non_cooling_skipped,
        "positive_guard_false_fallthrough_skipped":
            snapshot.positive_guard_false_fallthrough_skipped,
        "capacity_limit_guard_false_fallthrough_skipped":
            snapshot.capacity_limit_guard_false_fallthrough_skipped,
        "capacity_limit_sensible_output_guard_false_fallthrough":
            snapshot.capacity_limit_sensible_output_guard_false_fallthrough,
        "capacity_limit_sensible_output_maximum_capacity_assignment_executed":
            snapshot.capacity_limit_sensible_output_maximum_capacity_assignment_executed,
        "preexisting_cooling_sensible_output_w":
            json_number(snapshot.preexisting_cooling_sensible_output_w),
        "preexisting_cooling_sensible_output_w_ieee_bits":
            ieee_bits(snapshot.preexisting_cooling_sensible_output_w),
        "maximum_total_cooling_capacity_read":
            snapshot.maximum_total_cooling_capacity_read,
        "maximum_total_cooling_capacity_w":
            json_number(snapshot.maximum_total_cooling_capacity_w),
        "maximum_total_cooling_capacity_w_ieee_bits":
            ieee_bits(snapshot.maximum_total_cooling_capacity_w),
        "cooling_sensible_output_assigned": snapshot.cooling_sensible_output_assigned,
        "assigned_cooling_sensible_output_w":
            json_number(snapshot.assigned_cooling_sensible_output_w),
        "assigned_cooling_sensible_output_w_ieee_bits":
            ieee_bits(snapshot.assigned_cooling_sensible_output_w),
        "resulting_cooling_sensible_output_w":
            json_number(snapshot.resulting_cooling_sensible_output_w),
        "resulting_cooling_sensible_output_w_ieee_bits":
            ieee_bits(snapshot.resulting_cooling_sensible_output_w),
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
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER,
    };

    use super::*;

    #[test]
    fn true_assignment_serializes_nonfinite_predecessor_and_finite_rhs_assigned_result() {
        let value = snapshot_json(snapshot(Route::Assignment {
            preexisting: f64::INFINITY,
            maximum: 10.0,
        }));
        assert!(value["preexisting_cooling_sensible_output_w"].is_null());
        assert_eq!(
            value["preexisting_cooling_sensible_output_w_ieee_bits"],
            "0x7ff0000000000000"
        );
        for field in [
            "maximum_total_cooling_capacity_w",
            "assigned_cooling_sensible_output_w",
            "resulting_cooling_sensible_output_w",
        ] {
            assert_eq!(value[field], 10.0, "{field}");
        }
        for field in [
            "maximum_total_cooling_capacity_w_ieee_bits",
            "assigned_cooling_sensible_output_w_ieee_bits",
            "resulting_cooling_sensible_output_w_ieee_bits",
        ] {
            assert_eq!(value[field], "0x4024000000000000", "{field}");
        }
    }

    #[test]
    fn false_guard_serializes_nan_predecessor_and_result_with_null_rhs_and_assigned() {
        let nan = f64::from_bits(0x7ff8_0000_0000_0042);
        let value = snapshot_json(snapshot(Route::GuardFalse(nan)));
        for field in [
            "preexisting_cooling_sensible_output_w",
            "resulting_cooling_sensible_output_w",
            "maximum_total_cooling_capacity_w",
            "assigned_cooling_sensible_output_w",
        ] {
            assert!(value[field].is_null(), "{field}");
        }
        for field in [
            "preexisting_cooling_sensible_output_w_ieee_bits",
            "resulting_cooling_sensible_output_w_ieee_bits",
        ] {
            assert_eq!(value[field], "0x7ff8000000000042", "{field}");
        }
        for field in [
            "maximum_total_cooling_capacity_w_ieee_bits",
            "assigned_cooling_sensible_output_w_ieee_bits",
        ] {
            assert!(value[field].is_null(), "{field}");
        }
        assert_eq!(
            value["capacity_limit_sensible_output_guard_false_fallthrough"],
            true
        );
    }

    #[test]
    fn inherited_skip_serializes_every_optional_value_and_bits_as_null() {
        let value = snapshot_json(snapshot(Route::InheritedSkip));
        for field in [
            "preexisting_cooling_sensible_output_w",
            "preexisting_cooling_sensible_output_w_ieee_bits",
            "maximum_total_cooling_capacity_w",
            "maximum_total_cooling_capacity_w_ieee_bits",
            "assigned_cooling_sensible_output_w",
            "assigned_cooling_sensible_output_w_ieee_bits",
            "resulting_cooling_sensible_output_w",
            "resulting_cooling_sensible_output_w_ieee_bits",
        ] {
            assert!(value[field].is_null(), "{field}");
        }
    }

    #[derive(Clone, Copy)]
    enum Route {
        InheritedSkip,
        GuardFalse(f64),
        Assignment { preexisting: f64, maximum: f64 },
    }

    fn snapshot(
        route: Route,
    ) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot
    {
        let evaluated = !matches!(route, Route::InheritedSkip);
        let guard_false = matches!(route, Route::GuardFalse(_));
        let assignment = matches!(route, Route::Assignment { .. });
        let preexisting = match route {
            Route::InheritedSkip => None,
            Route::GuardFalse(value) => Some(value),
            Route::Assignment { preexisting, .. } => Some(preexisting),
        };
        let maximum = match route {
            Route::Assignment { maximum, .. } => Some(maximum),
            Route::InheritedSkip | Route::GuardFalse(_) => None,
        };
        let result = if assignment { maximum } else { preexisting };
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER,
            system: IdealLoadsAirSystemId(0),
            parent_call_ordinal: 1,
            controlled_zone: ZoneId(0),
            unit_body_entered: true,
            predecessor_cooling_body_entered: true,
            predecessor_no_outdoor_air_fallback_entered: true,
            predecessor_positive_supply_mass_flow_body_entered: true,
            predecessor_active_guard_false_fallthrough: false,
            predecessor_capacity_limit_guard_evaluated: evaluated,
            predecessor_capacity_limit_body_entered: evaluated,
            predecessor_active_capacity_limit_guard_false_fallthrough: false,
            predecessor_capacity_limit_cp_air_assignment_executed: evaluated,
            predecessor_capacity_limit_sensible_output_assignment_executed: evaluated,
            predecessor_capacity_limit_sensible_output_guard_evaluated: evaluated,
            predecessor_capacity_limit_sensible_output_guard_false_fallthrough: guard_false,
            predecessor_capacity_limit_sensible_output_adjustment_body_entered: assignment,
            unit_off_skipped: false,
            non_cooling_skipped: false,
            positive_guard_false_fallthrough_skipped: false,
            capacity_limit_guard_false_fallthrough_skipped: !evaluated,
            capacity_limit_sensible_output_guard_false_fallthrough: guard_false,
            capacity_limit_sensible_output_maximum_capacity_assignment_executed: assignment,
            preexisting_cooling_sensible_output_w: preexisting,
            maximum_total_cooling_capacity_read: assignment,
            maximum_total_cooling_capacity_w: maximum,
            cooling_sensible_output_assigned: assignment,
            assigned_cooling_sensible_output_w: maximum,
            resulting_cooling_sensible_output_w: result,
        }
    }
}
