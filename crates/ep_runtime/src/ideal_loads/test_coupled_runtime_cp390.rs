//! Non-vacuous CP390 coupled-runtime integration tests.

use super::*;
use crate::ideal_loads::{
    DirectZonePurchasedAirScheduledCouplingInput, DirectZonePurchasedAirScheduledCouplingOutput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentLifecycleSummary as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitLifecycleSummary as Lifecycle,
    PurchasedAirRuntimeState, bind_direct_zone_purchased_air_model,
    couple_model_bound_direct_zone_purchased_air,
    purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_lifecycle_summary,
    purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_lifecycle_summary,
};
use crate::schedules::precompute_schedule_cache;
use ep_model::{
    AutosizeOrNumber, DehumidificationControlType, HumidificationControlType, IdealLoadsLimit,
    SimulationModel,
};

use super::super::DirectZonePurchasedAirCoupledRuntimeError as Error;
use super::super::cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_validation::{
    snapshot_matches_release, validate_lifecycle,
};

#[test]
fn cp390_preserves_cp389_temperature_on_representative_direct_routes() {
    for (limit, maximum_capacity_w, availability) in [
        (IdealLoadsLimit::NoLimit, 5_000.0, 0.0),
        (IdealLoadsLimit::NoLimit, 5_000.0, 1.0),
        (IdealLoadsLimit::LimitCapacity, 500.0, 1.0),
    ] {
        let (model, output, lifecycle, predecessor) =
            validator_fixture(limit, 0.020, maximum_capacity_w, availability, 1);
        assert!(validate(&model, &output, &lifecycle, &predecessor, 1).is_ok());
        let snapshot = output.calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit;
        let cp389 = output.calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment;
        assert_eq!(
            snapshot.preexisting_supply_temperature_c.map(f64::to_bits),
            cp389.resulting_supply_temperature_c.map(f64::to_bits)
        );
        assert_eq!(
            snapshot.resulting_supply_temperature_c.map(f64::to_bits),
            cp389.resulting_supply_temperature_c.map(f64::to_bits)
        );
        assert!(!snapshot.dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_executed);
        assert_eq!(lifecycle.state.transition_count, 1);
        assert_eq!(lifecycle.state.inactive_transition_count, 1);
        assert_eq!(lifecycle.state.source_site_execution_count, 0);
        assert_eq!(
            lifecycle.state.predecessor_route_counts,
            predecessor.state.predecessor_route_counts
        );
    }
}

#[test]
fn cp390_rejects_one_bit_drift_in_cp389_retained_result() {
    let (model, output, lifecycle, mut predecessor) =
        validator_fixture(IdealLoadsLimit::LimitCapacity, 0.020, 500.0, 1.0, 1);
    let value = predecessor
        .state
        .latest
        .as_mut()
        .expect("CP389 latest")
        .resulting_supply_temperature_c
        .as_mut()
        .expect("CP389 retained temperature");
    *value = f64::from_bits(value.to_bits() ^ 1);
    assert_eq!(
        validate(&model, &output, &lifecycle, &predecessor, 1),
        Err(latest_violation())
    );
}

#[test]
fn cp390_rejects_cp389_source_order_corruption() {
    let (model, output, lifecycle, mut predecessor) =
        validator_fixture(IdealLoadsLimit::LimitCapacity, 0.020, 500.0, 1.0, 1);
    predecessor
        .state
        .latest
        .as_mut()
        .expect("CP389 latest")
        .source_order = &["forged-cp389-source-order"];
    assert_eq!(
        validate(&model, &output, &lifecycle, &predecessor, 1),
        Err(latest_violation())
    );
}

