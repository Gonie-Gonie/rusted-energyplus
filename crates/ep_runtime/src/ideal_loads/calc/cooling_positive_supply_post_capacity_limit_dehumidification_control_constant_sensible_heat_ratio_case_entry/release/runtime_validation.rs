//! Persistent CP348 runtime-state validation.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId};

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CASE_ENTRY_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntryRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntryRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntrySnapshot,
};
use super::snapshot_validation::{
    cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_snapshot_is_exact_direct_release,
    snapshot_route, snapshots_match_exact,
};
use crate::ideal_loads::calc::cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry::transition::{
    next_transition_fits as pure_next_transition_fits, predecessor_route,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> bool {
    unit.system == system
        && unit.calc_entry.system == system
        && unit
            .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case
            .system
            == system
        && unit
            .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry
            .system
            == system
}

pub(super) fn call_order_is_pending(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
) -> bool {
    let ordinal = predecessor.parent_call_ordinal;
    unit.calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry
        .transition_count
        .checked_add(1)
        == Some(ordinal)
        && unit.calc_entry.call_count == ordinal
        && unit
            .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case
            .transition_count
            == ordinal
}

pub(super) fn pending_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
    witness: Option<
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntrySnapshot,
    >,
    selector: DehumidificationControlType,
) -> bool {
    let Some(route) = predecessor_route(predecessor) else {
        return false;
    };
    let state = &unit
        .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry;
    let predecessor_state =
        &unit.calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case;
    state_is_consistent(state, witness, predecessor.system, selector)
        && state.transition_count.checked_add(1) == Some(predecessor.parent_call_ordinal)
        && state
            .unit_off_skip_count
            .checked_add(usize::from(route == Route::UnitOff))
            == Some(predecessor_state.unit_off_skip_count)
        && state
            .non_cooling_skip_count
            .checked_add(usize::from(route == Route::NonCooling))
            == Some(predecessor_state.non_cooling_skip_count)
        && state
            .positive_guard_false_fallthrough_skip_count
            .checked_add(usize::from(route == Route::PositiveGuardFalseFallthrough))
            == Some(predecessor_state.positive_guard_false_fallthrough_skip_count)
        && state
            .dehumidification_control_none_case_completed_skip_count
            .checked_add(usize::from(
                route == Route::DehumidificationControlNoneCaseCompletedSkip,
            ))
            == Some(predecessor_state.dehumidification_control_none_case_completion_count)
        && state
            .dehumidification_control_constant_sensible_heat_ratio_case_entry_count
            .checked_add(usize::from(
                route == Route::DehumidificationControlConstantSensibleHeatRatioCaseEntered,
            ))
            == Some(
                predecessor_state
                    .dehumidification_control_constant_sensible_heat_ratio_case_selection_count,
            )
        && state
            .dehumidification_control_humidistat_case_selected_skip_count
            .checked_add(usize::from(
                route == Route::DehumidificationControlHumidistatCaseSelectedSkip,
            ))
            == Some(predecessor_state.dehumidification_control_humidistat_case_selection_count)
        && state
            .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count
            .checked_add(usize::from(
                route == Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip,
            ))
            == Some(
                predecessor_state
                    .dehumidification_control_constant_supply_humidity_ratio_case_selection_count,
            )
}

pub(in crate::ideal_loads::calc) fn next_transition_fits(
    state: &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntryRuntimeState,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
) -> bool {
    predecessor_route(predecessor).is_some_and(|route| pure_next_transition_fits(state, route))
}

