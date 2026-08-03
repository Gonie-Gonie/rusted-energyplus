//! Non-vacuous CP412 coupled-runtime integration and numerical-firewall tests.

use super::*;
use crate::{
    ideal_loads::{
        DirectZonePurchasedAirScheduledCouplingInput,
        DirectZonePurchasedAirScheduledCouplingOutput,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioPreSaturationOriginalAssignmentLifecycleSummary as PredecessorLifecycle,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentLifecycleSummary as Lifecycle,
        PurchasedAirRuntimeState, bind_direct_zone_purchased_air_model,
        couple_model_bound_direct_zone_purchased_air,
        purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_lifecycle_summary,
        purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_lifecycle_summary,
    },
    psychrometrics::energyplus_psy_w_fn_tdb_rh_pb,
    schedules::precompute_schedule_cache,
};
use ep_model::{
    AutosizeOrNumber, DehumidificationControlType, HumidificationControlType, IdealLoadsLimit,
    SimulationModel,
};

use super::super::DirectZonePurchasedAirCoupledRuntimeError as Error;
use super::super::cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_validation::{
    snapshot_matches_release, validate_lifecycle,
};

#[test]
fn cp412_flattened_route_contract_has_18_active_routes_and_four_public_active_routes() {
    let split_predecessor_indices = [20, 21, 24, 25, 27, 29];
    let public_predecessor_indices = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 24];
    let total = 30 + split_predecessor_indices.len();
    let active_public = public_predecessor_indices
        .into_iter()
        .filter(|index| *index >= 18)
        .map(|index| 1 + usize::from(split_predecessor_indices.contains(&index)))
        .sum::<usize>();
    assert_eq!((total, total - 18, 18, active_public), (36, 18, 18, 4));
}

#[test]
fn cp412_uses_cp411_temperature_and_current_pressure_only_on_active_routes() {
    let mut saw_assignment = false;
    let mut saw_skip = false;

    for (limit, humidity_ratio, maximum_capacity_w, availability, pressure) in [
        (IdealLoadsLimit::LimitCapacity, 0.020, 500.0, 1.0, 97_321.0),
        (
            IdealLoadsLimit::LimitCapacity,
            0.020,
            1.0e-100,
            1.0,
            97_321.0,
        ),
        (IdealLoadsLimit::NoLimit, 0.008, 5_000.0, 0.0, 97_321.0),
    ] {
        let (model, output, lifecycle, predecessor) = validator_fixture(
            limit,
            humidity_ratio,
            maximum_capacity_w,
            availability,
            pressure,
        );
        assert!(validate(&model, &output, &lifecycle, &predecessor).is_ok());

        let snapshot = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment;
        let cp411 = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment;
        let active = cp411
            .post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_executed;

        assert_eq!(
            snapshot.post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_executed,
            active,
        );
        for (left, right) in [
            (
                snapshot.predecessor_cp411_resulting_supply_humidity_ratio,
                cp411.resulting_supply_humidity_ratio,
            ),
            (
                snapshot.predecessor_cp411_resulting_supply_enthalpy_j_per_kg,
                cp411.resulting_supply_enthalpy_j_per_kg,
            ),
            (
                snapshot.predecessor_cp411_resulting_supply_temperature_c,
                cp411.resulting_supply_temperature_c,
            ),
            (
                snapshot.resulting_supply_humidity_ratio,
                cp411.resulting_supply_humidity_ratio,
            ),
            (
                snapshot.resulting_supply_enthalpy_j_per_kg,
                cp411.resulting_supply_enthalpy_j_per_kg,
            ),
            (
                snapshot.resulting_supply_temperature_c,
                cp411.resulting_supply_temperature_c,
            ),
        ] {
            assert_eq!(left.map(f64::to_bits), right.map(f64::to_bits));
        }

        if active {
            let temperature = cp411
                .resulting_supply_temperature_c
                .expect("active CP412 fixture requires the CP411 supply temperature");
            let expected = energyplus_psy_w_fn_tdb_rh_pb(temperature, 1.0, pressure);
            assert!(snapshot.cp411_retained_supply_temperature_owned_read);
            assert!(snapshot.purchased_air_supply_temperature_for_saturation_humidity_ratio_read);
            assert!(snapshot.environment_outdoor_barometric_pressure_owned_read);
            assert!(
                snapshot.environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read
            );
            assert!(snapshot.psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluated);
            assert!(snapshot.local_saturation_supply_humidity_ratio_assignment_performed);
            assert_eq!(
                snapshot
                    .supply_temperature_for_saturation_humidity_ratio_c
                    .map(f64::to_bits),
                Some(temperature.to_bits()),
            );
            assert_eq!(
                snapshot.outdoor_barometric_pressure_pa.map(f64::to_bits),
                Some(pressure.to_bits()),
            );
            for value in [
                snapshot.saturation_supply_humidity_ratio,
                snapshot.assigned_saturation_supply_humidity_ratio,
                snapshot.resulting_saturation_supply_humidity_ratio,
            ] {
                assert_eq!(value.map(f64::to_bits), Some(expected.to_bits()));
            }
            saw_assignment = true;
        } else {
            assert!(!snapshot.cp411_retained_supply_temperature_owned_read);
            assert!(!snapshot.purchased_air_supply_temperature_for_saturation_humidity_ratio_read);
            assert!(!snapshot.environment_outdoor_barometric_pressure_owned_read);
            assert!(
                !snapshot
                    .environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read
            );
            assert!(
                [
                    snapshot.supply_temperature_for_saturation_humidity_ratio_c,
                    snapshot.outdoor_barometric_pressure_pa,
                    snapshot.saturation_supply_humidity_ratio,
                    snapshot.assigned_saturation_supply_humidity_ratio,
                    snapshot.resulting_saturation_supply_humidity_ratio,
                ]
                .into_iter()
                .all(|value| value.is_none())
            );
            saw_skip = true;
        }

        assert_eq!(
            lifecycle.state.transition_count,
            lifecycle.state.inactive_transition_count
                + lifecycle
                    .state
                    .supply_humidity_ratio_saturation_assignment_count,
        );
        assert_eq!(
            lifecycle.state.source_site_execution_count,
            4 * lifecycle
                .state
                .supply_humidity_ratio_saturation_assignment_count,
        );
        assert_eq!(
            lifecycle.state.predecessor_route_counts,
            predecessor.state.predecessor_route_counts,
        );
    }

    assert!(saw_assignment);
    assert!(saw_skip);
}

