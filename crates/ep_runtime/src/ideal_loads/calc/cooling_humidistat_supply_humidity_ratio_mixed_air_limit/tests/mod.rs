use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use super::{
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitActiveOperands as Operands,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitRetainedRoute as Route,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitRuntimeState as State,
    advance_cooling_humidistat_supply_humidity_ratio_mixed_air_limit_state as advance,
};
use super::release::snapshot_route;
use crate::ideal_loads::calc::cooling_positive_supply_temperature_mixed_air_limit::source_shaped_two_argument_minimum;
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_MINIMUM_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_MINIMUM_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_MINIMUM_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitSnapshot as Predecessor,
};

mod ieee;
mod private_release;
mod public_release;
mod release_corruption;

pub(super) const U: Route = Route::UnitOff;
pub(super) const N: Route = Route::NonCooling;
pub(super) const P: Route = Route::PositiveGuardFalseFallthrough;
pub(super) const C0: Route = Route::DehumidificationControlNoneCaseCompletedSkip;
pub(super) const Q: Route =
    Route::DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip;
pub(super) const H: Route =
    Route::DehumidificationControlHumidistatSupplyHumidityRatioMixedAirLimitExecuted;
pub(super) const CSH: Route =
    Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip;

pub(super) fn operands(route: Route, mixed: f64) -> Option<Operands> {
    (route == H).then_some(Operands {
        mixed_air_humidity_ratio: mixed,
    })
}

pub(super) fn predecessor(route: Route, ordinal: usize, local: f64) -> Predecessor {
    let selected = matches!(route, C0 | Q | H | CSH);
    let humidistat = route == H;
    let selector = match route {
        C0 => Some(DehumidificationControlType::None),
        Q => Some(DehumidificationControlType::ConstantSensibleHeatRatio),
        H => Some(DehumidificationControlType::Humidistat),
        CSH => Some(DehumidificationControlType::ConstantSupplyHumidityRatio),
        _ => None,
    };
    Predecessor {
        source:
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_MINIMUM_LIMIT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_MINIMUM_LIMIT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_MINIMUM_LIMIT_SOURCE_ORDER,
        system: IdealLoadsAirSystemId(1),
        parent_call_ordinal: ordinal,
        controlled_zone: ZoneId(2),
        unit_body_entered: route != U,
        predecessor_cooling_body_entered: matches!(route, P | C0 | Q | H | CSH),
        predecessor_no_outdoor_air_fallback_entered: matches!(route, P | C0 | Q | H | CSH),
        predecessor_positive_supply_mass_flow_body_entered: selected,
        unit_off_skipped: route == U,
        non_cooling_skipped: route == N,
        positive_guard_false_fallthrough_skipped: route == P,
        predecessor_dehumidification_control_type: selector,
        predecessor_dehumidification_control_none_case_completed_skip: route == C0,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
            route == Q,
        predecessor_dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_executed:
            humidistat,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            route == CSH,
        predecessor_resulting_supply_humidity_ratio_for_dehumidification: humidistat
            .then_some(local),
        dehumidification_control_none_case_completed_skip: route == C0,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: route == Q,
        dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_executed:
            humidistat,
        dehumidification_control_constant_supply_humidity_ratio_case_selected_skip: route == CSH,
        supply_humidity_ratio_for_dehumidification_for_minimum_limit_maximum_read: humidistat,
        supply_humidity_ratio_for_dehumidification_before_minimum_limit: humidistat
            .then_some(local),
        minimum_cooling_supply_air_humidity_ratio_for_maximum_read: humidistat,
        minimum_cooling_supply_air_humidity_ratio: humidistat.then_some(local),
        source_shaped_two_argument_maximum_evaluated: humidistat,
        maximum_supply_humidity_ratio_for_dehumidification: humidistat.then_some(local),
        supply_humidity_ratio_for_dehumidification_assignment_performed: humidistat,
        assigned_supply_humidity_ratio_for_dehumidification: humidistat.then_some(local),
        resulting_supply_humidity_ratio_for_dehumidification: humidistat.then_some(local),
    }
}

#[test]
fn seven_routes_preserve_exact_cp361_lineage_and_execute_only_h() {
    for route in [U, N, P, C0, Q, H, CSH] {
        let predecessor = predecessor(route, 1, 0.0077);
        let mut state = State::new(predecessor.system);
        let snapshot = advance(&mut state, predecessor, operands(route, 0.006))
            .expect("canonical route must advance");
        assert_eq!(snapshot_route(snapshot), Some(route));
        assert_eq!(state.transition_count, 1);
        assert_eq!(
            state.source_site_execution_count,
            usize::from(route == H)
                * PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE_ORDER
                    .len()
        );
        assert_eq!(
            snapshot.resulting_supply_humidity_ratio,
            (route == H).then_some(0.006)
        );
    }
}

