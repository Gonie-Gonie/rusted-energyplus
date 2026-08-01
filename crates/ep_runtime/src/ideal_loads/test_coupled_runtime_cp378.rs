//! Non-vacuous CP378 coupled-runtime integration tests.

use super::*;
use crate::heat_balance::state::{ZoneAirTemperatureCoefficients, ZoneHeatBalanceState};
use crate::ideal_loads::{
    DirectZonePurchasedAirScheduledCouplingInput, DirectZonePurchasedAirScheduledCouplingOutput,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirRuntimeState, bind_direct_zone_purchased_air_model,
    couple_model_bound_direct_zone_purchased_air,
    purchased_air_calc_cooling_supply_humidity_ratio_saturation_assignment_lifecycle_summary,
    purchased_air_calc_cooling_supply_humidity_ratio_saturation_limit_assignment_lifecycle_summary,
};
use crate::schedules::precompute_schedule_cache;
use ep_model::{DehumidificationControlType, HumidificationControlType, SimulationModel, ZoneId};

use super::super::DirectZonePurchasedAirCoupledRuntimeError as Error;
use super::super::cooling_supply_humidity_ratio_saturation_limit_assignment_validation::{
    snapshot_matches_release, snapshots_match_exact_bits, validate_lifecycle,
};

#[test]
fn cp378_follows_cp377_records_four_sites_and_reconciles_all_humidity_projections() {
    let (model, output, lifecycle, predecessor) = validator_fixture(2_000_000.0);
    assert!(validate(&model, &output, &lifecycle, &predecessor).is_ok());
    assert_eq!(lifecycle.state.transition_count, 1);
    assert_eq!(lifecycle.state.source_site_execution_count, 4);
    assert_eq!(
        lifecycle
            .state
            .cp376_original_supply_humidity_ratio_owner_count,
        1
    );
    assert_eq!(
        lifecycle
            .state
            .cp377_saturation_supply_humidity_ratio_owner_count,
        1
    );
    let snapshot = output.calculation_cooling_supply_humidity_ratio_saturation_limit_assignment;
    let result = snapshot
        .resulting_supply_humidity_ratio
        .expect("active CP378 result");
    let left = snapshot
        .original_supply_humidity_ratio_before_saturation_limit
        .expect("active original operand");
    let right = snapshot
        .saturation_supply_humidity_ratio_for_limit
        .expect("active saturation operand");
    assert!(
        right < left,
        "fixture must exercise the saturation-selected lane"
    );
    assert_eq!(result.to_bits(), right.to_bits());
    for value in [
        output
            .coupling
            .purchased_air
            .calculation
            .supply_humidity_ratio,
        output
            .coupling
            .purchased_air
            .supply_node_update
            .humidity_ratio,
        output.coupling.purchased_air.report.supply_humidity_ratio,
    ] {
        assert_eq!(value.to_bits(), result.to_bits());
    }
}

#[test]
fn cp378_counter_latest_and_numerical_corruption_are_rejected() {
    let (model, output, lifecycle, predecessor) = validator_fixture(101_325.0);
    let mut count = lifecycle.clone();
    count.state.source_site_execution_count = 3;
    assert!(validate(&model, &output, &count, &predecessor).is_err());

    let mut latest = lifecycle.clone();
    let snapshot = latest
        .state
        .latest
        .as_mut()
        .expect("CP378 fixture must retain latest evidence");
    snapshot.cp377_saturation_supply_humidity_ratio_owned_read = false;
    assert_eq!(
        validate(&model, &output, &latest, &predecessor),
        Err(latest_violation()),
    );

    for corrupt in [
        corrupt_calculation as fn(&mut DirectZonePurchasedAirScheduledCouplingOutput),
        corrupt_node,
        corrupt_report,
    ] {
        let mut output = output;
        corrupt(&mut output);
        let binding = bind_direct_zone_purchased_air_model(&model).expect("direct binding");
        assert!(!snapshot_matches_release(&output, 1, &binding));
    }
}

#[test]
fn cp378_latest_comparison_preserves_ieee_bits() {
    let (_model, _output, lifecycle, _predecessor) = validator_fixture(101_325.0);
    let mut left = lifecycle
        .state
        .latest
        .expect("CP378 fixture must retain latest evidence");
    let mut right = left;
    left.original_supply_humidity_ratio_before_saturation_limit = Some(0.0);
    right.original_supply_humidity_ratio_before_saturation_limit = Some(-0.0);
    assert!(!snapshots_match_exact_bits(left, right));

    let nan = f64::from_bits(0x7ff8_0000_0000_0378);
    left.original_supply_humidity_ratio_before_saturation_limit = Some(nan);
    right.original_supply_humidity_ratio_before_saturation_limit = Some(nan);
    assert!(snapshots_match_exact_bits(left, right));
    right.original_supply_humidity_ratio_before_saturation_limit =
        Some(f64::from_bits(nan.to_bits() ^ 1));
    assert!(!snapshots_match_exact_bits(left, right));
}

fn validator_fixture(
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
    typed.schedules[3].hourly_value = 1.0;
    typed.ideal_loads_air_systems[0].dehumidification_control_type =
        DehumidificationControlType::None;
    typed.ideal_loads_air_systems[0].humidification_control_type = HumidificationControlType::None;
    typed.ideal_loads_air_systems[0].minimum_cooling_supply_air_humidity_ratio = f64::NAN;
    let model = SimulationModel::from_typed(typed);
    let cache = precompute_schedule_cache(&model.typed, 1)
        .expect("CP378 fixture schedule cache must compile");
    let binding =
        bind_direct_zone_purchased_air_model(&model).expect("CP378 fixture must bind directly");
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
            barometric_pressure_pa: pressure,
            system_timestep_seconds: binding.nominal_system_timestep_seconds,
        },
    )
    .expect("CP378 fixture coupling must complete");
    let system = output.initialization.system;
    (
        model,
        output,
        purchased_air_calc_cooling_supply_humidity_ratio_saturation_limit_assignment_lifecycle_summary(
            &runtime, system,
        )
        .expect("CP378 fixture lifecycle must resolve"),
        purchased_air_calc_cooling_supply_humidity_ratio_saturation_assignment_lifecycle_summary(
            &runtime, system,
        )
        .expect("CP377 fixture predecessor lifecycle must resolve"),
    )
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
    Error::CalcCoolingSupplyHumidityRatioSaturationLimitAssignmentLifecycleInvariant {
        field: "latest_release_snapshot_ready",
        expected: 1,
        actual: 0,
    }
}

fn different(value: f64) -> f64 {
    f64::from_bits(value.to_bits() ^ 1)
}

fn corrupt_calculation(output: &mut DirectZonePurchasedAirScheduledCouplingOutput) {
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
}

fn corrupt_node(output: &mut DirectZonePurchasedAirScheduledCouplingOutput) {
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
}

fn corrupt_report(output: &mut DirectZonePurchasedAirScheduledCouplingOutput) {
    output.coupling.purchased_air.report.supply_humidity_ratio =
        different(output.coupling.purchased_air.report.supply_humidity_ratio);
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
