//! CP361 release error construction.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitError as Error;
use crate::ideal_loads::PurchasedAirUnitRuntimeState;

pub(super) fn predecessor_mismatch(system: IdealLoadsAirSystemId) -> Error {
    Error::CoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentSnapshotMismatch {
        system,
    }
}

pub(super) fn call_order_error(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Error {
    Error::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_transition_count:
            unit.calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment
                .transition_count,
        cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_transition_count:
            unit.calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit
                .transition_count,
    }
}
