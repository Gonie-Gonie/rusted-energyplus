mod release;

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use super::{
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE_ORDER,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakRetainedRoute as Route,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakRuntimeState as State,
    advance_cooling_constant_supply_humidity_ratio_case_break_state as advance,
    advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_case_break,
    cooling_constant_supply_humidity_ratio_case_break_snapshot_is_exact_direct_release,
    private_constant_supply_humidity_ratio_case_break_counterfactual_from_direct_release,
    private_constant_supply_humidity_ratio_case_break_counterfactual_links_to_direct_release,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentSnapshot as Predecessor,
};

const U: Route = Route::UnitOff;
const N: Route = Route::NonCooling;
const P: Route = Route::PositiveGuardFalseFallthrough;
const C0: Route = Route::DehumidificationControlNoneCaseCompletedSkip;
const Q: Route = Route::DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip;
const H: Route = Route::DehumidificationControlHumidistatCaseCompletedSkip;
const CSH: Route = Route::DehumidificationControlConstantSupplyHumidityRatioCaseBreak;

#[test]
fn source_boundary_single_site_and_seven_route_algebra_are_exact() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2236"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2238"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE_ORDER,
        &[
            "exit-purchased-air-dehumidification-control-constant-supply-humidity-ratio-case-via-break"
        ]
    );

    let mut state = State::new(IdealLoadsAirSystemId(7));
    for (index, route) in [U, N, P, C0, Q, H, CSH].into_iter().enumerate() {
        let snapshot = advance(&mut state, predecessor(route, index + 1, 0.0077))
            .expect("CP366 pure transition");
        assert_eq!(
            snapshot
                .dehumidification_control_constant_supply_humidity_ratio_case_exited_via_break,
            route == CSH
        );
        assert_eq!(
            cooling_constant_supply_humidity_ratio_case_break_snapshot_is_exact_direct_release(
                snapshot
            ),
            matches!(route, U | N | P | C0)
        );
    }
    assert_eq!(
        [
            state.unit_off_skip_count,
            state.non_cooling_skip_count,
            state.positive_guard_false_fallthrough_skip_count,
            state.dehumidification_control_none_case_completed_skip_count,
            state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
            state.dehumidification_control_humidistat_case_completed_skip_count,
            state.dehumidification_control_constant_supply_humidity_ratio_case_break_count,
        ],
        [1; 7]
    );
    assert_eq!(state.transition_count, 7);
    assert_eq!(state.source_site_execution_count, 1);
}

#[test]
fn transition_is_evidence_only_and_all_overflow_is_transactional() {
    let mut expected = None;
    for value in [
        0.0077,
        -0.0,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::from_bits(0x7ff8_0000_0000_0366),
    ] {
        let snapshot = advance(
            &mut State::new(IdealLoadsAirSystemId(7)),
            predecessor(CSH, 1, value),
        )
        .expect("CP366 raw control transition");
        if let Some(expected) = expected {
            assert_eq!(snapshot, expected);
        } else {
            expected = Some(snapshot);
        }
    }
    every_counter_overflow_rejects_without_mutation();
}

#[test]
fn forged_provenance_selector_control_flags_and_one_hot_are_transactional() {
    let mut forged = predecessor(CSH, 1, 0.0077);
    forged.source = "forged";
    rejects_without_mutation(forged);

    let mut forged = predecessor(CSH, 1, 0.0077);
    forged.predecessor_dehumidification_control_type =
        Some(DehumidificationControlType::Humidistat);
    rejects_without_mutation(forged);

    let mut forged = predecessor(CSH, 1, 0.0077);
    forged.dehumidification_control_none_case_completed_skip = true;
    rejects_without_mutation(forged);

    let mut forged = predecessor(C0, 1, 0.0077);
    forged.dehumidification_control_constant_supply_humidity_ratio_assignment_executed = true;
    rejects_without_mutation(forged);
}

#[test]
fn every_counter_overflow_rejects_without_mutation() {
    macro_rules! reject_overflow {
        ($field:ident, $route:expr) => {{
            let mut state = State::new(IdealLoadsAirSystemId(7));
            state.$field = usize::MAX;
            let before = state.clone();
            assert!(advance(&mut state, predecessor($route, 1, 0.0077)).is_none());
            assert_eq!(state, before);
        }};
    }
    reject_overflow!(transition_count, CSH);
    reject_overflow!(unit_off_skip_count, U);
    reject_overflow!(non_cooling_skip_count, N);
    reject_overflow!(positive_guard_false_fallthrough_skip_count, P);
    reject_overflow!(witnessed_positive_guard_false_fallthrough_skip_count, P);
    reject_overflow!(dehumidification_control_none_case_completed_skip_count, C0);
    reject_overflow!(
        witnessed_dehumidification_control_none_case_completed_skip_count,
        C0
    );
    reject_overflow!(
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        Q
    );
    reject_overflow!(
        witnessed_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        Q
    );
    reject_overflow!(
        dehumidification_control_humidistat_case_completed_skip_count,
        H
    );
    reject_overflow!(
        witnessed_dehumidification_control_humidistat_case_completed_skip_count,
        H
    );
    reject_overflow!(
        dehumidification_control_constant_supply_humidity_ratio_case_break_count,
        CSH
    );
    reject_overflow!(
        witnessed_dehumidification_control_constant_supply_humidity_ratio_case_break_count,
        CSH
    );
    reject_overflow!(source_site_execution_count, CSH);
}

fn rejects_without_mutation(predecessor: Predecessor) {
    let mut state = State::new(IdealLoadsAirSystemId(7));
    let before = state.clone();
    assert!(advance(&mut state, predecessor).is_none());
    assert_eq!(state, before);
}

fn predecessor(route: Route, ordinal: usize, value: f64) -> Predecessor {
    let active = matches!(route, C0 | Q | H | CSH);
    let selector = match route {
        C0 => Some(DehumidificationControlType::None),
        Q => Some(DehumidificationControlType::ConstantSensibleHeatRatio),
        H => Some(DehumidificationControlType::Humidistat),
        CSH => Some(DehumidificationControlType::ConstantSupplyHumidityRatio),
        _ => None,
    };
    let assigned = route == CSH;
    Predecessor {
        source: PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER,
        system: IdealLoadsAirSystemId(7),
        parent_call_ordinal: ordinal,
        controlled_zone: ZoneId(9),
        unit_body_entered: route != U,
        predecessor_cooling_body_entered: matches!(route, P | C0 | Q | H | CSH),
        predecessor_no_outdoor_air_fallback_entered: matches!(route, P | C0 | Q | H | CSH),
        predecessor_positive_supply_mass_flow_body_entered: active,
        unit_off_skipped: route == U,
        non_cooling_skipped: route == N,
        positive_guard_false_fallthrough_skipped: route == P,
        predecessor_dehumidification_control_type: selector,
        predecessor_dehumidification_control_none_case_completed_skip: route == C0,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
            route == Q,
        predecessor_dehumidification_control_humidistat_case_completed_skip: route == H,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_entered: route
            == CSH,
        dehumidification_control_none_case_completed_skip: route == C0,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: route == Q,
        dehumidification_control_humidistat_case_completed_skip: route == H,
        dehumidification_control_constant_supply_humidity_ratio_assignment_executed: assigned,
        minimum_cooling_supply_air_humidity_ratio_read: assigned,
        minimum_cooling_supply_air_humidity_ratio: assigned.then_some(value),
        supply_humidity_ratio_assigned: assigned,
        assigned_supply_humidity_ratio: assigned.then_some(value),
        resulting_supply_humidity_ratio: assigned.then_some(value),
    }
}
