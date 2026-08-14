//! CP321/CP340/CP382 retained owner and predecessor validation.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardSnapshot as Snapshot,
};
use super::snapshot_validation::snapshot_links_to_predecessor;
use crate::ideal_loads::calc::cooling_positive_supply_capacity_limit_sensible_output_guard::{
    completed_direct_cooling_positive_supply_capacity_limit_sensible_output_guard_is_consistent,
    cooling_positive_supply_capacity_limit_sensible_output_guard_snapshots_match_bit_exact as cp340_snapshots_match_bit_exact,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment::{
    cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_committed_latest_snapshot_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_snapshots_match_bit_exact as cp382_snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
    cooling_capacity_zero_flow_reset_snapshot_is_exact_direct_release,
    cooling_positive_supply_capacity_limit_sensible_output_guard_snapshot_is_exact_direct_release,
    cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_snapshot_is_exact_direct_release,
};

pub(super) fn guard_links_to_predecessor(
    guard: Snapshot,
    predecessor: Predecessor,
) -> bool {
    snapshot_links_to_predecessor(guard, predecessor)
}

pub(super) fn direct_predecessor_is_retained_and_complete(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> bool {
    let Some(retained) = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment
        .latest
    else {
        return false;
    };
    let Some(witness) = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_latest_witness(system.id)
    else {
        return false;
    };
    system.id == predecessor.system
        && cp382_snapshots_match_bit_exact(retained, predecessor)
        && cp382_snapshots_match_bit_exact(witness, predecessor)
        && cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_snapshot_is_exact_direct_release(predecessor)
        && cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_committed_latest_snapshot_is_consistent(
            unit,
            system.id,
            predecessor,
            witness,
        )
}

pub(super) fn retained_active_input(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> Option<Option<ActiveInput>> {
    if !predecessor.dehumidification_total_output_assignment_executed {
        return Some(None);
    }

    let cooling_total_output_w = predecessor.cooling_total_output_w?;
    let cp321 = unit.calc_cooling_capacity_zero_flow_reset.latest?;
    let cp340 = unit
        .calc_cooling_positive_supply_capacity_limit_sensible_output_guard
        .latest?;
    let cp340_witness = runtime
        .cooling_positive_supply_capacity_limit_sensible_output_guard_latest_witness(system.id)?;
    let maximum_total_cooling_capacity_w = cp321.maximum_total_cooling_capacity_w?;

    let same_call = [
        (cp321.system, cp321.parent_call_ordinal, cp321.controlled_zone),
        (cp340.system, cp340.parent_call_ordinal, cp340.controlled_zone),
    ]
    .into_iter()
    .all(|(owner_system, ordinal, zone)| {
        owner_system == predecessor.system
            && ordinal == predecessor.parent_call_ordinal
            && zone == predecessor.controlled_zone
    });
    let cp321_is_owner = cp321.cooling_body_entered
        && cp321.cooling_limit_condition_satisfied == Some(true)
        && cp321.maximum_total_cooling_capacity_read
        && maximum_total_cooling_capacity_w.is_finite()
        && maximum_total_cooling_capacity_w >= 0.0
        && cooling_capacity_zero_flow_reset_snapshot_is_exact_direct_release(cp321);
    let cp340_corroborates = cp340.capacity_limit_sensible_output_guard_evaluated
        && cp340.maximum_total_cooling_capacity_read
        && cp340
            .maximum_total_cooling_capacity_w
            .is_some_and(|value| value.to_bits() == maximum_total_cooling_capacity_w.to_bits())
        && cooling_positive_supply_capacity_limit_sensible_output_guard_snapshot_is_exact_direct_release(cp340)
        && cp340_snapshots_match_bit_exact(cp340, cp340_witness)
        && completed_direct_cooling_positive_supply_capacity_limit_sensible_output_guard_is_consistent(
            runtime,
            unit,
            system,
            cp340,
            Some(cp340_witness),
        );
    if !same_call || !cp321_is_owner || !cp340_corroborates {
        return None;
    }

    Some(Some(ActiveInput {
        cooling_total_output_w,
        maximum_total_cooling_capacity_w,
        cp382_cooling_total_output_owned_read: true,
        cp321_maximum_total_cooling_capacity_owned_read: true,
        cp340_same_call_maximum_total_cooling_capacity_bit_corroborated: true,
    }))
}