#[test]
fn public_direct_u_n_p_c0_routes_are_complete_null() {
    for route in [U, N, P, C0] {
        let predecessor = predecessor(route, 1, 0.0077);
        let mut state = State::new(predecessor.system);
        let snapshot = advance(&mut state, predecessor, None).unwrap();
        assert!(super::cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshot_is_exact_direct_release(snapshot));
        assert!([
            snapshot.predecessor_resulting_supply_humidity_ratio_for_dehumidification,
            snapshot.mixed_air_humidity_ratio,
            snapshot.supply_humidity_ratio_for_dehumidification_before_mixed_air_limit,
            snapshot.minimum_supply_humidity_ratio,
            snapshot.assigned_supply_humidity_ratio,
            snapshot.resulting_supply_humidity_ratio,
        ]
        .into_iter()
        .all(|value| value.is_none()));
    }
}

#[test]
fn source_minimum_is_right_biased_for_ties_and_unordered_values() {
    let right_nan = f64::from_bits(0x7ff8_0000_0000_0042);
    let left_nan = f64::from_bits(0x7ff8_0000_0000_0011);
    for (mixed, local, expected) in [
        (0.006, 0.0077, 0.006),
        (0.008, 0.0077, 0.0077),
        (0.0077, 0.0077, 0.0077),
        (-0.0, 0.0, 0.0),
        (0.0, -0.0, -0.0),
        (left_nan, 0.0077, 0.0077),
        (0.0077, right_nan, right_nan),
        (left_nan, right_nan, right_nan),
        (f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
    ] {
        assert_eq!(
            source_shaped_two_argument_minimum(mixed, local).to_bits(),
            expected.to_bits()
        );
    }
}

#[test]
fn malformed_cp361_source_prefix_selector_and_numeric_lineage_are_transactional() {
    let canonical = predecessor(H, 1, 0.0077);
    let corruptions: [fn(&mut Predecessor); 7] = [
        |value| value.source = "wrong-source",
        |value| value.first_excluded_source = "wrong-excluded-source",
        |value| value.source_order = &[],
        |value| value.predecessor_positive_supply_mass_flow_body_entered = false,
        |value| {
            value.predecessor_dehumidification_control_type =
                Some(DehumidificationControlType::None)
        },
        |value| value.minimum_cooling_supply_air_humidity_ratio_for_maximum_read = false,
        |value| value.resulting_supply_humidity_ratio_for_dehumidification = Some(0.0081),
    ];
    for corrupt in corruptions {
        let mut malformed = canonical;
        corrupt(&mut malformed);
        let mut state = State::new(canonical.system);
        let before = state.clone();
        assert!(advance(&mut state, malformed, operands(H, 0.006)).is_none());
        assert_eq!(state, before);
    }
}

#[test]
fn every_counter_overflow_rejects_without_partial_mutation() {
    fn rejected(mut state: State, route: Route) {
        let before = state.clone();
        assert!(advance(
            &mut state,
            predecessor(route, 1, 0.0077),
            operands(route, 0.006),
        )
        .is_none());
        assert_eq!(state, before);
    }

    let system = IdealLoadsAirSystemId(1);
    let mut state = State::new(system);
    state.transition_count = usize::MAX;
    rejected(state, U);
    let mut state = State::new(system);
    state.unit_off_skip_count = usize::MAX;
    rejected(state, U);
    let mut state = State::new(system);
    state.non_cooling_skip_count = usize::MAX;
    rejected(state, N);

    macro_rules! reject_route_pair {
        ($route:expr, $count:ident, $witness:ident) => {{
            let mut state = State::new(system);
            state.$count = usize::MAX;
            rejected(state, $route);
            let mut state = State::new(system);
            state.$witness = usize::MAX;
            rejected(state, $route);
        }};
    }
    reject_route_pair!(
        P,
        positive_guard_false_fallthrough_skip_count,
        witnessed_positive_guard_false_fallthrough_skip_count
    );
    reject_route_pair!(
        C0,
        dehumidification_control_none_case_completed_skip_count,
        witnessed_dehumidification_control_none_case_completed_skip_count
    );
    reject_route_pair!(
        Q,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        witnessed_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count
    );
    reject_route_pair!(
        H,
        dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_count,
        witnessed_dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_count
    );
    reject_route_pair!(
        CSH,
        dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        witnessed_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count
    );

    let mut state = State::new(system);
    state.source_site_execution_count = usize::MAX - 3;
    rejected(state, H);
    macro_rules! reject_site {
        ($field:ident) => {{
            let mut state = State::new(system);
            state.$field = usize::MAX;
            rejected(state, H);
        }};
    }
    reject_site!(mixed_air_humidity_ratio_for_minimum_read_count);
    reject_site!(
        supply_humidity_ratio_for_dehumidification_for_mixed_air_limit_minimum_read_count
    );
    reject_site!(source_shaped_two_argument_minimum_evaluation_count);
    reject_site!(supply_humidity_ratio_assignment_count);
}
