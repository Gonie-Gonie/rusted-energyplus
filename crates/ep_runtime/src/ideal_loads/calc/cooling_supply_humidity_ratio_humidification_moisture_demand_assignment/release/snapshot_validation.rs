//! Exact CP372 humidifying-demand assignment snapshot validation.

use super::super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardRetainedRoute as PredecessorRoute,
    cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_snapshot_route,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardSnapshot as Predecessor,
    cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_snapshot_is_exact_direct_release,
};

pub(in crate::ideal_loads) fn cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_snapshot_is_exact_direct_release(
    snapshot: Snapshot,
) -> bool {
    let predecessor = predecessor_snapshot(snapshot);
    cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_snapshot_is_exact_direct_release(
        predecessor,
    ) && snapshot_route(snapshot).is_some()
}

pub(super) fn snapshot_route(snapshot: Snapshot) -> Option<Route> {
    if snapshot.source
        != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER
    {
        return None;
    }
    let route = match cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_snapshot_route(
        predecessor_snapshot(snapshot),
    )? {
        PredecessorRoute::UnitOff => Route::UnitOff,
        PredecessorRoute::NonCooling => Route::NonCooling,
        PredecessorRoute::PositiveGuardFalseFallthrough => Route::PositiveGuardFalseFallthrough,
        PredecessorRoute::HeatingAvailabilityGuardFalseFallthrough => {
            Route::HeatingAvailabilityGuardFalseFallthrough
        }
        PredecessorRoute::HumidificationControlGuardFalseFallthrough => {
            Route::HumidificationControlGuardFalseFallthrough
        }
        PredecessorRoute::DehumidificationControlHumidistatBodyEntered => {
            Route::DehumidificationControlHumidistatMoistureDemandAssignmentExecuted
        }
        PredecessorRoute::DehumidificationControlNoneBodyEntered => {
            Route::DehumidificationControlNoneMoistureDemandAssignmentExecuted
        }
        PredecessorRoute::DehumidificationControlGuardFalseFallthrough => {
            Route::DehumidificationControlGuardFalseFallthrough
        }
    };
    values_fit_route(snapshot, route).then_some(route)
}

pub(in crate::ideal_loads::calc) fn snapshots_match_bit_exact(
    mut left: Snapshot,
    mut right: Snapshot,
) -> bool {
    let values_match = option_bits_match(
        left.zone_humidifying_setpoint_moisture_demand_kg_per_s,
        right.zone_humidifying_setpoint_moisture_demand_kg_per_s,
    ) && option_bits_match(
        left.assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s,
        right.assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s,
    ) && option_bits_match(
        left.resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s,
        right.resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s,
    );
    for snapshot in [&mut left, &mut right] {
        snapshot.zone_humidifying_setpoint_moisture_demand_kg_per_s = None;
        snapshot.assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s = None;
        snapshot.resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s = None;
    }
    values_match && left == right
}

pub(super) fn predecessor_snapshot(snapshot: Snapshot) -> Predecessor {
    Predecessor {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE_ORDER,
        system: snapshot.system,
        parent_call_ordinal: snapshot.parent_call_ordinal,
        controlled_zone: snapshot.controlled_zone,
        unit_body_entered: snapshot.unit_body_entered,
        predecessor_cooling_body_entered: snapshot.predecessor_cooling_body_entered,
        predecessor_no_outdoor_air_fallback_entered: snapshot.predecessor_no_outdoor_air_fallback_entered,
        predecessor_positive_supply_mass_flow_body_entered: snapshot.predecessor_positive_supply_mass_flow_body_entered,
        unit_off_skipped: snapshot.unit_off_skipped,
        non_cooling_skipped: snapshot.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: snapshot.positive_guard_false_fallthrough_skipped,
        predecessor_dehumidification_control_type: snapshot.predecessor_dehumidification_control_type,
        predecessor_dehumidification_control_none_case_completed_skip: snapshot.predecessor_dehumidification_control_none_case_completed_skip,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        predecessor_dehumidification_control_humidistat_case_completed_skip: snapshot.predecessor_dehumidification_control_humidistat_case_completed_skip,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip: snapshot.predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
        predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break: snapshot.predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break,
        dehumidification_control_none_case_completed_skip: snapshot.dehumidification_control_none_case_completed_skip,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: snapshot.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        dehumidification_control_humidistat_case_completed_skip: snapshot.dehumidification_control_humidistat_case_completed_skip,
        dehumidification_control_constant_supply_humidity_ratio_case_completed_skip: snapshot.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
        predecessor_heating_on_read: snapshot.predecessor_heating_on_read,
        predecessor_heating_on: snapshot.predecessor_heating_on,
        predecessor_cooling_supply_humidity_ratio_humidification_body_entered: snapshot.predecessor_cooling_supply_humidity_ratio_humidification_body_entered,
        predecessor_heating_on_guard_false_fallthrough: snapshot.predecessor_heating_on_guard_false_fallthrough,
        predecessor_humidification_control_type_read: snapshot.predecessor_humidification_control_type_read,
        predecessor_humidification_control_type: snapshot.predecessor_humidification_control_type,
        predecessor_humidification_control_type_humidistat: snapshot.predecessor_humidification_control_type_humidistat,
        predecessor_humidification_control_body_entered: snapshot.predecessor_humidification_control_body_entered,
        predecessor_humidification_control_guard_false_fallthrough: snapshot.predecessor_humidification_control_guard_false_fallthrough,
        dehumidification_control_type_first_read: snapshot.predecessor_dehumidification_control_type_first_read,
        first_dehumidification_control_type: snapshot.predecessor_first_dehumidification_control_type,
        dehumidification_control_type_humidistat: snapshot.predecessor_dehumidification_control_type_humidistat,
        dehumidification_control_type_second_read: snapshot.predecessor_dehumidification_control_type_second_read,
        second_dehumidification_control_type: snapshot.predecessor_second_dehumidification_control_type,
        dehumidification_control_type_none: snapshot.predecessor_dehumidification_control_type_none,
        dehumidification_control_body_entered: snapshot.predecessor_dehumidification_control_body_entered,
        dehumidification_control_guard_false_fallthrough: snapshot.predecessor_dehumidification_control_guard_false_fallthrough,
    }
}

fn values_fit_route(snapshot: Snapshot, route: Route) -> bool {
    let active = matches!(
        route,
        Route::DehumidificationControlHumidistatMoistureDemandAssignmentExecuted
            | Route::DehumidificationControlNoneMoistureDemandAssignmentExecuted
    );
    if !active {
        return !snapshot.humidification_moisture_demand_assignment_executed
            && !snapshot.zone_humidifying_setpoint_moisture_demand_read
            && snapshot
                .zone_humidifying_setpoint_moisture_demand_kg_per_s
                .is_none()
            && !snapshot.zone_humidifying_setpoint_moisture_demand_assigned
            && snapshot
                .assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s
                .is_none()
            && snapshot
                .resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s
                .is_none();
    }
    snapshot.humidification_moisture_demand_assignment_executed
        && snapshot.zone_humidifying_setpoint_moisture_demand_read
        && snapshot.zone_humidifying_setpoint_moisture_demand_assigned
        && snapshot
            .zone_humidifying_setpoint_moisture_demand_kg_per_s
            .is_some()
        && option_bits_match(
            snapshot.zone_humidifying_setpoint_moisture_demand_kg_per_s,
            snapshot.assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s,
        )
        && option_bits_match(
            snapshot.assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s,
            snapshot.resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s,
        )
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
