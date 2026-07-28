//! Pure CP347-to-CP348 constant-SHR case-entry transition.

use ep_model::DehumidificationControlType;

use super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CASE_ENTRY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CASE_ENTRY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CASE_ENTRY_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntryRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntryRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntrySnapshot,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
};

pub(in crate::ideal_loads::calc) fn advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_state(
    state: &mut PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntryRuntimeState,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
) -> Option<
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntrySnapshot,
>{
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
        }
        Route::DehumidificationControlNoneCaseCompletedSkip => {
            state.dehumidification_control_none_case_completed_skip_count += 1;
        }
        Route::DehumidificationControlConstantSensibleHeatRatioCaseEntered => {
            state.dehumidification_control_constant_sensible_heat_ratio_case_entry_count += 1;
            state.source_site_execution_count +=
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CASE_ENTRY_SOURCE_ORDER
                    .len();
            state.dehumidification_control_constant_sensible_heat_ratio_case_entry_site_count += 1;
        }
        Route::DehumidificationControlHumidistatCaseSelectedSkip => {
            state.dehumidification_control_humidistat_case_selected_skip_count += 1;
        }
        Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip => {
            state
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count +=
                1;
        }
    }

    let none_completed = route == Route::DehumidificationControlNoneCaseCompletedSkip;
    let constant_sensible_entered =
        route == Route::DehumidificationControlConstantSensibleHeatRatioCaseEntered;
    let humidistat_skip = route == Route::DehumidificationControlHumidistatCaseSelectedSkip;
    let constant_supply_skip =
        route == Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip;
    let snapshot = PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntrySnapshot {
        source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CASE_ENTRY_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CASE_ENTRY_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CASE_ENTRY_SOURCE_ORDER,
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
        predecessor_dehumidification_control_none_case_completed: none_completed,
        dehumidification_control_none_case_completed_skip: none_completed,
        dehumidification_control_constant_sensible_heat_ratio_case_entered:
            constant_sensible_entered,
        dehumidification_control_humidistat_case_selected_skip: humidistat_skip,
        dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            constant_supply_skip,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    Some(snapshot)
}

pub(in crate::ideal_loads::calc) fn predecessor_route(
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
) -> Option<Route> {
    if predecessor.source
        != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_FIRST_EXCLUDED_SOURCE
        || predecessor.source_order
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE_ORDER
    {
        return None;
    }

    let inactive_none_case = none_case_is_inactive(predecessor);
    let inactive_capacity_join = !predecessor.capacity_join_entered();
    if predecessor.unit_off_skipped
        && !predecessor.non_cooling_skipped
        && !predecessor.unit_body_entered
        && !predecessor.predecessor_cooling_body_entered
        && !predecessor.predecessor_no_outdoor_air_fallback_entered
        && !predecessor.predecessor_positive_supply_mass_flow_body_entered
        && !predecessor.positive_guard_false_fallthrough_skipped
        && inactive_capacity_join
        && predecessor
            .predecessor_assigned_supply_humidity_ratio
            .is_none()
        && !predecessor.predecessor_dehumidification_control_type_read
        && predecessor
            .predecessor_dehumidification_control_type
            .is_none()
        && !predecessor.predecessor_dehumidification_control_switch_dispatched
        && inactive_none_case
    {
        return Some(Route::UnitOff);
    }
    if !predecessor.unit_off_skipped
        && predecessor.non_cooling_skipped
        && predecessor.unit_body_entered
        && !predecessor.predecessor_cooling_body_entered
        && !predecessor.predecessor_no_outdoor_air_fallback_entered
        && !predecessor.predecessor_positive_supply_mass_flow_body_entered
        && !predecessor.positive_guard_false_fallthrough_skipped
        && inactive_capacity_join
        && predecessor
            .predecessor_assigned_supply_humidity_ratio
            .is_none()
        && !predecessor.predecessor_dehumidification_control_type_read
        && predecessor
            .predecessor_dehumidification_control_type
            .is_none()
        && !predecessor.predecessor_dehumidification_control_switch_dispatched
        && inactive_none_case
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
        && inactive_capacity_join
        && predecessor
            .predecessor_assigned_supply_humidity_ratio
            .is_none()
        && !predecessor.predecessor_dehumidification_control_type_read
        && predecessor
            .predecessor_dehumidification_control_type
            .is_none()
        && !predecessor.predecessor_dehumidification_control_switch_dispatched
        && inactive_none_case
    {
        return Some(Route::PositiveGuardFalseFallthrough);
    }

    let active = !predecessor.unit_off_skipped
        && !predecessor.non_cooling_skipped
        && predecessor.unit_body_entered
        && predecessor.predecessor_cooling_body_entered
        && predecessor.predecessor_no_outdoor_air_fallback_entered
        && predecessor.predecessor_positive_supply_mass_flow_body_entered
        && !predecessor.positive_guard_false_fallthrough_skipped
        && predecessor.capacity_join_count() == 1
        && predecessor
            .predecessor_post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed
        && predecessor
            .predecessor_assigned_supply_humidity_ratio
            .is_some()
        && predecessor.predecessor_dehumidification_control_type_read
        && predecessor.predecessor_dehumidification_control_switch_dispatched;
    if !active {
        return None;
    }
    match predecessor.predecessor_dehumidification_control_type? {
        DehumidificationControlType::None if none_case_is_completed(predecessor) => {
            Some(Route::DehumidificationControlNoneCaseCompletedSkip)
        }
        DehumidificationControlType::ConstantSensibleHeatRatio if inactive_none_case => {
            Some(Route::DehumidificationControlConstantSensibleHeatRatioCaseEntered)
        }
        DehumidificationControlType::Humidistat if inactive_none_case => {
            Some(Route::DehumidificationControlHumidistatCaseSelectedSkip)
        }
        DehumidificationControlType::ConstantSupplyHumidityRatio if inactive_none_case => {
            Some(Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip)
        }
        _ => None,
    }
}

