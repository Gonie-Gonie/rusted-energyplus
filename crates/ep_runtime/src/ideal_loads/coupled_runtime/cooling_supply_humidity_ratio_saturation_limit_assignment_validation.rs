//! Coupled-runtime validation for CP378 saturation-limit assignment evidence.

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot as PredecessorSnapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot as Snapshot,
    cooling_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release,
    cooling_supply_humidity_ratio_saturation_limit_assignment_snapshot_is_exact_direct_release,
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
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    lifecycle::validate(
        lifecycle,
        predecessor,
        timestep_count,
        latest_output,
        binding,
    )
}

fn violation(field: &'static str, expected: usize, actual: usize) -> Error {
    Error::CalcCoolingSupplyHumidityRatioSaturationLimitAssignmentLifecycleInvariant {
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
