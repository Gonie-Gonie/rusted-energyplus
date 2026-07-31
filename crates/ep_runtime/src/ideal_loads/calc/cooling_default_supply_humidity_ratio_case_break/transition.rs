//! Pure CP367-to-CP368 default supply-humidity-ratio case-break transition.

use ep_model::DehumidificationControlType;

use super::{
    PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE,
    PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE_ORDER,
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioCaseBreakRetainedRoute as Route,
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioCaseBreakRuntimeState as State,
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioCaseBreakSnapshot as Snapshot,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentSnapshot as Predecessor,
};

pub(in crate::ideal_loads::calc) fn advance_cooling_default_supply_humidity_ratio_case_break_state(
    state: &mut State,
    predecessor: Predecessor,
) -> Option<Snapshot> {
    if state.system != predecessor.system {
        return None;
    }
    let route = predecessor_route(predecessor)?;
    if !next_transition_fits(state, route) {
        return None;
    }

    state.transition_count += 1;
    match route {
        Route::UnitOff => state.unit_off_skip_count += 1,
        Route::NonCooling => state.non_cooling_skip_count += 1,
        Route::PositiveGuardFalseFallthrough => {
            state.positive_guard_false_fallthrough_skip_count += 1;
            state.witnessed_positive_guard_false_fallthrough_skip_count += 1;
        }
        Route::DehumidificationControlNoneCaseCompletedSkip => {
            state.dehumidification_control_none_case_completed_skip_count += 1;
            state.witnessed_dehumidification_control_none_case_completed_skip_count += 1;
        }
        Route::DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip => {
            state
                .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count +=
                1;
            state
                .witnessed_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count += 1;
        }
        Route::DehumidificationControlHumidistatCaseCompletedSkip => {
            state.dehumidification_control_humidistat_case_completed_skip_count += 1;
            state.witnessed_dehumidification_control_humidistat_case_completed_skip_count += 1;
        }
        Route::DehumidificationControlConstantSupplyHumidityRatioCaseCompletedSkip => {
            state
                .dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count +=
                1;
            state
                .witnessed_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count += 1;
        }
    }

    let snapshot = Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE_ORDER,
        system: state.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.predecessor_cooling_body_entered,
        predecessor_no_outdoor_air_fallback_entered: predecessor
            .predecessor_no_outdoor_air_fallback_entered,
        predecessor_positive_supply_mass_flow_body_entered: predecessor
            .predecessor_positive_supply_mass_flow_body_entered,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor
            .positive_guard_false_fallthrough_skipped,
        predecessor_dehumidification_control_type: predecessor
            .predecessor_dehumidification_control_type,
        predecessor_dehumidification_control_none_case_completed_skip: predecessor
            .dehumidification_control_none_case_completed_skip,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
            predecessor
                .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        predecessor_dehumidification_control_humidistat_case_completed_skip: predecessor
            .dehumidification_control_humidistat_case_completed_skip,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip:
            predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
        predecessor_dehumidification_control_default_supply_humidity_ratio_mixed_air_assignment_executed:
            predecessor
                .dehumidification_control_default_supply_humidity_ratio_mixed_air_assignment_executed,
        dehumidification_control_none_case_completed_skip: route
            == Route::DehumidificationControlNoneCaseCompletedSkip,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: route
            == Route::DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip,
        dehumidification_control_humidistat_case_completed_skip: route
            == Route::DehumidificationControlHumidistatCaseCompletedSkip,
        dehumidification_control_constant_supply_humidity_ratio_case_completed_skip: route
            == Route::DehumidificationControlConstantSupplyHumidityRatioCaseCompletedSkip,
        dehumidification_control_default_supply_humidity_ratio_case_exited_via_break: false,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    Some(snapshot)
}

