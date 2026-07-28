mod release_corruption;

use super::*;
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
};
use crate::psychrometrics::energyplus_psy_h_fn_tdb_w;

#[derive(Clone, Copy)]
enum Route {
    UnitOff,
    NonCooling,
    GuardFalse,
    Assigned,
}

fn predecessor(
    route: Route,
    ordinal: usize,
) -> PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot {
    let unit_off = matches!(route, Route::UnitOff);
    let non_cooling = matches!(route, Route::NonCooling);
    let guard_false = matches!(route, Route::GuardFalse);
    let assigned = matches!(route, Route::Assigned);
    let cooling = guard_false || assigned;
    let temperature = assigned.then_some(15.0);
    let humidity = assigned.then_some(0.005);
    let enthalpy = assigned.then_some(energyplus_psy_h_fn_tdb_w(15.0, 0.005));

    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
        system: ep_model::IdealLoadsAirSystemId(3),
        parent_call_ordinal: ordinal,
        controlled_zone: ep_model::ZoneId(4),
        unit_body_entered: !unit_off,
        predecessor_cooling_body_entered: cooling,
        predecessor_no_outdoor_air_fallback_entered: cooling,
        predecessor_positive_supply_mass_flow_body_entered: assigned,
        predecessor_active_guard_false_fallthrough: guard_false,
        unit_off_skipped: unit_off,
        non_cooling_skipped: non_cooling,
        positive_guard_false_fallthrough_skipped: guard_false,
        supply_enthalpy_assignment_executed: assigned,
        supply_temperature_for_enthalpy_read: assigned,
        supply_temperature_c: temperature,
        supply_humidity_ratio_for_enthalpy_read: assigned,
        supply_humidity_ratio: humidity,
        psychrometric_supply_enthalpy_evaluated: assigned,
        psychrometric_supply_enthalpy_result_j_per_kg: enthalpy,
        supply_enthalpy_assigned: assigned,
        supply_enthalpy_j_per_kg: enthalpy,
    }
}

fn advance(
    state: &mut PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRuntimeState,
    route: Route,
    ordinal: usize,
    cooling_limit: ep_model::IdealLoadsLimit,
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot {
    advance_cooling_positive_supply_capacity_limit_guard_state(
        state,
        predecessor(route, ordinal),
        matches!(route, Route::Assigned).then_some(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardActiveInput {
                cooling_limit,
            },
        ),
    )
}

#[test]
fn source_boundary_and_exact_five_sites_are_stable() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2195"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2196"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE_ORDER,
        &[
            "read-cooling-limit-for-capacity-comparison",
            "compare-cooling-limit-equal-to-capacity",
            "read-cooling-limit-for-flow-rate-and-capacity-comparison-after-first-false",
            "compare-cooling-limit-equal-to-flow-rate-and-capacity",
            "enter-capacity-limit-body-if-compound-condition-satisfied",
        ]
    );
}

#[test]
fn lazy_selector_guard_matches_all_four_limits_and_counts_dynamic_sites() {
    use ep_model::IdealLoadsLimit::{
        LimitCapacity, LimitFlowRate, LimitFlowRateAndCapacity, NoLimit,
    };

    for (limit, capacity, second, combined, body, sites) in [
        (NoLimit, false, true, false, false, 4),
        (LimitFlowRate, false, true, false, false, 4),
        (LimitCapacity, true, false, false, true, 3),
        (LimitFlowRateAndCapacity, false, true, true, true, 5),
    ] {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRuntimeState::new(
                ep_model::IdealLoadsAirSystemId(3),
            );
        let snapshot = advance(&mut state, Route::Assigned, 1, limit);

        assert!(
            cooling_positive_supply_capacity_limit_guard_snapshot_is_exact_direct_release(
                snapshot
            )
        );
        assert!(snapshot.capacity_limit_guard_evaluated);
        assert_eq!(snapshot.first_cooling_limit, Some(limit));
        assert_eq!(snapshot.cooling_limit_capacity, Some(capacity));
        assert_eq!(snapshot.second_cooling_limit_read, second);
        assert_eq!(snapshot.second_cooling_limit, second.then_some(limit));
        assert_eq!(
            snapshot.cooling_limit_flow_rate_and_capacity,
            second.then_some(combined)
        );
        assert_eq!(snapshot.cooling_limit_condition_satisfied, Some(body));
        assert_eq!(snapshot.cooling_limit_rejected, !body);
        assert_eq!(snapshot.capacity_limit_body_entered, body);
        assert_eq!(snapshot.active_guard_false_fallthrough, !body);
        assert_eq!(state.source_site_execution_count, sites);
        assert_eq!(state.cooling_limit_rejected_count, usize::from(!body));
        assert_eq!(
            state.active_guard_false_fallthrough_count,
            usize::from(!body)
        );
    }
}

