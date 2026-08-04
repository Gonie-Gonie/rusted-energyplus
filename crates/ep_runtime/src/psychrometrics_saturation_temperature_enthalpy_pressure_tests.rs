use super::energyplus_psy_tsat_fn_h_pb_raw;

fn assert_close(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "actual={actual:?}, expected={expected:?}, tolerance={tolerance:?}",
    );
}

#[test]
fn upstream_v26_1_raw_vectors_cover_all_nine_seed_regions_and_clamps() {
    let pressure = 101_325.0;
    for (enthalpy, expected, tolerance) in [
        (75_223.0 - 17_863.7, 20.0, 0.001),
        (27_298.0 - 17_863.7, 0.0, 0.001),
        (-670.11 - 17_863.7, -20.0, 0.001),
        (-22_137.9 - 17_863.7, -40.0, 0.001),
        (-42_399.0 - 17_863.7, -60.0, 0.1),
        (-52_399.0 - 17_863.7, -60.0, 0.1),
        (183_790.0 - 17_863.7, 40.0, 0.001),
        (475_770.0 - 17_863.7, 60.0, 0.001),
        (1_544_500.0 - 17_863.7, 80.0, 0.001),
        (3_835_300.0 - 17_863.7, 90.0, 0.001),
        (45_866_000.0 - 17_863.7, 100.0, 1.0),
    ] {
        assert_close(
            energyplus_psy_tsat_fn_h_pb_raw(enthalpy, pressure),
            expected,
            tolerance,
        );
    }
}

#[test]
fn upstream_v26_1_pressure_correction_vector_matches() {
    let enthalpy = 75_223.0 - 17_863.7;
    assert_close(
        energyplus_psy_tsat_fn_h_pb_raw(enthalpy, 91_325.0),
        18.819,
        0.001,
    );
}

#[test]
fn near_standard_pressure_skips_correction_and_is_repeatable() {
    let enthalpy = 30_000.0;
    let expected: f64 = 10.318_382_617_164_701;
    assert_eq!(
        energyplus_psy_tsat_fn_h_pb_raw(enthalpy, 101_325.0).to_bits(),
        expected.to_bits(),
    );
    assert_eq!(
        energyplus_psy_tsat_fn_h_pb_raw(enthalpy, 101_325.0).to_bits(),
        energyplus_psy_tsat_fn_h_pb_raw(enthalpy, 101_325.0).to_bits(),
    );
}

#[test]
fn representable_one_percent_inside_band_and_nan_use_the_uncorrected_seed() {
    let enthalpy = 30_000.0;
    let seed = energyplus_psy_tsat_fn_h_pb_raw(enthalpy, 101_330.0);
    let upper_threshold: f64 = 101_330.0 + 0.01 * 101_330.0;
    let lower_threshold: f64 = 101_330.0 - 0.01 * 101_330.0;
    let upper_inside = f64::from_bits(upper_threshold.to_bits() - 1);
    let lower_inside = f64::from_bits(lower_threshold.to_bits() + 1);

    assert!(
        (upper_inside - 101_330.0).abs() / 101_330.0 <= 0.01,
        "test value must exercise the source's ordered false branch",
    );
    assert!(
        (lower_inside - 101_330.0).abs() / 101_330.0 <= 0.01,
        "test value must exercise the source's ordered false branch",
    );
    assert_eq!(
        energyplus_psy_tsat_fn_h_pb_raw(enthalpy, upper_inside).to_bits(),
        seed.to_bits(),
    );
    assert_eq!(
        energyplus_psy_tsat_fn_h_pb_raw(enthalpy, lower_inside).to_bits(),
        seed.to_bits(),
    );
    assert_eq!(
        energyplus_psy_tsat_fn_h_pb_raw(enthalpy, f64::NAN).to_bits(),
        seed.to_bits(),
    );
}

#[test]
fn unordered_enthalpy_propagates_through_the_source_ordered_floor_and_seed() {
    assert!(energyplus_psy_tsat_fn_h_pb_raw(f64::NAN, 101_330.0).is_nan());
}
