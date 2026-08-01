//! Non-vacuous CP385 coupled-runtime integration tests.

use super::*;
use crate::heat_balance::state::{ZoneAirTemperatureCoefficients, ZoneHeatBalanceState};
use crate::ideal_loads::{
    DirectZonePurchasedAirScheduledCouplingInput, DirectZonePurchasedAirScheduledCouplingOutput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentLifecycleSummary as Operands,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentLifecycleSummary as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirRuntimeState, bind_direct_zone_purchased_air_model,
    couple_model_bound_direct_zone_purchased_air,
    purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_lifecycle_summary,
    purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_lifecycle_summary,
    purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_lifecycle_summary,
};
use crate::schedules::precompute_schedule_cache;
use ep_model::{
    AutosizeOrNumber, DehumidificationControlType, HumidificationControlType, IdealLoadsLimit,
    SimulationModel, ZoneId,
};

use super::super::DirectZonePurchasedAirCoupledRuntimeError as Error;
use super::super::cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_validation::{
    snapshot_matches_release, validate_lifecycle,
};

#[test]
fn cp385_closes_outer_guard_false_and_assignment_routes_with_exact_arithmetic() {
    let mut saw_outer = false;
    let mut saw_guard_false = false;
    let mut saw_assignment = false;
    for (limit, humidity_ratio, maximum_capacity_w) in [
        (IdealLoadsLimit::LimitCapacity, 0.020, 500.0),
        (IdealLoadsLimit::LimitCapacity, 0.020, 1.0e9),
        (IdealLoadsLimit::LimitCapacity, 0.008, 5_000.0),
    ] {
        let (model, output, lifecycle, predecessor, operands) =
            validator_fixture(limit, humidity_ratio, maximum_capacity_w, 1);
        assert!(validate(&model, &output, &lifecycle, &predecessor, &operands, 1).is_ok());

        let snapshot = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment;
        let assignment = snapshot.supply_enthalpy_assignment_executed;
        let guard_false = snapshot.dehumidification_total_output_capacity_guard_false_fallthrough;
        let outer = !assignment && !guard_false;
        assert_eq!(snapshot.cp379_retained_supply_enthalpy_owned_read, !outer);

        if outer {
            assert!(snapshot.preexisting_supply_enthalpy_j_per_kg.is_none());
            assert!(snapshot.resulting_supply_enthalpy_j_per_kg.is_none());
        } else if guard_false {
            assert_eq!(
                snapshot
                    .preexisting_supply_enthalpy_j_per_kg
                    .map(f64::to_bits),
                snapshot
                    .resulting_supply_enthalpy_j_per_kg
                    .map(f64::to_bits),
            );
            assert!(!snapshot.cp329_retained_mixed_air_enthalpy_owned_read);
            assert!(!snapshot.cp384_retained_cooling_total_output_owned_read);
            assert!(!snapshot.cp330_retained_supply_mass_flow_rate_owned_read);
        } else {
            let mixed = snapshot
                .mixed_air_enthalpy_j_per_kg
                .expect("active CP385 mixed-air enthalpy");
            let total = snapshot
                .cooling_total_output_w
                .expect("active CP385 cooling total output");
            let flow = snapshot
                .supply_mass_flow_rate_kg_per_s
                .expect("active CP385 supply mass flow");
            let specific = total / flow;
            let expected = mixed - specific;
            assert_eq!(
                snapshot.specific_cooling_output_j_per_kg.map(f64::to_bits),
                Some(specific.to_bits()),
            );
            assert_eq!(
                snapshot
                    .resulting_supply_enthalpy_j_per_kg
                    .map(f64::to_bits),
                Some(expected.to_bits()),
            );
        }

        let assignments = usize::from(assignment);
        let retained = usize::from(!outer);
        assert_eq!(
            lifecycle
                .state
                .post_saturation_capacity_limited_dehumidification_supply_enthalpy_assignment_count,
            assignments,
        );
        assert_eq!(lifecycle.state.source_site_execution_count, 6 * assignments);
        assert_eq!(
            lifecycle
                .state
                .cp379_retained_supply_enthalpy_owned_read_count,
            retained,
        );
        for count in [
            lifecycle
                .state
                .cp329_retained_mixed_air_enthalpy_owned_read_count,
            lifecycle.state.mixed_air_enthalpy_read_count,
            lifecycle
                .state
                .cp384_retained_cooling_total_output_owned_read_count,
            lifecycle.state.cooling_total_output_read_count,
            lifecycle
                .state
                .cp330_retained_supply_mass_flow_rate_owned_read_count,
            lifecycle.state.supply_mass_flow_rate_read_count,
            lifecycle.state.specific_cooling_output_calculation_count,
            lifecycle.state.supply_enthalpy_difference_calculation_count,
            lifecycle.state.supply_enthalpy_assignment_write_count,
        ] {
            assert_eq!(count, assignments);
        }
        saw_outer |= outer;
        saw_guard_false |= guard_false;
        saw_assignment |= assignment;
    }
    assert!(saw_outer, "fixture must exercise a complete-null route");
    assert!(saw_guard_false, "fixture must exercise a guard-false route");
    assert!(saw_assignment, "fixture must exercise an assignment route");
}

