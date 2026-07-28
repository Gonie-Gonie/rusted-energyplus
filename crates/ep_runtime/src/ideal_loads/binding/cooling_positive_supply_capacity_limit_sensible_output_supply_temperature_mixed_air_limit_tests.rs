use super::*;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRuntimeState,
    cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle_summary,
};

fn run_case(
    cooling_limit: IdealLoadsLimit,
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
        system.maximum_cooling_air_flow_rate_m3_per_s = None;
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
    .expect("source-ordered CP344 coupling");
    (runtime, output)
}

#[test]
fn scheduled_binding_applies_capacity_limit_mixed_air_minimum_bit_exactly() {
    let (runtime, output) = run_case(
        IdealLoadsLimit::LimitCapacity,
        Some(1.0),
        3_000.0,
        1.0,
    );
    let predecessor = output
        .calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment;
    let mixed_air = output.calculation_cooling_mixed_air_call;
    let corroborating =
        output.calculation_cooling_positive_supply_temperature_mixed_air_limit;
    let limit = output
        .calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit;
    let left = predecessor
        .resulting_supply_temperature_c
        .expect("CP343 resulting supply temperature");
    let right = mixed_air
        .mixed_air_temperature_c
        .expect("CP329 mixed-air temperature");
    let expected = if left < right { left } else { right };

    assert!(limit.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed);
    assert!(
        cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release(
            limit,
        )
    );
    assert_eq!(
        limit.preexisting_supply_temperature_c.map(f64::to_bits),
        Some(left.to_bits())
    );
    assert_eq!(
        limit
            .supply_temperature_before_mixed_air_limit_c
            .map(f64::to_bits),
        Some(left.to_bits())
    );
    assert_eq!(
        limit.mixed_air_temperature_c.map(f64::to_bits),
        Some(right.to_bits())
    );
    assert_eq!(
        corroborating
            .mixed_air_temperature_c
            .map(f64::to_bits),
        Some(right.to_bits())
    );
    for value in [
        limit.minimum_supply_temperature_c,
        limit.assigned_supply_temperature_c,
        limit.resulting_supply_temperature_c,
    ] {
        assert_eq!(value.map(f64::to_bits), Some(expected.to_bits()));
    }

    let state =
        purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP344 lifecycle")
        .state;
    assert_counter_shape(&state, true, false, false, false, false, false);
}

#[test]
fn scheduled_binding_preserves_cp343_result_on_sensible_output_guard_fallthrough() {
    let (runtime, output) = run_case(
        IdealLoadsLimit::LimitCapacity,
        Some(1.0e9),
        3_000.0,
        1.0,
    );
    let predecessor = output
        .calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment;
    let limit = output
        .calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit;
    let preserved = predecessor
        .resulting_supply_temperature_c
        .expect("CP343 preserved supply temperature");

    assert!(limit.capacity_limit_sensible_output_guard_false_fallthrough);
    assert!(!limit.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed);
    assert_eq!(
        limit.preexisting_supply_temperature_c.map(f64::to_bits),
        Some(preserved.to_bits())
    );
    assert_eq!(
        limit.resulting_supply_temperature_c.map(f64::to_bits),
        Some(preserved.to_bits())
    );
    assert_skipped_rhs_is_null(limit);

    let state =
        purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP344 false-guard lifecycle")
        .state;
    assert_counter_shape(&state, false, false, false, false, false, true);
}

#[test]
fn scheduled_binding_preserves_all_inherited_null_routes() {
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
            maximum_capacity_w,
            independent_load_w,
            availability,
        );
        let snapshot = output
            .calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit;

        assert!(!snapshot.capacity_limit_sensible_output_guard_false_fallthrough);
        assert!(
            !snapshot
                .capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed
        );
        assert!(snapshot.preexisting_supply_temperature_c.is_none());
        assert!(snapshot.resulting_supply_temperature_c.is_none());
        assert_skipped_rhs_is_null(snapshot);

        let state =
            purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP344 inherited-skip lifecycle")
            .state;
        assert_counter_shape(
            &state,
            false,
            unit_off,
            non_cooling,
            positive_guard_false,
            capacity_guard_false,
            false,
        );
    }
}

fn assert_skipped_rhs_is_null(
    snapshot: crate::ideal_loads::
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
) {
    assert!(!snapshot.supply_temperature_for_minimum_read);
    assert!(
        snapshot
            .supply_temperature_before_mixed_air_limit_c
            .is_none()
    );
    assert!(!snapshot.mixed_air_temperature_for_minimum_read);
    assert!(snapshot.mixed_air_temperature_c.is_none());
    assert!(!snapshot.source_shaped_two_argument_minimum_evaluated);
    assert!(snapshot.minimum_supply_temperature_c.is_none());
    assert!(!snapshot.supply_temperature_assignment_performed);
    assert!(snapshot.assigned_supply_temperature_c.is_none());
}

#[allow(clippy::too_many_arguments)]
fn assert_counter_shape(
    state: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRuntimeState,
    executed: bool,
    unit_off: bool,
    non_cooling: bool,
    positive_guard_false: bool,
    capacity_guard_false: bool,
    sensible_guard_false: bool,
) {
    let executions = usize::from(executed);
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
        state.capacity_limit_sensible_output_guard_false_fallthrough_count,
        usize::from(sensible_guard_false)
    );
    assert_eq!(
        state.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count,
        executions
    );
    assert_eq!(state.source_site_execution_count, 4 * executions);
    assert_eq!(state.supply_temperature_for_minimum_read_count, executions);
    assert_eq!(
        state.mixed_air_temperature_for_minimum_read_count,
        executions
    );
    assert_eq!(
        state.source_shaped_two_argument_minimum_evaluation_count,
        executions
    );
    assert_eq!(state.supply_temperature_assignment_write_count, executions);
}
