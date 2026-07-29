use super::{H, State, advance, operands, predecessor};

#[test]
fn transition_preserves_right_biased_ieee_bits_without_numeric_gate() {
    let left_nan = f64::from_bits(0x7ff8_0000_0000_0011);
    let right_nan = f64::from_bits(0x7ff8_0000_0000_0042);
    for (mixed, local, expected_bits) in [
        (-0.0, 0.0, 0.0f64.to_bits()),
        (0.0, -0.0, (-0.0f64).to_bits()),
        (left_nan, 0.0077, 0.0077f64.to_bits()),
        (0.006, right_nan, right_nan.to_bits()),
        (left_nan, right_nan, right_nan.to_bits()),
        (
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::NEG_INFINITY.to_bits(),
        ),
        (
            f64::NEG_INFINITY,
            f64::INFINITY,
            f64::NEG_INFINITY.to_bits(),
        ),
    ] {
        let retained = predecessor(H, 1, local);
        let mut state = State::new(retained.system);
        let snapshot = advance(&mut state, retained, operands(H, mixed))
            .expect("CP362 must retain IEEE evidence without a numeric gate");

        assert_eq!(
            snapshot.mixed_air_humidity_ratio.map(f64::to_bits),
            Some(mixed.to_bits())
        );
        assert_eq!(
            snapshot
                .supply_humidity_ratio_for_dehumidification_before_mixed_air_limit
                .map(f64::to_bits),
            Some(local.to_bits())
        );
        for value in [
            snapshot.minimum_supply_humidity_ratio,
            snapshot.assigned_supply_humidity_ratio,
            snapshot.resulting_supply_humidity_ratio,
        ] {
            assert_eq!(value.map(f64::to_bits), Some(expected_bits));
        }
        assert_eq!(state.transition_count, 1);
        assert_eq!(state.source_site_execution_count, 4);
        assert_eq!(
            (
                state.mixed_air_humidity_ratio_for_minimum_read_count,
                state
                    .supply_humidity_ratio_for_dehumidification_for_mixed_air_limit_minimum_read_count,
                state.source_shaped_two_argument_minimum_evaluation_count,
                state.supply_humidity_ratio_assignment_count,
            ),
            (1, 1, 1, 1)
        );
    }
}
