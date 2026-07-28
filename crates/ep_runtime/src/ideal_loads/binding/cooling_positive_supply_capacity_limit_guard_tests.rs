use super::*;
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    cooling_positive_supply_capacity_limit_guard_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_positive_supply_capacity_limit_guard_lifecycle_summary,
};

fn run_case(
    cooling_limit: IdealLoadsLimit,
    maximum_flow_m3_per_s: Option<f64>,
    maximum_capacity_w: Option<f64>,
    independent_load_w: f64,
    availability: f64,
) -> (
    PurchasedAirRuntimeState,
    DirectZonePurchasedAirScheduledCouplingOutput,
) {
    let (model, cache) = fixture(|typed| {
        let system = &mut typed.ideal_loads_air_systems[0];
        system.cooling_limit = cooling_limit;
        system.maximum_cooling_air_flow_rate_m3_per_s =
            maximum_flow_m3_per_s.map(AutosizeOrNumber::Value);
        system.maximum_total_cooling_capacity_w = maximum_capacity_w.map(AutosizeOrNumber::Value);
        schedule_mut(typed, ScheduleId(3)).hourly_value = availability;
    });
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut zone_state = zone_state_for_temp_independent_load(independent_load_w);
    let mut runtime = PurchasedAirRuntimeState::default();
    let output = couple_model_bound_direct_zone_purchased_air(
        DirectZonePurchasedAirScheduledCouplingInput {
            binding: &binding,
            schedule_cache: &cache,
            schedule_sample_index: 0,
            zone_state: &mut zone_state,
            purchased_air_runtime_state: &mut runtime,
            begin_environment: true,
            barometric_pressure_pa: binding.limit_context.barometric_pressure_pa,
            system_timestep_seconds: binding.nominal_system_timestep_seconds,
        },
    )
    .expect("source-ordered CP337 coupling");
    (runtime, output)
}

#[test]
fn scheduled_binding_uses_only_the_typed_cooling_limit_selector() {
    for (
        cooling_limit,
        maximum_flow_m3_per_s,
        maximum_capacity_w,
        capacity_match,
        combined_match,
        source_sites,
    ) in [
        (
            IdealLoadsLimit::LimitCapacity,
            None,
            Some(5_000.0),
            true,
            false,
            3,
        ),
        (
            IdealLoadsLimit::LimitFlowRateAndCapacity,
            Some(0.05),
            Some(5_000.0),
            false,
            true,
            5,
        ),
        (IdealLoadsLimit::NoLimit, None, None, false, false, 4),
        (
            IdealLoadsLimit::LimitFlowRate,
            Some(0.05),
            None,
            false,
            false,
            4,
        ),
    ] {
        let (runtime, output) = run_case(
            cooling_limit,
            maximum_flow_m3_per_s,
            maximum_capacity_w,
            3_000.0,
            1.0,
        );
        let predecessor = output.calculation_cooling_positive_supply_enthalpy_assignment;
        let guard = output.calculation_cooling_positive_supply_capacity_limit_guard;
        let condition_satisfied = capacity_match || combined_match;

        assert!(predecessor.supply_enthalpy_assignment_executed);
        assert!(cooling_positive_supply_capacity_limit_guard_snapshot_is_exact_direct_release(
            guard,
        ));
        assert!(guard.capacity_limit_guard_evaluated);
        assert!(guard.first_cooling_limit_read);
        assert_eq!(guard.first_cooling_limit, Some(cooling_limit));
        assert!(guard.cooling_limit_capacity_comparison_evaluated);
        assert_eq!(guard.cooling_limit_capacity, Some(capacity_match));
        assert_eq!(guard.second_cooling_limit_read, !capacity_match);
        assert_eq!(
            guard.second_cooling_limit,
            (!capacity_match).then_some(cooling_limit)
        );
        assert_eq!(
            guard.cooling_limit_flow_rate_and_capacity_comparison_evaluated,
            !capacity_match
        );
        assert_eq!(
            guard.cooling_limit_flow_rate_and_capacity,
            (!capacity_match).then_some(combined_match)
        );
        assert_eq!(
            guard.cooling_limit_condition_satisfied,
            Some(condition_satisfied)
        );
        assert_eq!(guard.cooling_limit_rejected, !condition_satisfied);
        assert_eq!(guard.capacity_limit_body_entered, condition_satisfied);
        assert_eq!(
            guard.active_guard_false_fallthrough,
            !condition_satisfied
        );

        let state =
            purchased_air_calc_cooling_positive_supply_capacity_limit_guard_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP337 lifecycle")
            .state;
        assert_counter_shape(
            &state,
            1,
            usize::from(capacity_match),
            usize::from(combined_match),
            source_sites,
        );
    }
}

