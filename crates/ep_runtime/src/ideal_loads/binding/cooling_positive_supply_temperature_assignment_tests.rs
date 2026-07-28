use super::*;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRuntimeState,
    cooling_positive_supply_temperature_assignment_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_positive_supply_temperature_assignment_lifecycle_summary,
};

fn run_case(
    cooling_limit: IdealLoadsLimit,
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
        system.maximum_total_cooling_capacity_w = maximum_capacity_w.map(AutosizeOrNumber::Value);
        schedule_mut(typed, ScheduleId(3)).hourly_value = availability;
    });
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut zone_state = zone_state_for_temp_independent_load(independent_load_w);
    let source_zone_node_temperature_c = zone_state.mean_air_temperature_c;
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
    .expect("source-ordered CP332 coupling");
    (runtime, output, source_zone_node_temperature_c)
}

#[test]
fn scheduled_binding_assigns_source_grouped_supply_temperature_bit_exactly() {
    let (runtime, output, source_zone_node_temperature_c) =
        run_case(IdealLoadsLimit::NoLimit, None, 3_000.0, 1.0);
    let predecessor = output.calculation_cooling_positive_supply_cp_air_assignment;
    let guard = output.calculation_cooling_supply_mass_flow_positive_guard;
    let assignment = output.calculation_cooling_positive_supply_temperature_assignment;

    assert!(predecessor.cp_air_assignment_executed);
    assert!(
        cooling_positive_supply_temperature_assignment_snapshot_is_exact_direct_release(assignment)
    );
    assert_eq!(assignment.system, predecessor.system);
    assert_eq!(
        assignment.parent_call_ordinal,
        predecessor.parent_call_ordinal
    );
    assert_eq!(assignment.controlled_zone, predecessor.controlled_zone);
    assert!(assignment.supply_temperature_assignment_executed);

    let load = output
        .calculation_entry
        .demand
        .remaining_output_req_to_cool_sp_w;
    let cp_air = predecessor.cp_air_j_per_kg_k.expect("CP331 CpAir");
    let mass_flow = guard
        .supply_mass_flow_rate_kg_per_s
        .expect("CP330 supply mass flow");
    let denominator = cp_air * mass_flow;
    let load_temperature = load / denominator;
    let expected = load_temperature + source_zone_node_temperature_c;

    assert_eq!(
        assignment.zone_cooling_setpoint_load_w.map(f64::to_bits),
        Some(load.to_bits())
    );
    assert_eq!(
        assignment.cp_air_j_per_kg_k.map(f64::to_bits),
        Some(cp_air.to_bits())
    );
    assert_eq!(
        assignment.supply_mass_flow_rate_kg_per_s.map(f64::to_bits),
        Some(mass_flow.to_bits())
    );
    assert_eq!(
        assignment
            .cp_air_times_supply_mass_flow_rate_w_per_k
            .map(f64::to_bits),
        Some(denominator.to_bits())
    );
    assert_eq!(
        assignment
            .zone_cooling_setpoint_load_over_denominator_c
            .map(f64::to_bits),
        Some(load_temperature.to_bits())
    );
    assert_eq!(
        assignment.zone_node_temperature_c.map(f64::to_bits),
        Some(source_zone_node_temperature_c.to_bits())
    );
    assert_eq!(
        assignment.calculated_supply_temperature_c.map(f64::to_bits),
        Some(expected.to_bits())
    );
    assert_eq!(
        assignment.supply_temperature_c.map(f64::to_bits),
        Some(expected.to_bits())
    );
    assert_eq!(
        assignment.zone_node_temperature_c.map(f64::to_bits),
        output
            .calculation_cooling_sensible_flow
            .zone_temperature_c
            .map(f64::to_bits)
    );
    assert_eq!(
        assignment.zone_node_temperature_c.map(f64::to_bits),
        output
            .calculation_cooling_mixed_air_call
            .recirculation_temperature_c
            .map(f64::to_bits)
    );
    assert_eq!(
        assignment.zone_node_temperature_c.map(f64::to_bits),
        output
            .calculation_cooling_mixed_air_call
            .mixed_air_temperature_c
            .map(f64::to_bits)
    );

    let state =
        purchased_air_calc_cooling_positive_supply_temperature_assignment_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP332 lifecycle")
        .state;
    assert_counter_shape(&state, true, false, false, false);
}

