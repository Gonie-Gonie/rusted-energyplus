//! Private CP329 latest-witness accessors.

use ep_model::IdealLoadsAirSystemId;

use super::super::PurchasedAirRuntimeState;
use crate::ideal_loads::PurchasedAirCalcCoolingMixedAirCallSnapshot;

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn cooling_mixed_air_call_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingMixedAirCallSnapshot> {
        self.cooling_mixed_air_call_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_mixed_air_call_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot: PurchasedAirCalcCoolingMixedAirCallSnapshot,
    ) {
        self.cooling_mixed_air_call_latest_witnesses
            .insert(system, snapshot);
    }
}
