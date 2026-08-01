use super::*;

#[test]
fn missing_direct_lifecycle_fails_closed() {
    assert!(
        validate_direct_lifecycle(
            None,
            DirectLifecyclePredecessors {
                pre_saturation_original_assignment_cp376: None,
                temperature_mixed_air_limit_cp334: None,
                capacity_temperature_mixed_air_limit_cp344: None,
            },
            None,
            None,
        )
        .is_err()
    );
}

#[test]
fn direct_counter_algebra_accepts_cp334_owner_and_rejects_overflow() {
    let system = IdealLoadsAirSystemId(0);
    let mut state = State::new(system);
    state.transition_count = 1;
    state.heating_availability_guard_false_fallthrough_count = 1;
    state.source_site_execution_count = 4;
    state.purchased_air_supply_temperature_for_saturation_humidity_ratio_read_count = 1;
    state.environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read_count = 1;
    state.psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluation_count = 1;
    state.local_saturation_supply_humidity_ratio_assignment_count = 1;
    state.cp334_supply_temperature_mixed_air_limit_owner_count = 1;
    state.environment_outdoor_barometric_pressure_owner_count = 1;
    let mut predecessor = PredecessorState::new(system);
    predecessor.transition_count = 1;
    predecessor.heating_availability_guard_false_fallthrough_count = 1;
    let cp344 = Cp344State::new(system);

    assert!(counts::validate(&state, &predecessor, &cp344, 1).is_ok());
    state.unit_off_skip_count = usize::MAX;
    predecessor.unit_off_skip_count = usize::MAX;
    assert!(counts::validate(&state, &predecessor, &cp344, 1).is_err());
}

#[test]
fn pressure_and_psychrometric_counters_must_match_the_active_partition() {
    let system = IdealLoadsAirSystemId(0);
    let mut state = State::new(system);
    state.transition_count = 1;
    state.unit_off_skip_count = 1;
    let mut predecessor = PredecessorState::new(system);
    predecessor.transition_count = 1;
    predecessor.unit_off_skip_count = 1;
    let cp344 = Cp344State::new(system);
    assert!(counts::validate(&state, &predecessor, &cp344, 1).is_ok());

    state.environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read_count = 1;
    assert!(counts::validate(&state, &predecessor, &cp344, 1).is_err());
}
