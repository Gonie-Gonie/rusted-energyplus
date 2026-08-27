//! CP428 public release errors.

use ep_model::{DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystemId};

/// Fail-closed CP428 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingZeroSupplyMassFlowSensibleOutputPositiveZeroAssignmentError {
    UnknownSystem { system: IdealLoadsAirSystemId },
    InitializationNotReady { system: IdealLoadsAirSystemId },
    SystemIdentityMismatch { expected: IdealLoadsAirSystemId, actual: IdealLoadsAirSystemId },
    SystemOutsideDirectSubset { system: IdealLoadsAirSystemId },
    DehumidificationControlTypeOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
        actual: DehumidificationControlType,
    },
    HumidificationControlTypeOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
        actual: HumidificationControlType,
    },
    CoolingZeroSupplyMassFlowSupplyTemperatureMixedAirAssignmentSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
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
    RuntimeStateInvariantViolation { system: IdealLoadsAirSystemId },
}

impl std::fmt::Display for PurchasedAirCalcCoolingZeroSupplyMassFlowSensibleOutputPositiveZeroAssignmentError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "CP428 zero-flow sensible-output positive-zero assignment release failed: {self:?}")
    }
}

impl std::error::Error for PurchasedAirCalcCoolingZeroSupplyMassFlowSensibleOutputPositiveZeroAssignmentError {}
