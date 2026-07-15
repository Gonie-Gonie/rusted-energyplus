use super::{
    ENERGYPLUS_MIN_HUMIDITY_RATIO, KELVIN_OFFSET, energyplus_humidity_ratio_floor,
    energyplus_moist_air_density_kg_per_m3, energyplus_moist_air_specific_heat_j_per_kg_k,
    energyplus_psy_cp_air_fn_w, energyplus_psy_cp_air_fn_w_fast, energyplus_psy_h_fn_tdb_w,
    energyplus_psy_h_fn_tdb_w_fast, energyplus_psy_hfg_air_fn_w_tdb,
    energyplus_psy_hg_air_fn_w_tdb, energyplus_psy_rho_air_fn_pb_tdb_w,
    energyplus_psy_rho_air_fn_pb_tdb_w_fast, energyplus_water_vapor_gas_enthalpy_j_per_kg,
};

fn assert_bits(actual: f64, expected_bits: u64) {
    assert_eq!(actual.to_bits(), expected_bits, "actual={actual:?}");
}

#[test]
fn psy_rho_matches_energyplus_26_1_source_vectors_bitwise() {
    // EnergyPlus 26.1 AirflowNetworkConditions.unit.cc uses the first two
    // PsyRhoAirFnPbTdbW inputs for dry air at standard pressure. The third
    // source-derived vector also locks the 1.6077687 water-vapor coefficient.
    let cases = [
        (101_325.0, 25.0, 0.0, 0x3ff2_f21f_d120_76ba),
        (101_325.0, 15.0, 0.0, 0x3ff3_9a71_9715_c63b),
        (101_325.0, 25.0, 0.008, 0x3ff2_b49c_45c6_b5e7),
    ];

    for (pressure_pa, dry_bulb_c, humidity_ratio, expected_bits) in cases {
        assert_bits(
            energyplus_psy_rho_air_fn_pb_tdb_w(pressure_pa, dry_bulb_c, humidity_ratio),
            expected_bits,
        );
    }
}

#[test]
fn psy_cp_matches_energyplus_26_1_source_vectors_bitwise() {
    // EnergyPlus 26.1 Psychrometrics.unit.cc exercises these humidity ratios
    // against the source expression and its numerical enthalpy derivative.
    let cases = [
        (0.008, 0x408f_ddb1_5b57_3eac),
        (0.0085, 0x408f_e520_ebed_fa44),
        (0.007, 0x408f_ced2_3a29_c77a),
        (0.0, 0x408f_66de_642b_f983),
    ];

    for (humidity_ratio, expected_bits) in cases {
        assert_bits(energyplus_psy_cp_air_fn_w(humidity_ratio), expected_bits);
        assert_bits(
            energyplus_moist_air_specific_heat_j_per_kg_k(humidity_ratio),
            expected_bits,
        );
    }
}

#[test]
fn canonical_functions_apply_the_source_humidity_floor() {
    let density_at_floor =
        energyplus_psy_rho_air_fn_pb_tdb_w(101_325.0, 25.0, ENERGYPLUS_MIN_HUMIDITY_RATIO);
    let cp_at_floor = energyplus_psy_cp_air_fn_w(ENERGYPLUS_MIN_HUMIDITY_RATIO);
    let below_floor = f64::from_bits(ENERGYPLUS_MIN_HUMIDITY_RATIO.to_bits() - 1);

    for humidity_ratio in [
        f64::NEG_INFINITY,
        -100.0,
        -1.0,
        -f64::MIN_POSITIVE,
        -0.0,
        0.0,
        f64::from_bits(1),
        below_floor,
        ENERGYPLUS_MIN_HUMIDITY_RATIO,
    ] {
        assert_bits(
            energyplus_psy_rho_air_fn_pb_tdb_w(101_325.0, 25.0, humidity_ratio),
            density_at_floor.to_bits(),
        );
        assert_bits(
            energyplus_psy_cp_air_fn_w(humidity_ratio),
            cp_at_floor.to_bits(),
        );
    }

    let above_floor = f64::from_bits(ENERGYPLUS_MIN_HUMIDITY_RATIO.to_bits() + 1);
    assert_bits(
        energyplus_humidity_ratio_floor(above_floor),
        above_floor.to_bits(),
    );
    let clearly_above_floor = 2.0 * ENERGYPLUS_MIN_HUMIDITY_RATIO;
    assert_ne!(
        energyplus_psy_rho_air_fn_pb_tdb_w(101_325.0, 25.0, clearly_above_floor).to_bits(),
        density_at_floor.to_bits()
    );
    assert_ne!(
        energyplus_psy_cp_air_fn_w(clearly_above_floor).to_bits(),
        cp_at_floor.to_bits()
    );
}

