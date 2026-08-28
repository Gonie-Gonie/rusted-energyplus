//! CP431 boundary, exhaustive route, short-circuit, and overflow tests.

mod schema_prefix;

use super::transition::{
    PurchasedAirCalcHeatingModeGuardActiveInput as ActiveInput,
    advance_heating_mode_guard_state as advance,
    advance_heating_mode_guard_state_with_validated_route as advance_validated,
    heating_mode_guard_route_from_committed_predecessor as successor_route,
    predecessor_route,
};
use super::PurchasedAirCalcHeatingModeGuardRuntimeState as State;
use crate::ideal_loads::calc::{
    PurchasedAirCalcCoolingEntryGateCommittedHeatingModeGuardNumericOperands as Numeric,
    PurchasedAirTemperatureControlType as Control,
    advance_heating_or_no_load_case_entry_state as advance_cp430,
    cp429_all_snapshots_for_successor_tests,
};
use crate::ideal_loads::PurchasedAirCalcHeatingOrNoLoadCaseEntrySnapshot as Predecessor;

#[test]
fn cp431_boundary_and_six_short_circuit_sites_are_exact() {
    assert_eq!(
        super::PURCHASED_AIR_CALC_HEATING_MODE_GUARD_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2348",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_HEATING_MODE_GUARD_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2349",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_HEATING_MODE_GUARD_SOURCE_ORDER,
        &[
            "read-minimum-outdoor-air-sensible-output",
            "read-heating-setpoint-demand",
            "compare-strict-less-than",
            "read-zone-temperature-control-type-after-short-circuit",
            "exclude-exact-single-cooling-control",
            "enter-heating-mode-body-if-admitted",
        ],
    );
}

#[test]
fn exhaustive_61_routes_split_only_index_one_and_account_exact_sites() {
    let mut state = State::new(cp430_all_snapshots()[0].system);
    let mut total = 0usize;
    let mut public = 0usize;
    let mut private = 0usize;
    for predecessor in cp430_all_snapshots() {
        let active = active_inputs();
        let inactive = [None];
        let inputs: &[Option<ActiveInput>] = if predecessor.heating_or_no_load_case_entered {
            &active
        } else {
            &inactive
        };
        for input in inputs.iter().copied() {
            let predecessor_route = predecessor_route(predecessor).expect("CP430 route");
            let route = successor_route(
                predecessor,
                predecessor_route,
                input,
            )
            .expect("CP431 route");
            let snapshot = advance_validated(
                &mut state,
                predecessor,
                predecessor_route,
                input,
                route,
            )
            .expect("CP431");
            total += 1;
            let single_cool_private = route.single_cool_blocked;
            if is_public_logical_index(route.logical_index) && !single_cool_private {
                public += 1;
            } else {
                private += 1;
            }
            assert_eq!(route.guard_evaluated, route.logical_index == 1);
            assert_eq!(snapshot.heating_mode_guard_evaluated, route.guard_evaluated);
            assert_bits(
                snapshot.resulting_supply_humidity_ratio,
                predecessor.resulting_supply_humidity_ratio,
            );
            assert_bits(
                snapshot.resulting_supply_enthalpy_j_per_kg,
                predecessor.resulting_supply_enthalpy_j_per_kg,
            );
            assert_bits(
                snapshot.resulting_supply_temperature_c,
                predecessor.resulting_supply_temperature_c,
            );
            assert!(super::heating_mode_guard_snapshot_is_exact(snapshot));
        }
    }
    assert_eq!((total, public, private), (61, 20, 41));
    assert_eq!(state.transition_count, 61);
    assert_eq!(state.inactive_transition_count, 58);
    assert_eq!(state.heating_mode_guard_evaluation_count, 3);
    assert_eq!(state.source_site_execution_count, 14);
    assert_eq!(state.minimum_outdoor_air_sensible_output_strictly_less_than_heating_setpoint_demand_count, 2);
    assert_eq!(state.temperature_control_type_read_after_sensible_comparison_short_circuit_count, 2);
    assert_eq!(state.single_cool_block_count, 1);
    assert_eq!(state.heating_operating_mode_body_entry_count, 1);
    assert_eq!(state.heating_mode_guard_false_fallthrough_count, 2);
    assert_eq!(nonzero_indices(&state.heating_mode_guard_evaluation_route_counts), [1]);
    assert_eq!(nonzero_indices(&state.heating_operating_mode_body_entry_route_counts), [1]);
    assert_eq!(nonzero_indices(&state.heating_mode_guard_false_fallthrough_route_counts), [1]);
    assert_eq!(state.cp430_supply_humidity_ratio_state_owner_count, 37);
    assert_eq!(state.unchanged_supply_humidity_ratio_preservation_count, 37);
    assert_eq!(state.cp430_supply_enthalpy_state_owner_count, 42);
    assert_eq!(state.unchanged_supply_enthalpy_preservation_count, 42);
    assert_eq!(state.cp430_supply_temperature_state_owner_count, 57);
    assert_eq!(state.unchanged_supply_temperature_preservation_count, 57);
    assert!(super::release::state_counts_are_consistent_for_test(&state));
}

