//! Instance-owned default-build EnergyPlus saturation caches and inverse paths.
//!
//! Sparse slots have the exact logical defaults of the original 2^20 arrays.
//! Warning aggregation and ErrorManager output are outside this numerical owner.

use super::{
    ENERGYPLUS_PSYCHROMETRIC_ITERATION_TOLERANCE, ENERGYPLUS_TSAT_PRESSURE_MAX_ITERATIONS,
    ENERGYPLUS_WET_BULB_MAX_ITERATIONS, energyplus_general_iterate, energyplus_psy_h_fn_tdb_w,
    energyplus_psy_psat_fn_temp_raw, energyplus_psy_tsat_fn_h_pb_raw_with_properties,
    energyplus_psy_w_fn_tdb_h,
};
use std::{cell::RefCell, collections::BTreeMap};

/// Logical length of every selected upstream direct-mapped cache.
pub const ENERGYPLUS_PSY_CACHE_SIZE: usize = 1 << 20;

/// Named source entry, retaining unsigned Twb and signed saturation tags.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EnergyPlusPsychrometricCacheSlot {
    Twb {
        index: usize,
        i_tdb: u64,
        i_w: u64,
        i_pb: u64,
        value: f64,
    },
    Psat {
        index: usize,
        i_tdb: i64,
        value: f64,
    },
    TsatPb {
        index: usize,
        i_h: i64,
        i_pb: i64,
        value: f64,
    },
    TsatHPb {
        index: usize,
        i_h: i64,
        i_pb: i64,
        value: f64,
    },
}

/// Scalar state read or written by the selected default-build paths.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EnergyPlusPsychrometricStateSnapshot {
    pub iconv_tol: f64,
    pub last_patm: f64,
    pub last_t_boil: f64,
    pub press_save: f64,
    pub t_sat_save: f64,
    pub warmup: bool,
    pub use_interpolation: bool,
}

/// Nondefault logical slots, in ascending index order in each array.
#[allow(missing_docs)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EnergyPlusPsychrometricFinalCaches {
    pub twb: Vec<EnergyPlusPsychrometricCacheSlot>,
    pub psat: Vec<EnergyPlusPsychrometricCacheSlot>,
    pub tsat_pb: Vec<EnergyPlusPsychrometricCacheSlot>,
    pub tsat_h_pb: Vec<EnergyPlusPsychrometricCacheSlot>,
}

/// Selected original callable routes; raw routes still own nested source state.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EnergyPlusPsychrometricFunction {
    Twb,
    TwbRaw,
    Psat,
    PsatRaw,
    TsatPb,
    TsatPbRaw,
    TsatHPb,
    TsatHPbRaw,
    WFromRh,
    WFromH,
    WFromTwb,
    H,
}

impl EnergyPlusPsychrometricFunction {
    /// Exact original symbol associated with this route.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Twb => "PsyTwbFnTdbWPb",
            Self::TwbRaw => "PsyTwbFnTdbWPb_raw",
            Self::Psat => "PsyPsatFnTemp",
            Self::PsatRaw => "PsyPsatFnTemp_raw",
            Self::TsatPb => "PsyTsatFnPb",
            Self::TsatPbRaw => "PsyTsatFnPb_raw",
            Self::TsatHPb => "PsyTsatFnHPb",
            Self::TsatHPbRaw => "PsyTsatFnHPb_raw",
            Self::WFromRh => "PsyWFnTdbRhPb",
            Self::WFromH => "PsyWFnTdbH",
            Self::WFromTwb => "PsyWFnTdbTwbPb",
            Self::H => "PsyHFnTdbW",
        }
    }
}

/// Before/after observation of one externally requested property operation.
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct EnergyPlusPsychrometricOperation {
    pub result: f64,
    pub state_before: EnergyPlusPsychrometricStateSnapshot,
    pub state_after: EnergyPlusPsychrometricStateSnapshot,
    pub cache_before: Option<EnergyPlusPsychrometricCacheSlot>,
    pub cache_after: Option<EnergyPlusPsychrometricCacheSlot>,
}

