//! CP360 release error construction.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentError as Error;
use crate::ideal_loads::PurchasedAirUnitRuntimeState;

pub(super) fn predecessor_mismatch(system: IdealLoadsAirSystemId) -> Error {
    Error::CoolingHumidistatMoistureDemandAssignmentSnapshotMismatch { system }
}

pub(super) fn call_order_error(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Error {
    Error::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        cooling_humidistat_moisture_demand_assignment_transition_count: unit
            .calc_cooling_humidistat_moisture_demand_assignment
            .transition_count,
        cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_transition_count:
            unit.calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment
                .transition_count,
    }
}
