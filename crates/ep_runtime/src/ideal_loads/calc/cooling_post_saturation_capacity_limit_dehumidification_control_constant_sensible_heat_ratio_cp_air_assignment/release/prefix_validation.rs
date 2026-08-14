//! CP386-to-CP387 retained-lineage validation.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_state,
};
use super::snapshot_validation::snapshots_match_bit_exact;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_switch::cooling_post_saturation_capacity_limit_dehumidification_control_switch_committed_latest_snapshot_is_consistent;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
    cooling_mixed_air_call_snapshot_is_exact_direct_release,
    cooling_mixed_air_call_snapshots_match_bit_exact,
};
use crate::psychrometrics::energyplus_psy_cp_air_fn_w;

pub(super) fn direct_predecessor_is_retained_and_complete(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> bool {
    let witness = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_control_switch_latest_witness(
            system.id,
        );
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_switch
        .latest
        .is_some_and(|retained| {
            crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_switch_snapshots_match_bit_exact(
                retained,
                predecessor,
            )
        })
        && witness.is_some_and(|witness| {
            cooling_post_saturation_capacity_limit_dehumidification_control_switch_committed_latest_snapshot_is_consistent(
                unit,
                system,
                witness,
            )
        })
}

pub(super) fn assignment_links_to_predecessor(
    snapshot: Snapshot,
    predecessor: Predecessor,
    active_input: Option<ActiveInput>,
) -> bool {
    let mut state = State::new(predecessor.system);
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_state(
        &mut state,
        predecessor,
        active_input,
    )
    .is_some_and(|expected| snapshots_match_bit_exact(expected, snapshot))
}

pub(super) fn active_operand_links_to_retained_owner(
    predecessor: Predecessor,
    owner: PurchasedAirCalcCoolingMixedAirCallSnapshot,
    owner_witness: PurchasedAirCalcCoolingMixedAirCallSnapshot,
) -> bool {
    let Some(route) = super::super::transition::routes::predecessor_route(predecessor) else {
        return false;
    };
    let Some(operand) = owner.mixed_air_humidity_ratio else {
        return false;
    };
    route.active
        && owner.system == predecessor.system
        && owner.parent_call_ordinal == predecessor.parent_call_ordinal
        && owner.controlled_zone == predecessor.controlled_zone
        && owner.cooling_call_executed
        && owner.no_outdoor_air_fallback_entered
        && owner.mixed_air_humidity_ratio_assigned
        && cooling_mixed_air_call_snapshot_is_exact_direct_release(owner)
        && cooling_mixed_air_call_snapshots_match_bit_exact(owner, owner_witness)
        && operand.is_finite()
        && operand >= 0.0
        && energyplus_psy_cp_air_fn_w(operand).is_finite()
}

pub(super) fn active_input_from_retained_owner(
    runtime: &PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> Option<ActiveInput> {
    if system.id != predecessor.system {
        return None;
    }
    let unit = runtime.units.get(&predecessor.system)?;
    let owner = unit.calc_cooling_mixed_air_call.latest?;
    let owner_witness = runtime.cooling_mixed_air_call_latest_witness(predecessor.system)?;
    if !crate::ideal_loads::calc::completed_direct_cooling_mixed_air_call_is_consistent(
        runtime,
        unit,
        system,
        owner,
        Some(owner_witness),
    ) || !active_operand_links_to_retained_owner(predecessor, owner, owner_witness)
    {
        return None;
    }
    Some(ActiveInput {
        mixed_air_humidity_ratio: owner.mixed_air_humidity_ratio?,
    })
}

#[cfg(test)]
pub(in crate::ideal_loads::calc) fn active_input_from_owner_for_test(
    predecessor: Predecessor,
    owner: PurchasedAirCalcCoolingMixedAirCallSnapshot,
    owner_witness: PurchasedAirCalcCoolingMixedAirCallSnapshot,
) -> Option<ActiveInput> {
    active_operand_links_to_retained_owner(predecessor, owner, owner_witness).then_some(
        ActiveInput {
            mixed_air_humidity_ratio: owner.mixed_air_humidity_ratio?,
        },
    )
}
