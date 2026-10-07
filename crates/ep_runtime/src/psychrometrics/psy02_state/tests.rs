//! Cache/state invariants independent of imported expected numerical outputs.

use super::*;
use crate::psychrometrics::{energyplus_psy_psat_fn_temp, energyplus_psy_twb_fn_tdb_w_pb};

#[test]
fn cold_zero_tag_hits_preserve_source_sentinels_without_running_raw_helpers() {
    let mut state = EnergyPlusPsychrometricsState::default();
    let initial = state.snapshot();
    for (function, input) in [
        (EnergyPlusPsychrometricFunction::Twb, vec![0.0; 3]),
        (EnergyPlusPsychrometricFunction::TsatPb, vec![0.0]),
        (EnergyPlusPsychrometricFunction::TsatHPb, vec![0.0; 2]),
    ] {
        let call = state.evaluate(function, &input).unwrap();
        assert_eq!(call.result.to_bits(), 0);
        assert_eq!(call.cache_before, call.cache_after);
        assert_eq!(call.state_after, initial);
    }
    assert_eq!(
        state.final_caches(),
        EnergyPlusPsychrometricFinalCaches::default()
    );
}

#[test]
fn psat_representative_and_direct_mapping_collision_retain_actual_last_writer() {
    let mut state = EnergyPlusPsychrometricsState::default();
    let temperature: f64 = 22.0;
    let first = state
        .evaluate(EnergyPlusPsychrometricFunction::Psat, &[temperature])
        .unwrap();
    let same_tag = f64::from_bits(temperature.to_bits() + 1);
    let hit = state
        .evaluate(EnergyPlusPsychrometricFunction::Psat, &[same_tag])
        .unwrap();
    assert_eq!(first.result.to_bits(), hit.result.to_bits());
    assert_eq!(hit.cache_before, hit.cache_after);
    let collided = f64::from_bits(temperature.to_bits() + (1_u64 << 48));
    let replacement = state
        .evaluate(EnergyPlusPsychrometricFunction::Psat, &[collided])
        .unwrap();
    assert_eq!(replacement.cache_before, first.cache_after);
    assert_ne!(replacement.cache_before, replacement.cache_after);
    assert_eq!(state.final_caches().psat.len(), 1);
    let recomputed = state
        .evaluate(EnergyPlusPsychrometricFunction::Psat, &[temperature])
        .unwrap();
    assert_eq!(recomputed.result.to_bits(), first.result.to_bits());
    assert_ne!(recomputed.cache_before, recomputed.cache_after);
}

#[test]
fn saturation_pressure_cache_uses_original_first_writer_not_representative() {
    let mut state = EnergyPlusPsychrometricsState::default();
    let pressure: f64 = 101325.0;
    let neighbor = f64::from_bits(pressure.to_bits() + 1);
    let first = state
        .evaluate(EnergyPlusPsychrometricFunction::TsatPb, &[pressure])
        .unwrap();
    assert_eq!(first.state_after.press_save.to_bits(), pressure.to_bits());
    let second = state
        .evaluate(EnergyPlusPsychrometricFunction::TsatPb, &[neighbor])
        .unwrap();
    assert_eq!(second.result.to_bits(), first.result.to_bits());
    assert_eq!(second.state_after.press_save.to_bits(), pressure.to_bits());
    let direct = state
        .evaluate(EnergyPlusPsychrometricFunction::TsatPbRaw, &[neighbor])
        .unwrap();
    assert_eq!(direct.state_after.press_save.to_bits(), neighbor.to_bits());
    assert!(direct.cache_before.is_none());
}

#[test]
fn wet_bulb_representatives_drive_boiling_saved_pressure_and_nested_caches() {
    let mut state = EnergyPlusPsychrometricsState::default();
    let p: f64 = 101325.123;
    let expected_pressure = f64::from_bits((p.to_bits() >> 32) << 32);
    let first = state
        .evaluate(EnergyPlusPsychrometricFunction::Twb, &[30.0, 0.008, p])
        .unwrap();
    assert!(first.result.is_finite() && first.result < 30.0);
    assert_eq!(
        first.state_after.last_patm.to_bits(),
        expected_pressure.to_bits()
    );
    assert_eq!(
        first.state_after.press_save.to_bits(),
        expected_pressure.to_bits()
    );
    let tables = state.final_caches();
    assert_eq!(tables.twb.len(), 1);
    assert_eq!(tables.tsat_pb.len(), 1);
    assert!(tables.psat.len() > 1);
    let repeat = state
        .evaluate(EnergyPlusPsychrometricFunction::Twb, &[30.0, 0.008, p])
        .unwrap();
    assert_eq!(repeat.cache_before, repeat.cache_after);
    assert_eq!(repeat.state_before, repeat.state_after);
    assert_eq!(repeat.result.to_bits(), first.result.to_bits());
    assert_eq!(state.final_caches(), tables);
}

#[test]
fn raw_pressure_saved_sentinel_is_an_observable_source_shortcut() {
    let mut state = EnergyPlusPsychrometricsState::default();
    let call = state
        .evaluate(EnergyPlusPsychrometricFunction::TsatPbRaw, &[-99999.0])
        .unwrap();
    assert_eq!(call.result, -99999.0);
    assert_eq!(call.state_before, call.state_after);
    assert!(state.final_caches().psat.is_empty());
}

#[test]
fn source_ordered_humidity_floors_preserve_positive_subfloor_and_signed_zero() {
    let mut state = EnergyPlusPsychrometricsState::default();
    let negative = state
        .evaluate(EnergyPlusPsychrometricFunction::WFromH, &[0.0, -1.0])
        .unwrap();
    assert_eq!(negative.result, 1.0e-5);
    let positive = state
        .evaluate(EnergyPlusPsychrometricFunction::WFromH, &[0.0, 1.0])
        .unwrap();
    assert!(positive.result > 0.0 && positive.result < 1.0e-5);
    let negative_zero = state
        .evaluate(EnergyPlusPsychrometricFunction::WFromH, &[0.0, -0.0])
        .unwrap();
    assert_eq!(negative_zero.result.to_bits(), (-0.0_f64).to_bits());
    let nan = state
        .evaluate(EnergyPlusPsychrometricFunction::WFromH, &[f64::NAN, 1.0])
        .unwrap();
    assert!(nan.result.is_nan());
}

#[test]
fn independent_owners_and_nested_run_lifetimes_restore_full_numerical_state() {
    with_fresh_psychrometric_state(|| {
        let _ = energyplus_psy_psat_fn_temp(22.0);
        let before = with_state(|s| (s.snapshot(), s.final_caches()));
        with_fresh_psychrometric_state(|| {
            assert!(with_state(|s| s.final_caches().psat.is_empty()));
            let _ = energyplus_psy_twb_fn_tdb_w_pb(35.0, 0.012, 95000.0);
            assert_ne!(with_state(|s| s.snapshot()), before.0);
        });
        assert_eq!(with_state(|s| (s.snapshot(), s.final_caches())), before);
    });
}

#[test]
#[allow(clippy::panic)] // Deliberate unwind proves restoration of a run owner.
fn run_owner_restores_after_unwind() {
    with_fresh_psychrometric_state(|| {
        let _ = energyplus_psy_psat_fn_temp(22.0);
        let before = with_state(|s| s.final_caches());
        assert!(
            std::panic::catch_unwind(|| with_fresh_psychrometric_state(|| {
                let _ = energyplus_psy_psat_fn_temp(-22.0);
                panic!("intentional nested run unwind");
            }))
            .is_err()
        );
        assert_eq!(with_state(|s| s.final_caches()), before);
    });
}
