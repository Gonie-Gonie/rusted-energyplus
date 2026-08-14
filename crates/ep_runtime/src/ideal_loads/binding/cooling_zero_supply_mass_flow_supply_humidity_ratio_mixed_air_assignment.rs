//! Model-bound CP426 zero-flow supply-humidity-ratio assignment adapter.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentSnapshot,
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyHumidityRatioMixedAirAssignmentSnapshot,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment,
};

use super::DirectZonePurchasedAirScheduledCouplingError;

pub(in crate::ideal_loads) fn advance_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp425: PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentSnapshot,
) -> Result<
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyHumidityRatioMixedAirAssignmentSnapshot,
    DirectZonePurchasedAirScheduledCouplingError,
> {
    advance_direct_no_oa_calc_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment(
        runtime,
        system,
        predecessor_cp425,
    )
    .map_err(
        DirectZonePurchasedAirScheduledCouplingError::
            CalculationCoolingZeroSupplyMassFlowSupplyHumidityRatioMixedAirAssignment,
    )
}
