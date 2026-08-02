//! Non-vacuous CP402 coupled-runtime integration tests.

use super::*;
use crate::{
    ideal_loads::{
        DirectZonePurchasedAirScheduledCouplingInput,
        DirectZonePurchasedAirScheduledCouplingOutput,
        PurchasedAirCalcCoolingCapacityZeroFlowResetLifecycleSummary as CapacityOwnerLifecycle,
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardLifecycleSummary as CapacityCorroboratorLifecycle,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentLifecycleSummary as PredecessorLifecycle,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardLifecycleSummary as Lifecycle,
        PurchasedAirRuntimeState, bind_direct_zone_purchased_air_model,
        couple_model_bound_direct_zone_purchased_air,
        purchased_air_calc_cooling_capacity_zero_flow_reset_lifecycle_summary,
        purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle_summary,
        purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_lifecycle_summary,
        purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_lifecycle_summary,
    },
    schedules::precompute_schedule_cache,
};
use ep_model::{
    AutosizeOrNumber, DehumidificationControlType, HumidificationControlType, IdealLoadsLimit,
    SimulationModel,
};

use super::super::DirectZonePurchasedAirCoupledRuntimeError as Error;
use super::super::cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_validation::{
    snapshot_matches_release, validate_lifecycle,
};

#[test]
fn cp402_executes_public_routes_with_raw_greater_than_or_equal_comparison() {
    let mut saw_active = false;
    let mut saw_inactive = false;
    let mut saw_body = false;
    let mut saw_guard_false = false;
    for (limit, humidity_ratio, maximum_capacity_w, availability) in [
        (IdealLoadsLimit::NoLimit, 0.008, 5_000.0, 0.0),
        (IdealLoadsLimit::NoLimit, 0.020, 5_000.0, 1.0),
        (
            IdealLoadsLimit::LimitCapacity,
            0.020,
            f64::MIN_POSITIVE,
            1.0,
        ),
        (IdealLoadsLimit::LimitCapacity, 0.020, 500.0, 1.0),
        (IdealLoadsLimit::LimitCapacity, 0.020, 5_000.0, 1.0),
        (IdealLoadsLimit::LimitCapacity, 0.020, 1.0e9, 1.0),
    ] {
        let (model, output, lifecycle, predecessor, owner, corroborator) =
            validator_fixture(limit, humidity_ratio, maximum_capacity_w, availability, 1);
        assert!(
            validate(
                &model,
                &output,
                &lifecycle,
                &predecessor,
                &owner,
                &corroborator,
                1,
            )
            .is_ok()
        );
        let snapshot = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard;
        let active = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment
            .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_assignment_executed;
        assert_eq!(
            snapshot
                .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_evaluated,
            active,
        );
        assert_eq!(lifecycle.state.transition_count, 1);
        assert_eq!(
            lifecycle.state.inactive_transition_count,
            usize::from(!active)
        );
        assert_eq!(
            lifecycle
                .state
                .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_evaluation_count,
            usize::from(active),
        );
        if active {
            let latent = snapshot
                .cooling_latent_output_w
                .expect("active CP402 latent-output operand");
            let maximum = snapshot
                .maximum_total_cooling_capacity_w
                .expect("active CP402 capacity operand");
            let result = latent >= maximum;
            assert_eq!(
                snapshot
                    .cooling_latent_output_greater_than_or_equal_to_maximum_total_cooling_capacity,
                Some(result),
            );
            assert_eq!(
                snapshot.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_body_entered,
                result,
            );
            assert_eq!(
                snapshot.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough,
                !result,
            );
            assert_eq!(
                lifecycle.state.source_site_execution_count,
                3 + usize::from(result),
            );
            saw_body |= result;
            saw_guard_false |= !result;
        } else {
            assert_eq!(lifecycle.state.source_site_execution_count, 0);
        }
        saw_active |= active;
        saw_inactive |= !active;
    }
    assert!(
        saw_active,
        "fixtures must execute public CP402 route 20 or 24"
    );
    assert!(
        saw_inactive,
        "fixtures must exercise an inactive public route"
    );
    assert!(saw_body, "fixtures must enter the raw-`>=` body");
    assert!(
        saw_guard_false,
        "fixtures must exercise raw-`>=` false fallthrough"
    );
}

