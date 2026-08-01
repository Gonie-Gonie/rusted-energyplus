//! CP385 exact-release and corruption validation.

use super::{predecessor_for_route, retained_input};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_state as advance,
    cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_snapshot_is_exact_direct_release,
};

#[test]
fn cp385_release_validator_rejects_each_arithmetic_or_provenance_corruption() {
    let predecessor = predecessor_for_route(3, 1, true, 1);
    let input = retained_input(predecessor, 40_000.0, 50_000.0, 2.0);
    let mut state = State::new(predecessor.system);
    let valid = advance(&mut state, predecessor, input).expect("assignment");
    assert!(cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_snapshot_is_exact_direct_release(valid));
    for index in 0..8 {
        let mut forged = valid;
        match index {
            0 => forged.cp379_retained_supply_enthalpy_owned_read = false,
            1 => forged.cp329_retained_mixed_air_enthalpy_owned_read = false,
            2 => forged.cp384_retained_cooling_total_output_owned_read = false,
            3 => forged.cp330_retained_supply_mass_flow_rate_owned_read = false,
            4 => forged.specific_cooling_output_j_per_kg = Some(123.0),
            5 => forged.calculated_supply_enthalpy_j_per_kg = Some(123.0),
            6 => forged.assigned_supply_enthalpy_j_per_kg = Some(123.0),
            7 => forged.resulting_supply_enthalpy_j_per_kg = Some(123.0),
            _ => unreachable!(),
        }
        assert!(!cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_snapshot_is_exact_direct_release(forged));
    }
}

#[test]
fn cp385_rejects_retained_input_shape_corruption_without_mutation() {
    for (outcome, assignment, input_present) in
        [(0, false, true), (1, false, false), (1, true, false)]
    {
        let predecessor = predecessor_for_route(3, outcome, assignment, 1);
        let mut state = State::new(predecessor.system);
        let input = input_present
            .then(|| retained_input(predecessor_for_route(3, 1, false, 1), 1.0, 2.0, 3.0).unwrap());
        let before = state.clone();
        assert!(advance(&mut state, predecessor, input).is_none());
        assert_eq!(state, before);
    }
}

#[test]
fn cp385_release_validator_rejects_inherited_control_flow_flag_drift() {
    for (route, valid) in [
        ("outer skip", snapshot_for_route(0, 0, false)),
        ("capacity guard false", snapshot_for_route(3, 0, false)),
        (
            "dehumidification guard false",
            snapshot_for_route(3, 2, false),
        ),
        (
            "CP384 capacity guard false fallthrough",
            snapshot_for_route(3, 1, false),
        ),
        (
            "CP384 maximum capacity assignment",
            snapshot_for_route(3, 1, true),
        ),
    ] {
        assert_exact_snapshot(valid);
        for flag_index in 0..10 {
            let mut forged = valid;
            let flag = invert_inherited_control_flow_flag(&mut forged, flag_index);
            assert!(
                !cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_snapshot_is_exact_direct_release(forged),
                "accepted {flag} drift for {route}",
            );
        }
    }
}

fn snapshot_for_route(inherited_route: usize, outcome: usize, assignment: bool) -> Snapshot {
    let predecessor = predecessor_for_route(inherited_route, outcome, assignment, 1);
    let input = retained_input(predecessor, 40_000.0, 50_000.0, 2.0);
    let mut state = State::new(predecessor.system);
    advance(&mut state, predecessor, input).expect("valid CP385 route")
}

fn assert_exact_snapshot(snapshot: Snapshot) {
    assert!(cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_snapshot_is_exact_direct_release(snapshot));
}

fn invert_inherited_control_flow_flag(snapshot: &mut Snapshot, index: usize) -> &'static str {
    match index {
        0 => {
            snapshot.predecessor_capacity_limit_guard_evaluated =
                !snapshot.predecessor_capacity_limit_guard_evaluated;
            "predecessor_capacity_limit_guard_evaluated"
        }
        1 => {
            snapshot.predecessor_capacity_limit_body_entered =
                !snapshot.predecessor_capacity_limit_body_entered;
            "predecessor_capacity_limit_body_entered"
        }
        2 => {
            snapshot.predecessor_active_capacity_limit_guard_false_fallthrough =
                !snapshot.predecessor_active_capacity_limit_guard_false_fallthrough;
            "predecessor_active_capacity_limit_guard_false_fallthrough"
        }
        3 => {
            snapshot.predecessor_dehumidification_guard_evaluated =
                !snapshot.predecessor_dehumidification_guard_evaluated;
            "predecessor_dehumidification_guard_evaluated"
        }
        4 => {
            snapshot.predecessor_dehumidification_body_entered =
                !snapshot.predecessor_dehumidification_body_entered;
            "predecessor_dehumidification_body_entered"
        }
        5 => {
            snapshot.predecessor_dehumidification_guard_false_fallthrough =
                !snapshot.predecessor_dehumidification_guard_false_fallthrough;
            "predecessor_dehumidification_guard_false_fallthrough"
        }
        6 => {
            snapshot.predecessor_dehumidification_total_output_assignment_executed =
                !snapshot.predecessor_dehumidification_total_output_assignment_executed;
            "predecessor_dehumidification_total_output_assignment_executed"
        }
        7 => {
            snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated =
                !snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated;
            "predecessor_dehumidification_total_output_capacity_guard_evaluated"
        }
        8 => {
            snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered =
                !snapshot
                    .predecessor_dehumidification_total_output_capacity_adjustment_body_entered;
            "predecessor_dehumidification_total_output_capacity_adjustment_body_entered"
        }
        9 => {
            snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough =
                !snapshot
                    .predecessor_dehumidification_total_output_capacity_guard_false_fallthrough;
            "predecessor_dehumidification_total_output_capacity_guard_false_fallthrough"
        }
        _ => unreachable!(),
    }
}
