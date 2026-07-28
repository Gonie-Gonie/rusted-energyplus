//! Private CP337 snapshot witnesses.

use ep_model::IdealLoadsAirSystemId;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot, PurchasedAirRuntimeState,
};

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn cooling_positive_supply_capacity_limit_guard_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot> {
        self.cooling_positive_supply_capacity_limit_guard_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_positive_supply_capacity_limit_guard_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    ) {
        self.cooling_positive_supply_capacity_limit_guard_latest_witnesses
            .insert(system, snapshot);
    }
}
