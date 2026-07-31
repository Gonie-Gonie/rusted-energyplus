//! Private CP367 snapshot witnesses.

use ep_model::IdealLoadsAirSystemId;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentSnapshot,
    PurchasedAirRuntimeState,
};

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn cooling_default_supply_humidity_ratio_mixed_air_assignment_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentSnapshot> {
        self.cooling_default_supply_humidity_ratio_mixed_air_assignment_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_default_supply_humidity_ratio_mixed_air_assignment_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot: PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentSnapshot,
    ) {
        self.cooling_default_supply_humidity_ratio_mixed_air_assignment_latest_witnesses
            .insert(system, snapshot);
    }

    #[cfg(test)]
    pub(in crate::ideal_loads) fn clear_cooling_default_supply_humidity_ratio_mixed_air_assignment_latest_witness_for_test(
        &mut self,
        system: IdealLoadsAirSystemId,
    ) {
        self.cooling_default_supply_humidity_ratio_mixed_air_assignment_latest_witnesses
            .remove(&system);
    }
}
