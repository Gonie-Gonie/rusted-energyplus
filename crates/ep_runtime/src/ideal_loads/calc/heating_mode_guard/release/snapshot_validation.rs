//! Exact CP431 prefix, route, local-shape, and bitwise validation.

use super::prefix::predecessor_cp430_snapshot;
use super::super::{
    PURCHASED_AIR_CALC_HEATING_MODE_GUARD_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_HEATING_MODE_GUARD_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_HEATING_MODE_GUARD_SOURCE_ORDER as ORDER,
    PurchasedAirCalcHeatingModeGuardSnapshot as Snapshot,
};
use super::super::transition::{
    PurchasedAirCalcHeatingModeGuardActiveInput as ActiveInput,
    PurchasedAirCalcHeatingModeGuardRetainedRoute as Route,
    heating_mode_guard_route_from_committed_predecessor,
    predecessor_route,
};
use crate::ideal_loads::calc::PurchasedAirCalcHeatingOrNoLoadCaseEntryRetainedRoute as PredecessorRoute;

pub(super) fn snapshot_is_exact(snapshot: Snapshot) -> bool {
    snapshot_route(snapshot).is_some()
        && crate::ideal_loads::heating_or_no_load_case_entry_snapshot_is_exact(
            predecessor_cp430_snapshot(snapshot),
        )
}

pub(super) fn snapshot_route(snapshot: Snapshot) -> Option<Route> {
    let predecessor = predecessor_cp430_snapshot(snapshot);
    let predecessor_route = predecessor_route(predecessor)?;
    let input = input_from_snapshot(snapshot)?;
    let route = heating_mode_guard_route_from_committed_predecessor(
        predecessor,
        predecessor_route,
        input,
    )?;
    prefix_and_local_shape_match(snapshot, predecessor, input, route).then_some(route)
}

pub(super) fn retained_route_matches_snapshot_bounded(
    snapshot: Snapshot,
    predecessor_route: PredecessorRoute,
    route: Route,
) -> bool {
    let predecessor = predecessor_cp430_snapshot(snapshot);
    let Some(input) = input_from_snapshot(snapshot) else {
        return false;
    };
    heating_mode_guard_route_from_committed_predecessor(
        predecessor,
        predecessor_route,
        input,
    ) == Some(route)
        && local_shape_is_exact(snapshot, predecessor, input, route)
}

pub(super) fn retained_route_matches_prior_snapshot_bounded(
    snapshot: Snapshot,
    route: Route,
) -> bool {
    retained_route_matches_snapshot_bounded(
        snapshot,
        PredecessorRoute {
            logical_index: route.logical_index,
            active: route.predecessor_active,
            predecessor_assignment_executed: route.predecessor_assignment_executed,
            predecessor_entered: route.predecessor_entered,
            assignment_executed: route.predecessor_total_output_assignment_executed,
            entered: route.predecessor_heating_or_no_load_case_entered,
        },
        route,
    )
}

pub(super) fn prefix_and_local_shape_match(
    snapshot: Snapshot,
    predecessor: crate::ideal_loads::PurchasedAirCalcHeatingOrNoLoadCaseEntrySnapshot,
    input: Option<ActiveInput>,
    route: Route,
) -> bool {
    crate::ideal_loads::heating_or_no_load_case_entry_snapshots_match_bit_exact(
        predecessor_cp430_snapshot(snapshot),
        predecessor,
    ) && local_shape_is_exact(snapshot, predecessor, input, route)
}

fn input_from_snapshot(snapshot: Snapshot) -> Option<Option<ActiveInput>> {
    if !snapshot.heating_mode_guard_evaluated {
        let local_absent = snapshot
            .minimum_outdoor_air_sensible_output_for_heating_mode_guard_w
            .is_none()
            && snapshot
                .heating_setpoint_demand_for_heating_mode_guard_w
                .is_none()
            && snapshot.temperature_control_type.is_none();
        return local_absent.then_some(None);
    }
    let minimum = snapshot.minimum_outdoor_air_sensible_output_for_heating_mode_guard_w?;
    let heating = snapshot.heating_setpoint_demand_for_heating_mode_guard_w?;
    Some(Some(ActiveInput {
        numeric: crate::ideal_loads::calc::PurchasedAirCalcCoolingEntryGateCommittedHeatingModeGuardNumericOperands {
            minimum_outdoor_air_sensible_output_w: minimum,
            heating_setpoint_demand_w: heating,
        },
        temperature_control_type: snapshot.temperature_control_type,
    }))
}

