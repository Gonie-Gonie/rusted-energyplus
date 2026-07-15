use super::{
    ENERGYPLUS_MIN_HUMIDITY_RATIO, energyplus_psy_rho_air_fn_pb_tdb_w, energyplus_psy_v_fn_tdb_w_pb,
};

fn assert_bits(actual: f64, expected_bits: u64) {
    assert_eq!(actual.to_bits(), expected_bits, "actual={actual:?}");
}

#[test]
fn psy_v_matches_pinned_energyplus_source_vectors() {
    // The first vector is used by EnergyPlus 26.1's functional API test. The
    // remaining bits pin the local transcription and source evaluation order;
    // they are not output captured from an external oracle run.
    let cases = [
        (24.0, 0.009, 101_325.0, 0x3feb_5849_ac3f_ac48),
        (20.0, 0.008, 101_325.0, 0x3fea_ef2b_11cc_dda0),
        (25.0, 0.01, 82_000.0, 0x3ff0_fa72_f554_a174),
        (-20.0, 0.008, 101_325.0, 0x3fe7_42eb_03bc_90db),
    ];

    for (dry_bulb_c, humidity_ratio, pressure_pa, expected_bits) in cases {
        assert_bits(
            energyplus_psy_v_fn_tdb_w_pb(dry_bulb_c, humidity_ratio, pressure_pa),
            expected_bits,
        );
    }

    let ems_fixture = energyplus_psy_v_fn_tdb_w_pb(30.0, 0.01, 101_325.0);
    assert!((ems_fixture - 0.873_152_783).abs() <= 1.0e-8);
}

#[test]
fn psy_v_applies_the_source_humidity_floor() {
    let at_floor = energyplus_psy_v_fn_tdb_w_pb(20.0, ENERGYPLUS_MIN_HUMIDITY_RATIO, 101_325.0);

    for humidity_ratio in [
        f64::NEG_INFINITY,
        -1.0,
        -f64::MIN_POSITIVE,
        -0.0,
        0.0,
        f64::from_bits(1),
        f64::from_bits(ENERGYPLUS_MIN_HUMIDITY_RATIO.to_bits() - 1),
        ENERGYPLUS_MIN_HUMIDITY_RATIO,
    ] {
        assert_bits(
            energyplus_psy_v_fn_tdb_w_pb(20.0, humidity_ratio, 101_325.0),
            at_floor.to_bits(),
        );
    }

    assert_bits(at_floor, 0x3fea_97b7_2da9_a504);
}

#[test]
fn psy_v_preserves_a_first_argument_nan_through_source_max_semantics() {
    let humidity_nan = f64::from_bits(0x7ff8_0000_0000_1234);
    assert!(energyplus_psy_v_fn_tdb_w_pb(20.0, humidity_nan, 101_325.0).is_nan());
    assert!(energyplus_psy_v_fn_tdb_w_pb(f64::NAN, 0.008, 101_325.0).is_nan());
    assert!(energyplus_psy_v_fn_tdb_w_pb(20.0, 0.008, f64::NAN).is_nan());
}

#[test]
fn psy_v_resets_every_strictly_negative_raw_volume_to_literal_point_83() {
    let fallback_bits = 0.83_f64.to_bits();

    for (dry_bulb_c, pressure_pa) in [
        (20.0, -101_325.0),
        (20.0, -f64::MAX),
        (-300.0, 101_325.0),
        (f64::NEG_INFINITY, 101_325.0),
    ] {
        assert_bits(
            energyplus_psy_v_fn_tdb_w_pb(dry_bulb_c, 0.008, pressure_pa),
            fallback_bits,
        );
    }
}

#[test]
fn psy_v_preserves_ordered_zero_and_infinity_division_behavior() {
    assert_eq!(
        energyplus_psy_v_fn_tdb_w_pb(20.0, 0.008, 0.0),
        f64::INFINITY
    );
    assert_bits(
        energyplus_psy_v_fn_tdb_w_pb(20.0, 0.008, -0.0),
        0.83_f64.to_bits(),
    );
    assert_bits(
        energyplus_psy_v_fn_tdb_w_pb(20.0, 0.008, f64::INFINITY),
        0.0_f64.to_bits(),
    );
    assert_bits(
        energyplus_psy_v_fn_tdb_w_pb(20.0, 0.008, f64::NEG_INFINITY),
        (-0.0_f64).to_bits(),
    );
    assert_eq!(
        energyplus_psy_v_fn_tdb_w_pb(f64::INFINITY, 0.008, 101_325.0),
        f64::INFINITY
    );

    assert_bits(
        energyplus_psy_v_fn_tdb_w_pb(-273.333_333_333_333_3, 0.008, 101_325.0),
        0.0_f64.to_bits(),
    );
}

#[test]
fn psy_v_tracks_density_reciprocals_only_with_source_constant_tolerance() {
    // EnergyPlus intentionally uses different legacy constants in its density
    // and specific-volume routines, so their product is close to, not exactly,
    // one across the ordinary source range.
    for (dry_bulb_c, humidity_ratio, pressure_pa) in [
        (-100.0, 0.03, 101_325.0),
        (-20.0, 0.008, 101_325.0),
        (24.0, 0.009, 101_325.0),
        (50.0, 0.02, 90_000.0),
        (200.0, 0.03, 101_325.0),
    ] {
        let volume = energyplus_psy_v_fn_tdb_w_pb(dry_bulb_c, humidity_ratio, pressure_pa);
        let density = energyplus_psy_rho_air_fn_pb_tdb_w(pressure_pa, dry_bulb_c, humidity_ratio);
        assert!(
            (volume * density - 1.0).abs() <= 0.002,
            "TDB={dry_bulb_c:?}, W={humidity_ratio:?}, PB={pressure_pa:?}, V={volume:?}, rho={density:?}"
        );
    }
}

#[test]
fn psy_v_is_stable_across_repeated_and_alternating_calls() {
    let cases = [
        (24.0, 0.009, 101_325.0, 0x3feb_5849_ac3f_ac48),
        (20.0, 0.0, 101_325.0, 0x3fea_97b7_2da9_a504),
        (20.0, 0.008, f64::NEG_INFINITY, (-0.0_f64).to_bits()),
        (20.0, 0.008, -101_325.0, 0.83_f64.to_bits()),
    ];

    for _ in 0..32 {
        for (dry_bulb_c, humidity_ratio, pressure_pa, expected_bits) in cases {
            assert_bits(
                energyplus_psy_v_fn_tdb_w_pb(dry_bulb_c, humidity_ratio, pressure_pa),
                expected_bits,
            );
        }
    }
}
