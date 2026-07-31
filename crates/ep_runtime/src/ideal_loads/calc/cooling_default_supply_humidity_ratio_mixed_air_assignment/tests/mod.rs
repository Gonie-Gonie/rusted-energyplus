mod release;

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use super::{
    PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentRuntimeState as State,
    advance_cooling_default_supply_humidity_ratio_mixed_air_assignment_state as advance,
    advance_direct_no_oa_calc_cooling_default_supply_humidity_ratio_mixed_air_assignment,
    cooling_default_supply_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release,
    private_default_supply_humidity_ratio_mixed_air_assignment_csh_counterfactual_from_direct_release,
    private_default_supply_humidity_ratio_mixed_air_assignment_csh_counterfactual_links_to_direct_release,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE_ORDER,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakSnapshot as Predecessor,
};

const U: Route = Route::UnitOff;
const N: Route = Route::NonCooling;
const P: Route = Route::PositiveGuardFalseFallthrough;
const C0: Route = Route::DehumidificationControlNoneCaseCompletedSkip;
const Q: Route = Route::DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip;
const H: Route = Route::DehumidificationControlHumidistatCaseCompletedSkip;
const CSH: Route = Route::DehumidificationControlConstantSupplyHumidityRatioCaseCompletedSkip;

#[test]
fn source_boundary_two_sites_and_seven_route_zero_execution_algebra_are_exact() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2238"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2239"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER,
        &[
            "read-purchased-air-mixed-air-humidity-ratio-for-dehumidification-control-default-assignment",
            "assign-purchased-air-supply-humidity-ratio-for-dehumidification-control-default-case",
        ]
    );

    let mut state = State::new(IdealLoadsAirSystemId(7));
    for (index, route) in [U, N, P, C0, Q, H, CSH].into_iter().enumerate() {
        let snapshot =
            advance(&mut state, predecessor(route, index + 1)).expect("CP367 pure transition");
        assert!(
            !snapshot
                .dehumidification_control_default_supply_humidity_ratio_mixed_air_assignment_executed
        );
        assert_eq!(
            snapshot.dehumidification_control_none_case_completed_skip,
            route == C0
        );
        assert_eq!(
            snapshot
                .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
            route == Q
        );
        assert_eq!(
            snapshot.dehumidification_control_humidistat_case_completed_skip,
            route == H
        );
        assert_eq!(
            snapshot
                .dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
            route == CSH
        );
        assert_eq!(
            cooling_default_supply_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release(
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
            state
                .dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count,
        ],
        [1; 7]
    );
    assert_eq!(state.transition_count, 7);
    assert_eq!(state.mixed_air_humidity_ratio_read_count, 0);
    assert_eq!(state.supply_humidity_ratio_assignment_count, 0);
    assert_eq!(state.source_site_execution_count, 0);
}

#[test]
fn provenance_selector_and_one_hot_corruption_are_transactional() {
    let mut forged = predecessor(CSH, 1);
    forged.source = "forged";
    rejects_without_mutation(forged);

    let mut forged = predecessor(CSH, 1);
    forged.predecessor_dehumidification_control_type =
        Some(DehumidificationControlType::Humidistat);
    rejects_without_mutation(forged);

    let mut forged = predecessor(CSH, 1);
    forged.dehumidification_control_none_case_completed_skip = true;
    rejects_without_mutation(forged);

    let mut forged = predecessor(C0, 1);
    forged.dehumidification_control_constant_supply_humidity_ratio_case_exited_via_break = true;
    rejects_without_mutation(forged);
}

#[test]
fn every_route_counter_overflow_and_every_nonzero_source_counter_are_transactional() {
    macro_rules! reject_overflow {
        ($field:ident, $route:expr) => {{
            let mut state = State::new(IdealLoadsAirSystemId(7));
            state.$field = usize::MAX;
            let before = state.clone();
            assert!(advance(&mut state, predecessor($route, 1)).is_none());
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
        dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count,
        CSH
    );
    reject_overflow!(
        witnessed_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count,
        CSH
    );
    reject_overflow!(mixed_air_humidity_ratio_read_count, CSH);
    reject_overflow!(supply_humidity_ratio_assignment_count, CSH);
    reject_overflow!(source_site_execution_count, CSH);
}

fn rejects_without_mutation(predecessor: Predecessor) {
    let mut state = State::new(IdealLoadsAirSystemId(7));
    let before = state.clone();
    assert!(advance(&mut state, predecessor).is_none());
    assert_eq!(state, before);
}

fn predecessor(route: Route, ordinal: usize) -> Predecessor {
    let active = matches!(route, C0 | Q | H | CSH);
    let selector = match route {
        C0 => Some(DehumidificationControlType::None),
        Q => Some(DehumidificationControlType::ConstantSensibleHeatRatio),
        H => Some(DehumidificationControlType::Humidistat),
        CSH => Some(DehumidificationControlType::ConstantSupplyHumidityRatio),
        _ => None,
    };
    Predecessor {
        source: PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE_ORDER,
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
        predecessor_dehumidification_control_constant_supply_humidity_ratio_assignment_executed:
            route == CSH,
        dehumidification_control_none_case_completed_skip: route == C0,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: route == Q,
        dehumidification_control_humidistat_case_completed_skip: route == H,
        dehumidification_control_constant_supply_humidity_ratio_case_exited_via_break: route == CSH,
    }
}
