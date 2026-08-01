//! Restricted pure CP390 route characterization.

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_state,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot as MixedAirOwner,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentSnapshot as Predecessor,
};

/// Characterizes one non-public CP390 route without mutating retained runtime state.
#[allow(dead_code)]
pub(in crate::ideal_loads) fn private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_characterization(
    predecessor: Predecessor,
    mixed_air_owner: Option<MixedAirOwner>,
) -> Option<Snapshot> {
    let mut state = State::new(predecessor.system);
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_state(
        &mut state,
        predecessor,
        mixed_air_owner,
    )
}
