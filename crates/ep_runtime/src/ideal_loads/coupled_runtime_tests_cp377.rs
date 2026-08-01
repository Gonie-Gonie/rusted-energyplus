use super::*;
use crate::heat_balance::state::{ZoneAirTemperatureCoefficients, ZoneHeatBalanceState};
use crate::ideal_loads::{
    DirectZonePurchasedAirScheduledCouplingInput, DirectZonePurchasedAirScheduledCouplingOutput,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitLifecycleSummary as Cp344Lifecycle,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitLifecycleSummary as Cp334Lifecycle,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirRuntimeState, bind_direct_zone_purchased_air_model,
    couple_model_bound_direct_zone_purchased_air,
    purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle_summary,
    purchased_air_calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle_summary,
    purchased_air_calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment_lifecycle_summary,
    purchased_air_calc_cooling_supply_humidity_ratio_saturation_assignment_lifecycle_summary,
};
use crate::schedules::precompute_schedule_cache;
use ep_model::{DehumidificationControlType, HumidificationControlType, SimulationModel, ZoneId};

use super::super::DirectZonePurchasedAirCoupledRuntimeError as Error;
use super::super::cooling_supply_humidity_ratio_saturation_assignment_validation::{
    snapshot_matches_release, snapshots_match_exact_bits, validate_lifecycle,
};

#[test]
fn cp377_follows_cp376_records_four_sites_and_does_not_feed_numerical_result() {
    let Some((model, output, lifecycle, predecessor, cp334, cp344)) = validator_fixture() else {
        return;
    };
    assert!(validate(&model, &output, &lifecycle, &predecessor, &cp334, &cp344).is_ok());
    let state = &lifecycle.state;
    let snapshot = output.calculation_cooling_supply_humidity_ratio_saturation_assignment;
    assert_eq!(state.transition_count, 1);
    assert_eq!(state.transition_count, predecessor.state.transition_count);
    assert_eq!(state.source_site_execution_count, 4);
    assert_eq!(state.environment_outdoor_barometric_pressure_owner_count, 1);
    assert_eq!(
        state.cp334_supply_temperature_mixed_air_limit_owner_count
            + state.cp344_capacity_limit_supply_temperature_mixed_air_limit_owner_count,
        1,
    );
    assert!(snapshot.environment_outdoor_barometric_pressure_owned_read);
    assert!(snapshot.psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluated);

    let numerical_owner = output
        .calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment
        .assigned_supply_humidity_ratio;
    let numerical_result = output
        .coupling
        .purchased_air
        .supply_node_update
        .humidity_ratio;
    assert_eq!(
        numerical_owner.map(f64::to_bits),
        Some(numerical_result.to_bits()),
    );
}

#[test]
fn cp377_counter_owner_and_latest_corruption_are_rejected() {
    let Some((model, output, lifecycle, predecessor, cp334, cp344)) = validator_fixture() else {
        return;
    };
    for mutate in [
        set_source_three as fn(&mut Lifecycle),
        set_pressure_read_zero,
        set_evaluation_zero,
        set_environment_owner_zero,
        set_both_temperature_owners,
    ] {
        let mut corrupt = lifecycle.clone();
        mutate(&mut corrupt);
        assert!(validate(&model, &output, &corrupt, &predecessor, &cp334, &cp344).is_err());
    }

    let mut latest = lifecycle.clone();
    let Some(snapshot) = latest.state.latest.as_mut() else {
        return;
    };
    snapshot.environment_outdoor_barometric_pressure_owned_read = false;
    assert_eq!(
        validate(&model, &output, &latest, &predecessor, &cp334, &cp344),
        Err(latest_violation()),
    );
}

#[test]
fn cp377_latest_comparison_preserves_ieee_bits() {
    let Some((_model, _output, lifecycle, _predecessor, _cp334, _cp344)) = validator_fixture()
    else {
        return;
    };
    let Some(mut left) = lifecycle.state.latest else {
        return;
    };
    let mut right = left;
    left.outdoor_barometric_pressure_pa = Some(0.0);
    right.outdoor_barometric_pressure_pa = Some(-0.0);
    assert!(!snapshots_match_exact_bits(left, right));

    let nan = f64::from_bits(0x7ff8_0000_0000_0377);
    left.outdoor_barometric_pressure_pa = Some(nan);
    right.outdoor_barometric_pressure_pa = Some(nan);
    assert!(snapshots_match_exact_bits(left, right));
    right.outdoor_barometric_pressure_pa = Some(f64::from_bits(nan.to_bits() ^ 1));
    assert!(!snapshots_match_exact_bits(left, right));
}

