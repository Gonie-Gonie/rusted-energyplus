//! Bounded CP430-to-CP431 latest-snapshot lineage validation.

use ep_runtime::{
    PURCHASED_AIR_CALC_HEATING_MODE_GUARD_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_HEATING_MODE_GUARD_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_HEATING_MODE_GUARD_SOURCE_ORDER as ORDER,
    PurchasedAirCalcHeatingModeGuardSnapshot as Snapshot,
    PurchasedAirCalcHeatingOrNoLoadCaseEntrySnapshot as Predecessor,
    PurchasedAirTemperatureControlType, heating_mode_guard_predecessor_cp430_snapshot,
};

use crate::pipeline::purchased_air_heating_or_no_load_case_entry::serialization::snapshot::snapshot_json as predecessor_json;

pub(super) fn lineage_is_exact(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    let common = predecessor_json(heating_mode_guard_predecessor_cp430_snapshot(snapshot))
        == predecessor_json(predecessor)
        && provenance_is_exact(
            snapshot.source,
            snapshot.first_excluded_source,
            snapshot.source_order,
        )
        && snapshot.heating_or_no_load_case_entered == predecessor.heating_or_no_load_case_entered
        && snapshot.cp430_retained_supply_humidity_ratio_state_owned
            == predecessor.resulting_supply_humidity_ratio.is_some()
        && snapshot.cp430_retained_supply_enthalpy_state_owned
            == predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        && snapshot.cp430_retained_supply_temperature_state_owned
            == predecessor.resulting_supply_temperature_c.is_some()
        && same(
            snapshot.predecessor_cp430_resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
        && same(
            snapshot.predecessor_cp430_resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && same(
            snapshot.predecessor_cp430_resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
        && same(
            snapshot.resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
        && same(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && same(
            snapshot.resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        );
    common
        && if predecessor.heating_or_no_load_case_entered {
            active_shape_is_exact(snapshot)
        } else {
            inactive_shape_is_exact(snapshot)
        }
}

fn active_shape_is_exact(snapshot: Snapshot) -> bool {
    snapshot.heating_mode_guard_evaluated
        && snapshot.cp311_retained_minimum_outdoor_air_sensible_output_owned_read
        && snapshot.cp312_same_call_minimum_outdoor_air_sensible_output_bit_corroborated
        && snapshot.minimum_outdoor_air_sensible_output_for_heating_mode_guard_read
        && snapshot.cp310_retained_heating_setpoint_demand_owned_read
        && snapshot.heating_setpoint_demand_for_heating_mode_guard_read
        && snapshot.minimum_outdoor_air_sensible_output_heating_setpoint_demand_comparison_evaluated
        && direct_guard_result_is_exact(
            snapshot.minimum_outdoor_air_sensible_output_for_heating_mode_guard_w,
            snapshot.heating_setpoint_demand_for_heating_mode_guard_w,
            snapshot.minimum_outdoor_air_sensible_output_strictly_less_than_heating_setpoint_demand,
            snapshot,
        )
}

fn inactive_shape_is_exact(snapshot: Snapshot) -> bool {
    !snapshot.heating_mode_guard_evaluated
        && !snapshot.cp311_retained_minimum_outdoor_air_sensible_output_owned_read
        && !snapshot.cp312_same_call_minimum_outdoor_air_sensible_output_bit_corroborated
        && !snapshot.minimum_outdoor_air_sensible_output_for_heating_mode_guard_read
        && snapshot
            .minimum_outdoor_air_sensible_output_for_heating_mode_guard_w
            .is_none()
        && !snapshot.cp310_retained_heating_setpoint_demand_owned_read
        && !snapshot.heating_setpoint_demand_for_heating_mode_guard_read
        && snapshot
            .heating_setpoint_demand_for_heating_mode_guard_w
            .is_none()
        && !snapshot
            .minimum_outdoor_air_sensible_output_heating_setpoint_demand_comparison_evaluated
        && snapshot
            .minimum_outdoor_air_sensible_output_strictly_less_than_heating_setpoint_demand
            .is_none()
        && !snapshot.prevalidated_temperature_control_type_owned_read
        && !snapshot.temperature_control_type_read_after_sensible_comparison_short_circuit
        && snapshot.temperature_control_type.is_none()
        && !snapshot.temperature_control_type_single_cool_comparison_evaluated
        && snapshot.temperature_control_type_permits_heating.is_none()
        && !snapshot.single_cool_blocked
        && !snapshot.heating_operating_mode_body_entered
        && !snapshot.heating_mode_guard_false_fallthrough
}

fn direct_guard_result_is_exact(
    left: Option<f64>,
    right: Option<f64>,
    result: Option<bool>,
    snapshot: Snapshot,
) -> bool {
    let (Some(left), Some(right)) = (left, right) else {
        return false;
    };
    let sensible = left < right;
    result == Some(sensible)
        && if sensible {
            snapshot.prevalidated_temperature_control_type_owned_read
                && snapshot.temperature_control_type_read_after_sensible_comparison_short_circuit
                && snapshot.temperature_control_type
                    == Some(PurchasedAirTemperatureControlType::DualHeatCool)
                && snapshot.temperature_control_type_single_cool_comparison_evaluated
                && snapshot.temperature_control_type_permits_heating == Some(true)
                && !snapshot.single_cool_blocked
                && snapshot.heating_operating_mode_body_entered
                && !snapshot.heating_mode_guard_false_fallthrough
        } else {
            !snapshot.prevalidated_temperature_control_type_owned_read
                && !snapshot.temperature_control_type_read_after_sensible_comparison_short_circuit
                && snapshot.temperature_control_type.is_none()
                && !snapshot.temperature_control_type_single_cool_comparison_evaluated
                && snapshot.temperature_control_type_permits_heating.is_none()
                && !snapshot.single_cool_blocked
                && !snapshot.heating_operating_mode_body_entered
                && snapshot.heating_mode_guard_false_fallthrough
        }
}

fn provenance_is_exact(source: &str, first_excluded_source: &str, source_order: &[&str]) -> bool {
    source == SOURCE && first_excluded_source == EXCLUDED && source_order == ORDER
}

fn same(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{EXCLUDED, ORDER, SOURCE, provenance_is_exact};

    #[test]
    fn snapshot_provenance_rejects_each_coordinated_field_forgery() {
        assert!(provenance_is_exact(SOURCE, EXCLUDED, ORDER));
        assert!(!provenance_is_exact("forged source", EXCLUDED, ORDER));
        assert!(!provenance_is_exact(SOURCE, "forged exclusion", ORDER));
        assert!(!provenance_is_exact(SOURCE, EXCLUDED, &["forged order"]));
    }
}
