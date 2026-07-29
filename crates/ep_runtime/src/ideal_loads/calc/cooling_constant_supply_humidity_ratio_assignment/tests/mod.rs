mod release;

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use super::{
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentRuntimeState as State,
    advance_cooling_constant_supply_humidity_ratio_assignment_state as advance,
    advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_assignment,
    cooling_constant_supply_humidity_ratio_assignment_snapshot_is_exact_direct_release,
    cooling_constant_supply_humidity_ratio_assignment_snapshots_match_bit_exact,
    private_constant_supply_humidity_ratio_assignment_counterfactual_from_direct_release,
    private_constant_supply_humidity_ratio_assignment_counterfactual_links_to_direct_release,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_SOURCE_ORDER,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntrySnapshot as Predecessor,
};

const U: Route = Route::UnitOff;
const N: Route = Route::NonCooling;
const P: Route = Route::PositiveGuardFalseFallthrough;
const C0: Route = Route::DehumidificationControlNoneCaseCompletedSkip;
const Q: Route = Route::DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip;
const H: Route = Route::DehumidificationControlHumidistatCaseCompletedSkip;
const CSH: Route = Route::DehumidificationControlConstantSupplyHumidityRatioAssigned;

#[test]
fn source_boundary_two_sites_and_seven_route_algebra_are_exact() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2235"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2236"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER,
        &[
            "read-purchased-air-minimum-cooling-supply-air-humidity-ratio-for-constant-supply-humidity-ratio-assignment",
            "assign-purchased-air-supply-humidity-ratio-for-constant-supply-humidity-ratio-case",
        ]
    );

    let minimum = f64::from_bits(0x3f7f_8a09_6bb9_8c7e);
    let mut state = State::new(IdealLoadsAirSystemId(7));
    for (index, route) in [U, N, P, C0, Q, H, CSH].into_iter().enumerate() {
        let snapshot = advance(
            &mut state,
            predecessor(route, index + 1),
            (route == CSH).then_some(minimum),
        )
        .expect("CP365 pure transition");
        assert_eq!(
            snapshot.dehumidification_control_constant_supply_humidity_ratio_assignment_executed,
            route == CSH
        );
        assert_eq!(
            snapshot.minimum_cooling_supply_air_humidity_ratio_read,
            route == CSH
        );
        assert_eq!(snapshot.supply_humidity_ratio_assigned, route == CSH);
        for value in [
            snapshot.minimum_cooling_supply_air_humidity_ratio,
            snapshot.assigned_supply_humidity_ratio,
            snapshot.resulting_supply_humidity_ratio,
        ] {
            assert_eq!(
                value.map(f64::to_bits),
                (route == CSH).then_some(minimum.to_bits())
            );
        }
        assert_eq!(
            cooling_constant_supply_humidity_ratio_assignment_snapshot_is_exact_direct_release(
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
            state.dehumidification_control_constant_supply_humidity_ratio_assignment_count,
        ],
        [1; 7]
    );
    assert_eq!(state.transition_count, 7);
    assert_eq!(state.source_site_execution_count, 2);
    assert_eq!(
        state.minimum_cooling_supply_air_humidity_ratio_read_count,
        1
    );
    assert_eq!(state.supply_humidity_ratio_assignment_count, 1);
}

#[test]
fn raw_assignment_preserves_every_binary64_pattern_bit_exact() {
    for minimum in [
        0.0077,
        -0.0,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::from_bits(0x7ff8_0000_0000_0042),
        f64::from_bits(0xfff8_0000_0000_0043),
    ] {
        let snapshot = advance(
            &mut State::new(IdealLoadsAirSystemId(7)),
            predecessor(CSH, 1),
            Some(minimum),
        )
        .expect("private CSH transition");
        for value in [
            snapshot.minimum_cooling_supply_air_humidity_ratio,
            snapshot.assigned_supply_humidity_ratio,
            snapshot.resulting_supply_humidity_ratio,
        ] {
            assert_eq!(value.map(f64::to_bits), Some(minimum.to_bits()));
        }
    }
}

