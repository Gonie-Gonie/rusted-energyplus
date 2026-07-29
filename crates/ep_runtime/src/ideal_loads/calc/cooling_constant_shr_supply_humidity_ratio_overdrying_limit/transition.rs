//! Pure CP353-to-CP354 constant-SHR supply-humidity-ratio limit transition.

use ep_model::DehumidificationControlType;

use super::{
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_OVERDRYING_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_OVERDRYING_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_OVERDRYING_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitRetainedRoute as Route,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitRuntimeState,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitSnapshot,
};
use crate::ideal_loads::calc::cooling_positive_supply_temperature_mixed_air_limit::source_shaped_two_argument_minimum;
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot,
};
use crate::psychrometrics::energyplus_psy_w_fn_tdb_h;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitActiveOperands
{
    /// Same-call CP345-owned current `PurchAir.SupplyHumRat`.
    pub supply_humidity_ratio_before_overdrying_limit: f64,
    /// Same-call private CP353-owned `PurchAir.SupplyTemp`.
    pub supply_temperature_c: f64,
    /// Same-call private CP353-owned local `SupplyEnthalpy`.
    pub supply_enthalpy_j_per_kg: f64,
}

pub(in crate::ideal_loads::calc) fn advance_cooling_constant_shr_supply_humidity_ratio_overdrying_limit_state(
    state: &mut PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitRuntimeState,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot,
    active_operands: Option<
        PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitActiveOperands,
    >,
) -> Option<PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitSnapshot> {
    if state.system != predecessor.system {
        return None;
    }
    let route = predecessor_route(predecessor)?;
    if !next_transition_fits(state, route) {
        return None;
    }
    let prepared = prepare_values(route, active_operands)?;

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
        Route::DehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioOverdryingLimitExecuted => {
            state
                .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_overdrying_limit_count +=
                1;
            state.source_site_execution_count +=
                PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_OVERDRYING_LIMIT_SOURCE_ORDER
                    .len();
            state.supply_humidity_ratio_for_overdrying_limit_minimum_read_count += 1;
            state.supply_temperature_for_humidity_ratio_inversion_read_count += 1;
            state.supply_enthalpy_for_humidity_ratio_inversion_read_count += 1;
            state.psychrometric_supply_humidity_ratio_evaluation_count += 1;
            state.source_shaped_two_argument_minimum_evaluation_count += 1;
            state.supply_humidity_ratio_assignment_write_count += 1;
            state
                .witnessed_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_overdrying_limit_count +=
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
        route == Route::DehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioOverdryingLimitExecuted;
    let humidistat_skip = route == Route::DehumidificationControlHumidistatCaseSelectedSkip;
    let constant_supply_skip =
        route == Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip;
    let snapshot = PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitSnapshot {
        source:
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_OVERDRYING_LIMIT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_OVERDRYING_LIMIT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_OVERDRYING_LIMIT_SOURCE_ORDER,
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
        predecessor_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_executed:
            predecessor
                .dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_executed,
        predecessor_dehumidification_control_humidistat_case_selected_skip: predecessor
            .dehumidification_control_humidistat_case_selected_skip,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
        dehumidification_control_none_case_completed_skip: none_skip,
        dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_overdrying_limit_executed:
            assignment,
        dehumidification_control_humidistat_case_selected_skip: humidistat_skip,
        dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            constant_supply_skip,
        supply_humidity_ratio_for_overdrying_limit_minimum_read: assignment,
        supply_humidity_ratio_before_overdrying_limit: prepared
            .supply_humidity_ratio_before_overdrying_limit,
        supply_temperature_for_humidity_ratio_inversion_read: assignment,
        supply_temperature_c: prepared.supply_temperature_c,
        supply_enthalpy_for_humidity_ratio_inversion_read: assignment,
        supply_enthalpy_j_per_kg: prepared.supply_enthalpy_j_per_kg,
        psychrometric_supply_humidity_ratio_evaluated: assignment,
        psychrometric_supply_humidity_ratio: prepared.psychrometric_supply_humidity_ratio,
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

pub(in crate::ideal_loads::calc) fn predecessor_route(
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot,
) -> Option<Route> {
    if predecessor.source
        != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_FIRST_EXCLUDED_SOURCE
        || predecessor.source_order
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE_ORDER
    {
        return None;
    }
    let local_count = usize::from(predecessor.dehumidification_control_none_case_completed_skip)
        + usize::from(
            predecessor
                .dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_executed,
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
                .dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_executed
                && predecessor.supply_enthalpy_assignment_performed =>
        {
            Some(Route::DehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioOverdryingLimitExecuted)
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
    mut left:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot,
    mut right:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot,
) -> bool {
    let values_match = [
        option_bits_match(
            left.supply_enthalpy_before_overdrying_limit_j_per_kg,
            right.supply_enthalpy_before_overdrying_limit_j_per_kg,
        ),
        option_bits_match(left.supply_temperature_c, right.supply_temperature_c),
        option_bits_match(
            left.psychrometric_minimum_supply_enthalpy_j_per_kg,
            right.psychrometric_minimum_supply_enthalpy_j_per_kg,
        ),
        option_bits_match(
            left.maximum_supply_enthalpy_j_per_kg,
            right.maximum_supply_enthalpy_j_per_kg,
        ),
        option_bits_match(
            left.assigned_supply_enthalpy_j_per_kg,
            right.assigned_supply_enthalpy_j_per_kg,
        ),
        option_bits_match(
            left.resulting_supply_enthalpy_j_per_kg,
            right.resulting_supply_enthalpy_j_per_kg,
        ),
    ]
    .into_iter()
    .all(|matches| matches);
    for snapshot in [&mut left, &mut right] {
        snapshot.supply_enthalpy_before_overdrying_limit_j_per_kg = None;
        snapshot.supply_temperature_c = None;
        snapshot.psychrometric_minimum_supply_enthalpy_j_per_kg = None;
        snapshot.maximum_supply_enthalpy_j_per_kg = None;
        snapshot.assigned_supply_enthalpy_j_per_kg = None;
        snapshot.resulting_supply_enthalpy_j_per_kg = None;
    }
    values_match && left == right
}

pub(in crate::ideal_loads::calc) fn next_transition_fits(
    state: &PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitRuntimeState,
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
        Route::DehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioOverdryingLimitExecuted => {
            let counters = [
                state
                    .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_overdrying_limit_count,
                state.supply_humidity_ratio_for_overdrying_limit_minimum_read_count,
                state.supply_temperature_for_humidity_ratio_inversion_read_count,
                state.supply_enthalpy_for_humidity_ratio_inversion_read_count,
                state.psychrometric_supply_humidity_ratio_evaluation_count,
                state.source_shaped_two_argument_minimum_evaluation_count,
                state.supply_humidity_ratio_assignment_write_count,
                state
                    .witnessed_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_overdrying_limit_count,
            ];
            counters
                .into_iter()
                .all(|counter| counter.checked_add(1).is_some())
                && state
                    .source_site_execution_count
                    .checked_add(
                        PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_OVERDRYING_LIMIT_SOURCE_ORDER
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
    active_operands: Option<
        PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitActiveOperands,
    >,
) -> bool {
    prepare_values(route, active_operands).is_some()
}

fn prepare_values(
    route: Route,
    active_operands: Option<
        PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitActiveOperands,
    >,
) -> Option<PreparedValues> {
    if route != Route::DehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioOverdryingLimitExecuted {
        return active_operands.is_none().then_some(PreparedValues::empty());
    }
    let operands = active_operands?;
    let psychrometric_humidity_ratio = energyplus_psy_w_fn_tdb_h(
        operands.supply_temperature_c,
        operands.supply_enthalpy_j_per_kg,
    );
    let minimum_humidity_ratio = source_shaped_two_argument_minimum(
        operands.supply_humidity_ratio_before_overdrying_limit,
        psychrometric_humidity_ratio,
    );
    Some(PreparedValues {
        supply_humidity_ratio_before_overdrying_limit: Some(
            operands.supply_humidity_ratio_before_overdrying_limit,
        ),
        supply_temperature_c: Some(operands.supply_temperature_c),
        supply_enthalpy_j_per_kg: Some(operands.supply_enthalpy_j_per_kg),
        psychrometric_supply_humidity_ratio: Some(psychrometric_humidity_ratio),
        minimum_supply_humidity_ratio: Some(minimum_humidity_ratio),
    })
}

fn active_prefix(
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot,
) -> bool {
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

fn inactive_prefix(
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot,
) -> bool {
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
    supply_humidity_ratio_before_overdrying_limit: Option<f64>,
    supply_temperature_c: Option<f64>,
    supply_enthalpy_j_per_kg: Option<f64>,
    psychrometric_supply_humidity_ratio: Option<f64>,
    minimum_supply_humidity_ratio: Option<f64>,
}

impl PreparedValues {
    const fn empty() -> Self {
        Self {
            supply_humidity_ratio_before_overdrying_limit: None,
            supply_temperature_c: None,
            supply_enthalpy_j_per_kg: None,
            psychrometric_supply_humidity_ratio: None,
            minimum_supply_humidity_ratio: None,
        }
    }
}
