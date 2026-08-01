//! Pure CP375-to-CP376 pre-saturation original assignment.

use ep_model::DehumidificationControlType;

use super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot as Snapshot,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE_ORDER,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentSnapshot as Predecessor,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE_ORDER,
};

/// Exact source-level owner of `PurchAir.SupplyHumRat` read at line 2258.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads) enum PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentOwner
{
    Cp375MaximumAssignment,
    Cp347NoneCase,
    Cp356ConstantShr,
    Cp362Humidistat,
    Cp365ConstantSupplyHumidityRatio,
}

/// Owner-resolved value needed on every positive-supply route.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentActiveInput
{
    /// Current purchased-air result-store bits.
    pub purchased_air_supply_humidity_ratio: f64,
    /// Exact latest source writer of those bits.
    pub owner: PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentOwner,
}

pub(in crate::ideal_loads::calc) fn advance_cooling_supply_humidity_ratio_pre_saturation_original_assignment_state(
    state: &mut State,
    predecessor: Predecessor,
    input: Option<
        PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentActiveInput,
    >,
) -> Option<Snapshot> {
    if state.system != predecessor.system {
        return None;
    }
    let route = predecessor_route(predecessor)?;
    let value = prepare_value(route, predecessor, input)?;
    if !next_transition_fits(state, route, input.map(|input| input.owner)) {
        return None;
    }

    state.transition_count += 1;
    increment_route_count(state, route);
    if let Some(input) = input {
        state.source_site_execution_count +=
            PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE_ORDER.len();
        state.purchased_air_supply_humidity_ratio_before_saturation_limit_read_count += 1;
        state.local_original_supply_humidity_ratio_before_saturation_limit_assignment_count += 1;
        increment_owner_count(state, input.owner);
    }

    let active = route_is_active(route);
    let owner = input.map(|input| input.owner);
    let snapshot = Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE_ORDER,
        system: state.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_off_skipped: route == Route::UnitOff,
        non_cooling_skipped: route == Route::NonCooling,
        positive_guard_false_fallthrough_skipped: route
            == Route::PositiveGuardFalseFallthrough,
        heating_availability_guard_false_fallthrough: route
            == Route::HeatingAvailabilityGuardFalseFallthrough,
        humidification_control_guard_false_fallthrough: route
            == Route::HumidificationControlGuardFalseFallthrough,
        dehumidification_control_humidistat_maximum_assignment_executed: route
            == Route::DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted,
        dehumidification_control_none_maximum_assignment_executed: route
            == Route::DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted,
        dehumidification_control_guard_false_fallthrough: route
            == Route::DehumidificationControlGuardFalseFallthrough,
        predecessor_dehumidification_control_type: predecessor
            .predecessor_dehumidification_control_type,
        predecessor_purchased_air_supply_humidity_ratio_assignment_performed: predecessor
            .purchased_air_supply_humidity_ratio_assignment_performed,
        predecessor_resulting_supply_humidity_ratio: predecessor.resulting_supply_humidity_ratio,
        cp375_maximum_assignment_owned_read: owner
            == Some(PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentOwner::Cp375MaximumAssignment),
        cp347_none_case_owned_read: owner
            == Some(PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentOwner::Cp347NoneCase),
        cp356_constant_shr_owned_read: owner
            == Some(PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentOwner::Cp356ConstantShr),
        cp362_humidistat_owned_read: owner
            == Some(PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentOwner::Cp362Humidistat),
        cp365_constant_supply_humidity_ratio_owned_read: owner
            == Some(PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentOwner::Cp365ConstantSupplyHumidityRatio),
        purchased_air_supply_humidity_ratio_read: active,
        purchased_air_supply_humidity_ratio_before_saturation_check: value,
        local_supply_humidity_ratio_original_assignment_performed: active,
        assigned_supply_humidity_ratio_original: value,
        resulting_supply_humidity_ratio_original: value,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    Some(snapshot)
}