#[test]
fn bit_exact_matcher_handles_nan_payloads_and_signed_zero() {
    let nan = f64::from_bits(0x7ff8_0000_0000_0042);
    let left = advance(
        &mut State::new(IdealLoadsAirSystemId(7)),
        predecessor(CSH, 1),
        Some(nan),
    )
    .expect("left");
    let right = advance(
        &mut State::new(IdealLoadsAirSystemId(7)),
        predecessor(CSH, 1),
        Some(nan),
    )
    .expect("right");
    assert!(
        cooling_constant_supply_humidity_ratio_assignment_snapshots_match_bit_exact(left, right)
    );

    let mut different_payload = right;
    different_payload.resulting_supply_humidity_ratio = Some(f64::from_bits(0x7ff8_0000_0000_0043));
    assert!(
        !cooling_constant_supply_humidity_ratio_assignment_snapshots_match_bit_exact(
            left,
            different_payload
        )
    );

    let positive_zero = advance(
        &mut State::new(IdealLoadsAirSystemId(7)),
        predecessor(CSH, 1),
        Some(0.0),
    )
    .expect("positive zero");
    let negative_zero = advance(
        &mut State::new(IdealLoadsAirSystemId(7)),
        predecessor(CSH, 1),
        Some(-0.0),
    )
    .expect("negative zero");
    assert!(
        !cooling_constant_supply_humidity_ratio_assignment_snapshots_match_bit_exact(
            positive_zero,
            negative_zero
        )
    );
}

#[test]
fn operand_presence_is_route_exact_and_transactional() {
    for (route, operand) in [(CSH, None), (C0, Some(0.0077)), (U, Some(f64::NAN))] {
        let mut state = State::new(IdealLoadsAirSystemId(7));
        let before = state.clone();
        assert!(advance(&mut state, predecessor(route, 1), operand).is_none());
        assert_eq!(state, before);
    }
}

#[test]
fn forged_provenance_selector_prefix_and_one_hot_lineage_are_rejected() {
    let mut forged = predecessor(CSH, 1);
    forged.source = "forged";
    rejects_without_mutation(forged, Some(0.0077));

    let mut forged = predecessor(CSH, 1);
    forged.source_order = &["forged"];
    rejects_without_mutation(forged, Some(0.0077));

    let mut forged = predecessor(CSH, 1);
    forged.predecessor_dehumidification_control_type =
        Some(DehumidificationControlType::Humidistat);
    rejects_without_mutation(forged, Some(0.0077));

    let mut forged = predecessor(CSH, 1);
    forged.predecessor_positive_supply_mass_flow_body_entered = false;
    rejects_without_mutation(forged, Some(0.0077));

    let mut forged = predecessor(CSH, 1);
    forged.dehumidification_control_none_case_completed_skip = true;
    rejects_without_mutation(forged, Some(0.0077));

    let mut forged = predecessor(CSH, 1);
    forged.predecessor_dehumidification_control_none_case_completed_skip = true;
    rejects_without_mutation(forged, Some(0.0077));
}

#[test]
fn every_counter_overflow_rejects_without_mutation() {
    macro_rules! reject_overflow {
        ($field:ident, $route:expr) => {{
            let mut state = State::new(IdealLoadsAirSystemId(7));
            state.$field = usize::MAX;
            let before = state.clone();
            assert!(
                advance(
                    &mut state,
                    predecessor($route, 1),
                    ($route == CSH).then_some(0.0077),
                )
                .is_none()
            );
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
        dehumidification_control_constant_supply_humidity_ratio_assignment_count,
        CSH
    );
    reject_overflow!(
        witnessed_dehumidification_control_constant_supply_humidity_ratio_assignment_count,
        CSH
    );
    reject_overflow!(source_site_execution_count, CSH);
    reject_overflow!(minimum_cooling_supply_air_humidity_ratio_read_count, CSH);
    reject_overflow!(supply_humidity_ratio_assignment_count, CSH);
}

#[test]
fn two_site_increment_preflight_rejects_max_minus_one() {
    let mut state = State::new(IdealLoadsAirSystemId(7));
    state.source_site_execution_count = usize::MAX - 1;
    let before = state.clone();
    assert!(advance(&mut state, predecessor(CSH, 1), Some(0.0077)).is_none());
    assert_eq!(state, before);
}

fn rejects_without_mutation(predecessor: Predecessor, operand: Option<f64>) {
    let mut state = State::new(IdealLoadsAirSystemId(7));
    let before = state.clone();
    assert!(advance(&mut state, predecessor, operand).is_none());
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
        source: PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_SOURCE_ORDER,
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
        predecessor_dehumidification_control_humidistat_case_exited_via_break: route == H,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            route == CSH,
        dehumidification_control_none_case_completed_skip: route == C0,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: route == Q,
        dehumidification_control_humidistat_case_completed_skip: route == H,
        dehumidification_control_constant_supply_humidity_ratio_case_entered: route == CSH,
    }
}
