//! JSON serialization for one CP381 comparison snapshot.

use ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardSnapshot,
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
        "dehumidification_guard_evaluated": snapshot.dehumidification_guard_evaluated,
        "cp378_supply_humidity_ratio_saturation_limit_owned_read": snapshot.cp378_supply_humidity_ratio_saturation_limit_owned_read,
        "cp379_same_call_supply_humidity_ratio_bit_corroborated": snapshot.cp379_same_call_supply_humidity_ratio_bit_corroborated,
        "purchased_air_supply_humidity_ratio_read": snapshot.purchased_air_supply_humidity_ratio_read,
        "supply_humidity_ratio": json_number(snapshot.supply_humidity_ratio),
        "supply_humidity_ratio_ieee_bits": ieee_bits(snapshot.supply_humidity_ratio),
        "cp329_mixed_air_humidity_ratio_owned_read": snapshot.cp329_mixed_air_humidity_ratio_owned_read,
        "purchased_air_mixed_air_humidity_ratio_read": snapshot.purchased_air_mixed_air_humidity_ratio_read,
        "mixed_air_humidity_ratio": json_number(snapshot.mixed_air_humidity_ratio),
        "mixed_air_humidity_ratio_ieee_bits": ieee_bits(snapshot.mixed_air_humidity_ratio),
        "supply_humidity_ratio_mixed_air_humidity_ratio_comparison_evaluated": snapshot.supply_humidity_ratio_mixed_air_humidity_ratio_comparison_evaluated,
        "supply_humidity_ratio_strictly_less_than_mixed_air_humidity_ratio": snapshot.supply_humidity_ratio_strictly_less_than_mixed_air_humidity_ratio,
        "dehumidification_body_entered": snapshot.dehumidification_body_entered,
        "dehumidification_guard_false_fallthrough": snapshot.dehumidification_guard_false_fallthrough,
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
    use super::*;

    #[test]
    fn non_finite_values_are_null_with_exact_ieee_sidecars() {
        for bits in [0x7ff8_0000_0000_0381, f64::INFINITY.to_bits()] {
            let value = Some(f64::from_bits(bits));
            assert!(json_number(value).is_null());
            assert_eq!(ieee_bits(value), Some(format!("0x{bits:016x}")));
        }
    }
}
