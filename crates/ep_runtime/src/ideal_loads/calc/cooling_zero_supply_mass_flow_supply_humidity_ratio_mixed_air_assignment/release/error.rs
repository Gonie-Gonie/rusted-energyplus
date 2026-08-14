//! CP426 public release errors.

use ep_model::{DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystemId};

/// Fail-closed CP426 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyHumidityRatioMixedAirAssignmentError {
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
    CoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentSnapshotMismatch {
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
    MixedAirHumidityRatioOwnerUnavailableOrInconsistent { system: IdealLoadsAirSystemId },
    RuntimeStateInvariantViolation { system: IdealLoadsAirSystemId },
}

impl std::fmt::Display for PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyHumidityRatioMixedAirAssignmentError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "CP426 zero-flow supply-humidity-ratio assignment release failed: {self:?}")
    }
}

impl std::error::Error for PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyHumidityRatioMixedAirAssignmentError {}
