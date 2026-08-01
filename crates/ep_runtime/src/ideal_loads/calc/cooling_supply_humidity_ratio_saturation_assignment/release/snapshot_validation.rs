//! Exact CP377 snapshot validation.

use super::super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot as Snapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentTemperatureOwner as Owner,
    advance_cooling_supply_humidity_ratio_saturation_assignment_state,
};
use crate::ideal_loads::PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot as Predecessor;
use crate::psychrometrics::energyplus_psy_w_fn_tdb_rh_pb;

pub(in crate::ideal_loads) fn cooling_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release(
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
            .supply_temperature_for_saturation_humidity_ratio_c
            .is_some_and(f64::is_finite)
        && snapshot
            .outdoor_barometric_pressure_pa
            .is_some_and(|pressure| pressure.is_finite() && pressure > 0.0)
        && snapshot
            .saturation_supply_humidity_ratio
            .is_some_and(f64::is_finite)
}

pub(super) fn snapshot_links_to_predecessor(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    if snapshot.system != predecessor.system
        || snapshot.parent_call_ordinal != predecessor.parent_call_ordinal
        || snapshot.controlled_zone != predecessor.controlled_zone
        || snapshot.predecessor_dehumidification_control_type
            != predecessor.predecessor_dehumidification_control_type
        || snapshot.predecessor_local_supply_humidity_ratio_original_assignment_performed
            != predecessor.local_supply_humidity_ratio_original_assignment_performed
        || !option_bits_match(
            snapshot.predecessor_resulting_supply_humidity_ratio_original,
            predecessor.resulting_supply_humidity_ratio_original,
        )
    {
        return false;
    }
    let input = if snapshot.purchased_air_supply_temperature_for_saturation_humidity_ratio_read {
        let (Some(supply_temperature_c), Some(outdoor_barometric_pressure_pa), Some(owner)) = (
            snapshot.supply_temperature_for_saturation_humidity_ratio_c,
            snapshot.outdoor_barometric_pressure_pa,
            snapshot_temperature_owner(snapshot),
        ) else {
            return false;
        };
        Some(ActiveInput {
            supply_temperature_c,
            temperature_owner: owner,
            outdoor_barometric_pressure_pa,
        })
    } else {
        None
    };
    let mut state = State::new(predecessor.system);
    advance_cooling_supply_humidity_ratio_saturation_assignment_state(
        &mut state,
        predecessor,
        input,
    )
    .is_some_and(|expected| snapshots_match_bit_exact(expected, snapshot))
}

pub(super) fn snapshot_route(snapshot: Snapshot) -> Option<Route> {
    if snapshot.source
        != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE_ORDER
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
    let owner_count = [
        snapshot.cp334_supply_temperature_mixed_air_limit_owned_read,
        snapshot.cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read,
    ]
    .into_iter()
    .filter(|owner| *owner)
    .count();
    let values = [
        snapshot.supply_temperature_for_saturation_humidity_ratio_c,
        snapshot.outdoor_barometric_pressure_pa,
        snapshot.saturation_supply_humidity_ratio,
        snapshot.assigned_saturation_supply_humidity_ratio,
        snapshot.resulting_saturation_supply_humidity_ratio,
    ];
    let local_shape = if active {
        snapshot.purchased_air_supply_temperature_for_saturation_humidity_ratio_read
            && snapshot.environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read
            && snapshot.psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluated
            && snapshot.local_saturation_supply_humidity_ratio_assignment_performed
            && snapshot.environment_outdoor_barometric_pressure_owned_read
            && owner_count == 1
            && values.into_iter().all(|value| value.is_some())
            && assigned_values_are_exact(snapshot)
    } else {
        !snapshot.purchased_air_supply_temperature_for_saturation_humidity_ratio_read
            && !snapshot.environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read
            && !snapshot.psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluated
            && !snapshot.local_saturation_supply_humidity_ratio_assignment_performed
            && !snapshot.environment_outdoor_barometric_pressure_owned_read
            && owner_count == 0
            && values.into_iter().all(|value| value.is_none())
    };
    let predecessor_shape = if active {
        snapshot.predecessor_local_supply_humidity_ratio_original_assignment_performed
            && snapshot
                .predecessor_resulting_supply_humidity_ratio_original
                .is_some()
            && snapshot.predecessor_dehumidification_control_type.is_some()
            && predecessor_selector_matches_route(snapshot, route)
    } else {
        !snapshot.predecessor_local_supply_humidity_ratio_original_assignment_performed
            && snapshot
                .predecessor_resulting_supply_humidity_ratio_original
                .is_none()
            && snapshot.predecessor_dehumidification_control_type.is_none()
    };
    local_shape && predecessor_shape
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
    let (Some(temperature), Some(pressure), Some(evaluated)) = (
        snapshot.supply_temperature_for_saturation_humidity_ratio_c,
        snapshot.outdoor_barometric_pressure_pa,
        snapshot.saturation_supply_humidity_ratio,
    ) else {
        return false;
    };
    let expected = energyplus_psy_w_fn_tdb_rh_pb(temperature, 1.0, pressure);
    evaluated.to_bits() == expected.to_bits()
        && option_bits_match(
            snapshot.saturation_supply_humidity_ratio,
            snapshot.assigned_saturation_supply_humidity_ratio,
        )
        && option_bits_match(
            snapshot.assigned_saturation_supply_humidity_ratio,
            snapshot.resulting_saturation_supply_humidity_ratio,
        )
}

pub(super) fn snapshot_temperature_owner(snapshot: Snapshot) -> Option<Owner> {
    if snapshot.cp334_supply_temperature_mixed_air_limit_owned_read {
        Some(Owner::Cp334MixedAirLimit)
    } else if snapshot.cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read {
        Some(Owner::Cp344CapacityMixedAirLimit)
    } else {
        None
    }
}

pub(in crate::ideal_loads::calc) fn snapshots_match_bit_exact(
    mut left: Snapshot,
    mut right: Snapshot,
) -> bool {
    let values_match = [
        option_bits_match(
            left.predecessor_resulting_supply_humidity_ratio_original,
            right.predecessor_resulting_supply_humidity_ratio_original,
        ),
        option_bits_match(
            left.supply_temperature_for_saturation_humidity_ratio_c,
            right.supply_temperature_for_saturation_humidity_ratio_c,
        ),
        option_bits_match(
            left.outdoor_barometric_pressure_pa,
            right.outdoor_barometric_pressure_pa,
        ),
        option_bits_match(
            left.saturation_supply_humidity_ratio,
            right.saturation_supply_humidity_ratio,
        ),
        option_bits_match(
            left.assigned_saturation_supply_humidity_ratio,
            right.assigned_saturation_supply_humidity_ratio,
        ),
        option_bits_match(
            left.resulting_saturation_supply_humidity_ratio,
            right.resulting_saturation_supply_humidity_ratio,
        ),
    ]
    .into_iter()
    .all(|matches| matches);
    for snapshot in [&mut left, &mut right] {
        snapshot.predecessor_resulting_supply_humidity_ratio_original = None;
        snapshot.supply_temperature_for_saturation_humidity_ratio_c = None;
        snapshot.outdoor_barometric_pressure_pa = None;
        snapshot.saturation_supply_humidity_ratio = None;
        snapshot.assigned_saturation_supply_humidity_ratio = None;
        snapshot.resulting_saturation_supply_humidity_ratio = None;
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
