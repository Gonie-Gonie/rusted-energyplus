//! CP364 predecessor provenance and exact seven-route validation.

use ep_model::DehumidificationControlType;

use super::{Predecessor, Route};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_SOURCE_ORDER,
};

pub(in crate::ideal_loads::calc) fn predecessor_route(predecessor: Predecessor) -> Option<Route> {
    if predecessor.source
        != PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_FIRST_EXCLUDED_SOURCE
        || predecessor.source_order
            != PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_SOURCE_ORDER
    {
        return None;
    }
    let predecessor_count = usize::from(
        predecessor.predecessor_dehumidification_control_none_case_completed_skip,
    ) + usize::from(
        predecessor
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
    ) + usize::from(
        predecessor.predecessor_dehumidification_control_humidistat_case_exited_via_break,
    ) + usize::from(
        predecessor
            .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
    );
    let local_count = usize::from(predecessor.dehumidification_control_none_case_completed_skip)
        + usize::from(
            predecessor.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        )
        + usize::from(predecessor.dehumidification_control_humidistat_case_completed_skip)
        + usize::from(
            predecessor.dehumidification_control_constant_supply_humidity_ratio_case_entered,
        );

    if predecessor.unit_off_skipped
        && !predecessor.non_cooling_skipped
        && !predecessor.unit_body_entered
        && inactive_prefix(predecessor)
        && predecessor_count == 0
        && local_count == 0
    {
        return Some(Route::UnitOff);
    }
    if !predecessor.unit_off_skipped
        && predecessor.non_cooling_skipped
        && predecessor.unit_body_entered
        && inactive_prefix(predecessor)
        && predecessor_count == 0
        && local_count == 0
    {
        return Some(Route::NonCooling);
    }
    if positive_guard_fallthrough(predecessor) && predecessor_count == 0 && local_count == 0 {
        return Some(Route::PositiveGuardFalseFallthrough);
    }
    if !active_prefix(predecessor) || predecessor_count != 1 || local_count != 1 {
        return None;
    }
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
            if predecessor.predecessor_dehumidification_control_humidistat_case_exited_via_break
                && predecessor.dehumidification_control_humidistat_case_completed_skip =>
        {
            Some(Route::DehumidificationControlHumidistatCaseCompletedSkip)
        }
        DehumidificationControlType::ConstantSupplyHumidityRatio
            if predecessor
                .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
                && predecessor
                    .dehumidification_control_constant_supply_humidity_ratio_case_entered =>
        {
            Some(Route::DehumidificationControlConstantSupplyHumidityRatioAssigned)
        }
        _ => None,
    }
}

pub(in crate::ideal_loads::calc) fn predecessor_snapshots_match_exact(
    left: Predecessor,
    right: Predecessor,
) -> bool {
    left == right
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