#[test]
fn cp385_rejects_one_bit_drift_in_cp384_and_cp382_retained_operands() {
    let (model, output, lifecycle, predecessor, operands) =
        validator_fixture(IdealLoadsLimit::LimitCapacity, 0.020, 500.0, 1);
    assert!(
        output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment
            .supply_enthalpy_assignment_executed,
        "fixture must enter the CP385 assignment route",
    );

    let mut bad_predecessor = predecessor.clone();
    flip_some(
        &mut bad_predecessor
            .state
            .latest
            .as_mut()
            .expect("CP384 latest")
            .resulting_cooling_total_output_w,
    );
    assert_eq!(
        validate(&model, &output, &lifecycle, &bad_predecessor, &operands, 1,),
        Err(latest_violation()),
    );

    let mut bad_operands = operands.clone();
    flip_some(
        &mut bad_operands
            .state
            .latest
            .as_mut()
            .expect("CP382 latest")
            .mixed_air_enthalpy_j_per_kg,
    );
    assert_eq!(
        validate(&model, &output, &lifecycle, &predecessor, &bad_operands, 1,),
        Err(latest_violation()),
    );

    let mut bad_operands = operands.clone();
    flip_some(
        &mut bad_operands
            .state
            .latest
            .as_mut()
            .expect("CP382 latest")
            .supply_mass_flow_rate_kg_per_s,
    );
    assert_eq!(
        validate(&model, &output, &lifecycle, &predecessor, &bad_operands, 1,),
        Err(latest_violation()),
    );

    let mut bad_operands = operands.clone();
    flip_some(
        &mut bad_operands
            .state
            .latest
            .as_mut()
            .expect("CP382 latest")
            .supply_enthalpy_j_per_kg,
    );
    assert_eq!(
        validate(&model, &output, &lifecycle, &predecessor, &bad_operands, 1,),
        Err(latest_violation()),
    );
}

#[test]
fn cp385_replay_accumulates_counts_and_keeps_latest_lineage() {
    let (model, output, lifecycle, predecessor, operands) =
        validator_fixture(IdealLoadsLimit::LimitCapacity, 0.020, 500.0, 2);
    assert!(validate(&model, &output, &lifecycle, &predecessor, &operands, 2).is_ok());
    assert_eq!(lifecycle.state.transition_count, 2);
    assert_eq!(
        lifecycle
            .state
            .latest
            .expect("CP385 replay latest")
            .parent_call_ordinal,
        2,
    );
    assert_eq!(
        lifecycle
            .state
            .post_saturation_capacity_limited_dehumidification_supply_enthalpy_assignment_count,
        2,
    );
    assert_eq!(lifecycle.state.source_site_execution_count, 12);
}

#[test]
fn cp385_is_evidence_only_and_does_not_consume_result_node_report_or_feedback() {
    let (model, output, lifecycle, predecessor, operands) =
        validator_fixture(IdealLoadsLimit::LimitCapacity, 0.020, 500.0, 1);
    let binding = bind_direct_zone_purchased_air_model(&model).expect("direct binding");
    assert!(snapshot_matches_release(&output, 1, &binding));

    let mut changed = output;
    changed
        .coupling
        .purchased_air
        .calculation
        .supply_enthalpy_j_per_kg = different(
        changed
            .coupling
            .purchased_air
            .calculation
            .supply_enthalpy_j_per_kg,
    );
    changed
        .coupling
        .purchased_air
        .supply_node_update
        .enthalpy_j_per_kg = different(
        changed
            .coupling
            .purchased_air
            .supply_node_update
            .enthalpy_j_per_kg,
    );
    changed
        .coupling
        .purchased_air
        .report
        .supply_air_total_cooling_rate_w = different(
        changed
            .coupling
            .purchased_air
            .report
            .supply_air_total_cooling_rate_w,
    );
    changed.coupling.feedback.sum_sys_mcp_t_w =
        different(changed.coupling.feedback.sum_sys_mcp_t_w);

    assert!(snapshot_matches_release(&changed, 1, &binding));
    assert!(
        validate_lifecycle(&lifecycle, &predecessor, &operands, 1, &changed, &binding,).is_ok(),
    );
}

