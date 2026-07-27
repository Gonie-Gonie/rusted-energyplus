use super::*;
use crate::ideal_loads::calc::cooling_humidification_flow_snapshot_is_exact_direct_release;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingHumidificationFlowRuntimeState,
    advance_direct_no_oa_calc_cooling_humidification_flow,
    purchased_air_calc_cooling_humidification_flow_lifecycle_summary,
};

#[test]
fn scheduled_binding_executes_cooling_humidification_flow_before_numerical_coupling() {
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
    .expect("source-ordered CP320 cooling coupling");

    assert_eq!(
        output.coupling.purchased_air.calculation.mode,
        IdealLoadsSensibleMode::Cooling
    );
    let predecessor = output.calculation_cooling_dehumidification_flow;
    let flow = output.calculation_cooling_humidification_flow;
    assert!(cooling_humidification_flow_snapshot_is_exact_direct_release(flow));
    assert_eq!(flow.parent_call_ordinal, predecessor.parent_call_ordinal);
    assert_eq!(flow.system, predecessor.system);
    assert_eq!(flow.controlled_zone, predecessor.controlled_zone);
    assert_eq!(
        flow.predecessor_cooling_body_entered,
        predecessor.cooling_body_entered
    );
    assert!(flow.cooling_body_entered);
    assert!(flow.supply_mass_flow_rate_for_humidification_reset_assigned);
    assert_eq!(
        flow.reset_supply_mass_flow_rate_for_humidification_kg_per_s
            .expect("humidification reset")
            .to_bits(),
        0.0_f64.to_bits()
    );
    assert!(flow.heating_on_read);
    assert_eq!(flow.heating_on, Some(true));
    assert!(flow.heating_on_body_entered);
    assert!(flow.humidification_control_type_read);
    assert_eq!(
        flow.humidification_control_type,
        Some(HumidificationControlType::None)
    );
    assert_eq!(flow.humidification_control_type_humidistat, Some(false));
    assert!(!flow.humidification_control_body_entered);
    assert!(!flow.dehumidification_control_type_first_read);
    assert!(!flow.dehumidification_control_type_second_read);
    assert!(!flow.zone_humidifying_setpoint_moisture_demand_read);
    assert!(!flow.maximum_heating_supply_air_humidity_ratio_read);
    assert!(!flow.zone_humidity_ratio_read);
    assert!(!flow.delta_humidity_ratio_calculated);
    assert!(!flow.humidification_flow_body_entered);
    assert!(!flow.supply_mass_flow_rate_for_humidification_assigned);
    assert_eq!(
        flow.resulting_supply_mass_flow_rate_for_humidification_kg_per_s
            .expect("reset humidification candidate")
            .to_bits(),
        0.0_f64.to_bits()
    );

    let lifecycle = purchased_air_calc_cooling_humidification_flow_lifecycle_summary(
        &runtime,
        binding.ideal_loads_air_system,
    )
    .expect("CP320 lifecycle");
    let state = lifecycle.state;
    assert_eq!(state.transition_count, 1);
    assert_eq!(state.cooling_body_entry_count, 1);
    assert_eq!(state.reset_assignment_count, 1);
    assert_eq!(state.heating_on_read_count, 1);
    assert_eq!(state.heating_on_body_entry_count, 1);
    assert_eq!(state.humidification_control_type_read_count, 1);
    assert_eq!(state.humidification_control_type_humidistat_count, 0);
    assert_eq!(state.humidification_control_type_fallthrough_count, 1);
    assert_eq!(state.dehumidification_control_type_first_read_count, 0);
    assert_eq!(state.moisture_demand_read_count, 0);
    assert_eq!(state.assignment_count, 0);
}

#[test]
fn scheduled_binding_records_unit_off_and_non_cooling_cp320_skips() {
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
        .expect("source-ordered CP320 skip coupling");

        let flow = output.calculation_cooling_humidification_flow;
        assert!(cooling_humidification_flow_snapshot_is_exact_direct_release(flow));
        assert_eq!(flow.unit_off_skipped, unit_off);
        assert_eq!(flow.non_cooling_skipped, non_cooling);
        assert!(!flow.cooling_body_entered);
        assert!(!flow.supply_mass_flow_rate_for_humidification_reset_assigned);
        assert_eq!(
            flow.reset_supply_mass_flow_rate_for_humidification_kg_per_s,
            None
        );
        assert!(!flow.heating_on_read);
        assert_eq!(flow.heating_on, None);
        assert!(!flow.humidification_control_type_read);
        assert_eq!(flow.humidification_control_type, None);
        assert_eq!(
            flow.resulting_supply_mass_flow_rate_for_humidification_kg_per_s,
            None
        );

        let lifecycle = purchased_air_calc_cooling_humidification_flow_lifecycle_summary(
            &runtime,
            binding.ideal_loads_air_system,
        )
        .expect("CP320 skip lifecycle");
        assert_eq!(lifecycle.state.transition_count, 1);
        assert_eq!(lifecycle.state.unit_off_skip_count, usize::from(unit_off));
        assert_eq!(
            lifecycle.state.non_cooling_skip_count,
            usize::from(non_cooling)
        );
        assert_eq!(lifecycle.state.cooling_body_entry_count, 0);
        assert_eq!(lifecycle.state.reset_assignment_count, 0);
        assert_eq!(lifecycle.state.heating_on_read_count, 0);
        assert_eq!(lifecycle.state.humidification_control_type_read_count, 0);
    }
}

#[test]
fn public_cooling_humidification_flow_replay_is_whole_state_transactional() {
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
    .expect("source-ordered CP320 call");

    let before_runtime = runtime.clone();
    let before_zone = zone_state.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_humidification_flow(
            &mut runtime,
            binding.system,
            output.calculation_cooling_dehumidification_flow,
        )
        .is_err()
    );
    assert_eq!(runtime, before_runtime);
    assert_eq!(zone_state, before_zone);

    let flow_state = &mut runtime
        .units
        .get_mut(&binding.ideal_loads_air_system)
        .expect("selected unit")
        .calc_cooling_humidification_flow;
    *flow_state =
        PurchasedAirCalcCoolingHumidificationFlowRuntimeState::new(binding.ideal_loads_air_system);
    flow_state.transition_count = usize::MAX;
    let before_runtime = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_humidification_flow(
            &mut runtime,
            binding.system,
            output.calculation_cooling_dehumidification_flow,
        )
        .is_err()
    );
    assert_eq!(runtime, before_runtime);
}
