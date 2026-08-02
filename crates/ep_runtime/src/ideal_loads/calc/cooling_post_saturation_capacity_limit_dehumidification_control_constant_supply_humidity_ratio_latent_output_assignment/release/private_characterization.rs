//! Restricted pure CP401 route characterization.

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentActiveOwners as ActiveOwners,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_state,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentSnapshot as TotalOutputOwner,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot as TotalOutputCorroborator,
};

/// Characterizes one non-public CP401 route without mutating retained runtime state.
#[allow(dead_code)]
pub(in crate::ideal_loads) fn private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_characterization(
    predecessor: Predecessor,
    cooling_total_output_owner: Option<TotalOutputOwner>,
    cooling_total_output_corroborator: Option<TotalOutputCorroborator>,
) -> Option<Snapshot> {
    let route = super::super::transition::routes::predecessor_route(predecessor)?;
    let active_owners = match route.active {
        true => Some(ActiveOwners {
            cooling_total_output_owner: cooling_total_output_owner?,
            cooling_total_output_corroborator: cooling_total_output_corroborator?,
        }),
        false
            if cooling_total_output_owner.is_none()
                && cooling_total_output_corroborator.is_none() =>
        {
            None
        }
        false => return None,
    };
    let mut state = State::new(predecessor.system);
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_state(
        &mut state,
        predecessor,
        active_owners,
    )
}
