//! Passive copies of actual raw preparation and existing weather consumers.
//!
//! Source indices belong to the prepared Rust environment. Consumer indices
//! belong to its projected series. Neither represents a native callback stage.

use super::{RawEpwHeaderState, RawEpwOutputs, RawEpwStreamState};
use crate::psychrometrics::production_trace::{ExecutionContext, current_execution_scope};
use crate::weather::{EpwRecord, WeatherTimestepSample};
use std::{
    cell::RefCell,
    panic::Location,
    path::{Path, PathBuf},
};

/// Maximum retained entries in each observation series and selected-record pool.
pub const OBSERVATION_LIMIT: usize = 10_000;

thread_local! {
    static ACTIVE: RefCell<Option<WeatherProductionTrace>> = RefCell::default();
}

/// A raw owner output and the literal projected record passed to selection.
#[derive(Clone, Copy, Debug)]
pub struct PreparedWeatherRecord {
    /// Actual zero-based index in the loaded source records.
    pub source_record_index: usize,
    /// Actual raw parser output, before physical compatibility projection.
    pub raw: RawEpwOutputs,
    /// Actual selected hourly compatibility record.
    pub projected: EpwRecord,
}

/// Actual runtime preparation after the existing environment selection returns.
#[derive(Clone, Debug)]
pub struct PreparedWeatherObservation {
    /// One-based actual preparation call count on this thread.
    pub sequence: u64,
    /// Actual caller of the passive preparation hook.
    pub caller: &'static Location<'static>,
    /// Existing Rust execution phase.
    pub phase: &'static str,
    /// Existing invocation context, when present.
    pub context: Option<ExecutionContext>,
    /// Actual supplied weather path; input byte hashes remain launcher evidence.
    pub path: PathBuf,
    /// Actual length of the supplied weather bytes.
    pub byte_length: usize,
    /// Actual selected raw header state after loading.
    pub header: RawEpwHeaderState,
    /// Actual supplied-byte stream observation after header parsing.
    pub stream_after_header: RawEpwStreamState,
    /// Actual supplied-byte stream observation after loading all records.
    pub final_stream: RawEpwStreamState,
    /// Total actual raw records loaded, including records not selected here.
    pub raw_record_count: usize,
    /// All selected records supplied to this hook, including an omitted suffix.
    pub selected_record_count: usize,
    /// Contiguous prefix of actual source-index/raw/projected tuples.
    pub selected_records: Vec<PreparedWeatherRecord>,
}

/// The existing consumer's actual hourly record and optional precomputed sample.
#[derive(Clone, Debug)]
pub struct WeatherConsumerObservation {
    /// One-based actual consumer-hook order; repeated identities are retained.
    pub sequence: u64,
    /// Actual caller, before existing weather-context consumption.
    pub caller: &'static Location<'static>,
    /// Existing Rust execution phase, not an inferred native weather stage.
    pub phase: &'static str,
    /// Existing actual invocation or referenced-output context.
    pub context: Option<ExecutionContext>,
    /// Actual index within the selected projected hourly series.
    pub record_index: usize,
    /// Actual one-based zone substep operand.
    pub zone_timestep: u32,
    /// Literal hourly record received by the consumer.
    pub record: EpwRecord,
    /// Already-computed sample received by the consumer, if available.
    pub sample: Option<WeatherTimestepSample>,
}

/// Bounded ordered copies; counters include every active hook invocation.
#[derive(Clone, Debug, Default)]
pub struct WeatherProductionTrace {
    /// All preparation hook calls on the collecting thread.
    pub total_prepared_count: u64,
    /// All selected tuples supplied to preparation hooks.
    pub total_selected_record_count: u64,
    /// Number of retained tuples in the bounded shared selected-record pool.
    pub retained_selected_record_count: usize,
    /// Contiguous prefix of actual preparation calls.
    pub prepared: Vec<PreparedWeatherObservation>,
    /// All consumer hook calls, including a potentially omitted suffix.
    pub total_consumer_count: u64,
    /// Contiguous prefix of actual consumer calls, without deduplication.
    pub consumers: Vec<WeatherConsumerObservation>,
}

struct Guard(Option<WeatherProductionTrace>);
impl Drop for Guard {
    fn drop(&mut self) {
        ACTIVE.with_borrow_mut(|active| *active = self.0.take());
    }
}

/// Observe one thread while running the unchanged closure. Disabled calls
/// allocate nothing; nested and unwinding calls restore the previous collector.
pub fn capture<R>(
    enabled: bool,
    execute: impl FnOnce() -> R,
) -> (R, Option<WeatherProductionTrace>) {
    if !enabled {
        return (execute(), None);
    }
    let previous =
        ACTIVE.with_borrow_mut(|active| active.replace(WeatherProductionTrace::default()));
    let _guard = Guard(previous);
    let result = execute();
    let trace = ACTIVE.with_borrow_mut(Option::take);
    (result, trace)
}

/// Whether the collecting thread has an active observer.
#[must_use]
pub fn is_active() -> bool {
    ACTIVE.with_borrow(Option::is_some)
}

/// Copy actual load/selection operands after the production selection returns.
/// This hook does not open files, parse records or perform compatibility policy.
#[track_caller]
pub fn record_prepared(
    path: &Path,
    byte_length: usize,
    header: &RawEpwHeaderState,
    stream_after_header: RawEpwStreamState,
    final_stream: RawEpwStreamState,
    raw_record_count: usize,
    selected: &[(usize, RawEpwOutputs, EpwRecord)],
) {
    if !is_active() {
        return;
    }
    let caller = Location::caller();
    let (phase, context) = current_execution_scope();
    ACTIVE.with_borrow_mut(|active| {
        let Some(trace) = active else { return };
        trace.total_prepared_count += 1;
        trace.total_selected_record_count += selected.len() as u64;
        if trace.prepared.len() == OBSERVATION_LIMIT {
            return;
        }
        let retained = selected
            .len()
            .min(OBSERVATION_LIMIT - trace.retained_selected_record_count);
        let selected_records = selected
            .iter()
            .take(retained)
            .map(
                |(source_record_index, raw, projected)| PreparedWeatherRecord {
                    source_record_index: *source_record_index,
                    raw: *raw,
                    projected: *projected,
                },
            )
            .collect();
        trace.retained_selected_record_count += retained;
        trace.prepared.push(PreparedWeatherObservation {
            sequence: trace.total_prepared_count,
            caller,
            phase,
            context,
            path: path.to_path_buf(),
            byte_length,
            header: header.clone(),
            stream_after_header,
            final_stream,
            raw_record_count,
            selected_record_count: selected.len(),
            selected_records,
        });
    });
}

/// Copy the existing consumer's operands before any weather-context reads.
#[track_caller]
pub(crate) fn record_consumer(
    record_index: usize,
    zone_timestep: u32,
    record: &EpwRecord,
    sample: Option<&WeatherTimestepSample>,
) {
    if !is_active() {
        return;
    }
    let caller = Location::caller();
    let (phase, context) = current_execution_scope();
    ACTIVE.with_borrow_mut(|active| {
        let Some(trace) = active else { return };
        trace.total_consumer_count += 1;
        if trace.consumers.len() == OBSERVATION_LIMIT {
            return;
        }
        trace.consumers.push(WeatherConsumerObservation {
            sequence: trace.total_consumer_count,
            caller,
            phase,
            context,
            record_index,
            zone_timestep,
            record: *record,
            sample: sample.copied(),
        });
    });
}
