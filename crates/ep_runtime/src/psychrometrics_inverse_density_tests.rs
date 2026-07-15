use super::{
    ENERGYPLUS_MIN_HUMIDITY_RATIO, KELVIN_OFFSET, energyplus_psy_h_fn_tdb_w,
    energyplus_psy_rhov_fn_tdb_rh_lbnd0c, energyplus_psy_rhov_fn_tdb_w_pb,
    energyplus_psy_rhov_fn_tdb_w_pb_fast, energyplus_psy_tdb_fn_h_w,
};

fn assert_bits(actual: f64, expected_bits: u64) {
    assert_eq!(actual.to_bits(), expected_bits, "actual={actual:?}");
}

fn assert_close(actual: f64, expected: f64, absolute_tolerance: f64) {
    assert!(
        (actual - expected).abs() <= absolute_tolerance,
        "actual={actual:?}, expected={expected:?}, tolerance={absolute_tolerance:?}"
    );
}

#[test]
fn psy_tdb_matches_pinned_source_formula_vectors_bitwise() {
    // The first vector is the exact input pair documented by the pinned
    // DXCoils.unit.cc saturation check. These local bits pin the Rust source
    // transcription; they are not output captured from an external run.
    let cases = [
        (
            38_853.039_955_973_93,
            0.009_870_370_393_138_59,
            0x402b_b189_3b18_2edd,
        ),
        (44_480.598_4, 0.008, 0x4038_0000_0000_0000),
        (24_141.615_548, 0.0, 0x4038_0000_0000_0001),
        (0.0, 0.0, 0xbf99_7c5d_6ce0_c19c),
    ];

    for (enthalpy_j_per_kg, humidity_ratio, expected_bits) in cases {
        assert_bits(
            energyplus_psy_tdb_fn_h_w(enthalpy_j_per_kg, humidity_ratio),
            expected_bits,
        );
    }
}

#[test]
fn psy_tdb_applies_the_source_humidity_floor_and_nan_semantics() {
    let at_floor = energyplus_psy_tdb_fn_h_w(40_000.0, ENERGYPLUS_MIN_HUMIDITY_RATIO);
    let below_floor = f64::from_bits(ENERGYPLUS_MIN_HUMIDITY_RATIO.to_bits() - 1);

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
            energyplus_psy_tdb_fn_h_w(40_000.0, humidity_ratio),
            at_floor.to_bits(),
        );
    }

    assert!(energyplus_psy_tdb_fn_h_w(40_000.0, f64::NAN).is_nan());
    assert!(energyplus_psy_tdb_fn_h_w(f64::NAN, 0.008).is_nan());
    assert_eq!(
        energyplus_psy_tdb_fn_h_w(f64::INFINITY, 0.008),
        f64::INFINITY
    );
    assert_eq!(
        energyplus_psy_tdb_fn_h_w(f64::NEG_INFINITY, 0.008),
        f64::NEG_INFINITY
    );
    assert!(energyplus_psy_tdb_fn_h_w(40_000.0, f64::INFINITY).is_nan());
}

#[test]
fn psy_h_and_tdb_round_trip_with_source_evaluation_rounding() {
    let cases = [
        (24.0, 0.008, 0x4038_0000_0000_0000),
        (24.0, 0.0, 0x4038_0000_0000_0001),
        (15.0, 0.005, 0x402e_0000_0000_0000),
        (-20.0, 0.008, 0xc033_ffff_ffff_ffff),
    ];

    for (dry_bulb_c, humidity_ratio, expected_round_trip_bits) in cases {
        let enthalpy_j_per_kg = energyplus_psy_h_fn_tdb_w(dry_bulb_c, humidity_ratio);
        assert_bits(
            energyplus_psy_tdb_fn_h_w(enthalpy_j_per_kg, humidity_ratio),
            expected_round_trip_bits,
        );
    }
}

