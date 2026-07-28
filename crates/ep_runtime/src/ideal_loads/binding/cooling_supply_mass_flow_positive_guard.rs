//! Model-bound CP330 transition adapter.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot, PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_positive_guard,
};

use super::DirectZonePurchasedAirScheduledCouplingError;

pub(super) fn advance_positive_guard(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingMixedAirCallSnapshot,
) -> Result<
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    DirectZonePurchasedAirScheduledCouplingError,
> {
    advance_direct_no_oa_calc_cooling_supply_mass_flow_positive_guard(runtime, system, predecessor)
        .map_err(
        DirectZonePurchasedAirScheduledCouplingError::CalculationCoolingSupplyMassFlowPositiveGuard,
    )
}
