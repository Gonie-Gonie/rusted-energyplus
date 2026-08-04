//! CP415 public release errors.

use ep_model::{DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystemId};

/// Fail-closed CP415 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitError
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
    CoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentSnapshotMismatch
    {
        system: IdealLoadsAirSystemId,
    },
    CoolingMixedAirCallSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        predecessor_transition_count: usize,
        transition_count: usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    SupplyTemperatureOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
        bits: u64,
    },
    MixedAirTemperatureOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
        bits: u64,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

impl std::fmt::Display for PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "CP415 saturation-temperature mixed-air-limit release failed: {self:?}")
    }
}

impl std::error::Error for PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitError {}
