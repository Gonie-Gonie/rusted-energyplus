//! JSON serialization for one CP352 snapshot.

use ep_model::DehumidificationControlType;
use ep_runtime::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentSnapshot,
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
        "unit_off_skipped": snapshot.unit_off_skipped,
        "non_cooling_skipped": snapshot.non_cooling_skipped,
        "positive_guard_false_fallthrough_skipped":
            snapshot.positive_guard_false_fallthrough_skipped,
        "predecessor_dehumidification_control_type":
            snapshot.predecessor_dehumidification_control_type.map(control_type_name),
        "predecessor_dehumidification_control_none_case_completed_skip":
            snapshot.predecessor_dehumidification_control_none_case_completed_skip,
        "predecessor_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_executed":
            snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_executed,
        "predecessor_dehumidification_control_humidistat_case_selected_skip":
            snapshot.predecessor_dehumidification_control_humidistat_case_selected_skip,
        "predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip":
            snapshot.predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
        "dehumidification_control_none_case_completed_skip":
            snapshot.dehumidification_control_none_case_completed_skip,
        "dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_executed":
            snapshot.dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_executed,
        "dehumidification_control_humidistat_case_selected_skip":
            snapshot.dehumidification_control_humidistat_case_selected_skip,
        "dehumidification_control_constant_supply_humidity_ratio_case_selected_skip":
            snapshot.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
        "mixed_air_enthalpy_read": snapshot.mixed_air_enthalpy_read,
        "mixed_air_enthalpy_j_per_kg": json_number(snapshot.mixed_air_enthalpy_j_per_kg),
        "mixed_air_enthalpy_j_per_kg_ieee_bits":
            ieee_bits(snapshot.mixed_air_enthalpy_j_per_kg),
        "cooling_total_output_read": snapshot.cooling_total_output_read,
        "cooling_total_output_w": json_number(snapshot.cooling_total_output_w),
        "cooling_total_output_w_ieee_bits": ieee_bits(snapshot.cooling_total_output_w),
        "supply_mass_flow_rate_read": snapshot.supply_mass_flow_rate_read,
        "supply_mass_flow_rate_kg_per_s":
            json_number(snapshot.supply_mass_flow_rate_kg_per_s),
        "supply_mass_flow_rate_kg_per_s_ieee_bits":
            ieee_bits(snapshot.supply_mass_flow_rate_kg_per_s),
        "specific_cooling_output_calculated": snapshot.specific_cooling_output_calculated,
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

fn control_type_name(control: DehumidificationControlType) -> &'static str {
    match control {
        DehumidificationControlType::None => "None",
        DehumidificationControlType::ConstantSensibleHeatRatio => "ConstantSensibleHeatRatio",
        DehumidificationControlType::Humidistat => "Humidistat",
        DehumidificationControlType::ConstantSupplyHumidityRatio => "ConstantSupplyHumidityRatio",
    }
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
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
    };

    use super::*;

    #[test]
    fn direct_none_release_serializes_symbolic_selector_and_null_numeric_evidence() {
        let value = snapshot_json(snapshot(None, false));

        assert_eq!(value["predecessor_dehumidification_control_type"], "None");
        for field in numeric_fields() {
            assert!(value[field].is_null(), "{field} value");
            assert!(
                value[format!("{field}_ieee_bits")].is_null(),
                "{field} bits"
            );
        }
    }

    #[test]
    fn nonfinite_numeric_evidence_keeps_authoritative_ieee_bits() {
        let value = snapshot_json(snapshot(Some(f64::NAN), true));
        let expected_bits = format!("0x{:016x}", f64::NAN.to_bits());

        assert_eq!(
            value["predecessor_dehumidification_control_type"],
            "ConstantSensibleHeatRatio"
        );
        for field in numeric_fields() {
            assert!(value[field].is_null(), "{field} JSON number");
            assert_eq!(value[format!("{field}_ieee_bits")], expected_bits);
        }
    }

    fn numeric_fields() -> [&'static str; 7] {
        [
            "mixed_air_enthalpy_j_per_kg",
            "cooling_total_output_w",
            "supply_mass_flow_rate_kg_per_s",
            "specific_cooling_output_j_per_kg",
            "calculated_supply_enthalpy_j_per_kg",
            "assigned_supply_enthalpy_j_per_kg",
            "resulting_supply_enthalpy_j_per_kg",
        ]
    }

    fn snapshot(
        numeric: Option<f64>,
        active: bool,
    ) -> PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentSnapshot
    {
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentSnapshot {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
            system: IdealLoadsAirSystemId(0),
            parent_call_ordinal: 1,
            controlled_zone: ZoneId(0),
            unit_body_entered: true,
            predecessor_cooling_body_entered: true,
            predecessor_no_outdoor_air_fallback_entered: true,
            predecessor_positive_supply_mass_flow_body_entered: true,
            unit_off_skipped: false,
            non_cooling_skipped: false,
            positive_guard_false_fallthrough_skipped: false,
            predecessor_dehumidification_control_type: Some(if active {
                DehumidificationControlType::ConstantSensibleHeatRatio
            } else {
                DehumidificationControlType::None
            }),
            predecessor_dehumidification_control_none_case_completed_skip: !active,
            predecessor_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_executed:
                active,
            predecessor_dehumidification_control_humidistat_case_selected_skip: false,
            predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
                false,
            dehumidification_control_none_case_completed_skip: !active,
            dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_executed:
                active,
            dehumidification_control_humidistat_case_selected_skip: false,
            dehumidification_control_constant_supply_humidity_ratio_case_selected_skip: false,
            mixed_air_enthalpy_read: active,
            mixed_air_enthalpy_j_per_kg: numeric,
            cooling_total_output_read: active,
            cooling_total_output_w: numeric,
            supply_mass_flow_rate_read: active,
            supply_mass_flow_rate_kg_per_s: numeric,
            specific_cooling_output_calculated: active,
            specific_cooling_output_j_per_kg: numeric,
            supply_enthalpy_calculated: active,
            calculated_supply_enthalpy_j_per_kg: numeric,
            supply_enthalpy_assigned: active,
            assigned_supply_enthalpy_j_per_kg: numeric,
            resulting_supply_enthalpy_j_per_kg: numeric,
        }
    }
}
