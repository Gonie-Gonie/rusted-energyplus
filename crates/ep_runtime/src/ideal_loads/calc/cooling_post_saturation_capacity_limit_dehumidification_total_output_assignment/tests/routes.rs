//! CP382 eighteen-route, provenance, and counter-algebra tests.

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentRuntimeState as State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_state as advance,
};
use super::{active_input, predecessor_for_route};
use crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_snapshot_is_exact_direct_release;

#[test]
fn cp382_retains_eighteen_routes_and_exact_counter_algebra() {
    let mut state = State::new(predecessor_for_route(0, 0, 1).system);
    let mut ordinal = 0;

    for inherited in 0..3 {
        ordinal += 1;
        let predecessor = predecessor_for_route(inherited, 0, ordinal);
        let snapshot = advance(&mut state, predecessor, None).expect("complete skip route");
        assert_eq!(state.latest_route, Some(expected_route(inherited, 0)));
        assert!(
            cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_snapshot_is_exact_direct_release(snapshot)
        );
    }

    for inherited in 3..8 {
        for outcome in 0..3 {
            ordinal += 1;
            let predecessor = predecessor_for_route(inherited, outcome, ordinal);
            let input = (outcome == 1)
                .then(|| active_input(1.5, 48_000.0, 40_000.0).expect("active CP382 input"));
            let snapshot = advance(&mut state, predecessor, input).expect("complete CP382 route");
            assert_eq!(state.latest_route, Some(expected_route(inherited, outcome)));
            assert_eq!(
                cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_snapshot_is_exact_direct_release(snapshot),
                inherited <= 4,
            );
            assert_eq!(
                snapshot.dehumidification_total_output_assignment_executed,
                outcome == 1,
            );
        }
    }

    assert_eq!(state.transition_count, 18);
    assert_eq!(state.unit_off_skip_count, 1);
    assert_eq!(state.non_cooling_skip_count, 1);
    assert_eq!(state.positive_guard_false_fallthrough_skip_count, 1);
    assert_eq!(state.heating_availability_guard_false_fallthrough_count, 3);
    assert_eq!(
        state.humidification_control_guard_false_fallthrough_count,
        3
    );
    assert_eq!(
        state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        3,
    );
    assert_eq!(
        state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        3,
    );
    assert_eq!(
        state.dehumidification_control_guard_false_fallthrough_count,
        3
    );
    assert_eq!(state.dehumidification_total_output_assignment_count, 5);
    assert_eq!(state.source_site_execution_count, 30);

    for count in [
        state.cp330_supply_mass_flow_rate_owned_read_count,
        state.cp329_same_call_supply_mass_flow_rate_bit_corroboration_count,
        state.cp339_same_call_supply_mass_flow_rate_bit_corroboration_count,
        state.supply_mass_flow_rate_read_count,
        state.cp329_mixed_air_enthalpy_owned_read_count,
        state.cp329_same_call_recirculation_enthalpy_bit_corroboration_count,
        state.cp339_same_call_mixed_air_enthalpy_bit_corroboration_count,
        state.mixed_air_enthalpy_read_count,
        state.cp379_post_saturation_supply_enthalpy_owned_read_count,
        state.cp379_same_call_supply_enthalpy_bits_corroboration_count,
        state.supply_enthalpy_read_count,
        state.enthalpy_difference_calculation_count,
        state.cooling_total_output_calculation_count,
        state.cooling_total_output_assignment_write_count,
    ] {
        assert_eq!(count, 5);
    }

    for counts in route_partition_counts(&state) {
        assert_eq!(counts, [1, 1, 1, 1]);
    }
}

#[test]
fn cp382_requires_input_only_for_assignment_and_all_eight_provenance_bits() {
    let assignment_predecessor = predecessor_for_route(4, 1, 1);
    let skipped_predecessor = predecessor_for_route(4, 0, 1);

    let mut skipped = State::new(skipped_predecessor.system);
    let before = skipped.clone();
    assert!(
        advance(
            &mut skipped,
            skipped_predecessor,
            active_input(1.0, 48_000.0, 40_000.0),
        )
        .is_none()
    );
    assert_eq!(skipped, before);

    let mut missing = State::new(assignment_predecessor.system);
    let before = missing.clone();
    assert!(advance(&mut missing, assignment_predecessor, None).is_none());
    assert_eq!(missing, before);

    for missing_evidence in 0..8 {
        let mut state = State::new(assignment_predecessor.system);
        let before = state.clone();
        let mut input = active_input(1.0, 48_000.0, 40_000.0).expect("active input");
        match missing_evidence {
            0 => input.cp330_supply_mass_flow_rate_owned_read = false,
            1 => input.cp329_same_call_supply_mass_flow_rate_bit_corroborated = false,
            2 => input.cp339_same_call_supply_mass_flow_rate_bit_corroborated = false,
            3 => input.cp329_mixed_air_enthalpy_owned_read = false,
            4 => input.cp329_same_call_recirculation_enthalpy_bit_corroborated = false,
            5 => input.cp339_same_call_mixed_air_enthalpy_bit_corroborated = false,
            6 => input.cp379_post_saturation_supply_enthalpy_owned_read = false,
            7 => input.cp379_same_call_supply_enthalpy_bits_corroborated = false,
            _ => unreachable!(),
        }
        assert!(advance(&mut state, assignment_predecessor, Some(input)).is_none());
        assert_eq!(state, before);
    }
}

fn route_partition_counts(state: &State) -> [[usize; 4]; 5] {
    [
        [
            state.heating_availability_guard_false_fallthrough_capacity_guard_false_count,
            state.heating_availability_guard_false_fallthrough_dehumidification_body_entry_count,
            state.heating_availability_guard_false_fallthrough_dehumidification_guard_false_count,
            state.heating_availability_guard_false_fallthrough_dehumidification_total_output_assignment_count,
        ],
        [
            state.humidification_control_guard_false_fallthrough_capacity_guard_false_count,
            state.humidification_control_guard_false_fallthrough_dehumidification_body_entry_count,
            state.humidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
            state.humidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count,
        ],
        [
            state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count,
            state.dehumidification_control_humidistat_maximum_assignment_dehumidification_body_entry_count,
            state.dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count,
            state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_assignment_count,
        ],
        [
            state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count,
            state.dehumidification_control_none_maximum_assignment_dehumidification_body_entry_count,
            state.dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count,
            state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_assignment_count,
        ],
        [
            state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count,
            state.dehumidification_control_guard_false_fallthrough_dehumidification_body_entry_count,
            state.dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
            state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count,
        ],
    ]
}

fn expected_route(inherited: usize, outcome: usize) -> Route {
    use Route as R;
    match (inherited, outcome) {
        (0, 0) => R::UnitOff,
        (1, 0) => R::NonCooling,
        (2, 0) => R::PositiveGuardFalseFallthrough,
        (3, 0) => R::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough,
        (3, 1) => R::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputAssigned,
        (3, 2) => R::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough,
        (4, 0) => R::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough,
        (4, 1) => R::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputAssigned,
        (4, 2) => R::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough,
        (5, 0) => R::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough,
        (5, 1) => R::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputAssigned,
        (5, 2) => R::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough,
        (6, 0) => R::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough,
        (6, 1) => R::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputAssigned,
        (6, 2) => R::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough,
        (7, 0) => R::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough,
        (7, 1) => R::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputAssigned,
        (7, 2) => R::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough,
        _ => unreachable!(),
    }
}
