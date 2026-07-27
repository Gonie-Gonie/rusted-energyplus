use super::*;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingEntryGatePredicateInput,
    purchased_air_calc_cooling_entry_gate_lifecycle_summary,
};

#[test]
fn public_cooling_entry_gate_rejects_forgery_subset_and_replay_without_mutation() {
    let (model, cache) = fixture(|_| {});
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut zone_state = zone_state_for_temp_independent_load(0.0);
    let mut runtime = PurchasedAirRuntimeState::default();
    let first = couple_model_bound_direct_zone_purchased_air(
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
    assert_eq!(
        first.calculation_cooling_entry_gate.cooling_body_entered,
        first.coupling.purchased_air.calculation.mode == IdealLoadsSensibleMode::Cooling
    );

    init_second_call(&mut runtime, &binding, &first);
    let second_entry = advance_purchased_air_calc_entry(
        &mut runtime,
        binding.ideal_loads_air_system,
        calc_context(&binding, first.coupling.prediction.zone_demand),
    )
    .expect("second Calc entry");
    let second_minimum_oa =
        advance_direct_no_oa_calc_minimum_oa_prefix(&mut runtime, binding.system, second_entry)
            .expect("second minimum-OA prefix");
    let before = purchased_air_calc_cooling_entry_gate_lifecycle_summary(
        &runtime,
        binding.ideal_loads_air_system,
    )
    .expect("CP312 state before rejected calls");

    let mut forged_entry = second_entry;
    forged_entry.demand.remaining_output_req_to_cool_sp_w += 1.0;
    assert_eq!(
        advance_direct_no_oa_calc_cooling_entry_gate(
            &mut runtime,
            binding.system,
            forged_entry,
            second_minimum_oa,
            PurchasedAirTemperatureControlType::DualHeatCool,
        ),
        Err(
            PurchasedAirCalcCoolingEntryGateError::CalculationEntrySnapshotMismatch {
                system: binding.ideal_loads_air_system,
            }
        )
    );
    let mut forged_minimum_oa = second_minimum_oa;
    forged_minimum_oa.minimum_outdoor_air_sensible_output_w = Some(-0.0);
    assert_eq!(
        advance_direct_no_oa_calc_cooling_entry_gate(
            &mut runtime,
            binding.system,
            second_entry,
            forged_minimum_oa,
            PurchasedAirTemperatureControlType::DualHeatCool,
        ),
        Err(
            PurchasedAirCalcCoolingEntryGateError::MinimumOaPrefixSnapshotMismatch {
                system: binding.ideal_loads_air_system,
            }
        )
    );
    let mut forged_outdoor_air_system = binding.system.clone();
    forged_outdoor_air_system.outdoor_air_inlet_node_name =
        Some(NormalizedName::new("FORGED OUTDOOR AIR"));
    assert_eq!(
        advance_direct_no_oa_calc_cooling_entry_gate(
            &mut runtime,
            &forged_outdoor_air_system,
            second_entry,
            second_minimum_oa,
            PurchasedAirTemperatureControlType::DualHeatCool,
        ),
        Err(
            PurchasedAirCalcCoolingEntryGateError::OutdoorAirOutsideDirectSubset {
                system: binding.ideal_loads_air_system,
            }
        )
    );
    assert_eq!(
        advance_direct_no_oa_calc_cooling_entry_gate(
            &mut runtime,
            binding.system,
            second_entry,
            second_minimum_oa,
            PurchasedAirTemperatureControlType::SingleHeat,
        ),
        Err(
            PurchasedAirCalcCoolingEntryGateError::TemperatureControlTypeOutsideDirectSubset {
                actual: PurchasedAirTemperatureControlType::SingleHeat,
            }
        )
    );
    assert_eq!(
        purchased_air_calc_cooling_entry_gate_lifecycle_summary(
            &runtime,
            binding.ideal_loads_air_system,
        )
        .expect("CP312 state after rejected calls"),
        before
    );

    let second = advance_direct_no_oa_calc_cooling_entry_gate(
        &mut runtime,
        binding.system,
        second_entry,
        second_minimum_oa,
        PurchasedAirTemperatureControlType::DualHeatCool,
    )
    .expect("exact second CP312 call");
    assert_eq!(second.parent_call_ordinal, 2);
    let after_success = purchased_air_calc_cooling_entry_gate_lifecycle_summary(
        &runtime,
        binding.ideal_loads_air_system,
    )
    .expect("CP312 state after success");
    assert_eq!(after_success.state.transition_count, 2);
    assert!(matches!(
        advance_direct_no_oa_calc_cooling_entry_gate(
            &mut runtime,
            binding.system,
            second_entry,
            second_minimum_oa,
            PurchasedAirTemperatureControlType::DualHeatCool,
        ),
        Err(
            PurchasedAirCalcCoolingEntryGateError::PredecessorCallOrder {
                calculation_entry_call_count: 2,
                minimum_oa_prefix_transition_count: 2,
                cooling_entry_gate_transition_count: 2,
                ..
            }
        )
    ));
    assert_eq!(
        purchased_air_calc_cooling_entry_gate_lifecycle_summary(
            &runtime,
            binding.ideal_loads_air_system,
        )
        .expect("CP312 state after replay"),
        after_success
    );
}

#[test]
fn public_active_release_rejects_nonfinite_cooling_predicate_transactionally() {
    let (model, cache) = fixture(|_| {});
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut zone_state = zone_state_for_temp_independent_load(0.0);
    let mut runtime = PurchasedAirRuntimeState::default();
    let first = couple_model_bound_direct_zone_purchased_air(
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
    init_second_call(&mut runtime, &binding, &first);
    let mut demand = first.coupling.prediction.zone_demand;
    demand.remaining_output_req_to_cool_sp_w = f64::NAN;
    let entry = advance_purchased_air_calc_entry(
        &mut runtime,
        binding.ideal_loads_air_system,
        calc_context(&binding, demand),
    )
    .expect("source-compatible nonfinite CP310 demand");
    let minimum_oa =
        advance_direct_no_oa_calc_minimum_oa_prefix(&mut runtime, binding.system, entry)
            .expect("CP311 does not consume cooling demand");
    let before = purchased_air_calc_cooling_entry_gate_lifecycle_summary(
        &runtime,
        binding.ideal_loads_air_system,
    )
    .expect("CP312 state before nonfinite rejection");

    assert_eq!(
        advance_direct_no_oa_calc_cooling_entry_gate(
            &mut runtime,
            binding.system,
            entry,
            minimum_oa,
            PurchasedAirTemperatureControlType::DualHeatCool,
        ),
        Err(
            PurchasedAirCalcCoolingEntryGateError::NonFinitePredicateInput {
                input: PurchasedAirCalcCoolingEntryGatePredicateInput::CoolingSetpointDemand,
            }
        )
    );
    assert_eq!(
        purchased_air_calc_cooling_entry_gate_lifecycle_summary(
            &runtime,
            binding.ideal_loads_air_system,
        )
        .expect("CP312 state after nonfinite rejection"),
        before
    );
}

fn init_second_call(
    runtime: &mut PurchasedAirRuntimeState,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
    first: &DirectZonePurchasedAirScheduledCouplingOutput,
) {
    init_purchased_air_runtime(
        runtime,
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
            heating_setpoint_c: first.schedules.heating_setpoint_c,
            cooling_setpoint_c: first.schedules.cooling_setpoint_c,
            overall_availability: first.schedules.overall_availability,
            heating_availability: 1.0,
            cooling_availability: 1.0,
        },
    )
    .expect("second initialization prefix");
}

fn calc_context(
    binding: &DirectZonePurchasedAirModelBinding<'_>,
    demand: crate::zone_equipment::ZoneSysEnergyDemand,
) -> PurchasedAirCalcEntryContext {
    PurchasedAirCalcEntryContext {
        controlled_zone: binding.zone,
        supply_node: binding.supply_node,
        zone_node: binding.zone_air_node,
        outdoor_air_node: None,
        recirculation_node: binding.return_node,
        demand,
        zone_component_availability: Some(PurchasedAirAvailabilityStatus::NoAction),
        overall_availability: 1.0,
        heating_availability: 1.0,
        cooling_availability: 1.0,
    }
}