pub(in crate::ideal_loads::calc) fn predecessor_snapshots_match_bit_exact(
    mut left:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
    mut right:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
) -> bool {
    let values_match = [
        option_bits_match(
            left.predecessor_assigned_supply_humidity_ratio,
            right.predecessor_assigned_supply_humidity_ratio,
        ),
        option_bits_match(
            left.mixed_air_humidity_ratio,
            right.mixed_air_humidity_ratio,
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
        snapshot.predecessor_assigned_supply_humidity_ratio = None;
        snapshot.mixed_air_humidity_ratio = None;
        snapshot.assigned_supply_humidity_ratio = None;
        snapshot.resulting_supply_humidity_ratio = None;
    }
    values_match && left == right
}

pub(in crate::ideal_loads::calc) fn next_transition_fits(
    state: &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntryRuntimeState,
    route: Route,
) -> bool {
    if state.transition_count.checked_add(1).is_none() {
        return false;
    }
    match route {
        Route::UnitOff => state.unit_off_skip_count.checked_add(1).is_some(),
        Route::NonCooling => state.non_cooling_skip_count.checked_add(1).is_some(),
        Route::PositiveGuardFalseFallthrough => state
            .positive_guard_false_fallthrough_skip_count
            .checked_add(1)
            .is_some(),
        Route::DehumidificationControlNoneCaseCompletedSkip => state
            .dehumidification_control_none_case_completed_skip_count
            .checked_add(1)
            .is_some(),
        Route::DehumidificationControlConstantSensibleHeatRatioCaseEntered => {
            state
                .dehumidification_control_constant_sensible_heat_ratio_case_entry_count
                .checked_add(1)
                .is_some()
                && state
                    .source_site_execution_count
                    .checked_add(
                        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CASE_ENTRY_SOURCE_ORDER
                            .len(),
                    )
                    .is_some()
                && state
                    .dehumidification_control_constant_sensible_heat_ratio_case_entry_site_count
                    .checked_add(1)
                    .is_some()
        }
        Route::DehumidificationControlHumidistatCaseSelectedSkip => state
            .dehumidification_control_humidistat_case_selected_skip_count
            .checked_add(1)
            .is_some(),
        Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip => state
            .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count
            .checked_add(1)
            .is_some(),
    }
}

fn none_case_is_completed(
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
) -> bool {
    predecessor.dehumidification_control_none_case_entered
        && predecessor.mixed_air_humidity_ratio_read
        && predecessor.supply_humidity_ratio_assignment_performed
        && predecessor.dehumidification_control_none_case_exited_via_break
        && predecessor.mixed_air_humidity_ratio.is_some()
        && option_bits_match(
            predecessor.predecessor_assigned_supply_humidity_ratio,
            predecessor.mixed_air_humidity_ratio,
        )
        && option_bits_match(
            predecessor.mixed_air_humidity_ratio,
            predecessor.assigned_supply_humidity_ratio,
        )
        && option_bits_match(
            predecessor.assigned_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
}

fn none_case_is_inactive(
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
) -> bool {
    !predecessor.dehumidification_control_none_case_entered
        && !predecessor.mixed_air_humidity_ratio_read
        && predecessor.mixed_air_humidity_ratio.is_none()
        && !predecessor.supply_humidity_ratio_assignment_performed
        && predecessor.assigned_supply_humidity_ratio.is_none()
        && predecessor.resulting_supply_humidity_ratio.is_none()
        && !predecessor.dehumidification_control_none_case_exited_via_break
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

trait CapacityJoin {
    fn capacity_join_count(&self) -> usize;
    fn capacity_join_entered(&self) -> bool {
        self.capacity_join_count() != 0
    }
}

impl CapacityJoin
    for PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot
{
    fn capacity_join_count(&self) -> usize {
        usize::from(self.predecessor_capacity_limit_guard_false_fallthrough)
            + usize::from(
                self.predecessor_capacity_limit_sensible_output_guard_false_fallthrough,
            )
            + usize::from(
                self.predecessor_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed,
            )
    }
}
