//! Private CP341 snapshot witnesses.

use ep_model::IdealLoadsAirSystemId;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
    PurchasedAirRuntimeState,
};

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
    >{
        self.cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot:
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
    ) {
        self.cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_latest_witnesses
            .insert(system, snapshot);
    }
}
