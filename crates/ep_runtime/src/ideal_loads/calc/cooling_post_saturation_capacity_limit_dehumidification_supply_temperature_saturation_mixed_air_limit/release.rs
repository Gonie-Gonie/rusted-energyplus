//! Release-bound CP415 saturation-temperature mixed-air limit.

use ep_model::{
    DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem,
    IdealLoadsAirSystemId,
};

use super::transition::{predecessor_route, RetainedRoute};
use super::{
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_state as advance,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitSnapshot as Snapshot,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_MIXED_AIR_LIMIT_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_MIXED_AIR_LIMIT_SOURCE_ORDER as ORDER,
};
use crate::ideal_loads::calc::completed_direct_cooling_mixed_air_call_is_consistent;
use crate::ideal_loads::{
    classify_no_oa_sensible_subset, cooling_mixed_air_call_snapshot_is_exact_direct_release,
    cooling_mixed_air_call_snapshots_match_bit_exact,
    cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_snapshot_is_exact,
    cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_snapshot_is_exact_direct_release,
    cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_snapshots_match_bit_exact,
    PurchasedAirCalcCoolingMixedAirCallSnapshot as MixedAirOwner,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
};

mod error;
mod runtime_validation;
mod snapshot;
pub use error::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitError;
use runtime_validation::{
    completed_predecessor_counts_match, pending_predecessor_counts_match,
    state_counts_are_consistent,
};
use snapshot::{cp414_shape, option_bits_match, snapshots_match_bit_exact};

/// Executes CP415 for the exact direct no-OA release route.
pub fn advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp414: Predecessor,
) -> Result<
    Snapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitError,
>{
    use PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitError as Error;

    let selected = predecessor_cp414.system;
    let unit = runtime
        .units
        .get(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    let witness = runtime.cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_latest_witness(selected);
    let predecessor_witness = runtime.cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_latest_witness(selected);
    if system.id != selected {
        return Err(Error::SystemIdentityMismatch {
            expected: selected,
            actual: system.id,
        });
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(Error::SystemOutsideDirectSubset { system: selected });
    }
    if system.dehumidification_control_type != DehumidificationControlType::None {
        return Err(Error::DehumidificationControlTypeOutsideDirectSubset {
            system: selected,
            actual: system.dehumidification_control_type,
        });
    }
    if system.humidification_control_type != HumidificationControlType::None {
        return Err(Error::HumidificationControlTypeOutsideDirectSubset {
            system: selected,
            actual: system.humidification_control_type,
        });
    }
    if !unit.topology_completed
        || unit.topology_failure.is_some()
        || unit.init_call_count == 0
        || unit.calc_entry.latest.is_none()
    {
        return Err(Error::InitializationNotReady { system: selected });
    }
    let controlled_zone = unit
        .controlled_zone
        .ok_or(Error::InitializationNotReady { system: selected })?;
    if predecessor_cp414.controlled_zone != controlled_zone {
        return Err(predecessor_mismatch(selected));
    }
    let Some(retained_predecessor) = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment
        .latest
    else {
        return Err(predecessor_mismatch(selected));
    };
    if !cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_snapshots_match_bit_exact(
        predecessor_cp414,
        retained_predecessor,
    ) || !predecessor_witness.is_some_and(|predecessor_witness| {
        cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_snapshots_match_bit_exact(
            predecessor_cp414,
            predecessor_witness,
        )
    }) || !cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_snapshot_is_exact_direct_release(
        predecessor_cp414,
    ) || !crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_latest_metadata_is_consistent(
        unit,
        unit.init_call_count,
    ) {
        return Err(predecessor_mismatch(selected));
    }
    let route = predecessor_route(predecessor_cp414)
        .ok_or(Error::PredecessorOutsideDirectSubset { system: selected })?;
    let mixed_air_owner = if route.active {
        let supply_temperature = predecessor_cp414
            .resulting_supply_temperature_c
            .ok_or(predecessor_mismatch(selected))?;
        if !supply_temperature.is_finite() {
            return Err(Error::SupplyTemperatureOutsideDirectSubset {
                system: selected,
                bits: supply_temperature.to_bits(),
            });
        }
        let owner = active_mixed_air_owner(runtime, unit, system, predecessor_cp414)
            .ok_or(Error::CoolingMixedAirCallSnapshotMismatch { system: selected })?;
        let mixed_air_temperature = owner
            .mixed_air_temperature_c
            .ok_or(Error::CoolingMixedAirCallSnapshotMismatch { system: selected })?;
        if !mixed_air_temperature.is_finite() {
            return Err(Error::MixedAirTemperatureOutsideDirectSubset {
                system: selected,
                bits: mixed_air_temperature.to_bits(),
            });
        }
        Some(owner)
    } else {
        None
    };
    let state = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit;
    if !pending_state_is_consistent(unit, state, witness)
        || !pending_predecessor_counts_match(
            state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment,
            route,
        )
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }
    let Some(expected_predecessor_transition_count) = state.transition_count.checked_add(1) else {
        return Err(call_order_error(unit, selected));
    };
    if unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment
        .transition_count
        != expected_predecessor_transition_count
        || predecessor_cp414.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }

    let mut next_state = state.clone();
    let snapshot = advance(&mut next_state, predecessor_cp414, mixed_air_owner)
        .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    if !snapshot_is_exact(snapshot)
        || !direct_subset_values_are_valid(snapshot)
        || !completed_state_is_consistent(&next_state, snapshot)
        || !completed_predecessor_counts_match(
            &next_state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment,
        )
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }

    let unit = runtime
        .units
        .get_mut(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit = next_state;
    runtime.set_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_latest_witness(selected, snapshot);
    Ok(snapshot)
}

