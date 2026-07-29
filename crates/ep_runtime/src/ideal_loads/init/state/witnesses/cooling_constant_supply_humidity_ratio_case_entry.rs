//! Private CP364 snapshot witnesses.

use ep_model::IdealLoadsAirSystemId;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntrySnapshot, PurchasedAirRuntimeState,
};

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn cooling_constant_supply_humidity_ratio_case_entry_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntrySnapshot> {
        self.cooling_constant_supply_humidity_ratio_case_entry_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_constant_supply_humidity_ratio_case_entry_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot: PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntrySnapshot,
    ) {
        self.cooling_constant_supply_humidity_ratio_case_entry_latest_witnesses
            .insert(system, snapshot);
    }

    #[cfg(test)]
    pub(in crate::ideal_loads) fn clear_cooling_constant_supply_humidity_ratio_case_entry_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
    ) {
        let _ = self
            .cooling_constant_supply_humidity_ratio_case_entry_latest_witnesses
            .remove(&system);
    }
}
