//! Model-bound CP378 transition adapter and numerical reconciliation.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    DirectZonePurchasedAirCouplingOutput,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_saturation_limit_assignment,
};

use super::DirectZonePurchasedAirScheduledCouplingError;

pub(in crate::ideal_loads) fn advance_cooling_supply_humidity_ratio_saturation_limit_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot,
) -> Result<
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot,
    DirectZonePurchasedAirScheduledCouplingError,
> {
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_saturation_limit_assignment(
        runtime,
        system,
        predecessor,
    )
    .map_err(
        DirectZonePurchasedAirScheduledCouplingError::
            CalculationCoolingSupplyHumidityRatioSaturationLimitAssignment,
    )
}

pub(in crate::ideal_loads) fn reconcile_cooling_supply_humidity_ratio_saturation_limit_assignment(
    snapshot: PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot,
    coupling: &DirectZonePurchasedAirCouplingOutput,
) -> Result<(), DirectZonePurchasedAirScheduledCouplingError> {
    if snapshot.unit_off_skipped
        || snapshot.non_cooling_skipped
        || snapshot.positive_guard_false_fallthrough_skipped
    {
        return Ok(());
    }

    let minimum = required_bits(
        snapshot.minimum_supply_humidity_ratio_after_saturation_limit,
        "minimum_supply_humidity_ratio_after_saturation_limit",
    )?;
    let assigned = required_bits(
        snapshot.assigned_supply_humidity_ratio,
        "assigned_supply_humidity_ratio",
    )?;
    let resulting = required_bits(
        snapshot.resulting_supply_humidity_ratio,
        "resulting_supply_humidity_ratio",
    )?;
    if minimum != resulting {
        return mismatch("minimum_supply_humidity_ratio_after_saturation_limit");
    }
    if assigned != resulting {
        return mismatch("assigned_supply_humidity_ratio");
    }

    for (field, value) in [
        (
            "coupling.purchased_air.calculation.supply_humidity_ratio",
            coupling.purchased_air.calculation.supply_humidity_ratio,
        ),
        (
            "coupling.purchased_air.supply_node_update.humidity_ratio",
            coupling.purchased_air.supply_node_update.humidity_ratio,
        ),
        (
            "coupling.purchased_air.report.supply_humidity_ratio",
            coupling.purchased_air.report.supply_humidity_ratio,
        ),
    ] {
        if value.to_bits() != resulting {
            return mismatch(field);
        }
    }
    Ok(())
}

fn required_bits(
    value: Option<f64>,
    field: &'static str,
) -> Result<u64, DirectZonePurchasedAirScheduledCouplingError> {
    value.map(f64::to_bits).ok_or(
        DirectZonePurchasedAirScheduledCouplingError::
            CalculationCoolingSupplyHumidityRatioSaturationLimitAssignmentNumericalInvariant {
                field,
            },
    )
}

fn mismatch<T>(field: &'static str) -> Result<T, DirectZonePurchasedAirScheduledCouplingError> {
    Err(
        DirectZonePurchasedAirScheduledCouplingError::
            CalculationCoolingSupplyHumidityRatioSaturationLimitAssignmentNumericalInvariant {
                field,
            },
    )
}
