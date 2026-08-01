//! CP390 source-shaped binary64 minimum tests.

use super::*;
use crate::ideal_loads::calc::cooling_positive_supply_temperature_mixed_air_limit::source_shaped_two_argument_minimum;

#[test]
fn source_shaped_minimum_is_right_biased_for_ties_and_unordered_inputs() {
    let nan_a = f64::from_bits(0x7ff8_0000_0000_0042);
    let nan_b = f64::from_bits(0x7ff8_0000_0000_0099);
    for (left, right, expected) in [
        (1.0, 2.0, 1.0),
        (2.0, 1.0, 1.0),
        (3.0, 3.0, 3.0),
        (-0.0, 0.0, 0.0),
        (0.0, -0.0, -0.0),
        (f64::NEG_INFINITY, 1.0, f64::NEG_INFINITY),
        (f64::INFINITY, 1.0, 1.0),
        (nan_a, 7.0, 7.0),
        (7.0, nan_b, nan_b),
        (nan_a, nan_b, nan_b),
    ] {
        assert_eq!(
            source_shaped_two_argument_minimum(left, right).to_bits(),
            expected.to_bits(),
        );
    }
}

#[test]
fn exact_nonfinite_cp389_left_is_not_coerced_or_rejected() {
    for ratio in [
        f64::from_bits(0x7ff8_0000_0000_0042),
        f64::INFINITY,
        f64::NEG_INFINITY,
    ] {
        let chain = fixtures::chain(
            3,
            1,
            true,
            Some(D::ConstantSensibleHeatRatio),
            1,
            ratio,
            18.0,
            1.0,
        );
        let left = chain.cp389.resulting_supply_temperature_c.expect("left");
        assert!(!left.is_finite());
        let right = chain
            .mixed_air_owner
            .mixed_air_temperature_c
            .expect("right");
        assert!(right.is_finite());
        let snapshot = private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_characterization(
            chain.cp389,
            Some(chain.mixed_air_owner),
        )
        .expect("CP390 accepts exact nonfinite left");
        let expected = source_shaped_two_argument_minimum(left, right);
        assert_eq!(
            snapshot.resulting_supply_temperature_c.map(f64::to_bits),
            Some(expected.to_bits()),
        );
        assert!(cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_snapshot_is_exact(snapshot));
    }
}

#[test]
fn exact_tie_uses_the_cp329_right_operand_bits() {
    let chain = fixtures::chain(
        3,
        1,
        true,
        Some(D::ConstantSensibleHeatRatio),
        1,
        -0.0,
        18.0,
        1.0,
    );
    let left = chain.cp389.resulting_supply_temperature_c.expect("left");
    let right = chain
        .mixed_air_owner
        .mixed_air_temperature_c
        .expect("right");
    assert_eq!(left.to_bits(), right.to_bits());
    let snapshot = private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_characterization(
        chain.cp389,
        Some(chain.mixed_air_owner),
    )
    .expect("CP390 tie");
    assert_eq!(
        snapshot.resulting_supply_temperature_c.map(f64::to_bits),
        Some(right.to_bits()),
    );
}
