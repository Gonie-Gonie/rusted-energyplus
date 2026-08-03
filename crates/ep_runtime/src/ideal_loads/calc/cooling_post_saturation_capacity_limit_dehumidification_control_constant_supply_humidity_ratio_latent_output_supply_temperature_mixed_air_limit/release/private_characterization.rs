//! Restricted pure CP408 route and IEEE characterization.

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirLimitRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirLimitSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_state as advance,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot as MixedAirOwner,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentSnapshot as Predecessor,
};

/// Characterizes any CP408 route without mutating retained runtime state.
#[allow(dead_code)]
pub(in crate::ideal_loads) fn private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_characterization(
    predecessor_cp407: Predecessor,
    cp329_mixed_air_owner: Option<MixedAirOwner>,
) -> Option<Snapshot> {
    let route = super::super::transition::routes::predecessor_route(predecessor_cp407)?;
    let owner = match route.active {
        true => Some(cp329_mixed_air_owner?),
        false if cp329_mixed_air_owner.is_none() => None,
        false => return None,
    };
    let mut state = State::new(predecessor_cp407.system);
    advance(&mut state, predecessor_cp407, owner)
}
