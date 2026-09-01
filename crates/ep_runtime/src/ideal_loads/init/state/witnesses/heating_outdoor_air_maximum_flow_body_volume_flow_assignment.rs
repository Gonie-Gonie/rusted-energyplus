use ep_model::IdealLoadsAirSystemId;

use super::super::PurchasedAirRuntimeState;
use crate::ideal_loads::PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentSnapshot;

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn heating_outdoor_air_maximum_flow_body_volume_flow_assignment_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentSnapshot> {
        self.heating_outdoor_air_maximum_flow_body_volume_flow_assignment_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_heating_outdoor_air_maximum_flow_body_volume_flow_assignment_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        witness: Option<PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentSnapshot>,
    ) {
        match witness {
            Some(witness) => {
                self.heating_outdoor_air_maximum_flow_body_volume_flow_assignment_latest_witnesses
                    .insert(system, witness);
            }
            None => {
                self.heating_outdoor_air_maximum_flow_body_volume_flow_assignment_latest_witnesses
                    .remove(&system);
            }
        }
    }
}
