//! Exact CP386 route and source-site accounting.

use ep_model::DehumidificationControlType;

use super::State;
use super::routes::RetainedRoute;
use crate::ideal_loads::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE_ORDER;

pub(super) fn next_transition_fits(state: &State, route: RetainedRoute) -> bool {
    if state.transition_count.checked_add(1).is_none()
        || state.predecessor_route_counts[route.predecessor.index()]
            .checked_add(1)
            .is_none()
    {
        return false;
    }
    let Some(selector) = route.selected_case else {
        return state.inactive_transition_count.checked_add(1).is_some();
    };
    if state.dehumidification_control_switch_count.checked_add(1).is_none()
        || state
            .source_site_execution_count
            .checked_add(
                PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE_ORDER.len(),
            )
            .is_none()
        || state.dehumidification_control_type_read_count.checked_add(1).is_none()
        || state.dehumidification_control_switch_dispatch_count.checked_add(1).is_none()
    {
        return false;
    }
    // Match the physical C++ case order without coupling to enum ordinals.
    match selector {
        DehumidificationControlType::ConstantSensibleHeatRatio => state
            .dehumidification_control_constant_sensible_heat_ratio_case_selection_count
            .checked_add(1)
            .is_some(),
        DehumidificationControlType::Humidistat => state
            .dehumidification_control_humidistat_case_selection_count
            .checked_add(1)
            .is_some(),
        DehumidificationControlType::None => state
            .dehumidification_control_none_case_selection_count
            .checked_add(1)
            .is_some(),
        DehumidificationControlType::ConstantSupplyHumidityRatio => state
            .dehumidification_control_constant_supply_humidity_ratio_case_selection_count
            .checked_add(1)
            .is_some(),
    }
}

pub(super) fn increment_counts(state: &mut State, route: RetainedRoute) {
    state.predecessor_route_counts[route.predecessor.index()] += 1;
    let Some(selector) = route.selected_case else {
        state.inactive_transition_count += 1;
        return;
    };
    state.dehumidification_control_switch_count += 1;
    state.source_site_execution_count +=
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE_ORDER.len();
    state.dehumidification_control_type_read_count += 1;
    state.dehumidification_control_switch_dispatch_count += 1;
    // Match the physical C++ case order without coupling to enum ordinals.
    match selector {
        DehumidificationControlType::ConstantSensibleHeatRatio => {
            state.dehumidification_control_constant_sensible_heat_ratio_case_selection_count += 1;
        }
        DehumidificationControlType::Humidistat => {
            state.dehumidification_control_humidistat_case_selection_count += 1;
        }
        DehumidificationControlType::None => {
            state.dehumidification_control_none_case_selection_count += 1;
        }
        DehumidificationControlType::ConstantSupplyHumidityRatio => {
            state.dehumidification_control_constant_supply_humidity_ratio_case_selection_count += 1;
        }
    }
}