pub(in crate::ideal_loads::calc) fn predecessor_route(predecessor: Predecessor) -> Option<Route> {
    if predecessor.source
        != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source_order
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE_ORDER
    {
        return None;
    }
    let indicators = [
        predecessor.unit_off_skipped,
        predecessor.non_cooling_skipped,
        predecessor.positive_guard_false_fallthrough_skipped,
        predecessor.predecessor_heating_on_guard_false_fallthrough,
        predecessor.predecessor_humidification_control_guard_false_fallthrough,
        predecessor
            .dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_executed,
        predecessor.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_executed,
        predecessor.predecessor_dehumidification_control_guard_false_fallthrough,
    ];
    if indicators.into_iter().filter(|active| *active).count() != 1 {
        return None;
    }
    let route = if predecessor.unit_off_skipped {
        Route::UnitOff
    } else if predecessor.non_cooling_skipped {
        Route::NonCooling
    } else if predecessor.positive_guard_false_fallthrough_skipped {
        Route::PositiveGuardFalseFallthrough
    } else if predecessor.predecessor_heating_on_guard_false_fallthrough {
        Route::HeatingAvailabilityGuardFalseFallthrough
    } else if predecessor.predecessor_humidification_control_guard_false_fallthrough {
        Route::HumidificationControlGuardFalseFallthrough
    } else if predecessor
        .dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_executed
    {
        Route::DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted
    } else if predecessor
        .dehumidification_control_none_supply_humidity_ratio_maximum_assignment_executed
    {
        Route::DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted
    } else {
        Route::DehumidificationControlGuardFalseFallthrough
    };
    predecessor_shape_matches_route(predecessor, route).then_some(route)
}

fn predecessor_shape_matches_route(predecessor: Predecessor, route: Route) -> bool {
    let predecessor_assignment_active = matches!(
        route,
        Route::DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted
            | Route::DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted
    );
    let values = [
        predecessor.purchased_air_supply_humidity_ratio_before_humidification_supply_maximum,
        predecessor.supply_humidity_ratio_for_humidification_for_supply_maximum,
        predecessor.maximum_supply_humidity_ratio,
        predecessor.assigned_supply_humidity_ratio,
        predecessor.resulting_supply_humidity_ratio,
    ];
    let numeric_shape = if predecessor_assignment_active {
        predecessor.purchased_air_supply_humidity_ratio_for_humidification_supply_maximum_read
            && predecessor.supply_humidity_ratio_for_humidification_for_supply_maximum_read
            && predecessor.source_shaped_two_argument_maximum_evaluated
            && predecessor.purchased_air_supply_humidity_ratio_assignment_performed
            && values.into_iter().all(|value| value.is_some())
            && option_bits_match(
                predecessor.assigned_supply_humidity_ratio,
                predecessor.resulting_supply_humidity_ratio,
            )
    } else {
        !predecessor.purchased_air_supply_humidity_ratio_for_humidification_supply_maximum_read
            && !predecessor.supply_humidity_ratio_for_humidification_for_supply_maximum_read
            && !predecessor.source_shaped_two_argument_maximum_evaluated
            && !predecessor.purchased_air_supply_humidity_ratio_assignment_performed
            && values.into_iter().all(|value| value.is_none())
    };
    let positive = route_is_positive(route);
    let prefix_shape = predecessor.predecessor_positive_supply_mass_flow_body_entered == positive
        && (positive
            == predecessor
                .predecessor_dehumidification_control_type
                .is_some());
    let selector_shape = match route {
        Route::DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted => {
            predecessor.predecessor_dehumidification_control_type
                == Some(DehumidificationControlType::Humidistat)
        }
        Route::DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted => {
            predecessor.predecessor_dehumidification_control_type
                == Some(DehumidificationControlType::None)
        }
        Route::DehumidificationControlGuardFalseFallthrough => matches!(
            predecessor.predecessor_dehumidification_control_type,
            Some(
                DehumidificationControlType::ConstantSensibleHeatRatio
                    | DehumidificationControlType::ConstantSupplyHumidityRatio
            )
        ),
        _ => true,
    };
    numeric_shape && prefix_shape && selector_shape
}

fn prepare_value(
    route: Route,
    predecessor: Predecessor,
    input: Option<
        PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentActiveInput,
    >,
) -> Option<Option<f64>> {
    if !route_is_active(route) {
        return input.is_none().then_some(None);
    }
    let input = input?;
    if !owner_matches_route(route, predecessor, input.owner) {
        return None;
    }
    if input.owner
        == PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentOwner::Cp375MaximumAssignment
        && !predecessor.resulting_supply_humidity_ratio.is_some_and(|value| {
            value.to_bits() == input.purchased_air_supply_humidity_ratio.to_bits()
        })
    {
        return None;
    }
    Some(Some(input.purchased_air_supply_humidity_ratio))
}

fn owner_matches_route(
    route: Route,
    predecessor: Predecessor,
    owner: PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentOwner,
) -> bool {
    if matches!(
        route,
        Route::DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted
            | Route::DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted
    ) {
        return owner
            == PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentOwner::Cp375MaximumAssignment;
    }
    match predecessor.predecessor_dehumidification_control_type {
        Some(DehumidificationControlType::None) => {
            owner
                == PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentOwner::Cp347NoneCase
        }
        Some(DehumidificationControlType::ConstantSensibleHeatRatio) => {
            owner
                == PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentOwner::Cp356ConstantShr
        }
        Some(DehumidificationControlType::Humidistat) => {
            owner
                == PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentOwner::Cp362Humidistat
        }
        Some(DehumidificationControlType::ConstantSupplyHumidityRatio) => {
            owner
                == PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentOwner::Cp365ConstantSupplyHumidityRatio
        }
        None => false,
    }
}

