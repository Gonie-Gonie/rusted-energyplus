//! Exact CP375 snapshot and binary64 validation.

use super::super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::calc::cooling_positive_supply_temperature_minimum_limit::source_shaped_two_argument_maximum;

mod route;

pub(in crate::ideal_loads) fn cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_snapshot_is_exact_direct_release(
    snapshot: Snapshot,
) -> bool {
    matches!(
        snapshot_route(snapshot),
        Some(
            Route::UnitOff
                | Route::NonCooling
                | Route::PositiveGuardFalseFallthrough
                | Route::HeatingAvailabilityGuardFalseFallthrough
                | Route::HumidificationControlGuardFalseFallthrough
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
            left.predecessor_resulting_supply_humidity_ratio_for_humidification,
            right.predecessor_resulting_supply_humidity_ratio_for_humidification,
        ),
        (
            left.purchased_air_supply_humidity_ratio_before_humidification_supply_maximum,
            right.purchased_air_supply_humidity_ratio_before_humidification_supply_maximum,
        ),
        (
            left.supply_humidity_ratio_for_humidification_for_supply_maximum,
            right.supply_humidity_ratio_for_humidification_for_supply_maximum,
        ),
        (
            left.maximum_supply_humidity_ratio,
            right.maximum_supply_humidity_ratio,
        ),
        (
            left.assigned_supply_humidity_ratio,
            right.assigned_supply_humidity_ratio,
        ),
        (
            left.resulting_supply_humidity_ratio,
            right.resulting_supply_humidity_ratio,
        ),
    ]
    .into_iter()
    .all(|(left, right)| option_bits_match(left, right));
    for snapshot in [&mut left, &mut right] {
        snapshot.predecessor_resulting_supply_humidity_ratio_for_humidification = None;
        snapshot.purchased_air_supply_humidity_ratio_before_humidification_supply_maximum = None;
        snapshot.supply_humidity_ratio_for_humidification_for_supply_maximum = None;
        snapshot.maximum_supply_humidity_ratio = None;
        snapshot.assigned_supply_humidity_ratio = None;
        snapshot.resulting_supply_humidity_ratio = None;
    }
    values_match && left == right
}

fn values_fit_route(snapshot: Snapshot, route: Route) -> bool {
    let active = matches!(
        route,
        Route::DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted
            | Route::DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted
    );
    let flags = [
        snapshot.purchased_air_supply_humidity_ratio_for_humidification_supply_maximum_read,
        snapshot.supply_humidity_ratio_for_humidification_for_supply_maximum_read,
        snapshot.source_shaped_two_argument_maximum_evaluated,
        snapshot.purchased_air_supply_humidity_ratio_assignment_performed,
    ];
    let values = [
        snapshot.predecessor_resulting_supply_humidity_ratio_for_humidification,
        snapshot.purchased_air_supply_humidity_ratio_before_humidification_supply_maximum,
        snapshot.supply_humidity_ratio_for_humidification_for_supply_maximum,
        snapshot.maximum_supply_humidity_ratio,
        snapshot.assigned_supply_humidity_ratio,
        snapshot.resulting_supply_humidity_ratio,
    ];
    if !active {
        return flags.into_iter().all(|flag| !flag)
            && values.into_iter().all(|value| value.is_none());
    }
    if !flags.into_iter().all(|flag| flag) {
        return false;
    }
    let (
        Some(predecessor_right),
        Some(left),
        Some(right),
        Some(maximum),
        Some(assigned),
        Some(resulting),
    ) = (
        snapshot.predecessor_resulting_supply_humidity_ratio_for_humidification,
        snapshot.purchased_air_supply_humidity_ratio_before_humidification_supply_maximum,
        snapshot.supply_humidity_ratio_for_humidification_for_supply_maximum,
        snapshot.maximum_supply_humidity_ratio,
        snapshot.assigned_supply_humidity_ratio,
        snapshot.resulting_supply_humidity_ratio,
    ) else {
        return false;
    };
    let expected = source_shaped_two_argument_maximum(left, right);
    predecessor_right.to_bits() == right.to_bits()
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
