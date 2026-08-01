//! CP384 raw binary64 assignment and preservation behavior.

use super::{predecessor_for_route, with_maximum, with_output};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentRuntimeState as State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_state as advance,
};

#[test]
fn cp384_assignment_copies_retained_maximum_bits_without_coercion() {
    for maximum in [-0.0, -1.0, f64::NEG_INFINITY, f64::from_bits(1)] {
        let predecessor = with_maximum(predecessor_for_route(5, 1, true, 1), maximum);
        assert!(predecessor.dehumidification_total_output_capacity_adjustment_body_entered);
        let mut state = State::new(predecessor.system);
        let snapshot = advance(&mut state, predecessor).expect("private IEEE assignment");
        let expected = maximum.to_bits();
        assert!(snapshot.cp383_retained_maximum_total_cooling_capacity_owned_read);
        assert_eq!(
            snapshot.maximum_total_cooling_capacity_w.map(f64::to_bits),
            Some(expected),
        );
        assert_eq!(
            snapshot.assigned_cooling_total_output_w.map(f64::to_bits),
            Some(expected),
        );
        assert_eq!(
            snapshot.resulting_cooling_total_output_w.map(f64::to_bits),
            Some(expected),
        );
    }
}

#[test]
fn cp384_guard_false_preserves_preexisting_nan_and_signed_zero_bits() {
    for output in [
        f64::from_bits(0x7ff8_0000_0000_0042),
        -0.0,
        f64::NEG_INFINITY,
    ] {
        let predecessor = with_output(predecessor_for_route(6, 1, false, 1), output);
        assert!(predecessor.dehumidification_total_output_capacity_guard_false_fallthrough);
        let mut state = State::new(predecessor.system);
        let snapshot = advance(&mut state, predecessor).expect("guard-false preservation");
        assert!(snapshot.dehumidification_total_output_capacity_guard_false_fallthrough);
        assert!(!snapshot.maximum_total_cooling_capacity_read);
        assert!(snapshot.maximum_total_cooling_capacity_w.is_none());
        assert!(!snapshot.cooling_total_output_assigned);
        assert_eq!(
            snapshot.preexisting_cooling_total_output_w.map(f64::to_bits),
            Some(output.to_bits()),
        );
        assert_eq!(
            snapshot.resulting_cooling_total_output_w.map(f64::to_bits),
            Some(output.to_bits()),
        );
    }
}

#[test]
fn cp384_requires_an_exact_cp383_route_and_owned_body_operand() {
    let predecessor = predecessor_for_route(3, 1, true, 1);
    for corruption in 0..3 {
        let mut forged = predecessor;
        match corruption {
            0 => forged.maximum_total_cooling_capacity_w = None,
            1 => forged.cp321_maximum_total_cooling_capacity_owned_read = false,
            2 => forged.maximum_total_cooling_capacity_read = false,
            _ => unreachable!(),
        }
        let mut state = State::new(forged.system);
        let before = state.clone();
        assert!(advance(&mut state, forged).is_none());
        assert_eq!(state, before);
    }
}
