use ep_model::IdealLoadsAirSystemId;

use super::super::PurchasedAirRuntimeState;
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardSnapshot;

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardSnapshot> {
        self.cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        witness: Option<PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardSnapshot>,
    ) {
        match witness {
            Some(witness) => {
                self.cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_latest_witnesses
                    .insert(system, witness);
            }
            None => {
                self.cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_latest_witnesses
                    .remove(&system);
            }
        }
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub(in crate::ideal_loads) fn clear_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_latest_witness_for_test(
        &mut self,
        system: IdealLoadsAirSystemId,
    ) {
        self.cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_latest_witnesses
            .remove(&system);
    }
}
