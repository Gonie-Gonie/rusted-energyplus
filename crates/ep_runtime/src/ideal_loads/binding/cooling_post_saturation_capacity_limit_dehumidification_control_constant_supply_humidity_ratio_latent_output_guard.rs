//! Model-bound CP402 post-saturation shared latent-output guard adapter.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardSnapshot,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard,
};

use super::DirectZonePurchasedAirScheduledCouplingError;

pub(in crate::ideal_loads) fn advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp401: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentSnapshot,
) -> Result<
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardSnapshot,
    DirectZonePurchasedAirScheduledCouplingError,
>{
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard(
        runtime,
        system,
        predecessor_cp401,
    )
    .map_err(
        DirectZonePurchasedAirScheduledCouplingError::
            CalculationCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuard,
    )
}
