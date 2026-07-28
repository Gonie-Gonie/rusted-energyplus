use super::*;
use crate::{
    ideal_loads::{
        PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentRuntimeState,
        cooling_positive_supply_cp_air_assignment_snapshot_is_exact_direct_release,
        purchased_air_calc_cooling_positive_supply_cp_air_assignment_lifecycle_summary,
    },
    psychrometrics::energyplus_psy_cp_air_fn_w,
};

fn run_case(
    cooling_limit: IdealLoadsLimit,
    maximum_capacity_w: Option<f64>,
    independent_load_w: f64,
    availability: f64,
    source_humidity_ratio: f64,
) -> (
    PurchasedAirRuntimeState,
    DirectZonePurchasedAirScheduledCouplingOutput,
) {
    let (model, cache) = fixture(|typed| {
        let system = &mut typed.ideal_loads_air_systems[0];
        system.cooling_limit = cooling_limit;
        system.maximum_total_cooling_capacity_w = maximum_capacity_w.map(AutosizeOrNumber::Value);
        schedule_mut(typed, ScheduleId(3)).hourly_value = availability;
    });
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut zone_state = zone_state_for_temp_independent_load(independent_load_w);
    zone_state.air_humidity_ratio = source_humidity_ratio;
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
    .expect("source-ordered CP331 coupling");
    (runtime, output)
}

#[test]
fn scheduled_binding_assigns_cp_air_from_the_live_zone_humidity_bit_exactly() {
    let source_humidity_ratio = 0.008_765_432_109_876_543_f64;
    let (runtime, output) = run_case(
        IdealLoadsLimit::NoLimit,
        None,
        3_000.0,
        1.0,
        source_humidity_ratio,
    );
    let predecessor = output.calculation_cooling_supply_mass_flow_positive_guard;
    let assignment = output.calculation_cooling_positive_supply_cp_air_assignment;

    assert!(predecessor.positive_supply_mass_flow_body_entered);
    assert!(cooling_positive_supply_cp_air_assignment_snapshot_is_exact_direct_release(assignment));
    assert_eq!(assignment.system, predecessor.system);
    assert_eq!(
        assignment.parent_call_ordinal,
        predecessor.parent_call_ordinal
    );
    assert_eq!(assignment.controlled_zone, predecessor.controlled_zone);
    assert!(assignment.predecessor_positive_supply_mass_flow_body_entered);
    assert!(!assignment.predecessor_active_guard_false_fallthrough);
    assert!(assignment.cp_air_assignment_executed);
    assert!(assignment.zone_humidity_ratio_read);
    assert_eq!(
        assignment.zone_humidity_ratio.map(f64::to_bits),
        Some(source_humidity_ratio.to_bits())
    );
    let mixed_air = output.calculation_cooling_mixed_air_call;
    assert_eq!(
        assignment.zone_humidity_ratio.map(f64::to_bits),
        mixed_air.recirculation_humidity_ratio.map(f64::to_bits)
    );
    assert_eq!(
        assignment.zone_humidity_ratio.map(f64::to_bits),
        mixed_air.mixed_air_humidity_ratio.map(f64::to_bits)
    );
    assert!(assignment.psychrometric_cp_air_evaluated);
    let expected_cp_air = energyplus_psy_cp_air_fn_w(source_humidity_ratio);
    assert_eq!(
        assignment
            .psychrometric_cp_air_result_j_per_kg_k
            .map(f64::to_bits),
        Some(expected_cp_air.to_bits())
    );
    assert!(assignment.cp_air_assigned);
    assert_eq!(
        assignment.cp_air_j_per_kg_k.map(f64::to_bits),
        Some(expected_cp_air.to_bits())
    );

    let state = purchased_air_calc_cooling_positive_supply_cp_air_assignment_lifecycle_summary(
        &runtime,
        output.initialization.system,
    )
    .expect("CP331 lifecycle")
    .state;
    assert_active_counter_shape(&state, true);
}

