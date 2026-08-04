//! Exact CP412 route refinement and CP413 saturation-guard validation.

use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardSnapshot as Snapshot,
};

/// One retained CP413 route.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) struct RetainedRoute {
    pub predecessor_index: usize,
    pub predecessor_guard_false_fallthrough: bool,
    pub predecessor_maximum_capacity_assignment_executed: bool,
    pub active: bool,
    pub body_entered: bool,
}

pub(in crate::ideal_loads::calc) fn predecessor_route(
    predecessor: Predecessor,
) -> Option<RetainedRoute> {
    let route = crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_snapshot_route(predecessor)?;
    Some(RetainedRoute {
        predecessor_index: route.predecessor_index,
        predecessor_guard_false_fallthrough: route.predecessor_guard_false_fallthrough,
        predecessor_maximum_capacity_assignment_executed: route
            .predecessor_maximum_capacity_assignment_executed,
        active: route.active && matches!(route.predecessor_index, 18..=29),
        body_entered: false,
    })
}

pub(in crate::ideal_loads::calc) fn snapshot_route(snapshot: Snapshot) -> Option<RetainedRoute> {
    let predecessor = cp412_shape(snapshot);
    let base = predecessor_route(predecessor)?;
    let comparison =
        snapshot.saturation_supply_humidity_ratio_strictly_less_than_original_supply_humidity_ratio;
    let route = RetainedRoute {
        body_entered: comparison == Some(true),
        ..base
    };
    local_shape_is_exact(snapshot, predecessor, route).then_some(route)
}

fn local_shape_is_exact(
    snapshot: Snapshot,
    predecessor: Predecessor,
    route: RetainedRoute,
) -> bool {
    let terminal_preserved = option_bits_match(
        predecessor.resulting_supply_humidity_ratio,
        snapshot.resulting_supply_humidity_ratio,
    ) && option_bits_match(
        predecessor.resulting_supply_enthalpy_j_per_kg,
        snapshot.resulting_supply_enthalpy_j_per_kg,
    ) && option_bits_match(
        predecessor.resulting_supply_temperature_c,
        snapshot.resulting_supply_temperature_c,
    );
    let owner_shape = snapshot.cp412_retained_supply_humidity_ratio_state_owned
        == predecessor.resulting_supply_humidity_ratio.is_some()
        && snapshot.cp412_retained_supply_enthalpy_state_owned
            == predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        && snapshot.cp412_retained_supply_temperature_state_owned
            == predecessor.resulting_supply_temperature_c.is_some();
    if !terminal_preserved || !owner_shape {
        return false;
    }
    if !route_is_active(route) {
        return !snapshot
            .post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_evaluated
            && !snapshot.cp412_saturation_supply_humidity_ratio_owned_read
            && !snapshot.saturation_supply_humidity_ratio_for_guard_read
            && snapshot.saturation_supply_humidity_ratio_for_guard.is_none()
            && !snapshot.cp411_original_supply_humidity_ratio_owned_read
            && !snapshot.cp412_same_call_original_supply_humidity_ratio_bit_corroborated
            && !snapshot.original_supply_humidity_ratio_for_guard_read
            && snapshot.original_supply_humidity_ratio_for_guard.is_none()
            && !snapshot.saturation_original_supply_humidity_ratio_comparison_evaluated
            && snapshot
                .saturation_supply_humidity_ratio_strictly_less_than_original_supply_humidity_ratio
                .is_none()
            && !snapshot.saturation_supply_humidity_ratio_guard_body_entered
            && !snapshot.saturation_supply_humidity_ratio_guard_false_fallthrough;
    }
    let (Some(predecessor_saturation), Some(predecessor_original), Some(cp411_terminal), Some(saturation), Some(original), Some(comparison)) = (
        predecessor.resulting_saturation_supply_humidity_ratio,
        predecessor.resulting_supply_humidity_ratio_original,
        predecessor.predecessor_cp411_resulting_supply_humidity_ratio,
        snapshot.saturation_supply_humidity_ratio_for_guard,
        snapshot.original_supply_humidity_ratio_for_guard,
        snapshot.saturation_supply_humidity_ratio_strictly_less_than_original_supply_humidity_ratio,
    ) else {
        return false;
    };
    snapshot
        .post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_evaluated
        && snapshot.cp412_saturation_supply_humidity_ratio_owned_read
        && snapshot.saturation_supply_humidity_ratio_for_guard_read
        && saturation.to_bits() == predecessor_saturation.to_bits()
        && snapshot.cp411_original_supply_humidity_ratio_owned_read
        && snapshot.cp412_same_call_original_supply_humidity_ratio_bit_corroborated
        && predecessor_original.to_bits() == cp411_terminal.to_bits()
        && snapshot.original_supply_humidity_ratio_for_guard_read
        && original.to_bits() == predecessor_original.to_bits()
        && snapshot.saturation_original_supply_humidity_ratio_comparison_evaluated
        && comparison == (saturation < original)
        && snapshot.saturation_supply_humidity_ratio_guard_body_entered == comparison
        && snapshot.saturation_supply_humidity_ratio_guard_false_fallthrough != comparison
}

