//! Pure CP361-to-CP362 Humidistat mixed-air-limit transition.

use super::{
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitRetainedRoute as Route,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitRuntimeState as State,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitSnapshot as Snapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitSnapshot as Predecessor;
use crate::ideal_loads::calc::cooling_positive_supply_temperature_mixed_air_limit::source_shaped_two_argument_minimum;

mod predecessor;
pub(in crate::ideal_loads::calc) use predecessor::{
    predecessor_route, predecessor_snapshots_match_bit_exact,
};

/// Same-call active CP362 value owned by CP329.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitActiveOperands
{
    /// Selected CP329 `PurchAir.MixedAirHumRat`.
    pub mixed_air_humidity_ratio: f64,
}

pub(in crate::ideal_loads::calc) fn advance_cooling_humidistat_supply_humidity_ratio_mixed_air_limit_state(
    state: &mut State,
    predecessor: Predecessor,
    operands: Option<
        PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitActiveOperands,
    >,
) -> Option<Snapshot> {
    if state.system != predecessor.system {
        return None;
    }
    let route = predecessor_route(predecessor)?;
    let (mixed, local, minimum) = prepare_values(route, predecessor, operands)?;
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
        Route::DehumidificationControlHumidistatSupplyHumidityRatioMixedAirLimitExecuted => {
            state
                .dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_count +=
                1;
            state.source_site_execution_count +=
                PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE_ORDER
                    .len();
            state.mixed_air_humidity_ratio_for_minimum_read_count += 1;
            state
                .supply_humidity_ratio_for_dehumidification_for_mixed_air_limit_minimum_read_count +=
                1;
            state.source_shaped_two_argument_minimum_evaluation_count += 1;
            state.supply_humidity_ratio_assignment_count += 1;
            state
                .witnessed_dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_count += 1;
        }
        Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip => {
            state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count +=
                1;
            state
                .witnessed_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count += 1;
        }
    }

    let active =
        route == Route::DehumidificationControlHumidistatSupplyHumidityRatioMixedAirLimitExecuted;
    let snapshot = Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE_ORDER,
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
            predecessor.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        predecessor_dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_executed:
            predecessor
                .dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_executed,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            predecessor.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
        predecessor_resulting_supply_humidity_ratio_for_dehumidification: predecessor
            .resulting_supply_humidity_ratio_for_dehumidification,
        dehumidification_control_none_case_completed_skip: route
            == Route::DehumidificationControlNoneCaseCompletedSkip,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: route
            == Route::DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip,
        dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_executed: active,
        dehumidification_control_constant_supply_humidity_ratio_case_selected_skip: route
            == Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip,
        mixed_air_humidity_ratio_for_minimum_read: active,
        mixed_air_humidity_ratio: mixed,
        supply_humidity_ratio_for_dehumidification_for_mixed_air_limit_minimum_read: active,
        supply_humidity_ratio_for_dehumidification_before_mixed_air_limit: local,
        source_shaped_two_argument_minimum_evaluated: active,
        minimum_supply_humidity_ratio: minimum,
        supply_humidity_ratio_assignment_performed: active,
        assigned_supply_humidity_ratio: minimum,
        resulting_supply_humidity_ratio: minimum,
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
        Route::DehumidificationControlHumidistatSupplyHumidityRatioMixedAirLimitExecuted => {
            checked_pair(
                state.dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_count,
                state.witnessed_dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_count,
            ) && state.source_site_execution_count.checked_add(
                PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE_ORDER
                    .len(),
            ).is_some()
                && all_site_counters_fit(state)
        }
        Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip => checked_pair(
            state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
            state
                .witnessed_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        ),
    }
}

fn prepare_values(
    route: Route,
    predecessor: Predecessor,
    operands: Option<
        PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitActiveOperands,
    >,
) -> Option<(Option<f64>, Option<f64>, Option<f64>)> {
    if route
        != Route::DehumidificationControlHumidistatSupplyHumidityRatioMixedAirLimitExecuted
    {
        return operands.is_none().then_some((None, None, None));
    }
    let mixed = operands?.mixed_air_humidity_ratio;
    let local = predecessor.resulting_supply_humidity_ratio_for_dehumidification?;
    let minimum = source_shaped_two_argument_minimum(mixed, local);
    Some((Some(mixed), Some(local), Some(minimum)))
}

fn all_site_counters_fit(state: &State) -> bool {
    [
        state.mixed_air_humidity_ratio_for_minimum_read_count,
        state.supply_humidity_ratio_for_dehumidification_for_mixed_air_limit_minimum_read_count,
        state.source_shaped_two_argument_minimum_evaluation_count,
        state.supply_humidity_ratio_assignment_count,
    ]
    .into_iter()
    .all(|count| count.checked_add(1).is_some())
}

fn checked_pair(left: usize, right: usize) -> bool {
    left.checked_add(1).is_some() && right.checked_add(1).is_some()
}
