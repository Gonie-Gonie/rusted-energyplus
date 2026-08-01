//! CP378 source-shaped minimum and raw IEEE characterization tests.

use super::super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentRuntimeState as State,
    advance_cooling_supply_humidity_ratio_saturation_limit_assignment_state as advance,
    cooling_supply_humidity_ratio_saturation_limit_assignment_snapshots_match_bit_exact,
};
use super::{predecessor_for_route, predecessor_for_route_with_psychrometrics};
use crate::ideal_loads::calc::cooling_positive_supply_temperature_mixed_air_limit::source_shaped_two_argument_minimum;

#[test]
fn cp378_uses_the_canonical_right_biased_source_minimum() {
    let left_nan = f64::from_bits(0x7ff8_0000_0000_0378);
    let right_nan = f64::from_bits(0x7ff8_0000_0000_1378);
    for (left, right, expected) in [
        (-0.0, 0.0, 0.0),
        (0.0, -0.0, -0.0),
        (left_nan, 0.25, 0.25),
        (0.25, right_nan, right_nan),
        (left_nan, right_nan, right_nan),
        (f64::NEG_INFINITY, 0.25, f64::NEG_INFINITY),
        (f64::INFINITY, 0.25, 0.25),
    ] {
        assert_eq!(
            source_shaped_two_argument_minimum(left, right).to_bits(),
            expected.to_bits(),
        );
    }
}

#[test]
fn cp378_pure_transition_preserves_raw_operand_and_result_bits() {
    let left_nan = f64::from_bits(0x7ff8_0000_0000_0378);
    let predecessor = predecessor_for_route(4, left_nan);
    let right = predecessor
        .resulting_saturation_supply_humidity_ratio
        .expect("finite saturation");
    let mut state = State::new(predecessor.system);
    let snapshot = advance(&mut state, predecessor).expect("raw left NaN");
    assert_eq!(
        snapshot
            .original_supply_humidity_ratio_before_saturation_limit
            .map(f64::to_bits),
        Some(left_nan.to_bits()),
    );
    assert_eq!(
        snapshot.resulting_supply_humidity_ratio.map(f64::to_bits),
        Some(right.to_bits()),
    );

    let predecessor = predecessor_for_route_with_psychrometrics(4, 0.25, 18.0, f64::NAN);
    let right_nan = predecessor
        .resulting_saturation_supply_humidity_ratio
        .expect("raw saturation NaN");
    assert!(right_nan.is_nan());
    let mut state = State::new(predecessor.system);
    let snapshot = advance(&mut state, predecessor).expect("raw right NaN");
    assert_eq!(
        snapshot.resulting_supply_humidity_ratio.map(f64::to_bits),
        Some(right_nan.to_bits()),
    );
}

#[test]
fn cp378_bit_matcher_rejects_nan_payload_and_one_bit_result_drift() {
    let predecessor = predecessor_for_route(4, f64::from_bits(0x7ff8_0000_0000_0378));
    let mut state = State::new(predecessor.system);
    let exact = advance(&mut state, predecessor).expect("exact snapshot");

    let mut drifted = exact;
    drifted.original_supply_humidity_ratio_before_saturation_limit = drifted
        .original_supply_humidity_ratio_before_saturation_limit
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    assert!(
        !cooling_supply_humidity_ratio_saturation_limit_assignment_snapshots_match_bit_exact(
            exact, drifted,
        )
    );

    let mut drifted = exact;
    drifted.resulting_supply_humidity_ratio = drifted
        .resulting_supply_humidity_ratio
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    assert!(
        !cooling_supply_humidity_ratio_saturation_limit_assignment_snapshots_match_bit_exact(
            exact, drifted,
        )
    );
}
