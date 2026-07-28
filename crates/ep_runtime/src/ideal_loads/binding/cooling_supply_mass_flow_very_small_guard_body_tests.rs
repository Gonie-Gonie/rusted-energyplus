use super::*;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyRuntimeState,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard_body,
    cooling_supply_mass_flow_very_small_guard_body_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_supply_mass_flow_very_small_guard_body_lifecycle_summary,
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
    .expect("source-ordered CP328 coupling");
    (runtime, output)
}

#[test]
fn scheduled_binding_consumes_cp327_body_entry_and_assigns_positive_zero() {
    for (limit, capacity, expected_assignment) in [
        (IdealLoadsLimit::NoLimit, None, false),
        (IdealLoadsLimit::LimitCapacity, Some(0.0), true),
    ] {
        let (runtime, output) = run_case(limit, capacity, 3_000.0, 1.0);
        let predecessor = output.calculation_cooling_supply_mass_flow_very_small_guard;
        let body = output.calculation_cooling_supply_mass_flow_very_small_guard_body;

        assert!(
            cooling_supply_mass_flow_very_small_guard_body_snapshot_is_exact_direct_release(body)
        );
        assert_eq!(body.system, predecessor.system);
        assert_eq!(body.parent_call_ordinal, predecessor.parent_call_ordinal);
        assert_eq!(body.controlled_zone, predecessor.controlled_zone);
        assert_eq!(
            body.predecessor_zero_flow_reset_body_entered,
            predecessor.zero_flow_reset_body_entered
        );
        assert_eq!(
            body.predecessor_active_guard_false_fallthrough,
            predecessor.active_guard_false_fallthrough
        );
        assert_eq!(
            body.predecessor_supply_mass_flow_rate_kg_per_s
                .map(f64::to_bits),
            predecessor.supply_mass_flow_rate_kg_per_s.map(f64::to_bits)
        );
        assert_eq!(body.zero_flow_reset_body_entered, expected_assignment);
        assert_eq!(
            body.supply_mass_flow_rate_positive_zero_assignment_performed,
            expected_assignment
        );
        if expected_assignment {
            assert_eq!(
                body.assigned_supply_mass_flow_rate_kg_per_s
                    .map(f64::to_bits),
                Some(0)
            );
            assert_eq!(
                body.resulting_supply_mass_flow_rate_kg_per_s
                    .map(f64::to_bits),
                Some(0)
            );
        } else {
            assert!(body.assigned_supply_mass_flow_rate_kg_per_s.is_none());
            assert_eq!(
                body.resulting_supply_mass_flow_rate_kg_per_s
                    .map(f64::to_bits),
                predecessor.supply_mass_flow_rate_kg_per_s.map(f64::to_bits)
            );
        }

        let state =
            purchased_air_calc_cooling_supply_mass_flow_very_small_guard_body_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP328 lifecycle")
            .state;
        assert_counter_shape(&state, usize::from(expected_assignment));
    }
}

fn assert_counter_shape(
    state: &PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyRuntimeState,
    assignments: usize,
) {
    assert_eq!(state.transition_count, 1);
    assert_eq!(state.cooling_body_entry_count, 1);
    assert_eq!(state.unit_off_skip_count, 0);
    assert_eq!(state.non_cooling_skip_count, 0);
    assert_eq!(state.zero_flow_reset_body_entry_count, assignments);
    assert_eq!(state.body_skip_count, 1 - assignments);
    assert_eq!(state.active_guard_false_fallthrough_count, 1 - assignments);
    assert_eq!(
        state.supply_mass_flow_rate_positive_zero_assignment_count,
        assignments
    );
}

#[test]
fn scheduled_binding_skips_the_cp328_site_when_cooling_is_inactive() {
    for (availability, independent_load_w, unit_off, non_cooling) in
        [(0.0, 3_000.0, true, false), (1.0, 0.0, false, true)]
    {
        let (runtime, output) = run_case(
            IdealLoadsLimit::NoLimit,
            None,
            independent_load_w,
            availability,
        );
        let body = output.calculation_cooling_supply_mass_flow_very_small_guard_body;

        assert!(
            cooling_supply_mass_flow_very_small_guard_body_snapshot_is_exact_direct_release(body)
        );
        assert_eq!(body.unit_off_skipped, unit_off);
        assert_eq!(body.non_cooling_skipped, non_cooling);
        assert!(!body.cooling_body_entered);
        assert!(body.body_skipped);
        assert!(body.predecessor_supply_mass_flow_rate_kg_per_s.is_none());
        assert!(!body.supply_mass_flow_rate_positive_zero_assignment_performed);
        assert!(body.assigned_supply_mass_flow_rate_kg_per_s.is_none());
        assert!(body.resulting_supply_mass_flow_rate_kg_per_s.is_none());

        let state =
            purchased_air_calc_cooling_supply_mass_flow_very_small_guard_body_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP328 skip lifecycle")
            .state;
        assert_eq!(state.transition_count, 1);
        assert_eq!(state.cooling_body_entry_count, 0);
        assert_eq!(state.unit_off_skip_count, usize::from(unit_off));
        assert_eq!(state.non_cooling_skip_count, usize::from(non_cooling));
        assert_eq!(state.zero_flow_reset_body_entry_count, 0);
        assert_eq!(state.body_skip_count, 1);
        assert_eq!(state.active_guard_false_fallthrough_count, 0);
        assert_eq!(
            state.supply_mass_flow_rate_positive_zero_assignment_count,
            0
        );
    }
}

#[test]
fn public_cp328_release_rejects_replay_and_forged_cp327_ordinal_without_mutation() {
    let (model, cache) = fixture(|_| {});
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut zone_state = zone_state_for_temp_independent_load(3_000.0);
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
    .expect("completed CP328 release");
    let predecessor = output.calculation_cooling_supply_mass_flow_very_small_guard;

    let before_replay = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard_body(
            &mut runtime,
            binding.system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before_replay);

    let mut forged = predecessor;
    forged.parent_call_ordinal += 1;
    let before_forgery = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard_body(
            &mut runtime,
            binding.system,
            forged,
        )
        .is_err()
    );
    assert_eq!(runtime, before_forgery);
}