pub(in crate::ideal_loads::calc) fn predecessor_route(predecessor: Predecessor) -> Option<Route> {
    if predecessor.source
        != PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source_order
            != PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER
    {
        return None;
    }
    if predecessor
        .dehumidification_control_default_supply_humidity_ratio_mixed_air_assignment_executed
    {
        return None;
    }
    let predecessor_count = usize::from(
        predecessor.predecessor_dehumidification_control_none_case_completed_skip,
    ) + usize::from(
        predecessor
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
    ) + usize::from(
        predecessor.predecessor_dehumidification_control_humidistat_case_completed_skip,
    ) + usize::from(
        predecessor
            .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_exited_via_break,
    );
    let local_count = usize::from(predecessor.dehumidification_control_none_case_completed_skip)
        + usize::from(
            predecessor.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        )
        + usize::from(predecessor.dehumidification_control_humidistat_case_completed_skip)
        + usize::from(
            predecessor.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
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
        match predecessor.predecessor_dehumidification_control_type? {
            DehumidificationControlType::None
                if predecessor.predecessor_dehumidification_control_none_case_completed_skip
                    && predecessor.dehumidification_control_none_case_completed_skip =>
            {
                Route::DehumidificationControlNoneCaseCompletedSkip
            }
            DehumidificationControlType::ConstantSensibleHeatRatio
                if predecessor
                    .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
                    && predecessor
                        .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip =>
            {
                Route::DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip
            }
            DehumidificationControlType::Humidistat
                if predecessor.predecessor_dehumidification_control_humidistat_case_completed_skip
                    && predecessor.dehumidification_control_humidistat_case_completed_skip =>
            {
                Route::DehumidificationControlHumidistatCaseCompletedSkip
            }
            DehumidificationControlType::ConstantSupplyHumidityRatio
                if predecessor
                    .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_exited_via_break
                    && predecessor
                        .dehumidification_control_constant_supply_humidity_ratio_case_completed_skip =>
            {
                Route::DehumidificationControlConstantSupplyHumidityRatioCaseCompletedSkip
            }
            _ => return None,
        }
    };
    Some(route)
}

pub(in crate::ideal_loads::calc) fn predecessor_snapshots_match_bit_exact(
    left: Predecessor,
    right: Predecessor,
) -> bool {
    crate::ideal_loads::calc::cooling_default_supply_humidity_ratio_mixed_air_assignment::
        cooling_default_supply_humidity_ratio_mixed_air_assignment_snapshots_match_exact(left, right)
}

pub(in crate::ideal_loads::calc) fn next_transition_fits(state: &State, route: Route) -> bool {
    if state.transition_count.checked_add(1).is_none()
        || state.dehumidification_control_default_supply_humidity_ratio_case_break_count != 0
        || state.source_site_execution_count != 0
    {
        return false;
    }
    match route {
        Route::UnitOff => state.unit_off_skip_count.checked_add(1).is_some(),
        Route::NonCooling => state.non_cooling_skip_count.checked_add(1).is_some(),
        Route::PositiveGuardFalseFallthrough => checked_pair(
            state.positive_guard_false_fallthrough_skip_count,
            state.witnessed_positive_guard_false_fallthrough_skip_count,
        ),
        Route::DehumidificationControlNoneCaseCompletedSkip => checked_pair(
            state.dehumidification_control_none_case_completed_skip_count,
            state.witnessed_dehumidification_control_none_case_completed_skip_count,
        ),
        Route::DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip => checked_pair(
            state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
            state
                .witnessed_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        ),
        Route::DehumidificationControlHumidistatCaseCompletedSkip => checked_pair(
            state.dehumidification_control_humidistat_case_completed_skip_count,
            state.witnessed_dehumidification_control_humidistat_case_completed_skip_count,
        ),
        Route::DehumidificationControlConstantSupplyHumidityRatioCaseCompletedSkip => checked_pair(
            state
                .dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count,
            state
                .witnessed_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count,
        ),
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

fn checked_pair(left: usize, right: usize) -> bool {
    left.checked_add(1).is_some() && right.checked_add(1).is_some()
}
