use super::{
    ENERGYPLUS_MIN_HUMIDITY_RATIO, KELVIN_OFFSET, energyplus_humidity_ratio_floor,
    energyplus_moist_air_density_kg_per_m3, energyplus_moist_air_specific_heat_j_per_kg_k,
    energyplus_psy_cp_air_fn_w, energyplus_psy_rho_air_fn_pb_tdb_w,
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
