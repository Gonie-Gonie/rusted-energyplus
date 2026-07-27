use super::*;
use crate::ideal_loads::calc::cooling_sensible_flow_snapshot_is_exact_direct_release;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSensibleFlowRuntimeState,
    advance_direct_no_oa_calc_cooling_sensible_flow,
    purchased_air_calc_cooling_sensible_flow_lifecycle_summary,
};

#[test]
fn scheduled_binding_executes_cooling_sensible_flow_before_numerical_coupling() {
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
    .expect("source-ordered CP318 cooling coupling");

    assert_eq!(
        output.coupling.purchased_air.calculation.mode,
        IdealLoadsSensibleMode::Cooling
    );
    let predecessor = output.calculation_cooling_economizer_body;
    let flow = output.calculation_cooling_sensible_flow;
    assert!(cooling_sensible_flow_snapshot_is_exact_direct_release(flow));
    assert_eq!(flow.parent_call_ordinal, predecessor.parent_call_ordinal);
    assert_eq!(flow.system, predecessor.system);
    assert_eq!(flow.controlled_zone, predecessor.controlled_zone);
    assert_eq!(
        flow.predecessor_economizer_calculation_body_executed,
        predecessor.economizer_calculation_body_executed
    );
    assert!(flow.cooling_body_entered);
    assert!(flow.supply_mass_flow_rate_for_cool_reset_assigned);
    assert_eq!(
        flow.reset_supply_mass_flow_rate_for_cool_kg_per_s
            .expect("cooling reset")
            .to_bits(),
        0.0_f64.to_bits()
    );
    assert!(flow.cooling_on_read);
    assert_eq!(flow.cooling_on, Some(true));
    assert!(flow.cooling_on_body_entered);
    assert!(flow.psychrometric_cp_air_evaluated);
    assert!(flow.delta_temperature_comparison_evaluated);
    assert_eq!(
        flow.delta_temperature_below_negative_small_temp_diff,
        Some(true)
    );
    assert!(flow.delta_temperature_body_entered);
    assert_eq!(
        flow.zone_cooling_setpoint_load_w
            .expect("source cooling demand")
            .to_bits(),
        output
            .calculation_entry
            .demand
            .remaining_output_req_to_cool_sp_w
            .to_bits()
    );
    let cp_air = flow
        .cp_air_for_first_division_j_per_kg_k
        .expect("first source divisor");
    let delta_temperature = flow
        .delta_temperature_for_second_division_c
        .expect("second source divisor");
    let expected_flow = (output
        .calculation_entry
        .demand
        .remaining_output_req_to_cool_sp_w
        / cp_air)
        / delta_temperature;
    let assigned_flow = flow
        .assigned_supply_mass_flow_rate_for_cool_kg_per_s
        .expect("source cooling-flow assignment");
    assert_eq!(assigned_flow.to_bits(), expected_flow.to_bits());
    assert!(assigned_flow > 0.0);

    let lifecycle = purchased_air_calc_cooling_sensible_flow_lifecycle_summary(
        &runtime,
        binding.ideal_loads_air_system,
    )
    .expect("CP318 lifecycle");
    let state = lifecycle.state;
    assert_eq!(state.transition_count, 1);
    assert_eq!(state.cooling_body_entry_count, 1);
    assert_eq!(state.unit_off_skip_count, 0);
    assert_eq!(state.non_cooling_skip_count, 0);
    assert_eq!(
        state.supply_mass_flow_rate_for_cool_reset_assignment_count,
        1
    );
    assert_eq!(state.cooling_on_read_count, 1);
    assert_eq!(state.cooling_on_body_entry_count, 1);
    assert_eq!(state.delta_temperature_body_entry_count, 1);
    assert_eq!(state.supply_mass_flow_rate_for_cool_assignment_count, 1);
}

#[test]
fn scheduled_binding_records_unit_off_and_non_cooling_cp318_skips() {
    for (availability, independent_load_w, unit_off, non_cooling) in
        [(0.0, 3_000.0, true, false), (1.0, 0.0, false, true)]
    {
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
        .expect("source-ordered CP318 skip coupling");

        let flow = output.calculation_cooling_sensible_flow;
        assert!(cooling_sensible_flow_snapshot_is_exact_direct_release(flow));
        assert_eq!(flow.unit_off_skipped, unit_off);
        assert_eq!(flow.non_cooling_skipped, non_cooling);
        assert!(!flow.cooling_body_entered);
        assert!(!flow.supply_mass_flow_rate_for_cool_reset_assigned);
        assert_eq!(flow.reset_supply_mass_flow_rate_for_cool_kg_per_s, None);
        assert!(!flow.cooling_on_read);
        assert_eq!(flow.cooling_on, None);
        assert!(!flow.cooling_on_body_entered);
        assert_eq!(flow.resulting_supply_mass_flow_rate_for_cool_kg_per_s, None);

        let lifecycle = purchased_air_calc_cooling_sensible_flow_lifecycle_summary(
            &runtime,
            binding.ideal_loads_air_system,
        )
        .expect("CP318 skip lifecycle");
        assert_eq!(lifecycle.state.transition_count, 1);
        assert_eq!(lifecycle.state.unit_off_skip_count, usize::from(unit_off));
        assert_eq!(
            lifecycle.state.non_cooling_skip_count,
            usize::from(non_cooling)
        );
        assert_eq!(lifecycle.state.cooling_body_entry_count, 0);
        assert_eq!(
            lifecycle
                .state
                .supply_mass_flow_rate_for_cool_reset_assignment_count,
            0
        );
        assert_eq!(lifecycle.state.cooling_on_read_count, 0);
        assert_eq!(
            lifecycle
                .state
                .supply_mass_flow_rate_for_cool_assignment_count,
            0
        );
    }
}

#[test]
fn public_cooling_sensible_flow_replay_is_whole_state_transactional() {
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
    .expect("source-ordered CP318 call");

    let before_runtime = runtime.clone();
    let before_zone = zone_state.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_sensible_flow(
            &mut runtime,
            binding.system,
            output.calculation_cooling_economizer_body,
            &zone_state,
        )
        .is_err()
    );
    assert_eq!(runtime, before_runtime);
    assert_eq!(zone_state, before_zone);

    let flow_state = &mut runtime
        .units
        .get_mut(&binding.ideal_loads_air_system)
        .expect("selected unit")
        .calc_cooling_sensible_flow;
    *flow_state =
        PurchasedAirCalcCoolingSensibleFlowRuntimeState::new(binding.ideal_loads_air_system);
    flow_state.transition_count = usize::MAX;
    let before_runtime = runtime.clone();
    let before_zone = zone_state.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_sensible_flow(
            &mut runtime,
            binding.system,
            output.calculation_cooling_economizer_body,
            &zone_state,
        )
        .is_err()
    );
    assert_eq!(runtime, before_runtime);
    assert_eq!(zone_state, before_zone);
}
