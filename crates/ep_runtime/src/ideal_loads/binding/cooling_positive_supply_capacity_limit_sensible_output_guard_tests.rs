use super::*;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
    cooling_positive_supply_capacity_limit_sensible_output_guard_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle_summary,
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
    .expect("source-ordered CP340 coupling");
    (runtime, output)
}

#[test]
fn scheduled_binding_preserves_exact_operands_and_both_comparison_routes() {
    for (cooling_limit, maximum_flow_m3_per_s) in [
        (IdealLoadsLimit::LimitCapacity, None),
        (IdealLoadsLimit::LimitFlowRateAndCapacity, Some(0.05)),
    ] {
        for (maximum_capacity_w, expected_comparison) in [(1.0, true), (1.0e9, false)] {
            let (runtime, output) = run_case(
                cooling_limit,
                maximum_flow_m3_per_s,
                Some(maximum_capacity_w),
                3_000.0,
                1.0,
            );
            let predecessor = output
                .calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment;
            let retained_capacity = output.calculation_cooling_capacity_zero_flow_reset;
            let guard =
                output.calculation_cooling_positive_supply_capacity_limit_sensible_output_guard;

            assert!(predecessor.capacity_limit_sensible_output_assignment_executed);
            assert!(
                cooling_positive_supply_capacity_limit_sensible_output_guard_snapshot_is_exact_direct_release(
                    guard,
                )
            );
            assert!(guard.capacity_limit_sensible_output_guard_evaluated);
            assert!(guard.cooling_sensible_output_read);
            assert!(guard.maximum_total_cooling_capacity_read);
            assert!(
                guard.cooling_sensible_output_maximum_capacity_comparison_evaluated
            );
            assert_eq!(
                guard.cooling_sensible_output_w.map(f64::to_bits),
                predecessor.cooling_sensible_output_w.map(f64::to_bits)
            );
            assert_eq!(
                guard.maximum_total_cooling_capacity_w.map(f64::to_bits),
                retained_capacity
                    .maximum_total_cooling_capacity_w
                    .map(f64::to_bits)
            );
            assert_eq!(
                guard.cooling_sensible_output_at_or_above_maximum_capacity,
                Some(expected_comparison)
            );
            assert_eq!(
                guard.capacity_limit_sensible_output_guard_false_fallthrough,
                !expected_comparison
            );
            assert_eq!(
                guard.capacity_limit_sensible_output_adjustment_body_entered,
                expected_comparison
            );

            let state =
                purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle_summary(
                    &runtime,
                    output.initialization.system,
                )
                .expect("CP340 lifecycle")
                .state;
            assert_counter_shape(
                &state,
                false,
                false,
                false,
                false,
                !expected_comparison,
                expected_comparison,
            );
        }
    }
}

#[test]
fn scheduled_binding_preserves_all_complete_null_skip_routes() {
    for (
        cooling_limit,
        maximum_capacity_w,
        independent_load_w,
        availability,
        unit_off,
        non_cooling,
        positive_guard_false,
        capacity_guard_false,
    ) in [
        (
            IdealLoadsLimit::NoLimit,
            None,
            3_000.0,
            0.0,
            true,
            false,
            false,
            false,
        ),
        (
            IdealLoadsLimit::NoLimit,
            None,
            0.0,
            1.0,
            false,
            true,
            false,
            false,
        ),
        (
            IdealLoadsLimit::LimitCapacity,
            Some(-0.0),
            3_000.0,
            1.0,
            false,
            false,
            true,
            false,
        ),
        (
            IdealLoadsLimit::NoLimit,
            None,
            3_000.0,
            1.0,
            false,
            false,
            false,
            true,
        ),
    ] {
        let (runtime, output) = run_case(
            cooling_limit,
            None,
            maximum_capacity_w,
            independent_load_w,
            availability,
        );
        let guard =
            output.calculation_cooling_positive_supply_capacity_limit_sensible_output_guard;

        assert_eq!(guard.unit_off_skipped, unit_off);
        assert_eq!(guard.non_cooling_skipped, non_cooling);
        assert_eq!(
            guard.positive_guard_false_fallthrough_skipped,
            positive_guard_false
        );
        assert_eq!(
            guard.capacity_limit_guard_false_fallthrough_skipped,
            capacity_guard_false
        );
        assert_complete_null(guard);

        let state =
            purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP340 skip lifecycle")
            .state;
        assert_counter_shape(
            &state,
            unit_off,
            non_cooling,
            positive_guard_false,
            capacity_guard_false,
            false,
            false,
        );
    }
}

fn assert_complete_null(
    snapshot: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
) {
    assert!(!snapshot.capacity_limit_sensible_output_guard_evaluated);
    assert!(!snapshot.cooling_sensible_output_read);
    assert!(snapshot.cooling_sensible_output_w.is_none());
    assert!(!snapshot.maximum_total_cooling_capacity_read);
    assert!(snapshot.maximum_total_cooling_capacity_w.is_none());
    assert!(
        !snapshot.cooling_sensible_output_maximum_capacity_comparison_evaluated
    );
    assert!(
        snapshot
            .cooling_sensible_output_at_or_above_maximum_capacity
            .is_none()
    );
    assert!(!snapshot.capacity_limit_sensible_output_guard_false_fallthrough);
    assert!(!snapshot.capacity_limit_sensible_output_adjustment_body_entered);
}

#[allow(clippy::too_many_arguments)]
fn assert_counter_shape(
    state: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardRuntimeState,
    unit_off: bool,
    non_cooling: bool,
    positive_guard_false: bool,
    capacity_guard_false: bool,
    comparison_false: bool,
    body_entered: bool,
) {
    let evaluated = usize::from(comparison_false || body_entered);
    let body_entered = usize::from(body_entered);
    assert_eq!(state.transition_count, 1);
    assert_eq!(state.unit_off_skip_count, usize::from(unit_off));
    assert_eq!(state.non_cooling_skip_count, usize::from(non_cooling));
    assert_eq!(
        state.positive_guard_false_fallthrough_skip_count,
        usize::from(positive_guard_false)
    );
    assert_eq!(
        state.capacity_limit_guard_false_fallthrough_skip_count,
        usize::from(capacity_guard_false)
    );
    assert_eq!(
        state.capacity_limit_sensible_output_guard_evaluation_count,
        evaluated
    );
    assert_eq!(state.source_site_execution_count, 3 * evaluated + body_entered);
    assert_eq!(state.cooling_sensible_output_read_count, evaluated);
    assert_eq!(
        state.maximum_total_cooling_capacity_read_count,
        evaluated
    );
    assert_eq!(
        state.cooling_sensible_output_maximum_capacity_comparison_count,
        evaluated
    );
    assert_eq!(
        state.capacity_limit_sensible_output_guard_false_fallthrough_count,
        usize::from(comparison_false)
    );
    assert_eq!(
        state.capacity_limit_sensible_output_adjustment_body_entry_count,
        body_entered
    );
}