#[test]
fn canonical_functions_propagate_nan_like_energyplus_cpp_max() {
    let humidity_nan = f64::from_bits(0x7ff8_0000_0000_1234);

    assert!(energyplus_psy_cp_air_fn_w(humidity_nan).is_nan());
    assert!(energyplus_psy_rho_air_fn_pb_tdb_w(101_325.0, 25.0, humidity_nan).is_nan());
    assert!(energyplus_psy_rho_air_fn_pb_tdb_w(f64::NAN, 25.0, 0.008).is_nan());
    assert!(energyplus_psy_rho_air_fn_pb_tdb_w(101_325.0, f64::NAN, 0.008).is_nan());
}

#[test]
fn canonical_functions_preserve_ieee_signed_and_infinite_results() {
    assert_bits(
        energyplus_psy_rho_air_fn_pb_tdb_w(0.0, 25.0, 0.008),
        0.0_f64.to_bits(),
    );
    assert_bits(
        energyplus_psy_rho_air_fn_pb_tdb_w(-0.0, 25.0, 0.008),
        (-0.0_f64).to_bits(),
    );
    assert!(energyplus_psy_rho_air_fn_pb_tdb_w(-101_325.0, 25.0, 0.008) < 0.0);
    assert!(energyplus_psy_rho_air_fn_pb_tdb_w(101_325.0, -300.0, 0.008) < 0.0);
    assert!(energyplus_psy_rho_air_fn_pb_tdb_w(101_325.0, -KELVIN_OFFSET, 0.008).is_infinite());
    assert_eq!(
        energyplus_psy_rho_air_fn_pb_tdb_w(f64::INFINITY, 25.0, 0.008),
        f64::INFINITY
    );
    assert_eq!(
        energyplus_psy_rho_air_fn_pb_tdb_w(f64::NEG_INFINITY, 25.0, 0.008),
        f64::NEG_INFINITY
    );
    assert_bits(
        energyplus_psy_rho_air_fn_pb_tdb_w(101_325.0, f64::INFINITY, 0.008),
        0.0_f64.to_bits(),
    );
    assert_bits(
        energyplus_psy_rho_air_fn_pb_tdb_w(101_325.0, f64::NEG_INFINITY, 0.008),
        (-0.0_f64).to_bits(),
    );
    assert_bits(
        energyplus_psy_rho_air_fn_pb_tdb_w(101_325.0, 25.0, f64::INFINITY),
        0.0_f64.to_bits(),
    );

    assert_eq!(energyplus_psy_cp_air_fn_w(f64::INFINITY), f64::INFINITY);
    assert_eq!(
        energyplus_psy_cp_air_fn_w(f64::NEG_INFINITY).to_bits(),
        energyplus_psy_cp_air_fn_w(ENERGYPLUS_MIN_HUMIDITY_RATIO).to_bits()
    );
}

#[test]
fn canonical_functions_retain_extreme_finite_ieee_behavior() {
    let density_from_max_pressure = energyplus_psy_rho_air_fn_pb_tdb_w(f64::MAX, 25.0, 0.008);
    assert!(density_from_max_pressure.is_finite());
    assert!(density_from_max_pressure > 0.0);

    assert_bits(
        energyplus_psy_rho_air_fn_pb_tdb_w(101_325.0, f64::MAX, 0.008),
        0.0_f64.to_bits(),
    );
    assert_bits(
        energyplus_psy_rho_air_fn_pb_tdb_w(101_325.0, 25.0, f64::MAX),
        0.0_f64.to_bits(),
    );
    assert!(energyplus_psy_cp_air_fn_w(1.0e300).is_finite());
    assert_eq!(energyplus_psy_cp_air_fn_w(f64::MAX), f64::INFINITY);
}

