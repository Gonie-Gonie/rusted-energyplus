//! Private CP410 latest-witness storage.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirRuntimeState;
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlDefaultCaseBreakSnapshot;

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlDefaultCaseBreakSnapshot>
    {
        self.cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlDefaultCaseBreakSnapshot,
    ) {
        self.cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break_latest_witnesses
            .insert(system, snapshot);
    }
}