#[test]
fn strict_ieee_comparison_and_thermostat_short_circuit_are_exact() {
    let predecessor = active_predecessor();
    for (minimum, heating) in [(0.0, -0.0), (-0.0, 0.0), (1.0, 1.0), (f64::NAN, 1.0)] {
        let input = active_input(minimum, heating, None);
        let snapshot = advance(&mut State::new(predecessor.system), predecessor, input)
            .expect("numeric false");
        assert_eq!(
            snapshot.minimum_outdoor_air_sensible_output_strictly_less_than_heating_setpoint_demand,
            Some(false),
        );
        assert!(!snapshot.prevalidated_temperature_control_type_owned_read);
        assert!(snapshot.temperature_control_type.is_none());
        assert!(snapshot.heating_mode_guard_false_fallthrough);
    }
    let blocked = advance(
        &mut State::new(predecessor.system),
        predecessor,
        active_input(0.0, 1.0, Some(Control::SingleCool)),
    )
    .expect("blocked");
    assert!(blocked.single_cool_blocked);
    assert!(!blocked.heating_operating_mode_body_entered);
    let body = advance(
        &mut State::new(predecessor.system),
        predecessor,
        active_input(0.0, 1.0, Some(Control::DualHeatCool)),
    )
    .expect("body");
    assert!(body.heating_operating_mode_body_entered);
    assert!(!body.heating_mode_guard_false_fallthrough);
}

#[test]
fn every_route_component_forgery_is_transactional() {
    let predecessor = active_predecessor();
    let input = active_input(0.0, 1.0, Some(Control::DualHeatCool));
    let predecessor_route = predecessor_route(predecessor).expect("route");
    let route = successor_route(predecessor, predecessor_route, input).expect("CP431 route");
    for component in 0..11 {
        let mut forged = route;
        match component {
            0 => forged.logical_index = 2,
            1 => forged.predecessor_active ^= true,
            2 => forged.predecessor_assignment_executed ^= true,
            3 => forged.predecessor_entered ^= true,
            4 => forged.predecessor_total_output_assignment_executed ^= true,
            5 => forged.predecessor_heating_or_no_load_case_entered ^= true,
            6 => forged.guard_evaluated ^= true,
            7 => forged.sensible_comparison_satisfied ^= true,
            8 => forged.single_cool_blocked ^= true,
            9 => forged.body_entered ^= true,
            _ => forged.false_fallthrough ^= true,
        }
        let mut state = State::new(predecessor.system);
        let before = state.clone();
        assert!(advance_validated(
            &mut state,
            predecessor,
            predecessor_route,
            input,
            forged,
        )
        .is_none());
        assert_eq!(state, before, "route component {component}");
    }
}

