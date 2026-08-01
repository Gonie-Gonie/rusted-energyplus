//! Exact CP379 snapshot and binary64 validation.

use super::super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot as Snapshot,
    advance_cooling_supply_enthalpy_post_saturation_assignment_state,
};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_saturation_assignment::PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentTemperatureOwner as TemperatureOwner;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot as TemperaturePrefix,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot as Predecessor,
};
use crate::psychrometrics::energyplus_psy_h_fn_tdb_w;

pub(in crate::ideal_loads) fn cooling_supply_enthalpy_post_saturation_assignment_snapshot_is_exact_direct_release(
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
        && snapshot.supply_temperature_c.is_some_and(f64::is_finite)
        && snapshot
            .supply_humidity_ratio
            .is_some_and(|value| value.is_finite() && value >= 0.0)
        && snapshot
            .psychrometric_supply_enthalpy_j_per_kg
            .is_some_and(f64::is_finite)
}

/// Accepts every exact CP379 route, including private active lineages.
#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn snapshot_is_exact(snapshot: Snapshot) -> bool {
    snapshot_route(snapshot).is_some()
}

pub(super) fn snapshot_links_to_prefix(
    snapshot: Snapshot,
    predecessor: Predecessor,
    temperature_prefix: TemperaturePrefix,
) -> bool {
    if snapshot.system != predecessor.system
        || snapshot.parent_call_ordinal != predecessor.parent_call_ordinal
        || snapshot.controlled_zone != predecessor.controlled_zone
        || snapshot.predecessor_dehumidification_control_type
            != predecessor.predecessor_dehumidification_control_type
        || snapshot.predecessor_supply_humidity_ratio_saturation_limit_assignment_performed
            != predecessor.purchased_air_supply_humidity_ratio_saturation_limit_assignment_performed
        || !option_bits_match(
            snapshot.predecessor_resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
        || temperature_prefix.system != predecessor.system
        || temperature_prefix.parent_call_ordinal != predecessor.parent_call_ordinal
        || temperature_prefix.controlled_zone != predecessor.controlled_zone
    {
        return false;
    }
    let active =
        predecessor.purchased_air_supply_humidity_ratio_saturation_limit_assignment_performed;
    let input = if active {
        let Some(owner) = temperature_owner(temperature_prefix) else {
            return false;
        };
        let Some(supply_temperature_c) =
            temperature_prefix.supply_temperature_for_saturation_humidity_ratio_c
        else {
            return false;
        };
        Some(ActiveInput {
            supply_temperature_c,
            temperature_owner: owner,
        })
    } else {
        None
    };
    let mut state = State::new(predecessor.system);
    advance_cooling_supply_enthalpy_post_saturation_assignment_state(&mut state, predecessor, input)
        .is_some_and(|expected| snapshots_match_bit_exact(expected, snapshot))
}

pub(in crate::ideal_loads::calc) fn snapshot_route(snapshot: Snapshot) -> Option<Route> {
    if snapshot.source
        != PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE_ORDER
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
    let common_flags = [
        snapshot.cp377_supply_temperature_owned_read,
        snapshot.cp378_supply_humidity_ratio_saturation_limit_owned_read,
        snapshot.purchased_air_supply_temperature_for_post_saturation_enthalpy_read,
        snapshot.purchased_air_supply_humidity_ratio_for_post_saturation_enthalpy_read,
        snapshot.psy_h_fn_tdb_w_for_post_saturation_enthalpy_evaluated,
        snapshot.local_supply_enthalpy_after_saturation_limit_assignment_performed,
    ];
    let values = [
        snapshot.predecessor_resulting_supply_humidity_ratio,
        snapshot.supply_temperature_c,
        snapshot.supply_humidity_ratio,
        snapshot.psychrometric_supply_enthalpy_j_per_kg,
        snapshot.assigned_supply_enthalpy_j_per_kg,
        snapshot.resulting_supply_enthalpy_j_per_kg,
    ];
    let temperature_owner_count =
        usize::from(snapshot.cp334_supply_temperature_mixed_air_limit_owned_read)
            + usize::from(
                snapshot.cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read,
            );
    if !active {
        return snapshot.predecessor_dehumidification_control_type.is_none()
            && !snapshot.predecessor_supply_humidity_ratio_saturation_limit_assignment_performed
            && common_flags.into_iter().all(|flag| !flag)
            && temperature_owner_count == 0
            && values.into_iter().all(|value| value.is_none());
    }
    snapshot.predecessor_dehumidification_control_type.is_some()
        && snapshot.predecessor_supply_humidity_ratio_saturation_limit_assignment_performed
        && predecessor_selector_matches_route(snapshot, route)
        && common_flags.into_iter().all(|flag| flag)
        && temperature_owner_count == 1
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
        Some(predecessor_humidity_ratio),
        Some(temperature),
        Some(humidity_ratio),
        Some(evaluated),
        Some(assigned),
        Some(resulting),
    ) = (
        snapshot.predecessor_resulting_supply_humidity_ratio,
        snapshot.supply_temperature_c,
        snapshot.supply_humidity_ratio,
        snapshot.psychrometric_supply_enthalpy_j_per_kg,
        snapshot.assigned_supply_enthalpy_j_per_kg,
        snapshot.resulting_supply_enthalpy_j_per_kg,
    )
    else {
        return false;
    };
    let expected = energyplus_psy_h_fn_tdb_w(temperature, humidity_ratio);
    predecessor_humidity_ratio.to_bits() == humidity_ratio.to_bits()
        && evaluated.to_bits() == expected.to_bits()
        && assigned.to_bits() == evaluated.to_bits()
        && resulting.to_bits() == assigned.to_bits()
}

pub(in crate::ideal_loads::calc) fn temperature_owner(
    snapshot: TemperaturePrefix,
) -> Option<TemperatureOwner> {
    match (
        snapshot.cp334_supply_temperature_mixed_air_limit_owned_read,
        snapshot.cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read,
    ) {
        (true, false) => Some(TemperatureOwner::Cp334MixedAirLimit),
        (false, true) => Some(TemperatureOwner::Cp344CapacityMixedAirLimit),
        _ => None,
    }
}

pub(in crate::ideal_loads::calc) fn snapshots_match_bit_exact(
    mut left: Snapshot,
    mut right: Snapshot,
) -> bool {
    let values_match = [
        (
            left.predecessor_resulting_supply_humidity_ratio,
            right.predecessor_resulting_supply_humidity_ratio,
        ),
        (left.supply_temperature_c, right.supply_temperature_c),
        (left.supply_humidity_ratio, right.supply_humidity_ratio),
        (
            left.psychrometric_supply_enthalpy_j_per_kg,
            right.psychrometric_supply_enthalpy_j_per_kg,
        ),
        (
            left.assigned_supply_enthalpy_j_per_kg,
            right.assigned_supply_enthalpy_j_per_kg,
        ),
        (
            left.resulting_supply_enthalpy_j_per_kg,
            right.resulting_supply_enthalpy_j_per_kg,
        ),
    ]
    .into_iter()
    .all(|(left, right)| option_bits_match(left, right));
    for snapshot in [&mut left, &mut right] {
        snapshot.predecessor_resulting_supply_humidity_ratio = None;
        snapshot.supply_temperature_c = None;
        snapshot.supply_humidity_ratio = None;
        snapshot.psychrometric_supply_enthalpy_j_per_kg = None;
        snapshot.assigned_supply_enthalpy_j_per_kg = None;
        snapshot.resulting_supply_enthalpy_j_per_kg = None;
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
