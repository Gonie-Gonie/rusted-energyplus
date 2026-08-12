//! Model-bound CP417 supply-enthalpy assignment adapter.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioAssignmentSnapshot,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment,
};

use super::DirectZonePurchasedAirScheduledCouplingError;

pub(in crate::ideal_loads) fn advance_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp416: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioAssignmentSnapshot,
) -> Result<
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentSnapshot,
    DirectZonePurchasedAirScheduledCouplingError,
>{
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment(
        runtime,
        system,
        predecessor_cp416,
    )
    .map_err(
        DirectZonePurchasedAirScheduledCouplingError::
            CalculationCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignment,
    )
}
