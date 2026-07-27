use super::*;

#[test]
fn exact_nan_calc_entry_snapshot_advances_minimum_oa_prefix_transactionally() {
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
    .expect("first source-ordered coupling");
    init_purchased_air_runtime(
        &mut runtime,
        &binding.init_manager_plan,
        &binding.init_topology_plan,
        binding.system,
        PurchasedAirInitCallContext {
            zone_equipment_inputs_filled: true,
            system_sizing_calculation: false,
            sizing: PurchasedAirHardSizeLegacyContext {
                current_zone_equipment_index: 1,
                zone_sizing_run_done: false,
            },
            begin_environment: false,
            standard_air_density_kg_per_m3: binding.limit_context.standard_air_density_kg_per_m3,
            heating_setpoint_c: output.schedules.heating_setpoint_c,
            cooling_setpoint_c: output.schedules.cooling_setpoint_c,
            overall_availability: output.schedules.overall_availability,
            heating_availability: 1.0,
            cooling_availability: 1.0,
        },
    )
    .expect("second initialization prefix");

    let availability_nan = f64::from_bits(0x7ff8_0000_0000_0001);
    let second = advance_purchased_air_calc_entry(
        &mut runtime,
        binding.ideal_loads_air_system,
        PurchasedAirCalcEntryContext {
            controlled_zone: binding.zone,
            supply_node: binding.supply_node,
            zone_node: binding.zone_air_node,
            outdoor_air_node: None,
            recirculation_node: binding.return_node,
            demand: output.coupling.prediction.zone_demand,
            zone_component_availability: Some(PurchasedAirAvailabilityStatus::NoAction),
            overall_availability: availability_nan,
            heating_availability: availability_nan,
            cooling_availability: availability_nan,
        },
    )
    .expect("source-compatible NaN schedules");
    assert_eq!(second.call_ordinal, 2);
    assert!(second.unit_body_entered);
    assert_eq!(
        second.overall_availability.to_bits(),
        availability_nan.to_bits()
    );

    let mut different_nan_payload = second;
    different_nan_payload.overall_availability = f64::from_bits(0x7ff8_0000_0000_0002);
    let before_mismatch = purchased_air_calc_minimum_oa_prefix_lifecycle_summary(
        &runtime,
        binding.ideal_loads_air_system,
    )
    .expect("minimum-OA prefix state before mismatch");
    assert_eq!(
        advance_direct_no_oa_calc_minimum_oa_prefix(
            &mut runtime,
            binding.system,
            different_nan_payload,
        ),
        Err(
            PurchasedAirCalcMinimumOaPrefixError::CalculationEntrySnapshotMismatch {
                system: binding.ideal_loads_air_system,
            }
        )
    );
    let after_mismatch = purchased_air_calc_minimum_oa_prefix_lifecycle_summary(
        &runtime,
        binding.ideal_loads_air_system,
    )
    .expect("minimum-OA prefix state after mismatch");
    assert_eq!(after_mismatch, before_mismatch);

    let second_prefix =
        advance_direct_no_oa_calc_minimum_oa_prefix(&mut runtime, binding.system, second)
            .expect("exact copied NaN snapshot must advance");
    assert_eq!(second_prefix.parent_call_ordinal, 2);
    assert!(second_prefix.unit_body_entered);
    let lifecycle = purchased_air_calc_minimum_oa_prefix_lifecycle_summary(
        &runtime,
        binding.ideal_loads_air_system,
    )
    .expect("minimum-OA prefix lifecycle after exact snapshot");
    assert_eq!(lifecycle.state.transition_count, 2);
    assert_eq!(lifecycle.state.source_execution_count, 2);
}
