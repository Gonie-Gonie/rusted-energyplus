//! Exact numeric-free CP366 snapshot validation.

use ep_model::DehumidificationControlType;

use super::super::{
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE_ORDER,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakRetainedRoute as Route,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakSnapshot as Snapshot,
};

pub(in crate::ideal_loads) fn cooling_constant_supply_humidity_ratio_case_break_snapshot_is_exact_direct_release(
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
        != PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order
            != PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE_ORDER
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
        snapshot
            .predecessor_dehumidification_control_constant_supply_humidity_ratio_assignment_executed,
    );
    let local_count = usize::from(snapshot.dehumidification_control_none_case_completed_skip)
        + usize::from(
            snapshot.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        )
        + usize::from(snapshot.dehumidification_control_humidistat_case_completed_skip)
        + usize::from(
            snapshot
                .dehumidification_control_constant_supply_humidity_ratio_case_exited_via_break,
        );
    if snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && !snapshot.unit_body_entered
        && inactive_prefix(snapshot)
        && predecessor_count == 0
        && local_count == 0
    {
        return Some(Route::UnitOff);
    }
    if !snapshot.unit_off_skipped
        && snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && inactive_prefix(snapshot)
        && predecessor_count == 0
        && local_count == 0
    {
        return Some(Route::NonCooling);
    }
    if positive_guard_fallthrough(snapshot) && predecessor_count == 0 && local_count == 0 {
        return Some(Route::PositiveGuardFalseFallthrough);
    }
    if !active_prefix(snapshot) || predecessor_count != 1 || local_count != 1 {
        return None;
    }
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
                .predecessor_dehumidification_control_constant_supply_humidity_ratio_assignment_executed
                && snapshot
                    .dehumidification_control_constant_supply_humidity_ratio_case_exited_via_break =>
        {
            Some(Route::DehumidificationControlConstantSupplyHumidityRatioCaseBreak)
        }
        _ => None,
    }
}

pub(in crate::ideal_loads::calc) fn snapshots_match_exact(
    left: Snapshot,
    right: Snapshot,
) -> bool {
    left == right
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
