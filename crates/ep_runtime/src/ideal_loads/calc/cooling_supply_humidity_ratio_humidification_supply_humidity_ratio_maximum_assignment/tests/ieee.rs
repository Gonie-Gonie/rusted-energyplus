//! CP375 source-shaped strict-less-than maximum characterization.

use super::*;

#[test]
fn cp375_maximum_is_left_biased_for_ties_signed_zero_and_unordered_nan() {
    let cases = [
        (-0.0, -0.0),
        (0.0, 0.0),
        (-0.0, 0.0),
        (0.0, -0.0),
        (f64::from_bits(0x7ff8_0000_0000_0375), 0.008),
        (0.008, f64::from_bits(0x7ff8_0000_0000_1375)),
        (
            f64::from_bits(0x7ff8_0000_0000_2375),
            f64::from_bits(0x7ff8_0000_0000_3375),
        ),
        (0.003, 0.008),
        (0.009, 0.008),
    ];
    for selector in [
        DehumidificationControlType::Humidistat,
        DehumidificationControlType::None,
    ] {
        for (left, desired_right) in cases {
            let predecessor = active_cp374(selector, 0.5, desired_right);
            let right = predecessor
                .resulting_supply_humidity_ratio_for_humidification
                .expect("CP374 right operand");
            let expected = if left < right { right } else { left };
            let mut state = State::new(predecessor.system);
            let snapshot = advance(
                &mut state,
                predecessor,
                Some(ActiveOperands {
                    purchased_air_supply_humidity_ratio: left,
                }),
            )
            .expect("active CP375 maximum");

            assert_eq!(
                snapshot
                    .purchased_air_supply_humidity_ratio_before_humidification_supply_maximum
                    .map(f64::to_bits),
                Some(left.to_bits())
            );
            assert_eq!(
                snapshot
                    .supply_humidity_ratio_for_humidification_for_supply_maximum
                    .map(f64::to_bits),
                Some(right.to_bits())
            );
            for result in [
                snapshot.maximum_supply_humidity_ratio,
                snapshot.assigned_supply_humidity_ratio,
                snapshot.resulting_supply_humidity_ratio,
            ] {
                assert_eq!(result.map(f64::to_bits), Some(expected.to_bits()));
            }
            assert_eq!(state.source_site_execution_count, 4);
        }
    }
}
