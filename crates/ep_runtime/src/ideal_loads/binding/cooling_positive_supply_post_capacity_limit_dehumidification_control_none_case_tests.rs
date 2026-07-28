use super::*;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseRuntimeState,
    cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_lifecycle_summary,
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
    .expect("source-ordered CP347 coupling");
    (runtime, output)
}

#[test]
fn scheduled_binding_completes_cp347_none_case_after_every_cp346_active_route() {
    for (cooling_limit, maximum_capacity_w) in [
        (IdealLoadsLimit::NoLimit, None),
        (IdealLoadsLimit::LimitCapacity, Some(1.0e9)),
        (IdealLoadsLimit::LimitCapacity, Some(1.0)),
    ] {
        let (runtime, output) = run_case(cooling_limit, maximum_capacity_w, 3_000.0, 1.0);
        let mixed_air_owner = output.calculation_cooling_mixed_air_call;
        let predecessor = output
            .calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch;
        let snapshot = output
            .calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case;

        assert!(predecessor.dehumidification_control_switch_dispatched);
        assert_eq!(
            predecessor.dehumidification_control_type,
            Some(DehumidificationControlType::None)
        );
        assert!(
            cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_snapshot_is_exact_direct_release(
                snapshot,
            )
        );
        assert!(snapshot.dehumidification_control_none_case_entered);
        assert!(snapshot.mixed_air_humidity_ratio_read);
        assert!(snapshot.supply_humidity_ratio_assignment_performed);
        assert!(snapshot.dehumidification_control_none_case_exited_via_break);
        let owner_bits = mixed_air_owner.mixed_air_humidity_ratio.map(f64::to_bits);
        assert_eq!(
            predecessor
                .predecessor_assigned_supply_humidity_ratio
                .map(f64::to_bits),
            owner_bits
        );
        assert_eq!(
            snapshot
                .predecessor_assigned_supply_humidity_ratio
                .map(f64::to_bits),
            owner_bits
        );
        for value in [
            snapshot.mixed_air_humidity_ratio,
            snapshot.assigned_supply_humidity_ratio,
            snapshot.resulting_supply_humidity_ratio,
        ] {
            assert_eq!(value.map(f64::to_bits), owner_bits);
        }

        let state =
            purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP347 lifecycle")
            .state;
        assert_counter_shape(&state, false, false, false, true);
    }
}

#[test]
fn scheduled_binding_skips_cp347_source_sites_only_on_u_n_and_p_routes() {
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
        (IdealLoadsLimit::NoLimit, None, 0.0, 1.0, false, true, false),
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
        );
        let snapshot = output
            .calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case;

        assert_eq!(snapshot.unit_off_skipped, unit_off);
        assert_eq!(snapshot.non_cooling_skipped, non_cooling);
        assert_eq!(
            snapshot.positive_guard_false_fallthrough_skipped,
            positive_false
        );
        assert!(!snapshot.dehumidification_control_none_case_entered);
        assert!(!snapshot.mixed_air_humidity_ratio_read);
        assert!(!snapshot.supply_humidity_ratio_assignment_performed);
        assert!(!snapshot.dehumidification_control_none_case_exited_via_break);
        for value in [
            snapshot.predecessor_assigned_supply_humidity_ratio,
            snapshot.mixed_air_humidity_ratio,
            snapshot.assigned_supply_humidity_ratio,
            snapshot.resulting_supply_humidity_ratio,
        ] {
            assert!(value.is_none());
        }

        let state =
            purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP347 lifecycle")
            .state;
        assert_counter_shape(&state, unit_off, non_cooling, positive_false, false);
    }
}

fn assert_counter_shape(
    state:
        &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseRuntimeState,
    unit_off: bool,
    non_cooling: bool,
    positive_false: bool,
    completed: bool,
) {
    let completed = usize::from(completed);
    assert_eq!(state.transition_count, 1);
    assert_eq!(state.unit_off_skip_count, usize::from(unit_off));
    assert_eq!(state.non_cooling_skip_count, usize::from(non_cooling));
    assert_eq!(
        state.positive_guard_false_fallthrough_skip_count,
        usize::from(positive_false)
    );
    assert_eq!(
        state.dehumidification_control_none_case_completion_count,
        completed
    );
    assert_eq!(state.source_site_execution_count, 4 * completed);
    for count in [
        state.dehumidification_control_none_case_entry_count,
        state.mixed_air_humidity_ratio_read_count,
        state.supply_humidity_ratio_assignment_count,
        state.dehumidification_control_none_case_break_count,
    ] {
        assert_eq!(count, completed);
    }
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
