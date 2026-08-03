//! Exact CP411-to-CP412 coupled-runtime lineage validation.

use crate::{
    ideal_loads::{
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot as Predecessor,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentSnapshot as Snapshot,
    },
    psychrometrics::energyplus_psy_w_fn_tdb_rh_pb,
};

pub(super) fn links_to_predecessor(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    snapshot.system == predecessor.system
        && snapshot.parent_call_ordinal == predecessor.parent_call_ordinal
        && snapshot.controlled_zone == predecessor.controlled_zone
        && flags(snapshot) == predecessor_flags(predecessor)
        && snapshot.predecessor_dehumidification_control_type
            == predecessor.predecessor_dehumidification_control_type
        && predecessor_values(snapshot)
            .into_iter()
            .zip(values(predecessor))
            .all(|(left, right)| option_bits_equal(left, right))
}

pub(super) fn assignment_shape(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    let active = predecessor
        .post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_executed;
    let carriers_match = option_bits_equal(
        snapshot.resulting_supply_humidity_ratio,
        predecessor.resulting_supply_humidity_ratio,
    ) && option_bits_equal(
        snapshot.resulting_supply_enthalpy_j_per_kg,
        predecessor.resulting_supply_enthalpy_j_per_kg,
    ) && option_bits_equal(
        snapshot.resulting_supply_temperature_c,
        predecessor.resulting_supply_temperature_c,
    );
    if !active {
        return !snapshot
            .post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_executed
            && snapshot.cp411_retained_supply_humidity_ratio_state_owned
                == predecessor.resulting_supply_humidity_ratio.is_some()
            && snapshot.cp411_retained_supply_enthalpy_state_owned
                == predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
            && snapshot.cp411_retained_supply_temperature_state_owned
                == predecessor.resulting_supply_temperature_c.is_some()
            && !snapshot.cp411_retained_supply_temperature_owned_read
            && !snapshot.purchased_air_supply_temperature_for_saturation_humidity_ratio_read
            && !snapshot.environment_outdoor_barometric_pressure_owned_read
            && !snapshot.environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read
            && !snapshot.psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluated
            && !snapshot.local_saturation_supply_humidity_ratio_assignment_performed
            && local_values(snapshot).into_iter().all(|value| value.is_none())
            && carriers_match;
    }

    let Some(temperature) = predecessor.resulting_supply_temperature_c else {
        return false;
    };
    let Some(pressure) = snapshot.outdoor_barometric_pressure_pa else {
        return false;
    };
    let saturation = energyplus_psy_w_fn_tdb_rh_pb(temperature, 1.0, pressure);
    snapshot
        .post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_executed
        && snapshot.cp411_retained_supply_humidity_ratio_state_owned
            == predecessor.resulting_supply_humidity_ratio.is_some()
        && snapshot.cp411_retained_supply_enthalpy_state_owned
            == predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        && snapshot.cp411_retained_supply_temperature_state_owned
        && snapshot.cp411_retained_supply_temperature_owned_read
        && snapshot.purchased_air_supply_temperature_for_saturation_humidity_ratio_read
        && snapshot.environment_outdoor_barometric_pressure_owned_read
        && snapshot.environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read
        && snapshot.psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluated
        && snapshot.local_saturation_supply_humidity_ratio_assignment_performed
        && temperature.is_finite()
        && pressure.is_finite()
        && pressure > 0.0
        && saturation.is_finite()
        && option_bits_equal(
            snapshot.supply_temperature_for_saturation_humidity_ratio_c,
            Some(temperature),
        )
        && [
            snapshot.saturation_supply_humidity_ratio,
            snapshot.assigned_saturation_supply_humidity_ratio,
            snapshot.resulting_saturation_supply_humidity_ratio,
        ]
        .into_iter()
        .all(|value| option_bits_equal(value, Some(saturation)))
        && carriers_match
}

pub(super) fn same_snapshot(mut left: Snapshot, mut right: Snapshot) -> bool {
    let numeric = numeric_values(left)
        .into_iter()
        .zip(numeric_values(right))
        .all(|(left, right)| option_bits_equal(left, right));
    clear_numeric_values(&mut left);
    clear_numeric_values(&mut right);
    numeric && left == right
}

