use super::*;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingOaMaxFlowBodyRuntimeState,
    advance_direct_no_oa_calc_cooling_oa_max_flow_body,
    purchased_air_calc_cooling_oa_max_flow_body_lifecycle_summary,
};

#[test]
fn scheduled_binding_orders_cooling_oa_max_flow_body_after_gate_before_numerical_calc() {
    let (model, cache) = fixture(|_| {});
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut zone_state = zone_state_for_temp_independent_load(0.0);
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
    .expect("source-ordered CP314 coupling");

    let gate = output.calculation_cooling_oa_max_flow_gate;
    let body = output.calculation_cooling_oa_max_flow_body;
    assert_eq!(body.parent_call_ordinal, gate.parent_call_ordinal);
    assert_eq!(body.system, gate.system);
    assert_eq!(
        body.predecessor_maximum_cooling_flow_body_entered,
        gate.maximum_cooling_flow_body_entered
    );
    assert!(body.body_skipped);
    assert_eq!(
        usize::from(body.unit_off_skipped)
            + usize::from(body.non_cooling_skipped)
            + usize::from(body.active_guard_false_economizer_fallthrough),
        1
    );
    assert!(!body.outdoor_air_mass_flow_rate_read);
    assert!(!body.standard_air_density_read);
    assert!(!body.warning_counter_read);
    assert!(!body.outdoor_air_mass_flow_clamp_assignment_performed);

    let lifecycle = purchased_air_calc_cooling_oa_max_flow_body_lifecycle_summary(
        &runtime,
        binding.ideal_loads_air_system,
    )
    .expect("CP314 lifecycle");
    assert_eq!(lifecycle.state.transition_count, 1);
    assert_eq!(lifecycle.state.body_entry_count, 0);
    assert_eq!(lifecycle.state.body_skip_count, 1);
    assert_eq!(
        lifecycle.state.unit_off_skip_count
            + lifecycle.state.non_cooling_skip_count
            + lifecycle
                .state
                .active_guard_false_economizer_fallthrough_count,
        1
    );
    assert_eq!(lifecycle.state.outdoor_air_mass_flow_rate_read_count, 0);
    assert_eq!(lifecycle.state.standard_air_density_read_count, 0);
    assert_eq!(lifecycle.state.warning_counter_read_count, 0);
    assert_eq!(lifecycle.state.outdoor_air_flow_max_cooling_output_index, 0);
    assert_eq!(
        lifecycle
            .state
            .characterized_total_warning_error_increment_count,
        0
    );
}

#[test]
fn public_cooling_oa_max_flow_body_rejects_forgery_and_replay_without_mutation() {
    let (model, cache) = fixture(|_| {});
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut zone_state = zone_state_for_temp_independent_load(0.0);
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
    .expect("source-ordered CP314 coupling");
    let before = purchased_air_calc_cooling_oa_max_flow_body_lifecycle_summary(
        &runtime,
        binding.ideal_loads_air_system,
    )
    .expect("CP314 state before rejected calls");

    let mut forged = output.calculation_cooling_oa_max_flow_gate;
    forged.parent_call_ordinal += 1;
    assert!(
        advance_direct_no_oa_calc_cooling_oa_max_flow_body(
            &mut runtime,
            binding.system,
            output.initialization,
            forged,
        )
        .is_err()
    );
    assert!(
        advance_direct_no_oa_calc_cooling_oa_max_flow_body(
            &mut runtime,
            binding.system,
            output.initialization,
            output.calculation_cooling_oa_max_flow_gate,
        )
        .is_err()
    );
    assert_eq!(
        purchased_air_calc_cooling_oa_max_flow_body_lifecycle_summary(
            &runtime,
            binding.ideal_loads_air_system,
        )
        .expect("CP314 state after rejected calls"),
        before
    );
}

#[test]
fn public_cooling_oa_max_flow_body_rejects_retained_and_supplied_negative_zero_gate_forgery() {
    let (model, cache) = fixture(|typed| {
        let system = &mut typed.ideal_loads_air_systems[0];
        system.cooling_limit = IdealLoadsLimit::LimitFlowRate;
        system.maximum_cooling_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Value(0.25));
    });
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
    .expect("source-ordered cooling coupling");
    assert_eq!(
        output
            .calculation_cooling_oa_max_flow_gate
            .outdoor_air_mass_flow_rate_kg_per_s,
        Some(0.0)
    );

    let unit = runtime
        .units
        .get_mut(&binding.ideal_loads_air_system)
        .expect("selected runtime unit");
    unit.calc_cooling_oa_max_flow_body =
        PurchasedAirCalcCoolingOaMaxFlowBodyRuntimeState::new(binding.ideal_loads_air_system);
    let mut forged = output.calculation_cooling_oa_max_flow_gate;
    forged.outdoor_air_mass_flow_rate_kg_per_s = Some(-0.0);
    unit.calc_cooling_oa_max_flow_gate.latest = Some(forged);
    let before = purchased_air_calc_cooling_oa_max_flow_body_lifecycle_summary(
        &runtime,
        binding.ideal_loads_air_system,
    )
    .expect("reset CP314 lifecycle");

    assert!(
        advance_direct_no_oa_calc_cooling_oa_max_flow_body(
            &mut runtime,
            binding.system,
            output.initialization,
            forged,
        )
        .is_err()
    );
    assert_eq!(
        purchased_air_calc_cooling_oa_max_flow_body_lifecycle_summary(
            &runtime,
            binding.ideal_loads_air_system,
        )
        .expect("CP314 state after rejected gate forgery"),
        before
    );
}
