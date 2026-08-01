//! CP387 predecessor and CP384/CP385 owner-bundle validation.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_state,
};
use super::snapshot_validation::snapshots_match_bit_exact;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentSnapshot as Owner,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot as Corroborator,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
};

pub(super) fn direct_predecessor_is_retained_and_complete(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> bool {
    let witness = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_latest_witness(system.id);
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment
        .latest
        .is_some_and(|retained| {
            crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_snapshots_match_bit_exact(
                retained,
                predecessor,
            )
        })
        && crate::ideal_loads::calc::completed_direct_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_is_consistent(
            runtime,
            unit,
            system,
            predecessor,
            witness,
        )
}

pub(super) fn assignment_links_to_predecessor(
    snapshot: Snapshot,
    predecessor: Predecessor,
    active_input: Option<ActiveInput>,
) -> bool {
    let mut state = State::new(predecessor.system);
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_state(
        &mut state,
        predecessor,
        active_input,
    )
    .is_some_and(|expected| snapshots_match_bit_exact(expected, snapshot))
}

pub(super) fn active_input_from_exact_owners(
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
    owner: Owner,
    corroborator: Corroborator,
) -> Option<ActiveInput> {
    if system.id != predecessor.system
        || system.dehumidification_control_type
            != ep_model::DehumidificationControlType::ConstantSensibleHeatRatio
    {
        return None;
    }
    let input = ActiveInput {
        cooling_total_output_owner: owner,
        cooling_total_output_corroborator: corroborator,
        cooling_sensible_heat_ratio: system.cooling_sensible_heat_ratio,
    };
    let mut state = State::new(predecessor.system);
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_state(
        &mut state,
        predecessor,
        Some(input),
    )?;
    Some(input)
}
