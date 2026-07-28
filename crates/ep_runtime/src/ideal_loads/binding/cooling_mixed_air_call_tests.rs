use super::*;
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_NO_OA_CHILD_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallRuntimeState,
    advance_direct_no_oa_calc_cooling_mixed_air_call,
    cooling_mixed_air_call_snapshot_is_exact_direct_release, moist_air_enthalpy_j_per_kg,
    purchased_air_calc_cooling_mixed_air_call_lifecycle_summary,
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
    let recirculation_temperature_c = zone_state.mean_air_temperature_c;
    let recirculation_humidity_ratio = zone_state.air_humidity_ratio;
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
    .expect("source-ordered CP329 coupling");
    (
        runtime,
        output,
        recirculation_temperature_c,
        recirculation_humidity_ratio,
    )
}

#[test]
fn scheduled_binding_executes_the_nine_site_call_and_exact_no_oa_child() {
    for (limit, capacity) in [
        (IdealLoadsLimit::NoLimit, None),
        (IdealLoadsLimit::LimitCapacity, Some(0.0)),
    ] {
        let (runtime, output, recirculation_temperature_c, recirculation_humidity_ratio) =
            run_case(limit, capacity, 3_000.0, 1.0);
        let predecessor = output.calculation_cooling_supply_mass_flow_very_small_guard_body;
        let call = output.calculation_cooling_mixed_air_call;

        assert!(cooling_mixed_air_call_snapshot_is_exact_direct_release(
            call
        ));
        assert_eq!(call.system, predecessor.system);
        assert_eq!(call.parent_call_ordinal, predecessor.parent_call_ordinal);
        assert_eq!(call.controlled_zone, predecessor.controlled_zone);
        assert_eq!(
            call.predecessor_zero_flow_reset_body_entered,
            predecessor.zero_flow_reset_body_entered
        );
        assert_eq!(
            call.predecessor_active_guard_false_fallthrough,
            predecessor.active_guard_false_fallthrough
        );
        assert!(call.cooling_call_executed);
        assert!(call.state_reference_bound);
        assert!(call.purchased_air_number_read);
        assert!(call.outdoor_air_mass_flow_rate_read);
        assert!(call.supply_mass_flow_rate_read);
        assert!(call.mixed_air_temperature_output_reference_bound);
        assert!(call.mixed_air_humidity_ratio_output_reference_bound);
        assert!(call.mixed_air_enthalpy_output_reference_bound);
        assert!(call.operating_mode_read);
        assert!(call.calc_purch_air_mixed_air_called);
        assert_eq!(
            call.outdoor_air_mass_flow_rate_kg_per_s.map(f64::to_bits),
            Some(0)
        );
        assert_eq!(
            call.supply_mass_flow_rate_kg_per_s.map(f64::to_bits),
            predecessor
                .resulting_supply_mass_flow_rate_kg_per_s
                .map(f64::to_bits)
        );
        assert_eq!(call.recirculation_node, Some(NodeId(2)));
        assert_eq!(
            call.recirculation_temperature_c.map(f64::to_bits),
            Some(recirculation_temperature_c.to_bits())
        );
        assert_eq!(
            call.recirculation_humidity_ratio.map(f64::to_bits),
            Some(recirculation_humidity_ratio.to_bits())
        );
        let enthalpy_projection =
            moist_air_enthalpy_j_per_kg(recirculation_temperature_c, recirculation_humidity_ratio);
        assert_eq!(
            call.recirculation_enthalpy_projection_j_per_kg
                .map(f64::to_bits),
            Some(enthalpy_projection.to_bits())
        );
        assert_eq!(
            call.mixed_air_temperature_c.map(f64::to_bits),
            call.recirculation_temperature_c.map(f64::to_bits)
        );
        assert_eq!(
            call.mixed_air_humidity_ratio.map(f64::to_bits),
            call.recirculation_humidity_ratio.map(f64::to_bits)
        );
        assert_eq!(
            call.mixed_air_enthalpy_projection_j_per_kg
                .map(f64::to_bits),
            call.recirculation_enthalpy_projection_j_per_kg
                .map(f64::to_bits)
        );
        assert_eq!(
            call.resulting_recirculation_mass_flow_rate_kg_per_s
                .map(f64::to_bits),
            call.supply_mass_flow_rate_kg_per_s.map(f64::to_bits)
        );
        assert_eq!(
            call.heat_recovery_sensible_output_w.map(f64::to_bits),
            Some(0)
        );
        assert_eq!(
            call.heat_recovery_latent_output_w.map(f64::to_bits),
            Some(0)
        );

        let state = purchased_air_calc_cooling_mixed_air_call_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP329 lifecycle")
        .state;
        assert_active_counter_shape(&state);
    }
}

