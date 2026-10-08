//! Passive observations of live weather calls and actual thermal operands.

use super::{
    ProductionWeatherContext, WeatherDayError, WeatherDayPhase, WeatherDayState, WeatherSession,
};
use crate::weather::raw::{RawEpwHeaderState, RawEpwOutputs, RawEpwStreamState, RawReadProvenance};
use std::{cell::RefCell, panic::Location};

/// Bounded retained prefixes; total counters never deduplicate calls.
pub const OBSERVATION_LIMIT: usize = 10_000;

/// Actual owner state before or after an executed selected weather operation.
#[derive(Clone, Debug)]
pub struct WeatherSessionSnapshot {
    /// Full current daily and auxiliary owners.
    pub state: WeatherDayState,
    /// Actual raw header storage.
    pub header: RawEpwHeaderState,
    /// Actual semantic stream flags and available position.
    pub stream: RawEpwStreamState,
    /// Last actual parser reference-argument writes.
    pub raw_arguments: RawEpwOutputs,
    /// Actual supplied-byte cursor, including failed tellg states.
    pub cursor_byte: usize,
    /// Actual cumulative getline attempts.
    pub line_read_count: usize,
    /// Actual Rust parser invocations, independent of native local variables.
    pub interpret_count: usize,
    /// Selected environment availability.
    pub available: bool,
    /// Actual nonfatal header/caller error flag.
    pub errors_found: bool,
    /// Actual environment-stamp request.
    pub print_environment_stamp: bool,
    /// Selected source environment's first-day search cycle.
    pub current_cycle: i32,
    /// Selected source environment's weekday setup flag.
    pub set_week_days: bool,
}
impl WeatherSessionSnapshot {
    /// Copies actual owners without reading input or producing weather values.
    #[must_use]
    pub fn from_session(session: &WeatherSession) -> Self {
        Self {
            state: session.state.clone(),
            header: session.cursor.header().clone(),
            stream: session.cursor.stream_state(),
            raw_arguments: *session.cursor.last_arguments(),
            cursor_byte: session.cursor.byte_cursor(),
            line_read_count: session.cursor.line_reads(),
            interpret_count: session.cursor.interpret_count(),
            available: session.available,
            errors_found: session.errors_found,
            print_environment_stamp: session.print_environment_stamp,
            current_cycle: session.current_cycle,
            set_week_days: session.set_week_days,
        }
    }
}

/// One actual invocation, including nested read-day operations in source order.
#[derive(Clone, Debug)]
pub struct WeatherDayOperationObservation {
    /// One-based completion order, including nested calls.
    pub sequence: u64,
    /// Actual executed selected operation.
    pub kind: &'static str,
    /// Actual owners before invocation.
    pub before: WeatherSessionSnapshot,
    /// Actual owners after the returned or failed invocation.
    pub after: WeatherSessionSnapshot,
    /// Actual returned failure, if present.
    pub error: Option<WeatherDayError>,
}

/// Actual owned operands copied before a thermal consumer receives them.
#[derive(Clone, Debug)]
pub struct WeatherDayConsumerObservation {
    /// One-based hook order; repeated identities remain separate.
    pub sequence: u64,
    /// Actual completed-operation count when these operands reached the hook.
    pub completed_operation_count: u64,
    /// Actual hook caller.
    pub caller: &'static Location<'static>,
    /// Thermal caller's civil hourly index, not a native EPW record index.
    pub record_index: usize,
    /// Actual one-based zone timestep.
    pub timestep: u32,
    /// Actual thermal phase.
    pub phase: WeatherDayPhase,
    /// Owned operands received from Today.
    pub context: ProductionWeatherContext,
    /// Actual raw source row underlying that Today hour.
    pub raw: RawEpwOutputs,
    /// Observed byte range and actual getline attempt.
    pub provenance: RawReadProvenance,
    /// Actual caller clocks and flags at consumption.
    pub caller_state: super::WeatherGlobalState,
}

/// Ordered actual operations and consumers from one production invocation.
#[derive(Clone, Debug, Default)]
pub struct WeatherDayProductionTrace {
    /// All active operation hooks, including an omitted suffix.
    pub total_operation_count: u64,
    /// Retained contiguous operation completion prefix.
    pub operations: Vec<WeatherDayOperationObservation>,
    /// All active consumer hooks, including an omitted suffix.
    pub total_consumer_count: u64,
    /// Retained contiguous consumer prefix.
    pub consumers: Vec<WeatherDayConsumerObservation>,
}

thread_local! { static ACTIVE: RefCell<Option<WeatherDayProductionTrace>> = RefCell::default(); }
struct Guard(Option<WeatherDayProductionTrace>);
impl Drop for Guard {
    fn drop(&mut self) {
        ACTIVE.with_borrow_mut(|active| *active = self.0.take());
    }
}

/// Captures actual execution on this thread; nesting and unwinding restore state.
pub fn capture<R>(
    enabled: bool,
    execute: impl FnOnce() -> R,
) -> (R, Option<WeatherDayProductionTrace>) {
    if !enabled {
        return (execute(), None);
    }
    let previous = ACTIVE.with_borrow_mut(|active| active.replace(Default::default()));
    let _guard = Guard(previous);
    let result = execute();
    (result, ACTIVE.with_borrow_mut(Option::take))
}

pub(super) fn snapshot(session: &WeatherSession) -> Option<WeatherSessionSnapshot> {
    ACTIVE
        .with_borrow(|active| active.is_some())
        .then(|| WeatherSessionSnapshot::from_session(session))
}

pub(super) fn record_operation(
    kind: &'static str,
    before: Option<WeatherSessionSnapshot>,
    session: &WeatherSession,
    error: Option<&WeatherDayError>,
) {
    let Some(before) = before else {
        return;
    };
    ACTIVE.with_borrow_mut(|active| {
        let Some(trace) = active else {
            return;
        };
        trace.total_operation_count += 1;
        if trace.operations.len() < OBSERVATION_LIMIT {
            trace.operations.push(WeatherDayOperationObservation {
                sequence: trace.total_operation_count,
                kind,
                before,
                after: WeatherSessionSnapshot::from_session(session),
                error: error.cloned(),
            });
        }
    });
}

#[track_caller]
pub(super) fn record_consumer(
    record_index: usize,
    timestep: u32,
    phase: WeatherDayPhase,
    context: ProductionWeatherContext,
    session: &WeatherSession,
) {
    let caller = Location::caller();
    ACTIVE.with_borrow_mut(|active| {
        let Some(trace) = active else {
            return;
        };
        trace.total_consumer_count += 1;
        if trace.consumers.len() == OBSERVATION_LIMIT {
            return;
        }
        let Some(slot) = session
            .today_raw
            .as_ref()
            .and_then(|day| day.hours.get(record_index % 24))
        else {
            return;
        };
        trace.consumers.push(WeatherDayConsumerObservation {
            sequence: trace.total_consumer_count,
            completed_operation_count: trace.total_operation_count,
            caller,
            record_index,
            timestep,
            phase,
            context,
            raw: slot.raw,
            provenance: slot.provenance,
            caller_state: session.state.global.clone(),
        });
    });
}
