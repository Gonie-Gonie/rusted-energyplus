//! Explicit last-call state for the two EnergyPlus specific-heat variants.

use super::{
    ENERGYPLUS_MIN_HUMIDITY_RATIO, energyplus_humidity_ratio_floor, energyplus_psy_cp_air_fn_w_raw,
};
use std::cell::RefCell;

thread_local! {
    static CP_AIR_CACHE: RefCell<EnergyPlusCpAirCache> = RefCell::new(EnergyPlusCpAirCache::default());
}

pub(super) fn normal(
    humidity_ratio: f64,
) -> (f64, EnergyPlusCpAirCacheState, EnergyPlusCpAirCacheState) {
    CP_AIR_CACHE.with_borrow_mut(|cache| {
        let before = cache.normal_state();
        let result = cache.psy_cp_air_fn_w(humidity_ratio);
        (result, before, cache.normal_state())
    })
}

pub(super) fn fast(
    humidity_ratio: f64,
) -> (f64, EnergyPlusCpAirCacheState, EnergyPlusCpAirCacheState) {
    CP_AIR_CACHE.with_borrow_mut(|cache| {
        let before = cache.fast_state();
        let result = cache.psy_cp_air_fn_w_fast(humidity_ratio);
        (result, before, cache.fast_state())
    })
}

/// Saved input and result for one EnergyPlus `PsyCpAirFnW` variant.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EnergyPlusCpAirCacheState {
    /// Original, unfloored humidity ratio in kgWater/kgDryAir.
    pub humidity_ratio_kg_per_kg: f64,
    /// Last returned specific heat in J/(kg K), including the initial sentinel.
    pub specific_heat_j_per_kg_k: f64,
}

impl Default for EnergyPlusCpAirCacheState {
    fn default() -> Self {
        Self {
            humidity_ratio_kg_per_kg: -100.0,
            specific_heat_j_per_kg_k: -100.0,
        }
    }
}

/// Per-execution equivalents of the independent normal and fast Cp caches.
///
/// EnergyPlus 26.1 `Psychrometrics.hh:700-740` gives each inline variant its
/// own function-local statics. An instance models one fresh execution, without
/// adding process-global mutable state or a concurrency policy to Rust. The
/// cold normal call at `W = -100` returns the source's saved `-100` before the
/// humidity floor; subsequent calls preserve the exact ordered equality test.
#[derive(Clone, Debug, Default)]
pub struct EnergyPlusCpAirCache {
    normal: EnergyPlusCpAirCacheState,
    fast: EnergyPlusCpAirCacheState,
}

impl EnergyPlusCpAirCache {
    /// Returns the normal variant's current saved input and result.
    #[must_use]
    pub fn normal_state(&self) -> EnergyPlusCpAirCacheState {
        self.normal
    }

    /// Returns the fast variant's independent saved input and result.
    #[must_use]
    pub fn fast_state(&self) -> EnergyPlusCpAirCacheState {
        self.fast
    }

    /// Calls the source normal Cp path, including its observable cache state.
    #[must_use]
    #[track_caller]
    pub fn psy_cp_air_fn_w(&mut self, humidity_ratio: f64) -> f64 {
        evaluate(&mut self.normal, humidity_ratio, false)
    }

    /// Calls the source fast Cp path with its independent cache state.
    ///
    /// As in EnergyPlus, debug builds assert `humidity_ratio >= 1.0e-5` before
    /// checking the saved input. The caller owns that precondition in release.
    #[must_use]
    #[track_caller]
    pub fn psy_cp_air_fn_w_fast(&mut self, humidity_ratio: f64) -> f64 {
        debug_assert!(humidity_ratio >= ENERGYPLUS_MIN_HUMIDITY_RATIO);
        evaluate(&mut self.fast, humidity_ratio, true)
    }
}

