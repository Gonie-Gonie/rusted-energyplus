//! Exact CP361 snapshot and binary64 validation.

use super::super::{
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitRetainedRoute as Route,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitSnapshot as Snapshot,
};
use crate::ideal_loads::calc::cooling_positive_supply_temperature_minimum_limit::source_shaped_two_argument_maximum;

mod route;

pub(in crate::ideal_loads) fn cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_snapshot_is_exact_direct_release(
    snapshot: Snapshot,
) -> bool {
    matches!(
        snapshot_route(snapshot),
        Some(
            Route::UnitOff
                | Route::NonCooling
                | Route::PositiveGuardFalseFallthrough
                | Route::DehumidificationControlNoneCaseCompletedSkip
        )
    )
}

pub(super) fn snapshot_route(snapshot: Snapshot) -> Option<Route> {
    let route = route::structural_route(snapshot)?;
    values_fit_route(snapshot, route).then_some(route)
}

pub(in crate::ideal_loads::calc) fn snapshots_match_bit_exact(
    mut left: Snapshot,
    mut right: Snapshot,
) -> bool {
    let values_match = [
        (
            left.predecessor_resulting_supply_humidity_ratio_for_dehumidification,
            right.predecessor_resulting_supply_humidity_ratio_for_dehumidification,
        ),
        (
            left.supply_humidity_ratio_for_dehumidification_before_minimum_limit,
            right.supply_humidity_ratio_for_dehumidification_before_minimum_limit,
        ),
        (
            left.minimum_cooling_supply_air_humidity_ratio,
            right.minimum_cooling_supply_air_humidity_ratio,
        ),
        (
            left.maximum_supply_humidity_ratio_for_dehumidification,
            right.maximum_supply_humidity_ratio_for_dehumidification,
        ),
        (
            left.assigned_supply_humidity_ratio_for_dehumidification,
            right.assigned_supply_humidity_ratio_for_dehumidification,
        ),
        (
            left.resulting_supply_humidity_ratio_for_dehumidification,
            right.resulting_supply_humidity_ratio_for_dehumidification,
        ),
    ]
    .into_iter()
    .all(|(left, right)| option_bits_match(left, right));
    for snapshot in [&mut left, &mut right] {
        snapshot.predecessor_resulting_supply_humidity_ratio_for_dehumidification = None;
        snapshot.supply_humidity_ratio_for_dehumidification_before_minimum_limit = None;
        snapshot.minimum_cooling_supply_air_humidity_ratio = None;
        snapshot.maximum_supply_humidity_ratio_for_dehumidification = None;
        snapshot.assigned_supply_humidity_ratio_for_dehumidification = None;
        snapshot.resulting_supply_humidity_ratio_for_dehumidification = None;
    }
    values_match && left == right
}

fn values_fit_route(snapshot: Snapshot, route: Route) -> bool {
    let active = route
        == Route::DehumidificationControlHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitExecuted;
    let flags = [
        snapshot.supply_humidity_ratio_for_dehumidification_for_minimum_limit_maximum_read,
        snapshot.minimum_cooling_supply_air_humidity_ratio_for_maximum_read,
        snapshot.source_shaped_two_argument_maximum_evaluated,
        snapshot.supply_humidity_ratio_for_dehumidification_assignment_performed,
    ];
    let values = [
        snapshot.predecessor_resulting_supply_humidity_ratio_for_dehumidification,
        snapshot.supply_humidity_ratio_for_dehumidification_before_minimum_limit,
        snapshot.minimum_cooling_supply_air_humidity_ratio,
        snapshot.maximum_supply_humidity_ratio_for_dehumidification,
        snapshot.assigned_supply_humidity_ratio_for_dehumidification,
        snapshot.resulting_supply_humidity_ratio_for_dehumidification,
    ];
    if !active {
        return flags.into_iter().all(|flag| !flag)
            && values.into_iter().all(|value| value.is_none());
    }
    if !flags.into_iter().all(|flag| flag) {
        return false;
    }
    let (
        Some(predecessor_left),
        Some(left),
        Some(right),
        Some(maximum),
        Some(assigned),
        Some(resulting),
    ) = (
        snapshot.predecessor_resulting_supply_humidity_ratio_for_dehumidification,
        snapshot.supply_humidity_ratio_for_dehumidification_before_minimum_limit,
        snapshot.minimum_cooling_supply_air_humidity_ratio,
        snapshot.maximum_supply_humidity_ratio_for_dehumidification,
        snapshot.assigned_supply_humidity_ratio_for_dehumidification,
        snapshot.resulting_supply_humidity_ratio_for_dehumidification,
    )
    else {
        return false;
    };
    let expected = source_shaped_two_argument_maximum(left, right);
    predecessor_left.to_bits() == left.to_bits()
        && maximum.to_bits() == expected.to_bits()
        && assigned.to_bits() == maximum.to_bits()
        && resulting.to_bits() == assigned.to_bits()
}

pub(super) fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
