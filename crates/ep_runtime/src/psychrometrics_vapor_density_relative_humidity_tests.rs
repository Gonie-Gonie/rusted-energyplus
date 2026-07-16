use super::{
    KELVIN_OFFSET, energyplus_psy_psat_fn_temp_default_numerical_projection,
    energyplus_psy_psat_fn_temp_raw, energyplus_psy_rh_fn_tdb_rhov,
    energyplus_psy_rh_fn_tdb_rhov_lbnd0c, energyplus_psy_rhov_fn_tdb_rh,
    energyplus_psy_rhov_fn_tdb_rh_lbnd0c, energyplus_psychrometric_psat_cache_temperature_c,
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

fn vapor_density_for_raw_relative_humidity(dry_bulb_c: f64, relative_humidity: f64) -> f64 {
    relative_humidity * energyplus_psy_psat_fn_temp_default_numerical_projection(dry_bulb_c)
        / (461.52 * (dry_bulb_c + KELVIN_OFFSET))
}

#[test]
fn vapor_density_and_relative_humidity_match_pinned_ems_vectors() {
    let vapor_density = energyplus_psy_rhov_fn_tdb_rh(30.0, 0.5);
    let relative_humidity = energyplus_psy_rh_fn_tdb_rhov(30.0, 0.01);
    assert_close(vapor_density, 0.015_174_171, 1.0e-8);
    assert_close(relative_humidity, 0.329_507_280_8, 1.0e-8);
    assert_ne!(
        vapor_density.to_bits(),
        energyplus_psy_rhov_fn_tdb_rh_lbnd0c(30.0, 0.5).to_bits()
    );
    assert_ne!(
        relative_humidity.to_bits(),
        energyplus_psy_rh_fn_tdb_rhov_lbnd0c(30.0, 0.01).to_bits()
    );
}

#[test]
fn both_formulas_use_the_default_cache_temperature_representative() {
    let dry_bulb_c = 30.123_456_789;
    let representative = energyplus_psychrometric_psat_cache_temperature_c(dry_bulb_c);
    let saturation_pressure = energyplus_psy_psat_fn_temp_raw(representative);
    assert_ne!(representative.to_bits(), dry_bulb_c.to_bits());
    assert_ne!(
        saturation_pressure.to_bits(),
        energyplus_psy_psat_fn_temp_raw(dry_bulb_c).to_bits()
    );
    assert_bits(
        energyplus_psy_psat_fn_temp_default_numerical_projection(dry_bulb_c),
        saturation_pressure,
    );

    let relative_humidity = 0.37;
    assert_bits(
        energyplus_psy_rhov_fn_tdb_rh(dry_bulb_c, relative_humidity),
        (saturation_pressure * relative_humidity) / (461.52 * (dry_bulb_c + KELVIN_OFFSET)),
    );

    let vapor_density = 0.01;
    assert_bits(
        energyplus_psy_rh_fn_tdb_rhov(dry_bulb_c, vapor_density),
        vapor_density * 461.52 * (dry_bulb_c + KELVIN_OFFSET) / saturation_pressure,
    );
}

#[test]
fn valid_domain_forward_and_inverse_paths_round_trip() {
    for (dry_bulb_c, relative_humidity) in [
        (-80.0, 0.2),
        (-20.0, 0.8),
        (0.0, 0.05),
        (30.0, 0.5),
        (100.0, 0.9),
    ] {
        let vapor_density = energyplus_psy_rhov_fn_tdb_rh(dry_bulb_c, relative_humidity);
        assert_close(
            energyplus_psy_rh_fn_tdb_rhov(dry_bulb_c, vapor_density),
            relative_humidity,
            4.0e-16,
        );
    }
}

#[test]
fn vapor_density_preserves_source_ieee_arithmetic() {
    assert!(energyplus_psy_rhov_fn_tdb_rh(f64::NAN, 0.5).is_nan());
    assert_bits(energyplus_psy_rhov_fn_tdb_rh(30.0, -0.0), -0.0);
    assert_bits(energyplus_psy_rhov_fn_tdb_rh(f64::INFINITY, 0.5), 0.0);
    assert_bits(energyplus_psy_rhov_fn_tdb_rh(f64::NEG_INFINITY, 0.5), -0.0);
    assert!(energyplus_psy_rhov_fn_tdb_rh(-KELVIN_OFFSET, 0.0).is_nan());
    assert_bits(
        energyplus_psy_rhov_fn_tdb_rh(-KELVIN_OFFSET, 0.5),
        f64::INFINITY,
    );

    for relative_humidity in [f64::NEG_INFINITY, -0.25, 1.5, f64::INFINITY] {
        assert_bits(
            energyplus_psy_rhov_fn_tdb_rh(30.0, relative_humidity),
            (energyplus_psy_psat_fn_temp_default_numerical_projection(30.0) * relative_humidity)
                / (461.52 * (30.0 + KELVIN_OFFSET)),
        );
    }
    assert!(energyplus_psy_rhov_fn_tdb_rh(30.0, f64::NAN).is_nan());
}

#[test]
fn relative_humidity_bypasses_the_formula_for_nonpositive_or_nan_vapor() {
    for vapor_density in [f64::NEG_INFINITY, -1.0, -0.0, 0.0, f64::NAN] {
        assert_bits(energyplus_psy_rh_fn_tdb_rhov(f64::NAN, vapor_density), 0.0);
    }
}

#[test]
fn relative_humidity_preserves_in_range_values_and_corrects_only_outside() {
    let low_in_range = 0.005;
    let low_in_range_vapor = vapor_density_for_raw_relative_humidity(30.0, low_in_range);
    assert_close(
        energyplus_psy_rh_fn_tdb_rhov(30.0, low_in_range_vapor),
        low_in_range,
        1.0e-18,
    );

    let high_vapor = vapor_density_for_raw_relative_humidity(30.0, 1.005);
    assert_bits(energyplus_psy_rh_fn_tdb_rhov(30.0, high_vapor), 1.0);

    let negative_vapor = vapor_density_for_raw_relative_humidity(-300.0, -0.025);
    assert!(negative_vapor > 0.0);
    assert_bits(energyplus_psy_rh_fn_tdb_rhov(-300.0, negative_vapor), 0.01);
}

#[test]
fn relative_humidity_preserves_positive_branch_ieee_results() {
    assert!(energyplus_psy_rh_fn_tdb_rhov(f64::NAN, 0.01).is_nan());
    assert_bits(energyplus_psy_rh_fn_tdb_rhov(30.0, f64::INFINITY), 1.0);
    assert_bits(energyplus_psy_rh_fn_tdb_rhov(f64::INFINITY, 0.01), 1.0);
    assert_bits(energyplus_psy_rh_fn_tdb_rhov(f64::NEG_INFINITY, 0.01), 0.01);
    assert_bits(energyplus_psy_rh_fn_tdb_rhov(-KELVIN_OFFSET, 0.01), 0.0);
    assert!(energyplus_psy_rh_fn_tdb_rhov(-KELVIN_OFFSET, f64::INFINITY).is_nan());
}
