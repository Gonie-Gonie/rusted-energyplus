use super::*;
use crate::ideal_loads::{
    advance_direct_no_oa_calc_cooling_oa_max_flow_gate,
    purchased_air_calc_cooling_oa_max_flow_gate_lifecycle_summary,
};

#[test]
fn scheduled_binding_orders_cooling_oa_max_flow_gate_before_numerical_calc() {
    let (model, cache) = cooling_flow_fixture();
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
    .expect("source-ordered CP313 coupling");

    assert_eq!(
        output
            .calculation_cooling_oa_max_flow_gate
            .parent_call_ordinal,
        output.calculation_cooling_entry_gate.parent_call_ordinal
    );
    assert_eq!(
        output.calculation_cooling_oa_max_flow_gate.system,
        binding.ideal_loads_air_system
    );
    assert_eq!(
        output
            .calculation_cooling_oa_max_flow_gate
            .predecessor_cooling_body_entered,
        output.calculation_cooling_entry_gate.cooling_body_entered
    );
    let lifecycle = purchased_air_calc_cooling_oa_max_flow_gate_lifecycle_summary(
        &runtime,
        binding.ideal_loads_air_system,
    )
    .expect("CP313 lifecycle");
    assert_eq!(lifecycle.state.transition_count, 1);
    assert_eq!(
        output.calculation_cooling_entry_gate.cooling_body_entered,
        output.coupling.purchased_air.calculation.mode == IdealLoadsSensibleMode::Cooling
    );
}

#[test]
fn public_cooling_oa_max_flow_gate_rejects_forgery_replay_and_overflow_without_mutation() {
    let (model, _) = cooling_flow_fixture();
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut runtime = PurchasedAirRuntimeState::default();
    let (initialization, minimum_oa, cooling_entry) =
        prepare_cooling_predecessors(&mut runtime, &binding);
    let before = purchased_air_calc_cooling_oa_max_flow_gate_lifecycle_summary(
        &runtime,
        binding.ideal_loads_air_system,
    )
    .expect("empty CP313 lifecycle");

    let mut forged_initialization = initialization;
    forged_initialization.maximum_cooling_air_mass_flow_rate_kg_per_s += 1.0;
    assert!(
        advance_direct_no_oa_calc_cooling_oa_max_flow_gate(
            &mut runtime,
            binding.system,
            forged_initialization,
            minimum_oa,
            cooling_entry,
        )
        .is_err()
    );
    let mut forged_cooling_entry = cooling_entry;
    forged_cooling_entry.parent_call_ordinal += 1;
    assert!(
        advance_direct_no_oa_calc_cooling_oa_max_flow_gate(
            &mut runtime,
            binding.system,
            initialization,
            minimum_oa,
            forged_cooling_entry,
        )
        .is_err()
    );
    assert_eq!(
        purchased_air_calc_cooling_oa_max_flow_gate_lifecycle_summary(
            &runtime,
            binding.ideal_loads_air_system,
        )
        .expect("CP313 state after rejected forgery"),
        before
    );

    let accepted = advance_direct_no_oa_calc_cooling_oa_max_flow_gate(
        &mut runtime,
        binding.system,
        initialization,
        minimum_oa,
        cooling_entry,
    )
    .expect("exact CP313 transition");
    assert!(accepted.predecessor_cooling_body_entered);
    assert!(!accepted.maximum_cooling_flow_body_entered);
    let after_success = purchased_air_calc_cooling_oa_max_flow_gate_lifecycle_summary(
        &runtime,
        binding.ideal_loads_air_system,
    )
    .expect("CP313 state after success");
    assert!(
        advance_direct_no_oa_calc_cooling_oa_max_flow_gate(
            &mut runtime,
            binding.system,
            initialization,
            minimum_oa,
            cooling_entry,
        )
        .is_err()
    );
    assert_eq!(
        purchased_air_calc_cooling_oa_max_flow_gate_lifecycle_summary(
            &runtime,
            binding.ideal_loads_air_system,
        )
        .expect("CP313 state after rejected replay"),
        after_success
    );

    let (mut overflow_runtime, overflow_initialization, overflow_minimum_oa, overflow_cooling) = {
        let mut runtime = PurchasedAirRuntimeState::default();
        let predecessors = prepare_cooling_predecessors(&mut runtime, &binding);
        (runtime, predecessors.0, predecessors.1, predecessors.2)
    };
    overflow_runtime
        .units
        .get_mut(&binding.ideal_loads_air_system)
        .expect("selected unit")
        .calc_cooling_oa_max_flow_gate
        .transition_count = usize::MAX;
    let before_overflow = purchased_air_calc_cooling_oa_max_flow_gate_lifecycle_summary(
        &overflow_runtime,
        binding.ideal_loads_air_system,
    )
    .expect("forged overflow lifecycle");
    assert!(
        advance_direct_no_oa_calc_cooling_oa_max_flow_gate(
            &mut overflow_runtime,
            binding.system,
            overflow_initialization,
            overflow_minimum_oa,
            overflow_cooling,
        )
        .is_err()
    );
    assert_eq!(
        purchased_air_calc_cooling_oa_max_flow_gate_lifecycle_summary(
            &overflow_runtime,
            binding.ideal_loads_air_system,
        )
        .expect("CP313 state after rejected overflow"),
        before_overflow
    );
}

