//! Private CP360 snapshot witnesses.

use ep_model::IdealLoadsAirSystemId;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentSnapshot,
    PurchasedAirRuntimeState,
};

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<
        PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentSnapshot,
    > {
        self.cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot: PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentSnapshot,
    ) {
        self.cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_latest_witnesses
            .insert(system, snapshot);
    }
}
