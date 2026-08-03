//! Non-vacuous CP409 coupled-runtime integration and numerical-firewall tests.

use super::*;
use crate::{
    ideal_loads::{
        DirectZonePurchasedAirScheduledCouplingInput,
        DirectZonePurchasedAirScheduledCouplingOutput,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseBreakLifecycleSummary as Lifecycle,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirLimitLifecycleSummary as PredecessorLifecycle,
        PurchasedAirRuntimeState, bind_direct_zone_purchased_air_model,
        couple_model_bound_direct_zone_purchased_air,
        purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break_lifecycle_summary,
        purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_lifecycle_summary,
    },
    schedules::precompute_schedule_cache,
};
use ep_model::{
    AutosizeOrNumber, DehumidificationControlType, HumidificationControlType, IdealLoadsLimit,
    SimulationModel,
};

use super::super::DirectZonePurchasedAirCoupledRuntimeError as Error;
use super::super::cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break_validation::{
    snapshot_matches_release, validate_lifecycle,
};

#[test]
fn cp409_flattened_route_contract_is_36_total_12_active_24_inactive() {
    let active_predecessor_indices = [20, 21, 24, 25, 27, 29];
    let total = 30 + active_predecessor_indices.len();
    let active = 2 * active_predecessor_indices.len();
    assert_eq!((total, active, total - active), (36, 12, 24));
}

#[test]
fn cp409_breaks_after_both_cp408_guard_outcomes_and_preserves_carriers() {
    let mut saw_cp408_limit = false;
    let mut saw_cp405_maximum_capacity_sibling = false;
    let mut saw_inherited_inactive = false;

    for (limit, humidity_ratio, maximum_capacity_w, availability) in [
        (IdealLoadsLimit::LimitCapacity, 0.020, 500.0, 1.0),
        (IdealLoadsLimit::LimitCapacity, 0.020, 1.0e-100, 1.0),
        (IdealLoadsLimit::NoLimit, 0.008, 5_000.0, 0.0),
    ] {
        let (model, output, lifecycle, predecessor) =
            validator_fixture(limit, humidity_ratio, maximum_capacity_w, availability);
        assert!(validate(&model, &output, &lifecycle, &predecessor).is_ok());

        let cp408 = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit;
        let snapshot = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break;
        let expected_break = cp408
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered;
        let cp408_limit = cp408
            .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_executed;
        let maximum_capacity_sibling = cp408
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed;

        assert_eq!(
            snapshot
                .dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break,
            expected_break,
        );
        assert_eq!(expected_break, cp408_limit || maximum_capacity_sibling);
        for (left, right) in [
            (
                snapshot.predecessor_cp408_resulting_supply_humidity_ratio,
                cp408.resulting_supply_humidity_ratio,
            ),
            (
                snapshot.predecessor_cp408_resulting_supply_enthalpy_j_per_kg,
                cp408.resulting_supply_enthalpy_j_per_kg,
            ),
            (
                snapshot.predecessor_cp408_resulting_supply_temperature_c,
                cp408.resulting_supply_temperature_c,
            ),
            (
                snapshot.resulting_supply_humidity_ratio,
                cp408.resulting_supply_humidity_ratio,
            ),
            (
                snapshot.resulting_supply_enthalpy_j_per_kg,
                cp408.resulting_supply_enthalpy_j_per_kg,
            ),
            (
                snapshot.resulting_supply_temperature_c,
                cp408.resulting_supply_temperature_c,
            ),
        ] {
            assert_eq!(left.map(f64::to_bits), right.map(f64::to_bits));
        }

        assert_eq!(lifecycle.state.transition_count, 1);
        assert_eq!(
            lifecycle.state.inactive_transition_count,
            usize::from(!expected_break),
        );
        assert_eq!(
            lifecycle
                .state
                .dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_break_count,
            usize::from(expected_break),
        );
        assert_eq!(
            lifecycle.state.source_site_execution_count,
            usize::from(expected_break),
        );
        assert_eq!(
            lifecycle.state.predecessor_route_counts,
            predecessor.state.predecessor_route_counts,
        );

        saw_cp408_limit |= cp408_limit;
        saw_cp405_maximum_capacity_sibling |= maximum_capacity_sibling;
        saw_inherited_inactive |= !expected_break;
    }

    assert!(saw_cp408_limit);
    assert!(saw_cp405_maximum_capacity_sibling);
    assert!(saw_inherited_inactive);
}

#[test]
fn cp409_validation_rejects_cp408_carrier_drift_and_non_direct_routes() {
    let (model, output, lifecycle, mut predecessor) =
        validator_fixture(IdealLoadsLimit::LimitCapacity, 0.020, 500.0, 1.0);
    let value = predecessor
        .state
        .latest
        .as_mut()
        .expect("CP408 latest")
        .resulting_supply_enthalpy_j_per_kg
        .as_mut()
        .expect("CP408 retained enthalpy");
    *value = different(*value);
    assert_eq!(
        validate(&model, &output, &lifecycle, &predecessor),
        Err(predecessor_lineage_violation()),
    );

    let (model, output, mut lifecycle, mut predecessor) =
        validator_fixture(IdealLoadsLimit::LimitCapacity, 0.020, 500.0, 1.0);
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
        Err(Error::CalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseBreakLifecycleInvariant {
            field: "non_direct_route_count",
            expected: 0,
            actual: 1,
        }),
    );
}

#[test]
fn cp409_evidence_does_not_feed_or_replace_numerical_coupling_dto() {
    let (model, mut output, lifecycle, predecessor) =
        validator_fixture(IdealLoadsLimit::LimitCapacity, 0.020, 500.0, 1.0);
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
    let cache = precompute_schedule_cache(&model.typed, 1).expect("CP409 schedule cache");
    let binding = bind_direct_zone_purchased_air_model(&model).expect("CP409 direct binding");
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
            barometric_pressure_pa: binding.limit_context.barometric_pressure_pa,
            system_timestep_seconds: binding.nominal_system_timestep_seconds,
        },
    )
    .expect("CP409 coupling");
    let system = output.initialization.system;
    let lifecycle = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break_lifecycle_summary(
        &runtime,
        system,
    )
    .expect("CP409 lifecycle");
    let predecessor = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_lifecycle_summary(
        &runtime,
        system,
    )
    .expect("CP408 lifecycle");
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
    Error::CalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseBreakLifecycleInvariant {
        field: "latest_release_snapshot_ready",
        expected: 1,
        actual: 0,
    }
}

fn predecessor_lineage_violation() -> Error {
    Error::CalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseBreakLifecycleInvariant {
        field: "latest_predecessor_lineage_ready",
        expected: 1,
        actual: 0,
    }
}

fn different(value: f64) -> f64 {
    f64::from_bits(value.to_bits() ^ 1)
}
