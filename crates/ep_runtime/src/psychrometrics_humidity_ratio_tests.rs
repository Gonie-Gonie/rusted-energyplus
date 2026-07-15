use super::{ENERGYPLUS_MIN_HUMIDITY_RATIO, energyplus_psy_h_fn_tdb_w, energyplus_psy_w_fn_tdb_h};

fn assert_bits(actual: f64, expected_bits: u64) {
    assert_eq!(actual.to_bits(), expected_bits, "actual={actual:?}");
}

#[test]
fn psy_w_tdb_h_matches_pinned_energyplus_source_vectors() {
    // The first vector is used by EnergyPlus 26.1's EMS unit fixture. The
    // remaining bits pin the local transcription and source evaluation order;
    // they are not output captured from an external oracle run.
    let cases = [
        (20.0, 30_000.0, 0x3f6f_f6a4_012f_81be),
        (24.0, 48_000.0, 0x3f83_372a_e815_90fe),
        (22.0, 36_000.0, 0x3f76_6373_40c9_b9bb),
        (10.0, 18_000.0, 0x3f69_da95_1e5c_de68),
        (30.0, 53_000.0, 0x3f82_4eb0_5ca5_362f),
        (-20.0, 5_000.0, 0x3f84_dc98_772d_12d8),
    ];

    for (dry_bulb_c, enthalpy_j_per_kg, expected_bits) in cases {
        assert_bits(
            energyplus_psy_w_fn_tdb_h(dry_bulb_c, enthalpy_j_per_kg),
            expected_bits,
        );
    }

    let ems_fixture = energyplus_psy_w_fn_tdb_h(20.0, 30_000.0);
    assert!((ems_fixture - 0.003_901_787_11).abs() <= 1.0e-8);
}

#[test]
fn psy_w_tdb_h_round_trips_the_canonical_enthalpy_formula() {
    for (dry_bulb_c, humidity_ratio) in [
        (24.0, 0.003),
        (20.0, 0.003),
        (10.0, 0.003),
        (-20.0, 0.008),
        (35.0, 0.02),
    ] {
        let enthalpy_j_per_kg = energyplus_psy_h_fn_tdb_w(dry_bulb_c, humidity_ratio);
        let recovered = energyplus_psy_w_fn_tdb_h(dry_bulb_c, enthalpy_j_per_kg);
        assert!(
            (recovered - humidity_ratio).abs() <= 1.0e-17,
            "TDB={dry_bulb_c:?}, W={humidity_ratio:?}, H={enthalpy_j_per_kg:?}, recovered={recovered:?}"
        );
    }
}

#[test]
fn psy_w_tdb_h_preserves_positive_values_below_the_return_floor() {
    let raw_positive_humidity_ratio = 5.0e-6;
    let dry_bulb_c = 20.0;
    let enthalpy_j_per_kg = 1.004_84e3 * dry_bulb_c
        + raw_positive_humidity_ratio * (2.500_94e6 + 1.858_95e3 * dry_bulb_c);
    let recovered = energyplus_psy_w_fn_tdb_h(dry_bulb_c, enthalpy_j_per_kg);

    assert!(recovered > 0.0);
    assert!(recovered < ENERGYPLUS_MIN_HUMIDITY_RATIO);
    assert!((recovered - raw_positive_humidity_ratio).abs() <= 1.0e-17);
}

#[test]
fn psy_w_tdb_h_floors_every_strictly_negative_raw_result() {
    let floor_bits = ENERGYPLUS_MIN_HUMIDITY_RATIO.to_bits();

    for (dry_bulb_c, enthalpy_j_per_kg) in [
        (20.0, 0.0),
        (0.0, -f64::MIN_POSITIVE),
        (0.0, -250.094),
        (0.0, -250.094_000_000_000_02),
        (20.0, f64::NEG_INFINITY),
    ] {
        assert_bits(
            energyplus_psy_w_fn_tdb_h(dry_bulb_c, enthalpy_j_per_kg),
            floor_bits,
        );
    }
}

#[test]
fn psy_w_tdb_h_preserves_ordered_zero_nan_and_infinity_behavior() {
    assert_bits(energyplus_psy_w_fn_tdb_h(0.0, -0.0), (-0.0_f64).to_bits());
    assert_bits(energyplus_psy_w_fn_tdb_h(0.0, 0.0), 0.0_f64.to_bits());
    assert!(energyplus_psy_w_fn_tdb_h(20.0, f64::NAN).is_nan());
    assert!(energyplus_psy_w_fn_tdb_h(f64::NAN, 30_000.0).is_nan());
    assert_eq!(
        energyplus_psy_w_fn_tdb_h(20.0, f64::INFINITY),
        f64::INFINITY
    );
    assert!(energyplus_psy_w_fn_tdb_h(f64::INFINITY, 30_000.0).is_nan());
    assert!(energyplus_psy_w_fn_tdb_h(f64::NEG_INFINITY, 30_000.0).is_nan());
}

#[test]
fn psy_w_tdb_h_preserves_the_source_denominator_pole() {
    let pole_c = -2_500_940.0 / 1_858.95;
    assert_eq!(energyplus_psy_w_fn_tdb_h(pole_c, 0.0), f64::INFINITY);
    assert_bits(
        energyplus_psy_w_fn_tdb_h(pole_c.next_down(), 0.0),
        ENERGYPLUS_MIN_HUMIDITY_RATIO.to_bits(),
    );
    assert!(energyplus_psy_w_fn_tdb_h(pole_c.next_up(), 0.0).is_finite());
    assert!(energyplus_psy_w_fn_tdb_h(pole_c.next_up(), 0.0) > 0.0);
}

#[test]
fn psy_w_tdb_h_is_stable_across_repeated_and_alternating_calls() {
    let cases = [
        (20.0, 30_000.0, 0x3f6f_f6a4_012f_81be),
        (22.0, 36_000.0, 0x3f76_6373_40c9_b9bb),
        (0.0, -0.0, (-0.0_f64).to_bits()),
        (
            20.0,
            f64::NEG_INFINITY,
            ENERGYPLUS_MIN_HUMIDITY_RATIO.to_bits(),
        ),
    ];

    for _ in 0..32 {
        for (dry_bulb_c, enthalpy_j_per_kg, expected_bits) in cases {
            assert_bits(
                energyplus_psy_w_fn_tdb_h(dry_bulb_c, enthalpy_j_per_kg),
                expected_bits,
            );
        }
    }
}
