//! Private CP328 latest-witness accessors.

use ep_model::IdealLoadsAirSystemId;

use super::super::PurchasedAirRuntimeState;
use crate::ideal_loads::PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot;

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn cooling_supply_mass_flow_very_small_guard_body_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot> {
        self.cooling_supply_mass_flow_very_small_guard_body_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_supply_mass_flow_very_small_guard_body_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
    ) {
        self.cooling_supply_mass_flow_very_small_guard_body_latest_witnesses
            .insert(system, snapshot);
    }
}
