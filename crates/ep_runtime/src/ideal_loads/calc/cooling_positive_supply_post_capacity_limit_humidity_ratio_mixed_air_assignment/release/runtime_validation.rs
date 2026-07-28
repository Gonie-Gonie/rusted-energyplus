//! Persistent CP345 runtime-state validation.

use ep_model::IdealLoadsAirSystemId;

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentActiveInput,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
};
use super::prefix_validation::predecessor_is_active;
use super::snapshot_validation::{
    cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release,
    snapshot_route, snapshots_match_bit_exact,
};
use crate::ideal_loads::{
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
        && unit.calc_cooling_supply_mass_flow_positive_guard.system == system
        && unit
            .calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment
            .system
            == system
        && unit
            .calc_cooling_positive_supply_enthalpy_assignment
            .system
            == system
        && unit
            .calc_cooling_positive_supply_capacity_limit_guard
            .system
            == system
        && unit
            .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit
            .system
            == system
        && unit
            .calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment
            .system
            == system
}

pub(super) fn call_order_is_pending(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
) -> bool {
    let ordinal = predecessor.parent_call_ordinal;
    unit.calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment
        .transition_count
        .checked_add(1)
        == Some(ordinal)
        && unit.calc_entry.call_count == ordinal
        && unit
            .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit
            .transition_count
            == ordinal
}

pub(in crate::ideal_loads::calc) fn pending_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    witness: Option<
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
    >,
) -> bool {
    let state =
        &unit.calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment;
    let predecessor_state = &unit
        .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit;
    let active = predecessor_is_active(predecessor);
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
            .assignment_after_capacity_limit_guard_false_fallthrough_count
            .checked_add(usize::from(
                predecessor.capacity_limit_guard_false_fallthrough_skipped,
            ))
            == Some(predecessor_state.capacity_limit_guard_false_fallthrough_skip_count)
        && state
            .assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count
            .checked_add(usize::from(
                predecessor.capacity_limit_sensible_output_guard_false_fallthrough,
            ))
            == Some(
                predecessor_state
                    .capacity_limit_sensible_output_guard_false_fallthrough_count,
            )
        && state
            .assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count
            .checked_add(usize::from(
                predecessor
                    .capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed,
            ))
            == Some(
                predecessor_state
                    .capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count,
            )
        && state
            .post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count
            .checked_add(usize::from(active))
            .is_some_and(|after| {
                pending_active_counter_algebra_matches(unit, predecessor, after)
            })
}

pub(in crate::ideal_loads::calc) fn next_transition_fits(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    active_input: Option<
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentActiveInput,
    >,
) -> bool {
    let state =
        &unit.calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment;
    let g = predecessor.capacity_limit_guard_false_fallthrough_skipped;
    let f = predecessor.capacity_limit_sensible_output_guard_false_fallthrough;
    let l = predecessor.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed;
    let active = g || f || l;
    if state.transition_count.checked_add(1).is_none()
        || active != active_input.is_some()
        || usize::from(g) + usize::from(f) + usize::from(l) != usize::from(active)
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
    if !active
        || state
            .post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count
            .checked_add(1)
            .is_none()
        || state
            .source_site_execution_count
            .checked_add(
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER
                    .len(),
            )
            .is_none()
        || state
            .mixed_air_humidity_ratio_read_count
            .checked_add(1)
            .is_none()
        || state
            .supply_humidity_ratio_assignment_count
            .checked_add(1)
            .is_none()
        || state
            .witnessed_post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count
            .checked_add(1)
            .is_none()
    {
        return false;
    }
    if g {
        state
            .assignment_after_capacity_limit_guard_false_fallthrough_count
            .checked_add(1)
            .is_some()
            && state
                .witnessed_assignment_after_capacity_limit_guard_false_fallthrough_count
                .checked_add(1)
                .is_some()
    } else if f {
        state
            .assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count
            .checked_add(1)
            .is_some()
            && state
                .witnessed_assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count
                .checked_add(1)
                .is_some()
    } else {
        state
            .assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count
            .checked_add(1)
            .is_some()
            && state
                .witnessed_assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count
                .checked_add(1)
                .is_some()
    }
}

pub(super) fn completed_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
    witness: Option<
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
    >,
) -> bool {
    let state =
        &unit.calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment;
    let predecessor = &unit
        .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit;
    state_is_consistent(state, witness, snapshot.system)
        && state.transition_count == predecessor.transition_count
        && state.unit_off_skip_count == predecessor.unit_off_skip_count
        && state.non_cooling_skip_count == predecessor.non_cooling_skip_count
        && state.positive_guard_false_fallthrough_skip_count
            == predecessor.positive_guard_false_fallthrough_skip_count
        && state.assignment_after_capacity_limit_guard_false_fallthrough_count
            == predecessor.capacity_limit_guard_false_fallthrough_skip_count
        && state
            .assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count
            == predecessor.capacity_limit_sensible_output_guard_false_fallthrough_count
        && state
            .assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count
            == predecessor
                .capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count
        && exact_active_counter_algebra_matches(
            unit,
            state.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count,
        )
        && state
            .latest
            .is_some_and(|latest| snapshots_match_bit_exact(latest, snapshot))
}

