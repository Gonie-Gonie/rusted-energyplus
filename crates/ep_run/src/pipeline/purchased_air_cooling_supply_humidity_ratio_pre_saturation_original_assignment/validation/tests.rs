use super::*;

#[test]
fn missing_direct_lifecycle_fails_closed() {
    assert!(
        validate_direct_lifecycle(
            None,
            DirectLifecyclePredecessors {
                maximum_assignment_cp375: None,
                none_case_cp347: None,
            },
            None,
            None,
        )
        .is_err()
    );
}

#[test]
fn direct_counter_algebra_accepts_cp347_owner_and_rejects_overflow() {
    let system = IdealLoadsAirSystemId(0);
    let mut state = State::new(system);
    state.transition_count = 1;
    state.heating_availability_guard_false_fallthrough_count = 1;
    state.source_site_execution_count = 2;
    state.purchased_air_supply_humidity_ratio_before_saturation_limit_read_count = 1;
    state.local_original_supply_humidity_ratio_before_saturation_limit_assignment_count = 1;
    state.cp347_none_case_owner_count = 1;
    let mut predecessor = PredecessorState::new(system);
    predecessor.transition_count = 1;
    predecessor.heating_availability_guard_false_fallthrough_count = 1;
    let mut owner = OwnerState::new(system);
    owner.dehumidification_control_none_case_completion_count = 1;

    assert!(validate_counts(&state, &predecessor, &owner, 1).is_ok());
    state.unit_off_skip_count = usize::MAX;
    predecessor.unit_off_skip_count = usize::MAX;
    assert!(validate_counts(&state, &predecessor, &owner, 1).is_err());
}

#[test]
fn public_direct_route_shape_rejects_coordinated_selector_forgery() {
    let no_owner = [false; 5];
    let cp347_owner = [false, true, false, false, false];
    let direct_none = (
        Some(DehumidificationControlType::None),
        Some(DehumidificationControlType::None),
    );
    let forged_private = (
        Some(DehumidificationControlType::ConstantSensibleHeatRatio),
        Some(DehumidificationControlType::ConstantSensibleHeatRatio),
    );

    for route in 0..=2 {
        assert!(public_direct_route_shape(
            Some(route),
            (None, None),
            no_owner,
        ));
        assert!(!public_direct_route_shape(
            Some(route),
            forged_private,
            no_owner,
        ));
    }
    for route in 3..=4 {
        assert!(public_direct_route_shape(
            Some(route),
            direct_none,
            cp347_owner,
        ));
        assert!(!public_direct_route_shape(
            Some(route),
            forged_private,
            cp347_owner,
        ));
    }
    for route in 5..=7 {
        assert!(!public_direct_route_shape(
            Some(route),
            direct_none,
            cp347_owner,
        ));
    }
}
