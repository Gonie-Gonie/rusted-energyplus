use super::*;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentRuntimeState,
    cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle_summary,
};

fn run_case(
    cooling_limit: IdealLoadsLimit,
    maximum_capacity_w: Option<f64>,
    independent_load_w: f64,
    availability: f64,
    humidity_ratio: f64,
) -> (
    PurchasedAirRuntimeState,
    DirectZonePurchasedAirScheduledCouplingOutput,
) {
    let (model, cache) = fixture(|typed| {
        let system = &mut typed.ideal_loads_air_systems[0];
        system.cooling_limit = cooling_limit;
        system.maximum_cooling_air_flow_rate_m3_per_s = None;
        system.maximum_total_cooling_capacity_w = maximum_capacity_w.map(AutosizeOrNumber::Value);
        schedule_mut(typed, ScheduleId(3)).hourly_value = availability;
    });
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut zone_state = zone_state_for_temp_independent_load(independent_load_w);
    zone_state.air_humidity_ratio = humidity_ratio;
    zone_state.zone_timestep_average_air_humidity_ratio = humidity_ratio;
    zone_state.previous_air_humidity_ratios = [humidity_ratio; 3];
    zone_state.previous_system_air_humidity_ratios = [humidity_ratio; 3];
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
    .expect("source-ordered CP345 coupling");
    (runtime, output)
}

#[test]
fn scheduled_binding_executes_cp345_after_every_cp344_active_route() {
    for (cooling_limit, maximum_capacity_w, expected_g, expected_f, expected_l) in [
        (IdealLoadsLimit::NoLimit, None, true, false, false),
        (
            IdealLoadsLimit::LimitCapacity,
            Some(1.0e9),
            false,
            true,
            false,
        ),
        (
            IdealLoadsLimit::LimitCapacity,
            Some(1.0),
            false,
            false,
            true,
        ),
    ] {
        let (runtime, output) = run_case(
            cooling_limit,
            maximum_capacity_w,
            3_000.0,
            1.0,
            -0.0,
        );
        let predecessor = output
            .calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit;
        let mixed_air = output.calculation_cooling_mixed_air_call;
        let assignment = output
            .calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment;
        let source = mixed_air
            .mixed_air_humidity_ratio
            .expect("CP329 mixed-air humidity ratio");

        assert_eq!(
            predecessor.capacity_limit_guard_false_fallthrough_skipped,
            expected_g
        );
        assert_eq!(
            predecessor.capacity_limit_sensible_output_guard_false_fallthrough,
            expected_f
        );
        assert_eq!(
            predecessor
                .capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed,
            expected_l
        );
        assert!(
            assignment.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed
        );
        assert!(
            cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release(
                assignment,
            )
        );
        assert_eq!(
            assignment.mixed_air_humidity_ratio.map(f64::to_bits),
            Some(source.to_bits())
        );
        assert_eq!(
            assignment
                .assigned_supply_humidity_ratio
                .map(f64::to_bits),
            Some(source.to_bits())
        );

        let state =
            purchased_air_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP345 lifecycle")
            .state;
        assert_counter_shape(&state, false, false, false, expected_g, expected_f, expected_l);
    }
}

#[test]
fn scheduled_binding_skips_cp345_only_on_u_n_and_p_routes() {
    for (
        cooling_limit,
        maximum_capacity_w,
        independent_load_w,
        availability,
        unit_off,
        non_cooling,
        positive_false,
    ) in [
        (
            IdealLoadsLimit::NoLimit,
            None,
            3_000.0,
            0.0,
            true,
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
        ),
        (
            IdealLoadsLimit::LimitCapacity,
            Some(-0.0),
            3_000.0,
            1.0,
            false,
            false,
            true,
        ),
    ] {
        let (runtime, output) = run_case(
            cooling_limit,
            maximum_capacity_w,
            independent_load_w,
            availability,
            0.008,
        );
        let assignment = output
            .calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment;

        assert_eq!(assignment.unit_off_skipped, unit_off);
        assert_eq!(assignment.non_cooling_skipped, non_cooling);
        assert_eq!(
            assignment.positive_guard_false_fallthrough_skipped,
            positive_false
        );
        assert!(
            !assignment
                .post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed
        );
        assert!(!assignment.mixed_air_humidity_ratio_read);
        assert!(assignment.mixed_air_humidity_ratio.is_none());
        assert!(!assignment.supply_humidity_ratio_assignment_performed);
        assert!(assignment.assigned_supply_humidity_ratio.is_none());

        let state =
            purchased_air_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP345 lifecycle")
            .state;
        assert_counter_shape(
            &state,
            unit_off,
            non_cooling,
            positive_false,
            false,
            false,
            false,
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn assert_counter_shape(
    state:
        &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentRuntimeState,
    unit_off: bool,
    non_cooling: bool,
    positive_false: bool,
    capacity_false: bool,
    sensible_false: bool,
    mixed_air_limit: bool,
) {
    let assignments =
        usize::from(capacity_false) + usize::from(sensible_false) + usize::from(mixed_air_limit);
    assert_eq!(state.transition_count, 1);
    assert_eq!(state.unit_off_skip_count, usize::from(unit_off));
    assert_eq!(state.non_cooling_skip_count, usize::from(non_cooling));
    assert_eq!(
        state.positive_guard_false_fallthrough_skip_count,
        usize::from(positive_false)
    );
    assert_eq!(
        state
            .assignment_after_capacity_limit_guard_false_fallthrough_count,
        usize::from(capacity_false)
    );
    assert_eq!(
        state
            .assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count,
        usize::from(sensible_false)
    );
    assert_eq!(
        state
            .assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count,
        usize::from(mixed_air_limit)
    );
    assert_eq!(
        state
            .post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count,
        assignments
    );
    assert_eq!(state.source_site_execution_count, 2 * assignments);
    assert_eq!(state.mixed_air_humidity_ratio_read_count, assignments);
    assert_eq!(state.supply_humidity_ratio_assignment_count, assignments);
}
