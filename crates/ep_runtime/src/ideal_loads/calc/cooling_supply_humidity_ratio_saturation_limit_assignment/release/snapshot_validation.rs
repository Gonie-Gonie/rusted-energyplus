//! Exact CP378 snapshot and binary64 validation.

use super::super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot as Snapshot,
    advance_cooling_supply_humidity_ratio_saturation_limit_assignment_state,
};
use crate::ideal_loads::PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot as Predecessor;
use crate::ideal_loads::calc::cooling_positive_supply_temperature_mixed_air_limit::source_shaped_two_argument_minimum;

pub(in crate::ideal_loads) fn cooling_supply_humidity_ratio_saturation_limit_assignment_snapshot_is_exact_direct_release(
    snapshot: Snapshot,
) -> bool {
    let Some(route) = snapshot_route(snapshot) else {
        return false;
    };
    if matches!(
        route,
        Route::UnitOff | Route::NonCooling | Route::PositiveGuardFalseFallthrough
    ) {
        return true;
    }
    matches!(
        route,
        Route::HeatingAvailabilityGuardFalseFallthrough
            | Route::HumidificationControlGuardFalseFallthrough
    ) && snapshot.predecessor_dehumidification_control_type
        == Some(ep_model::DehumidificationControlType::None)
        && snapshot
            .saturation_supply_humidity_ratio_for_limit
            .is_some_and(|value| value.is_finite() && value > 0.0)
}

pub(super) fn snapshot_links_to_predecessor(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    if snapshot.system != predecessor.system
        || snapshot.parent_call_ordinal != predecessor.parent_call_ordinal
        || snapshot.controlled_zone != predecessor.controlled_zone
        || snapshot.predecessor_dehumidification_control_type
            != predecessor.predecessor_dehumidification_control_type
        || snapshot.predecessor_local_supply_humidity_ratio_original_assignment_performed
            != predecessor.predecessor_local_supply_humidity_ratio_original_assignment_performed
        || snapshot.predecessor_local_saturation_supply_humidity_ratio_assignment_performed
            != predecessor.local_saturation_supply_humidity_ratio_assignment_performed
        || !option_bits_match(
            snapshot.predecessor_resulting_supply_humidity_ratio_original,
            predecessor.predecessor_resulting_supply_humidity_ratio_original,
        )
        || !option_bits_match(
            snapshot.predecessor_resulting_saturation_supply_humidity_ratio,
            predecessor.resulting_saturation_supply_humidity_ratio,
        )
    {
        return false;
    }
    let mut state = State::new(predecessor.system);
    advance_cooling_supply_humidity_ratio_saturation_limit_assignment_state(&mut state, predecessor)
        .is_some_and(|expected| snapshots_match_bit_exact(expected, snapshot))
}

pub(in crate::ideal_loads::calc) fn snapshot_route(snapshot: Snapshot) -> Option<Route> {
    if snapshot.source
        != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE_ORDER
    {
        return None;
    }
    let flags = [
        snapshot.unit_off_skipped,
        snapshot.non_cooling_skipped,
        snapshot.positive_guard_false_fallthrough_skipped,
        snapshot.heating_availability_guard_false_fallthrough,
        snapshot.humidification_control_guard_false_fallthrough,
        snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        snapshot.dehumidification_control_none_maximum_assignment_executed,
        snapshot.dehumidification_control_guard_false_fallthrough,
    ];
    if flags.into_iter().filter(|flag| *flag).count() != 1 {
        return None;
    }
    let route = if snapshot.unit_off_skipped {
        Route::UnitOff
    } else if snapshot.non_cooling_skipped {
        Route::NonCooling
    } else if snapshot.positive_guard_false_fallthrough_skipped {
        Route::PositiveGuardFalseFallthrough
    } else if snapshot.heating_availability_guard_false_fallthrough {
        Route::HeatingAvailabilityGuardFalseFallthrough
    } else if snapshot.humidification_control_guard_false_fallthrough {
        Route::HumidificationControlGuardFalseFallthrough
    } else if snapshot.dehumidification_control_humidistat_maximum_assignment_executed {
        Route::DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted
    } else if snapshot.dehumidification_control_none_maximum_assignment_executed {
        Route::DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted
    } else {
        Route::DehumidificationControlGuardFalseFallthrough
    };
    snapshot_shape_matches_route(snapshot, route).then_some(route)
}

