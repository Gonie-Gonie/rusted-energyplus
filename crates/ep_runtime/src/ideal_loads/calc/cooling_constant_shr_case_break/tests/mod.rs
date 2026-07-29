use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use super::release::{
    private_active_predecessor_links_to_direct_release, snapshots_match_exact_for_test,
};
use super::{
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_CASE_BREAK_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_CASE_BREAK_SOURCE_ORDER,
    PurchasedAirCalcCoolingConstantShrCaseBreakRetainedRoute as Route,
    PurchasedAirCalcCoolingConstantShrCaseBreakRuntimeState as State,
    advance_cooling_constant_shr_case_break_state as advance,
    advance_direct_no_oa_calc_cooling_constant_shr_case_break,
    completed_direct_cooling_constant_shr_case_break_is_consistent,
    cooling_constant_shr_case_break_snapshot_is_exact_direct_release,
    private_humidistat_counterfactual_from_direct_release,
    private_humidistat_counterfactual_links_to_direct_release,
    purchased_air_calc_cooling_constant_shr_case_break_lifecycle_summary,
};
use crate::ideal_loads::calc::cooling_constant_shr_supply_humidity_ratio_mixed_air_limit::{
    completed_cp355_case, private_active_counterfactual_from_direct_release,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitSnapshot as Predecessor,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit,
};

const U: Route = Route::UnitOff;
const N: Route = Route::NonCooling;
const P: Route = Route::PositiveGuardFalseFallthrough;
const C0: Route = Route::DehumidificationControlNoneCaseCompletedSkip;
const Q: Route = Route::DehumidificationControlConstantSensibleHeatRatioCaseBreak;
const H: Route = Route::DehumidificationControlHumidistatCaseSelectedSkip;
const CSH: Route = Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip;

#[test]
fn source_boundary_single_site_and_seven_route_algebra_are_exact() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_CASE_BREAK_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2227"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2229"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_CASE_BREAK_SOURCE_ORDER,
        &[
            "exit-purchased-air-dehumidification-control-constant-sensible-heat-ratio-case-via-break"
        ]
    );

    let mut state = State::new(IdealLoadsAirSystemId(7));
    for (index, route) in [U, N, P, C0, Q, H, CSH].into_iter().enumerate() {
        let snapshot = advance(&mut state, predecessor(route, index + 1)).expect("CP357 route");
        assert_eq!(
            snapshot.dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break,
            route == Q
        );
        if route == Q {
            assert!(!snapshot.dehumidification_control_humidistat_case_selected_skip);
        }
    }
    assert_eq!(
        [
            state.unit_off_skip_count,
            state.non_cooling_skip_count,
            state.positive_guard_false_fallthrough_skip_count,
            state.dehumidification_control_none_case_completed_skip_count,
            state.dehumidification_control_constant_sensible_heat_ratio_case_break_count,
            state.dehumidification_control_humidistat_case_selected_skip_count,
            state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        ],
        [1; 7]
    );
    assert_eq!(state.transition_count, 7);
    assert_eq!(state.source_site_execution_count, 1);
}

