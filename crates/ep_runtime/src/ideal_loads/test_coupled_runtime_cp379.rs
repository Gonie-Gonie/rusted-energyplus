//! Non-vacuous CP379 coupled-runtime integration tests.

use super::*;
use crate::heat_balance::state::{ZoneAirTemperatureCoefficients, ZoneHeatBalanceState};
use crate::ideal_loads::{
    DirectZonePurchasedAirScheduledCouplingInput, DirectZonePurchasedAirScheduledCouplingOutput,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentLifecycleSummary as TemperatureLifecycle,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentLifecycleSummary as HumidityLifecycle,
    PurchasedAirRuntimeState, bind_direct_zone_purchased_air_model,
    couple_model_bound_direct_zone_purchased_air,
    purchased_air_calc_cooling_supply_enthalpy_post_saturation_assignment_lifecycle_summary,
    purchased_air_calc_cooling_supply_humidity_ratio_saturation_assignment_lifecycle_summary,
    purchased_air_calc_cooling_supply_humidity_ratio_saturation_limit_assignment_lifecycle_summary,
};
use crate::psychrometrics::energyplus_psy_h_fn_tdb_w;
use crate::schedules::precompute_schedule_cache;
use ep_model::{DehumidificationControlType, HumidificationControlType, SimulationModel, ZoneId};

use super::super::DirectZonePurchasedAirCoupledRuntimeError as Error;
use super::super::cooling_supply_enthalpy_post_saturation_assignment_validation::{
    snapshot_matches_release, snapshots_match_exact_bits, validate_lifecycle,
};

#[test]
fn cp379_follows_cp378_records_four_sites_and_uses_the_canonical_helper() {
    let (model, output, lifecycle, humidity, temperature) = validator_fixture(101_325.0, 0.008);
    assert!(validate(&model, &output, &lifecycle, &humidity, &temperature).is_ok());
    assert_eq!(lifecycle.state.transition_count, 1);
    assert_eq!(lifecycle.state.source_site_execution_count, 4);
    assert_eq!(
        lifecycle
            .state
            .local_supply_enthalpy_after_saturation_limit_assignment_count,
        1
    );
    assert_eq!(
        lifecycle
            .state
            .cp334_supply_temperature_mixed_air_limit_owner_count
            + lifecycle
                .state
                .cp344_capacity_limit_supply_temperature_mixed_air_limit_owner_count,
        1
    );
    assert_eq!(
        lifecycle
            .state
            .cp378_supply_humidity_ratio_saturation_limit_owner_count,
        1
    );
    let snapshot = output.calculation_cooling_supply_enthalpy_post_saturation_assignment;
    let temperature_c = snapshot.supply_temperature_c.expect("active temperature");
    let humidity_ratio = snapshot
        .supply_humidity_ratio
        .expect("active humidity ratio");
    assert_eq!(
        snapshot
            .resulting_supply_enthalpy_j_per_kg
            .map(f64::to_bits),
        Some(energyplus_psy_h_fn_tdb_w(temperature_c, humidity_ratio).to_bits()),
    );
}

#[test]
fn cp379_rejects_evidence_corruption_but_does_not_reconcile_numerical_enthalpy() {
    let (model, output, lifecycle, humidity, temperature) = validator_fixture(101_325.0, 0.0);
    let binding = bind_direct_zone_purchased_air_model(&model).expect("direct binding");
    let snapshot = output.calculation_cooling_supply_enthalpy_post_saturation_assignment;
    let canonical = snapshot
        .resulting_supply_enthalpy_j_per_kg
        .expect("active CP379 enthalpy");
    let numerical = output
        .coupling
        .purchased_air
        .calculation
        .supply_enthalpy_j_per_kg;
    assert_ne!(canonical.to_bits(), numerical.to_bits());
    assert!(snapshot_matches_release(&output, 1, &binding));

    let mut changed_numerical = output;
    changed_numerical
        .coupling
        .purchased_air
        .calculation
        .supply_enthalpy_j_per_kg = different(numerical);
    assert!(
        snapshot_matches_release(&changed_numerical, 1, &binding),
        "CP379 must not reconcile with or claim the unchanged numerical enthalpy"
    );

    let mut count = lifecycle.clone();
    count.state.source_site_execution_count = 3;
    assert!(validate(&model, &output, &count, &humidity, &temperature).is_err());

    let mut latest = lifecycle;
    latest
        .state
        .latest
        .as_mut()
        .expect("CP379 latest")
        .cp378_supply_humidity_ratio_saturation_limit_owned_read = false;
    assert_eq!(
        validate(&model, &output, &latest, &humidity, &temperature),
        Err(latest_violation()),
    );
}

