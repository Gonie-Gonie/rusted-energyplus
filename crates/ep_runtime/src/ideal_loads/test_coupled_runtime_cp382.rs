//! Non-vacuous CP382 coupled-runtime integration tests.

use super::*;
use crate::heat_balance::state::{ZoneAirTemperatureCoefficients, ZoneHeatBalanceState};
use crate::ideal_loads::{
    DirectZonePurchasedAirScheduledCouplingInput, DirectZonePurchasedAirScheduledCouplingOutput,
    PurchasedAirCalcCoolingMixedAirCallLifecycleSummary as MixedAirOwner,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentLifecycleSummary as EarlyTotalCorroborator,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardLifecycleSummary as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentLifecycleSummary as SupplyEnthalpyOwner,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardLifecycleSummary as SupplyMassFlowOwner,
    PurchasedAirRuntimeState, bind_direct_zone_purchased_air_model,
    couple_model_bound_direct_zone_purchased_air,
    purchased_air_calc_cooling_mixed_air_call_lifecycle_summary,
    purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle_summary,
    purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_lifecycle_summary,
    purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_lifecycle_summary,
    purchased_air_calc_cooling_supply_enthalpy_post_saturation_assignment_lifecycle_summary,
    purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle_summary,
};
use crate::schedules::precompute_schedule_cache;
use ep_model::{
    AutosizeOrNumber, DehumidificationControlType, HumidificationControlType, IdealLoadsLimit,
    SimulationModel, ZoneId,
};

use super::super::DirectZonePurchasedAirCoupledRuntimeError as Error;
use super::super::cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_validation::{
    snapshot_matches_release, validate_lifecycle,
};

#[test]
fn cp382_follows_cp381_and_locks_assignment_false_and_outer_false_shapes() {
    for (limit, humidity_ratio, expected_assignment, expected_sites) in [
        (IdealLoadsLimit::LimitCapacity, 0.020, true, 6),
        (IdealLoadsLimit::LimitCapacity, 0.008, false, 0),
        (IdealLoadsLimit::LimitFlowRateAndCapacity, 0.020, true, 6),
        (IdealLoadsLimit::NoLimit, 0.020, false, 0),
        (IdealLoadsLimit::LimitFlowRate, 0.020, false, 0),
    ] {
        let (
            model,
            output,
            lifecycle,
            predecessor,
            supply_mass_flow_owner,
            mixed_air_owner,
            early_total_corroborator,
            supply_enthalpy_owner,
        ) = validator_fixture(limit, humidity_ratio);
        assert!(
            validate(
                &model,
                &output,
                &lifecycle,
                &predecessor,
                &supply_mass_flow_owner,
                &mixed_air_owner,
                &early_total_corroborator,
                &supply_enthalpy_owner,
            )
            .is_ok()
        );
        let snapshot = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment;
        assert_eq!(lifecycle.state.transition_count, 1);
        assert_eq!(lifecycle.state.source_site_execution_count, expected_sites);
        assert_eq!(
            snapshot.dehumidification_total_output_assignment_executed,
            expected_assignment,
        );
        assert_eq!(
            lifecycle
                .state
                .dehumidification_total_output_assignment_count,
            usize::from(expected_assignment),
        );
        assert_eq!(
            lifecycle
                .state
                .dehumidification_total_output_assignment_count,
            predecessor.state.dehumidification_body_entry_count,
        );
    }
}

