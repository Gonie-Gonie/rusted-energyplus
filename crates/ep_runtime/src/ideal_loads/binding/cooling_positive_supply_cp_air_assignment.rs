//! Model-bound CP331 transition adapter.

use ep_model::IdealLoadsAirSystem;

use crate::{
    heat_balance::state::ZoneHeatBalanceState,
    ideal_loads::{
        PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
        PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot, PurchasedAirRuntimeState,
        advance_direct_no_oa_calc_cooling_positive_supply_cp_air_assignment,
    },
};

use super::DirectZonePurchasedAirScheduledCouplingError;

pub(super) fn advance_positive_supply_cp_air_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    zone_state: &ZoneHeatBalanceState,
) -> Result<
    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
    DirectZonePurchasedAirScheduledCouplingError,
> {
    advance_direct_no_oa_calc_cooling_positive_supply_cp_air_assignment(
        runtime,
        system,
        predecessor,
        zone_state,
    )
    .map_err(
        DirectZonePurchasedAirScheduledCouplingError::
            CalculationCoolingPositiveSupplyCpAirAssignment,
    )
}
