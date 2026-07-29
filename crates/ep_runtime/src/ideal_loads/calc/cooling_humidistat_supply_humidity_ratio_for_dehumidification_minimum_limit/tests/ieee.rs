use super::{H, operands, predecessor};
use crate::ideal_loads::calc::cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit::{
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitRuntimeState as State,
    advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_state as advance,
};
use ep_model::IdealLoadsAirSystemId;

#[test]
fn source_shaped_maximum_is_left_biased_for_ties_zeroes_and_nan() {
    let left_nan = f64::from_bits(0x7ff8_0000_0000_0042);
    let right_nan = f64::from_bits(0x7ff8_0000_0000_0043);
    for (left, right, expected) in [
        (0.006, 0.0077, 0.0077),
        (0.008, 0.0077, 0.008),
        (0.0077, 0.0077, 0.0077),
        (-0.0, 0.0, -0.0),
        (0.0, -0.0, 0.0),
        (left_nan, 2.0, left_nan),
        (1.0, right_nan, 1.0),
        (left_nan, right_nan, left_nan),
        (f64::NEG_INFINITY, f64::INFINITY, f64::INFINITY),
    ] {
        let predecessor = predecessor(H, 1, left);
        assert_eq!(
            predecessor
                .resulting_supply_humidity_ratio_for_dehumidification
                .expect("CP360 left")
                .to_bits(),
            left.to_bits()
        );
        let snapshot = advance(
            &mut State::new(IdealLoadsAirSystemId(7)),
            predecessor,
            operands(H, right),
        )
        .expect("private-H CP361");
        for actual in [
            snapshot.maximum_supply_humidity_ratio_for_dehumidification,
            snapshot.assigned_supply_humidity_ratio_for_dehumidification,
            snapshot.resulting_supply_humidity_ratio_for_dehumidification,
        ] {
            assert_eq!(
                actual.expect("maximum evidence").to_bits(),
                expected.to_bits()
            );
        }
    }
}

#[test]
fn left_nan_payload_mismatch_is_rejected_without_mutation() {
    let left = f64::from_bits(0x7ff8_0000_0000_0042);
    let mut predecessor = predecessor(H, 1, left);
    predecessor.resulting_supply_humidity_ratio_for_dehumidification =
        Some(f64::from_bits(0x7ff8_0000_0000_0043));
    let mut state = State::new(IdealLoadsAirSystemId(7));
    let before = state.clone();
    assert!(advance(&mut state, predecessor, operands(H, 0.0077)).is_none());
    assert_eq!(state, before);
}