#[test]
fn cp402_rejects_cp401_owner_cp321_owner_and_cp340_corroborator_drift() {
    let (model, output, lifecycle, predecessor, owner, corroborator) =
        validator_fixture(IdealLoadsLimit::LimitCapacity, 0.020, 500.0, 1.0, 1);

    let mut forged_predecessor = predecessor.clone();
    let value = forged_predecessor
        .state
        .latest
        .as_mut()
        .expect("CP401 latest")
        .cooling_latent_output_w
        .as_mut()
        .expect("active CP401 latent output");
    *value = different(*value);
    assert!(
        validate(
            &model,
            &output,
            &lifecycle,
            &forged_predecessor,
            &owner,
            &corroborator,
            1,
        )
        .is_err()
    );

    let mut forged_owner = owner.clone();
    let value = forged_owner
        .state
        .latest
        .as_mut()
        .expect("CP321 latest")
        .maximum_total_cooling_capacity_w
        .as_mut()
        .expect("active CP321 maximum capacity");
    *value = different(*value);
    assert!(
        validate(
            &model,
            &output,
            &lifecycle,
            &predecessor,
            &forged_owner,
            &corroborator,
            1,
        )
        .is_err()
    );

    let mut forged_corroborator = corroborator.clone();
    let value = forged_corroborator
        .state
        .latest
        .as_mut()
        .expect("CP340 latest")
        .maximum_total_cooling_capacity_w
        .as_mut()
        .expect("active CP340 maximum capacity");
    *value = different(*value);
    assert!(
        validate(
            &model,
            &output,
            &lifecycle,
            &predecessor,
            &owner,
            &forged_corroborator,
            1,
        )
        .is_err()
    );
}

#[test]
fn cp402_rejects_non_direct_route_accounting() {
    let (model, output, lifecycle, predecessor, owner, corroborator) =
        validator_fixture(IdealLoadsLimit::LimitCapacity, 0.020, 500.0, 1.0, 1);
    let mut forged_lifecycle = lifecycle.clone();
    let mut forged_predecessor = predecessor.clone();
    let public_active_route = [20, 24]
        .into_iter()
        .find(|index| forged_lifecycle.state.predecessor_route_counts[*index] > 0)
        .expect("fixture must select a public active CP402 route");
    forged_lifecycle.state.predecessor_route_counts[public_active_route] -= 1;
    forged_lifecycle.state.predecessor_route_counts[21] += 1;
    if forged_lifecycle.state.adjustment_body_entry_route_counts[public_active_route] > 0 {
        forged_lifecycle.state.adjustment_body_entry_route_counts[public_active_route] -= 1;
        forged_lifecycle.state.adjustment_body_entry_route_counts[21] += 1;
    } else {
        forged_lifecycle.state.guard_false_fallthrough_route_counts[public_active_route] -= 1;
        forged_lifecycle.state.guard_false_fallthrough_route_counts[21] += 1;
    }
    forged_predecessor.state.predecessor_route_counts[public_active_route] -= 1;
    forged_predecessor.state.predecessor_route_counts[21] += 1;
    assert!(
        validate(
            &model,
            &output,
            &forged_lifecycle,
            &forged_predecessor,
            &owner,
            &corroborator,
            1,
        )
        .is_err()
    );
}

#[test]
fn cp402_validation_has_no_numerical_coupling_dto_feed() {
    let (model, mut output, lifecycle, predecessor, owner, corroborator) =
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
        .supply_humidity_ratio = different(
        output
            .coupling
            .purchased_air
            .calculation
            .supply_humidity_ratio,
    );
    output.coupling.feedback.sum_sys_mcp_t_w = different(output.coupling.feedback.sum_sys_mcp_t_w);
    assert!(
        validate(
            &model,
            &output,
            &lifecycle,
            &predecessor,
            &owner,
            &corroborator,
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
    CapacityOwnerLifecycle,
    CapacityCorroboratorLifecycle,
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
    let cache = precompute_schedule_cache(&model.typed, 1).expect("CP402 schedule cache");
    let binding = bind_direct_zone_purchased_air_model(&model).expect("CP402 direct binding");
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
            .expect("CP402 coupling"),
        );
    }
    let output = latest.expect("at least one CP402 step");
    let system = output.initialization.system;
    let lifecycle = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_lifecycle_summary(&runtime, system).expect("CP402 lifecycle");
    let predecessor = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_lifecycle_summary(&runtime, system).expect("CP401 lifecycle");
    let owner =
        purchased_air_calc_cooling_capacity_zero_flow_reset_lifecycle_summary(&runtime, system)
            .expect("CP321 lifecycle");
    let corroborator = purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle_summary(&runtime, system).expect("CP340 lifecycle");
    (model, output, lifecycle, predecessor, owner, corroborator)
}

#[allow(clippy::too_many_arguments)]
fn validate(
    model: &SimulationModel,
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    lifecycle: &Lifecycle,
    predecessor: &PredecessorLifecycle,
    owner: &CapacityOwnerLifecycle,
    corroborator: &CapacityCorroboratorLifecycle,
    timestep_count: usize,
) -> Result<(), Error> {
    let binding = bind_direct_zone_purchased_air_model(model).map_err(|_| latest_violation())?;
    assert!(snapshot_matches_release(output, timestep_count, &binding));
    validate_lifecycle(
        lifecycle,
        predecessor,
        owner,
        corroborator,
        timestep_count,
        output,
        &binding,
    )
}

fn latest_violation() -> Error {
    Error::CalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardLifecycleInvariant {
        field: "latest_release_snapshot_ready",
        expected: 1,
        actual: 0,
    }
}

fn different(value: f64) -> f64 {
    f64::from_bits(value.to_bits() ^ 1)
}
