//! Private CP383 latest-witness storage.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirRuntimeState;
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardSnapshot;

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardSnapshot,
    > {
        self.cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardSnapshot,
    ) {
        self.cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_latest_witnesses
            .insert(system, snapshot);
    }
}
