//! Non-vacuous CP406 coupled-runtime integration tests.

use super::*;
use crate::{
    ideal_loads::{
        DirectZonePurchasedAirScheduledCouplingInput,
        DirectZonePurchasedAirScheduledCouplingOutput,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntryLifecycleSummary as Lifecycle,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputMaximumCapacityAssignmentLifecycleSummary as PredecessorLifecycle,
        PurchasedAirRuntimeState, bind_direct_zone_purchased_air_model,
        couple_model_bound_direct_zone_purchased_air,
        purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_lifecycle_summary,
        purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_lifecycle_summary,
    },
    schedules::precompute_schedule_cache,
};
use ep_model::{
    AutosizeOrNumber, DehumidificationControlType, HumidificationControlType, IdealLoadsLimit,
    SimulationModel,
};

use super::super::DirectZonePurchasedAirCoupledRuntimeError as Error;
use super::super::cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_validation::{
    snapshot_matches_release, validate_lifecycle,
};

#[test]
fn cp406_preserves_all_route_classes_and_enters_only_guard_false_else_routes() {
    let mut saw_else_entry = false;
    let mut saw_true_sibling_skip = false;
    let mut saw_inherited_inactive = false;
    for (limit, humidity_ratio, maximum_capacity_w, availability) in [
        (IdealLoadsLimit::LimitCapacity, 0.020, 500.0, 1.0),
        (IdealLoadsLimit::LimitCapacity, 0.020, 1.0e-100, 1.0),
        (IdealLoadsLimit::NoLimit, 0.008, 5_000.0, 0.0),
    ] {
        let (model, output, lifecycle, predecessor) =
            validator_fixture(limit, humidity_ratio, maximum_capacity_w, availability, 1);
        assert!(validate(&model, &output, &lifecycle, &predecessor, 1).is_ok());

        let predecessor_snapshot = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment;
        let snapshot = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry;
        let guard_false = predecessor_snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough;
        let assignment = predecessor_snapshot
            .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed;
        let else_entry = guard_false && !assignment;
        let inherited_inactive = !guard_false && !assignment;

        assert_eq!(
            snapshot.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entered,
            else_entry,
        );
        assert_eq!(lifecycle.state.transition_count, 1);
        assert_eq!(
            lifecycle.state.inactive_transition_count,
            usize::from(assignment || inherited_inactive),
        );
        assert_eq!(
            lifecycle
                .state
                .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_count,
            usize::from(else_entry),
        );
        assert_eq!(
            lifecycle.state.source_site_execution_count,
            usize::from(else_entry),
        );
        for (left, right) in [
            (
                snapshot.predecessor_cp405_resulting_supply_humidity_ratio,
                predecessor_snapshot.resulting_supply_humidity_ratio,
            ),
            (
                snapshot.predecessor_cp405_resulting_supply_enthalpy_j_per_kg,
                predecessor_snapshot.resulting_supply_enthalpy_j_per_kg,
            ),
            (
                snapshot.predecessor_cp405_resulting_supply_temperature_c,
                predecessor_snapshot.resulting_supply_temperature_c,
            ),
            (
                snapshot.resulting_supply_humidity_ratio,
                predecessor_snapshot.resulting_supply_humidity_ratio,
            ),
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

        saw_else_entry |= else_entry;
        saw_true_sibling_skip |= assignment;
        saw_inherited_inactive |= inherited_inactive;
    }
    assert!(saw_else_entry, "fixture set must enter CP406");
    assert!(
        saw_true_sibling_skip,
        "fixture set must skip the else sibling"
    );
    assert!(
        saw_inherited_inactive,
        "fixture set must preserve inactivity"
    );
}

#[test]
fn cp406_rejects_cp405_carrier_and_route_drift() {
    let (model, output, lifecycle, mut predecessor) =
        validator_fixture(IdealLoadsLimit::LimitCapacity, 0.020, 500.0, 1.0, 1);
    let value = predecessor
        .state
        .latest
        .as_mut()
        .expect("CP405 latest")
        .resulting_supply_enthalpy_j_per_kg
        .as_mut()
        .expect("CP405 enthalpy carrier");
    *value = different(*value);
    assert!(validate(&model, &output, &lifecycle, &predecessor, 1).is_err());

    let (model, output, mut lifecycle, predecessor) =
        validator_fixture(IdealLoadsLimit::LimitCapacity, 0.020, 500.0, 1.0, 1);
    lifecycle.state.predecessor_route_counts[21] += 1;
    assert!(validate(&model, &output, &lifecycle, &predecessor, 1).is_err());
}

#[test]
fn cp406_validation_has_no_numerical_coupling_dto_feed() {
    let (model, mut output, lifecycle, predecessor) =
        validator_fixture(IdealLoadsLimit::LimitCapacity, 0.020, 500.0, 1.0, 1);
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
    assert!(validate(&model, &output, &lifecycle, &predecessor, 1).is_ok());
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
    let cache = precompute_schedule_cache(&model.typed, 1).expect("CP406 schedule cache");
    let binding = bind_direct_zone_purchased_air_model(&model).expect("CP406 direct binding");
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
            .expect("CP406 coupling"),
        );
    }
    let output = latest.expect("at least one CP406 step");
    let system = output.initialization.system;
    let lifecycle = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_lifecycle_summary(&runtime, system).expect("CP406 lifecycle");
    let predecessor = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_lifecycle_summary(&runtime, system).expect("CP405 lifecycle");
    (model, output, lifecycle, predecessor)
}

fn validate(
    model: &SimulationModel,
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    lifecycle: &Lifecycle,
    predecessor: &PredecessorLifecycle,
    timestep_count: usize,
) -> Result<(), Error> {
    let binding = bind_direct_zone_purchased_air_model(model).map_err(|_| latest_violation())?;
    assert!(snapshot_matches_release(output, timestep_count, &binding));
    validate_lifecycle(lifecycle, predecessor, timestep_count, output, &binding)
}

fn latest_violation() -> Error {
    Error::CalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntryLifecycleInvariant {
        field: "latest_release_snapshot_ready",
        expected: 1,
        actual: 0,
    }
}

fn different(value: f64) -> f64 {
    f64::from_bits(value.to_bits() ^ 1)
}
