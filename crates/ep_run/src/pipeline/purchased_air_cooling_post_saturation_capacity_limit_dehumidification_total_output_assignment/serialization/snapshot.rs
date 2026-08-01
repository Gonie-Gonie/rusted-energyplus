//! JSON serialization for one CP382 assignment snapshot.

use ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentSnapshot,
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
        "dehumidification_total_output_assignment_executed": snapshot.dehumidification_total_output_assignment_executed,
        "cp330_supply_mass_flow_rate_owned_read": snapshot.cp330_supply_mass_flow_rate_owned_read,
        "cp329_same_call_supply_mass_flow_rate_bit_corroborated": snapshot.cp329_same_call_supply_mass_flow_rate_bit_corroborated,
        "cp339_same_call_supply_mass_flow_rate_bit_corroborated": snapshot.cp339_same_call_supply_mass_flow_rate_bit_corroborated,
        "supply_mass_flow_rate_read": snapshot.supply_mass_flow_rate_read,
        "supply_mass_flow_rate_kg_per_s": json_number(snapshot.supply_mass_flow_rate_kg_per_s),
        "supply_mass_flow_rate_kg_per_s_ieee_bits": ieee_bits(snapshot.supply_mass_flow_rate_kg_per_s),
        "cp329_mixed_air_enthalpy_owned_read": snapshot.cp329_mixed_air_enthalpy_owned_read,
        "cp329_same_call_recirculation_enthalpy_bit_corroborated": snapshot.cp329_same_call_recirculation_enthalpy_bit_corroborated,
        "cp339_same_call_mixed_air_enthalpy_bit_corroborated": snapshot.cp339_same_call_mixed_air_enthalpy_bit_corroborated,
        "mixed_air_enthalpy_read": snapshot.mixed_air_enthalpy_read,
        "mixed_air_enthalpy_j_per_kg": json_number(snapshot.mixed_air_enthalpy_j_per_kg),
        "mixed_air_enthalpy_j_per_kg_ieee_bits": ieee_bits(snapshot.mixed_air_enthalpy_j_per_kg),
        "cp379_post_saturation_supply_enthalpy_owned_read": snapshot.cp379_post_saturation_supply_enthalpy_owned_read,
        "cp379_same_call_supply_enthalpy_bits_corroborated": snapshot.cp379_same_call_supply_enthalpy_bits_corroborated,
        "supply_enthalpy_read": snapshot.supply_enthalpy_read,
        "supply_enthalpy_j_per_kg": json_number(snapshot.supply_enthalpy_j_per_kg),
        "supply_enthalpy_j_per_kg_ieee_bits": ieee_bits(snapshot.supply_enthalpy_j_per_kg),
        "enthalpy_difference_calculated": snapshot.enthalpy_difference_calculated,
        "mixed_air_minus_supply_enthalpy_j_per_kg": json_number(snapshot.mixed_air_minus_supply_enthalpy_j_per_kg),
        "mixed_air_minus_supply_enthalpy_j_per_kg_ieee_bits": ieee_bits(snapshot.mixed_air_minus_supply_enthalpy_j_per_kg),
        "cooling_total_output_calculated": snapshot.cooling_total_output_calculated,
        "calculated_cooling_total_output_w": json_number(snapshot.calculated_cooling_total_output_w),
        "calculated_cooling_total_output_w_ieee_bits": ieee_bits(snapshot.calculated_cooling_total_output_w),
        "cooling_total_output_assigned": snapshot.cooling_total_output_assigned,
        "cooling_total_output_w": json_number(snapshot.cooling_total_output_w),
        "cooling_total_output_w_ieee_bits": ieee_bits(snapshot.cooling_total_output_w),
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
        for bits in [0x7ff8_0000_0000_0382, f64::NEG_INFINITY.to_bits()] {
            let value = Some(f64::from_bits(bits));
            assert!(json_number(value).is_null());
            assert_eq!(ieee_bits(value), Some(format!("0x{bits:016x}")));
        }
    }

    #[test]
    fn skipped_numeric_values_and_sidecars_are_both_null() {
        assert!(json_number(None).is_null());
        assert_eq!(ieee_bits(None), None);
    }
}
