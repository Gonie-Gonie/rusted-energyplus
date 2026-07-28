//! JSON serialization for one CP344 snapshot.

use ep_runtime::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
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
        "predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed":
            snapshot.predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed,
        "predecessor_capacity_limit_sensible_output_supply_enthalpy_assignment_executed":
            snapshot.predecessor_capacity_limit_sensible_output_supply_enthalpy_assignment_executed,
        "predecessor_capacity_limit_sensible_output_supply_temperature_assignment_executed":
            snapshot.predecessor_capacity_limit_sensible_output_supply_temperature_assignment_executed,
        "unit_off_skipped": snapshot.unit_off_skipped,
        "non_cooling_skipped": snapshot.non_cooling_skipped,
        "positive_guard_false_fallthrough_skipped":
            snapshot.positive_guard_false_fallthrough_skipped,
        "capacity_limit_guard_false_fallthrough_skipped":
            snapshot.capacity_limit_guard_false_fallthrough_skipped,
        "capacity_limit_sensible_output_guard_false_fallthrough":
            snapshot.capacity_limit_sensible_output_guard_false_fallthrough,
        "capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed":
            snapshot.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed,
        "preexisting_supply_temperature_c":
            json_number(snapshot.preexisting_supply_temperature_c),
        "preexisting_supply_temperature_c_ieee_bits":
            ieee_bits(snapshot.preexisting_supply_temperature_c),
        "supply_temperature_for_minimum_read":
            snapshot.supply_temperature_for_minimum_read,
        "supply_temperature_before_mixed_air_limit_c":
            json_number(snapshot.supply_temperature_before_mixed_air_limit_c),
        "supply_temperature_before_mixed_air_limit_c_ieee_bits":
            ieee_bits(snapshot.supply_temperature_before_mixed_air_limit_c),
        "mixed_air_temperature_for_minimum_read":
            snapshot.mixed_air_temperature_for_minimum_read,
        "mixed_air_temperature_c":
            json_number(snapshot.mixed_air_temperature_c),
        "mixed_air_temperature_c_ieee_bits":
            ieee_bits(snapshot.mixed_air_temperature_c),
        "source_shaped_two_argument_minimum_evaluated":
            snapshot.source_shaped_two_argument_minimum_evaluated,
        "minimum_supply_temperature_c":
            json_number(snapshot.minimum_supply_temperature_c),
        "minimum_supply_temperature_c_ieee_bits":
            ieee_bits(snapshot.minimum_supply_temperature_c),
        "supply_temperature_assignment_performed":
            snapshot.supply_temperature_assignment_performed,
        "assigned_supply_temperature_c":
            json_number(snapshot.assigned_supply_temperature_c),
        "assigned_supply_temperature_c_ieee_bits":
            ieee_bits(snapshot.assigned_supply_temperature_c),
        "resulting_supply_temperature_c":
            json_number(snapshot.resulting_supply_temperature_c),
        "resulting_supply_temperature_c_ieee_bits":
            ieee_bits(snapshot.resulting_supply_temperature_c),
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
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER,
    };

    use super::*;

    #[test]
    fn executed_minimum_serializes_values_and_exact_ieee_bits() {
        let value = snapshot_json(snapshot(Route::Executed {
            left: 16.0,
            right: 18.0,
        }));
        for (field, expected) in [
            ("preexisting_supply_temperature_c", 16.0_f64),
            ("supply_temperature_before_mixed_air_limit_c", 16.0),
            ("mixed_air_temperature_c", 18.0),
            ("minimum_supply_temperature_c", 16.0),
            ("assigned_supply_temperature_c", 16.0),
            ("resulting_supply_temperature_c", 16.0),
        ] {
            assert_eq!(value[field], expected, "{field}");
            assert_eq!(
                value[format!("{field}_ieee_bits")],
                format!("0x{:016x}", expected.to_bits()),
                "{field}"
            );
        }
    }

    #[test]
    fn guard_false_serializes_preserved_temperature_and_null_rhs() {
        let value = snapshot_json(snapshot(Route::GuardFalse(-0.0)));
        for field in [
            "preexisting_supply_temperature_c",
            "resulting_supply_temperature_c",
        ] {
            assert_eq!(value[field], -0.0, "{field}");
            assert_eq!(
                value[format!("{field}_ieee_bits")],
                "0x8000000000000000",
                "{field}"
            );
        }
        for field in rhs_fields() {
            assert!(value[field].is_null(), "{field}");
            assert!(
                value[format!("{field}_ieee_bits")].is_null(),
                "{field} bits"
            );
        }
    }

    #[test]
    fn inherited_skip_serializes_every_optional_value_and_bits_as_null() {
        let value = snapshot_json(snapshot(Route::InheritedSkip));
        for field in [
            "preexisting_supply_temperature_c",
            "supply_temperature_before_mixed_air_limit_c",
            "mixed_air_temperature_c",
            "minimum_supply_temperature_c",
            "assigned_supply_temperature_c",
            "resulting_supply_temperature_c",
        ] {
            assert!(value[field].is_null(), "{field}");
            assert!(
                value[format!("{field}_ieee_bits")].is_null(),
                "{field} bits"
            );
        }
    }

    #[test]
    fn serializer_keeps_nonfinite_bits_while_numeric_values_are_null() {
        let nan = f64::from_bits(0x7ff8_0000_0000_3440);
        let value = snapshot_json(snapshot(Route::Executed {
            left: nan,
            right: 18.0,
        }));
        for field in [
            "preexisting_supply_temperature_c",
            "supply_temperature_before_mixed_air_limit_c",
        ] {
            assert!(value[field].is_null(), "{field}");
            assert_eq!(
                value[format!("{field}_ieee_bits")],
                "0x7ff8000000003440",
                "{field}"
            );
        }
        for field in [
            "minimum_supply_temperature_c",
            "assigned_supply_temperature_c",
            "resulting_supply_temperature_c",
        ] {
            assert_eq!(value[field], 18.0, "{field}");
            assert_eq!(
                value[format!("{field}_ieee_bits")],
                format!("0x{:016x}", 18.0_f64.to_bits()),
                "{field}"
            );
        }
    }

    fn rhs_fields() -> [&'static str; 4] {
        [
            "supply_temperature_before_mixed_air_limit_c",
            "mixed_air_temperature_c",
            "minimum_supply_temperature_c",
            "assigned_supply_temperature_c",
        ]
    }

    #[derive(Clone, Copy)]
    enum Route {
        InheritedSkip,
        GuardFalse(f64),
        Executed { left: f64, right: f64 },
    }

    fn snapshot(
        route: Route,
    ) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot
    {
        let inherited_skip = matches!(route, Route::InheritedSkip);
        let guard_false = matches!(route, Route::GuardFalse(_));
        let execution = matches!(route, Route::Executed { .. });
        let preexisting = match route {
            Route::InheritedSkip => None,
            Route::GuardFalse(value) | Route::Executed { left: value, .. } => Some(value),
        };
        let right = match route {
            Route::Executed { right, .. } => Some(right),
            Route::InheritedSkip | Route::GuardFalse(_) => None,
        };
        let minimum = preexisting
            .zip(right)
            .map(|(left, right)| if left < right { left } else { right });

        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER,
            system: IdealLoadsAirSystemId(0),
            parent_call_ordinal: 1,
            controlled_zone: ZoneId(0),
            unit_body_entered: !inherited_skip,
            predecessor_cooling_body_entered: !inherited_skip,
            predecessor_no_outdoor_air_fallback_entered: !inherited_skip,
            predecessor_positive_supply_mass_flow_body_entered: !inherited_skip,
            predecessor_active_guard_false_fallthrough: false,
            predecessor_capacity_limit_guard_evaluated: !inherited_skip,
            predecessor_capacity_limit_body_entered: !inherited_skip,
            predecessor_active_capacity_limit_guard_false_fallthrough: false,
            predecessor_capacity_limit_cp_air_assignment_executed: !inherited_skip,
            predecessor_capacity_limit_sensible_output_assignment_executed: !inherited_skip,
            predecessor_capacity_limit_sensible_output_guard_evaluated: !inherited_skip,
            predecessor_capacity_limit_sensible_output_guard_false_fallthrough: guard_false,
            predecessor_capacity_limit_sensible_output_adjustment_body_entered: execution,
            predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed:
                execution,
            predecessor_capacity_limit_sensible_output_supply_enthalpy_assignment_executed:
                execution,
            predecessor_capacity_limit_sensible_output_supply_temperature_assignment_executed:
                execution,
            unit_off_skipped: inherited_skip,
            non_cooling_skipped: false,
            positive_guard_false_fallthrough_skipped: false,
            capacity_limit_guard_false_fallthrough_skipped: false,
            capacity_limit_sensible_output_guard_false_fallthrough: guard_false,
            capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed: execution,
            preexisting_supply_temperature_c: preexisting,
            supply_temperature_for_minimum_read: execution,
            supply_temperature_before_mixed_air_limit_c: execution
                .then_some(preexisting)
                .flatten(),
            mixed_air_temperature_for_minimum_read: execution,
            mixed_air_temperature_c: right,
            source_shaped_two_argument_minimum_evaluated: execution,
            minimum_supply_temperature_c: minimum,
            supply_temperature_assignment_performed: execution,
            assigned_supply_temperature_c: minimum,
            resulting_supply_temperature_c: if execution { minimum } else { preexisting },
        }
    }
}
