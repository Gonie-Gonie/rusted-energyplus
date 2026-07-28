//! Pure CP345-to-CP346 dehumidification-control switch dispatch.

use ep_model::DehumidificationControlType;

use super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchActiveInput
{
    pub dehumidification_control_type: DehumidificationControlType,
}

pub(in crate::ideal_loads::calc) fn advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_state(
    state: &mut PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchRuntimeState,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
    active_input: Option<
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchActiveInput,
    >,
) -> Option<
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
> {
    let active =
        predecessor.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed;
    let dehumidification_control_type =
        active_input.map(|input| input.dehumidification_control_type);

    let route = match (
        predecessor.unit_off_skipped,
        predecessor.non_cooling_skipped,
        predecessor.positive_guard_false_fallthrough_skipped,
        active,
        dehumidification_control_type,
    ) {
        (true, false, false, false, None) => Route::UnitOff,
        (false, true, false, false, None) => Route::NonCooling,
        (false, false, true, false, None) => Route::PositiveGuardFalseFallthrough,
        (false, false, false, true, Some(DehumidificationControlType::None)) => {
            Route::DehumidificationControlNoneCaseSelected
        }
        (
            false,
            false,
            false,
            true,
            Some(DehumidificationControlType::ConstantSensibleHeatRatio),
        ) => Route::DehumidificationControlConstantSensibleHeatRatioCaseSelected,
        (false, false, false, true, Some(DehumidificationControlType::Humidistat)) => {
            Route::DehumidificationControlHumidistatCaseSelected
        }
        (
            false,
            false,
            false,
            true,
            Some(DehumidificationControlType::ConstantSupplyHumidityRatio),
        ) => Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelected,
        _ => return None,
    };

    state.transition_count += 1;
    match route {
        Route::UnitOff => state.unit_off_skip_count += 1,
        Route::NonCooling => state.non_cooling_skip_count += 1,
        Route::PositiveGuardFalseFallthrough => {
            state.positive_guard_false_fallthrough_skip_count += 1;
            state.witnessed_positive_guard_false_fallthrough_skip_count += 1;
        }
        Route::DehumidificationControlNoneCaseSelected
        | Route::DehumidificationControlConstantSensibleHeatRatioCaseSelected
        | Route::DehumidificationControlHumidistatCaseSelected
        | Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelected => {
            state.dehumidification_control_switch_count += 1;
            state.source_site_execution_count +=
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE_ORDER
                    .len();
            state.dehumidification_control_type_read_count += 1;
            state.dehumidification_control_switch_dispatch_count += 1;
            match route {
                Route::DehumidificationControlNoneCaseSelected => {
                    state.dehumidification_control_none_case_selection_count += 1;
                    state.witnessed_dehumidification_control_none_case_selection_count += 1;
                }
                Route::DehumidificationControlConstantSensibleHeatRatioCaseSelected => {
                    state
                        .dehumidification_control_constant_sensible_heat_ratio_case_selection_count +=
                        1;
                    state
                        .witnessed_dehumidification_control_constant_sensible_heat_ratio_case_selection_count +=
                        1;
                }
                Route::DehumidificationControlHumidistatCaseSelected => {
                    state.dehumidification_control_humidistat_case_selection_count += 1;
                    state.witnessed_dehumidification_control_humidistat_case_selection_count += 1;
                }
                Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelected => {
                    state
                        .dehumidification_control_constant_supply_humidity_ratio_case_selection_count +=
                        1;
                    state
                        .witnessed_dehumidification_control_constant_supply_humidity_ratio_case_selection_count +=
                        1;
                }
                Route::UnitOff
                | Route::NonCooling
                | Route::PositiveGuardFalseFallthrough => {}
            }
        }
    }

    let snapshot =
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE_ORDER,
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
                .capacity_limit_guard_false_fallthrough_skipped,
            predecessor_capacity_limit_sensible_output_guard_false_fallthrough: predecessor
                .capacity_limit_sensible_output_guard_false_fallthrough,
            predecessor_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed:
                predecessor
                    .capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed,
            predecessor_post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed:
                active,
            predecessor_assigned_supply_humidity_ratio: predecessor
                .assigned_supply_humidity_ratio,
            dehumidification_control_type_read: active,
            dehumidification_control_type,
            dehumidification_control_switch_dispatched: active,
        };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    Some(snapshot)
}