/// Equivalent of one fresh EnergyPlusData's selected numerical property state.
#[derive(Clone, Debug)]
pub struct EnergyPlusPsychrometricsState {
    state: EnergyPlusPsychrometricStateSnapshot,
    slots: [BTreeMap<usize, EnergyPlusPsychrometricCacheSlot>; 4],
}

impl Default for EnergyPlusPsychrometricsState {
    fn default() -> Self {
        Self {
            state: EnergyPlusPsychrometricStateSnapshot {
                iconv_tol: ENERGYPLUS_PSYCHROMETRIC_ITERATION_TOLERANCE,
                last_patm: -99999.0,
                last_t_boil: -99999.0,
                press_save: -99999.0,
                t_sat_save: -99999.0,
                warmup: false,
                use_interpolation: false,
            },
            slots: std::array::from_fn(|_| BTreeMap::new()),
        }
    }
}

impl EnergyPlusPsychrometricsState {
    /// Current scalar state; warning recurrence is deliberately not fabricated.
    pub fn snapshot(&self) -> EnergyPlusPsychrometricStateSnapshot {
        self.state
    }

    /// Explicit source settings for diagnostic unit probes, never expected values.
    pub fn set_controls(&mut self, warmup: Option<bool>, iconv_tol: Option<f64>) {
        if let Some(value) = warmup {
            self.state.warmup = value;
        }
        if let Some(value) = iconv_tol {
            self.state.iconv_tol = value;
        }
    }

    /// Nondefault entries of all four source arrays, including nested mutations.
    pub fn final_caches(&self) -> EnergyPlusPsychrometricFinalCaches {
        let rows = |kind: usize| {
            self.slots[kind]
                .iter()
                .filter_map(|(&index, &slot)| {
                    (!slot_bits_equal(slot, default_slot(kind, index))).then_some(slot)
                })
                .collect()
        };
        EnergyPlusPsychrometricFinalCaches {
            twb: rows(0),
            psat: rows(1),
            tsat_pb: rows(2),
            tsat_h_pb: rows(3),
        }
    }

