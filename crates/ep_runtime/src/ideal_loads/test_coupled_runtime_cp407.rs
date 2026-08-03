//! Non-vacuous CP407 coupled-runtime integration and numerical-firewall tests.

use super::*;
use crate::{
    ideal_loads::{
        DirectZonePurchasedAirScheduledCouplingInput,
        DirectZonePurchasedAirScheduledCouplingOutput,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntryLifecycleSummary as PredecessorLifecycle,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentLifecycleSummary as Lifecycle,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentLifecycleSummary as EnthalpyLifecycle,
        PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentLifecycleSummary as HumidityLifecycle,
        PurchasedAirRuntimeState, bind_direct_zone_purchased_air_model,
        couple_model_bound_direct_zone_purchased_air,
        purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_lifecycle_summary,
        purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_lifecycle_summary,
        purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_lifecycle_summary,
        purchased_air_calc_cooling_supply_humidity_ratio_saturation_limit_assignment_lifecycle_summary,
    },
    schedules::precompute_schedule_cache,
};
use ep_model::{
    AutosizeOrNumber, DehumidificationControlType, HumidificationControlType, IdealLoadsLimit,
    SimulationModel,
};

use super::super::DirectZonePurchasedAirCoupledRuntimeError as Error;
use super::super::cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_validation::{
    snapshot_matches_release, validate_lifecycle,
};

#[test]
fn cp407_preserves_three_partitions_and_uses_only_cp378_cp385_operands() {
    let mut saw_assignment = false;
    let mut saw_cp405_sibling = false;
    let mut saw_inherited_inactive = false;
    for (limit, humidity_ratio, maximum_capacity_w, availability) in [
        (IdealLoadsLimit::LimitCapacity, 0.020, 500.0, 1.0),
        (IdealLoadsLimit::LimitCapacity, 0.020, 1.0e-100, 1.0),
        (IdealLoadsLimit::NoLimit, 0.008, 5_000.0, 0.0),
    ] {
        let (model, output, lifecycle, predecessor, humidity, enthalpy) =
            validator_fixture(limit, humidity_ratio, maximum_capacity_w, availability);
        assert!(
            validate(
                &model,
                &output,
                &lifecycle,
                &predecessor,
                &humidity,
                &enthalpy
            )
            .is_ok()
        );

        let predecessor_snapshot = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry;
        let snapshot = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment;
        let assignment = predecessor_snapshot
            .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entered;
        assert_eq!(
            snapshot.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_executed,
            assignment,
        );
        assert_eq!(lifecycle.state.transition_count, 1);
        assert_eq!(
            lifecycle.state.inactive_transition_count,
            usize::from(!assignment)
        );
        assert_eq!(
            lifecycle.state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_count,
            usize::from(assignment),
        );
        assert_eq!(
            lifecycle.state.source_site_execution_count,
            4 * usize::from(assignment)
        );

        if assignment {
            let owner_h = output
                .calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment
                .resulting_supply_enthalpy_j_per_kg
                .expect("CP385 H owner");
            let owner_w = output
                .calculation_cooling_supply_humidity_ratio_saturation_limit_assignment
                .resulting_supply_humidity_ratio
                .expect("CP378 W owner");
            let expected = crate::psychrometrics::energyplus_psy_tdb_fn_h_w(owner_h, owner_w);
            assert_eq!(
                snapshot.supply_enthalpy_j_per_kg.map(f64::to_bits),
                Some(owner_h.to_bits())
            );
            assert_eq!(
                snapshot.supply_humidity_ratio.map(f64::to_bits),
                Some(owner_w.to_bits())
            );
            assert_eq!(
                snapshot.assigned_supply_temperature_c.map(f64::to_bits),
                Some(expected.to_bits())
            );
            assert_eq!(
                snapshot.resulting_supply_temperature_c.map(f64::to_bits),
                Some(expected.to_bits())
            );
        }

        saw_assignment |= assignment;
        saw_cp405_sibling |= predecessor_snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed;
        saw_inherited_inactive |= !assignment
            && !predecessor_snapshot
                .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed;
    }
    assert!(saw_assignment);
    assert!(saw_cp405_sibling);
    assert!(saw_inherited_inactive);
}

#[test]
fn cp407_validation_rejects_owner_and_route_drift() {
    let (model, output, lifecycle, predecessor, humidity, mut enthalpy) =
        validator_fixture(IdealLoadsLimit::LimitCapacity, 0.020, 500.0, 1.0);
    let value = enthalpy
        .state
        .latest
        .as_mut()
        .expect("CP385 latest")
        .resulting_supply_enthalpy_j_per_kg
        .as_mut()
        .expect("CP385 H owner");
    *value = different(*value);
    assert!(
        validate(
            &model,
            &output,
            &lifecycle,
            &predecessor,
            &humidity,
            &enthalpy
        )
        .is_err()
    );

    let (model, output, mut lifecycle, predecessor, humidity, enthalpy) =
        validator_fixture(IdealLoadsLimit::LimitCapacity, 0.020, 500.0, 1.0);
    lifecycle.state.predecessor_route_counts[21] += 1;
    assert!(
        validate(
            &model,
            &output,
            &lifecycle,
            &predecessor,
            &humidity,
            &enthalpy
        )
        .is_err()
    );
}

#[test]
fn cp407_evidence_does_not_feed_or_replace_numerical_coupling_dto() {
    let (model, mut output, lifecycle, predecessor, humidity, enthalpy) =
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
    assert!(
        validate(
            &model,
            &output,
            &lifecycle,
            &predecessor,
            &humidity,
            &enthalpy
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
) -> (
    SimulationModel,
    DirectZonePurchasedAirScheduledCouplingOutput,
    Lifecycle,
    PredecessorLifecycle,
    HumidityLifecycle,
    EnthalpyLifecycle,
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
    let cache = precompute_schedule_cache(&model.typed, 1).expect("CP407 schedule cache");
    let binding = bind_direct_zone_purchased_air_model(&model).expect("CP407 direct binding");
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
    .expect("CP407 coupling");
    let system = output.initialization.system;
    let lifecycle = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_lifecycle_summary(&runtime, system).expect("CP407 lifecycle");
    let predecessor = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_lifecycle_summary(&runtime, system).expect("CP406 lifecycle");
    let humidity = purchased_air_calc_cooling_supply_humidity_ratio_saturation_limit_assignment_lifecycle_summary(&runtime, system).expect("CP378 lifecycle");
    let enthalpy = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_lifecycle_summary(&runtime, system).expect("CP385 lifecycle");
    (model, output, lifecycle, predecessor, humidity, enthalpy)
}

fn validate(
    model: &SimulationModel,
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    lifecycle: &Lifecycle,
    predecessor: &PredecessorLifecycle,
    humidity: &HumidityLifecycle,
    enthalpy: &EnthalpyLifecycle,
) -> Result<(), Error> {
    let binding = bind_direct_zone_purchased_air_model(model).map_err(|_| latest_violation())?;
    assert!(snapshot_matches_release(output, 1, &binding));
    validate_lifecycle(
        lifecycle,
        predecessor,
        humidity,
        enthalpy,
        1,
        output,
        &binding,
    )
}

fn latest_violation() -> Error {
    Error::CalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentLifecycleInvariant {
        field: "latest_release_snapshot_ready",
        expected: 1,
        actual: 0,
    }
}

fn different(value: f64) -> f64 {
    f64::from_bits(value.to_bits() ^ 1)
}