fn exact_active_counter_algebra_matches(
    unit: &PurchasedAirUnitRuntimeState,
    active: usize,
) -> bool {
    let state =
        &unit.calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment;
    active_counter_algebra_matches(
        unit,
        active,
        state.assignment_after_capacity_limit_guard_false_fallthrough_count,
        state.assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count,
        state
            .assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count,
    )
}

fn pending_active_counter_algebra_matches(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    active: usize,
) -> bool {
    let state =
        &unit.calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment;
    let Some(g) = state
        .assignment_after_capacity_limit_guard_false_fallthrough_count
        .checked_add(usize::from(
            predecessor.capacity_limit_guard_false_fallthrough_skipped,
        ))
    else {
        return false;
    };
    let Some(f) = state
        .assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count
        .checked_add(usize::from(
            predecessor.capacity_limit_sensible_output_guard_false_fallthrough,
        ))
    else {
        return false;
    };
    let Some(l) = state
        .assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count
        .checked_add(usize::from(
            predecessor.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed,
        ))
    else {
        return false;
    };
    active_counter_algebra_matches(unit, active, g, f, l)
}

fn active_counter_algebra_matches(
    unit: &PurchasedAirUnitRuntimeState,
    active: usize,
    g: usize,
    f: usize,
    l: usize,
) -> bool {
    let predecessor = &unit
        .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit;
    let predecessor_active = predecessor
        .capacity_limit_guard_false_fallthrough_skip_count
        .checked_add(predecessor.capacity_limit_sensible_output_guard_false_fallthrough_count)
        .and_then(|count| {
            count.checked_add(
                predecessor.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count,
            )
        });
    let capacity_body = f.checked_add(l);
    g.checked_add(f).and_then(|count| count.checked_add(l)) == Some(active)
        && predecessor_active == Some(active)
        && active
            == unit
                .calc_cooling_supply_mass_flow_positive_guard
                .positive_supply_mass_flow_body_entry_count
        && active
            == unit
                .calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment
                .supply_humidity_ratio_mixed_air_assignment_count
        && active
            == unit
                .calc_cooling_positive_supply_enthalpy_assignment
                .supply_enthalpy_assignment_count
        && active
            == unit
                .calc_cooling_positive_supply_capacity_limit_guard
                .capacity_limit_guard_evaluation_count
        && g == unit
            .calc_cooling_positive_supply_capacity_limit_guard
            .active_guard_false_fallthrough_count
        && capacity_body
            == Some(
                unit.calc_cooling_positive_supply_capacity_limit_guard
                    .capacity_limit_body_entry_count,
            )
}

fn state_is_consistent(
    state: &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentRuntimeState,
    witness: Option<
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
    >,
    expected_system: IdealLoadsAirSystemId,
) -> bool {
    let Some(route_partition) = state
        .unit_off_skip_count
        .checked_add(state.non_cooling_skip_count)
        .and_then(|count| count.checked_add(state.positive_guard_false_fallthrough_skip_count))
        .and_then(|count| {
            count.checked_add(
                state.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count,
            )
        })
    else {
        return false;
    };
    let active = state.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count;
    let Some(provenance_partition) = state
        .assignment_after_capacity_limit_guard_false_fallthrough_count
        .checked_add(
            state.assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count,
        )
        .and_then(|count| {
            count.checked_add(
                state
                    .assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count,
            )
        })
    else {
        return false;
    };
    let Some(expected_source_sites) = active.checked_mul(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER
            .len(),
    ) else {
        return false;
    };
    let counters_match = state.system == expected_system
        && route_partition == state.transition_count
        && provenance_partition == active
        && state.source_site_execution_count == expected_source_sites
        && state.mixed_air_humidity_ratio_read_count == active
        && state.supply_humidity_ratio_assignment_count == active
        && state.witnessed_positive_guard_false_fallthrough_skip_count
            == state.positive_guard_false_fallthrough_skip_count
        && state
            .witnessed_post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count
            == active
        && state.witnessed_assignment_after_capacity_limit_guard_false_fallthrough_count
            == state.assignment_after_capacity_limit_guard_false_fallthrough_count
        && state
            .witnessed_assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count
            == state
                .assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count
        && state
            .witnessed_assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count
            == state
                .assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count;
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
                && cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release(
                    latest,
                )
                && snapshots_match_bit_exact(latest, witness)
        }
        _ => false,
    }
}