#[test]
fn psy_rhov_from_rh_matches_pinned_source_formula_vectors() {
    // std::exp is allowed platform-level last-bit variation, so these pinned
    // source-formula checks use a tight absolute tolerance rather than bits.
    let cases = [
        (0.0, 1.0, 0.004_843_314_228_479_12),
        (20.0, 0.5, 0.008_636_869_194_982_5),
        (-20.0, 0.8, 0.000_853_524_624_501_83),
    ];

    for (dry_bulb_c, relative_humidity, expected_density) in cases {
        assert_close(
            energyplus_psy_rhov_fn_tdb_rh_lbnd0c(dry_bulb_c, relative_humidity),
            expected_density,
            1.0e-15,
        );
    }
}

#[test]
fn psy_rhov_from_rh_preserves_unclamped_inputs_and_ieee_edges() {
    let at_zero_c = energyplus_psy_rhov_fn_tdb_rh_lbnd0c(0.0, 0.8);
    let below_zero_c = energyplus_psy_rhov_fn_tdb_rh_lbnd0c(-20.0, 0.8);
    assert_ne!(below_zero_c.to_bits(), at_zero_c.to_bits());

    let half_rh = energyplus_psy_rhov_fn_tdb_rh_lbnd0c(20.0, 0.5);
    assert_bits(
        energyplus_psy_rhov_fn_tdb_rh_lbnd0c(20.0, 1.0),
        (2.0 * half_rh).to_bits(),
    );
    assert!(energyplus_psy_rhov_fn_tdb_rh_lbnd0c(20.0, -0.5) < 0.0);
    assert!(
        energyplus_psy_rhov_fn_tdb_rh_lbnd0c(20.0, 1.5)
            > energyplus_psy_rhov_fn_tdb_rh_lbnd0c(20.0, 1.0)
    );
    assert_bits(
        energyplus_psy_rhov_fn_tdb_rh_lbnd0c(20.0, -0.0),
        (-0.0_f64).to_bits(),
    );
    assert_bits(
        energyplus_psy_rhov_fn_tdb_rh_lbnd0c(f64::INFINITY, 0.5),
        0.0_f64.to_bits(),
    );
    assert_bits(
        energyplus_psy_rhov_fn_tdb_rh_lbnd0c(f64::NEG_INFINITY, 0.5),
        (-0.0_f64).to_bits(),
    );
    assert!(energyplus_psy_rhov_fn_tdb_rh_lbnd0c(-KELVIN_OFFSET, 0.5).is_infinite());
    assert!(energyplus_psy_rhov_fn_tdb_rh_lbnd0c(f64::NAN, 0.5).is_nan());
    assert!(energyplus_psy_rhov_fn_tdb_rh_lbnd0c(20.0, f64::NAN).is_nan());
}

#[test]
fn psy_rhov_from_w_matches_pinned_source_formula_vectors_bitwise() {
    // These bits pin the local transcription of the rational source formula;
    // they are not output captured from an external EnergyPlus run.
    let cases = [
        (20.0, 0.008, 101_325.0, 0x3f83_7a32_215d_cd6a),
        (25.0, 0.01, 82_000.0, 0x3f83_4fbb_e7ce_bfd9),
        (20.0, 0.0, 101_325.0, 0x3ee9_4051_34f0_2f6e),
        (-20.0, 0.008, 101_325.0, 0x3f86_8e0f_6054_b531),
    ];

    for (dry_bulb_c, humidity_ratio, pressure_pa, expected_bits) in cases {
        assert_bits(
            energyplus_psy_rhov_fn_tdb_w_pb(dry_bulb_c, humidity_ratio, pressure_pa),
            expected_bits,
        );
    }
}