#[test]
fn scheduled_binding_skips_cp332_after_the_active_positive_guard_falls_through() {
    let (runtime, output, _) = run_case(IdealLoadsLimit::LimitCapacity, Some(0.0), 3_000.0, 1.0);
    let predecessor = output.calculation_cooling_positive_supply_cp_air_assignment;
    let assignment = output.calculation_cooling_positive_supply_temperature_assignment;

    assert!(predecessor.positive_guard_false_fallthrough_skipped);
    assert!(
        cooling_positive_supply_temperature_assignment_snapshot_is_exact_direct_release(assignment)
    );
    assert!(assignment.positive_guard_false_fallthrough_skipped);
    assert!(!assignment.supply_temperature_assignment_executed);
    assert_snapshot_has_no_source_values(assignment);

    let state =
        purchased_air_calc_cooling_positive_supply_temperature_assignment_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP332 false-guard lifecycle")
        .state;
    assert_counter_shape(&state, false, false, false, true);
}

#[test]
fn scheduled_binding_preserves_unit_off_and_non_cooling_cp332_skip_routes() {
    for (availability, independent_load_w, unit_off, non_cooling) in
        [(0.0, 3_000.0, true, false), (1.0, 0.0, false, true)]
    {
        let (runtime, output, _) = run_case(
            IdealLoadsLimit::NoLimit,
            None,
            independent_load_w,
            availability,
        );
        let assignment = output.calculation_cooling_positive_supply_temperature_assignment;

        assert!(
            cooling_positive_supply_temperature_assignment_snapshot_is_exact_direct_release(
                assignment
            )
        );
        assert_eq!(assignment.unit_off_skipped, unit_off);
        assert_eq!(assignment.non_cooling_skipped, non_cooling);
        assert!(!assignment.supply_temperature_assignment_executed);
        assert_snapshot_has_no_source_values(assignment);

        let state =
            purchased_air_calc_cooling_positive_supply_temperature_assignment_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP332 skipped lifecycle")
            .state;
        assert_counter_shape(&state, false, unit_off, non_cooling, false);
    }
}

fn assert_snapshot_has_no_source_values(
    snapshot: crate::ideal_loads::
        PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
) {
    assert!(!snapshot.zone_cooling_setpoint_load_read);
    assert!(snapshot.zone_cooling_setpoint_load_w.is_none());
    assert!(!snapshot.cp_air_read);
    assert!(snapshot.cp_air_j_per_kg_k.is_none());
    assert!(!snapshot.supply_mass_flow_rate_read);
    assert!(snapshot.supply_mass_flow_rate_kg_per_s.is_none());
    assert!(!snapshot.cp_air_times_supply_mass_flow_rate_calculated);
    assert!(
        snapshot
            .cp_air_times_supply_mass_flow_rate_w_per_k
            .is_none()
    );
    assert!(!snapshot.zone_cooling_setpoint_load_over_denominator_calculated);
    assert!(
        snapshot
            .zone_cooling_setpoint_load_over_denominator_c
            .is_none()
    );
    assert!(!snapshot.zone_node_temperature_read);
    assert!(snapshot.zone_node_temperature_c.is_none());
    assert!(!snapshot.supply_temperature_calculated);
    assert!(snapshot.calculated_supply_temperature_c.is_none());
    assert!(!snapshot.supply_temperature_assigned);
    assert!(snapshot.supply_temperature_c.is_none());
}

fn assert_counter_shape(
    state: &PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRuntimeState,
    assigned: bool,
    unit_off: bool,
    non_cooling: bool,
    guard_false: bool,
) {
    let assignments = usize::from(assigned);
    assert_eq!(state.transition_count, 1);
    assert_eq!(state.unit_off_skip_count, usize::from(unit_off));
    assert_eq!(state.non_cooling_skip_count, usize::from(non_cooling));
    assert_eq!(
        state.positive_guard_false_fallthrough_skip_count,
        usize::from(guard_false)
    );
    assert_eq!(state.supply_temperature_assignment_count, assignments);
    assert_eq!(state.source_site_execution_count, 8 * assignments);
    assert_eq!(state.zone_cooling_setpoint_load_read_count, assignments);
    assert_eq!(state.cp_air_read_count, assignments);
    assert_eq!(state.supply_mass_flow_rate_read_count, assignments);
    assert_eq!(
        state.cp_air_times_supply_mass_flow_rate_calculation_count,
        assignments
    );
    assert_eq!(
        state.zone_cooling_setpoint_load_over_denominator_calculation_count,
        assignments
    );
    assert_eq!(state.zone_node_temperature_read_count, assignments);
    assert_eq!(state.supply_temperature_calculation_count, assignments);
    assert_eq!(state.supply_temperature_assignment_write_count, assignments);
}
