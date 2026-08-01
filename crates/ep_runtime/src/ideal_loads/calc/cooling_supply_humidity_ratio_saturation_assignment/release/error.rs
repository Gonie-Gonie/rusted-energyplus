//! CP377 release errors.

use ep_model::{DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystemId};

use crate::ideal_loads::PurchasedAirUnitRuntimeState;

/// Fail-closed CP377 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentError {
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
    CoolingSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    SupplyTemperatureOwnerMismatch {
        system: IdealLoadsAirSystemId,
    },
    SupplyTemperatureOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
        bits: u64,
    },
    BarometricPressureOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
        bits: u64,
    },
    SaturationHumidityRatioOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
        bits: u64,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cp376_transition_count: usize,
        cp377_transition_count: usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

impl std::fmt::Display for PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "CP377 saturation-humidity-ratio assignment release failed: {self:?}"
        )
    }
}

impl std::error::Error for PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentError {}

pub(super) fn predecessor_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentError {
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentError::CoolingSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshotMismatch { system }
}

pub(super) fn owner_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentError {
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentError::SupplyTemperatureOwnerMismatch {
        system,
    }
}

pub(super) fn call_order_error(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentError {
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        cp376_transition_count: unit
            .calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment
            .transition_count,
        cp377_transition_count: unit
            .calc_cooling_supply_humidity_ratio_saturation_assignment
            .transition_count,
    }
}