fn local_shape_is_exact(
    snapshot: Snapshot,
    predecessor: crate::ideal_loads::PurchasedAirCalcHeatingOrNoLoadCaseEntrySnapshot,
    input: Option<ActiveInput>,
    route: Route,
) -> bool {
    let numeric = input.map(|value| value.numeric);
    let temperature = input.and_then(|value| value.temperature_control_type);
    let minimum = numeric.map(|value| value.minimum_outdoor_air_sensible_output_w);
    let heating = numeric.map(|value| value.heating_setpoint_demand_w);
    let permits = temperature.map(|value| {
        value != crate::ideal_loads::PurchasedAirTemperatureControlType::SingleCool
    });
    snapshot.source == SOURCE
        && snapshot.first_excluded_source == EXCLUDED
        && snapshot.source_order == ORDER
        && snapshot.system == predecessor.system
        && snapshot.parent_call_ordinal == predecessor.parent_call_ordinal
        && snapshot.controlled_zone == predecessor.controlled_zone
        && snapshot.heating_or_no_load_case_entered
            == predecessor.heating_or_no_load_case_entered
        && snapshot.heating_mode_guard_evaluated == route.guard_evaluated
        && snapshot.cp311_retained_minimum_outdoor_air_sensible_output_owned_read
            == route.guard_evaluated
        && snapshot.cp312_same_call_minimum_outdoor_air_sensible_output_bit_corroborated
            == route.guard_evaluated
        && snapshot.minimum_outdoor_air_sensible_output_for_heating_mode_guard_read
            == route.guard_evaluated
        && same(
            snapshot.minimum_outdoor_air_sensible_output_for_heating_mode_guard_w,
            minimum,
        )
        && snapshot.cp310_retained_heating_setpoint_demand_owned_read
            == route.guard_evaluated
        && snapshot.heating_setpoint_demand_for_heating_mode_guard_read
            == route.guard_evaluated
        && same(snapshot.heating_setpoint_demand_for_heating_mode_guard_w, heating)
        && snapshot
            .minimum_outdoor_air_sensible_output_heating_setpoint_demand_comparison_evaluated
            == route.guard_evaluated
        && snapshot
            .minimum_outdoor_air_sensible_output_strictly_less_than_heating_setpoint_demand
            == route.guard_evaluated.then_some(route.sensible_comparison_satisfied)
        && snapshot.prevalidated_temperature_control_type_owned_read
            == route.sensible_comparison_satisfied
        && snapshot.temperature_control_type_read_after_sensible_comparison_short_circuit
            == route.sensible_comparison_satisfied
        && snapshot.temperature_control_type == temperature
        && snapshot.temperature_control_type_single_cool_comparison_evaluated
            == route.sensible_comparison_satisfied
        && snapshot.temperature_control_type_permits_heating == permits
        && snapshot.single_cool_blocked == route.single_cool_blocked
        && snapshot.heating_operating_mode_body_entered == route.body_entered
        && snapshot.heating_mode_guard_false_fallthrough == route.false_fallthrough
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
        )
}