#[test]
fn guarded_wrappers_preserve_their_compatibility_contracts() {
    for (pressure_pa, dry_bulb_c) in [
        (1_000.0, 25.0),
        (0.0, 25.0),
        (f64::INFINITY, 25.0),
        (f64::NAN, 25.0),
        (101_325.0, -KELVIN_OFFSET),
        (101_325.0, f64::INFINITY),
        (101_325.0, f64::NAN),
    ] {
        assert!(energyplus_moist_air_density_kg_per_m3(pressure_pa, dry_bulb_c, 0.008).is_none());
    }

    let floor_density =
        energyplus_moist_air_density_kg_per_m3(101_325.0, 25.0, ENERGYPLUS_MIN_HUMIDITY_RATIO)
            .expect("valid guarded density input");
    let nan_density = energyplus_moist_air_density_kg_per_m3(101_325.0, 25.0, f64::NAN)
        .expect("the compatibility wrapper historically floors NaN humidity");
    assert_bits(nan_density, floor_density.to_bits());

    let floor_specific_heat =
        energyplus_moist_air_specific_heat_j_per_kg_k(ENERGYPLUS_MIN_HUMIDITY_RATIO);
    assert_bits(
        energyplus_moist_air_specific_heat_j_per_kg_k(f64::NAN),
        floor_specific_heat.to_bits(),
    );

    let guarded = energyplus_moist_air_density_kg_per_m3(82_000.0, 20.0, 0.008)
        .expect("valid guarded density input");
    assert_bits(
        guarded,
        energyplus_psy_rho_air_fn_pb_tdb_w(82_000.0, 20.0, 0.008).to_bits(),
    );
}

#[test]
fn psy_cp_repeated_and_alternating_calls_are_output_stable() {
    let cases = [
        (0.008, 0x408f_ddb1_5b57_3eac),
        (0.008, 0x408f_ddb1_5b57_3eac),
        (0.0085, 0x408f_e520_ebed_fa44),
        (0.008, 0x408f_ddb1_5b57_3eac),
        (0.007, 0x408f_ced2_3a29_c77a),
        (0.0085, 0x408f_e520_ebed_fa44),
        // The pure numerical path floors the upstream cache's invalid sentinel.
        (-100.0, 0x408f_66de_642b_f983),
        (-0.0, 0x408f_66de_642b_f983),
        (0.0, 0x408f_66de_642b_f983),
        (0.008, 0x408f_ddb1_5b57_3eac),
    ];

    for _ in 0..32 {
        for (humidity_ratio, expected_bits) in cases {
            assert_bits(energyplus_psy_cp_air_fn_w(humidity_ratio), expected_bits);
        }
    }

    for _ in 0..4 {
        assert!(energyplus_psy_cp_air_fn_w(f64::NAN).is_nan());
        assert_bits(energyplus_psy_cp_air_fn_w(0.008), 0x408f_ddb1_5b57_3eac);
    }
}

#[test]
fn psy_rho_fast_matches_pinned_source_formula_vectors_bitwise() {
    // These bits pin the local transcription of the EnergyPlus 26.1 source
    // expression; they are not output captured from an external oracle run.
    let cases = [
        (101_325.0, 25.0, 1.0e-5, 0x3ff2_f21f_d120_76ba),
        (101_325.0, 25.0, 0.008, 0x3ff2_b49c_45c6_b5e7),
        (82_000.0, 20.0, 0.0075, 0x3fee_d115_0f49_8c09),
    ];

    for (pressure_pa, dry_bulb_c, humidity_ratio, expected_bits) in cases {
        assert_bits(
            energyplus_psy_rho_air_fn_pb_tdb_w_fast(pressure_pa, dry_bulb_c, humidity_ratio),
            expected_bits,
        );
    }
}