fn snapshot_shape_matches_route(snapshot: Snapshot, route: Route) -> bool {
    let active = !matches!(
        route,
        Route::UnitOff | Route::NonCooling | Route::PositiveGuardFalseFallthrough
    );
    let flags = [
        snapshot.cp376_original_supply_humidity_ratio_owned_read,
        snapshot.cp377_saturation_supply_humidity_ratio_owned_read,
        snapshot.local_original_supply_humidity_ratio_for_saturation_limit_minimum_read,
        snapshot.local_saturation_supply_humidity_ratio_for_saturation_limit_minimum_read,
        snapshot.source_shaped_two_argument_minimum_evaluated,
        snapshot.purchased_air_supply_humidity_ratio_saturation_limit_assignment_performed,
    ];
    let values = [
        snapshot.predecessor_resulting_supply_humidity_ratio_original,
        snapshot.predecessor_resulting_saturation_supply_humidity_ratio,
        snapshot.original_supply_humidity_ratio_before_saturation_limit,
        snapshot.saturation_supply_humidity_ratio_for_limit,
        snapshot.minimum_supply_humidity_ratio_after_saturation_limit,
        snapshot.assigned_supply_humidity_ratio,
        snapshot.resulting_supply_humidity_ratio,
    ];
    if !active {
        return !snapshot.predecessor_local_supply_humidity_ratio_original_assignment_performed
            && !snapshot.predecessor_local_saturation_supply_humidity_ratio_assignment_performed
            && snapshot.predecessor_dehumidification_control_type.is_none()
            && flags.into_iter().all(|flag| !flag)
            && values.into_iter().all(|value| value.is_none());
    }
    snapshot.predecessor_local_supply_humidity_ratio_original_assignment_performed
        && snapshot.predecessor_local_saturation_supply_humidity_ratio_assignment_performed
        && snapshot.predecessor_dehumidification_control_type.is_some()
        && predecessor_selector_matches_route(snapshot, route)
        && flags.into_iter().all(|flag| flag)
        && values.into_iter().all(|value| value.is_some())
        && assigned_values_are_exact(snapshot)
}

fn predecessor_selector_matches_route(snapshot: Snapshot, route: Route) -> bool {
    use ep_model::DehumidificationControlType::{
        ConstantSensibleHeatRatio, ConstantSupplyHumidityRatio, Humidistat, None as NoneControl,
    };
    match route {
        Route::DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted => {
            snapshot.predecessor_dehumidification_control_type == Some(Humidistat)
        }
        Route::DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted => {
            snapshot.predecessor_dehumidification_control_type == Some(NoneControl)
        }
        Route::DehumidificationControlGuardFalseFallthrough => matches!(
            snapshot.predecessor_dehumidification_control_type,
            Some(ConstantSensibleHeatRatio | ConstantSupplyHumidityRatio)
        ),
        Route::HeatingAvailabilityGuardFalseFallthrough
        | Route::HumidificationControlGuardFalseFallthrough => true,
        _ => false,
    }
}

fn assigned_values_are_exact(snapshot: Snapshot) -> bool {
    let (
        Some(predecessor_original),
        Some(predecessor_saturation),
        Some(original),
        Some(saturation),
        Some(minimum),
        Some(assigned),
        Some(resulting),
    ) = (
        snapshot.predecessor_resulting_supply_humidity_ratio_original,
        snapshot.predecessor_resulting_saturation_supply_humidity_ratio,
        snapshot.original_supply_humidity_ratio_before_saturation_limit,
        snapshot.saturation_supply_humidity_ratio_for_limit,
        snapshot.minimum_supply_humidity_ratio_after_saturation_limit,
        snapshot.assigned_supply_humidity_ratio,
        snapshot.resulting_supply_humidity_ratio,
    )
    else {
        return false;
    };
    let expected = source_shaped_two_argument_minimum(original, saturation);
    predecessor_original.to_bits() == original.to_bits()
        && predecessor_saturation.to_bits() == saturation.to_bits()
        && minimum.to_bits() == expected.to_bits()
        && assigned.to_bits() == minimum.to_bits()
        && resulting.to_bits() == assigned.to_bits()
}

pub(in crate::ideal_loads::calc) fn snapshots_match_bit_exact(
    mut left: Snapshot,
    mut right: Snapshot,
) -> bool {
    let values_match = [
        (
            left.predecessor_resulting_supply_humidity_ratio_original,
            right.predecessor_resulting_supply_humidity_ratio_original,
        ),
        (
            left.predecessor_resulting_saturation_supply_humidity_ratio,
            right.predecessor_resulting_saturation_supply_humidity_ratio,
        ),
        (
            left.original_supply_humidity_ratio_before_saturation_limit,
            right.original_supply_humidity_ratio_before_saturation_limit,
        ),
        (
            left.saturation_supply_humidity_ratio_for_limit,
            right.saturation_supply_humidity_ratio_for_limit,
        ),
        (
            left.minimum_supply_humidity_ratio_after_saturation_limit,
            right.minimum_supply_humidity_ratio_after_saturation_limit,
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
        snapshot.predecessor_resulting_supply_humidity_ratio_original = None;
        snapshot.predecessor_resulting_saturation_supply_humidity_ratio = None;
        snapshot.original_supply_humidity_ratio_before_saturation_limit = None;
        snapshot.saturation_supply_humidity_ratio_for_limit = None;
        snapshot.minimum_supply_humidity_ratio_after_saturation_limit = None;
        snapshot.assigned_supply_humidity_ratio = None;
        snapshot.resulting_supply_humidity_ratio = None;
    }
    values_match && left == right
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
