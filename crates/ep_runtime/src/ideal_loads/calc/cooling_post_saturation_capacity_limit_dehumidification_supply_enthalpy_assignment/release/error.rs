//! CP417 public release errors.

use ep_model::{DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystemId};

/// Fail-closed CP417 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentError
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
    CoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioAssignmentSnapshotMismatch
    {
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
    SupplyHumidityRatioOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
        bits: u64,
    },
    PsychrometricSupplyEnthalpyOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
        bits: u64,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

impl std::fmt::Display
    for PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentError
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "CP417 supply-enthalpy assignment release failed: {self:?}"
        )
    }
}

impl std::error::Error
    for PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentError
{
}
