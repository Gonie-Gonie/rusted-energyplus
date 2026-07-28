//! JSON serialization for one CP339 snapshot.

use ep_runtime::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
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
        "unit_off_skipped": snapshot.unit_off_skipped,
        "non_cooling_skipped": snapshot.non_cooling_skipped,
        "positive_guard_false_fallthrough_skipped":
            snapshot.positive_guard_false_fallthrough_skipped,
        "capacity_limit_guard_false_fallthrough_skipped":
            snapshot.capacity_limit_guard_false_fallthrough_skipped,
        "capacity_limit_sensible_output_assignment_executed":
            snapshot.capacity_limit_sensible_output_assignment_executed,
        "supply_mass_flow_rate_read": snapshot.supply_mass_flow_rate_read,
        "supply_mass_flow_rate_kg_per_s": json_number(snapshot.supply_mass_flow_rate_kg_per_s),
        "supply_mass_flow_rate_kg_per_s_ieee_bits":
            ieee_bits(snapshot.supply_mass_flow_rate_kg_per_s),
        "mixed_air_enthalpy_read": snapshot.mixed_air_enthalpy_read,
        "mixed_air_enthalpy_j_per_kg": json_number(snapshot.mixed_air_enthalpy_j_per_kg),
        "mixed_air_enthalpy_j_per_kg_ieee_bits":
            ieee_bits(snapshot.mixed_air_enthalpy_j_per_kg),
        "supply_enthalpy_read": snapshot.supply_enthalpy_read,
        "supply_enthalpy_j_per_kg": json_number(snapshot.supply_enthalpy_j_per_kg),
        "supply_enthalpy_j_per_kg_ieee_bits": ieee_bits(snapshot.supply_enthalpy_j_per_kg),
        "enthalpy_difference_calculated":
            snapshot.enthalpy_difference_calculated,
        "mixed_air_minus_supply_enthalpy_j_per_kg":
            json_number(snapshot.mixed_air_minus_supply_enthalpy_j_per_kg),
        "mixed_air_minus_supply_enthalpy_j_per_kg_ieee_bits":
            ieee_bits(snapshot.mixed_air_minus_supply_enthalpy_j_per_kg),
        "cooling_sensible_output_calculated": snapshot.cooling_sensible_output_calculated,
        "calculated_cooling_sensible_output_w":
            json_number(snapshot.calculated_cooling_sensible_output_w),
        "calculated_cooling_sensible_output_w_ieee_bits":
            ieee_bits(snapshot.calculated_cooling_sensible_output_w),
        "cooling_sensible_output_assigned": snapshot.cooling_sensible_output_assigned,
        "cooling_sensible_output_w": json_number(snapshot.cooling_sensible_output_w),
        "cooling_sensible_output_w_ieee_bits": ieee_bits(snapshot.cooling_sensible_output_w),
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
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    };

    use super::*;

    #[test]
    fn nonfinite_numeric_is_null_while_ieee_bits_remain_authoritative() {
        for (value, expected_bits) in [
            (f64::INFINITY, "0x7ff0000000000000"),
            (f64::NEG_INFINITY, "0xfff0000000000000"),
            (f64::from_bits(0x7ff8_0000_0000_0042), "0x7ff8000000000042"),
        ] {
            assert!(json_number(Some(value)).is_null());
            assert_eq!(ieee_bits(Some(value)).as_deref(), Some(expected_bits));
        }
        assert!(json_number(None).is_null());
        assert_eq!(ieee_bits(None), None);
    }

    #[test]
    fn full_snapshot_json_preserves_active_nonfinite_bits_and_skip_nulls() {
        let flow = f64::INFINITY;
        let mixed = 42_441.175_2_f64;
        let supply = mixed;
        let difference = mixed - supply;
        let output = flow * difference;
        let active = snapshot(Some([flow, mixed, supply, difference, output, output]));
        let active_json = snapshot_json(active);
        for field in [
            "supply_mass_flow_rate_kg_per_s",
            "calculated_cooling_sensible_output_w",
            "cooling_sensible_output_w",
        ] {
            assert!(active_json[field].is_null(), "{field}");
        }
        for (field, value) in [
            ("mixed_air_enthalpy_j_per_kg", mixed),
            ("supply_enthalpy_j_per_kg", supply),
            ("mixed_air_minus_supply_enthalpy_j_per_kg", difference),
        ] {
            assert_eq!(active_json[field], json!(value), "{field}");
        }
        for (field, value) in [
            ("supply_mass_flow_rate_kg_per_s_ieee_bits", flow),
            ("mixed_air_enthalpy_j_per_kg_ieee_bits", mixed),
            ("supply_enthalpy_j_per_kg_ieee_bits", supply),
            (
                "mixed_air_minus_supply_enthalpy_j_per_kg_ieee_bits",
                difference,
            ),
            ("calculated_cooling_sensible_output_w_ieee_bits", output),
            ("cooling_sensible_output_w_ieee_bits", output),
        ] {
            assert_eq!(
                active_json[field],
                format!("0x{:016x}", value.to_bits()),
                "{field}"
            );
        }

        let skipped_json = snapshot_json(snapshot(None));
        for field in [
            "supply_mass_flow_rate_kg_per_s",
            "supply_mass_flow_rate_kg_per_s_ieee_bits",
            "mixed_air_enthalpy_j_per_kg",
            "mixed_air_enthalpy_j_per_kg_ieee_bits",
            "supply_enthalpy_j_per_kg",
            "supply_enthalpy_j_per_kg_ieee_bits",
            "mixed_air_minus_supply_enthalpy_j_per_kg",
            "mixed_air_minus_supply_enthalpy_j_per_kg_ieee_bits",
            "calculated_cooling_sensible_output_w",
            "calculated_cooling_sensible_output_w_ieee_bits",
            "cooling_sensible_output_w",
            "cooling_sensible_output_w_ieee_bits",
        ] {
            assert!(skipped_json[field].is_null(), "{field}");
        }
    }

    fn snapshot(
        values: Option<[f64; 6]>,
    ) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot {
        let active = values.is_some();
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
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
            unit_off_skipped: false,
            non_cooling_skipped: false,
            positive_guard_false_fallthrough_skipped: false,
            capacity_limit_guard_false_fallthrough_skipped: !active,
            capacity_limit_sensible_output_assignment_executed: active,
            supply_mass_flow_rate_read: active,
            supply_mass_flow_rate_kg_per_s: values.map(|values| values[0]),
            mixed_air_enthalpy_read: active,
            mixed_air_enthalpy_j_per_kg: values.map(|values| values[1]),
            supply_enthalpy_read: active,
            supply_enthalpy_j_per_kg: values.map(|values| values[2]),
            enthalpy_difference_calculated: active,
            mixed_air_minus_supply_enthalpy_j_per_kg: values.map(|values| values[3]),
            cooling_sensible_output_calculated: active,
            calculated_cooling_sensible_output_w: values.map(|values| values[4]),
            cooling_sensible_output_assigned: active,
            cooling_sensible_output_w: values.map(|values| values[5]),
        }
    }
}
