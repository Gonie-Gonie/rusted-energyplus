//! Opt-in actual kernel observations on the executing thread.
//!
//! Exact tuples are interned; ordered IDs retain every call including repeats.
//! No calculation inputs, numerical/cache state or operation results are changed.
//! Worker threads are outside this collector; EnergyPlus stages are not inferred.

mod context;
pub use context::{
    CalendarObservation, EnvironmentObservation, ExecutionContext, ExecutionContextGuard,
    SystemCallObservation, ZoneTimestepObservation, output_step, register_environment_axis,
    register_time_axis, system_call, zone_step,
};

use super::EnergyPlusCpAirCacheState;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::panic::Location;

const EVENT_LIMIT: usize = 20_000_000;
const UNIQUE_TUPLE_LIMIT: usize = 100_000;

thread_local! {
    static ACTIVE_TRACE: RefCell<Option<TraceBuffer>> = RefCell::default();
}

/// Actual caller location supplied by `track_caller`.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PsychrometricCallSite {
    /// Rust source path.
    pub file: &'static str,
    /// One-based source line.
    pub line: u32,
    /// One-based source column.
    pub column: u32,
}

/// Real Cp state immediately around the numerical call.
#[derive(Clone, Debug)]
pub struct CpCacheObservation {
    /// Actual state before the call, including the cold sentinel.
    pub before: EnergyPlusCpAirCacheState,
    /// Actual state after the call.
    pub after: EnergyPlusCpAirCacheState,
    /// Source's exact `lastW == W` comparison.
    pub hit: bool,
}

/// Distinct exact tuple retained at its first actual occurrence.
#[derive(Clone, Debug)]
pub struct PsychrometricCall {
    /// First occurrence; every occurrence order is in `ordered_ids`.
    pub sequence: u64,
    /// Exact source routine name.
    pub routine: &'static str,
    /// Actual Rust pipeline scope.
    pub phase: &'static str,
    /// Actual Rust caller, never an inferred EnergyPlus call site.
    pub caller: PsychrometricCallSite,
    /// Bound invocation or referenced-output context, when available.
    pub context: Option<ExecutionContext>,
    /// Input IEEE bits in source argument order.
    pub input_bits: Vec<u64>,
    /// Already computed output IEEE bits.
    pub result_bits: u64,
    /// Actual state of a cache-owning Cp call.
    pub cp_cache: Option<CpCacheObservation>,
}

/// Bounded, lossless ordered prefix of the collecting thread's observations.
#[derive(Clone, Debug)]
pub struct PsychrometricProductionTrace {
    /// Maximum stored ordered event IDs.
    pub event_limit: usize,
    /// Maximum distinct exact tuples.
    pub unique_tuple_limit: usize,
    /// All calls, including explicitly omitted trailing calls.
    pub total_call_count: u64,
    /// Calls omitted after the first storage limit was reached.
    pub omitted_call_count: u64,
    /// First exhausted bound, or `None` for complete captures.
    pub truncation_reason: Option<&'static str>,
    /// Exact totals for each observed routine.
    pub routine_counts: BTreeMap<&'static str, u64>,
    /// Dictionary in first-appearance order; rows may occur many times.
    pub calls: Vec<PsychrometricCall>,
    /// Dictionary IDs in actual order. Index plus one is call sequence.
    pub ordered_ids: Vec<u32>,
}

