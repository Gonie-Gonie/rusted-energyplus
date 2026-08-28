use ep_model::IdealLoadsAirSystemId;

use super::super::PurchasedAirRuntimeState;
use crate::ideal_loads::PurchasedAirCalcHeatingModeGuardSnapshot;

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn heating_mode_guard_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcHeatingModeGuardSnapshot> {
        self.heating_mode_guard_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_heating_mode_guard_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        witness: Option<PurchasedAirCalcHeatingModeGuardSnapshot>,
    ) {
        match witness {
            Some(witness) => {
                self.heating_mode_guard_latest_witnesses
                    .insert(system, witness);
            }
            None => {
                self.heating_mode_guard_latest_witnesses.remove(&system);
            }
        }
    }
}
