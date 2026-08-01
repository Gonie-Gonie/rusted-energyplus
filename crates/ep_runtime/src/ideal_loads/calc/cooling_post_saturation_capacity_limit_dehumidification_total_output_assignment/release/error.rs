//! CP382 fail-closed release errors.

use ep_model::{DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystemId};

use crate::ideal_loads::PurchasedAirUnitRuntimeState;

/// Active CP382 retained operand rejected before mutation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentInput
{
    /// Retained CP330 supply mass flow rate.
    SupplyMassFlowRate,
    /// Retained CP329 mixed-air enthalpy projection.
    MixedAirEnthalpy,
    /// Retained CP379 post-saturation supply enthalpy.
    SupplyEnthalpy,
}

/// Fail-closed CP382 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentError
{
    UnknownSystem {
        system: IdealLoadsAirSystemId,
    },
    InitializationNotReady {
        system: IdealLoadsAirSystemId,
    },
    SystemIdentityMismatch {
        expected: IdealLoadsAirSystemId,
        actual: IdealLoadsAirSystemId,
    },
    SystemOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    DehumidificationControlTypeOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
        actual: DehumidificationControlType,
    },
    HumidificationControlTypeOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
        actual: HumidificationControlType,
    },
    CoolingPostSaturationCapacityLimitDehumidificationGuardSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    ActiveOperandOwnerLineageMismatch {
        system: IdealLoadsAirSystemId,
    },
    InvalidActiveInput {
        system: IdealLoadsAirSystemId,
        input:
            PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentInput,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        predecessor_transition_count: usize,
        assignment_transition_count: usize,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

pub(super) fn predecessor_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentError {
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentError::
        CoolingPostSaturationCapacityLimitDehumidificationGuardSnapshotMismatch { system }
}

pub(super) fn call_order_error(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentError {
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentError::
        PredecessorCallOrder {
            system,
            init_call_count: unit.init_call_count,
            predecessor_transition_count: unit
                .calc_cooling_post_saturation_capacity_limit_dehumidification_guard
                .transition_count,
            assignment_transition_count: unit
                .calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment
                .transition_count,
        }
}
