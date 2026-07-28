//! Persistent CP347 runtime-state validation.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId};

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseActiveInput,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
};
use super::prefix_validation::predecessor_selects_none_case;
use super::snapshot_validation::{
    cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_snapshot_is_exact_direct_release,
    snapshot_route, snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> bool {
    unit.system == system
        && unit.calc_entry.system == system
        && unit.calc_cooling_mixed_air_call.system == system
        && unit
            .calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment
            .system
            == system
        && unit
            .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch
            .system
            == system
        && unit
            .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case
            .system
            == system
}

pub(super) fn call_order_is_pending(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
) -> bool {
    let ordinal = predecessor.parent_call_ordinal;
    unit.calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case
        .transition_count
        .checked_add(1)
        == Some(ordinal)
        && unit.calc_entry.call_count == ordinal
        && unit
            .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch
            .transition_count
            == ordinal
}

pub(in crate::ideal_loads::calc) fn pending_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
    witness: Option<
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
    >,
    selector: DehumidificationControlType,
) -> bool {
    let state =
        &unit.calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case;
    let predecessor_state =
        &unit.calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch;
    state_is_consistent(state, witness, predecessor.system, selector)
        && state.transition_count.checked_add(1) == Some(predecessor.parent_call_ordinal)
        && state
            .unit_off_skip_count
            .checked_add(usize::from(predecessor.unit_off_skipped))
            == Some(predecessor_state.unit_off_skip_count)
        && state
            .non_cooling_skip_count
            .checked_add(usize::from(predecessor.non_cooling_skipped))
            == Some(predecessor_state.non_cooling_skip_count)
        && state
            .positive_guard_false_fallthrough_skip_count
            .checked_add(usize::from(
                predecessor.positive_guard_false_fallthrough_skipped,
            ))
            == Some(predecessor_state.positive_guard_false_fallthrough_skip_count)
        && state
            .dehumidification_control_none_case_completion_count
            .checked_add(usize::from(predecessor_selects_none_case(predecessor)))
            == Some(predecessor_state.dehumidification_control_none_case_selection_count)
        && state
            .dehumidification_control_constant_sensible_heat_ratio_case_selection_count
            .checked_add(usize::from(
                predecessor.dehumidification_control_type
                    == Some(DehumidificationControlType::ConstantSensibleHeatRatio),
            ))
            == Some(
                predecessor_state
                    .dehumidification_control_constant_sensible_heat_ratio_case_selection_count,
            )
        && state
            .dehumidification_control_humidistat_case_selection_count
            .checked_add(usize::from(
                predecessor.dehumidification_control_type
                    == Some(DehumidificationControlType::Humidistat),
            ))
            == Some(predecessor_state.dehumidification_control_humidistat_case_selection_count)
        && state
            .dehumidification_control_constant_supply_humidity_ratio_case_selection_count
            .checked_add(usize::from(
                predecessor.dehumidification_control_type
                    == Some(DehumidificationControlType::ConstantSupplyHumidityRatio),
            ))
            == Some(
                predecessor_state
                    .dehumidification_control_constant_supply_humidity_ratio_case_selection_count,
            )
}

pub(in crate::ideal_loads::calc) fn next_transition_fits(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
    active_input: Option<
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseActiveInput,
    >,
) -> bool {
    let state =
        &unit.calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case;
    if state.transition_count.checked_add(1).is_none() {
        return false;
    }
    if predecessor.unit_off_skipped {
        return active_input.is_none() && state.unit_off_skip_count.checked_add(1).is_some();
    }
    if predecessor.non_cooling_skipped {
        return active_input.is_none() && state.non_cooling_skip_count.checked_add(1).is_some();
    }
    if predecessor.positive_guard_false_fallthrough_skipped {
        return active_input.is_none()
            && state
                .positive_guard_false_fallthrough_skip_count
                .checked_add(1)
                .is_some()
            && state
                .witnessed_positive_guard_false_fallthrough_skip_count
                .checked_add(1)
                .is_some();
    }
    match predecessor.dehumidification_control_type {
        Some(DehumidificationControlType::None) => {
            active_input.is_some_and(|input| {
                predecessor
                    .predecessor_assigned_supply_humidity_ratio
                    .is_some_and(|predecessor| {
                        predecessor.to_bits() == input.mixed_air_humidity_ratio.to_bits()
                    })
            })
                && state
                    .dehumidification_control_none_case_completion_count
                    .checked_add(1)
                    .is_some()
                && state
                    .source_site_execution_count
                    .checked_add(
                        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE_ORDER
                            .len(),
                    )
                    .is_some()
                && state
                    .dehumidification_control_none_case_entry_count
                    .checked_add(1)
                    .is_some()
                && state
                    .mixed_air_humidity_ratio_read_count
                    .checked_add(1)
                    .is_some()
                && state
                    .supply_humidity_ratio_assignment_count
                    .checked_add(1)
                    .is_some()
                && state
                    .dehumidification_control_none_case_break_count
                    .checked_add(1)
                    .is_some()
                && state
                    .witnessed_dehumidification_control_none_case_completion_count
                    .checked_add(1)
                    .is_some()
        }
        Some(DehumidificationControlType::ConstantSensibleHeatRatio) => {
            active_input.is_none()
                && state
                    .dehumidification_control_constant_sensible_heat_ratio_case_selection_count
                    .checked_add(1)
                    .is_some()
                && state
                    .witnessed_dehumidification_control_constant_sensible_heat_ratio_case_selection_count
                    .checked_add(1)
                    .is_some()
        }
        Some(DehumidificationControlType::Humidistat) => {
            active_input.is_none()
                && state
                    .dehumidification_control_humidistat_case_selection_count
                    .checked_add(1)
                    .is_some()
                && state
                    .witnessed_dehumidification_control_humidistat_case_selection_count
                    .checked_add(1)
                    .is_some()
        }
        Some(DehumidificationControlType::ConstantSupplyHumidityRatio) => {
            active_input.is_none()
                && state
                    .dehumidification_control_constant_supply_humidity_ratio_case_selection_count
                    .checked_add(1)
                    .is_some()
                && state
                    .witnessed_dehumidification_control_constant_supply_humidity_ratio_case_selection_count
                    .checked_add(1)
                    .is_some()
        }
        None => false,
    }
}

