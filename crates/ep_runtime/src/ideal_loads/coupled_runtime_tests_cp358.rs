use super::*;

#[test]
fn cp358_coupled_direct_none_is_exact_skip_and_cp345_remains_numerical_owner() {
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
        .calc_cooling_humidistat_case_entry_lifecycle;
    let state = &lifecycle.state;
    assert!(state.latest.is_some(), "latest CP358 snapshot");
    let Some(snapshot) = state.latest else {
        return;
    };
    let predecessor = simulation
        .summary
        .calc_cooling_constant_shr_case_break_lifecycle
        .state
        .latest;
    assert!(predecessor.is_some(), "latest CP357 snapshot");
    let Some(predecessor) = predecessor else {
        return;
    };
    assert_eq!(state.transition_count, 1);
    assert_eq!(
        state.dehumidification_control_none_case_completed_skip_count,
        1
    );
    assert_eq!(
        state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        0
    );
    assert_eq!(
        state.dehumidification_control_humidistat_case_entry_count,
        0
    );
    assert_eq!(state.source_site_execution_count, 0);
    assert!(snapshot.dehumidification_control_none_case_completed_skip);
    assert!(!snapshot.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip);
    assert!(!snapshot.dehumidification_control_humidistat_case_entered);

    let projected =
        super::super::cooling_humidistat_case_entry_validation::expected_snapshot(predecessor);
    assert!(
        super::super::cooling_humidistat_case_entry_validation::snapshots_match_exact(
            &snapshot, &projected,
        )
    );
    let mut corruptions = [snapshot; 6];
    corruptions[0].source_order = &["forged-cp358-source-order"];
    corruptions[1].system = IdealLoadsAirSystemId(1);
    corruptions[2].parent_call_ordinal += 1;
    corruptions[3].controlled_zone = ZoneId(1);
    corruptions[4].dehumidification_control_none_case_completed_skip = false;
    corruptions[5].dehumidification_control_humidistat_case_entered = true;
    for corrupted in corruptions {
        assert!(
            !super::super::cooling_humidistat_case_entry_validation::snapshots_match_exact(
                &corrupted, &projected,
            )
        );
    }

    let owner = simulation
        .summary
        .calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle
        .state
        .latest;
    assert!(owner.is_some(), "latest CP345 numerical owner");
    let Some(owner) = owner else {
        return;
    };
    assert!(
        owner.assigned_supply_humidity_ratio.is_some(),
        "CP345 numerical humidity value"
    );
    let Some(owner_value) = owner.assigned_supply_humidity_ratio else {
        return;
    };
    let result = simulation
        .results
        .find_series(SYSTEM_KEY, ZONE_IDEAL_LOADS_SUPPLY_AIR_HUMIDITY_RATIO);
    assert!(result.is_some(), "coupled humidity result");
    let Some(result) = result else {
        return;
    };
    assert_eq!(result.values[0].to_bits(), owner_value.to_bits());
}
