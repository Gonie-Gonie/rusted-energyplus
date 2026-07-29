//! Exact CP362 snapshot, lineage, and binary64 validation.

use super::super::{
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitRetainedRoute as Route,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitSnapshot as Snapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitSnapshot as Predecessor;
use crate::ideal_loads::calc::cooling_positive_supply_temperature_mixed_air_limit::source_shaped_two_argument_minimum;
use crate::ideal_loads::calc::cooling_humidistat_supply_humidity_ratio_mixed_air_limit::transition::{
    predecessor_route,
};

mod route;

pub(in crate::ideal_loads) fn cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshot_is_exact_direct_release(
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

pub(in crate::ideal_loads::calc::cooling_humidistat_supply_humidity_ratio_mixed_air_limit) fn snapshot_route(
    snapshot: Snapshot,
) -> Option<Route> {
    let route = route::structural_route(snapshot)?;
    values_fit_route(snapshot, route).then_some(route)
}

pub(in crate::ideal_loads) fn cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshots_match_bit_exact(
    mut left: Snapshot,
    mut right: Snapshot,
) -> bool {
    let values_match = [
        (
            left.predecessor_resulting_supply_humidity_ratio_for_dehumidification,
            right.predecessor_resulting_supply_humidity_ratio_for_dehumidification,
        ),
        (
            left.mixed_air_humidity_ratio,
            right.mixed_air_humidity_ratio,
        ),
        (
            left.supply_humidity_ratio_for_dehumidification_before_mixed_air_limit,
            right.supply_humidity_ratio_for_dehumidification_before_mixed_air_limit,
        ),
        (
            left.minimum_supply_humidity_ratio,
            right.minimum_supply_humidity_ratio,
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
        snapshot.predecessor_resulting_supply_humidity_ratio_for_dehumidification = None;
        snapshot.mixed_air_humidity_ratio = None;
        snapshot.supply_humidity_ratio_for_dehumidification_before_mixed_air_limit = None;
        snapshot.minimum_supply_humidity_ratio = None;
        snapshot.assigned_supply_humidity_ratio = None;
        snapshot.resulting_supply_humidity_ratio = None;
    }
    values_match && left == right
}

pub(in crate::ideal_loads) fn cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshot_links_to_predecessor(
    snapshot: Snapshot,
    predecessor: Predecessor,
) -> bool {
    let Some(route) = predecessor_route(predecessor) else {
        return false;
    };
    snapshot_route(snapshot) == Some(route)
        && snapshot.system == predecessor.system
        && snapshot.parent_call_ordinal == predecessor.parent_call_ordinal
        && snapshot.controlled_zone == predecessor.controlled_zone
        && snapshot.unit_body_entered == predecessor.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
            == predecessor.predecessor_cooling_body_entered
        && snapshot.predecessor_no_outdoor_air_fallback_entered
            == predecessor.predecessor_no_outdoor_air_fallback_entered
        && snapshot.predecessor_positive_supply_mass_flow_body_entered
            == predecessor.predecessor_positive_supply_mass_flow_body_entered
        && snapshot.unit_off_skipped == predecessor.unit_off_skipped
        && snapshot.non_cooling_skipped == predecessor.non_cooling_skipped
        && snapshot.positive_guard_false_fallthrough_skipped
            == predecessor.positive_guard_false_fallthrough_skipped
        && snapshot.predecessor_dehumidification_control_type
            == predecessor.predecessor_dehumidification_control_type
        && snapshot.predecessor_dehumidification_control_none_case_completed_skip
            == predecessor.dehumidification_control_none_case_completed_skip
        && snapshot
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
            == predecessor.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
        && snapshot
            .predecessor_dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_executed
            == predecessor
                .dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_executed
        && snapshot
            .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
            == predecessor.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
        && option_bits_match(
            snapshot.predecessor_resulting_supply_humidity_ratio_for_dehumidification,
            predecessor.resulting_supply_humidity_ratio_for_dehumidification,
        )
}

fn values_fit_route(snapshot: Snapshot, route: Route) -> bool {
    let active =
        route == Route::DehumidificationControlHumidistatSupplyHumidityRatioMixedAirLimitExecuted;
    let flags = [
        snapshot.mixed_air_humidity_ratio_for_minimum_read,
        snapshot.supply_humidity_ratio_for_dehumidification_for_mixed_air_limit_minimum_read,
        snapshot.source_shaped_two_argument_minimum_evaluated,
        snapshot.supply_humidity_ratio_assignment_performed,
    ];
    let values = [
        snapshot.predecessor_resulting_supply_humidity_ratio_for_dehumidification,
        snapshot.mixed_air_humidity_ratio,
        snapshot.supply_humidity_ratio_for_dehumidification_before_mixed_air_limit,
        snapshot.minimum_supply_humidity_ratio,
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
        Some(predecessor_local),
        Some(mixed),
        Some(local),
        Some(minimum),
        Some(assigned),
        Some(resulting),
    ) = (
        snapshot.predecessor_resulting_supply_humidity_ratio_for_dehumidification,
        snapshot.mixed_air_humidity_ratio,
        snapshot.supply_humidity_ratio_for_dehumidification_before_mixed_air_limit,
        snapshot.minimum_supply_humidity_ratio,
        snapshot.assigned_supply_humidity_ratio,
        snapshot.resulting_supply_humidity_ratio,
    )
    else {
        return false;
    };
    let expected = source_shaped_two_argument_minimum(mixed, local);
    predecessor_local.to_bits() == local.to_bits()
        && minimum.to_bits() == expected.to_bits()
        && assigned.to_bits() == minimum.to_bits()
        && resulting.to_bits() == assigned.to_bits()
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
