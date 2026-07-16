use super::{energyplus_cpcw, energyplus_cphw, energyplus_f6, energyplus_f7};

fn assert_bits(actual: f64, expected: f64) {
    assert_eq!(
        actual.to_bits(),
        expected.to_bits(),
        "actual={actual:?}, expected={expected:?}"
    );
}

#[test]
fn polynomials_match_local_source_coefficient_regressions() {
    // For H=30000 and near-standard pressure, upstream routine 26 adds
    // 17863.7, selects case 4, skips pressure correction, and returns this F6
    // result. The EMS test asserts that enclosing result to 1e-8.
    assert_bits(
        energyplus_f6(
            47_863.7,
            -2.011_47e1,
            9.049_36e-4,
            -6.833_05e-9,
            2.326_1e-14,
            7.272_37e-20,
            -6.319_39e-25,
        ),
        10.318_382_617_164_701,
    );
    // The upstream raw saturation-temperature suite reaches this case-9
    // endpoint with a coarse enclosing tolerance; the exact helper value is a
    // local source-literal regression.
    assert_bits(
        energyplus_f7(
            45_866_000.0,
            7.605_65e11,
            5.805_34e4,
            -7.364_33e-3,
            5.115_31e-10,
            -1.936_19e-17,
            3.705_11e-25,
            -2.773_13e-33,
        ),
        99.012_530_366_052_39,
    );
}

#[test]
fn f6_preserves_horner_order_without_fused_multiply_add() {
    let x = 75_223.0;
    let (a0, a1, a2, a3, a4, a5) = (
        -1.821_24e1,
        8.316_83e-4,
        -6.164_61e-9,
        3.064_11e-14,
        -8.609_64e-20,
        1.030_03e-25,
    );
    let horner = energyplus_f6(x, a0, a1, a2, a3, a4, a5);
    let expanded = a0
        + x * a1
        + (x * x) * a2
        + (x * x * x) * a3
        + (x * x * x * x) * a4
        + (x * x * x * x * x) * a5;
    let fused = x.mul_add(
        x.mul_add(x.mul_add(x.mul_add(x.mul_add(a5, a4), a3), a2), a1),
        a0,
    );

    assert_bits(horner, 20.000_608_480_205_198);
    assert_ne!(horner.to_bits(), expanded.to_bits());
    assert_ne!(horner.to_bits(), fused.to_bits());
}

#[test]
fn f7_preserves_horner_order_and_scales_only_after_the_polynomial() {
    let x = 0.1;
    let horner = energyplus_f7(x, -10.0, -10.0, -10.0, -10.0, -10.0, -10.0, -10.0);
    let expanded = (-10.0
        + x * -10.0
        + (x * x) * -10.0
        + (x * x * x) * -10.0
        + (x * x * x * x) * -10.0
        + (x * x * x * x * x) * -10.0
        + (x * x * x * x * x * x) * -10.0)
        / 1.0e10;

    assert_bits(horner, -1.111_111e-9);
    assert_ne!(horner.to_bits(), expanded.to_bits());

    let source_division = energyplus_f7(0.0, 7.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let incorrectly_multiplied = 7.0_f64 * 1.0e-10;
    assert_bits(source_division, 7.0e-10);
    assert_ne!(source_division.to_bits(), incorrectly_multiplied.to_bits());

    let source_overflow = energyplus_f7(2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, f64::MAX);
    let incorrectly_prescaled = (f64::MAX / 1.0e10) * 64.0;
    assert_bits(source_overflow, f64::INFINITY);
    assert!(incorrectly_prescaled.is_finite());
}

#[test]
fn polynomial_zero_and_nonfinite_edges_follow_source_arithmetic() {
    assert_bits(energyplus_f6(-0.0, -0.0, 0.0, 0.0, 0.0, 0.0, 0.0), -0.0);
    assert_bits(
        energyplus_f7(-0.0, -0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        -0.0,
    );
    assert!(energyplus_f6(f64::NAN, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0).is_nan());
    assert!(energyplus_f7(f64::INFINITY, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0).is_nan());
    // Even X=0 evaluates the nested multiplication, so a nonfinite
    // coefficient is not skipped.
    assert!(energyplus_f6(0.0, 1.0, f64::INFINITY, 0.0, 0.0, 0.0, 0.0).is_nan());
}

#[test]
fn chilled_water_specific_heat_matches_the_upstream_ems_assertion() {
    assert_bits(energyplus_cpcw(30.0), 4_180.0);
}

#[test]
fn hot_water_specific_heat_matches_the_upstream_ems_assertion() {
    assert_bits(energyplus_cphw(60.0), 4_180.0);
}

#[test]
fn water_specific_heat_helpers_ignore_every_temperature_class() {
    let quiet_nan_with_payload = f64::from_bits(0x7ff8_dead_beef_0001);
    for temperature_c in [
        f64::NEG_INFINITY,
        -273.15,
        -0.0,
        0.0,
        100.0,
        f64::INFINITY,
        quiet_nan_with_payload,
    ] {
        assert_bits(energyplus_cpcw(temperature_c), 4_180.0);
        assert_bits(energyplus_cphw(temperature_c), 4_180.0);
    }
}

#[test]
fn repeated_and_alternating_calls_are_output_stable() {
    let f6_first = energyplus_f6(0.1, -10.0, -10.0, -10.0, -10.0, -10.0, -10.0);
    let f7_first = energyplus_f7(0.1, -10.0, -10.0, -10.0, -10.0, -10.0, -10.0, -10.0);

    for _ in 0..16 {
        assert_bits(
            energyplus_f6(0.1, -10.0, -10.0, -10.0, -10.0, -10.0, -10.0),
            f6_first,
        );
        assert_bits(
            energyplus_f7(0.1, -10.0, -10.0, -10.0, -10.0, -10.0, -10.0, -10.0),
            f7_first,
        );
        assert_bits(energyplus_cpcw(f64::NAN), 4_180.0);
        assert_bits(energyplus_cphw(f64::INFINITY), 4_180.0);
    }
}
