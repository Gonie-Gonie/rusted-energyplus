//! Non-vacuous CP400 coupled-runtime integration tests.

use super::*;
use crate::{
    ideal_loads::{
        DirectZonePurchasedAirScheduledCouplingInput,
        DirectZonePurchasedAirScheduledCouplingOutput,
        PurchasedAirCalcCoolingMixedAirCallLifecycleSummary as MixedAirLifecycle,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentLifecycleSummary as PredecessorLifecycle,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentLifecycleSummary as Lifecycle,
        PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardLifecycleSummary as SupplyFlowLifecycle,
        PurchasedAirRuntimeState, bind_direct_zone_purchased_air_model,
        couple_model_bound_direct_zone_purchased_air,
        purchased_air_calc_cooling_mixed_air_call_lifecycle_summary,
        purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_lifecycle_summary,
        purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_lifecycle_summary,
        purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle_summary,
    },
    schedules::precompute_schedule_cache,
};
use ep_model::{
    AutosizeOrNumber, DehumidificationControlType, HumidificationControlType, IdealLoadsLimit,
    SimulationModel,
};

use super::super::DirectZonePurchasedAirCoupledRuntimeError as Error;
use super::super::cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_validation::{
    snapshot_matches_release, validate_lifecycle,
};

#[test]
fn cp400_executes_public_routes_with_exact_owner_order_and_preserves_carriers() {
    let mut saw_active = false;
    let mut saw_inactive = false;
    for (limit, availability, capacity) in [
        (IdealLoadsLimit::NoLimit, 0.0, 5_000.0),
        (IdealLoadsLimit::LimitCapacity, 1.0, 500.0),
    ] {
        let (model, output, lifecycle, predecessor, supply_flow, mixed_air) =
            validator_fixture(limit, 0.020, capacity, availability, 1);
        assert!(
            validate(
                &model,
                &output,
                &lifecycle,
                &predecessor,
                &supply_flow,
                &mixed_air,
                1,
            )
            .is_ok()
        );
        let snapshot = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment;
        let cp399 = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment;
        let active = cp399
            .dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed;
        assert_eq!(
            snapshot
                .dehumidification_control_none_or_constant_supply_humidity_ratio_sensible_output_assignment_executed,
            active
        );
        if active {
            let flow = output
                .calculation_cooling_supply_mass_flow_positive_guard
                .supply_mass_flow_rate_kg_per_s
                .expect("CP330 flow owner");
            let cp_air = cp399.cp_air_j_per_kg_k.expect("CP399 CpAir owner");
            let mixed_temperature = output
                .calculation_cooling_mixed_air_call
                .mixed_air_temperature_c
                .expect("CP329 mixed-air temperature owner");
            let supply_temperature = cp399
                .resulting_supply_temperature_c
                .expect("CP399 supply-temperature owner");
            let first_product = flow * cp_air;
            let difference = mixed_temperature - supply_temperature;
            let result = first_product * difference;
            for (left, right) in [
                (snapshot.supply_mass_flow_rate_kg_per_s, Some(flow)),
                (snapshot.cp_air_j_per_kg_k, Some(cp_air)),
                (
                    snapshot.supply_mass_flow_rate_times_cp_air_w_per_k,
                    Some(first_product),
                ),
                (snapshot.mixed_air_temperature_c, Some(mixed_temperature)),
                (snapshot.supply_temperature_c, Some(supply_temperature)),
                (
                    snapshot.mixed_air_minus_supply_temperature_k,
                    Some(difference),
                ),
                (snapshot.calculated_cooling_sensible_output_w, Some(result)),
                (snapshot.cooling_sensible_output_w, Some(result)),
            ] {
                assert_eq!(left.map(f64::to_bits), right.map(f64::to_bits));
            }
        }
        let assignments = usize::from(active);
        assert_eq!(lifecycle.state.transition_count, 1);
        assert_eq!(lifecycle.state.inactive_transition_count, 1 - assignments);
        assert_eq!(
            lifecycle
                .state
                .dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_count,
            assignments
        );
        assert_eq!(lifecycle.state.source_site_execution_count, 8 * assignments);
        saw_active |= active;
        saw_inactive |= !active;
    }
    assert!(
        saw_active,
        "fixtures must execute public CP400 route 20 or 24"
    );
    assert!(
        saw_inactive,
        "fixtures must also exercise an inactive route"
    );
}

