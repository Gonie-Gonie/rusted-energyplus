//! CP381 release errors.

use ep_model::{DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystemId};

use crate::ideal_loads::PurchasedAirUnitRuntimeState;

/// Fail-closed CP381 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardError {
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
    CoolingPostSaturationCapacityLimitGuardSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    SupplyHumidityRatioOwnerLineageMismatch {
        system: IdealLoadsAirSystemId,
    },
    MixedAirHumidityRatioOwnerLineageMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cp329_transition_count: usize,
        cp378_transition_count: usize,
        cp379_transition_count: usize,
        cp380_transition_count: usize,
        cp381_transition_count: usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

impl std::fmt::Display
    for PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardError
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "CP381 post-saturation capacity-limit dehumidification guard release failed: {self:?}"
        )
    }
}

impl std::error::Error
    for PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardError
{
}

pub(super) fn predecessor_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardError {
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardError::
        CoolingPostSaturationCapacityLimitGuardSnapshotMismatch { system }
}

pub(super) fn call_order_error(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardError {
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardError::
        PredecessorCallOrder {
            system,
            init_call_count: unit.init_call_count,
            calculation_entry_call_count: unit.calc_entry.call_count,
            cp329_transition_count: unit.calc_cooling_mixed_air_call.transition_count,
            cp378_transition_count: unit
                .calc_cooling_supply_humidity_ratio_saturation_limit_assignment
                .transition_count,
            cp379_transition_count: unit
                .calc_cooling_supply_enthalpy_post_saturation_assignment
                .transition_count,
            cp380_transition_count: unit
                .calc_cooling_post_saturation_capacity_limit_guard
                .transition_count,
            cp381_transition_count: unit
                .calc_cooling_post_saturation_capacity_limit_dehumidification_guard
                .transition_count,
        }
}
