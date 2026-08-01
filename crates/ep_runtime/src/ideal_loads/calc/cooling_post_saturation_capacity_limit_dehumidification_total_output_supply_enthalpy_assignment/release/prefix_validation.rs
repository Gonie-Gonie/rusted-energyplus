//! Retained CP382/CP384 owner-bundle and direct-predecessor validation.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentActiveOperands as ActiveOperands,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentRetainedInput as RetainedInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot as Snapshot,
};
use super::snapshot_validation::snapshot_links_to_predecessor;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment::{
    completed_direct_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_snapshots_match_bit_exact as cp384_snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentSnapshot as Cp382,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
    cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_snapshot_is_exact_direct_release,
};

pub(super) fn assignment_links_to_predecessors(
    assignment: Snapshot,
    predecessor: Predecessor,
    retained_input: Option<RetainedInput>,
) -> bool {
    snapshot_links_to_predecessor(assignment, predecessor, retained_input)
}

pub(super) fn direct_predecessor_is_retained_and_complete(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    supplied: Predecessor,
) -> bool {
    let Some(retained) = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment
        .latest
    else {
        return false;
    };
    let Some(witness) = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_latest_witness(system.id)
    else {
        return false;
    };
    system.id == supplied.system
        && cp384_snapshots_match_bit_exact(retained, supplied)
        && cp384_snapshots_match_bit_exact(witness, supplied)
        && cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_snapshot_is_exact_direct_release(supplied)
        && completed_direct_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_is_consistent(
            runtime,
            unit,
            system,
            retained,
            Some(witness),
        )
}

pub(super) fn retained_cp382_lineage_is_exact(
    predecessor: Predecessor,
    cp382: Option<Cp382>,
    cp382_witness: Option<Cp382>,
) -> bool {
    let active = predecessor.predecessor_dehumidification_total_output_capacity_guard_evaluated;
    if !active {
        return cp382.is_none() && cp382_witness.is_none();
    }
    let (Some(cp382), Some(witness)) = (cp382, cp382_witness) else {
        return false;
    };
    predecessor.system == cp382.system
        && predecessor.parent_call_ordinal == cp382.parent_call_ordinal
        && predecessor.controlled_zone == cp382.controlled_zone
        && crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment::cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_snapshots_match_bit_exact(cp382, witness)
        && cp382.dehumidification_total_output_assignment_executed
        && cp382.cp330_supply_mass_flow_rate_owned_read
        && cp382.cp329_mixed_air_enthalpy_owned_read
        && cp382.cp379_post_saturation_supply_enthalpy_owned_read
        && cp382.supply_mass_flow_rate_kg_per_s.is_some()
        && cp382.mixed_air_enthalpy_j_per_kg.is_some()
        && cp382.supply_enthalpy_j_per_kg.is_some()
        && cp382.cooling_total_output_w.is_some()
}

pub(super) fn retained_input_from_prefix(
    retained_cp384: Predecessor,
    cp382: Option<Cp382>,
) -> Option<RetainedInput> {
    if !retained_cp384.predecessor_dehumidification_total_output_capacity_guard_evaluated {
        return None;
    }
    let cp382 = cp382?;
    let preexisting_supply_enthalpy_j_per_kg = cp382.supply_enthalpy_j_per_kg?;
    let active_operands = if retained_cp384
        .dehumidification_total_output_maximum_capacity_assignment_executed
    {
        Some(ActiveOperands {
            mixed_air_enthalpy_j_per_kg: cp382.mixed_air_enthalpy_j_per_kg?,
            cooling_total_output_w: retained_cp384.resulting_cooling_total_output_w?,
            supply_mass_flow_rate_kg_per_s: cp382.supply_mass_flow_rate_kg_per_s?,
        })
    } else {
        None
    };
    Some(RetainedInput {
        preexisting_supply_enthalpy_j_per_kg,
        active_operands,
    })
}