pub(super) fn completed_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
    witness: Option<
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
    >,
    selector: DehumidificationControlType,
) -> bool {
    let state =
        &unit.calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case;
    let predecessor =
        &unit.calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch;
    state_is_consistent(state, witness, snapshot.system, selector)
        && state.transition_count == predecessor.transition_count
        && state.unit_off_skip_count == predecessor.unit_off_skip_count
        && state.non_cooling_skip_count == predecessor.non_cooling_skip_count
        && state.positive_guard_false_fallthrough_skip_count
            == predecessor.positive_guard_false_fallthrough_skip_count
        && state.dehumidification_control_none_case_completion_count
            == predecessor.dehumidification_control_none_case_selection_count
        && state.dehumidification_control_constant_sensible_heat_ratio_case_selection_count
            == predecessor
                .dehumidification_control_constant_sensible_heat_ratio_case_selection_count
        && state.dehumidification_control_humidistat_case_selection_count
            == predecessor.dehumidification_control_humidistat_case_selection_count
        && state.dehumidification_control_constant_supply_humidity_ratio_case_selection_count
            == predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_selection_count
        && state
            .latest
            .is_some_and(|latest| snapshots_match_bit_exact(latest, snapshot))
}

fn state_is_consistent(
    state: &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseRuntimeState,
    witness: Option<
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
    >,
    expected_system: IdealLoadsAirSystemId,
    selector: DehumidificationControlType,
) -> bool {
    let Some(route_partition) = state
        .unit_off_skip_count
        .checked_add(state.non_cooling_skip_count)
        .and_then(|count| count.checked_add(state.positive_guard_false_fallthrough_skip_count))
        .and_then(|count| {
            count.checked_add(state.dehumidification_control_none_case_completion_count)
        })
        .and_then(|count| {
            count.checked_add(
                state.dehumidification_control_constant_sensible_heat_ratio_case_selection_count,
            )
        })
        .and_then(|count| {
            count.checked_add(state.dehumidification_control_humidistat_case_selection_count)
        })
        .and_then(|count| {
            count.checked_add(
                state.dehumidification_control_constant_supply_humidity_ratio_case_selection_count,
            )
        })
    else {
        return false;
    };
    let Some(active) = state
        .dehumidification_control_none_case_completion_count
        .checked_add(
            state.dehumidification_control_constant_sensible_heat_ratio_case_selection_count,
        )
        .and_then(|count| {
            count.checked_add(state.dehumidification_control_humidistat_case_selection_count)
        })
        .and_then(|count| {
            count.checked_add(
                state.dehumidification_control_constant_supply_humidity_ratio_case_selection_count,
            )
        })
    else {
        return false;
    };
    let Some(expected_source_sites) = state
        .dehumidification_control_none_case_completion_count
        .checked_mul(
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE_ORDER
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
        && state.dehumidification_control_none_case_completion_count == expected_none
        && state.dehumidification_control_constant_sensible_heat_ratio_case_selection_count
            == expected_constant_sensible
        && state.dehumidification_control_humidistat_case_selection_count == expected_humidistat
        && state.dehumidification_control_constant_supply_humidity_ratio_case_selection_count
            == expected_constant_supply
        && state.dehumidification_control_none_case_entry_count == expected_none
        && state.mixed_air_humidity_ratio_read_count == expected_none
        && state.supply_humidity_ratio_assignment_count == expected_none
        && state.dehumidification_control_none_case_break_count == expected_none
        && state.witnessed_positive_guard_false_fallthrough_skip_count
            == state.positive_guard_false_fallthrough_skip_count
        && state.witnessed_dehumidification_control_none_case_completion_count
            == state.dehumidification_control_none_case_completion_count
        && state
            .witnessed_dehumidification_control_constant_sensible_heat_ratio_case_selection_count
            == state.dehumidification_control_constant_sensible_heat_ratio_case_selection_count
        && state.witnessed_dehumidification_control_humidistat_case_selection_count
            == state.dehumidification_control_humidistat_case_selection_count
        && state
            .witnessed_dehumidification_control_constant_supply_humidity_ratio_case_selection_count
            == state.dehumidification_control_constant_supply_humidity_ratio_case_selection_count;
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
                && cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_snapshot_is_exact_direct_release(
                    latest,
                )
                && snapshots_match_bit_exact(latest, witness)
                && (!latest.predecessor_dehumidification_control_type_read
                    || latest.predecessor_dehumidification_control_type == Some(selector))
        }
        _ => false,
    }
}