    /// Calls an original route in source argument order and observes actual state.
    /// Argument count is a dispatch contract, not a physical-input clamp.
    #[track_caller]
    pub fn evaluate(
        &mut self,
        function: EnergyPlusPsychrometricFunction,
        inputs: &[f64],
    ) -> Result<EnergyPlusPsychrometricOperation, &'static str> {
        use EnergyPlusPsychrometricFunction as F;
        let count = match function {
            F::Psat | F::PsatRaw | F::TsatPb | F::TsatPbRaw => 1,
            F::TsatHPb | F::TsatHPbRaw | F::WFromH | F::H => 2,
            _ => 3,
        };
        if inputs.len() != count {
            return Err("incorrect psychrometric argument count");
        }
        let _guard = super::psy02_trace::enter();
        Ok(self.evaluate_validated(function, inputs))
    }

    pub(super) fn evaluate_validated(
        &mut self,
        function: EnergyPlusPsychrometricFunction,
        inputs: &[f64],
    ) -> EnergyPlusPsychrometricOperation {
        use EnergyPlusPsychrometricFunction as F;
        let state_before = self.state;
        let target = self.target(function, inputs);
        let cache_before = target.map(|(kind, index)| self.slot(kind, index));
        let result = match function {
            F::Psat => self.psat(inputs[0]),
            F::PsatRaw => energyplus_psy_psat_fn_temp_raw(inputs[0]),
            F::TsatPb => self.tsat_pb(inputs[0]),
            F::TsatPbRaw => self.tsat_pb_raw(inputs[0]),
            F::TsatHPb => self.tsat_h_pb(inputs[0], inputs[1]),
            F::TsatHPbRaw => self.tsat_h_pb_raw(inputs[0], inputs[1]),
            F::Twb => self.twb(inputs[0], inputs[1], inputs[2]),
            F::TwbRaw => self.twb_raw(inputs[0], inputs[1], inputs[2]),
            F::WFromRh => self.w_from_rh(inputs[0], inputs[1], inputs[2]),
            F::WFromH => energyplus_psy_w_fn_tdb_h(inputs[0], inputs[1]),
            F::WFromTwb => self.w_from_twb(inputs[0], inputs[1], inputs[2]),
            F::H => energyplus_psy_h_fn_tdb_w(inputs[0], inputs[1]),
        };
        EnergyPlusPsychrometricOperation {
            result,
            state_before,
            state_after: self.state,
            cache_before,
            cache_after: target.map(|(kind, index)| self.slot(kind, index)),
        }
    }

    fn target(
        &self,
        function: EnergyPlusPsychrometricFunction,
        x: &[f64],
    ) -> Option<(usize, usize)> {
        use EnergyPlusPsychrometricFunction as F;
        match function {
            F::Twb => Some((
                0,
                ((x[0].to_bits() >> 32) ^ (x[1].to_bits() >> 32) ^ (x[2].to_bits() >> 32)) as usize
                    & (ENERGYPLUS_PSY_CACHE_SIZE - 1),
            )),
            F::Psat => Some((
                1,
                signed_tag(x[0], 28) as usize & (ENERGYPLUS_PSY_CACHE_SIZE - 1),
            )),
            F::TsatPb => Some((
                2,
                signed_tag(x[0], 28) as usize & (ENERGYPLUS_PSY_CACHE_SIZE - 1),
            )),
            F::TsatHPb => Some((
                3,
                (signed_tag(x[0], 24) ^ signed_tag(x[1], 24)) as usize
                    & (ENERGYPLUS_PSY_CACHE_SIZE - 1),
            )),
            _ => None,
        }
    }

    fn slot(&self, kind: usize, index: usize) -> EnergyPlusPsychrometricCacheSlot {
        self.slots[kind]
            .get(&index)
            .copied()
            .unwrap_or_else(|| default_slot(kind, index))
    }

    fn psat(&mut self, t: f64) -> f64 {
        let tag = signed_tag(t, 28);
        let index = tag as usize & (ENERGYPLUS_PSY_CACHE_SIZE - 1);
        let EnergyPlusPsychrometricCacheSlot::Psat { i_tdb, value, .. } = self.slot(1, index)
        else {
            unreachable!()
        };
        if i_tdb == tag {
            return value;
        }
        self.slots[1].insert(
            index,
            EnergyPlusPsychrometricCacheSlot::Psat {
                index,
                i_tdb: tag,
                value,
            },
        );
        let value = energyplus_psy_psat_fn_temp_raw(representative(tag, 28));
        self.slots[1].insert(
            index,
            EnergyPlusPsychrometricCacheSlot::Psat {
                index,
                i_tdb: tag,
                value,
            },
        );
        value
    }

    fn tsat_pb(&mut self, p: f64) -> f64 {
        let tag = signed_tag(p, 28);
        let index = tag as usize & (ENERGYPLUS_PSY_CACHE_SIZE - 1);
        let EnergyPlusPsychrometricCacheSlot::TsatPb {
            i_h, i_pb, value, ..
        } = self.slot(2, index)
        else {
            unreachable!()
        };
        if i_pb == tag {
            return value;
        }
        self.slots[2].insert(
            index,
            EnergyPlusPsychrometricCacheSlot::TsatPb {
                index,
                i_h,
                i_pb: tag,
                value,
            },
        );
        let value = self.tsat_pb_raw(p);
        self.slots[2].insert(
            index,
            EnergyPlusPsychrometricCacheSlot::TsatPb {
                index,
                i_h,
                i_pb: tag,
                value,
            },
        );
        value
    }

    fn tsat_pb_raw(&mut self, p: f64) -> f64 {
        if p == self.state.press_save {
            return self.state.t_sat_save;
        }
        self.state.press_save = p;
        let result = if p >= 1555000.0 {
            200.0
        } else if p <= 0.0017 {
            -100.0
        } else if p > 611.0 && p < 611.25 {
            0.0
        } else {
            let (mut t, mut x1, mut y1) = (100.0, 0.0, 0.0);
            for iteration in 1..=ENERGYPLUS_TSAT_PRESSURE_MAX_ITERATIONS {
                let error = p - self.psat(t);
                let (next, converged) = energyplus_general_iterate(
                    t,
                    error,
                    &mut x1,
                    &mut y1,
                    iteration,
                    ENERGYPLUS_PSYCHROMETRIC_ITERATION_TOLERANCE,
                );
                t = next;
                if converged {
                    break;
                }
            }
            t
        };
        self.state.t_sat_save = result;
        result
    }

    fn tsat_h_pb(&mut self, h: f64, p: f64) -> f64 {
        let ht = signed_tag(h, 24);
        let pt = signed_tag(p, 24);
        let index = (ht ^ pt) as usize & (ENERGYPLUS_PSY_CACHE_SIZE - 1);
        let EnergyPlusPsychrometricCacheSlot::TsatHPb {
            i_h, i_pb, value, ..
        } = self.slot(3, index)
        else {
            unreachable!()
        };
        if i_h == ht && i_pb == pt {
            return value;
        }
        self.slots[3].insert(
            index,
            EnergyPlusPsychrometricCacheSlot::TsatHPb {
                index,
                i_h: ht,
                i_pb: pt,
                value,
            },
        );
        let value = self.tsat_h_pb_raw(h, p);
        self.slots[3].insert(
            index,
            EnergyPlusPsychrometricCacheSlot::TsatHPb {
                index,
                i_h: ht,
                i_pb: pt,
                value,
            },
        );
        value
    }

    fn tsat_h_pb_raw(&mut self, h: f64, p: f64) -> f64 {
        energyplus_psy_tsat_fn_h_pb_raw_with_properties(h, p, |t, pressure| {
            energyplus_psy_h_fn_tdb_w(t, self.w_from_twb(t, t, pressure))
        })
    }

    fn w_from_rh(&mut self, t: f64, rh: f64, p: f64) -> f64 {
        let dew = rh * self.psat(t);
        let difference = p - dew;
        let denominator = if difference < 1000.0 {
            1000.0
        } else {
            difference
        };
        let w = dew * 0.62198 / denominator;
        if w < 1.0e-5 { 1.0e-5 } else { w }
    }

    fn w_from_twb(&mut self, t: f64, wet: f64, p: f64) -> f64 {
        let wet = if wet > t { t } else { wet };
        let psat = self.psat(wet);
        let star = 0.62198 * psat / (p - psat);
        let w = ((2501.0 - 2.381 * wet) * star - (t - wet)) / (2501.0 + 1.805 * t - 4.186 * wet);
        if w < 0.0 {
            self.w_from_rh(t, 0.0001, p)
        } else {
            w
        }
    }

    fn twb(&mut self, t: f64, w: f64, p: f64) -> f64 {
        let tt = t.to_bits() >> 32;
        let wt = w.to_bits() >> 32;
        let pt = p.to_bits() >> 32;
        let index = (tt ^ (wt ^ pt)) as usize & (ENERGYPLUS_PSY_CACHE_SIZE - 1);
        let EnergyPlusPsychrometricCacheSlot::Twb {
            i_tdb,
            i_w,
            i_pb,
            value,
            ..
        } = self.slot(0, index)
        else {
            unreachable!()
        };
        if i_tdb == tt && i_w == wt && i_pb == pt {
            return value;
        }
        self.slots[0].insert(
            index,
            EnergyPlusPsychrometricCacheSlot::Twb {
                index,
                i_tdb: tt,
                i_w: wt,
                i_pb: pt,
                value,
            },
        );
        let value = self.twb_raw(
            f64::from_bits(tt << 32),
            f64::from_bits(wt << 32),
            f64::from_bits(pt << 32),
        );
        self.slots[0].insert(
            index,
            EnergyPlusPsychrometricCacheSlot::Twb {
                index,
                i_tdb: tt,
                i_w: wt,
                i_pb: pt,
                value,
            },
        );
        value
    }

    fn twb_raw(&mut self, t: f64, w: f64, p: f64) -> f64 {
        let w = if w < 0.0 { 1.0e-5 } else { w };
        let boil = if p != self.state.last_patm {
            let boil = self.tsat_pb(p);
            self.state.last_patm = p;
            self.state.last_t_boil = boil;
            boil
        } else {
            self.state.last_t_boil
        };
        let (mut wet, mut x1, mut y1) = (t, 0.0, 0.0);
        for iteration in 1..=ENERGYPLUS_WET_BULB_MAX_ITERATIONS {
            if wet >= boil - 0.09 {
                wet = boil - 0.1;
            }
            let psat = self.psat(wet);
            let star = 0.62198 * psat / (p - psat);
            let new_w = if wet >= 0.0 {
                ((2501.0 - 2.326 * wet) * star - 1.006 * (t - wet))
                    / (2501.0 + 1.86 * t - 4.186 * wet)
            } else {
                ((2830.0 - 0.24 * wet) * star - 1.006 * (t - wet)) / (2830.0 + 1.86 * t - 2.1 * wet)
            };
            let (next, converged) = energyplus_general_iterate(
                wet,
                w - new_w,
                &mut x1,
                &mut y1,
                iteration,
                self.state.iconv_tol,
            );
            wet = next;
            if converged {
                break;
            }
        }
        if wet > t { t } else { wet }
    }
}

