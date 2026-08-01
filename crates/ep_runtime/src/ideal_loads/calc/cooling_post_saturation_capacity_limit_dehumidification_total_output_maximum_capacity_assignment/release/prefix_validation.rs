//! Retained CP383 owner and direct-predecessor validation.

use ep_model::IdealLoadsAirSystem;

use super::super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentSnapshot as Snapshot;
use super::snapshot_validation::snapshot_links_to_predecessor;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_guard::{
    completed_direct_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_snapshots_match_bit_exact as cp383_snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
    cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_snapshot_is_exact_direct_release,
};

pub(super) fn assignment_links_to_predecessor(
    assignment: Snapshot,
    predecessor: Predecessor,
) -> bool {
    snapshot_links_to_predecessor(assignment, predecessor)
}

pub(super) fn direct_predecessor_is_retained_and_complete(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    supplied: Predecessor,
) -> bool {
    let Some(retained) = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard
        .latest
    else {
        return false;
    };
    let Some(witness) = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_latest_witness(
            system.id,
        )
    else {
        return false;
    };
    system.id == supplied.system
        && cp383_snapshots_match_bit_exact(retained, supplied)
        && cp383_snapshots_match_bit_exact(witness, supplied)
        && cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_snapshot_is_exact_direct_release(supplied)
        && completed_direct_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_is_consistent(
            runtime,
            unit,
            system,
            retained,
            Some(witness),
        )
}

pub(super) fn retained_operand_is_admissible(predecessor: Predecessor) -> bool {
    if !predecessor.dehumidification_total_output_capacity_adjustment_body_entered {
        return true;
    }
    predecessor.cp321_maximum_total_cooling_capacity_owned_read
        && predecessor.maximum_total_cooling_capacity_read
        && predecessor
            .maximum_total_cooling_capacity_w
            .is_some_and(|value| value.is_finite() && value > 0.0)
}
