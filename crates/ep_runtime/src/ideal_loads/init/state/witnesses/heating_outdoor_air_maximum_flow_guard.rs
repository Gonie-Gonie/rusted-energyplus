use ep_model::IdealLoadsAirSystemId;

use super::super::PurchasedAirRuntimeState;
use crate::ideal_loads::PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardSnapshot;

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn heating_outdoor_air_maximum_flow_guard_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardSnapshot> {
        self.heating_outdoor_air_maximum_flow_guard_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_heating_outdoor_air_maximum_flow_guard_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        witness: Option<PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardSnapshot>,
    ) {
        match witness {
            Some(witness) => {
                self.heating_outdoor_air_maximum_flow_guard_latest_witnesses
                    .insert(system, witness);
            }
            None => {
                self.heating_outdoor_air_maximum_flow_guard_latest_witnesses
                    .remove(&system);
            }
        }
    }
}
