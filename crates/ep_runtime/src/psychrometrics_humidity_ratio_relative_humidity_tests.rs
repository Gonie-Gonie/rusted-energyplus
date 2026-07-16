use super::{
    ENERGYPLUS_MIN_HUMIDITY_RATIO, energyplus_psy_psat_fn_temp_default_numerical_projection,
    energyplus_psy_psat_fn_temp_raw, energyplus_psy_rh_fn_tdb_w_pb,
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

fn source_grouped_raw_relative_humidity(
    saturation_pressure_pa: f64,
    humidity_ratio: f64,
    atmospheric_pressure_pa: f64,
) -> f64 {
    let degree_of_saturation = humidity_ratio
        / (0.621_98 * saturation_pressure_pa / (atmospheric_pressure_pa - saturation_pressure_pa));
    degree_of_saturation
        / (1.0 - (1.0 - degree_of_saturation) * (saturation_pressure_pa / atmospheric_pressure_pa))
}

fn humidity_ratio_for_relative_humidity(
    saturation_pressure_pa: f64,
    relative_humidity: f64,
    atmospheric_pressure_pa: f64,
) -> f64 {
    let vapor_pressure_pa = relative_humidity * saturation_pressure_pa;
    0.621_98 * vapor_pressure_pa / (atmospheric_pressure_pa - vapor_pressure_pa)
}

#[test]
fn humidity_ratio_relative_humidity_matches_upstream_vectors() {
    assert_close(
        energyplus_psy_rh_fn_tdb_w_pb(30.0, 0.01, 101_325.0),
        0.377_598_442,
        1.0e-8,
    );
    assert_close(
        energyplus_psy_rh_fn_tdb_w_pb(24.0, 0.009, 101_325.0),
        0.484_150_857_767_150_3,
        1.0e-12,
    );
}

#[test]
fn formula_uses_the_default_cache_representative_and_source_grouping() {
    let dry_bulb_c = 30.123_456_789;
    let humidity_ratio = 0.01;
    let atmospheric_pressure_pa = 101_325.0;
    let representative = energyplus_psychrometric_psat_cache_temperature_c(dry_bulb_c);
    let cached_saturation_pressure = energyplus_psy_psat_fn_temp_raw(representative);
    let exact_saturation_pressure = energyplus_psy_psat_fn_temp_raw(dry_bulb_c);
    let expected = source_grouped_raw_relative_humidity(
        cached_saturation_pressure,
        humidity_ratio,
        atmospheric_pressure_pa,
    );
    let no_cache_result = source_grouped_raw_relative_humidity(
        exact_saturation_pressure,
        humidity_ratio,
        atmospheric_pressure_pa,
    );

    assert_ne!(representative.to_bits(), dry_bulb_c.to_bits());
    assert_ne!(
        cached_saturation_pressure.to_bits(),
        exact_saturation_pressure.to_bits()
    );
    assert_bits(
        energyplus_psy_rh_fn_tdb_w_pb(dry_bulb_c, humidity_ratio, atmospheric_pressure_pa),
        expected,
    );
    assert_ne!(expected.to_bits(), no_cache_result.to_bits());
}

#[test]
fn humidity_ratio_floor_preserves_source_ordered_max_semantics() {
    let at_floor = energyplus_psy_rh_fn_tdb_w_pb(30.0, ENERGYPLUS_MIN_HUMIDITY_RATIO, 101_325.0);
    let below_floor = ENERGYPLUS_MIN_HUMIDITY_RATIO.next_down();

    for humidity_ratio in [
        f64::NEG_INFINITY,
        -1.0,
        -0.0,
        0.0,
        f64::from_bits(1),
        below_floor,
        ENERGYPLUS_MIN_HUMIDITY_RATIO,
    ] {
        assert_bits(
            energyplus_psy_rh_fn_tdb_w_pb(30.0, humidity_ratio, 101_325.0),
            at_floor,
        );
    }

    assert_close(at_floor, 0.000_383_663_184_101_110_3, 1.0e-18);
    assert_ne!(
        energyplus_psy_rh_fn_tdb_w_pb(30.0, ENERGYPLUS_MIN_HUMIDITY_RATIO.next_up(), 101_325.0,)
            .to_bits(),
        at_floor.to_bits()
    );
    assert!(energyplus_psy_rh_fn_tdb_w_pb(30.0, f64::NAN, 101_325.0).is_nan());
    assert!(energyplus_psy_rh_fn_tdb_w_pb(30.0, f64::INFINITY, 101_325.0).is_nan());
}

#[test]
fn valid_domain_inverse_vectors_round_trip() {
    for (dry_bulb_c, relative_humidity, atmospheric_pressure_pa) in [
        (30.0, 0.005, 101_325.0),
        (30.0, 0.2, 101_325.0),
        (24.0, 0.5, 101_325.0),
        (-20.0, 0.8, 101_325.0),
        (24.0, 1.0, 101_325.0),
    ] {
        let saturation_pressure_pa =
            energyplus_psy_psat_fn_temp_default_numerical_projection(dry_bulb_c);
        let humidity_ratio = humidity_ratio_for_relative_humidity(
            saturation_pressure_pa,
            relative_humidity,
            atmospheric_pressure_pa,
        );
        assert!(humidity_ratio > ENERGYPLUS_MIN_HUMIDITY_RATIO);
        assert_close(
            energyplus_psy_rh_fn_tdb_w_pb(dry_bulb_c, humidity_ratio, atmospheric_pressure_pa),
            relative_humidity,
            4.0e-16,
        );
    }
}

#[test]
fn correction_changes_only_raw_values_outside_zero_to_one() {
    let low_in_range = energyplus_psy_rh_fn_tdb_w_pb(30.0, 0.000_130_347_858_468_244_36, 101_325.0);
    assert!(low_in_range > 0.0);
    assert!(low_in_range < 0.01);
    assert_close(low_in_range, 0.005, 2.0e-18);

    assert_bits(
        energyplus_psy_rh_fn_tdb_w_pb(30.0, 0.027_346_099_663_569_396, 101_325.0),
        1.0,
    );
    assert_bits(
        energyplus_psy_rh_fn_tdb_w_pb(30.0, 0.027_772_475_036_271_612, 101_325.0),
        1.0,
    );
    assert_bits(
        energyplus_psy_rh_fn_tdb_w_pb(30.0, 0.000_652_286_088_667_575_9, -101_325.0),
        0.01,
    );
    assert_bits(
        energyplus_psy_rh_fn_tdb_w_pb(30.0, 0.001_567_788_463_740_83, -101_325.0),
        0.01,
    );
}

#[test]
fn pressure_singularities_preserve_the_unsimplified_source_formula() {
    let saturation_pressure_pa = energyplus_psy_psat_fn_temp_default_numerical_projection(30.0);

    assert_bits(energyplus_psy_rh_fn_tdb_w_pb(30.0, 0.01, 0.0), 0.0);
    assert_bits(energyplus_psy_rh_fn_tdb_w_pb(30.0, 0.01, -0.0), -0.0);
    assert!(energyplus_psy_rh_fn_tdb_w_pb(30.0, 0.01, saturation_pressure_pa).is_nan());
    for atmospheric_pressure_pa in [f64::NEG_INFINITY, f64::INFINITY, f64::NAN] {
        assert!(energyplus_psy_rh_fn_tdb_w_pb(30.0, 0.01, atmospheric_pressure_pa).is_nan());
    }
}

#[test]
fn pure_projection_documents_nonfinite_temperature_behavior_outside_the_claim() {
    let expected_high_branch = 0.001_031_008_601_714_219_4;
    assert_close(
        energyplus_psy_rh_fn_tdb_w_pb(f64::NAN, 0.01, 101_325.0),
        expected_high_branch,
        1.0e-18,
    );
    assert_close(
        energyplus_psy_rh_fn_tdb_w_pb(f64::INFINITY, 0.01, 101_325.0),
        expected_high_branch,
        1.0e-18,
    );
    assert_bits(
        energyplus_psy_rh_fn_tdb_w_pb(f64::NEG_INFINITY, 0.01, 101_325.0),
        1.0,
    );
    assert_bits(energyplus_psy_rh_fn_tdb_w_pb(-273.15, 0.01, 101_325.0), 1.0);
    assert!(energyplus_psy_rh_fn_tdb_w_pb(30.0, f64::MAX, 101_325.0).is_nan());
}
