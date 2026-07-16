use super::{
    energyplus_psy_psat_fn_temp_default_numerical_projection, energyplus_psy_psat_fn_temp_raw,
    energyplus_psy_w_fn_tdb_rh_pb, energyplus_psy_w_fn_tdb_twb_pb,
    energyplus_psychrometric_humidity_ratio_from_wet_bulb_guess,
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
    dry_bulb_c: f64,
    wet_bulb_c: f64,
    atmospheric_pressure_pa: f64,
    wet_saturation_pressure_pa: f64,
) -> f64 {
    let saturated_humidity_ratio = 0.621_98 * wet_saturation_pressure_pa
        / (atmospheric_pressure_pa - wet_saturation_pressure_pa);
    ((2501.0 - 2.381 * wet_bulb_c) * saturated_humidity_ratio - (dry_bulb_c - wet_bulb_c))
        / (2501.0 + 1.805 * dry_bulb_c - 4.186 * wet_bulb_c)
}

#[test]
fn wet_bulb_humidity_ratio_matches_upstream_vectors() {
    assert_close(
        energyplus_psy_w_fn_tdb_twb_pb(30.0, 16.0, 101_325.0),
        0.005_624_362,
        1.0e-8,
    );
    assert_close(
        energyplus_psy_w_fn_tdb_twb_pb(24.0, 17.0, 101_325.0),
        0.009_235_642_823_366_752,
        1.0e-15,
    );
}

#[test]
fn formula_uses_the_default_cache_representative_and_source_grouping() {
    let dry_bulb_c = 30.123_456_789;
    let wet_bulb_c = 16.123_456_789;
    let atmospheric_pressure_pa = 101_325.0;
    let representative = energyplus_psychrometric_psat_cache_temperature_c(wet_bulb_c);
    let cached_saturation_pressure = energyplus_psy_psat_fn_temp_raw(representative);
    let exact_saturation_pressure = energyplus_psy_psat_fn_temp_raw(wet_bulb_c);
    let expected = source_grouped_humidity_ratio(
        dry_bulb_c,
        wet_bulb_c,
        atmospheric_pressure_pa,
        cached_saturation_pressure,
    );
    let no_cache_result = source_grouped_humidity_ratio(
        dry_bulb_c,
        wet_bulb_c,
        atmospheric_pressure_pa,
        exact_saturation_pressure,
    );

    assert_ne!(representative.to_bits(), wet_bulb_c.to_bits());
    assert_ne!(
        cached_saturation_pressure.to_bits(),
        exact_saturation_pressure.to_bits()
    );
    assert_bits(
        energyplus_psy_w_fn_tdb_twb_pb(dry_bulb_c, wet_bulb_c, atmospheric_pressure_pa),
        expected,
    );
    assert_ne!(expected.to_bits(), no_cache_result.to_bits());
}

#[test]
fn wet_bulb_above_dry_bulb_clamps_before_saturation_pressure() {
    let saturated = energyplus_psy_w_fn_tdb_twb_pb(30.0, 30.0, 101_325.0);
    for wet_bulb_c in [
        30.0_f64.next_up(),
        30.005,
        30.01,
        30.01_f64.next_up(),
        31.0,
        f64::INFINITY,
    ] {
        assert_bits(
            energyplus_psy_w_fn_tdb_twb_pb(30.0, wet_bulb_c, 101_325.0),
            saturated,
        );
    }
}

#[test]
fn source_formula_has_no_freezing_branch_and_differs_from_the_guarded_guess() {
    let dry_bulb_c = -3.0;
    let wet_bulb_c = -5.0;
    let atmospheric_pressure_pa = 101_325.0;
    let source_result =
        energyplus_psy_w_fn_tdb_twb_pb(dry_bulb_c, wet_bulb_c, atmospheric_pressure_pa);
    let guarded_guess = energyplus_psychrometric_humidity_ratio_from_wet_bulb_guess(
        dry_bulb_c,
        wet_bulb_c,
        atmospheric_pressure_pa,
    )
    .expect("guarded compatibility guess should accept this vector");

    assert_ne!(source_result.to_bits(), guarded_guess.to_bits());
    assert_bits(
        source_result,
        source_grouped_humidity_ratio(
            dry_bulb_c,
            wet_bulb_c,
            atmospheric_pressure_pa,
            energyplus_psy_psat_fn_temp_default_numerical_projection(wet_bulb_c),
        ),
    );
}

