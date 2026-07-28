//! Private CP347 snapshot witnesses.

use ep_model::IdealLoadsAirSystemId;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
    PurchasedAirRuntimeState,
};

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
    >{
        self.cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot:
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
    ) {
        self.cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_latest_witnesses
            .insert(system, snapshot);
    }
}