fn same(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

pub(super) fn snapshots_match_bit_exact(mut left: Snapshot, mut right: Snapshot) -> bool {
    macro_rules! clear {
        ($field:ident) => {
            if !same(left.$field, right.$field) {
                return false;
            }
            left.$field = None;
            right.$field = None;
        };
    }
    clear!(predecessor_cp409_resulting_supply_humidity_ratio);
    clear!(predecessor_cp409_resulting_supply_enthalpy_j_per_kg);
    clear!(predecessor_cp409_resulting_supply_temperature_c);
    clear!(predecessor_cp410_resulting_supply_humidity_ratio);
    clear!(predecessor_cp410_resulting_supply_enthalpy_j_per_kg);
    clear!(predecessor_cp410_resulting_supply_temperature_c);
    clear!(purchased_air_supply_humidity_ratio_before_saturation_check);
    clear!(assigned_supply_humidity_ratio_original);
    clear!(resulting_supply_humidity_ratio_original);
    clear!(predecessor_cp411_resulting_supply_humidity_ratio);
    clear!(predecessor_cp411_resulting_supply_enthalpy_j_per_kg);
    clear!(predecessor_cp411_resulting_supply_temperature_c);
    clear!(supply_temperature_for_saturation_humidity_ratio_c);
    clear!(outdoor_barometric_pressure_pa);
    clear!(saturation_supply_humidity_ratio);
    clear!(assigned_saturation_supply_humidity_ratio);
    clear!(resulting_saturation_supply_humidity_ratio);
    clear!(predecessor_cp412_resulting_supply_humidity_ratio);
    clear!(predecessor_cp412_resulting_supply_enthalpy_j_per_kg);
    clear!(predecessor_cp412_resulting_supply_temperature_c);
    clear!(saturation_supply_humidity_ratio_for_guard);
    clear!(original_supply_humidity_ratio_for_guard);
    clear!(predecessor_cp413_resulting_supply_humidity_ratio);
    clear!(predecessor_cp413_resulting_supply_enthalpy_j_per_kg);
    clear!(predecessor_cp413_resulting_supply_temperature_c);
    clear!(supply_enthalpy_for_saturation_temperature_j_per_kg);
    clear!(outdoor_barometric_pressure_for_saturation_temperature_pa);
    clear!(psychrometric_saturation_supply_temperature_result_c);
    clear!(assigned_saturation_supply_temperature_c);
    clear!(predecessor_cp414_resulting_supply_humidity_ratio);
    clear!(predecessor_cp414_resulting_supply_enthalpy_j_per_kg);
    clear!(predecessor_cp414_resulting_supply_temperature_c);
    clear!(preexisting_supply_temperature_c);
    clear!(supply_temperature_before_mixed_air_limit_c);
    clear!(mixed_air_temperature_c);
    clear!(minimum_supply_temperature_c);
    clear!(assigned_supply_temperature_c);
    clear!(predecessor_cp415_resulting_supply_humidity_ratio);
    clear!(predecessor_cp415_resulting_supply_enthalpy_j_per_kg);
    clear!(predecessor_cp415_resulting_supply_temperature_c);
    clear!(supply_temperature_c);
    clear!(supply_enthalpy_j_per_kg);
    clear!(psychrometric_supply_humidity_ratio);
    clear!(assigned_supply_humidity_ratio);
    clear!(predecessor_cp416_resulting_supply_humidity_ratio);
    clear!(predecessor_cp416_resulting_supply_enthalpy_j_per_kg);
    clear!(predecessor_cp416_resulting_supply_temperature_c);
    clear!(supply_temperature_for_enthalpy_c);
    clear!(supply_humidity_ratio_for_enthalpy);
    clear!(psychrometric_supply_enthalpy_j_per_kg);
    clear!(assigned_supply_enthalpy_j_per_kg);
    clear!(predecessor_cp418_resulting_supply_humidity_ratio);
    clear!(predecessor_cp418_resulting_supply_enthalpy_j_per_kg);
    clear!(predecessor_cp418_resulting_supply_temperature_c);
    clear!(mixed_air_humidity_ratio_for_cp_air);
    clear!(psychrometric_cp_air_result_j_per_kg_k);
    clear!(cp_air_j_per_kg_k);
    clear!(predecessor_cp419_resulting_supply_humidity_ratio);
    clear!(predecessor_cp419_resulting_supply_enthalpy_j_per_kg);
    clear!(predecessor_cp419_resulting_supply_temperature_c);
    clear!(supply_mass_flow_rate_kg_per_s);
    clear!(cp419_cp_air_for_sensible_output_j_per_kg_k);
    clear!(supply_mass_flow_rate_times_cp_air_w_per_k);
    clear!(mixed_air_temperature_for_sensible_output_c);
    clear!(supply_temperature_for_sensible_output_c);
    clear!(mixed_air_minus_supply_temperature_k);
    clear!(calculated_cooling_sensible_output_w);
    clear!(cooling_sensible_output_w);
    clear!(predecessor_cp420_resulting_supply_humidity_ratio);
    clear!(predecessor_cp420_resulting_supply_enthalpy_j_per_kg);
    clear!(predecessor_cp420_resulting_supply_temperature_c);
    clear!(cp420_cooling_sensible_output_for_capacity_guard_w);
    clear!(maximum_total_cooling_capacity_w);
    clear!(predecessor_cp421_resulting_supply_humidity_ratio);
    clear!(predecessor_cp421_resulting_supply_enthalpy_j_per_kg);
    clear!(predecessor_cp421_resulting_supply_temperature_c);
    clear!(preexisting_cooling_sensible_output_for_maximum_capacity_assignment_w);
    clear!(maximum_total_cooling_capacity_for_sensible_output_assignment_w);
    clear!(assigned_cooling_sensible_output_from_maximum_capacity_w);
    clear!(resulting_cooling_sensible_output_after_maximum_capacity_assignment_w);
    clear!(predecessor_cp422_resulting_supply_humidity_ratio);
    clear!(predecessor_cp422_resulting_supply_enthalpy_j_per_kg);
    clear!(predecessor_cp422_resulting_supply_temperature_c);
    clear!(mixed_air_temperature_for_sensible_output_supply_temperature_c);
    clear!(cooling_sensible_output_for_supply_temperature_w);
    clear!(supply_mass_flow_rate_for_sensible_output_supply_temperature_kg_per_s);
    clear!(cp_air_for_sensible_output_supply_temperature_j_per_kg_k);
    clear!(supply_mass_flow_rate_times_cp_air_for_sensible_output_supply_temperature_w_per_k);
    clear!(cooling_sensible_output_over_air_capacity_rate_k);
    clear!(calculated_sensible_output_supply_temperature_c);
    clear!(assigned_sensible_output_supply_temperature_c);
    clear!(predecessor_cp424_resulting_supply_humidity_ratio);
    clear!(predecessor_cp424_resulting_supply_enthalpy_j_per_kg);
    clear!(predecessor_cp424_resulting_supply_temperature_c);
    clear!(mixed_air_enthalpy_for_zero_supply_mass_flow_supply_enthalpy_j_per_kg);
    clear!(assigned_supply_enthalpy_from_mixed_air_j_per_kg);
    clear!(predecessor_cp425_resulting_supply_humidity_ratio);
    clear!(predecessor_cp425_resulting_supply_enthalpy_j_per_kg);
    clear!(predecessor_cp425_resulting_supply_temperature_c);
    clear!(mixed_air_humidity_ratio_for_zero_supply_mass_flow_supply_humidity_ratio_assignment);
    clear!(assigned_supply_humidity_ratio_from_mixed_air);
    clear!(predecessor_cp426_resulting_supply_humidity_ratio);
    clear!(predecessor_cp426_resulting_supply_enthalpy_j_per_kg);
    clear!(predecessor_cp426_resulting_supply_temperature_c);
    clear!(mixed_air_temperature_for_zero_supply_mass_flow_supply_temperature_assignment_c);
    clear!(assigned_supply_temperature_from_mixed_air_c);
    clear!(predecessor_cp427_resulting_supply_humidity_ratio);
    clear!(predecessor_cp427_resulting_supply_enthalpy_j_per_kg);
    clear!(predecessor_cp427_resulting_supply_temperature_c);
    clear!(assigned_cooling_sensible_output_w);
    clear!(predecessor_cp428_resulting_supply_humidity_ratio);
    clear!(predecessor_cp428_resulting_supply_enthalpy_j_per_kg);
    clear!(predecessor_cp428_resulting_supply_temperature_c);
    clear!(assigned_cooling_total_output_w);
    clear!(predecessor_cp430_resulting_supply_humidity_ratio);
    clear!(predecessor_cp430_resulting_supply_enthalpy_j_per_kg);
    clear!(predecessor_cp430_resulting_supply_temperature_c);
    clear!(minimum_outdoor_air_sensible_output_for_heating_mode_guard_w);
    clear!(heating_setpoint_demand_for_heating_mode_guard_w);
    clear!(resulting_supply_humidity_ratio);
    clear!(resulting_supply_enthalpy_j_per_kg);
    clear!(resulting_supply_temperature_c);
    left == right
}