#[test]
fn public_cooling_oa_max_flow_gate_rejects_nonfinite_cache_while_unit_off() {
    let (model, _) = cooling_flow_fixture();
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut runtime = PurchasedAirRuntimeState::default();
    let (mut initialization, minimum_oa, cooling_entry) =
        prepare_predecessors(&mut runtime, &binding, 0.0);
    assert!(!cooling_entry.unit_body_entered);
    assert!(!cooling_entry.cooling_body_entered);

    let nonfinite = f64::from_bits(0x7ff8_0000_0000_0313);
    initialization.maximum_cooling_air_mass_flow_rate_kg_per_s = nonfinite;
    runtime
        .units
        .get_mut(&binding.ideal_loads_air_system)
        .expect("selected unit")
        .maximum_cooling_air_mass_flow_rate_kg_per_s = nonfinite;
    let before = purchased_air_calc_cooling_oa_max_flow_gate_lifecycle_summary(
        &runtime,
        binding.ideal_loads_air_system,
    )
    .expect("CP313 state before invalid cache");

    assert!(
        advance_direct_no_oa_calc_cooling_oa_max_flow_gate(
            &mut runtime,
            binding.system,
            initialization,
            minimum_oa,
            cooling_entry,
        )
        .is_err()
    );
    assert_eq!(
        purchased_air_calc_cooling_oa_max_flow_gate_lifecycle_summary(
            &runtime,
            binding.ideal_loads_air_system,
        )
        .expect("CP313 state after invalid cache"),
        before
    );
}

fn prepare_cooling_predecessors(
    runtime: &mut PurchasedAirRuntimeState,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> (
    PurchasedAirInitSnapshot,
    PurchasedAirCalcMinimumOaPrefixSnapshot,
    PurchasedAirCalcCoolingEntryGateSnapshot,
) {
    prepare_predecessors(runtime, binding, 1.0)
}

fn prepare_predecessors(
    runtime: &mut PurchasedAirRuntimeState,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
    overall_availability: f64,
) -> (
    PurchasedAirInitSnapshot,
    PurchasedAirCalcMinimumOaPrefixSnapshot,
    PurchasedAirCalcCoolingEntryGateSnapshot,
) {
    let initialization = init_purchased_air_runtime(
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
            begin_environment: true,
            standard_air_density_kg_per_m3: binding.limit_context.standard_air_density_kg_per_m3,
            heating_setpoint_c: 20.0,
            cooling_setpoint_c: 24.0,
            overall_availability,
            heating_availability: 1.0,
            cooling_availability: 1.0,
        },
    )
    .expect("initialized direct release unit");
    let demand = crate::zone_equipment::ZoneSysEnergyDemand::from_output_required_setpoint_loads(
        binding.zone,
        0.0,
        -1_000.0,
    );
    let entry = advance_purchased_air_calc_entry(
        runtime,
        binding.ideal_loads_air_system,
        PurchasedAirCalcEntryContext {
            controlled_zone: binding.zone,
            supply_node: binding.supply_node,
            zone_node: binding.zone_air_node,
            outdoor_air_node: None,
            recirculation_node: binding.return_node,
            demand,
            zone_component_availability: Some(PurchasedAirAvailabilityStatus::NoAction),
            overall_availability,
            heating_availability: 1.0,
            cooling_availability: 1.0,
        },
    )
    .expect("CP310 predecessor");
    let minimum_oa = advance_direct_no_oa_calc_minimum_oa_prefix(runtime, binding.system, entry)
        .expect("CP311 predecessor");
    let cooling_entry = advance_direct_no_oa_calc_cooling_entry_gate(
        runtime,
        binding.system,
        entry,
        minimum_oa,
        PurchasedAirTemperatureControlType::DualHeatCool,
    )
    .expect("CP312 predecessor");
    assert_eq!(
        cooling_entry.cooling_body_entered,
        overall_availability > 0.0
    );
    (initialization, minimum_oa, cooling_entry)
}

fn cooling_flow_fixture() -> (SimulationModel, ScheduleSeriesCache) {
    fixture(|typed| {
        let system = &mut typed.ideal_loads_air_systems[0];
        system.cooling_limit = IdealLoadsLimit::LimitFlowRate;
        system.maximum_cooling_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Value(0.25));
    })
}
