//! Model-bound CP422 sensible-output maximum-capacity assignment adapter.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentSnapshot,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment,
};

use super::DirectZonePurchasedAirScheduledCouplingError;

pub(in crate::ideal_loads) fn advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp421: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardSnapshot,
) -> Result<
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentSnapshot,
    DirectZonePurchasedAirScheduledCouplingError,
>{
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment(
        runtime,
        system,
        predecessor_cp421,
    )
    .map_err(
        DirectZonePurchasedAirScheduledCouplingError::
            CalculationCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignment,
    )
}
