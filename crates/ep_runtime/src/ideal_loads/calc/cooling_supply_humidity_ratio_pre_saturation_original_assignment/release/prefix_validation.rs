//! CP376 immediate-predecessor and direct CP347 owner validation.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentOwner as Owner,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot as Snapshot,
    advance_cooling_supply_humidity_ratio_pre_saturation_original_assignment_state,
};
use super::snapshot_validation::snapshot_links_to_predecessor;
use crate::ideal_loads::calc::cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case::{
    completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_is_consistent,
};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment::{
    completed_direct_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_is_consistent,
    cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_snapshots_match_bit_exact as cp375_snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot as Cp347Snapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
    cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_snapshot_is_exact_direct_release,
    cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_snapshot_is_exact_direct_release,
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
        .calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment
        .latest
    else {
        return false;
    };
    let Some(witness) = runtime
        .cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_latest_witness(
            system.id,
        )
    else {
        return false;
    };
    system.id == predecessor.system
        && cp375_snapshots_match_bit_exact(retained, predecessor)
        && cp375_snapshots_match_bit_exact(witness, predecessor)
        && cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_snapshot_is_exact_direct_release(
            predecessor,
        )
        && completed_direct_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_is_consistent(
            runtime,
            unit,
            system,
            predecessor,
            Some(witness),
        )
}

pub(super) fn direct_cp347_owner(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> Option<f64> {
    let owner = unit
        .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case
        .latest?;
    let witness = runtime
        .cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_latest_witness(
            system.id,
        )?;
    if system.id != predecessor.system
        || unit.system != system.id
        || owner.system != predecessor.system
        || owner.parent_call_ordinal != predecessor.parent_call_ordinal
        || owner.controlled_zone != predecessor.controlled_zone
        || !cp347_snapshots_match_bit_exact(owner, witness)
        || !cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_snapshot_is_exact_direct_release(
            owner,
        )
        || !completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_is_consistent(
            runtime,
            unit,
            system,
            owner,
            Some(witness),
        )
    {
        return None;
    }
    owner.resulting_supply_humidity_ratio
}

#[allow(dead_code)]
pub(super) fn direct_assignment_from_owner(
    predecessor: Predecessor,
    owner: f64,
) -> Option<Snapshot> {
    let mut state = State::new(predecessor.system);
    let assignment =
        advance_cooling_supply_humidity_ratio_pre_saturation_original_assignment_state(
            &mut state,
            predecessor,
            Some(ActiveInput {
                purchased_air_supply_humidity_ratio: owner,
                owner: Owner::Cp347NoneCase,
            }),
        )?;
    assignment_links_to_predecessor(assignment, predecessor).then_some(assignment)
}

fn cp347_snapshots_match_bit_exact(mut left: Cp347Snapshot, mut right: Cp347Snapshot) -> bool {
    let values_match = [
        option_bits_match(
            left.predecessor_assigned_supply_humidity_ratio,
            right.predecessor_assigned_supply_humidity_ratio,
        ),
        option_bits_match(
            left.mixed_air_humidity_ratio,
            right.mixed_air_humidity_ratio,
        ),
        option_bits_match(
            left.assigned_supply_humidity_ratio,
            right.assigned_supply_humidity_ratio,
        ),
        option_bits_match(
            left.resulting_supply_humidity_ratio,
            right.resulting_supply_humidity_ratio,
        ),
    ]
    .into_iter()
    .all(|matches| matches);
    for snapshot in [&mut left, &mut right] {
        snapshot.predecessor_assigned_supply_humidity_ratio = None;
        snapshot.mixed_air_humidity_ratio = None;
        snapshot.assigned_supply_humidity_ratio = None;
        snapshot.resulting_supply_humidity_ratio = None;
    }
    values_match && left == right
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
