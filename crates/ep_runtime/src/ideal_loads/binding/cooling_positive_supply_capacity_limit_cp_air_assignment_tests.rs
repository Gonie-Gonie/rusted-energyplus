use super::*;
use crate::{
    ideal_loads::{
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentRuntimeState,
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
        cooling_positive_supply_capacity_limit_cp_air_assignment_snapshot_is_exact_direct_release,
        purchased_air_calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle_summary,
    },
    psychrometrics::energyplus_psy_cp_air_fn_w,
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
    .expect("source-ordered CP338 coupling");
    (runtime, output)
}

#[test]
fn scheduled_binding_assigns_cp_air_for_both_capacity_limit_selectors() {
    let source_humidity_ratio = 0.008_765_432_109_876_543_f64;
    for (cooling_limit, maximum_flow_m3_per_s) in [
        (IdealLoadsLimit::LimitCapacity, None),
        (IdealLoadsLimit::LimitFlowRateAndCapacity, Some(0.05)),
    ] {
        let (runtime, output) = run_case(
            cooling_limit,
            maximum_flow_m3_per_s,
            Some(5_000.0),
            3_000.0,
            1.0,
            source_humidity_ratio,
        );
        let predecessor = output.calculation_cooling_positive_supply_capacity_limit_guard;
        let mixed_air = output.calculation_cooling_mixed_air_call;
        let assignment =
            output.calculation_cooling_positive_supply_capacity_limit_cp_air_assignment;

        assert!(predecessor.capacity_limit_body_entered);
        assert!(
            cooling_positive_supply_capacity_limit_cp_air_assignment_snapshot_is_exact_direct_release(
                assignment,
            )
        );
        assert!(assignment.predecessor_capacity_limit_guard_evaluated);
        assert!(assignment.predecessor_capacity_limit_body_entered);
        assert!(
            !assignment.predecessor_active_capacity_limit_guard_false_fallthrough
        );
        assert!(assignment.capacity_limit_cp_air_assignment_executed);
        assert!(assignment.mixed_air_humidity_ratio_read);
        assert_eq!(
            assignment.mixed_air_humidity_ratio.map(f64::to_bits),
            mixed_air.mixed_air_humidity_ratio.map(f64::to_bits)
        );
        let expected_cp_air = energyplus_psy_cp_air_fn_w(
            mixed_air
                .mixed_air_humidity_ratio
                .expect("same-call CP329 mixed-air humidity ratio"),
        );
        assert_eq!(
            assignment
                .psychrometric_cp_air_result_j_per_kg_k
                .map(f64::to_bits),
            Some(expected_cp_air.to_bits())
        );
        assert_eq!(
            assignment.cp_air_j_per_kg_k.map(f64::to_bits),
            Some(expected_cp_air.to_bits())
        );

        let state =
            purchased_air_calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP338 lifecycle")
            .state;
        assert_counter_shape(&state, false, false, false, false, true);
    }
}

#[test]
fn scheduled_binding_preserves_capacity_guard_false_fallthroughs_as_complete_null() {
    for (cooling_limit, maximum_flow_m3_per_s) in [
        (IdealLoadsLimit::NoLimit, None),
        (IdealLoadsLimit::LimitFlowRate, Some(0.05)),
    ] {
        let (runtime, output) = run_case(
            cooling_limit,
            maximum_flow_m3_per_s,
            None,
            3_000.0,
            1.0,
            0.008,
        );
        let predecessor = output.calculation_cooling_positive_supply_capacity_limit_guard;
        let assignment =
            output.calculation_cooling_positive_supply_capacity_limit_cp_air_assignment;

        assert!(predecessor.active_guard_false_fallthrough);
        assert!(assignment.capacity_limit_guard_false_fallthrough_skipped);
        assert_complete_null(assignment);
        let state =
            purchased_air_calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP338 capacity-guard-false lifecycle")
            .state;
        assert_counter_shape(&state, false, false, false, true, false);
    }
}

#[test]
fn scheduled_binding_preserves_inherited_complete_null_skip_routes() {
    for (
        cooling_limit,
        maximum_capacity_w,
        independent_load_w,
        availability,
        unit_off,
        non_cooling,
        positive_guard_false,
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
            Some(0.0),
            3_000.0,
            1.0,
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
            0.008,
        );
        let assignment =
            output.calculation_cooling_positive_supply_capacity_limit_cp_air_assignment;

        assert_eq!(assignment.unit_off_skipped, unit_off);
        assert_eq!(assignment.non_cooling_skipped, non_cooling);
        assert_eq!(
            assignment.positive_guard_false_fallthrough_skipped,
            positive_guard_false
        );
        assert!(!assignment.capacity_limit_guard_false_fallthrough_skipped);
        assert_complete_null(assignment);
        let state =
            purchased_air_calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP338 inherited skip lifecycle")
            .state;
        assert_counter_shape(
            &state,
            unit_off,
            non_cooling,
            positive_guard_false,
            false,
            false,
        );
    }
}

fn assert_complete_null(
    snapshot: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
) {
    assert!(!snapshot.capacity_limit_cp_air_assignment_executed);
    assert!(!snapshot.mixed_air_humidity_ratio_read);
    assert!(snapshot.mixed_air_humidity_ratio.is_none());
    assert!(!snapshot.psychrometric_cp_air_evaluated);
    assert!(snapshot.psychrometric_cp_air_result_j_per_kg_k.is_none());
    assert!(!snapshot.cp_air_assigned);
    assert!(snapshot.cp_air_j_per_kg_k.is_none());
}

fn assert_counter_shape(
    state: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentRuntimeState,
    unit_off: bool,
    non_cooling: bool,
    positive_guard_false: bool,
    capacity_guard_false: bool,
    assigned: bool,
) {
    let assigned = usize::from(assigned);
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
    assert_eq!(state.capacity_limit_cp_air_assignment_count, assigned);
    assert_eq!(state.source_site_execution_count, 3 * assigned);
    assert_eq!(state.mixed_air_humidity_ratio_read_count, assigned);
    assert_eq!(state.psychrometric_cp_air_evaluation_count, assigned);
    assert_eq!(state.cp_air_assignment_write_count, assigned);
}
