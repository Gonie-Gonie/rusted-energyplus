//! CP380 release errors.

use ep_model::{DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystemId};

use crate::ideal_loads::PurchasedAirUnitRuntimeState;

/// Fail-closed CP380 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardError {
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
    CoolingSupplyEnthalpyPostSaturationAssignmentSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    CoolingLimitSelectorLineageMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cp337_transition_count: usize,
        cp379_transition_count: usize,
        cp380_transition_count: usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

impl std::fmt::Display for PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "CP380 post-saturation capacity-limit guard release failed: {self:?}"
        )
    }
}

impl std::error::Error for PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardError {}

pub(super) fn predecessor_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardError {
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardError::
        CoolingSupplyEnthalpyPostSaturationAssignmentSnapshotMismatch { system }
}

pub(super) fn selector_lineage_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardError {
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardError::
        CoolingLimitSelectorLineageMismatch { system }
}

pub(super) fn call_order_error(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardError {
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        cp337_transition_count: unit
            .calc_cooling_positive_supply_capacity_limit_guard
            .transition_count,
        cp379_transition_count: unit
            .calc_cooling_supply_enthalpy_post_saturation_assignment
            .transition_count,
        cp380_transition_count: unit
            .calc_cooling_post_saturation_capacity_limit_guard
            .transition_count,
    }
}
