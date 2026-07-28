use super::*;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchRuntimeState,
    cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_lifecycle_summary,
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
        system.dehumidification_control_type = DehumidificationControlType::None;
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
    .expect("source-ordered CP346 coupling");
    (runtime, output)
}

#[test]
fn scheduled_binding_dispatches_cp346_none_case_after_every_cp345_active_route() {
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
        let (runtime, output) = run_case(cooling_limit, maximum_capacity_w, 3_000.0, 1.0);
        let predecessor = output
            .calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment;
        let dispatch = output
            .calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch;

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
            predecessor
                .post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed
        );
        assert!(
            cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_snapshot_is_exact_direct_release(
                dispatch,
            )
        );
        assert!(
            dispatch
                .predecessor_post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed
        );
        assert_eq!(
            dispatch
                .predecessor_assigned_supply_humidity_ratio
                .map(f64::to_bits),
            predecessor.assigned_supply_humidity_ratio.map(f64::to_bits)
        );
        assert!(dispatch.dehumidification_control_type_read);
        assert_eq!(
            dispatch.dehumidification_control_type,
            Some(DehumidificationControlType::None)
        );
        assert!(dispatch.dehumidification_control_switch_dispatched);

        let state =
            purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP346 lifecycle")
            .state;
        assert_counter_shape(
            &state, false, false, false, expected_g, expected_f, expected_l,
        );
    }
}

#[test]
fn scheduled_binding_skips_cp346_selector_only_on_u_n_and_p_routes() {
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
        let (runtime, output) =
            run_case(cooling_limit, maximum_capacity_w, independent_load_w, availability);
        let dispatch = output
            .calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch;

        assert_eq!(dispatch.unit_off_skipped, unit_off);
        assert_eq!(dispatch.non_cooling_skipped, non_cooling);
        assert_eq!(
            dispatch.positive_guard_false_fallthrough_skipped,
            positive_false
        );
        assert!(
            !dispatch
                .predecessor_post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed
        );
        assert!(dispatch.predecessor_assigned_supply_humidity_ratio.is_none());
        assert!(!dispatch.dehumidification_control_type_read);
        assert!(dispatch.dehumidification_control_type.is_none());
        assert!(!dispatch.dehumidification_control_switch_dispatched);

        let state =
            purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP346 lifecycle")
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
        &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchRuntimeState,
    unit_off: bool,
    non_cooling: bool,
    positive_false: bool,
    capacity_false: bool,
    sensible_false: bool,
    mixed_air_limit: bool,
) {
    let dispatches =
        usize::from(capacity_false) + usize::from(sensible_false) + usize::from(mixed_air_limit);
    assert_eq!(state.transition_count, 1);
    assert_eq!(state.unit_off_skip_count, usize::from(unit_off));
    assert_eq!(state.non_cooling_skip_count, usize::from(non_cooling));
    assert_eq!(
        state.positive_guard_false_fallthrough_skip_count,
        usize::from(positive_false)
    );
    assert_eq!(state.dehumidification_control_switch_count, dispatches);
    assert_eq!(state.source_site_execution_count, 2 * dispatches);
    assert_eq!(
        state.dehumidification_control_type_read_count,
        dispatches
    );
    assert_eq!(
        state.dehumidification_control_switch_dispatch_count,
        dispatches
    );
    assert_eq!(
        state.dehumidification_control_none_case_selection_count,
        dispatches
    );
    assert_eq!(
        state.dehumidification_control_constant_sensible_heat_ratio_case_selection_count,
        0
    );
    assert_eq!(
        state.dehumidification_control_humidistat_case_selection_count,
        0
    );
    assert_eq!(
        state.dehumidification_control_constant_supply_humidity_ratio_case_selection_count,
        0
    );
}
