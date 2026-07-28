//! Persistent CP344 runtime-state validation.

use ep_model::IdealLoadsAirSystemId;

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRetainedInput,
};
use super::snapshot_validation::{
    cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release,
    snapshot_route, snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
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
            .calc_cooling_positive_supply_capacity_limit_sensible_output_guard
            .system
            == system
        && unit
            .calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment
            .system
            == system
        && unit
            .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment
            .system
            == system
        && unit
            .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment
            .system
            == system
        && unit
            .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit
            .system
            == system
}

pub(super) fn call_order_is_pending(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
) -> bool {
    let ordinal = predecessor.parent_call_ordinal;
    unit.calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit
        .transition_count
        .checked_add(1)
        == Some(ordinal)
        && unit.calc_entry.call_count == ordinal
        && unit
            .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment
            .transition_count
            == ordinal
}

pub(super) fn pending_supply_temperature_mixed_air_limit_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
    witness: Option<
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    >,
) -> bool {
    let state = &unit
        .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit;
    let predecessor_state = &unit
        .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment;
    state_is_consistent(state, witness, predecessor.system)
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
            .capacity_limit_guard_false_fallthrough_skip_count
            .checked_add(usize::from(
                predecessor.capacity_limit_guard_false_fallthrough_skipped,
            ))
            == Some(predecessor_state.capacity_limit_guard_false_fallthrough_skip_count)
        && state
            .capacity_limit_sensible_output_guard_false_fallthrough_count
            .checked_add(usize::from(
                predecessor.capacity_limit_sensible_output_guard_false_fallthrough,
            ))
            == Some(predecessor_state.capacity_limit_sensible_output_guard_false_fallthrough_count)
        && state
            .capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count
            .checked_add(usize::from(
                predecessor
                    .capacity_limit_sensible_output_supply_temperature_assignment_executed,
            ))
            == Some(
                predecessor_state
                    .capacity_limit_sensible_output_supply_temperature_assignment_count,
            )
}

pub(super) fn next_supply_temperature_mixed_air_limit_transition_fits(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
    retained_input: Option<
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRetainedInput,
    >,
) -> bool {
    let state = &unit
        .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit;
    let active = predecessor.capacity_limit_sensible_output_guard_false_fallthrough
        || predecessor.capacity_limit_sensible_output_supply_temperature_assignment_executed;
    if state.transition_count.checked_add(1).is_none()
        || active != retained_input.is_some()
        || predecessor.capacity_limit_sensible_output_supply_temperature_assignment_executed
            != retained_input
                .and_then(|input| input.active_operands)
                .is_some()
    {
        return false;
    }
    if predecessor.unit_off_skipped {
        return state.unit_off_skip_count.checked_add(1).is_some();
    }
    if predecessor.non_cooling_skipped {
        return state.non_cooling_skip_count.checked_add(1).is_some();
    }
    if predecessor.positive_guard_false_fallthrough_skipped {
        return state
            .positive_guard_false_fallthrough_skip_count
            .checked_add(1)
            .is_some()
            && state
                .witnessed_positive_guard_false_fallthrough_skip_count
                .checked_add(1)
                .is_some();
    }
    if predecessor.capacity_limit_guard_false_fallthrough_skipped {
        return state
            .capacity_limit_guard_false_fallthrough_skip_count
            .checked_add(1)
            .is_some()
            && state
                .witnessed_capacity_limit_guard_false_fallthrough_skip_count
                .checked_add(1)
                .is_some();
    }
    if predecessor.capacity_limit_sensible_output_guard_false_fallthrough {
        return state
            .capacity_limit_sensible_output_guard_false_fallthrough_count
            .checked_add(1)
            .is_some()
            && state
                .witnessed_capacity_limit_sensible_output_guard_false_fallthrough_count
                .checked_add(1)
                .is_some();
    }
    if !predecessor.capacity_limit_sensible_output_supply_temperature_assignment_executed {
        return false;
    }
    state
        .capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count
        .checked_add(1)
        .is_some()
        && state
            .source_site_execution_count
            .checked_add(
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER
                    .len(),
            )
            .is_some()
        && state
            .supply_temperature_for_minimum_read_count
            .checked_add(1)
            .is_some()
        && state
            .mixed_air_temperature_for_minimum_read_count
            .checked_add(1)
            .is_some()
        && state
            .source_shaped_two_argument_minimum_evaluation_count
            .checked_add(1)
            .is_some()
        && state
            .supply_temperature_assignment_write_count
            .checked_add(1)
            .is_some()
        && state
            .witnessed_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count
            .checked_add(1)
            .is_some()
}

