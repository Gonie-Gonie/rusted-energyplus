use super::{
    ENERGYPLUS_PSYCHROMETRIC_ITERATION_TOLERANCE, ENERGYPLUS_TSAT_PRESSURE_MAX_ITERATIONS,
    energyplus_psy_psat_fn_temp_default_numerical_projection, energyplus_psy_psat_fn_temp_raw,
    energyplus_psy_tsat_fn_pb_raw,
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

fn source_iterate(
    current_x: f64,
    current_y: f64,
    previous_x: &mut f64,
    previous_y: &mut f64,
    iteration: u32,
) -> (f64, bool) {
    if iteration != 1
        && ((current_x - *previous_x).abs() < ENERGYPLUS_PSYCHROMETRIC_ITERATION_TOLERANCE
            || current_y == 0.0)
    {
        return (current_x, true);
    }

    let result_x = if iteration == 1 {
        if current_x.abs() > 1.0e-9 {
            current_x * 1.1
        } else {
            0.1
        }
    } else {
        let mut delta_y = current_y - *previous_y;
        if delta_y.abs() < 1.0e-9 {
            delta_y = 1.0e-9;
        }
        (current_y * *previous_x - *previous_y * current_x) / delta_y
    };
    *previous_x = current_x;
    *previous_y = current_y;
    (result_x, false)
}

fn source_solver(pressure_pa: f64, cached_psat: bool) -> f64 {
    if pressure_pa >= 1_555_000.0 {
        return 200.0;
    }
    if pressure_pa <= 0.0017 {
        return -100.0;
    }
    if pressure_pa > 611.0 && pressure_pa < 611.25 {
        return 0.0;
    }

    let mut saturation_temperature_c = 100.0;
    let mut previous_temperature_c = 0.0;
    let mut previous_error_pa = 0.0;
    for iteration in 1..=ENERGYPLUS_TSAT_PRESSURE_MAX_ITERATIONS {
        let saturation_pressure_pa = if cached_psat {
            energyplus_psy_psat_fn_temp_default_numerical_projection(saturation_temperature_c)
        } else {
            energyplus_psy_psat_fn_temp_raw(saturation_temperature_c)
        };
        let (next_temperature_c, converged) = source_iterate(
            saturation_temperature_c,
            pressure_pa - saturation_pressure_pa,
            &mut previous_temperature_c,
            &mut previous_error_pa,
            iteration,
        );
        saturation_temperature_c = next_temperature_c;
        if converged {
            break;
        }
    }
    saturation_temperature_c
}

#[test]
fn saturation_temperature_matches_upstream_vectors() {
    assert_close(energyplus_psy_tsat_fn_pb_raw(101_325.0), 99.974, 0.001);
    assert_bits(energyplus_psy_tsat_fn_pb_raw(1_555_000.0), 200.0);
    assert_bits(energyplus_psy_tsat_fn_pb_raw(0.0017), -100.0);
    assert_bits(energyplus_psy_tsat_fn_pb_raw(611.1), 0.0);
}

#[test]
fn pressure_bounds_are_inclusive_and_ordered() {
    for pressure_pa in [
        f64::NEG_INFINITY,
        // The pure core intentionally excludes the source's initial
        // Press_Save/tSat_Save = -99999 last-call sentinel shortcut.
        -99_999.0,
        -1.0,
        -0.0,
        0.0,
        0.0017_f64.next_down(),
        0.0017,
    ] {
        assert_bits(energyplus_psy_tsat_fn_pb_raw(pressure_pa), -100.0);
    }
    for pressure_pa in [1_555_000.0, 1_555_000.0_f64.next_up(), f64::INFINITY] {
        assert_bits(energyplus_psy_tsat_fn_pb_raw(pressure_pa), 200.0);
    }
}

#[test]
fn triple_point_shortcut_is_strict_on_both_sides() {
    for pressure_pa in [611.0_f64.next_up(), 611.1, 611.25_f64.next_down()] {
        assert_bits(energyplus_psy_tsat_fn_pb_raw(pressure_pa), 0.0);
    }

    assert_bits(
        energyplus_psy_tsat_fn_pb_raw(611.0),
        source_solver(611.0, true),
    );
    assert_bits(
        energyplus_psy_tsat_fn_pb_raw(611.25),
        source_solver(611.25, true),
    );
    assert_ne!(
        energyplus_psy_tsat_fn_pb_raw(611.0).to_bits(),
        0.0_f64.to_bits()
    );
    assert_ne!(
        energyplus_psy_tsat_fn_pb_raw(611.25).to_bits(),
        0.0_f64.to_bits()
    );
}

#[test]
fn iterative_path_preserves_source_order_and_default_psat_projection() {
    for pressure_pa in [
        0.0017_f64.next_up(),
        64.0,
        1_000.0,
        101_325.0,
        500_000.0,
        1_555_000.0_f64.next_down(),
    ] {
        assert_bits(
            energyplus_psy_tsat_fn_pb_raw(pressure_pa),
            source_solver(pressure_pa, true),
        );
    }
}

#[test]
fn nested_psat_cache_representatives_change_the_iterative_projection() {
    let pressure_pa = 101_325.0;
    let cached_result = source_solver(pressure_pa, true);
    let no_cache_result = source_solver(pressure_pa, false);

    assert_bits(energyplus_psy_tsat_fn_pb_raw(pressure_pa), cached_result);
    assert_ne!(cached_result.to_bits(), no_cache_result.to_bits());
}

#[test]
fn pressure_and_saturation_pressure_round_trip_with_source_tolerance() {
    for temperature_c in [-60.0, -20.0, 20.0, 100.0, 160.0] {
        let pressure_pa = energyplus_psy_psat_fn_temp_default_numerical_projection(temperature_c);
        assert_close(
            energyplus_psy_tsat_fn_pb_raw(pressure_pa),
            temperature_c,
            0.001,
        );
    }
}

#[test]
fn nan_runs_the_bounded_source_iteration_and_remains_nan() {
    assert!(energyplus_psy_tsat_fn_pb_raw(f64::NAN).is_nan());
}

#[test]
fn iteration_contract_pins_source_literals_and_exhaustion_edge() {
    assert_eq!(ENERGYPLUS_TSAT_PRESSURE_MAX_ITERATIONS, 50);
    assert_bits(ENERGYPLUS_PSYCHROMETRIC_ITERATION_TOLERANCE, 0.0001);
    assert!(energyplus_psy_tsat_fn_pb_raw(1_555_000.0_f64.next_down()).is_nan());
}

#[test]
fn repeated_and_alternating_calls_are_output_stable() {
    let first = energyplus_psy_tsat_fn_pb_raw(101_325.0);
    let second = energyplus_psy_tsat_fn_pb_raw(2_000.0);

    for _ in 0..16 {
        assert_bits(energyplus_psy_tsat_fn_pb_raw(101_325.0), first);
        assert_bits(energyplus_psy_tsat_fn_pb_raw(2_000.0), second);
    }
}
