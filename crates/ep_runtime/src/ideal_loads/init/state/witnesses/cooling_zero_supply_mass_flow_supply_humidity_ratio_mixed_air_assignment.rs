use ep_model::IdealLoadsAirSystemId;

use super::super::PurchasedAirRuntimeState;
use crate::ideal_loads::PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyHumidityRatioMixedAirAssignmentSnapshot;

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyHumidityRatioMixedAirAssignmentSnapshot>
    {
        self.cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        witness: Option<
            PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyHumidityRatioMixedAirAssignmentSnapshot,
        >,
    ) {
        match witness {
            Some(witness) => {
                self.cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_latest_witnesses
                    .insert(system, witness);
            }
            None => {
                self.cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_latest_witnesses
                    .remove(&system);
            }
        }
    }
}
