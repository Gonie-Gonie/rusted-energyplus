//! CP360 predecessor provenance, route, and binary64-shape validation.

use ep_model::DehumidificationControlType;

use super::{Predecessor, Route};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE_ORDER,
};

pub(in crate::ideal_loads::calc) fn predecessor_route(predecessor: Predecessor) -> Option<Route> {
    if predecessor.source
        != PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source_order
            != PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE_ORDER
    {
        return None;
    }
    let predecessor_count = usize::from(
        predecessor.predecessor_dehumidification_control_none_case_completed_skip,
    ) + usize::from(
        predecessor
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
    ) + usize::from(
        predecessor
            .predecessor_dehumidification_control_humidistat_moisture_demand_assignment_executed,
    ) + usize::from(
        predecessor
            .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
    );
    let local_count = usize::from(predecessor.dehumidification_control_none_case_completed_skip)
        + usize::from(
            predecessor.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        )
        + usize::from(
            predecessor
                .dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_executed,
        )
        + usize::from(
            predecessor.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
        );
    let route = if predecessor.unit_off_skipped
        && !predecessor.non_cooling_skipped
        && !predecessor.unit_body_entered
        && inactive_prefix(predecessor)
        && predecessor_count == 0
        && local_count == 0
    {
        Route::UnitOff
    } else if !predecessor.unit_off_skipped
        && predecessor.non_cooling_skipped
        && predecessor.unit_body_entered
        && inactive_prefix(predecessor)
        && predecessor_count == 0
        && local_count == 0
    {
        Route::NonCooling
    } else if positive_guard_fallthrough(predecessor) && predecessor_count == 0 && local_count == 0
    {
        Route::PositiveGuardFalseFallthrough
    } else {
        if !active_prefix(predecessor) || predecessor_count != 1 || local_count != 1 {
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
            if predecessor
                .predecessor_dehumidification_control_humidistat_moisture_demand_assignment_executed
                && predecessor
                    .dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_executed =>
        {
            Some(
                Route::DehumidificationControlHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitExecuted,
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
        == Route::DehumidificationControlHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitExecuted;
    let flags = [
        predecessor.zone_dehumidifying_setpoint_moisture_demand_read,
        predecessor.supply_mass_flow_rate_read,
        predecessor.moisture_demand_derived_supply_humidity_ratio_calculated,
        predecessor.zone_node_humidity_ratio_read,
        predecessor.supply_humidity_ratio_for_dehumidification_calculated,
        predecessor.supply_humidity_ratio_for_dehumidification_assigned,
    ];
    let values = [
        predecessor.predecessor_resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        predecessor.zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        predecessor.supply_mass_flow_rate_kg_per_s,
        predecessor.moisture_demand_derived_supply_humidity_ratio,
        predecessor.zone_node_humidity_ratio,
        predecessor.calculated_supply_humidity_ratio_for_dehumidification,
        predecessor.assigned_supply_humidity_ratio_for_dehumidification,
        predecessor.resulting_supply_humidity_ratio_for_dehumidification,
    ];
    if !active {
        return flags.into_iter().all(|flag| !flag)
            && values.into_iter().all(|value| value.is_none());
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
        predecessor.predecessor_resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        predecessor.zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        predecessor.supply_mass_flow_rate_kg_per_s,
        predecessor.moisture_demand_derived_supply_humidity_ratio,
        predecessor.zone_node_humidity_ratio,
        predecessor.calculated_supply_humidity_ratio_for_dehumidification,
        predecessor.assigned_supply_humidity_ratio_for_dehumidification,
        predecessor.resulting_supply_humidity_ratio_for_dehumidification,
    )
    else {
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

pub(in crate::ideal_loads::calc) fn predecessor_snapshots_match_bit_exact(
    mut left: Predecessor,
    mut right: Predecessor,
) -> bool {
    let values_match = [
        (
            left.predecessor_resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
            right.predecessor_resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        ),
        (
            left.zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
            right.zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        ),
        (
            left.supply_mass_flow_rate_kg_per_s,
            right.supply_mass_flow_rate_kg_per_s,
        ),
        (
            left.moisture_demand_derived_supply_humidity_ratio,
            right.moisture_demand_derived_supply_humidity_ratio,
        ),
        (
            left.zone_node_humidity_ratio,
            right.zone_node_humidity_ratio,
        ),
        (
            left.calculated_supply_humidity_ratio_for_dehumidification,
            right.calculated_supply_humidity_ratio_for_dehumidification,
        ),
        (
            left.assigned_supply_humidity_ratio_for_dehumidification,
            right.assigned_supply_humidity_ratio_for_dehumidification,
        ),
        (
            left.resulting_supply_humidity_ratio_for_dehumidification,
            right.resulting_supply_humidity_ratio_for_dehumidification,
        ),
    ]
    .into_iter()
    .all(|(left, right)| option_bits_match(left, right));
    for snapshot in [&mut left, &mut right] {
        snapshot.predecessor_resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s = None;
        snapshot.zone_dehumidifying_setpoint_moisture_demand_kg_per_s = None;
        snapshot.supply_mass_flow_rate_kg_per_s = None;
        snapshot.moisture_demand_derived_supply_humidity_ratio = None;
        snapshot.zone_node_humidity_ratio = None;
        snapshot.calculated_supply_humidity_ratio_for_dehumidification = None;
        snapshot.assigned_supply_humidity_ratio_for_dehumidification = None;
        snapshot.resulting_supply_humidity_ratio_for_dehumidification = None;
    }
    values_match && left == right
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
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
