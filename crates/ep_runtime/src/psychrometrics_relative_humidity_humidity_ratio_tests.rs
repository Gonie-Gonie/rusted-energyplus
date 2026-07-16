use super::{
    ENERGYPLUS_MIN_HUMIDITY_RATIO, energyplus_psy_psat_fn_temp_default_numerical_projection,
    energyplus_psy_psat_fn_temp_raw, energyplus_psy_rh_fn_tdb_w_pb, energyplus_psy_w_fn_tdb_rh_pb,
    energyplus_psychrometric_humidity_ratio_from_rh,
    energyplus_psychrometric_psat_cache_temperature_c,
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

fn source_grouped_humidity_ratio(
    saturation_pressure_pa: f64,
    relative_humidity: f64,
    atmospheric_pressure_pa: f64,
) -> f64 {
    let dew_pressure_pa = relative_humidity * saturation_pressure_pa;
    let pressure_difference_pa = atmospheric_pressure_pa - dew_pressure_pa;
    let denominator_pa = if pressure_difference_pa < 1000.0 {
        1000.0
    } else {
        pressure_difference_pa
    };
    let humidity_ratio = dew_pressure_pa * 0.621_98 / denominator_pa;
    if humidity_ratio < ENERGYPLUS_MIN_HUMIDITY_RATIO {
        ENERGYPLUS_MIN_HUMIDITY_RATIO
    } else {
        humidity_ratio
    }
}

#[test]
fn relative_humidity_humidity_ratio_matches_upstream_vectors() {
    assert_close(
        energyplus_psy_w_fn_tdb_rh_pb(30.0, 0.5, 101_325.0),
        0.013_310_952_8,
        1.0e-8,
    );
    assert_close(
        energyplus_psy_w_fn_tdb_rh_pb(24.0, 0.5, 101_325.0),
        0.009_299_028_449_273_4,
        1.0e-15,
    );
}

#[test]
fn formula_uses_the_default_cache_representative_and_source_grouping() {
    let dry_bulb_c = 30.123_456_789;
    let relative_humidity = 0.37;
    let atmospheric_pressure_pa = 101_325.0;
    let representative = energyplus_psychrometric_psat_cache_temperature_c(dry_bulb_c);
    let cached_saturation_pressure = energyplus_psy_psat_fn_temp_raw(representative);
    let exact_saturation_pressure = energyplus_psy_psat_fn_temp_raw(dry_bulb_c);
    let expected = source_grouped_humidity_ratio(
        cached_saturation_pressure,
        relative_humidity,
        atmospheric_pressure_pa,
    );
    let no_cache_result = source_grouped_humidity_ratio(
        exact_saturation_pressure,
        relative_humidity,
        atmospheric_pressure_pa,
    );

    assert_ne!(representative.to_bits(), dry_bulb_c.to_bits());
    assert_ne!(
        cached_saturation_pressure.to_bits(),
        exact_saturation_pressure.to_bits()
    );
    assert_bits(
        energyplus_psy_w_fn_tdb_rh_pb(dry_bulb_c, relative_humidity, atmospheric_pressure_pa),
        expected,
    );
    assert_ne!(expected.to_bits(), no_cache_result.to_bits());
}

#[test]
fn valid_domain_forward_and_inverse_paths_round_trip() {
    for (dry_bulb_c, relative_humidity, atmospheric_pressure_pa) in [
        (-20.0, 0.8, 101_325.0),
        (0.0, 0.2, 101_325.0),
        (24.0, 0.5, 101_325.0),
        (30.0, 1.0, 101_325.0),
        (45.0, 0.4, 90_000.0),
    ] {
        let humidity_ratio =
            energyplus_psy_w_fn_tdb_rh_pb(dry_bulb_c, relative_humidity, atmospheric_pressure_pa);
        assert!(humidity_ratio > ENERGYPLUS_MIN_HUMIDITY_RATIO);
        assert_close(
            energyplus_psy_rh_fn_tdb_w_pb(dry_bulb_c, humidity_ratio, atmospheric_pressure_pa),
            relative_humidity,
            5.0e-16,
        );
    }
}

#[test]
fn denominator_floor_uses_the_source_strict_ordered_comparison() {
    let dry_bulb_c = 100.0;
    let relative_humidity = 1.0;
    let dew_pressure_pa = energyplus_psy_psat_fn_temp_default_numerical_projection(dry_bulb_c);

    for (pressure_difference_pa, expected_denominator_pa) in
        [(999.0, 1000.0), (1000.0, 1000.0), (1001.0, 1001.0)]
    {
        let atmospheric_pressure_pa = dew_pressure_pa + pressure_difference_pa;
        assert_bits(
            energyplus_psy_w_fn_tdb_rh_pb(dry_bulb_c, relative_humidity, atmospheric_pressure_pa),
            dew_pressure_pa * 0.621_98 / expected_denominator_pa,
        );
    }

    assert_bits(
        energyplus_psy_w_fn_tdb_rh_pb(dry_bulb_c, relative_humidity, 0.0),
        dew_pressure_pa * 0.621_98 / 1000.0,
    );

    let pivot_pressure_pa = energyplus_psy_psat_fn_temp_default_numerical_projection(30.0) + 1000.0;
    assert_bits(
        energyplus_psy_w_fn_tdb_rh_pb(30.0, 1.0, pivot_pressure_pa.next_down()),
        2.640_945_890_909_728,
    );
    assert_bits(
        energyplus_psy_w_fn_tdb_rh_pb(30.0, 1.0, pivot_pressure_pa),
        2.640_945_890_909_728,
    );
    assert_bits(
        energyplus_psy_w_fn_tdb_rh_pb(30.0, 1.0, pivot_pressure_pa.next_up()),
        2.640_945_890_909_725_4,
    );
}

#[test]
fn final_floor_is_strict_and_normalizes_negative_zero() {
    for relative_humidity in [-1.0, -0.0, 0.0, f64::from_bits(1)] {
        assert_bits(
            energyplus_psy_w_fn_tdb_rh_pb(30.0, relative_humidity, 101_325.0),
            ENERGYPLUS_MIN_HUMIDITY_RATIO,
        );
    }

    assert_bits(
        energyplus_psy_w_fn_tdb_rh_pb(30.0, 0.000_191_833_134_142_342_3, 101_325.0),
        ENERGYPLUS_MIN_HUMIDITY_RATIO,
    );
    assert_bits(
        energyplus_psy_w_fn_tdb_rh_pb(30.0, 0.000_383_663_184_101_110_24, 101_325.0),
        ENERGYPLUS_MIN_HUMIDITY_RATIO,
    );
    assert_bits(
        energyplus_psy_w_fn_tdb_rh_pb(30.0, 0.000_422_028_823_997_380_2, 101_325.0),
        1.099_999_999_999_999_8e-5,
    );

    let saturation_pressure_pa = energyplus_psy_psat_fn_temp_default_numerical_projection(30.0);
    let target_humidity_ratio = ENERGYPLUS_MIN_HUMIDITY_RATIO.next_up();
    let dew_pressure_pa = target_humidity_ratio * 101_325.0 / (0.621_98 + target_humidity_ratio);
    let relative_humidity = dew_pressure_pa / saturation_pressure_pa;
    assert!(
        energyplus_psy_w_fn_tdb_rh_pb(30.0, relative_humidity, 101_325.0)
            > ENERGYPLUS_MIN_HUMIDITY_RATIO
    );
}

#[test]
fn ordered_branches_preserve_source_ieee_results() {
    assert_bits(
        energyplus_psy_w_fn_tdb_rh_pb(30.0, 1.5, 101_325.0),
        0.041_718_487_814_777_54,
    );
    assert!(energyplus_psy_w_fn_tdb_rh_pb(30.0, f64::NAN, 101_325.0).is_nan());
    assert!(energyplus_psy_w_fn_tdb_rh_pb(30.0, 0.5, f64::NAN).is_nan());
    assert_bits(
        energyplus_psy_w_fn_tdb_rh_pb(30.0, f64::INFINITY, 101_325.0),
        f64::INFINITY,
    );
    assert!(energyplus_psy_w_fn_tdb_rh_pb(30.0, f64::NEG_INFINITY, 101_325.0).is_nan());
    assert_bits(
        energyplus_psy_w_fn_tdb_rh_pb(30.0, 0.5, f64::INFINITY),
        ENERGYPLUS_MIN_HUMIDITY_RATIO,
    );

    let nan_temperature_result = energyplus_psy_w_fn_tdb_rh_pb(f64::NAN, 0.5, 101_325.0);
    assert_bits(
        nan_temperature_result,
        source_grouped_humidity_ratio(1_555_073.745_636_215, 0.5, 101_325.0),
    );
}

#[test]
fn canonical_helper_remains_distinct_from_the_guarded_compatibility_wrapper() {
    assert!(energyplus_psy_w_fn_tdb_rh_pb(30.0, f64::NAN, 101_325.0).is_nan());
    assert_eq!(
        energyplus_psychrometric_humidity_ratio_from_rh(30.0, f64::NAN, 101_325.0),
        Some(ENERGYPLUS_MIN_HUMIDITY_RATIO)
    );

    assert!(energyplus_psy_w_fn_tdb_rh_pb(f64::NAN, 0.5, 101_325.0).is_finite());
    assert_eq!(
        energyplus_psychrometric_humidity_ratio_from_rh(f64::NAN, 0.5, 101_325.0),
        None
    );

    assert!(energyplus_psy_w_fn_tdb_rh_pb(30.0, 0.5, f64::NAN).is_nan());
    assert_eq!(
        energyplus_psychrometric_humidity_ratio_from_rh(30.0, 0.5, f64::NAN),
        Some(1.320_472_945_454_864)
    );
}
