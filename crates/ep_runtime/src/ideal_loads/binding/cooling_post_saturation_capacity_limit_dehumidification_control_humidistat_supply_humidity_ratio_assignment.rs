//! Model-bound CP395 post-saturation Humidistat supply-humidity-ratio assignment adapter.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseEntrySnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatSupplyHumidityRatioAssignmentSnapshot,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment,
};

use super::DirectZonePurchasedAirScheduledCouplingError;

pub(in crate::ideal_loads) fn advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp394: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseEntrySnapshot,
) -> Result<
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatSupplyHumidityRatioAssignmentSnapshot,
    DirectZonePurchasedAirScheduledCouplingError,
>{
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment(
        runtime,
        system,
        predecessor_cp394,
    )
    .map_err(
        DirectZonePurchasedAirScheduledCouplingError::
            CalculationCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatSupplyHumidityRatioAssignment,
    )
}
