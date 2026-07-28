//! Private CP330 snapshot witnesses.

use ep_model::IdealLoadsAirSystemId;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot, PurchasedAirRuntimeState,
};

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn cooling_supply_mass_flow_positive_guard_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot> {
        self.cooling_supply_mass_flow_positive_guard_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_supply_mass_flow_positive_guard_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot: PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    ) {
        self.cooling_supply_mass_flow_positive_guard_latest_witnesses
            .insert(system, snapshot);
    }
}
