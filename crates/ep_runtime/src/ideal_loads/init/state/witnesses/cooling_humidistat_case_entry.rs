//! Private CP358 snapshot witnesses.

use ep_model::IdealLoadsAirSystemId;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingHumidistatCaseEntrySnapshot, PurchasedAirRuntimeState,
};

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn cooling_humidistat_case_entry_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingHumidistatCaseEntrySnapshot> {
        self.cooling_humidistat_case_entry_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_humidistat_case_entry_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot: PurchasedAirCalcCoolingHumidistatCaseEntrySnapshot,
    ) {
        self.cooling_humidistat_case_entry_latest_witnesses
            .insert(system, snapshot);
    }
}