pub(in crate::ideal_loads::calc) fn cp412_shape(snapshot: Snapshot) -> Predecessor {
    use crate::ideal_loads::{
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE as EXCLUDED,
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE as SOURCE,
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE_ORDER as ORDER,
    };
    Predecessor {
        source: SOURCE,
        first_excluded_source: EXCLUDED,
        source_order: ORDER,
        system: snapshot.system,
        parent_call_ordinal: snapshot.parent_call_ordinal,
        controlled_zone: snapshot.controlled_zone,
        unit_off_skipped: snapshot.unit_off_skipped,
        non_cooling_skipped: snapshot.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: snapshot.positive_guard_false_fallthrough_skipped,
        heating_availability_guard_false_fallthrough: snapshot.heating_availability_guard_false_fallthrough,
        humidification_control_guard_false_fallthrough: snapshot.humidification_control_guard_false_fallthrough,
        dehumidification_control_humidistat_maximum_assignment_executed: snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        dehumidification_control_none_maximum_assignment_executed: snapshot.dehumidification_control_none_maximum_assignment_executed,
        dehumidification_control_guard_false_fallthrough: snapshot.dehumidification_control_guard_false_fallthrough,
        predecessor_capacity_limit_guard_evaluated: snapshot.predecessor_capacity_limit_guard_evaluated,
        predecessor_capacity_limit_body_entered: snapshot.predecessor_capacity_limit_body_entered,
        predecessor_active_capacity_limit_guard_false_fallthrough: snapshot.predecessor_active_capacity_limit_guard_false_fallthrough,
        predecessor_dehumidification_guard_evaluated: snapshot.predecessor_dehumidification_guard_evaluated,
        predecessor_dehumidification_body_entered: snapshot.predecessor_dehumidification_body_entered,
        predecessor_dehumidification_guard_false_fallthrough: snapshot.predecessor_dehumidification_guard_false_fallthrough,
        predecessor_dehumidification_total_output_assignment_executed: snapshot.predecessor_dehumidification_total_output_assignment_executed,
        predecessor_dehumidification_total_output_capacity_guard_evaluated: snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated,
        predecessor_dehumidification_total_output_capacity_adjustment_body_entered: snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        predecessor_dehumidification_total_output_capacity_guard_false_fallthrough: snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_capacity_guard_false_fallthrough: snapshot.dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_maximum_capacity_assignment_executed: snapshot.dehumidification_total_output_maximum_capacity_assignment_executed,
        predecessor_supply_enthalpy_assignment_executed: snapshot.predecessor_supply_enthalpy_assignment_executed,
        predecessor_dehumidification_control_type_read: snapshot.predecessor_dehumidification_control_type_read,
        predecessor_dehumidification_control_type: snapshot.predecessor_dehumidification_control_type,
        predecessor_dehumidification_control_switch_dispatched: snapshot.predecessor_dehumidification_control_switch_dispatched,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered: snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break: snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break,
        predecessor_dehumidification_control_humidistat_case_entered: snapshot.predecessor_dehumidification_control_humidistat_case_entered,
        predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed: snapshot.predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed,
        predecessor_dehumidification_control_humidistat_case_exited_via_break: snapshot.predecessor_dehumidification_control_humidistat_case_exited_via_break,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered: snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough: snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed: snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break: snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break,
        predecessor_cp409_resulting_supply_humidity_ratio: snapshot.predecessor_cp409_resulting_supply_humidity_ratio,
        predecessor_cp409_resulting_supply_enthalpy_j_per_kg: snapshot.predecessor_cp409_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp409_resulting_supply_temperature_c: snapshot.predecessor_cp409_resulting_supply_temperature_c,
        predecessor_dehumidification_control_default_case_exited_via_break: snapshot.predecessor_dehumidification_control_default_case_exited_via_break,
        predecessor_cp410_resulting_supply_humidity_ratio: snapshot.predecessor_cp410_resulting_supply_humidity_ratio,
        predecessor_cp410_resulting_supply_enthalpy_j_per_kg: snapshot.predecessor_cp410_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp410_resulting_supply_temperature_c: snapshot.predecessor_cp410_resulting_supply_temperature_c,
        post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_executed: snapshot.post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_executed,
        cp410_retained_supply_humidity_ratio_state_owned: snapshot.cp410_retained_supply_humidity_ratio_state_owned,
        cp410_retained_supply_enthalpy_state_owned: snapshot.cp410_retained_supply_enthalpy_state_owned,
        cp410_retained_supply_temperature_state_owned: snapshot.cp410_retained_supply_temperature_state_owned,
        cp410_retained_supply_humidity_ratio_owned_read: snapshot.cp410_retained_supply_humidity_ratio_owned_read,
        purchased_air_supply_humidity_ratio_read: snapshot.purchased_air_supply_humidity_ratio_read,
        purchased_air_supply_humidity_ratio_before_saturation_check: snapshot.purchased_air_supply_humidity_ratio_before_saturation_check,
        local_supply_humidity_ratio_original_assignment_performed: snapshot.local_supply_humidity_ratio_original_assignment_performed,
        assigned_supply_humidity_ratio_original: snapshot.assigned_supply_humidity_ratio_original,
        resulting_supply_humidity_ratio_original: snapshot.resulting_supply_humidity_ratio_original,
        predecessor_cp411_resulting_supply_humidity_ratio: snapshot.predecessor_cp411_resulting_supply_humidity_ratio,
        predecessor_cp411_resulting_supply_enthalpy_j_per_kg: snapshot.predecessor_cp411_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp411_resulting_supply_temperature_c: snapshot.predecessor_cp411_resulting_supply_temperature_c,
        post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_executed: snapshot.post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_executed,
        cp411_retained_supply_humidity_ratio_state_owned: snapshot.cp411_retained_supply_humidity_ratio_state_owned,
        cp411_retained_supply_enthalpy_state_owned: snapshot.cp411_retained_supply_enthalpy_state_owned,
        cp411_retained_supply_temperature_state_owned: snapshot.cp411_retained_supply_temperature_state_owned,
        cp411_retained_supply_temperature_owned_read: snapshot.cp411_retained_supply_temperature_owned_read,
        purchased_air_supply_temperature_for_saturation_humidity_ratio_read: snapshot.purchased_air_supply_temperature_for_saturation_humidity_ratio_read,
        supply_temperature_for_saturation_humidity_ratio_c: snapshot.supply_temperature_for_saturation_humidity_ratio_c,
        environment_outdoor_barometric_pressure_owned_read: snapshot.environment_outdoor_barometric_pressure_owned_read,
        environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read: snapshot.environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read,
        outdoor_barometric_pressure_pa: snapshot.outdoor_barometric_pressure_pa,
        psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluated: snapshot.psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluated,
        saturation_supply_humidity_ratio: snapshot.saturation_supply_humidity_ratio,
        local_saturation_supply_humidity_ratio_assignment_performed: snapshot.local_saturation_supply_humidity_ratio_assignment_performed,
        assigned_saturation_supply_humidity_ratio: snapshot.assigned_saturation_supply_humidity_ratio,
        resulting_saturation_supply_humidity_ratio: snapshot.resulting_saturation_supply_humidity_ratio,
        resulting_supply_humidity_ratio: snapshot.predecessor_cp412_resulting_supply_humidity_ratio,
        resulting_supply_enthalpy_j_per_kg: snapshot.predecessor_cp412_resulting_supply_enthalpy_j_per_kg,
        resulting_supply_temperature_c: snapshot.predecessor_cp412_resulting_supply_temperature_c,
    }
}

