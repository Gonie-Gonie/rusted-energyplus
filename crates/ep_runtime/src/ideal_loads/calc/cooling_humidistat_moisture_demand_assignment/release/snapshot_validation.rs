//! Exact CP359 snapshot validation.

use ep_model::DehumidificationControlType;

use super::super::{
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentSnapshot as Snapshot,
};

pub(in crate::ideal_loads) fn cooling_humidistat_moisture_demand_assignment_snapshot_is_exact_direct_release(
    snapshot: Snapshot,
) -> bool {
    matches!(
        snapshot_route(snapshot),
        Some(
            Route::UnitOff
                | Route::NonCooling
                | Route::PositiveGuardFalseFallthrough
                | Route::DehumidificationControlNoneCaseCompletedSkip
        )
    )
}

pub(super) fn snapshot_route(snapshot: Snapshot) -> Option<Route> {
    if !provenance_is_exact(snapshot) {
        return None;
    }
    let predecessor_count = usize::from(
        snapshot.predecessor_dehumidification_control_none_case_completed_skip,
    ) + usize::from(
        snapshot
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
    ) + usize::from(
        snapshot.predecessor_dehumidification_control_humidistat_case_entered,
    ) + usize::from(
        snapshot
            .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
    );
    let local_count = usize::from(snapshot.dehumidification_control_none_case_completed_skip)
        + usize::from(
            snapshot.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        )
        + usize::from(
            snapshot.dehumidification_control_humidistat_moisture_demand_assignment_executed,
        )
        + usize::from(
            snapshot.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
        );
    let route = if snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && !snapshot.unit_body_entered
        && inactive_prefix(snapshot)
        && predecessor_count == 0
        && local_count == 0
    {
        Route::UnitOff
    } else if !snapshot.unit_off_skipped
        && snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && inactive_prefix(snapshot)
        && predecessor_count == 0
        && local_count == 0
    {
        Route::NonCooling
    } else if !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && snapshot.positive_guard_false_fallthrough_skipped
        && snapshot.predecessor_dehumidification_control_type.is_none()
        && predecessor_count == 0
        && local_count == 0
    {
        Route::PositiveGuardFalseFallthrough
    } else {
        if !active_prefix(snapshot) || predecessor_count != 1 || local_count != 1 {
            return None;
        }
        match snapshot.predecessor_dehumidification_control_type? {
            DehumidificationControlType::None
                if snapshot.predecessor_dehumidification_control_none_case_completed_skip
                    && snapshot.dehumidification_control_none_case_completed_skip =>
            {
                Route::DehumidificationControlNoneCaseCompletedSkip
            }
            DehumidificationControlType::ConstantSensibleHeatRatio
                if snapshot
                    .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
                    && snapshot
                        .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip =>
            {
                Route::DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip
            }
            DehumidificationControlType::Humidistat
                if snapshot.predecessor_dehumidification_control_humidistat_case_entered
                    && snapshot
                        .dehumidification_control_humidistat_moisture_demand_assignment_executed =>
            {
                Route::DehumidificationControlHumidistatMoistureDemandAssignmentExecuted
            }
            DehumidificationControlType::ConstantSupplyHumidityRatio
                if snapshot
                    .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
                    && snapshot
                        .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip =>
            {
                Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip
            }
            _ => return None,
        }
    };
    values_fit_route(snapshot, route).then_some(route)
}

pub(in crate::ideal_loads::calc) fn snapshots_match_bit_exact(
    mut left: Snapshot,
    mut right: Snapshot,
) -> bool {
    let values_match = option_bits_match(
        left.zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        right.zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
    ) && option_bits_match(
        left.assigned_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        right.assigned_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
    ) && option_bits_match(
        left.resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        right.resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
    );
    for snapshot in [&mut left, &mut right] {
        snapshot.zone_dehumidifying_setpoint_moisture_demand_kg_per_s = None;
        snapshot.assigned_zone_dehumidifying_setpoint_moisture_demand_kg_per_s = None;
        snapshot.resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s = None;
    }
    values_match && left == right
}

fn values_fit_route(snapshot: Snapshot, route: Route) -> bool {
    if route != Route::DehumidificationControlHumidistatMoistureDemandAssignmentExecuted {
        return !snapshot.zone_dehumidifying_setpoint_moisture_demand_read
            && snapshot
                .zone_dehumidifying_setpoint_moisture_demand_kg_per_s
                .is_none()
            && !snapshot.zone_dehumidifying_setpoint_moisture_demand_assigned
            && snapshot
                .assigned_zone_dehumidifying_setpoint_moisture_demand_kg_per_s
                .is_none()
            && snapshot
                .resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s
                .is_none();
    }
    snapshot.zone_dehumidifying_setpoint_moisture_demand_read
        && snapshot.zone_dehumidifying_setpoint_moisture_demand_assigned
        && snapshot
            .zone_dehumidifying_setpoint_moisture_demand_kg_per_s
            .is_some()
        && option_bits_match(
            snapshot.zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
            snapshot.assigned_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        )
        && option_bits_match(
            snapshot.assigned_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
            snapshot.resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        )
}

fn provenance_is_exact(snapshot: Snapshot) -> bool {
    snapshot.source == PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER
}

fn active_prefix(snapshot: Snapshot) -> bool {
    !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.predecessor_no_outdoor_air_fallback_entered
        && snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.positive_guard_false_fallthrough_skipped
        && snapshot.predecessor_dehumidification_control_type.is_some()
}

fn inactive_prefix(snapshot: Snapshot) -> bool {
    !snapshot.predecessor_cooling_body_entered
        && !snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.positive_guard_false_fallthrough_skipped
        && snapshot.predecessor_dehumidification_control_type.is_none()
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
