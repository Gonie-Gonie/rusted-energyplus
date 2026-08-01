//! CP373 release error construction.

use ep_model::{
    DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystemId,
};

use crate::ideal_loads::PurchasedAirUnitRuntimeState;

/// Fail-closed CP373 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentError {
    UnknownSystem { system: IdealLoadsAirSystemId },
    InitializationNotReady { system: IdealLoadsAirSystemId },
    SystemIdentityMismatch {
        expected: IdealLoadsAirSystemId,
        actual: IdealLoadsAirSystemId,
    },
    SystemOutsideDirectSubset { system: IdealLoadsAirSystemId },
    DehumidificationControlTypeOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
        actual: DehumidificationControlType,
    },
    HumidificationControlTypeOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
        actual: HumidificationControlType,
    },
    CoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_transition_count: usize,
        cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_transition_count: usize,
    },
    PredecessorOutsideDirectSubset { system: IdealLoadsAirSystemId },
    RuntimeStateInvariantViolation { system: IdealLoadsAirSystemId },
}

impl std::fmt::Display for PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "CP373 humidification humidity-ratio assignment release failed: {self:?}")
    }
}

impl std::error::Error for PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentError {}

pub(super) fn predecessor_mismatch(system: IdealLoadsAirSystemId) -> PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentError {
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentError::CoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentSnapshotMismatch { system }
}

pub(super) fn call_order_error(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentError {
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_transition_count:
            unit.calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment.transition_count,
        cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_transition_count:
            unit.calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment.transition_count,
    }
}
