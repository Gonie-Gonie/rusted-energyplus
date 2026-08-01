//! Coupled-runtime validation for CP382 total-output assignment evidence.

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE,
    PurchasedAirCalcCoolingMixedAirCallLifecycleSummary as MixedAirLifecycle,
    PurchasedAirCalcCoolingMixedAirCallRuntimeState as MixedAirState,
    PurchasedAirCalcCoolingMixedAirCallSnapshot as MixedAirSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentLifecycleSummary as EarlyTotalLifecycle,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRuntimeState as EarlyTotalState,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot as EarlyTotalSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardSnapshot as PredecessorSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentSnapshot as Snapshot,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentLifecycleSummary as SupplyEnthalpyLifecycle,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentRuntimeState as SupplyEnthalpyState,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot as SupplyEnthalpySnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardLifecycleSummary as SupplyMassFlowLifecycle,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRuntimeState as SupplyMassFlowState,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot as SupplyMassFlowSnapshot,
    cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_snapshot_is_exact_direct_release,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

mod lifecycle;
mod routes;
mod snapshot;

pub(in crate::ideal_loads) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    snapshot::matches_release(output, call_ordinal, binding)
}

#[allow(clippy::too_many_arguments)]
pub(in crate::ideal_loads) fn validate_lifecycle(
    lifecycle: &Lifecycle,
    predecessor_cp381: &PredecessorLifecycle,
    supply_mass_flow_owner_cp330: &SupplyMassFlowLifecycle,
    mixed_air_owner_cp329: &MixedAirLifecycle,
    early_total_corroborator_cp339: &EarlyTotalLifecycle,
    supply_enthalpy_owner_cp379: &SupplyEnthalpyLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    lifecycle::validate(
        lifecycle,
        predecessor_cp381,
        supply_mass_flow_owner_cp330,
        mixed_air_owner_cp329,
        early_total_corroborator_cp339,
        supply_enthalpy_owner_cp379,
        timestep_count,
        latest_output,
        binding,
    )
}

fn violation(field: &'static str, expected: usize, actual: usize) -> Error {
    Error::CalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

fn exact_optional_f64(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
