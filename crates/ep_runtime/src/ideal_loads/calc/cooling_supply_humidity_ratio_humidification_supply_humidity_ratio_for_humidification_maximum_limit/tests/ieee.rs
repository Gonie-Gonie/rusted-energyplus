//! CP374 source-shaped strict-less-than minimum characterization.

use super::*;

#[test]
fn cp374_minimum_is_right_biased_for_ties_signed_zero_and_unordered_nan() {
    let cases = [
        (-0.0, -0.0, 0.0),
        (0.0, 0.0, -0.0),
        (0.004, 0.0, 0.004),
        (f64::from_bits(0x7ff8_0000_0000_0374), 0.0, 0.008),
        (0.008, 0.0, f64::from_bits(0x7ff8_0000_0000_1374)),
        (
            f64::from_bits(0x7ff8_0000_0000_2374),
            0.0,
            f64::from_bits(0x7ff8_0000_0000_3374),
        ),
        (0.003, 0.0, 0.008),
        (0.009, 0.0, 0.008),
    ];
    for selector in [
        DehumidificationControlType::Humidistat,
        DehumidificationControlType::None,
    ] {
        for (demand, zone_humidity_ratio, right) in cases {
            let predecessor = active_cp373(selector, demand, 1.0, zone_humidity_ratio);
            let left = predecessor
                .resulting_supply_humidity_ratio_for_humidification
                .expect("CP373 left operand");
            let expected = if left < right { left } else { right };
            let mut state = State::new(predecessor.system);
            let snapshot = advance(
                &mut state,
                predecessor,
                Some(ActiveOperands {
                    maximum_heating_supply_air_humidity_ratio: right,
                }),
            )
            .expect("active CP374 minimum");

            assert_eq!(
                snapshot
                    .supply_humidity_ratio_for_humidification_before_maximum_limit
                    .map(f64::to_bits),
                Some(left.to_bits())
            );
            assert_eq!(
                snapshot
                    .maximum_heating_supply_air_humidity_ratio
                    .map(f64::to_bits),
                Some(right.to_bits())
            );
            for result in [
                snapshot.minimum_supply_humidity_ratio_for_humidification,
                snapshot.assigned_supply_humidity_ratio_for_humidification,
                snapshot.resulting_supply_humidity_ratio_for_humidification,
            ] {
                assert_eq!(result.map(f64::to_bits), Some(expected.to_bits()));
            }
            assert_eq!(state.source_site_execution_count, 4);
        }
    }
}
