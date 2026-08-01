use super::*;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardRuntimeState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardSnapshot,
    cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_lifecycle_summary,
};

fn run_case(
    cooling_limit: IdealLoadsLimit,
    air_humidity_ratio: f64,
    independent_load_w: f64,
    availability: f64,
    maximum_capacity_w: f64,
) -> (
    PurchasedAirRuntimeState,
    DirectZonePurchasedAirScheduledCouplingOutput,
) {
    let (model, cache) = fixture(|typed| {
        let system = &mut typed.ideal_loads_air_systems[0];
        system.cooling_limit = cooling_limit;
        system.maximum_cooling_air_flow_rate_m3_per_s = matches!(
            cooling_limit,
            IdealLoadsLimit::LimitFlowRate | IdealLoadsLimit::LimitFlowRateAndCapacity
        )
        .then_some(AutosizeOrNumber::Value(0.05));
        system.maximum_total_cooling_capacity_w = matches!(
            cooling_limit,
            IdealLoadsLimit::LimitCapacity | IdealLoadsLimit::LimitFlowRateAndCapacity
        )
        .then_some(AutosizeOrNumber::Value(maximum_capacity_w));
        system.dehumidification_control_type = DehumidificationControlType::None;
        system.humidification_control_type = HumidificationControlType::None;
        system.minimum_cooling_supply_air_humidity_ratio = f64::NAN;
        schedule_mut(typed, ScheduleId(3)).hourly_value = availability;
    });
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut zone_state = zone_state_for_temp_independent_load(independent_load_w);
    zone_state.air_humidity_ratio = air_humidity_ratio;
    zone_state.zone_timestep_average_air_humidity_ratio = air_humidity_ratio;
    zone_state.previous_air_humidity_ratios = [air_humidity_ratio; 3];
    zone_state.previous_system_air_humidity_ratios = [air_humidity_ratio; 3];
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
    .expect("source-ordered CP383 coupling");
    (runtime, output)
}

#[test]
fn binding_places_cp383_after_cp382_and_uses_exact_same_call_capacity_owners() {
    for maximum_capacity_w in [500.0, 5_000.0, 1.0e9] {
        let (runtime, output) = run_case(
            IdealLoadsLimit::LimitCapacity,
            0.020,
            3_000.0,
            1.0,
            maximum_capacity_w,
        );
        let predecessor = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment;
        let capacity_owner = output.calculation_cooling_capacity_zero_flow_reset;
        let capacity_corroborator =
            output.calculation_cooling_positive_supply_capacity_limit_sensible_output_guard;
        let snapshot = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard;

        assert!(predecessor.dehumidification_total_output_assignment_executed);
        assert!(
            cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_snapshot_is_exact_direct_release(
                snapshot,
            )
        );
        assert!(snapshot.dehumidification_total_output_capacity_guard_evaluated);
        assert!(snapshot.cp382_cooling_total_output_owned_read);
        assert!(snapshot.cp321_maximum_total_cooling_capacity_owned_read);
        assert!(snapshot.cp340_same_call_maximum_total_cooling_capacity_bit_corroborated);
        assert_eq!(
            snapshot.cooling_total_output_w.map(f64::to_bits),
            predecessor.cooling_total_output_w.map(f64::to_bits),
        );
        assert_eq!(
            snapshot.maximum_total_cooling_capacity_w.map(f64::to_bits),
            capacity_owner
                .maximum_total_cooling_capacity_w
                .map(f64::to_bits),
        );
        assert_eq!(
            snapshot.maximum_total_cooling_capacity_w.map(f64::to_bits),
            capacity_corroborator
                .maximum_total_cooling_capacity_w
                .map(f64::to_bits),
        );
        let expected = predecessor.cooling_total_output_w.unwrap()
            > capacity_owner.maximum_total_cooling_capacity_w.unwrap();
        assert_eq!(
            snapshot.cooling_total_output_strictly_greater_than_maximum_total_cooling_capacity,
            Some(expected),
        );
        assert_eq!(
            snapshot.dehumidification_total_output_capacity_adjustment_body_entered,
            expected,
        );
        assert_eq!(
            snapshot.dehumidification_total_output_capacity_guard_false_fallthrough,
            !expected,
        );

        let state =
            purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP383 lifecycle")
            .state;
        assert_counter_shape(&state, true, expected);
    }
}

#[test]
fn binding_keeps_cp383_complete_null_for_cp382_false_and_outer_false_routes() {
    for (limit, humidity, load, availability, maximum_capacity_w) in [
        (IdealLoadsLimit::LimitCapacity, 0.008, 3_000.0, 1.0, 5_000.0),
        (IdealLoadsLimit::NoLimit, 0.020, 3_000.0, 1.0, 5_000.0),
        (IdealLoadsLimit::NoLimit, 0.020, 3_000.0, 0.0, 5_000.0),
        (IdealLoadsLimit::NoLimit, 0.020, 0.0, 1.0, 5_000.0),
        (IdealLoadsLimit::LimitCapacity, 0.020, 3_000.0, 1.0, 0.0),
    ] {
        let (runtime, output) = run_case(limit, humidity, load, availability, maximum_capacity_w);
        let predecessor = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment;
        let snapshot = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard;
        assert!(!predecessor.dehumidification_total_output_assignment_executed);
        assert_complete_null(snapshot);
        let state =
            purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP383 skipped lifecycle")
            .state;
        assert_counter_shape(&state, false, false);
    }
}

fn assert_complete_null(
    snapshot: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardSnapshot,
) {
    assert!(!snapshot.dehumidification_total_output_capacity_guard_evaluated);
    assert!(!snapshot.cooling_total_output_read);
    assert!(snapshot.cooling_total_output_w.is_none());
    assert!(!snapshot.maximum_total_cooling_capacity_read);
    assert!(snapshot.maximum_total_cooling_capacity_w.is_none());
    assert!(!snapshot.cooling_total_output_maximum_total_cooling_capacity_comparison_evaluated);
    assert!(
        snapshot
            .cooling_total_output_strictly_greater_than_maximum_total_cooling_capacity
            .is_none()
    );
    assert!(!snapshot.dehumidification_total_output_capacity_adjustment_body_entered);
    assert!(!snapshot.dehumidification_total_output_capacity_guard_false_fallthrough);
}

fn assert_counter_shape(
    state: &PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardRuntimeState,
    evaluated: bool,
    body_entered: bool,
) {
    let evaluated = usize::from(evaluated);
    let body_entered = usize::from(body_entered);
    assert_eq!(state.transition_count, 1);
    assert_eq!(
        state.dehumidification_total_output_capacity_guard_evaluation_count,
        evaluated
    );
    assert_eq!(
        state.source_site_execution_count,
        3 * evaluated + body_entered
    );
    assert_eq!(state.cooling_total_output_read_count, evaluated);
    assert_eq!(state.maximum_total_cooling_capacity_read_count, evaluated);
    assert_eq!(
        state.cooling_total_output_maximum_total_cooling_capacity_comparison_count,
        evaluated
    );
    assert_eq!(
        state.dehumidification_total_output_capacity_adjustment_body_entry_count,
        body_entered
    );
    assert_eq!(
        state.dehumidification_total_output_capacity_guard_false_fallthrough_count,
        evaluated - body_entered
    );
}
