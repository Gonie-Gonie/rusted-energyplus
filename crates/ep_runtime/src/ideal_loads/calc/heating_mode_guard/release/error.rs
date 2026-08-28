//! CP431 public release errors.

use ep_model::{DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystemId};

/// Active line-2348 scalar rejected by the finite release boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PurchasedAirCalcHeatingModeGuardPredicateInput {
    /// Minimum-outdoor-air sensible output used by the first comparison operand.
    MinimumOutdoorAirSensibleOutput,
    /// Remaining heating-setpoint demand used by the second comparison operand.
    HeatingSetpointDemand,
}

/// Fail-closed CP431 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcHeatingModeGuardError {
    UnknownSystem { system: IdealLoadsAirSystemId },
    InitializationNotReady { system: IdealLoadsAirSystemId },
    SystemIdentityMismatch {
        expected: IdealLoadsAirSystemId,
        actual: IdealLoadsAirSystemId,
    },
    SystemOutsideDirectSubset { system: IdealLoadsAirSystemId },
    DehumidificationControlTypeOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
        actual: DehumidificationControlType,
    },
    HumidificationControlTypeOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
        actual: HumidificationControlType,
    },
    HeatingOrNoLoadCaseEntrySnapshotMismatch { system: IdealLoadsAirSystemId },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        predecessor_transition_count: usize,
        transition_count: usize,
    },
    PredecessorOutsideDirectSubset { system: IdealLoadsAirSystemId },
    PredecessorMixedAirTemperatureWitnessUnavailableOrInconsistent {
        system: IdealLoadsAirSystemId,
    },
    HeatingModeGuardInputsUnavailableOrInconsistent { system: IdealLoadsAirSystemId },
    NonFinitePredicateInput {
        input: PurchasedAirCalcHeatingModeGuardPredicateInput,
    },
    RuntimeStateInvariantViolation { system: IdealLoadsAirSystemId },
}

impl std::fmt::Display for PurchasedAirCalcHeatingModeGuardError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "CP431 heating-mode-guard release failed: {self:?}")
    }
}

impl std::error::Error for PurchasedAirCalcHeatingModeGuardError {}
