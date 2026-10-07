//! Opt-in selected top-level property operations, preserving ordered replay.
//!
//! Nested original helper effects remain in the owning state and final caches;
//! they are not dispatched twice by replay. The existing PSY-01 trace is separate.

use super::{
    EnergyPlusPsychrometricFinalCaches, EnergyPlusPsychrometricOperation,
    EnergyPlusPsychrometricStateSnapshot, production_trace, psy02_state,
};
use production_trace::{ExecutionContext, PsychrometricCallSite};
use std::{
    cell::{Cell, RefCell},
    collections::{BTreeMap, HashMap},
    panic::Location,
};

const EVENT_LIMIT: usize = 20_000_000;
const UNIQUE_LIMIT: usize = 100_000;
thread_local! {
    static TRACE: RefCell<Option<Buffer>> = RefCell::default();
    static DEPTH: Cell<usize> = Cell::default();
}

/// A first-occurrence dictionary row of a real outer property invocation.
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct Psy02RootCall {
    pub sequence: u64,
    pub routine: &'static str,
    pub phase: &'static str,
    pub caller: PsychrometricCallSite,
    pub context: Option<ExecutionContext>,
    pub input_bits: Vec<u64>,
    pub operation: EnergyPlusPsychrometricOperation,
}

/// Ordered selected roots and the whole owner's final sparse cache state.
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct Psy02ProductionTrace {
    pub event_limit: usize,
    pub unique_tuple_limit: usize,
    pub total_root_count: u64,
    pub omitted_root_count: u64,
    pub truncation_reason: Option<&'static str>,
    pub routine_counts: BTreeMap<&'static str, u64>,
    pub dictionary: Vec<Psy02RootCall>,
    pub ordered_ids: Vec<u32>,
    pub initial_state: EnergyPlusPsychrometricStateSnapshot,
    pub final_state: EnergyPlusPsychrometricStateSnapshot,
    pub final_caches: EnergyPlusPsychrometricFinalCaches,
}

#[derive(Eq, Hash, PartialEq)]
struct Key {
    routine: &'static str,
    phase: &'static str,
    caller: PsychrometricCallSite,
    context: Option<ExecutionContext>,
    inputs: Vec<u64>,
    result: u64,
    before: ([u64; 5], bool, bool),
    after: ([u64; 5], bool, bool),
    slot_before: Option<[u64; 6]>,
    slot_after: Option<[u64; 6]>,
}
struct Buffer {
    event_limit: usize,
    unique_limit: usize,
    total: u64,
    truncation: Option<&'static str>,
    counts: BTreeMap<&'static str, u64>,
    ids: HashMap<Key, u32>,
    dictionary: Vec<Psy02RootCall>,
    ordered: Vec<u32>,
}
struct CaptureGuard(Option<Buffer>);
impl Drop for CaptureGuard {
    fn drop(&mut self) {
        TRACE.with_borrow_mut(|trace| *trace = self.0.take());
    }
}
pub(super) struct OperationGuard;
impl Drop for OperationGuard {
    fn drop(&mut self) {
        DEPTH.with(|depth| depth.set(depth.get() - 1));
    }
}
pub(super) fn enter() -> OperationGuard {
    DEPTH.with(|depth| depth.set(depth.get() + 1));
    OperationGuard
}

/// Captures only selected externally requested calls on this thread. Disabled
/// captures do not allocate or change property state. Nested/unwinding captures
/// restore the predecessor. Limits retain one contiguous prefix, never gaps.
pub fn capture<R>(enabled: bool, execute: impl FnOnce() -> R) -> (R, Option<Psy02ProductionTrace>) {
    if !enabled {
        return (execute(), None);
    }
    capture_with_limits(EVENT_LIMIT, UNIQUE_LIMIT, execute)
}
fn capture_with_limits<R>(
    event_limit: usize,
    unique_limit: usize,
    execute: impl FnOnce() -> R,
) -> (R, Option<Psy02ProductionTrace>) {
    let initial_state = psy02_state::with_state(|state| state.snapshot());
    let previous = TRACE.with_borrow_mut(|trace| {
        trace.replace(Buffer {
            event_limit,
            unique_limit,
            total: 0,
            truncation: None,
            counts: BTreeMap::new(),
            ids: HashMap::new(),
            dictionary: Vec::new(),
            ordered: Vec::new(),
        })
    });
    let _guard = CaptureGuard(previous);
    let result = execute();
    let (final_state, final_caches) =
        psy02_state::with_state(|state| (state.snapshot(), state.final_caches()));
    let trace = TRACE.with_borrow_mut(|trace| {
        trace.take().map(|buffer| Psy02ProductionTrace {
            event_limit: buffer.event_limit,
            unique_tuple_limit: buffer.unique_limit,
            total_root_count: buffer.total,
            omitted_root_count: buffer.total - buffer.ordered.len() as u64,
            truncation_reason: buffer.truncation,
            routine_counts: buffer.counts,
            dictionary: buffer.dictionary,
            ordered_ids: buffer.ordered,
            initial_state,
            final_state,
            final_caches,
        })
    });
    (result, trace)
}