#[derive(Clone, Eq, Hash, PartialEq)]
enum InputBits {
    One([u64; 1]),
    Two([u64; 2]),
    Three([u64; 3]),
    Other(Box<[u64]>),
}
impl InputBits {
    fn from_inputs(inputs: &[f64]) -> Self {
        match inputs {
            [a] => Self::One([a.to_bits()]),
            [a, b] => Self::Two([a.to_bits(), b.to_bits()]),
            [a, b, c] => Self::Three([a.to_bits(), b.to_bits(), c.to_bits()]),
            other => Self::Other(other.iter().map(|input| input.to_bits()).collect()),
        }
    }
    fn as_slice(&self) -> &[u64] {
        match self {
            Self::One(bits) => bits,
            Self::Two(bits) => bits,
            Self::Three(bits) => bits,
            Self::Other(bits) => bits,
        }
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct CallKey {
    routine: &'static str,
    phase: &'static str,
    caller: PsychrometricCallSite,
    context: Option<ExecutionContext>,
    inputs: InputBits,
    result_bits: u64,
    cp_cache: Option<([u64; 2], [u64; 2], bool)>,
}
struct TraceBuffer {
    phase: &'static str,
    context: Option<ExecutionContext>,
    hourly_calendar: Vec<CalendarObservation>,
    environment_points: Vec<EnvironmentObservation>,
    event_limit: usize,
    unique_tuple_limit: usize,
    truncation_reason: Option<&'static str>,
    total_call_count: u64,
    routine_counts: BTreeMap<&'static str, u64>,
    dictionary_ids: HashMap<CallKey, u32>,
    calls: Vec<PsychrometricCall>,
    ordered_ids: Vec<u32>,
}
struct CaptureGuard {
    previous: Option<TraceBuffer>,
}
impl Drop for CaptureGuard {
    fn drop(&mut self) {
        ACTIVE_TRACE.with(|active| *active.borrow_mut() = self.previous.take());
    }
}
struct PhaseGuard {
    previous: Option<&'static str>,
}
impl Drop for PhaseGuard {
    fn drop(&mut self) {
        if let Some(previous) = self.previous {
            ACTIVE_TRACE.with(|active| {
                if let Some(trace) = active.borrow_mut().as_mut() {
                    trace.phase = previous;
                }
            });
        }
    }
}

/// Executes unchanged; disabled captures allocate nothing. Nested collectors
/// restore their predecessor on return or panic. Bound exhaustion omits only a
/// contiguous trailing suffix, while total routine counts continue.
pub fn capture<R>(
    enabled: bool,
    execute: impl FnOnce() -> R,
) -> (R, Option<PsychrometricProductionTrace>) {
    if !enabled {
        return (execute(), None);
    }
    capture_with_limits(EVENT_LIMIT, UNIQUE_TUPLE_LIMIT, execute)
}
fn capture_with_limits<R>(
    event_limit: usize,
    unique_tuple_limit: usize,
    execute: impl FnOnce() -> R,
) -> (R, Option<PsychrometricProductionTrace>) {
    let previous = ACTIVE_TRACE.with(|active| {
        active.borrow_mut().replace(TraceBuffer {
            phase: "pipeline",
            context: None,
            hourly_calendar: Vec::new(),
            environment_points: Vec::new(),
            event_limit,
            unique_tuple_limit,
            truncation_reason: None,
            total_call_count: 0,
            routine_counts: BTreeMap::new(),
            dictionary_ids: HashMap::new(),
            calls: Vec::new(),
            ordered_ids: Vec::new(),
        })
    });
    let _guard = CaptureGuard { previous };
    let result = execute();
    let trace = ACTIVE_TRACE.with(|active| {
        active
            .borrow_mut()
            .take()
            .map(|buffer| PsychrometricProductionTrace {
                event_limit: buffer.event_limit,
                unique_tuple_limit: buffer.unique_tuple_limit,
                total_call_count: buffer.total_call_count,
                omitted_call_count: buffer.total_call_count - buffer.ordered_ids.len() as u64,
                truncation_reason: buffer.truncation_reason,
                routine_counts: buffer.routine_counts,
                calls: buffer.calls,
                ordered_ids: buffer.ordered_ids,
            })
    });
    (result, trace)
}

/// Tags only the duration of the actual Rust operation.
pub fn in_phase<R>(phase: &'static str, execute: impl FnOnce() -> R) -> R {
    let previous = ACTIVE_TRACE.with(|active| {
        active
            .borrow_mut()
            .as_mut()
            .map(|trace| std::mem::replace(&mut trace.phase, phase))
    });
    let _guard = PhaseGuard { previous };
    execute()
}
#[track_caller]
pub(crate) fn record(routine: &'static str, inputs: &[f64], result: f64) {
    record_call(routine, inputs, result, None);
}
#[track_caller]
pub(crate) fn record_cp(
    routine: &'static str,
    inputs: &[f64],
    result: f64,
    before: EnergyPlusCpAirCacheState,
    after: EnergyPlusCpAirCacheState,
) {
    let hit = inputs
        .first()
        .is_some_and(|input| before.humidity_ratio_kg_per_kg == *input);
    record_call(
        routine,
        inputs,
        result,
        Some(CpCacheObservation { before, after, hit }),
    );
}
fn cache_bits(state: EnergyPlusCpAirCacheState) -> [u64; 2] {
    [
        state.humidity_ratio_kg_per_kg.to_bits(),
        state.specific_heat_j_per_kg_k.to_bits(),
    ]
}
#[track_caller]
fn record_call(
    routine: &'static str,
    inputs: &[f64],
    result: f64,
    cp_cache: Option<CpCacheObservation>,
) {
    let caller = Location::caller();
    ACTIVE_TRACE.with(|active| {
        let mut active = active.borrow_mut();
        let Some(trace) = active.as_mut() else {
            return;
        };
        trace.total_call_count += 1;
        *trace.routine_counts.entry(routine).or_default() += 1;
        if trace.truncation_reason.is_some() {
            return;
        }
        if trace.ordered_ids.len() == trace.event_limit {
            trace.truncation_reason = Some("event_limit");
            return;
        }
        let key = CallKey {
            routine,
            phase: trace.phase,
            caller: PsychrometricCallSite {
                file: caller.file(),
                line: caller.line(),
                column: caller.column(),
            },
            context: trace.context,
            inputs: InputBits::from_inputs(inputs),
            result_bits: result.to_bits(),
            cp_cache: cp_cache
                .as_ref()
                .map(|cache| (cache_bits(cache.before), cache_bits(cache.after), cache.hit)),
        };
        let id = if let Some(id) = trace.dictionary_ids.get(&key) {
            *id
        } else {
            if trace.calls.len() == trace.unique_tuple_limit {
                trace.truncation_reason = Some("unique_tuple_limit");
                return;
            }
            // Configured dictionary bound is 100,000, below u32::MAX.
            let id = trace.calls.len() as u32;
            trace.calls.push(PsychrometricCall {
                sequence: trace.total_call_count,
                routine,
                phase: key.phase,
                caller: key.caller.clone(),
                context: key.context,
                input_bits: key.inputs.as_slice().to_vec(),
                result_bits: key.result_bits,
                cp_cache,
            });
            trace.dictionary_ids.insert(key, id);
            id
        };
        trace.ordered_ids.push(id);
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn disabled_capture_leaves_return_value_and_tls_unchanged() {
        let (result, trace) = capture(false, || {
            let _context = zone_step(0, 1, 4, 900.0);
            record("PsyCpAirFnW", &[0.008], 1020.0);
            -0.0_f64
        });
        assert_eq!(result.to_bits(), (-0.0_f64).to_bits());
        assert!(trace.is_none());
        assert!(ACTIVE_TRACE.with(|active| active.borrow().is_none()));
    }
    #[test]
    fn captures_exact_ieee_values_phase_and_actual_caller_order() {
        let nan = f64::from_bits(0x7ff8_0000_0000_0042);
        let expected_line = line!() + 2;
        let (_, trace) = capture(true, || {
            record("PsyHFnTdbW", &[-0.0, nan], nan);
            in_phase("rust_runtime", || record("PsyCpAirFnW", &[0.008], 1020.0));
            record("PsyTdbFnHW", &[1.0, 2.0], 3.0);
        });
        let trace = trace.expect("enabled capture");
        assert_eq!(trace.total_call_count, 3);
        assert_eq!(trace.omitted_call_count, 0);
        assert_eq!(trace.ordered_ids, [0, 1, 2]);
        assert_eq!(
            trace.calls[0].input_bits,
            [(-0.0_f64).to_bits(), nan.to_bits()]
        );
        assert_eq!(trace.calls[0].result_bits, nan.to_bits());
        assert_eq!(trace.calls[0].caller.line, expected_line);
        assert_eq!(trace.calls[1].phase, "rust_runtime");
        assert_eq!(trace.calls[2].phase, "pipeline");
        assert_eq!(trace.calls[2].sequence, 3);
    }
    #[test]
    fn dictionary_preserves_repetitions_cache_state_and_scoped_context() {
        let cold = EnergyPlusCpAirCacheState::default();
        let hot = EnergyPlusCpAirCacheState {
            humidity_ratio_kg_per_kg: 0.008,
            specific_heat_j_per_kg_k: 1020.0,
        };
        let (_, trace) = capture(true, || {
            for (hour, before) in [(0, cold), (0, hot), (0, hot), (1, hot), (0, hot)] {
                let _context = zone_step(hour, 1, 4, 900.0);
                record_cp("PsyCpAirFnW", &[0.008], 1020.0, before, hot);
            }
            let _validation = output_step(0, 4, 900.0);
            for _ in 0..2 {
                record_cp("PsyCpAirFnW", &[0.008], 1020.0, hot, hot);
            }
        });
        let trace = trace.expect("capture");
        assert_eq!(trace.ordered_ids, [0, 1, 1, 2, 1, 3, 3]);
        assert_eq!(trace.calls.len(), 4);
        assert!(!trace.calls[0].cp_cache.as_ref().expect("cache").hit);
        assert!(trace.calls[1].cp_cache.as_ref().expect("cache").hit);
        assert_eq!(
            trace.calls[2]
                .context
                .expect("context")
                .zone_timestep
                .expect("zone")
                .hour_index,
            1
        );
        assert_eq!(
            trace.calls[3].context.expect("context").scope,
            "coupled_output_validation"
        );
        assert_eq!(trace.omitted_call_count, 0);
    }
    #[test]
    fn storage_bounds_report_a_contiguous_omitted_suffix() {
        for (events, unique, expected_ids, reason) in [
            (2, 10, vec![0, 1], "event_limit"),
            (10, 1, vec![0], "unique_tuple_limit"),
        ] {
            let (_, trace) = capture_with_limits(events, unique, || {
                for input in [1.0, 2.0, 1.0, 3.0] {
                    record("test", &[input], input);
                }
            });
            let trace = trace.expect("capture");
            assert_eq!(trace.ordered_ids, expected_ids);
            assert_eq!(trace.total_call_count, 4);
            assert_eq!(trace.omitted_call_count, 4 - trace.ordered_ids.len() as u64);
            assert_eq!(trace.truncation_reason, Some(reason));
            assert_eq!(trace.routine_counts["test"], 4);
        }
    }
    #[test]
    #[allow(
        clippy::panic,
        reason = "Exercise restoration after a caller closure panics."
    )]
    fn nested_capture_and_unwinding_restore_the_outer_collector() {
        let (_, outer) = capture(true, || {
            let (_, inner) = capture(true, || record("PsyHFnTdbW", &[1.0, 2.0], 3.0));
            assert_eq!(inner.expect("inner capture").total_call_count, 1);
            let panic = std::panic::catch_unwind(|| {
                capture(true, || {
                    in_phase("interrupted", || {
                        let _context = zone_step(0, 1, 4, 900.0);
                        panic!("test phase unwinding");
                    });
                });
            });
            assert!(panic.is_err());
            record("PsyCpAirFnW", &[0.008], 1020.0);
        });
        let outer = outer.expect("outer capture");
        assert_eq!(outer.total_call_count, 1);
        assert_eq!(outer.calls[0].phase, "pipeline");
        assert_eq!(outer.calls[0].context, None);
        assert!(ACTIVE_TRACE.with(|active| active.borrow().is_none()));
    }
}