#[test]
fn cp379_latest_comparison_preserves_ieee_bits() {
    let (_model, _output, lifecycle, _humidity, _temperature) = validator_fixture(101_325.0, 0.008);
    let mut left = lifecycle.state.latest.expect("CP379 latest");
    let mut right = left;
    left.resulting_supply_enthalpy_j_per_kg = Some(0.0);
    right.resulting_supply_enthalpy_j_per_kg = Some(-0.0);
    assert!(!snapshots_match_exact_bits(left, right));

    let nan = f64::from_bits(0x7ff8_0000_0000_0379);
    left.resulting_supply_enthalpy_j_per_kg = Some(nan);
    right.resulting_supply_enthalpy_j_per_kg = Some(nan);
    assert!(snapshots_match_exact_bits(left, right));
    right.resulting_supply_enthalpy_j_per_kg = Some(f64::from_bits(nan.to_bits() ^ 1));
    assert!(!snapshots_match_exact_bits(left, right));
}

fn validator_fixture(
    pressure: f64,
    humidity_ratio: f64,
) -> (
    SimulationModel,
    DirectZonePurchasedAirScheduledCouplingOutput,
    Lifecycle,
    HumidityLifecycle,
    TemperatureLifecycle,
) {
    let mut typed = exact_model(1).typed;
    typed.schedules[1].hourly_value = 20.0;
    typed.schedules[2].hourly_value = 24.0;
    typed.schedules[3].hourly_value = 1.0;
    typed.ideal_loads_air_systems[0].dehumidification_control_type =
        DehumidificationControlType::None;
    typed.ideal_loads_air_systems[0].humidification_control_type = HumidificationControlType::None;
    typed.ideal_loads_air_systems[0].minimum_cooling_supply_air_humidity_ratio = f64::NAN;
    let model = SimulationModel::from_typed(typed);
    let cache = precompute_schedule_cache(&model.typed, 1).expect("CP379 schedule cache");
    let binding = bind_direct_zone_purchased_air_model(&model).expect("CP379 direct binding");
    let mut zone_state =
        cooling_zone_state(binding.nominal_system_timestep_seconds, humidity_ratio);
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
    .expect("CP379 coupling");
    let system = output.initialization.system;
    (
        model,
        output,
        purchased_air_calc_cooling_supply_enthalpy_post_saturation_assignment_lifecycle_summary(
            &runtime, system,
        )
        .expect("CP379 lifecycle"),
        purchased_air_calc_cooling_supply_humidity_ratio_saturation_limit_assignment_lifecycle_summary(
            &runtime, system,
        )
        .expect("CP378 lifecycle"),
        purchased_air_calc_cooling_supply_humidity_ratio_saturation_assignment_lifecycle_summary(
            &runtime, system,
        )
        .expect("CP377 lifecycle"),
    )
}

fn validate(
    model: &SimulationModel,
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    lifecycle: &Lifecycle,
    humidity: &HumidityLifecycle,
    temperature: &TemperatureLifecycle,
) -> Result<(), Error> {
    let binding = bind_direct_zone_purchased_air_model(model).map_err(|_| latest_violation())?;
    assert!(snapshot_matches_release(output, 1, &binding));
    validate_lifecycle(lifecycle, humidity, temperature, 1, output, &binding)
}

fn latest_violation() -> Error {
    Error::CalcCoolingSupplyEnthalpyPostSaturationAssignmentLifecycleInvariant {
        field: "latest_release_snapshot_ready",
        expected: 1,
        actual: 0,
    }
}

fn different(value: f64) -> f64 {
    f64::from_bits(value.to_bits() ^ 1)
}

fn cooling_zone_state(system_timestep_seconds: f64, humidity_ratio: f64) -> ZoneHeatBalanceState {
    ZoneHeatBalanceState {
        zone_id: ZoneId(0),
        zone_name: "ZONE ONE".to_string(),
        mean_air_temperature_c: 22.0,
        zone_timestep_average_air_temperature_c: 22.0,
        previous_mean_air_temperatures_c: [0.0; 3],
        previous_system_mean_air_temperatures_c: [0.0; 3],
        previous_system_timestep_count: 1,
        air_humidity_ratio: humidity_ratio,
        zone_timestep_average_air_humidity_ratio: humidity_ratio,
        previous_air_humidity_ratios: [humidity_ratio; 3],
        previous_system_air_humidity_ratios: [humidity_ratio; 3],
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
