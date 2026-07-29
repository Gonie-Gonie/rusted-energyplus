//! Model-bound CP360 transition adapter.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentSnapshot,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentSnapshot,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment,
};

use super::DirectZonePurchasedAirScheduledCouplingError;

pub(super) fn advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentSnapshot,
) -> Result<
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentSnapshot,
    DirectZonePurchasedAirScheduledCouplingError,
> {
    advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment(
        runtime,
        system,
        predecessor,
    )
    .map_err(
        DirectZonePurchasedAirScheduledCouplingError::
            CalculationCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignment,
    )
}
