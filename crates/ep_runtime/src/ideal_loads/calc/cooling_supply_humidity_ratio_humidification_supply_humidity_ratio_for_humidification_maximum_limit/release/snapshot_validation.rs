//! Exact CP374 snapshot and binary64 validation.

use super::super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitSnapshot as Snapshot,
};
use crate::ideal_loads::calc::cooling_positive_supply_temperature_mixed_air_limit::source_shaped_two_argument_minimum;

mod route;

pub(in crate::ideal_loads) fn cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_snapshot_is_exact_direct_release(
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
            left.supply_humidity_ratio_for_humidification_before_maximum_limit,
            right.supply_humidity_ratio_for_humidification_before_maximum_limit,
        ),
        (
            left.maximum_heating_supply_air_humidity_ratio,
            right.maximum_heating_supply_air_humidity_ratio,
        ),
        (
            left.minimum_supply_humidity_ratio_for_humidification,
            right.minimum_supply_humidity_ratio_for_humidification,
        ),
        (
            left.assigned_supply_humidity_ratio_for_humidification,
            right.assigned_supply_humidity_ratio_for_humidification,
        ),
        (
            left.resulting_supply_humidity_ratio_for_humidification,
            right.resulting_supply_humidity_ratio_for_humidification,
        ),
    ]
    .into_iter()
    .all(|(left, right)| option_bits_match(left, right));
    for snapshot in [&mut left, &mut right] {
        snapshot.predecessor_resulting_supply_humidity_ratio_for_humidification = None;
        snapshot.supply_humidity_ratio_for_humidification_before_maximum_limit = None;
        snapshot.maximum_heating_supply_air_humidity_ratio = None;
        snapshot.minimum_supply_humidity_ratio_for_humidification = None;
        snapshot.assigned_supply_humidity_ratio_for_humidification = None;
        snapshot.resulting_supply_humidity_ratio_for_humidification = None;
    }
    values_match && left == right
}

fn values_fit_route(snapshot: Snapshot, route: Route) -> bool {
    let active = matches!(
        route,
        Route::DehumidificationControlHumidistatSupplyHumidityRatioForHumidificationMaximumLimitExecuted
            | Route::DehumidificationControlNoneSupplyHumidityRatioForHumidificationMaximumLimitExecuted
    );
    let flags = [
        snapshot.supply_humidity_ratio_for_humidification_for_maximum_limit_minimum_read,
        snapshot.maximum_heating_supply_air_humidity_ratio_for_minimum_read,
        snapshot.source_shaped_two_argument_minimum_evaluated,
        snapshot.supply_humidity_ratio_for_humidification_assignment_performed,
    ];
    let values = [
        snapshot.predecessor_resulting_supply_humidity_ratio_for_humidification,
        snapshot.supply_humidity_ratio_for_humidification_before_maximum_limit,
        snapshot.maximum_heating_supply_air_humidity_ratio,
        snapshot.minimum_supply_humidity_ratio_for_humidification,
        snapshot.assigned_supply_humidity_ratio_for_humidification,
        snapshot.resulting_supply_humidity_ratio_for_humidification,
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
        Some(minimum),
        Some(assigned),
        Some(resulting),
    ) = (
        snapshot.predecessor_resulting_supply_humidity_ratio_for_humidification,
        snapshot.supply_humidity_ratio_for_humidification_before_maximum_limit,
        snapshot.maximum_heating_supply_air_humidity_ratio,
        snapshot.minimum_supply_humidity_ratio_for_humidification,
        snapshot.assigned_supply_humidity_ratio_for_humidification,
        snapshot.resulting_supply_humidity_ratio_for_humidification,
    ) else {
        return false;
    };
    let expected = source_shaped_two_argument_minimum(left, right);
    predecessor_left.to_bits() == left.to_bits()
        && minimum.to_bits() == expected.to_bits()
        && assigned.to_bits() == minimum.to_bits()
        && resulting.to_bits() == assigned.to_bits()
}

pub(super) fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
