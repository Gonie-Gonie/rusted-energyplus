//! CP378 immediate-predecessor and transitive original-owner validation.

use ep_model::IdealLoadsAirSystem;

use super::super::PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot as Snapshot;
use super::snapshot_validation::snapshot_links_to_predecessor;
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_pre_saturation_original_assignment::{
    completed_direct_cooling_supply_humidity_ratio_pre_saturation_original_assignment_is_consistent,
    cooling_supply_humidity_ratio_pre_saturation_original_assignment_snapshots_match_bit_exact as cp376_snapshots_match_bit_exact,
};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_saturation_assignment::{
    completed_direct_cooling_supply_humidity_ratio_saturation_assignment_is_consistent,
    cooling_supply_humidity_ratio_saturation_assignment_snapshots_match_bit_exact as cp377_snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
    cooling_supply_humidity_ratio_pre_saturation_original_assignment_snapshot_is_exact_direct_release,
    cooling_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release,
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
    predecessor: Predecessor,
) -> bool {
    let Some(retained) = unit
        .calc_cooling_supply_humidity_ratio_saturation_assignment
        .latest
    else {
        return false;
    };
    let Some(witness) =
        runtime.cooling_supply_humidity_ratio_saturation_assignment_latest_witness(system.id)
    else {
        return false;
    };
    system.id == predecessor.system
        && cp377_snapshots_match_bit_exact(retained, predecessor)
        && cp377_snapshots_match_bit_exact(witness, predecessor)
        && cooling_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release(
            predecessor,
        )
        && completed_direct_cooling_supply_humidity_ratio_saturation_assignment_is_consistent(
            runtime,
            unit,
            system,
            predecessor,
            Some(witness),
        )
}

pub(super) fn direct_original_owner_is_retained_and_complete(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> bool {
    let Some(original) = unit
        .calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment
        .latest
    else {
        return false;
    };
    let Some(witness) = runtime
        .cooling_supply_humidity_ratio_pre_saturation_original_assignment_latest_witness(system.id)
    else {
        return false;
    };
    same_call(original.system, original.parent_call_ordinal, original.controlled_zone, predecessor)
        && cp376_snapshots_match_bit_exact(original, witness)
        && cooling_supply_humidity_ratio_pre_saturation_original_assignment_snapshot_is_exact_direct_release(
            original,
        )
        && completed_direct_cooling_supply_humidity_ratio_pre_saturation_original_assignment_is_consistent(
            runtime,
            unit,
            system,
            original,
            Some(witness),
        )
        && predecessor.predecessor_local_supply_humidity_ratio_original_assignment_performed
            == original.local_supply_humidity_ratio_original_assignment_performed
        && option_bits_match(
            predecessor.predecessor_resulting_supply_humidity_ratio_original,
            original.resulting_supply_humidity_ratio_original,
        )
}

fn same_call(
    system: ep_model::IdealLoadsAirSystemId,
    ordinal: usize,
    zone: ep_model::ZoneId,
    predecessor: Predecessor,
) -> bool {
    system == predecessor.system
        && ordinal == predecessor.parent_call_ordinal
        && zone == predecessor.controlled_zone
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