#[test]
fn every_supplied_cp430_route_component_forgery_is_transactional() {
    let predecessor = active_predecessor();
    let input = active_input(0.0, 1.0, Some(Control::DualHeatCool));
    let predecessor_route = predecessor_route(predecessor).expect("CP430 route");
    let route = successor_route(predecessor, predecessor_route, input).expect("CP431 route");
    for component in 0..6 {
        let mut forged = predecessor_route;
        match component {
            0 => forged.logical_index = 2,
            1 => forged.active ^= true,
            2 => forged.predecessor_assignment_executed ^= true,
            3 => forged.predecessor_entered ^= true,
            4 => forged.assignment_executed ^= true,
            _ => forged.entered ^= true,
        }
        let mut state = State::new(predecessor.system);
        let before = state.clone();
        assert!(advance_validated(&mut state, predecessor, forged, input, route).is_none());
        assert_eq!(state, before, "CP430 route component {component}");
    }
}

#[test]
fn prior_latest_route_local_matcher_rejects_each_ancestry_component_forgery() {
    let predecessor = active_predecessor();
    let input = active_input(0.0, 1.0, Some(Control::DualHeatCool));
    let predecessor_route = predecessor_route(predecessor).expect("CP430 route");
    let route = successor_route(predecessor, predecessor_route, input).expect("CP431 route");
    let snapshot = advance_validated(
        &mut State::new(predecessor.system),
        predecessor,
        predecessor_route,
        input,
        route,
    )
    .expect("CP431");
    assert!(super::release::retained_prior_route_matches_for_test(snapshot, route));
    for component in 0..6 {
        let mut forged = route;
        match component {
            0 => forged.logical_index = 2,
            1 => forged.predecessor_active ^= true,
            2 => forged.predecessor_assignment_executed ^= true,
            3 => forged.predecessor_entered ^= true,
            4 => forged.predecessor_total_output_assignment_executed ^= true,
            _ => forged.predecessor_heating_or_no_load_case_entered ^= true,
        }
        assert!(
            !super::release::retained_prior_route_matches_for_test(snapshot, forged),
            "prior route component {component}",
        );
    }
}

#[test]
fn every_active_counter_overflow_is_transactional() {
    let predecessor = active_predecessor();
    let input = active_input(0.0, 1.0, Some(Control::DualHeatCool));
    let predecessor_route = predecessor_route(predecessor).expect("route");
    let route = successor_route(predecessor, predecessor_route, input).expect("route");
    for counter in 0..12 {
        let mut state = State::new(predecessor.system);
        match counter {
            0 => state.transition_count = usize::MAX,
            1 => state.predecessor_route_counts[1] = usize::MAX,
            2 => state.heating_mode_guard_evaluation_count = usize::MAX,
            3 => state.heating_mode_guard_evaluation_route_counts[1] = usize::MAX,
            4 => state.source_site_execution_count = usize::MAX,
            5 => state.cp311_retained_minimum_outdoor_air_sensible_output_owner_read_count = usize::MAX,
            6 => state.cp312_same_call_minimum_outdoor_air_sensible_output_bit_corroboration_count = usize::MAX,
            7 => state.minimum_outdoor_air_sensible_output_heating_setpoint_demand_comparison_count = usize::MAX,
            8 => state.prevalidated_temperature_control_type_owner_read_count = usize::MAX,
            9 => state.temperature_control_type_permits_heating_count = usize::MAX,
            10 => state.heating_operating_mode_body_entry_count = usize::MAX,
            _ => state.heating_operating_mode_body_entry_route_counts[1] = usize::MAX,
        }
        let before = state.clone();
        assert!(advance_validated(
            &mut state,
            predecessor,
            predecessor_route,
            input,
            route,
        )
        .is_none());
        assert_eq!(state, before, "counter {counter}");
    }
}

