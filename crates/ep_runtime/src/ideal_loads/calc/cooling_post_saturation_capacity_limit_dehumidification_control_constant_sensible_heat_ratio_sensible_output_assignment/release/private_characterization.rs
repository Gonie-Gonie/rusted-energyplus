//! Restricted pure CP388 route characterization.

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_state,
};
use super::prefix_validation::active_input_from_exact_owners;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentSnapshot as Owner,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot as Corroborator,
};
use ep_model::IdealLoadsAirSystem;

/// Characterizes one non-public CP388 route without mutating retained runtime state.
#[allow(dead_code)]
pub(in crate::ideal_loads) fn private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_characterization(
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
    owner: Option<Owner>,
    corroborator: Option<Corroborator>,
) -> Option<Snapshot> {
    if system.id != predecessor.system {
        return None;
    }
    let route = super::super::transition::routes::predecessor_route(predecessor)?;
    let active_input = if route.active {
        Some(active_input_from_exact_owners(
            system,
            predecessor,
            owner?,
            corroborator?,
        )?)
    } else {
        if owner.is_some() || corroborator.is_some() {
            return None;
        }
        None
    };
    let mut state = State::new(predecessor.system);
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_state(
        &mut state,
        predecessor,
        active_input,
    )
}
