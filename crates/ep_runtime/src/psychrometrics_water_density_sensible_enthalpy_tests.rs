use super::{
    energyplus_psy_delta_h_sen_fn_tdb2_tdb1_w, energyplus_psy_delta_h_sen_fn_tdb2_w2_tdb1_w1,
    energyplus_psy_h_fn_tdb_w, energyplus_rho_h2o,
};

fn assert_bits(actual: f64, expected: f64) {
    assert_eq!(
        actual.to_bits(),
        expected.to_bits(),
        "actual={actual:?}, expected={expected:?}"
    );
}

fn assert_close(actual: f64, expected: f64, absolute_tolerance: f64) {
    assert!(
        (actual - expected).abs() <= absolute_tolerance,
        "actual={actual:?}, expected={expected:?}, tolerance={absolute_tolerance:?}"
    );
}

#[test]
fn water_density_matches_upstream_ems_vector() {
    assert_close(energyplus_rho_h2o(20.0), 998.233_186_265_2, 1.0e-8);
}

#[test]
fn water_density_preserves_separate_power_and_sum_order() {
    let source_order = energyplus_rho_h2o(150.0);
    let horner_rewrite: f64 =
        1_000.120_7 + 150.0 * (8.321_587_4e-4 + 150.0 * (-4.929_976e-3 + 150.0 * 8.479_186_3e-6));

    assert_bits(source_order, 917.938_317_573_500_2);
    assert_ne!(source_order.to_bits(), horner_rewrite.to_bits());
}

#[test]
fn water_density_has_no_runtime_range_guard() {
    assert_bits(energyplus_rho_h2o(0.0), 1_000.120_7);
    assert_bits(energyplus_rho_h2o(-273.15), 459.257_746_643_910_65);
    assert!(energyplus_rho_h2o(f64::INFINITY).is_nan());
    assert_bits(energyplus_rho_h2o(f64::NEG_INFINITY), f64::NEG_INFINITY);
    assert!(energyplus_rho_h2o(f64::NAN).is_nan());
}

#[test]
fn one_humidity_delta_preserves_sign_floor_and_zero() {
    assert_bits(
        energyplus_psy_delta_h_sen_fn_tdb2_tdb1_w(30.0, 20.0, 0.01),
        10_234.295,
    );
    assert_bits(
        energyplus_psy_delta_h_sen_fn_tdb2_tdb1_w(20.0, 30.0, 0.01),
        -10_234.295,
    );
    assert_bits(
        energyplus_psy_delta_h_sen_fn_tdb2_tdb1_w(-0.0, 0.0, 0.01),
        -0.0,
    );

    let floor_result = energyplus_psy_delta_h_sen_fn_tdb2_tdb1_w(30.0, 20.0, 1.0e-5);
    for humidity_ratio in [1.0e-5_f64.next_down(), -0.0, f64::NEG_INFINITY, f64::NAN] {
        assert_bits(
            energyplus_psy_delta_h_sen_fn_tdb2_tdb1_w(30.0, 20.0, humidity_ratio),
            floor_result,
        );
    }
    assert_ne!(
        energyplus_psy_delta_h_sen_fn_tdb2_tdb1_w(30.0, 20.0, 0.01).to_bits(),
        floor_result.to_bits()
    );
}

#[test]
fn one_humidity_delta_matches_the_stated_enthalpy_subtraction() {
    for (dry_bulb_2_c, dry_bulb_1_c, humidity_ratio) in [
        (30.0, 20.0, 0.01),
        (-15.0, 35.0, 0.002),
        (70.0, -20.0, 1.0e-5),
    ] {
        let simplified =
            energyplus_psy_delta_h_sen_fn_tdb2_tdb1_w(dry_bulb_2_c, dry_bulb_1_c, humidity_ratio);
        let subtraction = energyplus_psy_h_fn_tdb_w(dry_bulb_2_c, humidity_ratio)
            - energyplus_psy_h_fn_tdb_w(dry_bulb_1_c, humidity_ratio);
        assert_close(simplified, subtraction, 1.0e-9);
    }
}

#[test]
fn one_humidity_delta_matches_upstream_doas_sensible_output() {
    let enthalpy_delta = energyplus_psy_delta_h_sen_fn_tdb2_tdb1_w(23.0, 23.9, 0.014_46);

    assert_bits(enthalpy_delta, -928.548_375_299_998_5);
    assert_close(enthalpy_delta * 4.406_4, -4_091.6, 0.1);
}

#[test]
fn two_humidity_delta_selects_the_ordered_minimum_and_delegates() {
    let expected = energyplus_psy_delta_h_sen_fn_tdb2_tdb1_w(30.0, 20.0, 0.01);
    assert_bits(
        energyplus_psy_delta_h_sen_fn_tdb2_w2_tdb1_w1(30.0, 0.02, 20.0, 0.01),
        expected,
    );
    assert_bits(
        energyplus_psy_delta_h_sen_fn_tdb2_w2_tdb1_w1(30.0, 0.01, 20.0, 0.02),
        expected,
    );
}

#[test]
fn two_humidity_delta_preserves_source_nan_asymmetry() {
    let first_argument_nan =
        energyplus_psy_delta_h_sen_fn_tdb2_w2_tdb1_w1(30.0, 0.02, 20.0, f64::NAN);
    let second_argument_nan =
        energyplus_psy_delta_h_sen_fn_tdb2_w2_tdb1_w1(30.0, f64::NAN, 20.0, 0.02);

    assert_bits(first_argument_nan, 10_420.19);
    assert_bits(second_argument_nan, 10_048.585_895);
    assert_ne!(first_argument_nan.to_bits(), second_argument_nan.to_bits());
}

#[test]
fn sensible_delta_ieee_edges_follow_the_unguarded_expression() {
    assert_bits(
        energyplus_psy_delta_h_sen_fn_tdb2_tdb1_w(f64::INFINITY, 20.0, 0.01),
        f64::INFINITY,
    );
    assert!(energyplus_psy_delta_h_sen_fn_tdb2_tdb1_w(20.0, 20.0, f64::INFINITY).is_nan());
    assert!(energyplus_psy_delta_h_sen_fn_tdb2_tdb1_w(f64::NAN, 20.0, 0.01).is_nan());
}
