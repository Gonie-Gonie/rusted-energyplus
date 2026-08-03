//! Restricted pure CP412 route characterization.

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_state,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot as Predecessor;

/// Characterizes one CP412 route without mutating retained runtime state.
#[allow(dead_code)]
pub(in crate::ideal_loads) fn private_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_characterization(
    predecessor: Predecessor,
    outdoor_barometric_pressure_pa: Option<f64>,
) -> Option<Snapshot> {
    let mut state = State::new(predecessor.system);
    let input = outdoor_barometric_pressure_pa.map(|outdoor_barometric_pressure_pa| ActiveInput {
        outdoor_barometric_pressure_pa,
    });
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_state(
        &mut state,
        predecessor,
        input,
    )
}