#[test]
fn inherited_skips_execute_no_cp337_source_sites_or_selector_reads() {
    for (route, unit_off, non_cooling, guard_false) in [
        (Route::UnitOff, true, false, false),
        (Route::NonCooling, false, true, false),
        (Route::GuardFalse, false, false, true),
    ] {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRuntimeState::new(
                ep_model::IdealLoadsAirSystemId(3),
            );
        let snapshot = advance(
            &mut state,
            route,
            1,
            ep_model::IdealLoadsLimit::LimitCapacity,
        );

        assert!(
            cooling_positive_supply_capacity_limit_guard_snapshot_is_exact_direct_release(
                snapshot
            )
        );
        assert_eq!(snapshot.unit_off_skipped, unit_off);
        assert_eq!(snapshot.non_cooling_skipped, non_cooling);
        assert_eq!(
            snapshot.positive_guard_false_fallthrough_skipped,
            guard_false
        );
        assert!(!snapshot.capacity_limit_guard_evaluated);
        assert!(!snapshot.first_cooling_limit_read);
        assert!(snapshot.first_cooling_limit.is_none());
        assert!(!snapshot.second_cooling_limit_read);
        assert!(snapshot.second_cooling_limit.is_none());
        assert!(snapshot.cooling_limit_condition_satisfied.is_none());
        assert!(!snapshot.cooling_limit_rejected);
        assert!(!snapshot.capacity_limit_body_entered);
        assert!(!snapshot.active_guard_false_fallthrough);
        assert_eq!(state.source_site_execution_count, 0);
    }
}

#[test]
fn counters_partition_inherited_and_active_routes() {
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
    for (ordinal, route) in [
        Route::UnitOff,
        Route::NonCooling,
        Route::GuardFalse,
        Route::Assigned,
    ]
    .into_iter()
    .enumerate()
    {
        advance(
            &mut state,
            route,
            ordinal + 1,
            ep_model::IdealLoadsLimit::LimitFlowRateAndCapacity,
        );
    }

    assert_eq!(state.transition_count, 4);
    assert_eq!(state.unit_off_skip_count, 1);
    assert_eq!(state.non_cooling_skip_count, 1);
    assert_eq!(state.positive_guard_false_fallthrough_skip_count, 1);
    assert_eq!(state.capacity_limit_guard_evaluation_count, 1);
    assert_eq!(state.first_cooling_limit_read_count, 1);
    assert_eq!(state.cooling_limit_capacity_comparison_count, 1);
    assert_eq!(state.cooling_limit_capacity_match_count, 0);
    assert_eq!(state.second_cooling_limit_read_count, 1);
    assert_eq!(
        state.cooling_limit_flow_rate_and_capacity_comparison_count,
        1
    );
    assert_eq!(
        state.cooling_limit_flow_rate_and_capacity_match_count,
        1
    );
    assert_eq!(state.capacity_limit_body_entry_count, 1);
    assert_eq!(state.cooling_limit_rejected_count, 0);
    assert_eq!(state.active_guard_false_fallthrough_count, 0);
    assert_eq!(state.source_site_execution_count, 5);
}

#[test]
fn exact_predicate_rejects_provenance_short_circuit_and_redundant_false_drift() {
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
    let exact = advance(
        &mut state,
        Route::Assigned,
        1,
        ep_model::IdealLoadsLimit::LimitCapacity,
    );

    let mut forged_source = exact;
    forged_source.source = "forged";
    assert!(
        !cooling_positive_supply_capacity_limit_guard_snapshot_is_exact_direct_release(
            forged_source
        )
    );
    let mut forged_second = exact;
    forged_second.second_cooling_limit_read = true;
    assert!(
        !cooling_positive_supply_capacity_limit_guard_snapshot_is_exact_direct_release(
            forged_second
        )
    );
    let mut forged_rejected = exact;
    forged_rejected.cooling_limit_rejected = true;
    assert!(
        !cooling_positive_supply_capacity_limit_guard_snapshot_is_exact_direct_release(
            forged_rejected
        )
    );
}
