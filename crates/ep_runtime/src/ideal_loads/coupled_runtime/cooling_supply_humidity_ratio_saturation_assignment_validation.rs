//! Coupled-runtime validation for CP377 saturation-assignment evidence.

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitLifecycleSummary as Cp344Lifecycle,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot as Cp344Snapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitLifecycleSummary as Cp334Lifecycle,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot as Cp334Snapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot as PredecessorSnapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot as Snapshot,
    cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release,
    cooling_positive_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release,
    cooling_supply_humidity_ratio_pre_saturation_original_assignment_snapshot_is_exact_direct_release,
    cooling_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

mod lifecycle;
mod snapshot;

pub(in crate::ideal_loads) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    snapshot::matches_release(output, call_ordinal, binding)
}

#[cfg(test)]
pub(in crate::ideal_loads) fn snapshots_match_exact_bits(left: Snapshot, right: Snapshot) -> bool {
    snapshot::snapshots_match_exact_bits(left, right)
}

pub(in crate::ideal_loads) fn validate_lifecycle(
    lifecycle: &Lifecycle,
    predecessor: &PredecessorLifecycle,
    cp334: &Cp334Lifecycle,
    cp344: &Cp344Lifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    lifecycle::validate(
        lifecycle,
        predecessor,
        cp334,
        cp344,
        timestep_count,
        latest_output,
        binding,
    )
}

fn violation(field: &'static str, expected: usize, actual: usize) -> Error {
    Error::CalcCoolingSupplyHumidityRatioSaturationAssignmentLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

fn option_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
