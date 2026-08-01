//! CP381 eighteen-route, owner, and counter-algebra tests.

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardRetainedRoute as Route,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardRuntimeState as State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_guard_state as advance,
};
use super::{active_input, predecessor_for_route};
use crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_guard_snapshot_is_exact_direct_release;

#[test]
fn cp381_retains_eighteen_routes_and_exact_counter_algebra() {
    let mut state = State::new(predecessor_for_route(0, false, 1).system);
    let mut ordinal = 0;

    for inherited in 0..3 {
        ordinal += 1;
        let predecessor = predecessor_for_route(inherited, false, ordinal);
        let snapshot = advance(&mut state, predecessor, None).expect("complete skip route");
        assert_eq!(state.latest_route, Some(expected_route(inherited, 0)));
        assert!(
            cooling_post_saturation_capacity_limit_dehumidification_guard_snapshot_is_exact_direct_release(snapshot)
        );
    }

    for inherited in 3..8 {
        ordinal += 1;
        let predecessor = predecessor_for_route(inherited, false, ordinal);
        let snapshot = advance(&mut state, predecessor, None).expect("CP380 outer-false route");
        assert_eq!(state.latest_route, Some(expected_route(inherited, 0)));
        assert_eq!(
            cooling_post_saturation_capacity_limit_dehumidification_guard_snapshot_is_exact_direct_release(snapshot),
            inherited <= 4,
        );

        for outcome in [1, 2] {
            ordinal += 1;
            let predecessor = predecessor_for_route(inherited, true, ordinal);
            let input = if outcome == 1 {
                active_input(0.007, 0.009)
            } else {
                active_input(0.009, 0.009)
            };
            let snapshot = advance(&mut state, predecessor, input).expect("active CP381 route");
            assert_eq!(state.latest_route, Some(expected_route(inherited, outcome)));
            assert_eq!(
                cooling_post_saturation_capacity_limit_dehumidification_guard_snapshot_is_exact_direct_release(snapshot),
                inherited <= 4,
            );
        }
    }

    assert_eq!(state.transition_count, 18);
    assert_eq!(state.unit_off_skip_count, 1);
    assert_eq!(state.non_cooling_skip_count, 1);
    assert_eq!(state.positive_guard_false_fallthrough_skip_count, 1);
    assert_eq!(state.dehumidification_guard_evaluation_count, 10);
    assert_eq!(state.dehumidification_body_entry_count, 5);
    assert_eq!(state.dehumidification_guard_false_fallthrough_count, 5);
    assert_eq!(
        state.supply_humidity_ratio_strictly_less_than_mixed_air_humidity_ratio_count,
        5,
    );
    assert_eq!(state.source_site_execution_count, 35);

    for (inherited, cp380_body, cp380_false, body, guard_false) in [
        (
            state.heating_availability_guard_false_fallthrough_count,
            state.heating_availability_guard_false_fallthrough_body_entry_count,
            state.heating_availability_guard_false_fallthrough_capacity_guard_false_count,
            state.heating_availability_guard_false_fallthrough_dehumidification_body_entry_count,
            state.heating_availability_guard_false_fallthrough_dehumidification_guard_false_count,
        ),
        (
            state.humidification_control_guard_false_fallthrough_count,
            state.humidification_control_guard_false_fallthrough_body_entry_count,
            state.humidification_control_guard_false_fallthrough_capacity_guard_false_count,
            state.humidification_control_guard_false_fallthrough_dehumidification_body_entry_count,
            state.humidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
        ),
        (
            state
                .dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
            state.dehumidification_control_humidistat_maximum_assignment_body_entry_count,
            state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count,
            state
                .dehumidification_control_humidistat_maximum_assignment_dehumidification_body_entry_count,
            state
                .dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count,
        ),
        (
            state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
            state.dehumidification_control_none_maximum_assignment_body_entry_count,
            state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count,
            state
                .dehumidification_control_none_maximum_assignment_dehumidification_body_entry_count,
            state
                .dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count,
        ),
        (
            state.dehumidification_control_guard_false_fallthrough_count,
            state.dehumidification_control_guard_false_fallthrough_body_entry_count,
            state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count,
            state
                .dehumidification_control_guard_false_fallthrough_dehumidification_body_entry_count,
            state
                .dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
        ),
    ] {
        assert_eq!(inherited, cp380_body + cp380_false);
        assert_eq!(cp380_body, body + guard_false);
    }
    assert_eq!(
        state.transition_count,
        state.unit_off_skip_count
            + state.non_cooling_skip_count
            + state.positive_guard_false_fallthrough_skip_count
            + 5
            + state.dehumidification_body_entry_count
            + state.dehumidification_guard_false_fallthrough_count,
    );
    assert_eq!(
        state.source_site_execution_count,
        3 * state.dehumidification_guard_evaluation_count + state.dehumidification_body_entry_count,
    );
}

