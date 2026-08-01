//! Exact CP373 snapshot and binary64 validation.

use super::super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentSnapshot as Predecessor,
    cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_snapshot_is_exact_direct_release,
};

mod route;

pub(in crate::ideal_loads) fn cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_snapshot_is_exact_direct_release(
    snapshot: Snapshot,
) -> bool {
    cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_snapshot_is_exact_direct_release(
        predecessor_snapshot(snapshot),
    ) && matches!(
        snapshot_route(snapshot),
        Some(
            Route::UnitOff
                | Route::NonCooling
                | Route::PositiveGuardFalseFallthrough
                | Route::HeatingAvailabilityGuardFalseFallthrough
                | Route::HumidificationControlGuardFalseFallthrough
        )
    )
}

pub(super) fn snapshot_route(snapshot: Snapshot) -> Option<Route> {
    let route = route::structural_route(snapshot)?;
    values_fit_route(snapshot, route).then_some(route)
}

pub(in crate::ideal_loads::calc) fn snapshots_match_bit_exact(
    mut left: Snapshot,
    mut right: Snapshot,
) -> bool {
    let values_match = [
        (
            left.predecessor_zone_humidifying_setpoint_moisture_demand_kg_per_s,
            right.predecessor_zone_humidifying_setpoint_moisture_demand_kg_per_s,
        ),
        (
            left.predecessor_assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s,
            right.predecessor_assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s,
        ),
        (
            left.predecessor_resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s,
            right.predecessor_resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s,
        ),
        (
            left.zone_humidifying_setpoint_moisture_demand_kg_per_s,
            right.zone_humidifying_setpoint_moisture_demand_kg_per_s,
        ),
        (
            left.supply_mass_flow_rate_kg_per_s,
            right.supply_mass_flow_rate_kg_per_s,
        ),
        (
            left.moisture_demand_derived_supply_humidity_ratio,
            right.moisture_demand_derived_supply_humidity_ratio,
        ),
        (left.zone_node_humidity_ratio, right.zone_node_humidity_ratio),
        (
            left.calculated_supply_humidity_ratio_for_humidification,
            right.calculated_supply_humidity_ratio_for_humidification,
        ),
        (
            left.assigned_supply_humidity_ratio_for_humidification,
            right.assigned_supply_humidity_ratio_for_humidification,
        ),
        (
            left.resulting_supply_humidity_ratio_for_humidification,
            right.resulting_supply_humidity_ratio_for_humidification,
        ),
    ]
    .into_iter()
    .all(|(left, right)| option_bits_match(left, right));
    for snapshot in [&mut left, &mut right] {
        snapshot.predecessor_zone_humidifying_setpoint_moisture_demand_kg_per_s = None;
        snapshot.predecessor_assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s = None;
        snapshot.predecessor_resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s = None;
        snapshot.zone_humidifying_setpoint_moisture_demand_kg_per_s = None;
        snapshot.supply_mass_flow_rate_kg_per_s = None;
        snapshot.moisture_demand_derived_supply_humidity_ratio = None;
        snapshot.zone_node_humidity_ratio = None;
        snapshot.calculated_supply_humidity_ratio_for_humidification = None;
        snapshot.assigned_supply_humidity_ratio_for_humidification = None;
        snapshot.resulting_supply_humidity_ratio_for_humidification = None;
    }
    values_match && left == right
}