#[test]
fn cp390_rejects_route_drift_and_remains_evidence_only() {
    let (model, mut output, mut lifecycle, predecessor) =
        validator_fixture(IdealLoadsLimit::LimitCapacity, 0.020, 500.0, 1.0, 2);
    let binding = bind_direct_zone_purchased_air_model(&model).expect("CP390 binding");
    output
        .coupling
        .prediction
        .predicted_loads
        .total_output_required_w = different(
        output
            .coupling
            .prediction
            .predicted_loads
            .total_output_required_w,
    );
    output
        .coupling
        .purchased_air
        .calculation
        .supply_temperature_c = different(
        output
            .coupling
            .purchased_air
            .calculation
            .supply_temperature_c,
    );
    output
        .coupling
        .purchased_air
        .supply_node_update
        .temperature_c = different(
        output
            .coupling
            .purchased_air
            .supply_node_update
            .temperature_c,
    );
    output
        .coupling
        .purchased_air
        .report
        .supply_air_total_cooling_rate_w = different(
        output
            .coupling
            .purchased_air
            .report
            .supply_air_total_cooling_rate_w,
    );
    output.coupling.feedback.sum_sys_mcp_t_w = different(output.coupling.feedback.sum_sys_mcp_t_w);
    assert!(snapshot_matches_release(&output, 2, &binding));
    assert!(validate_lifecycle(&lifecycle, &predecessor, 2, &output, &binding).is_ok());

    lifecycle.state.predecessor_route_counts[20] += 1;
    assert!(validate_lifecycle(&lifecycle, &predecessor, 2, &output, &binding).is_err());
}

fn validator_fixture(
    cooling_limit: IdealLoadsLimit,
    air_humidity_ratio: f64,
    maximum_capacity_w: f64,
    availability: f64,
    steps: usize,
) -> (
    SimulationModel,
    DirectZonePurchasedAirScheduledCouplingOutput,
    Lifecycle,
    Predecessor,
) {
    let mut typed = exact_model(1).typed;
    typed.schedules[1].hourly_value = 20.0;
    typed.schedules[2].hourly_value = 24.0;
    typed.schedules[3].hourly_value = availability;
    let system = &mut typed.ideal_loads_air_systems[0];
    system.cooling_limit = cooling_limit;
    system.maximum_cooling_air_flow_rate_m3_per_s = None;
    system.maximum_total_cooling_capacity_w = matches!(
        cooling_limit,
        IdealLoadsLimit::LimitCapacity | IdealLoadsLimit::LimitFlowRateAndCapacity
    )
    .then_some(AutosizeOrNumber::Value(maximum_capacity_w));
    system.dehumidification_control_type = DehumidificationControlType::None;
    system.humidification_control_type = HumidificationControlType::None;
    system.minimum_cooling_supply_air_humidity_ratio = f64::NAN;
    let model = SimulationModel::from_typed(typed);
    let cache = precompute_schedule_cache(&model.typed, 1).expect("CP390 schedule cache");
    let binding = bind_direct_zone_purchased_air_model(&model).expect("CP390 direct binding");
    let mut runtime = PurchasedAirRuntimeState::default();
    let mut latest = None;
    for step in 0..steps {
        let mut zone_state = super::coupled_runtime_tests_cp389::cooling_zone_state(
            binding.nominal_system_timestep_seconds,
            air_humidity_ratio,
        );
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
            .expect("CP390 coupling"),
        );
    }
    let output = latest.expect("at least one CP390 step");
    let system = output.initialization.system;
    let lifecycle = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_lifecycle_summary(&runtime, system).expect("CP390 lifecycle");
    let predecessor = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_lifecycle_summary(&runtime, system).expect("CP389 lifecycle");
    (model, output, lifecycle, predecessor)
}

fn validate(
    model: &SimulationModel,
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    lifecycle: &Lifecycle,
    predecessor: &Predecessor,
    timestep_count: usize,
) -> Result<(), Error> {
    let binding = bind_direct_zone_purchased_air_model(model).map_err(|_| latest_violation())?;
    assert!(snapshot_matches_release(output, timestep_count, &binding));
    validate_lifecycle(lifecycle, predecessor, timestep_count, output, &binding)
}

fn latest_violation() -> Error {
    Error::CalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitLifecycleInvariant {
        field: "latest_release_snapshot_ready",
        expected: 1,
        actual: 0,
    }
}

fn different(value: f64) -> f64 {
    f64::from_bits(value.to_bits() ^ 1)
}