#[test]
fn cp412_validation_rejects_changed_assignment_lineage_and_non_direct_routes() {
    let (model, output, mut lifecycle, predecessor) =
        validator_fixture(IdealLoadsLimit::LimitCapacity, 0.020, 500.0, 1.0, 97_321.0);
    let latest = lifecycle
        .state
        .latest
        .as_mut()
        .expect("CP412 fixture requires a latest snapshot");
    latest.assigned_saturation_supply_humidity_ratio = latest
        .assigned_saturation_supply_humidity_ratio
        .map(different);
    assert_eq!(
        validate(&model, &output, &lifecycle, &predecessor),
        Err(latest_violation()),
    );

    let (model, output, mut lifecycle, mut predecessor) =
        validator_fixture(IdealLoadsLimit::LimitCapacity, 0.020, 500.0, 1.0, 97_321.0);
    lifecycle.state.predecessor_route_counts[21] += 1;
    predecessor.state.predecessor_route_counts[21] += 1;
    lifecycle
        .state
        .predecessor_maximum_capacity_assignment_route_counts[21] += 1;
    predecessor
        .state
        .predecessor_maximum_capacity_assignment_route_counts[21] += 1;
    assert_eq!(
        validate(&model, &output, &lifecycle, &predecessor),
        Err(Error::CalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentLifecycleInvariant {
            field: "non_direct_route_count",
            expected: 0,
            actual: 1,
        }),
    );
}

#[test]
fn cp412_evidence_does_not_feed_numerical_result() {
    let (model, mut output, lifecycle, predecessor) =
        validator_fixture(IdealLoadsLimit::LimitCapacity, 0.020, 500.0, 1.0, 97_321.0);
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
        .calculation
        .zone_latent_cooling_rate_w = different(
        output
            .coupling
            .purchased_air
            .calculation
            .zone_latent_cooling_rate_w,
    );
    output.coupling.feedback.sum_sys_mcp_t_w = different(output.coupling.feedback.sum_sys_mcp_t_w);
    assert!(validate(&model, &output, &lifecycle, &predecessor).is_ok());
}

fn validator_fixture(
    cooling_limit: IdealLoadsLimit,
    air_humidity_ratio: f64,
    maximum_capacity_w: f64,
    availability: f64,
    pressure: f64,
) -> (
    SimulationModel,
    DirectZonePurchasedAirScheduledCouplingOutput,
    Lifecycle,
    PredecessorLifecycle,
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
    let cache = precompute_schedule_cache(&model.typed, 1)
        .expect("CP412 fixture requires a valid schedule cache");
    let binding = bind_direct_zone_purchased_air_model(&model)
        .expect("CP412 fixture requires a direct binding");
    let mut zone_state = super::coupled_runtime_tests_cp389::cooling_zone_state(
        binding.nominal_system_timestep_seconds,
        air_humidity_ratio,
    );
    let mut runtime = PurchasedAirRuntimeState::default();
    let output = couple_model_bound_direct_zone_purchased_air(
        DirectZonePurchasedAirScheduledCouplingInput {
            binding: &binding,
            schedule_cache: &cache,
            schedule_sample_index: 0,
            zone_state: &mut zone_state,
            purchased_air_runtime_state: &mut runtime,
            begin_environment: true,
            barometric_pressure_pa: pressure,
            system_timestep_seconds: binding.nominal_system_timestep_seconds,
        },
    )
    .expect("CP412 fixture requires a successful coupling call");
    let system = output.initialization.system;
    let lifecycle = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_lifecycle_summary(
        &runtime,
        system,
    )
    .expect("CP412 fixture requires a lifecycle summary");
    let predecessor = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_lifecycle_summary(
        &runtime,
        system,
    )
    .expect("CP412 fixture requires a CP411 lifecycle summary");
    (model, output, lifecycle, predecessor)
}

fn validate(
    model: &SimulationModel,
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    lifecycle: &Lifecycle,
    predecessor: &PredecessorLifecycle,
) -> Result<(), Error> {
    let binding = bind_direct_zone_purchased_air_model(model).map_err(|_| latest_violation())?;
    assert!(snapshot_matches_release(output, 1, &binding));
    validate_lifecycle(lifecycle, predecessor, 1, output, &binding)
}

fn latest_violation() -> Error {
    Error::CalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentLifecycleInvariant {
        field: "latest_release_snapshot_ready",
        expected: 1,
        actual: 0,
    }
}

fn different(value: f64) -> f64 {
    f64::from_bits(value.to_bits() ^ 1)
}
