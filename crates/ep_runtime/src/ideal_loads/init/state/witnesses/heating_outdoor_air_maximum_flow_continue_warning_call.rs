//! Private CP440 latest-witness storage.

use ep_model::IdealLoadsAirSystemId;

use crate::ideal_loads::PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningCallSnapshot;

use super::PurchasedAirRuntimeState;

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn heating_outdoor_air_maximum_flow_continue_warning_call_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningCallSnapshot> {
        self.heating_outdoor_air_maximum_flow_continue_warning_call_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_heating_outdoor_air_maximum_flow_continue_warning_call_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        witness: Option<PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningCallSnapshot>,
    ) {
        match witness {
            Some(snapshot) => {
                self.heating_outdoor_air_maximum_flow_continue_warning_call_latest_witnesses
                    .insert(system, snapshot);
            }
            None => {
                self.heating_outdoor_air_maximum_flow_continue_warning_call_latest_witnesses
                    .remove(&system);
            }
        }
    }
}
