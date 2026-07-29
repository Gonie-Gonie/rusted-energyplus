use super::*;

#[test]
fn cp360_coupled_direct_none_is_exact_null_skip_and_cp345_stays_owner() {
    let mut typed = exact_model(1).typed;
    typed.schedules[1].hourly_value = 0.0;
    typed.schedules[2].hourly_value = 15.0;
    typed.schedules[3].hourly_value = 1.0;
    typed.ideal_loads_air_systems[0].dehumidification_control_type =
        DehumidificationControlType::None;
    let model = SimulationModel::from_typed(typed);
    let cache_result = precompute_schedule_cache(&model.typed, 1);
    assert!(cache_result.is_ok());
    let Ok(cache) = cache_result else {
        return;
    };
    let weather = weather_series_with_conditions(&model, 1, 30.0, 15.0, 30.0, 101_325.0);
    let mut options = DirectZonePurchasedAirCoupledOptions::hourly_samples(1);
    options.initial_zone_air_temperature_c = INITIAL_ZONE_TEMPERATURE_C;
    let simulation_result =
        simulate_direct_zone_purchased_air_coupled_heat_balance(&model, &weather, &cache, options);
    assert!(simulation_result.is_ok());
    let Ok(simulation) = simulation_result else {
        return;
    };

    let lifecycle = &simulation
        .summary
        .calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_lifecycle;
    let state = &lifecycle.state;
    assert!(state.latest.is_some());
    let Some(snapshot) = state.latest else {
        return;
    };
    let predecessor = simulation
        .summary
        .calc_cooling_humidistat_moisture_demand_assignment_lifecycle
        .state
        .latest;
    assert!(predecessor.is_some());
    let Some(predecessor) = predecessor else {
        return;
    };
    assert_eq!(state.transition_count, 1);
    assert_eq!(
        state.dehumidification_control_none_case_completed_skip_count,
        1
    );
    assert_eq!(
        state
            .dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_count,
        0
    );
    assert_eq!(
        (
            state.source_site_execution_count,
            state.zone_dehumidifying_setpoint_moisture_demand_read_count,
            state.supply_mass_flow_rate_read_count,
            state.moisture_demand_derived_supply_humidity_ratio_calculation_count,
            state.zone_node_humidity_ratio_read_count,
            state.supply_humidity_ratio_for_dehumidification_calculation_count,
            state.supply_humidity_ratio_for_dehumidification_assignment_count,
        ),
        (0, 0, 0, 0, 0, 0, 0)
    );
    assert!(snapshot.dehumidification_control_none_case_completed_skip);
    assert!(
        !snapshot
            .dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_executed
    );
    assert!(!snapshot.zone_dehumidifying_setpoint_moisture_demand_read);
    assert!(!snapshot.supply_mass_flow_rate_read);
    assert!(!snapshot.moisture_demand_derived_supply_humidity_ratio_calculated);
    assert!(!snapshot.zone_node_humidity_ratio_read);
    assert!(!snapshot.supply_humidity_ratio_for_dehumidification_calculated);
    assert!(!snapshot.supply_humidity_ratio_for_dehumidification_assigned);
    assert!(
        snapshot
            .resulting_supply_humidity_ratio_for_dehumidification
            .is_none()
    );

    let projected =
        super::super::cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_validation::expected_snapshot(
            predecessor,
        );
    assert!(
        super::super::cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_validation::
            snapshots_match_bit_exact(&snapshot, &projected)
    );
    let mut structural = snapshot;
    structural.source_order = &["forged-cp360-source-order"];
    assert!(
        !super::super::cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_validation::
            snapshots_match_bit_exact(&structural, &projected)
    );
    let mut numeric = snapshot;
    numeric.supply_mass_flow_rate_read = true;
    numeric.supply_mass_flow_rate_kg_per_s = Some(-0.0);
    assert!(
        !super::super::cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_validation::
            snapshots_match_bit_exact(&numeric, &projected)
    );

    let owner = simulation
        .summary
        .calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle
        .state
        .latest;
    assert!(owner.is_some());
    let Some(owner) = owner else {
        return;
    };
    assert!(owner.assigned_supply_humidity_ratio.is_some());
    let Some(owner_value) = owner.assigned_supply_humidity_ratio else {
        return;
    };
    let result = simulation
        .results
        .find_series(SYSTEM_KEY, ZONE_IDEAL_LOADS_SUPPLY_AIR_HUMIDITY_RATIO);
    assert!(result.is_some());
    let Some(result) = result else {
        return;
    };
    assert_eq!(result.values[0].to_bits(), owner_value.to_bits());
}
