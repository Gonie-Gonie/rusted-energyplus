//! JSON serialization for one CP342 snapshot.

use ep_runtime::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
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
        "unit_off_skipped": snapshot.unit_off_skipped,
        "non_cooling_skipped": snapshot.non_cooling_skipped,
        "positive_guard_false_fallthrough_skipped":
            snapshot.positive_guard_false_fallthrough_skipped,
        "capacity_limit_guard_false_fallthrough_skipped":
            snapshot.capacity_limit_guard_false_fallthrough_skipped,
        "capacity_limit_sensible_output_guard_false_fallthrough":
            snapshot.capacity_limit_sensible_output_guard_false_fallthrough,
        "capacity_limit_sensible_output_supply_enthalpy_assignment_executed":
            snapshot.capacity_limit_sensible_output_supply_enthalpy_assignment_executed,
        "preexisting_supply_enthalpy_j_per_kg":
            json_number(snapshot.preexisting_supply_enthalpy_j_per_kg),
        "preexisting_supply_enthalpy_j_per_kg_ieee_bits":
            ieee_bits(snapshot.preexisting_supply_enthalpy_j_per_kg),
        "mixed_air_enthalpy_read": snapshot.mixed_air_enthalpy_read,
        "mixed_air_enthalpy_j_per_kg":
            json_number(snapshot.mixed_air_enthalpy_j_per_kg),
        "mixed_air_enthalpy_j_per_kg_ieee_bits":
            ieee_bits(snapshot.mixed_air_enthalpy_j_per_kg),
        "cooling_sensible_output_read": snapshot.cooling_sensible_output_read,
        "cooling_sensible_output_w":
            json_number(snapshot.cooling_sensible_output_w),
        "cooling_sensible_output_w_ieee_bits":
            ieee_bits(snapshot.cooling_sensible_output_w),
        "supply_mass_flow_rate_read": snapshot.supply_mass_flow_rate_read,
        "supply_mass_flow_rate_kg_per_s":
            json_number(snapshot.supply_mass_flow_rate_kg_per_s),
        "supply_mass_flow_rate_kg_per_s_ieee_bits":
            ieee_bits(snapshot.supply_mass_flow_rate_kg_per_s),
        "specific_cooling_output_calculated":
            snapshot.specific_cooling_output_calculated,
        "specific_cooling_output_j_per_kg":
            json_number(snapshot.specific_cooling_output_j_per_kg),
        "specific_cooling_output_j_per_kg_ieee_bits":
            ieee_bits(snapshot.specific_cooling_output_j_per_kg),
        "supply_enthalpy_calculated": snapshot.supply_enthalpy_calculated,
        "calculated_supply_enthalpy_j_per_kg":
            json_number(snapshot.calculated_supply_enthalpy_j_per_kg),
        "calculated_supply_enthalpy_j_per_kg_ieee_bits":
            ieee_bits(snapshot.calculated_supply_enthalpy_j_per_kg),
        "supply_enthalpy_assigned": snapshot.supply_enthalpy_assigned,
        "assigned_supply_enthalpy_j_per_kg":
            json_number(snapshot.assigned_supply_enthalpy_j_per_kg),
        "assigned_supply_enthalpy_j_per_kg_ieee_bits":
            ieee_bits(snapshot.assigned_supply_enthalpy_j_per_kg),
        "resulting_supply_enthalpy_j_per_kg":
            json_number(snapshot.resulting_supply_enthalpy_j_per_kg),
        "resulting_supply_enthalpy_j_per_kg_ieee_bits":
            ieee_bits(snapshot.resulting_supply_enthalpy_j_per_kg),
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
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
    };

    use super::*;

    #[test]
    fn true_assignment_serializes_finite_values_and_exact_ieee_bits() {
        let value = snapshot_json(snapshot(Route::Assignment {
            preexisting: 40_000.0,
            mixed_air: 50_000.0,
            sensible: 2_000.0,
            flow: 2.0,
        }));
        for (field, expected) in [
            ("preexisting_supply_enthalpy_j_per_kg", 40_000.0_f64),
            ("mixed_air_enthalpy_j_per_kg", 50_000.0),
            ("cooling_sensible_output_w", 2_000.0),
            ("supply_mass_flow_rate_kg_per_s", 2.0),
            ("specific_cooling_output_j_per_kg", 1_000.0),
            ("calculated_supply_enthalpy_j_per_kg", 49_000.0),
            ("assigned_supply_enthalpy_j_per_kg", 49_000.0),
            ("resulting_supply_enthalpy_j_per_kg", 49_000.0),
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
    fn pure_false_guard_characterization_serializes_preserved_nonfinite_value_and_null_rhs() {
        // Serializer characterization only: complete public CP342 false routes
        // retain the finite SupplyEnthalpy proven by their CP339 lineage.
        let nan = f64::from_bits(0x7ff8_0000_0000_0042);
        let value = snapshot_json(snapshot(Route::GuardFalse(nan)));
        for field in [
            "preexisting_supply_enthalpy_j_per_kg",
            "resulting_supply_enthalpy_j_per_kg",
        ] {
            assert!(value[field].is_null(), "{field}");
            assert_eq!(
                value[format!("{field}_ieee_bits")],
                "0x7ff8000000000042",
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
            "preexisting_supply_enthalpy_j_per_kg",
            "mixed_air_enthalpy_j_per_kg",
            "cooling_sensible_output_w",
            "supply_mass_flow_rate_kg_per_s",
            "specific_cooling_output_j_per_kg",
            "calculated_supply_enthalpy_j_per_kg",
            "assigned_supply_enthalpy_j_per_kg",
            "resulting_supply_enthalpy_j_per_kg",
        ] {
            assert!(value[field].is_null(), "{field}");
            assert!(
                value[format!("{field}_ieee_bits")].is_null(),
                "{field} bits"
            );
        }
    }

    #[test]
    fn pure_assignment_characterization_serializes_derived_infinities_as_bits() {
        // This arbitrary IEEE snapshot does not broaden public reachability.
        let value = snapshot_json(snapshot(Route::Assignment {
            preexisting: 40_000.0,
            mixed_air: 50_000.0,
            sensible: f64::MAX,
            flow: f64::MIN_POSITIVE,
        }));
        assert!(value["specific_cooling_output_j_per_kg"].is_null());
        assert_eq!(
            value["specific_cooling_output_j_per_kg_ieee_bits"],
            "0x7ff0000000000000"
        );
        for field in [
            "calculated_supply_enthalpy_j_per_kg",
            "assigned_supply_enthalpy_j_per_kg",
            "resulting_supply_enthalpy_j_per_kg",
        ] {
            assert!(value[field].is_null(), "{field}");
            assert_eq!(
                value[format!("{field}_ieee_bits")],
                "0xfff0000000000000",
                "{field}"
            );
        }
    }

    fn active_value_fields() -> [&'static str; 6] {
        [
            "mixed_air_enthalpy_j_per_kg",
            "cooling_sensible_output_w",
            "supply_mass_flow_rate_kg_per_s",
            "specific_cooling_output_j_per_kg",
            "calculated_supply_enthalpy_j_per_kg",
            "assigned_supply_enthalpy_j_per_kg",
        ]
    }

    #[derive(Clone, Copy)]
    enum Route {
        InheritedSkip,
        GuardFalse(f64),
        Assignment {
            preexisting: f64,
            mixed_air: f64,
            sensible: f64,
            flow: f64,
        },
    }

    fn snapshot(
        route: Route,
    ) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot
    {
        let inherited_skip = matches!(route, Route::InheritedSkip);
        let guard_false = matches!(route, Route::GuardFalse(_));
        let assignment = matches!(route, Route::Assignment { .. });
        let preexisting = match route {
            Route::InheritedSkip => None,
            Route::GuardFalse(value) => Some(value),
            Route::Assignment { preexisting, .. } => Some(preexisting),
        };
        let active = match route {
            Route::Assignment {
                mixed_air,
                sensible,
                flow,
                ..
            } => {
                let specific = sensible / flow;
                let result = mixed_air - specific;
                Some((mixed_air, sensible, flow, specific, result))
            }
            Route::InheritedSkip | Route::GuardFalse(_) => None,
        };
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
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
            unit_off_skipped: inherited_skip,
            non_cooling_skipped: false,
            positive_guard_false_fallthrough_skipped: false,
            capacity_limit_guard_false_fallthrough_skipped: false,
            capacity_limit_sensible_output_guard_false_fallthrough: guard_false,
            capacity_limit_sensible_output_supply_enthalpy_assignment_executed: assignment,
            preexisting_supply_enthalpy_j_per_kg: preexisting,
            mixed_air_enthalpy_read: assignment,
            mixed_air_enthalpy_j_per_kg: active.map(|values| values.0),
            cooling_sensible_output_read: assignment,
            cooling_sensible_output_w: active.map(|values| values.1),
            supply_mass_flow_rate_read: assignment,
            supply_mass_flow_rate_kg_per_s: active.map(|values| values.2),
            specific_cooling_output_calculated: assignment,
            specific_cooling_output_j_per_kg: active.map(|values| values.3),
            supply_enthalpy_calculated: assignment,
            calculated_supply_enthalpy_j_per_kg: active.map(|values| values.4),
            supply_enthalpy_assigned: assignment,
            assigned_supply_enthalpy_j_per_kg: active.map(|values| values.4),
            resulting_supply_enthalpy_j_per_kg: if assignment {
                active.map(|values| values.4)
            } else {
                preexisting
            },
        }
    }
}
