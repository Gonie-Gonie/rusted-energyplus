use super::{
    ENERGYPLUS_MIN_HUMIDITY_RATIO, energyplus_psy_h_fn_tdb_rh_pb, energyplus_psy_h_fn_tdb_w,
    energyplus_psy_psat_fn_temp_default_numerical_projection, energyplus_psy_psat_fn_temp_raw,
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

fn ordered_humidity_floor(humidity_ratio: f64) -> f64 {
    if humidity_ratio < ENERGYPLUS_MIN_HUMIDITY_RATIO {
        ENERGYPLUS_MIN_HUMIDITY_RATIO
    } else {
        humidity_ratio
    }
}

fn source_grouped_enthalpy(
    dry_bulb_c: f64,
    relative_humidity: f64,
    atmospheric_pressure_pa: f64,
    saturation_pressure_pa: f64,
) -> f64 {
    let dew_pressure_pa = relative_humidity * saturation_pressure_pa;
    let pressure_difference_pa = atmospheric_pressure_pa - dew_pressure_pa;
    let denominator_pa = if pressure_difference_pa < 1000.0 {
        1000.0
    } else {
        pressure_difference_pa
    };
    let humidity_ratio = ordered_humidity_floor(dew_pressure_pa * 0.621_98 / denominator_pa);
    let humidity_ratio = ordered_humidity_floor(humidity_ratio);
    let humidity_ratio = ordered_humidity_floor(humidity_ratio);
    1.004_84e3 * dry_bulb_c + humidity_ratio * (2.500_94e6 + 1.858_95e3 * dry_bulb_c)
}

fn source_default_projection(
    dry_bulb_c: f64,
    relative_humidity: f64,
    atmospheric_pressure_pa: f64,
) -> f64 {
    source_grouped_enthalpy(
        dry_bulb_c,
        relative_humidity,
        atmospheric_pressure_pa,
        energyplus_psy_psat_fn_temp_default_numerical_projection(dry_bulb_c),
    )
}

#[test]
fn relative_humidity_enthalpy_matches_upstream_vectors() {
    assert_close(
        energyplus_psy_h_fn_tdb_rh_pb(30.0, 0.5, 101_325.0),
        64_177.426_349_195,
        1.0e-8,
    );
    assert_close(
        energyplus_psy_h_fn_tdb_rh_pb(24.0, 0.5, 101_325.0),
        47_787.346_504_384_46,
        1.0e-12,
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
    let expected = source_grouped_enthalpy(
        dry_bulb_c,
        relative_humidity,
        atmospheric_pressure_pa,
        cached_saturation_pressure,
    );
    let no_cache_result = source_grouped_enthalpy(
        dry_bulb_c,
        relative_humidity,
        atmospheric_pressure_pa,
        exact_saturation_pressure,
    );

    assert_ne!(representative.to_bits(), dry_bulb_c.to_bits());
    assert_ne!(
        cached_saturation_pressure.to_bits(),
        exact_saturation_pressure.to_bits()
    );
    assert_bits(
        energyplus_psy_h_fn_tdb_rh_pb(dry_bulb_c, relative_humidity, atmospheric_pressure_pa),
        expected,
    );
    assert_ne!(expected.to_bits(), no_cache_result.to_bits());
}

#[test]
fn ordinary_finite_vectors_preserve_nested_source_composition() {
    for (dry_bulb_c, relative_humidity, atmospheric_pressure_pa) in [
        (-20.0, 0.8, 101_325.0),
        (0.0, 0.2, 101_325.0),
        (24.0, 0.5, 101_325.0),
        (30.0, 1.0, 101_325.0),
        (45.0, 0.4, 90_000.0),
        (30.0, 1.5, 101_325.0),
    ] {
        assert_bits(
            energyplus_psy_h_fn_tdb_rh_pb(dry_bulb_c, relative_humidity, atmospheric_pressure_pa),
            source_default_projection(dry_bulb_c, relative_humidity, atmospheric_pressure_pa),
        );
    }
}

#[test]
fn nested_humidity_floors_preserve_the_source_threshold() {
    let at_floor = energyplus_psy_h_fn_tdb_w(24.0, ENERGYPLUS_MIN_HUMIDITY_RATIO);
    for relative_humidity in [
        -1.0,
        -0.0,
        0.0,
        f64::from_bits(1),
        0.000_191_833_134_142_342_3,
    ] {
        let actual = energyplus_psy_h_fn_tdb_rh_pb(24.0, relative_humidity, 101_325.0);
        assert_bits(actual, at_floor);
    }
}

#[test]
fn inherited_pressure_floor_keeps_its_strict_ordered_boundary() {
    let dry_bulb_c = 100.0;
    let relative_humidity = 1.0;
    let dew_pressure_pa = energyplus_psy_psat_fn_temp_default_numerical_projection(dry_bulb_c);
    let pivot_pressure_pa = dew_pressure_pa + 1000.0;

    for atmospheric_pressure_pa in [
        pivot_pressure_pa.next_down(),
        pivot_pressure_pa,
        pivot_pressure_pa.next_up(),
    ] {
        assert_bits(
            energyplus_psy_h_fn_tdb_rh_pb(dry_bulb_c, relative_humidity, atmospheric_pressure_pa),
            source_default_projection(dry_bulb_c, relative_humidity, atmospheric_pressure_pa),
        );
    }
}

#[test]
fn nonfinite_inputs_preserve_ordered_source_behavior() {
    assert!(energyplus_psy_h_fn_tdb_rh_pb(30.0, f64::NAN, 101_325.0).is_nan());
    assert!(energyplus_psy_h_fn_tdb_rh_pb(30.0, 0.5, f64::NAN).is_nan());
    assert!(energyplus_psy_h_fn_tdb_rh_pb(f64::NAN, 0.5, 101_325.0).is_nan());
    assert_bits(
        energyplus_psy_h_fn_tdb_rh_pb(30.0, f64::INFINITY, 101_325.0),
        f64::INFINITY,
    );
    assert!(energyplus_psy_h_fn_tdb_rh_pb(30.0, f64::NEG_INFINITY, 101_325.0).is_nan());
    assert_bits(
        energyplus_psy_h_fn_tdb_rh_pb(24.0, 0.5, f64::INFINITY),
        energyplus_psy_h_fn_tdb_w(24.0, ENERGYPLUS_MIN_HUMIDITY_RATIO),
    );
}

#[test]
fn repeated_and_alternating_calls_are_output_stable() {
    let first = energyplus_psy_h_fn_tdb_rh_pb(30.0, 0.5, 101_325.0);
    let second = energyplus_psy_h_fn_tdb_rh_pb(-5.0, 0.8, 90_000.0);

    for _ in 0..16 {
        assert_bits(energyplus_psy_h_fn_tdb_rh_pb(30.0, 0.5, 101_325.0), first);
        assert_bits(energyplus_psy_h_fn_tdb_rh_pb(-5.0, 0.8, 90_000.0), second);
    }
}
