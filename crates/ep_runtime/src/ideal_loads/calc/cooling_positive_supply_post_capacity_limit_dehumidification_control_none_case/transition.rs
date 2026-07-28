//! Pure CP346-to-CP347 dehumidification-control `None` case.

use ep_model::DehumidificationControlType;

use super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseActiveInput
{
    pub mixed_air_humidity_ratio: f64,
}

pub(in crate::ideal_loads::calc) fn advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_state(
    state: &mut PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseRuntimeState,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
    active_input: Option<
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseActiveInput,
    >,
) -> Option<
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
> {
    let route = match (
        predecessor.unit_off_skipped,
        predecessor.non_cooling_skipped,
        predecessor.positive_guard_false_fallthrough_skipped,
        predecessor
            .predecessor_post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed,
        predecessor.dehumidification_control_type_read,
        predecessor.dehumidification_control_type,
        predecessor.dehumidification_control_switch_dispatched,
        active_input.is_some(),
    ) {
        (true, false, false, false, false, None, false, false) => Route::UnitOff,
        (false, true, false, false, false, None, false, false) => Route::NonCooling,
        (false, false, true, false, false, None, false, false) => {
            Route::PositiveGuardFalseFallthrough
        }
        (false, false, false, true, true, Some(DehumidificationControlType::None), true, true) => {
            Route::DehumidificationControlNoneCaseCompleted
        }
        (
            false,
            false,
            false,
            true,
            true,
            Some(DehumidificationControlType::ConstantSensibleHeatRatio),
            true,
            false,
        ) => Route::DehumidificationControlConstantSensibleHeatRatioCaseSelected,
        (
            false,
            false,
            false,
            true,
            true,
            Some(DehumidificationControlType::Humidistat),
            true,
            false,
        ) => Route::DehumidificationControlHumidistatCaseSelected,
        (
            false,
            false,
            false,
            true,
            true,
            Some(DehumidificationControlType::ConstantSupplyHumidityRatio),
            true,
            false,
        ) => Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelected,
        _ => return None,
    };

    let mixed_air_humidity_ratio = active_input.map(|input| input.mixed_air_humidity_ratio);
    let none_case_completed = route == Route::DehumidificationControlNoneCaseCompleted;
    let values_fit_route = match route {
        Route::UnitOff | Route::NonCooling | Route::PositiveGuardFalseFallthrough => predecessor
            .predecessor_assigned_supply_humidity_ratio
            .is_none(),
        Route::DehumidificationControlNoneCaseCompleted => match (
            predecessor.predecessor_assigned_supply_humidity_ratio,
            mixed_air_humidity_ratio,
        ) {
            (Some(predecessor), Some(owner)) => predecessor.to_bits() == owner.to_bits(),
            _ => false,
        },
        Route::DehumidificationControlConstantSensibleHeatRatioCaseSelected
        | Route::DehumidificationControlHumidistatCaseSelected
        | Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelected => predecessor
            .predecessor_assigned_supply_humidity_ratio
            .is_some(),
    };
    if !values_fit_route {
        return None;
    }
    let assigned_supply_humidity_ratio = none_case_completed
        .then_some(mixed_air_humidity_ratio)
        .flatten();
    let resulting_supply_humidity_ratio = assigned_supply_humidity_ratio;

    state.transition_count += 1;
    match route {
        Route::UnitOff => state.unit_off_skip_count += 1,
        Route::NonCooling => state.non_cooling_skip_count += 1,
        Route::PositiveGuardFalseFallthrough => {
            state.positive_guard_false_fallthrough_skip_count += 1;
            state.witnessed_positive_guard_false_fallthrough_skip_count += 1;
        }
        Route::DehumidificationControlNoneCaseCompleted => {
            state.dehumidification_control_none_case_completion_count += 1;
            state.source_site_execution_count +=
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE_ORDER
                    .len();
            state.dehumidification_control_none_case_entry_count += 1;
            state.mixed_air_humidity_ratio_read_count += 1;
            state.supply_humidity_ratio_assignment_count += 1;
            state.dehumidification_control_none_case_break_count += 1;
            state.witnessed_dehumidification_control_none_case_completion_count += 1;
        }
        Route::DehumidificationControlConstantSensibleHeatRatioCaseSelected => {
            state.dehumidification_control_constant_sensible_heat_ratio_case_selection_count += 1;
            state
                .witnessed_dehumidification_control_constant_sensible_heat_ratio_case_selection_count +=
                1;
        }
        Route::DehumidificationControlHumidistatCaseSelected => {
            state.dehumidification_control_humidistat_case_selection_count += 1;
            state.witnessed_dehumidification_control_humidistat_case_selection_count += 1;
        }
        Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelected => {
            state.dehumidification_control_constant_supply_humidity_ratio_case_selection_count += 1;
            state
                .witnessed_dehumidification_control_constant_supply_humidity_ratio_case_selection_count +=
                1;
        }
    }

    let snapshot =
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE_ORDER,
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
            predecessor_capacity_limit_guard_false_fallthrough: predecessor
                .predecessor_capacity_limit_guard_false_fallthrough,
            predecessor_capacity_limit_sensible_output_guard_false_fallthrough: predecessor
                .predecessor_capacity_limit_sensible_output_guard_false_fallthrough,
            predecessor_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed:
                predecessor
                    .predecessor_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed,
            predecessor_post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed:
                predecessor
                    .predecessor_post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed,
            predecessor_assigned_supply_humidity_ratio: predecessor
                .predecessor_assigned_supply_humidity_ratio,
            predecessor_dehumidification_control_type_read: predecessor
                .dehumidification_control_type_read,
            predecessor_dehumidification_control_type: predecessor
                .dehumidification_control_type,
            predecessor_dehumidification_control_switch_dispatched: predecessor
                .dehumidification_control_switch_dispatched,
            dehumidification_control_none_case_entered: none_case_completed,
            mixed_air_humidity_ratio_read: none_case_completed,
            mixed_air_humidity_ratio,
            supply_humidity_ratio_assignment_performed: none_case_completed,
            assigned_supply_humidity_ratio,
            resulting_supply_humidity_ratio,
            dehumidification_control_none_case_exited_via_break: none_case_completed,
        };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    Some(snapshot)
}
