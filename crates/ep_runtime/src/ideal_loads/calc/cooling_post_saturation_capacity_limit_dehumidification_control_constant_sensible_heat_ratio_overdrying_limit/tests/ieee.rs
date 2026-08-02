//! CP391 source-shaped binary64 maximum tests.

use super::*;
use crate::ideal_loads::calc::cooling_positive_supply_temperature_minimum_limit::source_shaped_two_argument_maximum;
use crate::psychrometrics::energyplus_psy_h_fn_tdb_w;

#[test]
fn source_shaped_maximum_is_strict_left_and_retains_left_for_ties_and_unordered() {
    let left_nan = f64::from_bits(0x7ff8_0000_0000_00a1);
    let right_nan = f64::from_bits(0x7ff8_0000_0000_00b2);
    let cases = [
        (1.0, 2.0, 2.0f64.to_bits()),
        (2.0, 1.0, 2.0f64.to_bits()),
        (0.0, -0.0, 0.0f64.to_bits()),
        (-0.0, 0.0, (-0.0f64).to_bits()),
        (left_nan, 1.0, left_nan.to_bits()),
        (1.0, right_nan, 1.0f64.to_bits()),
        (left_nan, right_nan, left_nan.to_bits()),
        (f64::NEG_INFINITY, 1.0, 1.0f64.to_bits()),
        (f64::INFINITY, 1.0, f64::INFINITY.to_bits()),
        (1.0, f64::INFINITY, f64::INFINITY.to_bits()),
        (1.0, f64::NEG_INFINITY, 1.0f64.to_bits()),
    ];
    for (left, right, expected_bits) in cases {
        assert_eq!(
            source_shaped_two_argument_maximum(left, right).to_bits(),
            expected_bits,
        );
    }
}

#[test]
fn active_routes_use_canonical_psychrometrics_and_preserve_temperature_bits() {
    for inherited in [3, 4, 7] {
        let chain = fixtures::chain(
            inherited,
            1,
            true,
            Some(D::ConstantSensibleHeatRatio),
            1,
            0.7,
            18.0,
            1.0,
        );
        let mut state =
            PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitRuntimeState::new(
                chain.cp390.system,
            );
        let snapshot =
            advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_state(
                &mut state,
                chain.cp390,
            )
            .expect("active CP391");
        let left = chain
            .cp390
            .resulting_supply_enthalpy_j_per_kg
            .expect("left");
        let temperature = chain
            .cp390
            .resulting_supply_temperature_c
            .expect("temperature");
        let psychrometric = energyplus_psy_h_fn_tdb_w(temperature, 1.0e-5);
        let expected = source_shaped_two_argument_maximum(left, psychrometric);
        assert_eq!(
            snapshot
                .psychrometric_minimum_supply_enthalpy_j_per_kg
                .map(f64::to_bits),
            Some(psychrometric.to_bits()),
        );
        assert_eq!(
            snapshot
                .resulting_supply_enthalpy_j_per_kg
                .map(f64::to_bits),
            Some(expected.to_bits()),
        );
        assert_eq!(
            snapshot.resulting_supply_temperature_c.map(f64::to_bits),
            chain.cp390.resulting_supply_temperature_c.map(f64::to_bits),
        );
    }
}

#[test]
fn source_decimal_humidity_literal_has_locked_binary64_bits() {
    assert_eq!((1.0e-5f64).to_bits(), 0x3ee4_f8b5_88e3_68f1);
}
