use std::collections::BTreeSet;

use super::super::transition::RetainedRoute;
use super::*;
use crate::ideal_loads::calc::advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_state_with_validated_route as advance_with_validated_route;

#[test]
fn snapshot_schema_is_exact_202_fields_71_optional_and_cp419_base_prefixed() {
    let cp419_source = include_str!(
        "../../cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment.rs"
    );
    let cp420_source = include_str!(
        "../../cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment.rs"
    );
    let cp419 = snapshot_fields(cp419_source, "CpAirAssignmentSnapshot");
    let cp420 = snapshot_fields(cp420_source, "SensibleOutputAssignmentSnapshot");

    assert_eq!(cp419.len(), 174);
    assert_eq!(cp420.len(), 202);
    assert_eq!(&cp420[..171], &cp419[..171]);
    assert_eq!(cp420.iter().collect::<BTreeSet<_>>().len(), 202);
    assert_eq!(
        snapshot_block(cp420_source, "SensibleOutputAssignmentSnapshot")
            .matches("Option<f64>")
            .count(),
        71
    );
    assert_eq!(&cp420[171..], CP420_TAIL_FIELDS);
}

#[test]
fn cold_and_validated_route_advances_match_bit_exact_for_active_and_inactive() {
    let predecessors = cp419_all_snapshots_for_successor_tests();
    for predecessor in [select(&predecessors, true), select(&predecessors, false)] {
        let route = predecessor_route(predecessor).expect("validated CP419 route");
        let input = route.active.then_some(active_input(0.25, 17.0));
        let mut cold_state = State::new(predecessor.system);
        let mut validated_state = State::new(predecessor.system);
        let cold = advance(&mut cold_state, predecessor, input).expect("cold CP420");
        let validated =
            advance_with_validated_route(&mut validated_state, predecessor, route, input)
                .expect("validated CP420");
        assert!(super::super::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_snapshots_match_bit_exact(cold, validated));
        assert_eq!(cold_state, validated_state);
    }
}

#[test]
fn validated_route_rejects_every_forged_component_transactionally() {
    let predecessors = cp419_all_snapshots_for_successor_tests();
    let predecessor = select(&predecessors, true);
    let route = predecessor_route(predecessor).expect("active route");
    let mut forgeries = Vec::new();
    let mut forged = route;
    forged.logical_index = (route.logical_index + 1) % 36;
    forgeries.push(forged);
    let mutations: [fn(&mut RetainedRoute); 7] = [
        |route: &mut _| {
            route.predecessor_guard_false_fallthrough = !route.predecessor_guard_false_fallthrough
        },
        |route: &mut _| {
            route.predecessor_guard_body_entered = !route.predecessor_guard_body_entered
        },
        |route: &mut _| {
            route.predecessor_saturation_temperature_assignment_executed =
                !route.predecessor_saturation_temperature_assignment_executed
        },
        |route: &mut _| {
            route.predecessor_saturation_temperature_mixed_air_limit_executed =
                !route.predecessor_saturation_temperature_mixed_air_limit_executed
        },
        |route: &mut _| {
            route.predecessor_supply_humidity_ratio_assignment_executed =
                !route.predecessor_supply_humidity_ratio_assignment_executed
        },
        |route: &mut _| {
            route.predecessor_supply_enthalpy_assignment_executed =
                !route.predecessor_supply_enthalpy_assignment_executed
        },
        |route: &mut _| route.active = !route.active,
    ];
    for mutate in mutations {
        let mut forged = route;
        mutate(&mut forged);
        forgeries.push(forged);
    }
    for forged in forgeries {
        let mut state = State::new(predecessor.system);
        let before = state.clone();
        assert!(
            advance_with_validated_route(
                &mut state,
                predecessor,
                forged,
                Some(active_input(0.25, 17.0)),
            )
            .is_none()
        );
        assert_eq!(state, before);
    }
}

