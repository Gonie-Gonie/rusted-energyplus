mod public_release;

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use super::{
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_BREAK_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_BREAK_SOURCE_ORDER,
    PurchasedAirCalcCoolingHumidistatCaseBreakRetainedRoute as Route,
    PurchasedAirCalcCoolingHumidistatCaseBreakRuntimeState as State,
    advance_cooling_humidistat_case_break_state as advance,
    advance_direct_no_oa_calc_cooling_humidistat_case_break,
    completed_direct_cooling_humidistat_case_break_is_consistent,
    cooling_humidistat_case_break_snapshot_is_exact_direct_release,
    private_constant_supply_humidity_ratio_counterfactual_from_direct_release,
    private_constant_supply_humidity_ratio_counterfactual_links_to_direct_release,
    private_humidistat_counterfactual_from_direct_release,
    private_humidistat_counterfactual_links_to_direct_release,
    purchased_air_calc_cooling_humidistat_case_break_lifecycle_summary,
};
use crate::ideal_loads::calc::cooling_constant_shr_supply_humidity_ratio_mixed_air_limit::completed_cp355_case;
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitSnapshot as Predecessor,
    PurchasedAirRuntimeState, advance_direct_no_oa_calc_cooling_constant_shr_case_break,
    advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit,
    advance_direct_no_oa_calc_cooling_humidistat_case_entry,
    advance_direct_no_oa_calc_cooling_humidistat_moisture_demand_assignment,
    advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment,
    advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit,
    advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit,
};

const U: Route = Route::UnitOff;
const N: Route = Route::NonCooling;
const P: Route = Route::PositiveGuardFalseFallthrough;
const C0: Route = Route::DehumidificationControlNoneCaseCompletedSkip;
const Q: Route = Route::DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip;
const H: Route = Route::DehumidificationControlHumidistatCaseBreak;
const CSH: Route = Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip;
const PRIVATE_DEMAND: f64 = -0.001;
const PRIVATE_ZONE_HUMIDITY: f64 = 0.008;

#[test]
fn source_boundary_single_site_and_seven_route_algebra_are_exact() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_BREAK_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2233"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2235"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_BREAK_SOURCE_ORDER,
        &["exit-purchased-air-dehumidification-control-humidistat-case-via-break"]
    );

    let mut state = State::new(IdealLoadsAirSystemId(7));
    for (index, route) in [U, N, P, C0, Q, H, CSH].into_iter().enumerate() {
        let snapshot = advance(&mut state, predecessor(route, index + 1));
        assert!(snapshot.is_some(), "CP363 pure transition must succeed");
        let Some(snapshot) = snapshot else {
            return;
        };
        assert_eq!(
            snapshot.dehumidification_control_humidistat_case_exited_via_break,
            route == H
        );
        if route == H {
            assert!(
                !snapshot
                    .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
            );
        }
    }
    assert_eq!(
        [
            state.unit_off_skip_count,
            state.non_cooling_skip_count,
            state.positive_guard_false_fallthrough_skip_count,
            state.dehumidification_control_none_case_completed_skip_count,
            state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
            state.dehumidification_control_humidistat_case_break_count,
            state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        ],
        [1; 7]
    );
    assert_eq!(state.transition_count, 7);
    assert_eq!(state.source_site_execution_count, 1);
}

#[test]
fn transition_is_evidence_only_and_all_overflow_is_transactional() {
    let mut noncanonical_numeric_owner = predecessor(H, 1);
    noncanonical_numeric_owner.mixed_air_humidity_ratio =
        Some(f64::from_bits(0x7ff8_0000_0000_00a5));
    noncanonical_numeric_owner.resulting_supply_humidity_ratio =
        Some(f64::from_bits(0x7ff8_0000_0000_00b6));
    let mut state = State::new(noncanonical_numeric_owner.system);
    assert!(advance(&mut state, noncanonical_numeric_owner).is_some());

    macro_rules! reject_overflow {
        ($field:ident, $route:expr) => {{
            let mut state = State::new(IdealLoadsAirSystemId(7));
            state.$field = usize::MAX;
            let before = state.clone();
            assert!(advance(&mut state, predecessor($route, 1)).is_none());
            assert_eq!(state, before);
        }};
    }
    reject_overflow!(transition_count, H);
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
    reject_overflow!(dehumidification_control_humidistat_case_break_count, H);
    reject_overflow!(source_site_execution_count, H);
    reject_overflow!(
        witnessed_dehumidification_control_humidistat_case_break_count,
        H
    );
    reject_overflow!(
        dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        CSH
    );
    reject_overflow!(
        witnessed_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        CSH
    );
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
        source:
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE_ORDER,
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
        predecessor_dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_executed:
            route == H,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            route == CSH,
        predecessor_resulting_supply_humidity_ratio_for_dehumidification: (route == H)
            .then_some(0.008),
        dehumidification_control_none_case_completed_skip: route == C0,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: route == Q,
        dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_executed:
            route == H,
        dehumidification_control_constant_supply_humidity_ratio_case_selected_skip: route == CSH,
        mixed_air_humidity_ratio_for_minimum_read: route == H,
        mixed_air_humidity_ratio: (route == H).then_some(0.007),
        supply_humidity_ratio_for_dehumidification_for_mixed_air_limit_minimum_read: route == H,
        supply_humidity_ratio_for_dehumidification_before_mixed_air_limit: (route == H)
            .then_some(0.008),
        source_shaped_two_argument_minimum_evaluated: route == H,
        minimum_supply_humidity_ratio: (route == H).then_some(0.007),
        supply_humidity_ratio_assignment_performed: route == H,
        assigned_supply_humidity_ratio: (route == H).then_some(0.007),
        resulting_supply_humidity_ratio: (route == H).then_some(0.007),
    }
}
