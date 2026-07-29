//! Pure CP364-to-CP365 constant-supply-humidity-ratio assignment transition.

use super::{
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntrySnapshot as Predecessor;

mod predecessor;
pub(in crate::ideal_loads::calc) use predecessor::{
    predecessor_route, predecessor_snapshots_match_exact,
};

struct PreparedValue {
    minimum: Option<f64>,
}

pub(in crate::ideal_loads::calc) fn advance_cooling_constant_supply_humidity_ratio_assignment_state(
    state: &mut State,
    predecessor: Predecessor,
    minimum_cooling_supply_air_humidity_ratio: Option<f64>,
) -> Option<Snapshot> {
    if state.system != predecessor.system {
        return None;
    }
    let route = predecessor_route(predecessor)?;
    let value = prepare_value(route, minimum_cooling_supply_air_humidity_ratio)?;
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
        Route::DehumidificationControlConstantSupplyHumidityRatioAssigned => {
            state.dehumidification_control_constant_supply_humidity_ratio_assignment_count += 1;
            state.source_site_execution_count +=
                PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER
                    .len();
            state.minimum_cooling_supply_air_humidity_ratio_read_count += 1;
            state.supply_humidity_ratio_assignment_count += 1;
            state
                .witnessed_dehumidification_control_constant_supply_humidity_ratio_assignment_count +=
                1;
        }
    }

    let active = route == Route::DehumidificationControlConstantSupplyHumidityRatioAssigned;
    let snapshot = Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER,
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
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_entered:
            predecessor.dehumidification_control_constant_supply_humidity_ratio_case_entered,
        dehumidification_control_none_case_completed_skip: route
            == Route::DehumidificationControlNoneCaseCompletedSkip,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: route
            == Route::DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip,
        dehumidification_control_humidistat_case_completed_skip: route
            == Route::DehumidificationControlHumidistatCaseCompletedSkip,
        dehumidification_control_constant_supply_humidity_ratio_assignment_executed: active,
        minimum_cooling_supply_air_humidity_ratio_read: active,
        minimum_cooling_supply_air_humidity_ratio: value.minimum,
        supply_humidity_ratio_assigned: active,
        assigned_supply_humidity_ratio: value.minimum,
        resulting_supply_humidity_ratio: value.minimum,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    Some(snapshot)
}

pub(in crate::ideal_loads::calc) fn next_transition_fits(state: &State, route: Route) -> bool {
    if state.transition_count.checked_add(1).is_none() {
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
        Route::DehumidificationControlConstantSupplyHumidityRatioAssigned => {
            checked_pair(
                state.dehumidification_control_constant_supply_humidity_ratio_assignment_count,
                state
                    .witnessed_dehumidification_control_constant_supply_humidity_ratio_assignment_count,
            ) && state
                .source_site_execution_count
                .checked_add(
                    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER
                        .len(),
                )
                .is_some()
                && state
                    .minimum_cooling_supply_air_humidity_ratio_read_count
                    .checked_add(1)
                    .is_some()
                && state
                    .supply_humidity_ratio_assignment_count
                    .checked_add(1)
                    .is_some()
        }
    }
}

fn prepare_value(route: Route, minimum: Option<f64>) -> Option<PreparedValue> {
    if route == Route::DehumidificationControlConstantSupplyHumidityRatioAssigned {
        Some(PreparedValue {
            minimum: Some(minimum?),
        })
    } else {
        minimum.is_none().then_some(PreparedValue { minimum: None })
    }
}

fn checked_pair(left: usize, right: usize) -> bool {
    left.checked_add(1).is_some() && right.checked_add(1).is_some()
}