pub(in crate::ideal_loads) fn private_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_characterization(
    predecessor: Predecessor,
    mixed_air_owner: Option<MixedAirOwner>,
) -> Option<Snapshot> {
    let mut state = State::new(predecessor.system);
    advance(&mut state, predecessor, mixed_air_owner)
}

pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_snapshot_is_exact_direct_release(
    snapshot: Snapshot,
) -> bool {
    cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_snapshot_is_exact_direct_release(
        cp414_shape(snapshot),
    ) && snapshot_is_exact(snapshot)
        && direct_subset_values_are_valid(snapshot)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_snapshot_is_exact(
    snapshot: Snapshot,
) -> bool {
    snapshot_is_exact(snapshot)
}

fn snapshot_is_exact(snapshot: Snapshot) -> bool {
    if snapshot.source != SOURCE
        || snapshot.first_excluded_source != EXCLUDED
        || snapshot.source_order != ORDER
    {
        return false;
    }
    let predecessor = cp414_shape(snapshot);
    if !cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_snapshot_is_exact(predecessor)
    {
        return false;
    }
    let Some(route) = predecessor_route(predecessor) else {
        return false;
    };
    let terminal_prefix_matches = option_bits_match(
        snapshot.predecessor_cp414_resulting_supply_humidity_ratio,
        predecessor.resulting_supply_humidity_ratio,
    ) && option_bits_match(
        snapshot.predecessor_cp414_resulting_supply_enthalpy_j_per_kg,
        predecessor.resulting_supply_enthalpy_j_per_kg,
    ) && option_bits_match(
        snapshot.predecessor_cp414_resulting_supply_temperature_c,
        predecessor.resulting_supply_temperature_c,
    ) && option_bits_match(
        snapshot.preexisting_supply_temperature_c,
        predecessor.resulting_supply_temperature_c,
    ) && option_bits_match(
        snapshot.resulting_supply_humidity_ratio,
        predecessor.resulting_supply_humidity_ratio,
    ) && option_bits_match(
        snapshot.resulting_supply_enthalpy_j_per_kg,
        predecessor.resulting_supply_enthalpy_j_per_kg,
    );
    if !terminal_prefix_matches
        || snapshot
            .post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_executed
            != route.active
        || snapshot.cp414_retained_supply_temperature_state_owned
            != predecessor.resulting_supply_temperature_c.is_some()
    {
        return false;
    }
    if !route.active {
        return !snapshot.cp414_retained_supply_temperature_owned_read
            && !snapshot.supply_temperature_for_minimum_read
            && snapshot
                .supply_temperature_before_mixed_air_limit_c
                .is_none()
            && !snapshot.cp329_retained_mixed_air_temperature_owned_read
            && !snapshot.mixed_air_temperature_for_minimum_read
            && snapshot.mixed_air_temperature_c.is_none()
            && !snapshot.source_shaped_two_argument_minimum_evaluated
            && snapshot.minimum_supply_temperature_c.is_none()
            && !snapshot.supply_temperature_assignment_performed
            && snapshot.assigned_supply_temperature_c.is_none()
            && option_bits_match(
                snapshot.resulting_supply_temperature_c,
                predecessor.resulting_supply_temperature_c,
            );
    }
    let (Some(left), Some(right), Some(minimum), Some(assigned), Some(resulting)) = (
        snapshot.supply_temperature_before_mixed_air_limit_c,
        snapshot.mixed_air_temperature_c,
        snapshot.minimum_supply_temperature_c,
        snapshot.assigned_supply_temperature_c,
        snapshot.resulting_supply_temperature_c,
    ) else {
        return false;
    };
    let expected = super::transition::source_shaped_two_argument_minimum(left, right);
    snapshot.cp414_retained_supply_temperature_owned_read
        && snapshot.supply_temperature_for_minimum_read
        && snapshot.cp329_retained_mixed_air_temperature_owned_read
        && snapshot.mixed_air_temperature_for_minimum_read
        && snapshot.source_shaped_two_argument_minimum_evaluated
        && snapshot.supply_temperature_assignment_performed
        && predecessor
            .resulting_supply_temperature_c
            .is_some_and(|value| value.to_bits() == left.to_bits())
        && minimum.to_bits() == expected.to_bits()
        && assigned.to_bits() == expected.to_bits()
        && resulting.to_bits() == expected.to_bits()
}

fn direct_subset_values_are_valid(snapshot: Snapshot) -> bool {
    !snapshot
        .post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_executed
        || (snapshot
            .supply_temperature_before_mixed_air_limit_c
            .is_some_and(f64::is_finite)
            && snapshot
                .mixed_air_temperature_c
                .is_some_and(f64::is_finite)
            && snapshot
                .minimum_supply_temperature_c
                .is_some_and(f64::is_finite))
}

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_snapshot_route(
    snapshot: Snapshot,
) -> Option<RetainedRoute> {
    snapshot_is_exact(snapshot)
        .then(|| predecessor_route(cp414_shape(snapshot)))
        .flatten()
}

pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_snapshots_match_bit_exact(
    left: Snapshot,
    right: Snapshot,
) -> bool {
    snapshots_match_bit_exact(left, right)
}

fn pending_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    state: &State,
    witness: Option<Snapshot>,
) -> bool {
    state.system == unit.system
        && state_counts_are_consistent(state)
        && state.transition_count
            == unit
                .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment
                .transition_count
                .saturating_sub(1)
        && match (state.latest, witness) {
            (Some(latest), Some(witness)) => {
                snapshots_match_bit_exact(latest, witness)
                    && state.latest_transition_ordinal == Some(state.transition_count)
                    && state.latest_route == predecessor_route(cp414_shape(latest))
            }
            (None, None) => state.transition_count == 0,
            _ => false,
        }
}

