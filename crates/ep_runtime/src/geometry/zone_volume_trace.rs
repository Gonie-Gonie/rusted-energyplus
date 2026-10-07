//! Optional copies of the actual heat-balance volume owner's return value.
//!
//! Capturing supplies no operands and performs no geometry calculation. The
//! initializer records its real success or rejection before creating zone state.

use super::{ZoneGeometryProperties, ZoneVolumeError};
use crate::psychrometrics::production_trace::{ExecutionContext, current_execution_scope};
use ep_model::ZoneId;
use std::{cell::RefCell, panic::Location};

const EVENT_LIMIT: usize = 10_000;
thread_local! {
    static ACTIVE: RefCell<Option<ZoneVolumeTrace>> = RefCell::default();
}

/// One real initializer call, including a failure before any capacity consumer.
#[derive(Clone, Debug)]
pub struct ZoneVolumeObservation {
    /// Own typed identity, never a native array index.
    pub zone_id: ZoneId,
    /// Own typed name.
    pub zone_name: String,
    /// Actual caller location.
    pub caller: &'static Location<'static>,
    /// Actual enclosing Rust operation.
    pub phase: &'static str,
    /// Existing execution context, when available.
    pub context: Option<ExecutionContext>,
    /// Copy of the successful owner's result; no reconstruction.
    pub properties: Option<ZoneGeometryProperties>,
    /// Actual rejected result's display text.
    pub rejection: Option<String>,
}

/// Contiguous bounded observation prefix on the collecting thread.
#[derive(Clone, Debug, Default)]
pub struct ZoneVolumeTrace {
    /// All actual observed invocations, including any omitted suffix.
    pub total_call_count: u64,
    /// Actual return values in invocation order.
    pub observations: Vec<ZoneVolumeObservation>,
}

struct Guard(Option<ZoneVolumeTrace>);
impl Drop for Guard {
    fn drop(&mut self) {
        ACTIVE.with_borrow_mut(|active| *active = self.0.take());
    }
}

/// Execute unchanged; disabled captures allocate nothing. Restore nested state
/// on normal return and unwinding. Storage exhaustion preserves a prefix.
pub fn capture<R>(enabled: bool, execute: impl FnOnce() -> R) -> (R, Option<ZoneVolumeTrace>) {
    if !enabled {
        return (execute(), None);
    }
    let previous = ACTIVE.with_borrow_mut(|active| active.replace(ZoneVolumeTrace::default()));
    let _guard = Guard(previous);
    let result = execute();
    let trace = ACTIVE.with_borrow_mut(Option::take);
    (result, trace)
}

/// Copy the actual result already obtained by the heat-balance initializer.
#[track_caller]
pub(crate) fn record(
    zone_id: ZoneId,
    zone_name: &str,
    result: &Result<ZoneGeometryProperties, ZoneVolumeError>,
) {
    let caller = Location::caller();
    ACTIVE.with_borrow_mut(|active| {
        let Some(trace) = active else { return };
        trace.total_call_count += 1;
        if trace.observations.len() == EVENT_LIMIT {
            return;
        }
        let (phase, context) = current_execution_scope();
        let (properties, rejection) = match result {
            Ok(value) => (Some(value.clone()), None),
            Err(error) => (None, Some(error.to_string())),
        };
        trace.observations.push(ZoneVolumeObservation {
            zone_id,
            zone_name: zone_name.to_owned(),
            caller,
            phase,
            context,
            properties,
            rejection,
        });
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rejected() -> Result<ZoneGeometryProperties, ZoneVolumeError> {
        Err(ZoneVolumeError::UnsupportedTopology("open edges".into()))
    }

    #[test]
    fn copies_real_rejection_and_initializer_caller() {
        let result = rejected();
        let expected_line = line!() + 2;
        let (_, trace) = capture(true, || {
            record(ZoneId(4), "TEST", &result);
        });
        let trace = trace.expect("active observer");
        assert_eq!(trace.total_call_count, 1);
        assert_eq!(trace.observations.len(), 1);
        let row = &trace.observations[0];
        assert_eq!(row.zone_id, ZoneId(4));
        assert_eq!(row.caller.line(), expected_line);
        assert!(row.properties.is_none());
        assert_eq!(
            row.rejection.as_deref(),
            Some("unsupported automatic zone topology: open edges")
        );
        assert!(ACTIVE.with_borrow(Option::is_none));
    }

    #[test]
    #[allow(
        clippy::panic,
        reason = "Exercise observer restoration when the caller unwinds."
    )]
    fn nested_and_unwinding_captures_restore_the_real_outer_observer() {
        let result = rejected();
        let (_, outer) = capture(true, || {
            record(ZoneId(0), "OUTER", &result);
            let (_, inner) = capture(true, || record(ZoneId(1), "INNER", &result));
            assert_eq!(inner.expect("inner").total_call_count, 1);
            let panic = std::panic::catch_unwind(|| {
                capture(true, || {
                    record(ZoneId(2), "UNWIND", &result);
                    panic!("observer restoration");
                })
            });
            assert!(panic.is_err());
            record(ZoneId(0), "OUTER", &result);
        });
        let outer = outer.expect("outer");
        assert_eq!(outer.total_call_count, 2);
        assert!(
            outer
                .observations
                .iter()
                .all(|row| row.zone_id == ZoneId(0))
        );
        assert!(ACTIVE.with_borrow(Option::is_none));
        let (value, disabled) = capture(false, || {
            record(ZoneId(0), "DISABLED", &result);
            -0.0_f64
        });
        assert_eq!(value.to_bits(), (-0.0_f64).to_bits());
        assert!(disabled.is_none());
    }
}
