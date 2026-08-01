//! Private CP373 snapshot witnesses.

use ep_model::IdealLoadsAirSystemId;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentSnapshot,
    PurchasedAirRuntimeState,
};

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentSnapshot> {
        self.cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot: PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentSnapshot,
    ) {
        self.cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_latest_witnesses
            .insert(system, snapshot);
    }
}
