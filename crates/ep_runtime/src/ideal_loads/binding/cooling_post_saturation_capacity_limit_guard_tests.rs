use super::*;
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardRuntimeState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardSnapshot,
    cooling_post_saturation_capacity_limit_guard_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_post_saturation_capacity_limit_guard_lifecycle_summary,
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
        system.dehumidification_control_type = DehumidificationControlType::None;
        system.humidification_control_type = HumidificationControlType::None;
        system.minimum_cooling_supply_air_humidity_ratio = f64::NAN;
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
    .expect("source-ordered CP380 coupling");
    (runtime, output)
}

#[test]
fn binding_places_cp380_after_cp379_and_preserves_lazy_selector_reads() {
    for (limit, max_flow, max_capacity, capacity_match, combined_match, source_sites) in [
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
        let (runtime, output) = run_case(limit, max_flow, max_capacity, 3_000.0, 1.0);
        let predecessor = output.calculation_cooling_supply_enthalpy_post_saturation_assignment;
        let snapshot = output.calculation_cooling_post_saturation_capacity_limit_guard;
        let selected = capacity_match || combined_match;

        assert!(predecessor.local_supply_enthalpy_after_saturation_limit_assignment_performed);
        assert!(
            cooling_post_saturation_capacity_limit_guard_snapshot_is_exact_direct_release(snapshot,)
        );
        assert_eq!(snapshot.system, predecessor.system);
        assert_eq!(
            snapshot.parent_call_ordinal,
            predecessor.parent_call_ordinal
        );
        assert_eq!(snapshot.controlled_zone, predecessor.controlled_zone);
        assert!(
            snapshot.predecessor_local_supply_enthalpy_after_saturation_limit_assignment_performed
        );
        assert!(snapshot.capacity_limit_guard_evaluated);
        assert!(snapshot.configured_cooling_limit_owned_read);
        assert!(snapshot.cp337_same_call_selector_lineage_corroborated);
        assert_eq!(snapshot.first_cooling_limit, Some(limit));
        assert_eq!(snapshot.cooling_limit_capacity, Some(capacity_match));
        assert_eq!(snapshot.second_cooling_limit_read, !capacity_match);
        assert_eq!(
            snapshot.second_cooling_limit,
            (!capacity_match).then_some(limit)
        );
        assert_eq!(
            snapshot.cooling_limit_flow_rate_and_capacity,
            (!capacity_match).then_some(combined_match),
        );
        assert_eq!(snapshot.cooling_limit_condition_satisfied, Some(selected));
        assert_eq!(snapshot.capacity_limit_body_entered, selected);
        assert_eq!(snapshot.active_guard_false_fallthrough, !selected);

        let state =
            purchased_air_calc_cooling_post_saturation_capacity_limit_guard_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP380 lifecycle")
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
fn binding_keeps_inactive_cp380_selector_evidence_completely_empty() {
    for (limit, max_capacity, load, availability, expected_routes) in [
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
        let (runtime, output) = run_case(limit, None, max_capacity, load, availability);
        let snapshot = output.calculation_cooling_post_saturation_capacity_limit_guard;
        assert_eq!(
            (
                snapshot.unit_off_skipped,
                snapshot.non_cooling_skipped,
                snapshot.positive_guard_false_fallthrough_skipped,
            ),
            expected_routes,
        );
        assert_no_selector_evidence(snapshot);
        let state =
            purchased_air_calc_cooling_post_saturation_capacity_limit_guard_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP380 skipped lifecycle")
            .state;
        assert_counter_shape(&state, 0, 0, 0, 0);
    }
}

fn assert_no_selector_evidence(
    snapshot: PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardSnapshot,
) {
    assert!(
        !snapshot.predecessor_local_supply_enthalpy_after_saturation_limit_assignment_performed
    );
    assert!(!snapshot.capacity_limit_guard_evaluated);
    assert!(!snapshot.configured_cooling_limit_owned_read);
    assert!(!snapshot.cp337_same_call_selector_lineage_corroborated);
    assert!(!snapshot.first_cooling_limit_read);
    assert!(snapshot.first_cooling_limit.is_none());
    assert!(!snapshot.cooling_limit_capacity_comparison_evaluated);
    assert!(snapshot.cooling_limit_capacity.is_none());
    assert!(!snapshot.second_cooling_limit_read);
    assert!(snapshot.second_cooling_limit.is_none());
    assert!(!snapshot.cooling_limit_flow_rate_and_capacity_comparison_evaluated);
    assert!(snapshot.cooling_limit_flow_rate_and_capacity.is_none());
    assert!(snapshot.cooling_limit_condition_satisfied.is_none());
    assert!(!snapshot.cooling_limit_rejected);
    assert!(!snapshot.capacity_limit_body_entered);
    assert!(!snapshot.active_guard_false_fallthrough);
}

fn assert_counter_shape(
    state: &PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardRuntimeState,
    active: usize,
    capacity_matches: usize,
    combined_matches: usize,
    source_sites: usize,
) {
    let second_comparisons = active - capacity_matches;
    let body_entries = capacity_matches + combined_matches;
    assert_eq!(state.capacity_limit_guard_evaluation_count, active);
    assert_eq!(state.source_site_execution_count, source_sites);
    assert_eq!(state.configured_cooling_limit_owned_read_count, active);
    assert_eq!(
        state.cp337_same_call_selector_lineage_corroboration_count,
        active
    );
    assert_eq!(state.first_cooling_limit_read_count, active);
    assert_eq!(state.cooling_limit_capacity_comparison_count, active);
    assert_eq!(state.cooling_limit_capacity_match_count, capacity_matches);
    assert_eq!(state.second_cooling_limit_read_count, second_comparisons);
    assert_eq!(
        state.cooling_limit_flow_rate_and_capacity_comparison_count,
        second_comparisons,
    );
    assert_eq!(
        state.cooling_limit_flow_rate_and_capacity_match_count,
        combined_matches,
    );
    assert_eq!(state.capacity_limit_body_entry_count, body_entries);
    assert_eq!(state.cooling_limit_rejected_count, active - body_entries);
    assert_eq!(
        state.active_guard_false_fallthrough_count,
        active - body_entries,
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE_ORDER.len(),
        5,
    );
}
