use super::{
    ENERGYPLUS_MIN_HUMIDITY_RATIO, energyplus_psy_psat_fn_temp_default_numerical_projection,
    energyplus_psy_tdp_fn_w_pb, energyplus_psy_tsat_fn_pb_raw,
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

fn source_projection(humidity_ratio: f64, atmospheric_pressure_pa: f64) -> f64 {
    let humidity_ratio = if humidity_ratio < ENERGYPLUS_MIN_HUMIDITY_RATIO {
        ENERGYPLUS_MIN_HUMIDITY_RATIO
    } else {
        humidity_ratio
    };
    let dew_pressure_pa = atmospheric_pressure_pa * humidity_ratio / (0.621_98 + humidity_ratio);
    energyplus_psy_tsat_fn_pb_raw(dew_pressure_pa)
}

#[test]
fn dew_point_matches_upstream_ems_vector() {
    assert_close(
        energyplus_psy_tdp_fn_w_pb(0.01, 101_325.0),
        14.044_515_576,
        1.0e-8,
    );
}

#[test]
fn dew_point_matches_local_energyplus_261_regression_vectors() {
    for (humidity_ratio, atmospheric_pressure_pa, expected_dew_point_c) in [
        (0.009_870_370_393_138_59, 101_325.0, 13.846_750_136_054_634),
        (
            ENERGYPLUS_MIN_HUMIDITY_RATIO,
            101_325.0,
            -56.929_429_462_063_695,
        ),
        (0.02, 101_325.0, 24.933_320_715_924_66),
        (0.01, 80_000.0, 10.454_125_833_978_923),
    ] {
        assert_close(
            energyplus_psy_tdp_fn_w_pb(humidity_ratio, atmospheric_pressure_pa),
            expected_dew_point_c,
            1.0e-9,
        );
    }
}

#[test]
fn humidity_floor_is_inclusive_and_ordered() {
    let expected = energyplus_psy_tdp_fn_w_pb(ENERGYPLUS_MIN_HUMIDITY_RATIO, 101_325.0);
    for humidity_ratio in [
        f64::NEG_INFINITY,
        -1.0,
        -0.0,
        0.0,
        ENERGYPLUS_MIN_HUMIDITY_RATIO.next_down(),
        ENERGYPLUS_MIN_HUMIDITY_RATIO,
    ] {
        assert_bits(
            energyplus_psy_tdp_fn_w_pb(humidity_ratio, 101_325.0),
            expected,
        );
    }

    assert_ne!(
        energyplus_psy_tdp_fn_w_pb(ENERGYPLUS_MIN_HUMIDITY_RATIO.next_up(), 101_325.0).to_bits(),
        expected.to_bits()
    );
}

#[test]
fn dew_pressure_composition_preserves_source_grouping() {
    for (humidity_ratio, atmospheric_pressure_pa) in [
        (0.01, 101_325.0),
        (ENERGYPLUS_MIN_HUMIDITY_RATIO.next_up(), 101_325.0),
        (0.25, 80_000.0),
        (1.0, 2_000_000.0),
    ] {
        assert_bits(
            energyplus_psy_tdp_fn_w_pb(humidity_ratio, atmospheric_pressure_pa),
            source_projection(humidity_ratio, atmospheric_pressure_pa),
        );
    }

    // The adjacent pressure first multiplies without overflow, then crosses
    // the source's multiplication-before-division overflow boundary.
    assert_ne!(
        energyplus_psy_tdp_fn_w_pb(f64::MAX, 1.0).to_bits(),
        200.0_f64.to_bits()
    );
    assert_bits(
        energyplus_psy_tdp_fn_w_pb(f64::MAX, 1.0_f64.next_up()),
        200.0,
    );
    // Regrouping as pressure * (humidity_ratio / denominator) would remain
    // near 1 Pa and therefore return a completely different temperature.
    assert_ne!(
        energyplus_psy_tsat_fn_pb_raw(1.0_f64.next_up() * (f64::MAX / (0.621_98 + f64::MAX)))
            .to_bits(),
        200.0_f64.to_bits()
    );
}

#[test]
fn pressure_edges_preserve_the_isolated_raw_projection() {
    assert_bits(energyplus_psy_tdp_fn_w_pb(0.01, f64::NEG_INFINITY), -100.0);
    assert_bits(energyplus_psy_tdp_fn_w_pb(0.01, -0.0), -100.0);
    // The pure projection intentionally excludes the public cache's fresh
    // tag-zero false hit, which returns 0 C upstream for positive zero.
    assert_bits(energyplus_psy_tdp_fn_w_pb(0.01, 0.0), -100.0);
    assert_bits(energyplus_psy_tdp_fn_w_pb(0.01, f64::from_bits(1)), -100.0);
    assert_bits(energyplus_psy_tdp_fn_w_pb(0.01, f64::INFINITY), 200.0);
    assert!(energyplus_psy_tdp_fn_w_pb(0.01, f64::NAN).is_nan());
}

#[test]
fn nonfinite_humidity_ratio_edges_follow_the_ordered_floor() {
    assert_bits(
        energyplus_psy_tdp_fn_w_pb(f64::NEG_INFINITY, 101_325.0),
        energyplus_psy_tdp_fn_w_pb(ENERGYPLUS_MIN_HUMIDITY_RATIO, 101_325.0),
    );
    assert!(energyplus_psy_tdp_fn_w_pb(f64::INFINITY, 101_325.0).is_nan());
    assert!(energyplus_psy_tdp_fn_w_pb(f64::NAN, 101_325.0).is_nan());
}

#[test]
fn humidity_ratio_and_dew_point_round_trip_on_the_modeled_path() {
    let atmospheric_pressure_pa = 101_325.0;
    for dew_point_c in [-50.0, -20.0, 0.0, 20.0, 40.0, 80.0] {
        let dew_pressure_pa = energyplus_psy_psat_fn_temp_default_numerical_projection(dew_point_c);
        let humidity_ratio =
            0.621_98 * dew_pressure_pa / (atmospheric_pressure_pa - dew_pressure_pa);
        assert_close(
            energyplus_psy_tdp_fn_w_pb(humidity_ratio, atmospheric_pressure_pa),
            dew_point_c,
            0.001,
        );
    }
}

#[test]
fn composed_pressure_reaches_raw_clamps_and_triple_shortcut() {
    let humidity_ratio = 0.01;
    let pressure_for_dew_pressure =
        |dew_pressure_pa: f64| dew_pressure_pa * (0.621_98 + humidity_ratio) / humidity_ratio;

    assert_bits(
        energyplus_psy_tdp_fn_w_pb(humidity_ratio, pressure_for_dew_pressure(0.001)),
        -100.0,
    );
    assert_bits(
        energyplus_psy_tdp_fn_w_pb(humidity_ratio, pressure_for_dew_pressure(611.1)),
        0.0,
    );
    assert_bits(
        energyplus_psy_tdp_fn_w_pb(humidity_ratio, pressure_for_dew_pressure(2_000_000.0)),
        200.0,
    );
}

#[test]
fn repeated_and_alternating_calls_are_output_stable() {
    let first = energyplus_psy_tdp_fn_w_pb(0.01, 101_325.0);
    let second = energyplus_psy_tdp_fn_w_pb(0.02, 80_000.0);

    for _ in 0..16 {
        assert_bits(energyplus_psy_tdp_fn_w_pb(0.01, 101_325.0), first);
        assert_bits(energyplus_psy_tdp_fn_w_pb(0.02, 80_000.0), second);
    }
}
