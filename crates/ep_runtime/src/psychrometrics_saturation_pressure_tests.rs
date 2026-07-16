use super::{
    energyplus_psy_psat_fn_temp_raw, energyplus_psychrometric_psat_cache_temperature_c,
    energyplus_psychrometric_saturation_pressure_pa,
};

fn assert_bits(actual: f64, expected_bits: u64) {
    assert_eq!(actual.to_bits(), expected_bits, "actual={actual:?}");
}

#[test]
fn psy_psat_raw_matches_pinned_default_source_formula_vectors() {
    // EnergyPlus 26.1's EMS fixture directly exercises 30 C. All exact bits
    // pin the local transcription; they are not captured from an external run.
    let cases = [
        (-100.0, 0x3f57_056c_eada_64b6),
        (-50.0, 0x400f_830a_e636_e685),
        (-0.0001, 0x4083_1930_34d3_ecf5),
        (0.0, 0x4083_193a_8360_0ba7),
        (0.0001, 0x4083_1944_d1f1_3a6b),
        (0.01, 0x4083_1d41_9600_fbea),
        (0.010_001, 0x4083_1d41_af27_f68d),
        (20.0, 0x40a2_459b_7e93_7b04),
        (25.0, 0x40a8_c26e_d52c_b6c9),
        (30.0, 0x40b0_9607_be0b_4926),
        (100.0, 0x40f8_c2ab_7820_a0e7),
        (200.0, 0x4137_ba81_bee2_03d6),
    ];

    for (temperature_c, expected_bits) in cases {
        assert_bits(
            energyplus_psy_psat_fn_temp_raw(temperature_c),
            expected_bits,
        );
    }

    let ems_fixture = energyplus_psy_psat_fn_temp_raw(30.0);
    assert!((ems_fixture - 4_246.030_243_592).abs() <= 1.0e-8);
}

#[test]
fn psy_psat_raw_preserves_the_source_triple_point_branch_boundary() {
    let ice_side = energyplus_psy_psat_fn_temp_raw(0.009_999);
    let source_rounded_triple_point = energyplus_psy_psat_fn_temp_raw(0.01);
    let water_side = energyplus_psy_psat_fn_temp_raw(0.010_001);

    assert_bits(ice_side, 0x4083_1d41_7b99_369e);
    assert_bits(source_rounded_triple_point, 0x4083_1d41_9600_fbea);
    assert_bits(water_side, 0x4083_1d41_af27_f68d);
    assert!(ice_side < source_rounded_triple_point);
    assert!(source_rounded_triple_point < water_side);

    let below_zero = energyplus_psy_psat_fn_temp_raw(-0.0001);
    let above_zero = energyplus_psy_psat_fn_temp_raw(0.0001);
    assert!((above_zero - below_zero).abs() < 0.02);
}

#[test]
fn psy_psat_raw_applies_source_range_clamps_and_ordered_nan_behavior() {
    let lower_bits = 0.001_405_102_123_874_164_f64.to_bits();
    let upper_bits = 1_555_073.745_636_215_f64.to_bits();

    for temperature_c in [f64::NEG_INFINITY, -f64::MAX, -101.0, -100.0] {
        assert_bits(energyplus_psy_psat_fn_temp_raw(temperature_c), lower_bits);
    }
    for temperature_c in [
        200.0_f64.next_up(),
        201.0,
        f64::MAX,
        f64::INFINITY,
        f64::NAN,
    ] {
        assert_bits(energyplus_psy_psat_fn_temp_raw(temperature_c), upper_bits);
    }
}

#[test]
fn psy_psat_raw_treats_signed_zero_identically() {
    assert_bits(
        energyplus_psy_psat_fn_temp_raw(-0.0),
        energyplus_psy_psat_fn_temp_raw(0.0).to_bits(),
    );
}

#[test]
fn guarded_adapter_quantizes_finite_inputs_before_the_raw_delegate() {
    for temperature_c in [
        (-100.0_f64).next_up(),
        -0.0001,
        0.0001,
        0.01,
        20.123_456_789,
        199.999_999,
    ] {
        let representative = energyplus_psychrometric_psat_cache_temperature_c(temperature_c);
        let expected = energyplus_psy_psat_fn_temp_raw(representative);
        let actual = energyplus_psychrometric_saturation_pressure_pa(temperature_c)
            .expect("finite compatibility input");
        assert_bits(actual, expected.to_bits());
    }

    assert_bits(
        energyplus_psychrometric_psat_cache_temperature_c(-0.0001),
        0xbf1a_36e2_e000_0000,
    );
    assert_bits(
        energyplus_psychrometric_psat_cache_temperature_c(0.0001),
        0x3f1a_36e2_e000_0000,
    );
}

#[test]
fn guarded_adapter_retains_its_distinct_nonfinite_contract() {
    for temperature_c in [f64::NEG_INFINITY, f64::INFINITY, f64::NAN] {
        assert!(energyplus_psychrometric_saturation_pressure_pa(temperature_c).is_none());
    }

    assert_bits(
        energyplus_psy_psat_fn_temp_raw(f64::NEG_INFINITY),
        0.001_405_102_123_874_164_f64.to_bits(),
    );
    assert_bits(
        energyplus_psy_psat_fn_temp_raw(f64::INFINITY),
        1_555_073.745_636_215_f64.to_bits(),
    );
    assert_bits(
        energyplus_psy_psat_fn_temp_raw(f64::NAN),
        1_555_073.745_636_215_f64.to_bits(),
    );
}

#[test]
fn psy_psat_raw_is_stable_across_repeated_and_alternating_calls() {
    let cases = [
        (-50.0, 0x400f_830a_e636_e685),
        (-0.0001, 0x4083_1930_34d3_ecf5),
        (0.010_001, 0x4083_1d41_af27_f68d),
        (25.0, 0x40a8_c26e_d52c_b6c9),
        (200.0, 0x4137_ba81_bee2_03d6),
    ];

    for _ in 0..32 {
        for (temperature_c, expected_bits) in cases {
            assert_bits(
                energyplus_psy_psat_fn_temp_raw(temperature_c),
                expected_bits,
            );
        }
    }
}
