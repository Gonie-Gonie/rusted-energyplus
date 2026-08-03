//! Restricted pure CP407 route and IEEE characterization.

use super::super::transition::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentActiveOwners as ActiveOwners;
use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_state as advance,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntrySnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot as EnthalpyOwner,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot as HumidityOwner,
};

/// Characterizes any CP407 route without mutating retained runtime state.
#[allow(dead_code)]
pub(in crate::ideal_loads) fn private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_characterization(
    predecessor_cp406: Predecessor,
    cp378_humidity_owner: Option<HumidityOwner>,
    cp385_enthalpy_owner: Option<EnthalpyOwner>,
) -> Option<Snapshot> {
    let route = super::super::transition::routes::predecessor_route(predecessor_cp406)?;
    let active_owners = match route.assignment_executed {
        true => Some(ActiveOwners {
            supply_humidity_ratio_owner: cp378_humidity_owner?,
            supply_enthalpy_owner: cp385_enthalpy_owner?,
        }),
        false if cp378_humidity_owner.is_none() && cp385_enthalpy_owner.is_none() => None,
        false => return None,
    };
    let mut state = State::new(predecessor_cp406.system);
    advance(&mut state, predecessor_cp406, active_owners)
}
