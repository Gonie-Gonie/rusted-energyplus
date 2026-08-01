//! Model-bound CP386 post-saturation dehumidification-control switch adapter.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_switch,
};

use super::DirectZonePurchasedAirScheduledCouplingError;

pub(in crate::ideal_loads) fn advance_cooling_post_saturation_capacity_limit_dehumidification_control_switch(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp385: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot,
) -> Result<
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchSnapshot,
    DirectZonePurchasedAirScheduledCouplingError,
> {
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_switch(
        runtime,
        system,
        predecessor_cp385,
    )
    .map_err(
        DirectZonePurchasedAirScheduledCouplingError::
            CalculationCoolingPostSaturationCapacityLimitDehumidificationControlSwitch,
    )
}