#[test]
fn psy_enthalpy_family_matches_pinned_source_formula_vectors_bitwise() {
    // These bits pin source evaluation order and coefficients locally; they
    // are intentionally narrower evidence than an external EnergyPlus oracle.
    let hfg_cases = [
        (-20.0, 2_500_940.0_f64.to_bits()),
        (0.0, 2_500_940.0_f64.to_bits()),
        (22.0, 0x4142_b0ea_7333_3333),
        (100.0, 2_268_835.0_f64.to_bits()),
    ];
    for (dry_bulb_c, expected_bits) in hfg_cases {
        assert_bits(
            energyplus_psy_hfg_air_fn_w_tdb(0.008, dry_bulb_c),
            expected_bits,
        );
    }

    let hg_cases = [
        (-20.0, 2_463_761.0_f64.to_bits()),
        (22.0, 0x4143_6486_7333_3333),
    ];
    for (dry_bulb_c, expected_bits) in hg_cases {
        assert_bits(
            energyplus_psy_hg_air_fn_w_tdb(0.008, dry_bulb_c),
            expected_bits,
        );
    }

    let h_cases = [
        (24.0, 0.008, 0x40e5_b813_2617_c1be),
        (24.0, 0.0, 0x40d7_9367_6523_7048),
        // This vector differs by one ULP from the legacy kJ-regrouped helper.
        (15.0, 0.005, 0x40db_112e_28f5_c290),
    ];
    for (dry_bulb_c, humidity_ratio, expected_bits) in h_cases {
        assert_bits(
            energyplus_psy_h_fn_tdb_w(dry_bulb_c, humidity_ratio),
            expected_bits,
        );
    }

    let cp_fast_cases = [
        (1.0e-5, 0x408f_66de_642b_f983),
        (0.008, 0x408f_ddb1_5b57_3eac),
        (0.0085, 0x408f_e520_ebed_fa44),
    ];
    for (humidity_ratio, expected_bits) in cp_fast_cases {
        assert_bits(
            energyplus_psy_cp_air_fn_w_fast(humidity_ratio),
            expected_bits,
        );
    }
}

#[test]
fn ordinary_and_fast_psychrometric_formulas_are_bit_equal_on_the_fast_domain() {
    let humidity_ratios = [
        ENERGYPLUS_MIN_HUMIDITY_RATIO,
        f64::from_bits(ENERGYPLUS_MIN_HUMIDITY_RATIO.to_bits() + 1),
        0.0075,
        0.008,
        0.02,
        f64::INFINITY,
    ];

    for humidity_ratio in humidity_ratios {
        assert_bits(
            energyplus_psy_rho_air_fn_pb_tdb_w_fast(82_000.0, 20.0, humidity_ratio),
            energyplus_psy_rho_air_fn_pb_tdb_w(82_000.0, 20.0, humidity_ratio).to_bits(),
        );
        assert_bits(
            energyplus_psy_h_fn_tdb_w_fast(24.0, humidity_ratio),
            energyplus_psy_h_fn_tdb_w(24.0, humidity_ratio).to_bits(),
        );
        assert_bits(
            energyplus_psy_cp_air_fn_w_fast(humidity_ratio),
            energyplus_psy_cp_air_fn_w(humidity_ratio).to_bits(),
        );
    }
}

#[test]
fn psy_h_applies_the_source_humidity_floor_and_propagates_nan() {
    let at_floor = energyplus_psy_h_fn_tdb_w(24.0, ENERGYPLUS_MIN_HUMIDITY_RATIO);
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
            energyplus_psy_h_fn_tdb_w(24.0, humidity_ratio),
            at_floor.to_bits(),
        );
    }

    assert!(energyplus_psy_h_fn_tdb_w(24.0, f64::NAN).is_nan());
    assert!(energyplus_psy_h_fn_tdb_w(f64::NAN, 0.008).is_nan());
}

#[test]
fn psy_hfg_and_hg_ignore_humidity_ratio_exactly() {
    let hfg_bits = energyplus_psy_hfg_air_fn_w_tdb(0.008, 22.0).to_bits();
    let hg_bits = energyplus_psy_hg_air_fn_w_tdb(0.008, 22.0).to_bits();

    for humidity_ratio in [
        f64::NEG_INFINITY,
        -100.0,
        -0.0,
        0.0,
        f64::INFINITY,
        f64::from_bits(0x7ff8_0000_0000_1234),
    ] {
        assert_bits(
            energyplus_psy_hfg_air_fn_w_tdb(humidity_ratio, 22.0),
            hfg_bits,
        );
        assert_bits(
            energyplus_psy_hg_air_fn_w_tdb(humidity_ratio, 22.0),
            hg_bits,
        );
    }
}