#[test]
fn cp381_requires_only_active_owner_evidence_and_rejects_placeholders_transactionally() {
    let active_predecessor = predecessor_for_route(4, true, 1);
    let skipped_predecessor = predecessor_for_route(4, false, 1);

    let mut skipped = State::new(skipped_predecessor.system);
    let before = skipped.clone();
    assert!(
        advance(
            &mut skipped,
            skipped_predecessor,
            active_input(0.007, 0.009),
        )
        .is_none()
    );
    assert_eq!(skipped, before);

    for missing_owner in 0..3 {
        let mut state = State::new(active_predecessor.system);
        let before = state.clone();
        let mut input = active_input(0.007, 0.009).expect("active input");
        match missing_owner {
            0 => input.cp378_supply_humidity_ratio_saturation_limit_owned_read = false,
            1 => input.cp379_same_call_supply_humidity_ratio_bit_corroborated = false,
            2 => input.cp329_mixed_air_humidity_ratio_owned_read = false,
            _ => unreachable!(),
        }
        assert!(advance(&mut state, active_predecessor, Some(input)).is_none());
        assert_eq!(state, before);
    }

    let mut missing_input = State::new(active_predecessor.system);
    let before = missing_input.clone();
    assert!(advance(&mut missing_input, active_predecessor, None).is_none());
    assert_eq!(missing_input, before);

    let mut wrong_system = State::new(ep_model::IdealLoadsAirSystemId(999));
    let before = wrong_system.clone();
    assert!(
        advance(
            &mut wrong_system,
            active_predecessor,
            active_input(0.007, 0.009),
        )
        .is_none()
    );
    assert_eq!(wrong_system, before);
}

fn expected_route(inherited: usize, outcome: usize) -> Route {
    use Route as R;
    match (inherited, outcome) {
        (0, 0) => R::UnitOff,
        (1, 0) => R::NonCooling,
        (2, 0) => R::PositiveGuardFalseFallthrough,
        (3, 0) => R::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough,
        (3, 1) => R::HeatingAvailabilityGuardFalseFallthroughDehumidificationBodyEntered,
        (3, 2) => {
            R::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough
        }
        (4, 0) => R::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough,
        (4, 1) => R::HumidificationControlGuardFalseFallthroughDehumidificationBodyEntered,
        (4, 2) => {
            R::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough
        }
        (5, 0) => {
            R::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough
        }
        (5, 1) => {
            R::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationBodyEntered
        }
        (5, 2) => {
            R::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough
        }
        (6, 0) => {
            R::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough
        }
        (6, 1) => {
            R::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationBodyEntered
        }
        (6, 2) => {
            R::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough
        }
        (7, 0) => {
            R::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
        }
        (7, 1) => {
            R::DehumidificationControlGuardFalseFallthroughDehumidificationBodyEntered
        }
        (7, 2) => {
            R::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough
        }
        _ => unreachable!(),
    }
}