#[test]
fn scheduled_binding_preserves_all_complete_null_skip_routes() {
    for (cooling_limit, maximum_capacity_w, independent_load_w, availability, expected) in [
        (
            IdealLoadsLimit::NoLimit,
            None,
            3_000.0,
            0.0,
            (true, false, false),
        ),
        (
            IdealLoadsLimit::NoLimit,
            None,
            0.0,
            1.0,
            (false, true, false),
        ),
        (
            IdealLoadsLimit::LimitCapacity,
            Some(0.0),
            3_000.0,
            1.0,
            (false, false, true),
        ),
    ] {
        let (runtime, output) = run_case(
            cooling_limit,
            None,
            maximum_capacity_w,
            independent_load_w,
            availability,
        );
        let guard = output.calculation_cooling_positive_supply_capacity_limit_guard;
        let (unit_off, non_cooling, positive_guard_false) = expected;

        assert_eq!(guard.unit_off_skipped, unit_off);
        assert_eq!(guard.non_cooling_skipped, non_cooling);
        assert_eq!(
            guard.positive_guard_false_fallthrough_skipped,
            positive_guard_false
        );
        assert_snapshot_has_no_selector_evidence(guard);
        let state =
            purchased_air_calc_cooling_positive_supply_capacity_limit_guard_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP337 skipped lifecycle")
            .state;
        assert_eq!(state.transition_count, 1);
        assert_eq!(state.unit_off_skip_count, usize::from(unit_off));
        assert_eq!(state.non_cooling_skip_count, usize::from(non_cooling));
        assert_eq!(
            state.positive_guard_false_fallthrough_skip_count,
            usize::from(positive_guard_false)
        );
        assert_counter_shape(&state, 0, 0, 0, 0);
    }
}

fn assert_snapshot_has_no_selector_evidence(
    snapshot: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
) {
    assert!(!snapshot.capacity_limit_guard_evaluated);
    assert!(!snapshot.first_cooling_limit_read);
    assert!(snapshot.first_cooling_limit.is_none());
    assert!(!snapshot.cooling_limit_capacity_comparison_evaluated);
    assert!(snapshot.cooling_limit_capacity.is_none());
    assert!(!snapshot.second_cooling_limit_read);
    assert!(snapshot.second_cooling_limit.is_none());
    assert!(
        !snapshot.cooling_limit_flow_rate_and_capacity_comparison_evaluated
    );
    assert!(snapshot.cooling_limit_flow_rate_and_capacity.is_none());
    assert!(snapshot.cooling_limit_condition_satisfied.is_none());
    assert!(!snapshot.cooling_limit_rejected);
    assert!(!snapshot.capacity_limit_body_entered);
    assert!(!snapshot.active_guard_false_fallthrough);
}

fn assert_counter_shape(
    state: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRuntimeState,
    active: usize,
    capacity_matches: usize,
    combined_matches: usize,
    source_sites: usize,
) {
    let second_comparisons = active - capacity_matches;
    let body_entries = capacity_matches + combined_matches;
    assert_eq!(state.capacity_limit_guard_evaluation_count, active);
    assert_eq!(state.source_site_execution_count, source_sites);
    assert_eq!(state.first_cooling_limit_read_count, active);
    assert_eq!(state.cooling_limit_capacity_comparison_count, active);
    assert_eq!(
        state.cooling_limit_capacity_match_count,
        capacity_matches
    );
    assert_eq!(state.second_cooling_limit_read_count, second_comparisons);
    assert_eq!(
        state.cooling_limit_flow_rate_and_capacity_comparison_count,
        second_comparisons
    );
    assert_eq!(
        state.cooling_limit_flow_rate_and_capacity_match_count,
        combined_matches
    );
    assert_eq!(
        state.cooling_limit_rejected_count,
        active - body_entries
    );
    assert_eq!(state.capacity_limit_body_entry_count, body_entries);
    assert_eq!(
        state.active_guard_false_fallthrough_count,
        active - body_entries
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE_ORDER.len(),
        5
    );
}