fn flags(snapshot: Snapshot) -> [bool; 40] {
    [
        snapshot.unit_off_skipped,
        snapshot.non_cooling_skipped,
        snapshot.positive_guard_false_fallthrough_skipped,
        snapshot.heating_availability_guard_false_fallthrough,
        snapshot.humidification_control_guard_false_fallthrough,
        snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        snapshot.dehumidification_control_none_maximum_assignment_executed,
        snapshot.dehumidification_control_guard_false_fallthrough,
        snapshot.predecessor_capacity_limit_guard_evaluated,
        snapshot.predecessor_capacity_limit_body_entered,
        snapshot.predecessor_active_capacity_limit_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_guard_evaluated,
        snapshot.predecessor_dehumidification_body_entered,
        snapshot.predecessor_dehumidification_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_total_output_assignment_executed,
        snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated,
        snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_maximum_capacity_assignment_executed,
        snapshot.predecessor_supply_enthalpy_assignment_executed,
        snapshot.predecessor_dehumidification_control_type_read,
        snapshot.predecessor_dehumidification_control_switch_dispatched,
        snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered,
        snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break,
        snapshot.predecessor_dehumidification_control_humidistat_case_entered,
        snapshot.predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed,
        snapshot.predecessor_dehumidification_control_humidistat_case_exited_via_break,
        snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered,
        snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed,
        snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break,
        snapshot.predecessor_dehumidification_control_default_case_exited_via_break,
        snapshot.post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_executed,
        snapshot.cp410_retained_supply_humidity_ratio_state_owned,
        snapshot.cp410_retained_supply_enthalpy_state_owned,
        snapshot.cp410_retained_supply_temperature_state_owned,
        snapshot.cp410_retained_supply_humidity_ratio_owned_read,
        snapshot.purchased_air_supply_humidity_ratio_read,
        snapshot.local_supply_humidity_ratio_original_assignment_performed,
    ]
}

fn predecessor_flags(snapshot: Predecessor) -> [bool; 40] {
    [
        snapshot.unit_off_skipped,
        snapshot.non_cooling_skipped,
        snapshot.positive_guard_false_fallthrough_skipped,
        snapshot.heating_availability_guard_false_fallthrough,
        snapshot.humidification_control_guard_false_fallthrough,
        snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        snapshot.dehumidification_control_none_maximum_assignment_executed,
        snapshot.dehumidification_control_guard_false_fallthrough,
        snapshot.predecessor_capacity_limit_guard_evaluated,
        snapshot.predecessor_capacity_limit_body_entered,
        snapshot.predecessor_active_capacity_limit_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_guard_evaluated,
        snapshot.predecessor_dehumidification_body_entered,
        snapshot.predecessor_dehumidification_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_total_output_assignment_executed,
        snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated,
        snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_maximum_capacity_assignment_executed,
        snapshot.predecessor_supply_enthalpy_assignment_executed,
        snapshot.predecessor_dehumidification_control_type_read,
        snapshot.predecessor_dehumidification_control_switch_dispatched,
        snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered,
        snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break,
        snapshot.predecessor_dehumidification_control_humidistat_case_entered,
        snapshot.predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed,
        snapshot.predecessor_dehumidification_control_humidistat_case_exited_via_break,
        snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered,
        snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed,
        snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break,
        snapshot.predecessor_dehumidification_control_default_case_exited_via_break,
        snapshot.post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_executed,
        snapshot.cp410_retained_supply_humidity_ratio_state_owned,
        snapshot.cp410_retained_supply_enthalpy_state_owned,
        snapshot.cp410_retained_supply_temperature_state_owned,
        snapshot.cp410_retained_supply_humidity_ratio_owned_read,
        snapshot.purchased_air_supply_humidity_ratio_read,
        snapshot.local_supply_humidity_ratio_original_assignment_performed,
    ]
}