pub(in crate::ideal_loads::calc) const fn route_is_active(route: RetainedRoute) -> bool {
    route.active && matches!(route.predecessor_index, 18..=29)
}

pub(in crate::ideal_loads::calc) const fn logical_route_index(route: RetainedRoute) -> usize {
    let mut extra = 0;
    let mut index = 0;
    while index < route.predecessor_index {
        if predecessor_index_is_split(index) {
            extra += 1;
        }
        index += 1;
    }
    route.predecessor_index
        + extra
        + if route.predecessor_maximum_capacity_assignment_executed {
            1
        } else {
            0
        }
}

pub(in crate::ideal_loads::calc) const fn predecessor_index_is_split(index: usize) -> bool {
    matches!(index, 20 | 21 | 24 | 25 | 27 | 29)
}

pub(in crate::ideal_loads::calc) const fn predecessor_index_is_public(index: usize) -> bool {
    matches!(index, 0..=8 | 20 | 24)
}

pub(super) const fn predecessor_has_supply_humidity_ratio(route: RetainedRoute) -> bool {
    route_is_active(route)
}

pub(super) const fn predecessor_has_supply_enthalpy(index: usize) -> bool {
    matches!(index, 5 | 8 | 11 | 14 | 17..=29)
}

pub(super) const fn predecessor_has_supply_temperature(index: usize) -> bool {
    index >= 3
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