#[test]
fn cp382_rejects_owner_bit_corruption_but_does_not_consume_numerical_results() {
    let (
        model,
        output,
        lifecycle,
        predecessor,
        supply_mass_flow_owner,
        mixed_air_owner,
        early_total_corroborator,
        supply_enthalpy_owner,
    ) = validator_fixture(IdealLoadsLimit::LimitCapacity, 0.020);
    let binding = bind_direct_zone_purchased_air_model(&model).expect("direct binding");
    assert!(snapshot_matches_release(&output, 1, &binding));

    let mut changed_numerical = output;
    changed_numerical
        .coupling
        .purchased_air
        .calculation
        .supply_enthalpy_j_per_kg = f64::from_bits(
        changed_numerical
            .coupling
            .purchased_air
            .calculation
            .supply_enthalpy_j_per_kg
            .to_bits()
            ^ 1,
    );
    changed_numerical.coupling.feedback.sum_sys_mcp_t_w = f64::from_bits(
        changed_numerical
            .coupling
            .feedback
            .sum_sys_mcp_t_w
            .to_bits()
            ^ 1,
    );
    assert!(snapshot_matches_release(&changed_numerical, 1, &binding));

    let mut bad_owner = supply_mass_flow_owner.clone();
    let owned_value = bad_owner
        .state
        .latest
        .as_mut()
        .expect("CP330 latest")
        .supply_mass_flow_rate_kg_per_s
        .as_mut()
        .expect("active CP330 value");
    *owned_value = f64::from_bits(owned_value.to_bits() ^ 1);
    assert_eq!(
        validate(
            &model,
            &output,
            &lifecycle,
            &predecessor,
            &bad_owner,
            &mixed_air_owner,
            &early_total_corroborator,
            &supply_enthalpy_owner,
        ),
        Err(
            Error::CalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentLifecycleInvariant {
                field: "latest_owner_lineage_ready",
                expected: 1,
                actual: 0,
            }
        ),
    );
}

#[allow(clippy::type_complexity)]
fn validator_fixture(
    cooling_limit: IdealLoadsLimit,
    air_humidity_ratio: f64,
) -> (
    SimulationModel,
    DirectZonePurchasedAirScheduledCouplingOutput,
    Lifecycle,
    Predecessor,
    SupplyMassFlowOwner,
    MixedAirOwner,
    EarlyTotalCorroborator,
    SupplyEnthalpyOwner,
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
    .then_some(AutosizeOrNumber::Value(5_000.0));
    system.dehumidification_control_type = DehumidificationControlType::None;
    system.humidification_control_type = HumidificationControlType::None;
    system.minimum_cooling_supply_air_humidity_ratio = f64::NAN;
    let model = SimulationModel::from_typed(typed);
    let cache = precompute_schedule_cache(&model.typed, 1).expect("CP382 schedule cache");
    let binding = bind_direct_zone_purchased_air_model(&model).expect("CP382 direct binding");
    let mut zone_state =
        cooling_zone_state(binding.nominal_system_timestep_seconds, air_humidity_ratio);
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
    .expect("CP382 coupling");
    let system = output.initialization.system;
    let lifecycle = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_lifecycle_summary(&runtime, system).expect("CP382 lifecycle");
    let predecessor = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_lifecycle_summary(&runtime, system).expect("CP381 lifecycle");
    let supply_mass_flow_owner =
        purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle_summary(
            &runtime, system,
        )
        .expect("CP330 lifecycle");
    let mixed_air_owner =
        purchased_air_calc_cooling_mixed_air_call_lifecycle_summary(&runtime, system)
            .expect("CP329 lifecycle");
    let early_total_corroborator = purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle_summary(&runtime, system).expect("CP339 lifecycle");
    let supply_enthalpy_owner =
        purchased_air_calc_cooling_supply_enthalpy_post_saturation_assignment_lifecycle_summary(
            &runtime, system,
        )
        .expect("CP379 lifecycle");
    (
        model,
        output,
        lifecycle,
        predecessor,
        supply_mass_flow_owner,
        mixed_air_owner,
        early_total_corroborator,
        supply_enthalpy_owner,
    )
}

#[allow(clippy::too_many_arguments)]
fn validate(
    model: &SimulationModel,
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    lifecycle: &Lifecycle,
    predecessor: &Predecessor,
    supply_mass_flow_owner: &SupplyMassFlowOwner,
    mixed_air_owner: &MixedAirOwner,
    early_total_corroborator: &EarlyTotalCorroborator,
    supply_enthalpy_owner: &SupplyEnthalpyOwner,
) -> Result<(), Error> {
    let binding = bind_direct_zone_purchased_air_model(model).map_err(|_| {
        Error::CalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentLifecycleInvariant {
            field: "latest_release_snapshot_ready",
            expected: 1,
            actual: 0,
        }
    })?;
    assert!(snapshot_matches_release(output, 1, &binding));
    validate_lifecycle(
        lifecycle,
        predecessor,
        supply_mass_flow_owner,
        mixed_air_owner,
        early_total_corroborator,
        supply_enthalpy_owner,
        1,
        output,
        &binding,
    )
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