#[test]
fn psy_rhov_from_w_applies_the_source_humidity_floor_and_nan_semantics() {
    let at_floor = energyplus_psy_rhov_fn_tdb_w_pb(20.0, ENERGYPLUS_MIN_HUMIDITY_RATIO, 101_325.0);
    let below_floor = f64::from_bits(ENERGYPLUS_MIN_HUMIDITY_RATIO.to_bits() - 1);

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
            energyplus_psy_rhov_fn_tdb_w_pb(20.0, humidity_ratio, 101_325.0),
            at_floor.to_bits(),
        );
    }

    assert!(energyplus_psy_rhov_fn_tdb_w_pb(20.0, f64::NAN, 101_325.0).is_nan());
    assert!(energyplus_psy_rhov_fn_tdb_w_pb(f64::NAN, 0.008, 101_325.0).is_nan());
    assert!(energyplus_psy_rhov_fn_tdb_w_pb(20.0, 0.008, f64::NAN).is_nan());
    assert!(energyplus_psy_rhov_fn_tdb_w_pb(20.0, f64::INFINITY, 101_325.0).is_nan());
}

#[test]
fn ordinary_and_fast_rhov_from_w_are_bit_equal_on_the_fast_domain() {
    for humidity_ratio in [
        ENERGYPLUS_MIN_HUMIDITY_RATIO,
        f64::from_bits(ENERGYPLUS_MIN_HUMIDITY_RATIO.to_bits() + 1),
        0.008,
        0.02,
    ] {
        assert_bits(
            energyplus_psy_rhov_fn_tdb_w_pb_fast(20.0, humidity_ratio, 101_325.0),
            energyplus_psy_rhov_fn_tdb_w_pb(20.0, humidity_ratio, 101_325.0).to_bits(),
        );
    }

    assert!(energyplus_psy_rhov_fn_tdb_w_pb(20.0, f64::INFINITY, 101_325.0).is_nan());
    assert!(energyplus_psy_rhov_fn_tdb_w_pb_fast(20.0, f64::INFINITY, 101_325.0).is_nan());
}

#[test]
fn psy_rhov_from_w_preserves_pressure_temperature_and_signed_zero_edges() {
    assert_bits(
        energyplus_psy_rhov_fn_tdb_w_pb(20.0, 0.008, 0.0),
        0.0_f64.to_bits(),
    );
    assert_bits(
        energyplus_psy_rhov_fn_tdb_w_pb(20.0, 0.008, -0.0),
        (-0.0_f64).to_bits(),
    );
    assert!(energyplus_psy_rhov_fn_tdb_w_pb(20.0, 0.008, -101_325.0) < 0.0);
    assert!(energyplus_psy_rhov_fn_tdb_w_pb(-KELVIN_OFFSET, 0.008, 101_325.0).is_infinite());
    assert_bits(
        energyplus_psy_rhov_fn_tdb_w_pb(f64::INFINITY, 0.008, 101_325.0),
        0.0_f64.to_bits(),
    );
    assert_bits(
        energyplus_psy_rhov_fn_tdb_w_pb(f64::NEG_INFINITY, 0.008, 101_325.0),
        (-0.0_f64).to_bits(),
    );
}

#[cfg(debug_assertions)]
#[test]
fn psy_rhov_fast_debug_asserts_the_source_humidity_precondition() {
    let below_floor = f64::from_bits(ENERGYPLUS_MIN_HUMIDITY_RATIO.to_bits() - 1);
    for humidity_ratio in [below_floor, 0.0, -1.0, f64::NAN] {
        assert!(
            std::panic::catch_unwind(|| {
                energyplus_psy_rhov_fn_tdb_w_pb_fast(20.0, humidity_ratio, 101_325.0)
            })
            .is_err()
        );
    }
}

#[cfg(not(debug_assertions))]
#[test]
fn psy_rhov_fast_does_not_floor_humidity_in_release_builds() {
    let humidity_ratio = 0.0;
    assert_bits(
        energyplus_psy_rhov_fn_tdb_w_pb_fast(20.0, humidity_ratio, 101_325.0),
        0.0_f64.to_bits(),
    );
    assert_ne!(
        energyplus_psy_rhov_fn_tdb_w_pb_fast(20.0, humidity_ratio, 101_325.0).to_bits(),
        energyplus_psy_rhov_fn_tdb_w_pb(20.0, humidity_ratio, 101_325.0).to_bits(),
    );
}
