use super::*;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    cooling_positive_supply_capacity_limit_sensible_output_assignment_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle_summary,
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
    .expect("source-ordered CP339 coupling");
    (runtime, output)
}

#[test]
fn scheduled_binding_assigns_exact_sensible_output_for_both_capacity_limit_selectors() {
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
        );
        let predecessor =
            output.calculation_cooling_positive_supply_capacity_limit_cp_air_assignment;
        let supply_flow = output.calculation_cooling_supply_mass_flow_positive_guard;
        let mixed_air = output.calculation_cooling_mixed_air_call;
        let supply_enthalpy = output.calculation_cooling_positive_supply_enthalpy_assignment;
        let assignment = output
            .calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment;

        assert!(predecessor.capacity_limit_cp_air_assignment_executed);
        assert!(
            cooling_positive_supply_capacity_limit_sensible_output_assignment_snapshot_is_exact_direct_release(
                assignment,
            )
        );
        assert!(assignment.capacity_limit_sensible_output_assignment_executed);
        assert!(assignment.supply_mass_flow_rate_read);
        assert!(assignment.mixed_air_enthalpy_read);
        assert!(assignment.supply_enthalpy_read);
        assert!(assignment.enthalpy_difference_calculated);
        assert!(assignment.cooling_sensible_output_calculated);
        assert!(assignment.cooling_sensible_output_assigned);

        assert_eq!(
            assignment.supply_mass_flow_rate_kg_per_s.map(f64::to_bits),
            supply_flow
                .supply_mass_flow_rate_kg_per_s
                .map(f64::to_bits)
        );
        assert_eq!(
            assignment.mixed_air_enthalpy_j_per_kg.map(f64::to_bits),
            mixed_air
                .mixed_air_enthalpy_projection_j_per_kg
                .map(f64::to_bits)
        );
        assert_eq!(
            assignment.supply_enthalpy_j_per_kg.map(f64::to_bits),
            supply_enthalpy.supply_enthalpy_j_per_kg.map(f64::to_bits)
        );

        let expected_difference = mixed_air
            .mixed_air_enthalpy_projection_j_per_kg
            .expect("same-call CP329 mixed-air enthalpy")
            - supply_enthalpy
                .supply_enthalpy_j_per_kg
                .expect("same-call CP336 supply enthalpy");
        let expected_output = supply_flow
            .supply_mass_flow_rate_kg_per_s
            .expect("same-call CP330 supply flow")
            * expected_difference;
        assert_eq!(
            assignment
                .mixed_air_minus_supply_enthalpy_j_per_kg
                .map(f64::to_bits),
            Some(expected_difference.to_bits())
        );
        assert_eq!(
            assignment
                .calculated_cooling_sensible_output_w
                .map(f64::to_bits),
            Some(expected_output.to_bits())
        );
        assert_eq!(
            assignment.cooling_sensible_output_w.map(f64::to_bits),
            Some(expected_output.to_bits())
        );

        let state =
            purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP339 lifecycle")
            .state;
        assert_counter_shape(&state, false, false, false, false, true);
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
            Some(0.0),
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
            .calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment;

        assert_eq!(assignment.unit_off_skipped, unit_off);
        assert_eq!(assignment.non_cooling_skipped, non_cooling);
        assert_eq!(
            assignment.positive_guard_false_fallthrough_skipped,
            positive_guard_false
        );
        assert_eq!(
            assignment.capacity_limit_guard_false_fallthrough_skipped,
            capacity_guard_false
        );
        assert_complete_null(assignment);
        let state =
            purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP339 skip lifecycle")
            .state;
        assert_counter_shape(
            &state,
            unit_off,
            non_cooling,
            positive_guard_false,
            capacity_guard_false,
            false,
        );
    }
}

fn assert_complete_null(
    snapshot: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
) {
    assert!(!snapshot.capacity_limit_sensible_output_assignment_executed);
    assert!(!snapshot.supply_mass_flow_rate_read);
    assert!(snapshot.supply_mass_flow_rate_kg_per_s.is_none());
    assert!(!snapshot.mixed_air_enthalpy_read);
    assert!(snapshot.mixed_air_enthalpy_j_per_kg.is_none());
    assert!(!snapshot.supply_enthalpy_read);
    assert!(snapshot.supply_enthalpy_j_per_kg.is_none());
    assert!(!snapshot.enthalpy_difference_calculated);
    assert!(
        snapshot
            .mixed_air_minus_supply_enthalpy_j_per_kg
            .is_none()
    );
    assert!(!snapshot.cooling_sensible_output_calculated);
    assert!(snapshot.calculated_cooling_sensible_output_w.is_none());
    assert!(!snapshot.cooling_sensible_output_assigned);
    assert!(snapshot.cooling_sensible_output_w.is_none());
}

fn assert_counter_shape(
    state: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRuntimeState,
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
    assert_eq!(
        state.capacity_limit_sensible_output_assignment_count,
        assigned
    );
    assert_eq!(state.source_site_execution_count, 6 * assigned);
    assert_eq!(state.supply_mass_flow_rate_read_count, assigned);
    assert_eq!(state.mixed_air_enthalpy_read_count, assigned);
    assert_eq!(state.supply_enthalpy_read_count, assigned);
    assert_eq!(state.enthalpy_difference_calculation_count, assigned);
    assert_eq!(state.cooling_sensible_output_calculation_count, assigned);
    assert_eq!(
        state.cooling_sensible_output_assignment_write_count,
        assigned
    );
}