fn set_source_three(lifecycle: &mut Lifecycle) {
    lifecycle.state.source_site_execution_count = 3;
}

fn set_pressure_read_zero(lifecycle: &mut Lifecycle) {
    lifecycle
        .state
        .environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read_count = 0;
}

fn set_evaluation_zero(lifecycle: &mut Lifecycle) {
    lifecycle
        .state
        .psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluation_count = 0;
}

fn set_environment_owner_zero(lifecycle: &mut Lifecycle) {
    lifecycle
        .state
        .environment_outdoor_barometric_pressure_owner_count = 0;
}

fn set_both_temperature_owners(lifecycle: &mut Lifecycle) {
    lifecycle
        .state
        .cp334_supply_temperature_mixed_air_limit_owner_count = 1;
    lifecycle
        .state
        .cp344_capacity_limit_supply_temperature_mixed_air_limit_owner_count = 1;
}

fn latest_violation() -> Error {
    Error::CalcCoolingSupplyHumidityRatioSaturationAssignmentLifecycleInvariant {
        field: "latest_release_snapshot_ready",
        expected: 1,
        actual: 0,
    }
}

fn validator_fixture() -> Option<(
    SimulationModel,
    DirectZonePurchasedAirScheduledCouplingOutput,
    Lifecycle,
    PredecessorLifecycle,
    Cp334Lifecycle,
    Cp344Lifecycle,
)> {
    let mut typed = exact_model(1).typed;
    typed.schedules[1].hourly_value = 20.0;
    typed.schedules[2].hourly_value = 24.0;
    typed.schedules[3].hourly_value = 1.0;
    typed.ideal_loads_air_systems[0].dehumidification_control_type =
        DehumidificationControlType::None;
    typed.ideal_loads_air_systems[0].humidification_control_type = HumidificationControlType::None;
    typed.ideal_loads_air_systems[0].minimum_cooling_supply_air_humidity_ratio = f64::NAN;
    let model = SimulationModel::from_typed(typed);
    let cache = precompute_schedule_cache(&model.typed, 1).ok()?;
    let binding = bind_direct_zone_purchased_air_model(&model).ok()?;
    let mut zone_state = cooling_zone_state(binding.nominal_system_timestep_seconds);
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
    .ok()?;
    let system = output.initialization.system;
    Some((
        model,
        output,
        purchased_air_calc_cooling_supply_humidity_ratio_saturation_assignment_lifecycle_summary(
            &runtime, system,
        )
        .ok()?,
        purchased_air_calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment_lifecycle_summary(
            &runtime, system,
        )
        .ok()?,
        purchased_air_calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle_summary(
            &runtime, system,
        )
        .ok()?,
        purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle_summary(
            &runtime, system,
        )
        .ok()?,
    ))
}

fn validate(
    model: &SimulationModel,
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    lifecycle: &Lifecycle,
    predecessor: &PredecessorLifecycle,
    cp334: &Cp334Lifecycle,
    cp344: &Cp344Lifecycle,
) -> Result<(), Error> {
    let binding = bind_direct_zone_purchased_air_model(model).map_err(|_| latest_violation())?;
    assert!(snapshot_matches_release(output, 1, &binding));
    validate_lifecycle(lifecycle, predecessor, cp334, cp344, 1, output, &binding)
}

fn cooling_zone_state(system_timestep_seconds: f64) -> ZoneHeatBalanceState {
    ZoneHeatBalanceState {
        zone_id: ZoneId(0),
        zone_name: "ZONE ONE".to_string(),
        mean_air_temperature_c: 22.0,
        zone_timestep_average_air_temperature_c: 22.0,
        previous_mean_air_temperatures_c: [0.0; 3],
        previous_system_mean_air_temperatures_c: [0.0; 3],
        previous_system_timestep_count: 1,
        air_humidity_ratio: 0.008,
        zone_timestep_average_air_humidity_ratio: 0.008,
        previous_air_humidity_ratios: [0.008; 3],
        previous_system_air_humidity_ratios: [0.008; 3],
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
