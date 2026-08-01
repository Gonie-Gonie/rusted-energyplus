//! Exact CP376 snapshot validation.

use super::super::{
    advance_cooling_supply_humidity_ratio_pre_saturation_original_assignment_state,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentOwner as Owner,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot as Snapshot,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE_ORDER,
};
use crate::ideal_loads::PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentSnapshot as Predecessor;

pub(in crate::ideal_loads) fn cooling_supply_humidity_ratio_pre_saturation_original_assignment_snapshot_is_exact_direct_release(
    snapshot: Snapshot,
) -> bool {
    match snapshot_route(snapshot) {
        Some(Route::UnitOff | Route::NonCooling | Route::PositiveGuardFalseFallthrough) => true,
        Some(
            Route::HeatingAvailabilityGuardFalseFallthrough
            | Route::HumidificationControlGuardFalseFallthrough,
        ) => {
            snapshot.predecessor_dehumidification_control_type
                == Some(ep_model::DehumidificationControlType::None)
                && snapshot.cp347_none_case_owned_read
        }
        _ => false,
    }
}

pub(super) fn snapshot_links_to_predecessor(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    let input = if snapshot.purchased_air_supply_humidity_ratio_read {
        let (Some(value), Some(owner)) = (
            snapshot.purchased_air_supply_humidity_ratio_before_saturation_check,
            snapshot_owner(snapshot),
        ) else {
            return false;
        };
        Some(ActiveInput {
            purchased_air_supply_humidity_ratio: value,
            owner,
        })
    } else {
        None
    };
    let mut state = State::new(predecessor.system);
    advance_cooling_supply_humidity_ratio_pre_saturation_original_assignment_state(
        &mut state,
        predecessor,
        input,
    )
    .is_some_and(|expected| snapshots_match_bit_exact(expected, snapshot))
}

pub(super) fn snapshot_route(snapshot: Snapshot) -> Option<Route> {
    if snapshot.source
        != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE_ORDER
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
        snapshot.cp375_maximum_assignment_owned_read,
        snapshot.cp347_none_case_owned_read,
        snapshot.cp356_constant_shr_owned_read,
        snapshot.cp362_humidistat_owned_read,
        snapshot.cp365_constant_supply_humidity_ratio_owned_read,
    ]
    .into_iter()
    .filter(|owner| *owner)
    .count();
    let values = [
        snapshot.purchased_air_supply_humidity_ratio_before_saturation_check,
        snapshot.assigned_supply_humidity_ratio_original,
        snapshot.resulting_supply_humidity_ratio_original,
    ];
    let local_shape = if active {
        snapshot.purchased_air_supply_humidity_ratio_read
            && snapshot.local_supply_humidity_ratio_original_assignment_performed
            && owner_count == 1
            && values.into_iter().all(|value| value.is_some())
            && option_bits_match(
                snapshot.purchased_air_supply_humidity_ratio_before_saturation_check,
                snapshot.assigned_supply_humidity_ratio_original,
            )
            && option_bits_match(
                snapshot.assigned_supply_humidity_ratio_original,
                snapshot.resulting_supply_humidity_ratio_original,
            )
    } else {
        !snapshot.purchased_air_supply_humidity_ratio_read
            && !snapshot.local_supply_humidity_ratio_original_assignment_performed
            && owner_count == 0
            && values.into_iter().all(|value| value.is_none())
    };
    let predecessor_shape = match route {
        Route::DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted
        | Route::DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted => {
            snapshot.predecessor_purchased_air_supply_humidity_ratio_assignment_performed
                && snapshot
                    .predecessor_resulting_supply_humidity_ratio
                    .is_some()
                && snapshot.cp375_maximum_assignment_owned_read
                && option_bits_match(
                    snapshot.predecessor_resulting_supply_humidity_ratio,
                    snapshot.purchased_air_supply_humidity_ratio_before_saturation_check,
                )
        }
        Route::HeatingAvailabilityGuardFalseFallthrough
        | Route::HumidificationControlGuardFalseFallthrough
        | Route::DehumidificationControlGuardFalseFallthrough => {
            !snapshot.predecessor_purchased_air_supply_humidity_ratio_assignment_performed
                && snapshot
                    .predecessor_resulting_supply_humidity_ratio
                    .is_none()
                && owner_matches_selector(snapshot)
        }
        _ => {
            snapshot.predecessor_dehumidification_control_type.is_none()
                && !snapshot.predecessor_purchased_air_supply_humidity_ratio_assignment_performed
                && snapshot
                    .predecessor_resulting_supply_humidity_ratio
                    .is_none()
        }
    };
    local_shape && predecessor_shape
}

fn owner_matches_selector(snapshot: Snapshot) -> bool {
    match snapshot.predecessor_dehumidification_control_type {
        Some(ep_model::DehumidificationControlType::None) => snapshot.cp347_none_case_owned_read,
        Some(ep_model::DehumidificationControlType::ConstantSensibleHeatRatio) => {
            snapshot.cp356_constant_shr_owned_read
        }
        Some(ep_model::DehumidificationControlType::Humidistat) => {
            snapshot.cp362_humidistat_owned_read
        }
        Some(ep_model::DehumidificationControlType::ConstantSupplyHumidityRatio) => {
            snapshot.cp365_constant_supply_humidity_ratio_owned_read
        }
        None => false,
    }
}

pub(super) fn snapshot_owner(snapshot: Snapshot) -> Option<Owner> {
    if snapshot.cp375_maximum_assignment_owned_read {
        Some(Owner::Cp375MaximumAssignment)
    } else if snapshot.cp347_none_case_owned_read {
        Some(Owner::Cp347NoneCase)
    } else if snapshot.cp356_constant_shr_owned_read {
        Some(Owner::Cp356ConstantShr)
    } else if snapshot.cp362_humidistat_owned_read {
        Some(Owner::Cp362Humidistat)
    } else if snapshot.cp365_constant_supply_humidity_ratio_owned_read {
        Some(Owner::Cp365ConstantSupplyHumidityRatio)
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
            left.predecessor_resulting_supply_humidity_ratio,
            right.predecessor_resulting_supply_humidity_ratio,
        ),
        option_bits_match(
            left.purchased_air_supply_humidity_ratio_before_saturation_check,
            right.purchased_air_supply_humidity_ratio_before_saturation_check,
        ),
        option_bits_match(
            left.assigned_supply_humidity_ratio_original,
            right.assigned_supply_humidity_ratio_original,
        ),
        option_bits_match(
            left.resulting_supply_humidity_ratio_original,
            right.resulting_supply_humidity_ratio_original,
        ),
    ]
    .into_iter()
    .all(|matches| matches);
    for snapshot in [&mut left, &mut right] {
        snapshot.predecessor_resulting_supply_humidity_ratio = None;
        snapshot.purchased_air_supply_humidity_ratio_before_saturation_check = None;
        snapshot.assigned_supply_humidity_ratio_original = None;
        snapshot.resulting_supply_humidity_ratio_original = None;
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
