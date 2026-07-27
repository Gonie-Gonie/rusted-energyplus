use super::*;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingEconomizerGuardRuntimeState,
    advance_direct_no_oa_calc_cooling_economizer_guard,
    purchased_air_calc_cooling_economizer_guard_lifecycle_summary,
};

#[test]
fn scheduled_binding_orders_cooling_economizer_guard_after_cp314_before_numerical_calc() {
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
    .expect("source-ordered CP315 coupling");

    let predecessor = output.calculation_cooling_oa_max_flow_body;
    let guard = output.calculation_cooling_economizer_guard;
    assert_eq!(guard.parent_call_ordinal, predecessor.parent_call_ordinal);
    assert_eq!(guard.system, predecessor.system);
    assert_eq!(guard.controlled_zone, predecessor.controlled_zone);
    assert_eq!(
        guard.predecessor_active_guard_false_economizer_fallthrough,
        predecessor.active_guard_false_economizer_fallthrough
    );
    assert_eq!(
        guard.economizer_guard_evaluated,
        output.coupling.purchased_air.calculation.mode == IdealLoadsSensibleMode::Cooling
    );
    if guard.economizer_guard_evaluated {
        assert!(guard.economizer_type_read);
        assert_eq!(
            guard.economizer_type,
            Some(OutdoorAirEconomizerType::NoEconomizer)
        );
        assert_eq!(guard.economizer_not_no_economizer, Some(false));
        assert!(!guard.economizer_body_entered);
        assert!(guard.no_economizer_fallthrough);
    } else {
        assert!(!guard.economizer_type_read);
        assert_eq!(guard.economizer_type, None);
        assert!(!guard.economizer_body_entered);
        assert!(!guard.no_economizer_fallthrough);
    }

    let lifecycle = purchased_air_calc_cooling_economizer_guard_lifecycle_summary(
        &runtime,
        binding.ideal_loads_air_system,
    )
    .expect("CP315 lifecycle");
    assert_eq!(lifecycle.state.transition_count, 1);
    assert_eq!(
        lifecycle.state.guard_evaluation_count,
        usize::from(guard.economizer_guard_evaluated)
    );
    assert_eq!(lifecycle.state.economizer_body_entry_count, 0);
}

#[test]
fn public_cooling_economizer_guard_rejects_forgery_replay_and_overflow_without_mutation() {
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
    .expect("source-ordered CP315 coupling");

    runtime
        .units
        .get_mut(&binding.ideal_loads_air_system)
        .expect("selected unit")
        .calc_cooling_economizer_guard =
        PurchasedAirCalcCoolingEconomizerGuardRuntimeState::new(binding.ideal_loads_air_system);
    let before = purchased_air_calc_cooling_economizer_guard_lifecycle_summary(
        &runtime,
        binding.ideal_loads_air_system,
    )
    .expect("reset CP315 lifecycle");

    let mut forged = output.calculation_cooling_oa_max_flow_body;
    forged.parent_call_ordinal += 1;
    assert!(
        advance_direct_no_oa_calc_cooling_economizer_guard(&mut runtime, binding.system, forged,)
            .is_err()
    );
    assert_eq!(
        purchased_air_calc_cooling_economizer_guard_lifecycle_summary(
            &runtime,
            binding.ideal_loads_air_system,
        )
        .expect("CP315 state after forgery"),
        before
    );

    advance_direct_no_oa_calc_cooling_economizer_guard(
        &mut runtime,
        binding.system,
        output.calculation_cooling_oa_max_flow_body,
    )
    .expect("exact CP315 transition");
    let after_success = purchased_air_calc_cooling_economizer_guard_lifecycle_summary(
        &runtime,
        binding.ideal_loads_air_system,
    )
    .expect("CP315 state after success");
    assert!(
        advance_direct_no_oa_calc_cooling_economizer_guard(
            &mut runtime,
            binding.system,
            output.calculation_cooling_oa_max_flow_body,
        )
        .is_err()
    );
    assert_eq!(
        purchased_air_calc_cooling_economizer_guard_lifecycle_summary(
            &runtime,
            binding.ideal_loads_air_system,
        )
        .expect("CP315 state after replay"),
        after_success
    );

    let state = &mut runtime
        .units
        .get_mut(&binding.ideal_loads_air_system)
        .expect("selected unit")
        .calc_cooling_economizer_guard;
    *state =
        PurchasedAirCalcCoolingEconomizerGuardRuntimeState::new(binding.ideal_loads_air_system);
    state.guard_evaluation_count = 1;
    let before_corruption = purchased_air_calc_cooling_economizer_guard_lifecycle_summary(
        &runtime,
        binding.ideal_loads_air_system,
    )
    .expect("forged retained-state lifecycle");
    assert_eq!(
        advance_direct_no_oa_calc_cooling_economizer_guard(
            &mut runtime,
            binding.system,
            output.calculation_cooling_oa_max_flow_body,
        ),
        Err(
            PurchasedAirCalcCoolingEconomizerGuardError::RuntimeStateInvariantViolation {
                system: binding.ideal_loads_air_system,
            }
        )
    );
    assert_eq!(
        purchased_air_calc_cooling_economizer_guard_lifecycle_summary(
            &runtime,
            binding.ideal_loads_air_system,
        )
        .expect("CP315 state after retained-state rejection"),
        before_corruption
    );

    let state = &mut runtime
        .units
        .get_mut(&binding.ideal_loads_air_system)
        .expect("selected unit")
        .calc_cooling_economizer_guard;
    *state =
        PurchasedAirCalcCoolingEconomizerGuardRuntimeState::new(binding.ideal_loads_air_system);
    state.transition_count = usize::MAX;
    let before_overflow = purchased_air_calc_cooling_economizer_guard_lifecycle_summary(
        &runtime,
        binding.ideal_loads_air_system,
    )
    .expect("forged overflow lifecycle");
    assert!(
        advance_direct_no_oa_calc_cooling_economizer_guard(
            &mut runtime,
            binding.system,
            output.calculation_cooling_oa_max_flow_body,
        )
        .is_err()
    );
    assert_eq!(
        purchased_air_calc_cooling_economizer_guard_lifecycle_summary(
            &runtime,
            binding.ideal_loads_air_system,
        )
        .expect("CP315 state after overflow"),
        before_overflow
    );
}

#[test]
fn public_cooling_economizer_guard_rejects_economizer_configuration_without_mutation() {
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
    .expect("source-ordered CP315 coupling");
    runtime
        .units
        .get_mut(&binding.ideal_loads_air_system)
        .expect("selected unit")
        .calc_cooling_economizer_guard =
        PurchasedAirCalcCoolingEconomizerGuardRuntimeState::new(binding.ideal_loads_air_system);
    let before = runtime.clone();

    for economizer_type in [
        OutdoorAirEconomizerType::DifferentialDryBulb,
        OutdoorAirEconomizerType::DifferentialEnthalpy,
    ] {
        let mut economizer = binding.system.clone();
        economizer.outdoor_air_economizer_type = economizer_type;
        assert_eq!(
            advance_direct_no_oa_calc_cooling_economizer_guard(
                &mut runtime,
                &economizer,
                output.calculation_cooling_oa_max_flow_body,
            ),
            Err(
                PurchasedAirCalcCoolingEconomizerGuardError::SystemOutsideDirectSubset {
                    system: binding.ideal_loads_air_system,
                }
            )
        );
        assert_eq!(runtime, before);
    }
}
