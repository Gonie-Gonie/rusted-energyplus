//! Coupled-runtime validation for CP380 post-saturation capacity-limit evidence.

use ep_model::IdealLoadsLimit;

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot as SelectorSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardSnapshot as Snapshot,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot as PredecessorSnapshot,
    cooling_post_saturation_capacity_limit_guard_snapshot_is_exact_direct_release,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

mod lifecycle;
mod snapshot;

pub(in crate::ideal_loads) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    snapshot::matches_release(output, call_ordinal, binding)
}

pub(in crate::ideal_loads) fn validate_lifecycle(
    lifecycle: &Lifecycle,
    predecessor: &PredecessorLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    lifecycle::validate(
        lifecycle,
        predecessor,
        timestep_count,
        latest_output,
        binding,
    )
}

fn violation(field: &'static str, expected: usize, actual: usize) -> Error {
    Error::CalcCoolingPostSaturationCapacityLimitGuardLifecycleInvariant {
        field,
        expected,
        actual,
    }
}
