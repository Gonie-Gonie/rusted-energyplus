use super::*;
use crate::heat_balance::state::{ZoneAirTemperatureCoefficients, ZoneHeatBalanceState};
use crate::ideal_loads::{
    DirectZonePurchasedAirScheduledCouplingInput, DirectZonePurchasedAirScheduledCouplingOutput,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirRuntimeState, bind_direct_zone_purchased_air_model,
    couple_model_bound_direct_zone_purchased_air,
    purchased_air_calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_lifecycle_summary,
    purchased_air_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_lifecycle_summary,
};
use crate::schedules::precompute_schedule_cache;
use ep_model::{DehumidificationControlType, HumidificationControlType, SimulationModel, ZoneId};

use super::super::DirectZonePurchasedAirCoupledRuntimeError as Error;
use super::super::cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_validation::{
    expected_snapshot, validate_lifecycle,
};

#[test]
fn cp373_lifecycle_matches_cp372_and_does_not_feed_numerical_result() {
    let fixture = validator_fixture();
    assert!(fixture.is_some(), "CP373 validator fixture");
    let Some((model, output, lifecycle, predecessor)) = fixture else {
        return;
    };
    assert!(validate(&model, &output, &lifecycle, &predecessor).is_ok());

    let state = &lifecycle.state;
    let predecessor_state = &predecessor.state;
    let predecessor_snapshot =
        output.calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment;
    let snapshot = output
        .calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment;

    assert_eq!(state.transition_count, 1);
    assert_eq!(state.transition_count, predecessor_state.transition_count);
    assert_eq!(
        state.humidification_control_guard_false_fallthrough_count,
        predecessor_state.humidification_control_guard_false_fallthrough_count,
    );
    assert_eq!(
        state.humidification_control_guard_false_fallthrough_count,
        1,
    );
    for count in [
        state.dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_assignment_count,
        state.dehumidification_control_none_supply_humidity_ratio_for_humidification_assignment_count,
        state.dehumidification_control_guard_false_fallthrough_count,
        state.source_site_execution_count,
        state.zone_humidifying_setpoint_moisture_demand_read_count,
        state.supply_mass_flow_rate_read_count,
        state.moisture_demand_derived_supply_humidity_ratio_calculation_count,
        state.zone_node_humidity_ratio_read_count,
        state.supply_humidity_ratio_for_humidification_calculation_count,
        state.supply_humidity_ratio_for_humidification_assignment_count,
    ] {
        assert_eq!(count, 0);
    }

    assert!(snapshot.predecessor_humidification_control_type_read);
    assert_eq!(
        snapshot.predecessor_humidification_control_type,
        Some(HumidificationControlType::None),
    );
    assert!(snapshot.predecessor_humidification_control_guard_false_fallthrough);
    assert!(!snapshot.predecessor_humidification_moisture_demand_assignment_executed);
    assert!(
        !snapshot
            .dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_assignment_executed
    );
    assert!(
        !snapshot
            .dehumidification_control_none_supply_humidity_ratio_for_humidification_assignment_executed
    );
    assert!(!snapshot.zone_humidifying_setpoint_moisture_demand_read);
    assert!(!snapshot.supply_mass_flow_rate_read);
    assert!(!snapshot.moisture_demand_derived_supply_humidity_ratio_calculated);
    assert!(!snapshot.zone_node_humidity_ratio_read);
    assert!(!snapshot.supply_humidity_ratio_for_humidification_calculated);
    assert!(!snapshot.supply_humidity_ratio_for_humidification_assigned);
    assert_eq!(
        [
            snapshot.predecessor_resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s,
            snapshot.zone_humidifying_setpoint_moisture_demand_kg_per_s,
            snapshot.supply_mass_flow_rate_kg_per_s,
            snapshot.moisture_demand_derived_supply_humidity_ratio,
            snapshot.zone_node_humidity_ratio,
            snapshot.calculated_supply_humidity_ratio_for_humidification,
            snapshot.assigned_supply_humidity_ratio_for_humidification,
            snapshot.resulting_supply_humidity_ratio_for_humidification,
        ],
        [None; 8],
    );
    assert_eq!(snapshot, expected_snapshot(predecessor_snapshot));

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
fn cp373_direct_counters_and_latest_corruption_are_rejected() {
    let fixture = validator_fixture();
    assert!(fixture.is_some(), "CP373 corruption fixture");
    let Some((model, output, lifecycle, predecessor)) = fixture else {
        return;
    };

    for mutate in [
        set_humidistat_assignment_one as fn(&mut Lifecycle),
        set_none_assignment_one,
        set_dehumidification_false_one,
        set_assignment_one,
        set_source_one,
        set_demand_read_one,
        set_flow_read_one,
        set_division_one,
        set_zone_humidity_read_one,
        set_addition_one,
    ] {
        let mut corrupt = lifecycle.clone();
        mutate(&mut corrupt);
        assert!(validate(&model, &output, &corrupt, &predecessor).is_err());
    }

    let mut latest = lifecycle.clone();
    assert!(latest.state.latest.is_some(), "CP373 latest");
    let Some(latest_snapshot) = latest.state.latest.as_mut() else {
        return;
    };
    latest_snapshot.supply_mass_flow_rate_read = true;
    assert_eq!(
        validate(&model, &output, &latest, &predecessor),
        Err(latest_violation()),
    );

    let mut predecessor_latest = predecessor.clone();
    assert!(
        predecessor_latest.state.latest.is_some(),
        "CP372 predecessor latest"
    );
    let Some(predecessor_latest_snapshot) = predecessor_latest.state.latest.as_mut() else {
        return;
    };
    predecessor_latest_snapshot.zone_humidifying_setpoint_moisture_demand_read = true;
    assert_eq!(
        validate(&model, &output, &lifecycle, &predecessor_latest),
        Err(latest_violation()),
    );
}

fn latest_violation() -> Error {
    Error::CalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentLifecycleInvariant {
        field: "latest_release_snapshot_ready",
        expected: 1,
        actual: 0,
    }
}

fn set_humidistat_assignment_one(lifecycle: &mut Lifecycle) {
    lifecycle.state.dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_assignment_count = 1;
}

fn set_none_assignment_one(lifecycle: &mut Lifecycle) {
    lifecycle
        .state
        .dehumidification_control_none_supply_humidity_ratio_for_humidification_assignment_count =
        1;
}

fn set_dehumidification_false_one(lifecycle: &mut Lifecycle) {
    lifecycle
        .state
        .dehumidification_control_guard_false_fallthrough_count = 1;
}

fn set_assignment_one(lifecycle: &mut Lifecycle) {
    lifecycle
        .state
        .supply_humidity_ratio_for_humidification_assignment_count = 1;
}

fn set_source_one(lifecycle: &mut Lifecycle) {
    lifecycle.state.source_site_execution_count = 1;
}

fn set_demand_read_one(lifecycle: &mut Lifecycle) {
    lifecycle
        .state
        .zone_humidifying_setpoint_moisture_demand_read_count = 1;
}

fn set_flow_read_one(lifecycle: &mut Lifecycle) {
    lifecycle.state.supply_mass_flow_rate_read_count = 1;
}

fn set_division_one(lifecycle: &mut Lifecycle) {
    lifecycle
        .state
        .moisture_demand_derived_supply_humidity_ratio_calculation_count = 1;
}

fn set_zone_humidity_read_one(lifecycle: &mut Lifecycle) {
    lifecycle.state.zone_node_humidity_ratio_read_count = 1;
}

fn set_addition_one(lifecycle: &mut Lifecycle) {
    lifecycle
        .state
        .supply_humidity_ratio_for_humidification_calculation_count = 1;
}

fn validator_fixture() -> Option<(
    SimulationModel,
    DirectZonePurchasedAirScheduledCouplingOutput,
    Lifecycle,
    PredecessorLifecycle,
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
    let lifecycle = purchased_air_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_lifecycle_summary(
        &runtime,
        output.initialization.system,
    )
    .ok()?;
    let predecessor = purchased_air_calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_lifecycle_summary(
        &runtime,
        output.initialization.system,
    )
    .ok()?;
    Some((model, output, lifecycle, predecessor))
}

fn validate(
    model: &SimulationModel,
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    lifecycle: &Lifecycle,
    predecessor: &PredecessorLifecycle,
) -> Result<(), Error> {
    let binding = bind_direct_zone_purchased_air_model(model).map_err(|_| latest_violation())?;
    validate_lifecycle(lifecycle, predecessor, 1, output, &binding)
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
