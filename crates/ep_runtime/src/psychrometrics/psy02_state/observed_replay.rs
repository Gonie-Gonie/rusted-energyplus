//! TEST ONLY: import authentic per-window BEFORE state, then observe real caches.
//! No Native files/formulas/expected outputs are available in normal production.
use super::{
    ENERGYPLUS_PSY_CACHE_SIZE, EnergyPlusPsychrometricCacheSlot as Slot,
    EnergyPlusPsychrometricStateSnapshot as Snapshot, EnergyPlusPsychrometricsState,
    PROPERTY_STATE,
};
use std::{
    cell::RefCell,
    collections::{BTreeSet, VecDeque},
    panic::{AssertUnwindSafe, catch_unwind, panic_any, resume_unwind},
};

const ENTRY_LIMIT: usize = 65_536;
const EVENT_LIMIT: usize = 65_536;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CacheRequest {
    Twb(u64, u64, u64),
    Psat(i64),
    TsatPb(i64),
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CacheStage {
    Before,
    Miss,
    TagWrite,
    ValueWrite,
    After,
}
#[derive(Clone, Debug)]
pub(crate) struct CacheEvent {
    pub(crate) stage: CacheStage,
    pub(crate) request: Option<CacheRequest>,
    pub(crate) slot: Slot,
    pub(crate) scalars: Snapshot,
}
#[derive(Clone, Debug)]
pub(crate) struct WindowBefore {
    pub(crate) scalars: Snapshot,
    /// Authentic first-touch BEFORE entries, in actual first-touch event order.
    pub(crate) first_touch: Vec<Slot>,
    pub(crate) tsat_precision_bits: u32,
}
#[derive(Debug)]
pub(crate) struct WindowObserved<R> {
    pub(crate) value: R,
    pub(crate) events: Vec<CacheEvent>,
    pub(crate) final_scalars: Snapshot,
    pub(crate) final_touched_slots: Vec<Slot>,
}
#[derive(Debug)]
pub(crate) struct ReplayUnknown(pub(crate) String, pub(crate) Vec<CacheEvent>);
impl std::fmt::Display for ReplayUnknown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for ReplayUnknown {}

struct WindowCapture {
    remaining: VecDeque<Slot>,
    imported: BTreeSet<(usize, usize)>,
    events: Vec<CacheEvent>,
}
std::thread_local! {
    static ACTIVE: RefCell<Option<WindowCapture>> = RefCell::default();
}

fn key(slot: Slot) -> (usize, usize) {
    match slot {
        Slot::Twb { index, .. } => (0, index),
        Slot::Psat { index, .. } => (1, index),
        Slot::TsatPb { index, .. } => (2, index),
        Slot::TsatHPb { index, .. } => (3, index),
    }
}
#[allow(clippy::panic)] // Typed test-only interruption prevents reading unknown cache memory.
fn unknown(message: impl Into<String>) -> ! {
    panic_any(ReplayUnknown(message.into(), Vec::new()))
}

struct WindowGuard {
    predecessor_owner: Option<EnergyPlusPsychrometricsState>,
    predecessor_capture: Option<WindowCapture>,
}
impl Drop for WindowGuard {
    fn drop(&mut self) {
        ACTIVE.with_borrow_mut(|active| *active = self.predecessor_capture.take());
        if let Some(owner) = self.predecessor_owner.take() {
            PROPERTY_STATE.with_borrow_mut(|state| *state = owner);
        }
    }
}

/// Executes on an isolated owner with authentic BEFORE scalars and guarded
/// unknown cache memory. Every actual first lookup must import its real entry;
/// default_slot is never accepted as evidence for an untouched selected key.
pub(crate) fn with_observed_window<R>(
    before: WindowBefore,
    execute: impl FnOnce() -> R,
) -> Result<WindowObserved<R>, ReplayUnknown> {
    if before.tsat_precision_bits != 24
        || before.scalars.use_interpolation
        || !before.scalars.iconv_tol.is_finite()
        || before.scalars.iconv_tol <= 0.0
        || before.first_touch.len() > ENTRY_LIMIT
    {
        return Err(ReplayUnknown(
            "unsupported observed window control/entry domain".into(),
            Vec::new(),
        ));
    }
    let mut keys = BTreeSet::new();
    for &slot in &before.first_touch {
        let (kind, index) = key(slot);
        if kind > 2 || index >= ENERGYPLUS_PSY_CACHE_SIZE || !keys.insert((kind, index)) {
            return Err(ReplayUnknown(
                "invalid/duplicate actual first-touch entry".into(),
                Vec::new(),
            ));
        }
    }
    let owner = EnergyPlusPsychrometricsState {
        state: before.scalars,
        slots: std::array::from_fn(|_| std::collections::BTreeMap::new()),
        // This numerical replay does not observe diagnostic state.
        ..EnergyPlusPsychrometricsState::default()
    };
    let predecessor_owner = PROPERTY_STATE.with_borrow_mut(|state| std::mem::replace(state, owner));
    let predecessor_capture = ACTIVE.with_borrow_mut(|active| {
        active.replace(WindowCapture {
            remaining: before.first_touch.into(),
            imported: BTreeSet::new(),
            events: Vec::new(),
        })
    });
    let guard = WindowGuard {
        predecessor_owner: Some(predecessor_owner),
        predecessor_capture,
    };
    let result = catch_unwind(AssertUnwindSafe(execute));
    let Some(capture) = ACTIVE.with_borrow_mut(Option::take) else {
        drop(guard);
        return Err(ReplayUnknown(
            "missing active observed window capture".into(),
            Vec::new(),
        ));
    };
    let (final_scalars, final_touched_slots) = PROPERTY_STATE.with_borrow(|state| {
        (
            state.snapshot(),
            capture
                .imported
                .iter()
                .map(|&(kind, index)| state.slot(kind, index))
                .collect(),
        )
    });
    drop(guard); // Always restore the complete predecessor before returning/resuming.
    match result {
        Ok(value) if capture.remaining.is_empty() => Ok(WindowObserved {
            value,
            events: capture.events,
            final_scalars,
            final_touched_slots,
        }),
        Ok(_) => Err(ReplayUnknown(
            format!(
                "{} actual first-touch entries were not consumed",
                capture.remaining.len()
            ),
            capture.events,
        )),
        Err(payload) => match payload.downcast::<ReplayUnknown>() {
            Ok(mut error) => {
                error.1 = capture.events;
                Err(*error)
            }
            Err(other) => resume_unwind(other),
        },
    }
}

impl EnergyPlusPsychrometricsState {
    pub(super) fn import_observed_before(&mut self, kind: usize, index: usize) {
        ACTIVE.with_borrow_mut(|active| {
            let Some(capture) = active else {
                return;
            };
            if capture.imported.contains(&(kind, index)) {
                return;
            }
            let Some(&slot) = capture.remaining.front() else {
                unknown(format!(
                    "Rust lookup has no actual BEFORE entry: {kind}/{index}"
                ));
            };
            if key(slot) != (kind, index) {
                unknown(format!(
                    "actual first-touch order differs: expected {:?}, Rust {kind}/{index}",
                    key(slot)
                ));
            }
            capture.remaining.pop_front();
            self.slots[kind].insert(index, slot); // BEFORE import only, never an expected AFTER/result.
            capture.imported.insert((kind, index));
        });
    }

    pub(super) fn observe_cache_before(
        &mut self,
        kind: usize,
        index: usize,
        request: CacheRequest,
    ) {
        self.import_observed_before(kind, index);
        self.record_cache_event(CacheStage::Before, kind, index, Some(request));
    }
    pub(super) fn observe_cache_point(&self, stage: CacheStage, kind: usize, index: usize) {
        self.record_cache_event(stage, kind, index, None);
    }
    fn record_cache_event(
        &self,
        stage: CacheStage,
        kind: usize,
        index: usize,
        request: Option<CacheRequest>,
    ) {
        ACTIVE.with_borrow_mut(|active| {
            let Some(capture) = active else {
                return;
            };
            if capture.events.len() == EVENT_LIMIT {
                unknown("observed cache event limit");
            }
            if !capture.imported.contains(&(kind, index)) {
                unknown("cache event precedes authentic BEFORE import");
            }
            capture.events.push(CacheEvent {
                stage,
                request,
                slot: self.slot(kind, index),
                scalars: self.snapshot(),
            });
        });
    }
}
