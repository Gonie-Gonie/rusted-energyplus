//! CP381 raw binary64 comparison characterization.

use super::{active_input, predecessor_for_route};
use crate::ideal_loads::private_cooling_post_saturation_capacity_limit_dehumidification_guard_characterization as characterize;

#[test]
fn cp381_private_characterization_uses_raw_rust_less_than_for_ieee_edges() {
    let nan_a = f64::from_bits(0x7ff8_0000_0000_0042);
    let nan_b = f64::from_bits(0xfff8_0000_0000_0043);
    for (supply, mixed, expected) in [
        (nan_a, 0.009, false),
        (0.007, nan_b, false),
        (-0.0, 0.0, false),
        (0.0, -0.0, false),
        (f64::NEG_INFINITY, 0.009, true),
        (0.007, f64::INFINITY, true),
        (f64::INFINITY, 0.009, false),
        (0.007, f64::NEG_INFINITY, false),
    ] {
        let predecessor = predecessor_for_route(4, true, 1);
        let snapshot = characterize(predecessor, active_input(supply, mixed))
            .expect("private raw-comparison characterization");
        assert_eq!(
            snapshot.supply_humidity_ratio.map(f64::to_bits),
            Some(supply.to_bits()),
        );
        assert_eq!(
            snapshot.mixed_air_humidity_ratio.map(f64::to_bits),
            Some(mixed.to_bits()),
        );
        assert_eq!(
            snapshot.supply_humidity_ratio_strictly_less_than_mixed_air_humidity_ratio,
            Some(expected),
        );
        assert_eq!(snapshot.dehumidification_body_entered, expected);
        assert_eq!(snapshot.dehumidification_guard_false_fallthrough, !expected);
    }
}

#[test]
fn cp381_bit_exact_snapshot_matching_distinguishes_signed_zero_and_nan_payloads() {
    use super::super::cooling_post_saturation_capacity_limit_dehumidification_guard_snapshots_match_bit_exact as matches;

    let predecessor = predecessor_for_route(4, true, 1);
    let positive_zero = characterize(predecessor, active_input(0.0, 0.0)).expect("+0 snapshot");
    let negative_zero = characterize(predecessor, active_input(-0.0, 0.0)).expect("-0 snapshot");
    assert!(!matches(positive_zero, negative_zero));

    let nan_a = f64::from_bits(0x7ff8_0000_0000_0042);
    let nan_b = f64::from_bits(0x7ff8_0000_0000_0043);
    let first = characterize(predecessor, active_input(nan_a, 0.0)).expect("NaN A snapshot");
    let second = characterize(predecessor, active_input(nan_b, 0.0)).expect("NaN B snapshot");
    assert!(!matches(first, second));
    assert!(matches(first, first));
}