fn signed_tag(value: f64, shift: u32) -> i64 {
    (value.to_bits() as i64) >> shift
}
fn representative(tag: i64, shift: u32) -> f64 {
    f64::from_bits((tag as u64) << shift)
}
fn default_slot(kind: usize, index: usize) -> EnergyPlusPsychrometricCacheSlot {
    use EnergyPlusPsychrometricCacheSlot as S;
    match kind {
        0 => S::Twb {
            index,
            i_tdb: 0,
            i_w: 0,
            i_pb: 0,
            value: 0.0,
        },
        1 => S::Psat {
            index,
            i_tdb: -1000,
            value: 0.0,
        },
        2 => S::TsatPb {
            index,
            i_h: 0,
            i_pb: 0,
            value: 0.0,
        },
        _ => S::TsatHPb {
            index,
            i_h: 0,
            i_pb: 0,
            value: 0.0,
        },
    }
}
fn slot_bits_equal(
    a: EnergyPlusPsychrometricCacheSlot,
    b: EnergyPlusPsychrometricCacheSlot,
) -> bool {
    use EnergyPlusPsychrometricCacheSlot as S;
    match (a, b) {
        (
            S::Twb {
                i_tdb: a,
                i_w: b,
                i_pb: c,
                value: d,
                ..
            },
            S::Twb {
                i_tdb: e,
                i_w: f,
                i_pb: g,
                value: h,
                ..
            },
        ) => a == e && b == f && c == g && d.to_bits() == h.to_bits(),
        (
            S::Psat {
                i_tdb: a, value: b, ..
            },
            S::Psat {
                i_tdb: c, value: d, ..
            },
        ) => a == c && b.to_bits() == d.to_bits(),
        (
            S::TsatPb {
                i_h: a,
                i_pb: b,
                value: c,
                ..
            },
            S::TsatPb {
                i_h: d,
                i_pb: e,
                value: f,
                ..
            },
        )
        | (
            S::TsatHPb {
                i_h: a,
                i_pb: b,
                value: c,
                ..
            },
            S::TsatHPb {
                i_h: d,
                i_pb: e,
                value: f,
                ..
            },
        ) => a == d && b == e && c.to_bits() == f.to_bits(),
        _ => false,
    }
}

thread_local! { static PROPERTY_STATE: RefCell<EnergyPlusPsychrometricsState> = RefCell::new(EnergyPlusPsychrometricsState::default()); }
struct StateGuard {
    previous: Option<EnergyPlusPsychrometricsState>,
}
impl Drop for StateGuard {
    fn drop(&mut self) {
        if let Some(previous) = self.previous.take() {
            PROPERTY_STATE.with_borrow_mut(|state| *state = previous);
        }
    }
}
/// Isolates one run's property state; restores outer state on return or unwind.
/// The independent PSY-01 process-static Cp owners are left intact.
pub fn with_fresh_psychrometric_state<R>(execute: impl FnOnce() -> R) -> R {
    let previous = PROPERTY_STATE.with_borrow_mut(std::mem::take);
    let _guard = StateGuard {
        previous: Some(previous),
    };
    execute()
}
pub(super) fn with_state<R>(execute: impl FnOnce(&mut EnergyPlusPsychrometricsState) -> R) -> R {
    PROPERTY_STATE.with_borrow_mut(execute)
}

#[cfg(test)]
mod tests;
