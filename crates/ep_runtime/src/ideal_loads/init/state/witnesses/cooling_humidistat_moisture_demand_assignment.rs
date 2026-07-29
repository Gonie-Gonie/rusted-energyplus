//! Private CP359 snapshot witnesses.

use ep_model::IdealLoadsAirSystemId;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentSnapshot, PurchasedAirRuntimeState,
};

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn cooling_humidistat_moisture_demand_assignment_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentSnapshot> {
        self.cooling_humidistat_moisture_demand_assignment_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_humidistat_moisture_demand_assignment_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot: PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentSnapshot,
    ) {
        self.cooling_humidistat_moisture_demand_assignment_latest_witnesses
            .insert(system, snapshot);
    }
}
