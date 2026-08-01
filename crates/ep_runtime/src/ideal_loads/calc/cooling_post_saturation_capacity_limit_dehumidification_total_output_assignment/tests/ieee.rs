//! CP382 raw binary64 subtraction/product characterization.

use super::{active_input, predecessor_for_route};
use crate::ideal_loads::private_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_characterization as characterize;

#[test]
fn cp382_private_characterization_preserves_grouped_raw_binary64_arithmetic() {
    let nan_a = f64::from_bits(0x7ff8_0000_0000_0042);
    let nan_b = f64::from_bits(0xfff8_0000_0000_0043);
    for (flow, mixed, supply) in [
        (1.5, 48_000.0, 40_000.0),
        (f64::INFINITY, 48_000.0, 40_000.0),
        (f64::MAX, f64::MAX, -f64::MAX),
        (-0.0, 1.0, 1.0),
        (0.0, -0.0, 0.0),
        (1.0, nan_a, 0.0),
        (1.0, 0.0, nan_b),
    ] {
        let predecessor = predecessor_for_route(4, 1, 1);
        let snapshot = characterize(predecessor, active_input(flow, mixed, supply))
            .expect("private raw-arithmetic characterization");
        let expected_difference = mixed - supply;
        let expected_output = flow * expected_difference;

        assert_eq!(
            snapshot.supply_mass_flow_rate_kg_per_s.map(f64::to_bits),
            Some(flow.to_bits()),
        );
        assert_eq!(
            snapshot
                .mixed_air_minus_supply_enthalpy_j_per_kg
                .map(f64::to_bits),
            Some(expected_difference.to_bits()),
        );
        assert_eq!(
            snapshot.calculated_cooling_total_output_w.map(f64::to_bits),
            Some(expected_output.to_bits()),
        );
        assert_eq!(
            snapshot.cooling_total_output_w.map(f64::to_bits),
            Some(expected_output.to_bits()),
        );
    }
}

#[test]
fn cp382_bit_exact_snapshot_matching_distinguishes_signed_zero_and_nan_payloads() {
    use super::super::cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_snapshots_match_bit_exact as matches;

    let predecessor = predecessor_for_route(4, 1, 1);
    let positive_zero =
        characterize(predecessor, active_input(0.0, 1.0, 1.0)).expect("positive-zero snapshot");
    let negative_zero =
        characterize(predecessor, active_input(-0.0, 1.0, 1.0)).expect("negative-zero snapshot");
    assert!(!matches(positive_zero, negative_zero));

    let nan_a = f64::from_bits(0x7ff8_0000_0000_0042);
    let nan_b = f64::from_bits(0x7ff8_0000_0000_0043);
    let first = characterize(predecessor, active_input(1.0, nan_a, 0.0)).expect("NaN A snapshot");
    let second = characterize(predecessor, active_input(1.0, nan_b, 0.0)).expect("NaN B snapshot");
    assert!(!matches(first, second));
    assert!(matches(first, first));
}
