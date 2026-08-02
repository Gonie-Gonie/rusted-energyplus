//! CP391 public release errors.

use ep_model::{DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystemId};

use crate::ideal_loads::PurchasedAirUnitRuntimeState;

/// Fail-closed CP391 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitError
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
    CoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitSnapshotMismatch
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
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

pub(super) fn predecessor_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitError{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitError::CoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitSnapshotMismatch { system }
}

pub(super) fn call_order_error(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitError{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        predecessor_transition_count: unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit
            .transition_count,
        transition_count: unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit
            .transition_count,
    }
}