fn validator_fixture(
    cooling_limit: IdealLoadsLimit,
    air_humidity_ratio: f64,
    maximum_capacity_w: f64,
    steps: usize,
) -> (
    SimulationModel,
    DirectZonePurchasedAirScheduledCouplingOutput,
    Lifecycle,
    Predecessor,
    Operands,
) {
    let mut typed = exact_model(1).typed;
    typed.schedules[1].hourly_value = 20.0;
    typed.schedules[2].hourly_value = 24.0;
    typed.schedules[3].hourly_value = 1.0;
    let system = &mut typed.ideal_loads_air_systems[0];
    system.cooling_limit = cooling_limit;
    system.maximum_cooling_air_flow_rate_m3_per_s = matches!(
        cooling_limit,
        IdealLoadsLimit::LimitFlowRate | IdealLoadsLimit::LimitFlowRateAndCapacity
    )
    .then_some(AutosizeOrNumber::Value(0.05));
    system.maximum_total_cooling_capacity_w = matches!(
        cooling_limit,
        IdealLoadsLimit::LimitCapacity | IdealLoadsLimit::LimitFlowRateAndCapacity
    )
    .then_some(AutosizeOrNumber::Value(maximum_capacity_w));
    system.dehumidification_control_type = DehumidificationControlType::None;
    system.humidification_control_type = HumidificationControlType::None;
    system.minimum_cooling_supply_air_humidity_ratio = f64::NAN;
    let model = SimulationModel::from_typed(typed);
    let cache = precompute_schedule_cache(&model.typed, 1).expect("CP385 schedule cache");
    let binding = bind_direct_zone_purchased_air_model(&model).expect("CP385 direct binding");
    let mut runtime = PurchasedAirRuntimeState::default();
    let mut latest = None;
    for step in 0..steps {
        let mut zone_state =
            cooling_zone_state(binding.nominal_system_timestep_seconds, air_humidity_ratio);
        latest = Some(
            couple_model_bound_direct_zone_purchased_air(
                DirectZonePurchasedAirScheduledCouplingInput {
                    binding: &binding,
                    schedule_cache: &cache,
                    schedule_sample_index: 0,
                    zone_state: &mut zone_state,
                    purchased_air_runtime_state: &mut runtime,
                    begin_environment: step == 0,
                    barometric_pressure_pa: binding.limit_context.barometric_pressure_pa,
                    system_timestep_seconds: binding.nominal_system_timestep_seconds,
                },
            )
            .expect("CP385 coupling"),
        );
    }
    let output = latest.expect("CP385 fixture requires at least one step");
    let system = output.initialization.system;
    let lifecycle = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_lifecycle_summary(&runtime, system).expect("CP385 lifecycle");
    let predecessor = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_lifecycle_summary(&runtime, system).expect("CP384 lifecycle");
    let operands = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_lifecycle_summary(&runtime, system).expect("CP382 lifecycle");
    (model, output, lifecycle, predecessor, operands)
}

fn validate(
    model: &SimulationModel,
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    lifecycle: &Lifecycle,
    predecessor: &Predecessor,
    operands: &Operands,
    timestep_count: usize,
) -> Result<(), Error> {
    let binding = bind_direct_zone_purchased_air_model(model).map_err(|_| latest_violation())?;
    assert!(snapshot_matches_release(output, timestep_count, &binding));
    validate_lifecycle(
        lifecycle,
        predecessor,
        operands,
        timestep_count,
        output,
        &binding,
    )
}

fn latest_violation() -> Error {
    Error::CalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentLifecycleInvariant {
        field: "latest_release_snapshot_ready",
        expected: 1,
        actual: 0,
    }
}

fn flip_some(value: &mut Option<f64>) {
    let value = value.as_mut().expect("active retained operand");
    *value = different(*value);
}

fn different(value: f64) -> f64 {
    f64::from_bits(value.to_bits() ^ 1)
}

fn cooling_zone_state(
    system_timestep_seconds: f64,
    air_humidity_ratio: f64,
) -> ZoneHeatBalanceState {
    ZoneHeatBalanceState {
        zone_id: ZoneId(0),
        zone_name: "ZONE ONE".to_string(),
        mean_air_temperature_c: 22.0,
        zone_timestep_average_air_temperature_c: 22.0,
        previous_mean_air_temperatures_c: [0.0; 3],
        previous_system_mean_air_temperatures_c: [0.0; 3],
        previous_system_timestep_count: 1,
        air_humidity_ratio,
        zone_timestep_average_air_humidity_ratio: air_humidity_ratio,
        previous_air_humidity_ratios: [air_humidity_ratio; 3],
        previous_system_air_humidity_ratios: [air_humidity_ratio; 3],
        use_zone_timestep_history: false,
        shorten_timestep_sys: false,
        prior_timestep_seconds: system_timestep_seconds,
        volume_m3: 100.0,
        air_heat_capacity_j_per_k: 0.0,
        convective_internal_gain_w: 0.0,
        opaque_surface_conductance_w_per_k: 100.0,
        opaque_surface_heat_gain_w: 0.0,
        opaque_surface_outside_conduction_w: 0.0,
        sum_ha_w_per_k: 100.0,
        sum_hat_surf_w: 3_000.0,
        sum_hat_ref_w: 0.0,
        sum_mcp_w_per_k: 0.0,
        sum_mcp_t_w: 0.0,
        sum_sys_mcp_w_per_k: 7.0,
        sum_sys_mcp_t_w: 11.0,
        system_dependent_zone_loads_lagged_w: 0.0,
        zone_air_temperature_coefficients: ZoneAirTemperatureCoefficients::ZERO,
        system_timestep_average_surface_convection_report_w: None,
        system_timestep_average_air_storage_report_w: None,
    }
}
