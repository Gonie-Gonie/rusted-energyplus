//! Private CP397 latest-witness storage.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirRuntimeState;
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlNoneCaseEntrySnapshot;

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlNoneCaseEntrySnapshot>
    {
        self.cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlNoneCaseEntrySnapshot,
    ) {
        self.cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry_latest_witnesses
            .insert(system, snapshot);
    }
}
