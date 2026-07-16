use super::{
    ENERGYPLUS_MIN_HUMIDITY_RATIO, energyplus_humidity_ratio_floor,
    energyplus_psy_tdp_fn_tdb_twb_pb, energyplus_psy_tdp_fn_w_pb, energyplus_psy_w_fn_tdb_twb_pb,
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

fn source_projection(dry_bulb_c: f64, wet_bulb_c: f64, atmospheric_pressure_pa: f64) -> f64 {
    let humidity_ratio = energyplus_humidity_ratio_floor(energyplus_psy_w_fn_tdb_twb_pb(
        dry_bulb_c,
        wet_bulb_c,
        atmospheric_pressure_pa,
    ));
    let dew_point_c = energyplus_psy_tdp_fn_w_pb(humidity_ratio, atmospheric_pressure_pa);
    if dew_point_c > wet_bulb_c {
        wet_bulb_c
    } else {
        dew_point_c
    }
}

#[test]
fn dew_point_matches_upstream_ems_vector() {
    assert_close(
        energyplus_psy_tdp_fn_tdb_twb_pb(30.0, 16.0, 101_325.0),
        5.573_987_554,
        1.0e-8,
    );
}

#[test]
fn composition_preserves_the_second_humidity_floor() {
    let dry_bulb_c = -45.0;
    let wet_bulb_c = -45.1;
    let atmospheric_pressure_pa = 101_325.0;
    let child_humidity_ratio =
        energyplus_psy_w_fn_tdb_twb_pb(dry_bulb_c, wet_bulb_c, atmospheric_pressure_pa);

    assert!(child_humidity_ratio >= 0.0);
    assert!(child_humidity_ratio < ENERGYPLUS_MIN_HUMIDITY_RATIO);
    assert_bits(
        energyplus_psy_tdp_fn_tdb_twb_pb(dry_bulb_c, wet_bulb_c, atmospheric_pressure_pa),
        energyplus_psy_tdp_fn_w_pb(ENERGYPLUS_MIN_HUMIDITY_RATIO, atmospheric_pressure_pa),
    );
}

#[test]
fn wet_bulb_clamp_applies_below_the_diagnostic_threshold() {
    let dry_bulb_c = -57.0;
    let wet_bulb_c = -57.0;
    let atmospheric_pressure_pa = 101_325.0;
    let child_humidity_ratio =
        energyplus_psy_w_fn_tdb_twb_pb(dry_bulb_c, wet_bulb_c, atmospheric_pressure_pa);
    let unclamped_dew_point_c =
        energyplus_psy_tdp_fn_w_pb(ENERGYPLUS_MIN_HUMIDITY_RATIO, atmospheric_pressure_pa);

    assert!(child_humidity_ratio >= 0.0);
    assert!(child_humidity_ratio < ENERGYPLUS_MIN_HUMIDITY_RATIO);
    assert!(unclamped_dew_point_c > wet_bulb_c);
    assert!(unclamped_dew_point_c <= wet_bulb_c + 0.1);
    assert_bits(
        energyplus_psy_tdp_fn_tdb_twb_pb(dry_bulb_c, wet_bulb_c, atmospheric_pressure_pa),
        wet_bulb_c,
    );
}

#[test]
fn wet_bulb_clamp_also_applies_above_the_diagnostic_threshold() {
    let dry_bulb_c = -60.0;
    let wet_bulb_c = -60.0;
    let atmospheric_pressure_pa = 101_325.0;
    let child_humidity_ratio =
        energyplus_psy_w_fn_tdb_twb_pb(dry_bulb_c, wet_bulb_c, atmospheric_pressure_pa);
    let unclamped_dew_point_c =
        energyplus_psy_tdp_fn_w_pb(ENERGYPLUS_MIN_HUMIDITY_RATIO, atmospheric_pressure_pa);

    assert!(child_humidity_ratio >= 0.0);
    assert!(child_humidity_ratio < ENERGYPLUS_MIN_HUMIDITY_RATIO);
    assert!(unclamped_dew_point_c > wet_bulb_c + 0.1);
    assert_bits(
        energyplus_psy_tdp_fn_tdb_twb_pb(dry_bulb_c, wet_bulb_c, atmospheric_pressure_pa),
        wet_bulb_c,
    );
}

#[test]
fn saturated_ordinary_vector_clamps_the_numerical_overshoot() {
    let dry_bulb_c = 20.0;
    let wet_bulb_c = 20.0;
    let atmospheric_pressure_pa = 101_325.0;
    let humidity_ratio =
        energyplus_psy_w_fn_tdb_twb_pb(dry_bulb_c, wet_bulb_c, atmospheric_pressure_pa);
    let unclamped_dew_point_c = energyplus_psy_tdp_fn_w_pb(humidity_ratio, atmospheric_pressure_pa);

    assert!(unclamped_dew_point_c > wet_bulb_c);
    assert!(unclamped_dew_point_c <= wet_bulb_c + 0.1);
    assert_bits(
        energyplus_psy_tdp_fn_tdb_twb_pb(dry_bulb_c, wet_bulb_c, atmospheric_pressure_pa),
        wet_bulb_c,
    );
}

#[test]
fn final_clamp_uses_the_original_wet_bulb_argument() {
    let dry_bulb_c = 10.0;
    let wet_bulb_c = 20.0;
    let atmospheric_pressure_pa = 101_325.0;
    let result = energyplus_psy_tdp_fn_tdb_twb_pb(dry_bulb_c, wet_bulb_c, atmospheric_pressure_pa);

    assert_bits(
        result,
        source_projection(dry_bulb_c, wet_bulb_c, atmospheric_pressure_pa),
    );
    assert!(result > dry_bulb_c);
    assert!(result < wet_bulb_c);
}

#[test]
fn strict_ordered_clamp_preserves_the_composed_positive_zero() {
    let dry_bulb_c = 0.0;
    let wet_bulb_c = -0.0;
    let atmospheric_pressure_pa = 101_325.0;
    let humidity_ratio =
        energyplus_psy_w_fn_tdb_twb_pb(dry_bulb_c, wet_bulb_c, atmospheric_pressure_pa);
    let unclamped_dew_point_c = energyplus_psy_tdp_fn_w_pb(humidity_ratio, atmospheric_pressure_pa);

    assert_bits(unclamped_dew_point_c, 0.0);
    assert_bits(
        energyplus_psy_tdp_fn_tdb_twb_pb(dry_bulb_c, wet_bulb_c, atmospheric_pressure_pa),
        0.0,
    );
    assert_ne!(wet_bulb_c.to_bits(), 0.0_f64.to_bits());
}

#[test]
fn pressure_infinities_preserve_child_order_before_the_final_clamp() {
    assert_bits(
        energyplus_psy_tdp_fn_tdb_twb_pb(16.0, 16.0, f64::INFINITY),
        16.0,
    );
    assert_bits(
        energyplus_psy_tdp_fn_tdb_twb_pb(16.0, 16.0, f64::NEG_INFINITY),
        -100.0,
    );
}

#[test]
fn unordered_inputs_return_the_composed_nan_instead_of_clamping() {
    for (dry_bulb_c, wet_bulb_c, atmospheric_pressure_pa) in [
        (f64::NAN, 16.0, 101_325.0),
        (30.0, f64::NAN, 101_325.0),
        (30.0, 16.0, f64::NAN),
        (30.0, f64::NEG_INFINITY, 101_325.0),
    ] {
        assert!(
            energyplus_psy_tdp_fn_tdb_twb_pb(dry_bulb_c, wet_bulb_c, atmospheric_pressure_pa,)
                .is_nan()
        );
    }
}

#[test]
fn repeated_and_alternating_calls_are_output_stable() {
    let first = energyplus_psy_tdp_fn_tdb_twb_pb(30.0, 16.0, 101_325.0);
    let second = energyplus_psy_tdp_fn_tdb_twb_pb(30.0, -20.0, 101_325.0);

    for _ in 0..16 {
        assert_bits(
            energyplus_psy_tdp_fn_tdb_twb_pb(30.0, 16.0, 101_325.0),
            first,
        );
        assert_bits(
            energyplus_psy_tdp_fn_tdb_twb_pb(30.0, -20.0, 101_325.0),
            second,
        );
    }
}
