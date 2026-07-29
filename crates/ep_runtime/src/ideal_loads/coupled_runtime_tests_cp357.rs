use super::*;

#[test]
fn cp357_coupled_direct_none_route_is_complete_skip_and_numerical_humidity_remains_unfed() {
    let mut typed = exact_model(1).typed;
    typed.schedules[1].hourly_value = 0.0;
    typed.schedules[2].hourly_value = 15.0;
    typed.schedules[3].hourly_value = 1.0;
    typed.ideal_loads_air_systems[0].dehumidification_control_type =
        DehumidificationControlType::None;
    let model = SimulationModel::from_typed(typed);
    let schedule_cache = precompute_schedule_cache(&model.typed, 1);
    assert!(schedule_cache.is_ok(), "one CP357 schedule sample");
    let Ok(schedule_cache) = schedule_cache else {
        return;
    };
    let weather = weather_series_with_conditions(&model, 1, 30.0, 15.0, 30.0, 101_325.0);
    let mut options = DirectZonePurchasedAirCoupledOptions::hourly_samples(1);
    options.initial_zone_air_temperature_c = INITIAL_ZONE_TEMPERATURE_C;
    let simulation = simulate_direct_zone_purchased_air_coupled_heat_balance(
        &model,
        &weather,
        &schedule_cache,
        options,
    );
    assert!(simulation.is_ok(), "valid CP357 direct simulation");
    let Ok(simulation) = simulation else {
        return;
    };
    let summary = &simulation.summary;
    let lifecycle = &summary.calc_cooling_constant_shr_case_break_lifecycle;
    let state = &lifecycle.state;
    assert!(state.latest.is_some(), "latest CP357 snapshot");
    let Some(snapshot) = state.latest else {
        return;
    };
    let predecessor = summary
        .calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_lifecycle
        .state
        .latest;
    assert!(predecessor.is_some(), "latest CP356 predecessor");
    let Some(predecessor) = predecessor else {
        return;
    };

    assert_eq!(
        lifecycle.source,
        crate::ideal_loads::PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_CASE_BREAK_SOURCE
    );
    assert_eq!(
        lifecycle.first_excluded_source,
        crate::ideal_loads::
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_CASE_BREAK_FIRST_EXCLUDED_SOURCE
    );
    assert_eq!(
        crate::ideal_loads::PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_CASE_BREAK_SOURCE_ORDER.len(),
        1
    );
    assert_eq!(state.transition_count, 1);
    assert_eq!(
        state.dehumidification_control_none_case_completed_skip_count,
        1
    );
    assert_eq!(
        state.dehumidification_control_constant_sensible_heat_ratio_case_break_count,
        0
    );
    assert_eq!(state.source_site_execution_count, 0);
    assert!(snapshot.dehumidification_control_none_case_completed_skip);
    assert!(!snapshot.dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break);
    assert!(!snapshot.dehumidification_control_humidistat_case_selected_skip);
    assert!(!snapshot.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip);

    let expected =
        super::super::cooling_constant_shr_case_break_validation::expected_snapshot(predecessor);
    assert!(
        super::super::cooling_constant_shr_case_break_validation::snapshots_match_exact(
            &snapshot, &expected,
        )
    );
    for corruption in ["source_order", "break_boolean"] {
        let mut corrupted = snapshot;
        match corruption {
            "source_order" => corrupted.source_order = &["forged-cp357-source-order"],
            "break_boolean" => {
                corrupted
                    .dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break =
                    true;
            }
            _ => unreachable!(),
        }
        assert!(
            !super::super::cooling_constant_shr_case_break_validation::snapshots_match_exact(
                &corrupted, &expected,
            ),
            "{corruption}"
        );
    }

    let numerical_owner = summary
        .calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle
        .state
        .latest;
    assert!(
        numerical_owner.is_some(),
        "unchanged numerical humidity-owner snapshot"
    );
    let Some(numerical_owner) = numerical_owner else {
        return;
    };
    let numerical_humidity = numerical_owner.assigned_supply_humidity_ratio;
    assert!(
        numerical_humidity.is_some(),
        "unchanged numerical supply humidity ratio"
    );
    let Some(numerical_humidity) = numerical_humidity else {
        return;
    };
    assert!(
        numerical_humidity.is_finite(),
        "CP357 complete-skip evidence must not replace numerical supply humidity"
    );
}
