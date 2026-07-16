use super::{
    energyplus_c_splineint,
    spline_tables::{
        ENERGYPLUS_TSAT_FN_PB_D2Y, ENERGYPLUS_TSAT_FN_PB_Y, ENERGYPLUS_TSAT_SPLINE_SAMPLE_COUNT,
    },
};

fn assert_bits(actual: f64, expected: f64) {
    assert_eq!(
        actual.to_bits(),
        expected.to_bits(),
        "actual={actual:?}, expected={expected:?}"
    );
}

fn fold_table(values: &[f64], mut hash: u64) -> u64 {
    for value in values {
        hash = (hash ^ value.to_bits()).wrapping_mul(0x100_0000_01b3);
    }
    hash
}

#[test]
fn immutable_source_tables_have_exact_length_and_bit_folds() {
    const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;

    assert_eq!(ENERGYPLUS_TSAT_FN_PB_Y.len(), 1_651);
    assert_eq!(ENERGYPLUS_TSAT_FN_PB_D2Y.len(), 1_651);
    assert_eq!(ENERGYPLUS_TSAT_SPLINE_SAMPLE_COUNT, 1_651);
    assert_eq!(
        fold_table(&ENERGYPLUS_TSAT_FN_PB_Y, FNV_OFFSET),
        0x5bcb_9a0f_e110_0db3
    );
    assert_eq!(
        fold_table(&ENERGYPLUS_TSAT_FN_PB_D2Y, FNV_OFFSET),
        0xbea1_3706_7252_03b6
    );
    assert_eq!(
        fold_table(
            &ENERGYPLUS_TSAT_FN_PB_D2Y,
            fold_table(&ENERGYPLUS_TSAT_FN_PB_Y, FNV_OFFSET),
        ),
        0x76d9_bfc7_53b3_fda0
    );
}

#[test]
fn spline_matches_source_knots_and_between_knot_vectors() {
    for (x, expected) in [
        (0.0, -100.0),
        (64.0, -24.888_128_36),
        (96.0, -22.468_871_203_027_334),
        (128.0, -17.741_971_21),
        (50_000.0, 81.318_278_578_999_98),
        (101_325.0, 99.974_100_598_95),
        (105_600.0, 101.135_929_3),
    ] {
        assert_bits(energyplus_c_splineint(1_651, x), expected);
    }
}

#[test]
fn shifted_bin_rule_preserves_the_source_discontinuity() {
    assert_bits(
        energyplus_c_splineint(1_651, 127.999),
        -74.703_938_874_723_95,
    );
    assert_bits(energyplus_c_splineint(1_651, 128.0), -17.741_971_21);
}

#[test]
fn endpoint_bins_extrapolate_without_clamping_x() {
    assert_bits(energyplus_c_splineint(1_651, -1.0), -101.165_878_715_115_64);
    assert_bits(
        energyplus_c_splineint(1_651, 105_664.0),
        101.153_043_159_200_01,
    );
    assert_bits(
        energyplus_c_splineint(1_651, 120_000.0),
        169.027_179_952_809_43,
    );
}

#[test]
fn sample_count_clamps_the_selected_table_prefix() {
    assert_bits(energyplus_c_splineint(2, 96.0), -22.468_871_203_027_334);
    assert_bits(energyplus_c_splineint(3, 160.0), -12.770_119_898_720_242);
}

#[test]
fn undefined_cpp_input_domain_is_rejected_before_indexing() {
    for (sample_data_size, x) in [
        (1, 0.0),
        (1_652, 0.0),
        (1_651, f64::NAN),
        (1_651, f64::INFINITY),
        (1_651, 2_147_483_648.0),
        (1_651, -2_147_483_649.0),
    ] {
        assert!(
            std::panic::catch_unwind(|| energyplus_c_splineint(sample_data_size, x)).is_err(),
            "sample_data_size={sample_data_size}, x={x:?}"
        );
    }
}

#[test]
fn repeated_and_alternating_calls_are_output_stable() {
    let first = energyplus_c_splineint(1_651, 101_325.0);
    let second = energyplus_c_splineint(1_651, 96.0);

    for _ in 0..16 {
        assert_bits(energyplus_c_splineint(1_651, 101_325.0), first);
        assert_bits(energyplus_c_splineint(1_651, 96.0), second);
    }
}
