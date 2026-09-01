//! Private CP438 latest-witness storage.

use ep_model::IdealLoadsAirSystemId;

use crate::ideal_loads::PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementSnapshot;

use super::PurchasedAirRuntimeState;

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn heating_outdoor_air_maximum_flow_first_warning_counter_increment_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementSnapshot>
    {
        self.heating_outdoor_air_maximum_flow_first_warning_counter_increment_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_heating_outdoor_air_maximum_flow_first_warning_counter_increment_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        witness: Option<
            PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementSnapshot,
        >,
    ) {
        match witness {
            Some(snapshot) => {
                self.heating_outdoor_air_maximum_flow_first_warning_counter_increment_latest_witnesses
                    .insert(system, snapshot);
            }
            None => {
                self.heating_outdoor_air_maximum_flow_first_warning_counter_increment_latest_witnesses
                    .remove(&system);
            }
        }
    }
}
