use super::*;
use crate::ideal_loads::purchased_air_calc_cooling_supply_mass_flow_ems_override_guard_lifecycle_summary;

fn run_case(
    independent_load_w: f64,
    availability: f64,
) -> (
    PurchasedAirRuntimeState,
    DirectZonePurchasedAirScheduledCouplingOutput,
) {
    let (model, cache) = fixture(|typed| {
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
    .expect("source-ordered CP323 coupling");
    (runtime, output)
}

#[test]
fn scheduled_binding_evaluates_the_retained_false_ems_guard_after_cp322() {
    let (runtime, output) = run_case(3_000.0, 1.0);
    let predecessor = output.calculation_cooling_supply_mass_flow_maximum;
    let guard = output.calculation_cooling_supply_mass_flow_ems_override_guard;

    assert!(predecessor.cooling_body_entered);
    assert_eq!(guard.system, predecessor.system);
    assert_eq!(guard.parent_call_ordinal, predecessor.parent_call_ordinal);
    assert!(guard.cooling_body_entered);
    assert!(guard.ems_supply_mass_flow_override_flag_read);
    assert_eq!(guard.ems_supply_mass_flow_override_enabled, Some(false));
    assert!(guard.ems_supply_mass_flow_override_guard_evaluated);
    assert!(!guard.ems_supply_mass_flow_override_body_entered);
    assert!(guard.ems_supply_mass_flow_override_guard_false_fallthrough);

    let state = purchased_air_calc_cooling_supply_mass_flow_ems_override_guard_lifecycle_summary(
        &runtime,
        output.initialization.system,
    )
    .expect("CP323 lifecycle")
    .state;
    assert_eq!(state.transition_count, 1);
    assert_eq!(state.cooling_body_entry_count, 1);
    assert_eq!(state.ems_supply_mass_flow_override_flag_read_count, 1);
    assert_eq!(
        state.ems_supply_mass_flow_override_guard_evaluation_count,
        1
    );
    assert_eq!(state.ems_supply_mass_flow_override_body_entry_count, 0);
    assert_eq!(
        state.ems_supply_mass_flow_override_guard_false_fallthrough_count,
        1
    );
}

#[test]
fn scheduled_binding_skips_all_line_2157_sites_when_cooling_is_inactive() {
    for (availability, independent_load_w, unit_off, non_cooling) in
        [(0.0, 3_000.0, true, false), (1.0, 0.0, false, true)]
    {
        let (runtime, output) = run_case(independent_load_w, availability);
        let guard = output.calculation_cooling_supply_mass_flow_ems_override_guard;
        assert_eq!(guard.unit_off_skipped, unit_off);
        assert_eq!(guard.non_cooling_skipped, non_cooling);
        assert!(!guard.cooling_body_entered);
        assert!(!guard.ems_supply_mass_flow_override_flag_read);
        assert_eq!(guard.ems_supply_mass_flow_override_enabled, None);
        assert!(!guard.ems_supply_mass_flow_override_guard_evaluated);
        assert!(!guard.ems_supply_mass_flow_override_body_entered);
        assert!(!guard.ems_supply_mass_flow_override_guard_false_fallthrough);

        let state =
            purchased_air_calc_cooling_supply_mass_flow_ems_override_guard_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP323 skip lifecycle")
            .state;
        assert_eq!(state.transition_count, 1);
        assert_eq!(state.unit_off_skip_count, usize::from(unit_off));
        assert_eq!(state.non_cooling_skip_count, usize::from(non_cooling));
        assert_eq!(state.ems_supply_mass_flow_override_flag_read_count, 0);
    }
}