#[test]
fn cp431_hot_path_is_bounded_and_owner_acquisition_is_short_circuited() {
    let source = include_str!("release.rs");
    let start = source.find("pub fn advance_direct_no_oa_calc_").expect("hot start");
    let end = source[start..]
        .find("#[allow(dead_code)]")
        .map(|offset| start + offset)
        .expect("hot end");
    let hot = &source[start..end];
    for forbidden in [
        "completed_",
        "snapshot_is_exact",
        "private_characterization",
        "predecessor_route(",
        "_snapshot_route(",
    ] {
        assert!(!hot.contains(forbidden), "{forbidden}");
    }
    assert_eq!(hot.matches("heating_or_no_load_case_entry_committed_latest_route(").count(), 1);
    assert_eq!(hot.matches("heating_mode_guard_numeric_operands(unit)").count(), 1);
    assert_eq!(hot.matches("heating_mode_guard_temperature_control_type(unit)").count(), 1);
    assert!(hot.find("if active").expect("active guard") < hot.find("numeric_operands(unit)").expect("numeric owner"));
    assert!(hot.find("if first_satisfied").expect("short circuit") < hot.find("temperature_control_type(unit)").expect("type owner"));

    let transition = include_str!("transition.rs");
    let start = transition
        .find("fn advance_heating_mode_guard_state_with_validated_route")
        .expect("validated transition start");
    let end = transition[start..]
        .find("pub(super) fn predecessor_route")
        .map(|offset| start + offset)
        .expect("validated transition end");
    assert_no_route_replay(&transition[start..end]);

    let validation = include_str!("release/snapshot_validation.rs");
    let start = validation
        .find("pub(super) fn retained_route_matches_snapshot_bounded")
        .expect("bounded retained matcher start");
    let end = validation[start..]
        .find("pub(super) fn prefix_and_local_shape_match")
        .map(|offset| start + offset)
        .expect("bounded retained matcher end");
    assert_no_route_replay(&validation[start..end]);
    assert_no_route_replay(include_str!("release/runtime_validation.rs"));
}

pub(in crate::ideal_loads::calc) fn cp431_all_snapshots_for_successor_tests() -> Vec<super::PurchasedAirCalcHeatingModeGuardSnapshot> {
    let mut snapshots = Vec::new();
    for predecessor in cp430_all_snapshots() {
        let active = active_inputs();
        let inactive = [None];
        let inputs: &[Option<ActiveInput>] = if predecessor.heating_or_no_load_case_entered {
            &active
        } else {
            &inactive
        };
        for input in inputs.iter().copied() {
            snapshots.push(advance(&mut State::new(predecessor.system), predecessor, input).expect("CP431"));
        }
    }
    snapshots
}

fn cp430_all_snapshots() -> Vec<Predecessor> {
    cp429_all_snapshots_for_successor_tests()
        .into_iter()
        .map(|predecessor| advance_cp430(&mut crate::ideal_loads::PurchasedAirCalcHeatingOrNoLoadCaseEntryRuntimeState::new(predecessor.system), predecessor).expect("CP430"))
        .collect()
}

fn active_predecessor() -> Predecessor {
    cp430_all_snapshots()
        .into_iter()
        .find(|snapshot| snapshot.heating_or_no_load_case_entered)
        .expect("active CP430")
}

fn active_inputs() -> [Option<ActiveInput>; 3] {
    [
        active_input(1.0, 1.0, None),
        active_input(0.0, 1.0, Some(Control::SingleCool)),
        active_input(0.0, 1.0, Some(Control::DualHeatCool)),
    ]
}

fn active_input(
    minimum: f64,
    heating: f64,
    temperature_control_type: Option<Control>,
) -> Option<ActiveInput> {
    Some(ActiveInput {
        numeric: Numeric {
            minimum_outdoor_air_sensible_output_w: minimum,
            heating_setpoint_demand_w: heating,
        },
        temperature_control_type,
    })
}

fn is_public_logical_index(index: usize) -> bool {
    matches!(index, 0..=8 | 20 | 21 | 26 | 27)
}

fn assert_bits(left: Option<f64>, right: Option<f64>) {
    assert_eq!(left.map(f64::to_bits), right.map(f64::to_bits));
}

fn nonzero_indices(values: &[usize; 36]) -> Vec<usize> {
    values
        .iter()
        .enumerate()
        .filter_map(|(index, count)| (*count != 0).then_some(index))
        .collect()
}

fn assert_no_route_replay(source: &str) {
    for forbidden in ["predecessor_route(", "_snapshot_route("] {
        assert!(!source.contains(forbidden), "{forbidden}");
    }
}
