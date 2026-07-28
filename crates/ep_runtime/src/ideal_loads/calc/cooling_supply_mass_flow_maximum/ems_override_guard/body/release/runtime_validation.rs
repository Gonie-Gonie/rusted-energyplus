use ep_model::{IdealLoadsAirSystemId, ZoneId};

use super::snapshot_validation::{
    cooling_supply_mass_flow_ems_override_body_snapshot_is_exact_direct_release, snapshot_route,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRuntimeState,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRuntimeState,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowMaximumRuntimeState,
    PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot, PurchasedAirUnitRuntimeState,
    cooling_supply_mass_flow_ems_override_guard_snapshot_is_exact_direct_release,
    cooling_supply_mass_flow_maximum_snapshot_is_exact_direct_release,
};

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    selected: IdealLoadsAirSystemId,
) -> bool {
    unit.system == selected
        && unit.calc_entry.system == selected
        && unit.calc_minimum_oa_prefix.system == selected
        && unit.calc_cooling_entry_gate.system == selected
        && unit.calc_cooling_oa_max_flow_gate.system == selected
        && unit.calc_cooling_oa_max_flow_body.system == selected
        && unit.calc_cooling_economizer_guard.system == selected
        && unit.calc_cooling_economizer_condition.system == selected
        && unit.calc_cooling_economizer_body.system == selected
        && unit.calc_cooling_sensible_flow.system == selected
        && unit.calc_cooling_dehumidification_flow.system == selected
        && unit.calc_cooling_humidification_flow.system == selected
        && unit.calc_cooling_capacity_zero_flow_reset.system == selected
        && unit.calc_cooling_supply_mass_flow_maximum.system == selected
        && unit.calc_cooling_supply_mass_flow_ems_override_guard.system == selected
        && unit.calc_cooling_supply_mass_flow_ems_override_body.system == selected
}

pub(super) fn call_order_is_pending_body(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot,
) -> bool {
    unit.init_call_count == unit.calc_entry.call_count
        && unit.calc_cooling_supply_mass_flow_maximum.transition_count == unit.calc_entry.call_count
        && unit
            .calc_cooling_supply_mass_flow_ems_override_guard
            .transition_count
            == unit.calc_cooling_supply_mass_flow_maximum.transition_count
        && unit
            .calc_cooling_supply_mass_flow_ems_override_body
            .transition_count
            .checked_add(1)
            == Some(
                unit.calc_cooling_supply_mass_flow_ems_override_guard
                    .transition_count,
            )
        && predecessor.parent_call_ordinal
            == unit
                .calc_cooling_supply_mass_flow_ems_override_guard
                .transition_count
}

pub(super) fn completed_supply_maximum_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot,
    witness: Option<PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot>,
) -> bool {
    let state = &unit.calc_cooling_supply_mass_flow_maximum;
    partition_is_consistent(
        state.transition_count,
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.cooling_body_entry_count,
    ) && state.transition_count == unit.calc_entry.call_count
        && state.latest == Some(predecessor)
        && witness == Some(predecessor)
        && state.latest_transition_ordinal == Some(state.transition_count)
        && state.latest_route == maximum_route(predecessor)
        && predecessor.parent_call_ordinal == state.transition_count
        && predecessor.system == state.system
        && unit.controlled_zone == Some(predecessor.controlled_zone)
        && cooling_supply_mass_flow_maximum_snapshot_is_exact_direct_release(predecessor)
        && maximum_source_counters_are_consistent(state)
}

pub(super) fn completed_guard_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot,
    witness: Option<PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot>,
) -> bool {
    let state = &unit.calc_cooling_supply_mass_flow_ems_override_guard;
    partition_is_consistent(
        state.transition_count,
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.cooling_body_entry_count,
    ) && state.transition_count == unit.calc_entry.call_count
        && state.latest == Some(predecessor)
        && witness == Some(predecessor)
        && state.latest_transition_ordinal == Some(state.transition_count)
        && state.latest_route == guard_route(predecessor)
        && predecessor.parent_call_ordinal == state.transition_count
        && predecessor.system == state.system
        && unit.controlled_zone == Some(predecessor.controlled_zone)
        && cooling_supply_mass_flow_ems_override_guard_snapshot_is_exact_direct_release(predecessor)
        && guard_source_counters_are_consistent(state)
}

pub(super) fn pending_body_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot,
    witness: Option<PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot>,
) -> bool {
    let state = &unit.calc_cooling_supply_mass_flow_ems_override_body;
    partition_is_consistent(
        state.transition_count,
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.cooling_body_entry_count,
    ) && body_latest_is_valid(state, unit.controlled_zone, witness)
        && state.transition_count.checked_add(1)
            == Some(
                unit.calc_cooling_supply_mass_flow_ems_override_guard
                    .transition_count,
            )
        && state
            .unit_off_skip_count
            .checked_add(usize::from(predecessor.unit_off_skipped))
            == Some(
                unit.calc_cooling_supply_mass_flow_ems_override_guard
                    .unit_off_skip_count,
            )
        && state
            .non_cooling_skip_count
            .checked_add(usize::from(predecessor.non_cooling_skipped))
            == Some(
                unit.calc_cooling_supply_mass_flow_ems_override_guard
                    .non_cooling_skip_count,
            )
        && state
            .cooling_body_entry_count
            .checked_add(usize::from(predecessor.cooling_body_entered))
            == Some(
                unit.calc_cooling_supply_mass_flow_ems_override_guard
                    .cooling_body_entry_count,
            )
        && state.body_entry_count.checked_add(usize::from(
            predecessor.ems_supply_mass_flow_override_body_entered,
        )) == Some(
            unit.calc_cooling_supply_mass_flow_ems_override_guard
                .ems_supply_mass_flow_override_body_entry_count,
        )
        && state
            .ems_disabled_fallthrough_count
            .checked_add(usize::from(
                predecessor.ems_supply_mass_flow_override_guard_false_fallthrough,
            ))
            == Some(
                unit.calc_cooling_supply_mass_flow_ems_override_guard
                    .ems_supply_mass_flow_override_guard_false_fallthrough_count,
            )
        && state.body_skip_count.checked_add(1)
            == Some(
                unit.calc_cooling_supply_mass_flow_ems_override_guard
                    .transition_count,
            )
        && body_source_counters_are_consistent(state)
}

