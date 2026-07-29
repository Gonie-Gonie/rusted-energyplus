//! CP361 source provenance and seven-route structural validation.

use ep_model::DehumidificationControlType;

use super::{Route, Snapshot};
use crate::ideal_loads::calc::cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit::{
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_MINIMUM_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_MINIMUM_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_MINIMUM_LIMIT_SOURCE_ORDER,
};

pub(super) fn structural_route(snapshot: Snapshot) -> Option<Route> {
    if snapshot.source
        != PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_MINIMUM_LIMIT_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_MINIMUM_LIMIT_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order
            != PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_MINIMUM_LIMIT_SOURCE_ORDER
    {
        return None;
    }
    let predecessor_count = usize::from(
        snapshot.predecessor_dehumidification_control_none_case_completed_skip,
    ) + usize::from(
        snapshot
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
    ) + usize::from(
        snapshot
            .predecessor_dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_executed,
    ) + usize::from(
        snapshot
            .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
    );
    let local_count = usize::from(snapshot.dehumidification_control_none_case_completed_skip)
        + usize::from(
            snapshot.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        )
        + usize::from(
            snapshot
                .dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_executed,
        )
        + usize::from(
            snapshot.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
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
            if snapshot
                .predecessor_dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_executed
                && snapshot
                    .dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_executed =>
        {
            Some(
                Route::DehumidificationControlHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitExecuted,
            )
        }
        DehumidificationControlType::ConstantSupplyHumidityRatio
            if snapshot
                .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
                && snapshot
                    .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip =>
        {
            Some(Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip)
        }
        _ => None,
    }
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
