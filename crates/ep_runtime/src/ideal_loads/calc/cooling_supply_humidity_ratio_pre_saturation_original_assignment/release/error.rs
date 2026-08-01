//! CP376 release errors.

use ep_model::{DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystemId};

use crate::ideal_loads::PurchasedAirUnitRuntimeState;

/// Fail-closed CP376 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentError {
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
    CoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    CoolingDehumidificationControlNoneCaseOwnerMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cp375_transition_count: usize,
        cp376_transition_count: usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

impl std::fmt::Display
    for PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentError
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "CP376 pre-saturation original-assignment release failed: {self:?}"
        )
    }
}

impl std::error::Error
    for PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentError
{
}

pub(super) fn predecessor_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentError {
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentError::CoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentSnapshotMismatch { system }
}

pub(super) fn owner_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentError {
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentError::CoolingDehumidificationControlNoneCaseOwnerMismatch { system }
}

pub(super) fn call_order_error(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentError {
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        cp375_transition_count: unit
            .calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment
            .transition_count,
        cp376_transition_count: unit
            .calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment
            .transition_count,
    }
}
