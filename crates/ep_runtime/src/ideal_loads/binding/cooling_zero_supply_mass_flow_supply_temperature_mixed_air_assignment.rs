//! Model-bound CP427 zero-flow supply-temperature assignment adapter.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyHumidityRatioMixedAirAssignmentSnapshot,
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyTemperatureMixedAirAssignmentSnapshot,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment,
};

use super::DirectZonePurchasedAirScheduledCouplingError;

pub(in crate::ideal_loads) fn advance_cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp426: PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyHumidityRatioMixedAirAssignmentSnapshot,
) -> Result<
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyTemperatureMixedAirAssignmentSnapshot,
    DirectZonePurchasedAirScheduledCouplingError,
> {
    advance_direct_no_oa_calc_cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment(
        runtime,
        system,
        predecessor_cp426,
    )
    .map_err(
        DirectZonePurchasedAirScheduledCouplingError::
            CalculationCoolingZeroSupplyMassFlowSupplyTemperatureMixedAirAssignment,
    )
}
