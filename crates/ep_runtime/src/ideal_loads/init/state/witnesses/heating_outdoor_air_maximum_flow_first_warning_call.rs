//! Private CP439 latest-witness storage.

use ep_model::IdealLoadsAirSystemId;

use crate::ideal_loads::PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCallSnapshot;

use super::PurchasedAirRuntimeState;

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn heating_outdoor_air_maximum_flow_first_warning_call_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCallSnapshot> {
        self.heating_outdoor_air_maximum_flow_first_warning_call_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_heating_outdoor_air_maximum_flow_first_warning_call_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        witness: Option<PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCallSnapshot>,
    ) {
        match witness {
            Some(snapshot) => {
                self.heating_outdoor_air_maximum_flow_first_warning_call_latest_witnesses
                    .insert(system, snapshot);
            }
            None => {
                self.heating_outdoor_air_maximum_flow_first_warning_call_latest_witnesses
                    .remove(&system);
            }
        }
    }
}