fn assert_active_counter_shape(state: &PurchasedAirCalcCoolingMixedAirCallRuntimeState) {
    assert_eq!(state.transition_count, 1);
    assert_eq!(state.cooling_call_count, 1);
    assert_eq!(state.unit_off_skip_count, 0);
    assert_eq!(state.non_cooling_skip_count, 0);
    assert_eq!(
        state.caller_source_site_execution_count,
        PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER.len()
    );
    assert_eq!(
        state.child_source_site_execution_count,
        PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_NO_OA_CHILD_SOURCE_ORDER.len()
    );
    assert_eq!(state.state_reference_bind_count, 1);
    assert_eq!(state.purchased_air_number_read_count, 1);
    assert_eq!(state.outdoor_air_mass_flow_rate_read_count, 1);
    assert_eq!(state.supply_mass_flow_rate_read_count, 1);
    assert_eq!(state.mixed_air_output_reference_bind_count, 3);
    assert_eq!(state.operating_mode_read_count, 1);
    assert_eq!(state.mixed_air_child_call_count, 1);
    assert_eq!(state.no_outdoor_air_fallback_count, 1);
    assert_eq!(state.recirculation_enthalpy_projection_count, 1);
    assert_eq!(state.mixed_air_output_assignment_count, 3);
    assert_eq!(state.heat_recovery_output_positive_zero_assignment_count, 2);
}

#[test]
fn scheduled_binding_skips_every_cp329_site_when_cooling_is_inactive() {
    for (availability, independent_load_w, unit_off, non_cooling) in
        [(0.0, 3_000.0, true, false), (1.0, 0.0, false, true)]
    {
        let (runtime, output, _, _) = run_case(
            IdealLoadsLimit::NoLimit,
            None,
            independent_load_w,
            availability,
        );
        let call = output.calculation_cooling_mixed_air_call;

        assert!(cooling_mixed_air_call_snapshot_is_exact_direct_release(
            call
        ));
        assert_eq!(call.unit_off_skipped, unit_off);
        assert_eq!(call.non_cooling_skipped, non_cooling);
        assert!(!call.cooling_call_executed);
        assert!(!call.calc_purch_air_mixed_air_called);
        assert!(call.supply_mass_flow_rate_kg_per_s.is_none());
        assert!(call.recirculation_temperature_c.is_none());
        assert!(call.recirculation_enthalpy_projection_j_per_kg.is_none());
        assert!(call.mixed_air_temperature_c.is_none());

        let state = purchased_air_calc_cooling_mixed_air_call_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP329 skip lifecycle")
        .state;
        assert_eq!(state.transition_count, 1);
        assert_eq!(state.cooling_call_count, 0);
        assert_eq!(state.unit_off_skip_count, usize::from(unit_off));
        assert_eq!(state.non_cooling_skip_count, usize::from(non_cooling));
        assert_eq!(state.caller_source_site_execution_count, 0);
        assert_eq!(state.child_source_site_execution_count, 0);
        assert_eq!(state.recirculation_enthalpy_projection_count, 0);
    }
}

#[test]
fn public_cp329_release_rejects_replay_and_forged_cp328_ordinal_without_mutation() {
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
    .expect("completed CP329 release");
    let predecessor = output.calculation_cooling_supply_mass_flow_very_small_guard_body;

    let before_replay = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_mixed_air_call(
            &mut runtime,
            binding.system,
            predecessor,
            &zone_state,
        )
        .is_err()
    );
    assert_eq!(runtime, before_replay);

    let mut forged = predecessor;
    forged.parent_call_ordinal += 1;
    let before_forgery = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_mixed_air_call(
            &mut runtime,
            binding.system,
            forged,
            &zone_state,
        )
        .is_err()
    );
    assert_eq!(runtime, before_forgery);
}
