use super::{
    KELVIN_OFFSET, energyplus_psy_rh_fn_tdb_rhov_lbnd0c, energyplus_psy_rhov_fn_tdb_rh_lbnd0c,
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
fn psy_rh_lbnd0c_matches_the_pinned_ems_source_vector() {
    // EnergyPlus EMSManager.unit.cc pins this source input pair to about
    // 0.3298971165. This is source-fixture evidence, not an external run.
    assert_close(
        energyplus_psy_rh_fn_tdb_rhov_lbnd0c(30.0, 0.01),
        0.329_897_116_5,
        1.0e-8,
    );
}

#[test]
fn psy_rh_lbnd0c_round_trips_the_unclamped_forward_formula() {
    for (dry_bulb_c, relative_humidity) in [(-20.0, 0.8), (0.0, 0.2), (20.0, 0.5), (35.0, 0.005)] {
        let vapor_density = energyplus_psy_rhov_fn_tdb_rh_lbnd0c(dry_bulb_c, relative_humidity);
        assert_close(
            energyplus_psy_rh_fn_tdb_rhov_lbnd0c(dry_bulb_c, vapor_density),
            relative_humidity,
            1.0e-15,
        );
    }
}

#[test]
fn psy_rh_lbnd0c_tests_vapor_positivity_before_the_formula() {
    for vapor_density in [f64::NEG_INFINITY, -1.0, -0.0, 0.0, f64::NAN] {
        assert_bits(
            energyplus_psy_rh_fn_tdb_rhov_lbnd0c(f64::NAN, vapor_density),
            0.0,
        );
    }
}

#[test]
fn psy_rh_lbnd0c_only_applies_the_lower_correction_to_negative_raw_rh() {
    let low_positive_rh = 0.005;
    let low_positive_vapor = energyplus_psy_rhov_fn_tdb_rh_lbnd0c(20.0, low_positive_rh);
    let recovered = energyplus_psy_rh_fn_tdb_rhov_lbnd0c(20.0, low_positive_vapor);
    assert!(recovered < 0.01);
    assert_close(recovered, low_positive_rh, 1.0e-15);

    assert_bits(energyplus_psy_rh_fn_tdb_rhov_lbnd0c(-300.0, 0.01), 0.01);
    assert_bits(
        energyplus_psy_rh_fn_tdb_rhov_lbnd0c(f64::NEG_INFINITY, 0.01),
        0.01,
    );
}

#[test]
fn psy_rh_lbnd0c_clamps_only_raw_values_above_one_to_one() {
    let supersaturated_vapor = energyplus_psy_rhov_fn_tdb_rh_lbnd0c(20.0, 1.5);
    assert_bits(
        energyplus_psy_rh_fn_tdb_rhov_lbnd0c(20.0, supersaturated_vapor),
        1.0,
    );
    assert_bits(
        energyplus_psy_rh_fn_tdb_rhov_lbnd0c(20.0, f64::INFINITY),
        1.0,
    );
    assert_bits(
        energyplus_psy_rh_fn_tdb_rhov_lbnd0c(f64::INFINITY, 0.01),
        1.0,
    );
}

#[test]
fn psy_rh_lbnd0c_preserves_raw_temperature_nan_and_zero_kelvin_edges() {
    assert!(energyplus_psy_rh_fn_tdb_rhov_lbnd0c(f64::NAN, 0.01).is_nan());
    assert_bits(
        energyplus_psy_rh_fn_tdb_rhov_lbnd0c(-KELVIN_OFFSET, 0.01),
        0.0,
    );
    assert!(energyplus_psy_rh_fn_tdb_rhov_lbnd0c(-KELVIN_OFFSET, f64::INFINITY).is_nan());

    let just_below_zero_kelvin = f64::from_bits((-KELVIN_OFFSET).to_bits() + 1);
    assert_bits(
        energyplus_psy_rh_fn_tdb_rhov_lbnd0c(just_below_zero_kelvin, f64::from_bits(1)),
        -0.0,
    );
}

#[test]
fn psy_rh_lbnd0c_preserves_the_source_exponential_pole_sides() {
    let low_side = -237.7_f64;
    assert_bits(energyplus_psy_rh_fn_tdb_rhov_lbnd0c(low_side, 0.01), 0.0);
    assert_bits(
        energyplus_psy_rh_fn_tdb_rhov_lbnd0c(low_side.next_up(), 0.01),
        1.0,
    );
}

#[test]
fn psy_rh_lbnd0c_does_not_apply_a_zero_c_temperature_bound() {
    let vapor_density = energyplus_psy_rhov_fn_tdb_rh_lbnd0c(-20.0, 0.8);
    assert_close(
        energyplus_psy_rh_fn_tdb_rhov_lbnd0c(-20.0, vapor_density),
        0.8,
        1.0e-15,
    );
    assert_ne!(
        energyplus_psy_rh_fn_tdb_rhov_lbnd0c(-20.0, vapor_density).to_bits(),
        energyplus_psy_rh_fn_tdb_rhov_lbnd0c(0.0, vapor_density).to_bits(),
    );
}
