use super::*;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentRuntimeState,
    cooling_positive_supply_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle_summary,
};

fn run_case(
    cooling_limit: IdealLoadsLimit,
    maximum_flow_m3_per_s: Option<f64>,
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
        system.maximum_cooling_air_flow_rate_m3_per_s =
            maximum_flow_m3_per_s.map(AutosizeOrNumber::Value);
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
    .expect("source-ordered CP335 coupling");
    (runtime, output)
}

#[test]
fn scheduled_binding_assigns_the_retained_mixed_air_humidity_ratio_bit_exactly() {
    let (runtime, output) = run_case(
        IdealLoadsLimit::LimitFlowRate,
        Some(0.05),
        None,
        3_000.0,
        1.0,
        -0.0,
    );
    let mixed_air = output.calculation_cooling_mixed_air_call;
    let assignment =
        output.calculation_cooling_positive_supply_humidity_ratio_mixed_air_assignment;
    let source = mixed_air
        .mixed_air_humidity_ratio
        .expect("CP329 mixed-air humidity ratio");

    assert!(assignment.supply_humidity_ratio_mixed_air_assignment_executed);
    assert!(
        cooling_positive_supply_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release(
            assignment,
        )
    );
    assert_eq!(source.to_bits(), (-0.0_f64).to_bits());
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
        purchased_air_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP335 lifecycle")
        .state;
    assert_counter_shape(&state, true, false, false, false);
}

#[test]
fn scheduled_binding_skips_cp335_after_the_positive_guard_falls_through() {
    let (runtime, output) = run_case(
        IdealLoadsLimit::LimitCapacity,
        None,
        Some(0.0),
        3_000.0,
        1.0,
        0.008,
    );
    let assignment =
        output.calculation_cooling_positive_supply_humidity_ratio_mixed_air_assignment;

    assert!(assignment.positive_guard_false_fallthrough_skipped);
    assert!(!assignment.supply_humidity_ratio_mixed_air_assignment_executed);
    assert_snapshot_has_no_source_values(assignment);
    let state =
        purchased_air_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP335 guard-false lifecycle")
        .state;
    assert_counter_shape(&state, false, false, false, true);
}

#[test]
fn scheduled_binding_preserves_unit_off_and_non_cooling_cp335_skip_routes() {
    for (availability, independent_load_w, unit_off, non_cooling) in
        [(0.0, 3_000.0, true, false), (1.0, 0.0, false, true)]
    {
        let (runtime, output) = run_case(
            IdealLoadsLimit::NoLimit,
            None,
            None,
            independent_load_w,
            availability,
            0.008,
        );
        let assignment =
            output.calculation_cooling_positive_supply_humidity_ratio_mixed_air_assignment;

        assert_eq!(assignment.unit_off_skipped, unit_off);
        assert_eq!(assignment.non_cooling_skipped, non_cooling);
        assert!(!assignment.supply_humidity_ratio_mixed_air_assignment_executed);
        assert_snapshot_has_no_source_values(assignment);
        let state =
            purchased_air_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP335 skipped lifecycle")
            .state;
        assert_counter_shape(&state, false, unit_off, non_cooling, false);
    }
}

fn assert_snapshot_has_no_source_values(
    snapshot: crate::ideal_loads::
        PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
) {
    assert!(!snapshot.mixed_air_humidity_ratio_read);
    assert!(snapshot.mixed_air_humidity_ratio.is_none());
    assert!(!snapshot.supply_humidity_ratio_assignment_performed);
    assert!(snapshot.assigned_supply_humidity_ratio.is_none());
}

fn assert_counter_shape(
    state: &PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentRuntimeState,
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
    assert_eq!(
        state.supply_humidity_ratio_mixed_air_assignment_count,
        executions
    );
    assert_eq!(state.source_site_execution_count, 2 * executions);
    assert_eq!(state.mixed_air_humidity_ratio_read_count, executions);
    assert_eq!(state.supply_humidity_ratio_assignment_count, executions);
}
