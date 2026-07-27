use super::*;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingEconomizerBodyError, PurchasedAirCalcCoolingEconomizerBodyRuntimeState,
    advance_direct_no_oa_calc_cooling_economizer_body,
    purchased_air_calc_cooling_economizer_body_lifecycle_summary,
};

#[test]
fn scheduled_binding_orders_cooling_economizer_body_after_cp316_before_numerical_calc() {
    for independent_load_w in [0.0, 3_000.0] {
        let (model, cache) = fixture(|_| {});
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
        .expect("source-ordered CP317 coupling");

        let predecessor = output.calculation_cooling_economizer_condition;
        let body = output.calculation_cooling_economizer_body;
        assert_eq!(body.parent_call_ordinal, predecessor.parent_call_ordinal);
        assert_eq!(body.system, predecessor.system);
        assert_eq!(body.controlled_zone, predecessor.controlled_zone);
        assert_eq!(
            body.predecessor_economizer_condition_evaluated,
            predecessor.economizer_condition_evaluated
        );
        assert_eq!(
            body.predecessor_economizer_calculation_body_entered,
            predecessor.economizer_calculation_body_entered
        );
        assert_eq!(
            body.no_economizer_outer_guard_fallthrough_skipped,
            predecessor.no_economizer_outer_guard_fallthrough_skipped
        );
        assert!(!body.economizer_calculation_body_executed);
        assert_body_has_no_public_direct_evidence(body);

        let lifecycle = purchased_air_calc_cooling_economizer_body_lifecycle_summary(
            &runtime,
            binding.ideal_loads_air_system,
        )
        .expect("CP317 lifecycle");
        assert_eq!(lifecycle.state.transition_count, 1);
        assert_eq!(lifecycle.state.body_execution_count, 0);
        assert_eq!(
            lifecycle
                .state
                .no_economizer_outer_guard_fallthrough_skip_count,
            usize::from(predecessor.no_economizer_outer_guard_fallthrough_skipped,)
        );
        assert_eq!(lifecycle.state.psychrometric_cp_air_evaluation_count, 0);
        assert_eq!(lifecycle.state.economizer_on_assignment_count, 0);
    }
}

#[test]
fn public_cooling_economizer_body_rejects_forgery_replay_and_overflow_without_mutation() {
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
    .expect("source-ordered CP317 coupling");

    let before_replay = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_economizer_body(
            &mut runtime,
            binding.system,
            output.calculation_cooling_economizer_condition,
        )
        .is_err()
    );
    assert_eq!(runtime, before_replay);

    let state = &mut runtime
        .units
        .get_mut(&binding.ideal_loads_air_system)
        .expect("selected unit")
        .calc_cooling_economizer_body;
    *state = PurchasedAirCalcCoolingEconomizerBodyRuntimeState::new(binding.ideal_loads_air_system);
    state.body_execution_count = 1;
    let before_corruption = runtime.clone();
    assert_eq!(
        advance_direct_no_oa_calc_cooling_economizer_body(
            &mut runtime,
            binding.system,
            output.calculation_cooling_economizer_condition,
        ),
        Err(
            PurchasedAirCalcCoolingEconomizerBodyError::RuntimeStateInvariantViolation {
                system: binding.ideal_loads_air_system,
            }
        )
    );
    assert_eq!(runtime, before_corruption);

    let state = &mut runtime
        .units
        .get_mut(&binding.ideal_loads_air_system)
        .expect("selected unit")
        .calc_cooling_economizer_body;
    *state = PurchasedAirCalcCoolingEconomizerBodyRuntimeState::new(binding.ideal_loads_air_system);
    state.transition_count = usize::MAX;
    let before_overflow = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_economizer_body(
            &mut runtime,
            binding.system,
            output.calculation_cooling_economizer_condition,
        )
        .is_err()
    );
    assert_eq!(runtime, before_overflow);
}

fn assert_body_has_no_public_direct_evidence(
    body: crate::ideal_loads::PurchasedAirCalcCoolingEconomizerBodySnapshot,
) {
    assert!(!body.zone_humidity_ratio_read);
    assert_eq!(body.zone_humidity_ratio, None);
    assert!(!body.psychrometric_cp_air_evaluated);
    assert!(!body.cp_air_assigned);
    assert_eq!(body.cp_air_j_per_kg_k, None);
    assert!(!body.outdoor_air_temperature_read);
    assert_eq!(body.outdoor_air_temperature_c, None);
    assert!(!body.zone_temperature_read);
    assert_eq!(body.zone_temperature_c, None);
    assert!(!body.delta_temperature_calculated);
    assert!(!body.delta_temperature_assigned);
    assert!(!body.delta_temperature_for_gate_read);
    assert!(!body.delta_temperature_body_entered);
    assert_eq!(body.delta_temperature_c, None);
    assert!(!body.zone_cooling_setpoint_load_read);
    assert!(!body.supply_mass_flow_rate_calculated);
    assert!(!body.cp_air_for_first_division_read);
    assert!(!body.delta_temperature_for_second_division_read);
    assert!(!body.initial_supply_mass_flow_rate_assigned);
    assert!(!body.cooling_limit_flow_rate_comparison_evaluated);
    assert!(!body.maximum_cooling_air_mass_flow_rate_read);
    assert!(!body.maximum_cooling_air_mass_flow_rate_clamp_upper_bound_read);
    assert!(!body.maximum_flow_clamp_body_entered);
    assert!(!body.supply_mass_flow_rate_for_clamp_read);
    assert!(!body.inner_max_evaluated);
    assert!(!body.supply_mass_flow_rate_clamped);
    assert!(!body.outer_min_evaluated);
    assert!(!body.clamped_supply_mass_flow_rate_assigned);
    assert!(!body.resulting_supply_mass_flow_rate_read);
    assert!(!body.outdoor_air_mass_flow_rate_read);
    assert!(!body.economizer_on_assigned);
    assert!(!body.economizer_activation_body_entered);
    assert!(!body.supply_mass_flow_rate_for_outdoor_air_assignment_read);
    assert!(!body.system_time_step_read);
    assert!(!body.economizer_active_time_assigned);
}