#[test]
fn psy_hfg_preserves_source_evaluation_order_at_ieee_edges() {
    assert_bits(
        energyplus_psy_hfg_air_fn_w_tdb(0.008, f64::NEG_INFINITY),
        2_500_940.0_f64.to_bits(),
    );
    assert!(energyplus_psy_hfg_air_fn_w_tdb(0.008, f64::INFINITY).is_nan());
    assert!(energyplus_psy_hfg_air_fn_w_tdb(0.008, f64::MAX).is_nan());
    assert!(energyplus_psy_hfg_air_fn_w_tdb(0.008, f64::NAN).is_nan());

    assert_eq!(
        energyplus_psy_hg_air_fn_w_tdb(0.008, f64::INFINITY),
        f64::INFINITY
    );
    assert_eq!(
        energyplus_psy_hg_air_fn_w_tdb(0.008, f64::NEG_INFINITY),
        f64::NEG_INFINITY
    );
    assert!(energyplus_psy_hg_air_fn_w_tdb(0.008, f64::NAN).is_nan());
}

#[test]
fn legacy_water_vapor_enthalpy_wrapper_delegates_bitwise() {
    for dry_bulb_c in [
        f64::NEG_INFINITY,
        -20.0,
        -0.0,
        0.0,
        22.0,
        f64::MAX,
        f64::INFINITY,
    ] {
        assert_bits(
            energyplus_water_vapor_gas_enthalpy_j_per_kg(dry_bulb_c),
            energyplus_psy_hg_air_fn_w_tdb(0.0, dry_bulb_c).to_bits(),
        );
    }
}

#[test]
fn psy_cp_fast_repeated_and_alternating_calls_are_output_stable() {
    let cases = [
        (0.008, 0x408f_ddb1_5b57_3eac),
        (0.008, 0x408f_ddb1_5b57_3eac),
        (0.0085, 0x408f_e520_ebed_fa44),
        (0.007, 0x408f_ced2_3a29_c77a),
        (ENERGYPLUS_MIN_HUMIDITY_RATIO, 0x408f_66de_642b_f983),
    ];

    for _ in 0..32 {
        for (humidity_ratio, expected_bits) in cases {
            assert_bits(
                energyplus_psy_cp_air_fn_w_fast(humidity_ratio),
                expected_bits,
            );
        }
    }
}

#[cfg(debug_assertions)]
#[test]
fn fast_paths_debug_assert_the_source_humidity_precondition() {
    let below_floor = f64::from_bits(ENERGYPLUS_MIN_HUMIDITY_RATIO.to_bits() - 1);
    for humidity_ratio in [below_floor, 0.0, -1.0, f64::NAN] {
        assert!(
            std::panic::catch_unwind(|| {
                energyplus_psy_rho_air_fn_pb_tdb_w_fast(101_325.0, 25.0, humidity_ratio)
            })
            .is_err()
        );
        assert!(
            std::panic::catch_unwind(|| energyplus_psy_h_fn_tdb_w_fast(25.0, humidity_ratio))
                .is_err()
        );
        assert!(
            std::panic::catch_unwind(|| energyplus_psy_cp_air_fn_w_fast(humidity_ratio)).is_err()
        );
    }
}

#[cfg(not(debug_assertions))]
#[test]
fn fast_paths_do_not_floor_humidity_in_release_builds() {
    let humidity_ratio = 0.0;
    let density_without_floor =
        101_325.0 / (287.0 * (25.0 + KELVIN_OFFSET) * (1.0 + 1.607_768_7 * humidity_ratio));
    let enthalpy_without_floor =
        1.004_84e3 * 24.0 + humidity_ratio * (2.500_94e6 + 1.858_95e3 * 24.0);
    let specific_heat_without_floor = 1.004_84e3 + humidity_ratio * 1.858_95e3;

    assert_bits(
        energyplus_psy_rho_air_fn_pb_tdb_w_fast(101_325.0, 25.0, humidity_ratio),
        density_without_floor.to_bits(),
    );
    assert_bits(
        energyplus_psy_h_fn_tdb_w_fast(24.0, humidity_ratio),
        enthalpy_without_floor.to_bits(),
    );
    assert_bits(
        energyplus_psy_cp_air_fn_w_fast(humidity_ratio),
        specific_heat_without_floor.to_bits(),
    );

    assert_ne!(
        density_without_floor.to_bits(),
        energyplus_psy_rho_air_fn_pb_tdb_w(101_325.0, 25.0, humidity_ratio).to_bits()
    );
    assert_ne!(
        enthalpy_without_floor.to_bits(),
        energyplus_psy_h_fn_tdb_w(24.0, humidity_ratio).to_bits()
    );
    assert_ne!(
        specific_heat_without_floor.to_bits(),
        energyplus_psy_cp_air_fn_w(humidity_ratio).to_bits()
    );
}
