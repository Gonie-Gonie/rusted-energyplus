//! JSON serialization for one CP343 snapshot.

use ep_runtime::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
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
        "unit_off_skipped": snapshot.unit_off_skipped,
        "non_cooling_skipped": snapshot.non_cooling_skipped,
        "positive_guard_false_fallthrough_skipped":
            snapshot.positive_guard_false_fallthrough_skipped,
        "capacity_limit_guard_false_fallthrough_skipped":
            snapshot.capacity_limit_guard_false_fallthrough_skipped,
        "capacity_limit_sensible_output_guard_false_fallthrough":
            snapshot.capacity_limit_sensible_output_guard_false_fallthrough,
        "capacity_limit_sensible_output_supply_temperature_assignment_executed":
            snapshot.capacity_limit_sensible_output_supply_temperature_assignment_executed,
        "preexisting_supply_temperature_c":
            json_number(snapshot.preexisting_supply_temperature_c),
        "preexisting_supply_temperature_c_ieee_bits":
            ieee_bits(snapshot.preexisting_supply_temperature_c),
        "supply_enthalpy_for_dry_bulb_inversion_read":
            snapshot.supply_enthalpy_for_dry_bulb_inversion_read,
        "supply_enthalpy_j_per_kg":
            json_number(snapshot.supply_enthalpy_j_per_kg),
        "supply_enthalpy_j_per_kg_ieee_bits":
            ieee_bits(snapshot.supply_enthalpy_j_per_kg),
        "supply_humidity_ratio_for_dry_bulb_inversion_read":
            snapshot.supply_humidity_ratio_for_dry_bulb_inversion_read,
        "supply_humidity_ratio":
            json_number(snapshot.supply_humidity_ratio),
        "supply_humidity_ratio_ieee_bits":
            ieee_bits(snapshot.supply_humidity_ratio),
        "psychrometric_supply_temperature_evaluated":
            snapshot.psychrometric_supply_temperature_evaluated,
        "psychrometric_supply_temperature_result_c":
            json_number(snapshot.psychrometric_supply_temperature_result_c),
        "psychrometric_supply_temperature_result_c_ieee_bits":
            ieee_bits(snapshot.psychrometric_supply_temperature_result_c),
        "supply_temperature_assigned": snapshot.supply_temperature_assigned,
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
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
    };

    use super::*;

    #[test]
    fn true_assignment_serializes_finite_values_and_exact_ieee_bits() {
        let value = snapshot_json(snapshot(Route::Assignment {
            preexisting: 16.0,
            enthalpy: 40_000.0,
            humidity: 0.008,
        }));
        let expected = ep_runtime::psychrometrics::energyplus_psy_tdb_fn_h_w(40_000.0, 0.008);
        for (field, expected) in [
            ("preexisting_supply_temperature_c", 16.0_f64),
            ("supply_enthalpy_j_per_kg", 40_000.0),
            ("supply_humidity_ratio", 0.008),
            ("psychrometric_supply_temperature_result_c", expected),
            ("assigned_supply_temperature_c", expected),
            ("resulting_supply_temperature_c", expected),
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
    fn false_guard_serializes_preserved_temperature_and_null_rhs() {
        let value = snapshot_json(snapshot(Route::GuardFalse(16.0)));
        for field in [
            "preexisting_supply_temperature_c",
            "resulting_supply_temperature_c",
        ] {
            assert_eq!(value[field], 16.0, "{field}");
            assert_eq!(
                value[format!("{field}_ieee_bits")],
                format!("0x{:016x}", 16.0_f64.to_bits()),
                "{field}"
            );
        }
        for field in active_value_fields() {
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
            "supply_enthalpy_j_per_kg",
            "supply_humidity_ratio",
            "psychrometric_supply_temperature_result_c",
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
    fn pure_assignment_characterization_serializes_nonfinite_result_as_null_and_bits() {
        // Serializer characterization only: this does not claim that the
        // complete public CP342-to-CP343 route reaches nonfinite operands.
        let value = snapshot_json(snapshot(Route::Assignment {
            preexisting: 16.0,
            enthalpy: f64::NEG_INFINITY,
            humidity: 0.008,
        }));
        assert!(value["supply_enthalpy_j_per_kg"].is_null());
        assert_eq!(
            value["supply_enthalpy_j_per_kg_ieee_bits"],
            "0xfff0000000000000"
        );
        for field in [
            "psychrometric_supply_temperature_result_c",
            "assigned_supply_temperature_c",
            "resulting_supply_temperature_c",
        ] {
            assert!(value[field].is_null(), "{field}");
            assert_eq!(
                value[format!("{field}_ieee_bits")],
                "0xfff0000000000000",
                "{field}"
            );
        }
    }

    fn active_value_fields() -> [&'static str; 4] {
        [
            "supply_enthalpy_j_per_kg",
            "supply_humidity_ratio",
            "psychrometric_supply_temperature_result_c",
            "assigned_supply_temperature_c",
        ]
    }

    #[derive(Clone, Copy)]
    enum Route {
        InheritedSkip,
        GuardFalse(f64),
        Assignment {
            preexisting: f64,
            enthalpy: f64,
            humidity: f64,
        },
    }

    fn snapshot(
        route: Route,
    ) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot
    {
        let inherited_skip = matches!(route, Route::InheritedSkip);
        let guard_false = matches!(route, Route::GuardFalse(_));
        let assignment = matches!(route, Route::Assignment { .. });
        let preexisting = match route {
            Route::InheritedSkip => None,
            Route::GuardFalse(value)
            | Route::Assignment {
                preexisting: value, ..
            } => Some(value),
        };
        let active = match route {
            Route::Assignment {
                enthalpy, humidity, ..
            } => {
                let result =
                    ep_runtime::psychrometrics::energyplus_psy_tdb_fn_h_w(enthalpy, humidity);
                Some((enthalpy, humidity, result))
            }
            Route::InheritedSkip | Route::GuardFalse(_) => None,
        };
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
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
            predecessor_capacity_limit_sensible_output_adjustment_body_entered: assignment,
            predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed:
                assignment,
            predecessor_capacity_limit_sensible_output_supply_enthalpy_assignment_executed:
                assignment,
            unit_off_skipped: inherited_skip,
            non_cooling_skipped: false,
            positive_guard_false_fallthrough_skipped: false,
            capacity_limit_guard_false_fallthrough_skipped: false,
            capacity_limit_sensible_output_guard_false_fallthrough: guard_false,
            capacity_limit_sensible_output_supply_temperature_assignment_executed: assignment,
            preexisting_supply_temperature_c: preexisting,
            supply_enthalpy_for_dry_bulb_inversion_read: assignment,
            supply_enthalpy_j_per_kg: active.map(|values| values.0),
            supply_humidity_ratio_for_dry_bulb_inversion_read: assignment,
            supply_humidity_ratio: active.map(|values| values.1),
            psychrometric_supply_temperature_evaluated: assignment,
            psychrometric_supply_temperature_result_c: active.map(|values| values.2),
            supply_temperature_assigned: assignment,
            assigned_supply_temperature_c: active.map(|values| values.2),
            resulting_supply_temperature_c: if assignment {
                active.map(|values| values.2)
            } else {
                preexisting
            },
        }
    }
}