#[test]
fn strictly_negative_raw_result_uses_the_relative_humidity_fallback() {
    for (dry_bulb_c, wet_bulb_c, atmospheric_pressure_pa) in [
        (30.0, -20.0, 101_325.0),
        (200.0, -50.0, 101_325.0),
        (30.0, 16.0, 1000.0),
    ] {
        let raw = source_grouped_humidity_ratio(
            dry_bulb_c,
            wet_bulb_c,
            atmospheric_pressure_pa,
            energyplus_psy_psat_fn_temp_default_numerical_projection(wet_bulb_c),
        );
        assert!(raw < 0.0, "raw={raw:?}");
        assert_bits(
            energyplus_psy_w_fn_tdb_twb_pb(dry_bulb_c, wet_bulb_c, atmospheric_pressure_pa),
            energyplus_psy_w_fn_tdb_rh_pb(dry_bulb_c, 0.0001, atmospheric_pressure_pa),
        );
    }
}

#[test]
fn pressure_pole_and_nonfinite_inputs_preserve_ordered_source_behavior() {
    let wet_saturation_pressure = energyplus_psy_psat_fn_temp_default_numerical_projection(16.0);
    assert_bits(
        energyplus_psy_w_fn_tdb_twb_pb(30.0, 16.0, wet_saturation_pressure),
        f64::INFINITY,
    );

    let pressure_below_pole = wet_saturation_pressure.next_down();
    let raw_below_pole =
        source_grouped_humidity_ratio(30.0, 16.0, pressure_below_pole, wet_saturation_pressure);
    assert!(raw_below_pole < 0.0, "raw={raw_below_pole:?}");
    assert_bits(
        energyplus_psy_w_fn_tdb_twb_pb(30.0, 16.0, pressure_below_pole),
        energyplus_psy_w_fn_tdb_rh_pb(30.0, 0.0001, pressure_below_pole),
    );

    let pressure_above_pole = wet_saturation_pressure.next_up();
    let raw_above_pole =
        source_grouped_humidity_ratio(30.0, 16.0, pressure_above_pole, wet_saturation_pressure);
    assert!(raw_above_pole > 0.0, "raw={raw_above_pole:?}");
    assert_bits(
        energyplus_psy_w_fn_tdb_twb_pb(30.0, 16.0, pressure_above_pole),
        raw_above_pole,
    );

    assert_bits(
        energyplus_psy_w_fn_tdb_twb_pb(16.0, 16.0, f64::NEG_INFINITY),
        -0.0,
    );
    assert_bits(
        energyplus_psy_w_fn_tdb_twb_pb(16.0, 16.0, f64::INFINITY),
        0.0,
    );
    assert!(energyplus_psy_w_fn_tdb_twb_pb(30.0, f64::NAN, 101_325.0).is_nan());
    assert!(energyplus_psy_w_fn_tdb_twb_pb(f64::NAN, 16.0, 101_325.0).is_nan());
    assert!(energyplus_psy_w_fn_tdb_twb_pb(30.0, 16.0, f64::NAN).is_nan());
    assert!(energyplus_psy_w_fn_tdb_twb_pb(30.0, f64::NEG_INFINITY, 101_325.0).is_nan());
}

#[test]
fn repeated_and_alternating_calls_are_output_stable() {
    let first = energyplus_psy_w_fn_tdb_twb_pb(30.0, 16.0, 101_325.0);
    let second = energyplus_psy_w_fn_tdb_twb_pb(5.0, -5.0, 90_000.0);

    for _ in 0..16 {
        assert_bits(energyplus_psy_w_fn_tdb_twb_pb(30.0, 16.0, 101_325.0), first);
        assert_bits(energyplus_psy_w_fn_tdb_twb_pb(5.0, -5.0, 90_000.0), second);
    }
}