#[test]
fn transition_is_evidence_only_and_overflow_is_transactional() {
    let mut noncanonical_numeric_owner = predecessor(Q, 1);
    for value in [
        &mut noncanonical_numeric_owner.supply_humidity_ratio_before_mixed_air_limit,
        &mut noncanonical_numeric_owner.mixed_air_humidity_ratio,
        &mut noncanonical_numeric_owner.minimum_supply_humidity_ratio,
        &mut noncanonical_numeric_owner.assigned_supply_humidity_ratio,
        &mut noncanonical_numeric_owner.resulting_supply_humidity_ratio,
    ] {
        *value = Some(f64::from_bits(0x7ff8_0000_0000_00a5));
    }
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
    reject_overflow!(transition_count, Q);
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
        dehumidification_control_constant_sensible_heat_ratio_case_break_count,
        Q
    );
    reject_overflow!(source_site_execution_count, Q);
    reject_overflow!(
        witnessed_dehumidification_control_constant_sensible_heat_ratio_case_break_count,
        Q
    );
    reject_overflow!(
        dehumidification_control_humidistat_case_selected_skip_count,
        H
    );
    reject_overflow!(
        witnessed_dehumidification_control_humidistat_case_selected_skip_count,
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

#[test]
fn public_direct_routes_skip_break_and_private_q_uses_only_cp356_bridge() {
    for (demand, availability, capacity, expected) in [
        (-1_000.0, 0.0, true, (true, false, false, false)),
        (1.0, 1.0, true, (false, true, false, false)),
        (-1.0e-40, 1.0, true, (false, false, true, false)),
        (-1_000.0, 1.0, false, (false, false, false, true)),
    ] {
        let (mut runtime, system, predecessor) =
            completed_cp356_case(demand, availability, capacity).expect("completed CP356 case");
        let snapshot = advance_direct_no_oa_calc_cooling_constant_shr_case_break(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP357 direct release");
        assert!(cooling_constant_shr_case_break_snapshot_is_exact_direct_release(snapshot));
        assert_eq!(
            (
                snapshot.unit_off_skipped,
                snapshot.non_cooling_skipped,
                snapshot.positive_guard_false_fallthrough_skipped,
                snapshot.dehumidification_control_none_case_completed_skip,
            ),
            expected
        );
        assert!(
            !snapshot.dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break
        );
        let summary = purchased_air_calc_cooling_constant_shr_case_break_lifecycle_summary(
            &runtime, system.id,
        )
        .expect("CP357 lifecycle");
        assert_eq!(summary.state.latest, Some(snapshot));
        assert_eq!(summary.state.source_site_execution_count, 0);
    }

    let (runtime, system, direct) =
        completed_cp356_case(-100_000.0, 1.0, true).expect("active direct CP356");
    let unit = runtime.units.get(&system.id).expect("selected unit");
    let private_q =
        private_active_counterfactual_from_direct_release(&runtime, unit, &system, direct)
            .expect("canonical private CP356");
    assert!(private_active_predecessor_links_to_direct_release(
        &runtime, unit, &system, direct, private_q
    ));
    let mut state = State::new(system.id);
    let snapshot = advance(&mut state, private_q).expect("private CP357 Q");
    assert!(snapshot.dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break);
    assert!(!snapshot.dehumidification_control_humidistat_case_selected_skip);
    assert_eq!(state.source_site_execution_count, 1);
    assert_eq!(
        state.dehumidification_control_constant_sensible_heat_ratio_case_break_count,
        1
    );
}

#[test]
fn canonical_private_humidistat_bridge_is_exact_and_fail_closed() {
    let (mut runtime, system, predecessor) =
        completed_cp356_case(-100_000.0, 1.0, true).expect("active direct CP356");
    let direct = advance_direct_no_oa_calc_cooling_constant_shr_case_break(
        &mut runtime,
        &system,
        predecessor,
    )
    .expect("direct CP357");
    let unit = runtime.units.get(&system.id).expect("selected unit");
    let private_h =
        private_humidistat_counterfactual_from_direct_release(&runtime, unit, &system, direct)
            .expect("canonical private-H CP357");
    assert_eq!(
        private_h.predecessor_dehumidification_control_type,
        Some(DehumidificationControlType::Humidistat)
    );
    assert!(private_h.predecessor_dehumidification_control_humidistat_case_selected_skip);
    assert!(private_h.dehumidification_control_humidistat_case_selected_skip);
    assert!(!private_h.dehumidification_control_none_case_completed_skip);
    assert!(!private_h.dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break);
    assert!(private_humidistat_counterfactual_links_to_direct_release(
        &runtime, unit, &system, direct, private_h
    ));

    let mut forged = private_h;
    forged.parent_call_ordinal = forged.parent_call_ordinal.wrapping_add(1);
    assert!(!private_humidistat_counterfactual_links_to_direct_release(
        &runtime, unit, &system, direct, forged
    ));
}

#[test]
fn corruption_identity_replay_and_runtime_forge_reject_without_mutation() {
    let (mut runtime, system, mut corrupted_source_order_predecessor) =
        completed_cp356_case(-1_000.0, 1.0, false).expect("completed CP356");
    corrupted_source_order_predecessor.source_order = &[];
    assert_rejected_unchanged(&mut runtime, &system, corrupted_source_order_predecessor);

    let (mut runtime, mut system, mismatched_system_predecessor) =
        completed_cp356_case(-1_000.0, 1.0, false).expect("completed CP356");
    system.id = IdealLoadsAirSystemId(system.id.0.wrapping_add(100));
    assert_rejected_unchanged(&mut runtime, &system, mismatched_system_predecessor);

    let (mut runtime, system, cp356_predecessor) =
        completed_cp356_case(-1_000.0, 1.0, false).expect("completed CP356");
    assert!(
        advance_direct_no_oa_calc_cooling_constant_shr_case_break(
            &mut runtime,
            &system,
            cp356_predecessor,
        )
        .is_ok()
    );
    assert_rejected_unchanged(&mut runtime, &system, cp356_predecessor);

    let (mut runtime, system, corrupted_state_predecessor) =
        completed_cp356_case(-1_000.0, 1.0, false).expect("completed CP356");
    runtime
        .units
        .get_mut(&system.id)
        .expect("selected unit")
        .calc_cooling_constant_shr_case_break
        .source_site_execution_count = 1;
    assert_rejected_unchanged(&mut runtime, &system, corrupted_state_predecessor);

    let snapshot = advance(
        &mut State::new(IdealLoadsAirSystemId(7)),
        predecessor(C0, 1),
    )
    .expect("CP357 C0");
    let mut forged = snapshot;
    forged.dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break = true;
    assert!(snapshots_match_exact_for_test(snapshot, snapshot));
    assert!(!snapshots_match_exact_for_test(snapshot, forged));
    assert!(!cooling_constant_shr_case_break_snapshot_is_exact_direct_release(forged));
}

#[test]
fn coordinated_witness_route_redistribution_is_rejected() {
    let (mut runtime, system, predecessor) =
        completed_cp356_case(-1_000.0, 1.0, false).expect("completed CP356");
    let snapshot = advance_direct_no_oa_calc_cooling_constant_shr_case_break(
        &mut runtime,
        &system,
        predecessor,
    )
    .expect("CP357 direct release");
    let witness = runtime.cooling_constant_shr_case_break_latest_witness(system.id);
    let unit = runtime.units.get(&system.id).expect("selected unit");
    assert!(
        completed_direct_cooling_constant_shr_case_break_is_consistent(
            &runtime, unit, &system, snapshot, witness,
        )
    );

    {
        let state = &mut runtime
            .units
            .get_mut(&system.id)
            .expect("selected unit")
            .calc_cooling_constant_shr_case_break;
        state.witnessed_dehumidification_control_none_case_completed_skip_count = 0;
        state.witnessed_dehumidification_control_constant_sensible_heat_ratio_case_break_count = 1;
    }
    let before = runtime.clone();
    let unit = runtime.units.get(&system.id).expect("selected unit");
    assert!(
        !completed_direct_cooling_constant_shr_case_break_is_consistent(
            &runtime, unit, &system, snapshot, witness,
        )
    );
    assert_eq!(runtime, before);
}

fn completed_cp356_case(
    cooling_demand_w: f64,
    availability: f64,
    capacity_limit: bool,
) -> Option<(
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    Predecessor,
)> {
    let (mut runtime, system, cp355) =
        completed_cp355_case(cooling_demand_w, availability, capacity_limit)?;
    let cp356 =
        advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit(
            &mut runtime,
            &system,
            cp355,
        )
        .ok()?;
    Some((runtime, system, cp356))
}

fn assert_rejected_unchanged(
    runtime: &mut PurchasedAirRuntimeState,
    system: &ep_model::IdealLoadsAirSystem,
    predecessor: Predecessor,
) {
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_constant_shr_case_break(runtime, system, predecessor,)
            .is_err()
    );
    assert_eq!(*runtime, before);
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
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE_ORDER,
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
        predecessor_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_minimum_limit_executed:
            route == Q,
        predecessor_dehumidification_control_humidistat_case_selected_skip: route == H,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            route == CSH,
        dehumidification_control_none_case_completed_skip: route == C0,
        dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_mixed_air_limit_executed:
            route == Q,
        dehumidification_control_humidistat_case_selected_skip: route == H,
        dehumidification_control_constant_supply_humidity_ratio_case_selected_skip: route == CSH,
        supply_humidity_ratio_for_mixed_air_limit_minimum_read: route == Q,
        supply_humidity_ratio_before_mixed_air_limit: (route == Q).then_some(0.008),
        mixed_air_humidity_ratio_for_minimum_read: route == Q,
        mixed_air_humidity_ratio: (route == Q).then_some(0.007),
        source_shaped_two_argument_minimum_evaluated: route == Q,
        minimum_supply_humidity_ratio: (route == Q).then_some(0.007),
        supply_humidity_ratio_assignment_performed: route == Q,
        assigned_supply_humidity_ratio: (route == Q).then_some(0.007),
        resulting_supply_humidity_ratio: (route == Q).then_some(0.007),
    }
}
