//! CP383 raw binary64 strict-greater behavior.

use super::{active_input, predecessor_for_route, predecessor_with_output};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_guard::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardRuntimeState as State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_state as advance,
};

#[test]
fn cp383_preserves_equality_signed_zero_and_unordered_nan_as_false() {
    for (mixed, capacity) in [
        (100.0, 100.0),
        (0.0, -0.0),
        (-0.0, 0.0),
        (f64::from_bits(0x7ff8_0000_0000_0042), 1.0),
        (1.0, f64::NAN),
    ] {
        let predecessor = predecessor_with_output(predecessor_for_route(3, 1, 1), mixed);
        let mut state = State::new(predecessor.system);
        let snapshot = advance(&mut state, predecessor, active_input(predecessor, capacity))
            .expect("raw IEEE case");
        assert_eq!(
            snapshot.cooling_total_output_strictly_greater_than_maximum_total_cooling_capacity,
            Some(false),
        );
        assert!(!snapshot.dehumidification_total_output_capacity_adjustment_body_entered);
        assert!(snapshot.dehumidification_total_output_capacity_guard_false_fallthrough);
    }
}

#[test]
fn cp383_preserves_infinity_ordering_without_clamp_or_finite_result_gate() {
    for (mixed, capacity, expected) in [
        (f64::INFINITY, 1.0, true),
        (f64::INFINITY, f64::INFINITY, false),
        (1.0, f64::NEG_INFINITY, true),
        (f64::NEG_INFINITY, 1.0, false),
    ] {
        let predecessor = predecessor_with_output(predecessor_for_route(3, 1, 1), mixed);
        let mut state = State::new(predecessor.system);
        let snapshot = advance(&mut state, predecessor, active_input(predecessor, capacity))
            .expect("infinity case");
        assert_eq!(
            snapshot.cooling_total_output_strictly_greater_than_maximum_total_cooling_capacity,
            Some(expected),
        );
        assert_eq!(
            snapshot.dehumidification_total_output_capacity_adjustment_body_entered,
            expected,
        );
    }
}

#[test]
fn cp383_requires_cp382_output_bits_and_all_owner_evidence() {
    let predecessor = predecessor_for_route(3, 1, 1);
    for corruption in 0..4 {
        let mut input = active_input(predecessor, 90.0).expect("active input");
        match corruption {
            0 => input.cooling_total_output_w = f64::from_bits(
                input.cooling_total_output_w.to_bits() ^ 1,
            ),
            1 => input.cp382_cooling_total_output_owned_read = false,
            2 => input.cp321_maximum_total_cooling_capacity_owned_read = false,
            3 => input.cp340_same_call_maximum_total_cooling_capacity_bit_corroborated = false,
            _ => unreachable!(),
        }
        let mut state = State::new(predecessor.system);
        let before = state.clone();
        assert!(advance(&mut state, predecessor, Some(input)).is_none());
        assert_eq!(state, before);
    }
}