#[track_caller]
pub(super) fn record(
    routine: &'static str,
    inputs: &[f64],
    operation: EnergyPlusPsychrometricOperation,
) {
    if DEPTH.with(Cell::get) != 1 {
        return;
    }
    record_root(routine, inputs, operation);
}
#[track_caller]
pub(super) fn record_stateless(routine: &'static str, inputs: &[f64], result: f64) {
    // Check both conditions before borrowing the numerical owner. A nested
    // W/H helper can execute while its outer property already owns that borrow.
    if !["PsyWFnTdbH", "PsyPsatFnTemp_raw"].contains(&routine)
        || DEPTH.with(Cell::get) != 0
        || !TRACE.with_borrow(Option::is_some)
    {
        return;
    }
    let state = psy02_state::with_state(|state| state.snapshot());
    record_root(
        routine,
        inputs,
        EnergyPlusPsychrometricOperation {
            result,
            state_before: state,
            state_after: state,
            cache_before: None,
            cache_after: None,
        },
    );
}
#[track_caller]
fn record_root(routine: &'static str, inputs: &[f64], operation: EnergyPlusPsychrometricOperation) {
    // Actual external raw calls also matter: their nested caches/scalars must
    // execute once in replay. Caller/context distinguish validation consumers
    // from physical producers; routine names alone do not claim HVAC coverage.
    let location = Location::caller();
    let (phase, context) = production_trace::current_execution_scope();
    TRACE.with_borrow_mut(|trace| {
        let Some(buffer) = trace.as_mut() else {
            return;
        };
        buffer.total += 1;
        *buffer.counts.entry(routine).or_default() += 1;
        if buffer.truncation.is_some() {
            return;
        }
        if buffer.ordered.len() == buffer.event_limit {
            buffer.truncation = Some("event_limit");
            return;
        }
        let caller = PsychrometricCallSite {
            file: location.file(),
            line: location.line(),
            column: location.column(),
        };
        let input_bits = inputs.iter().map(|x| x.to_bits()).collect::<Vec<_>>();
        let key = Key {
            routine,
            phase,
            caller: caller.clone(),
            context,
            inputs: input_bits.clone(),
            result: operation.result.to_bits(),
            before: state_key(operation.state_before),
            after: state_key(operation.state_after),
            slot_before: operation.cache_before.map(slot_key),
            slot_after: operation.cache_after.map(slot_key),
        };
        let id = if let Some(&id) = buffer.ids.get(&key) {
            id
        } else {
            if buffer.dictionary.len() == buffer.unique_limit {
                buffer.truncation = Some("unique_tuple_limit");
                return;
            }
            let id = buffer.dictionary.len() as u32;
            buffer.ids.insert(key, id);
            buffer.dictionary.push(Psy02RootCall {
                sequence: buffer.total,
                routine,
                phase,
                caller,
                context,
                input_bits,
                operation,
            });
            id
        };
        buffer.ordered.push(id);
    });
}
fn state_key(s: EnergyPlusPsychrometricStateSnapshot) -> ([u64; 5], bool, bool) {
    (
        [
            s.iconv_tol.to_bits(),
            s.last_patm.to_bits(),
            s.last_t_boil.to_bits(),
            s.press_save.to_bits(),
            s.t_sat_save.to_bits(),
        ],
        s.warmup,
        s.use_interpolation,
    )
}
fn slot_key(s: super::EnergyPlusPsychrometricCacheSlot) -> [u64; 6] {
    use super::EnergyPlusPsychrometricCacheSlot as S;
    match s {
        S::Twb {
            index,
            i_tdb,
            i_w,
            i_pb,
            value,
        } => [0, index as u64, i_tdb, i_w, i_pb, value.to_bits()],
        S::Psat {
            index,
            i_tdb,
            value,
        } => [1, index as u64, i_tdb as u64, 0, 0, value.to_bits()],
        S::TsatPb {
            index,
            i_h,
            i_pb,
            value,
        } => [2, index as u64, i_h as u64, i_pb as u64, 0, value.to_bits()],
        S::TsatHPb {
            index,
            i_h,
            i_pb,
            value,
        } => [3, index as u64, i_h as u64, i_pb as u64, 0, value.to_bits()],
    }
}

#[cfg(test)]
mod tests;
