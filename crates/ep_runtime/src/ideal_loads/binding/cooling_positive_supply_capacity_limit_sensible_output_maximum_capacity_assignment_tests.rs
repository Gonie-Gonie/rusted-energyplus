use super::*;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
    cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_lifecycle_summary,
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
    .expect("source-ordered CP341 coupling");
    (runtime, output)
}

#[test]
fn scheduled_binding_preserves_false_and_true_assignment_routes() {
    for (maximum_capacity_w, expected_assignment) in [(1.0e9, false), (1.0, true)] {
        let (runtime, output) = run_case(
            IdealLoadsLimit::LimitCapacity,
            None,
            Some(maximum_capacity_w),
            3_000.0,
            1.0,
        );
        let predecessor =
            output.calculation_cooling_positive_supply_capacity_limit_sensible_output_guard;
        let assignment = output
            .calculation_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment;

        assert!(
            cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_snapshot_is_exact_direct_release(
                assignment,
            )
        );
        assert_eq!(
            assignment.capacity_limit_sensible_output_maximum_capacity_assignment_executed,
            expected_assignment
        );
        assert_eq!(
            assignment
                .preexisting_cooling_sensible_output_w
                .map(f64::to_bits),
            predecessor.cooling_sensible_output_w.map(f64::to_bits)
        );
        if expected_assignment {
            assert!(assignment.maximum_total_cooling_capacity_read);
            assert!(assignment.cooling_sensible_output_assigned);
            assert_eq!(
                assignment
                    .assigned_cooling_sensible_output_w
                    .map(f64::to_bits),
                predecessor
                    .maximum_total_cooling_capacity_w
                    .map(f64::to_bits)
            );
            assert_eq!(
                assignment.maximum_total_cooling_capacity_w.map(f64::to_bits),
                predecessor
                    .maximum_total_cooling_capacity_w
                    .map(f64::to_bits)
            );
            assert_eq!(
                assignment
                    .resulting_cooling_sensible_output_w
                    .map(f64::to_bits),
                predecessor
                    .maximum_total_cooling_capacity_w
                    .map(f64::to_bits)
            );
        } else {
            assert!(!assignment.maximum_total_cooling_capacity_read);
            assert!(assignment.maximum_total_cooling_capacity_w.is_none());
            assert!(!assignment.cooling_sensible_output_assigned);
            assert!(assignment.assigned_cooling_sensible_output_w.is_none());
            assert_eq!(
                assignment
                    .resulting_cooling_sensible_output_w
                    .map(f64::to_bits),
                assignment
                    .preexisting_cooling_sensible_output_w
                    .map(f64::to_bits)
            );
        }

        let state =
            purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP341 lifecycle")
            .state;
        assert_counter_shape(&state, false, false, false, false, !expected_assignment, expected_assignment);
    }
}

#[test]
fn scheduled_binding_preserves_complete_null_first_four_skip_routes() {
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
        let assignment = output
            .calculation_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment;

        assert_complete_null(assignment);
        let state =
            purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP341 skip lifecycle")
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
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
) {
    assert!(!snapshot.capacity_limit_sensible_output_maximum_capacity_assignment_executed);
    assert!(snapshot.preexisting_cooling_sensible_output_w.is_none());
    assert!(!snapshot.maximum_total_cooling_capacity_read);
    assert!(snapshot.maximum_total_cooling_capacity_w.is_none());
    assert!(!snapshot.cooling_sensible_output_assigned);
    assert!(snapshot.assigned_cooling_sensible_output_w.is_none());
    assert!(snapshot.resulting_cooling_sensible_output_w.is_none());
}

#[allow(clippy::too_many_arguments)]
fn assert_counter_shape(
    state: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentRuntimeState,
    unit_off: bool,
    non_cooling: bool,
    positive_guard_false: bool,
    capacity_guard_false: bool,
    comparison_false: bool,
    assignment: bool,
) {
    let assignment = usize::from(assignment);
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
        usize::from(comparison_false)
    );
    assert_eq!(
        state.capacity_limit_sensible_output_maximum_capacity_assignment_count,
        assignment
    );
    assert_eq!(state.source_site_execution_count, 2 * assignment);
    assert_eq!(state.maximum_total_cooling_capacity_read_count, assignment);
    assert_eq!(state.cooling_sensible_output_assignment_write_count, assignment);
}
