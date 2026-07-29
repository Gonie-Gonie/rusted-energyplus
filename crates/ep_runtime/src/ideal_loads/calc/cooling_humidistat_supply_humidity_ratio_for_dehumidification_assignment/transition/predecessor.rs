//! CP359 predecessor provenance, route, and numeric-shape validation.

use ep_model::DehumidificationControlType;

use super::{Predecessor, Route};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER,
};

pub(in crate::ideal_loads::calc) fn predecessor_route(predecessor: Predecessor) -> Option<Route> {
    if predecessor.source != PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source_order
            != PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER
    {
        return None;
    }
    let prior_count = usize::from(
        predecessor.predecessor_dehumidification_control_none_case_completed_skip,
    ) + usize::from(
        predecessor
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
    ) + usize::from(
        predecessor.predecessor_dehumidification_control_humidistat_case_entered,
    ) + usize::from(
        predecessor
            .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
    );
    let local_count = usize::from(predecessor.dehumidification_control_none_case_completed_skip)
        + usize::from(
            predecessor.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        )
        + usize::from(
            predecessor.dehumidification_control_humidistat_moisture_demand_assignment_executed,
        )
        + usize::from(
            predecessor.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
        );
    let route = if predecessor.unit_off_skipped
        && !predecessor.non_cooling_skipped
        && !predecessor.unit_body_entered
        && inactive_prefix(predecessor)
        && prior_count == 0
        && local_count == 0
    {
        Route::UnitOff
    } else if !predecessor.unit_off_skipped
        && predecessor.non_cooling_skipped
        && predecessor.unit_body_entered
        && inactive_prefix(predecessor)
        && prior_count == 0
        && local_count == 0
    {
        Route::NonCooling
    } else if positive_guard_fallthrough(predecessor) && prior_count == 0 && local_count == 0 {
        Route::PositiveGuardFalseFallthrough
    } else {
        if !active_prefix(predecessor) || prior_count != 1 || local_count != 1 {
            return None;
        }
        active_route(predecessor)?
    };
    predecessor_numeric_shape_matches(predecessor, route).then_some(route)
}

fn active_route(predecessor: Predecessor) -> Option<Route> {
    match predecessor.predecessor_dehumidification_control_type? {
        DehumidificationControlType::None
            if predecessor.predecessor_dehumidification_control_none_case_completed_skip
                && predecessor.dehumidification_control_none_case_completed_skip =>
        {
            Some(Route::DehumidificationControlNoneCaseCompletedSkip)
        }
        DehumidificationControlType::ConstantSensibleHeatRatio
            if predecessor
                .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
                && predecessor
                    .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip =>
        {
            Some(Route::DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip)
        }
        DehumidificationControlType::Humidistat
            if predecessor.predecessor_dehumidification_control_humidistat_case_entered
                && predecessor
                    .dehumidification_control_humidistat_moisture_demand_assignment_executed =>
        {
            Some(
                Route::DehumidificationControlHumidistatSupplyHumidityRatioForDehumidificationAssignmentExecuted,
            )
        }
        DehumidificationControlType::ConstantSupplyHumidityRatio
            if predecessor
                .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
                && predecessor
                    .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip =>
        {
            Some(Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip)
        }
        _ => None,
    }
}

fn predecessor_numeric_shape_matches(predecessor: Predecessor, route: Route) -> bool {
    let active = route
        == Route::DehumidificationControlHumidistatSupplyHumidityRatioForDehumidificationAssignmentExecuted;
    if !active {
        return !predecessor.zone_dehumidifying_setpoint_moisture_demand_read
            && predecessor
                .zone_dehumidifying_setpoint_moisture_demand_kg_per_s
                .is_none()
            && !predecessor.zone_dehumidifying_setpoint_moisture_demand_assigned
            && predecessor
                .assigned_zone_dehumidifying_setpoint_moisture_demand_kg_per_s
                .is_none()
            && predecessor
                .resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s
                .is_none();
    }
    let (Some(read), Some(assigned), Some(resulting)) = (
        predecessor.zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        predecessor.assigned_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        predecessor.resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
    ) else {
        return false;
    };
    predecessor.zone_dehumidifying_setpoint_moisture_demand_read
        && predecessor.zone_dehumidifying_setpoint_moisture_demand_assigned
        && read.to_bits() == assigned.to_bits()
        && read.to_bits() == resulting.to_bits()
}

fn positive_guard_fallthrough(predecessor: Predecessor) -> bool {
    !predecessor.unit_off_skipped
        && !predecessor.non_cooling_skipped
        && predecessor.unit_body_entered
        && predecessor.predecessor_cooling_body_entered
        && predecessor.predecessor_no_outdoor_air_fallback_entered
        && !predecessor.predecessor_positive_supply_mass_flow_body_entered
        && predecessor.positive_guard_false_fallthrough_skipped
        && predecessor
            .predecessor_dehumidification_control_type
            .is_none()
}

fn active_prefix(predecessor: Predecessor) -> bool {
    !predecessor.unit_off_skipped
        && !predecessor.non_cooling_skipped
        && predecessor.unit_body_entered
        && predecessor.predecessor_cooling_body_entered
        && predecessor.predecessor_no_outdoor_air_fallback_entered
        && predecessor.predecessor_positive_supply_mass_flow_body_entered
        && !predecessor.positive_guard_false_fallthrough_skipped
        && predecessor
            .predecessor_dehumidification_control_type
            .is_some()
}

fn inactive_prefix(predecessor: Predecessor) -> bool {
    !predecessor.predecessor_cooling_body_entered
        && !predecessor.predecessor_no_outdoor_air_fallback_entered
        && !predecessor.predecessor_positive_supply_mass_flow_body_entered
        && !predecessor.positive_guard_false_fallthrough_skipped
        && predecessor
            .predecessor_dehumidification_control_type
            .is_none()
}