fn body_latest_is_valid(
    state: &PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRuntimeState,
    controlled_zone: Option<ZoneId>,
    witness: Option<PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot>,
) -> bool {
    match (
        state.transition_count,
        state.latest,
        state.latest_route,
        state.latest_transition_ordinal,
        witness,
    ) {
        (0, None, None, None, None) => true,
        (count, Some(latest), Some(route), Some(ordinal), Some(witness)) if count > 0 => {
            latest == witness
                && ordinal == count
                && latest.parent_call_ordinal == count
                && latest.system == state.system
                && controlled_zone == Some(latest.controlled_zone)
                && cooling_supply_mass_flow_ems_override_body_snapshot_is_exact_direct_release(
                    latest,
                )
                && snapshot_route(latest) == Some(route)
        }
        _ => false,
    }
}

fn maximum_source_counters_are_consistent(
    state: &PurchasedAirCalcCoolingSupplyMassFlowMaximumRuntimeState,
) -> bool {
    [
        state.outdoor_air_mass_flow_rate_read_count,
        state.supply_mass_flow_rate_for_cool_read_count,
        state.supply_mass_flow_rate_for_dehumidification_read_count,
        state.supply_mass_flow_rate_for_humidification_read_count,
        state.positive_zero_vs_outdoor_air_comparison_count,
        state.cooling_vs_dehumidification_comparison_count,
        state.leading_vs_candidate_pair_comparison_count,
        state.leading_vs_humidification_comparison_count,
        state.maximum_evaluation_count,
        state.supply_mass_flow_rate_assignment_count,
    ]
    .into_iter()
    .all(|count| count == state.cooling_body_entry_count)
}

fn guard_source_counters_are_consistent(
    state: &PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRuntimeState,
) -> bool {
    state.ems_supply_mass_flow_override_flag_read_count == state.cooling_body_entry_count
        && state.ems_supply_mass_flow_override_guard_evaluation_count
            == state.cooling_body_entry_count
        && state.ems_supply_mass_flow_override_body_entry_count == 0
        && state.ems_supply_mass_flow_override_guard_false_fallthrough_count
            == state.cooling_body_entry_count
}

fn body_source_counters_are_consistent(
    state: &PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRuntimeState,
) -> bool {
    state.body_entry_count == 0
        && state.ems_supply_mass_flow_override_value_read_count == 0
        && state.supply_mass_flow_rate_override_assignment_count == 0
        && state.outdoor_air_mass_flow_rate_for_minimum_read_count == 0
        && state.supply_mass_flow_rate_for_minimum_read_count == 0
        && state.source_shaped_two_argument_minimum_evaluation_count == 0
        && state.outdoor_air_mass_flow_rate_assignment_count == 0
        && state.body_skip_count == state.transition_count
        && state.ems_disabled_fallthrough_count == state.cooling_body_entry_count
}

fn partition_is_consistent(
    transitions: usize,
    unit_off: usize,
    non_cooling: usize,
    cooling: usize,
) -> bool {
    unit_off
        .checked_add(non_cooling)
        .and_then(|count| count.checked_add(cooling))
        == Some(transitions)
}

fn maximum_route(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot,
) -> Option<
    crate::ideal_loads::calc::cooling_supply_mass_flow_maximum::
        PurchasedAirCalcCoolingSupplyMassFlowMaximumRetainedRoute,
>{
    use crate::ideal_loads::calc::cooling_supply_mass_flow_maximum::PurchasedAirCalcCoolingSupplyMassFlowMaximumRetainedRoute as Route;
    if !cooling_supply_mass_flow_maximum_snapshot_is_exact_direct_release(snapshot) {
        None
    } else if snapshot.unit_off_skipped {
        Some(Route::UnitOff)
    } else if snapshot.non_cooling_skipped {
        Some(Route::NonCooling)
    } else {
        Some(Route::CoolingMaximumAssigned)
    }
}

fn guard_route(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot,
) -> Option<
    crate::ideal_loads::calc::cooling_supply_mass_flow_maximum::ems_override_guard::
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRetainedRoute,
>{
    use crate::ideal_loads::calc::cooling_supply_mass_flow_maximum::ems_override_guard::PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRetainedRoute as Route;
    if !cooling_supply_mass_flow_ems_override_guard_snapshot_is_exact_direct_release(snapshot) {
        None
    } else if snapshot.unit_off_skipped {
        Some(Route::UnitOff)
    } else if snapshot.non_cooling_skipped {
        Some(Route::NonCooling)
    } else {
        Some(Route::OverrideGuardFalseFallthrough)
    }
}