#[test]
fn scheduled_binding_accepts_negative_zero_humidity_with_exact_cp329_lineage() {
    let source_humidity_ratio = -0.0_f64;
    let (runtime, output) = run_case(
        IdealLoadsLimit::NoLimit,
        None,
        3_000.0,
        1.0,
        source_humidity_ratio,
    );
    let predecessor = output.calculation_cooling_supply_mass_flow_positive_guard;
    let mixed_air = output.calculation_cooling_mixed_air_call;
    let assignment = output.calculation_cooling_positive_supply_cp_air_assignment;

    assert!(predecessor.positive_supply_mass_flow_body_entered);
    assert!(cooling_positive_supply_cp_air_assignment_snapshot_is_exact_direct_release(assignment));
    assert_eq!(
        assignment.zone_humidity_ratio.map(f64::to_bits),
        Some(source_humidity_ratio.to_bits())
    );
    assert_eq!(
        assignment.zone_humidity_ratio.map(f64::to_bits),
        mixed_air.recirculation_humidity_ratio.map(f64::to_bits)
    );
    assert_eq!(
        assignment.zone_humidity_ratio.map(f64::to_bits),
        mixed_air.mixed_air_humidity_ratio.map(f64::to_bits)
    );
    let expected_cp_air = energyplus_psy_cp_air_fn_w(source_humidity_ratio);
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

    let state = purchased_air_calc_cooling_positive_supply_cp_air_assignment_lifecycle_summary(
        &runtime,
        output.initialization.system,
    )
    .expect("CP331 negative-zero lifecycle")
    .state;
    assert_active_counter_shape(&state, true);
}

fn assert_active_counter_shape(
    state: &PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentRuntimeState,
    assigned: bool,
) {
    assert_eq!(state.transition_count, 1);
    assert_eq!(state.unit_off_skip_count, 0);
    assert_eq!(state.non_cooling_skip_count, 0);
    assert_eq!(
        state.positive_guard_false_fallthrough_skip_count,
        usize::from(!assigned)
    );
    assert_eq!(state.cp_air_assignment_count, usize::from(assigned));
    assert_eq!(state.source_site_execution_count, 3 * usize::from(assigned));
    assert_eq!(state.zone_humidity_ratio_read_count, usize::from(assigned));
    assert_eq!(
        state.psychrometric_cp_air_evaluation_count,
        usize::from(assigned)
    );
    assert_eq!(state.cp_air_assignment_write_count, usize::from(assigned));
}

#[test]
fn scheduled_binding_skips_cp331_after_the_active_positive_guard_falls_through() {
    let (runtime, output) = run_case(
        IdealLoadsLimit::LimitCapacity,
        Some(0.0),
        3_000.0,
        1.0,
        0.008,
    );
    let predecessor = output.calculation_cooling_supply_mass_flow_positive_guard;
    let assignment = output.calculation_cooling_positive_supply_cp_air_assignment;

    assert!(predecessor.active_guard_false_fallthrough);
    assert!(cooling_positive_supply_cp_air_assignment_snapshot_is_exact_direct_release(assignment));
    assert!(assignment.positive_guard_false_fallthrough_skipped);
    assert!(!assignment.cp_air_assignment_executed);
    assert!(!assignment.zone_humidity_ratio_read);
    assert!(assignment.zone_humidity_ratio.is_none());
    assert!(!assignment.psychrometric_cp_air_evaluated);
    assert!(assignment.psychrometric_cp_air_result_j_per_kg_k.is_none());
    assert!(!assignment.cp_air_assigned);
    assert!(assignment.cp_air_j_per_kg_k.is_none());

    let state = purchased_air_calc_cooling_positive_supply_cp_air_assignment_lifecycle_summary(
        &runtime,
        output.initialization.system,
    )
    .expect("CP331 false-guard lifecycle")
    .state;
    assert_active_counter_shape(&state, false);
}

#[test]
fn scheduled_binding_preserves_unit_off_and_non_cooling_cp331_skip_routes() {
    for (availability, independent_load_w, unit_off, non_cooling) in
        [(0.0, 3_000.0, true, false), (1.0, 0.0, false, true)]
    {
        let (runtime, output) = run_case(
            IdealLoadsLimit::NoLimit,
            None,
            independent_load_w,
            availability,
            0.008,
        );
        let assignment = output.calculation_cooling_positive_supply_cp_air_assignment;

        assert!(
            cooling_positive_supply_cp_air_assignment_snapshot_is_exact_direct_release(assignment)
        );
        assert_eq!(assignment.unit_off_skipped, unit_off);
        assert_eq!(assignment.non_cooling_skipped, non_cooling);
        assert!(!assignment.positive_guard_false_fallthrough_skipped);
        assert!(!assignment.cp_air_assignment_executed);
        assert!(assignment.zone_humidity_ratio.is_none());
        assert!(assignment.cp_air_j_per_kg_k.is_none());

        let state = purchased_air_calc_cooling_positive_supply_cp_air_assignment_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP331 skipped lifecycle")
        .state;
        assert_eq!(state.transition_count, 1);
        assert_eq!(state.unit_off_skip_count, usize::from(unit_off));
        assert_eq!(state.non_cooling_skip_count, usize::from(non_cooling));
        assert_eq!(state.positive_guard_false_fallthrough_skip_count, 0);
        assert_eq!(state.cp_air_assignment_count, 0);
        assert_eq!(state.source_site_execution_count, 0);
    }
}