pub(super) fn completed_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntrySnapshot,
    witness: Option<
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntrySnapshot,
    >,
    selector: DehumidificationControlType,
) -> bool {
    let state = &unit
        .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry;
    let predecessor =
        &unit.calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case;
    state_is_consistent(state, witness, snapshot.system, selector)
        && state.transition_count == predecessor.transition_count
        && state.unit_off_skip_count == predecessor.unit_off_skip_count
        && state.non_cooling_skip_count == predecessor.non_cooling_skip_count
        && state.positive_guard_false_fallthrough_skip_count
            == predecessor.positive_guard_false_fallthrough_skip_count
        && state.dehumidification_control_none_case_completed_skip_count
            == predecessor.dehumidification_control_none_case_completion_count
        && state.dehumidification_control_constant_sensible_heat_ratio_case_entry_count
            == predecessor
                .dehumidification_control_constant_sensible_heat_ratio_case_selection_count
        && state.dehumidification_control_humidistat_case_selected_skip_count
            == predecessor.dehumidification_control_humidistat_case_selection_count
        && state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count
            == predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_selection_count
        && state
            .latest
            .is_some_and(|latest| snapshots_match_exact(latest, snapshot))
}

fn state_is_consistent(
    state: &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntryRuntimeState,
    witness: Option<
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntrySnapshot,
    >,
    expected_system: IdealLoadsAirSystemId,
    selector: DehumidificationControlType,
) -> bool {
    let Some(route_partition) = state
        .unit_off_skip_count
        .checked_add(state.non_cooling_skip_count)
        .and_then(|count| count.checked_add(state.positive_guard_false_fallthrough_skip_count))
        .and_then(|count| {
            count.checked_add(state.dehumidification_control_none_case_completed_skip_count)
        })
        .and_then(|count| {
            count.checked_add(
                state.dehumidification_control_constant_sensible_heat_ratio_case_entry_count,
            )
        })
        .and_then(|count| {
            count.checked_add(state.dehumidification_control_humidistat_case_selected_skip_count)
        })
        .and_then(|count| {
            count.checked_add(
                state
                    .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
            )
        })
    else {
        return false;
    };
    let Some(active) = state
        .dehumidification_control_none_case_completed_skip_count
        .checked_add(
            state.dehumidification_control_constant_sensible_heat_ratio_case_entry_count,
        )
        .and_then(|count| {
            count.checked_add(state.dehumidification_control_humidistat_case_selected_skip_count)
        })
        .and_then(|count| {
            count.checked_add(
                state
                    .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
            )
        })
    else {
        return false;
    };
    let Some(expected_source_sites) = state
        .dehumidification_control_constant_sensible_heat_ratio_case_entry_count
        .checked_mul(
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CASE_ENTRY_SOURCE_ORDER
                .len(),
        )
    else {
        return false;
    };
    let expected_none = usize::from(selector == DehumidificationControlType::None) * active;
    let expected_constant_sensible =
        usize::from(selector == DehumidificationControlType::ConstantSensibleHeatRatio) * active;
    let expected_humidistat =
        usize::from(selector == DehumidificationControlType::Humidistat) * active;
    let expected_constant_supply =
        usize::from(selector == DehumidificationControlType::ConstantSupplyHumidityRatio) * active;
    let counters_match = state.system == expected_system
        && route_partition == state.transition_count
        && state.source_site_execution_count == expected_source_sites
        && state.dehumidification_control_none_case_completed_skip_count == expected_none
        && state.dehumidification_control_constant_sensible_heat_ratio_case_entry_count
            == expected_constant_sensible
        && state.dehumidification_control_humidistat_case_selected_skip_count
            == expected_humidistat
        && state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count
            == expected_constant_supply
        && state.dehumidification_control_constant_sensible_heat_ratio_case_entry_site_count
            == expected_constant_sensible;
    if !counters_match {
        return false;
    }
    match (state.transition_count, state.latest, witness) {
        (0, None, None) => {
            state.latest_route.is_none() && state.latest_transition_ordinal.is_none()
        }
        (count, Some(latest), Some(witness)) => {
            count > 0
                && state.latest_transition_ordinal == Some(count)
                && snapshot_route(latest) == state.latest_route
                && latest.system == expected_system
                && latest.parent_call_ordinal == count
                && cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_snapshot_is_exact_direct_release(
                    latest,
                )
                && snapshots_match_exact(latest, witness)
                && (!latest.unit_body_entered
                    || latest.predecessor_dehumidification_control_type.is_none()
                    || latest.predecessor_dehumidification_control_type == Some(selector))
        }
        _ => false,
    }
}
