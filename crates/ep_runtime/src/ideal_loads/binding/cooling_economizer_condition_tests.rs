use super::*;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingEconomizerConditionRuntimeState,
    advance_direct_no_oa_calc_cooling_economizer_condition,
    purchased_air_calc_cooling_economizer_condition_lifecycle_summary,
};

#[test]
fn scheduled_binding_orders_cooling_economizer_condition_after_cp315_before_numerical_calc() {
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
        .expect("source-ordered CP316 coupling");

        let predecessor = output.calculation_cooling_economizer_guard;
        let condition = output.calculation_cooling_economizer_condition;
        assert_eq!(
            condition.parent_call_ordinal,
            predecessor.parent_call_ordinal
        );
        assert_eq!(condition.system, predecessor.system);
        assert_eq!(condition.controlled_zone, predecessor.controlled_zone);
        assert_eq!(
            condition.predecessor_economizer_guard_evaluated,
            predecessor.economizer_guard_evaluated
        );
        assert_eq!(
            condition.predecessor_economizer_body_entered,
            predecessor.economizer_body_entered
        );
        assert_eq!(
            condition.predecessor_no_economizer_fallthrough,
            predecessor.no_economizer_fallthrough
        );
        assert_eq!(
            condition.no_economizer_outer_guard_fallthrough_skipped,
            predecessor.no_economizer_fallthrough
        );
        assert!(!condition.economizer_condition_evaluated);
        assert_condition_has_no_public_direct_evidence(condition);
        if output.coupling.purchased_air.calculation.mode == IdealLoadsSensibleMode::Cooling {
            assert!(predecessor.no_economizer_fallthrough);
            assert!(condition.no_economizer_outer_guard_fallthrough_skipped);
        }

        let lifecycle = purchased_air_calc_cooling_economizer_condition_lifecycle_summary(
            &runtime,
            binding.ideal_loads_air_system,
        )
        .expect("CP316 lifecycle");
        assert_eq!(lifecycle.state.transition_count, 1);
        assert_eq!(lifecycle.state.condition_evaluation_count, 0);
        assert_eq!(
            lifecycle
                .state
                .no_economizer_outer_guard_fallthrough_skip_count,
            usize::from(predecessor.no_economizer_fallthrough)
        );
        assert_eq!(lifecycle.state.economizer_calculation_body_entry_count, 0);
        assert_eq!(lifecycle.state.economizer_condition_fallthrough_count, 0);
    }
}

#[test]
fn public_cooling_economizer_condition_rejects_forgery_replay_and_overflow_without_mutation() {
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
    .expect("source-ordered CP316 coupling");

    let before = runtime.clone();
    let mut forged = output.calculation_cooling_economizer_guard;
    forged.parent_call_ordinal += 1;
    assert!(
        advance_direct_no_oa_calc_cooling_economizer_condition(
            &mut runtime,
            binding.system,
            forged,
        )
        .is_err()
    );
    assert_eq!(runtime, before);

    let before_replay = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_economizer_condition(
            &mut runtime,
            binding.system,
            output.calculation_cooling_economizer_guard,
        )
        .is_err()
    );
    assert_eq!(runtime, before_replay);

    let state = &mut runtime
        .units
        .get_mut(&binding.ideal_loads_air_system)
        .expect("selected unit")
        .calc_cooling_economizer_condition;
    *state =
        PurchasedAirCalcCoolingEconomizerConditionRuntimeState::new(binding.ideal_loads_air_system);
    state.condition_evaluation_count = 1;
    let before_corruption = runtime.clone();
    assert_eq!(
        advance_direct_no_oa_calc_cooling_economizer_condition(
            &mut runtime,
            binding.system,
            output.calculation_cooling_economizer_guard,
        ),
        Err(
            PurchasedAirCalcCoolingEconomizerConditionError::RuntimeStateInvariantViolation {
                system: binding.ideal_loads_air_system,
            }
        )
    );
    assert_eq!(runtime, before_corruption);

    let state = &mut runtime
        .units
        .get_mut(&binding.ideal_loads_air_system)
        .expect("selected unit")
        .calc_cooling_economizer_condition;
    *state =
        PurchasedAirCalcCoolingEconomizerConditionRuntimeState::new(binding.ideal_loads_air_system);
    state.transition_count = usize::MAX;
    let before_overflow = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_economizer_condition(
            &mut runtime,
            binding.system,
            output.calculation_cooling_economizer_guard,
        )
        .is_err()
    );
    assert_eq!(runtime, before_overflow);
}

#[test]
fn public_cooling_economizer_condition_rejects_economizer_configuration_without_mutation() {
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
    .expect("source-ordered CP316 coupling");
    runtime
        .units
        .get_mut(&binding.ideal_loads_air_system)
        .expect("selected unit")
        .calc_cooling_economizer_condition =
        PurchasedAirCalcCoolingEconomizerConditionRuntimeState::new(binding.ideal_loads_air_system);
    let before = runtime.clone();

    for economizer_type in [
        OutdoorAirEconomizerType::DifferentialDryBulb,
        OutdoorAirEconomizerType::DifferentialEnthalpy,
    ] {
        let mut economizer = binding.system.clone();
        economizer.outdoor_air_economizer_type = economizer_type;
        assert_eq!(
            advance_direct_no_oa_calc_cooling_economizer_condition(
                &mut runtime,
                &economizer,
                output.calculation_cooling_economizer_guard,
            ),
            Err(
                PurchasedAirCalcCoolingEconomizerConditionError::SystemOutsideDirectSubset {
                    system: binding.ideal_loads_air_system,
                }
            )
        );
        assert_eq!(runtime, before);
    }
}

fn assert_condition_has_no_public_direct_evidence(
    condition: PurchasedAirCalcCoolingEconomizerConditionSnapshot,
) {
    assert!(!condition.differential_dry_bulb_economizer_type_read);
    assert_eq!(condition.differential_dry_bulb_economizer_type, None);
    assert!(!condition.differential_dry_bulb_selector_comparison_evaluated);
    assert_eq!(condition.differential_dry_bulb_selector_matched, None);
    assert!(!condition.outdoor_air_temperature_read);
    assert_eq!(condition.outdoor_air_temperature_c, None);
    assert!(!condition.recirculation_air_temperature_read);
    assert_eq!(condition.recirculation_air_temperature_c, None);
    assert!(!condition.dry_bulb_temperature_comparison_evaluated);
    assert_eq!(
        condition.outdoor_air_temperature_below_recirculation_temperature,
        None
    );
    assert!(!condition.differential_enthalpy_economizer_type_read);
    assert_eq!(condition.differential_enthalpy_economizer_type, None);
    assert!(!condition.differential_enthalpy_selector_comparison_evaluated);
    assert_eq!(condition.differential_enthalpy_selector_matched, None);
    assert!(!condition.outdoor_air_enthalpy_read);
    assert_eq!(condition.outdoor_air_enthalpy_j_per_kg, None);
    assert!(!condition.recirculation_air_enthalpy_read);
    assert_eq!(condition.recirculation_air_enthalpy_j_per_kg, None);
    assert!(!condition.enthalpy_comparison_evaluated);
    assert_eq!(
        condition.outdoor_air_enthalpy_below_recirculation_enthalpy,
        None
    );
    assert_eq!(condition.economizer_condition_satisfied, None);
    assert!(!condition.economizer_calculation_body_entered);
    assert!(!condition.economizer_condition_fallthrough);
}
