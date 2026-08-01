//! Non-vacuous CP389 coupled-runtime integration tests.

use super::*;
use crate::heat_balance::state::{ZoneAirTemperatureCoefficients, ZoneHeatBalanceState};
use crate::ideal_loads::{
    DirectZonePurchasedAirScheduledCouplingInput, DirectZonePurchasedAirScheduledCouplingOutput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentLifecycleSummary as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirRuntimeState, bind_direct_zone_purchased_air_model,
    couple_model_bound_direct_zone_purchased_air,
    purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_lifecycle_summary,
    purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_lifecycle_summary,
};
use crate::schedules::precompute_schedule_cache;
use ep_model::{
    AutosizeOrNumber, DehumidificationControlType, HumidificationControlType, IdealLoadsLimit,
    SimulationModel, ZoneId,
};

use super::super::DirectZonePurchasedAirCoupledRuntimeError as Error;
use super::super::cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_validation::{
    snapshot_matches_release, validate_lifecycle,
};

#[test]
fn cp389_preserves_cp379_temperature_on_representative_direct_routes() {
    for (limit, maximum_capacity_w, availability) in [
        (IdealLoadsLimit::NoLimit, 5_000.0, 0.0),
        (IdealLoadsLimit::NoLimit, 5_000.0, 1.0),
        (IdealLoadsLimit::LimitCapacity, 500.0, 1.0),
    ] {
        let (model, output, lifecycle, predecessor) =
            validator_fixture(limit, 0.020, maximum_capacity_w, availability, 1);
        assert!(validate(&model, &output, &lifecycle, &predecessor, 1).is_ok());
        let owner = output.calculation_cooling_supply_enthalpy_post_saturation_assignment;
        let snapshot = output.calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment;
        assert_eq!(
            snapshot.preexisting_supply_temperature_c.map(f64::to_bits),
            owner.supply_temperature_c.map(f64::to_bits)
        );
        assert_eq!(
            snapshot.resulting_supply_temperature_c.map(f64::to_bits),
            owner.supply_temperature_c.map(f64::to_bits)
        );
        assert!(!snapshot.dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_executed);
        assert_eq!(lifecycle.state.transition_count, 1);
        assert_eq!(lifecycle.state.inactive_transition_count, 1);
        assert_eq!(lifecycle.state.source_site_execution_count, 0);
        assert_eq!(
            lifecycle.state.predecessor_route_counts,
            predecessor.state.predecessor_route_counts
        );
    }
}

#[test]
fn cp389_rejects_one_bit_drift_in_cp388_retained_result() {
    let (model, output, lifecycle, mut predecessor) =
        validator_fixture(IdealLoadsLimit::LimitCapacity, 0.020, 500.0, 1.0, 1);
    let value = predecessor
        .state
        .latest
        .as_mut()
        .expect("CP388 latest")
        .resulting_supply_enthalpy_j_per_kg
        .as_mut()
        .expect("CP388 retained enthalpy");
    *value = f64::from_bits(value.to_bits() ^ 1);
    assert_eq!(
        validate(&model, &output, &lifecycle, &predecessor, 1),
        Err(latest_violation())
    );
}

#[test]
fn cp389_rejects_route_drift_and_accepts_replay() {
    let (model, output, mut lifecycle, predecessor) =
        validator_fixture(IdealLoadsLimit::LimitCapacity, 0.020, 500.0, 1.0, 2);
    assert!(validate(&model, &output, &lifecycle, &predecessor, 2).is_ok());
    assert_eq!(lifecycle.state.transition_count, 2);
    assert_eq!(lifecycle.state.inactive_transition_count, 2);
    lifecycle.state.predecessor_route_counts[20] += 1;
    assert!(validate(&model, &output, &lifecycle, &predecessor, 2).is_err());
}

#[test]
fn cp389_is_evidence_only_under_numerical_output_mutations() {
    let (model, output, lifecycle, predecessor) =
        validator_fixture(IdealLoadsLimit::LimitCapacity, 0.020, 500.0, 1.0, 1);
    let binding = bind_direct_zone_purchased_air_model(&model).expect("direct binding");
    let mut changed = output;
    changed
        .coupling
        .prediction
        .predicted_loads
        .total_output_required_w = different(
        changed
            .coupling
            .prediction
            .predicted_loads
            .total_output_required_w,
    );
    changed
        .coupling
        .purchased_air
        .calculation
        .supply_temperature_c = different(
        changed
            .coupling
            .purchased_air
            .calculation
            .supply_temperature_c,
    );
    changed
        .coupling
        .purchased_air
        .calculation
        .supply_enthalpy_j_per_kg = different(
        changed
            .coupling
            .purchased_air
            .calculation
            .supply_enthalpy_j_per_kg,
    );
    changed
        .coupling
        .purchased_air
        .supply_node_update
        .temperature_c = different(
        changed
            .coupling
            .purchased_air
            .supply_node_update
            .temperature_c,
    );
    changed
        .coupling
        .purchased_air
        .supply_node_update
        .enthalpy_j_per_kg = different(
        changed
            .coupling
            .purchased_air
            .supply_node_update
            .enthalpy_j_per_kg,
    );
    changed
        .coupling
        .purchased_air
        .report
        .supply_air_total_cooling_rate_w = different(
        changed
            .coupling
            .purchased_air
            .report
            .supply_air_total_cooling_rate_w,
    );
    changed.coupling.feedback.sum_sys_mcp_t_w =
        different(changed.coupling.feedback.sum_sys_mcp_t_w);
    assert!(snapshot_matches_release(&changed, 1, &binding));
    assert!(validate_lifecycle(&lifecycle, &predecessor, 1, &changed, &binding).is_ok());
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
    Predecessor,
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
    let cache = precompute_schedule_cache(&model.typed, 1).expect("CP389 schedule cache");
    let binding = bind_direct_zone_purchased_air_model(&model).expect("CP389 direct binding");
    let mut runtime = PurchasedAirRuntimeState::default();
    let mut latest = None;
    for step in 0..steps {
        let mut zone_state =
            cooling_zone_state(binding.nominal_system_timestep_seconds, air_humidity_ratio);
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
            .expect("CP389 coupling"),
        );
    }
    let output = latest.expect("at least one CP389 step");
    let system = output.initialization.system;
    let lifecycle = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_lifecycle_summary(&runtime, system).expect("CP389 lifecycle");
    let predecessor = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_lifecycle_summary(&runtime, system).expect("CP388 lifecycle");
    (model, output, lifecycle, predecessor)
}

fn validate(
    model: &SimulationModel,
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    lifecycle: &Lifecycle,
    predecessor: &Predecessor,
    timestep_count: usize,
) -> Result<(), Error> {
    let binding = bind_direct_zone_purchased_air_model(model).map_err(|_| latest_violation())?;
    assert!(snapshot_matches_release(output, timestep_count, &binding));
    validate_lifecycle(lifecycle, predecessor, timestep_count, output, &binding)
}

fn latest_violation() -> Error {
    Error::CalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentLifecycleInvariant {
        field: "latest_release_snapshot_ready",
        expected: 1,
        actual: 0,
    }
}

fn different(value: f64) -> f64 {
    f64::from_bits(value.to_bits() ^ 1)
}

pub(super) fn cooling_zone_state(
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