fn evaluate(state: &mut EnergyPlusCpAirCacheState, humidity_ratio: f64, fast: bool) -> f64 {
    if state.humidity_ratio_kg_per_kg == humidity_ratio {
        return state.specific_heat_j_per_kg_k;
    }
    let effective_humidity_ratio = if fast {
        humidity_ratio
    } else {
        energyplus_humidity_ratio_floor(humidity_ratio)
    };
    let specific_heat = energyplus_psy_cp_air_fn_w_raw(effective_humidity_ratio);
    state.humidity_ratio_kg_per_kg = humidity_ratio;
    state.specific_heat_j_per_kg_k = specific_heat;
    specific_heat
}

#[cfg(test)]
mod tests {
    use super::EnergyPlusCpAirCache;
    use crate::psychrometrics::{energyplus_psy_cp_air_fn_w, energyplus_psy_cp_air_fn_w_fast};

    #[test]
    fn cold_normal_sentinel_is_observable_before_flooring() {
        let mut cache = EnergyPlusCpAirCache::default();
        assert_eq!(cache.psy_cp_air_fn_w(-100.0), -100.0);
        assert_eq!(cache.psy_cp_air_fn_w(-100.0), -100.0);
        assert_eq!(
            cache.psy_cp_air_fn_w(0.008),
            energyplus_psy_cp_air_fn_w(0.008)
        );
        let floored = energyplus_psy_cp_air_fn_w(-100.0);
        assert_eq!(cache.psy_cp_air_fn_w(-100.0), floored);
        assert_eq!(cache.psy_cp_air_fn_w(-100.0), floored);
        assert_eq!(cache.normal_state().humidity_ratio_kg_per_kg, -100.0);
        assert_eq!(cache.normal_state().specific_heat_j_per_kg_k, floored);
    }

    #[test]
    fn normal_and_fast_variants_have_independent_saved_values() {
        let mut cache = EnergyPlusCpAirCache::default();
        assert_eq!(
            cache.psy_cp_air_fn_w_fast(0.008),
            energyplus_psy_cp_air_fn_w_fast(0.008)
        );
        assert_eq!(cache.psy_cp_air_fn_w(-100.0), -100.0);
        let fast_before = cache.fast_state();
        let _ = cache.psy_cp_air_fn_w(0.02);
        assert_eq!(cache.fast_state(), fast_before);
    }

    #[test]
    fn equality_reuses_the_signed_zero_input_and_nan_never_matches() {
        let mut cache = EnergyPlusCpAirCache::default();
        let dry_cp = cache.psy_cp_air_fn_w(-0.0);
        assert_eq!(cache.psy_cp_air_fn_w(0.0), dry_cp);
        assert_eq!(
            cache.normal_state().humidity_ratio_kg_per_kg.to_bits(),
            (-0.0_f64).to_bits()
        );
        assert!(cache.psy_cp_air_fn_w(f64::NAN).is_nan());
        assert!(cache.psy_cp_air_fn_w(f64::NAN).is_nan());
        assert_eq!(
            cache.psy_cp_air_fn_w(0.008),
            energyplus_psy_cp_air_fn_w(0.008)
        );
        assert_eq!(cache.normal_state().humidity_ratio_kg_per_kg, 0.008);
    }

    #[test]
    fn canonical_entry_points_retain_source_state_on_the_execution_thread() {
        let execution = std::thread::spawn(|| {
            let fast_value = energyplus_psy_cp_air_fn_w_fast(0.008);
            assert_eq!(energyplus_psy_cp_air_fn_w(-100.0), -100.0);
            let normal_value = energyplus_psy_cp_air_fn_w(0.008);
            assert_eq!(normal_value, fast_value);
            assert_eq!(
                energyplus_psy_cp_air_fn_w(0.02),
                energyplus_psy_cp_air_fn_w_fast(0.02)
            );
            assert_eq!(energyplus_psy_cp_air_fn_w(0.008), normal_value);
            assert_eq!(
                energyplus_psy_cp_air_fn_w(-100.0),
                energyplus_psy_cp_air_fn_w(1.0e-5)
            );
        });
        assert!(execution.join().is_ok());
    }
}
