//! Non-vacuous CP404 coupled-runtime integration tests.

use super::*;
use crate::{
    ideal_loads::{
        DirectZonePurchasedAirScheduledCouplingInput,
        DirectZonePurchasedAirScheduledCouplingOutput,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyHumidityRatioAssignmentLifecycleSummary as Lifecycle,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirAssignmentLifecycleSummary as PredecessorLifecycle,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentLifecycleSummary as EnthalpyOwnerLifecycle,
        PurchasedAirRuntimeState, bind_direct_zone_purchased_air_model,
        couple_model_bound_direct_zone_purchased_air,
        purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment_lifecycle_summary,
        purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment_lifecycle_summary,
        purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_lifecycle_summary,
    },
    psychrometrics::energyplus_psy_w_fn_tdb_h,
    schedules::precompute_schedule_cache,
};
use ep_model::{
    AutosizeOrNumber, DehumidificationControlType, HumidificationControlType, IdealLoadsLimit,
    SimulationModel,
};

use super::super::DirectZonePurchasedAirCoupledRuntimeError as Error;
use super::super::cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment_validation::{
    snapshot_matches_release, validate_lifecycle,
};

#[test]
fn cp404_preserves_observed_routes_and_assigns_psychrometric_humidity_ratio() {
    let mut saw_assignment = false;
    let mut saw_inactive = false;
    for (limit, humidity_ratio, maximum_capacity_w, availability) in [
        (
            IdealLoadsLimit::LimitCapacity,
            0.020,
            f64::MIN_POSITIVE,
            1.0,
        ),
        (IdealLoadsLimit::LimitCapacity, 0.020, 1.0e9, 1.0),
        (IdealLoadsLimit::NoLimit, 0.008, 5_000.0, 0.0),
    ] {
        let (model, output, lifecycle, predecessor, enthalpy_owner) =
            validator_fixture(limit, humidity_ratio, maximum_capacity_w, availability, 1);
        assert!(
            validate(
                &model,
                &output,
                &lifecycle,
                &predecessor,
                &enthalpy_owner,
                1,
            )
            .is_ok()
        );

        let predecessor_snapshot = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment;
        let enthalpy_owner_snapshot = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment;
        let snapshot = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment;
        let assignment = predecessor_snapshot
            .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_supply_temperature_mixed_air_assignment_executed;
        let guard_false = predecessor_snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough;
        let inactive = !predecessor_snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_evaluated;

        assert_eq!(
            snapshot.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_supply_humidity_ratio_assignment_executed,
            assignment,
        );
        assert_eq!(lifecycle.state.transition_count, 1);
        assert_eq!(
            lifecycle.state.inactive_transition_count,
            usize::from(inactive),
        );
        assert_eq!(
            lifecycle.state.predecessor_guard_false_fallthrough_count,
            usize::from(guard_false),
        );
        assert_eq!(
            lifecycle.state.supply_humidity_ratio_assignment_count,
            usize::from(assignment),
        );
        assert_eq!(
            lifecycle.state.source_site_execution_count,
            4 * usize::from(assignment),
        );
        for (left, right) in [
            (
                snapshot.resulting_supply_enthalpy_j_per_kg,
                predecessor_snapshot.resulting_supply_enthalpy_j_per_kg,
            ),
            (
                snapshot.resulting_supply_temperature_c,
                predecessor_snapshot.resulting_supply_temperature_c,
            ),
        ] {
            assert_eq!(left.map(f64::to_bits), right.map(f64::to_bits));
        }

        if assignment {
            let temperature = predecessor_snapshot
                .resulting_supply_temperature_c
                .expect("CP404 body must consume CP403 supply temperature");
            let enthalpy = predecessor_snapshot
                .resulting_supply_enthalpy_j_per_kg
                .expect("CP404 body must consume CP403 supply enthalpy");
            let expected = energyplus_psy_w_fn_tdb_h(temperature, enthalpy);
            assert!(enthalpy_owner_snapshot.supply_enthalpy_assignment_executed);
            assert_eq!(
                enthalpy_owner_snapshot
                    .resulting_supply_enthalpy_j_per_kg
                    .map(f64::to_bits),
                Some(enthalpy.to_bits()),
            );
            assert_eq!(
                snapshot.supply_temperature_c.map(f64::to_bits),
                Some(temperature.to_bits()),
            );
            assert_eq!(
                snapshot.supply_enthalpy_j_per_kg.map(f64::to_bits),
                Some(enthalpy.to_bits()),
            );
            for value in [
                snapshot.psychrometric_supply_humidity_ratio,
                snapshot.assigned_supply_humidity_ratio,
                snapshot.resulting_supply_humidity_ratio,
            ] {
                assert_eq!(value.map(f64::to_bits), Some(expected.to_bits()));
            }
            for count in [
                lifecycle.state.supply_temperature_owned_read_count,
                lifecycle
                    .state
                    .supply_temperature_for_humidity_ratio_inversion_read_count,
                lifecycle.state.supply_enthalpy_owned_read_count,
                lifecycle
                    .state
                    .cp385_same_call_supply_enthalpy_bit_corroboration_count,
                lifecycle
                    .state
                    .supply_enthalpy_for_humidity_ratio_inversion_read_count,
                lifecycle
                    .state
                    .psychrometric_supply_humidity_ratio_evaluation_count,
                lifecycle.state.supply_humidity_ratio_assignment_write_count,
            ] {
                assert_eq!(count, 1);
            }
        } else {
            assert!(snapshot.supply_temperature_c.is_none());
            assert!(snapshot.supply_enthalpy_j_per_kg.is_none());
            assert!(snapshot.psychrometric_supply_humidity_ratio.is_none());
            assert!(snapshot.assigned_supply_humidity_ratio.is_none());
            assert_eq!(
                snapshot.resulting_supply_humidity_ratio.map(f64::to_bits),
                predecessor_snapshot
                    .resulting_supply_humidity_ratio
                    .map(f64::to_bits),
            );
        }

        saw_assignment |= assignment;
        saw_inactive |= inactive;
    }
    assert!(saw_assignment, "fixture set must enter the CP404 body");
    assert!(
        saw_inactive,
        "fixture set must preserve a CP403 inactive route"
    );
}

