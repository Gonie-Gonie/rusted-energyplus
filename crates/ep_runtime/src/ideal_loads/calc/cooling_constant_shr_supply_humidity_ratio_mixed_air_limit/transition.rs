//! Pure CP355-to-CP356 constant-SHR supply-humidity-ratio mixed-air transition.

use ep_model::DehumidificationControlType;

use super::{
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitRetainedRoute as Route,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitRuntimeState,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitSnapshot,
};
use crate::ideal_loads::calc::cooling_positive_supply_temperature_mixed_air_limit::source_shaped_two_argument_minimum;
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MINIMUM_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MINIMUM_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MINIMUM_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitSnapshot as Predecessor,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitActiveOperands
{
    /// Same-call retained/witnessed CP329 `PurchAir.MixedAirHumRat`.
    pub mixed_air_humidity_ratio: f64,
}

pub(in crate::ideal_loads::calc) fn advance_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_state(
    state: &mut PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitRuntimeState,
    predecessor: Predecessor,
    active_operands: Option<
        PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitActiveOperands,
    >,
) -> Option<PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitSnapshot> {
    if state.system != predecessor.system {
        return None;
    }
    let route = predecessor_route(predecessor)?;
    if !next_transition_fits(state, route) {
        return None;
    }
    let prepared = prepare_values(route, predecessor, active_operands)?;

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
        Route::DehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioMixedAirLimitExecuted => {
            state
                .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_mixed_air_limit_count +=
                1;
            state.source_site_execution_count +=
                PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE_ORDER
                    .len();
            state.supply_humidity_ratio_for_mixed_air_limit_minimum_read_count += 1;
            state.mixed_air_humidity_ratio_for_minimum_read_count += 1;
            state.source_shaped_two_argument_minimum_evaluation_count += 1;
            state.supply_humidity_ratio_assignment_write_count += 1;
            state
                .witnessed_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_mixed_air_limit_count +=
                1;
        }
        Route::DehumidificationControlHumidistatCaseSelectedSkip => {
            state.dehumidification_control_humidistat_case_selected_skip_count += 1;
            state.witnessed_dehumidification_control_humidistat_case_selected_skip_count += 1;
        }
        Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip => {
            state
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count +=
                1;
            state
                .witnessed_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count +=
                1;
        }
    }

    let none_skip = route == Route::DehumidificationControlNoneCaseCompletedSkip;
    let assignment =
        route == Route::DehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioMixedAirLimitExecuted;
    let humidistat_skip = route == Route::DehumidificationControlHumidistatCaseSelectedSkip;
    let constant_supply_skip =
        route == Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip;
    let snapshot = PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitSnapshot {
        source:
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE_ORDER,
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
        predecessor_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_minimum_limit_executed:
            predecessor
                .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_minimum_limit_executed,
        predecessor_dehumidification_control_humidistat_case_selected_skip: predecessor
            .dehumidification_control_humidistat_case_selected_skip,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
        dehumidification_control_none_case_completed_skip: none_skip,
        dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_mixed_air_limit_executed:
            assignment,
        dehumidification_control_humidistat_case_selected_skip: humidistat_skip,
        dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            constant_supply_skip,
        supply_humidity_ratio_for_mixed_air_limit_minimum_read: assignment,
        supply_humidity_ratio_before_mixed_air_limit: prepared
            .supply_humidity_ratio_before_mixed_air_limit,
        mixed_air_humidity_ratio_for_minimum_read: assignment,
        mixed_air_humidity_ratio: prepared.mixed_air_humidity_ratio,
        source_shaped_two_argument_minimum_evaluated: assignment,
        minimum_supply_humidity_ratio: prepared.minimum_supply_humidity_ratio,
        supply_humidity_ratio_assignment_performed: assignment,
        assigned_supply_humidity_ratio: prepared.minimum_supply_humidity_ratio,
        resulting_supply_humidity_ratio: prepared.minimum_supply_humidity_ratio,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    Some(snapshot)
}

pub(in crate::ideal_loads::calc) fn predecessor_route(predecessor: Predecessor) -> Option<Route> {
    if predecessor.source
        != PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MINIMUM_LIMIT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MINIMUM_LIMIT_FIRST_EXCLUDED_SOURCE
        || predecessor.source_order
            != PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MINIMUM_LIMIT_SOURCE_ORDER
    {
        return None;
    }
    let local_count = usize::from(predecessor.dehumidification_control_none_case_completed_skip)
        + usize::from(
            predecessor
                .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_minimum_limit_executed,
        )
        + usize::from(predecessor.dehumidification_control_humidistat_case_selected_skip)
        + usize::from(
            predecessor.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
        );
    if predecessor.unit_off_skipped
        && !predecessor.non_cooling_skipped
        && !predecessor.unit_body_entered
        && inactive_prefix(predecessor)
        && local_count == 0
    {
        return Some(Route::UnitOff);
    }
    if !predecessor.unit_off_skipped
        && predecessor.non_cooling_skipped
        && predecessor.unit_body_entered
        && inactive_prefix(predecessor)
        && local_count == 0
    {
        return Some(Route::NonCooling);
    }
    if !predecessor.unit_off_skipped
        && !predecessor.non_cooling_skipped
        && predecessor.unit_body_entered
        && predecessor.predecessor_cooling_body_entered
        && predecessor.predecessor_no_outdoor_air_fallback_entered
        && !predecessor.predecessor_positive_supply_mass_flow_body_entered
        && predecessor.positive_guard_false_fallthrough_skipped
        && predecessor
            .predecessor_dehumidification_control_type
            .is_none()
        && local_count == 0
    {
        return Some(Route::PositiveGuardFalseFallthrough);
    }
    if !active_prefix(predecessor) || local_count != 1 {
        return None;
    }
    match predecessor.predecessor_dehumidification_control_type? {
        DehumidificationControlType::None
            if predecessor.dehumidification_control_none_case_completed_skip =>
        {
            Some(Route::DehumidificationControlNoneCaseCompletedSkip)
        }
        DehumidificationControlType::ConstantSensibleHeatRatio
            if predecessor
                .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_minimum_limit_executed
                && predecessor.supply_humidity_ratio_assignment_performed
                && predecessor.resulting_supply_humidity_ratio.is_some() =>
        {
            Some(
                Route::DehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioMixedAirLimitExecuted,
            )
        }
        DehumidificationControlType::Humidistat
            if predecessor.dehumidification_control_humidistat_case_selected_skip =>
        {
            Some(Route::DehumidificationControlHumidistatCaseSelectedSkip)
        }
        DehumidificationControlType::ConstantSupplyHumidityRatio
            if predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip =>
        {
            Some(Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip)
        }
        _ => None,
    }
}

