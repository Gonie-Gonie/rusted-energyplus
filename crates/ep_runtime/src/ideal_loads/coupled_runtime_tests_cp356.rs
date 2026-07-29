use super::*;

#[test]
fn cp356_coupled_direct_none_route_is_complete_skip_and_numerical_humidity_remains_unfed() {
    let mut typed = exact_model(1).typed;
    typed.schedules[1].hourly_value = 0.0;
    typed.schedules[2].hourly_value = 15.0;
    typed.schedules[3].hourly_value = 1.0;
    typed.ideal_loads_air_systems[0].dehumidification_control_type =
        DehumidificationControlType::None;
    let model = SimulationModel::from_typed(typed);
    let schedule_cache =
        precompute_schedule_cache(&model.typed, 1).expect("one CP356 schedule sample");
    let weather = weather_series_with_conditions(&model, 1, 30.0, 15.0, 30.0, 101_325.0);
    let mut options = DirectZonePurchasedAirCoupledOptions::hourly_samples(1);
    options.initial_zone_air_temperature_c = INITIAL_ZONE_TEMPERATURE_C;
    let simulation = simulate_direct_zone_purchased_air_coupled_heat_balance(
        &model,
        &weather,
        &schedule_cache,
        options,
    )
    .expect("valid CP356 direct simulation");
    let summary = &simulation.summary;
    let lifecycle =
        &summary.calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_lifecycle;
    let state = &lifecycle.state;
    let snapshot = state.latest.expect("latest CP356 snapshot");
    let predecessor = summary
        .calc_cooling_constant_shr_supply_humidity_ratio_minimum_limit_lifecycle
        .state
        .latest
        .expect("latest CP355 predecessor");

    assert_eq!(
        lifecycle.source,
        crate::ideal_loads::PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE
    );
    assert_eq!(
        lifecycle.first_excluded_source,
        crate::ideal_loads::PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE
    );
    assert_eq!(
        crate::ideal_loads::PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE_ORDER
            .len(),
        4
    );
    assert_eq!(state.transition_count, 1);
    assert_eq!(
        state.dehumidification_control_none_case_completed_skip_count,
        1
    );
    assert_eq!(
        state
            .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_mixed_air_limit_count,
        0
    );
    for count in [
        state.source_site_execution_count,
        state.supply_humidity_ratio_for_mixed_air_limit_minimum_read_count,
        state.mixed_air_humidity_ratio_for_minimum_read_count,
        state.source_shaped_two_argument_minimum_evaluation_count,
        state.supply_humidity_ratio_assignment_write_count,
    ] {
        assert_eq!(count, 0);
    }
    assert!(snapshot.dehumidification_control_none_case_completed_skip);
    assert!(
        !snapshot
            .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_mixed_air_limit_executed
    );
    assert!(!snapshot.supply_humidity_ratio_for_mixed_air_limit_minimum_read);
    assert!(!snapshot.mixed_air_humidity_ratio_for_minimum_read);
    assert!(!snapshot.source_shaped_two_argument_minimum_evaluated);
    assert!(!snapshot.supply_humidity_ratio_assignment_performed);
    assert_eq!(
        [
            snapshot.supply_humidity_ratio_before_mixed_air_limit,
            snapshot.mixed_air_humidity_ratio,
            snapshot.minimum_supply_humidity_ratio,
            snapshot.assigned_supply_humidity_ratio,
            snapshot.resulting_supply_humidity_ratio,
        ],
        [None; 5]
    );

    let expected =
        super::super::cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_validation::expected_snapshot(
            predecessor,
        );
    assert!(
        super::super::cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_validation::snapshots_match_exact_bits(
            &snapshot,
            &expected,
        )
    );
    let mut corrupted = snapshot;
    corrupted.source_order = &["forged-cp356-source-order"];
    assert!(
        !super::super::cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_validation::snapshots_match_exact_bits(
            &corrupted,
            &expected,
        )
    );

    let numerical_humidity = summary
        .calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle
        .state
        .latest
        .expect("unchanged numerical humidity-owner snapshot")
        .assigned_supply_humidity_ratio
        .expect("unchanged numerical supply humidity ratio");
    assert!(
        numerical_humidity.is_finite(),
        "CP356 complete-null evidence must not replace numerical supply humidity"
    );
}