pub(super) fn completed_supply_temperature_mixed_air_limit_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    witness: Option<
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    >,
) -> bool {
    let state = &unit
        .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit;
    let predecessor = &unit
        .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment;
    state_is_consistent(state, witness, snapshot.system)
        && state.transition_count == predecessor.transition_count
        && state.unit_off_skip_count == predecessor.unit_off_skip_count
        && state.non_cooling_skip_count == predecessor.non_cooling_skip_count
        && state.positive_guard_false_fallthrough_skip_count
            == predecessor.positive_guard_false_fallthrough_skip_count
        && state.capacity_limit_guard_false_fallthrough_skip_count
            == predecessor.capacity_limit_guard_false_fallthrough_skip_count
        && state.capacity_limit_sensible_output_guard_false_fallthrough_count
            == predecessor.capacity_limit_sensible_output_guard_false_fallthrough_count
        && state.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count
            == predecessor
                .capacity_limit_sensible_output_supply_temperature_assignment_count
        && exact_prefix_counter_algebra_matches(unit, state)
        && state
            .latest
            .is_some_and(|latest| snapshots_match_bit_exact(latest, snapshot))
}

fn exact_prefix_counter_algebra_matches(
    unit: &PurchasedAirUnitRuntimeState,
    state: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRuntimeState,
) -> bool {
    let false_fallthroughs =
        state.capacity_limit_sensible_output_guard_false_fallthrough_count;
    let limits =
        state.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count;
    false_fallthroughs.checked_add(limits)
        == Some(
            unit.calc_cooling_positive_supply_capacity_limit_sensible_output_guard
                .capacity_limit_sensible_output_guard_evaluation_count,
        )
        && limits
            == unit
                .calc_cooling_positive_supply_capacity_limit_sensible_output_guard
                .capacity_limit_sensible_output_adjustment_body_entry_count
        && limits
            == unit
                .calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment
                .capacity_limit_sensible_output_maximum_capacity_assignment_count
        && limits
            == unit
                .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment
                .capacity_limit_sensible_output_supply_enthalpy_assignment_count
        && limits
            == unit
                .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment
                .capacity_limit_sensible_output_supply_temperature_assignment_count
}

fn state_is_consistent(
    state: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRuntimeState,
    witness: Option<
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    >,
    expected_system: IdealLoadsAirSystemId,
) -> bool {
    let Some(route_partition) = state
        .unit_off_skip_count
        .checked_add(state.non_cooling_skip_count)
        .and_then(|count| count.checked_add(state.positive_guard_false_fallthrough_skip_count))
        .and_then(|count| {
            count.checked_add(state.capacity_limit_guard_false_fallthrough_skip_count)
        })
        .and_then(|count| {
            count.checked_add(state.capacity_limit_sensible_output_guard_false_fallthrough_count)
        })
        .and_then(|count| {
            count.checked_add(
                state.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count,
            )
        })
    else {
        return false;
    };
    let Some(expected_source_sites) = state
        .capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count
        .checked_mul(
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER
                .len(),
        )
    else {
        return false;
    };
    let limited =
        state.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count;
    let counters_match = state.system == expected_system
        && route_partition == state.transition_count
        && state.source_site_execution_count == expected_source_sites
        && state.supply_temperature_for_minimum_read_count == limited
        && state.mixed_air_temperature_for_minimum_read_count == limited
        && state.source_shaped_two_argument_minimum_evaluation_count == limited
        && state.supply_temperature_assignment_write_count == limited
        && state.witnessed_positive_guard_false_fallthrough_skip_count
            == state.positive_guard_false_fallthrough_skip_count
        && state.witnessed_capacity_limit_guard_false_fallthrough_skip_count
            == state.capacity_limit_guard_false_fallthrough_skip_count
        && state.witnessed_capacity_limit_sensible_output_guard_false_fallthrough_count
            == state.capacity_limit_sensible_output_guard_false_fallthrough_count
        && state
            .witnessed_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count
            == limited;
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
                && cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release(
                    latest,
                )
                && snapshots_match_bit_exact(latest, witness)
        }
        _ => false,
    }
}
