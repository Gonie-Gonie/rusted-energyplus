use super::*;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitRuntimeState,
    cooling_positive_supply_temperature_minimum_limit_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_positive_supply_temperature_minimum_limit_lifecycle_summary,
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
    f64,
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
    let minimum_supply_temperature_c = binding.system.minimum_cooling_supply_air_temperature_c;
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
    .expect("source-ordered CP333 coupling");
    (runtime, output, minimum_supply_temperature_c)
}

#[test]
fn scheduled_binding_applies_source_shaped_minimum_temperature_limit_bit_exactly() {
    let (runtime, output, minimum_supply_temperature_c) = run_case(
        IdealLoadsLimit::LimitFlowRate,
        Some(0.05),
        None,
        3_000.0,
        1.0,
    );
    let predecessor = output.calculation_cooling_positive_supply_temperature_assignment;
    let limit = output.calculation_cooling_positive_supply_temperature_minimum_limit;
    let supply_before = predecessor
        .supply_temperature_c
        .expect("CP332 supply temperature");
    let expected = if supply_before < minimum_supply_temperature_c {
        minimum_supply_temperature_c
    } else {
        supply_before
    };

    assert!(limit.supply_temperature_minimum_limit_executed);
    assert!(
        cooling_positive_supply_temperature_minimum_limit_snapshot_is_exact_direct_release(limit)
    );
    assert_eq!(
        limit
            .supply_temperature_before_minimum_limit_c
            .map(f64::to_bits),
        Some(supply_before.to_bits())
    );
    assert_eq!(
        limit
            .minimum_cooling_supply_air_temperature_c
            .map(f64::to_bits),
        Some(minimum_supply_temperature_c.to_bits())
    );
    assert_eq!(
        limit.maximum_supply_temperature_c.map(f64::to_bits),
        Some(expected.to_bits())
    );
    assert_eq!(
        limit.assigned_supply_temperature_c.map(f64::to_bits),
        Some(expected.to_bits())
    );
    assert_eq!(
        limit
            .minimum_cooling_supply_air_temperature_c
            .map(f64::to_bits),
        output
            .calculation_cooling_sensible_flow
            .minimum_cooling_supply_air_temperature_c
            .map(f64::to_bits)
    );

    let state =
        purchased_air_calc_cooling_positive_supply_temperature_minimum_limit_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP333 lifecycle")
        .state;
    assert_counter_shape(&state, true, false, false, false);
}

#[test]
fn scheduled_binding_skips_cp333_after_the_positive_guard_falls_through() {
    let (runtime, output, _) = run_case(
        IdealLoadsLimit::LimitCapacity,
        None,
        Some(0.0),
        3_000.0,
        1.0,
    );
    let limit = output.calculation_cooling_positive_supply_temperature_minimum_limit;

    assert!(limit.positive_guard_false_fallthrough_skipped);
    assert!(!limit.supply_temperature_minimum_limit_executed);
    assert_snapshot_has_no_source_values(limit);
    let state =
        purchased_air_calc_cooling_positive_supply_temperature_minimum_limit_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP333 guard-false lifecycle")
        .state;
    assert_counter_shape(&state, false, false, false, true);
}

#[test]
fn scheduled_binding_preserves_unit_off_and_non_cooling_cp333_skip_routes() {
    for (availability, independent_load_w, unit_off, non_cooling) in
        [(0.0, 3_000.0, true, false), (1.0, 0.0, false, true)]
    {
        let (runtime, output, _) = run_case(
            IdealLoadsLimit::NoLimit,
            None,
            None,
            independent_load_w,
            availability,
        );
        let limit = output.calculation_cooling_positive_supply_temperature_minimum_limit;

        assert_eq!(limit.unit_off_skipped, unit_off);
        assert_eq!(limit.non_cooling_skipped, non_cooling);
        assert!(!limit.supply_temperature_minimum_limit_executed);
        assert_snapshot_has_no_source_values(limit);
        let state =
            purchased_air_calc_cooling_positive_supply_temperature_minimum_limit_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP333 skipped lifecycle")
            .state;
        assert_counter_shape(&state, false, unit_off, non_cooling, false);
    }
}

fn assert_snapshot_has_no_source_values(
    snapshot: crate::ideal_loads::
        PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
) {
    assert!(!snapshot.supply_temperature_for_maximum_read);
    assert!(snapshot.supply_temperature_before_minimum_limit_c.is_none());
    assert!(!snapshot.minimum_cooling_supply_air_temperature_for_maximum_read);
    assert!(snapshot.minimum_cooling_supply_air_temperature_c.is_none());
    assert!(!snapshot.source_shaped_two_argument_maximum_evaluated);
    assert!(snapshot.maximum_supply_temperature_c.is_none());
    assert!(!snapshot.supply_temperature_assignment_performed);
    assert!(snapshot.assigned_supply_temperature_c.is_none());
}

fn assert_counter_shape(
    state: &PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitRuntimeState,
    executed: bool,
    unit_off: bool,
    non_cooling: bool,
    guard_false: bool,
) {
    let executions = usize::from(executed);
    assert_eq!(state.transition_count, 1);
    assert_eq!(state.unit_off_skip_count, usize::from(unit_off));
    assert_eq!(state.non_cooling_skip_count, usize::from(non_cooling));
    assert_eq!(
        state.positive_guard_false_fallthrough_skip_count,
        usize::from(guard_false)
    );
    assert_eq!(state.supply_temperature_minimum_limit_count, executions);
    assert_eq!(state.source_site_execution_count, 4 * executions);
    assert_eq!(state.supply_temperature_for_maximum_read_count, executions);
    assert_eq!(
        state.minimum_cooling_supply_air_temperature_for_maximum_read_count,
        executions
    );
    assert_eq!(
        state.source_shaped_two_argument_maximum_evaluation_count,
        executions
    );
    assert_eq!(state.supply_temperature_assignment_count, executions);
}