pub(super) fn predecessor_snapshot(snapshot: Snapshot) -> Predecessor {
    Predecessor {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER,
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
        predecessor_dehumidification_control_type_first_read: snapshot.predecessor_dehumidification_control_type_first_read,
        predecessor_first_dehumidification_control_type: snapshot.predecessor_first_dehumidification_control_type,
        predecessor_dehumidification_control_type_humidistat: snapshot.predecessor_dehumidification_control_type_humidistat,
        predecessor_dehumidification_control_type_second_read: snapshot.predecessor_dehumidification_control_type_second_read,
        predecessor_second_dehumidification_control_type: snapshot.predecessor_second_dehumidification_control_type,
        predecessor_dehumidification_control_type_none: snapshot.predecessor_dehumidification_control_type_none,
        predecessor_dehumidification_control_body_entered: snapshot.predecessor_dehumidification_control_body_entered,
        predecessor_dehumidification_control_guard_false_fallthrough: snapshot.predecessor_dehumidification_control_guard_false_fallthrough,
        humidification_moisture_demand_assignment_executed: snapshot.predecessor_humidification_moisture_demand_assignment_executed,
        zone_humidifying_setpoint_moisture_demand_read: snapshot.predecessor_zone_humidifying_setpoint_moisture_demand_read,
        zone_humidifying_setpoint_moisture_demand_kg_per_s: snapshot.predecessor_zone_humidifying_setpoint_moisture_demand_kg_per_s,
        zone_humidifying_setpoint_moisture_demand_assigned: snapshot.predecessor_zone_humidifying_setpoint_moisture_demand_assigned,
        assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s: snapshot.predecessor_assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s,
        resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s: snapshot.predecessor_resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s,
    }
}

fn values_fit_route(snapshot: Snapshot, route: Route) -> bool {
    let active = matches!(
        route,
        Route::DehumidificationControlHumidistatSupplyHumidityRatioForHumidificationAssignmentExecuted
            | Route::DehumidificationControlNoneSupplyHumidityRatioForHumidificationAssignmentExecuted
    );
    let flags = [
        snapshot.zone_humidifying_setpoint_moisture_demand_read,
        snapshot.supply_mass_flow_rate_read,
        snapshot.moisture_demand_derived_supply_humidity_ratio_calculated,
        snapshot.zone_node_humidity_ratio_read,
        snapshot.supply_humidity_ratio_for_humidification_calculated,
        snapshot.supply_humidity_ratio_for_humidification_assigned,
    ];
    if !active {
        return flags.into_iter().all(|flag| !flag)
            && [
                snapshot.predecessor_resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s,
                snapshot.zone_humidifying_setpoint_moisture_demand_kg_per_s,
                snapshot.supply_mass_flow_rate_kg_per_s,
                snapshot.moisture_demand_derived_supply_humidity_ratio,
                snapshot.zone_node_humidity_ratio,
                snapshot.calculated_supply_humidity_ratio_for_humidification,
                snapshot.assigned_supply_humidity_ratio_for_humidification,
                snapshot.resulting_supply_humidity_ratio_for_humidification,
            ]
            .into_iter()
            .all(|value| value.is_none());
    }
    if !flags.into_iter().all(|flag| flag) {
        return false;
    }
    let (
        Some(predecessor_demand),
        Some(demand),
        Some(flow),
        Some(quotient),
        Some(zone_humidity),
        Some(calculated),
        Some(assigned),
        Some(resulting),
    ) = (
        snapshot.predecessor_resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s,
        snapshot.zone_humidifying_setpoint_moisture_demand_kg_per_s,
        snapshot.supply_mass_flow_rate_kg_per_s,
        snapshot.moisture_demand_derived_supply_humidity_ratio,
        snapshot.zone_node_humidity_ratio,
        snapshot.calculated_supply_humidity_ratio_for_humidification,
        snapshot.assigned_supply_humidity_ratio_for_humidification,
        snapshot.resulting_supply_humidity_ratio_for_humidification,
    ) else {
        return false;
    };
    let expected_quotient = demand / flow;
    let expected_calculated = expected_quotient + zone_humidity;
    predecessor_demand.to_bits() == demand.to_bits()
        && quotient.to_bits() == expected_quotient.to_bits()
        && calculated.to_bits() == expected_calculated.to_bits()
        && assigned.to_bits() == calculated.to_bits()
        && resulting.to_bits() == calculated.to_bits()
}

pub(super) fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
