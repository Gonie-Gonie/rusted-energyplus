//! Non-vacuous CP399 coupled-runtime integration tests.

use super::*;
use crate::{
    ideal_loads::{
        DirectZonePurchasedAirScheduledCouplingInput,
        DirectZonePurchasedAirScheduledCouplingOutput,
        PurchasedAirCalcCoolingMixedAirCallLifecycleSummary as MixedAirLifecycle,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseEntryLifecycleSummary as PredecessorLifecycle,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentLifecycleSummary as Lifecycle,
        PurchasedAirRuntimeState, bind_direct_zone_purchased_air_model,
        couple_model_bound_direct_zone_purchased_air,
        purchased_air_calc_cooling_mixed_air_call_lifecycle_summary,
        purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry_lifecycle_summary,
        purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_lifecycle_summary,
    },
    psychrometrics::energyplus_psy_cp_air_fn_w,
    schedules::precompute_schedule_cache,
};
use ep_model::{
    AutosizeOrNumber, DehumidificationControlType, HumidificationControlType, IdealLoadsLimit,
    SimulationModel,
};

use super::super::DirectZonePurchasedAirCoupledRuntimeError as Error;
use super::super::cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_validation::{
    snapshot_matches_release, validate_lifecycle,
};

#[test]
fn cp399_executes_public_none_routes_from_cp329_and_preserves_all_carriers() {
    let mut saw_active = false;
    let mut saw_inactive = false;
    for (limit, availability, capacity) in [
        (IdealLoadsLimit::NoLimit, 0.0, 5_000.0),
        (IdealLoadsLimit::NoLimit, 1.0, 5_000.0),
        (IdealLoadsLimit::LimitCapacity, 1.0, 500.0),
    ] {
        let (model, output, lifecycle, predecessor, mixed_air) =
            validator_fixture(limit, 0.020, capacity, availability, 1);
        assert!(validate(&model, &output, &lifecycle, &predecessor, &mixed_air, 1).is_ok());
        let snapshot = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment;
        let cp398 = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry;
        let active = cp398
            .dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered;
        assert_eq!(
            snapshot
                .dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed,
            active
        );
        assert_eq!(snapshot.mixed_air_humidity_ratio_read, active);
        assert_eq!(snapshot.psychrometric_cp_air_evaluated, active);
        assert_eq!(snapshot.cp_air_assigned, active);
        if active {
            let owner = output
                .calculation_cooling_mixed_air_call
                .mixed_air_humidity_ratio
                .expect("active CP399 CP329 owner");
            let expected = energyplus_psy_cp_air_fn_w(owner);
            assert_eq!(
                snapshot.mixed_air_humidity_ratio.map(f64::to_bits),
                Some(owner.to_bits())
            );
            assert_eq!(
                snapshot.cp_air_j_per_kg_k.map(f64::to_bits),
                Some(expected.to_bits())
            );
            assert_eq!(
                snapshot
                    .psychrometric_cp_air_result_j_per_kg_k
                    .map(f64::to_bits),
                Some(expected.to_bits())
            );
        } else {
            assert!(snapshot.mixed_air_humidity_ratio.is_none());
            assert!(snapshot.psychrometric_cp_air_result_j_per_kg_k.is_none());
            assert!(snapshot.cp_air_j_per_kg_k.is_none());
        }
        for (left, right) in [
            (
                snapshot.predecessor_cp397_resulting_supply_humidity_ratio,
                cp398.predecessor_cp397_resulting_supply_humidity_ratio,
            ),
            (
                snapshot.predecessor_cp397_resulting_supply_enthalpy_j_per_kg,
                cp398.predecessor_cp397_resulting_supply_enthalpy_j_per_kg,
            ),
            (
                snapshot.predecessor_cp397_resulting_supply_temperature_c,
                cp398.predecessor_cp397_resulting_supply_temperature_c,
            ),
            (
                snapshot.predecessor_cp398_resulting_supply_humidity_ratio,
                cp398.resulting_supply_humidity_ratio,
            ),
            (
                snapshot.predecessor_cp398_resulting_supply_enthalpy_j_per_kg,
                cp398.resulting_supply_enthalpy_j_per_kg,
            ),
            (
                snapshot.predecessor_cp398_resulting_supply_temperature_c,
                cp398.resulting_supply_temperature_c,
            ),
            (
                snapshot.resulting_supply_humidity_ratio,
                cp398.resulting_supply_humidity_ratio,
            ),
            (
                snapshot.resulting_supply_enthalpy_j_per_kg,
                cp398.resulting_supply_enthalpy_j_per_kg,
            ),
            (
                snapshot.resulting_supply_temperature_c,
                cp398.resulting_supply_temperature_c,
            ),
        ] {
            assert_eq!(left.map(f64::to_bits), right.map(f64::to_bits));
        }
        let assignments = usize::from(active);
        assert_eq!(lifecycle.state.inactive_transition_count, 1 - assignments);
        assert_eq!(
            lifecycle
                .state
                .dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_count,
            assignments
        );
        assert_eq!(lifecycle.state.source_site_execution_count, 3 * assignments);
        assert_eq!(
            lifecycle.state.mixed_air_humidity_ratio_read_count,
            assignments
        );
        assert_eq!(
            lifecycle.state.psychrometric_cp_air_evaluation_count,
            assignments
        );
        assert_eq!(lifecycle.state.cp_air_assignment_write_count, assignments);
        saw_active |= active;
        saw_inactive |= !active;
    }
    assert!(
        saw_active,
        "fixtures must execute public CP399 route 20 or 24"
    );
    assert!(
        saw_inactive,
        "fixtures must also exercise an inactive public route"
    );
}

