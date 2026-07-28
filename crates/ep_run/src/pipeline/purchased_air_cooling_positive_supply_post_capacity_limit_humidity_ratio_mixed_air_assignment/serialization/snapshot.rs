//! JSON serialization for one CP345 snapshot.

use ep_runtime::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
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
        "post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed":
            snapshot.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed,
        "mixed_air_humidity_ratio_read": snapshot.mixed_air_humidity_ratio_read,
        "mixed_air_humidity_ratio": json_number(snapshot.mixed_air_humidity_ratio),
        "mixed_air_humidity_ratio_ieee_bits":
            ieee_bits(snapshot.mixed_air_humidity_ratio),
        "supply_humidity_ratio_assignment_performed":
            snapshot.supply_humidity_ratio_assignment_performed,
        "assigned_supply_humidity_ratio":
            json_number(snapshot.assigned_supply_humidity_ratio),
        "assigned_supply_humidity_ratio_ieee_bits":
            ieee_bits(snapshot.assigned_supply_humidity_ratio),
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
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER,
    };

    use super::*;

    #[test]
    fn serializer_exposes_each_g_f_l_assignment_provenance_route() {
        for (route, field) in [
            (
                Route::CapacityGuardFalse,
                "capacity_limit_guard_false_fallthrough_skipped",
            ),
            (
                Route::SensibleGuardFalse,
                "capacity_limit_sensible_output_guard_false_fallthrough",
            ),
            (
                Route::MixedAirLimit,
                "capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed",
            ),
        ] {
            let value = snapshot_json(snapshot(route, Some(-0.0)));
            assert_eq!(value[field], true, "{field}");
            assert_eq!(
                value["post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed"],
                true
            );
            assert_eq!(
                value["assigned_supply_humidity_ratio_ieee_bits"],
                "0x8000000000000000"
            );
        }
    }

    #[test]
    fn serializer_maps_nonfinite_numeric_values_to_null_but_preserves_bits() {
        let nan = f64::from_bits(0x7ff8_0000_0000_3450);
        let value = snapshot_json(snapshot(Route::MixedAirLimit, Some(nan)));
        for field in ["mixed_air_humidity_ratio", "assigned_supply_humidity_ratio"] {
            assert!(value[field].is_null(), "{field}");
            assert_eq!(
                value[format!("{field}_ieee_bits")],
                "0x7ff8000000003450",
                "{field} bits"
            );
        }
    }

    #[test]
    fn serializer_keeps_skipped_numeric_values_and_bits_null() {
        let value = snapshot_json(snapshot(Route::UnitOff, None));
        assert_eq!(value["unit_off_skipped"], true);
        assert_eq!(
            value["post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed"],
            false
        );
        for field in [
            "mixed_air_humidity_ratio",
            "assigned_supply_humidity_ratio",
            "mixed_air_humidity_ratio_ieee_bits",
            "assigned_supply_humidity_ratio_ieee_bits",
        ] {
            assert!(value[field].is_null(), "{field}");
        }
    }

    #[derive(Clone, Copy)]
    enum Route {
        UnitOff,
        CapacityGuardFalse,
        SensibleGuardFalse,
        MixedAirLimit,
    }

    fn snapshot(
        route: Route,
        humidity_ratio: Option<f64>,
    ) -> PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot
    {
        let unit_off = matches!(route, Route::UnitOff);
        let g = matches!(route, Route::CapacityGuardFalse);
        let f = matches!(route, Route::SensibleGuardFalse);
        let l = matches!(route, Route::MixedAirLimit);
        let active = g || f || l;
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER,
            system: IdealLoadsAirSystemId(0),
            parent_call_ordinal: 1,
            controlled_zone: ZoneId(0),
            unit_body_entered: active,
            predecessor_cooling_body_entered: active,
            predecessor_no_outdoor_air_fallback_entered: active,
            predecessor_positive_supply_mass_flow_body_entered: active,
            predecessor_active_guard_false_fallthrough: false,
            predecessor_capacity_limit_guard_evaluated: active,
            predecessor_capacity_limit_body_entered: f || l,
            predecessor_active_capacity_limit_guard_false_fallthrough: g,
            predecessor_capacity_limit_cp_air_assignment_executed: f || l,
            predecessor_capacity_limit_sensible_output_assignment_executed: f || l,
            predecessor_capacity_limit_sensible_output_guard_evaluated: f || l,
            predecessor_capacity_limit_sensible_output_guard_false_fallthrough: f,
            predecessor_capacity_limit_sensible_output_adjustment_body_entered: l,
            predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed: l,
            predecessor_capacity_limit_sensible_output_supply_enthalpy_assignment_executed: l,
            predecessor_capacity_limit_sensible_output_supply_temperature_assignment_executed: l,
            unit_off_skipped: unit_off,
            non_cooling_skipped: false,
            positive_guard_false_fallthrough_skipped: false,
            capacity_limit_guard_false_fallthrough_skipped: g,
            capacity_limit_sensible_output_guard_false_fallthrough: f,
            capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed: l,
            post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed: active,
            mixed_air_humidity_ratio_read: active,
            mixed_air_humidity_ratio: active.then_some(humidity_ratio).flatten(),
            supply_humidity_ratio_assignment_performed: active,
            assigned_supply_humidity_ratio: active.then_some(humidity_ratio).flatten(),
        }
    }
}
