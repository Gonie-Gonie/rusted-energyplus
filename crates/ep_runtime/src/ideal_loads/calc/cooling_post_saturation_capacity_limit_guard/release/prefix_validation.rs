//! CP337/CP379-to-CP380 retained lineage validation.

use ep_model::IdealLoadsAirSystem;

use super::super::PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardSnapshot as Snapshot;
use super::snapshot_validation::snapshot_links_to_predecessor;
use crate::ideal_loads::calc::cooling_positive_supply_capacity_limit_guard::{
    cooling_positive_supply_capacity_limit_guard_committed_latest_snapshot_is_consistent,
};
use crate::ideal_loads::calc::cooling_supply_enthalpy_post_saturation_assignment::{
    cooling_supply_enthalpy_post_saturation_assignment_committed_latest_snapshot_is_consistent,
    cooling_supply_enthalpy_post_saturation_assignment_snapshots_match_bit_exact as cp379_snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot as SelectorLineage,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
    cooling_positive_supply_capacity_limit_guard_snapshot_is_exact_direct_release,
    cooling_supply_enthalpy_post_saturation_assignment_snapshot_is_exact_direct_release,
};

pub(super) fn guard_links_to_predecessor(
    guard: Snapshot,
    predecessor: Predecessor,
    cooling_limit: ep_model::IdealLoadsLimit,
) -> bool {
    snapshot_links_to_predecessor(guard, predecessor, cooling_limit)
}

pub(super) fn direct_predecessor_is_retained_and_complete(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> bool {
    let Some(retained) = unit
        .calc_cooling_supply_enthalpy_post_saturation_assignment
        .latest
    else {
        return false;
    };
    let Some(witness) =
        runtime.cooling_supply_enthalpy_post_saturation_assignment_latest_witness(system.id)
    else {
        return false;
    };
    system.id == predecessor.system
        && cp379_snapshots_match_bit_exact(retained, predecessor)
        && cp379_snapshots_match_bit_exact(witness, predecessor)
        && cooling_supply_enthalpy_post_saturation_assignment_snapshot_is_exact_direct_release(
            predecessor,
        )
        && cooling_supply_enthalpy_post_saturation_assignment_committed_latest_snapshot_is_consistent(
            unit, witness,
        )
}

pub(super) fn direct_selector_lineage_is_retained_and_complete(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> bool {
    let Some(retained) = unit
        .calc_cooling_positive_supply_capacity_limit_guard
        .latest
    else {
        return false;
    };
    let Some(witness) =
        runtime.cooling_positive_supply_capacity_limit_guard_latest_witness(system.id)
    else {
        return false;
    };
    retained == witness
        && selector_lineage_matches_predecessor(retained, predecessor, system)
        && cooling_positive_supply_capacity_limit_guard_snapshot_is_exact_direct_release(retained)
        && cooling_positive_supply_capacity_limit_guard_committed_latest_snapshot_is_consistent(
            unit, system, witness,
        )
}

fn selector_lineage_matches_predecessor(
    selector: SelectorLineage,
    predecessor: Predecessor,
    system: &IdealLoadsAirSystem,
) -> bool {
    let active = predecessor.local_supply_enthalpy_after_saturation_limit_assignment_performed;
    selector.system == predecessor.system
        && selector.parent_call_ordinal == predecessor.parent_call_ordinal
        && selector.controlled_zone == predecessor.controlled_zone
        && selector.unit_off_skipped == predecessor.unit_off_skipped
        && selector.non_cooling_skipped == predecessor.non_cooling_skipped
        && selector.positive_guard_false_fallthrough_skipped
            == predecessor.positive_guard_false_fallthrough_skipped
        && selector.capacity_limit_guard_evaluated == active
        && if active {
            selector.first_cooling_limit == Some(system.cooling_limit)
                && (!selector.second_cooling_limit_read
                    || selector.second_cooling_limit == Some(system.cooling_limit))
        } else {
            selector.first_cooling_limit.is_none() && selector.second_cooling_limit.is_none()
        }
}