#[test]
fn active_and_inactive_paths_preserve_raw_ieee_grouping_without_finite_gate() {
    let predecessors = cp419_all_snapshots_for_successor_tests();
    let active = select(&predecessors, true);
    let supply = active.resulting_supply_temperature_c.expect("supply T");
    for input in [
        active_input(f64::INFINITY, supply),
        active_input(0.0, f64::INFINITY),
        active_input(f64::from_bits(0x7ff8_0000_0000_0420), f64::NEG_INFINITY),
        active_input(f64::NEG_INFINITY, 17.0),
    ] {
        let snapshot =
            advance(&mut State::new(active.system), active, Some(input)).expect("raw IEEE CP420");
        assert_formula_bits(snapshot);
    }

    let inactive = select(&predecessors, false);
    let snapshot =
        advance(&mut State::new(inactive.system), inactive, None).expect("inactive CP420");
    assert!(!snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_executed);
    assert!(snapshot.supply_mass_flow_rate_kg_per_s.is_none());
    assert!(snapshot.cooling_sensible_output_w.is_none());

    let mut state = State::new(inactive.system);
    let before = state.clone();
    assert!(advance(&mut state, inactive, Some(active_input(0.25, 17.0))).is_none());
    assert_eq!(state, before);
}

fn select(
    predecessors: &[crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentSnapshot],
    active: bool,
) -> crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentSnapshot{
    predecessors.iter().copied().find(|snapshot| {
        snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_executed == active
    }).expect("requested CP419 route")
}

fn active_input(flow: f64, mixed: f64) -> ActiveInput {
    ActiveInput {
        supply_mass_flow_rate_kg_per_s: flow,
        mixed_air_temperature_c: mixed,
    }
}

fn snapshot_fields<'a>(source: &'a str, suffix: &str) -> Vec<&'a str> {
    snapshot_block(source, suffix)
        .lines()
        .filter_map(|line| line.trim().strip_prefix("pub "))
        .filter_map(|line| line.split_once(':').map(|(name, _)| name))
        .collect()
}

fn snapshot_block<'a>(source: &'a str, suffix: &str) -> &'a str {
    source
        .split_once(suffix)
        .expect("snapshot declaration")
        .1
        .split_once("/// Final selected-unit")
        .expect("snapshot terminator")
        .0
}

const CP420_TAIL_FIELDS: &[&str] = &[
    "predecessor_cp419_resulting_supply_humidity_ratio",
    "predecessor_cp419_resulting_supply_enthalpy_j_per_kg",
    "predecessor_cp419_resulting_supply_temperature_c",
    "post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_executed",
    "cp419_retained_supply_humidity_ratio_state_owned",
    "cp419_retained_supply_enthalpy_state_owned",
    "cp419_retained_supply_temperature_state_owned",
    "cp330_retained_supply_mass_flow_rate_owned_read",
    "cp329_supply_mass_flow_rate_bit_corroborated",
    "supply_mass_flow_rate_read",
    "supply_mass_flow_rate_kg_per_s",
    "cp419_retained_cp_air_owned_read",
    "cp_air_read",
    "cp419_cp_air_for_sensible_output_j_per_kg_k",
    "supply_mass_flow_rate_times_cp_air_calculated",
    "supply_mass_flow_rate_times_cp_air_w_per_k",
    "cp329_retained_mixed_air_temperature_for_sensible_output_owned_read",
    "mixed_air_temperature_read",
    "mixed_air_temperature_for_sensible_output_c",
    "cp419_retained_supply_temperature_owned_read",
    "supply_temperature_read",
    "supply_temperature_for_sensible_output_c",
    "mixed_air_minus_supply_temperature_calculated",
    "mixed_air_minus_supply_temperature_k",
    "cooling_sensible_output_calculated",
    "calculated_cooling_sensible_output_w",
    "cooling_sensible_output_assigned",
    "cooling_sensible_output_w",
    "resulting_supply_humidity_ratio",
    "resulting_supply_enthalpy_j_per_kg",
    "resulting_supply_temperature_c",
];