pub(in crate::ideal_loads::calc) fn predecessor_snapshots_match_bit_exact(
    mut left: Predecessor,
    mut right: Predecessor,
) -> bool {
    let values_match = [
        option_bits_match(
            left.supply_humidity_ratio_before_minimum_limit,
            right.supply_humidity_ratio_before_minimum_limit,
        ),
        option_bits_match(
            left.minimum_cooling_supply_air_humidity_ratio,
            right.minimum_cooling_supply_air_humidity_ratio,
        ),
        option_bits_match(
            left.maximum_supply_humidity_ratio,
            right.maximum_supply_humidity_ratio,
        ),
        option_bits_match(
            left.assigned_supply_humidity_ratio,
            right.assigned_supply_humidity_ratio,
        ),
        option_bits_match(
            left.resulting_supply_humidity_ratio,
            right.resulting_supply_humidity_ratio,
        ),
    ]
    .into_iter()
    .all(|matches| matches);
    for snapshot in [&mut left, &mut right] {
        snapshot.supply_humidity_ratio_before_minimum_limit = None;
        snapshot.minimum_cooling_supply_air_humidity_ratio = None;
        snapshot.maximum_supply_humidity_ratio = None;
        snapshot.assigned_supply_humidity_ratio = None;
        snapshot.resulting_supply_humidity_ratio = None;
    }
    values_match && left == right
}

pub(in crate::ideal_loads::calc) fn next_transition_fits(
    state: &PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitRuntimeState,
    route: Route,
) -> bool {
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
        Route::DehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioMixedAirLimitExecuted => {
            let counters = [
                state
                    .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_mixed_air_limit_count,
                state.supply_humidity_ratio_for_mixed_air_limit_minimum_read_count,
                state.mixed_air_humidity_ratio_for_minimum_read_count,
                state.source_shaped_two_argument_minimum_evaluation_count,
                state.supply_humidity_ratio_assignment_write_count,
                state
                    .witnessed_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_mixed_air_limit_count,
            ];
            counters
                .into_iter()
                .all(|counter| counter.checked_add(1).is_some())
                && state
                    .source_site_execution_count
                    .checked_add(
                        PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE_ORDER
                            .len(),
                    )
                    .is_some()
        }
        Route::DehumidificationControlHumidistatCaseSelectedSkip => checked_pair(
            state.dehumidification_control_humidistat_case_selected_skip_count,
            state.witnessed_dehumidification_control_humidistat_case_selected_skip_count,
        ),
        Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip => checked_pair(
            state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
            state
                .witnessed_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        ),
    }
}

pub(in crate::ideal_loads::calc) fn input_fits_route(
    route: Route,
    predecessor: Predecessor,
    active_operands: Option<
        PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitActiveOperands,
    >,
) -> bool {
    prepare_values(route, predecessor, active_operands).is_some()
}

fn prepare_values(
    route: Route,
    predecessor: Predecessor,
    active_operands: Option<
        PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitActiveOperands,
    >,
) -> Option<PreparedValues> {
    if route
        != Route::DehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioMixedAirLimitExecuted
    {
        return active_operands.is_none().then_some(PreparedValues::empty());
    }
    let left = predecessor.resulting_supply_humidity_ratio?;
    let right = active_operands?.mixed_air_humidity_ratio;
    let minimum = source_shaped_two_argument_minimum(left, right);
    Some(PreparedValues {
        supply_humidity_ratio_before_mixed_air_limit: Some(left),
        mixed_air_humidity_ratio: Some(right),
        minimum_supply_humidity_ratio: Some(minimum),
    })
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

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

#[derive(Clone, Copy)]
struct PreparedValues {
    supply_humidity_ratio_before_mixed_air_limit: Option<f64>,
    mixed_air_humidity_ratio: Option<f64>,
    minimum_supply_humidity_ratio: Option<f64>,
}

impl PreparedValues {
    const fn empty() -> Self {
        Self {
            supply_humidity_ratio_before_mixed_air_limit: None,
            mixed_air_humidity_ratio: None,
            minimum_supply_humidity_ratio: None,
        }
    }
}
