//! CP329/CP378/CP379/CP380-to-CP381 retained owner and predecessor validation.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardSnapshot as Snapshot,
};
use super::snapshot_validation::snapshot_links_to_predecessor;
use crate::ideal_loads::calc::cooling_mixed_air_call::{
    cooling_mixed_air_call_committed_latest_mixed_air_humidity_ratio,
    cooling_mixed_air_call_snapshots_match_bit_exact,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_guard::{
    completed_direct_cooling_post_saturation_capacity_limit_guard_is_consistent,
    cooling_post_saturation_capacity_limit_guard_snapshots_match_exact as cp380_snapshots_match_exact,
};
use crate::ideal_loads::calc::cooling_supply_enthalpy_post_saturation_assignment::{
    cooling_supply_enthalpy_post_saturation_assignment_committed_latest_snapshot_is_consistent,
    cooling_supply_enthalpy_post_saturation_assignment_snapshots_match_bit_exact as cp379_snapshots_match_bit_exact,
};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_saturation_limit_assignment::{
    cooling_supply_humidity_ratio_saturation_limit_assignment_committed_latest_snapshot_is_consistent,
    cooling_supply_humidity_ratio_saturation_limit_assignment_snapshots_match_bit_exact as cp378_snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
    cooling_mixed_air_call_snapshot_is_exact_direct_release,
    cooling_post_saturation_capacity_limit_guard_snapshot_is_exact_direct_release,
    cooling_supply_enthalpy_post_saturation_assignment_snapshot_is_exact_direct_release,
    cooling_supply_humidity_ratio_saturation_limit_assignment_snapshot_is_exact_direct_release,
};

pub(super) fn guard_links_to_predecessor(guard: Snapshot, predecessor: Predecessor) -> bool {
    snapshot_links_to_predecessor(guard, predecessor)
}

pub(super) fn direct_predecessor_is_retained_and_complete(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> bool {
    let Some(retained) = unit
        .calc_cooling_post_saturation_capacity_limit_guard
        .latest
    else {
        return false;
    };
    let Some(witness) =
        runtime.cooling_post_saturation_capacity_limit_guard_latest_witness(system.id)
    else {
        return false;
    };
    system.id == predecessor.system
        && cp380_snapshots_match_exact(retained, predecessor)
        && cp380_snapshots_match_exact(witness, predecessor)
        && cooling_post_saturation_capacity_limit_guard_snapshot_is_exact_direct_release(
            predecessor,
        )
        && completed_direct_cooling_post_saturation_capacity_limit_guard_is_consistent(
            runtime,
            unit,
            system,
            predecessor,
            Some(witness),
        )
}

pub(super) fn retained_mixed_air_owner_is_valid(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> bool {
    let Some(cp329) = unit.calc_cooling_mixed_air_call.latest else {
        return false;
    };
    let Some(witness) = runtime.cooling_mixed_air_call_latest_witness(system.id) else {
        return false;
    };
    cp329.system == predecessor.system
        && cp329.parent_call_ordinal == predecessor.parent_call_ordinal
        && cp329.controlled_zone == predecessor.controlled_zone
        && cp329.cooling_call_executed
        && cp329.mixed_air_humidity_ratio_assigned
        && cp329.mixed_air_humidity_ratio.is_some_and(f64::is_finite)
        && cooling_mixed_air_call_snapshots_match_bit_exact(cp329, witness)
        && cooling_mixed_air_call_snapshot_is_exact_direct_release(cp329)
        && cooling_mixed_air_call_committed_latest_mixed_air_humidity_ratio(unit, witness)
            .is_some_and(|committed| {
                cp329
                    .mixed_air_humidity_ratio
                    .is_some_and(|retained| retained.to_bits() == committed.to_bits())
            })
}

pub(super) fn retained_active_input(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> Option<ActiveInput> {
    if !predecessor.capacity_limit_body_entered {
        return None;
    }
    let cp329 = unit.calc_cooling_mixed_air_call.latest?;
    let cp329_witness = runtime.cooling_mixed_air_call_latest_witness(system.id)?;
    let cp378 = unit
        .calc_cooling_supply_humidity_ratio_saturation_limit_assignment
        .latest?;
    let cp378_witness = runtime
        .cooling_supply_humidity_ratio_saturation_limit_assignment_latest_witness(system.id)?;
    let cp379 = unit
        .calc_cooling_supply_enthalpy_post_saturation_assignment
        .latest?;
    let cp379_witness =
        runtime.cooling_supply_enthalpy_post_saturation_assignment_latest_witness(system.id)?;
    let supply = cp378.resulting_supply_humidity_ratio?;
    let mixed = cp329.mixed_air_humidity_ratio?;

    let same_call = [
        (
            cp329.system,
            cp329.parent_call_ordinal,
            cp329.controlled_zone,
        ),
        (
            cp378.system,
            cp378.parent_call_ordinal,
            cp378.controlled_zone,
        ),
        (
            cp379.system,
            cp379.parent_call_ordinal,
            cp379.controlled_zone,
        ),
    ]
    .into_iter()
    .all(|(owner_system, ordinal, zone)| {
        owner_system == predecessor.system
            && ordinal == predecessor.parent_call_ordinal
            && zone == predecessor.controlled_zone
    });
    let supply_route_matches = route_flags_cp378(cp378) == route_flags_cp380(predecessor)
        && route_flags_cp379(cp379) == route_flags_cp380(predecessor);
    if !same_call
        || !supply_route_matches
        || !cp329.cooling_call_executed
        || !cp329.mixed_air_humidity_ratio_assigned
        || !cp378.purchased_air_supply_humidity_ratio_saturation_limit_assignment_performed
        || !cp379.local_supply_enthalpy_after_saturation_limit_assignment_performed
        || !cp379.cp378_supply_humidity_ratio_saturation_limit_owned_read
        || !option_bits_match(cp379.predecessor_resulting_supply_humidity_ratio, Some(supply))
        || !option_bits_match(cp379.supply_humidity_ratio, Some(supply))
        || !supply.is_finite()
        || supply < 0.0
        || !mixed.is_finite()
        || !cp378_snapshots_match_bit_exact(cp378, cp378_witness)
        || !cp379_snapshots_match_bit_exact(cp379, cp379_witness)
        || !cooling_mixed_air_call_snapshots_match_bit_exact(cp329, cp329_witness)
        || !cooling_supply_humidity_ratio_saturation_limit_assignment_snapshot_is_exact_direct_release(cp378)
        || !cooling_supply_enthalpy_post_saturation_assignment_snapshot_is_exact_direct_release(cp379)
        || !cooling_mixed_air_call_snapshot_is_exact_direct_release(cp329)
        || !cooling_supply_humidity_ratio_saturation_limit_assignment_committed_latest_snapshot_is_consistent(
            unit, cp378_witness,
        )
        || !cooling_supply_enthalpy_post_saturation_assignment_committed_latest_snapshot_is_consistent(
            unit, cp379_witness,
        )
        || cooling_mixed_air_call_committed_latest_mixed_air_humidity_ratio(
            unit, cp329_witness,
        )
        .is_none_or(|committed| committed.to_bits() != mixed.to_bits())
    {
        return None;
    }
    Some(ActiveInput {
        supply_humidity_ratio: supply,
        mixed_air_humidity_ratio: mixed,
        cp378_supply_humidity_ratio_saturation_limit_owned_read: true,
        cp379_same_call_supply_humidity_ratio_bit_corroborated: true,
        cp329_mixed_air_humidity_ratio_owned_read: true,
    })
}

fn route_flags_cp380(snapshot: Predecessor) -> [bool; 8] {
    [
        snapshot.unit_off_skipped,
        snapshot.non_cooling_skipped,
        snapshot.positive_guard_false_fallthrough_skipped,
        snapshot.heating_availability_guard_false_fallthrough,
        snapshot.humidification_control_guard_false_fallthrough,
        snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        snapshot.dehumidification_control_none_maximum_assignment_executed,
        snapshot.dehumidification_control_guard_false_fallthrough,
    ]
}

fn route_flags_cp378(
    snapshot: crate::ideal_loads::PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot,
) -> [bool; 8] {
    [
        snapshot.unit_off_skipped,
        snapshot.non_cooling_skipped,
        snapshot.positive_guard_false_fallthrough_skipped,
        snapshot.heating_availability_guard_false_fallthrough,
        snapshot.humidification_control_guard_false_fallthrough,
        snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        snapshot.dehumidification_control_none_maximum_assignment_executed,
        snapshot.dehumidification_control_guard_false_fallthrough,
    ]
}

fn route_flags_cp379(
    snapshot: crate::ideal_loads::PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot,
) -> [bool; 8] {
    [
        snapshot.unit_off_skipped,
        snapshot.non_cooling_skipped,
        snapshot.positive_guard_false_fallthrough_skipped,
        snapshot.heating_availability_guard_false_fallthrough,
        snapshot.humidification_control_guard_false_fallthrough,
        snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        snapshot.dehumidification_control_none_maximum_assignment_executed,
        snapshot.dehumidification_control_guard_false_fallthrough,
    ]
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
