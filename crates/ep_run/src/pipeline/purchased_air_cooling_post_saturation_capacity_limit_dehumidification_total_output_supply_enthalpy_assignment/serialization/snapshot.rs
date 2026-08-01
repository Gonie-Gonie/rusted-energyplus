//! JSON serialization for one CP385 supply-enthalpy assignment snapshot.

use ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot,
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
        "supply_enthalpy_assignment_executed": snapshot.supply_enthalpy_assignment_executed,
        "preexisting_supply_enthalpy_j_per_kg": json_number(snapshot.preexisting_supply_enthalpy_j_per_kg),
        "preexisting_supply_enthalpy_j_per_kg_ieee_bits": ieee_bits(snapshot.preexisting_supply_enthalpy_j_per_kg),
        "cp379_retained_supply_enthalpy_owned_read": snapshot.cp379_retained_supply_enthalpy_owned_read,
        "cp329_retained_mixed_air_enthalpy_owned_read": snapshot.cp329_retained_mixed_air_enthalpy_owned_read,
        "mixed_air_enthalpy_read": snapshot.mixed_air_enthalpy_read,
        "mixed_air_enthalpy_j_per_kg": json_number(snapshot.mixed_air_enthalpy_j_per_kg),
        "mixed_air_enthalpy_j_per_kg_ieee_bits": ieee_bits(snapshot.mixed_air_enthalpy_j_per_kg),
        "cp384_retained_cooling_total_output_owned_read": snapshot.cp384_retained_cooling_total_output_owned_read,
        "cooling_total_output_read": snapshot.cooling_total_output_read,
        "cooling_total_output_w": json_number(snapshot.cooling_total_output_w),
        "cooling_total_output_w_ieee_bits": ieee_bits(snapshot.cooling_total_output_w),
        "cp330_retained_supply_mass_flow_rate_owned_read": snapshot.cp330_retained_supply_mass_flow_rate_owned_read,
        "supply_mass_flow_rate_read": snapshot.supply_mass_flow_rate_read,
        "supply_mass_flow_rate_kg_per_s": json_number(snapshot.supply_mass_flow_rate_kg_per_s),
        "supply_mass_flow_rate_kg_per_s_ieee_bits": ieee_bits(snapshot.supply_mass_flow_rate_kg_per_s),
        "specific_cooling_output_calculated": snapshot.specific_cooling_output_calculated,
        "specific_cooling_output_j_per_kg": json_number(snapshot.specific_cooling_output_j_per_kg),
        "specific_cooling_output_j_per_kg_ieee_bits": ieee_bits(snapshot.specific_cooling_output_j_per_kg),
        "supply_enthalpy_difference_calculated": snapshot.supply_enthalpy_difference_calculated,
        "calculated_supply_enthalpy_j_per_kg": json_number(snapshot.calculated_supply_enthalpy_j_per_kg),
        "calculated_supply_enthalpy_j_per_kg_ieee_bits": ieee_bits(snapshot.calculated_supply_enthalpy_j_per_kg),
        "supply_enthalpy_assigned": snapshot.supply_enthalpy_assigned,
        "assigned_supply_enthalpy_j_per_kg": json_number(snapshot.assigned_supply_enthalpy_j_per_kg),
        "assigned_supply_enthalpy_j_per_kg_ieee_bits": ieee_bits(snapshot.assigned_supply_enthalpy_j_per_kg),
        "resulting_supply_enthalpy_j_per_kg": json_number(snapshot.resulting_supply_enthalpy_j_per_kg),
        "resulting_supply_enthalpy_j_per_kg_ieee_bits": ieee_bits(snapshot.resulting_supply_enthalpy_j_per_kg),
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
    fn nonfinite_numbers_are_null_with_authoritative_ieee_sidecars() {
        for bits in [0x7ff8_0000_0000_0385, f64::INFINITY.to_bits()] {
            let value = Some(f64::from_bits(bits));
            assert!(json_number(value).is_null());
            assert_eq!(ieee_bits(value), Some(format!("0x{bits:016x}")));
        }
        assert!(json_number(None).is_null());
        assert_eq!(ieee_bits(None), None);
    }
}