#[test]
fn cp400_rejects_predecessor_and_independent_owner_bit_drift() {
    let (model, output, lifecycle, predecessor, supply_flow, mixed_air) =
        validator_fixture(IdealLoadsLimit::LimitCapacity, 0.020, 500.0, 1.0, 1);
    let mut forged_predecessor = predecessor.clone();
    let value = forged_predecessor
        .state
        .latest
        .as_mut()
        .expect("CP399 latest")
        .cp_air_j_per_kg_k
        .as_mut()
        .expect("CP399 CpAir");
    *value = different(*value);
    assert!(
        validate(
            &model,
            &output,
            &lifecycle,
            &forged_predecessor,
            &supply_flow,
            &mixed_air,
            1,
        )
        .is_err()
    );

    let mut forged_supply_flow = supply_flow.clone();
    let value = forged_supply_flow
        .state
        .latest
        .as_mut()
        .expect("CP330 latest")
        .supply_mass_flow_rate_kg_per_s
        .as_mut()
        .expect("CP330 flow");
    *value = different(*value);
    assert!(
        validate(
            &model,
            &output,
            &lifecycle,
            &predecessor,
            &forged_supply_flow,
            &mixed_air,
            1,
        )
        .is_err()
    );

    let mut forged_mixed_air = mixed_air.clone();
    let value = forged_mixed_air
        .state
        .latest
        .as_mut()
        .expect("CP329 latest")
        .mixed_air_temperature_c
        .as_mut()
        .expect("CP329 mixed-air temperature");
    *value = different(*value);
    assert!(
        validate(
            &model,
            &output,
            &lifecycle,
            &predecessor,
            &supply_flow,
            &forged_mixed_air,
            1,
        )
        .is_err()
    );
}

#[test]
fn cp400_validation_is_independent_of_numerical_coupling_state() {
    let (model, mut output, lifecycle, predecessor, supply_flow, mixed_air) =
        validator_fixture(IdealLoadsLimit::LimitCapacity, 0.020, 500.0, 1.0, 1);
    output
        .coupling
        .purchased_air
        .calculation
        .zone_sensible_cooling_rate_w = different(
        output
            .coupling
            .purchased_air
            .calculation
            .zone_sensible_cooling_rate_w,
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
    output.coupling.feedback.sum_sys_mcp_t_w = different(output.coupling.feedback.sum_sys_mcp_t_w);
    assert!(
        validate(
            &model,
            &output,
            &lifecycle,
            &predecessor,
            &supply_flow,
            &mixed_air,
            1,
        )
        .is_ok()
    );
}

#[allow(clippy::type_complexity)]
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
    PredecessorLifecycle,
    SupplyFlowLifecycle,
    MixedAirLifecycle,
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
    let cache = precompute_schedule_cache(&model.typed, 1).expect("CP400 schedule cache");
    let binding = bind_direct_zone_purchased_air_model(&model).expect("CP400 direct binding");
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
            .expect("CP400 coupling"),
        );
    }
    let output = latest.expect("at least one CP400 step");
    let system = output.initialization.system;
    let lifecycle = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_lifecycle_summary(&runtime, system).expect("CP400 lifecycle");
    let predecessor = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_lifecycle_summary(&runtime, system).expect("CP399 lifecycle");
    let supply_flow = purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle_summary(
        &runtime, system,
    )
    .expect("CP330 lifecycle");
    let mixed_air = purchased_air_calc_cooling_mixed_air_call_lifecycle_summary(&runtime, system)
        .expect("CP329 lifecycle");
    (
        model,
        output,
        lifecycle,
        predecessor,
        supply_flow,
        mixed_air,
    )
}

#[allow(clippy::too_many_arguments)]
fn validate(
    model: &SimulationModel,
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    lifecycle: &Lifecycle,
    predecessor: &PredecessorLifecycle,
    supply_flow: &SupplyFlowLifecycle,
    mixed_air: &MixedAirLifecycle,
    timestep_count: usize,
) -> Result<(), Error> {
    let binding = bind_direct_zone_purchased_air_model(model).map_err(|_| latest_violation())?;
    assert!(snapshot_matches_release(output, timestep_count, &binding));
    validate_lifecycle(
        lifecycle,
        predecessor,
        supply_flow,
        mixed_air,
        timestep_count,
        output,
        &binding,
    )
}

fn latest_violation() -> Error {
    Error::CalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentLifecycleInvariant {
        field: "latest_release_snapshot_ready",
        expected: 1,
        actual: 0,
    }
}

fn different(value: f64) -> f64 {
    f64::from_bits(value.to_bits() ^ 1)
}
