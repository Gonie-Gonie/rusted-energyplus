//! Model-bound CP332 transition adapter.

use ep_model::IdealLoadsAirSystem;

use crate::{
    heat_balance::state::ZoneHeatBalanceState,
    ideal_loads::{
        PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
        PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
        PurchasedAirRuntimeState,
        advance_direct_no_oa_calc_cooling_positive_supply_temperature_assignment,
    },
};

use super::DirectZonePurchasedAirScheduledCouplingError;

pub(super) fn advance_positive_supply_temperature_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
    zone_state: &ZoneHeatBalanceState,
) -> Result<
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
    DirectZonePurchasedAirScheduledCouplingError,
> {
    advance_direct_no_oa_calc_cooling_positive_supply_temperature_assignment(
        runtime,
        system,
        predecessor,
        zone_state,
    )
    .map_err(
        DirectZonePurchasedAirScheduledCouplingError::
            CalculationCoolingPositiveSupplyTemperatureAssignment,
    )
}