fn predecessor_values(snapshot: Snapshot) -> [Option<f64>; 12] {
    [
        snapshot.predecessor_cp409_resulting_supply_humidity_ratio,
        snapshot.predecessor_cp409_resulting_supply_enthalpy_j_per_kg,
        snapshot.predecessor_cp409_resulting_supply_temperature_c,
        snapshot.predecessor_cp410_resulting_supply_humidity_ratio,
        snapshot.predecessor_cp410_resulting_supply_enthalpy_j_per_kg,
        snapshot.predecessor_cp410_resulting_supply_temperature_c,
        snapshot.purchased_air_supply_humidity_ratio_before_saturation_check,
        snapshot.assigned_supply_humidity_ratio_original,
        snapshot.resulting_supply_humidity_ratio_original,
        snapshot.predecessor_cp411_resulting_supply_humidity_ratio,
        snapshot.predecessor_cp411_resulting_supply_enthalpy_j_per_kg,
        snapshot.predecessor_cp411_resulting_supply_temperature_c,
    ]
}

fn values(snapshot: Predecessor) -> [Option<f64>; 12] {
    [
        snapshot.predecessor_cp409_resulting_supply_humidity_ratio,
        snapshot.predecessor_cp409_resulting_supply_enthalpy_j_per_kg,
        snapshot.predecessor_cp409_resulting_supply_temperature_c,
        snapshot.predecessor_cp410_resulting_supply_humidity_ratio,
        snapshot.predecessor_cp410_resulting_supply_enthalpy_j_per_kg,
        snapshot.predecessor_cp410_resulting_supply_temperature_c,
        snapshot.purchased_air_supply_humidity_ratio_before_saturation_check,
        snapshot.assigned_supply_humidity_ratio_original,
        snapshot.resulting_supply_humidity_ratio_original,
        snapshot.resulting_supply_humidity_ratio,
        snapshot.resulting_supply_enthalpy_j_per_kg,
        snapshot.resulting_supply_temperature_c,
    ]
}

fn local_values(snapshot: Snapshot) -> [Option<f64>; 5] {
    [
        snapshot.supply_temperature_for_saturation_humidity_ratio_c,
        snapshot.outdoor_barometric_pressure_pa,
        snapshot.saturation_supply_humidity_ratio,
        snapshot.assigned_saturation_supply_humidity_ratio,
        snapshot.resulting_saturation_supply_humidity_ratio,
    ]
}

fn numeric_values(snapshot: Snapshot) -> [Option<f64>; 20] {
    let predecessor = predecessor_values(snapshot);
    let local = local_values(snapshot);
    [
        predecessor[0],
        predecessor[1],
        predecessor[2],
        predecessor[3],
        predecessor[4],
        predecessor[5],
        predecessor[6],
        predecessor[7],
        predecessor[8],
        predecessor[9],
        predecessor[10],
        predecessor[11],
        local[0],
        local[1],
        local[2],
        local[3],
        local[4],
        snapshot.resulting_supply_humidity_ratio,
        snapshot.resulting_supply_enthalpy_j_per_kg,
        snapshot.resulting_supply_temperature_c,
    ]
}

fn clear_numeric_values(snapshot: &mut Snapshot) {
    snapshot.predecessor_cp409_resulting_supply_humidity_ratio = None;
    snapshot.predecessor_cp409_resulting_supply_enthalpy_j_per_kg = None;
    snapshot.predecessor_cp409_resulting_supply_temperature_c = None;
    snapshot.predecessor_cp410_resulting_supply_humidity_ratio = None;
    snapshot.predecessor_cp410_resulting_supply_enthalpy_j_per_kg = None;
    snapshot.predecessor_cp410_resulting_supply_temperature_c = None;
    snapshot.purchased_air_supply_humidity_ratio_before_saturation_check = None;
    snapshot.assigned_supply_humidity_ratio_original = None;
    snapshot.resulting_supply_humidity_ratio_original = None;
    snapshot.predecessor_cp411_resulting_supply_humidity_ratio = None;
    snapshot.predecessor_cp411_resulting_supply_enthalpy_j_per_kg = None;
    snapshot.predecessor_cp411_resulting_supply_temperature_c = None;
    snapshot.supply_temperature_for_saturation_humidity_ratio_c = None;
    snapshot.outdoor_barometric_pressure_pa = None;
    snapshot.saturation_supply_humidity_ratio = None;
    snapshot.assigned_saturation_supply_humidity_ratio = None;
    snapshot.resulting_saturation_supply_humidity_ratio = None;
    snapshot.resulting_supply_humidity_ratio = None;
    snapshot.resulting_supply_enthalpy_j_per_kg = None;
    snapshot.resulting_supply_temperature_c = None;
}

fn option_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
