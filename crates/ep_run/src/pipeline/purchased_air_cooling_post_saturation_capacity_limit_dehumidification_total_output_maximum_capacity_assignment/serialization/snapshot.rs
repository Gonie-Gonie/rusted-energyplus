//! JSON serialization for one CP384 maximum-capacity assignment snapshot.

use ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentSnapshot,
) -> Value {
    json!({
        "source": snapshot.source,
        "first_excluded_source": snapshot.first_excluded_source,
        "source_order": snapshot.source_order,
        "system": snapshot.system.0,
        "parent_call_ordinal": snapshot.parent_call_ordinal,
        "controlled_zone": snapshot.controlled_zone.0,
        "unit_off_skipped": snapshot.unit_off_skipped,
        "non_cooling_skipped": snapshot.non_cooling_skipped,
        "positive_guard_false_fallthrough_skipped": snapshot.positive_guard_false_fallthrough_skipped,
        "heating_availability_guard_false_fallthrough": snapshot.heating_availability_guard_false_fallthrough,
        "humidification_control_guard_false_fallthrough": snapshot.humidification_control_guard_false_fallthrough,
        "dehumidification_control_humidistat_maximum_assignment_executed": snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        "dehumidification_control_none_maximum_assignment_executed": snapshot.dehumidification_control_none_maximum_assignment_executed,
        "dehumidification_control_guard_false_fallthrough": snapshot.dehumidification_control_guard_false_fallthrough,
        "predecessor_capacity_limit_guard_evaluated": snapshot.predecessor_capacity_limit_guard_evaluated,
        "predecessor_capacity_limit_body_entered": snapshot.predecessor_capacity_limit_body_entered,
        "predecessor_active_capacity_limit_guard_false_fallthrough": snapshot.predecessor_active_capacity_limit_guard_false_fallthrough,
        "predecessor_dehumidification_guard_evaluated": snapshot.predecessor_dehumidification_guard_evaluated,
        "predecessor_dehumidification_body_entered": snapshot.predecessor_dehumidification_body_entered,
        "predecessor_dehumidification_guard_false_fallthrough": snapshot.predecessor_dehumidification_guard_false_fallthrough,
        "predecessor_dehumidification_total_output_assignment_executed": snapshot.predecessor_dehumidification_total_output_assignment_executed,
        "predecessor_dehumidification_total_output_capacity_guard_evaluated": snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated,
        "predecessor_dehumidification_total_output_capacity_adjustment_body_entered": snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        "predecessor_dehumidification_total_output_capacity_guard_false_fallthrough": snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        "dehumidification_total_output_capacity_guard_false_fallthrough": snapshot.dehumidification_total_output_capacity_guard_false_fallthrough,
        "dehumidification_total_output_maximum_capacity_assignment_executed": snapshot.dehumidification_total_output_maximum_capacity_assignment_executed,
        "preexisting_cooling_total_output_w": json_number(snapshot.preexisting_cooling_total_output_w),
        "preexisting_cooling_total_output_w_ieee_bits": ieee_bits(snapshot.preexisting_cooling_total_output_w),
        "cp383_retained_maximum_total_cooling_capacity_owned_read": snapshot.cp383_retained_maximum_total_cooling_capacity_owned_read,
        "maximum_total_cooling_capacity_read": snapshot.maximum_total_cooling_capacity_read,
        "maximum_total_cooling_capacity_w": json_number(snapshot.maximum_total_cooling_capacity_w),
        "maximum_total_cooling_capacity_w_ieee_bits": ieee_bits(snapshot.maximum_total_cooling_capacity_w),
        "cooling_total_output_assigned": snapshot.cooling_total_output_assigned,
        "assigned_cooling_total_output_w": json_number(snapshot.assigned_cooling_total_output_w),
        "assigned_cooling_total_output_w_ieee_bits": ieee_bits(snapshot.assigned_cooling_total_output_w),
        "resulting_cooling_total_output_w": json_number(snapshot.resulting_cooling_total_output_w),
        "resulting_cooling_total_output_w_ieee_bits": ieee_bits(snapshot.resulting_cooling_total_output_w),
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
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER,
    };

    use super::*;

    #[test]
    fn generic_nonfinite_and_skip_serialization_preserves_exact_sidecars() {
        for bits in [0x7ff8_0000_0000_0384, f64::INFINITY.to_bits()] {
            let value = Some(f64::from_bits(bits));
            assert!(json_number(value).is_null());
            assert_eq!(ieee_bits(value), Some(format!("0x{bits:016x}")));
        }
        assert!(json_number(None).is_null());
        assert_eq!(ieee_bits(None), None);
    }

    #[test]
    fn outer_skip_serializes_every_numeric_value_and_sidecar_as_null() {
        let value = snapshot_json(snapshot(Route::OuterSkip));
        for field in numeric_fields() {
            assert!(value[field].is_null(), "{field}");
            assert!(
                value[format!("{field}_ieee_bits")].is_null(),
                "{field} bits"
            );
        }
    }

    #[test]
    fn guard_false_preserves_nonfinite_preexisting_and_result_bits_only() {
        let nan = f64::from_bits(0x7ff8_0000_0000_0384);
        let value = snapshot_json(snapshot(Route::GuardFalse(nan)));
        for field in [
            "preexisting_cooling_total_output_w",
            "resulting_cooling_total_output_w",
        ] {
            assert!(value[field].is_null(), "{field}");
            assert_eq!(value[format!("{field}_ieee_bits")], "0x7ff8000000000384");
        }
        for field in [
            "maximum_total_cooling_capacity_w",
            "maximum_total_cooling_capacity_w_ieee_bits",
            "assigned_cooling_total_output_w",
            "assigned_cooling_total_output_w_ieee_bits",
        ] {
            assert!(value[field].is_null(), "{field}");
        }
    }

    #[test]
    fn body_assignment_caps_nonfinite_preexisting_to_finite_maximum_bits() {
        let value = snapshot_json(snapshot(Route::Body {
            preexisting: f64::INFINITY,
            maximum: 10.0,
        }));
        assert!(value["preexisting_cooling_total_output_w"].is_null());
        assert_eq!(
            value["preexisting_cooling_total_output_w_ieee_bits"],
            "0x7ff0000000000000"
        );
        for field in [
            "maximum_total_cooling_capacity_w",
            "assigned_cooling_total_output_w",
            "resulting_cooling_total_output_w",
        ] {
            assert_eq!(value[field], 10.0, "{field}");
            assert_eq!(
                value[format!("{field}_ieee_bits")],
                "0x4024000000000000",
                "{field} bits"
            );
        }
    }

    #[derive(Clone, Copy)]
    enum Route {
        OuterSkip,
        GuardFalse(f64),
        Body { preexisting: f64, maximum: f64 },
    }

    fn snapshot(
        route: Route,
    ) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentSnapshot
    {
        let evaluated = !matches!(route, Route::OuterSkip);
        let guard_false = matches!(route, Route::GuardFalse(_));
        let assignment = matches!(route, Route::Body { .. });
        let preexisting = match route {
            Route::OuterSkip => None,
            Route::GuardFalse(value) => Some(value),
            Route::Body { preexisting, .. } => Some(preexisting),
        };
        let maximum = match route {
            Route::Body { maximum, .. } => Some(maximum),
            Route::OuterSkip | Route::GuardFalse(_) => None,
        };
        let resulting = if assignment { maximum } else { preexisting };
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentSnapshot {
            source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
            first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            source_order: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER,
            system: IdealLoadsAirSystemId(0),
            parent_call_ordinal: 1,
            controlled_zone: ZoneId(0),
            unit_off_skipped: false,
            non_cooling_skipped: false,
            positive_guard_false_fallthrough_skipped: false,
            heating_availability_guard_false_fallthrough: true,
            humidification_control_guard_false_fallthrough: false,
            dehumidification_control_humidistat_maximum_assignment_executed: false,
            dehumidification_control_none_maximum_assignment_executed: false,
            dehumidification_control_guard_false_fallthrough: false,
            predecessor_capacity_limit_guard_evaluated: true,
            predecessor_capacity_limit_body_entered: true,
            predecessor_active_capacity_limit_guard_false_fallthrough: false,
            predecessor_dehumidification_guard_evaluated: evaluated,
            predecessor_dehumidification_body_entered: evaluated,
            predecessor_dehumidification_guard_false_fallthrough: false,
            predecessor_dehumidification_total_output_assignment_executed: evaluated,
            predecessor_dehumidification_total_output_capacity_guard_evaluated: evaluated,
            predecessor_dehumidification_total_output_capacity_adjustment_body_entered: assignment,
            predecessor_dehumidification_total_output_capacity_guard_false_fallthrough: guard_false,
            dehumidification_total_output_capacity_guard_false_fallthrough: guard_false,
            dehumidification_total_output_maximum_capacity_assignment_executed: assignment,
            preexisting_cooling_total_output_w: preexisting,
            cp383_retained_maximum_total_cooling_capacity_owned_read: assignment,
            maximum_total_cooling_capacity_read: assignment,
            maximum_total_cooling_capacity_w: maximum,
            cooling_total_output_assigned: assignment,
            assigned_cooling_total_output_w: maximum,
            resulting_cooling_total_output_w: resulting,
        }
    }

    fn numeric_fields() -> [&'static str; 4] {
        [
            "preexisting_cooling_total_output_w",
            "maximum_total_cooling_capacity_w",
            "assigned_cooling_total_output_w",
            "resulting_cooling_total_output_w",
        ]
    }
}