#[test]
fn cp404_rejects_cp403_temperature_and_cp385_enthalpy_owner_drift() {
    let (model, output, lifecycle, predecessor, enthalpy_owner) = validator_fixture(
        IdealLoadsLimit::LimitCapacity,
        0.020,
        f64::MIN_POSITIVE,
        1.0,
        1,
    );
    assert!(
        output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment
            .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_supply_temperature_mixed_air_assignment_executed,
        "corruption fixture must enter CP404",
    );

    let mut forged_predecessor = predecessor.clone();
    let temperature = forged_predecessor
        .state
        .latest
        .as_mut()
        .expect("CP403 latest")
        .resulting_supply_temperature_c
        .as_mut()
        .expect("CP403 supply-temperature owner");
    *temperature = different(*temperature);
    assert!(
        validate(
            &model,
            &output,
            &lifecycle,
            &forged_predecessor,
            &enthalpy_owner,
            1,
        )
        .is_err()
    );

    let mut forged_enthalpy_owner = enthalpy_owner.clone();
    let enthalpy = forged_enthalpy_owner
        .state
        .latest
        .as_mut()
        .expect("CP385 latest")
        .resulting_supply_enthalpy_j_per_kg
        .as_mut()
        .expect("CP385 supply-enthalpy owner");
    *enthalpy = different(*enthalpy);
    assert!(
        validate(
            &model,
            &output,
            &lifecycle,
            &predecessor,
            &forged_enthalpy_owner,
            1,
        )
        .is_err()
    );
}

