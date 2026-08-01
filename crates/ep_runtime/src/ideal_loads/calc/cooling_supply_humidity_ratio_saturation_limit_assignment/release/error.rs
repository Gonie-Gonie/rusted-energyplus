//! CP378 release errors.

use ep_model::{DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystemId};

use crate::ideal_loads::PurchasedAirUnitRuntimeState;

/// Fail-closed CP378 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentError {
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
    CoolingSupplyHumidityRatioSaturationAssignmentSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    OriginalSupplyHumidityRatioOwnerMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cp377_transition_count: usize,
        cp378_transition_count: usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

impl std::fmt::Display
    for PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentError
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "CP378 saturation-limit assignment release failed: {self:?}"
        )
    }
}

impl std::error::Error
    for PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentError
{
}

pub(super) fn predecessor_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentError {
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentError::CoolingSupplyHumidityRatioSaturationAssignmentSnapshotMismatch { system }
}

pub(super) fn original_owner_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentError {
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentError::OriginalSupplyHumidityRatioOwnerMismatch { system }
}

pub(super) fn call_order_error(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentError {
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        cp377_transition_count: unit
            .calc_cooling_supply_humidity_ratio_saturation_assignment
            .transition_count,
        cp378_transition_count: unit
            .calc_cooling_supply_humidity_ratio_saturation_limit_assignment
            .transition_count,
    }
}
