//! Exact CP365 snapshot and binary64 validation.

use ep_model::DehumidificationControlType;

use super::super::{
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentSnapshot as Snapshot,
};

pub(in crate::ideal_loads) fn cooling_constant_supply_humidity_ratio_assignment_snapshot_is_exact_direct_release(
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
    if snapshot.source
        != PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order
            != PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER
    {
        return None;
    }
    let predecessor_count = usize::from(
        snapshot.predecessor_dehumidification_control_none_case_completed_skip,
    ) + usize::from(
        snapshot
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
    ) + usize::from(
        snapshot.predecessor_dehumidification_control_humidistat_case_completed_skip,
    ) + usize::from(
        snapshot.predecessor_dehumidification_control_constant_supply_humidity_ratio_case_entered,
    );
    let local_count = usize::from(snapshot.dehumidification_control_none_case_completed_skip)
        + usize::from(
            snapshot.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        )
        + usize::from(snapshot.dehumidification_control_humidistat_case_completed_skip)
        + usize::from(
            snapshot.dehumidification_control_constant_supply_humidity_ratio_assignment_executed,
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
    } else if positive_guard_fallthrough(snapshot) && predecessor_count == 0 && local_count == 0 {
        Route::PositiveGuardFalseFallthrough
    } else {
        if !active_prefix(snapshot) || predecessor_count != 1 || local_count != 1 {
            return None;
        }
        active_route(snapshot)?
    };
    values_fit_route(snapshot, route).then_some(route)
}

pub(in crate::ideal_loads::calc) fn snapshots_match_bit_exact(
    mut left: Snapshot,
    mut right: Snapshot,
) -> bool {
    let values_match = [
        (
            left.minimum_cooling_supply_air_humidity_ratio,
            right.minimum_cooling_supply_air_humidity_ratio,
        ),
        (
            left.assigned_supply_humidity_ratio,
            right.assigned_supply_humidity_ratio,
        ),
        (
            left.resulting_supply_humidity_ratio,
            right.resulting_supply_humidity_ratio,
        ),
    ]
    .into_iter()
    .all(|(left, right)| option_bits_match(left, right));
    for snapshot in [&mut left, &mut right] {
        snapshot.minimum_cooling_supply_air_humidity_ratio = None;
        snapshot.assigned_supply_humidity_ratio = None;
        snapshot.resulting_supply_humidity_ratio = None;
    }
    values_match && left == right
}

pub(super) fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

fn active_route(snapshot: Snapshot) -> Option<Route> {
    match snapshot.predecessor_dehumidification_control_type? {
        DehumidificationControlType::None
            if snapshot.predecessor_dehumidification_control_none_case_completed_skip
                && snapshot.dehumidification_control_none_case_completed_skip =>
        {
            Some(Route::DehumidificationControlNoneCaseCompletedSkip)
        }
        DehumidificationControlType::ConstantSensibleHeatRatio
            if snapshot
                .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
                && snapshot
                    .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip =>
        {
            Some(Route::DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip)
        }
        DehumidificationControlType::Humidistat
            if snapshot.predecessor_dehumidification_control_humidistat_case_completed_skip
                && snapshot.dehumidification_control_humidistat_case_completed_skip =>
        {
            Some(Route::DehumidificationControlHumidistatCaseCompletedSkip)
        }
        DehumidificationControlType::ConstantSupplyHumidityRatio
            if snapshot
                .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_entered
                && snapshot
                    .dehumidification_control_constant_supply_humidity_ratio_assignment_executed =>
        {
            Some(Route::DehumidificationControlConstantSupplyHumidityRatioAssigned)
        }
        _ => None,
    }
}

fn values_fit_route(snapshot: Snapshot, route: Route) -> bool {
    let active = route == Route::DehumidificationControlConstantSupplyHumidityRatioAssigned;
    if snapshot.minimum_cooling_supply_air_humidity_ratio_read != active
        || snapshot.supply_humidity_ratio_assigned != active
    {
        return false;
    }
    if !active {
        return snapshot.minimum_cooling_supply_air_humidity_ratio.is_none()
            && snapshot.assigned_supply_humidity_ratio.is_none()
            && snapshot.resulting_supply_humidity_ratio.is_none();
    }
    let (Some(minimum), Some(assigned), Some(resulting)) = (
        snapshot.minimum_cooling_supply_air_humidity_ratio,
        snapshot.assigned_supply_humidity_ratio,
        snapshot.resulting_supply_humidity_ratio,
    ) else {
        return false;
    };
    minimum.to_bits() == assigned.to_bits() && assigned.to_bits() == resulting.to_bits()
}

fn positive_guard_fallthrough(snapshot: Snapshot) -> bool {
    !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && snapshot.positive_guard_false_fallthrough_skipped
        && snapshot.predecessor_dehumidification_control_type.is_none()
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