#[test]
fn cp399_rejects_cp398_and_cp329_bit_drift() {
    let (model, output, lifecycle, mut predecessor, mixed_air) =
        validator_fixture(IdealLoadsLimit::LimitCapacity, 0.020, 500.0, 1.0, 1);
    let value = predecessor
        .state
        .latest
        .as_mut()
        .expect("CP398 latest")
        .resulting_supply_enthalpy_j_per_kg
        .as_mut()
        .expect("CP398 retained enthalpy");
    *value = different(*value);
    assert_eq!(
        validate(&model, &output, &lifecycle, &predecessor, &mixed_air, 1),
        Err(latest_violation())
    );

    let (model, output, lifecycle, predecessor, mut mixed_air) =
        validator_fixture(IdealLoadsLimit::LimitCapacity, 0.020, 500.0, 1.0, 1);
    let value = mixed_air
        .state
        .latest
        .as_mut()
        .expect("CP329 latest")
        .mixed_air_humidity_ratio
        .as_mut()
        .expect("CP329 mixed-air humidity");
    *value = different(*value);
    assert_eq!(
        validate(&model, &output, &lifecycle, &predecessor, &mixed_air, 1),
        Err(active_operand_owner_violation())
    );
}

#[test]
fn cp399_validation_is_independent_of_numerical_output_state() {
    let (model, mut output, lifecycle, predecessor, mixed_air) =
        validator_fixture(IdealLoadsLimit::LimitCapacity, 0.020, 500.0, 1.0, 2);
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
        .supply_enthalpy_j_per_kg = different(
        output
            .coupling
            .purchased_air
            .calculation
            .supply_enthalpy_j_per_kg,
    );
    output
        .coupling
        .purchased_air
        .supply_node_update
        .humidity_ratio = different(
        output
            .coupling
            .purchased_air
            .supply_node_update
            .humidity_ratio,
    );
    output.coupling.feedback.sum_sys_mcp_t_w = different(output.coupling.feedback.sum_sys_mcp_t_w);
    assert!(validate(&model, &output, &lifecycle, &predecessor, &mixed_air, 2).is_ok());
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
    PredecessorLifecycle,
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
    let cache = precompute_schedule_cache(&model.typed, 1).expect("CP399 schedule cache");
    let binding = bind_direct_zone_purchased_air_model(&model).expect("CP399 direct binding");
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
            .expect("CP399 coupling"),
        );
    }
    let output = latest.expect("at least one CP399 step");
    let system = output.initialization.system;
    let lifecycle = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_lifecycle_summary(&runtime, system).expect("CP399 lifecycle");
    let predecessor = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry_lifecycle_summary(&runtime, system).expect("CP398 lifecycle");
    let mixed_air = purchased_air_calc_cooling_mixed_air_call_lifecycle_summary(&runtime, system)
        .expect("CP329 lifecycle");
    (model, output, lifecycle, predecessor, mixed_air)
}

fn validate(
    model: &SimulationModel,
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    lifecycle: &Lifecycle,
    predecessor: &PredecessorLifecycle,
    mixed_air: &MixedAirLifecycle,
    timestep_count: usize,
) -> Result<(), Error> {
    let binding = bind_direct_zone_purchased_air_model(model).map_err(|_| latest_violation())?;
    assert!(snapshot_matches_release(output, timestep_count, &binding));
    validate_lifecycle(
        lifecycle,
        predecessor,
        mixed_air,
        timestep_count,
        output,
        &binding,
    )
}

fn latest_violation() -> Error {
    Error::CalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentLifecycleInvariant {
        field: "latest_release_snapshot_ready",
        expected: 1,
        actual: 0,
    }
}

fn active_operand_owner_violation() -> Error {
    Error::CalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentLifecycleInvariant {
        field: "active_operand_owner_ready",
        expected: 1,
        actual: 0,
    }
}

fn different(value: f64) -> f64 {
    f64::from_bits(value.to_bits() ^ 1)
}