fn completed_state_is_consistent(state: &State, snapshot: Snapshot) -> bool {
    state
        .latest
        .is_some_and(|latest| snapshots_match_bit_exact(latest, snapshot))
        && state.latest_transition_ordinal == Some(state.transition_count)
        && state.latest_route == predecessor_route(cp414_shape(snapshot))
        && state_counts_are_consistent(state)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_latest_metadata_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    expected_transition_count: usize,
) -> bool {
    let state = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit;
    state.transition_count == expected_transition_count
        && state.transition_count
            == unit
                .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment
                .transition_count
        && state
            .latest
            .is_some_and(|latest| completed_state_is_consistent(state, latest))
        && completed_predecessor_counts_match(
            state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment,
        )
}

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn completed_direct_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    if unit.system != system.id
        || snapshot.system != system.id
        || unit.controlled_zone != Some(snapshot.controlled_zone)
        || !snapshot_is_exact(snapshot)
        || !direct_subset_values_are_valid(snapshot)
        || !witness.is_some_and(|witness| snapshots_match_bit_exact(snapshot, witness))
        || !cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_latest_metadata_is_consistent(
            unit,
            unit.init_call_count,
        )
    {
        return false;
    }
    let predecessor = cp414_shape(snapshot);
    let predecessor_witness = runtime.cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_latest_witness(system.id);
    let retained_predecessor = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment
        .latest;
    if !retained_predecessor.is_some_and(|retained| {
        cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_snapshots_match_bit_exact(
            retained,
            predecessor,
        )
    }) || !predecessor_witness.is_some_and(|witness| {
        cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_snapshots_match_bit_exact(
            witness,
            predecessor,
        )
    }) {
        return false;
    }
    let Some(route) = predecessor_route(predecessor) else {
        return false;
    };
    !route.active
        || active_mixed_air_owner(runtime, unit, system, predecessor).is_some_and(|owner| {
            option_bits_match(
                snapshot.mixed_air_temperature_c,
                owner.mixed_air_temperature_c,
            )
        })
}

fn active_mixed_air_owner(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> Option<MixedAirOwner> {
    let owner = unit.calc_cooling_mixed_air_call.latest?;
    let witness = runtime.cooling_mixed_air_call_latest_witness(system.id)?;
    if owner.system != predecessor.system
        || owner.parent_call_ordinal != predecessor.parent_call_ordinal
        || owner.controlled_zone != predecessor.controlled_zone
        || !owner.mixed_air_temperature_assigned
        || !cooling_mixed_air_call_snapshot_is_exact_direct_release(owner)
        || !cooling_mixed_air_call_snapshots_match_bit_exact(owner, witness)
        || !completed_direct_cooling_mixed_air_call_is_consistent(
            runtime,
            unit,
            system,
            owner,
            Some(witness),
        )
    {
        return None;
    }
    Some(owner)
}

fn predecessor_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitError{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitError::CoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentSnapshotMismatch { system }
}

fn call_order_error(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitError{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        predecessor_transition_count: unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment.transition_count,
        transition_count: unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit.transition_count,
    }
}