#[test]
fn cp404_rejects_non_direct_route_accounting() {
    let (model, output, lifecycle, predecessor, enthalpy_owner) = validator_fixture(
        IdealLoadsLimit::LimitCapacity,
        0.020,
        f64::MIN_POSITIVE,
        1.0,
        1,
    );
    let mut forged_lifecycle = lifecycle.clone();
    let mut forged_predecessor = predecessor.clone();
    let public_active_route = [20, 24]
        .into_iter()
        .find(|index| forged_lifecycle.state.predecessor_route_counts[*index] > 0)
        .expect("fixture must select a public active route");
    forged_lifecycle.state.predecessor_route_counts[public_active_route] -= 1;
    forged_lifecycle.state.predecessor_route_counts[21] += 1;
    forged_predecessor.state.predecessor_route_counts[public_active_route] -= 1;
    forged_predecessor.state.predecessor_route_counts[21] += 1;
    if forged_lifecycle
        .state
        .supply_humidity_ratio_assignment_route_counts[public_active_route]
        > 0
    {
        forged_lifecycle
            .state
            .supply_humidity_ratio_assignment_route_counts[public_active_route] -= 1;
        forged_lifecycle
            .state
            .supply_humidity_ratio_assignment_route_counts[21] += 1;
        forged_predecessor
            .state
            .supply_temperature_mixed_air_assignment_route_counts[public_active_route] -= 1;
        forged_predecessor
            .state
            .supply_temperature_mixed_air_assignment_route_counts[21] += 1;
    } else {
        forged_lifecycle
            .state
            .predecessor_guard_false_fallthrough_route_counts[public_active_route] -= 1;
        forged_lifecycle
            .state
            .predecessor_guard_false_fallthrough_route_counts[21] += 1;
        forged_predecessor
            .state
            .predecessor_guard_false_fallthrough_route_counts[public_active_route] -= 1;
        forged_predecessor
            .state
            .predecessor_guard_false_fallthrough_route_counts[21] += 1;
    }
    assert!(
        validate(
            &model,
            &output,
            &forged_lifecycle,
            &forged_predecessor,
            &enthalpy_owner,
            1,
        )
        .is_err()
    );
}

#[test]
fn cp404_validation_has_no_numerical_coupling_dto_feed() {
    let (model, mut output, lifecycle, predecessor, enthalpy_owner) = validator_fixture(
        IdealLoadsLimit::LimitCapacity,
        0.020,
        f64::MIN_POSITIVE,
        1.0,
        1,
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
    output
        .coupling
        .purchased_air
        .calculation
        .zone_latent_cooling_rate_w = different(
        output
            .coupling
            .purchased_air
            .calculation
            .zone_latent_cooling_rate_w,
    );
    output.coupling.feedback.sum_sys_mcp_t_w = different(output.coupling.feedback.sum_sys_mcp_t_w);
    assert!(
        validate(
            &model,
            &output,
            &lifecycle,
            &predecessor,
            &enthalpy_owner,
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
    EnthalpyOwnerLifecycle,
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
    let cache = precompute_schedule_cache(&model.typed, 1).expect("CP404 schedule cache");
    let binding = bind_direct_zone_purchased_air_model(&model).expect("CP404 direct binding");
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
            .expect("CP404 coupling"),
        );
    }
    let output = latest.expect("at least one CP404 step");
    let system = output.initialization.system;
    let lifecycle = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment_lifecycle_summary(&runtime, system).expect("CP404 lifecycle");
    let predecessor = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment_lifecycle_summary(&runtime, system).expect("CP403 lifecycle");
    let enthalpy_owner = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_lifecycle_summary(&runtime, system).expect("CP385 lifecycle");
    (model, output, lifecycle, predecessor, enthalpy_owner)
}

fn validate(
    model: &SimulationModel,
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    lifecycle: &Lifecycle,
    predecessor: &PredecessorLifecycle,
    enthalpy_owner: &EnthalpyOwnerLifecycle,
    timestep_count: usize,
) -> Result<(), Error> {
    let binding = bind_direct_zone_purchased_air_model(model).map_err(|_| latest_violation())?;
    assert!(snapshot_matches_release(output, timestep_count, &binding));
    validate_lifecycle(
        lifecycle,
        predecessor,
        enthalpy_owner,
        timestep_count,
        output,
        &binding,
    )
}

fn latest_violation() -> Error {
    Error::CalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyHumidityRatioAssignmentLifecycleInvariant {
        field: "latest_release_snapshot_ready",
        expected: 1,
        actual: 0,
    }
}

fn different(value: f64) -> f64 {
    f64::from_bits(value.to_bits() ^ 1)
}
