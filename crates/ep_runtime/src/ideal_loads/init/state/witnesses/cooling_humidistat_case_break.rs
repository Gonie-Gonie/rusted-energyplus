//! Private CP363 snapshot witnesses.

use ep_model::IdealLoadsAirSystemId;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingHumidistatCaseBreakSnapshot, PurchasedAirRuntimeState,
};

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn cooling_humidistat_case_break_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingHumidistatCaseBreakSnapshot> {
        self.cooling_humidistat_case_break_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_humidistat_case_break_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot: PurchasedAirCalcCoolingHumidistatCaseBreakSnapshot,
    ) {
        self.cooling_humidistat_case_break_latest_witnesses
            .insert(system, snapshot);
    }

    #[cfg(test)]
    pub(in crate::ideal_loads) fn clear_cooling_humidistat_case_break_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
    ) {
        let _ = self
            .cooling_humidistat_case_break_latest_witnesses
            .remove(&system);
    }
}