pub(in crate::ideal_loads::calc) fn next_transition_fits(
    state: &State,
    route: Route,
    owner: Option<PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentOwner>,
) -> bool {
    state.transition_count.checked_add(1).is_some()
        && route_count(state, route).checked_add(1).is_some()
        && (!route_is_active(route)
            || (state
                .source_site_execution_count
                .checked_add(
                    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE_ORDER.len(),
                )
                .is_some()
                && state
                    .purchased_air_supply_humidity_ratio_before_saturation_limit_read_count
                    .checked_add(1)
                    .is_some()
                && state
                    .local_original_supply_humidity_ratio_before_saturation_limit_assignment_count
                    .checked_add(1)
                    .is_some()
                && owner.is_some_and(|owner| owner_count(state, owner).checked_add(1).is_some())))
}

pub(in crate::ideal_loads::calc) fn route_is_active(route: Route) -> bool {
    !matches!(
        route,
        Route::UnitOff | Route::NonCooling | Route::PositiveGuardFalseFallthrough
    )
}

fn route_is_positive(route: Route) -> bool {
    !matches!(
        route,
        Route::UnitOff | Route::NonCooling | Route::PositiveGuardFalseFallthrough
    )
}

pub(in crate::ideal_loads::calc) fn route_count(state: &State, route: Route) -> usize {
    match route {
        Route::UnitOff => state.unit_off_skip_count,
        Route::NonCooling => state.non_cooling_skip_count,
        Route::PositiveGuardFalseFallthrough => state.positive_guard_false_fallthrough_skip_count,
        Route::HeatingAvailabilityGuardFalseFallthrough => {
            state.heating_availability_guard_false_fallthrough_count
        }
        Route::HumidificationControlGuardFalseFallthrough => {
            state.humidification_control_guard_false_fallthrough_count
        }
        Route::DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted => {
            state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count
        }
        Route::DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted => {
            state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count
        }
        Route::DehumidificationControlGuardFalseFallthrough => {
            state.dehumidification_control_guard_false_fallthrough_count
        }
    }
}

fn increment_route_count(state: &mut State, route: Route) {
    match route {
        Route::UnitOff => state.unit_off_skip_count += 1,
        Route::NonCooling => state.non_cooling_skip_count += 1,
        Route::PositiveGuardFalseFallthrough => {
            state.positive_guard_false_fallthrough_skip_count += 1;
        }
        Route::HeatingAvailabilityGuardFalseFallthrough => {
            state.heating_availability_guard_false_fallthrough_count += 1;
        }
        Route::HumidificationControlGuardFalseFallthrough => {
            state.humidification_control_guard_false_fallthrough_count += 1;
        }
        Route::DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted => {
            state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count += 1;
        }
        Route::DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted => {
            state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count += 1;
        }
        Route::DehumidificationControlGuardFalseFallthrough => {
            state.dehumidification_control_guard_false_fallthrough_count += 1;
        }
    }
}

pub(in crate::ideal_loads::calc) fn owner_count(
    state: &State,
    owner: PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentOwner,
) -> usize {
    match owner {
        PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentOwner::Cp375MaximumAssignment => state.cp375_maximum_assignment_owner_count,
        PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentOwner::Cp347NoneCase => state.cp347_none_case_owner_count,
        PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentOwner::Cp356ConstantShr => state.cp356_constant_shr_owner_count,
        PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentOwner::Cp362Humidistat => state.cp362_humidistat_owner_count,
        PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentOwner::Cp365ConstantSupplyHumidityRatio => state.cp365_constant_supply_humidity_ratio_owner_count,
    }
}

fn increment_owner_count(
    state: &mut State,
    owner: PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentOwner,
) {
    match owner {
        PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentOwner::Cp375MaximumAssignment => state.cp375_maximum_assignment_owner_count += 1,
        PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentOwner::Cp347NoneCase => state.cp347_none_case_owner_count += 1,
        PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentOwner::Cp356ConstantShr => state.cp356_constant_shr_owner_count += 1,
        PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentOwner::Cp362Humidistat => state.cp362_humidistat_owner_count += 1,
        PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentOwner::Cp365ConstantSupplyHumidityRatio => state.cp365_constant_supply_humidity_ratio_owner_count += 1,
    }
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
