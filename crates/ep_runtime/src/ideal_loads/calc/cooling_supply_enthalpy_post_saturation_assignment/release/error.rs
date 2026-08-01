//! CP379 release errors.

use ep_model::{DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystemId};

use crate::ideal_loads::PurchasedAirUnitRuntimeState;

/// Fail-closed CP379 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentError {
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
    CoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    CoolingSupplyTemperatureOwnerMismatch {
        system: IdealLoadsAirSystemId,
    },
    InvalidSupplyTemperature {
        system: IdealLoadsAirSystemId,
    },
    InvalidSupplyHumidityRatio {
        system: IdealLoadsAirSystemId,
    },
    InvalidPsychrometricSupplyEnthalpy {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cp377_transition_count: usize,
        cp378_transition_count: usize,
        cp379_transition_count: usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

impl std::fmt::Display for PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "CP379 post-saturation enthalpy assignment release failed: {self:?}"
        )
    }
}

impl std::error::Error for PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentError {}

pub(super) fn predecessor_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentError {
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentError::CoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshotMismatch { system }
}

pub(super) fn temperature_owner_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentError {
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentError::CoolingSupplyTemperatureOwnerMismatch { system }
}

pub(super) fn call_order_error(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentError {
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        cp377_transition_count: unit
            .calc_cooling_supply_humidity_ratio_saturation_assignment
            .transition_count,
        cp378_transition_count: unit
            .calc_cooling_supply_humidity_ratio_saturation_limit_assignment
            .transition_count,
        cp379_transition_count: unit
            .calc_cooling_supply_enthalpy_post_saturation_assignment
            .transition_count,
    }
}
